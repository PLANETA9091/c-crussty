package net.minecraft.world.level.levelgen.synth;

import java.lang.ref.Cleaner;
import java.util.Collections;
import java.util.Map;
import java.util.WeakHashMap;
import java.util.concurrent.atomic.AtomicBoolean;

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
 *   3. on GC of the instance the handle is freed by a {@link Cleaner}
 *      (replaces the previous Finalizer-based Handle.finalize(); finalizers
 *      resurrect through the Finalizer queue and stall under minor-GC churn),
 *      plus an explicit {@link #releaseHandle(ImprovedNoise)} for callers
 *      that want deterministic release.
 *
 * Lifecycle safety:
 *   - the native free symbol
 *     (Java_net_minecraft_world_level_levelgen_synth_PaperNativeImprovedNoise_nativeFreeHandle,
 *     see src/jni_table.rs) is invoked AT MOST ONCE per handle: the
 *     Cleaner's clean() contract is at-most-once AND the cleaning action
 *     holds an AtomicBoolean guard, so explicit release, GC-driven release
 *     and a race between both can never double-free;
 *   - the cleaning action is a lambda capturing ONLY local constructor
 *     state (the raw long handle + its own fresh AtomicBoolean) — NEVER
 *     `this`. A `this` capture would chain Handle -> action -> Cleaner's
 *     internal registry, keeping every Handle strongly reachable forever
 *     (leak: no cleanup, ever). javac therefore emits the synthetic lambda
 *     method as STATIC — keep it that way (verify with javap -p);
 *   - like the old Finalizer design, a handle for an already-dead instance
 *     is only expunged from HANDLES by map activity (WeakHashMap expunges
 *     cleared entries on get/put, i.e. the next noise() call), after which
 *     the Cleaner thread frees it.
 *
 * EMBED CONTRACT (do not break): src/improved_noise.rs include_bytes!s and
 * defines EXACTLY TWO classes — this one and ImprovedNoiseNativeOps$Handle.
 * There must be NO additional class files (nested/static helper classes or
 * anonymous classes would ship a $Releaser/$1 file the runtime never
 * defines => NoClassDefFoundError on first noise()). The lambda is the
 * zero-extra-classfile form: LambdaMetafactory spins its implementation at
 * runtime inside the kernel loader (hidden classes bypass the byte hook).
 * scripts/build_noise.sh enforces the 2-file ship set.
 *
 * The rewritten body runs INSIDE ImprovedNoise, so it reads the private
 * `p/xo/yo/zo` fields of `this` itself and passes them here as arguments —
 * no field access from this class, and no field-modifier changes in the
 * retransformed class file (the JVM rejects those with
 * JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED). Compiled against
 * compile-time stubs in RuntimeStubs.java; the real kernels ship the real
 * shapes + the crussty bridge PaperNativeImprovedNoise (bootstrap).
 *
 * Class-file target: --release 11 (major 55). Cleaner is a Java 9+ API, so
 * --release 8 is no longer possible without runtime reflection. This is safe
 * because the ONLY consumer is the kernel JVM, which the runtime guard in
 * src/improved_noise.rs checks: bridge major (55) must be <= JVM major
 * (java.class.version, 65 on the required Java 21 of MC 1.21.x) — otherwise
 * the hook stays dormant instead of crashing. See scripts/build_noise.sh.
 */
public final class ImprovedNoiseNativeOps {
    /**
     * One shared Cleaner with a named low-priority daemon thread. Daemon:
     * pending cleanups at JVM exit are irrelevant (the OS reclaims the whole
     * process' native memory; Finalizer never guaranteed shutdown cleanup
     * either).
     */
    private static final Cleaner CLEANER = Cleaner.create(r -> {
        Thread t = new Thread(r, "crussty-improved-noise-cleaner");
        t.setDaemon(true);
        t.setPriority(Thread.NORM_PRIORITY - 1);
        return t;
    });

    /** Owns one native handle; frees it once the owning noise dies. */
    private static final class Handle {
        final long nativeHandle;
        final Cleaner.Cleanable cleanable;

        Handle(long nativeHandle) {
            this.nativeHandle = nativeHandle;
            // Captures ONLY locals (raw handle + its own freed flag), never
            // `this` — see the embed/safety notes above.
            AtomicBoolean freed = new AtomicBoolean(false);
            this.cleanable = CLEANER.register(this, () -> {
                if (nativeHandle != 0L && freed.compareAndSet(false, true)) {
                    PaperNativeImprovedNoise.nativeFreeHandle(nativeHandle);
                }
            });
        }

        /** Deterministic release; idempotent, race-free against the GC path. */
        void release() {
            // clean() unregisters and runs the action at most once; the
            // AtomicBoolean captured by the action is the explicit
            // double-free guard on the native call itself.
            cleanable.clean();
        }
    }

    /** Per-instance handles. Key is the kernel ImprovedNoise itself. */
    private static final Map<ImprovedNoise, Handle> HANDLES =
            Collections.synchronizedMap(new WeakHashMap<>());

    private ImprovedNoiseNativeOps() {}

    private static long handle(ImprovedNoise self, byte[] p, double xo, double yo, double zo) {
        // WeakHashMap.get expunges entries whose key was cleared by GC, so
        // this hot-path read also drives the Cleaner toward dead handles.
        Handle h = HANDLES.get(self);
        if (h == null) {
            // Once-per-instance path: build + register under the map lock so
            // two racing threads cannot build two native handles for one
            // instance (previously the loser of that race survived only via
            // finalize(); now it is prevented outright).
            synchronized (HANDLES) {
                h = HANDLES.get(self);
                if (h == null) {
                    long raw = PaperNativeImprovedNoise.nativeBuildHandle(p, xo, yo, zo);
                    if (raw == 0L) { // native rejected the permutation (never, kernel uses 256)
                        return 0L;
                    }
                    h = new Handle(raw);
                    HANDLES.put(self, h);
                }
            }
        }
        return h.nativeHandle;
    }

    /**
     * Explicit lifecycle hook: deterministically releases the native handle
     * registered for {@code self} (the action calls the existing native free
     * symbol PaperNativeImprovedNoise.nativeFreeHandle). No-op safe:
     *   - no handle registered (or already removed) -> does nothing;
     *   - handle 0L or already freed (by the Cleaner) -> action skips the
     *     native call (AtomicBoolean guard);
     *   - safe to call concurrently with noise() on the same instance.
     * After this call, sampling {@code self} transparently rebuilds a fresh
     * handle on the next noise() call.
     */
    public static void releaseHandle(ImprovedNoise self) {
        Handle h;
        synchronized (HANDLES) {
            h = HANDLES.remove(self);
        }
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
}
