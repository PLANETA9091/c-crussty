package net.minecraft.world.level.levelgen.synth;

/** Bench-local declarations of the REAL native bridge exports
 *  (Java_net_minecraft_world_level_levelgen_synth_PaperNativeImprovedNoise_*
 *  live in native/libpaper_native_jni.so — System.load'd by the driver). */
final class PaperNativeImprovedNoise {
    private PaperNativeImprovedNoise() {}
    static native long nativeBuildHandle(byte[] p, double xo, double yo, double zo);
    static native void nativeFreeHandle(long handle);
    static native double nativeNoise(long handle, double x, double y, double z, double yScale, double yMax);
}
