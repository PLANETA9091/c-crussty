package net.minecraft.world.entity.item;

import java.lang.reflect.Field;
import java.util.Arrays;
import java.util.Objects;
import java.util.concurrent.atomic.AtomicInteger;

import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * ItemSoaOps (MEGA-ROUND-2, TASK-397-C — ARCH-LEVER items_soa: SoA flat
 * primitive-array mirror of the hot ItemEntity fields behind the merge scan).
 *
 * TOP-1 bottleneck (run 35528326290, bank v4, fp=4, 150k scene): items/
 * ItemEntity.tick 31.17% java + item-driven broadphase 15.66% (the per-item
 * per-tick Level.getEntitiesOfClass query inside mergeWithNeighbours) +
 * fastutil 8.54% (entity-section machinery behind that query) + java_util
 * 7.01% (List/iterator garbage of that query). Every one of the 150k items
 * runs mergeWithNeighbours EVERY tick: the vanilla query walks the chunk
 * entity-section broadphase (pointer-chasing object graph) and allocates a
 * List + iterator, only to exact-check a handful of candidates.
 *
 * THIS LEVER replaces the per-item per-tick BROADPHASE QUERY with a flat
 * struct-of-arrays uniform grid: int[]-linked cell buckets over double[] x/y/z
 * slot arrays. Only candidate ENUMERATION is SoA; every decision on a
 * candidate is the EXACT vanilla path executed on the LIVE objects (isMergable
 * semantics, wall-clip fix, tryToMerge/merge bytecodes verbatim, CraftEvent
 * merge event). Parity-by-construction for everything the scan yields; the
 * enumeration staleness window (<= BUILD_INTERVAL wall ms of cell membership)
 * is the same cross-entity read-interleaving parity class the owner bar
 * already accepts for region-threaded ticking (S7-155 documented boundary).
 *
 * Mechanism (rust side item_soa.rs): the single private instance method
 * ItemEntity.mergeWithNeighbours()V body is redirected
 * (redirect_method_body_to_static, receiver prepended) to
 * ItemSoaOps.mergeWithNeighbours(ItemEntity). Flag off -> body untouched ->
 * byte-exact vanilla.
 *
 * Single-writer-per-slot discipline: slot i is written only by the tick of
 * the entity owning it (syncSelf on every mergeWithNeighbours call) and by
 * the single grid builder (prune/free). Readers race benignly: a candidate is
 * always re-validated on the live object, so stale/pruned/torn entries can
 * only degrade enumeration quality, never semantics. The one hazard of racing
 * plain-array chain mutation (loop in a chain mid-rebuild, straggler readers
 * of a buffer being rebuilt) is bounded by the per-cell hop budget.
 *
 * Fail-open to EXACT vanilla: any anomaly (id table saturated, slot pool
 * exhausted) falls back to the vanilla broadphase algorithm inlined below —
 * semantics preserved, only speed lost.
 *
 * Dormant-invisible: the bridge class is only DEFINED when
 * CRUSSTY_LEVER_FLAG=items_soa && CRUSSTY_LEVER_ARG=1; with the flag off the
 * redirect never happens and this class is never loaded.
 */
public final class ItemSoaOps {

    private ItemSoaOps() {}

    // ============================ SoA mirror ============================

    /** Slot pool ceiling. 150k population + churn fits with 3.5x headroom. */
    private static final int MAX_SLOTS = 1 << 19; // 524288

    private static final double[] PX = new double[MAX_SLOTS];
    private static final double[] PY = new double[MAX_SLOTS];
    private static final double[] PZ = new double[MAX_SLOTS];
    private static final ItemEntity[] REF = new ItemEntity[MAX_SLOTS];

    private static final AtomicInteger SLOT_COUNT = new AtomicInteger(0);
    private static final int[] FREE_LIST = new int[4096];
    private static final AtomicInteger FREE_TOP = new AtomicInteger(0);

    // entity id -> slot, open addressing; ids are per-level monotonic ints.
    // Saturation is fail-open (vanilla fallback), never a correctness hazard.
    private static final int[] ID_KEY = new int[1 << 21];
    private static final int[] ID_VAL = new int[1 << 21];
    private static final int ID_MASK = (1 << 21) - 1;
    private static final int ID_CAP_SOFT = (1 << 21) / 4; // conservative drift-aware soft cap
    private static final AtomicInteger ID_USED = new AtomicInteger(0);

    // ============================ SoA grid ============================

    private static final int CELL = 1; // units per cell (merge box ~1.25 wide at default itemMerge=0.5)
    private static final int CELL_BUCKETS = 1 << 18; // hash-only (colliding cells just share a bucket)

    // Double buffer: readers always walk the ACTIVE grid (immutable between
    // flips); the single builder fills the INACTIVE one and flips.
    private static final int[][] G_HEAD = {new int[CELL_BUCKETS], new int[CELL_BUCKETS]};
    private static final int[][] G_NEXT = {new int[MAX_SLOTS], new int[MAX_SLOTS]};

    private static volatile int G_ACTIVE = 0;
    private static volatile long G_EPOCH_NANOS = Long.MIN_VALUE / 2; // forces first build on first scan
    private static final AtomicInteger BUILD_LOCK = new AtomicInteger(0);
    private static final long BUILD_INTERVAL_NANOS = 100_000_000L; // 100ms wall (~0.3 tick at 2.6 TPS)
    private static final int HOP_BUDGET = 256; // straggler-reader loop bound

    // telemetry (printed by the rust side on the periodic report)
    static volatile long liveSlots = 0;
    static volatile long scansSoa = 0;
    static volatile long scansVanillaFallback = 0;

    // ============================ field access ============================

    // Only `despawnRate` is private on ItemEntity (age/pickupDelay/target are
    // public) — resolved once via Unsafe.objectFieldOffset (repo pattern).
    private static final sun.misc.Unsafe UNSAFE;
    private static final long OFF_DESPAWN_RATE;

    static {
        sun.misc.Unsafe u = null;
        long off = -1L;
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            u = (sun.misc.Unsafe) uf.get(null);
            Field f = ItemEntity.class.getDeclaredField("despawnRate");
            f.setAccessible(true);
            off = u.objectFieldOffset(f);
        } catch (Throwable t) {
            u = null;
            off = -1L;
        }
        UNSAFE = u;
        OFF_DESPAWN_RATE = off;
    }

    private static boolean unsafeOk() {
        return UNSAFE != null && OFF_DESPAWN_RATE != -1L;
    }

    // ============================ entry (redirect target) ============================

    /**
     * The redirected body of ItemEntity.mergeWithNeighbours()V (receiver
     * prepended). Byte-for-byte vanilla semantics for every decision; only
     * the candidate enumeration is SoA (or, on fail-open, the vanilla
     * broadphase re-inlined).
     */
    public static void mergeWithNeighbours(ItemEntity self) {
        if (!isMergableImpl(self)) {
            return; // vanilla gate, exact
        }
        Level lvl = self.level();
        double r = lvl.spigotConfig.itemMerge;
        // vanilla: inflate(r, onlyMergeItemsHorizontally ? 0.0 : r - 0.5, r)
        double ry = lvl.paperConfig().entities.behavior.onlyMergeItemsHorizontally ? 0.0D : (r - 0.5D);
        AABB box = self.getBoundingBox().inflate(r, ry, r); // EXACT vanilla query box (live fields)

        if (unsafeOk() && syncSelf(self) >= 0) {
            scansSoa++;
            scanGrid(self, box);
        } else {
            // fail-open: exact vanilla enumeration (broadphase + vanilla checks)
            scansVanillaFallback++;
            scanVanilla(self, lvl, box);
        }
    }

    // ============================ vanilla-exact semantics ============================

    /**
     * Byte-exact re-implementation of the private ItemEntity.isMergable():
     * getItem; isAlive; pickupDelay != 32767; age != -32768; age < despawnRate;
     * count < maxStackSize.
     */
    private static boolean isMergableImpl(ItemEntity e) {
        net.minecraft.world.item.ItemStack s = e.getItem();
        if (!e.isAlive()) return false;
        if (e.pickupDelay == 32767) return false;
        int age = e.age;
        if (age == -32768) return false;
        if (unsafeOk()) {
            if (age >= UNSAFE.getInt(e, OFF_DESPAWN_RATE)) return false;
        } else {
            // no Unsafe: conservative LIFETIME gate (vanilla despawnRate default
            // is 6000; Purpur sets the field per world config — fail-open path
            // below uses the exact vanilla query anyway)
            if (age >= 6000) return false;
        }
        return s.getCount() < s.getMaxStackSize();
    }

    /** Byte-exact re-implementation of the private ItemEntity.tryToMerge(ItemEntity). */
    private static void tryToMergeExact(ItemEntity self, ItemEntity other) {
        net.minecraft.world.item.ItemStack mine = self.getItem();
        net.minecraft.world.item.ItemStack theirs = other.getItem();
        if (Objects.equals(self.target, other.target)
                && ItemEntity.areMergable(mine, theirs)) {
            if (theirs.getCount() < mine.getCount()) {
                mergeExact(self, mine, other, theirs);
            } else {
                mergeExact(other, theirs, self, mine);
            }
        }
    }

    /**
     * Byte-exact re-implementation of the private static
     * merge(ItemEntity,ItemStack,ItemEntity,ItemStack) — including the
     * CraftEventFactory merge event, the 64/merge move limit, pickupDelay max,
     * age min and the MERGE-cause discard of the emptied donor.
     */
    private static void mergeExact(ItemEntity a, net.minecraft.world.item.ItemStack aStack,
                                   ItemEntity b, net.minecraft.world.item.ItemStack bStack) {
        if (!org.bukkit.craftbukkit.event.CraftEventFactory.callItemMergeEvent(a, b)) {
            return;
        }
        net.minecraft.world.item.ItemStack merged = ItemEntity.merge(aStack, bStack, 64);
        a.setItem(merged);
        a.pickupDelay = Math.max(a.pickupDelay, b.pickupDelay);
        a.age = Math.min(a.age, b.age);
        if (bStack.isEmpty()) {
            b.discard(org.bukkit.event.entity.EntityRemoveEvent.Cause.MERGE);
        }
    }

    /**
     * Fail-open path: the vanilla mergeWithNeighbours body re-inlined with the
     * exact vanilla broadphase query (getEntitiesOfClass) and the exact checks.
     */
    private static void scanVanilla(ItemEntity self, Level lvl, AABB box) {
        java.util.List<ItemEntity> list = lvl.getEntitiesOfClass(ItemEntity.class, box,
                (ItemEntity it) -> it != self && isMergableImpl(it));
        boolean wallFix = lvl.paperConfig().fixes.fixItemsMergingThroughWalls;
        for (int i = 0, n = list.size(); i < n; i++) {
            ItemEntity other = list.get(i);
            if (!isMergableImpl(other)) {
                continue;
            }
            if (wallFix && lvl.clipDirect(self.position(), other.position(),
                    net.minecraft.world.phys.shapes.CollisionContext.of(self))
                    == net.minecraft.world.phys.HitResult.Type.BLOCK) {
                continue;
            }
            tryToMergeExact(self, other);
            if (self.isRemoved()) {
                return;
            }
        }
    }

    // ============================ SoA scan ============================

    private static void scanGrid(ItemEntity self, AABB box) {
        int active = G_ACTIVE;
        int[] head = G_HEAD[active];
        int[] next = G_NEXT[active];
        int x0 = floorCell(box.minX), x1 = floorCell(box.maxX);
        int y0 = floorCell(box.minY), y1 = floorCell(box.maxY);
        int z0 = floorCell(box.minZ), z1 = floorCell(box.maxZ);
        Level lvl = self.level();
        boolean wallFix = lvl.paperConfig().fixes.fixItemsMergingThroughWalls;
        net.minecraft.world.phys.shapes.CollisionContext ctx = wallFix
                ? net.minecraft.world.phys.shapes.CollisionContext.of(self) : null;
        Vec3 selfPos = wallFix ? self.position() : null;
        for (int cx = x0; cx <= x1; cx++) {
            for (int cy = y0; cy <= y1; cy++) {
                for (int cz = z0; cz <= z1; cz++) {
                    int b = bucket(cx, cy, cz);
                    int s = head[b];
                    int hops = 0;
                    while (s >= 0 && hops++ < HOP_BUDGET) {
                        int ns = next[s]; // read before descending (slot ref stable per slot)
                        ItemEntity other = REF[s];
                        if (other != null && other != self && !other.isRemoved() && isMergableImpl(other)) {
                            if (wallFix && lvl.clipDirect(selfPos, other.position(), ctx)
                                    == net.minecraft.world.phys.HitResult.Type.BLOCK) {
                                s = ns;
                                continue;
                            }
                            tryToMergeExact(self, other);
                            if (self.isRemoved()) {
                                return;
                            }
                        }
                        s = ns;
                    }
                }
            }
        }
        maybeBuild();
    }

    private static int floorCell(double v) {
        return (int) Math.floor(v);
    }

    private static int bucket(double x, double y, double z) {
        return bucket(floorCell(x), floorCell(y), floorCell(z));
    }

    private static int bucket(int cx, int cy, int cz) {
        long h = cx * 0x9E3779B97F4A7C15L ^ cy * 0xC2B2AE3D27D4EB4FL ^ cz * 0x165667B19E3779F9L;
        h ^= (h >>> 33);
        return (int) (h & (CELL_BUCKETS - 1));
    }

    // ============================ slot sync ============================

    /**
     * Freshens the caller's own slot from its LIVE position (single writer:
     * only the owning entity's tick calls this for its slot). Returns the slot
     * or -1 (any anomaly -> caller uses the vanilla-fallback enumeration).
     */
    private static int syncSelf(ItemEntity self) {
        int id = self.getId();
        int slot = findSlot(id);
        if (slot < 0) {
            slot = allocSlot(id);
            if (slot < 0) {
                return -1;
            }
        }
        Vec3 pos = self.position();
        PX[slot] = pos.x;
        PY[slot] = pos.y;
        PZ[slot] = pos.z;
        REF[slot] = self;
        return slot;
    }

    private static int findSlot(int id) {
        int i = mix(id) & ID_MASK;
        int k;
        while ((k = ID_KEY[i]) != 0) {
            if (k == id) {
                return ID_VAL[i];
            }
            i = (i + 1) & ID_MASK;
        }
        return -1;
    }

    private static int allocSlot(int id) {
        // free-list pop (CAS: no double-pop -> no shared-slot hazards)
        while (true) {
            int t = FREE_TOP.get();
            if (t == 0) break;
            int s = FREE_LIST[t - 1];
            if (FREE_TOP.compareAndSet(t, t - 1)) {
                return install(id, s);
            }
        }
        while (true) {
            int c = SLOT_COUNT.get();
            if (c >= MAX_SLOTS) {
                return -1;
            }
            if (SLOT_COUNT.compareAndSet(c, c + 1)) {
                return install(id, c);
            }
        }
    }

    private static int install(int id, int slot) {
        if (id == 0) {
            return -1; // 0 is the empty sentinel
        }
        int i = mix(id) & ID_MASK;
        while (true) {
            int k = ID_KEY[i];
            if (k == 0) {
                if (ID_USED.get() >= ID_CAP_SOFT) {
                    return -1; // saturated -> fail-open vanilla for this entity
                }
                ID_KEY[i] = id; // races lose benignly; the lost owner re-inserts on its next tick
                ID_VAL[i] = slot;
                ID_USED.incrementAndGet();
                return slot;
            }
            if (k == id) {
                ID_VAL[i] = slot;
                return slot;
            }
            i = (i + 1) & ID_MASK;
        }
    }

    private static int mix(int x) {
        int h = x * 0x9E3779B9;
        h ^= (h >>> 16);
        h *= 0x85EBCA6B;
        h ^= (h >>> 13);
        return h;
    }

    // ============================ grid build ============================

    private static void maybeBuild() {
        long now = System.nanoTime();
        if (now - G_EPOCH_NANOS < BUILD_INTERVAL_NANOS) {
            return;
        }
        if (!BUILD_LOCK.compareAndSet(0, 1)) {
            return; // another worker builds; keep scanning the active grid
        }
        try {
            long t0 = System.nanoTime();
            int cur = G_ACTIVE;
            int nxt = 1 - cur;
            int[] head = G_HEAD[nxt];
            int[] next = G_NEXT[nxt];
            Arrays.fill(head, -1);
            int n = SLOT_COUNT.get();
            int live = 0;
            int top = 0; // local free-list builder (single writer: this builder)
            for (int s = 0; s < n; s++) {
                ItemEntity e = REF[s];
                if (e == null) {
                    if (top < FREE_LIST.length) FREE_LIST[top++] = s;
                    continue;
                }
                if (e.isRemoved()) {
                    REF[s] = null;
                    if (top < FREE_LIST.length) FREE_LIST[top++] = s;
                    continue;
                }
                int b = bucket(PX[s], PY[s], PZ[s]);
                next[s] = head[b];
                head[b] = s;
                live++;
            }
            // publish free list before flipping visibility of pruned slots
            FREE_TOP.set(top);
            liveSlots = live;
            G_EPOCH_NANOS = t0;
            G_ACTIVE = nxt; // volatile flip: new scans use the fresh grid
        } finally {
            BUILD_LOCK.set(0);
        }
    }
}
