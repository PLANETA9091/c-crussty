
/** P500 bench group 22 — generated. DO NOT EDIT.
 *  PaperNativeNoiseChunkFlatCacheContext(II[J)I -> [newFalseContextSummary, newTrueContextSummary, oldFalseContextSummary, oldTrueContextSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G22 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static int p1;
    static long[] p2;

    @Override public String fqcn() { return "PaperNativeNoiseChunkFlatCacheContext"; }
    @Override public String sig() { return "(II[J)I"; }
    @Override public String[] methods() { return new String[]{"newFalseContextSummary", "newTrueContextSummary", "oldFalseContextSummary", "oldTrueContextSummary"}; }

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
            p2 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeNoiseChunkFlatCacheContext.newFalseContextSummary(p0, p1, p2); }
        case 1 -> { acc = PaperNativeNoiseChunkFlatCacheContext.newTrueContextSummary(p0, p1, p2); }
        case 2 -> { acc = PaperNativeNoiseChunkFlatCacheContext.oldFalseContextSummary(p0, p1, p2); }
        case 3 -> { acc = PaperNativeNoiseChunkFlatCacheContext.oldTrueContextSummary(p0, p1, p2); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p2.length > 0 ? p2[0] : 0L;
    }
    static long dst1() {
        return p2.length > 1 ? p2[1] : 0L;
    }
}
