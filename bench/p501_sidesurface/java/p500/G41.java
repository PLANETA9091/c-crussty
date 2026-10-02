import net.minecraft.world.level.biome.PaperNativeClimateRTree;

/** P500 bench group 41 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.biome.PaperNativeClimateRTree(JJJJJJJJI)I -> [nativeSearchBoundedOnePacked, nativeSearchCurrentOnePacked]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G41 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long p0;
    static long p1;
    static long p2;
    static long p3;
    static long p4;
    static long p5;
    static long p6;
    static long p7;
    static int p8;

    @Override public String fqcn() { return "net.minecraft.world.level.biome.PaperNativeClimateRTree"; }
    @Override public String sig() { return "(JJJJJJJJI)I"; }
    @Override public String[] methods() { return new String[]{"nativeSearchBoundedOnePacked", "nativeSearchCurrentOnePacked"}; }

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
            p1 = 0x5DEECE66DL + 67890L;
            p2 = 0x5DEECE66DL + 13579L;
            p3 = 0x5DEECE66DL + 12345L;
            p4 = 0x5DEECE66DL + 67890L;
            p5 = 0x5DEECE66DL + 13579L;
            p6 = 0x5DEECE66DL + 12345L;
            p7 = 0x5DEECE66DL + 67890L;
            p8 = n;
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeSearchBoundedOnePacked(p0, p1, p2, p3, p4, p5, p6, p7, p8); }
        case 1 -> { acc = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeSearchCurrentOnePacked(p0, p1, p2, p3, p4, p5, p6, p7, p8); }
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
