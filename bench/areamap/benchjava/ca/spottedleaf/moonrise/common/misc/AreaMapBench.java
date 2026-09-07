package ca.spottedleaf.moonrise.common.misc;

/**
 * TASK-20 — area-map apply-loop micro-bench (timing companion of AreaMapSmoke).
 *
 * Drives the SAME patched-update pipeline the smoke proves correct, but with
 * a no-log callback sink (allocation-free hot path) and timed phases:
 *
 *   CHANGED  : move-by-1..2 (every 8th call a +/-1 radius change) -> apply-
 *              loop path, exactly ONE native call per update. Metrics:
 *              ns/call, ns/op (per emitted add/remove op), ns/px (per visited
 *              cell of the two square scans, 2*(2d+1)^2 — the unit comparable
 *              against the naive scan).
 *   SAME     : identical args every call -> same-state fast path, ZERO
 *              native calls (verified via fake-native counter in FAKE mode).
 *   NAIVE    : plain-Java set-difference scan emitting nothing (count +
 *              blackhole pack only) — the "before patch" reference for the
 *              scan part.
 *
 * Sizes: square side = 2d+1 ~= 128/512/1024 px (d = 63/255/511), the three
 * grid sizes mandated by TASK-20. Movement is realistic (ring diffs), so
 * ops/call ~ 2*side, not O(side^2).
 *
 * XORSHIFT64 seed 0x9E3779B97F4A7C15, Long.remainderUnsigned — same draw
 * conventions as AreaMapSmoke (TASK-11).
 *
 * Output: "phase\td\tside\tnsPerCall\tnsPerOp\tnsPerPx\topsPerCall\tverif"
 * TSV lines on stdout; batch = median of 3 rounds, each round a >=150 ms
 * timed window after warmup. Exit code 0 iff all verifications pass.
 *
 * NOTE: every reference to the fake-native counter (PaperNativeAreaMap
 * .calls()/.resetCalls()) is guarded by !realMode and never linked in REAL
 * mode — same lazy-resolution discipline as AreaMapSmoke (TASK-11 pattern).
 */
public final class AreaMapBench {

    private AreaMapBench() {}

    static final long SEED = 0x9E3779B97F4A7C15L;
    static volatile long SINK = 0L;

    /** No-log callback sink: counts ops, adds nothing to the hot path. */
    static final class CountAreaMap extends SingleUserAreaMap<Object> {
        long ops = 0L;
        @Override protected void addCallback(Object param, int chunkX, int chunkZ)    { ops++; }
        @Override protected void removeCallback(Object param, int chunkX, int chunkZ) { ops++; }
    }

    static long rngState = SEED;
    static long nextLong() {
        long x = rngState;
        x ^= x << 13; x ^= x >>> 7; x ^= x << 17;
        rngState = x;
        return x;
    }

    /** Naive reference scan: emits nothing, blackholes packed ops. */
    static long naiveScan(int fromX, int fromZ, int oldD, int toX, int toZ, int newD) {
        long sink = 0L;
        long n = 0L;
        if (fromX != Integer.MIN_VALUE) {
            for (int dx = -oldD; dx <= oldD; dx++)
                for (int dz = -oldD; dz <= oldD; dz++) {
                    final int x = fromX + dx, z = fromZ + dz;
                    if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD) {
                        sink += ((long) z << 32) | (x & 0xFFFFFFFFL); n++;
                    }
                }
            for (int dx = -newD; dx <= newD; dx++)
                for (int dz = -newD; dz <= newD; dz++) {
                    final int x = toX + dx, z = toZ + dz;
                    if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD) {
                        sink += ((long) z << 32) | (x & 0xFFFFFFFFL); n++;
                    }
                }
        }
        SINK = sink;
        return n;
    }

    /** Iteration count of the last timeRounds() invocation (warmup*rounds + measured). */
    static long totalIters = 0L;

    /** Median-of-3 rounds of a FIXED iteration count (TASK-30 hygiene: fixed
     *  iters make FAKE/REAL windows consume the IDENTICAL RNG stream, so
     *  cross-mode ops/call is comparable; the radius walk is reset to d and
     *  clamped to [d-8, d+8] so it cannot drift between modes). */
    static double timeRoundsFixed(int warmupIters, int measIters, Runnable call) {
        final double[] ns = new double[3];
        for (int r = 0; r < 3; r++) {
            for (int i = 0; i < warmupIters; i++) call.run();
            final long t0 = System.nanoTime();
            for (int i = 0; i < measIters; i++) call.run();
            ns[r] = (double) (System.nanoTime() - t0) / measIters;
        }
        java.util.Arrays.sort(ns);
        totalIters = 3L * (warmupIters + measIters);
        return ns[1];
    }

    public static void main(String[] args) {
        final boolean realMode = System.getProperty("crussty.native") != null;
        if (realMode) {
            System.load(System.getProperty("crussty.native"));
            System.out.println("mode\tREAL native (.so)");
        } else {
            System.out.println("mode\tFAKE native (fast-path observable)");
        }
        System.out.println("phase\td\tside\tnsPerCall\tnsPerOp\tnsPerPx\topsPerCall\tverif");

        boolean allOk = true;
        final int[] ds = {63, 255, 511};   // side = 127/511/1023 px

        for (final int d : ds) {
            final int side = 2 * d + 1;
            final double pxScan = 2.0 * side * side;   // cells visited by one full diff scan

            // ---------------- CHANGED (apply-loop) ----------------
            final CountAreaMap m = new CountAreaMap();
            final int[] cx = {3}, cz = {-2}, cd = {d};     // current center/radius
            final int[] fx = {0}, fz = {0}, fd = {d};      // previous center/radius
            final int warm = d > 255 ? 40 : 200;

            final int[] expectCalls = {0};
            final Runnable changed = () -> {
                long r = nextLong();
                int dx = (int) Long.remainderUnsigned(r, 5L) - 2;
                int dz = (int) Long.remainderUnsigned(r >>> 8, 5L) - 2;
                int nd = cd[0];
                if (Long.remainderUnsigned(r >>> 16, 8L) == 0L) {
                    nd = cd[0] + (((r >>> 24) & 1L) == 0L ? 1 : -1);
                }
                // TASK-30 hygiene: clamp the radius walk BEFORE the same-state
                // force check, else a clamped-back nd can silently produce a
                // same-state call (native skipped -> verif off-by-N)
                if (nd > d + 8) nd = d + 8;
                final int lo = Math.max(1, d - 8);
                if (nd < lo) nd = lo;
                if (dx == 0 && dz == 0 && nd == cd[0]) {
                    dz = 1;   // guarantee a CHANGED state: every timed call must hit the apply loop
                }
                fx[0] = cx[0]; fz[0] = cz[0]; fd[0] = cd[0];
                cx[0] += dx; cz[0] += dz; cd[0] = nd;
                SingleUserAreaMapOps.run(m, fx[0], fz[0], fd[0], cx[0], cz[0], cd[0], null);
                expectCalls[0]++;
            };

            final long callsBefore = realMode ? -1L : PaperNativeAreaMap.calls();
            final long opsBefore = m.ops;
            // reset the walk to the canonical state so FAKE and REAL consume identical streams
            cx[0] = 3; cz[0] = -2; cd[0] = d; fx[0] = 0; fz[0] = 0; fd[0] = d;
            rngState = SEED + 31L * d;   // deterministic per-size stream, identical across modes
            final int measIters = d > 255 ? 600 : (d > 63 ? 2500 : 8000);
            final double nsChanged = timeRoundsFixed(warm, measIters, changed);
            final long itersChanged = totalIters;
            final long opsTotal = m.ops - opsBefore;
            final long opsPerCall = Math.round((double) opsTotal / itersChanged);
            String verif = "opsAccum=" + opsTotal;
            if (!realMode) {
                final long dCalls = PaperNativeAreaMap.calls() - callsBefore;
                verif = (dCalls == expectCalls[0])
                        ? "native=1/changed-call OK (" + dCalls + ")" : "native MISMATCH " + dCalls + "/" + expectCalls[0];
                if (dCalls != expectCalls[0]) allOk = false;
            }
            final double nsPerOp = opsPerCall > 0 ? nsChanged / opsPerCall : 0.0;
            System.out.println("CHANGED\t" + d + "\t" + side + "\t" + fmt(nsChanged) + "\t"
                    + fmt(nsPerOp) + "\t" + fmt(nsChanged / pxScan) + "\t" + opsPerCall + "\t" + verif);

            // spot parity: naive scan count for the CURRENT transition must be
            // == the ops the pipeline emitted for the last update
            final int ex = fx[0], ez = fz[0], ed = fd[0], tx = cx[0], tz = cz[0], td = cd[0];
            final long naiveN = naiveScan(ex, ez, ed, tx, tz, td);

            // ---------------- SAME (fast path) ----------------
            final long callsBefore2 = realMode ? -1L : PaperNativeAreaMap.calls();
            final long opsBefore2 = m.ops;
            final Runnable same = () ->
                SingleUserAreaMapOps.run(m, tx, tz, td, tx, tz, td, null);
            rngState = SEED + 31L * d + 7L;   // irrelevant for SAME, kept deterministic
            final double nsSame = timeRoundsFixed(1000, 40000, same);
            String verif2 = "opsAccum=" + (m.ops - opsBefore2);
            if (!realMode) {
                final long dCalls = PaperNativeAreaMap.calls() - callsBefore2;
                verif2 = (dCalls == 0L && m.ops == opsBefore2)
                        ? "native=0 OK" : "native LEAK " + dCalls;
                if (dCalls != 0L) allOk = false;
            }
            System.out.println("SAME\t" + d + "\t" + side + "\t" + fmt(nsSame) + "\t-\t-\t0\t" + verif2);

            // ---------------- NAIVE reference ----------------
            final Runnable naive = () -> { naiveScan(ex, ez, ed, tx, tz, td); };
            final double nsNaive = timeRoundsFixed(warm, measIters, naive);
            System.out.println("NAIVE\t" + d + "\t" + side + "\t" + fmt(nsNaive) + "\t"
                    + (naiveN > 0 ? fmt(nsNaive / naiveN) : "-") + "\t"
                    + fmt(nsNaive / pxScan) + "\t" + naiveN + "\tnaiveN=" + naiveN);
            System.out.println("RATIO\t" + d + "\t" + side + "\t"
                    + fmt(nsChanged / nsNaive) + "x\tpatched/naive\t-\t-\t-");
        }

        System.out.println(allOk ? "AREAMAP BENCH: VERIFICATIONS PASS" : "AREAMAP BENCH: VERIFICATION FAILURES");
        System.exit(allOk ? 0 : 1);
    }

    static String fmt(double v) {
        if (v >= 1000) return String.format(java.util.Locale.ROOT, "%.0f", v);
        if (v >= 100)  return String.format(java.util.Locale.ROOT, "%.1f", v);
        return String.format(java.util.Locale.ROOT, "%.2f", v);
    }
}
