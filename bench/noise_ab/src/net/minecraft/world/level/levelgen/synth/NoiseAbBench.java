package net.minecraft.world.level.levelgen.synth;

import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.ImprovedNoiseNativeOps;
import net.minecraft.world.level.levelgen.synth.ImprovedNoiseNativeOpsLegacy;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicReference;

/**
 * P500 noise-bridge A/B: legacy bridge (git HEAD baseline: Finalizer-era
 * map access on every call via Collections.synchronizedMap) vs current
 * bridge (Cleaner lifecycle + per-thread direct-mapped identity cache in
 * front of the map).
 *
 * Both bridges live in the SAME JVM and call the SAME closed-source native
 * kernels (PaperNativeImprovedNoise.nativeNoise), so any measured delta is
 * pure Java-side bridge overhead — exactly the layer c-crussty owns.
 *
 * Profiles:
 *   single  — one noise object, one thread (pure hit path)
 *   inter8  — 8 noises round-robin, one thread (PerlinNoise-octave-like mix)
 *   inter64 — 64 noises round-robin, one thread (BlendedNoise-ish depth)
 *   mt2x8   — 2 worker threads, each with 8 PRIVATE noises round-robin
 *             (models a 2-CPU chunk worker pool = worker count == cores,
 *             the real Paper config; NOTE mt4x8 on a 2-CPU box measures
 *             oversubscription scheduler noise, not the bridge)
 *   rawMt2  — 2 threads calling PaperNativeImprovedNoise.nativeNoise
 *             DIRECTLY with prebuilt handles (no bridge at all). Isolates
 *             native-kernel multi-thread behavior: if rawMt2 per-op cost
 *             rises vs single-thread native cost, the closed kernel itself
 *             serializes internally, and bridge-level mt differences are
 *             second-order effects.
 *   mt4x8   — 4 worker threads, each with 8 PRIVATE noises round-robin
 *             (models chunk workers; the legacy monitor is global, so this
 *             is where contention should show)
 *   churn   — 50k fresh noises, one noise() each, drop (build path + no
 *             double-free smoke; NOT a release-latency benchmark)
 *
 * Methodology (mirrors bench/p500): calibrate ops/batch to ~120 ms,
 * median-of-5 batches, forward+reverse order with min-of-medians to cancel
 * order bias, SINK defeat DCE, parity check (new result == legacy result)
 * before any timing.
 *
 * Usage: NoiseAbBench <path-to-libpaper_native_jni.so> [profile ...]
 */
public final class NoiseAbBench {
    private static volatile double SINK = 0.0;

    interface Sampler { double sample(ImprovedNoise n, double x, double y, double z); }

    static final class Bridge {
        final String name;
        final Sampler sampler;
        double paritySum;
        Bridge(String name, Sampler sampler) {
            this.name = name;
            this.sampler = sampler;
        }
        double sample(ImprovedNoise n, double x, double y, double z) {
            return sampler.sample(n, x, y, z);
        }
    }

    public static void main(String[] args) throws Exception {
        String so = args[0];
        System.load(so);
        List<String> profiles = args.length > 1
                ? Arrays.asList(args).subList(1, args.length)
                : List.of("single", "inter8", "inter64", "mt2x8", "rawMt2", "mt4x8", "churn");

        byte[] perm = permutation();
        Bridge legacy = new Bridge("legacy(synchronizedMap+finalize)",
                (n, x, y, z) -> ImprovedNoiseNativeOpsLegacy.noise(n, perm, 0, 0, 0, x, y, z, 1.0, 1.0));
        Bridge current = new Bridge("current(Cleaner+direct-mapped)",
                (n, x, y, z) -> ImprovedNoiseNativeOps.noise(n, perm, 0, 0, 0, x, y, z, 1.0, 1.0));

        System.out.printf(Locale.ROOT, "native lib loaded: %s%n", so);
        System.out.printf(Locale.ROOT, "%-14s %-34s %12s%n", "profile", "bridge", "ns/op");

        Bridge[] order1 = { legacy, current };
        Bridge[] order2 = { current, legacy };
        for (String p : profiles) {
            if (p.equals("churn")) { churn(order1); churn(order2); continue; }
            if (p.equals("rawMt2")) { rawMt2(); continue; }
            // Each runProfile executes the bridges in ONE order; running both
            // orders and taking per-bridge minima cancels order bias (same
            // protocol as bench/p500).
            double[] fwd = runProfile(p, order1);
            double[] rev = runProfile(p, order2);
            System.out.printf(Locale.ROOT, "%-14s %-34s %12.1f%n", p, legacy.name, Math.min(fwd[0], rev[0]));
            System.out.printf(Locale.ROOT, "%-14s %-34s %12.1f%n", p, current.name, Math.min(fwd[1], rev[1]));
        }
        System.out.println("done");
    }

    // ---- profiles -----------------------------------------------------------

    static double[] runProfile(String profile, Bridge[] bridges) throws Exception {
        switch (profile) {
            case "single": return sampled("single", 1, 1, bridges);
            case "inter8": return sampled("inter8", 1, 8, bridges);
            case "inter64": return sampled("inter64", 1, 64, bridges);
            case "mt2x8": return sampled("mt2x8", 2, 8, bridges);
            case "mt4x8": return sampled("mt4x8", 4, 8, bridges);
            default: throw new IllegalArgumentException("unknown profile " + profile);
        }
    }

    /**
     * threads * noises identity-distinct noise objects; each thread cycles its
     * own subset round-robin. Returns min over both orders of the per-op cost.
     */
    static double[] sampled(String label, int threads, int noises, Bridge[] bridges) throws Exception {
        byte[] perm = permutation();
        // per-thread private noise sets -> measures bridge/map overhead, not
        // native handle contention (native kernels are assumed stateless).
        ImprovedNoise[][] sets = new ImprovedNoise[threads][];
        for (int t = 0; t < threads; t++) {
            ImprovedNoise[] set = new ImprovedNoise[noises];
            for (int i = 0; i < noises; i++) set[i] = new ImprovedNoise();
            sets[t] = set;
        }
        double[] best = new double[bridges.length];
        for (int b = 0; b < bridges.length; b++) best[b] = Double.POSITIVE_INFINITY;
        for (int b = 0; b < bridges.length; b++) {
            Bridge bridge = bridges[b];
            parityCheck(bridge, sets[0][0], perm);
            int innerOps = calibrate(bridge, sets);
            double[] med = new double[5];
            for (int rep = 0; rep < 5; rep++) med[rep] = timedRun(bridge, sets, innerOps);
            Arrays.sort(med);
            double nsPerOp = med[2] / ((double) threads * innerOps);
            best[b] = Math.min(best[b], nsPerOp);
            System.out.printf(Locale.ROOT, "    [%s] %s batch median: %.1f ns/op%n", label, bridge.name, nsPerOp);
        }
        // Bit-exact parity: both bridges call the SAME native kernel with the
        // SAME arguments, so any value difference means the bridge layers are
        // NOT equivalent — fail loudly, never publish a timing.
        if (Double.doubleToRawLongBits(bridges[0].paritySum) != Double.doubleToRawLongBits(bridges[1].paritySum)) {
            throw new IllegalStateException(String.format(Locale.ROOT,
                    "parity FAIL on %s: %.17g vs %.17g", label, bridges[0].paritySum, bridges[1].paritySum));
        }
        return best;
    }

    static int calibrate(Bridge bridge, ImprovedNoise[][] sets) {
        int innerOps = 4096;
        long t0 = System.nanoTime();
        runOnce(bridge, sets, innerOps);
        long dt = System.nanoTime() - t0;
        if (dt < 1_000_000L) return innerOps * 16;
        // target ~120 ms per batch
        long target = 120_000_000L;
        long scale = Math.min(64L, Math.max(1L, target / Math.max(1L, dt)));
        return (int) Math.min(Integer.MAX_VALUE / 2, innerOps * scale);
    }

    static void runOnce(Bridge bridge, ImprovedNoise[][] sets, int innerOps) {
        if (sets.length == 1) {
            ImprovedNoise[] set = sets[0];
            int n = set.length;
            double x = 0.5;
            for (int i = 0; i < innerOps; i++) {
                // round-robin through the set: interleaved octaves pattern
                x += bridge.sample(set[i & (n - 1)], x, i * 0.125, i * 0.25);
            }
            SINK = x;
        } else {
            Thread[] ts = new Thread[sets.length];
            CountDownLatch start = new CountDownLatch(1);
            CountDownLatch done = new CountDownLatch(sets.length);
            for (int t = 0; t < sets.length; t++) {
                ImprovedNoise[] set = sets[t];
                ts[t] = new Thread(() -> {
                    try {
                        start.await();
                        double x = 0.5 + Thread.currentThread().getId() * 0.31;
                        for (int i = 0; i < innerOps; i++) {
                            x += bridge.sample(set[i & (set.length - 1)], x, i * 0.125, i * 0.25);
                        }
                        SINK = x;
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    } finally {
                        done.countDown();
                    }
                }, "bench-" + t);
                ts[t].start();
            }
            start.countDown();
            try { done.await(); } catch (InterruptedException e) { Thread.currentThread().interrupt(); }
        }
    }

    static double timedRun(Bridge bridge, ImprovedNoise[][] sets, int innerOps) {
        long t0 = System.nanoTime();
        runOnce(bridge, sets, innerOps);
        return (double) (System.nanoTime() - t0);
    }

    // ---- parity -------------------------------------------------------------

    static void parityCheck(Bridge bridge, ImprovedNoise noise, byte[] perm) {
        double acc = 0.0;
        for (int i = 0; i < 64; i++) {
            acc += bridge.sample(noise, i * 0.5, i * 0.25, i * 0.125);
        }
        if (!Double.isFinite(acc)) {
            throw new IllegalStateException("bridge " + bridge.name + " produced non-finite values: " + acc);
        }
        bridge.paritySum = acc;
    }

    // ---- raw native (no bridge) --------------------------------------------

    /**
     * Direct nativeNoise with prebuilt handles: 1 thread then 2 threads,
     * identical work per thread. No bridge, no map, no cache — pure closed-
     * kernel behavior under concurrency.
     */
    static void rawMt2() throws Exception {
        byte[] perm = permutation();
        ImprovedNoise[] noises = new ImprovedNoise[2];
        for (int i = 0; i < 2; i++) noises[i] = new ImprovedNoise();
        // Prebuild two independent handles directly via the native symbol —
        // both bridge variants use exactly the same native build path, so
        // this bypasses no logic the A/B depends on.
        long[] handles = new long[2];
        for (int i = 0; i < 2; i++) {
            handles[i] = PaperNativeImprovedNoise.nativeBuildHandle(perm, 0, 0, 0);
        }
        int innerOps = 1 << 20; // native is fast; fixed large batch
        double single = rawRun(handles, noises, 1, innerOps);
        double multi = rawRun(handles, noises, 2, innerOps);
        System.out.printf(Locale.ROOT, "    [rawMt2] nativeNoise 1 thread: %.1f ns/op; 2 threads: %.1f ns/op (per-op wall); ratio %.2fx%n",
                single, multi, multi / single);
        if (multi > single * 1.35) {
            System.out.println("    [rawMt2] => the closed kernel ITSELF serializes under concurrency: bridge-level mt deltas are second-order.");
        } else {
            System.out.println("    [rawMt2] => the closed kernel scales across threads: bridge-level mt deltas are bridge-owned.");
        }
        for (int i = 0; i < 2; i++) PaperNativeImprovedNoise.nativeFreeHandle(handles[i]);
    }

    static double rawRun(long[] handles, ImprovedNoise[] noises, int threads, int innerOps) throws Exception {
        if (threads == 1) {
            long t0 = System.nanoTime();
            double x = 0.5;
            for (int i = 0; i < innerOps; i++) {
                x += PaperNativeImprovedNoise.nativeNoise(handles[i & 1], x, i * 0.125, i * 0.25, 1.0, 1.0);
            }
            SINK = x;
            return (double) (System.nanoTime() - t0) / innerOps;
        }
        Thread[] ts = new Thread[threads];
        CountDownLatch start = new CountDownLatch(1);
        CountDownLatch done = new CountDownLatch(threads);
        long t0 = System.nanoTime();
        for (int t = 0; t < threads; t++) {
            final int tid = t;
            ts[t] = new Thread(() -> {
                try {
                    start.await();
                    double x = 0.5 + tid * 0.31;
                    int h = tid & 1;
                    for (int i = 0; i < innerOps; i++) {
                        x += PaperNativeImprovedNoise.nativeNoise(handles[h], x, i * 0.125, i * 0.25, 1.0, 1.0);
                    }
                    SINK = x;
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    done.countDown();
                }
            }, "raw-" + tid);
            ts[t].start();
        }
        start.countDown();
        done.await();
        return (double) (System.nanoTime() - t0) / ((long) threads * innerOps);
    }

    // ---- churn ---------------------------------------------------------------

    static void churn(Bridge[] bridges) {
        for (Bridge bridge : bridges) {
            long t0 = System.nanoTime();
            for (int round = 0; round < 3; round++) {
                ImprovedNoise[] batch = new ImprovedNoise[50_000];
                byte[] perm = permutation();
                for (int i = 0; i < batch.length; i++) {
                    batch[i] = new ImprovedNoise();
                    bridge.sample(batch[i], i * 0.5, i * 0.25, i * 0.125);
                }
                batch = null; // all 50k become garbage with live native handles
                System.gc();
                try { Thread.sleep(200); } catch (InterruptedException e) { Thread.currentThread().interrupt(); }
            }
            long dt = System.nanoTime() - t0;
            System.out.printf(Locale.ROOT, "    [churn] %-34s 3x50k build+drop: %d ms (smoke: no crash, no double free)%n",
                    bridge.name, dt / 1_000_000L);
        }
    }

    static byte[] permutation() {
        byte[] p = new byte[256];
        for (int i = 0; i < 256; i++) p[i] = (byte) i;
        // deterministic xorshift shuffle
        long s = 0x9E3779B97F4A7C15L;
        for (int i = 255; i > 0; i--) {
            s ^= s << 13; s ^= s >>> 7; s ^= s << 17;
            int j = (int) ((s >>> 33) % (i + 1));
            byte t = p[i]; p[i] = p[j]; p[j] = t;
        }
        return p;
    }
}
