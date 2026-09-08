import crussty.batch.PaperNativeBatchDispatch;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * TASK-24 (C3) batch-floor bench — dispatcher cost before/after the
 * control-plane scratch reuse (db7cf27 = BEFORE, master = AFTER).
 *
 * TASK-48 extension: --kernels 12,13 drives the wave-1 SHAPE A′ kernels
 * (PaperNativeDensityAp2MinMaxFill old/newSummary, (III[J)I — P500 g9, direct
 * 119.8 ns) through the v2 shape-packed scalar plane (3 longs per op), with
 * DIRECT stub references of the same shape. Everything else (protocol,
 * medians, parity gate) is unchanged.
 *
 * G3 wave-1 spike (2026-09-09): --kernels 14 benches the NEW shape C
 * `(IIIII[I[J)I` kernel PaperNativeStaticCacheGet.newBatchSummary (P500 g42,
 * global floor anchor 34.6 ns). Shape-C wire encoding (batch_table/batch_api
 * docs): the v2 scalar plane packs FIVE long slots per op (narrowed jint
 * scalars, P500 G42 case-1 config p0..p4 = 16/31/3/15/63), args1 packs the
 * int[] key slice one int per long slot (argCounts[i] = slot count = 16,
 * P500 key-fill pattern), outs = ONE return-carried long per op (the closed
 * kernel leaves its trailing long[] dst untouched — probe-verified
 * 2026-09-09).
 *
 * WIRE v3 (S7-14, descriptor-parser port BATCH_API_PROPOSAL §4/§5):
 * --kernels 15,16,17 drive the wave-1 REF-PLANE shapes D/E/F — the P500
 * g35/g39/g40 OPTIMIZED members (PaperNativeRangeChoice
 * .optimizedFillArraySummary ([D[I[I[II[J)I, direct 81.6 ns;
 * PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary
 * (I[Ljava/lang/Object;[J)I, direct 88.0 ns; .newRemovedCountSummary
 * (I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I, direct
 * 88.6 ns — all PARITY-grade, baseline.tsv:55/58/59). Wire rules: the v3
 * scalar plane packs 1/1/2 jint long-slots per op (D/E/F), the 7th refArgs
 * argument packs the INPUT refs per op at the prefix sum of
 * Shape::refs(kernelIds[i]) (D=4: [D,[I,[I,[I in descriptor order; E=1:
 * Object[]; F=3: Object[]×3), argCounts[i] = OUTPUT capacity 64 (A-style:
 * the kernel returns the count written into dst), outs = n*64. Element
 * values follow the P500 setup(16, true) patterns (double[] (i%97)*0.5-24;
 * int[] (i*0x9E3779B1)&0x3FF; Object[] "plugin-"+i; int scalars n then
 * SMALL[0]=7) so batch == direct parity is apples-to-apples.
 *
 * v3 parity lanes (per wave-1 kernel): one DIRECT stub call vs one K=1
 * batch with the same fresh inputs — the kernel RETURN must match AND the
 * dst arrays must be BYTE-IDENTICAL over the FULL 64 lanes (both sides
 * sentinel-prefilled; the dispatcher propagates only the reported prefix,
 * so any kernel writing beyond its reported count into dst is exposed).
 *
 * v3 mutation probe: the §5 zero-copy contract hands the caller's arrays to
 * the kernels — inputs are checksummed (raw-bit/content) before and after a
 * K=1 batch and one honest
 * {@code MUTATION\tkernel=%d\tinput=NONE|DETECTED} line is printed per
 * kernel (P500's fresh-args rule documents mutating kernels: NONE is the
 * observed evidence for THESE probed inputs, not a contract guarantee).
 *
 * Measures {@code PaperNativeBatchDispatch.run} wall-clock at batch sizes
 * K={1,8,16,64,256} (batch_api.rs module doc: "measures the dispatcher delta
 * at batch sizes 1/8/16/64/256").
 *
 * Protocol per (kernel, K):
 *   1. settle loop  ~1M kernel ops at THIS K (grows the high-water
 *      capacities to n once; afterwards both arms are in steady state),
 *   2. one verification batch: batch-dispatched outs must equal a DIRECT
 *      stub call result (kernel-by-kernel long parity, P500 args),
 *   3. R=11 measured rounds of M batches (M scaled so every round covers
 *      ~--ops kernel ops), System.nanoTime per round, medians reported.
 *
 * Exit codes: 0 ok, 2 harness/kernel error (bad abi, negative run() code,
 * parity mismatch).
 *
 * Signed: agent-7625532f (TASK-24 bench tail; TASK-48 A′ extension); G3
 * shape-C extension Task 2-b (S7-8, Job 366450); wire-v3 D/E/F extension
 * S7-14 (Task S7-14-A).
 */
public final class BatchFloorBench {

    private static final long ABI_EXPECTED = (3L << 16) | 18; // TABLE_VERSION=3, KERNEL_COUNT=18 (S7-14 wire v3 + wave-1 D/E/F)

    // P500 G42 case-1 config (setup(16, true)): the same arguments the
    // direct stub gets, so batch == direct parity is apples-to-apples.
    private static final int G42_P0 = 16, G42_P1 = 31, G42_P2 = 3, G42_P3 = 15, G42_P4 = 63;
    private static final int G42_KEYS = 16;

    // P500 setup(16, true) wave-1 scalars: the FIRST int scalar of a
    // descriptor is n (16); the SECOND takes SMALL[0] = 7 (gen_p500_bench.py
    // setup rule — g40's leading + mid-list jints).
    private static final int W1_N = 16;
    private static final int W1_SMALL0 = 7;

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

    // Shape dispatch: ids 12/13 are the TASK-48 wave-1 A′ kernels
    // ((III[J)I, P500 g9 DensityAp2MinMaxFill pair); everything else in the
    // bench remains shape A. A′ scalar triple = P500 G9 args: p0=16, p1=31,
    // p2=3 (G9.setup: SMALL[1], SMALL[2]).
    private static boolean isAPrime(int kernelId) {
        return kernelId == 12 || kernelId == 13;
    }

    /** G3 shape C: id 14 = g42 StaticCacheGet newBatchSummary (IIIII[I[J)I. */
    private static boolean isShapeC(int kernelId) {
        return kernelId == 14;
    }

    // Wire v3 (S7-14): ids 15/16/17 = wave-1 ref-plane shapes D/E/F.
    private static boolean isShapeD(int kernelId) { return kernelId == 15; }
    private static boolean isShapeE(int kernelId) { return kernelId == 16; }
    private static boolean isShapeF(int kernelId) { return kernelId == 17; }
    private static boolean isWave1V3(int kernelId) {
        return isShapeD(kernelId) || isShapeE(kernelId) || isShapeF(kernelId);
    }

    /** Ref-plane slots per op (Shape::refs mirror; D=4, E=1, F=3, else 0). */
    private static int refsPerOp(int kernelId) {
        if (isShapeD(kernelId)) return 4;
        if (isShapeE(kernelId)) return 1;
        if (isShapeF(kernelId)) return 3;
        return 0;
    }

    // P500 setup(16, true) input builders (gen_p500_bench.py setup rules).
    private static double[] g35Fills() {
        double[] a = new double[W1_N];
        for (int i = 0; i < a.length; i++) a[i] = (i % 97) * 0.5 - 24.0;
        return a;
    }

    private static int[] g35Hashed() {
        int[] a = new int[W1_N];
        for (int i = 0; i < a.length; i++) a[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
        return a;
    }

    private static Object[] g39Objs() {
        Object[] a = new Object[W1_N];
        for (int i = 0; i < a.length; i++) a[i] = "plugin-" + i;
        return a;
    }

    /** P500 G42 key-fill pattern: {@code (j * 0x9E3779B1) & 0x3FF}. */
    private static long keyPattern(int j) {
        return (j * 0x9E3779B1L) & 0x3FF;
    }

    private static int[] g42Keys() {
        int[] keys = new int[G42_KEYS];
        for (int j = 0; j < keys.length; j++) keys[j] = (int) keyPattern(j);
        return keys;
    }

    // Shared wave-1 DIRECT-arm inputs (single-threaded bench, built once per
    // kernel case — P500 G0 dst-reuse style: the DIRECT arm reuses inputs so
    // its number isolates kernel+transition, not allocation; the PARITY lane
    // builds fresh inputs per side below).
    private static double[] dFills;
    private static int[] dHA, dHB, dHC;
    private static Object[] eObjs, fOA, fOB, fOC;

    private static void buildSharedWave1Inputs(int kernelId) {
        if (isShapeD(kernelId)) {
            dFills = g35Fills();
            dHA = g35Hashed();
            dHB = g35Hashed();
            dHC = g35Hashed();
        } else if (isShapeE(kernelId)) {
            eObjs = g39Objs();
        } else if (isShapeF(kernelId)) {
            fOA = g39Objs();
            fOB = g39Objs();
            fOC = g39Objs();
        }
    }

    private static void benchKernel(int kernelId, int k, int rounds, long opsPerRound) {
        int n = k;
        int[] ids = new int[n];
        long[] args0;
        long[] args1;
        int[] counts;
        long[] outs;
        int[] offs;
        Object[] refArgs;
        if (isShapeC(kernelId)) {
            // G3 shape-C wire: five packed scalar-plane slots per op in args0,
            // the int[] key payload packed one int per long slot in args1,
            // argCounts[i] = input slot count, ONE return-carried long per op.
            args0 = new long[n * 5];
            args1 = new long[n * G42_KEYS];
            counts = new int[n];
            outs = new long[n];
            offs = new int[n];
            for (int i = 0; i < n; i++) {
                ids[i] = kernelId;
                args0[i * 5] = G42_P0;
                args0[i * 5 + 1] = G42_P1;
                args0[i * 5 + 2] = G42_P2;
                args0[i * 5 + 3] = G42_P3;
                args0[i * 5 + 4] = G42_P4;
                for (int j = 0; j < G42_KEYS; j++) args1[i * G42_KEYS + j] = keyPattern(j);
                counts[i] = G42_KEYS;
                offs[i] = i;
            }
            refArgs = new Object[0];
        } else {
            int scalarW = isAPrime(kernelId) ? 3 : (isShapeF(kernelId) ? 2 : 1); // v2/v3 scalar-plane width per op
            args0 = new long[n * scalarW];
            args1 = new long[0];
            counts = new int[n];
            outs = new long[n * 64];
            offs = new int[n];
            Arrays.fill(ids, kernelId);
            for (int i = 0; i < n; i++) {
                if (scalarW == 3) {
                    args0[3 * i] = 16L;   // P500 G9: p0 = 16
                    args0[3 * i + 1] = 31L; // SMALL[1]
                    args0[3 * i + 2] = 3L;  // SMALL[2]
                } else if (scalarW == 2) {
                    args0[2 * i] = W1_N;      // G40: p0 = n = 16
                    args0[2 * i + 1] = W1_SMALL0; // G40: p4 = SMALL[0] = 7
                } else {
                    args0[i] = 16L;       // P500 G0 small shape: p0 = 16
                }
            }
            Arrays.fill(counts, 64);          // shape-A/A′/D/E/F OUTPUT capacity = dst long[64]
            for (int i = 0; i < n; i++) offs[i] = i * 64;
            // Wire v3 ref plane: per-op INPUT refs at the prefix sum of
            // refsPerOp (single-shape batch ⇒ slot base = i * refsPerOp).
            buildSharedWave1Inputs(kernelId);
            int rp = refsPerOp(kernelId);
            refArgs = new Object[n * rp];
            for (int i = 0; i < n; i++) {
                int base = i * rp;
                if (isShapeD(kernelId)) {
                    refArgs[base] = dFills;
                    refArgs[base + 1] = dHA;
                    refArgs[base + 2] = dHB;
                    refArgs[base + 3] = dHC;
                } else if (isShapeE(kernelId)) {
                    refArgs[base] = eObjs;
                } else if (isShapeF(kernelId)) {
                    refArgs[base] = fOA;
                    refArgs[base + 1] = fOB;
                    refArgs[base + 2] = fOC;
                }
            }
        }

        int ret = PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, refArgs);
        if (ret != n) {
            System.err.printf("kernel %d K=%d: run()=%d (expected %d) — harness error%n",
                    kernelId, n, ret, n);
            System.exit(2);
        }

        // ---- settle: ~1M kernel ops at this K (steady state for both arms) ----
        long settleBatches = Math.max(2000, 1_000_000L / n);
        long acc = 0;
        for (long b = 0; b < settleBatches; b++) {
            acc += PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, refArgs);
        }
        if (acc != settleBatches * n) {
            System.err.printf("kernel %d K=%d: settle returned inconsistent counts%n", kernelId, n);
            System.exit(2);
        }

        // ---- parity: batch result must equal the DIRECT per-op stub call ----
        parityCheck(kernelId);

        // ---- measured rounds ----
        long batches = Math.max(500, opsPerRound / n);
        double[] perBatchNs = new double[rounds];
        for (int r = 0; r < rounds; r++) {
            long t0 = System.nanoTime();
            for (long b = 0; b < batches; b++) {
                PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, refArgs);
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
        for (long b = 0; b < 20000; b++) acc2 += directCall(kernelId, dst);
        double[] directNs = new double[rounds];
        for (int r = 0; r < rounds; r++) {
            long t0 = System.nanoTime();
            for (long b = 0; b < directCalls; b++) acc2 += directCall(kernelId, dst);
            long t1 = System.nanoTime();
            directNs[r] = (double) (t1 - t0) / directCalls;
        }
        if (acc2 == 0) System.err.println("(unreachable guard: direct acc)");
        System.out.printf("RESULT_DIRECT\tkernel=%d\tcalls=%d\trounds=%d\t"
                        + "direct_ns_median=%.1f\tdirect_ns_min=%.1f\tdirect_ns_max=%.1f%n",
                kernelId, directCalls, rounds, median(directNs),
                Arrays.stream(directNs).min().orElse(0), Arrays.stream(directNs).max().orElse(0));
    }

    private static int directCall(int kernelId, long[] dst) {
        if (isAPrime(kernelId)) {
            return (kernelId == 13)
                    ? PaperNativeDensityAp2MinMaxFill.newSummary(16, 31, 3, dst)
                    : PaperNativeDensityAp2MinMaxFill.oldSummary(16, 31, 3, dst);
        }
        if (isShapeC(kernelId)) {
            return PaperNativeStaticCacheGet.newBatchSummary(G42_P0, G42_P1, G42_P2, G42_P3, G42_P4, g42Keys(), dst);
        }
        if (isShapeD(kernelId)) {
            return PaperNativeRangeChoice.optimizedFillArraySummary(dFills, dHA, dHB, dHC, W1_N, dst);
        }
        if (isShapeE(kernelId)) {
            return PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary(W1_N, eObjs, dst);
        }
        if (isShapeF(kernelId)) {
            return PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary(W1_N, fOA, fOB, fOC, W1_SMALL0, dst);
        }
        return switch (kernelId) {
            case 3 -> PaperNativeAquiferIndexStride.newBatchSummary(16, dst);
            default -> PaperNativeAquiferIndexStride.oldBatchSummary(16, dst);
        };
    }

    // ---- wire v3 input checksums (mutation probe) ------------------------

    private static long checksum(double[] a) {
        long h = 1;
        for (double v : a) h = h * 31 + Double.doubleToRawLongBits(v);
        return h;
    }

    private static long checksum(int[] a) {
        long h = 1;
        for (int v : a) h = h * 31 + v;
        return h;
    }

    private static long checksum(long[] a) {
        long h = 1;
        for (long v : a) h = h * 31 + v;
        return h;
    }

    /** Object[]: content hash for typed arrays/strings, identity for opaque
     *  objects (identityHashCode detects element REPLACEMENT; in-place
     *  mutation of an opaque object is NOT visible — documented probe
     *  limitation; the probed wave-1 elements are Strings). */
    private static long checksum(Object[] a) {
        long h = 1;
        for (Object o : a) {
            h = h * 31 + objectChecksum(o);
        }
        return h;
    }

    private static long objectChecksum(Object o) {
        if (o == null) return 0;
        if (o instanceof double[] d) return checksum(d);
        if (o instanceof int[] i) return checksum(i);
        if (o instanceof long[] l) return checksum(l);
        if (o instanceof Object[] oa) return checksum(oa);
        if (o instanceof String s) return s.hashCode();
        return System.identityHashCode(o);
    }

    /** One checksum over a wave-1 op's ref-plane inputs (descriptor order). */
    private static long wave1InputChecksum(int kernelId, Object[] refArgs) {
        long h = 1;
        for (Object o : refArgs) {
            h = h * 31 + objectChecksum(o);
        }
        return h;
    }

    /**
     * Direct-call parity: one DIRECT stub call (fresh dst, P500 G0 shape) vs
     * one K=1 batch dispatch — payload longs must match. The closed kernels
     * fully overwrite the dst prefix they report (P500 drivers reuse one dst
     * array across 100k+ calls), so history-independence is a precondition
     * the P500 suite already relies on; any mismatch here means the harness
     * (not the kernel) is miswired.
     */
    private static void parityCheck(int kernelId) {
        if (isShapeC(kernelId)) {
            // Shape-C parity: return-carried result — one K=1 batch with the
            // P500 G42 case-1 wire shape must land the SAME jint the direct
            // stub returns (fresh key array + fresh dst on both sides; the
            // kernel's return value is NOT a count — probe: constant -5 —
            // so the shape-A count-range guard does not apply).
            int directRet = PaperNativeStaticCacheGet.newBatchSummary(
                    G42_P0, G42_P1, G42_P2, G42_P3, G42_P4, g42Keys(), new long[64]);
            int[] ids1 = {kernelId};
            long[] args0_1 = {G42_P0, G42_P1, G42_P2, G42_P3, G42_P4};
            long[] args1_1 = new long[G42_KEYS];
            for (int j = 0; j < G42_KEYS; j++) args1_1[j] = keyPattern(j);
            int[] counts1 = {G42_KEYS};
            long[] outs1 = new long[1];
            int[] offs1 = {0};
            int ret = PaperNativeBatchDispatch.run(ids1, args0_1, args1_1, counts1, outs1, offs1, new Object[0]);
            if (ret != 1) {
                System.err.printf("kernel %d: parity run()=%d%n", kernelId, ret);
                System.exit(2);
            }
            if (outs1[0] != directRet) {
                System.err.printf("PARITY FAIL kernel %d (shape C): direct=%d batch=%d (stub=newBatchSummary)%n",
                        kernelId, directRet, outs1[0]);
                System.exit(2);
            }
            System.out.printf("# parity OK kernel=%d shape=C ret=%d (stub=newBatchSummary: batch == direct, return-carried)%n",
                    kernelId, directRet);
            return;
        }
        if (isWave1V3(kernelId)) {
            // Wire-v3 wave-1 parity: fresh P500 setup(16,true) inputs on BOTH
            // sides; the kernel RETURN must match AND the dst arrays must be
            // byte-identical over the FULL 64 lanes (both sentinel-prefilled:
            // the dispatcher propagates only the reported prefix, so any
            // write beyond the reported count into dst is exposed here).
            int[] ids1 = {kernelId};
            long[] args0_1 = isShapeF(kernelId) ? new long[]{W1_N, W1_SMALL0} : new long[]{W1_N};
            int[] counts1 = {64};
            long[] outs1 = new long[64];
            Arrays.fill(outs1, 0x5A5A5A5A5A5A5A5AL);
            int[] offs1 = {0};
            long[] directDst = new long[64];
            Arrays.fill(directDst, 0x5A5A5A5A5A5A5A5AL);

            Object[] refArgs1;
            int directRet;
            if (isShapeD(kernelId)) {
                double[] fills = g35Fills();
                int[] hA = g35Hashed(), hB = g35Hashed(), hC = g35Hashed();
                refArgs1 = new Object[]{fills, hA, hB, hC};
                directRet = PaperNativeRangeChoice.optimizedFillArraySummary(g35Fills(), g35Hashed(), g35Hashed(), g35Hashed(), W1_N, directDst);
            } else if (isShapeE(kernelId)) {
                Object[] objs = g39Objs();
                refArgs1 = new Object[]{objs};
                directRet = PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary(W1_N, g39Objs(), directDst);
            } else {
                Object[] oa = g39Objs(), ob = g39Objs(), oc = g39Objs();
                refArgs1 = new Object[]{oa, ob, oc};
                directRet = PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary(W1_N, g39Objs(), g39Objs(), g39Objs(), W1_SMALL0, directDst);
            }

            long before = wave1InputChecksum(kernelId, refArgs1);
            int ret = PaperNativeBatchDispatch.run(ids1, args0_1, new long[0], counts1, outs1, offs1, refArgs1);
            long after = wave1InputChecksum(kernelId, refArgs1);
            System.out.printf("MUTATION\tkernel=%d\tinput=%s%n", kernelId, before == after ? "NONE" : "DETECTED");
            if (ret != 1) {
                System.err.printf("kernel %d: parity run()=%d%n", kernelId, ret);
                System.exit(2);
            }
            if (outs1[0] != directRet) {
                System.err.printf("PARITY FAIL kernel %d (wave-1 v3): direct ret=%d batch ret=%d%n",
                        kernelId, directRet, outs1[0]);
                System.exit(2);
            }
            if (!Arrays.equals(directDst, outs1)) {
                for (int j = 0; j < 64; j++) {
                    if (directDst[j] != outs1[j]) {
                        System.err.printf("PARITY FAIL kernel %d (wave-1 v3) lane %d: direct=%d batch=%d (full-dst byte compare)%n",
                                kernelId, j, directDst[j], outs1[j]);
                        break;
                    }
                }
                System.exit(2);
            }
            System.out.printf("# parity OK kernel=%d shape=%s ret=%d (batch == direct: return + full dst[64] byte-identical, sentinel-prefilled)%n",
                    kernelId, isShapeD(kernelId) ? "D" : isShapeE(kernelId) ? "E" : "F", directRet);
            return;
        }
        boolean aPrime = isAPrime(kernelId);
        String stubMethod = aPrime
                ? ((kernelId == 13) ? "newSummary" : "oldSummary")
                : ((kernelId == 3) ? "newBatchSummary" : "oldBatchSummary");
        long[] directDst = new long[64];
        // Direct per-op native (auto-bound to libpaper_native_jni.so, P500 real-mode style)
        int directCount = directCall(kernelId, directDst);
        if (directCount < 0 || directCount > 64) {
            System.err.printf("kernel %d: direct stub returned %d — unexpected shape%n", kernelId, directCount);
            System.exit(2);
        }
        // One K=1 batch with the same wire shape (v2: scalar plane width per shape)
        int[] ids1 = {kernelId};
        long[] args0_1 = aPrime ? new long[]{16L, 31L, 3L} : new long[]{16L};
        int[] counts1 = {64};
        long[] outs1 = new long[64];
        int[] offs1 = {0};
        int ret = PaperNativeBatchDispatch.run(ids1, args0_1, new long[0], counts1, outs1, offs1, new Object[0]);
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
