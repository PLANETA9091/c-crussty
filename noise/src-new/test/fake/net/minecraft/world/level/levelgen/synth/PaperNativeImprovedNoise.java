package net.minecraft.world.level.levelgen.synth;

import java.util.Collections;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;

/**
 * Smoke-test double of the crussty bridge natives: same FQCN + signatures as
 * the real PaperNativeImprovedNoise (see RuntimeStubs.java / jni_table.rs),
 * but pure-Java so the REAL ImprovedNoiseNativeOps lifecycle logic can be
 * exercised on a machine without the closed .so. Lives ONLY in
 * noise/src-new/test — never shipped, never embedded.
 *
 * Accounting:
 *   BUILT   total nativeBuildHandle calls,
 *   FREED_CALLS total nativeFreeHandle calls,
 *   FREED_UNIQUE distinct handles ever freed — FREED_CALLS must always equal
 *   FREED_UNIQUE.size() (any excess = a double-free in the bridge).
 */
public final class PaperNativeImprovedNoise {
    public static final AtomicLong BUILT = new AtomicLong();
    public static final AtomicLong FREED_CALLS = new AtomicLong();
    public static final Set<Long> FREED_UNIQUE =
            Collections.newSetFromMap(new ConcurrentHashMap<>());
    private static final AtomicLong SEQ = new AtomicLong();

    private PaperNativeImprovedNoise() {}

    public static long nativeBuildHandle(byte[] p, double xo, double yo, double zo) {
        if (p == null || p.length != 256) return 0L; // mirror native rejection
        BUILT.incrementAndGet();
        return SEQ.incrementAndGet();
    }

    /** Current fake-handle sequence value (smoke-test bookkeeping only). */
    public static long lastSeq() {
        return SEQ.get();
    }

    public static void nativeFreeHandle(long handle) {
        if (handle != 0L) {
            FREED_CALLS.incrementAndGet();
            FREED_UNIQUE.add(handle);
        }
    }

    public static double nativeNoise(long handle, double x, double y, double z, double yScale, double yMax) {
        return handle + x + y + z + yScale + yMax;
    }

    public static double nativeNoiseNoYScale(long handle, double x, double y, double z) {
        return handle + x + y + z;
    }

    public static int nativeFill(long handle, double[] xs, double[] ys, double[] zs, double[] yScales, double[] yMaxs, double[] out) {
        return 0;
    }

    public static int nativeFillNoYScale(long handle, double[] xs, double[] ys, double[] zs, double[] out) {
        return 0;
    }
}
