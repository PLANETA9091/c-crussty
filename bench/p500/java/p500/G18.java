
/** P500 bench group 18 — generated. DO NOT EDIT.
 *  PaperNativeLegacyProvidedAliasRemoval(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I -> [newReverseAliasRemoveSummary, oldValuesRemoveIfSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G18 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static java.lang.Object[] p1;
    static java.lang.Object[] p2;
    static int p3;
    static long[] p4;

    @Override public String fqcn() { return "PaperNativeLegacyProvidedAliasRemoval"; }
    @Override public String sig() { return "(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I"; }
    @Override public String[] methods() { return new String[]{"newReverseAliasRemoveSummary", "oldValuesRemoveIfSummary"}; }

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
            p3 = SMALL[1];
            p4 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeLegacyProvidedAliasRemoval.newReverseAliasRemoveSummary(p0, p1, p2, p3, p4); }
        case 1 -> { acc = PaperNativeLegacyProvidedAliasRemoval.oldValuesRemoveIfSummary(p0, p1, p2, p3, p4); }
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
