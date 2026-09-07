
/** P500 bench group 34 — generated. DO NOT EDIT.
 *  PaperNativeProtoChunkHeightmap(I)J -> [newCachedContainsSummary, oldEnumSetForeachSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G34 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;

    @Override public String fqcn() { return "PaperNativeProtoChunkHeightmap"; }
    @Override public String sig() { return "(I)J"; }
    @Override public String[] methods() { return new String[]{"newCachedContainsSummary", "oldEnumSetForeachSummary"}; }

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
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeProtoChunkHeightmap.newCachedContainsSummary(p0); }
        case 1 -> { acc = PaperNativeProtoChunkHeightmap.oldEnumSetForeachSummary(p0); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return 0L;
    }
    static long dst1() {
        return 0L;
    }
}
