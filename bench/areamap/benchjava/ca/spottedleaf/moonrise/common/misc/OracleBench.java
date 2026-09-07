package ca.spottedleaf.moonrise.common.misc;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

/**
 * TASK-30 — per-call correctness ORACLE for the area-map apply pipeline
 * (correctness-only: NO timing measurements, NO benchmark comparisons).
 *
 * Root-grounds the APPLY_BENCH_RESIZE_MIX.md ops-count anomaly
 * (REAL 645 vs FAKE 374 ops/call at d=63, d=255/511 within 4.6%):
 *
 *   S1 PURE-MOVE  : pure 1-chunk moves on a bounded 8-direction ring walk
 *                   (center returns to origin every 8 calls), fixed radius.
 *   S2 PURE-RESIZE: same center, radius ping-pong +/-1 (d -> d+1 -> d ...).
 *   S3 MIXED      : the anomaly mix — a 1-chunk move on EVERY call plus a
 *                   deterministic +/-1 resize on every 4th call (alternate
 *                   direction), radius bounded by the ping-pong.
 *                   d=63 primary; d=255/511 mixed spots.
 *
 * For EVERY call the naive (op, x, z) multiset (adds = New\Old, removes =
 * Old\New — TASK-11 conventions) is compared against the ops the pipeline
 * emitted via a fresh RecordingMap (order-preserving add/remove callback log,
 * compared as multisets per (op, cell)). Emission faults classify as:
 *
 *   DUPLICATES : an (op,cell) emitted MORE times than the naive diff has
 *   STALE      : an (op,cell) emitted that the naive diff does not contain
 *                at all (rows from previous transitions / garbage)
 *   STUB-DROP  : FAKE mode only — naive diff size EXCEEDS the offered
 *                scratch capacity (the stub's n < ops.length cap binds)
 *   PARITY     : none of the above — emitted == naive on every call
 *
 * The offered scratch capacity is probed per call by reflection into the
 * bridge's private ThreadLocal Scratch (post-run read == the array actually
 * handed to the native/stub for that call) and checked against the bridge's
 * grow contract: capacity >= maxOps(oldD,newD) = (2*oldD+1)^2 + (2*newD+1)^2,
 * the exact worst-case diff bound, BEFORE the native call — so a drop is
 * structurally impossible; the probe makes that observable.
 *
 * REPLAY section: replays the bench's exact CHANGED RNG stream at d=63
 * (XORSHIFT64 seed 0x9E3779B97F4A7C15, identical draw order and clamping)
 * and computes the exact naive diff size per call O(1) via square
 * intersection (cross-checked against full enumeration on the first 64
 * calls). Window means quantifying the anomaly come from the committed
 * apply_bench_raw.tsv iteration counts: FAKE window 52108 calls (native
 * counter), REAL window ~10003 calls (opsAccum 6451692 / opsPerCall 645
 * bounds iters to [9996, 10010]). This is stream arithmetic, not timing.
 *
 * Output: TSV per-call lines + per-stream summaries + final verdict line
 * "TASK-30 VERDICT: <classification>". Exit 0 iff parity holds on every
 * call (REAL mode is the correctness gate; FAKE must also be PARITY to
 * exit 0, since the stub is the parity reference).
 */
public final class OracleBench {

    private OracleBench() {}

    static int failures = 0;
    static boolean realMode;

    // ---------------- classification flags ----------------
    static boolean anyStubDrop   = false;
    static boolean anyDuplicates = false;
    static boolean anyStale      = false;
    static long    parityCalls   = 0L;
    static long    parityPass    = 0L;

    /** per-stream aggregates: [calls, pass, minCap, maxNaive] */
    static final Map<String, long[]> AGG = new HashMap<String, long[]>();

    // ---------------- recording map (TASK-11 conventions) ----------------
    /** Order-preserving callback log: parallel arrays of packed cell + op. */
    static final class OracleRec extends SingleUserAreaMap<Object> {
        long[] keys = new long[1024];
        byte[] ops  = new byte[1024];
        int n = 0;
        void push(long key, byte op) {
            if (n == keys.length) {
                keys = Arrays.copyOf(keys, n * 2);
                ops  = Arrays.copyOf(ops,  n * 2);
            }
            keys[n] = key; ops[n] = op; n++;
        }
        @Override protected void addCallback(Object param, int chunkX, int chunkZ) {
            push(pack(chunkX, chunkZ), (byte) 0);
        }
        @Override protected void removeCallback(Object param, int chunkX, int chunkZ) {
            push(pack(chunkX, chunkZ), (byte) 1);
        }
    }

    static long pack(int x, int z) { return ((long) z << 32) | (x & 0xFFFFFFFFL); }

    // ---------------- naive reference (multiset) ----------------
    static void bump(final Map<Long, Integer> m, final long k) {
        final Integer c = m.get(k);
        m.put(k, c == null ? Integer.valueOf(1) : Integer.valueOf(c.intValue() + 1));
    }

    /** adds = New\Old (op 0), removes = Old\New (op 1) — same convention as
     *  the fake stub, the smoke's naive(), and the bridge's apply loop. */
    static void naiveMultiset(int fx, int fz, int fd, int tx, int tz, int td,
                              final Map<Long, Integer> adds, final Map<Long, Integer> removes) {
        for (int dx = -fd; dx <= fd; dx++)
            for (int dz = -fd; dz <= fd; dz++) {
                final int x = fx + dx, z = fz + dz;
                if (Math.abs(x - tx) > td || Math.abs(z - tz) > td) bump(removes, pack(x, z));
            }
        for (int dx = -td; dx <= td; dx++)
            for (int dz = -td; dz <= td; dz++) {
                final int x = tx + dx, z = tz + dz;
                if (Math.abs(x - fx) > fd || Math.abs(z - fz) > fd) bump(adds, pack(x, z));
            }
    }

    /** Exact symmetric-difference SIZE of two axis-aligned squares, O(1):
     *  |A| + |B| - 2|A n B| with per-axis overlap clamped at 0. */
    static long geomDiff(int fx, int fz, int fd, int tx, int tz, int td) {
        final long ox = Math.max(0L, Math.min((long) fx + fd, (long) tx + td)
                                     - Math.max((long) fx - fd, (long) tx - td) + 1L);
        final long oz = Math.max(0L, Math.min((long) fz + fd, (long) tz + td)
                                     - Math.max((long) fz - fd, (long) tz - td) + 1L);
        final long a = 2L * fd + 1L, b = 2L * td + 1L;
        return a * a + b * b - 2L * ox * oz;
    }

    // ---------------- bridge scratch capacity probe ----------------
    static Object   scratchTL      = null;
    static java.lang.reflect.Field scratchOpsField = null;

    static void initReflection() throws Exception {
        final Class<?> opsCls = Class.forName("ca.spottedleaf.moonrise.common.misc.SingleUserAreaMapOps");
        final java.lang.reflect.Field tl = opsCls.getDeclaredField("SCRATCH");
        tl.setAccessible(true);
        scratchTL = tl.get(null);
        final Class<?> scrCls = Class.forName("ca.spottedleaf.moonrise.common.misc.SingleUserAreaMapOps$Scratch");
        scratchOpsField = scrCls.getDeclaredField("ops");
        scratchOpsField.setAccessible(true);
    }

    /** Post-run read: grow-only ThreadLocal array == the array actually
     *  passed to nativeUpdateOpsBatch for the call just executed. */
    static int offeredCapacity() {
        try {
            final Object s = ((ThreadLocal<?>) scratchTL).get();
            return ((byte[]) scratchOpsField.get(s)).length;
        } catch (final Exception e) {
            throw new RuntimeException("capacity probe failed", e);
        }
    }

    /** Bridge grow contract bound: (2*oldD+1)^2 + (2*newD+1)^2 (int-safe here). */
    static long maxOpsBound(int oldD, int newD) {
        final long os = 2L * oldD + 1L, ns = 2L * newD + 1L;
        return os * os + ns * ns;
    }

    // ---------------- per-call verification ----------------
    static void verifyCall(final String stream, final int d, final int call,
                           final int fx, final int fz, final int fd,
                           final int tx, final int tz, final int td) {
        if (fx == tx && fz == tz && fd == td) {
            System.out.println(stream + "\t" + d + "\t" + call + "\tSAME-STATE-CALL\t-\t-\t-\t-\tBADSTREAM");
            failures++;
            return;
        }
        // 1) expected multiset
        final Map<Long, Integer> eA = new HashMap<Long, Integer>(), eR = new HashMap<Long, Integer>();
        naiveMultiset(fx, fz, fd, tx, tz, td, eA, eR);
        int naiveN = 0;
        for (final Integer v : eA.values()) naiveN += v.intValue();
        for (final Integer v : eR.values()) naiveN += v.intValue();

        // 2) pipeline emission on a fresh recording map (no harness cross-call state)
        final OracleRec m = new OracleRec();
        SingleUserAreaMapOps.run(m, fx, fz, fd, tx, tz, td, null);

        // 3) capacity actually offered for THIS call (post-run probe)
        final int  cap      = offeredCapacity();
        final long maxBound = maxOpsBound(fd, td);
        final boolean capOk  = cap >= maxBound;   // bridge grow contract
        final boolean roomOk = cap >= naiveN;     // stub physically could not drop

        // 4) actual multiset + fault classification
        final Map<Long, Integer> aA = new HashMap<Long, Integer>(), aR = new HashMap<Long, Integer>();
        for (int i = 0; i < m.n; i++) bump(m.ops[i] == 0 ? aA : aR, m.keys[i]);

        boolean dup = false, stale = false, missing = false;
        dup    |= exceeds(aA, eA) || exceeds(aR, eR);         // more copies than naive, naive count > 0
        stale  |= hasZero(aA, eA) || hasZero(aR, eR);         // emitted rows naive does not contain
        missing |= exceeds(eA, aA) || exceeds(eR, aR);        // naive rows never emitted

        if (!roomOk && !realMode) anyStubDrop = true;
        if (dup)    anyDuplicates = true;
        if (stale)  anyStale = true;

        final boolean ok = aA.equals(eA) && aR.equals(eR);
        parityCalls++;
        if (ok) parityPass++; else failures++;

        final long[] ag = agg(stream);
        ag[0]++;
        if (ok) ag[1]++;
        if (cap < ag[2]) ag[2] = cap;
        if (naiveN > ag[3]) ag[3] = naiveN;

        String res;
        if (ok) res = "PASS";
        else {
            res = "FAIL";
            if (stale)   res += "+STALE";
            if (dup)     res += "+DUP";
            if (missing) res += "+MISSING";
        }
        System.out.println(stream + "\t" + d + "\t" + call + "\t" + naiveN + "\t" + cap
                + "\t" + mapSum(eA) + "/" + mapSum(eR) + "\t" + mapSum(aA) + "/" + mapSum(aR)
                + "\t" + (capOk ? "capOK" : "CAP<MAXOPS") + "\t" + res);
        if (!capOk) failures++;
    }

    /** actual count > expected count where expected > 0 */
    static boolean exceeds(final Map<Long, Integer> act, final Map<Long, Integer> exp) {
        for (final Map.Entry<Long, Integer> e : act.entrySet()) {
            final Integer c = exp.get(e.getKey());
            if (c != null && e.getValue().intValue() > c.intValue()) return true;
        }
        return false;
    }

    /** actual contains rows with expected count 0 */
    static boolean hasZero(final Map<Long, Integer> act, final Map<Long, Integer> exp) {
        for (final Map.Entry<Long, Integer> e : act.entrySet()) {
            final Integer c = exp.get(e.getKey());
            if (c == null || c.intValue() == 0) return true;
        }
        return false;
    }

    static int mapSum(final Map<Long, Integer> m) {
        int s = 0;
        for (final Integer v : m.values()) s += v.intValue();
        return s;
    }

    static long[] agg(final String stream) {
        long[] a = AGG.get(stream);
        if (a == null) { a = new long[]{0L, 0L, Long.MAX_VALUE, 0L}; AGG.put(stream, a); }
        return a;
    }

    // ---------------- deterministic streams (no RNG) ----------------
    static final int[][] DIR8 = { {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1} };

    /** (a) pure 1-chunk moves, bounded ring walk, fixed radius. */
    static void runPureMove(final int d, final int calls) {
        int cx = 0, cz = 0;
        for (int i = 0; i < calls; i++) {
            final int fx = cx, fz = cz;
            cx += DIR8[i % 8][0]; cz += DIR8[i % 8][1];
            verifyCall("S1-MOVE", d, i + 1, fx, fz, d, cx, cz, d);
        }
    }

    /** (b) pure same-center resizes +/-1, ping-pong (bounded). */
    static void runPureResize(final int d, final int calls) {
        int r = d;
        for (int i = 0; i < calls; i++) {
            final int fd = r;
            r += (i % 2 == 0) ? 1 : -1;
            verifyCall("S2-RESIZE", d, i + 1, 5, 7, fd, 5, 7, r);
        }
    }

    /** (c) the anomaly mix: 1-chunk move EVERY call + +/-1 resize every 4th
     *  call (alternate direction) — bounded center and radius. */
    static void runMixed(final int d, final int calls) {
        int cx = 0, cz = 0, r = d;
        for (int i = 0; i < calls; i++) {
            final int fx = cx, fz = cz, fd = r;
            cx += DIR8[i % 8][0]; cz += DIR8[i % 8][1];
            if (i % 8 == 0) r += 1; else if (i % 8 == 4) r -= 1;
            verifyCall("S3-MIX", d, i + 1, fx, fz, fd, cx, cz, r);
        }
    }

    // ---------------- bench-stream replay (stream arithmetic, NOT timing) ----------------
    static void replayBenchStream() {
        final long SEED = 0x9E3779B97F4A7C15L;
        long rng = SEED;
        int cx = 3, cz = -2, cd = 63;
        long sumOps = 0L, sumR = 0L;
        long minR = Long.MAX_VALUE, maxR = Long.MIN_VALUE;
        final int FAKE_WINDOW = 52108;                   // FAKE d=63 native-call counter (raw TSV)
        final int[] REAL_WINDOWS = {9996, 10003, 10010}; // REAL bounds from opsAccum/opsPerCall rounding
        int wi = 0;

        for (int call = 1; call <= FAKE_WINDOW; call++) {
            final long r0 = rng;
            rng ^= rng << 13; rng ^= rng >>> 7; rng ^= rng << 17;
            int dx = (int) Long.remainderUnsigned(r0, 5L) - 2;
            int dz = (int) Long.remainderUnsigned(r0 >>> 8, 5L) - 2;
            int nd = cd;
            if (Long.remainderUnsigned(r0 >>> 16, 8L) == 0L) {
                nd = cd + (((r0 >>> 24) & 1L) == 0L ? 1 : -1);
                if (nd < 1) nd = 1;
            }
            if (dx == 0 && dz == 0 && nd == cd) dz = 1;
            final int fx = cx, fz = cz, fd = cd;
            cx += dx; cz += dz; cd = nd;

            final long g = geomDiff(fx, fz, fd, cx, cz, cd);
            if (call <= 64) { // cross-check the O(1) geometry against full enumeration
                final Map<Long, Integer> tA = new HashMap<Long, Integer>(), tR = new HashMap<Long, Integer>();
                naiveMultiset(fx, fz, fd, cx, cz, cd, tA, tR);
                final int en = mapSum(tA) + mapSum(tR);
                if (en != (int) g) {
                    System.out.println("REPLAY\tgeom-mismatch\tcall=" + call + "\tgeom=" + g + "\tenum=" + en);
                    failures++;
                }
            }
            sumOps += g; sumR += cd;
            if (cd < minR) minR = cd;
            if (cd > maxR) maxR = cd;

            if (wi < REAL_WINDOWS.length && call == REAL_WINDOWS[wi]) {
                System.out.println("REPLAY\tREALwindow\titers=" + call + "\tmeanNaiveOps="
                        + fmt2((double) sumOps / call) + "\tmeanRadius=" + fmt2((double) sumR / call)
                        + "\tminR=" + minR + "\tmaxR=" + maxR + "\tbenchReported=645");
                wi++;
            }
        }
        System.out.println("REPLAY\tFAKEwindow\titers=" + FAKE_WINDOW + "\tmeanNaiveOps="
                + fmt2((double) sumOps / FAKE_WINDOW) + "\tmeanRadius=" + fmt2((double) sumR / FAKE_WINDOW)
                + "\tminR=" + minR + "\tmaxR=" + maxR + "\tbenchReported=374");
    }

    static String fmt2(final double v) {
        return String.format(java.util.Locale.ROOT, "%.2f", v);
    }

    // ---------------- main ----------------
    public static void main(final String[] args) throws Exception {
        realMode = System.getProperty("crussty.native") != null;
        if (realMode) System.load(System.getProperty("crussty.native"));
        System.out.println("mode\t" + (realMode ? "REAL native (.so)" : "FAKE native (counting stub)"));
        System.out.println("stream\td\tcall\tnaiveOps\tcapOps\texpA/expR\tactA/actR\tcapCheck\tresult");

        initReflection();

        runPureMove(63, 64);
        runPureResize(63, 64);
        runMixed(63, 128);
        runMixed(255, 8);
        runMixed(511, 4);

        for (final Map.Entry<String, long[]> e : AGG.entrySet()) {
            final long[] a = e.getValue();
            System.out.println("SUMMARY\t" + e.getKey() + "\tcalls=" + a[0] + "\tpass=" + a[1]
                    + "\tfail=" + (a[0] - a[1]) + "\tminCapOps=" + a[2] + "\tmaxNaiveOps=" + a[3]);
        }

        replayBenchStream();

        final String cls;
        if (anyStubDrop)            cls = "STUB-DROP";
        else if (anyStale)          cls = "STALE";
        else if (anyDuplicates)     cls = "DUPLICATES";
        else if (failures == 0)     cls = "PARITY";
        else                        cls = "MIXED-UNKNOWN";
        System.out.println("parityCalls=" + parityCalls + "\tparityPass=" + parityPass
                + "\tfailures=" + failures + "\tmode=" + (realMode ? "REAL" : "FAKE"));
        System.out.println("TASK-30 VERDICT: " + cls);
        // correctness gate: exit 0 iff parity held on every call (both modes are
        // required to be clean for the PARITY classification; REAL is the kernel gate)
        System.exit(failures == 0 ? 0 : 1);
    }
}
