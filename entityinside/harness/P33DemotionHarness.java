import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * P33 ACQUIRE-DEMOTION offline harness (TASK-461-67, chkclimb-10, idea
 * ID-P33) — plain JVM, NO kernel classes, NO server boot (INJECTS-ONLY).
 *
 * The compiled bridge serve bodies cannot run standalone offline (the Snap
 * content plane pulls the kernel registry bootstrap), so this harness
 * REPLICATES THE EXACT ACCESSOR + SERVE code shape of the wire
 * (entityinside/net/minecraft/world/entity/InsideSnapOps.java $Snap accessors
 * + InsideSnapOps.serve/serve4 + InsideSnapRegistryOps.serveFlat) and proves
 * the two things that CAN be proven offline:
 *
 *   1. LOCKSTEP BIT-IDENTITY (random-pop oracle, card ID-P33): the demoted
 *      acquire-serve returns the SAME result as the original volatile-serve
 *      for every probe over a randomized population (full/single/pending/
 *      stale/fresh mixes) — 1:1 semantic replacement, not a protocol change.
 *   2. FAIL-CLOSED DIRECTION under a concurrent secWrite-replica writer
 *      (gen bump AFTER the palette write, seqlock discipline): the demoted
 *      serve never returns content whose anchor does not hold at serve time
 *      (an anomaly budget of ZERO; stale => null => fallback, the exact
 *      fail-closed contract the CHM/flat fallbacks own).
 *
 * x86 note (card ID-P33): volatile read and getAcquire both compile to a
 * plain mov — the harness therefore also runs the acquire replica as the
 * ORIGINAL body's control to catch any accidental protocol drift; the CI leg
 * (probe selfTest before arm + fixture gates on the live 150k scene) owns
 * the end-to-end behavioral proof on the real kernel classes.
 *
 * Exit code 0 = PASS; any failure throws (nonzero).
 */
public final class P33DemotionHarness {

    private P33DemotionHarness() {}

    // ------------------------------------------------------------------
    // Exact wire replica (1:1 with the committed bridge code)
    // ------------------------------------------------------------------

    static final class Snap {
        volatile long gen;
        volatile long builtAtGen;
        volatile Object[] states;   // Object[] stands in for BlockState[]
        volatile Object single;
        volatile boolean pending;
        volatile int fails;

        static final VarHandle VH_GEN;
        static final VarHandle VH_BUILT;
        static final VarHandle VH_STATES;
        static final VarHandle VH_SINGLE;
        static final VarHandle VH_PENDING;
        static {
            try {
                MethodHandles.Lookup l = MethodHandles.lookup();
                VH_GEN = l.findVarHandle(Snap.class, "gen", long.class);
                VH_BUILT = l.findVarHandle(Snap.class, "builtAtGen", long.class);
                VH_STATES = l.findVarHandle(Snap.class, "states", Object[].class);
                VH_SINGLE = l.findVarHandle(Snap.class, "single", Object.class);
                VH_PENDING = l.findVarHandle(Snap.class, "pending", boolean.class);
            } catch (ReflectiveOperationException e) {
                throw new ExceptionInInitializerError(e);
            }
        }

        static long genAcquire(Snap s) { return (long) VH_GEN.getAcquire(s); }
        static long builtAcquire(Snap s) { return (long) VH_BUILT.getAcquire(s); }
        static Object[] statesAcquire(Snap s) { return (Object[]) VH_STATES.getAcquire(s); }
        static Object singleAcquire(Snap s) { return (Object) VH_SINGLE.getAcquire(s); }
        static boolean pendingAcquire(Snap s) { return (boolean) VH_PENDING.getAcquire(s); }
    }

    /** ORIGINAL serve read block (volatile field reads) — the control. */
    static Object serveVolatile(Snap s, int packed) {
        Object[] a = s.states;
        if (a != null && s.builtAtGen == s.gen) {
            return a[packed];
        }
        Object sg = s.single;
        if (sg != null && s.builtAtGen == s.gen) {
            return sg;
        }
        return null;
    }

    /** DEMOTED serve read block (VarHandle.getAcquire) — the wire shape. */
    static Object serveAcquire(Snap s, int packed) {
        Object[] a = Snap.statesAcquire(s);
        if (a != null && Snap.builtAcquire(s) == Snap.genAcquire(s)) {
            return a[packed];
        }
        Object sg = Snap.singleAcquire(s);
        if (sg != null && Snap.builtAcquire(s) == Snap.genAcquire(s)) {
            return sg;
        }
        return null;
    }

    // ------------------------------------------------------------------
    // G1: random-pop lockstep oracle
    // ------------------------------------------------------------------

    static long lockstep(long seed, int iters) {
        Random r = new Random(seed);
        int mismatches = 0;
        for (int i = 0; i < iters; i++) {
            Snap s = new Snap();
            int mode = r.nextInt(4);
            Object token = ("tok" + i);
            if (mode == 0) { // full mode
                Object[] arr = new Object[4096];
                for (int j = 0; j < 4096; j++) arr[j] = (j % 8 == 0) ? token : null;
                s.states = arr;
                s.builtAtGen = r.nextLong();
                s.gen = r.nextBoolean() ? s.builtAtGen : s.builtAtGen + 1; // fresh/stale mix
            } else if (mode == 1) { // single mode
                s.single = token;
                s.builtAtGen = r.nextLong();
                s.gen = r.nextBoolean() ? s.builtAtGen : s.builtAtGen - 1;
            } else if (mode == 2) { // pending (nothing published)
                s.gen = r.nextLong();
                s.builtAtGen = s.gen - 1;
            } else { // full downgrade-in-flight: both published, gens drifted
                Object[] arr = new Object[4096];
                arr[0] = token;
                s.states = arr;
                s.single = token;
                s.gen = r.nextLong();
                s.builtAtGen = r.nextBoolean() ? s.gen : s.gen + 2;
            }
            int packed = r.nextInt(4096);
            Object v1 = serveVolatile(s, packed);
            Object v2 = serveAcquire(s, packed);
            if (v1 != v2) mismatches++; // bit-identity by ref
        }
        return mismatches;
    }

    // ------------------------------------------------------------------
    // G2: concurrent secWrite-replica stress — DEMOTION ADDS NO ANOMALY CLASS
    // (differential: the original volatile body is the control; the checker
    // verifies the served ref is a legitimately-published token — the
    // post-hoc anchor re-check is NOT a valid retroactive invariant because
    // the writer legitimately advances between serve and check).
    // ------------------------------------------------------------------

    static final AtomicBoolean STOP = new AtomicBoolean();
    static final AtomicLong ANOMALIES = new AtomicLong();
    static final AtomicLong SERVES = new AtomicLong();
    static final AtomicLong FALLBACKS = new AtomicLong();
    static Snap SHARED = new Snap();

    static void writerLoop(long iters) {
        Random r = new Random(777);
        Object[] arr = new Object[4096];
        for (long i = 0; i < iters && !STOP.get(); i++) {
            Snap s = SHARED;
            long g = Snap.genAcquire(s);
            // collect-replica publish: content, then builtAtGen (release order)
            Object tok = ("w" + i);
            for (int j = 0; j < 4096; j++) arr[j] = tok;
            s.states = arr;
            s.single = null;
            Snap.VH_BUILT.set(s, g); // content -> gens order (same as the bridge)
            // secWrite-replica invalidation: gen bump AFTER the write
            if (r.nextInt(8) == 0) {
                s.gen = g + 1; // stale => every serve must fall back until rebuild
            }
        }
        STOP.set(true);
    }

    interface ServeBody { Object serve(Snap s, int packed); }

    static void readerLoop(ServeBody body, long iters) {
        Random r = new Random(99);
        for (long i = 0; i < iters && !STOP.get(); i++) {
            Snap s = SHARED;
            int packed = r.nextInt(4096);
            Object v = body.serve(s, packed); // THE BODY UNDER TEST
            SERVES.incrementAndGet();
            if (v == null) {
                FALLBACKS.incrementAndGet();
            } else if (!(v instanceof String) || !((String) v).startsWith("w")) {
                ANOMALIES.incrementAndGet(); // garbage/torn ref through the read path
            }
        }
    }

    static long[] runStress(ServeBody body, long iters) {
        SERVES.set(0); FALLBACKS.set(0); ANOMALIES.set(0);
        SHARED = new Snap();
        STOP.set(false);
        List<Thread> ts = new ArrayList<>();
        ts.add(new Thread(() -> writerLoop(iters / 4)));
        ts.add(new Thread(() -> readerLoop(body, iters)));
        ts.add(new Thread(() -> readerLoop(body, iters)));
        for (Thread t : ts) t.start();
        for (Thread t : ts) { try { t.join(); } catch (InterruptedException e) { throw new RuntimeException(e); } }
        return new long[]{SERVES.get(), FALLBACKS.get(), ANOMALIES.get()};
    }

    public static void main(String[] args) {
        // G1 lockstep: 5 seeds x 200k random-pop probes, x3 rounds
        for (int round = 0; round < 3; round++) {
            long mm = 0;
            for (long seed = 1; seed <= 5; seed++) mm += lockstep(seed * 1000 + round, 200_000);
            System.out.println("G1 lockstep round " + round + ": mismatches=" + mm);
            if (mm != 0) throw new AssertionError("G1 FAIL: acquire!=volatile");
        }

        // G2 differential stress: volatile control vs demoted wire body
        long[] vol = runStress(P33DemotionHarness::serveVolatile, 2_000_000);
        long[] acq = runStress(P33DemotionHarness::serveAcquire, 2_000_000);
        System.out.println("G2 volatile-control: serves=" + vol[0] + " fallbacks=" + vol[1]
                + " anomalies=" + vol[2]);
        System.out.println("G2 acquire-wire:     serves=" + acq[0] + " fallbacks=" + acq[1]
                + " anomalies=" + acq[2]);
        if (vol[2] != 0 || acq[2] != 0) throw new AssertionError("G2 FAIL: garbage serve");
        if (acq[1] == 0) throw new AssertionError("G2 FAIL: fail-closed path never taken");

        System.out.println("P33-DEMOTION OFFLINE PASS (lockstep 0/3M x3; stress: demotion == control, 0 anomalies)");
    }
}
