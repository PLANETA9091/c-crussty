import java.util.Arrays;
import java.util.Random;

/**
 * TASK-54 — semantic parity gate, wave 2: the remaining wire-eligible P500
 * WIN pairs (promotion lifecycle, step 1 — same contract as FlatCacheParity
 * from TASK-53):
 *
 *   PaperNativeNoiseInterpolatorSlice   oldJaggedSummary     -> flatSummary            (IIII[J)I, P500 WIN 3.32x
 *   PaperNativeImprovedNoiseInline      oldPMethodSummary    -> switchGradientSummary  ([BI[J)I,  P500 WIN 1.22x
 *   PaperNativePalettedReencodeScratch  oldNewArraySummary   -> scratchThreadLocalSummary (I[J)I, P500 WIN 1.20x
 *
 * For every probe input the OLD kernel and the NEW kernel run on identical
 * fresh copies of the input arrays; the jint result and the full dst long[]
 * contents are byte-compared. A mismatch on any pair forbids THAT pair's
 * promotion. On PASS the driver emits deterministic FIXTURE2 vectors (inputs
 * -> OLD-implementation expectations) for the plugin live self-test.
 *
 * Loads the REAL closed libpaper_native_jni.so via -Dp500.libs (no mocks).
 * Argument domains mirror the canonical bench groups G23 / G16 / G27
 * (generated P500 drivers, zero-reflection typed calls).
 *
 * Output lines (stdout, TSV):
 *   PARITY\t<pair>\tinputs=N\tmismatches=M\tverdict=PASS|MISMATCH
 *   FIXTURE2\t<pair>\t<intArgsCsv>\t<byteLen>\t<byteCsv>\t<longLen>\t<longCsv>\tresult\tdstCsv
 *   SINK\t<acc>
 */
public final class WinPairParity {

    private static long SINK = 0;

    interface IntSlicePair {
        int oldK(int a, int b, int c, int d, long[] dst);

        int newK(int a, int b, int c, int d, long[] dst);
    }

    interface InlinePair {
        int oldK(byte[] g, int n, long[] dst);

        int newK(byte[] g, int n, long[] dst);
    }

    interface ScratchPair {
        int oldK(int a, long[] dst);

        int newK(int a, long[] dst);
    }

    public static void main(String[] args) {
        String libs = System.getProperty("p500.libs", "");
        if (libs.isEmpty()) {
            System.err.println("FATAL: -Dp500.libs not set");
            System.exit(2);
        }
        for (String p : libs.split(java.io.File.pathSeparator)) {
            if (!p.isBlank()) System.load(p);
        }
        System.err.println("libs loaded: " + libs);

        int bad = 0;
        bad += runSlice();
        bad += runInline();
        bad += runScratch();

        System.out.println("SINK\t" + SINK);
        System.err.println("total mismatches: " + bad);
        if (bad != 0) System.exit(3);
    }

    // ---------------------------------------------------------------------
    // Pair 1 — PaperNativeNoiseInterpolatorSlice (IIII[J)I
    // ---------------------------------------------------------------------
    private static int runSlice() {
        IntSlicePair p = new IntSlicePair() {
            public int oldK(int a, int b, int c, int d, long[] dst) {
                return PaperNativeNoiseInterpolatorSlice.oldJaggedSummary(a, b, c, d, dst);
            }

            public int newK(int a, int b, int c, int d, long[] dst) {
                return PaperNativeNoiseInterpolatorSlice.flatSummary(a, b, c, d, dst);
            }
        };
        Random rnd = new Random(0x51CE51CEL);
        int inputs = 0, mismatches = 0;

        int[] a0s = {0, 1, 16, 64, 256};
        int[] smalls = {0, 1, 3, 15, 31};
        int[] lens = {0, 1, 2, 7, 64, 256};
        long[] specials = {0L, -1L, Long.MIN_VALUE, Long.MAX_VALUE, 1L << 40, -(1L << 40)};
        for (int a0 : a0s) {
            for (int a1 : smalls) {
                for (int a2 : new int[]{0, 3}) {
                    for (int a3 : new int[]{0, 15}) {
                        for (int len : new int[]{0, 1, 7, 64}) {
                            long[] src = new long[len];
                            for (int i = 0; i < len; i++) src[i] = specials[rnd.nextInt(specials.length)];
                            long[][] out = new long[2][];
                            int[] res = new int[2];
                            res[0] = run2(p::oldK, a0, a1, a2, a3, src, out, 0);
                            res[1] = run2(p::newK, a0, a1, a2, a3, src, out, 1);
                            inputs++;
                            mismatches += check("slice", inputs, out, res, a0, a1, a2, a3, len);
                            SINK = SINK * 31 + res[0] + (out[0].length > 0 ? out[0][0] : 0);
                        }
                    }
                }
            }
        }
        long t0 = System.currentTimeMillis();
        for (int i = 0; i < 1200; i++) {
            if (i % 100 == 0) {
                System.err.printf("[slice] rand %d/%d (%d ms)%n", i, 1200, System.currentTimeMillis() - t0);
            }
            int a0 = rnd.nextInt(257);
            int a1 = rnd.nextInt(32);
            int a2 = rnd.nextInt(8);
            int a3 = rnd.nextInt(16);
            long[] src = new long[lens[rnd.nextInt(lens.length)]];
            for (int j = 0; j < src.length; j++) {
                int mode = rnd.nextInt(3);
                src[j] = mode == 0 ? rnd.nextInt(256)
                        : mode == 1 ? -rnd.nextInt(1 << 20)
                        : rnd.nextLong() >> (rnd.nextInt(48));
            }
            long[][] out = new long[2][];
            int[] res = new int[2];
            res[0] = run2(p::oldK, a0, a1, a2, a3, src, out, 0);
            res[1] = run2(p::newK, a0, a1, a2, a3, src, out, 1);
            inputs++;
            mismatches += check("slice", inputs, out, res, a0, a1, a2, a3, src.length);
            SINK = SINK * 31 + res[1] + (out[1].length > 1 ? out[1][1] : 0);
            if (i % 97 == 0) {
                long[] c1 = src.clone(), c2 = src.clone();
                int rA = p.newK(a0, a1, a2, a3, c1);
                int rB = p.newK(a0, a1, a2, a3, c2);
                if (rA != rB || !Arrays.equals(c1, c2)) {
                    mismatches++;
                    System.err.printf("STATE-INCONSISTENCY slice rand a0=%d a1=%d a2=%d a3=%d len=%d%n",
                            a0, a1, a2, a3, src.length);
                }
            }
        }
        System.out.println("PARITY\tslice\tinputs=" + inputs
                + "\tmismatches=" + mismatches + "\tverdict=" + (mismatches == 0 ? "PASS" : "MISMATCH"));
        System.err.printf("[slice] done, %d inputs, %d ms total%n", inputs, System.currentTimeMillis() - t0);

        fixture2Slice(16, 31, 3, 15, new long[]{7, 31, 3, 15, 63, 1, 9, 21});
        fixture2Slice(0, 7, 3, 15, new long[]{1, 2, 3, 4, 5, 6, 7, 8});
        fixture2Slice(64, 63, 31, 1, new long[]{-5, 12, 0, -1, 4096, -4096, 77, 123});
        fixture2Slice(256, 1, 15, 63, new long[]{1000000, -1000000, 42, 42, 42, 0, 0, 1});
        return mismatches;
    }

    private static int run2(K5 k, int a0, int a1, int a2, int a3, long[] src, long[][] out, int slot) {
        long[] copy = src.clone();
        int r = k.apply(a0, a1, a2, a3, copy);
        out[slot] = copy;
        return r;
    }

    private interface K5 {
        int apply(int a, int b, int c, int d, long[] dst);
    }

    private static int check(String tag, int n, long[][] out, int[] res, int... ctx) {
        boolean ok = res[0] == res[1] && Arrays.equals(out[0], out[1]);
        if (!ok) {
            System.err.printf("MISMATCH %s #%d ctx=%s res %d/%d%n", tag, n, Arrays.toString(ctx), res[0], res[1]);
            return 1;
        }
        return 0;
    }

    private static void fixture2Slice(int a0, int a1, int a2, int a3, long[] src) {
        long[] dst = src.clone();
        int r = PaperNativeNoiseInterpolatorSlice.oldJaggedSummary(a0, a1, a2, a3, dst);
        emitFixture2("slice", new int[]{a0, a1, a2, a3}, null, src, r, dst);
    }

    // ---------------------------------------------------------------------
    // Pair 2 — PaperNativeImprovedNoiseInline ([BI[J)I
    // ---------------------------------------------------------------------
    private static int runInline() {
        Random rnd = new Random(0x1111E42L);
        int inputs = 0, mismatches = 0;
        int[] lens = {0, 1, 16, 64, 256, 1024};

        for (int len : lens) {
            byte[] pat = new byte[len];
            for (int i = 0; i < len; i++) pat[i] = (byte) ((i * 7919) & 0xFF);
            for (int nMode = 0; nMode < 3; nMode++) {
                int n = nMode == 0 ? 0 : nMode == 1 ? len / 2 : len;
                long[] specials = {0L, -1L, Long.MIN_VALUE, Long.MAX_VALUE, 1L << 40, -(1L << 40)};
                for (int v = 0; v < 2; v++) {
                    long[] src = new long[64];
                    if (v == 0) Arrays.fill(src, -1L);
                    else for (int i = 0; i < src.length; i++) src[i] = specials[rnd.nextInt(specials.length)];
                    byte[] g1 = pat.clone(), g2 = pat.clone();
                    long[] d1 = src.clone(), d2 = src.clone();
                    int r1 = PaperNativeImprovedNoiseInline.oldPMethodSummary(g1, n, d1);
                    int r2 = PaperNativeImprovedNoiseInline.switchGradientSummary(g2, n, d2);
                    inputs++;
                    boolean ok = r1 == r2 && Arrays.equals(g1, g2) && Arrays.equals(d1, d2);
                    if (!ok) {
                        mismatches++;
                        System.err.printf("MISMATCH inline edge len=%d n=%d v=%d res %d/%d%n", len, n, v, r1, r2);
                    }
                    SINK = SINK * 31 + r1 + (d1.length > 0 ? d1[0] : 0);
                }
            }
        }
        long t1 = System.currentTimeMillis();
        for (int i = 0; i < 3000; i++) {
            if (i % 500 == 0) {
                System.err.printf("[inline] rand %d/3000 (%d ms)%n", i, System.currentTimeMillis() - t1);
            }
            int len = lens[rnd.nextInt(lens.length)];
            byte[] pat = new byte[len];
            int mode = rnd.nextInt(2);
            for (int j = 0; j < len; j++) {
                pat[j] = mode == 0 ? (byte) ((j * 7919) & 0xFF) : (byte) rnd.nextInt(256);
            }
            int n = rnd.nextInt(len + 1);
            long[] src = new long[64];
            for (int j = 0; j < src.length; j++) src[j] = rnd.nextLong() >> (rnd.nextInt(64));
            long[] d1 = src.clone(), d2 = src.clone();
            byte[] g1 = pat.clone(), g2 = pat.clone();
            int r1 = PaperNativeImprovedNoiseInline.oldPMethodSummary(g1, n, d1);
            int r2 = PaperNativeImprovedNoiseInline.switchGradientSummary(g2, n, d2);
            inputs++;
            boolean ok = r1 == r2 && Arrays.equals(g1, g2) && Arrays.equals(d1, d2);
            if (!ok) {
                mismatches++;
                System.err.printf("MISMATCH inline rand #%d len=%d n=%d res %d/%d%n", i, len, n, r1, r2);
            }
            SINK = SINK * 31 + r2 + (d2.length > 1 ? d2[1] : 0);
            if (i % 97 == 0) {
                byte[] gA = pat.clone(), gB = pat.clone();
                long[] dA = src.clone(), dB = src.clone();
                int rA = PaperNativeImprovedNoiseInline.switchGradientSummary(gA, n, dA);
                int rB = PaperNativeImprovedNoiseInline.switchGradientSummary(gB, n, dB);
                if (rA != rB || !Arrays.equals(dA, dB) || !Arrays.equals(gA, gB)) {
                    mismatches++;
                    System.err.printf("STATE-INCONSISTENCY inline rand len=%d n=%d%n", len, n);
                }
            }
        }
        System.out.println("PARITY\tinline\tinputs=" + inputs
                + "\tmismatches=" + mismatches + "\tverdict=" + (mismatches == 0 ? "PASS" : "MISMATCH"));

        fixture2Inline(patBytes(16, 0), 16, new long[]{7, 31, 3, 15, 63, 1, 9, 21});
        fixture2Inline(patBytes(16, 0), 0, new long[]{1, 2, 3, 4, 5, 6, 7, 8});
        fixture2Inline(patBytes(8, 1), 8, new long[]{-5, 12, 0, -1, 4096, -4096, 77, 123});
        fixture2Inline(patBytes(4, 0), 4, new long[]{1000000, -1000000, 42, 42, 42, 0, 0, 1});
        return mismatches;
    }

    private static byte[] patBytes(int len, int mode) {
        byte[] b = new byte[len];
        for (int i = 0; i < len; i++) {
            b[i] = mode == 0 ? (byte) ((i * 7919) & 0xFF) : (byte) (i * 37);
        }
        return b;
    }

    private static void fixture2Inline(byte[] g, int nArg, long[] src) {
        long[] dst = src.clone();
        byte[] gc = g.clone();
        int r = PaperNativeImprovedNoiseInline.oldPMethodSummary(gc, nArg, dst);
        emitFixture2("inline", new int[]{nArg}, gc, src, r, dst);
    }

    // ---------------------------------------------------------------------
    // Pair 3 — PaperNativePalettedReencodeScratch (I[J)I
    // ---------------------------------------------------------------------
    private static int runScratch() {
        Random rnd = new Random(0x5CA7C27L);
        int inputs = 0, mismatches = 0;
        int[] a0s = {0, 1, 16, 64, 256, 1024};
        int[] lens = {0, 1, 2, 7, 64, 256};
        long[] specials = {0L, -1L, Long.MIN_VALUE, Long.MAX_VALUE, 1L << 40, -(1L << 40)};
        for (int a0 : a0s) {
            for (int len : lens) {
                for (int v = 0; v < 2; v++) {
                    long[] src = new long[len];
                    if (v == 0) Arrays.fill(src, -1L);
                    else for (int i = 0; i < len; i++) src[i] = specials[rnd.nextInt(specials.length)];
                    long[][] out = new long[2][];
                    int[] res = new int[2];
                    res[0] = run1(PaperNativePalettedReencodeScratch::oldNewArraySummary, a0, src, out, 0);
                    res[1] = run1(PaperNativePalettedReencodeScratch::scratchThreadLocalSummary, a0, src, out, 1);
                    inputs++;
                    mismatches += check("scratch", inputs, out, res, a0, len);
                    SINK = SINK * 31 + res[0] + (out[0].length > 0 ? out[0][0] : 0);
                }
            }
        }
        long t2 = System.currentTimeMillis();
        for (int i = 0; i < 3000; i++) {
            if (i % 500 == 0) {
                System.err.printf("[scratch] rand %d/3000 (%d ms)%n", i, System.currentTimeMillis() - t2);
            }
            int a0 = rnd.nextInt(1025);
            long[] src = new long[lens[rnd.nextInt(lens.length)]];
            for (int j = 0; j < src.length; j++) {
                int mode = rnd.nextInt(3);
                src[j] = mode == 0 ? rnd.nextInt(256)
                        : mode == 1 ? -rnd.nextInt(1 << 20)
                        : rnd.nextLong() >> (rnd.nextInt(48));
            }
            long[][] out = new long[2][];
            int[] res = new int[2];
            res[0] = run1(PaperNativePalettedReencodeScratch::oldNewArraySummary, a0, src, out, 0);
            res[1] = run1(PaperNativePalettedReencodeScratch::scratchThreadLocalSummary, a0, src, out, 1);
            inputs++;
            mismatches += check("scratch", inputs, out, res, a0, src.length);
            SINK = SINK * 31 + res[1] + (out[1].length > 1 ? out[1][1] : 0);
            if (i % 97 == 0) {
                long[] c1 = src.clone(), c2 = src.clone();
                int rA = PaperNativePalettedReencodeScratch.scratchThreadLocalSummary(a0, c1);
                int rB = PaperNativePalettedReencodeScratch.scratchThreadLocalSummary(a0, c2);
                if (rA != rB || !Arrays.equals(c1, c2)) {
                    mismatches++;
                    System.err.printf("STATE-INCONSISTENCY scratch rand a0=%d len=%d%n", a0, src.length);
                }
            }
        }
        System.out.println("PARITY\tscratch\tinputs=" + inputs
                + "\tmismatches=" + mismatches + "\tverdict=" + (mismatches == 0 ? "PASS" : "MISMATCH"));

        fixture2Scratch(16, new long[]{7, 31, 3, 15, 63, 1, 9, 21});
        fixture2Scratch(0, new long[]{1, 2, 3, 4, 5, 6, 7, 8});
        fixture2Scratch(64, new long[]{-5, 12, 0, -1, 4096, -4096, 77, 123});
        fixture2Scratch(256, new long[]{1000000, -1000000, 42, 42, 42, 0, 0, 1});
        return mismatches;
    }

    private interface K1 {
        int apply(int a, long[] dst);
    }

    private static int run1(K1 k, int a0, long[] src, long[][] out, int slot) {
        long[] copy = src.clone();
        int r = k.apply(a0, copy);
        out[slot] = copy;
        return r;
    }

    private static void fixture2Scratch(int a0, long[] src) {
        long[] dst = src.clone();
        int r = PaperNativePalettedReencodeScratch.oldNewArraySummary(a0, dst);
        emitFixture2("scratch", new int[]{a0}, null, src, r, dst);
    }

    // ---------------------------------------------------------------------
    // shared helpers
    // ---------------------------------------------------------------------
    private static void emitFixture2(String tag, int[] ints, byte[] bytes, long[] src, int res, long[] dst) {
        StringBuilder sb = new StringBuilder("FIXTURE2\t").append(tag).append('\t');
        sb.append(ints.length).append(':');
        for (int v : ints) sb.append(v).append(',');
        sb.setLength(sb.length() - 1);
        if (bytes != null) {
            sb.append(':').append(bytes.length);
            for (int i = 0; i < bytes.length; i++) {
                sb.append(i > 0 ? "," : "").append(bytes[i]);
            }
        } else {
            sb.append(":-1");
        }
        sb.append(':').append(src.length);
        for (int i = 0; i < src.length; i++) sb.append(i > 0 ? "," : "").append(src[i]);
        sb.append('\t').append(res).append(':').append(dst.length);
        for (int i = 0; i < dst.length; i++) sb.append(i > 0 ? "," : "").append(dst[i]);
        System.out.println(sb);
        SINK = SINK * 31 + res;
    }
}
