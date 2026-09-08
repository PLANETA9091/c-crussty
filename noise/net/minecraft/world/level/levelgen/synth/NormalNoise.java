// Compile-time stub for the TASK-108 bridge. NOT shipped: the real class is
// bootstrap-defined by the manifest-driven native surface (src/jni_table.rs
// rows ~282-291) on every boot. Signatures in lock-step with the JNI exports
// manifest and the G-ABI-2 decode (bench/step0_noise Step0NormalFillBench).
package net.minecraft.world.level.levelgen.synth;

public final class NormalNoise {
    private NormalNoise() {}
    public static native NormalNoise create(
        net.minecraft.util.RandomSource random,
        int firstOctave,
        double... amplitudes);
}
