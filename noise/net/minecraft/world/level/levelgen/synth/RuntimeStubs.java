package net.minecraft.world.level.levelgen.synth;

/**
 * Compile-time stubs so ImprovedNoiseNativeOps.java and
 * PerlinNoiseNativeOps.java can be compiled standalone (--release 8) against
 * the real kernel class shapes. NOT shipped: the real classes come from the
 * kernel jar / the crussty bridge injection. Package-private on purpose
 * (javac: one public top-level class per file).
 *
 * Shapes mirror the 1.21.10 kernel classes plus the bridge natives we
 * register at runtime (see src/jni_table.rs — full-name PaperNative*).
 * Signatures must stay in lock-step with the JNI exports manifest.
 *
 * No PRIVATE field declarations are needed: the rewritten bodies read the
 * kernel's own privates from inside the kernel classes, and
 * PerlinNoiseNativeOps extracts them reflectively at build time. PUBLIC
 * shapes that the bridges link against ARE declared (ImprovedNoise.xo/yo/zo
 * field refs, ImprovedNoise.noise, PerlinNoise.wrap) — a missing public
 * member here would surface as a runtime NoSuchField/NoSuchMethodError in
 * the kernel loader, so the stubs must stay shape-accurate.
 */
final class PaperNativeImprovedNoise {
    private PaperNativeImprovedNoise() {}
    static native long nativeBuildHandle(byte[] p, double xo, double yo, double zo);
    static native void nativeFreeHandle(long handle);
    static native double nativeNoise(long handle, double x, double y, double z, double yScale, double yMax);
    static native double nativeNoiseNoYScale(long handle, double x, double y, double z);
    static native int nativeFill(long handle, double[] xs, double[] ys, double[] zs, double[] yScales, double[] yMaxs, double[] out);
    static native int nativeFillNoYScale(long handle, double[] xs, double[] ys, double[] zs, double[] out);
}

// ImprovedNoise / PerlinNoise / PaperNativePerlinNoise moved to PUBLIC
// own-file stubs (same package) — NormalNoiseBatchOps (levelgen package)
// links them cross-package, which package-private stubs cannot serve.
