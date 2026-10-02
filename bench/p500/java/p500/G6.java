
/** P500 bench group 6 — generated. DO NOT EDIT.
 *  PaperNativeClimateParameterDistance([J[J[J[J)I -> [branchDistanceSum, oldDistanceSum, subtractFirstDistanceSum]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G6 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long[] p0;
    static long[] p1;
    static long[] p2;
    static long[] p3;

    @Override public String fqcn() { return "PaperNativeClimateParameterDistance"; }
    @Override public String sig() { return "([J[J[J[J)I"; }
    @Override public String[] methods() { return new String[]{"branchDistanceSum", "oldDistanceSum", "subtractFirstDistanceSum"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new long[Math.max(8, n)]; for (int i = 0; i < p0.length; i++) p0[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p1 = new long[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p2 = new long[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p3 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeClimateParameterDistance.branchDistanceSum(p0, p1, p2, p3); }
        case 1 -> { acc = PaperNativeClimateParameterDistance.oldDistanceSum(p0, p1, p2, p3); }
        case 2 -> { acc = PaperNativeClimateParameterDistance.subtractFirstDistanceSum(p0, p1, p2, p3); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p3.length > 0 ? p3[0] : 0L;
    }
    static long dst1() {
        return p3.length > 1 ? p3[1] : 0L;
    }
}
