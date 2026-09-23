
/** P500 bench group 0 — generated. DO NOT EDIT.
 *  PaperNativeBeardifierBury(I[D[D[D[J)I -> [currentBatchSummary, optimizedBatchSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G0 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static double[] p1;
    static double[] p2;
    static double[] p3;
    static long[] p4;

    @Override public String fqcn() { return "PaperNativeBeardifierBury"; }
    @Override public String sig() { return "(I[D[D[D[J)I"; }
    @Override public String[] methods() { return new String[]{"currentBatchSummary", "optimizedBatchSummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = n;
            p1 = new double[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (i % 97) * 0.5 - 24.0;
            p2 = new double[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (i % 97) * 0.5 - 24.0;
            p3 = new double[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (i % 97) * 0.5 - 24.0;
            p4 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeBeardifierBury.currentBatchSummary(p0, p1, p2, p3, p4); }
        case 1 -> { acc = PaperNativeBeardifierBury.optimizedBatchSummary(p0, p1, p2, p3, p4); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p4.length > 0 ? p4[0] : 0L;
    }
    static long dst1() {
        return p4.length > 1 ? p4[1] : 0L;
    }
}
