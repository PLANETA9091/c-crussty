import net.minecraft.world.level.biome.PaperNativeClimateRTree;

/** P500 bench group 39 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.biome.PaperNativeClimateRTree(J)J -> [nativeChecksumTreeHandle]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G39 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long p0;

    @Override public String fqcn() { return "net.minecraft.world.level.biome.PaperNativeClimateRTree"; }
    @Override public String sig() { return "(J)J"; }
    @Override public String[] methods() { return new String[]{"nativeChecksumTreeHandle"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = 0x5DEECE66DL + 12345L;
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeChecksumTreeHandle(p0); }
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
