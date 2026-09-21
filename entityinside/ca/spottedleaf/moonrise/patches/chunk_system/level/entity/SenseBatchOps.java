package ca.spottedleaf.moonrise.patches.chunk_system.level.entity;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Predicate;

import net.minecraft.server.level.FullChunkStatus;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.phys.AABB;

/**
 * SenseBatchOps (TASK-399-D, mega-round-3, agent D — vector cmp399_sensebatch:
 * BATCHED SENSING QUERIES).
 *
 * ARCHITECTURE — batched transport, NOT a memo:
 * The two EntityLookup getEntities funnels
 *   (1) getEntities(Entity, AABB, List, Predicate)
 *   (2) <T> getEntities(Class<? extends T>, Entity, AABB, List, Predicate)
 * are body-swapped (classfile.rs::patch_entitylookup_sensebatch) to delegate
 * here. During a RegionTickOps bucket AI phase (ThreadLocal slab with a
 * non-zero generation), each (ChunkEntitySlices, query-class) pair is scanned
 * EXACTLY ONCE per bucket by the vanilla ChunkEntitySlices.getEntities over
 * the full section column; every sensing query for that pair afterwards
 * filters the pre-assembled candidate slice with the vanilla per-entity
 * checks in the vanilla bytecode positions:
 *   e != source  ->  bbox.intersects(box)  ->  pred.test
 * Outside a phase the vanilla region-loop is replicated byte-exactly
 * (vanillaSrc / vanillaCls below — same loop shape as the swapped bodies).
 *
 * What this is NOT: no cross-tick state, no result caching keyed by AABB.
 * Candidate slabs are invalidated by a per-bucket generation counter and can
 * only be observed by queries of the SAME bucket phase on the SAME thread —
 * the reuse window is one tick-bucket, i.e. strictly inside the phase where
 * the existing RegionTickOps parallel model already accepts intra-phase
 * visibility races between regions. Candidate ORDER is vanilla: slabs are
 * produced by vanilla ChunkEntitySlices.getEntities (sections ascending,
 * per-class pool-slot order) and consumed in the vanilla region-loop order
 * regZ -> regX -> z -> x, so the emitted candidate sequence is identical to
 * an unbatched scan modulo entities that moved/died mid-phase (documented
 * LEVER-V399-D.md).
 *
 * Fail-closed: if this class fails to define, region_threads' bridge list
 * aborts and RegionTickOps never runs; if the EntityLookup retarget does not
 * apply, vanilla transport runs; ARMED=false (flag absent) makes beginBucket
 * a no-op and every swapped body takes the replicated vanilla path.
 */
public final class SenseBatchOps {

    private SenseBatchOps() {}

    /** Set once by the rust bridge (SenseBatchOps.arm()) when CRUSSTY_LEVER_FLAG=cmp399_sensebatch. */
    public static volatile boolean ARMED = false;

    /** Java-side ARM marker (task contract: java-marker required). */
    public static void arm() {
        ARMED = true;
        System.err.println(
                "[crussty-plugin] cmp399_sensebatch: java-side ARMED (SenseBatchOps batched transport live; EntityLookup funnels routed)");
    }

    /** One-shot marker for the first actually batched bucket. */
    private static volatile boolean MARKED = false;

    /** Bucket generation: bumped by beginBucket(), zeroed by endBucket(). */
    private static final java.util.concurrent.atomic.AtomicLong GEN =
            new java.util.concurrent.atomic.AtomicLong(0L);

    /** Eviction cap so a pathological bucket cannot grow the map unbounded. */
    private static final int SLAB_CAP = 1024;

    private static final ThreadLocal<Slab> SLAB = new ThreadLocal<>();

    /** Map key: chunk-slices identity + query class identity (null = all-entities). */
    private static final class Key {
        ChunkEntitySlices slices;
        Class<?> clazz;
        int hash;

        Key() {}

        void set(final ChunkEntitySlices slices, final Class<?> clazz) {
            this.slices = slices;
            this.clazz = clazz;
            this.hash = System.identityHashCode(slices) * 31
                    + (clazz == null ? 0 : clazz.hashCode());
        }

        @Override
        public boolean equals(final Object o) {
            if (this == o) return true;
            if (!(o instanceof Key)) return false;
            final Key k = (Key) o;
            return this.slices == k.slices && this.clazz == k.clazz;
        }

        @Override
        public int hashCode() {
            return this.hash;
        }
    }

    private static final class Entry {
        long gen;
        Entity[] cands;
    }

    private static final class Slab {
        final HashMap<Key, Entry> map = new HashMap<>(256);
        final Key probe = new Key();
        final ArrayList<Entity> tmp = new ArrayList<>(256);
        long gen;
    }

    /** RegionTickOps calls at the start of each bucket AI phase (per ticking thread). */
    public static void beginBucket() {
        if (!ARMED) return;
        Slab s = SLAB.get();
        if (s == null) {
            s = new Slab();
            SLAB.set(s);
        }
        s.gen = GEN.incrementAndGet();
        if (s.map.size() > SLAB_CAP) {
            s.map.clear();
        }
        if (!MARKED) {
            MARKED = true;
            System.err.println(
                    "[crussty-plugin] cmp399_sensebatch: first batched bucket tick (slab transport engaged)");
        }
    }

    /** RegionTickOps calls in the bucket-tick finally (per ticking thread). */
    public static void endBucket() {
        if (!ARMED) return;
        final Slab s = SLAB.get();
        if (s != null) {
            s.gen = 0L; // entries become stale; arrays are dropped lazily
        }
    }

    // ---------------------------------------------------------------------
    // Retargeted entry points (bodies of the two EntityLookup funnels).
    // ---------------------------------------------------------------------

    /** Body of EntityLookup.getEntities(Entity, AABB, List, Predicate). */
    public static void getEntitiesSrc(final EntityLookup lookup, final Entity source, final AABB box,
                                      final List<Entity> list, final Predicate<? super Entity> pred) {
        final Slab s = SLAB.get();
        if (s == null || s.gen == 0L) {
            vanillaSrc(lookup, source, box, list, pred);
            return;
        }
        batched(lookup, null, source, box, list, pred, s);
    }

    /** Body of EntityLookup.getEntities(Class, Entity, AABB, List, Predicate). */
    @SuppressWarnings({"rawtypes", "unchecked"})
    public static void getEntitiesCls(final EntityLookup lookup, final Class<?> clazz, final Entity source,
                                      final AABB box, final List list, final Predicate pred) {
        final Slab s = SLAB.get();
        if (s == null || s.gen == 0L) {
            vanillaCls(lookup, clazz, source, box, list, pred);
            return;
        }
        batched(lookup, clazz, source, box, list, pred, s);
    }

    // ---------------------------------------------------------------------
    // Batched transport.
    // ---------------------------------------------------------------------

    private static void batched(final EntityLookup lookup, final Class<?> clazz, final Entity source,
                                final AABB box, final List list, final Predicate pred, final Slab s) {
        // Chunk bounds: byte-exact clone of the swapped vanilla bodies
        // (Mth.floor(coordinate) -/+ 2, then >> 4).
        final int minChunkX = (Mth.floor(box.minX) - 2) >> 4;
        final int minChunkZ = (Mth.floor(box.minZ) - 2) >> 4;
        final int maxChunkX = (Mth.floor(box.maxX) + 2) >> 4;
        final int maxChunkZ = (Mth.floor(box.maxZ) + 2) >> 4;
        final int regMinX = minChunkX >> 5;
        final int regMinZ = minChunkZ >> 5;
        final int regMaxX = maxChunkX >> 5;
        final int regMaxZ = maxChunkZ >> 5;

        for (int regZ = regMinZ; regZ <= regMaxZ; regZ++) {
            final int zStart = regZ == regMinZ ? minChunkZ & 31 : 0;
            final int zEnd = regZ == regMaxZ ? maxChunkZ & 31 : 31;
            for (int regX = regMinX; regX <= regMaxX; regX++) {
                final int xStart = regX == regMinX ? minChunkX & 31 : 0;
                final int xEnd = regX == regMaxX ? maxChunkX & 31 : 31;
                final EntityLookup.ChunkSlicesRegion region = lookup.getRegion(regX, regZ);
                if (region == null) {
                    continue;
                }
                for (int lz = zStart; lz <= zEnd; lz++) {
                    for (int lx = xStart; lx <= xEnd; lx++) {
                        final ChunkEntitySlices slices = region.get(lx | (lz << 5));
                        if (slices == null || !slices.status.isOrAfter(FullChunkStatus.FULL)) {
                            continue;
                        }
                        final Entity[] cands = slabFor(slices, clazz, s);
                        for (int i = 0; i < cands.length; i++) {
                            final Entity e = cands[i];
                            if (e == source) {
                                continue;
                            }
                            if (!e.getBoundingBox().intersects(box)) {
                                continue;
                            }
                            if (pred != null && !pred.test(e)) {
                                continue;
                            }
                            list.add(e);
                        }
                    }
                }
            }
        }
    }

    /** One vanilla scan per (chunk-slices, class) pair per bucket generation. */
    private static Entity[] slabFor(final ChunkEntitySlices slices, final Class<?> clazz, final Slab s) {
        final Key probe = s.probe;
        probe.set(slices, clazz);
        Entry e = s.map.get(probe);
        if (e != null && e.gen == s.gen) {
            return e.cands;
        }
        // Full-column box: guarantees the vanilla section clamp covers every
        // section this slice can hold; any entity registered in this slice
        // contains its own (in-column) position, so intersects() is total.
        final AABB column = new AABB(
                (double)(slices.chunkX << 4),
                (double)(slices.minSection << 4) - 4.0,
                (double)(slices.chunkZ << 4),
                (double)((slices.chunkX + 1) << 4),
                (double)((slices.maxSection + 1) << 4) + 4.0,
                (double)((slices.chunkZ + 1) << 4));
        s.tmp.clear();
        if (clazz == null) {
            slices.getEntities((Entity) null, column, s.tmp, null);
        } else {
            // raw-Class call: the erased signature matches the bytecode the
            // swapped body used to emit; no inference on wildcards.
            @SuppressWarnings("rawtypes")
            Class rawClazz = clazz;
            slices.getEntities(rawClazz, null, column, (List) s.tmp, null);
        }
        final Entity[] arr = s.tmp.toArray(new Entity[0]);
        if (e == null) {
            e = new Entry();
            final Key stored = new Key();
            stored.set(slices, clazz);
            s.map.put(stored, e);
        } else {
            // drop the stale ref table before overwrite (retention hygiene)
            e.cands = null;
        }
        e.gen = s.gen;
        e.cands = arr;
        return arr;
    }

    // ---------------------------------------------------------------------
    // Vanilla fallbacks — byte-exact replication of the swapped bodies
    // (fail-safe when no bucket phase is active on this thread).
    // ---------------------------------------------------------------------

    private static void vanillaSrc(final EntityLookup lookup, final Entity source, final AABB box,
                                   final List<Entity> list, final Predicate<? super Entity> pred) {
        final int minChunkX = (Mth.floor(box.minX) - 2) >> 4;
        final int minChunkZ = (Mth.floor(box.minZ) - 2) >> 4;
        final int maxChunkX = (Mth.floor(box.maxX) + 2) >> 4;
        final int maxChunkZ = (Mth.floor(box.maxZ) + 2) >> 4;
        final int regMinX = minChunkX >> 5;
        final int regMinZ = minChunkZ >> 5;
        final int regMaxX = maxChunkX >> 5;
        final int regMaxZ = maxChunkZ >> 5;
        for (int regZ = regMinZ; regZ <= regMaxZ; regZ++) {
            final int zStart = regZ == regMinZ ? minChunkZ & 31 : 0;
            final int zEnd = regZ == regMaxZ ? maxChunkZ & 31 : 31;
            for (int regX = regMinX; regX <= regMaxX; regX++) {
                final int xStart = regX == regMinX ? minChunkX & 31 : 0;
                final int xEnd = regX == regMaxX ? maxChunkX & 31 : 31;
                final EntityLookup.ChunkSlicesRegion region = lookup.getRegion(regX, regZ);
                if (region == null) {
                    continue;
                }
                for (int lz = zStart; lz <= zEnd; lz++) {
                    for (int lx = xStart; lx <= xEnd; lx++) {
                        final ChunkEntitySlices slices = region.get(lx | (lz << 5));
                        if (slices != null && slices.status.isOrAfter(FullChunkStatus.FULL)) {
                            slices.getEntities(source, box, list, pred);
                        }
                    }
                }
            }
        }
    }

    @SuppressWarnings({"rawtypes", "unchecked"})
    private static void vanillaCls(final EntityLookup lookup, final Class<?> clazz, final Entity source,
                                   final AABB box, final List list, final Predicate pred) {
        final int minChunkX = (Mth.floor(box.minX) - 2) >> 4;
        final int minChunkZ = (Mth.floor(box.minZ) - 2) >> 4;
        final int maxChunkX = (Mth.floor(box.maxX) + 2) >> 4;
        final int maxChunkZ = (Mth.floor(box.maxZ) + 2) >> 4;
        final int regMinX = minChunkX >> 5;
        final int regMinZ = minChunkZ >> 5;
        final int regMaxX = maxChunkX >> 5;
        final int regMaxZ = maxChunkZ >> 5;
        for (int regZ = regMinZ; regZ <= regMaxZ; regZ++) {
            final int zStart = regZ == regMinZ ? minChunkZ & 31 : 0;
            final int zEnd = regZ == regMaxZ ? maxChunkZ & 31 : 31;
            for (int regX = regMinX; regX <= regMaxX; regX++) {
                final int xStart = regX == regMinX ? minChunkX & 31 : 0;
                final int xEnd = regX == regMaxX ? maxChunkX & 31 : 31;
                final EntityLookup.ChunkSlicesRegion region = lookup.getRegion(regX, regZ);
                if (region == null) {
                    continue;
                }
                for (int lz = zStart; lz <= zEnd; lz++) {
                    for (int lx = xStart; lx <= xEnd; lx++) {
                        final ChunkEntitySlices slices = region.get(lx | (lz << 5));
                        if (slices != null && slices.status.isOrAfter(FullChunkStatus.FULL)) {
                            @SuppressWarnings("rawtypes")
                            Class rawClazz = clazz;
                            slices.getEntities(rawClazz, source, box, list, pred);
                        }
                    }
                }
            }
        }
    }
}
