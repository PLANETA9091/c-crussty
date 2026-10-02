import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;

/**
 * TASK-149 phase-1 probe — differential rc probe for nativeEncodeSectionData/-Sized.
 * Goal: localize the pre-write validation that yields rc=-3 on the identification
 * hypothesis layout (counts, bits, palIds, palSizes, palOffs, data | biome-analog | dst).
 * One JVM, many variants, one PROBE row each.
 */
public final class ChunkEncodeSectionProbe {

    static short[] counts(int n, int v) { short[] a = new short[n]; java.util.Arrays.fill(a, (short) v); return a; }
    static byte[] bytes(int n, int v) { byte[] a = new byte[n]; java.util.Arrays.fill(a, (byte) v); return a; }
    static int[] ints(int n, int v) { int[] a = new int[n]; java.util.Arrays.fill(a, v); return a; }
    static long[] longs(int n, long v) { long[] a = new long[n]; java.util.Arrays.fill(a, v); return a; }

    static void probe(String tag, int rc, int lenCounts, int lenPal, int lenData, int dstCap) {
        System.out.println("PROBE\t" + tag + "\trc=" + rc + "\tcounts=" + lenCounts + "\tpal=" + lenPal + "\tdata=" + lenData + "\tdst=" + dstCap);
    }

    public static void main(String[] args) throws Exception {
        String libs = System.getProperty("chunkencode.libs", "");
        for (String p : libs.split(java.io.File.pathSeparator)) if (!p.isBlank()) System.load(p);
        try { SharedConstants.tryDetectVersion(); } catch (Throwable ignored) { }
        try { Bootstrap.bootStrap(); } catch (Throwable ignored) { }

        // ---- plain variant: hypothesis layout (counts, bits, pal, sizes, offs, data, bioBits, bioPal, bioSizes, bioOffs, bioData, dst)
        for (int n : new int[]{0, 1, 24, 26}) {
            int[] sizes = new int[0], pal = new int[0], offs = new int[0];
            long[] data = new long[0];
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(
                    counts(n, 0), bytes(n, 0), pal, bytes(n, 0), offs, data,
                    bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0], new byte[1 << 20]);
            probe("allair_n" + n, rc, n, pal.length, data.length, 1 << 20);
        }

        // single-value sections N=24: sizes=1 with ids
        {
            int n = 24;
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(
                    counts(n, 4096), bytes(n, 0), ints(n, 1), bytes(n, 1), ints(n, 0), new long[0],
                    bytes(n, 0), ints(n, 1), bytes(n, 1), ints(n, 0), new long[0], new byte[1 << 20]);
            probe("single_sizes1_n24", rc, n, n, 0, 1 << 20);
        }
        // linear 4-bit sections N=24: palette 2 ids, 256 longs each
        {
            int n = 24;
            int[] pal = new int[n * 2]; int[] offs = new int[n];
            long[] data = new long[n * 256];
            for (int i = 0; i < n; i++) { pal[2 * i] = 0; pal[2 * i + 1] = 1; offs[i] = 2 * i; }
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(
                    counts(n, 4096), bytes(n, 4), pal, bytes(n, 2), offs, data,
                    bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0], new byte[1 << 20]);
            probe("linear4_n24", rc, n, pal.length, data.length, 1 << 20);
        }
        // same but biome arrays zero-size (not n-length) — length-invariant check
        {
            int n = 24;
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionData(
                    counts(n, 0), bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0],
                    new byte[0], new int[0], new byte[0], new int[0], new long[0], new byte[1 << 20]);
            probe("allair_biome0len_n24", rc, n, 0, 0, 1 << 20);
        }
        // ---- sized variant: (N, counts, bits, pal, sizes, palCap, offs, data, dataCap, bioBits, bioPal, bioSizes, bioPalCap, bioOffs, bioData, bioDataCap, dst, dstCap)
        for (int n : new int[]{1, 24, 26}) {
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionDataSized(
                    n, counts(n, 0), bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0,
                    bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0, new byte[1 << 20], 1 << 20);
            probe("sized_allair_n" + n, rc, n, 0, 0, 1 << 20);
        }
        {
            int n = 24;
            int[] pal = new int[n * 2]; int[] offs = new int[n];
            long[] data = new long[n * 256];
            for (int i = 0; i < n; i++) { pal[2 * i] = 0; pal[2 * i + 1] = 1; offs[i] = 2 * i; }
            int rc = net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode.nativeEncodeSectionDataSized(
                    n, counts(n, 4096), bytes(n, 4), pal, bytes(n, 2), pal.length, offs, data, data.length,
                    bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0, new byte[1 << 20], 1 << 20);
            probe("sized_linear4_n24", rc, n, pal.length, data.length, 1 << 20);
        }
        System.out.println("SINK\t0");
    }
}
