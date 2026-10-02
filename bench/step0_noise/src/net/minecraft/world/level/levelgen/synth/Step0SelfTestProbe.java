package net.minecraft.world.level.levelgen.synth;

import it.unimi.dsi.fastutil.doubles.DoubleList;
import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.NormalNoise;
import net.minecraft.world.level.levelgen.synth.PerlinNoise;
import net.minecraft.util.RandomSource;

import java.lang.reflect.Field;
import java.util.Arrays;

/** TASK-108 smoke follow-up probe: replicate the SERVER selfTest failing case
 *  (seed 1234L, n=1 coords (1000,-64,2000), scales sxz=2 sy=3) OUTSIDE the
 *  server using the SAME closed-lib natives via System.load (G-ABI-2 rig).
 *  Isolates: fill-kernel divergence vs server-side bridge bug. CPU-only. */
public class Step0SelfTestProbe {
    static Object F(Class<?> k, String n, Object o) throws Exception {
        Field f = k.getDeclaredField(n); f.setAccessible(true); return f.get(o);
    }
    static long buildHandle(PerlinNoise pn) throws Exception {
        ImprovedNoise[] lv = (ImprovedNoise[]) F(PerlinNoise.class, "noiseLevels", pn);
        int n = lv.length;
        byte[] a0 = new byte[256 * n]; byte[] a1 = new byte[n];
        double[] xo = new double[n], yo = new double[n], zo = new double[n];
        Field fp = ImprovedNoise.class.getDeclaredField("p"); fp.setAccessible(true);
        for (int i = 0; i < n; i++) {
            if (lv[i] != null) {
                byte[] p = (byte[]) fp.get(lv[i]);
                System.arraycopy(p, 0, a0, 256 * i, 256);
                a1[i] = 1; xo[i] = lv[i].xo; yo[i] = lv[i].yo; zo[i] = lv[i].zo;
            }
        }
        double[] amps = ((DoubleList) F(PerlinNoise.class, "amplitudes", pn)).toDoubleArray();
        double inF = (Double) F(PerlinNoise.class, "lowestFreqInputFactor", pn);
        double valF = (Double) F(PerlinNoise.class, "lowestFreqValueFactor", pn);
        return PaperNativePerlinNoise.nativeBuildHandle(a0, a1, xo, yo, zo, amps, inF, valF);
    }
    static String hex(double v) {
        return Double.toHexString(v) + "(" + Double.doubleToRawLongBits(v) + ")";
    }
    public static void main(String[] args) throws Exception {
        System.load(args[0]);
        NormalNoise ref = NormalNoise.create(RandomSource.create(1234L), -3, 1.0, 1.0, 1.0, 1.0);
        long h1 = buildHandle((PerlinNoise) F(NormalNoise.class, "first", ref));
        long h2 = buildHandle((PerlinNoise) F(NormalNoise.class, "second", ref));
        double vf = (Double) F(NormalNoise.class, "valueFactor", ref);
        System.out.println("PROBE handles h1=" + h1 + " h2=" + h2 + " vf=" + hex(vf));
        if (h1 == 0 || h2 == 0) { System.out.println("PROBE HANDLE BUILD FAILED"); return; }

        // gather dump (compare with the live server's forensicLine gather)
        PerlinNoise fpn = (PerlinNoise) F(NormalNoise.class, "first", ref);
        ImprovedNoise[] lv = (ImprovedNoise[]) F(PerlinNoise.class, "noiseLevels", fpn);
        double[] amps = ((DoubleList) F(PerlinNoise.class, "amplitudes", fpn)).toDoubleArray();
        System.out.println("PROBE gather lv.len=" + lv.length
            + " amps=" + Arrays.toString(amps)
            + " inF=" + F(PerlinNoise.class, "lowestFreqInputFactor", fpn)
            + " valF=" + F(PerlinNoise.class, "lowestFreqValueFactor", fpn));
        for (int i = 0; i < lv.length; i++) {
            if (lv[i] != null) {
                Field fpf = ImprovedNoise.class.getDeclaredField("p"); fpf.setAccessible(true);
                byte[] p = (byte[]) fpf.get(lv[i]);
                System.out.println("PROBE gather lvl[" + i + "] p0..3=" + p[0] + "," + p[1] + "," + p[2] + "," + p[3]
                    + " xyz=" + lv[i].xo + "," + lv[i].yo + "," + lv[i].zo);
            } else {
                System.out.println("PROBE gather lvl[" + i + "] NULL");
            }
        }

        // 0) handle sanity: nativeGetValue vs Java getValue on a few points
        double[][] pts = {{2000.0, -192.0, 4000.0}, {1.5, -2.5, 3.5}, {-123.75, 64.25, 999.5}};
        for (double[] p : pts) {
            double jv = ref.getValue(p[0], p[1], p[2]);
            double nv = PaperNativeNormalNoise.nativeGetValue(h1, h2, p[0], p[1], p[2], vf);
            boolean eq = Double.doubleToRawLongBits(jv) == Double.doubleToRawLongBits(nv);
            System.out.println("PROBE getValue x=" + p[0] + " y=" + p[1] + " z=" + p[2]
                + " java=" + hex(jv) + " native=" + hex(nv) + (eq ? " MATCH" : " MISMATCH"));
        }

        // 1) the exact selfTest failing case: n=1 scaled fill
        int[] bx = {1000}, by = {-64}, bz = {2000};
        double sxz = 2.0, sy = 3.0;
        double want = ref.getValue(bx[0] * sxz, by[0] * sy, bz[0] * sxz);
        double[] out = {Double.NaN};
        int rc = PaperNativeNormalNoise.nativeFillScaledPositions(h1, h2, vf, bx, by, bz, sxz, sy, out);
        System.out.println("PROBE fill n=1 rc=" + rc + " want=" + hex(want) + " got=" + hex(out[0])
            + (Double.doubleToRawLongBits(want) == Double.doubleToRawLongBits(out[0]) ? " MATCH" : " MISMATCH"));

        // 2) selfTest coord pattern n=256, find first mismatch index
        int n = 256;
        int[] BX = new int[n], BY = new int[n], BZ = new int[n];
        for (int i = 0; i < n; i++) { BX[i] = 1000 + i * 3; BY[i] = -64 + i; BZ[i] = 2000 - i * 2; }
        double[] want2 = new double[n], out2 = new double[n];
        Arrays.fill(out2, Double.NaN);
        for (int i = 0; i < n; i++) want2[i] = ref.getValue(BX[i] * sxz, BY[i] * sy, BZ[i] * sxz);
        rc = PaperNativeNormalNoise.nativeFillScaledPositions(h1, h2, vf, BX, BY, BZ, sxz, sy, out2);
        int bad = -1;
        for (int i = 0; i < n; i++) {
            if (Double.doubleToRawLongBits(want2[i]) != Double.doubleToRawLongBits(out2[i])) { bad = i; break; }
        }
        System.out.println("PROBE fill n=256 rc=" + rc + " firstBad=" + bad
            + (bad >= 0 ? " want=" + hex(want2[bad]) + " got=" + hex(out2[bad]) : ""));

        // 3) regression control: small-coord case that G-ABI-2 proved (n=64)
        double[] CX = new double[64], CY = new double[64], CZ = new double[64];
        for (int i = 0; i < 64; i++) { CX[i] = -32 + i; CY[i] = i - 16; CZ[i] = 16 - i; }
        double[] want3 = new double[64], out3 = new double[64];
        Arrays.fill(out3, Double.NaN);
        for (int i = 0; i < 64; i++) want3[i] = ref.getValue(CX[i], CY[i], CZ[i]);
        rc = PaperNativeNormalNoise.nativeFillPositions(h1, h2, vf, CX, CY, CZ, out3);
        int ok3 = 0;
        for (int i = 0; i < 64; i++) if (Double.doubleToRawLongBits(want3[i]) == Double.doubleToRawLongBits(out3[i])) ok3++;
        System.out.println("PROBE control n=64 rc=" + rc + " bitparity=" + ok3 + "/64");

        // 4) VALUE HUNT: which candidate transformation equals the LIVE
        //    smoke-3 got (raw bits 4587476950501348877)? Each candidate is a
        //    plausible wrong-binding / wrong-arg / wrong-handle outcome.
        final long TARGET = 4587476950501348877L;
        int[] sbx = {1000}, sby = {-64}, sbz = {2000};
        double[][] cands = new double[12][];
        String[] names = new String[12];
        int ci = 0;
        // nativeGetValue variants (G-NORMAL legacy decode is suspect)
        cands[ci] = new double[]{PaperNativeNormalNoise.nativeGetValue(h1, h2, 1000.0, -64.0, 2000.0, vf)};
        names[ci++] = "getValue unscaled (1000,-64,2000,vf)";
        cands[ci] = new double[]{PaperNativeNormalNoise.nativeGetValue(h1, h2, 2000.0, -192.0, 4000.0, vf)};
        names[ci++] = "getValue scaled (2000,-192,4000,vf)";
        cands[ci] = new double[]{PaperNativeNormalNoise.nativeGetValue(h1, h2, 1000.0, -64.0, 2000.0, 1.0)};
        names[ci++] = "getValue unscaled vf=1.0";
        cands[ci] = new double[]{PaperNativeNormalNoise.nativeGetValue(h1, h2, 1000.0, -64.0, 2000.0, 1.0181268882175227d)};
        names[ci++] = "getValue unscaled last=IF";
        // fill variants
        double[] o1 = {Double.NaN};
        PaperNativeNormalNoise.nativeFillScaledPositions(h1, h2, vf, sbx, sby, sbz, 3.0, 2.0, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillScaled swapped scales (sxz=3,sy=2)";
        o1[0] = Double.NaN;
        PaperNativeNormalNoise.nativeFillScaledPositions(h1, h2, 1.0, sbx, sby, sbz, 2.0, 3.0, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillScaled vf=1.0";
        double[] dx1 = {1000.0}, dy1 = {-64.0}, dz1 = {2000.0};
        o1[0] = Double.NaN;
        PaperNativeNormalNoise.nativeFillPositions(h1, h2, vf, dx1, dy1, dz1, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillPositions unscaled (vf)";
        o1[0] = Double.NaN;
        PaperNativeNormalNoise.nativeFillShiftA(h1, h2, vf, sbx, sbz, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillShiftA(bx,bz)";
        o1[0] = Double.NaN;
        PaperNativeNormalNoise.nativeFillShiftB(h1, h2, vf, sbx, sbz, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillShiftB(bx,bz)";
        // handle-swap fill variant
        o1[0] = Double.NaN;
        PaperNativeNormalNoise.nativeFillScaledPositions(h2, h1, vf, sbx, sby, sbz, 2.0, 3.0, o1);
        cands[ci] = new double[]{o1[0]};
        names[ci++] = "fillScaled h1<->h2 swapped";
        // vanilla refs
        cands[ci] = new double[]{ref.getValue(1000.0, -64.0, 2000.0)};
        names[ci++] = "java getValue unscaled";
        cands[ci] = new double[]{ref.getValue(2000.0, -192.0, 4000.0)};
        names[ci++] = "java getValue scaled (want)";
        for (int k = 0; k < ci; k++) {
            boolean hit = Double.doubleToRawLongBits(cands[k][0]) == TARGET;
            System.out.println("PROBE hunt[" + names[k] + "] = " + hex(cands[k][0])
                + (hit ? "  <=== LIVE-GOT MATCH" : ""));
        }
        System.out.println("PROBE target live-got = " + hex(Double.longBitsToDouble(TARGET)) + "(" + TARGET + ")");
    }
}
