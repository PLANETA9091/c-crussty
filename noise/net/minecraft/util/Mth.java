// Compile-time stub for the TASK-108 v3 interpreter. NOT shipped
// (scripts/build_noise.sh drops it; the real Mth resolves in the kernel
// loader — the interpreter invokes the kernel's own clampedMap, so the
// arithmetic is bit-exact by construction).
package net.minecraft.util;

public final class Mth {
    private Mth() {
    }

    public static double clampedMap(double x, double fromLow, double fromHigh, double toLow, double toHigh) {
        return 0.0D;
    }
}
