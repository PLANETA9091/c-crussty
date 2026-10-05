//! chunk-factory -- the Native Chunk Factory (CRUSSTY mandate, Phase 0 skeleton).
//!
//! # Mandate
//!
//! Vanilla-identical Minecraft worldgen implemented in Rust, targeting a
//! **>=100x pregeneration speedup** vs vanilla Paper 1.21.10 at **100%
//! semantic compatibility**. A CHUNK is a pure function:
//!
//! ```text
//! f(seed, datapack_hash, DataVersion, x, z) -> NBT
//! ```
//!
//! Same inputs, byte-for-byte-semantic same chunk -- no hidden state, no
//! ambient JVM behavior, no ordering dependence. Every input that can change
//! generation (world seed, datapack content, target DataVersion) is part of
//! the key; every cache this crate ever builds keys on
//! `(seed, world spec hash)` (invariant I5; no unkeyed global state).
//!
//! # How: all Rust, no GPU, no approximate math
//!
//! The implementation must reproduce Java's arithmetic BIT-EXACTLY:
//!
//! * wrapping `i32`/`i64` semantics (Java int/long overflow rules),
//! * exact floor/cast behavior (Java `(int)` truncates toward zero;
//!   `Math.floor` / `floorDiv` / `floorMod` semantics reproduced precisely),
//! * Java's two random lineages: the Legacy `Random` (48-bit LCG) and
//!   Xoroshiro128++ (1.21 worldgen), including their seeding derivations,
//! * positional factories seeded from MD5 digests (structure placement),
//! * BANNED: FMA contraction, `f32` substitutions, lookup-table shortcuts,
//!   reordered float math -- anything that shifts one ULP shifts blocks.
//!
//! No GPU. No approximate noise. No fewer octaves. No skipping of
//! neighbor-chunk influence and no conservative-bounds culling that vanilla
//! does not also perform (invariant I4): work is culled only along PROVEN
//! conservative bounds (see K2).
//!
//! # Key accelerations (K1..K6, owner worklog)
//!
//! The unit of work is the CHUNK/region, never a single `noise()` call:
//!
//! * **K1** -- worldgen-graph compiler: datapack -> IR -> optimized plan
//!   (what vanilla WOULD run, extracted, not hand-reimplemented).
//! * **K2** -- exact interval culling on density-node bounds: skip cells
//!   with a proven sign; exact values on corners shared with sign-mixed
//!   cells.
//! * **K3** -- cross-chunk deduplication: carver path caches per source
//!   chunk, 2D fields, aquifer cells, structure scans computed once and
//!   re-rasterized into neighbor targets.
//! * **K4** -- section-sparse chunk model: light only in active sections
//!   (step occlusion and slabs/plats accounted).
//! * **K5** -- features/structures ported by TIERS, each tier gated before
//!   enable, everything unported falls back to Java (I8).
//! * **K6** -- bit-exact discipline end to end; the ground truth is the
//!   real JVM (golden vectors from actual Paper classes), never a
//!   re-implementation of it.
//!
//! # Delivery ladders
//!
//! * **Ladder A** -- offline `.mca` region writer: generate chunks out of
//!   process and hand vanilla a finished region file.
//! * **Ladder B** -- virtual region files via the Moonrise chunk-IO hook: a
//!   disk-missing chunk is materialized from Rust NBT with ONE JNI call per
//!   chunk (no per-stage crossings).
//! * **Ladder C** -- optional stage bridges (`noise+surface+carve ->`
//!   `sections`), finer-grained than B; only for intermediate measurements
//!   where the ladder B boundary is proven insufficient.
//!
//! Each ladder ships and is MEASURED separately (Amdahl accounting per
//! ladder lives in the owner worklog, Phase 0 P0.2).
//!
//! # Phases and the GATE
//!
//! * **Phase 0** -- profiling + golden harness; THIS crate skeleton (policy
//!   gate + coverage reporting). Writing generation code in this crate is
//!   BANNED in Phase 0.
//! * **Phase 1** -- IR extraction: datapack/preset -> declarative stage
//!   graph (what vanilla WOULD run, extracted, not reimplemented by hand).
//! * **Phase 2** -- bit-exact Tier1 kernels (noise/random/density) + GATE.
//! * **Phase 3** -- ladders A and B wired behind the gate, coverage %.
//! * **Phase 4** -- features, light, NBT (incl. palette bitpacking +
//!   compression).
//! * **Phase 5** -- structures (piece-based + jigsaw), spawn.
//!
//! **GATE (Phase 2 exit, and precondition for ANY native enable):**
//! `>=10^4 chunks x >=5 seeds x {vanilla, Terralith, Tectonic}` with **0
//! divergences**, AND the noise stage measured **>=30x hot Java per core**.
//! If either leg fails, native stays off. No exceptions, no partial credit.
//!
//! # Fallback law (invariant I8)
//!
//! Any chunk containing unsupported content (structures/features not yet
//! covered, blending with an old world, a foreign BlockPopulator, anything
//! outside the proven lane set) is generated ENTIRELY by Java -- never a
//! partial native/Java chunk. Native coverage percentage is ALWAYS
//! reported, never implied -- see [`coverage::CoverageReport`] and
//! [`gate::coverage_ratio`].
//!
//! # Gate and ledger (invariant I9)
//!
//! Nothing routes to Rust generation unless the enable gate says so:
//! [`gate::decide`] mirrors `src/kernel_policy.rs` (the do-not-wire
//! doctrine) but for CHUNK STATUS lanes. Default is dormant (`Off`): every
//! chunk stays on the Java path until lanes are individually proven and
//! registered in `gate::NATIVE_READY` -- which requires a passing row in
//! [`ledger::PROMOTED`] (the evidence ledger: zero-diff + speed GATE
//! numbers, one row per lane, per-feature gates in Phase 4). The env
//! switch is `CRUSSTY_CHUNK_FACTORY`
//! (`off`/unset/empty/unknown => Off; `strict`; `audit`); there is no
//! value that forces an unproven lane native.
//!
//! # Modules
//!
//! * [`gate`] -- the enable gate (registries, decision function, audit).
//! * [`coverage`] -- native-vs-Java fallback reporting types.
//! * [`ledger`] -- promotion evidence ledger (GATE arithmetic + registry).
//!
//! Future modules are deliberately ABSENT in Phase 0 (skeleton only); they
//! are stubbed as TODO comments below and must not be added before the
//! phase that needs them.
//!
//! This crate is standalone: excluded from the root `crussty` workspace,
//! zero external dependencies, std only.

pub mod climate;
pub mod coverage;
pub mod gate;
pub mod ledger;

// --- Phase 1 / Phase 2 modules (opened by owner directive 2026-10-05: //
// "do everything now; test on GH CI"). Phase 0's ban on generation code //
// is lifted; every module below is bound by the golden-vector gate: the //
// ground truth is the REAL JVM (bench/golden VectorCapture CSVs), and //
// bit-exact equality is the acceptance criterion, P1/P2 style.          //

/// Minimal JSON parser (zero-dep, I7) for worldgen datapack files.
pub mod interpolator;
pub mod json;
/// Java math helpers (Mth + JLS cast semantics), bit-exact.
pub mod mth;
/// MD5 (RandomSupport.seedFromHashOf).
pub mod md5;
/// LegacyRandomSource + RandomSupport + MarsagliaPolarGaussian (P2.1).
pub mod jrandom;
/// Xoroshiro128++ lineage + positional factories (P2.1).
pub mod xoroshiro;
/// ImprovedNoise / PerlinNoise / NormalNoise / BlendedNoise (P2.2).
pub mod noise;
/// Density-function IR + scalar reference evaluator + CubicSpline (P1.5/P2.3).
pub mod density;
/// Worldgen datapack loading, DF parsing, RandomState wiring, spec hash
/// (P1.1/P1.2/P1.7).
pub mod router;
/// Golden-vector CSV loading + Java hex-float parsing (P0.3/P1 gate).
pub mod vanilla_biomes;
pub mod vectors;
/// Ladder A: offline .mca region writer (P3.1).
pub mod region;

// TODO(phase-2+): surface rules, aquifers, carvers, heightmaps, biomes
// TODO(phase-3):  ladder B (Moonrise chunk-IO bridge), stage C bridges
// TODO(phase-4):  features, light engine, palette bitpacking, compression
// TODO(phase-5):  jigsaw, structure starts + references, spawn

#[cfg(test)]
mod tests {
    use crate::gate::{parse_mode, PolicyMode};

    /// Smoke test for the shipped default: with NO env value at all the
    /// gate mode parses to `Off` -- default-dormant, every chunk on the
    /// Java path. This is the out-of-the-box contract of the whole crate:
    /// it must be inert until an operator AND a GATE pass open a lane.
    #[test]
    fn gate_mode_parses_off_by_default() {
        assert_eq!(parse_mode(None), PolicyMode::Off); // env unset
        assert_eq!(parse_mode(Some("")), PolicyMode::Off); // env empty
        assert_eq!(parse_mode(Some("off")), PolicyMode::Off); // explicit off
    }

    #[test]
    fn unknown_env_value_never_widens_the_gate() {
        // Fail-safe: there is no env value that forces native generation.
        for garbage in ["1", "yes", "allow", "on", "enable", "garbage"] {
            assert_eq!(
                parse_mode(Some(garbage)),
                PolicyMode::Off,
                "unknown value {garbage} must not widen the gate"
            );
        }
    }
}
