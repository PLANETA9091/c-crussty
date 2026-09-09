import net.minecraft.world.level.biome.PaperNativeClimateRTree;

/** P500 bench group 43 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.biome.PaperNativeClimateRTree([J[J)J -> [nativeBuildTreeHandle]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G43 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long[] p0;
    static long[] p1;

    @Override public String fqcn() { return "net.minecraft.world.level.biome.PaperNativeClimateRTree"; }
    @Override public String sig() { return "([J[J)J"; }
    @Override public String[] methods() { return new String[]{"nativeBuildTreeHandle"}; }

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
            p1 = new long[64];
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeBuildTreeHandle(p0, p1); }
        default -> acc = 0L;
        }
        acc = acc * 31 + (dst0() ^ dst1());
        return acc;
    }
    static long dst0() {
        return p1.length > 0 ? p1[0] : 0L;
    }
    static long dst1() {
        return p1.length > 1 ? p1[1] : 0L;
    }
}
