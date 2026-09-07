package net.minecraft.world.level.levelgen.synth;

import java.lang.management.GarbageCollectorMXBean;
import java.lang.management.ManagementFactory;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * TASK-17 lifecycle soak: 10-minute churn of the phantom-reaper bridge
 * ({@code ImprovedNoiseNativeOps}, TASK-01/09) under GC pressure on a
 * small heap (driver is launched by run_soak.sh with -Xms256m -Xmx256m
 * -XX:+UseG1GC and the real libpaper_native_jni.so kernels).
 *
 * Profile (churn patterns reused from LifecycleBench M2/M4 + the
 * bench/noise_ab churn profile):
 *   - waves of W=50k ImprovedNoise instances, one native handle each,
 *     built+sampled then dropped whole (20k-burst profile scaled up);
 *   - EVEN waves = GC-pressure windows: a parallel junk allocator
 *     (64x4096 B/iteration, the gcchurn profile) plus explicit
 *     System.gc() every ~500ms until the reaper frees the whole wave;
 *   - ODD waves = quiet windows: NO explicit gc, no junker — a 1.5s
 *     allocation pause, then the next wave's builds trigger natural
 *     young GCs; the observer timestamps the phantom reclaim end-to-end
 *     (exercises the quiet-reclaim path and the 2s expunge tick).
 *
 * Assertions the summary derives from the emitted metrics:
 *   A1 0 double-free / no JNI fatal error (freed_total == built_total
 *      exactly, JVM alive at exit, no hs_err / fatal markers — the
 *      bridge's AtomicBoolean CAS release() keeps free at-most-once);
 *   A2 handle count bounded (live_handles_max <= 2x wave, == 0 settled);
 *   A3 no OutOfMemoryError;
 *   A4 throughput does not collapse (first-vs-last-wave ns/op drift).
 */
public final class SoakBench {
    static volatile double SINK;

    static {
        // load the REAL native kernels (same .so the server injects)
        final String p = System.getProperty("crussty.native", "");
        if (p.isBlank()) {
            throw new IllegalStateException("pass -Dcrussty.native=/abs/path/libpaper_native_jni.so");
        }
        System.load(p);
    }

    // ---------- configuration ----------
    static final long RECLAIM_WAIT_MS = 12_000;   // explicit budget for pressure waves
    static final long PENDING_STALE_MS = 60_000;  // safety cap for quiet pendings
    static final long QUIET_PAUSE_MS = 1_500;     // no-gc window after a quiet drop

    // ---------- reclaim bookkeeping (observer thread) ----------

    static final class Pend {
        final int wave;
        final boolean pressure;
        final long target;
        final long t0Nanos;
        Pend(int wave, boolean pressure, long target, long t0Nanos) {
            this.wave = wave; this.pressure = pressure;
            this.target = target; this.t0Nanos = t0Nanos;
        }
    }

    static final List<Pend> PENDING = Collections.synchronizedList(new ArrayList<>());
    static final List<Long> RECLAIM_PRESSURE_MS = Collections.synchronizedList(new ArrayList<>());
    static final List<Long> RECLAIM_QUIET_MS = Collections.synchronizedList(new ArrayList<>());
    static final AtomicLong RECLAIM_TIMEOUTS = new AtomicLong();
    static final AtomicLong LIVE_MAX = new AtomicLong();
    static volatile boolean OOME;
    static volatile boolean DONE;

    public static void main(String[] argv) throws Exception {
        final long durationSec = argv.length > 0 ? Long.parseLong(argv[0]) : 600;
        final int waveSize = argv.length > 1 ? Integer.parseInt(argv[1]) : 50_000;
        System.out.println("soak\tconfig\tduration_s\t" + durationSec);
        System.out.println("soak\tconfig\twave_size\t" + waveSize);
        System.out.println("soak\tconfig\tvm_version\t" + System.getProperty("java.vm.version"));
        System.out.flush();

        final long tStart = System.nanoTime();
        final long gcCount0 = gcCount(), gcTime0 = gcTimeMs();
        final long deadline = tStart + durationSec * 1_000_000_000L;

        final Thread observer = new Thread(SoakBench::observe, "soak-observer");
        observer.setDaemon(true);
        observer.start();

        // parallel junk allocator, toggled only during GC-pressure waves
        final AtomicBoolean junkOn = new AtomicBoolean(false);
        final Thread junker = new Thread(() -> {
            long acc = 0;
            while (!DONE) {
                if (!junkOn.get()) {
                    try { Thread.sleep(20); } catch (InterruptedException e) { return; }
                    continue;
                }
                byte[][] junkArr = new byte[64][];
                for (int i = 0; i < 64; i++) junkArr[i] = new byte[4096];
                acc += junkArr.length;
            }
            SINK += acc;
        }, "soak-junker");
        junker.setDaemon(true);
        junker.start();

        long builtTotal = 0;
        int wave = 0, pressureWaves = 0, quietWaves = 0;
        long firstPressureNs = -1, lastPressureNs = -1;
        long firstQuietNs = -1, lastQuietNs = -1;

        try {
            while (System.nanoTime() < deadline) {
                final boolean pressure = (wave % 2 == 0);
                junkOn.set(pressure); // junker runs through build + observation
                final long freedBase = ImprovedNoiseNativeOps.freedHandles();

                // ---- build phase: W instances, one native handle each ----
                final ImprovedNoise[] hold = new ImprovedNoise[waveSize];
                final long tBuild0 = System.nanoTime();
                for (int i = 0; i < waveSize; i++) {
                    final ImprovedNoise in = new ImprovedNoise(wave * 31 + i);
                    SINK += ImprovedNoiseNativeOps.noise(in, in.p, in.xo, in.yo, in.zo,
                            i, i * 0.5, i * 0.25, 1.0, 2.0);
                    hold[i] = in;
                }
                final long buildNsPer = (System.nanoTime() - tBuild0) / waveSize;
                builtTotal += waveSize;
                if (pressure) pressureWaves++; else quietWaves++;
                if (pressure) {
                    if (firstPressureNs < 0) firstPressureNs = buildNsPer;
                    lastPressureNs = buildNsPer;
                } else {
                    if (firstQuietNs < 0) firstQuietNs = buildNsPer;
                    lastQuietNs = buildNsPer;
                }
                System.out.println("wave\t" + wave + "\t" + (pressure ? "pressure" : "quiet")
                        + "\tns_per_build_and_sample\t" + buildNsPer);
                System.out.println("wave\t" + wave + "\t" + (pressure ? "pressure" : "quiet")
                        + "\thandles_built_cum\t" + builtTotal);
                System.out.flush();

                // ---- drop the whole burst (die while pressure may continue) ----
                final long tDrop0 = System.nanoTime();
                for (int i = 0; i < waveSize; i++) hold[i] = null;
                final long dropNs = System.nanoTime() - tDrop0;
                System.out.println("wave\t" + wave + "\t" + (pressure ? "pressure" : "quiet")
                        + "\tns_per_drop_wave\t" + dropNs);
                System.out.flush();

                // ---- register reclaim expectation; observer timestamps it ----
                final Pend pend = new Pend(wave, pressure, freedBase + waveSize, System.nanoTime());
                PENDING.add(pend);

                if (pressure) {
                    // explicit GC pressure: System.gc() every ~500ms until the
                    // reaper has freed the whole wave (cap RECLAIM_WAIT_MS)
                    final long tWait0 = System.nanoTime();
                    while (System.nanoTime() - tWait0 < RECLAIM_WAIT_MS * 1_000_000L
                            && PENDING.contains(pend)) {
                        Thread.sleep(500);
                        System.gc();
                    }
                    if (PENDING.contains(pend)) { // reaper did not keep up in budget
                        PENDING.remove(pend);
                        RECLAIM_TIMEOUTS.incrementAndGet();
                        System.out.println("wave\t" + wave + "\tpressure\treclaim_ms\tTIMEOUT>"
                                + RECLAIM_WAIT_MS);
                    }
                } else {
                    // quiet window: NO explicit gc — hands off; natural GCs from
                    // the next wave's allocations drive the phantom notifications
                    Thread.sleep(QUIET_PAUSE_MS);
                }
                junkOn.set(false);
                wave++;
            }
        } catch (OutOfMemoryError e) {
            OOME = true;
            System.out.println("sum\toome\t1");
            System.out.println("sum\toome_detail\t" + e.getClass().getName());
        }
        final long wallS = (System.nanoTime() - tStart) / 1_000_000_000L;

        // ---- settle: every handle must have been freed EXACTLY once ----
        System.gc(); System.gc();
        Thread.sleep(300);
        ImprovedNoiseNativeOps.expungeNow();
        Thread.sleep(200);
        // stop the observer AFTER settle so the last quiet pending is recorded
        DONE = true;
        Thread.sleep(100);
        final long freedTotal = ImprovedNoiseNativeOps.freedHandles();
        final int liveAfter = ImprovedNoiseNativeOps.liveHandles();

        System.out.println("sum\twall_s\t" + wallS);
        System.out.println("sum\twaves\t" + wave);
        System.out.println("sum\twaves_pressure\t" + pressureWaves);
        System.out.println("sum\twaves_quiet\t" + quietWaves);
        System.out.println("sum\thandles_built_total\t" + builtTotal);
        System.out.println("sum\tfreed_total\t" + freedTotal);
        System.out.println("sum\tnative_unfreed\t" + (builtTotal - freedTotal));
        System.out.println("sum\tlive_handles_max\t" + LIVE_MAX.get());
        System.out.println("sum\tlive_handles_after_settle\t" + liveAfter);
        System.out.println("sum\treclaim_pressure_p50_ms\t" + pct(RECLAIM_PRESSURE_MS, 50));
        System.out.println("sum\treclaim_pressure_p95_ms\t" + pct(RECLAIM_PRESSURE_MS, 95));
        System.out.println("sum\treclaim_pressure_max_ms\t" + max(RECLAIM_PRESSURE_MS));
        System.out.println("sum\treclaim_pressure_count\t" + RECLAIM_PRESSURE_MS.size());
        System.out.println("sum\treclaim_pressure_timeouts\t" + RECLAIM_TIMEOUTS.get());
        System.out.println("sum\treclaim_quiet_p50_ms\t" + pct(RECLAIM_QUIET_MS, 50));
        System.out.println("sum\treclaim_quiet_p95_ms\t" + pct(RECLAIM_QUIET_MS, 95));
        System.out.println("sum\treclaim_quiet_max_ms\t" + max(RECLAIM_QUIET_MS));
        System.out.println("sum\treclaim_quiet_count\t" + RECLAIM_QUIET_MS.size());
        System.out.println("sum\tns_build_pressure_first\t" + firstPressureNs);
        System.out.println("sum\tns_build_pressure_last\t" + lastPressureNs);
        System.out.println("sum\tns_build_quiet_first\t" + firstQuietNs);
        System.out.println("sum\tns_build_quiet_last\t" + lastQuietNs);
        System.out.println("sum\tgc_collections\t" + (gcCount() - gcCount0));
        System.out.println("sum\tgc_time_ms\t" + (gcTimeMs() - gcTime0));
        System.out.println("sum\toome\t" + (OOME ? 1 : 0));
        System.out.println("sum\tSINK\t" + Double.doubleToRawLongBits(SINK));
        System.out.println("soak\tdone\t1\t1");
        System.out.flush();
        observer.join(2000);
    }

    /** Observer: timestamps reclaim completions and tracks live-handle max.
     *  Never calls System.gc() and never touches the stripes except the
     *  2Hz liveHandles() probe (explicitly allowed — this is not the A/B
     *  fairness contract; it is the boundedness assertion). */
    static void observe() {
        long lastLiveProbe = System.nanoTime();
        while (!DONE) {
            final long freed = ImprovedNoiseNativeOps.freedHandles(); // AtomicLong only
            final long now = System.nanoTime();
            synchronized (PENDING) {
                for (int i = PENDING.size() - 1; i >= 0; i--) {
                    final Pend p = PENDING.get(i);
                    if (freed >= p.target) {
                        final long ms = (now - p.t0Nanos) / 1_000_000L;
                        (p.pressure ? RECLAIM_PRESSURE_MS : RECLAIM_QUIET_MS).add(ms);
                        System.out.println("wave\t" + p.wave + "\t" + (p.pressure ? "pressure" : "quiet")
                                + "\treclaim_ms\t" + ms);
                        PENDING.remove(i);
                    } else if (now - p.t0Nanos > PENDING_STALE_MS * 1_000_000L) {
                        PENDING.remove(i);
                        RECLAIM_TIMEOUTS.incrementAndGet();
                        System.out.println("wave\t" + p.wave + "\tstale\treclaim_ms\tTIMEOUT>"
                                + PENDING_STALE_MS);
                    }
                }
            }
            if (now - lastLiveProbe > 500_000_000L) { // ~2 Hz
                lastLiveProbe = now;
                final int live = ImprovedNoiseNativeOps.liveHandles();
                LIVE_MAX.accumulateAndGet(live, Math::max);
            }
            try { Thread.sleep(5); } catch (InterruptedException e) { return; }
        }
    }

    // ---------- percentile helpers (nearest-rank) ----------

    static long pct(List<Long> xs, int p) {
        if (xs.isEmpty()) return -1;
        final List<Long> s = new ArrayList<>(xs);
        Collections.sort(s);
        int idx = (int) Math.ceil(p / 100.0 * s.size()) - 1;
        if (idx < 0) idx = 0;
        if (idx >= s.size()) idx = s.size() - 1;
        return s.get(idx);
    }

    static long max(List<Long> xs) {
        long m = -1;
        for (final long v : xs) if (v > m) m = v;
        return m;
    }

    static long gcCount() {
        long n = 0;
        for (GarbageCollectorMXBean b : ManagementFactory.getGarbageCollectorMXBeans()) {
            final long c = b.getCollectionCount();
            if (c >= 0) n += c;
        }
        return n;
    }

    static long gcTimeMs() {
        long n = 0;
        for (GarbageCollectorMXBean b : ManagementFactory.getGarbageCollectorMXBeans()) {
            final long c = b.getCollectionTime();
            if (c >= 0) n += c;
        }
        return n;
    }
}
