package net.minecraft.world.level.levelgen.synth;

import java.util.Collections;
import java.util.Map;
import java.util.WeakHashMap;

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
 *   3. on GC of the instance the handle is freed (Finalizer; WeakHashMap key).
 *
 * The rewritten body runs INSIDE ImprovedNoise, so it reads the private
 * `p/xo/yo/zo` fields of `this` itself and passes them here as arguments —
 * no field access from this class, and no field-modifier changes in the
 * retransformed class file (the JVM rejects those with
 * JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED). Compiled against
 * compile-time stubs in RuntimeStubs.java; the real kernels ship the real
 * shapes + the crussty bridge PaperNativeImprovedNoise (bootstrap).
 */
final class ImprovedNoiseNativeOpsOld {
    // ---- observability counters (identical in NEW impl; mechanism untouched) ----
    static final java.util.concurrent.atomic.AtomicLong FREED =
            new java.util.concurrent.atomic.AtomicLong();
    static int liveHandles() { return HANDLES.size(); }
    static long freedHandles() { return FREED.get(); }
    static void expungeNow() { HANDLES.size(); } // real map access = expunge (old's only reclaim trigger)

    /** Owns one native handle; frees it once the owning noise dies. */
    private static final class Handle {
        final long nativeHandle;
        Handle(long nativeHandle) {
            this.nativeHandle = nativeHandle;
        }
        @Override
        protected void finalize() {
            if (this.nativeHandle != 0L) {
                PaperNativeImprovedNoise.nativeFreeHandle(this.nativeHandle);
                FREED.incrementAndGet();
            }
        }
    }

    /** Per-instance handles. Key is the kernel ImprovedNoise itself. */
    private static final Map<ImprovedNoise, Handle> HANDLES =
            Collections.synchronizedMap(new WeakHashMap<>());

    private ImprovedNoiseNativeOpsOld() {}

    private static long handle(ImprovedNoise self, byte[] p, double xo, double yo, double zo) {
        Handle h = HANDLES.get(self);
        if (h == null) {
            long raw = PaperNativeImprovedNoise.nativeBuildHandle(p, xo, yo, zo);
            if (raw == 0L) { // native rejected the permutation (never, kernel uses 256)
                return 0L;
            }
            h = new Handle(raw);
            HANDLES.put(self, h);
        }
        return h.nativeHandle;
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