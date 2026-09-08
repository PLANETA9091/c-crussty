// Compile-time stub for the PerlinNoiseNativeOps/NormalNoiseBatchOps
// bridges (TASK-70 winner pack ABI + G-NORMAL composition). PUBLIC
// (cross-package link from NormalNoiseBatchOps). NOT shipped: the real
// class is bootstrap-defined by the manifest-driven native surface on
// every boot (src/jni_table.rs).
package net.minecraft.world.level.levelgen.synth;

public final class PaperNativePerlinNoise {
    private PaperNativePerlinNoise() {}
    public static native long nativeBuildHandle(byte[] a0, byte[] a1, double[] a2, double[] a3, double[] a4, double[] a5, double a6, double a7);
    public static native void nativeFreeHandle(long handle);
    public static native double nativeGetValue(long handle, double x, double y, double z, double y0, double y1, boolean flag);
    public static native double nativeGetValueNoYScale(long handle, double x, double y, double z);
}
