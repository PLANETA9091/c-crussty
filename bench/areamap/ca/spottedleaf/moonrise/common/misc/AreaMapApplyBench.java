package ca.spottedleaf.moonrise.common.misc;

import java.lang.reflect.Method;
import java.util.Arrays;

/**
 * Headless micro-bench for the area-map bridge (TASK-20):
 * apply-loop (native enumerate + apply callbacks) vs same-state fast path,
 * ns/px at nominal grid sizes 128/512/1024 (d = 63/255/511, side = 2d+1).
 *
 * Follows the bench/areamap conventions from TASK-11 (AreaMapSmoke): the fake
 * PaperNativeAreaMap (pure-Java enumeration, fast-path observable via a call
 * counter) runs in classes-bench-fake; the real libpaper_native_jni.so runs
 * via -Dcrussty.native in classes-bench-real. No build-system changes, no new
 * dependencies (plain java + javac only).
 *
 * Patterns (opaque byte[] mask, one update per mask element):
 *   SAME    - every update same-state (fromX==toX && fromZ==toZ && oldD==newD)
 *             -> exercises ONLY the fast path (0 native calls, 0 callbacks);
 *   MIX50   - alternating same-state / 1-chunk-move updates;
 *   CHANGED - every update a 1-chunk move along an 8-direction cycle (the
 *             realistic "player walking" shape: both squares enumerated,
 *             difference ~2..4 columns of ops).
 *
 * Opacity: mask bytes are built under a synchronized block (never inlined)
 * and mixed with an opaque runtime zero (o = z63*(z63-1), z63 = nanoTime>>>63
 * - == 0 at runtime, not foldable by C2), so the fast-path branch inside
 * run() can never be statically folded and the timed loop can never be
 * eliminated even when every update is same-state. A dependent sink sum is
 * accumulated and printed.
 *
 * Methodology per (size x pattern): pre-probe (scratch grow + JNI resolve),
 * warmup run, re-probe after JIT, then RUNS=5 timed runs (System.nanoTime);
 * median and min reported. Adaptive N targets ~0.35 s per timed run (SAME
 * pattern uses a fixed 8M updates - the fast path is ~sub-ns cheap).
 *
 * REAL-only forensic probe (PROBE lines): calls the native DIRECTLY with
 * caller buffers of len = cap and 4*cap for the SAME difference workload -
 * distinguishes "per-cell enumeration cost" (flat vs len) from an O(len)
 * per-call buffer-touch (JNI copy-in/out or internal memset), which would
 * dominate large-grid apply-loop cost. Also reports whether the native
 * writes beyond n into the ops buffer (sentinel check).
 *
 * ns/px uses px = (2d+1)^2 (actual cells per square). Driver-loop overhead
 * (opaque mask load + state update) is included in ALL numbers, so the
 * fast-path ns/update is an upper bound on the bridge fast path itself.
 *
 * NOT a CI gate - run manually: bench/areamap/run_bench.sh
 * Output: TSV-ish lines (SANITY/PROBE/BENCH/PREDICT/SPEEDUP/...) on stdout.
 */
public final class AreaMapApplyBench {

    private AreaMapApplyBench() {}

    /** Minimum-shape callback target: counters only, zero allocation. */
    static final class CountingMap extends SingleUserAreaMap<Object> {
        long adds, removes;
        @Override protected void addCallback(final Object param, final int chunkX, final int chunkZ) { this.adds++; }
        @Override protected void removeCallback(final Object param, final int chunkX, final int chunkZ) { this.removes++; }
        long total() { return this.adds + this.removes; }
    }

    private static final Object PARAM = new Object();
    private static final Object PATTERNS_LOCK = new Object();

    private static final boolean REAL_MODE = System.getProperty("crussty.native") != null;

    // The call counter exists only in the FAKE variant of PaperNativeAreaMap
    // (the realdecl variant is a pure native declaration), so counting is
    // done reflectively - the timed loops never reference the class directly.
    private static final Method RESET_CALLS, CALLS;
    static {
        Method r = null, c = null;
        if (!REAL_MODE) {
            try {
                final Class<?> k = Class.forName("ca.spottedleaf.moonrise.common.misc.PaperNativeAreaMap");
                r = k.getDeclaredMethod("resetCalls");
                c = k.getDeclaredMethod("calls");
            } catch (final ReflectiveOperationException e) {
                throw new ExceptionInInitializerError(e);
            }
        }
        RESET_CALLS = r;
        CALLS = c;
    }

    static final int[] DS = {63, 255, 511};              // side 127/511/1023 -> nominal 128/512/1024
    static final String[] PATTERNS = {"SAME", "MIX50", "CHANGED"};
    static final int RUNS = 5;
    static final long TARGET_NS = 350_000_000L;          // ~0.35 s per timed run
    static final int N_SAME = 8_000_000;                 // fast-path blocks: fixed update count
    static final int N_MIN = 16, N_MAX = 200_000;

    // 8-direction walk cycle; sums to (0,0) so the square stays near origin.
    private static final int[] DIRX = {1, 1, 0, -1, -1, -1, 0, 1};
    private static final int[] DIRZ = {0, 1, 1, 1, 0, -1, -1, -1};

    /**
     * Opaque per-update same-state mask. synchronized => never inlined, the
     * allocation cannot be scalar-replaced at call sites; the opaque runtime
     * zero o keeps per-element values unfoldable even under full propagation.
     * Runtime semantics: SAME -> all 1, CHANGED -> all 0, MIX50 -> i&1.
     */
    static byte[] pattern(final String p) {
        synchronized (PATTERNS_LOCK) {
            final byte[] out = new byte[1024];
            final long z63 = System.nanoTime() >>> 63;   // 0 or 1, unknown to C2
            final long o = z63 * (z63 - 1);              // == 0 at runtime, opaque
            for (int i = 0; i < out.length; i++) {
                out[i] = (byte) ((p.equals("SAME") ? 1 : (p.equals("CHANGED") ? 0 : (i & 1))) + o);
            }
            return out;
        }
    }

    static long expectedCalls(final byte[] pat, final int n) {
        long out = 0L;
        for (int i = 0; i < n; i++) {
            if (pat[i & (pat.length - 1)] == 0) out++;
        }
        return out;
    }

    /** One drive of n updates; returns sink (anti-DCE token). */
    static long drive(final CountingMap map, final int d, final byte[] pat, final int n) {
        int cx = 0, cz = 0, dir = 0;
        long sink = 0L;
        for (int i = 0; i < n; i++) {
            final boolean same = pat[i & (pat.length - 1)] != 0;
            int tx = cx, tz = cz;
            if (!same) {
                final int di = dir++ & 7;
                tx = cx + DIRX[di];
                tz = cz + DIRZ[di];
            }
            SingleUserAreaMapOps.run(map, cx, cz, d, tx, tz, d, PARAM);
            cx = tx; cz = tz;
            sink += tx + tz;
        }
        return sink;
    }

    static int clampN(final double perUpdateNs) {
        final long n = (long) ((TARGET_NS / Math.max(1.0d, perUpdateNs)) + 0.5d);
        return (int) Math.max(N_MIN, Math.min(N_MAX, n));
    }

    static void invokeV(final Method m) {   // void methods: invoke without unboxing
        try {
            m.invoke(null);
        } catch (final Exception e) {
            throw new RuntimeException(e);
        }
    }

    static long invoke0(final Method m) {   // long methods: box + unbox
        try {
            return ((Long) m.invoke(null)).longValue();
        } catch (final Exception e) {
            throw new RuntimeException(e);
        }
    }

    static double median(final double[] per) {
        final double[] srt = per.clone();
        Arrays.sort(srt);
        return srt[per.length / 2];
    }

    static double minimum(final double[] per) {
        final double[] srt = per.clone();
        Arrays.sort(srt);
        return srt[0];
    }

    /** ns/update over a 16-update probe. */
    static double timeProbe(final CountingMap map, final int d, final byte[] pat) {
        final long t0 = System.nanoTime();
        drive(map, d, pat, 16);
        return (System.nanoTime() - t0) / 16.0d;
    }

    static String mode() {
        return REAL_MODE ? "REAL" : "FAKE";
    }

    /**
     * REAL-only forensic: per-call native cost vs buffer length (len = cap vs
     * 4*cap) for the IDENTICAL difference workload, plus a sentinel check for
     * writes beyond n. Flat cost => per-cell enumeration; cost ~ len => O(len)
     * per-call buffer-touch (JNI copy or memset) dominates large grids.
     */
    static void nativeProbe() {
        for (final int d : new int[]{63, 511}) {
            final int cap = 2 * (2 * d + 1) * (2 * d + 1);
            for (final int mult : new int[]{1, 4}) {
                final int len = cap * mult;
                final byte[] ops = new byte[len];
                final long[] keys = new long[len];
                Arrays.fill(ops, (byte) 7);                     // sentinel
                final int n0 = PaperNativeAreaMap.nativeUpdateOpsBatch(0, 0, d, 1, 0, d, ops, keys);
                int non7 = 0;
                for (int i = n0; i < len; i++) {
                    if (ops[i] != 7) non7++;
                }
                Arrays.fill(ops, (byte) 7);
                for (int w = 0; w < 3; w++) {                    // warmup
                    PaperNativeAreaMap.nativeUpdateOpsBatch(0, 0, d, 1, 0, d, ops, keys);
                }
                final long t0 = System.nanoTime();
                PaperNativeAreaMap.nativeUpdateOpsBatch(0, 0, d, 1, 0, d, ops, keys);
                final double perCall = (System.nanoTime() - t0) / 1.0d;
                final int k = (int) Math.max(5L, Math.min(50_000L, (long) (200_000_000.0d / Math.max(1.0d, perCall))));
                final double[] per = new double[5];
                for (int r = 0; r < per.length; r++) {
                    final long s = System.nanoTime();
                    for (int c = 0; c < k; c++) {
                        PaperNativeAreaMap.nativeUpdateOpsBatch(0, 0, d, 1, 0, d, ops, keys);
                    }
                    per[r] = (System.nanoTime() - s) / (double) k;
                }
                System.out.println(String.format(
                    "PROBE mode=%s d=%d mult=%d len=%d n0=%d median_ns_per_call=%.1f writes_beyond_n=%d",
                    mode(), d, mult, len, n0, median(per), non7));
            }
        }
    }

    public static void main(final String[] args) {
        if (REAL_MODE) {
            System.load(System.getProperty("crussty.native"));
            System.out.println("mode: REAL natives (production enumerate + apply)");
        } else {
            System.out.println("mode: FAKE native (pure-Java enumeration reference)");
        }
        System.out.println("note: ns/px uses px=(2d+1)^2; driver-loop overhead included -> fast-path ns/update is an upper bound");

        final long tStart = System.nanoTime();

        // ---- sanity probe (both modes): d=32 cardinal move + same-state ----
        {
            final CountingMap m = new CountingMap();
            if (!REAL_MODE) invokeV(RESET_CALLS);
            SingleUserAreaMapOps.run(m, 0, 0, 32, 1, 0, 32, PARAM);   // cardinal move: 2*(2*32+1) ops
            SingleUserAreaMapOps.run(m, 7, 7, 32, 7, 7, 32, PARAM);   // same-state: no ops
            final long cb = m.total();
            final long calls = REAL_MODE ? -1L : invoke0(CALLS);
            System.out.println("SANITY mode=" + mode() + " d=32 callbacks=" + cb + " expected=130 calls=" + calls
                + " -> " + (cb == 130L && (REAL_MODE || calls == 1L) ? "OK" : "FAIL"));
        }

        if (REAL_MODE) {
            nativeProbe();
        }

        // ---- global JIT warmup (same bytecode as the timed loops) ----
        for (final String p : PATTERNS) {
            drive(new CountingMap(), 15, pattern(p), 20_000);
        }

        for (final int d : DS) {
            final double[] med = new double[PATTERNS.length];
            final double[] min = new double[PATTERNS.length];

            for (int pi = 0; pi < PATTERNS.length; pi++) {
                final String p = PATTERNS[pi];
                final byte[] pat = pattern(p);
                final boolean fast = p.equals("SAME");

                final CountingMap probeMap = new CountingMap();
                drive(probeMap, d, pat, 8);                       // scratch grow + JNI resolve
                int n = fast ? N_SAME : clampN(timeProbe(probeMap, d, pat));
                drive(probeMap, d, pat, n);                       // warmup run at N
                if (!fast) {
                    n = clampN(timeProbe(probeMap, d, pat));      // re-probe post-JIT
                    drive(probeMap, d, pat, n);                   // re-warm at final N
                }

                final double[] per = new double[RUNS];
                final long[] cbs = new long[RUNS];
                long calls = -1L, sinkTotal = 0L;
                for (int r = 0; r < RUNS; r++) {
                    final CountingMap m = new CountingMap();
                    if (!REAL_MODE) invokeV(RESET_CALLS);
                    final long t0 = System.nanoTime();
                    sinkTotal += drive(m, d, pat, n);
                    final long t1 = System.nanoTime();
                    per[r] = (t1 - t0) / (double) n;
                    cbs[r] = m.total();
                    if (!REAL_MODE) calls = invoke0(CALLS);
                }
                med[pi] = median(per);
                min[pi] = minimum(per);

                final long px = (2L * d + 1) * (2L * d + 1);
                System.out.println(String.format(
                    "BENCH mode=%s grid=%dx%d d=%d side=%d px=%d pattern=%s runs=%d N=%d"
                        + " median_ns_per_update=%.2f min_ns_per_update=%.2f"
                        + " median_ns_per_px=%.4g min_ns_per_px=%.4g callbacks_last=%d calls=%d sink=%d",
                    mode(), 2 * (d + 1), 2 * (d + 1), d, 2 * d + 1, px, p, RUNS, n,
                    med[pi], min[pi],
                    med[pi] / px, min[pi] / px,
                    cbs[RUNS - 1], calls, sinkTotal));

                if (REAL_MODE) {
                    System.out.println("CHECK mode=" + mode() + " pattern=" + p
                        + " skipped (no native call counter; TASK-11 S2 covers the 0-call proof)");
                } else {
                    final long exp = expectedCalls(pat, n);
                    System.out.println("CHECK mode=" + mode() + " pattern=" + p + " expected_calls=" + exp
                        + " actual_calls=" + calls + " -> " + (exp == calls ? "OK" : "FAIL"));
                }
            }

            // ---- cross-pattern summary for this size ----
            final double predictedMix = (med[0] + med[2]) / 2.0d;
            System.out.println(String.format(
                "PREDICT mode=%s grid=%dx%d pattern=MIX50 predicted_ns_per_update=%.1f measured=%.1f delta_pct=%.1f",
                mode(), 2 * (d + 1), 2 * (d + 1), predictedMix, med[1],
                (med[1] - predictedMix) * 100.0d / predictedMix));
            System.out.println(String.format(
                "SPEEDUP mode=%s grid=%dx%d apply_ns_per_update=%.2f fast_ns_per_update=%.2f"
                    + " apply_ns_per_px=%.4g fast_ns_per_px=%.4g apply_over_fast=%.0fx",
                mode(), 2 * (d + 1), 2 * (d + 1), med[2], med[0],
                med[2] / ((2L * d + 1) * (2L * d + 1)),
                med[0] / ((2L * d + 1) * (2L * d + 1)),
                med[2] / med[0]));
        }

        System.out.println(String.format("AREAMAP APPLY BENCH: DONE mode=%s total_s=%.1f",
            mode(), (System.nanoTime() - tStart) / 1e9));
    }
}
