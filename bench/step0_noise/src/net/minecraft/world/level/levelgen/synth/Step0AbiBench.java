package net.minecraft.world.level.levelgen.synth;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.util.RandomSource;

/**
 * G-ABI (TASK-70) — decode the whole-object PerlinNoise handle ABI of the
 * closed lib: PaperNativePerlinNoise.nativeBuildHandle([B[B[D[D[D[DDD)J.
 *
 * Phase A: parity-driven candidate sweep. The ImprovedNoise handle ABI is
 * proven (TASK-67: byte[] p = the production private byte[] p field, plus
 * xo/yo/zo). The PerlinNoise handle is hypothesized to bake the same state
 * per octave slot + loop parameters read from the REAL object via reflection:
 *   byte[] a0/a1 : octave p-tables (packing candidates, see PACKS)
 *   double[] a2..a5 : per-slot coords/amplitudes (order candidates)
 *   double a6/a7 : lowestFreqInputFactor / lowestFreqValueFactor (both orders)
 * Each candidate builds a handle from the real config and runs a 512-sample
 * bit-parity gate vs the real Java getValue(x,y,z,0,0,false). The first
 * fully-passing candidate (in declared order) wins.
 *
 * Phase B: full parity on the winner — 20000 samples canonical path
 * (flag=false, y0=y1=0), 5000 samples random y0/y1, 2000 samples flag=true
 * (the -yo substitution path), on BOTH configs: continuous octaves (-3, 8
 * slots) and gapped octaves (IntStream 0,2,4,6 — null slots + zero
 * amplitudes). Also probes nativeGetValueNoYScale parity vs the canonical
 * path.
 *
 * Phase C: P500 timing on the winning handle — Java 6-arg getValue vs
 * native whole-getValue per-call (120 ms batches, median-of-5, fwd/rev,
 * min-of-medians, SINK, shared coordinate pool, 2 s warmup).
 *
 * CPU-only, no server. One run per invocation; BENCH.lock held by caller.
 */
public final class Step0AbiBench {
    private Step0AbiBench() {}

    private static double SINK = 0.0;

    private interface Op { long unit(int i); }

    private static final class Cfg {
        String name;
        int nSlots;
        byte[][] p;          // per slot; null = absent octave
        double[] xo, yo, zo; // per slot; 0.0 where absent
        double[] amp;
        double inF, valF;
        int firstOctave;
    }

    private static Cfg extract(PerlinNoise pn, String name) throws Exception {
        Cfg c = new Cfg();
        c.name = name;
        java.lang.reflect.Field fN = PerlinNoise.class.getDeclaredField("noiseLevels");
        fN.setAccessible(true);
        ImprovedNoise[] lv = (ImprovedNoise[]) fN.get(pn);
        c.nSlots = lv.length;
        c.p = new byte[lv.length][];
        c.xo = new double[lv.length];
        c.yo = new double[lv.length];
        c.zo = new double[lv.length];
        for (int i = 0; i < lv.length; i++) {
            if (lv[i] != null) {
                java.lang.reflect.Field fp = ImprovedNoise.class.getDeclaredField("p");
                fp.setAccessible(true);
                c.p[i] = (byte[]) fp.get(lv[i]);
                c.xo[i] = lv[i].xo; c.yo[i] = lv[i].yo; c.zo[i] = lv[i].zo;
            }
        }
        java.lang.reflect.Field fA = PerlinNoise.class.getDeclaredField("amplitudes");
        fA.setAccessible(true);
        it.unimi.dsi.fastutil.doubles.DoubleList amp = (it.unimi.dsi.fastutil.doubles.DoubleList) fA.get(pn);
        c.amp = amp.toDoubleArray();
        c.firstOctave = (Integer) getField(PerlinNoise.class, pn, "firstOctave");
        c.inF = (double) getField(PerlinNoise.class, pn, "lowestFreqInputFactor");
        c.valF = (double) getField(PerlinNoise.class, pn, "lowestFreqValueFactor");
        System.out.printf("config %s: slots=%d firstOctave=%d inF=%.6g valF=%.6g present=%d%n",
            name, c.nSlots, c.firstOctave, c.inF, c.valF, presentCount(c));
        StringBuilder amps = new StringBuilder("  amplitudes:");
        for (double a : c.amp) amps.append(' ').append(a);
        System.out.println(amps);
        return c;
    }

    private static Object getField(Class<?> k, Object o, String f) throws Exception {
        java.lang.reflect.Field rf = k.getDeclaredField(f);
        rf.setAccessible(true);
        return rf.get(o);
    }

    private static int presentCount(Cfg c) {
        int n = 0;
        for (byte[] b : c.p) if (b != null) n++;
        return n;
    }

    @FunctionalInterface
    private interface Pack { long build(Cfg c) throws Throwable; }

    private static byte[] concatAll(Cfg c) {
        byte[] out = new byte[256 * c.nSlots];
        for (int i = 0; i < c.nSlots; i++)
            if (c.p[i] != null) System.arraycopy(c.p[i], 0, out, 256 * i, 256);
        return out;
    }

    private static byte[] concatPresent(Cfg c) {
        int np = presentCount(c);
        byte[] out = new byte[256 * np];
        int w = 0;
        for (int i = 0; i < c.nSlots; i++)
            if (c.p[i] != null) System.arraycopy(c.p[i], 0, out, 256 * (w++), 256);
        return out;
    }

    private static byte[] mask(Cfg c) {
        byte[] m = new byte[c.nSlots];
        for (int i = 0; i < c.nSlots; i++) m[i] = (byte) (c.p[i] != null ? 1 : 0);
        return m;
    }

    private static double[][] coordsAmp(Cfg c, boolean ampFirst) {
        double[] a = ampFirst ? c.amp : c.xo;
        double[] b = ampFirst ? c.xo : c.yo;
        double[] d = ampFirst ? c.yo : c.zo;
        double[] e = ampFirst ? c.zo : c.amp;
        return new double[][]{a, b, d, e};
    }

    public static void main(String[] args) throws Exception {
        System.load(args[0]);
        System.out.println("lib loaded: " + args[0]);

        final PerlinNoise cont = PerlinNoise.create(
            RandomSource.create(777L), -3,
            DoubleArrayList.wrap(new double[]{1, 1, 1, 1, 1, 1, 1, 1}));
        final PerlinNoise gap = PerlinNoise.create(
            RandomSource.create(4242L), java.util.stream.IntStream.of(0, 2, 4, 6).boxed().collect(java.util.stream.Collectors.toList()));
        final Cfg cc = extract(cont, "cont(-3,8)");
        final Cfg cg = extract(gap, "gap(0,2,4,6)");

        // ---- Phase A: candidate sweep on GAPPED config (discriminates
        // p-table indexing: slot-indexed 256*N vs compacted present-only) ----
        // pack priority: P3 (all-slot-concat + mask), P2 (present-concat +
        // mask), P0 (all-concat + empty -> handle=0 validation evidence),
        // P1 (empty + all-concat); double order D1 (xo,yo,zo,amp) / D2
        // (amp,xo,yo,zo); factor order F1 (inF,valF) / F2 (valF,inF).
        final int POOL = 8192;
        final double[] xs = new double[POOL], ys = new double[POOL], zs = new double[POOL];
        java.util.Random r = new java.util.Random(777L);
        for (int i = 0; i < POOL; i++) {
            xs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys[i] = -64.0 + r.nextDouble() * 384.0;
            zs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
        }

        java.util.List<String> names = new java.util.ArrayList<>();
        java.util.List<Pack> packs = new java.util.ArrayList<>();
        for (String ptag : new String[]{"P3", "P2", "P0", "P1"}) {
            for (boolean d1 : new boolean[]{true, false}) {
                for (boolean f1 : new boolean[]{true, false}) {
                    final String id = ptag + (d1 ? "/D1" : "/D2") + (f1 ? "/F1" : "/F2");
                    names.add(id);
                    final boolean fp3 = ptag.equals("P3");
                    final boolean fp2 = ptag.equals("P2");
                    final boolean fp0 = ptag.equals("P0");
                    packs.add((Cfg c) -> {
                        byte[] a0 = fp3 ? concatAll(c) : fp2 ? concatPresent(c) : fp0 ? concatAll(c) : new byte[0];
                        byte[] a1 = fp3 ? mask(c) : fp2 ? mask(c) : fp0 ? new byte[0] : concatAll(c);
                        double[][] da = coordsAmp(c, !d1);
                        if (f1) return PaperNativePerlinNoise.nativeBuildHandle(a0, a1, da[0], da[1], da[2], da[3], c.inF, c.valF);
                        return PaperNativePerlinNoise.nativeBuildHandle(a0, a1, da[0], da[1], da[2], da[3], c.valF, c.inF);
                    });
                }
            }
        }

        long winner = 0L;
        String winnerId = null;
        int winPack = -1;
        StringBuilder csv = new StringBuilder("candidate,handle,parity_mismatch_512_gapcfg\n");
        for (int k = 0; k < packs.size(); k++) {
            long h = 0L;
            int mism;
            try {
                h = packs.get(k).build(cg);
                mism = (h == 0L) ? -1 : parityMini(h, gap, xs, ys, zs, 512);
            } catch (Throwable t) {
                mism = -2;
            }
            System.out.printf("candidate %-12s handle=%d mism=%d%n", names.get(k), h, mism);
            csv.append(names.get(k)).append(',').append(h).append(',').append(mism).append('\n');
            if (h != 0L && mism == 0 && winner == 0L) { winner = h; winnerId = names.get(k); winPack = k; }
            if (h != 0L && winner != h) PaperNativePerlinNoise.nativeFreeHandle(h);
        }
        System.out.println("WINNER: " + winnerId + " handle(gap-obj)=" + winner);
        if (winner == 0L) {
            System.out.println("--BEGIN_CSV--\n" + csv + "--END_CSV--");
            System.out.println("VERDICT: G-ABI NO-DECODE (no candidate passed; sweep-2 needed)");
            return;
        }

        // ---- Phase B: full parity on winner pack, ONE HANDLE PER OBJECT ----
        long hCont = 0L;
        try { hCont = packs.get(winPack).build(cc); } catch (Throwable t) { hCont = 0L; }
        if (hCont == 0L) {
            System.out.println("VERDICT: G-ABI NO-DECODE (winner pack failed to build cont handle)");
            System.out.println("--BEGIN_CSV--\n" + csv + "--END_CSV--");
            return;
        }
        int fullGap  = parity6(winner, gap, xs, ys, zs, 20000, 0.0, 0.0, false, r);
        int flagGap  = parity6(winner, gap, xs, ys, zs, 2000, 0.0, 0.0, true, r);
        int fullCont = parity6(hCont, cont, xs, ys, zs, 20000, 0.0, 0.0, false, r);
        int y01Cont  = parity6(hCont, cont, xs, ys, zs, 5000, Double.NaN, Double.NaN, false, r);
        int flagCont = parity6(hCont, cont, xs, ys, zs, 2000, 0.0, 0.0, true, r);
        // NoYScale probe vs canonical path (parity only)
        int noYs = 0;
        for (int i = 0; i < 2000; i++) {
            int j = r.nextInt(POOL);
            double jv = cont.getValue(xs[j], ys[j], zs[j], 0.0, 0.0, false);
            double nv = PaperNativePerlinNoise.nativeGetValueNoYScale(hCont, xs[j], ys[j], zs[j]);
            if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) noYs++;
        }
        System.out.printf("FULL PARITY: gap=%d/20000 gapFlagT=%d/2000 cont=%d/20000 y0y1=%d/5000 flagT=%d/2000 noYScale_vs_canon=%d/2000%n",
            fullGap, flagGap, fullCont, y01Cont, flagCont, noYs);
        csv.append("full_gap,").append(fullGap).append("\n");
        csv.append("flag_gap,").append(flagGap).append("\n");
        csv.append("full_cont,").append(fullCont).append("\n");
        csv.append("y01_cont,").append(y01Cont).append("\n");
        csv.append("flag_cont,").append(flagCont).append("\n");
        csv.append("noyscale,").append(noYs).append("\n");

        // ---- Phase C: P500 timing Java vs native whole-getValue (cont obj) ----
        final long wh = hCont;
        java.util.List<String> tn = new java.util.ArrayList<>();
        java.util.List<Op> ops = new java.util.ArrayList<>();
        tn.add("J_getValue6"); ops.add((i) -> { SINK += cont.getValue(xs[i], ys[i], zs[i], 0.0, 0.0, false); return 1; });
        tn.add("N_getValue6"); ops.add((i) -> { SINK += PaperNativePerlinNoise.nativeGetValue(wh, xs[i], ys[i], zs[i], 0.0, 0.0, false); return 1; });
        final long TARGET_NS = 120_000_000L;
        int n = tn.size();
        double[][] fwd = new double[n][], rev = new double[n][];
        for (int a = 0; a < n; a++) fwd[a] = arm(tn.get(a), ops.get(a), POOL, TARGET_NS);
        for (int a = n - 1; a >= 0; a--) rev[a] = arm("[rev]" + tn.get(a), ops.get(a), POOL, TARGET_NS);
        System.out.println("== G-ABI timing (ns/call, min-of-medians) ==");
        double[] best = new double[n];
        for (int a = 0; a < n; a++) {
            best[a] = Math.min(median(fwd[a]), median(rev[a]));
            System.out.printf("%-24s %10.1f ns/call%n", tn.get(a), best[a]);
            csv.append(tn.get(a)).append(',').append(String.format("%.1f", best[a])).append('\n');
        }
        double ratio = best[1] / best[0];
        System.out.printf("native/Java = %.3fx  (G-ABI GO iff ratio <= 1.00 and full parity 0)%n", ratio);
        csv.append("ratio,").append(String.format("%.3f", ratio)).append('\n');

        PaperNativePerlinNoise.nativeFreeHandle(winner);
        PaperNativePerlinNoise.nativeFreeHandle(hCont);
        System.out.println("--BEGIN_CSV--");
        System.out.print(csv);
        System.out.println("--END_CSV--");
        System.out.println("sink=" + SINK + " winner=" + winnerId);
    }

    private static int parityMini(long h, PerlinNoise pn, double[] xs, double[] ys, double[] zs, int n) {
        return parity6(h, pn, xs, ys, zs, n, 0.0, 0.0, false, new java.util.Random(1L));
    }

    private static int parity6(long h, PerlinNoise pn, double[] xs, double[] ys, double[] zs,
                               int n, double y0, double y1, boolean flag, java.util.Random rnd) {
        int mism = 0;
        for (int i = 0; i < n; i++) {
            int j = rnd.nextInt(xs.length);
            double b0 = Double.isNaN(y0) ? (rnd.nextDouble() * 2.0 - 1.0) : y0;
            double b1 = Double.isNaN(y1) ? (rnd.nextDouble() * 2.0 - 1.0) : y1;
            double jv = pn.getValue(xs[j], ys[j], zs[j], b0, b1, flag);
            double nv = PaperNativePerlinNoise.nativeGetValue(h, xs[j], ys[j], zs[j], b0, b1, flag);
            if (Double.doubleToRawLongBits(jv) != Double.doubleToRawLongBits(nv)) mism++;
        }
        return mism;
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
