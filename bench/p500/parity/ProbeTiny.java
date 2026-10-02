public class ProbeTiny {
    public static void main(String[] a) {
        for (String p : System.getProperty("p500.libs", "").split(java.io.File.pathSeparator)) {
            if (!p.isBlank()) System.load(p);
        }
        long[] dst = new long[64];
        for (int i = 0; i < 64; i++) dst[i] = (i % 2 == 0) ? 0L : -1L;
        for (int a0 : new int[]{0, 16, 256}) {
            long t = System.nanoTime();
            int r = PaperNativeNoiseInterpolatorSlice.oldJaggedSummary(a0, 31, 3, 15, dst);
            System.err.printf("slice old a0=%d -> %d in %.2f ms%n", a0, r, (System.nanoTime() - t) / 1e6);
        }
        for (int a0 : new int[]{16, 256}) {
            long t = System.nanoTime();
            int r = PaperNativePalettedReencodeScratch.oldNewArraySummary(a0, dst);
            System.err.printf("scratch old a0=%d -> %d in %.2f ms%n", a0, r, (System.nanoTime() - t) / 1e6);
        }
        byte[] g = new byte[256];
        for (int i = 0; i < 256; i++) g[i] = (byte) ((i * 7919) & 0xFF);
        for (int n : new int[]{0, 256}) {
            long t = System.nanoTime();
            int r = PaperNativeImprovedNoiseInline.oldPMethodSummary(g, n, dst);
            System.err.printf("inline old n=%d -> %d in %.3f ms%n", n, r, (System.nanoTime() - t) / 1e6);
        }
        System.err.println("SINK " + dst[0]);
    }
}
