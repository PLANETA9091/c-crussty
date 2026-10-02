import net.minecraft.world.level.biome.PaperNativeClimate;

/** P500 bench group 38 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.biome.PaperNativeClimate([J[J[J[I[J)I -> [nodeBestMatchUniqueBatch]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G38 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long[] p0;
    static long[] p1;
    static long[] p2;
    static int[] p3;
    static long[] p4;

    @Override public String fqcn() { return "net.minecraft.world.level.biome.PaperNativeClimate"; }
    @Override public String sig() { return "([J[J[J[I[J)I"; }
    @Override public String[] methods() { return new String[]{"nodeBestMatchUniqueBatch"}; }

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
            p3 = new int[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p4 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.biome.PaperNativeClimate.nodeBestMatchUniqueBatch(p0, p1, p2, p3, p4); }
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
