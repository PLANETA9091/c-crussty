package net.minecraft.world.entity.item;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Predicate;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * ItemMergeIndexOps (ROUND-396 / TASK-396-A, vector A — ITEMS-INDEX,
 * lever_flag="items_index").
 *
 * Architecture swap for the TOP-1 bottleneck (items 31.17% java + item-driven
 * broadphase 15.66% on bank v4 baseline, 115655 samples): a 1.0-grid hashed
 * index of ItemEntity buckets that replaces the per-item-per-tick
 * O(section-population) broadphase scan
 *   Level.getEntitiesOfClass(ItemEntity.class, AABB.inflate(merge, ...), pred)
 * inside ItemEntity.mergeWithNeighbours()V.
 *
 * Two STRICT receiver-prepended static retargets (byte-patch by
 * src/items_index.rs, pattern of region_threads / FluidBitmaskOps):
 *   1. ItemEntity.mergeWithNeighbours()V: the single
 *      Level.getEntitiesOfClass(Class;AABB;Predicate)List site ->
 *      ItemMergeIndexOps.getMergeCandidates(Level;Class;AABB;Predicate)List.
 *      Same candidate set by construction: the index enumerates a SUPERSET
 *      (every bucket overlapped by the query AABB), then applies the EXACT
 *      vanilla filter chain — box.intersects(entity.getBoundingBox()) plus
 *      the ORIGINAL vanilla predicate (the very lambda instance vanilla
 *      passed: other != this && other.isMergable()).
 *   2. ItemEntity.tick()V: the single Entity.move(MoverType;Vec3)V site ->
 *      ItemMergeIndexOps.moveIndexed(ItemEntity;MoverType;Vec3)V — exact
 *      vanilla move, then reconcile() re-slots the item into its grid bucket.
 *      An ItemEntity's position changes ONLY through its own ticked move (or
 *      spawn/teleport setPos), so the index is position-exact at every query
 *      (bounded documented deviation: <=1 tick for items spawned/teleported
 *      between ticks; the querying item itself always reconciles first,
 *      because tick() runs move (offset 269) BEFORE merge (offset 471)).
 *
 * Parity: when the lever flag is absent this class is never defined nor
 * wired (dormant-invisible, exact vanilla bytes served). The vanilla
 * fallback inside getMergeCandidates reproduces the original virtual call
 * 1:1 (parity by construction, same fallback contract as the Lithium
 * item_entity_merging redirect). tryToMerge/merge/despawn/pickup vanilla
 * bodies are untouched. Dead (merged-away/despawned) items are purged
 * lazily by queries and by reconcile.
 *
 * Thread safety: region_threads=4 workers tick disjoint spatial regions;
 * bucket mutation is serialized per-bucket (synchronized ArrayList), the
 * two maps are ConcurrentHashMaps. No nested classes — single classfile for
 * the kernel-loader delivery (S7-163 lesson); the one lambda below compiles
 * to invokedynamic + synthetic method, not a nested class file.
 */
public final class ItemMergeIndexOps {

    /** Lever flag value this bridge arms under (bench exports CRUSSTY_LEVER_FLAG). */
    public static final String LEVER_FLAG = "items_index";

    /** Belt-and-braces gate: rust already gates class definition on the same env. */
    static final boolean ARMED =
            LEVER_FLAG.equals(System.getenv("CRUSSTY_LEVER_FLAG"));

    /** Grid bucket key -> live (or stale-not-yet-purged) item list. */
    static final ConcurrentHashMap<Long, ArrayList<ItemEntity>> BUCKETS =
            new ConcurrentHashMap<>();

    /** ItemEntity -> current bucket key (single-element long box, one alloc per item). */
    static final ConcurrentHashMap<ItemEntity, long[]> KEY_BY_ENTITY =
            new ConcurrentHashMap<>();

    private static final java.util.concurrent.atomic.AtomicLong RECONCILED =
            new java.util.concurrent.atomic.AtomicLong();

    private ItemMergeIndexOps() {}

    // ------------------------------------------------------------------
    // Splice 1: merge-candidate query (retarget of the getEntitiesOfClass
    // site inside ItemEntity.mergeWithNeighbours()V).
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
        // position, which may sit up to (entity half-extent) OUTSIDE the
        // query box while their own AABB still intersects it (ItemEntity
        // width 0.25 -> half-extent 0.125). Expand the query box by 0.5
        // (>= any ItemEntity half-extent) before mapping to cells; the
        // exact vanilla filter in collect() keeps the candidate set
        // bit-identical to level.getEntitiesOfClass.
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
    // Splice 2: post-move reconcile (retarget of the Entity.move site
    // inside ItemEntity.tick()V; tick runs move BEFORE merge, so the
    // querying item is always index-exact at its own merge query).
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
                    "[ItemMergeIndexOps] armed; index live (first reconcile)");
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
    // Grid keying: 1.0-block cells, SectionPos-style packing
    // (x:26b << 38 | z:26b << 12 | y:12b) — matches vanilla section
    // packing conventions; world height range fits the 12-bit y field.
    // ------------------------------------------------------------------

    static long key(double x, double y, double z) {
        return pack(Mth.floor(x), Mth.floor(y), Mth.floor(z));
    }

    static long pack(int x, int y, int z) {
        return ((long) (x & 0x3FFFFFF) << 38) | ((long) (z & 0x3FFFFFF) << 12)
                | (long) (y & 0xFFF);
    }

    /** Direct-mapped Long box cache — steady-state map ops stay alloc-free. */
    private static final Long[] BOX_CACHE = new Long[1024];

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
