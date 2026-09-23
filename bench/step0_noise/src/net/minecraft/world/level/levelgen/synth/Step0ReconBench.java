package net.minecraft.world.level.levelgen.synth;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.util.RandomSource;

/**
 * G-RECON (TASK-69) — owner-cost measurement arm. Pure Java, NO natives:
 * measures the REAL production owner bodies from the deployed patched jar —
 * PerlinNoise.getValue (the octave-loop owner; internally calls the 5-arg
 * ImprovedNoise.noise with yScale=0,yMax=0 per octave), NormalNoise.getValue
 * (two PerlinNoise trees), and the production-corner octave sample
 * (ImprovedNoise.noise(x,y,z,0,0)). Effective N_o and owner-loop overhead
 * are derived from these against the TASK-67 raw-sample numbers.
 *
 * <p>Same P500 hygiene: 120 ms batches, median-of-5, fwd/rev,
 * min-of-medians, SINK, shared coordinate pool. No parity gate needed here —
 * there is no native counterpart in this run (the whole-object native ABI
 * decode is a follow-up probe; see the recon report).
 */
public final class Step0ReconBench {
    private Step0ReconBench() {}

    private static double SINK = 0.0;

    private interface Op { long unit(int i); }

    public static void main(String[] args) {
        final ImprovedNoise in = new ImprovedNoise(RandomSource.create(12345L));

        // vanilla-terrain-like octave set: firstOctave -3, flat amplitudes
        final PerlinNoise perlin = PerlinNoise.create(
            RandomSource.create(777L), -3,
            DoubleArrayList.wrap(new double[]{1,1,1,1,1,1,1,1}));
        final NormalNoise normal = NormalNoise.create(
            RandomSource.create(999L), -3, 1,1,1,1,1,1,1,1);

        final int nOct;
        try {
            java.lang.reflect.Field f = PerlinNoise.class.getDeclaredField("noiseLevels");
            f.setAccessible(true);
            nOct = ((ImprovedNoise[]) f.get(perlin)).length;
        } catch (Exception e) { throw new RuntimeException(e); }
        System.out.println("recon config: PerlinNoise noiseLevels.length=" + nOct);

        final int POOL = 8192;
        final double[] xs = new double[POOL], ys = new double[POOL], zs = new double[POOL];
        java.util.Random r = new java.util.Random(777L);
        for (int i = 0; i < POOL; i++) {
            xs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys[i] = -64.0 + r.nextDouble() * 384.0;
            zs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
        }

        java.util.List<String> names = new java.util.ArrayList<>();
        java.util.List<Op> ops = new java.util.ArrayList<>();
        names.add("P_getValue3_perlin"); ops.add((i) -> { SINK += perlin.getValue(xs[i], ys[i], zs[i]); return 1; });
        names.add("NN_getValue3_normal"); ops.add((i) -> { SINK += normal.getValue(xs[i], ys[i], zs[i]); return 1; });
        names.add("OCT_corner_noise00"); ops.add((i) -> { SINK += in.noise(xs[i], ys[i], zs[i], 0.0, 0.0); return 1; });
        names.add("OCT_raw_noise3"); ops.add((i) -> { SINK += in.noise(xs[i], ys[i], zs[i]); return 1; });

        final long TARGET_NS = 120_000_000L;
        int n = names.size();
        double[][] fwd = new double[n][], rev = new double[n][];
        for (int a = 0; a < n; a++) fwd[a] = arm(names.get(a), ops.get(a), POOL, TARGET_NS);
        for (int a = n - 1; a >= 0; a--) rev[a] = arm("[rev]" + names.get(a), ops.get(a), POOL, TARGET_NS);

        System.out.println("== G-RECON owner costs (ns/call, min-of-medians) ==");
        StringBuilder csv = new StringBuilder("arm,ns_call\n");
        double[] best = new double[n];
        for (int a = 0; a < n; a++) {
            best[a] = Math.min(median(fwd[a]), median(rev[a]));
            System.out.printf("%-24s %10.1f ns/call%n", names.get(a), best[a]);
            csv.append(names.get(a)).append(',').append(String.format("%.1f", best[a])).append('\n');
        }
        double octCorner = best[2], octRaw = best[3];
        System.out.printf("%n derivation:%n");
        System.out.printf("  effective N_o (corner / raw)  = %.1f ns/call ÷ %.1f = %.2f octaves-equivalent%n", best[0], octCorner, best[0] / octCorner);
        System.out.printf("  owner-loop overhead share     = 1 - %.1f/(%.1f*%.1f) = %.1f%%%n", best[0], octCorner, (double) nOct, 100.0 * (1.0 - best[0] / (octCorner * nOct)));
        System.out.printf("  NormalNoise vs 2x PerlinNoise = %.1f vs %.1f (tree-loop overhead %.1f%%)%n",
            best[1], 2 * best[0], 100.0 * (best[1] / (2 * best[0]) - 1.0));
        System.out.println("--BEGIN_CSV--");
        System.out.print(csv);
        System.out.println("--END_CSV--");
        System.out.println("sink=" + SINK + " nOct=" + nOct);
    }

    private static double[] arm(String name, Op op, int pool, long targetNs) {
        double[] meds = new double[5];
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
            System.out.printf("  %-24s rep%d: %d ops in %.1f ms -> %.1f ns/op%n",
                name, rep, opsDone, dt / 1e6, meds[rep]);
        }
        return meds;
    }

    private static double median(double[] a) {
        double[] c = a.clone();
        java.util.Arrays.sort(c);
        return c[c.length / 2];
    }
}
