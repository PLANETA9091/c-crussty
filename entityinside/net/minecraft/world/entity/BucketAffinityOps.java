package net.minecraft.world.entity;

import net.minecraft.world.level.ChunkPos;

/**
 * BucketAffinityOps (TASK-459-CX6, WILD law-11 idea C-X6) — ticking-bucket
 * AFFINITY SHUFFLE stub for RegionTickOps (region_threads=4).
 *
 * STRICT DORMANT SCAFFOLD: this class is NOT compiled into any tracked
 * build/ blob, is NOT referenced by RegionTickOps, and defines no byte
 * hooks. It is the java-side CONTRACT for the future lever carrier
 * (lesson-408: a lever baked into SOURCES without a rebuilt tracked blob
 * = placebo sleeping gate — the carrier must rebuild blobs and verify
 * javap flat==nested 10/10 BEFORE any dispatch).
 *
 * Idea (RESEARCH-459-CX6.md): the vanilla w=4 bucketing is the 8-chunk
 * region checkerboard ((rx&1)<<1)|(rz&1) — orthogonal region neighbours
 * are never on the same worker, so every worker's broadphase working set
 * (moonrise EntityLookup entity sections, chunk refs, movement-AABB vs
 * 8x8-chunk region reads; broadphase lane 8.8-10.9% of tick wall) runs
 * cache-cold. The affinity mapping swaps bucket MEMBERSHIP ONLY:
 *   affinityBucketOf(w=4) = ((floorDiv(rx,2)&1)<<1) | (floorDiv(rz,2)&1)
 * i.e. 2x2 blocks of 8-chunk regions (16x16-chunk tile per worker), so
 * adjacent regions co-locate on one worker with P=1/2 (vs 0 vanilla) and
 * the per-worker tick set becomes a spatially coherent tile.
 *
 * PARITY (preregistered G2, bit-for-bit): the RegionTickOps.forEach
 * partition loop appends in snapshot order regardless of bucket id — the
 * shuffle changes which bucket an entity lands in, never the order inside
 * a bucket; every intra-bucket sequence stays a subsequence of the vanilla
 * snapshot. Verified offline by the bucket_sequences oracle + the Rust
 * unit tests in src/bucket_affinity.rs.
 *
 * Future wire site (single, census'd): RegionTickOps.java:573
 *   int s = bucketOf(e, w);
 * -> when ARMED (lever flag STRICT-eq "cmp459_cx6" AND region_threads>=2):
 *   int s = BucketAffinityOps.affinityBucketOf(e, w);
 * Fail-closed: this method returns VANILLA mapping (NOT_ARMED sentinel at
 * the gate) whenever the lever is not armed — the call site change is then
 * a bit-identical no-op.
 */
public final class BucketAffinityOps {

    private static final int REGION_CHUNKS = 8;

    /** Sentinel returned when NOT armed (caller keeps vanilla mapping). */
    public static final int NOT_ARMED = -1;

    private BucketAffinityOps() {}

    /**
     * STRICT eq lever gate (round-400 protocol — never starts_with /
     * contains: полу-вооружённый мост AIOOBE lesson TASK-400-D). Full arm
     * also requires CRUSSTY_REGION_THREADS >= 2 (composition with the
     * region lever, same requirement set as BatchCollector S7-160).
     */
    public static boolean isArmed() {
        String flag = System.getenv("CRUSSTY_LEVER_FLAG");
        if (flag == null || !"cmp459_cx6".equals(flag.trim())) return false;
        String workers = System.getenv("CRUSSTY_REGION_THREADS");
        if (workers == null) return false;
        try {
            return Integer.parseInt(workers.trim()) >= 2;
        } catch (NumberFormatException nfe) {
            return false; // fail-closed
        }
    }

    /**
     * Affinity bucket for the snapshot partition (single future wire site:
     * RegionTickOps.forEach). NOT armed -> NOT_ARMED sentinel (vanilla
     * caller path untouched). Hash mapping for w outside {2,4} is returned
     * unchanged (out of CX6 scope).
     */
    public static int affinityBucketOf(Entity entity, int w) {
        if (!isArmed()) return NOT_ARMED;
        ChunkPos cp = entity.chunkPosition();
        int rx = Math.floorDiv(cp.x, REGION_CHUNKS);
        int rz = Math.floorDiv(cp.z, REGION_CHUNKS);
        if (w == 4) return ((Math.floorDiv(rx, 2) & 1) << 1) | (Math.floorDiv(rz, 2) & 1);
        if (w == 2) return Math.floorDiv(rx, 2) & 1;
        return vanillaBucketOf(rx, rz, w);
    }

    /**
     * Vanilla RegionTickOps.bucketOf on region coords — replicated
     * bit-for-bit for the RegionLockstepHarness oracle (G2: identical
     * per-bucket subsequences vanilla vs affinity; mapping_delta census
     * for the G1 effect marker). Public API: the harness must be able to
     * diff both mappings on one snapshot.
     */
    public static int vanillaBucketOf(int rx, int rz, int w) {
        if (w == 4) return ((rx & 1) << 1) | (rz & 1);
        if (w == 2) return rx & 1;
        return Math.floorMod(rx * 668265261 + rz * 374761393, w);
    }
}
