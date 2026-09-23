
/** P500 bench group 42 — generated. DO NOT EDIT.
 *  PaperNativeStaticCacheGet(IIIII[I[J)I -> [newBatchSummary, oldBatchSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G42 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static int p1;
    static int p2;
    static int p3;
    static int p4;
    static int[] p5;
    static long[] p6;

    @Override public String fqcn() { return "PaperNativeStaticCacheGet"; }
    @Override public String sig() { return "(IIIII[I[J)I"; }
    @Override public String[] methods() { return new String[]{"newBatchSummary", "oldBatchSummary"}; }

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
            p1 = SMALL[1];
            p2 = SMALL[2];
            p3 = SMALL[3];
            p4 = SMALL[4];
            p5 = new int[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p6 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeStaticCacheGet.newBatchSummary(p0, p1, p2, p3, p4, p5, p6); }
        case 1 -> { acc = PaperNativeStaticCacheGet.oldBatchSummary(p0, p1, p2, p3, p4, p5, p6); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p6.length > 0 ? p6[0] : 0L;
    }
    static long dst1() {
        return p6.length > 1 ? p6[1] : 0L;
    }
}
