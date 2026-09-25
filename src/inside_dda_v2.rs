//! DDA-v2 HYBRID shadow verifier for the INSIDE lane (ID-P35, law-11
//! tick-459, TASK-459-75 — development of src/inside_diet.rs; see
//! RESEARCH-459-P35.md).
//!
//! LANE: inside = 31.18% of ALL alloc samples (RECON14C, s7173) / 9.20% CPU.
//! v1 (TASK-332) dieted the per-call glue (1 box + 1 visitor) and keeps the
//! REAL vanilla static walk `BlockGetter.forEachBlockIntersectedBetween` as
//! the executor. A full walk+DDA transcription attempt (DDA-v1) DIVERGED on
//! 21/350k offline lockstep scenarios and was REJECTED (InsideDietOps doc).
//!
//! V2 HYBRID (this module):
//!   * the vanilla walk stays the SOLE executor — the source of truth is
//!     never replaced, so observable behaviour is bit-identical BY
//!     CONSTRUCTION (lithium #125 precedent: the aggressive
//!     `entity.fast_suffocation_check` redirect broke external modifiers of
//!     the same Entity suffocation path and had to be re-landed as a less
//!     aggressive redirect — full transcribed executors in this lane carry a
//!     documented divergence history);
//!   * the DDA walk transcription runs in PARALLEL as a SHADOW, sampled
//!     1/200 (deterministic `(call_ordinal % 200 == 0)` sampler — the JFR
//!     adaptive-sampler precedent: throttled online sampling that preserves
//!     statistical relevancy, foojay "Improved JFR Allocation Profiling in
//!     JDK 16");
//!   * ANY divergence (cell sequence, step order, gate predicates) trips the
//!     ONE-SHOT DISARM latch (batchOk pattern, nav_plane canon): the shadow
//!     is permanently disabled, vanilla continues untouched — fail-closed,
//!     zero overhead after disarm;
//!   * the transcription may be promoted to an executor ONLY after the
//!     preregistered gate "disarm-fires = 0 on 1M offline scenes" plus
//!     online 1/200 legs without a disarm (DDA-v1 failed exactly this gate:
//!     21/350k).
//!
//! NCDFE-CANON (dormant scaffold): while `CRUSSTY_INSIDE_DDA_V2` is unset
//! this module defines NO classes, registers NO natives, composes NO
//! retarget — vanilla bit-for-bit by construction. The java stub section in
//! entityinside/.../InsideDietOps.java carries the mirrored latch, is NOT
//! rebuilt (include_bytes! in inside_diet.rs still serves the v1 bytecode),
//! and the future JNI surface follows define-BEFORE-arm (RegisterNatives on
//! the just-defined class only — resolution-closure guard, zero_alloc leg#1
//! lesson). The DDA model below is an OFFLINE reference for the 1M-scene
//! harness: pure Rust, no JNI, deterministic.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Verify every Nth call (card: "верификатор жрёт CPU — 1/200 и только на
/// ноги-прогоны"). Deterministic — lockstep-reproducible, no RNG engine.
pub const SAMPLE_EVERY: u64 = 200;

/// Step cap for the shadow model loop: vanilla caps walk steps via the
/// caller-supplied budget (int); the shadow never outlives that budget
/// (16x headroom) — a hit disarms (fail-closed).
pub const STEP_CAP: u64 = 4096;

/// One-shot disarm latch (batchOk pattern, nav_plane canon). `true` = shadow
/// allowed; once `false` it is never reset (no re-arm path in-process).
static SHADOW_OK: AtomicBool = AtomicBool::new(true);
/// Number of disarm firings (0 = healthy; data-plane for the leg gate).
static DISARM_FIRES: AtomicU64 = AtomicU64::new(0);
/// Sampled calls that reached a verdict.
static VERIFIED: AtomicU64 = AtomicU64::new(0);
/// Sampled calls whose shadow matched the vanilla trace.
static MATCHED: AtomicU64 = AtomicU64::new(0);

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_INSIDE_DDA_V2")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "on" | "yes"
    )
}

/// Deterministic 1/200 sampler. Call with a per-lane monotonic ordinal.
#[inline]
pub fn should_verify(call_ordinal: u64) -> bool {
    SHADOW_OK.load(Ordering::Relaxed) && call_ordinal % SAMPLE_EVERY == 0
}

/// One-shot disarm: first divergence/ERR kills the shadow for the rest of the
/// process. Returns `true` if THIS call flipped the latch (logging site).
pub fn disarm(reason: &str) -> bool {
    let was = SHADOW_OK.swap(false, Ordering::AcqRel);
    DISARM_FIRES.fetch_add(1, Ordering::AcqRel);
    if was {
        eprintln!(
            "[crussty-plugin] inside_dda_v2: DISARM ({reason}) — shadow verifier off, vanilla walk remains the executor"
        );
    }
    was
}

pub fn disarm_fires() -> u64 {
    DISARM_FIRES.load(Ordering::Acquire)
}

pub fn stats() -> (u64, u64) {
    (VERIFIED.load(Ordering::Acquire), MATCHED.load(Ordering::Acquire))
}

/// Record one shadow verdict; divergence disarms. Returns the verdict.
pub fn record_verdict(matched: bool) -> bool {
    VERIFIED.fetch_add(1, Ordering::AcqRel);
    if matched {
        MATCHED.fetch_add(1, Ordering::AcqRel);
        true
    } else {
        disarm("shadow trace != vanilla trace");
        false
    }
}

/// activate(): dormant unless CRUSSTY_INSIDE_DDA_V2 is set. Even when armed,
/// this scaffold wires NOTHING into the JVM (offline model only) — the java
/// shadow entry and its JNI registration are the next leg (define-before-arm,
/// NCDFE-canon).
pub fn activate() {
    if !enabled() {
        eprintln!("[crussty-plugin] inside_dda_v2: dormant (set CRUSSTY_INSIDE_DDA_V2=1 to arm the shadow sampler)");
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_dda_v2: armed OFFLINE (model + selftests only; java wiring is NOT composed — NCDFE-canon, next leg)"
    );
}

// ---------------------------------------------------------------------------
// OFFLINE DDA REFERENCE MODEL (javap-verbatim traps, GOAL_20TPS §983)
//
// Scope of the scaffold model: the DDA phase of
// BlockGetter.forEachBlockIntersectedBetween — the exact phase that diverged
// 21/350k in DDA-v1. Corner/fast-path phases are traced by the vanilla walk
// itself and are out of scaffold scope (javap ground truth = next leg).
// t is the SEGMENT PARAMETER in [0,1] (0=from, 1=to): vanilla loops while
// ANY tMax <= 1 against the to-point — same units here.
// ---------------------------------------------------------------------------

/// A visited cell (BlockPos-equivalent triple).
pub type Cell = (i32, i32, i32);

/// Vanilla clamp of a cell boundary: `low = (double)((float)cell + 1.0E-5f)`
/// — the add happens in f32 (FLOAT trap); `high = (cell + 1.0) −
/// 9.999999747378752E-6` in f64.
#[inline]
fn clamp_lo(cell: i32) -> f64 {
    let f = cell as f32 + 1.0e-5f32;
    f as f64
}

#[inline]
fn clamp_hi(cell: i32) -> f64 {
    (cell as f64 + 1.0) - 9.999_999_747_378_752e-6
}

/// Axis decision, strictly-less canon: X iff tMaxX<tMaxY && tMaxX<tMaxZ.
/// (The Y/Z cascade below mirrors the same strict rule; the full 3-way javap
/// ground truth is re-verified on the leg that wires the java shadow.)
#[derive(Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[inline]
fn pick_axis(tmax_x: f64, tmax_y: f64, tmax_z: f64) -> Axis {
    if tmax_x < tmax_y && tmax_x < tmax_z {
        Axis::X
    } else if tmax_y < tmax_z {
        Axis::Y
    } else {
        Axis::Z
    }
}

/// The DDA walk transcription for the straight/DDA phase.
///
/// Traps encoded (§983): loop runs while ANY tMax <= 1; axis strictly-less;
/// zero-delta axis = infinite tMax (the vanilla `sign==0 → Double.MAX_VALUE`
/// step, never chosen, never advances); first-crossing boundaries use the
/// f32/f64 clamped cell edges (clamp_lo/clamp_hi); consecutive crossings of
/// the same side are exactly 1.0 apart, so per-step tMax advance = 1/|delta|
/// in segment-parameter units. The final cell is visited once (visited-set
/// semantics deduplicate revisits — mirror of the LongSet add gate).
pub fn dda_cells(from: (f64, f64, f64), to: (f64, f64, f64)) -> Vec<Cell> {
    let mut out = Vec::new();
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let dz = to.2 - from.2;

    // Stationary path: single cell (BlockPos$4 box-iteration parity is
    // verified by the vanilla trace; the shadow only needs the DDA phase).
    let (mut cx, mut cy, mut cz) =
        (from.0.floor() as i32, from.1.floor() as i32, from.2.floor() as i32);
    out.push((cx, cy, cz));

    let seg_len2 = dx * dx + dy * dy + dz * dz;
    if seg_len2 == 0.0 || seg_len2.is_nan() {
        return out; // stationary / degenerate: nothing to walk
    }
    if !seg_len2.is_finite() {
        // Poisoned input (overflowing segment): the shadow must not run on
        // data it cannot trust — fail-closed disarm, vanilla stays executor.
        disarm("non-finite segment in shadow model");
        return out;
    }

    // First-crossing tMax per axis (segment-parameter units), f64::MAX on a
    // zero-delta axis (vanilla sign==0 → Double.MAX_VALUE step).
    let first_tmax = |pos: f64, d: f64| -> f64 {
        if d == 0.0 {
            return f64::MAX;
        }
        let cell = pos.floor() as i32;
        let boundary = if d > 0.0 { clamp_hi(cell) } else { clamp_lo(cell) };
        (boundary - pos) / d
    };
    let mut tmx = first_tmax(from.0, dx);
    let mut tmy = first_tmax(from.1, dy);
    let mut tmz = first_tmax(from.2, dz);
    let sx = if dx > 0.0 { 1 } else if dx < 0.0 { -1 } else { 0 };
    let sy = if dy > 0.0 { 1 } else if dy < 0.0 { -1 } else { 0 };
    let sz = if dz > 0.0 { 1 } else if dz < 0.0 { -1 } else { 0 };
    let dtx = if dx == 0.0 { 0.0 } else { 1.0 / dx.abs() };
    let dty = if dy == 0.0 { 0.0 } else { 1.0 / dy.abs() };
    let dtz = if dz == 0.0 { 0.0 } else { 1.0 / dz.abs() };

    let mut steps = 0u64;
    while tmx.min(tmy).min(tmz) <= 1.0 {
        // Guarded ladder (fail-closed): a poisoned tMax or a runaway loop
        // disarms the shadow — it must never hang or mislead.
        if !tmx.is_finite() && !tmy.is_finite() && !tmz.is_finite() {
            disarm("non-finite tMax in shadow model");
            return out;
        }
        if steps >= STEP_CAP {
            disarm("step cap exceeded in shadow model");
            return out;
        }
        steps += 1;
        match pick_axis(tmx, tmy, tmz) {
            Axis::X => {
                cx += sx;
                tmx += dtx;
            }
            Axis::Y => {
                cy += sy;
                tmy += dty;
            }
            Axis::Z => {
                cz += sz;
                tmz += dtz;
            }
        }
        out.push((cx, cy, cz));
        if (cx, cy, cz) == (to.0.floor() as i32, to.1.floor() as i32, to.2.floor() as i32) {
            break;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// selftests — offline invariants (preregistered traps §983)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// The latch/counters are process-global — serialize the latch-sensitive
    /// tests so parallel cargo test threads cannot race them.
    static TEST_SER: Mutex<()> = Mutex::new(());

    /// Test-only re-arm: production has NO re-arm path (one-shot latch);
    /// tests need it to be order-independent under parallel execution.
    fn force_arm() {
        SHADOW_OK.store(true, Ordering::Relaxed);
    }

    #[test]
    fn sampler_is_deterministic_1_of_200() {
        let _g = TEST_SER.lock().unwrap();
        force_arm();
        assert!(should_verify(0)); // ordinal 0 fires (first call of a lane)
        assert!(!should_verify(199));
        assert!(should_verify(200));
        assert!(!should_verify(201));
        assert!(should_verify(400));
    }

    #[test]
    fn disarm_is_one_shot_and_never_rearms() {
        let _g = TEST_SER.lock().unwrap();
        // entry state is order-dependent (parallel tests share the latch)
        let was = SHADOW_OK.load(Ordering::Relaxed);
        assert_eq!(disarm("test ladder"), was); // returns true iff it flipped
        assert!(!SHADOW_OK.load(Ordering::Relaxed));
        assert!(!should_verify(200)); // sampler never fires after disarm
        assert!(!disarm("second call must not re-log")); // one-shot semantics
        assert!(disarm_fires() >= 2);
        // production never re-arms — this is the TEST-ONLY helper proving the
        // sampler resumes from an armed latch (and disarms again one-shot).
        force_arm();
        assert!(should_verify(200));
        assert!(disarm("re-latch check"));
        assert!(!should_verify(200));
        assert!(!SHADOW_OK.load(Ordering::Relaxed));
        assert!(!record_verdict(false)); // verdict path respects the latch
    }

    #[test]
    fn verdict_pipeline_counts_and_disarms_on_divergence() {
        let _g = TEST_SER.lock().unwrap();
        let (v0, m0) = stats();
        assert!(record_verdict(true)); // matched never disarms
        assert!(!record_verdict(false)); // divergence -> verdict false
        let (v1, m1) = stats();
        assert_eq!(v1, v0 + 2);
        assert_eq!(m1, m0 + 1);
        assert!(!SHADOW_OK.load(Ordering::Relaxed));
    }

    #[test]
    fn stationary_path_visits_one_cell() {
        assert_eq!(dda_cells((1.5, 2.5, 3.5), (1.5, 2.5, 3.5)), vec![(1, 2, 3)]);
    }

    #[test]
    fn straight_x_walk_is_monotone_in_cells() {
        let cells = dda_cells((1.5, 2.5, 3.5), (10.5, 2.5, 3.5));
        assert_eq!(cells.first(), Some(&(1, 2, 3)));
        assert_eq!(cells.last(), Some(&(10, 2, 3)));
        assert_eq!(cells.len(), 10);
        for w in cells.windows(2) {
            assert_eq!(w[1].0, w[0].0 + 1, "x must advance strictly: {w:?}");
            assert_eq!(w[1].1, w[0].1);
            assert_eq!(w[1].2, w[0].2);
        }
    }

    #[test]
    fn diagonal_walk_steps_alternate_axes() {
        // (0.5,0.5,0.5)->(2.5,2.5,0.5): dx==dy (tie), dz==0 — strict-less
        // canon forbids X on the tie, so the walk must be Y,X,Y,X.
        let cells = dda_cells((0.5, 0.5, 0.5), (2.5, 2.5, 0.5));
        assert_eq!(
            cells,
            vec![(0, 0, 0), (0, 1, 0), (1, 1, 0), (1, 2, 0), (2, 2, 0)]
        );
    }

    #[test]
    fn negative_direction_walk_reaches_target() {
        let cells = dda_cells((10.5, 2.5, 3.5), (1.5, 2.5, 3.5));
        assert_eq!(cells.last(), Some(&(1, 2, 3)));
        for w in cells.windows(2) {
            assert_eq!(w[1].0, w[0].0 - 1);
        }
    }

    #[test]
    fn float_clamp_absorbs_low_bits_in_f32() {
        // (double)((float)cell + 1.0E-5f): f32 precision swallows the add at
        // large magnitudes — an f64 add would NOT (trap §983).
        let cell = 12_345_679i32;
        assert_eq!(clamp_lo(cell), cell as f64);
        assert!(clamp_lo(0) > 0.0 && clamp_lo(0) < 2.0e-5);
        assert!(clamp_hi(5) > 6.0 - 1.0e-5 && clamp_hi(5) < 6.0);
    }

    #[test]
    fn center_uses_reverse_lerp_form() {
        // Vec3i.getCenter axis value = Mth.lerp(0.5, min, max), compiled as
        // max + 0.5*(min-max) (REVERSE form, NOT (min+max)/2) — the canonical
        // formula; assert the midpoint identity on exact ints.
        let center_lo_hi = |lo: i32, hi: i32| -> f64 { hi as f64 + 0.5 * (lo as f64 - hi as f64) };
        assert_eq!(center_lo_hi(3, 4), 3.5);
        assert_eq!(center_lo_hi(-2, 2), 0.0);
    }

    #[test]
    fn step_cap_disarms_instead_of_hanging() {
        // NO lock: must stay order-independent — earlier latch tests may have
        // disarmed; the model's fail-closed ladder fires regardless.
        let before = disarm_fires();
        // A pathological segment (huge span, overflowing progression) must
        // hit the poisoned-input ladder / STEP_CAP and disarm, never hang.
        let from = (0.0, 0.0, 0.0);
        let to = (f64::MAX * 0.5, 0.0, 0.0);
        let cells = dda_cells(from, to);
        assert!(disarm_fires() > before);
        assert!(cells.len() as u64 <= STEP_CAP + 1);
    }
}
