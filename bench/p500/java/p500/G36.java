
/** P500 bench group 36 — generated. DO NOT EDIT.
 *  PaperNativeRemapperIndexCleanup(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I -> [newLazyCleanupSummary, oldEagerCleanupSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G36 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static java.lang.Object[] p1;
    static java.lang.Object[] p2;
    static java.lang.Object[] p3;
    static java.lang.Object[] p4;
    static java.lang.Object[] p5;
    static long[] p6;

    @Override public String fqcn() { return "PaperNativeRemapperIndexCleanup"; }
    @Override public String sig() { return "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I"; }
    @Override public String[] methods() { return new String[]{"newLazyCleanupSummary", "oldEagerCleanupSummary"}; }

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
            p1 = new Object[Math.max(8, n)];
            p2 = new Object[Math.max(8, n)];
            p3 = new Object[Math.max(8, n)];
            p4 = new Object[Math.max(8, n)];
            p5 = new Object[Math.max(8, n)];
            p6 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeRemapperIndexCleanup.newLazyCleanupSummary(p0, p1, p2, p3, p4, p5, p6); }
        case 1 -> { acc = PaperNativeRemapperIndexCleanup.oldEagerCleanupSummary(p0, p1, p2, p3, p4, p5, p6); }
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
