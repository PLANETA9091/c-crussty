
/** P500 bench group 47 — generated. DO NOT EDIT.
 *  PaperNativeWaypointHotPath(I)D -> [cachedChunkVisibleValue, directAzimuthValue, guardedAtOrBeyondRangeValue, guardedReallyFarValue, oldAtOrBeyondRangeValue, oldAzimuthValue, oldChunkVisibleValue, oldReallyFarValue, oldWaypointManagerValue, optimizedWaypointManagerValue]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G47 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;

    @Override public String fqcn() { return "PaperNativeWaypointHotPath"; }
    @Override public String sig() { return "(I)D"; }
    @Override public String[] methods() { return new String[]{"cachedChunkVisibleValue", "directAzimuthValue", "guardedAtOrBeyondRangeValue", "guardedReallyFarValue", "oldAtOrBeyondRangeValue", "oldAzimuthValue", "oldChunkVisibleValue", "oldReallyFarValue", "oldWaypointManagerValue", "optimizedWaypointManagerValue"}; }

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
        case 0 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.cachedChunkVisibleValue(p0)); }
        case 1 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.directAzimuthValue(p0)); }
        case 2 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.guardedAtOrBeyondRangeValue(p0)); }
        case 3 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.guardedReallyFarValue(p0)); }
        case 4 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.oldAtOrBeyondRangeValue(p0)); }
        case 5 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.oldAzimuthValue(p0)); }
        case 6 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.oldChunkVisibleValue(p0)); }
        case 7 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.oldReallyFarValue(p0)); }
        case 8 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.oldWaypointManagerValue(p0)); }
        case 9 -> { acc = Double.doubleToRawLongBits(PaperNativeWaypointHotPath.optimizedWaypointManagerValue(p0)); }
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
