import crussty.batch.PaperNativeBatchDispatch;

/**
 * TASK-50 (D1) paired probe — shape-B staging: old (bulk args1->arena->in_arr,
 * unbounded per-thread arena) vs new (per-op args1->in_stage->in_arr, bounded
 * by IN_SCRATCH_CAP). agent-7625532f.
 *
 * NOT a kernel-quality bench: ids 10/11 (PaperNativeClimateRTree build
 * handles, closed .so) reject synthesized input (ret=8/op, no exception —
 * BATCH_ROLLOUT_AB mechanical precedent). Both arms run the identical closed
 * reject-paths, so the measured DIFF is the dispatcher's shape-B staging
 * path only. That is exactly what D1 changed.
 *
 * Cells: OPS=64 shape-B ops, len in {64, 1024, 4096} (= IN_SCRATCH_CAP).
 *   old: per batch  memset(total_in) + memcpy(total_in) bulk + 64 back-copies
 *        + arena high-water retained per thread (len=4096: 2 MiB/thread!)
 *   new: per batch  64x (GetLongArrayRegion src-offset + SetLongArrayRegion)
 *        + in_stage high-water 32 KiB/thread max, lazy
 * Threads: 4 long-lived (arena/scratch is per-thread — RSS retention shows).
 * Output: TSV lines on stdout (arm prefixed by the runner), plus RSS block.
 *
 * Exit codes: 0 ok, 2 sanity failure (ret != OPS or abi mismatch).
 */
public class D1StagingProbe {

    static final int OPS = 64;          // shape-B ops per batch
    static final int WARM = 20;         // warmup batches per cell
    static final int ROUNDS = 41;       // timed batches per cell (odd -> true median)
    static final int THREADS = 4;
    static final int[] LENS = {64, 1024, 4096};
    static final int KERNEL_ID = 10;    // PaperNativeClimateRTree.buildTreeHandle ([J[J)J

    public static void main(String[] args) throws Exception {
        // Same load contract as BatchFloorBench: CLOSED lib via the JVM first
        // (binds the direct stubs / kernel symbols), then the module .so (the
        // Rust-side dlopen of the same path just bumps the refcount).
        String lib = arg(args, "--lib", null);
        String nativeLib = arg(args, "--native", null);
        if (lib == null || nativeLib == null) {
            System.out.println("# FAIL usage: D1StagingProbe --lib <libcrussty.so> --native <libpaper_native_jni.so>");
            System.exit(2);
        }
        System.load(nativeLib);
        System.load(lib);

        int abi = PaperNativeBatchDispatch.abiVersion();
        // Sanity gate (post-spike the table grew: KERNEL_COUNT 15 -> abi
        // 131087; the probe only needs the A'-era floor + identical arms).
        if (abi < ((2 << 16) | 14)) {
            System.out.println("# FAIL abi=" + abi + " expected>= " + ((2 << 16) | 14));
            System.exit(2);
        }
        System.out.println("# D1StagingProbe abi=" + abi
                + " ops=" + OPS + " warm=" + WARM + " rounds=" + ROUNDS
                + " threads=" + THREADS + " lens=" + java.util.Arrays.toString(LENS));

        long[] rssBefore = rssKb();
        StringBuilder tsv = new StringBuilder();

        // Workers park ALIVE after their burst — the D1 hazard is retention
        // on LONG-LIVED (pooled) threads; measuring after join() would let
        // the thread-locals destruct and hide it.
        java.util.concurrent.CountDownLatch parked = new java.util.concurrent.CountDownLatch(THREADS);
        java.util.concurrent.CountDownLatch release = new java.util.concurrent.CountDownLatch(1);
        Thread[] workers = new Thread[THREADS];
        final Throwable[] err = new Throwable[1];
        for (int t = 0; t < THREADS; t++) {
            final int tid = t;
            workers[t] = new Thread(() -> {
                try { runCells(tid, tsv); parked.countDown(); release.await(); }
                catch (Throwable e) { err[0] = e; parked.countDown(); }
            }, "d1-worker-" + t);
            workers[t].start();
        }
        parked.await();
        if (err[0] != null) {
            System.out.println("# FAIL worker exception " + err[0]);
            System.exit(2);
        }
        Thread.sleep(300);
        long[] rssAlive = rssKb();
        release.countDown();
        for (Thread w : workers) w.join();

        System.out.println("# RSS_KB before=" + rssBefore[0]
                + " alive_after_burst=" + rssAlive[0]
                + " hwm=" + rssAlive[1]
                + " delta_alive=" + (rssAlive[0] - rssBefore[0]));
        System.out.print(tsv);
    }

    static String arg(String[] a, String name, String dflt) {
        for (int i = 0; i < a.length - 1; i++) if (a[i].equals(name)) return a[i + 1];
        return dflt;
    }

    /** One worker: all cells; appends "cell\tlen\tns_median\t..." rows. */
    static void runCells(int tid, StringBuilder tsv) {
        for (int len : LENS) {
            int[] ids = new int[OPS];
            java.util.Arrays.fill(ids, KERNEL_ID);
            int[] counts = new int[OPS];
            java.util.Arrays.fill(counts, len);
            int[] offs = new int[OPS];
            for (int i = 0; i < OPS; i++) offs[i] = i;
            long[] args0 = new long[0];                 // shape B: 0-wide scalar plane
            long[] args1 = new long[OPS * len];         // packed prefix, filled
            for (int i = 0; i < args1.length; i++) args1[i] = 0x434C494D5254L + i;
            long[] outs = new long[OPS];

            // warmup + sanity
            for (int r = 0; r < WARM; r++) {
                int ret = PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, new Object[0]);
                if (ret != OPS) throw new IllegalStateException(
                        "ret=" + ret + " expected=" + OPS + " tid=" + tid + " len=" + len);
            }
            // timed
            long[] ns = new long[ROUNDS];
            for (int r = 0; r < ROUNDS; r++) {
                long t0 = System.nanoTime();
                PaperNativeBatchDispatch.run(ids, args0, args1, counts, outs, offs, new Object[0]);
                ns[r] = System.nanoTime() - t0;
            }
            java.util.Arrays.sort(ns);
            long med = ns[ROUNDS / 2];
            long p25 = ns[ROUNDS / 4];
            long p75 = ns[(3 * ROUNDS) / 4];
            synchronized (tsv) {
                tsv.append("RESULT\ttid=").append(tid).append("\tlen=").append(len)
                   .append("\tns_median=").append(med)
                   .append("\tns_p25=").append(p25)
                   .append("\tns_p75=").append(p75)
                   .append("\ttotal_in_longs=").append((long) OPS * len)
                   .append('\n');
            }
        }
    }

    static long[] rssKb() throws Exception {
        long vmrss = -1, hwm = -1;
        for (String line : java.nio.file.Files.readAllLines(
                java.nio.file.Paths.get("/proc/self/status"))) {
            if (line.startsWith("VmRSS:")) vmrss = Long.parseLong(line.replaceAll("\\D+", ""));
            else if (line.startsWith("VmHWM:")) hwm = Long.parseLong(line.replaceAll("\\D+", ""));
        }
        return new long[]{vmrss, hwm};
    }
}
