
/** P500 bench group 45 — generated. DO NOT EDIT.
 *  PaperNativeWaypointDistanceGuard(I[D[D[D[D[D[D[D[J)I -> [guardedAtOrBeyondRangeSummary, oldAtOrBeyondRangeSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G45 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static double[] p1;
    static double[] p2;
    static double[] p3;
    static double[] p4;
    static double[] p5;
    static double[] p6;
    static double[] p7;
    static long[] p8;

    @Override public String fqcn() { return "PaperNativeWaypointDistanceGuard"; }
    @Override public String sig() { return "(I[D[D[D[D[D[D[D[J)I"; }
    @Override public String[] methods() { return new String[]{"guardedAtOrBeyondRangeSummary", "oldAtOrBeyondRangeSummary"}; }

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
            p4 = new double[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (i % 97) * 0.5 - 24.0;
            p5 = new double[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (i % 97) * 0.5 - 24.0;
            p6 = new double[Math.max(8, n)]; for (int i = 0; i < p6.length; i++) p6[i] = (i % 97) * 0.5 - 24.0;
            p7 = new double[Math.max(8, n)]; for (int i = 0; i < p7.length; i++) p7[i] = (i % 97) * 0.5 - 24.0;
            p8 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeWaypointDistanceGuard.guardedAtOrBeyondRangeSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8); }
        case 1 -> { acc = PaperNativeWaypointDistanceGuard.oldAtOrBeyondRangeSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p8.length > 0 ? p8[0] : 0L;
    }
    static long dst1() {
        return p8.length > 1 ? p8[1] : 0L;
    }
}
