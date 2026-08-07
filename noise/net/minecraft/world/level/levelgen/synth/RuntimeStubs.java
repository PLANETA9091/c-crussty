package net.minecraft.world.level.levelgen.synth;

/**
 * Compile-time stubs so ImprovedNoiseNativeOps.java can be compiled
 * standalone (--release 8) against the real kernel class shapes. NOT shipped:
 * the real classes come from the kernel jar / the crussty bridge injection.
 * Package-private on purpose (javac: one public top-level class per file).
 *
 * Shapes mirror the 1.21.10 kernel ImprovedNoise plus the bridge natives we
 * register at runtime (see modules/crussty/src/jni_table.rs — full-name
 * PaperNativeImprovedNoise). Signatures must stay in lock-step with the JNI
 * exports in paper-native-jni/src/lib.rs.
 *
 * No field declarations are needed: the bridge never reads kernel fields —
 * the rewritten noise() body (inside ImprovedNoise) reads its own private
 * `p/xo/yo/zo` and passes them as arguments (field-modifier changes are
 * illegal in a retransformed class file).
 */
final class ImprovedNoise {
}

final class PaperNativeImprovedNoise {
    private PaperNativeImprovedNoise() {}
    static native long nativeBuildHandle(byte[] p, double xo, double yo, double zo);
    static native void nativeFreeHandle(long handle);
    static native double nativeNoise(long handle, double x, double y, double z, double yScale, double yMax);
    static native double nativeNoiseNoYScale(long handle, double x, double y, double z);
    static native int nativeFill(long handle, double[] xs, double[] ys, double[] zs, double[] yScales, double[] yMaxs, double[] out);
    static native int nativeFillNoYScale(long handle, double[] xs, double[] ys, double[] zs, double[] out);
}