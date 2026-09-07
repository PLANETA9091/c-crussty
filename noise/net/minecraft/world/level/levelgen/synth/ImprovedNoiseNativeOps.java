package net.minecraft.world.level.levelgen.synth;

import java.lang.ref.PhantomReference;
import java.lang.ref.ReferenceQueue;
import java.util.Map;
import java.util.WeakHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * Native sample path for the kernel's {@code ImprovedNoise.noise(DDDDD)}.
 *
 * The byte hook (src/improved_noise.rs) rewires that method body to this
 * static bridge:
 *
 *   1. buildHandle once per ImprovedNoise instance (native copy of the
 *      permutation + xo/yo/zo — see paper-native-core::improved_noise),
 *   2. sample via the native handle; the native noise() applies the xo/yo/zo
 *      offsets internally, so we pass the RAW (fell-on) x/y/z,
 *   3. on GC of the instance the handle is freed PROMPTLY by a dedicated
 *      reaper thread (phantom-reference notification — no finalizer queue).
 *
 * Lifecycle design (TASK-01/09, replaces finalize() + one global map):
 *
 *   OLD: one synchronized WeakHashMap + Handle.finalize().
 *        - every noise() sample took the ONE global map lock — parallel
 *          world-gen threads serialized on a 20ns lock that sits inside a
 *          ~115ns JNI transition;
 *        - native memory stayed pinned until (a) a map access expunged the
 *          stale weak entry AND (b) the finalizer thread got around to the
 *          Handle — a quiet period after a gen burst pinned every handle;
 *        - finalizer-queue backlog under churn is a known GC brake.
 *
 *   NEW: STRIPES independent WeakHashMaps keyed by identityHashCode stripe
 *        (contention /STRIPES; vanilla ImprovedNoise uses identity equals,
 *        so per-stripe WeakHashMap semantics are unchanged), and the Handle
 *        itself is a PhantomReference to its owner: when the owner becomes
 *        phantom-reachable, the reaper daemon drains the queue and frees
 *        native memory on ITS OWN thread — never on the finalizer thread,
 *        and never gated on map access. A 2s quiet tick expunges stale
 *        entries so the Java-side map does not grow on idle servers.
 *
 *        Safety of free-vs-sample: nativeFreeHandle can only run once the
 *        owner is phantom-reachable, and any in-flight noise() call holds
 *        the owner strongly (its `self` parameter), so a handle can never
 *        be freed under a concurrent sample of the same instance.
 *
 * The rewritten body runs INSIDE ImprovedNoise, so it reads the private
 * `p/xo/yo/zo` fields of `this` itself and passes them here as arguments —
 * no field access from this class, and no field-modifier changes in the
 * retransformed class file (the JVM rejects those with
 * JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED). Compiled against
 * compile-time stubs in RuntimeStubs.java; the real kernels ship the real
 * shapes + the crussty bridge PaperNativeImprovedNoise (bootstrap).
 */
public final class ImprovedNoiseNativeOps {

    /** Stripe count (power of two). 16 = gen-pool sized; lock cost /16. */
    static final int STRIPES = 16;
    /** Quiet-period tick: expunge stale weak entries this often (ms). */
    static final long EXPUNGE_INTERVAL_MS = 2000;

    /** Reaper-freed counter — bench/self-test observability. */
    private static final AtomicLong FREED = new AtomicLong();

    /** Owns one native handle; ALSO the death notification for its owner.
     *  The freed flag makes the native free AT-MOST-ONCE across BOTH paths
     *  that can free: the reaper (owner became phantom-reachable) and an
     *  explicit releaseHandle() call (adopted from the sibling-agent's
     *  Cleaner variant) — without it, release-then-death would double-free
     *  the same native handle (UB in the closed .so). */
    private static final class Handle extends PhantomReference<ImprovedNoise> {
        final long nativeHandle;
        final AtomicBoolean freed = new AtomicBoolean(false);
        Handle(ImprovedNoise owner, long nativeHandle) {
            super(owner, QUEUE);
            this.nativeHandle = nativeHandle;
        }
        /** CAS-guarded native free; true if THIS caller won the free. */
        boolean release() {
            if (freed.compareAndSet(false, true)) {
                if (nativeHandle != 0L) {
                    PaperNativeImprovedNoise.nativeFreeHandle(nativeHandle);
                    FREED.incrementAndGet();
                }
                return true;
            }
            return false;
        }
    }

    /**
     * Dedicated reaper: frees native handles outside the finalizer thread.
     * Drains in burst mode (poll loop) once a notification arrives, and on
     * quiet timeouts expunges stale weak entries from all stripes.
     */
    private static final class Reaper implements Runnable {
        /** Explicit package-private ctor: an implicit one would inherit the
         *  class's PRIVATE access and force javac (--release 8, pre-nestmates)
         *  to mint a synthetic $1 marker class for the outer call. */
        Reaper() {}
        @Override public void run() {
            for (;;) {
                final Handle h;
                try {
                    h = (Handle) QUEUE.remove(EXPUNGE_INTERVAL_MS);
                } catch (InterruptedException e) {
                    continue; // daemon; never terminate
                }
                if (h != null) {
                    free(h);
                    Handle n;
                    while ((n = (Handle) QUEUE.poll()) != null) { // burst drain
                        free(n);
                    }
                } else {
                    expunge();
                }
            }
        }

        static void free(Handle h) {
            h.release(); // CAS-guarded: at-most-once even vs explicit release
        }
    }

    /** Death-notification queue shared by all handles. */
    static final ReferenceQueue<ImprovedNoise> QUEUE = new ReferenceQueue<>();

    /** Per-instance handles, striped by owner identity to split the lock.
     *  Key is the kernel ImprovedNoise itself (weak — vanilla type uses
     *  identity equals, so WeakHashMap semantics per stripe are identical
     *  to the old single map). */
    @SuppressWarnings("unchecked")
    static final Map<ImprovedNoise, Handle>[] STRIPE_MAPS = newMap();

    @SuppressWarnings("unchecked")
    private static Map<ImprovedNoise, Handle>[] newMap() {
        final Map<ImprovedNoise, Handle>[] a = new Map[STRIPES];
        for (int i = 0; i < STRIPES; i++) {
            a[i] = new WeakHashMap<>(64);
        }
        return a;
    }

    static {
        final Thread t = new Thread(new Reaper(), "crussty-noise-handle-reaper");
        t.setDaemon(true);
        t.start();
    }

    private ImprovedNoiseNativeOps() {}

    static Map<ImprovedNoise, Handle> stripe(ImprovedNoise self) {
        return STRIPE_MAPS[System.identityHashCode(self) & (STRIPES - 1)];
    }

    /** WeakHashMap expunges stale entries on access; a size() probe per
     *  stripe on quiet ticks keeps the Java side tight on idle servers. */
    static void expunge() {
        for (int i = 0; i < STRIPES; i++) {
            final Map<ImprovedNoise, Handle> s = STRIPE_MAPS[i];
            synchronized (s) { s.size(); }
        }
    }

    private static long handle(ImprovedNoise self, byte[] p, double xo, double yo, double zo) {
        final Map<ImprovedNoise, Handle> stripe = stripe(self);
        synchronized (stripe) {
            final Handle h = stripe.get(self);
            if (h != null) {
                return h.nativeHandle;
            }
            final long raw = PaperNativeImprovedNoise.nativeBuildHandle(p, xo, yo, zo);
            if (raw == 0L) { // native rejected the permutation (never, kernel uses 256)
                return 0L;
            }
            stripe.put(self, new Handle(self, raw));
            return raw;
        }
    }

    /**
     * Deterministic release of the handle registered for {@code self}
     * (adopted from the sibling-agent's Cleaner variant): idempotent,
     * race-free against the reaper (shared CAS freed-flag), no-op safe.
     * After this call, sampling {@code self} transparently rebuilds a
     * fresh handle on the next noise() call.
     */
    public static void releaseHandle(ImprovedNoise self) {
        final Map<ImprovedNoise, Handle> stripe = stripe(self);
        Handle h;
        synchronized (stripe) { h = stripe.remove(self); }
        if (h != null) {
            h.release();
        }
    }

    /**
     * bridge.noise(ImprovedNoise, p, xo, yo, zo, x, y, z, yScale, yMax) ->
     * double. p/xo/yo/zo are read from `this` by the rewritten kernel body;
     * x/y/z/yScale/yMax are its local slots (1/3/5/7/9) — raw coordinates;
     * native applies the offset.
     */
    public static double noise(
        ImprovedNoise self, byte[] p, double xo, double yo, double zo,
        double x, double y, double z, double yScale, double yMax
    ) {
        long h = handle(self, p, xo, yo, zo);
        return PaperNativeImprovedNoise.nativeNoise(h, x, y, z, yScale, yMax);
    }

    // ---------- observability (bench / self-test; not on the hot path) ----------

    /** Handles currently registered (stale entries until next expunge). */
    public static int liveHandles() {
        int n = 0;
        for (int i = 0; i < STRIPES; i++) {
            final Map<ImprovedNoise, Handle> s = STRIPE_MAPS[i];
            synchronized (s) { n += s.size(); }
        }
        return n;
    }

    /** Handles freed by the reaper so far (monotonic). */
    public static long freedHandles() {
        return FREED.get();
    }

    /** Force an expunge tick now (bench/self-test; also idempotent). */
    public static void expungeNow() {
        expunge();
    }
}
