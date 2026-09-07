import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.ImprovedNoiseNativeOps;
import net.minecraft.world.level.levelgen.synth.PaperNativeImprovedNoise;

/**
 * Smoke test for the B1 Cleaner lifecycle of the REAL
 * ImprovedNoiseNativeOps bridge, run against pure-Java doubles of the
 * natives (noise/src-new/test/fake/...), so it needs no closed-source .so.
 *
 * It verifies the invariants the production kernel JVM would rely on:
 *   1. one handle per live instance, cached across repeated noise() calls;
 *   2. explicit releaseHandle(): frees once, is no-op-safe on repeat,
 *      and the next noise() transparently rebuilds;
 *   3. GC path: dropping instances + a single map poke (noise() on a live
 *      dummy) drains all handles via the Cleaner thread;
 *   4. no double-free ever: every native free call hits a DISTINCT handle
 *      (FREED_CALLS == FREED_UNIQUE.size() at every checkpoint).
 *
 * Compile & run (from repo root, any JDK >= 11):
 *   javac -d /tmp/noise-smoke \
 *     noise/src-new/test/fake/net/minecraft/world/level/levelgen/synth/ImprovedNoise.java \
 *     noise/src-new/test/fake/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.java \
 *     noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java \
 *     noise/src-new/test/SmokeLifecycle.java
 *   java -cp /tmp/noise-smoke SmokeLifecycle
 * Exit 0 = PASS (prints PASS lines per scenario).
 */
public final class SmokeLifecycle {

    static byte[] P;

    public static void main(String[] args) throws Exception {
        P = new byte[256];
        for (int i = 0; i < 256; i++) P[i] = (byte) i;

        scenarioCaching();
        scenarioExplicitRelease();
        scenarioGcDrain();
        scenarioDoubleFreeGuard();

        long calls = PaperNativeImprovedNoise.FREED_CALLS.get();
        long uniq = PaperNativeImprovedNoise.FREED_UNIQUE.size();
        check("global: free calls == unique handles freed", calls == uniq,
                calls + " calls vs " + uniq + " unique");
        System.out.println("SMOKE LIFECYCLE: ALL PASS");
    }

    // 1) handle built once per instance, cached across calls
    static void scenarioCaching() {
        long built0 = PaperNativeImprovedNoise.BUILT.get();
        ImprovedNoise a = new ImprovedNoise();
        double sum = 0;
        for (int i = 0; i < 100; i++) sum += noise(a);
        long built = PaperNativeImprovedNoise.BUILT.get() - built0;
        check("caching: 100 noise() calls on one instance build exactly 1 handle", built == 1,
                "built=" + built);
        check("caching: samples flow through the native path", sum != 0, "sum=" + sum);
        ImprovedNoiseNativeOps.releaseHandle(a); // tidy up for accounting below
    }

    // 2) explicit release: once, no-op on repeat, rebuild on next use
    static void scenarioExplicitRelease() throws Exception {
        long built0 = PaperNativeImprovedNoise.BUILT.get();
        long freed0 = PaperNativeImprovedNoise.FREED_CALLS.get();
        ImprovedNoise a = new ImprovedNoise();
        noise(a);
        check("explicit: first use builds", PaperNativeImprovedNoise.BUILT.get() - built0 == 1,
                "built=" + (PaperNativeImprovedNoise.BUILT.get() - built0));
        ImprovedNoiseNativeOps.releaseHandle(a);
        quiesce(50);
        check("explicit: releaseHandle frees exactly once",
                PaperNativeImprovedNoise.FREED_CALLS.get() - freed0 == 1,
                "freed=" + (PaperNativeImprovedNoise.FREED_CALLS.get() - freed0));
        ImprovedNoiseNativeOps.releaseHandle(a); // repeat — must be a no-op
        quiesce(50);
        check("explicit: repeated releaseHandle is a no-op",
                PaperNativeImprovedNoise.FREED_CALLS.get() - freed0 == 1,
                "freed=" + (PaperNativeImprovedNoise.FREED_CALLS.get() - freed0));
        noise(a); // transparent rebuild
        check("explicit: next noise() rebuilds after release",
                PaperNativeImprovedNoise.BUILT.get() - built0 == 2,
                "built=" + (PaperNativeImprovedNoise.BUILT.get() - built0));
        ImprovedNoiseNativeOps.releaseHandle(a);
        quiesce(50);
    }

    // 3) GC path: drop instances, poke the map once, Cleaner drains all
    static void scenarioGcDrain() throws Exception {
        int n = 500;
        ImprovedNoise[] hold = new ImprovedNoise[n];
        long lo = PaperNativeImprovedNoise.lastSeq();
        for (int i = 0; i < n; i++) {
            hold[i] = new ImprovedNoise();
            noise(hold[i]);
        }
        long hi = PaperNativeImprovedNoise.lastSeq(); // fake handles are sequential: this round owns (lo, hi]
        check("gc: n handles built", hi - lo == n, "built=" + (hi - lo));
        hold = null; // drop all refs wholesale

        ImprovedNoise poke = new ImprovedNoise(); // stays live: map access expunges cleared entries
        long deadline = System.currentTimeMillis() + 30_000;
        while (!allFreed(lo, hi) && System.currentTimeMillis() < deadline) {
            System.gc();
            noise(poke); // WeakHashMap.get expunges cleared entries -> Cleaner can act
            Thread.sleep(10);
        }
        ImprovedNoiseNativeOps.releaseHandle(poke);
        quiesce(50);
        check("gc: Cleaner freed all " + n + " dropped-instance handles",
                allFreed(lo, hi), "missing=" + missingCount(lo, hi));
    }

    static boolean allFreed(long lo, long hi) {
        return missingCount(lo, hi) == 0;
    }

    /** How many handles of the (lo, hi] batch have NOT been freed yet. */
    static long missingCount(long lo, long hi) {
        long missing = 0;
        for (long h = lo + 1; h <= hi; h++) {
            if (!PaperNativeImprovedNoise.FREED_UNIQUE.contains(h)) missing++;
        }
        return missing;
    }

    // 4) explicit release racing with the GC path must never double-free.
    //    Deterministic construction is impossible; instead we release a live
    //    instance, then hammer it: repeated release + gc + rebuild + drop.
    static void scenarioDoubleFreeGuard() throws Exception {
        long calls0 = PaperNativeImprovedNoise.FREED_CALLS.get();
        long uniq0 = PaperNativeImprovedNoise.FREED_UNIQUE.size();
        ImprovedNoise a = new ImprovedNoise();
        for (int round = 0; round < 50; round++) {
            noise(a);
            ImprovedNoiseNativeOps.releaseHandle(a); // explicit path
            ImprovedNoiseNativeOps.releaseHandle(a); // no-op repeat
            if (round % 10 == 0) System.gc();        // GC path may also see it
        }
        ImprovedNoiseNativeOps.releaseHandle(a);
        quiesce(100);
        long calls = PaperNativeImprovedNoise.FREED_CALLS.get() - calls0;
        long uniq = PaperNativeImprovedNoise.FREED_UNIQUE.size() - uniq0;
        check("double-free: free calls == unique handles over 50 rebuild cycles",
                calls == uniq, calls + " calls vs " + uniq + " unique");
    }

    // ---------- helpers ----------

    static double noise(ImprovedNoise self) {
        return ImprovedNoiseNativeOps.noise(self, P, 0.5, 0.5, 0.5, 1.5, 2.5, 3.5, 0.0, 0.0);
    }

    /** Give reference-processing threads a moment; tolerate GC-less JDKs. */
    static void quiesce(int ms) throws Exception {
        long end = System.currentTimeMillis() + ms;
        while (System.currentTimeMillis() < end) {
            System.gc();
            Thread.sleep(5);
        }
    }

    static void check(String what, boolean ok, String detail) {
        System.out.println((ok ? "PASS  " : "FAIL  ") + what + (ok ? "" : "  [" + detail + "]"));
        if (!ok) System.exit(1);
    }
}
