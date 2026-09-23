import net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise;

/** P500 bench group 47 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise(JDDDDDZ)D -> [nativeGetValue]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G47 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static long p0;
    static double p1;
    static double p2;
    static double p3;
    static double p4;
    static double p5;
    static boolean p6;

    @Override public String fqcn() { return "net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise"; }
    @Override public String sig() { return "(JDDDDDZ)D"; }
    @Override public String[] methods() { return new String[]{"nativeGetValue"}; }

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
            p1 = 0.5;
            p2 = -12.25;
            p3 = 3.75;
            p4 = 0.5;
            p5 = -12.25;
            p6 = true;
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = Double.doubleToRawLongBits(net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise.nativeGetValue(p0, p1, p2, p3, p4, p5, p6)); }
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
