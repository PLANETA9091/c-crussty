// Compile-time stub for the ImprovedNoiseNativeOps/PerlinNoiseNativeOps/
// NormalNoiseBatchOps bridges. PUBLIC (NormalNoiseBatchOps in the levelgen
// package links the type cross-package). NOT shipped.
//
// ⚠ CONSTANT-FOLDING HAZARD (smoke-4..6 evidence, 2026-09-09): these fields
// MUST NOT be `final ... = <constant>` — javac constant-folds reads of final
// fields with constant initializers, so `lv[i].xo` in the bridge compiles to
// a literal 0.0 instead of a GETFIELD, silently building native handles with
// zero offsets (parity garbage; the probe rig never caught it because its own
// stub declares non-final fields). Non-final, no initializer = real GETFIELD.
package net.minecraft.world.level.levelgen.synth;

public final class ImprovedNoise {
    private ImprovedNoise() {}
    public double xo;
    public double yo;
    public double zo;
    public double noise(double x, double y, double z, double yScale, double yMax) {
        return 0.0D;
    }
}
