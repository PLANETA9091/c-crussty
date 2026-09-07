package net.minecraft.world.level.levelgen.synth;

import java.lang.management.GarbageCollectorMXBean;
import java.lang.management.ManagementFactory;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * P500-style lifecycle A/B: OLD (finalize + one global synchronized
 * WeakHashMap) vs NEW (phantom reaper + 16 identity stripes), both driving
 * the SAME native kernels (nativeBuildHandle / nativeNoise /
 * nativeFreeHandle from libpaper_native_jni.so, System.load'ed by run.sh).
 *
 *   java net.minecraft...LifecycleBench <old|new> hotpath|churn|gcchurn
 *
 * Output: TSV lines  "impl\tscenario\tmetric\tvalue"  on stdout.
 *
 * Fairness contract:
 *   - identical native kernels, identical argument values, same JVM flags;
 *   - both impls expose the same counters (FREED/liveHandles); counters add
 *     no map access on the quiet path (plain AtomicLong reads);
 *   - the quiet-reclaim phase does ONE System.gc() at start (a GC is not a
 *     map access — it is what a real server hits right after a gen burst)
 *     and then NEVER touches the map: exactly the "world gen went quiet"
 *     condition under which the old design pinned native memory;
 *   - the tickled phase touches the map every 5ms on both impls (a real
 *     continuing gen does that implicitly on every noise() of a fresh
 *     instance), isolating WHO does the freeing: finalizer thread vs reaper.
 */
public final class LifecycleBench {
    static volatile double SINK;

    static {
        // load the REAL native kernels (same .so the server injects)
        final String p = System.getProperty("crussty.native", "");
        if (p.isBlank()) {
            throw new IllegalStateException("pass -Dcrussty.native=/abs/path/libpaper_native_jni.so");
        }
        System.load(p);
    }

    interface Ops {
        double noise(ImprovedNoise self, double x, double y, double z);
        int mapEntries();   // map size (NOTE: a real map access — never call in quiet phase)
        long freed();       // AtomicLong read — quiet-safe
        void tickle();      // real map access (expunge parity)
        String name();
    }

    static Ops newOps() {
        return new Ops() {
            public double noise(ImprovedNoise s, double x, double y, double z) {
                return ImprovedNoiseNativeOps.noise(s, s.p, s.xo, s.yo, s.zo, x, y, z, 1.0, 2.0);
            }
            public int mapEntries() { return ImprovedNoiseNativeOps.liveHandles(); }
            public long freed() { return ImprovedNoiseNativeOps.freedHandles(); }
            public void tickle() { ImprovedNoiseNativeOps.expungeNow(); }
            public String name() { return "new"; }
        };
    }

    static Ops oldOps() {
        return new Ops() {
            public double noise(ImprovedNoise s, double x, double y, double z) {
                return ImprovedNoiseNativeOpsOld.noise(s, s.p, s.xo, s.yo, s.zo, x, y, z, 1.0, 2.0);
            }
            public int mapEntries() { return ImprovedNoiseNativeOpsOld.liveHandles(); }
            public long freed() { return ImprovedNoiseNativeOpsOld.freedHandles(); }
            public void tickle() { ImprovedNoiseNativeOpsOld.expungeNow(); }
            public String name() { return "old"; }
        };
    }

    public static void main(String[] argv) throws Exception {
        final String impl = argv[0];        // old | new
        final String scenario = argv[1];    // hotpath | churn | gcchurn
        final Ops ops = impl.equals("old") ? oldOps() : newOps();
        // force both impl classes initialized once up front (reaper thread up)
        oldOps(); newOps();
        System.out.println("impl\tscenario\tmetric\tvalue");

        switch (scenario) {
            case "hotpath" -> hotpath(ops);
            case "churn" -> churn(ops, false);
            case "gcchurn" -> churn(ops, true);
            default -> throw new IllegalArgumentException(scenario);
        }
        System.out.println(ops.name() + "\t" + scenario + "\tSINK\t" +
                Double.doubleToRawLongBits(SINK));
        System.out.flush();
    }

    // ---------- M1: hot-path ns/noise under T-thread contention ----------

    static final int HP_INSTANCES = 256;
    static final int HP_ROUNDS = 5;
    static final int HP_WARM = 2;
    static final long HP_BATCH_NS = 120_000_000L;

    static void hotpath(Ops ops) throws Exception {
        final ImprovedNoise[] inst = new ImprovedNoise[HP_INSTANCES];
        for (int i = 0; i < HP_INSTANCES; i++) {
            inst[i] = new ImprovedNoise(i);
            ops.noise(inst[i], i, i * 0.5, i * 0.25); // build all handles up front
        }

        for (int threads : new int[]{1, 2, 4, 8}) {
            double best = Double.MAX_VALUE;
            for (int r = 0; r < HP_WARM + HP_ROUNDS; r++) {
                final int T = threads;
                final CountDownLatch start = new CountDownLatch(1);
                final CountDownLatch done = new CountDownLatch(T);
                final AtomicLong calls = new AtomicLong();
                final Thread[] ts = new Thread[T];
                for (int t = 0; t < T; t++) {
                    final int base = t;
                    ts[t] = new Thread(() -> {
                        try { start.await(); } catch (InterruptedException e) { return; }
                        long n = 0;
                        double acc = 0;
                        final long deadline = System.nanoTime() + HP_BATCH_NS;
                        // every thread walks the SAME 256 instances (max contention)
                        while (System.nanoTime() < deadline) {
                            for (int i = 0; i < HP_INSTANCES; i++) {
                                final ImprovedNoise in = inst[(i + base) & (HP_INSTANCES - 1)];
                                acc += ops.noise(in, i, i * 0.5, i * 0.25);
                                n++;
                            }
                        }
                        SINK += acc;
                        calls.addAndGet(n);
                        done.countDown();
                    }, "hp-" + t);
                    ts[t].start();
                }
                final long t0 = System.nanoTime();
                start.countDown();
                done.await();
                final long dt = System.nanoTime() - t0;
                if (r >= HP_WARM) best = Math.min(best, (double) dt / calls.get());
                for (Thread th : ts) th.join();
            }
            System.out.printf("%s\thotpath\tns_per_noise_t%d\t%.1f%n", ops.name(), threads, best);
        }
        System.out.println(ops.name() + "\thotpath\tmap_entries\t" + ops.mapEntries());
    }

    // ---------- M2/M4: churn -> reclamation latency + leak check ----------

    static final int CHURN_B = 20_000;
    static final long RECLAIM_TIMEOUT_MS = 12_000;

    static void churn(Ops ops, boolean junk) throws Exception {
        final String sc = junk ? "gcchurn" : "churn";
        final AtomicBoolean stopJunk = new AtomicBoolean(false);
        final Thread junker;
        if (junk) {
            junker = new Thread(() -> {
                long acc = 0;
                while (!stopJunk.get()) {
                    byte[][] junkArr = new byte[64][];
                    for (int i = 0; i < 64; i++) junkArr[i] = new byte[4096];
                    acc += junkArr.length;
                }
                SINK += acc;
            }, "junker");
            junker.setDaemon(true);
            junker.start();
        } else {
            junker = null;
        }

        // ---- build phase: B instances, one native handle each ----
        final ImprovedNoise[] hold = new ImprovedNoise[CHURN_B];
        final long tBuild0 = System.nanoTime();
        for (int i = 0; i < CHURN_B; i++) {
            final ImprovedNoise in = new ImprovedNoise(i);
            SINK += ops.noise(in, i, i * 0.5, i * 0.25); // builds handle
            hold[i] = in;
        }
        System.out.println(ops.name() + "\t" + sc + "\tns_per_build_and_sample\t" +
                ((System.nanoTime() - tBuild0) / CHURN_B));
        System.out.println(ops.name() + "\t" + sc + "\thandles_built\t" + ops.mapEntries());

        // ---- GC health DURING churn: die + drop while pressure continues ----
        final long gcCount0 = gcCount(), gcTime0 = gcTimeMs();
        final long tDrop0 = System.nanoTime();
        for (int i = 0; i < CHURN_B; i++) hold[i] = null;      // whole burst dies
        final long dropNs = System.nanoTime() - tDrop0;
        System.out.println(ops.name() + "\t" + sc + "\tns_per_drop_wave\t" + dropNs);

        // ---- quiet reclaim: ONE GC (not a map access), then hands off the map ----
        System.gc();
        final long tRec0 = System.nanoTime();
        long freed = ops.freed();
        boolean complete = freed >= CHURN_B;
        while (!complete && System.nanoTime() - tRec0 < RECLAIM_TIMEOUT_MS * 1_000_000L) {
            Thread.sleep(5);
            freed = ops.freed();          // AtomicLong only — no map access
            complete = freed >= CHURN_B;
        }
        System.out.println(ops.name() + "\t" + sc + "\tquiet_reclaim_ms\t" +
                (complete ? (System.nanoTime() - tRec0) / 1_000_000L : "TIMEOUT>" + RECLAIM_TIMEOUT_MS));

        // ---- tickled reclaim: map touched every 5ms (gen continues) ----
        // A real continuing gen ALLOCATES -> GCs keep coming; model that with
        // a gc() every ~500ms so the old 2-GC finalizer chain can complete.
        final long tTick0 = System.nanoTime();
        int tick = 0;
        while (ops.freed() < CHURN_B && System.nanoTime() - tTick0 < RECLAIM_TIMEOUT_MS * 1_000_000L) {
            ops.tickle();
            Thread.sleep(5);
            if (++tick % 100 == 0) System.gc();
        }
        System.out.println(ops.name() + "\t" + sc + "\ttickled_reclaim_ms\t" +
                (ops.freed() >= CHURN_B ? (System.nanoTime() - tTick0) / 1_000_000L
                                        : "TIMEOUT>" + RECLAIM_TIMEOUT_MS));

        // ---- settle + leak check ----
        System.gc(); System.gc();
        Thread.sleep(300);
        ops.tickle();
        Thread.sleep(200);
        System.out.println(ops.name() + "\t" + sc + "\tfreed_total\t" + ops.freed());
        System.out.println(ops.name() + "\t" + sc + "\tnative_unfreed\t" + (CHURN_B - ops.freed()));
        System.out.println(ops.name() + "\t" + sc + "\tmap_entries_after_settle\t" + ops.mapEntries());
        System.out.println(ops.name() + "\t" + sc + "\tgc_collections\t" + (gcCount() - gcCount0));
        System.out.println(ops.name() + "\t" + sc + "\tgc_time_ms\t" + (gcTimeMs() - gcTime0));

        stopJunk.set(true);
        if (junker != null) junker.join(1000);
    }

    static long gcCount() {
        long n = 0;
        for (GarbageCollectorMXBean b : ManagementFactory.getGarbageCollectorMXBeans()) {
            long c = b.getCollectionCount();
            if (c >= 0) n += c;
        }
        return n;
    }

    static long gcTimeMs() {
        long n = 0;
        for (GarbageCollectorMXBean b : ManagementFactory.getGarbageCollectorMXBeans()) {
            long c = b.getCollectionTime();
            if (c >= 0) n += c;
        }
        return n;
    }
}
