import net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise;

/** P500 bench group 48 — generated. DO NOT EDIT.
 *  net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise([B[B[D[D[D[DDD)J -> [nativeBuildHandle]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G48 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static byte[] p0;
    static byte[] p1;
    static double[] p2;
    static double[] p3;
    static double[] p4;
    static double[] p5;
    static double p6;
    static double p7;

    @Override public String fqcn() { return "net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise"; }
    @Override public String sig() { return "([B[B[D[D[D[DDD)J"; }
    @Override public String[] methods() { return new String[]{"nativeBuildHandle"}; }

    @Override public void setup(int s) {
        switch (s) {
        case 0 -> setup(p500.Bench.N, true);
        case 1 -> setup(16, true);
        case 2 -> setup(1, true);
        case 3 -> setup(16, false);
        }
    }
    private static void setup(int n, boolean strings) {
            p0 = new byte[256]; for (int i = 0; i < 256; i++) p0[i] = (byte) ((i * 7919) & 0xFF);
            p1 = new byte[256]; for (int i = 0; i < 256; i++) p1[i] = (byte) ((i * 7919) & 0xFF);
            p2 = new double[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (i % 97) * 0.5 - 24.0;
            p3 = new double[Math.max(8, n)]; for (int i = 0; i < p3.length; i++) p3[i] = (i % 97) * 0.5 - 24.0;
            p4 = new double[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (i % 97) * 0.5 - 24.0;
            p5 = new double[Math.max(8, n)]; for (int i = 0; i < p5.length; i++) p5[i] = (i % 97) * 0.5 - 24.0;
            p6 = 0.5;
            p7 = -12.25;
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise.nativeBuildHandle(p0, p1, p2, p3, p4, p5, p6, p7); }
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
