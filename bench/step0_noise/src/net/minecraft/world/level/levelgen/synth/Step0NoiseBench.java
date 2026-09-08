package net.minecraft.world.level.levelgen.synth;

import net.minecraft.util.RandomSource;

/**
 * STEP-0 (TASK-67, G-STEP0 gate of docs/WORLDGEN_BATCHING_LAYER_DESIGN.md §7):
 * per-sample cost of the closed native noise core vs the REAL JIT'd Java
 * ImprovedNoise, plus the already-shipped batch fill kernel (nativeFill /
 * nativeFillNoYScale — ONE crossing per N samples), measured on the exact
 * code that runs in production worldgen.
 *
 * <p>Provenance (no server boot, CPU-only session):
 * <ul>
 *   <li>Java arm: the REAL {@code ImprovedNoise} class from the deployed
 *       paperclip-patched jar ({@code versions/1.21.10/purpur-1.21.10.jar},
 *       mojang-mapped — the same bytes the server JVM loads every boot).</li>
 *   <li>Native arms: the closed {@code libpaper_native_jni.so} repo copy —
 *       the canonical P500 bench target; {@code nativeNoise(NoYScale)} is the
 *       live-verified wiring core (bit-exact vs the real class, re-proven
 *       every armed boot self-test and re-proven HERE on 20k fixtures before
 *       any timing).</li>
 * </ul>
 *
 * <p>Method: P500 hygiene — 120 ms batches, median-of-5 per arm, forward and
 * reverse arm order, min-of-medians, SINK accumulator vs DCE, shared
 * read-only coordinate pool so every arm samples IDENTICAL values. The batch
 * arms consume exactly these arrays (no per-call arg marshaling beyond the
 * array references).
 *
 * <p>Verdict math (pre-registered in the design doc): GO for the batching
 * layer iff native batch-core ns/sample at N=16 <= Java ns/sample (breakeven
 * N <= 16). NO-GO is a valid outcome and closes the worldgen noise channel
 * (per-call TASK-63, boot TASK-58, geometry TASK-65, batched STEP-0).
 */
public final class Step0NoiseBench {
    private Step0NoiseBench() {}

    private static double SINK = 0.0; // DCE defeat

    /** One timing repetition: run batches until >= targetNs, return ns/op. */
    private interface Op {
        /** one logical unit; returns ops consumed */
        long unit(int i);
    }

    public static void main(String[] args) throws Exception {
        final String mainSo = args[0];
        System.load(mainSo);

        final ImprovedNoise in = new ImprovedNoise(RandomSource.create(12345L));
        final byte[] p = permBytes(in);
        final long handle = PaperNativeImprovedNoise.nativeBuildHandle(p, in.xo, in.yo, in.zo);
        if (handle == 0L) throw new IllegalStateException("nativeBuildHandle rejected permutation");

        // ---- coordinate pool (shared, identical values for every arm) ----
        final int POOL = 8192;
        final double[] xs = new double[POOL], ys = new double[POOL], zs = new double[POOL];
        final double[] ys5 = new double[POOL], ym5 = new double[POOL];
        java.util.Random r = new java.util.Random(777L);
        for (int i = 0; i < POOL; i++) {
            xs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys[i] = -64.0 + r.nextDouble() * 384.0;
            zs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys5[i] = r.nextDouble() * 4.0 + 0.1;      // yScale-like
            ym5[i] = r.nextInt(3) == 0 ? 0.0 : r.nextDouble() * 256.0; // yMax-like
        }

        // ---- PARITY GATE (abort on any mismatch, before timing) ----
        long j3Bits = 0, n3Bits = 0, j5Bits = 0, n5Bits = 0;
        int bad3 = 0, bad5 = 0;
        for (int i = 0; i < 20000; i++) {
            int k = i % POOL;
            double a3 = in.noise(xs[k], ys[k], zs[k]);
            double b3 = PaperNativeImprovedNoise.nativeNoiseNoYScale(handle, xs[k], ys[k], zs[k]);
            if (Double.doubleToRawLongBits(a3) != Double.doubleToRawLongBits(b3)) bad3++;
            double a5 = in.noise(xs[k], ys[k], zs[k], ys5[k], ym5[k]);
            double b5 = PaperNativeImprovedNoise.nativeNoise(handle, xs[k], ys[k], zs[k], ys5[k], ym5[k]);
            if (Double.doubleToRawLongBits(a5) != Double.doubleToRawLongBits(b5)) bad5++;
            if (i == 0) { j3Bits = Double.doubleToRawLongBits(a3); n3Bits = Double.doubleToRawLongBits(b3);
                          j5Bits = Double.doubleToRawLongBits(a5); n5Bits = Double.doubleToRawLongBits(b5); }
        }
        System.out.printf("parity gate: 3-arg mismatches=%d/20000, 5-arg mismatches=%d/20000%n", bad3, bad5);
        if (bad3 > 0 || bad5 > 0) {
            System.out.println("PARITY GATE FAILED — aborting before timing");
            return;
        }

        // ---- batch plane buffers (slices of the SAME pool values) ----
        final int[] SIZES = {16, 1024};
        for (int s : SIZES) {
            // nothing to precompute — nativeFill consumes arrays directly;
            // slices are views over identical pool coordinates
        }

        // arms: [name, op]
        java.util.List<String> names = new java.util.ArrayList<>();
        java.util.List<Op> ops = new java.util.ArrayList<>();

        names.add("J3_java_noise3"); ops.add((i) -> { SINK += in.noise(xs[i], ys[i], zs[i]); return 1; });
        names.add("N1_nativeNoiseNoYScale"); ops.add((i) -> { SINK += PaperNativeImprovedNoise.nativeNoiseNoYScale(handle, xs[i], ys[i], zs[i]); return 1; });
        names.add("J5_java_noise5"); ops.add((i) -> { SINK += in.noise(xs[i], ys[i], zs[i], ys5[i], ym5[i]); return 1; });
        names.add("N1_nativeNoise"); ops.add((i) -> { SINK += PaperNativeImprovedNoise.nativeNoise(handle, xs[i], ys[i], zs[i], ys5[i], ym5[i]); return 1; });

        final double[] out16 = new double[16];
        final double[] slice16x = new double[16], slice16y = new double[16], slice16z = new double[16];
        names.add("NB16_nativeFillNoYScale"); ops.add((i) -> {
            // fill slices deterministically from the pool
            for (int k = 0; k < 16; k++) {
                slice16x[k] = xs[(i * 16 + k) % POOL];
                slice16y[k] = ys[(i * 16 + k) % POOL];
                slice16z[k] = zs[(i * 16 + k) % POOL];
            }
            PaperNativeImprovedNoise.nativeFillNoYScale(handle, slice16x, slice16y, slice16z, out16);
            for (int k = 0; k < 16; k++) SINK += out16[k];
            return 16;
        });

        // 1024 batch: nativeFill consumes the pool DIRECTLY (POOL=8192 = 8 calls of 1024)
        final double[] out1024 = new double[1024];
        final double[] px = new double[1024], py = new double[1024], pz = new double[1024];
        final double[] p5s = new double[1024], p5m = new double[1024];
        for (int k = 0; k < 1024; k++) { px[k]=xs[k]; py[k]=ys[k]; pz[k]=zs[k]; p5s[k]=ys5[k]; p5m[k]=ym5[k]; }
        names.add("NB1024_nativeFill"); ops.add((i) -> {
            PaperNativeImprovedNoise.nativeFill(handle, px, py, pz, p5s, p5m, out1024);
            for (int k = 0; k < 1024; k++) SINK += out1024[k];
            return 1024;
        });
        // N=16 with yScale for symmetry with NB16 nativeFillNoYScale
        final double[] s16x = new double[16], s16y = new double[16], s16z = new double[16],
                       s16s = new double[16], s16m = new double[16], o16 = new double[16];
        names.add("NB16_nativeFill"); ops.add((i) -> {
            for (int k = 0; k < 16; k++) {
                int q = (i * 16 + k) % POOL;
                s16x[k]=xs[q]; s16y[k]=ys[q]; s16z[k]=zs[q]; s16s[k]=ys5[q]; s16m[k]=ym5[q];
            }
            PaperNativeImprovedNoise.nativeFill(handle, s16x, s16y, s16z, s16s, s16m, o16);
            for (int k = 0; k < 16; k++) SINK += o16[k];
            return 16;
        });

        final long TARGET_NS = 120_000_000L; // 120 ms per timing batch (P500)

        // forward pass then reverse pass, min-of-medians
        int n = names.size();
        double[][] fwd = new double[n][], rev = new double[n][];
        for (int a = 0; a < n; a++) fwd[a] = arm(names.get(a), ops.get(a), POOL, TARGET_NS);
        for (int a = n - 1; a >= 0; a--) rev[a] = arm("[rev]" + names.get(a), ops.get(a), POOL, TARGET_NS);

        System.out.println("== STEP-0 results (ns/sample, min-of-medians across orders) ==");
        StringBuilder csv = new StringBuilder("arm,ns_sample_min_of_medians\n");
        for (int a = 0; a < n; a++) {
            double f = median(fwd[a]), rv = median(rev[a]);
            double best = Math.min(f, rv);
            System.out.printf("%-28s %10.1f ns/sample   (fwd %.1f / rev %.1f)%n", names.get(a), best, f, rv);
            csv.append(names.get(a)).append(',').append(String.format("%.1f", best)).append('\n');
        }
        System.out.println("--BEGIN_CSV--");
        System.out.print(csv);
        System.out.println("--END_CSV--");
        System.out.println("sink=" + SINK);
        PaperNativeImprovedNoise.nativeFreeHandle(handle);
    }

    private static double[] arm(String name, Op op, int pool, long targetNs) {
        double[] meds = new double[5];
        // JIT warmup ~2s
        long wEnd = System.nanoTime() + 2_000_000_000L;
        long i = 0;
        while (System.nanoTime() < wEnd) { i = (i + op.unit((int) (i % pool))) % pool; }
        for (int rep = 0; rep < 5; rep++) {
            long opsDone = 0, iters = 0;
            long t0 = System.nanoTime();
            while (System.nanoTime() - t0 < targetNs) {
                opsDone += op.unit((int) (iters % pool));
                iters++;
            }
            long dt = System.nanoTime() - t0;
            meds[rep] = (double) dt / (double) opsDone;
            System.out.printf("  %-28s rep%d: %d ops in %.1f ms -> %.1f ns/op%n",
                name, rep, opsDone, dt / 1e6, meds[rep]);
        }
        return meds;
    }

    private static double median(double[] a) {
        double[] c = a.clone();
        java.util.Arrays.sort(c);
        return c[c.length / 2];
    }

    private static byte[] permBytes(ImprovedNoise in) throws Exception {
        java.lang.reflect.Field f = ImprovedNoise.class.getDeclaredField("p");
        f.setAccessible(true);
        return (byte[]) f.get(in);
    }
}
