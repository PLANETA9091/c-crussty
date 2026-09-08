package net.minecraft.world.level.levelgen.synth;

/**
 * Compile-time declaration of the heritage closed-lib natives for the
 * NormalNoise ABI probe (TASK-79). The real shapes live in
 * {@code libpaper_native_jni.so} (JNI_EXPORTS.manifest rows 282-283); the
 * probe binds them by symbol name after System.load(absolute path) —
 * same mechanism as PaperNativePerlinNoise in the G-ABI rig (TASK-70).
 *
 * Not shipped: probe-only class, compiled into classes_normal/ and never
 * packaged into the module.
 */
public final class PaperNativeNormalNoise {
    private PaperNativeNormalNoise() {}

    public static native boolean nativeCheck();

    /** Heritage whole-Noise composition kernel: two PerlinNoise handles
     *  (TASK-70 ABI) + x, y, z + a fourth double (semantics probed:
     *  hypothesized = NormalNoise.valueFactor). */
    public static native double nativeGetValue(
        long firstHandle, long secondHandle,
        double x, double y, double z, double fourth);
}
