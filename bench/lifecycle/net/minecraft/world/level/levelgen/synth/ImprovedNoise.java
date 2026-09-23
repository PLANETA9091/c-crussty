package net.minecraft.world.level.levelgen.synth;

/** Bench-local stand-in for the kernel ImprovedNoise (same FQCN as the
 *  compile-time stub; the map key only needs identity semantics). */
final class ImprovedNoise {
    final byte[] p = new byte[256];
    final double xo, yo, zo;
    ImprovedNoise(int seed) {
        for (int i = 0; i < p.length; i++) p[i] = (byte) (seed + i);
        xo = seed * 0.031; yo = seed * 0.017; zo = seed * 0.043;
    }
}
