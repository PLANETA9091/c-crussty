package net.minecraft.world.entity;

import org.bukkit.event.entity.EntityRemoveEvent;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.util.Arrays;

import net.minecraft.server.MinecraftServer;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.tags.FluidTags;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.phys.Vec3;
import net.kyori.adventure.util.TriState;

/**
 * ITEMS-BATCH (TASK-446-B, vector cmp449_mega4): Rust rest-plane for
 * ItemEntity.tick — ONE bulk JNI per tick per region thread, zero per-entity
 * JNI (law-6 whole-subsystem vector; see RESEARCH-B-446-ITEMS.md).
 *
 * SHAPE (item_merge.rs / items_subsys2 precedents): the kernel ItemEntity.tick
 * body is whole-body replaced (cplug asm replace_body) by
 * {@link #tick(ItemEntity)}. Per tick the first bridge entry on a region
 * thread DRAINS the previous tick's snapshot batch with ONE native
 * {@code planeDecide([DI[I[J)I} (impl: src/items_batch.rs): rust classifies
 * every item from primitive snapshots (stride-12 doubles: id,x,y,z,vx,vy,vz,
 * age,pickupDelay,tickCount,flags,0) into FULL/REST decisions (resting
 * predicate = items_subsys2 planeResting replica; faithful recheck 1/32 —
 * the accepted items_subsys2 REST_PLANE deviation, fluid/fire onset latency
 * <= 32 ticks). Java then executes per item:
 *
 *  - REST  -> {@code e.inactiveTick()} (VANILLA method: pickupDelay--/age++/
 *           full despawn flow) + vanilla merge cadence (%40, private
 *           mergeWithNeighbours via MethodHandle — vanilla scan/radii/walls).
 *  - FULL  -> {@link #vanillaTick(ItemEntity)}: byte-parity replica of the
 *           vanilla ItemEntity.tick body (purpur-1.21.10 offsets 0..588,
 *           javap-verified; merge section invokes the original private
 *           mergeWithNeighbours; Entity.tick despawnTime gate replicated).
 *
 * The heavy per-tick world-voxel scans (fluid 32%, inside-blocks 28%,
 * collision 25% of the 29.5% items lane at vanilla anchors) therefore run
 * ONLY on full-path items (moving + 1/32 recheck) — settled items cost one
 * vanilla inactiveTick + one snapshot write.
 *
 * CYCLE-3 (TASK-448-B, RESEARCH-B-446-ITEMS.md §7): DRAIN-THROTTLE — the
 * snapshot append + the bulk planeDecide drain happen once per
 * {@link #DRAIN_EVERY} = 4 server ticks per region thread (meta[6]); between
 * drains items only probe the decision hash. Decisions are 4..8 ticks stale
 * (inside the accepted 32-tick recheck envelope, §7.5; unknown ids stay FULL
 * fail-open; fresh drops run FULL until their first sighting). The rust
 * recheck cadence is TICK-TRUE (tickCount − last_full_tick ≥ 32), so the
 * cadence is invariant under the throttle. Also new: X-RAY reason counters
 * (stats[3..7]: ground/fluid/hdsqr/pickupDelay/other) printed in the EFFECT
 * log — the per-run answer to "why does an item run FULL" (§7.6).
 *
 * GATE (double): ENABLED baked at compile from
 * {@code "cmp449_mega4".equals(getenv("CRUSSTY_LEVER_FLAG"))} AND the rust
 * side arms only on the same STRICT-eq flag. Fail-closed: MH resolve /
 * native probe / planeDecide rc&lt;0 / retransform failure -> vanilla replica
 * path (or, pre-serve, 100% vanilla class); empty/other flag = the class is
 * never defined or retransformed (vanilla bit-in-bit).
 *
 * S7-163: NO nested classes (single classfile output, pinned by the blob
 * checks). Zero allocation in steady state (grow-only scratch + ThreadLocals;
 * ids are monotonic so the per-tick decision hash never clears keys, values
 * are cleared every 64 drains).
 */
public final class ItemBatchOps {

    private static final String LEVER_FLAG = trimToEmpty(System.getenv("CRUSSTY_LEVER_FLAG"));

    /** Compile-time-baked java gate (double-gated with the rust env gate). */
    private static final boolean ENABLED = "cmp449_mega4".equals(LEVER_FLAG);

    private static final int PROBE_MAGIC = 0x1D46;

    private static final MethodHandle MH_MERGE_WITH_NEIGHBOURS; // ItemEntity private mergeWithNeighbours()V
    private static final MethodHandle MH_IS_MERGABLE;          // ItemEntity private isMergable()Z (vanilla call-site gate, offsets 463..467)
    private static final MethodHandle MH_DESPAWN_RATE;          // ItemEntity private int despawnRate
    private static final MethodHandle MH_DESPAWN_TIME;          // Entity private final int despawnTime

    /** true after successful static MethodHandle resolve. */
    private static final boolean READY;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    /** Permanently disarmed plane (native rc!=0, MH failure). */
    private static volatile boolean broken;

    // ---- natives (impl: src/items_batch.rs, RegisterNatives after define) ----
    private static native int planeProbe();
    /** One bulk call per tick per thread: classify the previous snapshot batch.
     *  rc 0 = ok, &lt;0 = structural break (java side disarms permanently). */
    private static native int planeDecide(double[] snap, int n, int[] out, long[] stats);

    // ---- snapshot layout (stride-12 doubles) ----
    private static final int STRIDE = 12;
    private static final int F_ON_GROUND = 1;
    private static final int F_IN_WATER = 2;
    private static final int F_IN_LAVA = 4;
    private static final int F_PORTAL = 8;
    private static final int F_REMOVED = 16;

    /** CYCLE-3: snapshot append + decide run once per 4 server ticks per
     *  region thread (the vanilla move-gate quantum); between drains items
     *  only probe the decision hash. Decision staleness ≤ 8 ticks — inside
     *  the accepted 32-tick recheck envelope (RESEARCH §7.5). */
    private static final int DRAIN_EVERY = 4;

    // ---- per-thread plane state (no nested classes: parallel ThreadLocals) ----
    // META: [0]=batchTick, [1]=top (snap fill), [2]=hashMask, [3]=hashCount,
    //       [4]=drainGen, [5]=lastEffectTick, [6]=lastDrainTick (throttle)
    private static final ThreadLocal<long[]> META =
            ThreadLocal.withInitial(() -> new long[8]);
    private static final ThreadLocal<double[]> SNAP =
            ThreadLocal.withInitial(() -> new double[STRIDE * 512]);
    private static final ThreadLocal<int[]> OUT =
            ThreadLocal.withInitial(() -> new int[512]);
    private static final ThreadLocal<long[]> STAT =
            ThreadLocal.withInitial(() -> new long[8]);
    private static final ThreadLocal<int[]> HKEY =
            ThreadLocal.withInitial(() -> new int[1024]);
    private static final ThreadLocal<int[]> HVAL =
            ThreadLocal.withInitial(() -> new int[1024]);

    static {
        boolean ok = false;
        MethodHandle merge = null;
        MethodHandle mergable = null;
        MethodHandle rate = null;
        MethodHandle time = null;
        try {
            Lookup itemLookup = MethodHandles.privateLookupIn(ItemEntity.class, MethodHandles.lookup());
            merge = itemLookup.unreflect(ItemEntity.class.getDeclaredMethod("mergeWithNeighbours"));
            mergable = itemLookup.unreflect(ItemEntity.class.getDeclaredMethod("isMergable"));
            rate = itemLookup.findGetter(ItemEntity.class, "despawnRate", int.class);
            Lookup entityLookup = MethodHandles.privateLookupIn(Entity.class, MethodHandles.lookup());
            time = entityLookup.findGetter(Entity.class, "despawnTime", int.class);
            ok = true;
        } catch (Throwable t) {
            LOG.severe("[crussty-plugin] items_batch: MethodHandle resolve failed: " + t);
        }
        MH_MERGE_WITH_NEIGHBOURS = merge;
        MH_IS_MERGABLE = mergable;
        MH_DESPAWN_RATE = rate;
        MH_DESPAWN_TIME = time;
        READY = ok;
        if (READY) {
            LOG.info("[crussty-plugin] items_batch: bridge ready (enabled=" + ENABLED + ")");
        }
    }

    private ItemBatchOps() {}

    private static String trimToEmpty(String s) {
        return s == null ? "" : s.trim();
    }

    // ------------------------------------------------------------------
    // Bridge entry (whole-body replacement target of ItemEntity.tick()V)
    // ------------------------------------------------------------------

    public static void tick(ItemEntity e) {
        if (!ENABLED || !READY || broken) {
            vanillaTick(e);
            return;
        }
        long[] meta = META.get();
        long st = MinecraftServer.getServer().getTickCount();
        // CYCLE-3 drain-throttle: append+decide once per DRAIN_EVERY ticks;
        // between drains items only probe the decision hash. meta[6]=0 before
        // the first drain -> the first entry always drains (empty drain is a
        // no-op that still stamps meta[0]/meta[6]).
        boolean drainTick = st - meta[6] >= DRAIN_EVERY;
        if (drainTick && (meta[1] != 0L || st != meta[0])) {
            drain(meta, st);
            if (broken) {
                vanillaTick(e);
                return;
            }
        }
        if (drainTick) {
            append(meta, e);
        }
        // decisions hold the last drained batch's classification (up to 8
        // ticks stale under the throttle; unknown ids -> FULL, fail-open)
        int action = lookup(meta, e.getId());
        if (action == 1) {
            // REST: vanilla minimal body (counters + full despawn flow) +
            // vanilla merge cadence for settled items (k=40 = the vanilla
            // unmoved branch, offsets 441..450; isMergable call-site gate
            // offsets 463..467; the private body self-gates isMergable too).
            e.inactiveTick();
            if (e.tickCount % 40 == 0 && !e.level().isClientSide() && isMergable(e)) {
                invokeVanillaMerge(e);
            }
        } else {
            vanillaTick(e);
        }
    }

    // ------------------------------------------------------------------
    // Batch drain / snapshot / decision-hash (zero-alloc steady state)
    // ------------------------------------------------------------------

    private static void drain(long[] meta, long st) {
        // TASK-447-B2 (run 36004849498 crash root-cause): meta[1] is the SNAPSHOT
        // FILL POSITION IN DOUBLES (append() adds STRIDE per item) — the
        // planeDecide contract wants the ITEM COUNT (rust decide_batch reads
        // n*STRIDE doubles and writes n outputs). The v1 drain passed the raw
        // doubles count as n: for any real batch the rust structural guard
        // (n*STRIDE > snap.len -> rc=-1) fired BEFORE any read/write (fail-closed
        // held — no heap corruption), the plane disarmed on its first populated
        // tick (ARM 13:30:04 -> rc=-1 13:30:05, run 36004849498) and the leg
        // measured the vanilla-replica fallback. selfTest passed because it
        // passes the item count (n=1) explicitly. Fixed: convert doubles ->
        // items here, and fail-closed on stride misalignment BEFORE the native.
        int nDoubles = (int) meta[1];
        meta[0] = st;
        meta[1] = 0L;
        meta[6] = st; // throttle anchor (CYCLE-3)
        if (nDoubles <= 0) {
            return;
        }
        if ((nDoubles % STRIDE) != 0) {
            // Snapshot invariant broken (append is the only writer and adds
            // whole strides) — structural break, disarm with nothing written.
            broken = true;
            LOG.severe("[crussty-plugin] items_batch: stride misalignment ("
                    + nDoubles + " doubles) — plane broken, permanent vanilla-replica path");
            return;
        }
        int n = nDoubles / STRIDE; // ITEM count — the planeDecide contract
        double[] snap = SNAP.get();
        int[] out = OUT.get();
        if (out.length < n) {
            out = new int[Math.max(n, out.length * 2)];
            OUT.set(out);
        }
        long[] stat = STAT.get();
        int rc;
        try {
            rc = planeDecide(snap, n, out, stat);
        } catch (Throwable t) {
            rc = -99;
        }
        if (rc != 0) {
            broken = true;
            LOG.severe("[crussty-plugin] items_batch: planeDecide rc=" + rc
                    + " — plane broken, permanent vanilla-replica path");
            return;
        }
        buildHash(meta, snap, out, n);
        // EFFECT marker: greppable proof the plane is live (max once / 1200
        // ticks / thread; stdout is NOT purged before markers per canon).
        // CYCLE-3: also the X-ray — why items ran FULL (stats[3..7], primary
        // reason priority portal/removed -> fluid -> pickupDelay -> ground ->
        // hdsqr; see RESEARCH-B-446-ITEMS.md §7.6).
        if (st - meta[5] >= 1200L) {
            meta[5] = st;
            LOG.info("[crussty-plugin] items_batch: EFFECT tick=" + st + " n=" + n
                    + " rest=" + stat[1] + " full=" + stat[0] + " rechecks=" + stat[2]
                    + " throttle=4 ground=" + stat[3] + " fluid=" + stat[4]
                    + " hdsqr=" + stat[5] + " pd=" + stat[6] + " other=" + stat[7]
                    + " items=" + n + " doubles=" + nDoubles);
        }
    }

    private static void append(long[] meta, ItemEntity e) {
        double[] snap = SNAP.get();
        int top = (int) meta[1];
        if (top + STRIDE > snap.length) {
            snap = Arrays.copyOf(snap, snap.length * 2);
            SNAP.set(snap);
        }
        int b = top;
        snap[b] = (double) e.getId();
        snap[b + 1] = e.getX();
        snap[b + 2] = e.getY();
        snap[b + 3] = e.getZ();
        Vec3 v = e.getDeltaMovement();
        snap[b + 4] = v.x;
        snap[b + 5] = v.y;
        snap[b + 6] = v.z;
        snap[b + 7] = (double) e.age;
        snap[b + 8] = (double) e.pickupDelay;
        snap[b + 9] = (double) e.tickCount;
        int f = 0;
        if (e.onGround()) {
            f |= F_ON_GROUND;
        }
        if (e.isInWater()) {
            f |= F_IN_WATER;
        }
        if (e.isInLava()) {
            f |= F_IN_LAVA;
        }
        if (e.portalProcess != null) {
            f |= F_PORTAL;
        }
        if (e.isRemoved()) {
            f |= F_REMOVED;
        }
        snap[b + 10] = (double) f;
        snap[b + 11] = 0.0D;
        meta[1] = top + STRIDE;
    }

    /** id -> action hash (open addressing, monotonic ids, values cleared
     *  every 64 drains; bounded probe walks make a hang impossible). */
    private static void buildHash(long[] meta, double[] snap, int[] out, int n) {
        long mask = meta[2];
        int[] hk = HKEY.get();
        int[] hv = HVAL.get();
        int need = n * 2 + 1;
        if (need > hk.length) {
            int cap = hk.length;
            while (cap < need) {
                cap <<= 1;
            }
            hk = new int[cap];
            hv = new int[cap];
            HKEY.set(hk);
            HVAL.set(hv);
            mask = cap - 1L;
            meta[2] = mask;
            meta[4] = 0L;
        }
        long gen = meta[4] + 1L;
        meta[4] = gen;
        if ((gen & 63L) == 1L) {
            Arrays.fill(hv, 0);
        }
        int m = (int) mask;
        int count = 0;
        for (int i = 0; i < n; i++) {
            int id = (int) snap[i * STRIDE];
            int action = out[i] + 1; // 1=FULL, 2=REST
            int slot = mix(id) & m;
            int walk = 0;
            while (hv[slot] != 0 && hk[slot] != id && walk <= m) {
                slot = (slot + 1) & m;
                walk++;
            }
            if (hv[slot] == 0) {
                count++;
            }
            hk[slot] = id;
            hv[slot] = action;
        }
        meta[3] = count;
    }

    private static int lookup(long[] meta, int id) {
        int m = (int) meta[2];
        int[] hk = HKEY.get();
        int[] hv = HVAL.get();
        int slot = mix(id) & m;
        int walk = 0;
        while (hv[slot] != 0 && walk <= m) {
            if (hk[slot] == id) {
                return hv[slot] - 1;
            }
            slot = (slot + 1) & m;
            walk++;
        }
        return 0;
    }

    private static int mix(int id) {
        int h = id * 0x9E3779B9;
        return h ^ (h >>> 16);
    }

    // ------------------------------------------------------------------
    // FULL path: byte-parity replica of vanilla ItemEntity.tick [0..588]
    // (javap purpur-1.21.10; merge + despawn via original private bodies)
    // ------------------------------------------------------------------

    private static void vanillaTick(ItemEntity e) {
        ItemStack stack = e.getItem(); // offset 0 — единственный synched-read на тик
        if (stack.isEmpty()) {
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
            return;
        }
        // Entity.tick (offsets 20..23): despawnTime-гейт + baseTick
        int despawnTime = getDespawnTime(e);
        if (despawnTime >= 0 && e.totalEntityAge >= despawnTime) {
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
        } else {
            e.baseTick();
        }
        // offsets 24..48
        if (e.pickupDelay > 0 && e.pickupDelay != 32767) {
            e.pickupDelay--;
        }
        // offsets 51..72
        e.xo = e.getX();
        e.yo = e.getY();
        e.zo = e.getZ();
        Vec3 vec3 = e.getDeltaMovement();
        // offsets 80..139: fluid-movement / gravity
        if (e.isInWater() && e.getFluidHeight(FluidTags.WATER) > 0.10000000149011612D) {
            setFluidMovement(e, 0.9900000095367432D);
        } else if (e.isInLava() && e.getFluidHeight(FluidTags.LAVA) > 0.10000000149011612D) {
            setFluidMovement(e, 0.949999988079071D);
        } else {
            e.applyGravity();
        }
        // offsets 140..225: noPhysics / moveTowardsClosestSpace
        if (e.level().isClientSide()) {
            e.noPhysics = false;
        } else {
            e.noPhysics = !e.level().noCollision(e, e.getBoundingBox().deflate(1.0E-7D));
            if (e.noPhysics) {
                e.moveTowardsClosestSpace(e.getX(),
                        (e.getBoundingBox().minY + e.getBoundingBox().maxY) / 2.0D, e.getZ());
            }
        }
        // offsets 226..375: move-гейт — КРИТИЧНО: ванильный скип (ifne 376)
        // перепрыгивает move + applyEffectsFromBlocks + friction + bounce
        // ЦЕЛИКОМ (ветка уходит прямо в merge-окно на 376) — реплика держит
        // тот же блок под одним !skipMove.
        boolean skipMove = e.onGround()
                && e.getDeltaMovement().horizontalDistanceSqr() <= 9.999999747378752E-6D
                && (e.tickCount + e.getId()) % 4 != 0;
        if (!skipMove) {
            e.move(MoverType.SELF, e.getDeltaMovement());
            // offset 272
            e.applyEffectsFromBlocks();
            // offsets 276..341: friction
            float f = 0.98F;
            if (e.frictionState == TriState.FALSE) {
                f = 1.0F;
            } else if (e.onGround()) {
                f = e.level().getBlockState(e.getBlockPosBelowThatAffectsMyMovement()).getBlock()
                        .getFriction() * 0.98F;
            }
            e.setDeltaMovement(e.getDeltaMovement().multiply(f, 0.9800000190734863D, f));
            // offsets 342..375: bounce
            if (e.onGround()) {
                Vec3 vec31 = e.getDeltaMovement();
                if (vec31.y < 0.0D) {
                    e.setDeltaMovement(vec31.multiply(1.0D, -0.5D, 1.0D));
                }
            }
        }
        // offsets 376..473: merge window (k = moved ? 2 : 40; isMergable
        // call-site gate offsets 463..467; the original private
        // mergeWithNeighbours self-gates isMergable at offset 0 — вызов
        // напрямую = ванильное поведение)
        boolean moved = Mth.floor(e.xo) != Mth.floor(e.getX())
                || Mth.floor(e.yo) != Mth.floor(e.getY())
                || Mth.floor(e.zo) != Mth.floor(e.getZ());
        int k = moved ? 2 : 40;
        if (e.tickCount % k == 0 && !e.level().isClientSide() && isMergable(e)) {
            invokeVanillaMerge(e);
        }
        // offsets 474..498
        if (e.age != -32768) {
            e.age++;
        }
        // offsets 499..506
        e.hasImpulse = e.hasImpulse | e.updateInWaterStateAndDoFluidPushing();
        // offsets 507..543
        if (!e.level().isClientSide()) {
            double d0 = e.getDeltaMovement().subtract(vec3).lengthSqr();
            if (d0 > 0.01D) {
                e.hasImpulse = true;
            }
        }
        // offsets 544..588: despawn — ванильный flow
        if (!e.level().isClientSide() && e.age >= getDespawnRate(e)) {
            if (org.bukkit.craftbukkit.event.CraftEventFactory.callItemDespawnEvent(e).isCancelled()) {
                e.age = 0;
                return;
            }
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
        }
    }

    /** Реплика приватного setFluidMovement(double) (offsets 0..45).
     *  ВНИМАНИЕ: y-инкремент в ядре = FLOAT 5.0E-4f через f2d (не 0.005!),
     *  javap-верифицировано на живом round-396-a jar 2026-09-24. */
    private static void setFluidMovement(ItemEntity e, double mult) {
        Vec3 vec3 = e.getDeltaMovement();
        e.setDeltaMovement(vec3.x * mult,
                vec3.y + (vec3.y < 0.05999999865889549D ? 5.0E-4F : 0.0F),
                vec3.z * mult);
    }

    private static void invokeVanillaMerge(ItemEntity e) {
        try {
            MH_MERGE_WITH_NEIGHBOURS.invokeExact(e);
        } catch (Throwable t) {
            broken = true;
            throw new RuntimeException(t);
        }
    }

    private static boolean isMergable(ItemEntity e) {
        try {
            return (boolean) MH_IS_MERGABLE.invokeExact(e);
        } catch (Throwable t) {
            broken = true;
            throw new RuntimeException(t);
        }
    }

    private static int getDespawnRate(ItemEntity e) {
        try {
            return (int) MH_DESPAWN_RATE.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    private static int getDespawnTime(Entity e) {
        try {
            return (int) MH_DESPAWN_TIME.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    // ------------------------------------------------------------------
    // selfTest (fail-closed: called by rust BEFORE retransform; false ->
    // the hook never arms, vanilla bit-in-bit)
    // ------------------------------------------------------------------

    /** Synthetic snapshots use sentinel ids 0x5E170000+n (GC'd by staleness). */
    public static boolean selfTest() {
        try {
            if (!ENABLED || !READY) {
                return false;
            }
            if (planeProbe() != PROBE_MAGIC) {
                return false;
            }
            long[] stat = new long[8];
            double[] s = new double[STRIDE];
            int[] out = new int[1];
            int base = 0x5E170000;
            // A: settled item — REST decisions appear with 1/32 faithful rechecks
            pack(s, base, 10.5D, 64.0D, 10.5D, 0, 0, 0, 100, 0, 5000, F_ON_GROUND);
            int rest = 0;
            int full = 0;
            for (int i = 0; i < 70; i++) {
                s[9] = (double) (5000 + i);
                if (planeDecide(s, 1, out, stat) != 0) {
                    return false;
                }
                if (out[0] == 1) {
                    rest++;
                } else {
                    full++;
                }
            }
            if (rest == 0 || full == 0 || rest + full != 70) {
                return false;
            }
            // B: in-water item — always FULL (buoyancy path stays vanilla)
            pack(s, base + 1, 10.5D, 64.0D, 10.5D, 0, 0, 0, 100, 0, 5000,
                    F_ON_GROUND | F_IN_WATER);
            for (int i = 0; i < 5; i++) {
                if (planeDecide(s, 1, out, stat) != 0) {
                    return false;
                }
                if (out[0] != 0) {
                    return false;
                }
            }
            // C: moving item (hdSqr above the vanilla gate) — always FULL
            pack(s, base + 2, 10.5D, 64.0D, 10.5D, 0.01D, 0, 0, 100, 0, 5000,
                    F_ON_GROUND);
            if (planeDecide(s, 1, out, stat) != 0) {
                return false;
            }
            if (out[0] != 0) {
                return false;
            }
            // D: pickup-delayed fresh drop — always FULL
            pack(s, base + 3, 10.5D, 64.0D, 10.5D, 0, 0, 0, 100, 10, 5000,
                    F_ON_GROUND);
            if (planeDecide(s, 1, out, stat) != 0) {
                return false;
            }
            if (out[0] != 0) {
                return false;
            }
            // E (cycle-3): throttle cadence — sightings 4 ticks apart must
            // recheck on the TICK axis (call 8 = tickCount delta 32), NOT on
            // the sighting count; landing FULL first, REST between.
            int thId = base + 4;
            long[] statE = new long[8];
            int fulls = 0;
            for (int k = 0; k < 10; k++) {
                pack(s, thId, 10.5D, 64.0D, 10.5D, 0, 0, 0, 100, 0, 5000 + 4 * k,
                        F_ON_GROUND);
                if (planeDecide(s, 1, out, statE) != 0) {
                    return false;
                }
                if (out[0] == 0) {
                    fulls++;
                }
                if (k >= 1 && k <= 7 && out[0] != 1) {
                    return false; // 4-tick-spaced sighting must be REST
                }
                if (k == 8 && out[0] != 0) {
                    return false; // recheck fires at tickCount delta 32
                }
                if (k == 9 && out[0] != 1) {
                    return false;
                }
            }
            if (fulls != 2) {
                return false; // exactly landing + 32-tick recheck
            }
            // F (cycle-3): X-ray reasons — an in-water item counts as fluid.
            pack(s, base + 5, 10.5D, 64.0D, 10.5D, 0, 0, 0, 100, 0, 6000,
                    F_ON_GROUND | F_IN_WATER);
            if (planeDecide(s, 1, out, statE) != 0) {
                return false;
            }
            if (out[0] != 0 || statE[4] < 1) {
                return false;
            }
            return true;
        } catch (Throwable t) {
            LOG.severe("[crussty-plugin] items_batch: selfTest threw: " + t);
            return false;
        }
    }

    private static void pack(double[] s, int id, double x, double y, double z,
            double vx, double vy, double vz, int age, int pd, int tickCount, int flags) {
        Arrays.fill(s, 0.0D);
        s[0] = (double) id;
        s[1] = x;
        s[2] = y;
        s[3] = z;
        s[4] = vx;
        s[5] = vy;
        s[6] = vz;
        s[7] = (double) age;
        s[8] = (double) pd;
        s[9] = (double) tickCount;
        s[10] = (double) flags;
        s[11] = 0.0D;
    }
}
