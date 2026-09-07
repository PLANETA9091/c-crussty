package areamapsmoke;

import ca.spottedleaf.moonrise.common.misc.SingleUserAreaMap;
import ca.spottedleaf.moonrise.common.misc.SmokeProbe;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.CyclicBarrier;

/**
 * Headless unit-smoke for the PATCHED SingleUserAreaMap.update() body.
 *
 * The JVM here runs the EXACT patch bytes the shipped Rust patcher produces
 * (tests/area_map_smoke/patch_tool.py is byte-parity-checked against
 * src/classfile.rs::patch_update; Boot.java defines the class from those
 * bytes), the REAL shipped bridge classes (area-map/build, rebuilt by
 * scripts/build_area_map.sh), and a bridge substitute:
 *   - variant "stub":    a counting Java fake for PaperNativeAreaMap
 *                        (same FQN/signature, increments a counter instead of
 *                        transitioning to JNI),
 *   - variant "real-so": the REAL native/libpaper_native_jni.so, bound by
 *                        exact JNI symbol name (no counting available).
 *
 * Mandatory checks (task 4-a):
 *   CHK-1  Scratch init: the first patched update() on a fresh thread does
 *          NOT NPE (Scratch ops/keys field initializers), incl. grow-only
 *          doubling past INITIAL_CAP=578.
 *   CHK-2  Same-state skip: identical squares -> native invoked 0 times,
 *          no callbacks, state stable; a changed control call fires it once.
 *   CHK-3  Changed state: native invoked exactly once and the applied
 *          callbacks equal naive_set_difference (adds = new\old, removes =
 *          old\new), plus NOT_SET / IllegalArgumentException guard behavior.
 */
public final class SmokeMain {

    private static final Object TOKEN = new Object();
    private static int failures = 0;

    public static void main(String[] args) {
        boolean counting = SmokeProbe.counting();
        String mode = counting ? "stub" : "real-so";
        System.out.println("=== c-crussty area-map headless unit-smoke (mode=" + mode + ") ===");
        System.out.println("jvm=" + System.getProperty("java.vm.version")
                + " | SingleUserAreaMap loader=" + SingleUserAreaMap.class.getClassLoader());

        chk0();
        chk1(counting);
        chk2(counting);
        chk3(counting);

        if (failures == 0) {
            System.out.println("SMOKE RESULT: PASS (mode=" + mode + ")");
            System.exit(0);
        } else {
            System.out.println("SMOKE RESULT: FAIL (" + failures + " failure(s), mode=" + mode + ")");
            System.exit(1);
        }
    }

    // ------------------------------------------------------------------
    // CHK-0: provenance sanity of the loaded environment
    // ------------------------------------------------------------------
    private static void chk0() {
        boolean ok = true;
        ok &= expect(SingleUserAreaMap.class.getClassLoader() == SmokeMain.class.getClassLoader(),
                "SingleUserAreaMap must come from the smoke loader (defines the patched bytes)");
        ok &= expect(
                SingleUserAreaMap.class.getClassLoader()
                        .getResource("ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap.class") == null,
                "patched class must be defined from bytes, not from a directory resource");
        ok &= expect(SingleUserAreaMap.NOT_SET == Integer.MIN_VALUE,
                "NOT_SET must be i32::MIN (fixture/kernel contract)");
        report(ok, "CHK-0", "provenance: patched bytes define the map class in the smoke loader; NOT_SET=i32::MIN");
    }

    // ------------------------------------------------------------------
    // CHK-1: Scratch init -- first patched update() per fresh thread, no NPE
    // ------------------------------------------------------------------
    private static void chk1(final boolean counting) {
        final int threads = 8;
        final CyclicBarrier barrier = new CyclicBarrier(threads);
        final List<List<Throwable>> errs = new ArrayList<>(threads);
        for (int i = 0; i < threads; i++) {
            errs.add(null);
        }
        final long c0 = SmokeProbe.invocations();

        Thread[] ts = new Thread[threads];
        for (int i = 0; i < threads; i++) {
            final int id = i;
            ts[i] = new Thread(() -> {
                List<Throwable> err = new ArrayList<>();
                errs.set(id, err);
                try {
                    barrier.await();
                } catch (Exception e) {
                    err.add(e);
                    return;
                }
                try {
                    RecordingMap m = new RecordingMap(TOKEN);
                    // NOT_SET first call: returns false, no native, no Scratch touch
                    if (m.update(3, 4, 5)) {
                        err.add(new AssertionError("fresh map update must return false (NOT_SET)"));
                    }
                    // FIRST Scratch-touching call on this thread -- the
                    // historical NPE point (Scratch.ops/keys null fields).
                    m.initState(10, 10, 2);
                    if (!m.update(12, 9, 3)) {
                        err.add(new AssertionError("first changed update must return true"));
                    }
                    checkCallbacks(m, 0, 10, 10, 2, 12, 9, 3, err, "chk1-t" + id + " initial");
                    // Grow-only doubling: cap(8, 9) = 17^2 + 19^2 = 650 > 578
                    m.initState(12, 9, 8);
                    int before = m.callbacks.size();
                    if (!m.update(12, 9, 9)) {
                        err.add(new AssertionError("grow update must return true"));
                    }
                    checkCallbacks(m, before, 12, 9, 8, 12, 9, 9, err, "chk1-t" + id + " grow");
                    if (m.callbacks.size() - before != 72) {
                        err.add(new AssertionError("grow diff must be 72 ring cells, got " + (m.callbacks.size() - before)));
                    }
                    // Same-state on the grown state: nothing new
                    before = m.callbacks.size();
                    if (!m.update(12, 9, 9)) {
                        err.add(new AssertionError("same-state update must return true"));
                    }
                    if (m.callbacks.size() != before) {
                        err.add(new AssertionError("same-state must not produce callbacks"));
                    }
                } catch (Throwable t) {
                    err.add(t); // an NPE here is exactly the Scratch regression
                }
            }, "amsmoke-" + id);
            ts[i].start();
        }
        boolean joined = true;
        for (Thread t : ts) {
            try {
                t.join(10_000);
                joined &= !t.isAlive();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                joined = false;
            }
        }
        int throwableCount = 0;
        StringBuilder first = new StringBuilder();
        for (List<Throwable> err : errs) {
            if (err == null) {
                throwableCount++;
                continue;
            }
            for (Throwable t : err) {
                throwableCount++;
                if (first.length() == 0) {
                    first.append(t);
                }
            }
        }
        boolean ok = joined && throwableCount == 0;
        if (counting) {
            long delta = SmokeProbe.invocations() - c0;
            ok &= expect(delta == 2L * threads,
                    "native must fire exactly 2x per thread, delta=" + delta + " expected " + 2 * threads);
            ok &= expect(SmokeProbe.lastOpsCapacity() >= 650,
                    "grow-only scratch must reach >= 650, got " + SmokeProbe.lastOpsCapacity());
            ok &= expect(SmokeProbe.lastKeysCapacity() == SmokeProbe.lastOpsCapacity(),
                    "keys capacity must track ops capacity");
        }
        report(ok, "CHK-1", threads + "/" + threads + " fresh threads: first patched update() without NPE "
                + "(Scratch init) + grow 578->" + (counting ? SmokeProbe.lastOpsCapacity() : "?")
                + " (cap 650) + same-state after grow; throwables=" + throwableCount
                + (first.length() > 0 ? " first=" + first : ""));
    }

    // ------------------------------------------------------------------
    // CHK-2: same-state fast path -- native skip + resulting state correct
    // ------------------------------------------------------------------
    private static void chk2(boolean counting) {
        RecordingMap m = new RecordingMap(TOKEN);
        m.initState(7, -3, 4);
        long c0 = SmokeProbe.invocations();
        int cb0 = m.callbacks.size();

        boolean allTrue = true;
        for (int i = 0; i < 1000; i++) {
            allTrue &= m.update(7, -3, 4);
        }
        long delta = SmokeProbe.invocations() - c0;

        boolean ok = true;
        ok &= expect(allTrue, "same-state update must return true (contract)");
        ok &= expect(m.callbacks.size() == cb0, "same-state must not produce callbacks (empty difference)");
        ok &= expect(m.getLastChunkX() == 7 && m.getLastChunkZ() == -3 && m.getLastDistance() == 4,
                "state must be stable across same-state updates");
        if (counting) {
            ok &= expect(delta == 0, "same-state skip: native delta must be 0, got " + delta);
        }

        // negative control: a genuinely changed call fires the native exactly once
        long c1 = SmokeProbe.invocations();
        int cb1 = m.callbacks.size();
        boolean control = m.update(7, -2, 4);
        ok &= expect(control, "control changed update must return true");
        if (counting) {
            long d1 = SmokeProbe.invocations() - c1;
            ok &= expect(d1 == 1, "control changed call: native delta must be 1, got " + d1);
        }
        ok &= checkCallbacks(m, cb1, 7, -3, 4, 7, -2, 4, "CHK-2 control");
        report(ok, "CHK-2", "1001 same-state updates -> native delta " + (counting ? delta : "n/a")
                + " (expected 0), no callbacks, state stable; control changed call fired once ("
                + (counting ? "delta 1" : "counting n/a") + ") with correct 9+9 diff");
    }

    // ------------------------------------------------------------------
    // CHK-3: guards + changed-state correctness (spots + seeded stream)
    // ------------------------------------------------------------------
    private static void chk3(boolean counting) {
        boolean ok = true;

        // 3a: NOT_SET guard
        RecordingMap fresh = new RecordingMap(TOKEN);
        long c0 = SmokeProbe.invocations();
        boolean r0 = fresh.update(2, 3, 4);
        ok &= expect(!r0, "fresh map update must return false (NOT_SET)");
        if (counting) {
            ok &= expect(SmokeProbe.invocations() - c0 == 0, "NOT_SET path must not call native");
        }
        ok &= expect(fresh.getLastChunkX() == Integer.MIN_VALUE
                && fresh.getLastChunkZ() == Integer.MIN_VALUE
                && fresh.getLastDistance() == Integer.MIN_VALUE,
                "fields must stay untouched on the NOT_SET path");
        ok &= expect(fresh.callbacks.isEmpty(), "no callbacks on the NOT_SET path");

        // 3b: negative newDistance -> IllegalArgumentException, fields untouched
        RecordingMap m = new RecordingMap(TOKEN);
        m.initState(5, 5, 5);
        long c1 = SmokeProbe.invocations();
        try {
            m.update(6, 6, -1);
            ok &= expect(false, "negative distance must throw IllegalArgumentException");
        } catch (IllegalArgumentException e) {
            ok &= expect(String.valueOf(e).contains("-1"), "IAE message must carry newDistance, got: " + e);
        } catch (Throwable t) {
            ok &= expect(false, "expected IllegalArgumentException, got " + t);
        }
        if (counting) {
            ok &= expect(SmokeProbe.invocations() - c1 == 0, "IAE guard must not call native");
        }
        ok &= expect(m.getLastChunkX() == 5 && m.getLastChunkZ() == 5 && m.getLastDistance() == 5,
                "fields must stay untouched after the IAE throw (guard precedes writes)");
        ok &= expect(m.callbacks.isEmpty(), "no callbacks after the IAE throw");

        // 3c: hand-verified spot cases (mirror of tests/extended/area_map_stress.py)
        int[][] spots = {
                {5, 5, 3, 5, 5, 3},   // identical squares -> same-state path, empty diff
                {0, 0, 0, 0, 0, 0},   // d=0 single cells, same-state
                {0, 0, 0, 1, 0, 0},   // one-step move: 1 add, 1 remove
                {0, 0, 1, 1, 1, 1},   // diagonal d=1: 5 adds, 5 removes
                {0, 0, 2, 0, 0, 1},   // shrink in place: 16 removes
                {0, 0, 1, 0, 0, 2},   // grow in place: 24 adds
                {-3, -3, 6, 3, 3, 6}, // big jump: 120 adds + 120 removes
        };
        for (int i = 0; i < spots.length; i++) {
            int[] s = spots[i];
            ok &= runRect(s[0], s[1], s[2], s[3], s[4], s[5], "CHK-3c spot#" + i, counting);
        }

        // 3d: seeded stream -- xorshift64 with the bridge_selftest seed and
        // draw order (from_x, from_z, to_x, to_z, old_d, new_d)
        Lcg rng = new Lcg(0x9E3779B97F4A7C15L);
        int rects = 512;
        int nativeSum = 0;
        for (int i = 0; i < rects; i++) {
            int fx = (int) Long.remainderUnsigned(rng.next(), 21) - 10;
            int fz = (int) Long.remainderUnsigned(rng.next(), 21) - 10;
            int tx = (int) Long.remainderUnsigned(rng.next(), 21) - 10;
            int tz = (int) Long.remainderUnsigned(rng.next(), 21) - 10;
            int od = (int) Long.remainderUnsigned(rng.next(), 7);
            int nd = (int) Long.remainderUnsigned(rng.next(), 7);
            ok &= runRect(fx, fz, od, tx, tz, nd, "CHK-3d rect#" + i, counting);
            if (counting) {
                nativeSum += (fx == tx && fz == tz && od == nd) ? 0 : 1;
            }
        }
        report(ok, "CHK-3", "guards (NOT_SET false/no-native, IAE + fields untouched) + 7 spot cases + "
                + rects + " seeded rects: callbacks == naive_set_difference, native called exactly once per changed update"
                + (counting ? " (total native deltas " + nativeSum + "/" + rects + " changed)" : " (counting n/a)"));
    }

    /** One full changed-state transition on a fresh map; returns true iff all
     *  expectations held. Identical squares legitimately take the same-state
     *  path (native delta 0). */
    private static boolean runRect(int fx, int fz, int od, int tx, int tz, int nd, String ctx, boolean counting) {
        boolean ok = true;
        boolean sameState = fx == tx && fz == tz && od == nd;
        RecordingMap m = new RecordingMap(TOKEN);
        m.initState(fx, fz, od);
        long c0 = SmokeProbe.invocations();
        boolean r;
        try {
            r = m.update(tx, tz, nd);
        } catch (Throwable t) {
            return expect(false, ctx + ": update threw " + t);
        }
        ok &= expect(r, ctx + ": update must return true");
        if (counting) {
            long d = SmokeProbe.invocations() - c0;
            ok &= expect(d == (sameState ? 0 : 1),
                    ctx + ": native delta must be " + (sameState ? 0 : 1) + ", got " + d);
        }
        ok &= expect(m.getLastChunkX() == tx && m.getLastChunkZ() == tz && m.getLastDistance() == nd,
                ctx + ": fields must equal the new state (written before run())");
        ok &= checkCallbacks(m, 0, fx, fz, od, tx, tz, nd, ctx);
        if (counting) {
            int cap = maxOps(od, nd);
            ok &= expect(SmokeProbe.lastOpsCapacity() >= cap,
                    ctx + ": scratch ops capacity " + SmokeProbe.lastOpsCapacity() + " must be >= maxOps=" + cap);
            ok &= expect(SmokeProbe.lastKeysCapacity() == SmokeProbe.lastOpsCapacity(),
                    ctx + ": keys capacity must equal ops capacity");
        }
        return ok;
    }

    /** Compares callbacks[from] of the map against the naive set difference.
     *  Also flags duplicate cells per op class. */
    private static boolean checkCallbacks(RecordingMap m, int from,
                                          int fx, int fz, int od, int tx, int tz, int nd,
                                          List<Throwable> err, String ctx) {
        Set<Long> adds = new HashSet<>();
        Set<Long> removes = new HashSet<>();
        boolean dup = false;
        for (int i = from; i < m.callbacks.size(); i++) {
            RecordingMap.Callback c = m.callbacks.get(i);
            long k = NaiveDiff.key(c.x(), c.z());
            dup |= !(c.add() ? adds.add(k) : removes.add(k));
        }
        Set<Long> eAdds = new HashSet<>();
        Set<Long> eRemoves = new HashSet<>();
        NaiveDiff.diff(fx, fz, od, tx, tz, nd, eAdds, eRemoves);
        boolean ok = true;
        if (dup) {
            err.add(new AssertionError(ctx + ": duplicate cells in callbacks"));
            ok = false;
        }
        if (!adds.equals(eAdds)) {
            err.add(new AssertionError(ctx + ": adds mismatch: got " + adds.size() + " cells, expected " + eAdds.size()));
            ok = false;
        }
        if (!removes.equals(eRemoves)) {
            err.add(new AssertionError(ctx + ": removes mismatch: got " + removes.size() + " cells, expected " + eRemoves.size()));
            ok = false;
        }
        return ok;
    }

    private static boolean checkCallbacks(RecordingMap m, int from,
                                          int fx, int fz, int od, int tx, int tz, int nd, String ctx) {
        return checkCallbacks(m, from, fx, fz, od, tx, tz, nd, new ArrayList<>(), ctx);
    }

    /** Mirror of SingleUserAreaMapOps.maxOps. */
    private static int maxOps(int oldD, int newD) {
        long oldSide = 2L * oldD + 1L;
        long newSide = 2L * newD + 1L;
        long cap = oldSide * oldSide + newSide * newSide;
        return cap > Integer.MAX_VALUE ? Integer.MAX_VALUE : (int) cap;
    }

    /** xorshift64 -- identical to the bridge_selftest stream (u64 wrap, logical >>). */
    private static final class Lcg {
        private long s;

        Lcg(long seed) {
            this.s = seed;
        }

        long next() {
            s ^= s << 13;
            s ^= s >>> 7;
            s ^= s << 17;
            return s;
        }
    }

    // ------------------------------------------------------------------
    // reporting
    // ------------------------------------------------------------------
    private static boolean expect(boolean cond, String what) {
        if (!cond) {
            System.out.println("  FAIL: " + what);
            failures++;
        }
        return cond;
    }

    private static void report(boolean ok, String tag, String what) {
        System.out.println((ok ? "PASS [" : "FAIL [") + tag + "] " + what);
    }
}
