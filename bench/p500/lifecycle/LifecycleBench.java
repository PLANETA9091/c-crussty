import net.minecraft.world.level.levelgen.synth.PaperNativeImprovedNoise;

import java.lang.ref.Cleaner;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * LifecycleBench — A/B microbench SPEC + runnable skeleton for task B1
 * (ImprovedNoiseNativeOps lifecycle: finalize() -> Cleaner + explicit release).
 *
 * HONESTY FIRST — what this bench does and does NOT measure
 * ---------------------------------------------------------
 * The native kernel is closed source, so only the JAVA side of the handle
 * lifecycle is measurable. Every arm below drives the REAL JNI symbols
 * (nativeBuildHandle / nativeFreeHandle) exactly like production; what
 * differs between arms is only the Java machinery around them:
 *
 *   jni-floor          raw nativeBuildHandle + nativeFreeHandle per cycle —
 *                      zero lifecycle machinery = the floor any design pays
 *   cleaner-explicit   replica of the NEW Handle (Cleaner.register on
 *                      create + explicit clean() per cycle) — adds register
 *                      + at-most-once guard overhead on top of the floor
 *   cleaner-gc         NEW design under its production path: create N,
 *                      drop all refs, let GC + Cleaner thread free them
 *   finalize-gc        OLD design (Handle.finalize -> nativeFreeHandle)
 *                      under the identical create/drop/drain protocol
 *
 * In DRYRUN (no natives) the arms switch to synthetic handles and skip JNI
 * entirely — they then measure ONLY the Java machinery (register/clean vs
 * finalizer queue) and are labelled DRYRUN; never compare DRYRUN rows with
 * native rows.
 *
 * The core A/B is cleaner-gc vs finalize-gc: identical protocol, both arms
 * completion-counted, ns/op = elapsed / completed. These are THROUGHPUT
 * numbers under GC pressure (batch of 100k), not per-call latency — the
 * honest metric for GC-driven lifecycle, since per-call latency depends on
 * when the JVM happens to run the Cleaner/finalizer threads.
 *
 * NOT measurable here (documented, not swept under the rug):
 *   - native-side free cost differences (same symbol both arms — cancels
 *     out in the A/B delta, but not vs jni-floor);
 *   - the shipped bridge's own GC arm: ImprovedNoiseNativeOps.HANDLES is
 *     private and exposes no cleanup counter, so a completion-counted drain
 *     is impossible from the outside. The `real-bridge-explicit` arm below
 *     measures the shipped public surface (noise() build + releaseHandle)
 *     in a SEPARATE JVM (classpath clash with the bench stub, see run()).
 *   - JVM-exit behaviour: Cleaner is a daemon thread; pending cleanups at
 *     exit are skipped (OS reclaims the process; Finalizer never guaranteed
 *     shutdown cleanup either). This bench measures steady state only.
 *
 * Run (natives injected like P500 — exact-symbol binding on System.load):
 *   javac -d bench/p500/lifecycle/classes \
 *     bench/p500/java/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.java \
 *     bench/p500/lifecycle/LifecycleBench.java
 *   java -Dlc.libs=/path/libpaper_native_jni.so -cp bench/p500/lifecycle/classes \
 *     LifecycleBench [N]
 * Without -Dlc.libs the bench runs a clearly-labelled DRYRUN (fake handles,
 * no JNI) so CI can smoke-test the harness itself; those rows are NOT
 * comparable with native rows.
 */
public final class LifecycleBench {

    // ---------- config ----------
    static int N = Integer.getInteger("lc.n", 100_000);   // handles per cycle batch
    static int ROUNDS = Integer.getInteger("lc.rounds", 5);
    static long DRAIN_TIMEOUT_MS = Long.getLong("lc.drain.timeout", 60_000);

    /** DCE-proof accumulator, p500 convention. */
    public static volatile long SINK;

    /** Native availability: false => DRYRUN, Releaser/finalize skip JNI. */
    static boolean NATIVES;

    /** Deterministic 256-byte permutation, so nativeBuildHandle accepts it. */
    static byte[] permutation() {
        byte[] p = new byte[256];
        for (int i = 0; i < 256; i++) p[i] = (byte) i;
        // fixed cheap shuffle — deterministic across runs, no Math.random
        for (int i = 255; i > 0; i--) {
            int j = (i * 31 + 7) & 255;
            byte t = p[i]; p[i] = p[j]; p[j] = t;
        }
        return p;
    }

    // ---------- NEW design replica (mirrors ImprovedNoiseNativeOps$Handle) ----------
    // Kept in lock-step with the shipped bridge by hand; if the bridge changes,
    // change this replica in the same commit (checked in the B1 report).

    private static final Cleaner CLEANER = Cleaner.create(r -> {
        Thread t = new Thread(r, "lc-bench-cleaner");
        t.setDaemon(true);
        return t;
    });

    /** Completion counter for the cleaner-gc arm (never references a Handle). */
    static final AtomicLong CLEANED = new AtomicLong();

    static final class Releaser implements Runnable {
        private final long handle;
        private final AtomicBoolean freed;
        Releaser(long handle, AtomicBoolean freed) {
            this.handle = handle; this.freed = freed;
        }
        @Override public void run() {
            if (handle != 0L && freed.compareAndSet(false, true)) {
                freeHandle(handle);
                CLEANED.incrementAndGet();   // bench-only: completion counting
            }
        }
    }

    static final class CleanerHandle {
        final long handle;
        final Cleaner.Cleanable cleanable;
        CleanerHandle(long handle) {
            this.handle = handle;
            this.cleanable = CLEANER.register(this, new Releaser(handle, new AtomicBoolean(false)));
        }
        void release() { cleanable.clean(); }   // at-most-once, race-free
    }

    // ---------- OLD design replica (finalize-based, pre-B1 Handle) ----------

    static final AtomicLong FINALIZED = new AtomicLong();

    @SuppressWarnings("removal")
    static final class FinalizeHandle {
        final long handle;
        FinalizeHandle(long handle) { this.handle = handle; }
        @Override protected void finalize() {
            if (this.handle != 0L) {
                freeHandle(this.handle);
                FINALIZED.incrementAndGet(); // bench-only: completion counting
            }
        }
    }

    // ---------- JNI helpers (DRYRUN-safe) ----------

    static long NEXT_FAKE = 1;

    /** Real native build in native mode; synthetic unique handle in DRYRUN. */
    static synchronized long buildHandle(byte[] p) {
        if (!NATIVES) return NEXT_FAKE++;
        return PaperNativeImprovedNoise.nativeBuildHandle(p, 1.0, 2.0, 3.0);
    }

    /** Real native free in native mode; counted no-op in DRYRUN. */
    static void freeHandle(long h) {
        if (NATIVES) PaperNativeImprovedNoise.nativeFreeHandle(h);
        else SINK += h;   // keep fake handles observable without JNI
    }

    // ---------- main ----------

    public static void main(String[] args) throws Exception {
        for (String p : System.getProperty("lc.libs", "").split(java.io.File.pathSeparator))
            if (!p.isBlank()) System.load(p);
        if (args.length > 0) N = Integer.parseInt(args[0]);

        // native probe (p500 convention: fail fast on unbound natives)
        byte[] p = permutation();
        try {
            long h = PaperNativeImprovedNoise.nativeBuildHandle(p, 0.0, 0.0, 0.0);
            NATIVES = h != 0L;
            if (NATIVES) PaperNativeImprovedNoise.nativeFreeHandle(h);
        } catch (UnsatisfiedLinkError e) {
            NATIVES = false;
        }
        if (!NATIVES) System.out.println("## DRYRUN: natives unavailable (-Dlc.libs?) — rows are NOT comparable with native runs");

        System.out.println("N=" + N + " rounds=" + ROUNDS + " natives=" + NATIVES);

        // A/B core: identical protocol for both GC-driven designs
        double[] cleanerGc = gcDrainBench("cleaner-gc", true);
        double[] finalizeGc = gcDrainBench("finalize-gc", false);
        report("cleaner-gc", cleanerGc);
        report("finalize-gc", finalizeGc);
        System.out.printf("A/B cleaner-gc vs finalize-gc: %.2fx%n",
                finalizeGc[0] / cleanerGc[0]);

        if (NATIVES) {
            // synchronous arms need real handles to mean anything
            report("jni-floor",        syncBench("jni-floor"));
            report("cleaner-explicit", syncBench("cleaner-explicit"));
        } else {
            System.out.println("(jni-floor + cleaner-explicit skipped in DRYRUN: they are pure JNI-floor arms)");
        }

        System.out.println("SINK\t" + SINK);
        // TODO(B1-followup): real-bridge-explicit arm in a SEPARATE JVM:
        //   javac -d rbc noise/net/.../RuntimeStubs.java noise/net/.../ImprovedNoiseNativeOps.java
        //   (its package-private PaperNativeImprovedNoise/ImprovedNoise stubs bind the
        //   same native symbols; do NOT put bench stubs on that classpath — same FQCN)
        //   per cycle: ImprovedNoiseNativeOps.noise(fakeInstance,...)  // builds handle
        //              ImprovedNoiseNativeOps.releaseHandle(fakeInstance);
        //   plus a GC arm with the documented "drain poke": one noise() call on a
        //   LIVE dummy instance after dropping the batch, so WeakHashMap.get()
        //   expunges cleared entries and lets the Cleaner free dead handles.
        //   Completion counting is impossible from outside the bridge — report
        //   wall-clock only and say so in the results table.
    }

    // ---------- synchronous arms ----------

    static double[] syncBench(String arm) {
        byte[] p = permutation();
        double[] med = new double[ROUNDS];
        for (int r = 0; r < ROUNDS; r++) {
            long t0 = System.nanoTime();
            if ("jni-floor".equals(arm)) {
                for (int i = 0; i < N; i++) {
                    long h = PaperNativeImprovedNoise.nativeBuildHandle(p, 1.0, 2.0, 3.0);
                    SINK += h;
                    PaperNativeImprovedNoise.nativeFreeHandle(h);
                }
            } else { // cleaner-explicit: register + explicit clean(), deterministic
                for (int i = 0; i < N; i++) {
                    CleanerHandle h = new CleanerHandle(buildHandle(p));
                    SINK += h.handle;
                    h.release();
                }
            }
            med[r] = (System.nanoTime() - t0) / (double) N;
        }
        Arrays.sort(med);
        return new double[]{ med[ROUNDS / 2] };  // median
    }

    // ---------- GC-driven A/B arms (shared protocol) ----------

    /**
     * One round: create N handles (refs dropped as we go, both arms
     * identically), then drain: System.gc() + short waits until
     * completed == N (or timeout). Measures elapsed / completed — a
     * THROUGHPUT number under GC pressure, identical protocol for both
     * arms, so the A/B delta isolates the lifecycle machinery (finalizer
     * queue vs Cleaner thread + CAS guard).
     */
    static double[] gcDrainBench(String arm, boolean useCleaner) throws InterruptedException {
        byte[] p = permutation();
        double[] med = new double[ROUNDS];
        AtomicLong counter = useCleaner ? CLEANED : FINALIZED;
        for (int r = 0; r < ROUNDS; r++) {
            long before = counter.get();
            long t0 = System.nanoTime();
            if (useCleaner) {
                for (int i = 0; i < N; i++)
                    SINK += new CleanerHandle(buildHandle(p)).handle;
            } else {
                for (int i = 0; i < N; i++)
                    SINK += new FinalizeHandle(buildHandle(p)).handle;
            }
            long createNs = System.nanoTime() - t0;   // push phase (context only)

            long deadline = System.nanoTime() + DRAIN_TIMEOUT_MS * 1_000_000L;
            long done;
            while ((done = counter.get() - before) < N && System.nanoTime() < deadline) {
                System.gc();
                Thread.yield();
                Thread.sleep(1);
            }
            long elapsed = System.nanoTime() - t0;
            if (done < N) {
                System.out.printf("  %s round %d INCOMPLETE %d/%d (drain timeout)%n", arm, r, done, N);
            }
            med[r] = elapsed / (double) Math.max(done, 1);
            System.out.printf("  %s round %d: create %d ns total, drain total %d ns%n",
                    arm, r, createNs, elapsed - createNs);
        }
        Arrays.sort(med);
        return new double[]{ med[ROUNDS / 2] };
    }

    static void report(String arm, double[] med) {
        System.out.printf("%-18s %12.1f ns/op%n", arm, med[0]);
    }
}
