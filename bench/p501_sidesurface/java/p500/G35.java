import net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode;

/** P500 bench group 35 — generated. DO NOT EDIT.
 *  net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode(I[S[B[I[BI[I[JI[B[I[BI[I[JI[BI)I -> [nativeEncodeSectionDataSized]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G35 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int p0;
    static short[] p1;
    static byte[] p2;
    static int[] p3;
    static byte[] p4;
    static int p5;
    static int[] p6;
    static long[] p7;
    static int p8;
    static byte[] p9;
    static int[] p10;
    static byte[] p11;
    static int p12;
    static int[] p13;
    static long[] p14;
    static int p15;
    static byte[] p16;
    static int p17;

    @Override public String fqcn() { return "net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode"; }
    @Override public String sig() { return "(I[S[B[I[BI[I[JI[B[I[BI[I[JI[BI)I"; }
    @Override public String[] methods() { return new String[]{"nativeEncodeSectionDataSized"}; }

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
            p1 = new short[Math.max(8, n)];
            p2 = new byte[256]; for (int i = 0; i < 256; i++) p2[i] = (byte) ((i * 7919) & 0xFF);
            p3 = new int[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p4 = new byte[256]; for (int i = 0; i < 256; i++) p4[i] = (byte) ((i * 7919) & 0xFF);
            p5 = SMALL[1];
            p6 = new int[Math.max(8, n)]; for (int i = 0; i < p6.length; i++) p6[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p7 = new long[Math.max(8, n)]; for (int i = 0; i < p7.length; i++) p7[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            p8 = SMALL[2];
            p9 = new byte[256]; for (int i = 0; i < 256; i++) p9[i] = (byte) ((i * 7919) & 0xFF);
            p10 = new int[Math.max(8, n)]; for (int i = 0; i < p10.length; i++) p10[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p11 = new byte[256]; for (int i = 0; i < 256; i++) p11[i] = (byte) ((i * 7919) & 0xFF);
            p12 = SMALL[3];
            p13 = new int[Math.max(8, n)]; for (int i = 0; i < p13.length; i++) p13[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p14 = new long[64];
            p15 = SMALL[4];
            p16 = new byte[256]; for (int i = 0; i < 256; i++) p16[i] = (byte) ((i * 7919) & 0xFF);
            p17 = SMALL[5];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionDataSized(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p14.length > 0 ? p14[0] : 0L;
    }
    static long dst1() {
        return p14.length > 1 ? p14[1] : 0L;
    }
}
