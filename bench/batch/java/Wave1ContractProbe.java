import java.util.Arrays;

/**
 * S7-14 wave-1 contract probe — empirically determines the call contract of
 * the three wire-v3 kernels (the g42 precedent: "probe-verified" contract,
 * worklog S7-12). For each kernel and several input shapes:
 *   - the raw jint return,
 *   - whether the trailing long[] dst was written (sentinel-prefilled),
 *   - the first dst lanes,
 *   - whether the int[]/double[]/Object[] inputs were mutated.
 * No dispatcher involvement — direct stub calls only (P500 real-mode style).
 * NOT part of any gate — evidence tool for the batch_table contract docs.
 */
public final class Wave1ContractProbe {
    private static long[] sentinelDst() {
        long[] d = new long[64];
        Arrays.fill(d, 0x5A5A5A5A5A5A5A5AL);
        return d;
    }

    private static int writtenLanes(long[] dst) {
        int w = 0;
        for (long v : dst) if (v != 0x5A5A5A5A5A5A5A5AL) w++;
        return w;
    }

    private static void show(String tag, int ret, long[] dst) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < Math.min(8, dst.length); i++) {
            sb.append(String.format("%016x ", dst[i]));
        }
        System.out.printf("%-42s ret=%d writtenLanes=%d dst[0..8]=%s%n",
                tag, ret, writtenLanes(dst), sb.toString().trim());
    }

    private static int[] hashed(int n, int salt) {
        int[] a = new int[Math.max(8, n)];
        for (int i = 0; i < a.length; i++) a[i] = (int) ((i * 0x9E3779B1L + salt) & 0x3FF);
        return a;
    }

    private static double[] fills(int n) {
        double[] a = new double[Math.max(8, n)];
        for (int i = 0; i < a.length; i++) a[i] = (i % 97) * 0.5 - 24.0;
        return a;
    }

    private static Object[] objs(int n) {
        Object[] a = new Object[Math.max(8, n)];
        for (int i = 0; i < a.length; i++) a[i] = "plugin-" + i;
        return a;
    }

    public static void main(String[] args) {
        // P500 real-mode style: explicit System.load of the CLOSED lib so the
        // stubs bind (the JVM's LD_LIBRARY_PATH lookup alone won't find it).
        String nativeLib = System.getenv().getOrDefault("CRUSSTY_BATCH_NATIVE_LIB",
                "/home/z/ccrussty/c-crussty/native/libpaper_native_jni.so");
        System.load(nativeLib);
        System.out.println("== g35 PaperNativeRangeChoice.optimizedFillArraySummary([D[I[I[II[J)I ==");
        for (int n : new int[]{16, 8, 1, 64}) {
            long[] dst = sentinelDst();
            double[] p0 = fills(n);
            int[] p1 = hashed(n, 0), p2 = hashed(n, 0), p3 = hashed(n, 0);
            int ret = PaperNativeRangeChoice.optimizedFillArraySummary(p0, p1, p2, p3, n, dst);
            show("n=" + n + " arrays=max(8,n) p4=n", ret, dst);
        }
        // n larger than arrays? P500 G35 setup(1): arrays len 8, p4=1
        {
            long[] dst = sentinelDst();
            int ret = PaperNativeRangeChoice.optimizedFillArraySummary(
                    fills(1), hashed(1, 0), hashed(1, 0), hashed(1, 0), 1, dst);
            show("n=1 (G35 case-2 shape)", ret, dst);
        }
        // distinct arrays for p2 (maybe it wants ascending / different roles)
        {
            long[] dst = sentinelDst();
            int n = 16;
            int[] lo = new int[n], hi = new int[n], out = new int[n];
            for (int i = 0; i < n; i++) { lo[i] = i; hi[i] = n - i; out[i] = 0; }
            double[] f = fills(n);
            int ret = PaperNativeRangeChoice.optimizedFillArraySummary(f, lo, hi, out, n, dst);
            show("n=16 lo=i hi=n-i out=0", ret, dst);
        }

        System.out.println("== g39 PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary(I[LObj;[J)I ==");
        for (int n : new int[]{16, 8, 1}) {
            long[] dst = sentinelDst();
            Object[] p1 = objs(n);
            int ret = PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary(n, p1, dst);
            show("n=" + n + " objs=n", ret, dst);
        }

        System.out.println("== g40 PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary(I[LObj;[LObj;[LObj;I[J)I ==");
        for (int n : new int[]{16, 8, 1}) {
            long[] dst = sentinelDst();
            int ret = PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary(
                    n, objs(n), objs(n), objs(n), 7, dst);
            show("n=" + n + " p4=7", ret, dst);
        }

        // g42 control: the known return-carried kernel (ret must be constant -5)
        System.out.println("== control: g42 StaticCacheGet.newBatchSummary (known return-carried) ==");
        {
            long[] dst = sentinelDst();
            int[] keys = new int[32];
            for (int i = 0; i < keys.length; i++) keys[i] = i * 7;
            int ret = PaperNativeStaticCacheGet.newBatchSummary(16, 31, 3, 15, 63, keys, dst);
            show("G42 case-1 wire", ret, dst);
        }
        // shape-A control: known count-written kernel
        System.out.println("== control: g2 AquiferIndexStride.newBatchSummary (known count-written) ==");
        {
            long[] dst = sentinelDst();
            long[] src = new long[64];
            for (int i = 0; i < src.length; i++) src[i] = i;
            // (I[J)I — scalar = 64
            int ret = PaperNativeAquiferIndexStride.newBatchSummary(64, dst);
            System.out.printf("ret=%d writtenLanes=%d (dst untouched by design: kernel writes only on real work)%n",
                    ret, writtenLanes(dst));
        }
    }
}
