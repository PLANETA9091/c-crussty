
/** P500 bench group 16 — generated. DO NOT EDIT.
 *  PaperNativeNbtCompoundMapCapacity([B[I[II[J)I -> [parseCapacitySummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G16 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static byte[] p0;
    static int[] p1;
    static int[] p2;
    static int p3;
    static long[] p4;

    @Override public String fqcn() { return "PaperNativeNbtCompoundMapCapacity"; }
    @Override public String sig() { return "([B[I[II[J)I"; }
    @Override public String[] methods() { return new String[]{"parseCapacitySummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new byte[256]; for (int i = 0; i < 256; i++) p0[i] = (byte) ((i * 7919) & 0xFF);
            p1 = new int[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = n;
            p4 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeNbtCompoundMapCapacity.parseCapacitySummary(p0, p1, p2, p3, p4); }
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
