
/** P500 bench group 35 — generated. DO NOT EDIT.
 *  PaperNativeRangeChoice([D[I[I[II[J)I -> [oldFillArraySummary, optimizedFillArraySummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G35 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static double[] p0;
    static int[] p1;
    static int[] p2;
    static int[] p3;
    static int p4;
    static long[] p5;

    @Override public String fqcn() { return "PaperNativeRangeChoice"; }
    @Override public String sig() { return "([D[I[I[II[J)I"; }
    @Override public String[] methods() { return new String[]{"oldFillArraySummary", "optimizedFillArraySummary"}; }

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
            p1 = new int[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = new int[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p4 = n;
            p5 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeRangeChoice.oldFillArraySummary(p0, p1, p2, p3, p4, p5); }
        case 1 -> { acc = PaperNativeRangeChoice.optimizedFillArraySummary(p0, p1, p2, p3, p4, p5); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p5.length > 0 ? p5[0] : 0L;
    }
    static long dst1() {
        return p5.length > 1 ? p5[1] : 0L;
    }
}
