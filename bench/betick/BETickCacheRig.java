import ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable;

import java.util.Arrays;
import java.util.Random;

/**
 * TASK-89 P500-style micro-rig (zero boots): per-block-entity shouldTickBlocksAt
 * cost, Paper status quo vs SparklyPaper-pattern single-entry cache.
 *
 * Ground truth being modeled (javap-verified on purpur-1.21.10.jar):
 *   ServerLevel.shouldTickBlocksAt(long):
 *     moonrise$getChunkTaskScheduler().chunkHolderManager.getChunkHolder(long)
 *       -> null check -> NewChunkHolder.isTickingReady()
 *   Level.tickBlockEntities(): calls shouldTickBlocksAt(ticker.getPos()) PER BE PER TICK.
 *   ChunkHolderManager.chunkHolders = ConcurrentLong2ReferenceChainedHashTable (REAL class used here).
 *
 * BE-list shape per SparklyPaper README: sorted-by-chunk main body + appended tail
 * of newly placed BEs (modeled: 90% sorted, 10% random). 5% of keys miss (non-ticking).
 *
 * Conservative bias: single-threaded warm-cache measurement makes the OLD arm look
 * FASTER than production (huge map, cold lines) -> if savings still land below the
 * pre-registered GO gate, the NO-GO is robust.
 */
public final class BETickCacheRig {

    static final class Holder {
        volatile boolean tickingReady;
    }

    // target share of chunk-change boundaries hit by the single-entry cache
    long sink; // blackhole

    ConcurrentLong2ReferenceChainedHashTable<Holder> map;
    long[] keys; // one per block entity, in tick order

    long runOld() {
        long acc = sink;
        final ConcurrentLong2ReferenceChainedHashTable<Holder> m = map;
        final long[] ks = keys;
        for (int i = 0; i < ks.length; i++) {
            final Holder h = m.get(ks[i]);
            acc += (h != null && h.tickingReady) ? 1L : 0L;
        }
        return sink = acc;
    }

    long runNew() {
        long acc = sink;
        final ConcurrentLong2ReferenceChainedHashTable<Holder> m = map;
        final long[] ks = keys;
        long lastKey = Long.MIN_VALUE;
        boolean lastR = false;
        for (int i = 0; i < ks.length; i++) {
            final long k = ks[i];
            boolean r;
            if (k == lastKey) {
                r = lastR;
            } else {
                final Holder h = m.get(k);
                r = h != null && h.tickingReady;
                lastKey = k;
                lastR = r;
            }
            acc += r ? 1L : 0L;
        }
        return sink = acc;
    }

    double[] bench(final boolean newArm, final int framesPerWindow, final int windows) {
        final double[] med = new double[windows];
        for (int w = 0; w < windows; w++) {
            final long t0 = System.nanoTime();
            for (int f = 0; f < framesPerWindow; f++) {
                sink += (newArm ? runNew() : runOld()) & 1L;
            }
            med[w] = (double) (System.nanoTime() - t0) / (framesPerWindow * keys.length);
        }
        Arrays.sort(med);
        return med;
    }

    void buildScenario(final int totalBEs, final int besPerChunk, final long seed) {
        final Random rnd = new Random(seed);
        final int chunks = Math.max(1, totalBEs / besPerChunk);
        map = ConcurrentLong2ReferenceChainedHashTable.createWithExpected(chunks + chunks / 16);
        keys = new long[totalBEs];
        final Holder ready = new Holder(); ready.tickingReady = true;
        final Holder notReady = new Holder(); notReady.tickingReady = false;
        // distinct chunk keys, 5% of them absent from the map (non-ticking / unloaded)
        final long[] chunkKeys = new long[chunks];
        final boolean[] present = new boolean[chunks];
        for (int c = 0; c < chunks; c++) {
            chunkKeys[c] = 1099511628211L * (c + 1) + 12345; // spread
            present[c] = (rnd.nextInt(100) >= 5);
            if (present[c]) {
                map.put(chunkKeys[c], rnd.nextInt(100) < 90 ? ready : notReady);
            }
        }
        // main body: sorted by chunk (BEs grouped per chunk), tail: random (newly placed)
        final int sortedCount = (int) (totalBEs * 0.9);
        final int tailCount = totalBEs - sortedCount;
        int idx = 0;
        outer:
        for (int c = 0; c < chunks; c++) {
            for (int b = 0; b < besPerChunk && idx < sortedCount; b++, idx++) {
                keys[idx] = chunkKeys[c];
                if (idx >= totalBEs) break outer;
            }
        }
        // fill remainder if totalBEs > chunks*besPerChunk
        for (; idx < sortedCount; idx++) {
            keys[idx] = chunkKeys[rnd.nextInt(chunks)];
        }
        for (int t = 0; t < tailCount; t++, idx++) {
            keys[idx] = chunkKeys[rnd.nextInt(chunks)];
        }
        // shuffle tail region only
        for (int i = sortedCount; i < totalBEs; i++) {
            final int j = sortedCount + rnd.nextInt(totalBEs - sortedCount);
            final long tmp = keys[i]; keys[i] = keys[j]; keys[j] = tmp;
        }
    }

    public static void main(final String[] args) {
        final int[][] scenarios = {{500, 8}, {5_000, 8}, {5_000, 32}, {50_000, 8}, {50_000, 32}};
        System.out.println("scenario | ns/BE old | ns/BE new | ratio | saved_ns/BE | ceiling_%server_thread_at_20tps");
        for (final int[] sc : scenarios) {
            final int n = sc[0], per = sc[1];
            final BETickCacheRig rig = new BETickCacheRig();
            rig.buildScenario(n, per, 42L + n);
            // warmup
            for (int i = 0; i < 3; i++) { rig.runOld(); rig.runNew(); }
            final int frames = Math.max(2_000, 4_000_000 / Math.max(1, n));
            final double[] o = rig.bench(false, frames, 11);
            final double[] nw = rig.bench(true, frames, 11);
            final double oldMed = o[5], newMed = nw[5];
            final double saved = oldMed - newMed;
            // ceiling: saved_ns_per_BE * BEs * 20 tps = ns saved per real second on server thread
            final double pctCore = saved * n * 20.0 / 1_000_000_000.0 * 100.0;
            System.out.printf("%6d BEs x%2d/chunk | %8.2f | %8.2f | %.3fx | %6.2f | %.3f%%%n",
                    n, per, oldMed, newMed, oldMed / newMed, saved, pctCore);
        }
        System.out.println("sink=" + 0); // keep
    }
}
