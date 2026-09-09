import net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode;

/** P500 bench group 37 — generated. DO NOT EDIT.
 *  net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode([S[B[I[B[I[J[B[I[B[I[J[B)I -> [nativeEncodeSectionData]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G37 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static short[] p0;
    static byte[] p1;
    static int[] p2;
    static byte[] p3;
    static int[] p4;
    static long[] p5;
    static byte[] p6;
    static int[] p7;
    static byte[] p8;
    static int[] p9;
    static long[] p10;
    static byte[] p11;

    @Override public String fqcn() { return "net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode"; }
    @Override public String sig() { return "([S[B[I[B[I[J[B[I[B[I[J[B)I"; }
    @Override public String[] methods() { return new String[]{"nativeEncodeSectionData"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new short[Math.max(8, n)];
            p1 = new byte[256]; for (int i = 0; i < 256; i++) p1[i] = (byte) ((i * 7919) & 0xFF);
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = new byte[256]; for (int i = 0; i < 256; i++) p3[i] = (byte) ((i * 7919) & 0xFF);
            p4 = new int[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p5 = new long[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p6 = new byte[256]; for (int i = 0; i < 256; i++) p6[i] = (byte) ((i * 7919) & 0xFF);
            p7 = new int[Math.max(8, n)]; for (int i = 0; i < p7.length; i++) p7[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p8 = new byte[256]; for (int i = 0; i < 256; i++) p8[i] = (byte) ((i * 7919) & 0xFF);
            p9 = new int[Math.max(8, n)]; for (int i = 0; i < p9.length; i++) p9[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p10 = new long[64];
            p11 = new byte[256]; for (int i = 0; i < 256; i++) p11[i] = (byte) ((i * 7919) & 0xFF);
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p10.length > 0 ? p10[0] : 0L;
    }
    static long dst1() {
        return p10.length > 1 ? p10[1] : 0L;
    }
}
