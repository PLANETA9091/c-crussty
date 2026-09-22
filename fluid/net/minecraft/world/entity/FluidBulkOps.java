package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import net.minecraft.core.BlockPos;
import net.minecraft.server.MinecraftServer;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TASK-416-C FLUID-BULK v2 (lever cmp416_fluid, flag STRICT-eq): subsystem
 * rebuild of the fluid-update plane Entity.updateFluidHeightAndDoFluidPushing
 — the top untouched lane of the era (16.0-16.7% vanilla, 13.76% on the
 * meganav-era mc1 profile).
 *
 * RECON (RESEARCH-C-416.md, mc1 leaf split): of the fluid lane the DAILY BREAD
 * is re-reading the same cells 10-100x per tick (PalettedContainer.get 3.0% +
 * SimpleBitStorage.get 1.2% + VarHandle chains + LevelChunk.getFluidState
 * 0.8% + FluidState.getType 0.5%); the wet math (getHeight/getFlow) is
 * irreducible vanilla work. 150k entities pile inside 0.5 blocks — every
 * member of a pile re-sweeps the same box cells.
 *
 * DESIGN (anti-lesson bl2 AMENDED — NO gather pass): a thread-confined CELL
 * LUT with per-slot validity stamps. Each cell's FluidState is read EXACTLY
 * ONCE per (thread, tick, GEN) — the first entity to touch the cell gathers
 * it for the whole pile (lazy probe-dedup); every later sweep of that cell is
 * one array probe. No separate traversal exists (the bl2 gather-scan cost is
 * structurally absent).
 *
 * v2 FIXES over the TASK-415-C draft (RESEARCH-C-416.md §"Отличия"):
 * 1. rustOk deadlock: the draft tested `!rustOk` before the only site that
 *    could set it (maybeEpoch) — the LUT could never arm. Entry now stamps
 *    the plane FIRST and drives maybeEpoch off the (tick) stamp.
 * 2. Unbounded retry: a GEN bump racing BOTH the sweep and its retry
 *    recursed. Now exactly ONE retry, then the bit-exact vanilla body.
 * 3. Per-tick bulk clear: the draft Arrays.fill'ed the 65536-slot ref array
 *    at every (tick, GEN) rollover (~65k card-table writes/thread/tick for
 *    nothing). Validity now rides a per-slot packed (tick,GEN) stamp array —
 *    rollover is a stamp swap, ZERO bulk writes.
 * 4. Lever renamed cmp415_fluid -> cmp416_fluid (STRICT-eq; empty/foreign
 *    flag = vanilla bit-in-byte).
 *
 * INVALIDATION (RECON-43 ec880c2 write-bump contract, NOT the forbidden
 * fluid_dirty memo / fluid_bitmask): the ONE retargeted write site
 * LevelChunk.setBlockState -> LevelChunkSection.setBlockState delegates
 * through {@link #secWrite}, which bumps the GLOBAL fluid generation
 * {@link #GEN} on a real fluid-state change (FluidState singleton
 * ref-compare). Slot validity = (tick, GEN) stamp; a mid-tick write bumps
 * GEN and the next sweep refills; a sweep that observed a GEN bump mid-flight
 * is redone ONCE, then falls to the vanilla body — never mixed.
 *
 * ONE BULK JNI/TICK (law 6): {@code fluidBulk(tick, io)} is invoked once per
 * server tick (JVM-wide stamp guard, EPOCH_LOCK, the MobScanOps protocol).
 * RUST (src/fluid_bulk.rs) owns the tick-epoch authority: validates the
 * census (calls/hits/wet/slow/presence/fill), enforces the fail-closed
 * disarm policy, and answers with the lifetime LUT fill count. Per-entity
 * JNI is absent by construction.
 *
 * VANILLA SEMANTICS (law 4): the sweep body is the javap transcript of the
 * vanilla method (identical to the deployed FluidPushGuardHook.slow): same
 * deflated-box cell bounds, same float/double height arithmetic, same
 * lastLavaContact writes, same flow accumulation ladder
 * (maxDepth<0.4 ? flow.scale(maxDepth) : flow), same normalize/scale/0.003/
 * 0.0045 push tail, same fluidHeight.put(tag, maxDepth) and boolean return.
 * With the lever flag unset the whole module never loads and Entity runs
 * vanilla bit-in-byte.
 *
 * FAIL-CLOSED: any Throwable in the fast path -> permanent vanilla body
 * (bit-exact); fluidBulk ERR_STRUCT -> LUT disarmed forever (vanilla); a
 * second GEN race inside the retry -> vanilla for that call.
 */
public final class FluidBulkOps {

    private FluidBulkOps() {}

    /** STRICT-eq lever gate (round-400 protocol; empty/foreign flag = vanilla). */
    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        if (f == null) return false;
        return f.trim().equals("cmp416_fluid");
    }

    private static final boolean ENABLED = leverEnabled();

    /** Global fluid generation (RECON-43 write-bump contract). Bumped by
     * {@link #secWrite} on real fluid-state changes; LUT validity key. */
    public static volatile long GEN = 0L;

    // ---------------- census (drained by fluidBulk each tick) ----------------
    public static volatile long C_CALLS = 0, C_HITS = 0, C_WET = 0,
            C_SLOW = 0, C_PRESENCE = 0, C_FILL = 0;

    // ---------------- the ONE bulk JNI per tick (src/fluid_bulk.rs) ----------
    private static native int fluidProbe();
    private static native int fluidBulk(int tick, long[] io);

    static final int PROBE_MAGIC = 0x4642; // "FB"
    static final int ERR_STRUCT = -1;
    static final int ERR_RANGE = -2;

    private static volatile boolean nativeOk;
    /** Rust policy verdict: true while the bulk channel is healthy. */
    private static volatile boolean rustOk;
    /** Permanent vanilla (ERR_STRUCT) — no more JNI attempts, no locking. */
    private static volatile boolean DISARMED;
    private static volatile long STAMPED_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    private static boolean probeOnce() {
        if (nativeOk) return true;
        synchronized (FluidBulkOps.class) {
            if (nativeOk) return true;
            try {
                nativeOk = fluidProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** ARM/EFFECT marker (one-shot, lands in server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;

    /**
     * Tick-epoch handshake: ONCE per server tick JVM-wide, the rust bulk
     * native validates the epoch and drains the census deltas. rc<0 keeps
     * the plane alive but disarmed (rustOk=false -> per-call vanilla body,
     * never wrong); ERR_STRUCT disarms permanently.
     */
    private static void maybeEpoch(long tick) {
        if (STAMPED_TICK == tick && rustOk) return; // hot: stamped this tick
        synchronized (EPOCH_LOCK) {
            if (STAMPED_TICK == tick && rustOk) return;
            if (!probeOnce()) {
                DISARMED = true;
                rustOk = false;
                System.err.println(
                        "[crussty-plugin] cmp416_fluid: fluidProbe failed — LUT disarmed to vanilla");
                STAMPED_TICK = tick;
                return;
            }
            long[] io = new long[8];
            io[0] = tick;
            io[1] = C_CALLS; io[2] = C_HITS; io[3] = C_WET;
            io[4] = C_SLOW; io[5] = C_PRESENCE; io[6] = C_FILL;
            io[7] = GEN;
            int rc;
            try {
                rc = fluidBulk((int) tick, io);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                rustOk = false;
                DISARMED = true;
                System.err.println(
                        "[crussty-plugin] cmp416_fluid: fluidBulk ERR_STRUCT — LUT disarmed to vanilla");
            } else {
                rustOk = true;
            }
            if (!ARM_LOGGED && rc >= 0) {
                ARM_LOGGED = true;
                System.err.println(
                        "[crussty-plugin] cmp416_fluid: EFFECT armed (bulk epoch stamped at tick " + tick
                                + ", rc=" + rc + ", lut fills: " + io[7] + ")");
            }
            STAMPED_TICK = tick;
        }
    }

    // ---------------- thread-confined LUT + presence cache ----------------

    static final int LUT_BITS = 16; // 65536 slots (~1.5 MiB/thread with stamps)
    static final int LUT_MASK = (1 << LUT_BITS) - 1;

    static final int PRES_BITS = 12; // 4096 columns
    static final int PRES_MASK = (1 << PRES_BITS) - 1;

    static final class Plane {
        final long[] key = new long[1 << LUT_BITS];
        /** Packed (tick, GEN) validity stamp per slot — replaces the v1
         * bulk-clear: rollover swaps ONE long, slots go stale by stamp. */
        final long[] tag = new long[1 << LUT_BITS];
        final FluidState[] val = new FluidState[1 << LUT_BITS];
        final long[] pkey = new long[1 << PRES_BITS];
        final boolean[] pval = new boolean[1 << PRES_BITS];
        long gen = Long.MIN_VALUE;
        long tick = Long.MIN_VALUE;
        long stamp = Long.MIN_VALUE;
    }

    private static final ThreadLocal<Plane> PLANE =
            ThreadLocal.withInitial(Plane::new);

    private static long mix(long k) {
        k *= 0x9E3779B97F4A7C15L;
        k ^= k >>> 29;
        k *= 0xBF58476D1CE4E5B9L;
        k ^= k >>> 32;
        return k;
    }

    /** Pack an (x,y,z) cell into a 64-bit probe key; y has 12 bits,
     * x/z 26 bits each — disjoint fields, no aliasing inside world bounds. */
    private static long pack(int x, int y, int z) {
        return ((x & 0x3FFFFFFL) << 38) | ((z & 0x3FFFFFFL) << 12) | (y & 0xFFFL);
    }

    /** Packed (tick, GEN) validity stamp: tick in the high 32 bits (the JVM
     * tick counter is a non-negative int), GEN in the low 32 (a wrap after
     * 2^32 fluid-changing writes can at worst force one spurious refill). */
    private static long stamp(long tick, long gen) {
        return (tick << 32) | (gen & 0xFFFFFFFFL);
    }

    // ---------------- the whole-body bridge ----------------

    public static boolean updateFluidHeightAndDoFluidPushing(Entity self, TagKey<Fluid> tag, double speed) {
        if (!ENABLED || DISARMED) {
            return slowVanilla(self, tag, speed);
        }
        final Plane p = PLANE.get();
        final long tick = MinecraftServer.getServer().getTickCount();
        if (p.tick != tick || p.gen != GEN) {
            // (tick, GEN) rollover: swap the stamp (NO bulk clear — v2).
            p.tick = tick;
            p.gen = GEN;
            p.stamp = stamp(tick, GEN);
        }
        // ONE bulk JNI/tick: the first sweep of a tick stamps the epoch and
        // drains the census to rust (subsequent sweeps see STAMPED_TICK).
        if (STAMPED_TICK != tick || !rustOk) {
            maybeEpoch(tick);
            if (!rustOk) {
                return slowVanilla(self, tag, speed);
            }
        }
        try {
            // Bounded GEN-race protocol: ONE retry, then the vanilla body.
            // The sweep writes NO entity state (except lastLavaContact, whose
            // value is overwritten by the clean pass); the fluidHeight/push
            // tail runs EXACTLY ONCE, from a non-raced pass — a raced sweep
            // can never double-apply the push (v1 side-effect bug).
            Res r = RES.get();
            long g0 = GEN;
            int st = sweep(self, tag, speed, p, r);
            if (st == SWEEP_UNLOADED) {
                return false;
            }
            if (g0 != GEN) {
                C_SLOW++;
                p.gen = GEN;
                p.stamp = stamp(tick, GEN);
                st = sweep(self, tag, speed, p, r);
                if (st == SWEEP_UNLOADED) {
                    return false;
                }
                if (GEN != p.gen) {
                    C_SLOW++;
                    return slowVanilla(self, tag, speed);
                }
            }
            return tail(self, tag, speed, r);
        } catch (Throwable th) {
            // Fail-dominant: any inconsistency degrades this plane to the
            // bit-exact vanilla body forever (matches fluid_guard protocol).
            rustOk = false;
            DISARMED = true;
            return slowVanilla(self, tag, speed);
        }
    }

    // Sweep protocol: SWEEP_UNLOADED = vanilla early-out (no fluidHeight put,
    // no push — vanilla touchingUnloadedChunk tail); SWEEP_DONE = full sweep,
    // results in the Res holder, caller runs the tail exactly once.
    private static final int SWEEP_UNLOADED = 0;
    private static final int SWEEP_DONE = 1;

    /** Thread-confined sweep result (entity ticking is one-thread-per-region
     * and never re-enters — the FluidPushGuardHook TL-buffer invariant). */
    private static final class Res {
        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth;
        boolean inFluid;
        int flowCount;
    }

    private static final ThreadLocal<Res> RES = ThreadLocal.withInitial(Res::new);

    /** Bit-exact vanilla sweep over the LUT data plane. SIDE-EFFECT-FREE on
     * entity state except lastLavaContact (mid-loop in vanilla, overwritten
     * by the clean pass after a retry); the fluidHeight/push tail lives in
     * {@link #tail} so a raced sweep can never double-apply a push. */
    private static int sweep(Entity self, TagKey<Fluid> tag, double speed, Plane p, Res r) {
        C_CALLS++;
        if (self.touchingUnloadedChunk()) {
            return SWEEP_UNLOADED;
        }
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushFlag = self.isPushedByFluid();

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0;
        boolean inFluid = false;
        int flowCount = 0;
        final long now = p.stamp;

        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        // (x,y,z) iteration identical to the vanilla body — the Res holder
        // only collects what the tail needs.
        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    long k = pack(x, y, z);
                    int slot = (int) mix(k) & LUT_MASK;
                    FluidState fs;
                    if (p.tag[slot] == now && p.key[slot] == k) {
                        fs = p.val[slot];
                        C_HITS++;
                    } else {
                        // vanilla fetch: the only read of this cell on this
                        // thread this (tick, gen) — the flat-section body of
                        // the guard, via Level.getFluidState semantics.
                        fs = fetch(p, level, x, y, z, minSection);
                        C_FILL++;
                        p.key[slot] = k;
                        p.tag[slot] = now;
                        p.val[slot] = fs;
                    }
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    C_WET++;
                    mpos.set(x, y, z);
                    if (tag == FluidTags.LAVA) {
                        self.lastLavaContact = mpos.immutable();
                    }
                    // javap-exact arithmetic: (float)y + height in FLOAT, then
                    // widened, minus box.minY in double (bytecode i2f/fadd/f2d).
                    double d0 = (double) ((float) y + fs.getHeight(level, mpos)) - box.minY;
                    if (d0 < 0.0) {
                        continue;
                    }
                    inFluid = true;
                    maxDepth = Math.max(maxDepth, d0);
                    if (!pushFlag) {
                        continue;
                    }
                    flowCount++; // javap @478-482: counted BEFORE getFlow
                    Vec3 flow = fs.getFlow(level, mpos);
                    if (maxDepth < 0.4) {
                        flowAcc = flowAcc.add(flow.scale(maxDepth));
                    } else {
                        flowAcc = flowAcc.add(flow);
                    }
                }
            }
        }
        r.flowAcc = flowAcc;
        r.maxDepth = maxDepth;
        r.inFluid = inFluid;
        r.flowCount = flowCount;
        return SWEEP_DONE;
    }

    /** The vanilla tail, run EXACTLY ONCE per bridge call from a non-raced
     * sweep: fluidHeight.put(tag, maxDepth) + the normalize/scale/0.003/
     * 0.0045 push ladder (javap @540-640). */
    private static boolean tail(Entity self, TagKey<Fluid> tag, double speed, Res r) {
        self.fluidHeight.put(tag, r.maxDepth);
        if (r.flowAcc == Vec3.ZERO) {
            return r.inFluid;
        }
        Vec3 flowAcc = r.flowAcc.scale(1.0 / r.flowCount); // javap @572-578
        Vec3 delta = self.getDeltaMovement();
        if (!(self instanceof net.minecraft.world.entity.player.Player)) {
            flowAcc = flowAcc.normalize();
        }
        flowAcc = flowAcc.scale(speed);
        if (Math.abs(delta.x) < 0.003 && Math.abs(delta.z) < 0.003 && flowAcc.length() < 0.0045) {
            flowAcc = flowAcc.normalize().scale(0.0045);
        }
        self.setDeltaMovement(delta.add(flowAcc));
        return true;
    }

    /**
     * The vanilla per-cell fetch with the guard's flat-section chunk protocol
     * (load=false after touchingUnloadedChunk guaranteed presence). Cached
     * column presence rides the documented invariant that chunk UNLOADS are
     * processed outside the region entity-tick phases; a presence-cache hit
     * that disagrees with the live map escalates to load=true (self-healing).
     */
    private static FluidState fetch(Plane p, Level level, int x, int y, int z, int minSection) {
        ChunkSource source = level.getChunkSource();
        int cx = x >> 4, cz = z >> 4;
        // presence key: hash of the column pair (its own table — key universe
        // is independent of the cell LUT, but kept hash-mixed for spread)
        long pk = mix(pack(cx, 0, cz)) | Long.MIN_VALUE;
        int ps = (int) mix(pk) & PRES_MASK;
        ChunkAccess chunk;
        if (p.pkey[ps] == pk && p.pval[ps]) {
            chunk = source.getChunk(cx, cz, ChunkStatus.FULL, false);
            if (chunk == null) {
                // presence said loaded, the map disagrees — vanilla retry.
                chunk = source.getChunk(cx, cz, ChunkStatus.FULL, true);
            }
        } else {
            C_PRESENCE++;
            chunk = source.getChunk(cx, cz, ChunkStatus.FULL, true);
            p.pkey[ps] = pk;
            p.pval[ps] = true;
        }
        LevelChunkSection sec = chunk.getSections()[(y >> 4) - minSection];
        return ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
    }

    /**
     * The faithful vanilla body (FluidPushGuardHook.slow verbatim semantics —
     * the deployed javap transcript of Entity.updateFluidHeightAndDoFluidPushing).
     * Used while the LUT plane is unarmed/broken; never mixed with the LUT.
     */
    private static boolean slowVanilla(Entity self, TagKey<Fluid> tag, double speed) {
        C_SLOW++;
        if (self.touchingUnloadedChunk()) {
            return false;
        }
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushFlag = self.isPushedByFluid();

        int cx0 = minX >> 4, cx1 = maxX >> 4, cz0 = minZ >> 4, cz1 = maxZ >> 4;
        int spanX = cx1 - cx0 + 1;
        int offset = -(cx0 + spanX * cz0);
        LevelChunkSection[][] flat = new LevelChunkSection[spanX * (cz1 - cz0 + 1)][];
        ChunkSource source = level.getChunkSource();
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                // load=false: touchingUnloadedChunk() has guaranteed presence.
                flat[cx + spanX * cz + offset] = source.getChunk(cx, cz, ChunkStatus.FULL, false).getSections();
            }
        }

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0;
        boolean inFluid = false;
        int flowCount = 0;
        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) + spanX * (z >> 4) + offset][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    mpos.set(x, y, z);
                    if (tag == FluidTags.LAVA) {
                        self.lastLavaContact = mpos.immutable();
                    }
                    double d0 = (double) ((float) y + fs.getHeight(level, mpos)) - box.minY;
                    if (d0 < 0.0) {
                        continue;
                    }
                    inFluid = true;
                    maxDepth = Math.max(maxDepth, d0);
                    if (!pushFlag) {
                        continue;
                    }
                    flowCount++;
                    Vec3 flow = fs.getFlow(level, mpos);
                    if (maxDepth < 0.4) {
                        flowAcc = flowAcc.add(flow.scale(maxDepth));
                    } else {
                        flowAcc = flowAcc.add(flow);
                    }
                }
            }
        }
        self.fluidHeight.put(tag, maxDepth);
        if (flowAcc == Vec3.ZERO) {
            return inFluid;
        }
        flowAcc = flowAcc.scale(1.0 / flowCount);
        Vec3 delta = self.getDeltaMovement();
        if (!(self instanceof net.minecraft.world.entity.player.Player)) {
            flowAcc = flowAcc.normalize();
        }
        flowAcc = flowAcc.scale(speed);
        if (Math.abs(delta.x) < 0.003 && Math.abs(delta.z) < 0.003 && flowAcc.length() < 0.0045) {
            flowAcc = flowAcc.normalize().scale(0.0045);
        }
        self.setDeltaMovement(delta.add(flowAcc));
        return true;
    }

    // ---------------- the RECON-43 write-bump retarget site ----------------

    /**
     * Delegate of the single LevelChunk.setBlockState ->
     * LevelChunkSection.setBlockState(IIILBlockState)BlockState call site
     * (retarget contract identical to the RECON-43 ledger: returns the OLD
     * state; a real fluid-state change (FluidState singleton ref-compare)
     * bumps the global GEN — the LUT invalidation edge. Plain block writes
     * never touch GEN, so static worlds keep a perfect hit rate.
     */
    public static BlockState secWrite(LevelChunkSection section, int x, int y, int z, BlockState newState) {
        BlockState old = section.setBlockState(x, y, z, newState);
        if (old.getFluidState() != newState.getFluidState()) {
            GEN++;
        }
        return old;
    }

    /** Reflective self-test (driven from src/fluid_bulk.rs after arming):
     * probe/pack/stamp/presence algebra sanity inside the live JVM. */
    public static boolean selfTest() {
        Plane p = new Plane();
        long k = pack(123456, 64, -654321);
        if (k == pack(123457, 64, -654321) || k == pack(123456, 65, -654321)) return false;
        int s = (int) mix(k) & LUT_MASK;
        if (s < 0 || s >= p.key.length) return false;
        p.key[s] = k;
        if (p.val[s] != null) return false;
        // stamp algebra: different tick or different GEN must differ; the
        // packed stamp of (tick,gen) must be stable and non-degenerate.
        long s1 = stamp(100, 7);
        long s2 = stamp(101, 7);
        long s3 = stamp(100, 8);
        if (s1 == s2 || s1 == s3 || s2 == s3) return false;
        if (stamp(100, 7) != s1) return false;
        p.tag[s] = s1;
        if (p.tag[s] != s1) return false;
        p.pkey[s & PRES_MASK] = mix(pack(3, 0, 4)) | Long.MIN_VALUE;
        if (p.pkey[s & PRES_MASK] >= 0) return false;
        // GEN visibility + census array shape
        long g0 = GEN;
        GEN = g0 + 1;
        boolean ok = GEN == g0 + 1;
        GEN = g0;
        return ok;
    }
}
