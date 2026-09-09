
/** P500 bench group 9 — generated. DO NOT EDIT.
 *  PaperNativeHash([B[B)I -> [sha256Digest]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G9 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static byte[] p0;
    static byte[] p1;

    @Override public String fqcn() { return "PaperNativeHash"; }
    @Override public String sig() { return "([B[B)I"; }
    @Override public String[] methods() { return new String[]{"sha256Digest"}; }

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
            p1 = new byte[256]; for (int i = 0; i < 256; i++) p1[i] = (byte) ((i * 7919) & 0xFF);
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeHash.sha256Digest(p0, p1); }
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
