package dev.crussty.golden;

// ============================================================================
// VectorHook — documented-NOT-implemented interface stub (NCF P0.3 vector half).
//
// Phase 2 plan (NOT implemented in v1): capture random draws / noise samples /
// density-function values out of the real Paper 1.21.10 classes for a fixed
// coordinate vector set, via java.lang.instrument retransform hooks — the same
// class-replacement machinery the CRUSSTY runtime uses, but as a SEPARATE
// capture module (or a second plugin) so the golden dumper stays pure.
//
// These vectors are the P1 gate: the scalar IR evaluator (P1.5) must reproduce
// them bit-in-bit before any Phase 2 optimization work starts (NCF worklog
// "ГЕЙТ P1: эталонный вычислитель совпадает с golden-векторами из P0.3
// бит-в-бит").
//
// v1 status: interface only. No bytecode manipulation happens in this plugin;
// loading it on an agent-armed server is already forbidden by the purity law
// in GoldenDumperPlugin's header.
// ============================================================================
public interface VectorHook {

    /*
     * Phase 2 sketch (to be implemented, not part of v1):
     *
     * - start(String outDir): retransform target classes
     *   (net.minecraft.world.level.levelgen.synth.{ImprovedNoise,PerlinNoise,NormalNoise},
     *   density function evaluators, Xoroshiro/Legacy RandomSource impls) with
     *   recording bytecodes; stream (site-id, inputs, outputs) tuples to
     *   <outDir>/vectors/*.jsonl keyed by the same (seed, world spec) as the
     *   chunk dumps.
     * - stop(): restore original class bytes.
     * - Determinism: captures only valid for the exact seed + DataVersion +
     *   datapack hash recorded in meta.properties (NCF invariant I5).
     *
     * Phase 2: capture random/noise/density vectors via retransform; not
     * implemented in v1.
     */
}
