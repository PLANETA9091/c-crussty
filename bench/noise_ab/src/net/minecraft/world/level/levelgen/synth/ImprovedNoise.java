package net.minecraft.world.level.levelgen.synth;

/**
 * Runtime shape stub for the noise A/B bench (NOT shipped, bench-only).
 * Identical role to the compile-time stub in noise/RuntimeStubs.java but kept
 * separate so the bench build never depends on files the build script
 * deletes. The bridge reads no fields of this type — identity + type are all
 * that matter here (identity hash drives the new bridge's direct-mapped
 * cache; identity equality drives the legacy WeakHashMap key).
 */
final class ImprovedNoise {
    ImprovedNoise() {}
}
