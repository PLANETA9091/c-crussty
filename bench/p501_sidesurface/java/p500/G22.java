
/** P500 bench group 22 — generated. DO NOT EDIT.
 *  PaperNativePerlinGetValue([B[B[D[D[D[D[DDDII[J)I -> [getValueVariantBatchSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G22 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static byte[] p0;
    static byte[] p1;
    static double[] p2;
    static double[] p3;
    static double[] p4;
    static double[] p5;
    static double[] p6;
    static double p7;
    static double p8;
    static int p9;
    static int p10;
    static long[] p11;

    @Override public String fqcn() { return "PaperNativePerlinGetValue"; }
    @Override public String sig() { return "([B[B[D[D[D[D[DDDII[J)I"; }
    @Override public String[] methods() { return new String[]{"getValueVariantBatchSummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new byte[256]; for (int i = 0; i < 256; i++) p0[i] = (byte) ((i * 7919) & 0xFF);
            p1 = new byte[256]; for (int i = 0; i < 256; i++) p1[i] = (byte) ((i * 7919) & 0xFF);
            p2 = new double[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (i % 97) * 0.5 - 24.0;
            p3 = new double[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (i % 97) * 0.5 - 24.0;
            p4 = new double[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (i % 97) * 0.5 - 24.0;
            p5 = new double[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (i % 97) * 0.5 - 24.0;
            p6 = new double[Math.max(8, n)]; for (int i = 0; i < p6.length; i++) p6[i] = (i % 97) * 0.5 - 24.0;
            p7 = 0.5;
            p8 = -12.25;
            p9 = n;
            p10 = SMALL[1];
            p11 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativePerlinGetValue.getValueVariantBatchSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p11.length > 0 ? p11[0] : 0L;
    }
    static long dst1() {
        return p11.length > 1 ? p11[1] : 0L;
    }
}
