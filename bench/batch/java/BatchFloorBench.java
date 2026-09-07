import crussty.batch.PaperNativeBatchDispatch;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * TASK-24 (C3) batch-floor bench — dispatcher cost before/after the
 * control-plane scratch reuse (db7cf27 = BEFORE, master = AFTER).
 *
 * Measures {@code PaperNativeBatchDispatch.run} wall-clock at batch sizes
 * K={1,8,16,64,256} (batch_api.rs module doc: "measures the dispatcher delta
 * at batch sizes 1/8/16/64/256"), shape-A kernel only (id 2/3:
 * PaperNativeAquiferIndexStride old/newBatchSummary — the P500 G0 shape
 * (I[J)I, scalar=n, dst=long[64]; the batch table's argCounts[i] is the
 * OUTPUT capacity so it is 64 here, matching the P500 stub dst).
 *
 * BEFORE (db7cf27) allocates the control-plane Vecs on every run(); AFTER
 * (28ad646) reuses them from the per-thread scratch at the capacity
 * high-water mark — steady state is allocation-free. The delta therefore
 * shows up as per-BATCH overhead shrinking to ~0 (largest per-op signal at
 * K=1, amortizing towards 1.0x as K grows).
 *
 * Protocol per (kernel, K):
 *   1. settle loop  ~1M kernel ops at THIS K (grows the high-water
 *      capacities to n once; afterwards both arms are in steady state),
 *   2. one verification batch: batch-dispatched outs must equal a DIRECT
 *      stub call result (kernel-by-kernel long parity, P500 G0 args),
 *   3. R=11 measured rounds of M batches (M scaled so every round covers
 *      ~--ops kernel ops), System.nanoTime per round, medians reported.
 *
 * Exit codes: 0 ok, 2 harness/kernel error (bad abi, negative run() code,
 * parity mismatch).
 *
 * Signed: agent-7625532f (TASK-24 bench tail).
 */
public final class BatchFloorBench {

    private static final long ABI_EXPECTED = (1L << 16) | 12; // TABLE_VERSION=1, KERNEL_COUNT=12

    public static void main(String[] args) {
        String lib = arg(args, "--lib", null);
        String nativeLib = arg(args, "--native", null);
        int rounds = Integer.parseInt(arg(args, "--rounds", "11"));
        long opsPerRound = Long.parseLong(arg(args, "--ops", "60000"));
        String[] kernelSpecs = arg(args, "--kernels", "2,3").split(",");
        int[] sizes = Arrays.stream(arg(args, "--sizes", "1,8,16,64,256").split(","))
                .mapToInt(Integer::parseInt).toArray();

        if (lib == null || nativeLib == null) {
            System.err.println("usage: BatchFloorBench --lib <libcrussty.so> --native <libpaper_native_jni.so> "
                    + "[--kernels 2,3] [--sizes 1,8,16,64,256] [--rounds 11] [--ops 60000]");
            System.exit(2);
        }
        // Load the CLOSED lib via the JVM first (P500 real-mode style) so the
        // direct stubs bind; the dispatcher's own Rust dlopen of the same path
        // then just bumps the refcount on the identical handle.
        System.load(nativeLib);
        System.load(lib);

        int abi = PaperNativeBatchDispatch.abiVersion();
        if (abi != ABI_EXPECTED) {
            System.err.println("ABI MISMATCH: abiVersion()=" + abi + " expected " + ABI_EXPECTED);
            System.exit(2);
        }
        System.out.printf("# abiVersion=%d (table=%d kernels=%d) lib=%s jdk=%s%n",
                abi, abi >> 16, abi & 0xFFFF, lib, System.getProperty("java.version"));

        for (String ks : kernelSpecs) {
            int kernelId = Integer.parseInt(ks.trim());
            for (int k : sizes) {
                benchKernel(kernelId, k, rounds, opsPerRound);
            }
        }
        System.out.println("# done");
    }

    // -----------------------------------------------------------------

    private static void benchKernel(int kernelId, int k, int rounds, long opsPerRound) {
        int n = k;
        int[] ids = new int[n];
        long[] args0 = new long[n];
        long[] args1 = new long[0];
        int[] counts = new int[n];
        long[] outs = new long[n * 64];
        int[] offs = new int[n];
        Arrays.fill(ids, kernelId);
        Arrays.fill(args0, 16L);          // P500 G0 small shape: p0 = 16
        Arrays.fill(counts, 64);          // shape-A OUTPUT capacity = dst long[64]
        for (int i = 0; i < n; i++) offs[i] = i * 64;

        int ret = PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs);
        if (ret != n) {
            System.err.printf("kernel %d K=%d: run()=%d (expected %d) — harness error%n",
                    kernelId, n, ret, n);
            System.exit(2);
        }

        // ---- settle: ~1M kernel ops at this K (steady state for both arms) ----
        long settleBatches = Math.max(2000, 1_000_000L / n);
        long acc = 0;
        for (long b = 0; b < settleBatches; b++) {
            acc += PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs);
        }
        if (acc != settleBatches * n) {
            System.err.printf("kernel %d K=%d: settle returned inconsistent counts%n", kernelId, n);
            System.exit(2);
        }

        // ---- parity: batch result must equal the DIRECT per-op stub call ----
        parityCheck(kernelId, args0[0]);

        // ---- measured rounds ----
        long batches = Math.max(500, opsPerRound / n);
        double[] perBatchNs = new double[rounds];
        for (int r = 0; r < rounds; r++) {
            long t0 = System.nanoTime();
            for (long b = 0; b < batches; b++) {
                PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs);
            }
            long t1 = System.nanoTime();
            perBatchNs[r] = (double) (t1 - t0) / batches;
        }
        double med = median(perBatchNs);
        double min = Arrays.stream(perBatchNs).min().orElse(0);
        double max = Arrays.stream(perBatchNs).max().orElse(0);

        System.out.printf("RESULT\tkernel=%d\tK=%d\tbatches=%d\trounds=%d\t"
                        + "batch_ns_median=%.1f\tbatch_ns_min=%.1f\tbatch_ns_max=%.1f\top_ns_median=%.1f%n",
                kernelId, n, batches, rounds, med, min, max, med / n);

        // ---- direct reference: per-op stub calls (module-INDEPENDENT: the
        // stub binds straight to libpaper_native_jni.so, the dispatcher .so is
        // not involved) — decomposes batch = kernel + dispatch overhead and
        // gives the breakeven K where batch beats direct. P500 G0 dst-reuse
        // style: one dst array across the whole loop (closed kernels fully
        // overwrite the prefix they report — P500 relies on the same). ----
        long directCalls = Math.max(500, opsPerRound);
        long[] dst = new long[64];
        long acc2 = 0;
        for (long b = 0; b < 20000; b++) acc2 += directCall(kernelId, 16, dst);
        double[] directNs = new double[rounds];
        for (int r = 0; r < rounds; r++) {
            long t0 = System.nanoTime();
            for (long b = 0; b < directCalls; b++) acc2 += directCall(kernelId, 16, dst);
            long t1 = System.nanoTime();
            directNs[r] = (double) (t1 - t0) / directCalls;
        }
        if (acc2 == 0) System.err.println("(unreachable guard: direct acc)");
        System.out.printf("RESULT_DIRECT\tkernel=%d\tcalls=%d\trounds=%d\t"
                        + "direct_ns_median=%.1f\tdirect_ns_min=%.1f\tdirect_ns_max=%.1f%n",
                kernelId, directCalls, rounds, median(directNs),
                Arrays.stream(directNs).min().orElse(0), Arrays.stream(directNs).max().orElse(0));
    }

    private static int directCall(int kernelId, int scalar, long[] dst) {
        return switch (kernelId) {
            case 3 -> PaperNativeAquiferIndexStride.newBatchSummary(scalar, dst);
            default -> PaperNativeAquiferIndexStride.oldBatchSummary(scalar, dst);
        };
    }

    /**
     * Direct-call parity: one DIRECT stub call (fresh dst, P500 G0 shape) vs
     * one K=1 batch dispatch — payload longs must match. The closed kernels
     * fully overwrite the dst prefix they report (P500 drivers reuse one dst
     * array across 100k+ calls), so history-independence is a precondition
     * the P500 suite already relies on; any mismatch here means the harness
     * (not the kernel) is miswired.
     */
    private static void parityCheck(int kernelId, long scalar) {
        String stubMethod = (kernelId == 3) ? "newBatchSummary" : "oldBatchSummary";
        long[] directDst = new long[64];
        // Direct per-op native (auto-bound to libpaper_native_jni.so, P500 real-mode style)
        int directCount = switch (kernelId) {
            case 3 -> PaperNativeAquiferIndexStride.newBatchSummary((int) scalar, directDst);
            default -> PaperNativeAquiferIndexStride.oldBatchSummary((int) scalar, directDst);
        };
        if (directCount < 0 || directCount > 64) {
            System.err.printf("kernel %d: direct stub returned %d — unexpected shape%n", kernelId, directCount);
            System.exit(2);
        }
        // One K=1 batch with the same wire shape
        int[] ids1 = {kernelId};
        long[] args0_1 = {scalar};
        int[] counts1 = {64};
        long[] outs1 = new long[64];
        int[] offs1 = {0};
        int ret = PaperNativeBatchDispatch.run(ids1, args0_1, new long[0], counts1, outs1, offs1);
        if (ret != 1) {
            System.err.printf("kernel %d: parity run()=%d%n", kernelId, ret);
            System.exit(2);
        }
        for (int j = 0; j < directCount; j++) {
            if (directDst[j] != outs1[j]) {
                System.err.printf("PARITY FAIL kernel %d lane %d: direct=%d batch=%d (stub=%s)%n",
                        kernelId, j, directDst[j], outs1[j], stubMethod);
                System.exit(2);
            }
        }
        System.out.printf("# parity OK kernel=%d directCount=%d (stub=%s: batch == direct, %d lanes)%n",
                kernelId, directCount, stubMethod, directCount);
    }

    private static double median(double[] v) {
        double[] c = v.clone();
        Arrays.sort(c);
        int m = c.length / 2;
        return (c.length & 1) == 1 ? c[m] : (c[m - 1] + c[m]) / 2.0;
    }

    private static String arg(String[] args, String name, String dflt) {
        for (int i = 0; i < args.length - 1; i++) {
            if (args[i].equals(name)) return args[i + 1];
        }
        return dflt;
    }
}
