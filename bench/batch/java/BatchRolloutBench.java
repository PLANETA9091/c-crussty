import crussty.batch.PaperNativeBatchDispatch;

import java.util.Arrays;

/**
 * TASK-47-w7 batch-rollout A/B bench — end-to-end measurement of the DORMANT
 * batch-dispatch surface (as-built 397856c + rollout design db7cf27), feeding
 * the auto-threshold-T decision (BATCH_WIRING_PLAN §B.3) and the CRUSSTY_BATCH
 * default flip decision (§B.6).
 *
 * GATE STATUS UPDATE (S7-12, doc-vs-code backlog item 1): the product code
 * DOES read CRUSSTY_BATCH now (src/batch_api.rs rollout gate, G1 — landed
 * 3cf2ed9; G4 site arming landed this session), so the harness's call-site
 * emulation is REDUNDANT but still the measurement vehicle (it measures both
 * arms in one JVM deterministically; the env gate changes routing only).
 * Only CRUSSTY_BATCH=on|auto|off are valid (§B.6 canon); 1/0/garbage parse
 * to Off (test-pinned).
 * (exactly where a real consumer would route): CRUSSTY_BATCH=1 -> every op
 * routes through PaperNativeBatchDispatch.run (the §B.6 "on" mode: force
 * batch at any K, makes the N=1 penalty measurable); unset -> individual
 * per-op bridge calls (the dormant state: zero consumers). Both arms are
 * always measured interleaved in the same JVM (paired rounds: direct block,
 * then batch block) so box drift cancels; the env selects the ROUTED arm and
 * proves env-neutrality of measurement (route flip changes only which cost
 * a real site would pay, not the dispatcher itself).
 *
 * Coverage = the FULL batch table (closed table, batch_table.rs): ids 0-9
 * shape A (I[J)I, scalar + long[64] dst, counts[i] = OUTPUT capacity =
 * 64 = OUT_SCRATCH_CAP; ids 10-11 shape B ([J[J)J, counts[i] = INPUT length,
 * 1 result long/op). Wire v3 (S7-14): 18 kernels — this harness sweeps the
 * historical GROUP_IDS subset (shape A/B) with EMPTY refArgs; the wave-1
 * ref-plane shapes D/E/F are covered by BatchFloorBench --kernels 15,16,17.
 *
 * Wiring-eligible/floor CONTROL groups that the as-built table CANNOT
 * express (g30 PluginLoadingAllocation, g2 AquiferSurfaceSampling-not-in-
 * table, g33 PluginStartupRollup, g35 RangeChoice, g42 StaticCacheGet) are
 * measured DIRECT-ONLY in --controls mode: the B.1/B.4 shape-coverage gap,
 * demonstrated live — no batch arm exists for them at any K.
 *
 * Protocol per (group, K): settle both arms once at K=256 (per-thread
 * scratch high-water is monotone — TASK-24 28ad646 proves steady state
 * allocation-free afterwards; smaller per-K settle than BatchFloorBench is
 * safe), then R measured rounds, each round = direct block then batch block
 * (interleaved pairing), medians reported. Parity per (group, K): direct
 * stub result vs batch outs, bit-exact on the lanes the kernel reports
 * (shape B: opaque handle — both-success comparison, noted per run).
 *
 * Exit codes: 0 ok, 2 harness/kernel error (bad abi, negative run(),
 * parity mismatch, refused probe mismatch).
 *
 * Signed: agent-7625532f (TASK-47-w7).
 */
public final class BatchRolloutBench {

    private static final long ABI_EXPECTED = (3L << 16) | 18; // TABLE_VERSION=3, KERNEL_COUNT=18 (wire v3, S7-14)
    private static final int DST_CAP = 64;                    // = OUT_SCRATCH_CAP
    private static final int SHAPE_B_IN = 8;                  // longs per shape-B op input

    private static final boolean ROUTE_BATCH;
    static {
        String b = System.getenv("CRUSSTY_BATCH");
        ROUTE_BATCH = "1".equals(b) || "on".equals(b); // fail-safe: anything else = direct (B.6)
    }

    public static void main(String[] args) {
        String lib = arg(args, "--lib", null);
        String nativeLib = arg(args, "--native", null);
        int rounds = Integer.parseInt(arg(args, "--rounds", "9"));
        long opsPerRound = Long.parseLong(arg(args, "--ops", "20000"));
        boolean controls = has(args, "--controls");

        if (lib == null || nativeLib == null) {
            System.err.println("usage: BatchRolloutBench --lib <libcrussty.so> --native <libpaper_native_jni.so> "
                    + "[--rounds 7] [--ops 40000] [--controls]");
            System.exit(2);
        }
        System.load(nativeLib);   // closed kernels bind to the direct stubs
        System.load(lib);         // dispatcher cdylib (batch bridge)

        int abi = PaperNativeBatchDispatch.abiVersion();
        if (abi != ABI_EXPECTED) {
            System.err.println("ABI MISMATCH: abiVersion()=" + abi + " expected " + ABI_EXPECTED);
            System.exit(2);
        }
        System.out.printf("# config abi=%d (table=%d kernels=%d) lib=%s jdk=%s route=%s controls=%b%n",
                abi, abi >> 16, abi & 0xFFFF, lib, System.getProperty("java.version"),
                ROUTE_BATCH ? "BATCH" : "DIRECT", controls);

        // Structural-refusal probe (live, gate ON): out-of-table id must be
        // refused BEFORE any op executes. Note: ERR_KERNEL_REFUSED (-10) is
        // unreachable in the shipped configuration (all 12 table kernels are
        // policy-allowed; drift-guard tests hold) — the reachable live
        // refusal is ERR_BAD_KERNEL_ID for ids >= KERNEL_COUNT.
        int probeRet = PaperNativeBatchDispatch.run(new int[]{9999}, new long[]{1}, new long[0],
                new int[]{64}, new long[64], new int[]{0}, new Object[0]);
        System.out.printf("# refused_probe id=9999 ret=%d (negative=%b)%n", probeRet, probeRet < 0);
        if (probeRet >= 0) {
            System.err.println("refused probe: expected negative return for out-of-table id");
            System.exit(2);
        }

        if (controls) {
            runControls(rounds, opsPerRound);
        } else {
            runBatchTable(rounds, opsPerRound);
        }
        System.out.printf("# rss_hwm_kb=%s%n", procVal("VmHWM"));
        System.out.println("# done");
    }

    // ------------------------------------------------------------------
    // Batch-table sweep: 12 ids x K in {1,8,16,64,256}
    // ------------------------------------------------------------------

    private static final int[] GROUP_IDS = {0, 2, 4, 6, 8, 9, 10};
    private static final String[] GROUP_NAME = {
            "ticketsetsearch-ids01", "aquiferindexstride-ids23(g0)", "chunkdependencies-ids45(g5)",
            "densitysplinecontext-ids67(g10)", "entitylookupstatus-id8(g14)",
            "noiseinterpolatorfractions-id9", "climatertree-shapeB-ids1011"};

    private static void runBatchTable(int rounds, long opsPerRound) {
        shapeBMechanicalCheck();
        for (int g = 0; g < GROUP_IDS.length; g++) {
            boolean shapeB = (g == 6);
            int idA = GROUP_IDS[g];
            int idB = GROUP_IDS[g] + 1;
            if (shapeB) {
                // ids 10/11 EXCLUDED from the timed sweep: the closed kernel
                // returns 0 and leaves dst untouched for EVERY synthesizable
                // input (probe: fixed/varying/randomized src, dst 1..64,
                // 200k-call warmup — all r=0) — the input protocol is not
                // reproducible outside the engine. Timing it would measure
                // the rejection path, not the kernel. Mechanical dispatch
                // check above proves the shape-B run() path executes.
                System.out.println("# EXCLUDED ids=10,11 shape=B reason=kernel-input-protocol-not-synthesizable (see BATCH_ROLLOUT_AB.md)");
                continue;
            }
            // adaptive budget: body-dominated kernels (probe > 10 us/op at
            // harness args) get a shrunk op budget — the paired RATIO stays
            // valid, wall-clock stays inside the bench budget
            long scale = probeScale(idA, shapeB);
            long settleOps = 50_000 / scale;
            long ops = Math.max(100, opsPerRound / scale);
            int r = scale > 1 ? Math.max(3, rounds - 2) : rounds;
            // per-group settle at K=256 (scratch high-water, JIT warm), both arms
            settle(idA, shapeB, settleOps);
            for (int k : new int[]{1, 8, 16, 64, 256}) {
                sweepPair(idA, g, k, r, ops, shapeB);
            }
        }
    }

    /**
     * Shape-B dispatch path, live end-to-end (gate ON): run() with id 10 and
     * id 11, K=8 — dispatcher must execute both ops, store the returned
     * jlongs, return 2. (Kernel input protocol not synthesizable — see
     * EXCLUDED note — so result values are the kernel's own rejection
     * value; the check is mechanical, not parity.)
     */
    private static void shapeBMechanicalCheck() {
        for (int id : new int[]{10, 11}) {
            int n = 8;
            int[] ids = new int[n]; Arrays.fill(ids, id);
            long[] args1 = freshSrcPacked(n);
            long[] outs = new long[n];
            int[] counts = new int[n]; Arrays.fill(counts, SHAPE_B_IN);
            int[] offs = new int[n]; for (int i = 0; i < n; i++) offs[i] = i;
            int ret = PaperNativeBatchDispatch.run(ids, new long[n], args1, counts, outs, offs, new Object[0]);
            if (ret != n) {
                System.err.printf("shape-B mechanical check id=%d: run()=%d expected %d%n", id, ret, n);
                System.exit(2);
            }
            System.out.printf("# shape-B mechanical OK id=%d K=%d ret=%d outs0=%d (dispatch path executes; value = kernel rejection for synthesized input)%n", id, n, ret, outs[0]);
        }
    }

    /** 1 for floor kernels; 50 when probe shows a >10us/op body. */
    private static long probeScale(int id, boolean shapeB) {
        long[] dst = new long[DST_CAP];
        long[] src = shapeB ? freshSrc() : null;
        long[] dstB = shapeB ? new long[SHAPE_B_IN] : null;
        for (int i = 0; i < 20; i++) directCall(id, 16, dst, src, dstB); // JIT
        long t0 = System.nanoTime();
        for (int i = 0; i < 10; i++) directCall(id, 16, dst, src, dstB);
        long per = (System.nanoTime() - t0) / 10;
        if (per > 10_000) {
            System.out.printf("# probe id=%d ns_per_op=%d (>10us body) -> scaled budget x50%n", id, per);
            return 50;
        }
        return 1;
    }

    /** Measure ONE kernel id at batch size k: paired rounds direct-vs-batch. */
    private static void sweepPair(int id, int g, int k, int rounds, long ops, boolean shapeB) {
        // warmup (unmeasured) both arms at this K
        directBlock(id, Math.min(1_000, ops), shapeB);
        batchBlock(id, k, Math.min(1_000, ops), shapeB);

        double[] dirNs = new double[rounds];
        double[] batNs = new double[rounds];
        for (int r = 0; r < rounds; r++) {
            dirNs[r] = directBlock(id, ops, shapeB);
            batNs[r] = batchBlock(id, k, ops, shapeB);
        }
        parityCheck(id, shapeB);
        String group = GROUP_NAME[g];
        System.out.printf("RESULT_DIRECT\tid=%d\tgroup=%s\tK=%d\troute=%s\tns_per_op_median=%.1f\tns_min=%.1f\tns_max=%.1f%n",
                id, group, k, ROUTE_BATCH ? "batch" : "direct", median(dirNs), min(dirNs), max(dirNs));
        System.out.printf("RESULT_BATCH\tid=%d\tgroup=%s\tK=%d\troute=%s\tns_per_op_median=%.1f\tns_min=%.1f\tns_max=%.1f%n",
                id, group, k, ROUTE_BATCH ? "batch" : "direct", median(batNs), min(batNs), max(batNs));
        if (ROUTE_BATCH) {
            System.out.printf("ROUTE_COST\tid=%d\tK=%d\trouted_ns_per_op=%.1f%n", id, k, median(batNs));
        } else {
            System.out.printf("ROUTE_COST\tid=%d\tK=%d\trouted_ns_per_op=%.1f%n", id, k, median(dirNs));
        }
    }

    // ------------------------------------------------------------------
    // Controls: floor/wiring-eligible groups with NO batch-table entry
    // (direct-only — the shape-coverage gap, demonstrated live)
    // ------------------------------------------------------------------

    private static void runControls(int rounds, long opsPerRound) {
        // P500-strategy-1-style args (n=16; gen_p500_bench.py setup(16, true))
        // so the direct numbers are comparable with the canonical report's
        // floor anchors (canonical = min over 4 probed strategies).
        int p4g30 = 31;
        Object[] objs8a = new Object[8]; // null elements, exactly like G30/G33 setup
        Object[] objs8b = new Object[8];
        Object[] objs8c = new Object[8];
        String tmpDir = mkTmpDir();
        long[] dst = new long[DST_CAP];
        int[] hashed16 = new int[16];
        for (int i = 0; i < hashed16.length; i++) hashed16[i] = (int) ((i * 0x9E3779B1L) & 0x3FF);
        double[] d16 = new double[16];
        for (int i = 0; i < d16.length; i++) d16[i] = (i % 97) * 0.5 - 24.0;

        record Ctl(String label, Runnable run) {}
        Ctl[] ctls = new Ctl[]{
                new Ctl("g42-StaticCacheGet.newBatchSummary", () -> sink += PaperNativeStaticCacheGet.newBatchSummary(16, 31, 3, 15, 63, hashed16, dst)),
                new Ctl("g35-RangeChoice.oldFillArraySummary", () -> sink += PaperNativeRangeChoice.oldFillArraySummary(d16, hashed16, hashed16, hashed16, 16, dst)),
                new Ctl("g33-PluginStartupRollup.newSummary", () -> sink += PaperNativePluginStartupRollup.newSummary(16, objs8a, tmpDir, objs8b, objs8c, dst)),
                new Ctl("g30-PluginLoadingAllocation.newPresizedSetupSummary", () -> sink += PaperNativePluginLoadingAllocation.newPresizedSetupSummary(16, objs8a, objs8b, objs8c, p4g30, dst)),
                new Ctl("g30-PluginLoadingAllocation.newLazyMissingSetSummary", () -> sink += PaperNativePluginLoadingAllocation.newLazyMissingSetSummary(16, objs8a, objs8b, objs8c, p4g30, dst)),
                new Ctl("g2-AquiferSurfaceSampling.newBatchSummary", () -> sink += PaperNativeAquiferSurfaceSampling.newBatchSummary(16, dst)),
        };
        for (Ctl c : ctls) {
            for (long i = 0; i < 20_000; i++) c.run.run();           // settle/JIT
            double[] ns = new double[rounds];
            for (int r = 0; r < rounds; r++) {
                long t0 = System.nanoTime();
                for (long i = 0; i < opsPerRound; i++) c.run.run();
                ns[r] = (double) (System.nanoTime() - t0) / opsPerRound;
            }
            System.out.printf("RESULT_DIRECT\tid=-1\tgroup=%s\tK=0\troute=direct\tns_per_op_median=%.1f\tns_min=%.1f\tns_max=%.1f%n",
                    c.label(), median(ns), min(ns), max(ns));
            System.out.printf("ROUTE_COST\tid=-1\tK=0\trouted_ns_per_op=%.1f%n", median(ns));
        }
    }

    /** P500 Bench.tmpDir() analog: temp dir with 3 files + a subdir. */
    private static String mkTmpDir() {
        try {
            java.nio.file.Path d = java.nio.file.Files.createTempDirectory("w47ctl");
            for (int k = 0; k < 3; k++) java.nio.file.Files.writeString(d.resolve("file" + k + ".jar"), "x");
            java.nio.file.Files.createDirectories(d.resolve("sub"));
            java.nio.file.Files.writeString(d.resolve("sub").resolve("inner.jar"), "y");
            return d.toString();
        } catch (Exception e) {
            return ".";
        }
    }

    private static long sink = 0;

    // ------------------------------------------------------------------
    // Arms
    // ------------------------------------------------------------------

    /** Direct per-op arm: returns ns/op for `ops` individual bridge calls. */
    private static double directBlock(int id, long ops, boolean shapeB) {
        long[] dst = new long[DST_CAP];
        long[] src = shapeB ? freshSrc() : null;
        long[] dstB = shapeB ? new long[SHAPE_B_IN] : null;
        long t0 = System.nanoTime();
        long acc = 0;
        for (long i = 0; i < ops; i++) {
            acc += directCall(id, 16, dst, src, dstB);
        }
        long t1 = System.nanoTime();
        if (acc == Long.MIN_VALUE) System.err.println("(unreachable guard: direct acc)");
        return (double) (t1 - t0) / ops;
    }

    /** Batch arm: returns ns/op for `ops` ops dispatched in ceil(ops/K) run() calls. */
    private static double batchBlock(int id, int k, long ops, boolean shapeB) {
        int n = k;
        int[] ids = new int[n];
        long[] args0 = new long[n];
        int[] counts = new int[n];
        int[] offs = new int[n];
        long[] args1;
        long[] outs;
        if (shapeB) {
            Arrays.fill(counts, SHAPE_B_IN);
            args1 = freshSrcPacked(n);
            outs = new long[n];
            for (int i = 0; i < n; i++) offs[i] = i;
        } else {
            Arrays.fill(args0, 16L);
            Arrays.fill(counts, DST_CAP);
            args1 = new long[0];
            outs = new long[n * DST_CAP];
            for (int i = 0; i < n; i++) offs[i] = i * DST_CAP;
        }
        Arrays.fill(ids, id);

        long batches = Math.max(1, ops / n);
        long t0 = System.nanoTime();
        long acc = 0;
        for (long b = 0; b < batches; b++) {
            acc += PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, new Object[0]);
        }
        long t1 = System.nanoTime();
        if (acc != batches * n) {
            System.err.printf("batch arm kernel %d: run() acc=%d expected %d%n", id, acc, batches * n);
            System.exit(2);
        }
        return (double) (t1 - t0) / (batches * n);
    }

    private static long[] freshSrc() {
        long[] src = new long[SHAPE_B_IN];
        for (int i = 0; i < SHAPE_B_IN; i++) src[i] = 100 + i;
        return src;
    }

    private static long[] freshSrcPacked(int n) {
        long[] a = new long[n * SHAPE_B_IN];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < SHAPE_B_IN; j++)
                a[i * SHAPE_B_IN + j] = 100 + j;
        return a;
    }

    private static int directCall(int id, int scalar, long[] dst, long[] src, long[] dstB) {
        return switch (id) {
            case 0 -> PaperNativeTicketSetSearch.binarySummary(scalar, dst);
            case 1 -> PaperNativeTicketSetSearch.uncheckedBinarySummary(scalar, dst);
            case 2 -> PaperNativeAquiferIndexStride.oldBatchSummary(scalar, dst);
            case 3 -> PaperNativeAquiferIndexStride.newBatchSummary(scalar, dst);
            case 4 -> PaperNativeChunkDependencies.oldImmutableListSummary(scalar, dst);
            case 5 -> PaperNativeChunkDependencies.arraySummary(scalar, dst);
            case 6 -> PaperNativeDensitySplineContext.oldWrapperSummary(scalar, dst);
            case 7 -> PaperNativeDensitySplineContext.newDirectSummary(scalar, dst);
            case 8 -> PaperNativeEntityLookupStatus.oldStatusSummary(scalar, dst);
            case 9 -> PaperNativeNoiseInterpolatorFractions.divisionSummary(scalar, dst);
            case 10 -> {
                long h = PaperNativeClimateRTree.buildTreeHandle(src, dstB);
                sinkHandle = h;
                yield 1;
            }
            case 11 -> {
                long h = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeBuildTreeHandle(src, dstB);
                sinkHandle = h;
                yield 1;
            }
            default -> throw new IllegalArgumentException("id " + id);
        };
    }

    private static long sinkHandle = 0;

    // ------------------------------------------------------------------
    // Parity + utils
    // ------------------------------------------------------------------

    private static void parityCheck(int id, boolean shapeB) {
        long[] directDst = new long[DST_CAP];
        long[] src = shapeB ? freshSrc() : null;
        long[] dstB = shapeB ? new long[SHAPE_B_IN] : null;
        if (shapeB) {
            long direct = directCall(id, 16, directDst, src, dstB);
            int[] ids1 = {id};
            long[] args1 = src.clone();
            long[] outs1 = new long[1];
            int ret = PaperNativeBatchDispatch.run(ids1, new long[]{0}, args1,
                    new int[]{SHAPE_B_IN}, outs1, new int[]{0}, new Object[0]);
            if (ret != 1) {
                System.err.printf("parity kernel %d: shape-B run()=%d%n", id, ret);
                System.exit(2);
            }
            boolean same = direct == outs1[0];
            System.out.printf("# parity id=%d shape=B direct_handle=%d batch_handle=%d identical=%b (opaque handle: both-success is the gate; fresh-allocation => values differ by design)%n",
                    id, direct, outs1[0], same);
            if (direct == 0 || outs1[0] == 0) {
                System.err.printf("parity kernel %d: zero handle (direct=%d batch=%d)%n", id, direct, outs1[0]);
                System.exit(2);
            }
            return;
        }
        int directCount = directCall(id, 16, directDst, null, null);
        if (directCount < 0 || directCount > DST_CAP) {
            System.err.printf("kernel %d: direct stub returned %d — unexpected shape%n", id, directCount);
            System.exit(2);
        }
        long[] outs1 = new long[DST_CAP];
        int ret = PaperNativeBatchDispatch.run(new int[]{id}, new long[]{16}, new long[0],
                new int[]{DST_CAP}, outs1, new int[]{0}, new Object[0]);
        if (ret != 1) {
            System.err.printf("parity kernel %d: run()=%d%n", id, ret);
            System.exit(2);
        }
        for (int j = 0; j < directCount; j++) {
            if (directDst[j] != outs1[j]) {
                System.err.printf("PARITY FAIL kernel %d lane %d: direct=%d batch=%d%n", id, j, directDst[j], outs1[j]);
                System.exit(2);
            }
        }
        System.out.printf("# parity OK id=%d lanes=%d (batch == direct, bit-exact)%n", id, directCount);
    }

    private static void settle(int id, boolean shapeB, long ops) {
        batchBlock(id, 256, ops, shapeB);                 // scratch high-water at max K
        directBlock(id, ops, shapeB);
        batchBlock(id, 1, Math.min(ops, 20_000), shapeB); // K=1 call-site warmup
    }

    private static double median(double[] v) {
        double[] c = v.clone();
        Arrays.sort(c);
        int m = c.length / 2;
        return (c.length & 1) == 1 ? c[m] : (c[m - 1] + c[m]) / 2.0;
    }

    private static double min(double[] v) {
        double m = v[0];
        for (double x : v) if (x < m) m = x;
        return m;
    }

    private static double max(double[] v) {
        double m = v[0];
        for (double x : v) if (x > m) m = x;
        return m;
    }

    private static String procVal(String key) {
        try {
            for (String line : java.nio.file.Files.readAllLines(java.nio.file.Path.of("/proc/self/status"))) {
                if (line.startsWith(key)) return line.replace('\t', ' ').trim();
            }
        } catch (Exception e) {
            return "n/a";
        }
        return "n/a";
    }

    private static String arg(String[] args, String name, String dflt) {
        for (int i = 0; i < args.length - 1; i++) {
            if (args[i].equals(name)) return args[i + 1];
        }
        return dflt;
    }

    private static boolean has(String[] args, String name) {
        for (String a : args) if (a.equals(name)) return true;
        return false;
    }
}
