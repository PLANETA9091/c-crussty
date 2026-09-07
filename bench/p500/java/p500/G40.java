
/** P500 bench group 40 — generated. DO NOT EDIT.
 *  PaperNativeSpigotLoadOrderDependency(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I -> [newRemovedCountSummary, oldRemovedCountSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G40 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static java.lang.Object[] p1;
    static java.lang.Object[] p2;
    static java.lang.Object[] p3;
    static int p4;
    static long[] p5;

    @Override public String fqcn() { return "PaperNativeSpigotLoadOrderDependency"; }
    @Override public String sig() { return "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I"; }
    @Override public String[] methods() { return new String[]{"newRemovedCountSummary", "oldRemovedCountSummary"}; }

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
            p4 = SMALL[1];
            p5 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary(p0, p1, p2, p3, p4, p5); }
        case 1 -> { acc = PaperNativeSpigotLoadOrderDependency.oldRemovedCountSummary(p0, p1, p2, p3, p4, p5); }
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
