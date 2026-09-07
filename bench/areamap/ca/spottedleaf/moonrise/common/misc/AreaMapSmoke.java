package ca.spottedleaf.moonrise.common.misc;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Headless unit-smoke for the area-map bridge JAVA half (TASK-11).
 *
 * The live server cannot exercise the same-state fast path (the hook only
 * arms when a real player tracking map exists), so this smoke drives
 * SingleUserAreaMapOps.run() DIRECTLY, closing the verification gap:
 *
 *   S1 parity-fake : apply-loop output == naive set difference, across a
 *                    grid of coords/distances (negative, disjoint, nested,
 *                    overlapping, big-d growth beyond INITIAL_CAP);
 *   S2 fast-path   : same-state (x,z,d) run() -> ZERO native calls, ZERO
 *                    callbacks (and a moved call DOES hit the native —
 *                    guards against a vacuously-passing counter);
 *   S3 min-value   : fromX == Integer.MIN_VALUE -> no native call, no ops;
 *   S4 threads     : 4 threads x interleaved cases -> parity per thread
 *                    (ThreadLocal scratch isolation, no cross-talk);
 *   S5 real-native : with -Dcrussty.native set, the REAL .so must agree
 *                    with the naive reference on the whole grid (fake class
 *                    replaced by the real binding in this JVM mode).
 *
 * Exit code 0 = ALL PASS. TSV-ish PASS/FAIL lines on stdout.
 */
public final class AreaMapSmoke {

    static int failures = 0;

    static void check(String name, boolean ok, String detail) {
        System.out.println((ok ? "PASS " : "FAIL ") + name + (ok ? "" : "  :: " + detail));
        if (!ok) failures++;
    }

    /** Naive reference: (op, x, z) rows for the difference of two squares. */
    static List<String> naive(int fromX, int fromZ, int oldD, int toX, int toZ, int newD) {
        final List<String> out = new ArrayList<String>();
        if (fromX == Integer.MIN_VALUE) return out;
        for (int dx = -oldD; dx <= oldD; dx++)
            for (int dz = -oldD; dz <= oldD; dz++) {
                final int x = fromX + dx, z = fromZ + dz;
                if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD)
                    out.add("R:" + x + ":" + z);
            }
        for (int dx = -newD; dx <= newD; dx++)
            for (int dz = -newD; dz <= newD; dz++) {
                final int x = toX + dx, z = toZ + dz;
                if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD)
                    out.add("A:" + x + ":" + z);
            }
        return out;
    }

    static boolean multisetEquals(List<String> a, List<String> b) {
        if (a.size() != b.size()) return false;
        final List<String> x = new ArrayList<String>(a), y = new ArrayList<String>(b);
        Collections.sort(x); Collections.sort(y);
        return x.equals(y);
    }

    /** One run() drive on a fresh recording map + multiset parity. */
    static void parityCase(String tag, int fromX, int fromZ, int oldD, int toX, int toZ, int newD) {
        final RecAreaMap map = new RecAreaMap();
        SingleUserAreaMapOps.run(map, fromX, fromZ, oldD, toX, toZ, newD, "p");
        final List<String> expected = naive(fromX, fromZ, oldD, toX, toZ, newD);
        check(tag, multisetEquals(map.log, expected),
                "expected " + expected.size() + " ops, got " + map.log.size()
                + (map.log.size() < 12 ? " diff=" + diff(expected, map.log) : ""));
    }

    static String diff(List<String> exp, List<String> got) {
        final List<String> e = new ArrayList<String>(exp), g = new ArrayList<String>(got);
        Collections.sort(e); Collections.sort(g);
        e.removeAll(g); g.removeAll(e);
        return "missing=" + e.subList(0, Math.min(4, e.size())) + " extra=" + g.subList(0, Math.min(4, g.size()));
    }

    public static void main(String[] args) {
        final boolean realMode = System.getProperty("crussty.native") != null;
        if (realMode) {
            System.load(System.getProperty("crussty.native"));
            System.out.println("mode: REAL natives");
        } else {
            System.out.println("mode: FAKE native (fast-path observable)");
        }

        // ---- S1: parity grid ----
        final int[] ds = {0, 1, 2, 8, 16, 32};   // 32 -> cap 8450 > INITIAL_CAP 578 (grow path)
        for (final int d0 : ds) for (final int d1 : ds) {
            parityCase("S1.move",      -3,  7, d0,  5, -2, d1);
            parityCase("S1.negative", -64,-64, d0, 64, 64, d1);
            parityCase("S1.nested",    10, 10, d0, 10, 10, Math.min(31, d1));  // same center, size change
        }
        parityCase("S1.disjoint",  -50, -50, 4, 50, 50, 4);
        parityCase("S1.touch",       0,   0, 3, 8,  0, 3);   // squares share one edge cell region
        if (failures == 0) System.out.println("S1 parity: ALL PASS");

        if (!realMode) {
            // ---- S2: same-state fast path ----
            PaperNativeAreaMap.resetCalls();
            final RecAreaMap m1 = new RecAreaMap();
            SingleUserAreaMapOps.run(m1, 5, 5, 4, 5, 5, 4, "p");       // same-state
            check("S2.fastpath.noNative", PaperNativeAreaMap.calls() == 0L,
                    "native calls after same-state: " + PaperNativeAreaMap.calls());
            check("S2.fastpath.noCallbacks", m1.log.isEmpty(), "callbacks: " + m1.log.size());
            SingleUserAreaMapOps.run(m1, 5, 5, 4, 6, 5, 4, "p");       // moved
            check("S2.moved.hitsNative", PaperNativeAreaMap.calls() == 1L,
                    "native calls after move: " + PaperNativeAreaMap.calls());
            check("S2.moved.parity", multisetEquals(m1.log, naive(5, 5, 4, 6, 5, 4)),
                    "ops=" + m1.log.size());

            // ---- S3: MIN_VALUE guard ----
            PaperNativeAreaMap.resetCalls();
            final RecAreaMap m2 = new RecAreaMap();
            SingleUserAreaMapOps.run(m2, Integer.MIN_VALUE, 0, 4, 0, 0, 4, "p");
            check("S3.minvalue.noNative", PaperNativeAreaMap.calls() == 0L,
                    "native calls: " + PaperNativeAreaMap.calls());
            check("S3.minvalue.noCallbacks", m2.log.isEmpty(), "callbacks: " + m2.log.size());
        }

        // ---- S4: thread isolation (both modes) ----
        final List<Throwable> threadErrs = Collections.synchronizedList(new ArrayList<Throwable>());
        Thread[] ts = new Thread[4];
        for (int t = 0; t < ts.length; t++) {
            final int seed = t;
            ts[t] = new Thread(() -> {
                try {
                    for (int i = 0; i < 40; i++) {
                        final int d0 = (i + seed) % 5, d1 = (i * 3 + seed) % 5;
                        final RecAreaMap m = new RecAreaMap();
                        SingleUserAreaMapOps.run(m, seed * 100 - 150, i - 20, d0, seed * 100 - 100, i + 10, d1, "t");
                        if (!multisetEquals(m.log, naive(seed * 100 - 150, i - 20, d0, seed * 100 - 100, i + 10, d1))) {
                            throw new IllegalStateException("parity t" + seed + " i" + i);
                        }
                    }
                } catch (Throwable e) { threadErrs.add(e); }
            }, "smoke-" + t);
            ts[t].start();
        }
        for (Thread t : ts) {
            while (t.isAlive()) {
                try { t.join(); } catch (InterruptedException e) { Thread.currentThread().interrupt(); }
            }
        }
        check("S4.threads.isolated", threadErrs.isEmpty(), threadErrs.toString());

        System.out.println(failures == 0 ? "AREAMAP SMOKE: ALL PASS" : "AREAMAP SMOKE: " + failures + " FAILURES");
        System.exit(failures == 0 ? 0 : 1);
    }

}
