package net.minecraft.world.entity.item;

import it.unimi.dsi.fastutil.longs.Long2DoubleOpenHashMap;
import it.unimi.dsi.fastutil.longs.LongOpenHashSet;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Predicate;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * ItemsComposeOps (MEGA-ROUND-2 / TASK-397-A, lever_flag "items_compose_ai").
 *
 * COMPOSITION of the two round-1 ARMED item levers on ONE byte pipeline:
 *   - vector A (items_index, ROUND-396-A): a 1.0-grid hashed index of
 *     ItemEntity merge-candidates replaces the per-scan
 *     O(section-population) broadphase dump
 *     Level.getEntitiesOfClass(ItemEntity.class, AABB.inflate(merge,...), pred)
 *     — the index answers WHERE the neighbours are (bucket superset +
 *     EXACT vanilla filter chain: box.intersects(getBoundingBox()) + the
 *     very vanilla Predicate instance).
 *   - vector I (items_wakeup, ROUND-396-I): event-driven wakeup scheduling
 *     replaces the per-item rescan cadence — a stationary item scans ONLY
 *     on an event: E1 first sighting, E2 displacement > 0.25 blocks since
 *     the last scan, E3 one-shot wake, E4 wake of neighbours after a scan
 *     that ACTUALLY merged (plus the unconditional post-teleport scan).
 *
 * COMPOSITION CONTRACTS (beyond each lever standalone):
 *   C1 The wakeup gate arms the index: no wakeup event -> the vanilla scan
 *      body never runs -> getMergeCandidates is never invoked -> neither a
 *      scan NOR an index query happens. The index itself stays
 *      position-exact for OTHER queries: reconcile() is driven by the
 *      item's own ticked move (O(1) map ops), which every live item runs
 *      every tick (ItemEntity.tick: move @269 BEFORE merge @471).
 *   C2 Merge candidates come ONLY from grid buckets: the retargeted
 *      getEntitiesOfClass site inside the vanilla scan body enumerates
 *      bucket supersets and applies the exact vanilla filter; there is NO
 *      broadphase dump left on the item merge path.
 *   C3 E4 neighbour wake reads the SAME grid buckets (superset box
 *      enumeration + exact AABB filter) — the wake path keeps zero
 *      broadphase dependence (wakeup lever used level.getEntitiesOfClass
 *      here; composed lever does not).
 *
 * Parity: the vanilla scan body is never re-implemented (reflection
 * delegate resolved at static init, selfTest() gate before retransform);
 * merge/despawn/pickup semantics untouched. Documented deviations are the
 * union of the two donor levers' DOC-DEV: candidate iteration order may
 * differ (A, same class of nondeterminism accepted upstream when
 * Paper/Moonrise replaced broadphase ordering) and merge TIMING may differ
 * by one event latency (I) — hard invariants hold: units conserved, zero
 * fabricated merges, entitiesLeft within the cap-64 bound.
 *
 * Fail-closed: any static-init/selfTest failure keeps ItemEntity vanilla
 * (rust refuses to retransform); any post-retransform delegate failure
 * degrades to a skipped scan (log-once) — a skipped scan can only DELAY a
 * merge to the next event, never fabricate one, never crash.
 *
 * Thread safety: region_threads=4 workers tick disjoint spatial regions;
 * bucket mutation serialized per-bucket (synchronized ArrayList), the two
 * index maps are ConcurrentHashMaps, wake bookkeeping serialized on one
 * monitor, the vanilla scan itself runs outside the locks.
 *
 * INJECTS-ONLY: defined into the KERNEL loader at runtime by
 * src/items_compose.rs (env CRUSSTY_LEVER_FLAG=items_compose_ai). Empty or
 * different flag = never defined, bytes never served = exact vanilla.
 */
public final class ItemsComposeOps {

    private ItemsComposeOps() {}

    // ---------- lever gate (belt-and-braces; rust gates the same env) ----------

    public static final String LEVER_FLAG = "items_compose_ai";

    static final boolean ARMED =
            LEVER_FLAG.equals(System.getenv("CRUSSTY_LEVER_FLAG"));

    // ---------- wakeup bookkeeping (vector I) ----------

    /** Task-spec event threshold: E2 total movement > 0.25 blocks. */
    static final double MOVE_EPS2 = 0.25 * 0.25;

    private static final Object LOCK = new Object();
    private static final Long2DoubleOpenHashMap LAST_X = new Long2DoubleOpenHashMap();
    private static final Long2DoubleOpenHashMap LAST_Y = new Long2DoubleOpenHashMap();
    private static final Long2DoubleOpenHashMap LAST_Z = new Long2DoubleOpenHashMap();
    private static final LongOpenHashSet ACTIVE = new LongOpenHashSet();

    // ---------- index bookkeeping (vector A) ----------

    /** Grid bucket key -> live (or stale-not-yet-purged) item list. */
    static final ConcurrentHashMap<Long, ArrayList<ItemEntity>> BUCKETS =
            new ConcurrentHashMap<>();

    /** ItemEntity -> current bucket key (single-element long box). */
    static final ConcurrentHashMap<ItemEntity, long[]> KEY_BY_ENTITY =
            new ConcurrentHashMap<>();

    /** Direct-mapped Long box cache — steady-state map ops stay alloc-free. */
    private static final Long[] BOX_CACHE = new Long[1024];

    private static final java.util.concurrent.atomic.AtomicLong RECONCILED =
            new java.util.concurrent.atomic.AtomicLong();

    // ---------- vanilla scan delegate ----------

    private static volatile Method VANILLA_MERGE;
    private static volatile String INIT_ERR;
    private static volatile boolean LOGGED_DELEGATE_FAIL;

    static {
        try {
            Method m = ItemEntity.class.getDeclaredMethod("mergeWithNeighbours");
            m.setAccessible(true);
            VANILLA_MERGE = m;
            System.err.println("[crussty-items-compose] static-init OK (delegate resolved)");
        } catch (Throwable t) {
            INIT_ERR = t.toString();
            System.err.println("[crussty-items-compose] static-init FAIL: " + INIT_ERR);
        }
    }

    /**
     * Pre-retransform contract (called by src/items_compose.rs through JNI):
     * "OK" only when the reflection delegate and the index structures are
     * usable. Any other value keeps ItemEntity vanilla (fail-closed).
     */
    public static String selfTest() {
        if (VANILLA_MERGE == null) {
            return "FAIL:" + INIT_ERR;
        }
        try {
            // touch both structures once (shapes + fastutil presence)
            BUCKETS.putIfAbsent(Long.valueOf(0x7FFFFFFF7FFFFFFFL), new ArrayList<>(0));
            KEY_BY_ENTITY.size();
            return "OK";
        } catch (Throwable t) {
            return "FAIL:" + t;
        }
    }

    // ------------------------------------------------------------------
    // Retarget 1: merge-candidate query (the getEntitiesOfClass site inside
    // ItemEntity.mergeWithNeighbours()V — mergeWithNeighbours:67 in the
    // baseline kernel). Called from the vanilla scan body, i.e. ONLY after
    // the wakeup gate let a scan through (contract C1).
    // ------------------------------------------------------------------

    public static List<ItemEntity> getMergeCandidates(
            Level level,
            Class<ItemEntity> cls,
            AABB box,
            Predicate<? super ItemEntity> predicate) {
        if (!ARMED) {
            // Exact vanilla path (lever not armed): same virtual call.
            return level.getEntitiesOfClass(cls, box, predicate);
        }
        ArrayList<ItemEntity> out = new ArrayList<>(4);
        // Superset enumeration: candidates are keyed by their reference
        // position, which may sit up to (half-extent) OUTSIDE the query box
        // while their own AABB still intersects it (ItemEntity width 0.25 ->
        // half-extent 0.125). Expand by 0.5 (>= any ItemEntity half-extent);
        // the exact vanilla filter in collect() keeps the candidate set
        // identical to level.getEntitiesOfClass.
        double pad = 0.5D;
        int x0 = Mth.floor(box.minX - pad);
        int x1 = Mth.floor(box.maxX + pad);
        int y0 = Mth.floor(box.minY - pad);
        int y1 = Mth.floor(box.maxY + pad);
        int z0 = Mth.floor(box.minZ - pad);
        int z1 = Mth.floor(box.maxZ + pad);
        for (int x = x0; x <= x1; x++) {
            for (int z = z0; z <= z1; z++) {
                for (int y = y0; y <= y1; y++) {
                    ArrayList<ItemEntity> bucket = BUCKETS.get(boxed(pack(x, y, z)));
                    if (bucket != null) {
                        collect(bucket, box, predicate, out);
                    }
                }
            }
        }
        return out;
    }

    /** Exact vanilla filter chain on a bucket's members; purges dead entries. */
    private static void collect(
            ArrayList<ItemEntity> bucket,
            AABB box,
            Predicate<? super ItemEntity> predicate,
            ArrayList<ItemEntity> out) {
        ArrayList<ItemEntity> dead = null;
        synchronized (bucket) {
            for (int i = 0; i < bucket.size(); i++) {
                ItemEntity e = bucket.get(i);
                if (e.isRemoved()) {
                    if (dead == null) {
                        dead = new ArrayList<>(2);
                    }
                    dead.add(e);
                    continue;
                }
                if (box.intersects(e.getBoundingBox()) && predicate.test(e)) {
                    out.add(e);
                }
            }
            if (dead != null) {
                bucket.removeAll(dead);
                for (int i = 0; i < dead.size(); i++) {
                    KEY_BY_ENTITY.remove(dead.get(i));
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Retarget 2: post-move reconcile (the Entity.move site inside
    // ItemEntity.tick()V — tick:269 in the baseline kernel). tick runs move
    // BEFORE merge, so the querying item is index-exact at its own scan.
    // ------------------------------------------------------------------

    public static void moveIndexed(ItemEntity entity, MoverType type, Vec3 movement) {
        entity.move(type, movement);
        if (ARMED) {
            reconcile(entity);
        }
    }

    static void reconcile(ItemEntity e) {
        if (e.isRemoved()) {
            unregister(e);
            return;
        }
        if (RECONCILED.incrementAndGet() == 1L) {
            System.out.println(
                    "[ItemsComposeOps] armed; wakeup+index live (first reconcile)");
        }
        long k = key(e.getX(), e.getY(), e.getZ());
        long[] slot = KEY_BY_ENTITY.get(e);
        if (slot == null) {
            long[] fresh = new long[] {k};
            long[] prev = KEY_BY_ENTITY.putIfAbsent(e, fresh);
            if (prev == null) {
                bucketOf(k).add(e);
                return;
            }
            slot = prev;
        }
        if (slot[0] != k) {
            ArrayList<ItemEntity> old = BUCKETS.get(boxed(slot[0]));
            if (old != null) {
                synchronized (old) {
                    old.remove(e);
                }
            }
            slot[0] = k;
            bucketOf(k).add(e);
        }
    }

    static void unregister(ItemEntity e) {
        long[] slot = KEY_BY_ENTITY.remove(e);
        if (slot != null) {
            ArrayList<ItemEntity> old = BUCKETS.get(boxed(slot[0]));
            if (old != null) {
                synchronized (old) {
                    old.remove(e);
                }
            }
        }
    }

    private static ArrayList<ItemEntity> bucketOf(long k) {
        Long kk = boxed(k);
        ArrayList<ItemEntity> b = BUCKETS.get(kk);
        if (b == null) {
            b = BUCKETS.computeIfAbsent(kk, key -> new ArrayList<>(2));
        }
        return b;
    }

    // ------------------------------------------------------------------
    // Retargets 3+4: the two mergeWithNeighbours call sites (tick:471 and
    // teleport:29 in the baseline kernel), gated by the wakeup events.
    // ------------------------------------------------------------------

    /**
     * Retarget of the tick()-scheduled scan site: E1/E2/E3 gate, then the
     * REAL vanilla body (which queries the index through retarget 1).
     */
    public static void mergeWithNeighbours(ItemEntity self) {
        try {
            long id = self.getId();
            if (scanNeeded(self, id)) {
                runVanilla(self, id);
            }
        } catch (Throwable t) {
            // paranoia net: a skipped scan can only delay a merge to the
            // next event — never fabricate one, never crash.
            logDelegateFail(t);
        }
    }

    /**
     * Retarget of the teleport() call site: vanilla semantics = unconditional
     * scan after teleport — kept unconditional (teleports are rare).
     */
    public static void mergeAfterTeleport(ItemEntity self) {
        try {
            runVanilla(self, self.getId());
        } catch (Throwable t) {
            logDelegateFail(t);
        }
    }

    // ---------- wakeup gate + scan executor (vector I logic) ----------

    /** E1/E2/E3 gate for the tick() call site. True = run the vanilla scan. */
    private static boolean scanNeeded(ItemEntity self, long id) {
        synchronized (LOCK) {
            if (ACTIVE.contains(id)) {
                return true; // E3 one-shot wake (consumed by stamp())
            }
            if (!LAST_X.containsKey(id)) {
                return true; // E1 first sighting: spawn / chunk load-in
            }
            double dx = self.getX() - LAST_X.get(id);
            double dy = self.getY() - LAST_Y.get(id);
            double dz = self.getZ() - LAST_Z.get(id);
            return dx * dx + dy * dy + dz * dz > MOVE_EPS2; // E2
        }
    }

    private static void runVanilla(ItemEntity self, long id) {
        Method m = VANILLA_MERGE;
        if (m == null) {
            // Without a delegate we cannot reach the vanilla body — skip
            // loudly (selfTest() should have blocked the retransform).
            if (!LOGGED_DELEGATE_FAIL) {
                LOGGED_DELEGATE_FAIL = true;
                System.err.println("[crussty-items-compose] delegate missing at call time — scans skipped (fail-safe)");
            }
            return;
        }
        double px = self.getX();
        double py = self.getY();
        double pz = self.getZ();
        ItemStack stack = self.getItem();
        int pc = stack.getCount();
        stamp(id, self.getX(), self.getY(), self.getZ()); // refresh stamp + consume ACTIVE_ONCE
        try {
            m.invoke(self);
        } catch (java.lang.reflect.InvocationTargetException ite) {
            Throwable cause = ite.getCause();
            if (cause instanceof RuntimeException) {
                throw (RuntimeException) cause;   // preserve vanilla crash semantics
            }
            if (cause instanceof Error) {
                throw (Error) cause;
            }
            logDelegateFail(ite);
            return;
        } catch (Throwable t) {
            logDelegateFail(t);
            return;
        }
        // E4 neighbour wake: only when the scan actually merged something
        // (self was the discarded donor OR self's stack changed in place).
        if (self.isRemoved() || self.getItem().getCount() != pc) {
            wakeZone(px, py, pz, id);
        }
    }

    private static void stamp(long id, double x, double y, double z) {
        synchronized (LOCK) {
            LAST_X.put(id, x);
            LAST_Y.put(id, y);
            LAST_Z.put(id, z);
            ACTIVE.remove(id);
        }
    }

    static void wake(long id) {
        synchronized (LOCK) {
            ACTIVE.add(id);
        }
    }

    private static void logDelegateFail(Throwable t) {
        if (!LOGGED_DELEGATE_FAIL) {
            LOGGED_DELEGATE_FAIL = true;
            System.err.println("[crussty-items-compose] delegate invoke failed: " + t);
        }
    }

    // ------------------------------------------------------------------
    // E4 via the grid (contract C3): wake every live item whose AABB
    // intersects the itemMerge-radius box around the merge position, from
    // bucket enumeration ONLY — no broadphase dump. Superset acceptable:
    // a wake is scheduling, not a merge result. Best-effort: a failure here
    // can only delay a merge to the next movement event.
    // ------------------------------------------------------------------

    private static void wakeZone(double px, double py, double pz, long excludeId) {
        try {
            double r = 0.5D; // itemMerge radius; grid pad adds the half-extent slack
            double pad = r + 0.5D;
            int x0 = Mth.floor(px - pad);
            int x1 = Mth.floor(px + pad);
            int y0 = Mth.floor(py - pad);
            int y1 = Mth.floor(py + pad);
            int z0 = Mth.floor(pz - pad);
            int z1 = Mth.floor(pz + pad);
            AABB box = new AABB(px - r, py - r, pz - r, px + r, py + r, pz + r);
            for (int x = x0; x <= x1; x++) {
                for (int z = z0; z <= z1; z++) {
                    for (int y = y0; y <= y1; y++) {
                        ArrayList<ItemEntity> bucket = BUCKETS.get(boxed(pack(x, y, z)));
                        if (bucket == null) {
                            continue;
                        }
                        synchronized (bucket) {
                            for (int i = 0; i < bucket.size(); i++) {
                                ItemEntity e = bucket.get(i);
                                if (!e.isRemoved()
                                        && e.getId() != excludeId
                                        && box.intersects(e.getBoundingBox())) {
                                    wake(e.getId());
                                }
                            }
                        }
                    }
                }
            }
        } catch (Throwable t) {
            logDelegateFail(t);
        }
    }

    // ------------------------------------------------------------------
    // Grid keying: 1.0-block cells, SectionPos-style packing
    // (x:26b << 38 | z:26b << 12 | y:12b).
    // ------------------------------------------------------------------

    static long key(double x, double y, double z) {
        return pack(Mth.floor(x), Mth.floor(y), Mth.floor(z));
    }

    static long pack(int x, int y, int z) {
        return ((long) (x & 0x3FFFFFF) << 38) | ((long) (z & 0x3FFFFFF) << 12)
                | (long) (y & 0xFFF);
    }

    private static Long boxed(long k) {
        int slot = (int) ((k * 0x9E3779B97F4A7C15L) >>> 54) & 0x3FF;
        Long cached = BOX_CACHE[slot];
        if (cached != null && cached.longValue() == k) {
            return cached;
        }
        Long fresh = Long.valueOf(k);
        BOX_CACHE[slot] = fresh;
        return fresh;
    }
}
