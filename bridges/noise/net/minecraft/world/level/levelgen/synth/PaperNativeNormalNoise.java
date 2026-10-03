// Compile-time stub for the TASK-108 bridge: the heritage NormalNoise BATCH
// FILL family (G-ABI-2 decoded bit-exact, first hypothesis; see
// bench/step0_noise/results/GABI2_NORMAL_FILL_ABI_2026-09-09.md).
// NOT shipped: bootstrap-defined by the manifest-driven native surface.
package net.minecraft.world.level.levelgen.synth;

public final class PaperNativeNormalNoise {
    private PaperNativeNormalNoise() {}
    /** (h1, h2, valueFactor, x[], y[], z[], out) -> count written */
    public static native int nativeFillPositions(long h1, long h2, double vf, double[] x, double[] y, double[] z, double[] out);
    /** (h1, h2, valueFactor, bx[], by[], bz[], sxz, sy, out) -> count written;
     *  per element getValue(bx*sxz, by*sy, bz*sxz) = DensityFunctions$Noise shape */
    public static native int nativeFillScaledPositions(long h1, long h2, double vf, int[] bx, int[] by, int[] bz, double sxz, double sy, double[] out);
    /** (h1, h2, valueFactor, a1[], a2[], out) -> count written;
     *  per element 4*getValue(a1*0.25, 0, a2*0.25) = ShiftA shape */
    public static native int nativeFillShiftA(long h1, long h2, double vf, int[] a1, int[] a2, double[] out);
    /** (h1, h2, valueFactor, a1[], a2[], out) -> count written;
     *  per element 4*getValue(a2*0.25, a1*0.25, 0) = ShiftB shape (x/z swap baked) */
    public static native int nativeFillShiftB(long h1, long h2, double vf, int[] a1, int[] a2, double[] out);
    /** (h1, h2, x, y, z, valueFactor) -> (A(x,y,z)+B(x*IF,y*IF,z*IF))*valueFactor (G-NORMAL) */
    public static native double nativeGetValue(long h1, long h2, double x, double y, double z, double vf);
}
