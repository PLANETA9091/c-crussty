//! Promotion ledger -- the evidence registry behind the enable gate.
//!
//! Same doctrine as `crussty/src/kernel_policy.rs`: a lane does not go
//! native because someone believes it is fast or correct; it goes native
//! because THIS module carries the measurement. The relationship:
//!
//! * [`crate::gate`] is the DECISION function (`decide()` /
//!   `NATIVE_READY` / `DO_NOT_ENABLE`);
//! * this module is the EVIDENCE ledger those rows point back to: an
//!   entry in `gate::NATIVE_READY` is only legitimate if a matching
//!   [`PROMOTED`] row (same lane id, evidence in [`Evidence::reference`])
//!   exists and passes [`gate_passes`].
//!
//! Promotion procedure (mirrors the kernel_policy lifecycle):
//!
//! 1. **Phase 2 GATE** (terrain lanes): a zero-diff run over
//!    `>= GATE_MIN_CHUNKS` chunks x `>= GATE_MIN_SEEDS` seeds x the
//!    `GATE_CORPORA` set `{vanilla, Terralith, Tectonic}` with
//!    **0 divergences**, AND the noise stage measured
//!    `>= GATE_MIN_NOISE_SPEEDUP` (30x) hot Java per core. BOTH legs or
//!    no promotion -- a fast-but-wrong kernel never promotes, and neither
//!    does a correct-but-slow one.
//! 2. **Per-feature gates (Phase 4)**: every feature type gets its OWN
//!    zero-diff gate before it may be enabled; a proven noise lane proves
//!    nothing about `trees`, `ores`, or light. One [`Evidence`] row per
//!    feature lane, same arithmetic, corpora per the feature's phase plan.
//! 3. **Mechanics**: append the [`Evidence`] row to [`PROMOTED`] with a
//!    durable `reference` (harness id / report path), then add the
//!    `(lane, evidence_tag)` row to `gate::NATIVE_READY` pointing at it.
//!    Demotion is the reverse edit: remove the row and the lane falls
//!    back to Java at the next boot. The test suites iterate both
//!    registries, so an edit that breaks the invariants fails
//!    `cargo test` immediately (kernel_policy pattern).
//!
//! PHASE 0: [`PROMOTED`] is intentionally EMPTY. Nothing has been measured,
//! nothing is proven, and the Phase 0 ban forbids writing generation code
//! at all. Do not seed placeholder rows -- an evidence row IS a claim.

/// GATE leg 1: minimum chunks per zero-diff corpus run (10^4).
pub const GATE_MIN_CHUNKS: u64 = 10_000;

/// GATE leg 1: minimum seeds per zero-diff corpus run.
pub const GATE_MIN_SEEDS: u64 = 5;

/// GATE corpora that must all be covered by a promoting run: vanilla
/// datapack plus the two big datapack-stress worlds.
pub const GATE_CORPORA: &[&str] = &["vanilla", "Terralith", "Tectonic"];

/// GATE leg 2: minimum noise-stage speedup vs HOT (JIT-warmed) Java,
/// per core. Cold-vs-JVM comparisons do not count (invariant I10).
pub const GATE_MIN_NOISE_SPEEDUP: f64 = 30.0;

/// One promotion record: the measured proof that a lane passed its gate.
///
/// All fields are `Copy`/`'static`: a ledger row is immutable published
/// evidence, never runtime state. `divergences` must be 0 for a promotion
/// (a nonzero row documents a FAILED gate and must not appear in
/// [`PROMOTED`] -- failed runs live in the worklog/report, not here).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Evidence {
    /// The chunk-status lane this evidence promotes, e.g.
    /// `noise/initial_shape` -- identical to the `gate::NATIVE_READY` key.
    pub gate: &'static str,
    /// Chunks compared zero-diff in the promoting run.
    pub chunks_tested: u64,
    /// Distinct world seeds used in the promoting run.
    pub seeds: u64,
    /// Datapack corpora covered (must include every [`GATE_CORPORA`] entry).
    pub corpora: &'static [&'static str],
    /// Divergences observed. Promotion requires exactly 0.
    pub divergences: u64,
    /// Measured noise-stage speedup vs hot Java per core, if the run
    /// included the performance leg (`Some(x)` with `x >= 30.0` required).
    pub noise_speedup_vs_hot_java: Option<f64>,
    /// Durable pointer to the full report (harness id, date, path).
    pub reference: &'static str,
}

/// The promotion ledger: lanes with GATE evidence on file.
///
/// PHASE 0: EMPTY, and it must stay empty until the Phase 2 GATE actually
/// passes with published numbers. An empty ledger is what keeps
/// `gate::NATIVE_READY` legitimately empty; a row here without a matching
/// gate-passing measurement would be a fabricated claim.
pub static PROMOTED: &[Evidence] = &[];

/// Does this evidence row satisfy the promotion gate (both legs)?
///
/// Encodes the Phase 2 GATE arithmetic exactly:
/// `divergences == 0` AND `chunks_tested >= GATE_MIN_CHUNKS` AND
/// `seeds >= GATE_MIN_SEEDS` AND every [`GATE_CORPORA`] corpus covered AND
/// `noise_speedup_vs_hot_java >= GATE_MIN_NOISE_SPEEDUP`.
/// Pure function; used by the registry tests and by future promotion
/// tooling so the thresholds live in exactly one place.
pub fn gate_passes(e: &Evidence) -> bool {
    e.divergences == 0
        && e.chunks_tested >= GATE_MIN_CHUNKS
        && e.seeds >= GATE_MIN_SEEDS
        && GATE_CORPORA
            .iter()
            .all(|required| e.corpora.iter().any(|have| have == required))
        && matches!(e.noise_speedup_vs_hot_java, Some(x) if x >= GATE_MIN_NOISE_SPEEDUP)
}

/// Look a lane up in the ledger. Exact gate-id match, allocation-free
/// linear scan (same shape as `gate::native_ready_entry`).
pub fn promoted_entry(gate: &str) -> Option<&'static Evidence> {
    PROMOTED.iter().find(|e| e.gate == gate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promoted_is_empty_in_phase_0() {
        // Phase 0 ban: no generation code exists, so no measurement can
        // exist either. This test gets REMOVED/updated when the first real
        // evidence row lands (Phase 2 GATE pass with published numbers) --
        // not before. If you are about to delete it, the GATE numbers must
        // already be in the owner worklog LOG with a harness reference.
        assert!(
            PROMOTED.is_empty(),
            "Phase 0: PROMOTED must stay empty until a real GATE pass"
        );
    }

    #[test]
    fn promoted_rows_must_carry_passing_evidence() {
        // Vacuously true while PROMOTED is empty, but the loop stays: any
        // future row that does not satisfy the GATE fails cargo test
        // immediately (kernel_policy registry-test pattern).
        for e in PROMOTED {
            assert!(!e.gate.is_empty(), "ledger row has an empty gate id");
            assert!(!e.reference.is_empty(), "ledger row {} has no reference", e.gate);
            assert!(
                gate_passes(e),
                "ledger row {} does not satisfy the GATE it claims",
                e.gate
            );
            assert!(
                promoted_entry(e.gate).is_some(),
                "ledger row {} is not findable by its own gate id",
                e.gate
            );
        }
    }

    #[test]
    fn gate_passes_requires_both_legs() {
        let base = Evidence {
            gate: "test/lane",
            chunks_tested: GATE_MIN_CHUNKS,
            seeds: GATE_MIN_SEEDS,
            corpora: GATE_CORPORA,
            divergences: 0,
            noise_speedup_vs_hot_java: Some(GATE_MIN_NOISE_SPEEDUP),
            reference: "test harness",
        };
        assert!(gate_passes(&base), "exactly-at-threshold row must pass");

        // Leg 1 violations: any divergence, thin chunk count, thin seeds,
        // a missing corpus.
        let mut e = base;
        e.divergences = 1;
        assert!(!gate_passes(&e), "1 divergence must fail the gate");
        e = base;
        e.chunks_tested = GATE_MIN_CHUNKS - 1;
        assert!(!gate_passes(&e), "too few chunks must fail the gate");
        e = base;
        e.seeds = GATE_MIN_SEEDS - 1;
        assert!(!gate_passes(&e), "too few seeds must fail the gate");
        e = base;
        e.corpora = &["vanilla", "Terralith"];
        assert!(!gate_passes(&e), "missing Tectonic corpus must fail the gate");

        // Leg 2 violations: no performance measurement, or below 30x.
        e = base;
        e.noise_speedup_vs_hot_java = None;
        assert!(!gate_passes(&e), "missing speedup leg must fail the gate");
        e = base;
        e.noise_speedup_vs_hot_java = Some(GATE_MIN_NOISE_SPEEDUP - 0.5);
        assert!(!gate_passes(&e), "sub-threshold speedup must fail the gate");
    }
}
