
/** P500 bench group 41 — generated. DO NOT EDIT.
 *  PaperNativeSpringFeatureMutablePos(I[I[I[I[Z[I[I[J)I -> [mutableBatchSummary, oldBatchSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G41 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static int[] p1;
    static int[] p2;
    static int[] p3;
    static boolean[] p4;
    static int[] p5;
    static int[] p6;
    static long[] p7;

    @Override public String fqcn() { return "PaperNativeSpringFeatureMutablePos"; }
    @Override public String sig() { return "(I[I[I[I[Z[I[I[J)I"; }
    @Override public String[] methods() { return new String[]{"mutableBatchSummary", "oldBatchSummary"}; }

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
            p1 = new int[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = new int[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p4 = new boolean[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (i % 3) != 0;
            p5 = new int[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p6 = new int[Math.max(8, n)]; for (int i = 0; i < p6.length; i++) p6[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p7 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeSpringFeatureMutablePos.mutableBatchSummary(p0, p1, p2, p3, p4, p5, p6, p7); }
        case 1 -> { acc = PaperNativeSpringFeatureMutablePos.oldBatchSummary(p0, p1, p2, p3, p4, p5, p6, p7); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p7.length > 0 ? p7[0] : 0L;
    }
    static long dst1() {
        return p7.length > 1 ? p7[1] : 0L;
    }
}
