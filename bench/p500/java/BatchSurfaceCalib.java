import java.util.Arrays;

/** TASK-51 — batch-surface calibration driver (PROVEN_WINS_SYNC §4.1 closure).
 *
 *  Measures the 5 PROVEN_WINS batch-surface kernels that have NO old/new pair
 *  in the canonical P500 rerun, using the P500 methodology (120 ms time-bounded
 *  batches, median of 5 rounds, 2 discarded warmups, DCE-proof sink, REAL
 *  closed-source .so loaded by exact path):
 *
 *    group "ticketset":  PaperNativeTicketSetSearch.binarySummary(int,long[])
 *                        PaperNativeTicketSetSearch.uncheckedBinarySummary(int,long[])
 *    group "fractions":  PaperNativeNoiseInterpolatorFractions.divisionSummary(int,long[])
 *    group "rtree":      PaperNativeClimateRTree.buildTreeHandle(long[],long[])
 *                        - lifecycle variant: build+checksum+free per op
 *                        - build-only variant: K=256 builds timed, frees untimed
 *    group "rtree-full": net.minecraft.world.level.biome.PaperNativeClimateRTree
 *                        .nativeBuildTreeHandle(long[],long[]) (full-name form, id 11)
 *
 *  Why a dedicated driver instead of groups.tsv rows (documented decision):
 *    1. buildTreeHandle returns a native handle — the generic p500.Bench driver
 *       never frees it, so a 120 ms batch would leak thousands of native R-trees.
 *    2. TicketSetSearch.binarySummary has cross-call state (per-call cost
 *       saturates under repetition) — needs a single-shot cold probe plus a
 *       per-round trajectory, which the paired-groups driver does not record.
 *    3. Appending rows to the generated groups.tsv would renumber nothing today
 *       (append-only) but couples the canonical harness to lifecycle-aware
 *       measurement; the calibration report is referenced from the registry instead.
 *
 *  Honest boundaries: closed-source kernel semantics are opaque; args are fixed
 *  deterministic values (documented per group); results calibrate the REGISTERED
 *  SURFACE at those inputs — they are not a gameplay-cost claim and no gate input
 *  derives from them (verdicts stay "P500 PARITY (batch surface)").
 *
 *  Output: one TSV line per (method, variant) on stdout:
 *    CALIB \t group \t fqcn.method \t sig \t variant \t median_ns_op \t r1..r5
 *           \t cv_pct \t calls_last_round \t single_first_ns
 *  Crash: CALIB_CRASH \t group \t exception  (exit 3), one JVM per group.
 */
public class BatchSurfaceCalib {

    static long SINK = 0;

    static final long BATCH_NS = 120_000_000L; // 120 ms — canonical P500 REAL batch
    static final int WARM = 2;
    static final int ROUNDS = 5;
    static final int RTREE_BUILD_K = 256;      // builds per build-only round (bounded live handles)

    public static void main(String[] args) {
        for (String p : System.getProperty("p500.libs", "").split(java.io.File.pathSeparator))
            if (!p.isBlank()) System.load(p);

        String group = args.length > 0 ? args[0] : "";
        try {
            switch (group) {
                case "ticketset" -> ticketset();
                case "fractions" -> fractions();
                case "rtree" -> rtree(false);
                case "rtree-full" -> rtree(true);
                default -> { System.out.println("CALIB_CRASH\t?\tunknown group: " + group); System.exit(2); }
            }
        } catch (Throwable t) {
            System.out.println("CALIB_CRASH\t" + group + "\t" + t);
            System.exit(3);
        }
        System.out.println("SINK\t" + SINK); // DCE-proof: everything feeds this
    }

    // ---------------------------------------------------------------- ticketset
    // Cross-call state expected: single-shot cold probe + per-round trajectory
    // expose the saturation shape instead of hiding it behind one median.
    static void ticketset() {
        final int a0 = 1000;                       // value used by src/lib.rs::live_proof
        final long[] a1 = new long[256];
        for (int i = 0; i < a1.length; i++) a1[i] = 0x9E3779B97F4A7C15L * i;

        String[] names = {"binarySummary", "uncheckedBinarySummary"};
        for (int m = 0; m < 2; m++) {
            final int idx = m;
            // cold single-shot (first JNI transition of this method in this JVM)
            long t0 = System.nanoTime();
            SINK += (idx == 0 ? PaperNativeTicketSetSearch.binarySummary(a0, a1)
                              : PaperNativeTicketSetSearch.uncheckedBinarySummary(a0, a1));
            long singleNs = System.nanoTime() - t0;

            double[] r = batch(() -> SINK += (idx == 0
                    ? PaperNativeTicketSetSearch.binarySummary(a0, a1)
                    : PaperNativeTicketSetSearch.uncheckedBinarySummary(a0, a1)));
            emit("ticketset", "PaperNativeTicketSetSearch." + names[idx], "(I[J)I",
                 "steady-state", r, -1, singleNs);
        }
    }

    // ---------------------------------------------------------------- fractions
    static void fractions() {
        final int a0 = 64;
        final long[] a1 = new long[64];
        for (int i = 0; i < a1.length; i++) a1[i] = (long) i * 0x5DEECE66DL;

        long t0 = System.nanoTime();
        SINK += PaperNativeNoiseInterpolatorFractions.divisionSummary(a0, a1);
        long singleNs = System.nanoTime() - t0;

        double[] r = batch(() -> SINK += PaperNativeNoiseInterpolatorFractions.divisionSummary(a0, a1));
        emit("fractions", "PaperNativeNoiseInterpolatorFractions.divisionSummary", "(I[J)I",
             "steady-state", r, -1, singleNs);
    }

    // ------------------------------------------------------------------- rtree
    // Full-name form (id 11) lives in a real package; short form (id 10) is the
    // default-package stub. Same measurement, flag picks the class.
    static void rtree(boolean fullName) {
        final long[] a0 = new long[64];
        final long[] a1 = new long[256];
        for (int i = 0; i < a0.length; i++) a0[i] = 0xD1B54A32D192ED03L * i;
        for (int i = 0; i < a1.length; i++) a1[i] = 0x9E3779B97F4A7C15L * (i + 7);

        if (fullName) {
            long t0 = System.nanoTime();
            long h = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeBuildTreeHandle(a0, a1);
            long singleNs = System.nanoTime() - t0;

            // lifecycle: build + checksum + free per op (all three timed; upper bound)
            double[] lc = batch(() -> {
                long hh = net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeBuildTreeHandle(a0, a1);
                SINK += net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeChecksumTreeHandle(hh);
                net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeFreeTreeHandle(hh);
            });
            emit("rtree-full", "net/minecraft/.../PaperNativeClimateRTree.nativeBuildTreeHandle", "([J[J)J",
                 "lifecycle(b+c+f)", lc, -1, singleNs);

            // build-only: K builds timed, frees untimed
            double[] bo = buildOnly(hh -> {
                SINK += hh;
                net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeFreeTreeHandle(hh);
            }, () -> net.minecraft.world.level.biome.PaperNativeClimateRTree.nativeBuildTreeHandle(a0, a1));
            emit("rtree-full", "net/minecraft/.../PaperNativeClimateRTree.nativeBuildTreeHandle", "([J[J)J",
                 "build-only(K=" + RTREE_BUILD_K + ")", bo, -1, singleNs);
        } else {
            long t0 = System.nanoTime();
            long h = PaperNativeClimateRTree.buildTreeHandle(a0, a1);
            long singleNs = System.nanoTime() - t0;

            double[] lc = batch(() -> {
                long hh = PaperNativeClimateRTree.buildTreeHandle(a0, a1);
                SINK += PaperNativeClimateRTree.checksumTreeHandle(hh);
                PaperNativeClimateRTree.freeTreeHandle(hh);
            });
            emit("rtree", "PaperNativeClimateRTree.buildTreeHandle", "([J[J)J",
                 "lifecycle(b+c+f)", lc, -1, singleNs);

            double[] bo = buildOnly(hh -> {
                SINK += hh;
                PaperNativeClimateRTree.freeTreeHandle(hh);
            }, () -> PaperNativeClimateRTree.buildTreeHandle(a0, a1));
            emit("rtree", "PaperNativeClimateRTree.buildTreeHandle", "([J[J)J",
                 "build-only(K=" + RTREE_BUILD_K + ")", bo, -1, singleNs);
        }
    }

    // ------------------------------------------------------------ shared timing
    interface Op { void run(); }

    /** Canonical P500 batch loop: WARM discarded batches, ROUNDS measured
     *  120 ms time-bounded batches; returns per-round ns/op. */
    static double[] batch(Op op) {
        for (int w = 0; w < WARM; w++) runBatch(op);
        double[] r = new double[ROUNDS];
        for (int i = 0; i < ROUNDS; i++) r[i] = runBatch(op);
        return r;
    }

    static double runBatch(Op op) {
        long calls = 0, acc = 0;
        long t0 = System.nanoTime(), el;
        do { op.run(); calls++; el = System.nanoTime() - t0; } while (el < BATCH_NS);
        LAST_CALLS = calls;
        return (double) el / calls;
    }

    static long LAST_CALLS;

    /** Build-only rounds: K builds inside the timed window, checksum of the last
     *  handle as DCE sink, all K frees AFTER the window (bounded live set = K). */
    interface BuildOp { long build(); }

    static double[] buildOnly(java.util.function.LongConsumer free, BuildOp build) {
        for (int w = 0; w < WARM; w++) buildOnlyRound(free, build);
        double[] r = new double[ROUNDS];
        for (int i = 0; i < ROUNDS; i++) r[i] = buildOnlyRound(free, build);
        return r;
    }

    static double buildOnlyRound(java.util.function.LongConsumer free, BuildOp build) {
        long[] slots = new long[RTREE_BUILD_K];
        long t0 = System.nanoTime();
        for (int i = 0; i < RTREE_BUILD_K; i++) slots[i] = build.build();
        long el = System.nanoTime() - t0;
        SINK += slots[RTREE_BUILD_K - 1];
        for (int i = 0; i < RTREE_BUILD_K; i++) { if (slots[i] != 0) free.accept(slots[i]); }
        LAST_CALLS = RTREE_BUILD_K;
        return (double) el / RTREE_BUILD_K;
    }

    static void emit(String group, String kernel, String sig, String variant,
                     double[] rounds, long callsOverride, long singleNs) {
        double[] s = rounds.clone();
        Arrays.sort(s);
        double median = s[s.length / 2];
        double mean = Arrays.stream(rounds).average().orElse(0);
        double cv = mean > 0 ? 100.0 * stddev(rounds, mean) / mean : 0;
        StringBuilder sb = new StringBuilder("CALIB\t").append(group).append('\t').append(kernel)
                .append('\t').append(sig).append('\t').append(variant)
                .append('\t').append(String.format("%.1f", median));
        for (double r : rounds) sb.append('\t').append(String.format("%.1f", r));
        sb.append('\t').append(String.format("%.2f", cv))
          .append('\t').append(callsOverride >= 0 ? callsOverride : LAST_CALLS)
          .append('\t').append(singleNs);
        System.out.println(sb);
    }

    static double stddev(double[] v, double mean) {
        double acc = 0;
        for (double x : v) acc += (x - mean) * (x - mean);
        return Math.sqrt(acc / v.length);
    }
}
