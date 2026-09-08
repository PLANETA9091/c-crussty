package net.minecraft.world.level.levelgen.synth;

import java.util.Arrays;

/**
 * G-ABI-2 (TASK-108) — decode the heritage NormalNoise BATCH FILL family of
 * the closed lib via parity-driven candidate sweep (TASK-79 discipline):
 *
 *   nativeFillPositions(JJD[D[D[D[D)I        (h1, h2, D?, x[], y[], z[], out[])
 *   nativeFillScaledPositions(JJD[I[I[IDD[D)I (h1, h2, D?, bx[], by[], bz[], D, D, out[])
 *   nativeFillShiftA(JJD[I[I[D)I             (h1, h2, D?, bx[], bz[], out[])
 *   nativeFillShiftB(JJD[I[I[D)I             (h1, h2, D?, bz[]?, bx[]?, out[])
 *
 * Oracle: the REAL 1.21.10 kernel classes (unpatched, plain classpath JVM):
 *   NormalNoise.getValue = (first.getValue(x,y,z)
 *                           + second.getValue(x*IF, y*IF, z*IF)) * valueFactor
 *   with IF = 1.0181268882175227 (bytecode-verified constant).
 * Handles: TASK-70 winner pack (P3/D1/F1) built from the objects' real fields
 * (Step0NormalBench.buildHandle verbatim).
 *
 * Known from G-NORMAL (TASK-79): nativeGetValue(JJDDDD)D 4th double =
 * valueFactor, IF baked in-kernel, bit-exact 0/20000. The fill family is
 * hypothesized to share that composition contract; the sweep localizes the
 * role of every arg and the exact per-element formula, including the
 * ShiftNoise-level 0.25/4.0 scaling question (baked vs caller-side).
 *
 * ShiftA (javap-decoded): ShiftNoise.compute(blockX, 0, blockZ)
 * ShiftB (javap-decoded): ShiftNoise.compute(blockZ, blockX, 0)  [x/z swapped!]
 * ShiftNoise.compute(x,y,z) = 4 * offsetNoise.getValue(x*0.25, y*0.25, z*0.25)
 *
 * CPU-only, no server, no .so deploy. One run per invocation; caller holds
 * BENCH.lock. Output: FILLABI lines (parse-friendly).
 */
public final class Step0NormalFillBench {
    private Step0NormalFillBench() {}

    static final double INPUT_FACTOR = 1.0181268882175227d;

    public static void main(String[] args) throws Exception {
        System.load(args[0]); // libpaper_native_jni.so (TASK-79 rig convention)
        final NormalNoise na = NormalNoise.create(
            net.minecraft.util.RandomSource.create(1234L), -3, 1.0, 1.0, 1.0, 1.0);
        final NormalNoise nb = NormalNoise.create(
            net.minecraft.util.RandomSource.create(777L), -2, 1.0, 1.5, 0.5);
        final double vfa = valueFactorOf(na), vfb = valueFactorOf(nb);
        final long ha1 = buildHandle(firstOf(na));
        final long ha2 = buildHandle(secondOf(na));
        final long hb1 = buildHandle(firstOf(nb));
        final long hb2 = buildHandle(secondOf(nb));
        System.out.println("FILLABI handles a=" + ha1 + "/" + ha2 + " b=" + hb1 + "/" + hb2
            + " vfa=" + vfa + " vfb=" + vfb);
        if (ha1 == 0 || ha2 == 0 || hb1 == 0 || hb2 == 0) {
            System.out.println("FILLABI ABORT handle-build");
            return;
        }

        // deterministic coordinate pools
        final int N = 64;
        final double[] dx = new double[N], dy = new double[N], dz = new double[N];
        final int[] bx = new int[N], by = new int[N], bz = new int[N];
        java.util.Random r = new java.util.Random(42);
        for (int i = 0; i < N; i++) {
            dx[i] = r.nextDouble() * 512 - 256;
            dy[i] = r.nextDouble() * 384 - 64;
            dz[i] = r.nextDouble() * 512 - 256;
            bx[i] = r.nextInt(4096) - 2048;
            by[i] = r.nextInt(384) - 64;
            bz[i] = r.nextInt(4096) - 2048;
        }

        sweepPositions("A", na, ha1, ha2, vfa, dx, dy, dz);
        sweepPositions("B", nb, hb1, hb2, vfb, dx, dy, dz);
        sweepScaled("A", na, ha1, ha2, vfa, bx, by, bz);
        sweepScaled("B", nb, hb1, hb2, vfb, bx, by, bz);
        sweepShift("A", na, ha1, ha2, vfa, bx, bz, true);
        sweepShift("B", nb, hb1, hb2, vfb, bx, bz, true);
        sweepShift("A-swap", na, ha1, ha2, vfa, bz, bx, false);
        sweepShift("B-swap", nb, hb1, hb2, vfb, bz, bx, false);
    }

    // ---------- per-native candidate sweeps ----------

    static void sweepPositions(String tag, NormalNoise ref, long h1, long h2,
                               double vf, double[] x, double[] y, double[] z) {
        final int n = x.length;
        double[] out = new double[n];
        double[] want = new double[n];
        for (int i = 0; i < n; i++) want[i] = ref.getValue(x[i], y[i], z[i]);
        // C1: d = valueFactor, raw coords
        Arrays.fill(out, Double.NaN);
        int rc = PaperNativeNormalNoise.nativeFillPositions(h1, h2, vf, x, y, z, out);
        rep("FillPositions", tag, "C1:4th=vf,raw", rc, out, want);
        // C2: d = 1.0
        Arrays.fill(out, Double.NaN);
        rc = PaperNativeNormalNoise.nativeFillPositions(h1, h2, 1.0, x, y, z, out);
        rep("FillPositions", tag, "C2:4th=1.0,raw", rc, out, want);
        // C3: d = INPUT_FACTOR
        Arrays.fill(out, Double.NaN);
        rc = PaperNativeNormalNoise.nativeFillPositions(h1, h2, INPUT_FACTOR, x, y, z, out);
        rep("FillPositions", tag, "C3:4th=IF,raw", rc, out, want);
    }

    static void sweepScaled(String tag, NormalNoise ref, long h1, long h2, double vf,
                            int[] bx, int[] by, int[] bz) {
        final int n = bx.length;
        double[] out = new double[n];
        // canonical DensityFunctions$Noise shape: (bx*sxz, by*sy, bz*sxz)
        for (double[] scales : new double[][]{{1.0, 1.0}, {0.25, 0.25}, {2.0, 1.0}, {0.125, 0.5}}) {
            double sxz = scales[0], sy = scales[1];
            double[] want = new double[n];
            for (int i = 0; i < n; i++) {
                want[i] = ref.getValue(bx[i] * sxz, by[i] * sy, bz[i] * sxz);
            }
            Arrays.fill(out, Double.NaN);
            int rc = PaperNativeNormalNoise.nativeFillScaledPositions(
                h1, h2, vf, bx, by, bz, sxz, sy, out);
            rep("FillScaled", tag, "C:vf,sxz=" + sxz + ",sy=" + sy, rc, out, want);
            if (parityCount(out, want) == n) return; // winner found; stop scaling variants
        }
    }

    /** kind=true: arr1 = blockX-side, arr2 = blockZ-side (ShiftA order);
     *  swap variant probes ShiftB's decoded x<->z swap. */
    static void sweepShift(String tag, NormalNoise ref, long h1, long h2, double vf,
                           int[] a1, int[] a2, boolean kind) {
        final int n = a1.length;
        double[] out = new double[n];
        // vanilla reference per javap:
        //   ShiftA: 4*ref.getValue(a1[i]*0.25, 0.0, a2[i]*0.25)
        //   ShiftB: 4*ref.getValue(a2[i]*0.25, a1[i]*0.25, 0.0)  [z->x, x->y]
        double[] wantA = new double[n], wantB = new double[n];
        for (int i = 0; i < n; i++) {
            wantA[i] = 4.0 * ref.getValue(a1[i] * 0.25, 0.0, a2[i] * 0.25);
            wantB[i] = 4.0 * ref.getValue(a2[i] * 0.25, a1[i] * 0.25, 0.0);
        }
        Arrays.fill(out, Double.NaN);
        int rc = PaperNativeNormalNoise.nativeFillShiftA(h1, h2, vf, a1, a2, out);
        rep("FillShiftA", tag + (kind ? "" : "-swap"), "vs-wantA", rc, out, wantA);
        rep("FillShiftA", tag + (kind ? "" : "-swap"), "vs-wantB", rc, out, wantB);
        Arrays.fill(out, Double.NaN);
        rc = PaperNativeNormalNoise.nativeFillShiftB(h1, h2, vf, a1, a2, out);
        rep("FillShiftB", tag + (kind ? "" : "-swap"), "vs-wantA", rc, out, wantA);
        rep("FillShiftB", tag + (kind ? "" : "-swap"), "vs-wantB", rc, out, wantB);
    }

    // ---------- reporting ----------

    static int parityCount(double[] got, double[] want) {
        int ok = 0;
        for (int i = 0; i < want.length; i++) {
            if (Double.doubleToRawLongBits(got[i]) == Double.doubleToRawLongBits(want[i])) ok++;
        }
        return ok;
    }

    static void rep(String kernel, String tag, String cand, int rc, double[] got, double[] want) {
        int ok = parityCount(got, want);
        int finite = 0;
        for (double v : got) if (!Double.isNaN(v) && !Double.isInfinite(v)) finite++;
        System.out.println("FILLABI " + kernel + " obj=" + tag + " cand=[" + cand
            + "] rc=" + rc + " finite=" + finite + "/" + got.length
            + " bitparity=" + ok + "/" + want.length
            + (ok == want.length ? "  <-- WINNER" : ""));
    }

    // ---------- handle build (TASK-70 winner pack P3/D1/F1, local copy) ----------

    static long buildHandle(PerlinNoise pn) {
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
            System.out.println("FILLABI handle build FAILED: " + t);
            return 0L;
        }
    }

    // ---------- reflection helpers ----------

    static double valueFactorOf(NormalNoise n) {
        try {
            java.lang.reflect.Field f = NormalNoise.class.getDeclaredField("valueFactor");
            f.setAccessible(true);
            return (Double) f.get(n);
        } catch (Throwable t) { throw new RuntimeException(t); }
    }

    static PerlinNoise firstOf(NormalNoise n) {
        try {
            java.lang.reflect.Field f = NormalNoise.class.getDeclaredField("first");
            f.setAccessible(true);
            return (PerlinNoise) f.get(n);
        } catch (Throwable t) { throw new RuntimeException(t); }
    }

    static PerlinNoise secondOf(NormalNoise n) {
        try {
            java.lang.reflect.Field f = NormalNoise.class.getDeclaredField("second");
            f.setAccessible(true);
            return (PerlinNoise) f.get(n);
        } catch (Throwable t) { throw new RuntimeException(t); }
    }
}
