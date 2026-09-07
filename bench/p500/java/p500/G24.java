
/** P500 bench group 24 — generated. DO NOT EDIT.
 *  PaperNativeObfHelperMaps([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I -> [directMapsSummary, oldStreamDefaultSummary, presizedStringPoolSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G24 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static java.lang.Object[] p0;
    static java.lang.Object[] p1;
    static int[] p2;
    static int[] p3;
    static java.lang.Object[] p4;
    static java.lang.Object[] p5;
    static java.lang.Object[] p6;
    static java.lang.Object[] p7;
    static java.lang.Object[] p8;
    static java.lang.Object[] p9;
    static long[] p10;

    @Override public String fqcn() { return "PaperNativeObfHelperMaps"; }
    @Override public String sig() { return "([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I"; }
    @Override public String[] methods() { return new String[]{"directMapsSummary", "oldStreamDefaultSummary", "presizedStringPoolSummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new Object[Math.max(8, n)];
            p1 = new Object[Math.max(8, n)];
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = new int[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p4 = new Object[Math.max(8, n)];
            p5 = new Object[Math.max(8, n)];
            p6 = new Object[Math.max(8, n)];
            p7 = new Object[Math.max(8, n)];
            p8 = new Object[Math.max(8, n)];
            p9 = new Object[Math.max(8, n)];
            p10 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeObfHelperMaps.directMapsSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10); }
        case 1 -> { acc = PaperNativeObfHelperMaps.oldStreamDefaultSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10); }
        case 2 -> { acc = PaperNativeObfHelperMaps.presizedStringPoolSummary(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p10.length > 0 ? p10[0] : 0L;
    }
    static long dst1() {
        return p10.length > 1 ? p10[1] : 0L;
    }
}
