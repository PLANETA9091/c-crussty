package net.minecraft.world.level.levelgen.synth;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.util.RandomSource;

/**
 * G-NORMAL (TASK-79, Session-0) — decode the heritage whole-Noise
 * composition kernel PaperNativeNormalNoise.nativeGetValue(JJDDDD)D of the
 * closed lib.
 *
 * Structure mirrors Step0AbiBench (TASK-70 discipline):
 *
 * Phase A — candidate sweep, bit-exact parity vs the REAL NormalNoise
 * object (NormalNoise.getValue(DDD)D, unpatched in this rig). The kernel
 * receives the two PROVEN PerlinNoise handles (TASK-70 winner pack P3/D1/F1
 * built from the object's actual first/second fields) + x/y/z + one fourth
 * double. Candidates (declared order, first full pass wins):
 *   C1: kernel applies INPUT_FACTOR internally, fourth=valueFactor
 *       (the shape of the Java body: (A(x,y,z)+B(xF,yF,zF))*vf)
 *   C2: same handles swapped
 *   C3: fourth=1.0 fixed (vf baked into kernel or absent)
 * Sweep uses 512 samples x valueFactor variations via TWO objects with
 * different valueFactor values.
 *
 * Phase B — full parity on the winner: 20000 samples on noise A, 20000 on
 * noise B (different octave configs + valueFactors).
 *
 * Phase C — P500 old-vs-optimized, THREE arms (production-honest):
 *   J  : NormalNoise.getValue(DDD)D pure Java (pristine kernel body)
 *   N2 : armed-today shape — 2 native PerlinNoise crossings + Java glue
 *        (what TASK-74's bridge already delivers underneath NormalNoise)
 *   N1 : TASK-79 target — ONE native crossing (whole-Noise kernel)
 * min-of-medians over fwd/rev, 120 ms batches, SINK, shared pool.
 *
 * CPU-only, no server, no .so deploy. One run per invocation; BENCH.lock
 * held by the caller.
 */
public final class Step0NormalBench {
    private Step0NormalBench() {}

    private static double SINK = 0.0;

    private interface Op { long unit(int i); }

    /** Build a PerlinNoise handle from the real object with the TASK-70
     *  winner pack (P3/D1/F1: slot-indexed concat p-tables + presence mask
     *  + xo/yo/zo + amplitudes + inF,valF). Returns 0 on any failure. */
    private static long buildHandle(PerlinNoise pn) {
        try {
            java.lang.reflect.Field fN = PerlinNoise.class.getDeclaredField("noiseLevels");
            fN.setAccessible(true);
            ImprovedNoise[] lv = (ImprovedNoise[]) fN.get(pn);
            final int n = lv.length;
            byte[] a0 = new byte[256 * n];
            byte[] a1 = new byte[n];
            double[] xo = new double[n], yo = new double[n], zo = new double[n];
            java.lang.reflect.Field fp = ImprovedNoise.class.getDeclaredField("p");
            fp.setAccessible(true);
            for (int i = 0; i < n; i++) {
                if (lv[i] != null) {
                    byte[] p = (byte[]) fp.get(lv[i]);
                    System.arraycopy(p, 0, a0, 256 * i, 256);
                    a1[i] = 1;
                    xo[i] = lv[i].xo; yo[i] = lv[i].yo; zo[i] = lv[i].zo;
                }
            }
            java.lang.reflect.Field fA = PerlinNoise.class.getDeclaredField("amplitudes");
            fA.setAccessible(true);
            double[] amps = ((it.unimi.dsi.fastutil.doubles.DoubleList) fA.get(pn)).toDoubleArray();
            java.lang.reflect.Field fI = PerlinNoise.class.getDeclaredField("lowestFreqInputFactor");
            fI.setAccessible(true);
            double inF = (Double) fI.get(pn);
            java.lang.reflect.Field fV = PerlinNoise.class.getDeclaredField("lowestFreqValueFactor");
            fV.setAccessible(true);
            double valF = (Double) fV.get(pn);
            return PaperNativePerlinNoise.nativeBuildHandle(a0, a1, xo, yo, zo, amps, inF, valF);
        } catch (Throwable t) {
            System.out.println("  handle build FAILED: " + t);
            return 0L;
        }
    }

    private static double valueFactor(NormalNoise nn) {
        try {
            java.lang.reflect.Field f = NormalNoise.class.getDeclaredField("valueFactor");
            f.setAccessible(true);
            return (Double) f.get(nn);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    /** Bit-exact parity of the composition kernel vs the real object. */
    private static int parity(long h1, long h2, NormalNoise nn, double vf,
                              double[] xs, double[] ys, double[] zs, int n,
                              java.util.Random rnd) {
        int mism = 0;
        for (int i = 0; i < n; i++) {
            int j = rnd.nextInt(xs.length);
            double jv = nn.getValue(xs[j], ys[j], zs[j]);
            double nv = PaperNativeNormalNoise.nativeGetValue(h1, h2, xs[j], ys[j], zs[j], vf);
            if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) mism++;
        }
        return mism;
    }

    private static long HA, HB; // native perlin handles of (first, second) of noise A
    private static double F = 1.0181268882175227d;

    private static double pA(double x, double y, double z) {
        return PaperNativePerlinNoise.nativeGetValue(HA, x, y, z, 0.0, 0.0, false);
    }
    private static double pB(double x, double y, double z) {
        return PaperNativePerlinNoise.nativeGetValue(HB, x, y, z, 0.0, 0.0, false);
    }

    /** Composition formula hypothesis: f(a0, b0, aF, bF, q) where a0/b0 =
     *  raw native samples at (x,y,z), aF/bF = samples at INPUT_FACTOR-scaled
     *  coords, q = the 4th double. Bit-exact sweep decides. */
    @FunctionalInterface
    private interface Formula { double f(double a0, double b0, double aF, double bF, double q); }

    public static void main(String[] args) throws Exception {
        System.load(args[0]);
        System.out.println("lib loaded: " + args[0]);
        System.out.println("nativeCheck()=" + PaperNativeNormalNoise.nativeCheck());

        // Two real NormalNoise objects with different shapes/factors
        // (create(RandomSource,int,double...) = the climate-noise factory).
        final NormalNoise na = NormalNoise.create(
            RandomSource.create(777L), -3, 1, 1, 1, 1, 1, 1, 1, 1);
        final NormalNoise nb = NormalNoise.create(
            RandomSource.create(4242L), 0, 1, 1, 1, 1);
        final double vfa = valueFactor(na);
        final double vfb = valueFactor(nb);
        System.out.printf("noise A: vf=%.6g  noise B: vf=%.6g%n", vfa, vfb);
        System.out.printf("INPUT_FACTOR=%.16f%n", 1.0181268882175227d);

        // Proven PerlinNoise handles of the real first/second fields.
        java.lang.reflect.Field f1 = NormalNoise.class.getDeclaredField("first");
        f1.setAccessible(true);
        java.lang.reflect.Field f2 = NormalNoise.class.getDeclaredField("second");
        f2.setAccessible(true);
        final PerlinNoise pa = (PerlinNoise) f1.get(na);
        final PerlinNoise pb = (PerlinNoise) f2.get(na);
        final long ha = buildHandle(pa);
        final long hb = buildHandle(pb);
        if (ha == 0L || hb == 0L) {
            System.out.println("VERDICT: G-NORMAL NO-DECODE (inner handle build failed)");
            return;
        }
        HA = ha; HB = hb;

        // Shared coordinate pool (production-like ranges).
        final int POOL = 8192;
        final double[] xs = new double[POOL], ys = new double[POOL], zs = new double[POOL];
        java.util.Random r = new java.util.Random(777L);
        for (int i = 0; i < POOL; i++) {
            xs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys[i] = -64.0 + r.nextDouble() * 384.0;
            zs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
        }

        // ---- Phase A0: raw shape probe on 3 coords (manual analysis aid) ----
        System.out.println("== RAW SHAPE PROBE ==");
        for (int j = 0; j < 3; j++) {
            double x = xs[j], y = ys[j], z = zs[j];
            double a0 = pA(x, y, z), b0 = pB(x, y, z);
            double aF = pA(x * F, y * F, z * F), bF = pB(x * F, y * F, z * F);
            double jv = na.getValue(x, y, z);
            System.out.printf("coord %d: J=%.17g a0=%.17g b0=%.17g aF=%.17g bF=%.17g%n",
                j, jv, a0, b0, aF, bF);
            for (double q : new double[]{0.0, 1.0, vfa, 2.0}) {
                double nv = PaperNativeNormalNoise.nativeGetValue(ha, hb, x, y, z, q);
                System.out.printf("   q=%.4g native=%.17g%n", q, nv);
            }
        }

        // ---- Phase A: formula sweep (bit-exact, 512 samples) ----
        record Cand(String id, long h1, long h2, Formula f, double qScale) {}
        final java.util.List<Cand> cands = new java.util.ArrayList<>();
        // (A,B) and (B,A) orders x formula family
        for (long[] pair : new long[][]{{ha, hb}, {hb, ha}}) {
            final boolean ab = pair[0] == ha;
            final String ord = ab ? "(A,B)" : "(B,A)";
            cands.add(new Cand("F1_mulQ" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (a0 + bF) * q, 0));
            cands.add(new Cand("F2_noF" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (a0 + b0) * q, 0));
            cands.add(new Cand("F3_fOnA" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (aF + b0) * q, 0));
            cands.add(new Cand("F4_aQplusB" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> a0 * q + bF, 0));
            cands.add(new Cand("F5_divQ" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (a0 + bF) / q, 0));
            cands.add(new Cand("F6_bQplusA" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> b0 * q + aF, 0));
            cands.add(new Cand("F7_sub" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (a0 - bF) * q, 0));
            cands.add(new Cand("F8_bothF" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> (aF + bF) * q, 0));
            cands.add(new Cand("F9_qTimesBoth" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> q * a0 + q * bF, 0));
            cands.add(new Cand("F10_sumQ" + ord, pair[0], pair[1],
                (a0, b0, aF, bF, q) -> a0 + bF + q, 0));
        }
        Cand winner = null;
        int winMism = -1;
        StringBuilder csv = new StringBuilder("candidate,mism_512\n");
        for (Cand c : cands) {
            int mism;
            try {
                mism = 0;
                java.util.Random pr = new java.util.Random(1L);
                for (int i = 0; i < 512 && mism == 0; i++) {
                    int j = pr.nextInt(xs.length);
                    double x = xs[j], y = ys[j], z = zs[j];
                    double q = c.qScale() != 0 ? c.qScale() : vfa;
                    double jv = na.getValue(x, y, z);
                    double nv = c.f().f(pA(x, y, z), pB(x, y, z),
                                        pA(x * F, y * F, z * F), pB(x * F, y * F, z * F), q);
                    if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) mism++;
                }
            } catch (Throwable t) {
                mism = -2;
            }
            System.out.printf("candidate %-20s mism=%d%n", c.id(), mism);
            csv.append(c.id()).append(',').append(mism).append('\n');
            if (mism == 0 && winner == null) winner = c;
        }
        if (winner == null) {
            System.out.println("--BEGIN_CSV--\n" + csv + "--END_CSV--");
            System.out.println("VERDICT: G-NORMAL NO-DECODE (no formula matched; kernel semantics need hypothesis sweep-3)");
            return;
        }
        System.out.println("WINNER: " + winner.id());

        // ---- Phase B: full parity on winner (both objects) ----
        // Formula-based parity: compose from raw native perlin samples exactly
        // as the winning hypothesis does, compare bit-exact vs the Java object.
        final Cand w = winner;
        java.util.Random rb = new java.util.Random(2L);
        int fullA = 0;
        for (int i = 0; i < 20000; i++) {
            int j = rb.nextInt(xs.length);
            double x = xs[j], y = ys[j], z = zs[j];
            double jv = na.getValue(x, y, z);
            double nv = w.f().f(pA(x, y, z), pB(x, y, z),
                                pA(x * F, y * F, z * F), pB(x * F, y * F, z * F), vfa);
            if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) fullA++;
        }
        // noise B: rebuild handles for its first/second, swap HA/HB, restore.
        final long hb1 = buildHandle((PerlinNoise) f1.get(nb));
        final long hb2 = buildHandle((PerlinNoise) f2.get(nb));
        int fullB = -1;
        if (hb1 != 0L && hb2 != 0L) {
            final long sHA = HA, sHB = HB;
            HA = hb1; HB = hb2;
            final double vfb2 = vfb;
            java.util.Random rb2 = new java.util.Random(3L);
            fullB = 0;
            for (int i = 0; i < 20000; i++) {
                int j = rb2.nextInt(xs.length);
                double x = xs[j], y = ys[j], z = zs[j];
                double jv = nb.getValue(x, y, z);
                double nv = w.f().f(pA(x, y, z), pB(x, y, z),
                                    pA(x * F, y * F, z * F), pB(x * F, y * F, z * F), vfb2);
                if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) fullB++;
            }
            HA = sHA; HB = sHB;
        }
        System.out.printf("FULL PARITY: A=%d/20000 B=%d/20000%n", fullA, fullB);
        csv.append("full_A,").append(fullA).append('\n');
        csv.append("full_B,").append(fullB).append('\n');

        // ---- Phase C: P500 timing, three arms ----
        final double vf = vfa;
        final double F = 1.0181268882175227d;
        final Cand win = winner;
        java.util.List<String> tn = new java.util.ArrayList<>();
        java.util.List<Op> ops = new java.util.ArrayList<>();
        // J: pristine kernel body (pure Java octave loops under the hood)
        tn.add("J_java_body");
        ops.add((i) -> { SINK += na.getValue(xs[i], ys[i], zs[i]); return 1; });
        // N2: armed-today shape = 2 whole-PerlinNoise crossings + Java glue
        tn.add("N2_2x_crossing");
        ops.add((i) -> {
            SINK += (PaperNativePerlinNoise.nativeGetValue(ha, xs[i], ys[i], zs[i], 0.0, 0.0, false)
                   + PaperNativePerlinNoise.nativeGetValue(hb, xs[i] * F, ys[i] * F, zs[i] * F, 0.0, 0.0, false)) * vf;
            return 1;
        });
        // N1: TASK-79 target = ONE crossing through the composition kernel
        tn.add("N1_1x_compose");
        ops.add((i) -> {
            SINK += PaperNativeNormalNoise.nativeGetValue(
                win.h1(), win.h2(), xs[i], ys[i], zs[i], vfa);
            return 1;
        });
        final long TARGET_NS = 120_000_000L;
        int n = tn.size();
        double[][] fwd = new double[n][], rev = new double[n][];
        for (int a = 0; a < n; a++) fwd[a] = arm(tn.get(a), ops.get(a), POOL, TARGET_NS);
        for (int a = n - 1; a >= 0; a--) rev[a] = arm("[rev]" + tn.get(a), ops.get(a), POOL, TARGET_NS);
        System.out.println("== G-NORMAL timing (ns/call, min-of-medians) ==");
        double[] best = new double[n];
        for (int a = 0; a < n; a++) {
            best[a] = Math.min(median(fwd[a]), median(rev[a]));
            System.out.printf("%-24s %10.1f ns/call%n", tn.get(a), best[a]);
            csv.append(tn.get(a)).append(',').append(String.format("%.1f", best[a])).append('\n');
        }
        System.out.printf("N1/J (kernel pair) = %.3fx%n", best[2] / best[0]);
        System.out.printf("N1/N2 (marginal vs armed-today) = %.3fx  (-%.1f ns/call)%n",
            best[2] / best[1], best[1] - best[2]);
        csv.append("N1_over_J,").append(String.format("%.3f", best[2] / best[0])).append('\n');
        csv.append("N1_over_N2,").append(String.format("%.3f", best[2] / best[1])).append('\n');

        PaperNativePerlinNoise.nativeFreeHandle(ha);
        PaperNativePerlinNoise.nativeFreeHandle(hb);
        if (hb1 != 0L) PaperNativePerlinNoise.nativeFreeHandle(hb1);
        if (hb2 != 0L) PaperNativePerlinNoise.nativeFreeHandle(hb2);
        System.out.println("--BEGIN_CSV--");
        System.out.print(csv);
        System.out.println("--END_CSV--");
        System.out.println("sink=" + SINK + " winner=" + winner.id());
        System.out.println("VERDICT: G-NORMAL " + (fullA == 0 && fullB == 0 ? "GO" : "NO-GO"));
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
