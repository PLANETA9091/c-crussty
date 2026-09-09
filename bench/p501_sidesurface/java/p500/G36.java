import net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode;

/** P500 bench group 36 — generated. DO NOT EDIT.
 *  net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode([J[J[J[J[BI[BI[B)I -> [nativeEncodeLightData]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G36 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long[] p0;
    static long[] p1;
    static long[] p2;
    static long[] p3;
    static byte[] p4;
    static int p5;
    static byte[] p6;
    static int p7;
    static byte[] p8;

    @Override public String fqcn() { return "net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode"; }
    @Override public String sig() { return "([J[J[J[J[BI[BI[B)I"; }
    @Override public String[] methods() { return new String[]{"nativeEncodeLightData"}; }

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
            p4 = new byte[256]; for (int i = 0; i < 256; i++) p4[i] = (byte) ((i * 7919) & 0xFF);
            p5 = n;
            p6 = new byte[256]; for (int i = 0; i < 256; i++) p6[i] = (byte) ((i * 7919) & 0xFF);
            p7 = SMALL[1];
            p8 = new byte[256]; for (int i = 0; i < 256; i++) p8[i] = (byte) ((i * 7919) & 0xFF);
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeLightData(p0, p1, p2, p3, p4, p5, p6, p7, p8); }
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
