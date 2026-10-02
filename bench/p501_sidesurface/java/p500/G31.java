
/** P500 bench group 31 — generated. DO NOT EDIT.
 *  PaperNativeWaypointManagerSkip(I[J)I -> [currentPlayerFullSummary, currentPlayerPartialSummary, currentWaypointFullSummary, currentWaypointPartialSummary, skipPlayerFullSummary, skipPlayerPartialSummary, skipWaypointFullSummary, skipWaypointPartialSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G31 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static long[] p1;

    @Override public String fqcn() { return "PaperNativeWaypointManagerSkip"; }
    @Override public String sig() { return "(I[J)I"; }
    @Override public String[] methods() { return new String[]{"currentPlayerFullSummary", "currentPlayerPartialSummary", "currentWaypointFullSummary", "currentWaypointPartialSummary", "skipPlayerFullSummary", "skipPlayerPartialSummary", "skipWaypointFullSummary", "skipWaypointPartialSummary"}; }

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
            p1 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeWaypointManagerSkip.currentPlayerFullSummary(p0, p1); }
        case 1 -> { acc = PaperNativeWaypointManagerSkip.currentPlayerPartialSummary(p0, p1); }
        case 2 -> { acc = PaperNativeWaypointManagerSkip.currentWaypointFullSummary(p0, p1); }
        case 3 -> { acc = PaperNativeWaypointManagerSkip.currentWaypointPartialSummary(p0, p1); }
        case 4 -> { acc = PaperNativeWaypointManagerSkip.skipPlayerFullSummary(p0, p1); }
        case 5 -> { acc = PaperNativeWaypointManagerSkip.skipPlayerPartialSummary(p0, p1); }
        case 6 -> { acc = PaperNativeWaypointManagerSkip.skipWaypointFullSummary(p0, p1); }
        case 7 -> { acc = PaperNativeWaypointManagerSkip.skipWaypointPartialSummary(p0, p1); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p1.length > 0 ? p1[0] : 0L;
    }
    static long dst1() {
        return p1.length > 1 ? p1[1] : 0L;
    }
}
