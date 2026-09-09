import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;

public final class ChunkEncodeSectionProbe2 {
    static short[] counts(int n) { short[] a = new short[n]; a[0] = 4096; return a; }
    public static void main(String[] args) throws Exception {
        String libs = System.getProperty("chunkencode.libs", "");
        for (String p : libs.split(java.io.File.pathSeparator)) if (!p.isBlank()) System.load(p);
        try { SharedConstants.tryDetectVersion(); } catch (Throwable ignored) { }
        try { Bootstrap.bootStrap(); } catch (Throwable ignored) { }
        
        // (c) light first: same-process sanity that the .so JNI wrapper works at all
        int rcLight = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeLightData(
                new long[]{0L}, new long[]{0L}, new long[]{0L}, new long[]{0L},
                new byte[2048], 0, new byte[2048], 0, new byte[4096]);
        System.out.println("PROBE\tlight_first\trc=" + rcLight);
        // (a) all arrays length 15
        int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(new short[15], new byte[15], new int[15], new byte[15],
                new int[15], new long[15], new byte[15], new int[15], new byte[15], new int[15],
                new long[15], new byte[1 << 20]);
        System.out.println("PROBE\tlen15_all\trc=" + rc);
        // (b) 0xF header byte on every array (payload shifted by 1)
        short[] c = new short[16]; java.util.Arrays.fill(c, (short) 0x0F00);
        byte[] b1 = new byte[16]; java.util.Arrays.fill(b1, (byte) 0x0F);
        int[] i15 = new int[16]; java.util.Arrays.fill(i15, 0x0F);
        long[] l15 = new long[16]; java.util.Arrays.fill(l15, 0x0F0F0F0F0F0F0F0FL);
        byte[] dst = new byte[1 << 20];
        int rc2 = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(c, b1, i15, b1, i15, l15, b1, i15, b1, i15, l15, dst);
        System.out.println("PROBE\thdr0f_all\trc=" + rc2 + "\tdst0=" + dst[0]);
        // (d) null arrays -> expect distinct code (localizes the -3 semantics)
        int rc3 = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(null, null, null, null, null, null, null, null, null, null, null, dst);
        System.out.println("PROBE\tnull_all\trc=" + rc3);
        // (e) only counts null
        int rc4 = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(null, new byte[24], new int[0], new byte[24], new int[24], new long[0],
                new byte[24], new int[0], new byte[24], new int[24], new long[0], dst);
        System.out.println("PROBE\tnull_counts\trc=" + rc4);
        System.out.println("SINK\t0");
    }
}
