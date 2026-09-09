
/** P500 bench group 15 — generated. DO NOT EDIT.
 *  PaperNativeLz4StreamRoundtrip([BII[J)I -> [roundtripSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G15 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static byte[] p0;
    static int p1;
    static int p2;
    static long[] p3;

    @Override public String fqcn() { return "PaperNativeLz4StreamRoundtrip"; }
    @Override public String sig() { return "([BII[J)I"; }
    @Override public String[] methods() { return new String[]{"roundtripSummary"}; }

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
            p1 = n;
            p2 = SMALL[1];
            p3 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeLz4StreamRoundtrip.roundtripSummary(p0, p1, p2, p3); }
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
