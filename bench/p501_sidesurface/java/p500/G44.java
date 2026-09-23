import net.minecraft.world.level.levelgen.PaperNativeNoiseChunkWrapCapacity;

/** P500 bench group 44 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.levelgen.PaperNativeNoiseChunkWrapCapacity([I[I[FI[J)I -> [shapeSummary]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G44 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int[] p0;
    static int[] p1;
    static float[] p2;
    static int p3;
    static long[] p4;

    @Override public String fqcn() { return "net.minecraft.world.level.levelgen.PaperNativeNoiseChunkWrapCapacity"; }
    @Override public String sig() { return "([I[I[FI[J)I"; }
    @Override public String[] methods() { return new String[]{"shapeSummary"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new int[Math.max(8, n)]; for (int i = 0; i < p0.length; i++) p0[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p1 = new int[Math.max(8, n)]; for (int i = 0; i < p1.length; i++) p1[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p2 = new float[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (i % 61) * 0.25f - 7.5f;
            p3 = n;
            p4 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.levelgen.PaperNativeNoiseChunkWrapCapacity.shapeSummary(p0, p1, p2, p3, p4); }
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
