
/** P500 bench group 20 — generated. DO NOT EDIT.
 *  PaperNativeNoiseGeneratorSettings([I[I[II[I)I -> [cachedIntSettings, holderValueSettings, lazyPrimitiveSettings, manualLazyObjectSettings, memoizedSupplierSettings]
 *  Typed argument fields + direct static calls into the kernels:
 *  zero reflection in the hot loop, zero boxing of kernel results. */
public final class G20 implements p500.Group {
    private static final int[] SMALL = {7, 31, 3, 15, 63, 1, 9, 21};
    static int[] p0;
    static int[] p1;
    static int[] p2;
    static int p3;
    static int[] p4;

    @Override public String fqcn() { return "PaperNativeNoiseGeneratorSettings"; }
    @Override public String sig() { return "([I[I[II[I)I"; }
    @Override public String[] methods() { return new String[]{"cachedIntSettings", "holderValueSettings", "lazyPrimitiveSettings", "manualLazyObjectSettings", "memoizedSupplierSettings"}; }

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
            p2 = new int[Math.max(8, n)]; for (int i = 0; i < p2.length; i++) p2[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
            p3 = n;
            p4 = new int[Math.max(8, n)]; for (int i = 0; i < p4.length; i++) p4[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
    }

    @Override public long call(int idx) {
        long acc;
        switch (idx) {
        case 0 -> { acc = PaperNativeNoiseGeneratorSettings.cachedIntSettings(p0, p1, p2, p3, p4); }
        case 1 -> { acc = PaperNativeNoiseGeneratorSettings.holderValueSettings(p0, p1, p2, p3, p4); }
        case 2 -> { acc = PaperNativeNoiseGeneratorSettings.lazyPrimitiveSettings(p0, p1, p2, p3, p4); }
        case 3 -> { acc = PaperNativeNoiseGeneratorSettings.manualLazyObjectSettings(p0, p1, p2, p3, p4); }
        case 4 -> { acc = PaperNativeNoiseGeneratorSettings.memoizedSupplierSettings(p0, p1, p2, p3, p4); }
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
