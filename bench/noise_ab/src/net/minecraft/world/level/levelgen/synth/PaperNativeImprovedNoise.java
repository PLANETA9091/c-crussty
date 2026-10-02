package net.minecraft.world.level.levelgen.synth;

/**
 * Runtime shape stub for the noise A/B bench: declarations of the closed
 * Crussty CE natives (same shapes as the compile-time stub in
 * noise/RuntimeStubs.java). No library loading here — the bench main()
 * System.load()s the .so BEFORE any call, and JVM native binding is lazy per
 * first call, which keeps this class compilable without any absolute paths.
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
