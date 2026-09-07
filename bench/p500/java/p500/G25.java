
/** P500 bench group 25 — generated. DO NOT EDIT.
 *  PaperNativeOreFeatureLoop([D[D[D[D[I[I[I[I[I[IIIIII[J)I -> [oldLoopSummary, optimizedLoopSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G25 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static double[] p0;
    static double[] p1;
    static double[] p2;
    static double[] p3;
    static int[] p4;
    static int[] p5;
    static int[] p6;
    static int[] p7;
    static int[] p8;
    static int[] p9;
    static int p10;
    static int p11;
    static int p12;
    static int p13;
    static int p14;
    static long[] p15;

    @Override public String fqcn() { return "PaperNativeOreFeatureLoop"; }
    @Override public String sig() { return "([D[D[D[D[I[I[I[I[I[IIIIII[J)I"; }
    @Override public String[] methods() { return new String[]{"oldLoopSummary", "optimizedLoopSummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new double[Math.max(8, n)]; for (int i = 0; i < p0.length; i++) p0[i] = (i % 97) * 0.5 - 24.0;
            p1 = new double[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (i % 97) * 0.5 - 24.0;
            p2 = new double[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (i % 97) * 0.5 - 24.0;
            p3 = new double[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (i % 97) * 0.5 - 24.0;
            p4 = new int[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p5 = new int[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p6 = new int[Math.max(8, n)]; for (int i = 0; i < p6.length; i++) p6[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p7 = new int[Math.max(8, n)]; for (int i = 0; i < p7.length; i++) p7[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p8 = new int[Math.max(8, n)]; for (int i = 0; i < p8.length; i++) p8[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p9 = new int[Math.max(8, n)]; for (int i = 0; i < p9.length; i++) p9[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p10 = n;
            p11 = SMALL[1];
            p12 = SMALL[2];
            p13 = SMALL[3];
            p14 = SMALL[4];
            p15 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeOreFeatureLoop.oldLoopSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15); }
        case 1 -> { acc = PaperNativeOreFeatureLoop.optimizedLoopSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p15.length > 0 ? p15[0] : 0L;
    }
    static long dst1() {
        return p15.length > 1 ? p15[1] : 0L;
    }
}
