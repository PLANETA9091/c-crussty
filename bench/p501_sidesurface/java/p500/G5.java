
/** P500 bench group 5 — generated. DO NOT EDIT.
 *  PaperNativeChunkTicketStage([J[J[B[JII[J)I -> [runBatch]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G5 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long[] p0;
    static long[] p1;
    static byte[] p2;
    static long[] p3;
    static int p4;
    static int p5;
    static long[] p6;

    @Override public String fqcn() { return "PaperNativeChunkTicketStage"; }
    @Override public String sig() { return "([J[J[B[JII[J)I"; }
    @Override public String[] methods() { return new String[]{"runBatch"}; }

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
            p2 = new byte[256]; for (int i = 0; i < 256; i++) p2[i] = (byte) ((i * 7919) & 0xFF);
            p3 = new long[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p4 = n;
            p5 = SMALL[1];
            p6 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeChunkTicketStage.runBatch(p0, p1, p2, p3, p4, p5, p6); }
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
