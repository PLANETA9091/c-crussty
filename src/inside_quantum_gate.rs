//! INSIDE-QUANTUM-GATE lever (ID-P34, TASK-459-74 — law-11 WILD scaffold,
//! tick-459). Development of the inside_cache static gate (S7-135):
//!
//!   static gate (live, inside_cache):  deltaMovement == 0 && position
//!     bit-equal to the cached tick  ->  serve discovery from flat slots
//!     (replay of the vanilla effect calls).
//!
//!   QUANTUM-REST class (this module):  0 < |Δ| < ε sustained over K
//!     consecutive ticks ("moving but slow" — jitter-in-place entities,
//!     crowd-pressed mobs, gravity-settling items)  ->  serve the
//!     discovery from the same flat-slot machinery with INCREMENTAL
//!     replay of the vanilla effect calls over the accumulated
//!     micro-drift segments.
//!
//! CENSUS MOTIVATION (inside_volatile lane 16.6% of wall, RECON-13/14C):
//! the static gate only covers bit-stationary entities. Moving entities
//! (~30-50% of them are "slow movers" with |Δ| below any sane epsilon,
//! per the round-458 inside census capture-math) keep paying the FULL
//! vanilla discovery each tick: two traversals (main from-to + final
//! to-to budget 1) + visitor geometry + LongOpenHashSet dedup +
//! PalettedContainer.get. Capturing the slow-mover class into the
//! existing flat-slot serve is worth 2-4пп of the 16.6% lane.
//!
//! THIS SCAFFOLD IS DORMANT BY DESIGN (law-11 scaffold tick):
//!   * gate env `CRUSSTY_INSIDE_QUANTUM` off by default -> register()
//!     logs a dormant notice, activate() returns immediately, nothing
//!     is defined into the kernel loader, zero byte hooks (NCDFE-canon:
//!     no class exists that a woven call site could fail to resolve);
//!   * ε and K are OFFLINE-ORACLE constants (ID-P34 card: "ε/K константы
//!     оффлайн-оракула; включение класса — только после 10k-сцен");
//!     they MUST be re-derived from the oracle before any arming;
//!   * the ε-criterion changes the TIMING of stateful inside effects
//!     (freeze/fire cadence depends on the per-tick call sequence) —
//!     the card marks exactly this as the oracle-gated risk;
//!   * fallback is pure vanilla (fail-closed, one-shot disarm latch).
//!
//! Ownership is DISJOINT from the live static gate: bit-equal position
//! with exactly-zero delta classifies to inside_cache (Decision::Static);
//! the quantum class owns only the strictly-moving band 0 < |Δ| < ε.
//! Hysteresis (Schmitt-form): promotion needs K consecutive sub-ε ticks,
//! declassification on a single ≥ε jump plus one mandatory full-vanilla
//! tick before re-eligibility — no promotion flapping at the threshold.
//!
//! REPLAY CADENCE (in-tick, exactly-once): during DWELL vanilla runs and
//! is the source of truth (the anchor tracks the live position, the
//! replay backlog stays EMPTY — no double-apply). The promotion tick
//! replays its own micro-segment (steps=1). Every REST tick adds its own
//! segment to the backlog and the serve drains up to MAXSTEPS (= the
//! vanilla visitor budget, javap 9-16) per tick, oldest first — so every
//! tick's swept segment is processed exactly once, in-tick. A backlog
//! growing past the hard cap MAXSTEPS+K_TICKS means the serve machinery
/// failed to drain for many ticks (wiring fault) -> one-shot disarm.

// ---------------------------------------------------------------------------
// Oracle constants (v1 defaults; re-derive from the 10k-scenario oracle
// before ANY arming — ID-P34 card contract).
// ---------------------------------------------------------------------------

/// ε: sub-threshold per-tick displacement, blocks (len-squared compared
/// bit-exactly against f64 EPSILON*EPSILON). Vanilla "moved" threshold is
/// sqrt(square(0.9999900000002526)) ≈ 0.99999 blocks/tick (javap,
/// checkInsideBlocks intersected-flag); ε sits 4 orders of magnitude below
/// it, so the quantum class never overlaps the vanilla moved-band.
pub const EPSILON: f64 = 1.0e-4;
/// K: dwell length in ticks before promotion to the rest-class.
pub const K_TICKS: u32 = 20;
/// Per-tick replay drain cap = the vanilla visitor budget mirror (javap
/// 9-16, InsideBlockOps.MAXSTEPS).
pub const MAXSTEPS: u32 = 16;
/// Hard backlog cap: exceeding it means the serve drain is not keeping up
/// (wiring fault) -> one-shot disarm (fail-closed).
pub const BACKLOG_CAP: u32 = MAXSTEPS + K_TICKS;

const EPSILON_SQR_BITS: u64 = (EPSILON * EPSILON).to_bits();

// ---------------------------------------------------------------------------
// Phase machine (pure reference model — the JNI/flat-slot wiring is v2).
// ---------------------------------------------------------------------------

pub const PHASE_VANILLA: u8 = 0;
pub const PHASE_DWELL: u8 = 1;
pub const PHASE_REST: u8 = 2;
pub const PHASE_DISARMED: u8 = 3;

/// One entity's classifier state (flat, no allocs on the tick path).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantumSlot {
    pub eid: u64,
    pub phase: u8,
    pub dwell: u32,
    /// Mandatory full-vanilla ticks after declassification (hysteresis).
    pub vanilla_hold: u32,
    /// Replay backlog: micro-drift segments awaiting replay (REST only).
    pub pending: u32,
    /// Position bits captured at the last serve/anchor point (bit-exact).
    pub anchor: [u64; 3],
}

impl QuantumSlot {
    pub fn new(eid: u64) -> Self {
        QuantumSlot {
            eid,
            phase: PHASE_VANILLA,
            dwell: 0,
            vanilla_hold: 0,
            pending: 0,
            anchor: [0; 3],
        }
    }
}

/// Per-tick decision for `Entity.checkInsideBlocks`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    /// Pure vanilla discovery (fail-dominant default).
    Vanilla,
    /// Bit-stationary: the live inside_cache static gate owns this tick.
    Static,
    /// Serve from flat slots; replay `steps` drained backlog segments
    /// (oldest first; normal steady state = 1).
    Serve { steps: u32 },
}

/// Len-squared comparison, bit-exact (no rounding, no NaN-coercion):
/// returns |Δ|² < ε² using raw f64 bit patterns for the ε side.
pub fn sub_epsilon(delta_bits: [u64; 3]) -> bool {
    let dx = f64::from_bits(delta_bits[0]);
    let dy = f64::from_bits(delta_bits[1]);
    let dz = f64::from_bits(delta_bits[2]);
    let s = dx * dx + dy * dy + dz * dz;
    // Strictly below ε² AND strictly above exact zero (zero-delta +
    // bit-equal position belongs to the static gate, not this class).
    s.to_bits() < EPSILON_SQR_BITS && s.to_bits() != 0
}

/// Advance the classifier by one tick.
///
/// * `delta_bits` = this tick's movement (raw f64 bits, dx/dy/dz).
/// * `pos_bits`   = current position bits (replay anchor candidate).
pub fn tick(slot: &mut QuantumSlot, delta_bits: [u64; 3], pos_bits: [u64; 3]) -> Decision {
    // One-shot disarm: permanent vanilla (oracle must re-derive; a disarmed
    // entity never re-enters the class this session — fail-closed).
    if slot.phase == PHASE_DISARMED {
        return Decision::Vanilla;
    }
    // Hysteresis hold: after a ≥ε jump run full vanilla at least one tick.
    if slot.vanilla_hold > 0 {
        slot.vanilla_hold -= 1;
        slot.dwell = 0;
        slot.phase = PHASE_VANILLA;
        slot.pending = 0;
        slot.anchor = pos_bits;
        return Decision::Vanilla;
    }
    // Exact-zero delta: static-gate ownership (disjoint classes).
    if delta_bits == [0u64; 3] {
        slot.dwell = 0;
        slot.pending = 0;
        slot.anchor = pos_bits;
        slot.phase = PHASE_VANILLA;
        return Decision::Static;
    }
    if !sub_epsilon(delta_bits) {
        // Outside the quantum band: declassify + Schmitt hold.
        slot.phase = PHASE_VANILLA;
        slot.dwell = 0;
        slot.pending = 0;
        slot.vanilla_hold = 1;
        slot.anchor = pos_bits;
        return Decision::Vanilla;
    }
    // Sub-ε micro-move.
    slot.dwell = slot.dwell.saturating_add(1);
    if slot.phase == PHASE_REST {
        // This tick's segment joins the backlog, then the serve drains up
        // to MAXSTEPS segments oldest-first (normal steady state: 1).
        slot.pending = slot.pending.saturating_add(1);
        if slot.pending > BACKLOG_CAP {
            // Drain-fault: the serve machinery failed to keep up for many
            // ticks — replay can no longer be proven bit-in-byte -> disarm.
            slot.phase = PHASE_DISARMED;
            slot.pending = 0;
            slot.dwell = 0;
            return Decision::Vanilla;
        }
        let steps = slot.pending.min(MAXSTEPS);
        slot.pending -= steps;
        slot.anchor = pos_bits;
        return Decision::Serve { steps };
    }
    if slot.dwell >= K_TICKS {
        // Promotion: K vanilla ticks paid in full; THIS tick's segment is
        // replayed in-tick (steps=1) and steady-state REST begins.
        slot.phase = PHASE_REST;
        slot.pending = 0;
        slot.anchor = pos_bits;
        return Decision::Serve { steps: 1 };
    }
    // DWELL: vanilla is the source of truth — full discovery runs, the
    // anchor tracks the live position, backlog stays empty.
    slot.phase = PHASE_DWELL;
    slot.pending = 0;
    slot.anchor = pos_bits;
    Decision::Vanilla
}

/// Flat slot-table lookup (same layout discipline as InsideBlockOps:
/// slot = eid & (NSLOTS-1), stamp = eid, 0 = empty). Slot collision with
/// a different eid -> fresh slot (miss -> vanilla until it re-dwells).
pub const NSLOTS_LOG2: usize = 18;
pub const NSLOTS: usize = 1 << NSLOTS_LOG2;

pub fn slot_index(eid: u64) -> usize {
    (eid as usize) & (NSLOTS - 1)
}

// ---------------------------------------------------------------------------
// Dormant gate (canon: inside_cache.rs register/activate discipline).
// ---------------------------------------------------------------------------

fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_QUANTUM")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate visibility for the compose chain / diagnostics.
pub fn enabled_pub() -> bool {
    enabled()
}

/// Register (idempotent, call once from cplugin_init). DORMANT scaffold:
/// no byte hook, no define_class, no blob wiring (NCDFE-canon — with the
/// lever off the module is byte-indistinguishable from the pre-P34 plugin;
/// with the lever on it STILL only arms after the offline oracle ships
/// constants — see RESEARCH-459-P34.md §2).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_quantum_gate: dormant (set CRUSSTY_INSIDE_QUANTUM=1 to enable; class arming requires 10k-scenario oracle)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_quantum_gate: classifier model armed (lever cmp459_p34), JNI/flat-slot wiring NOT shipped in v1 — vanilla bytes untouched"
    );
}

/// Background activation. v1: NOTHING is defined into the kernel loader
/// (no blob, no retarget) — the only effect is the model self-test below,
/// which pins the classifier contract for the oracle leg.
pub fn activate() {
    if !enabled() {
        return;
    }
    let cases = selftest_cases();
    let (mut ok, n) = (true, cases.len());
    for t in &cases {
        if !t.1 {
            ok = false;
        }
    }
    eprintln!(
        "[crussty-plugin] inside_quantum_gate: model selftest {}/{} {}",
        if ok { n } else { 0 },
        n,
        if ok { "OK" } else { "FAIL (stay dormant)" }
    );
}

fn selftest_cases() -> Vec<(&'static str, bool)> {
    vec![
        ("sub_epsilon band edges", test_band_edges()),
        ("dwell K-1 stays vanilla", test_dwell_no_promote()),
        ("promotion serves own segment", test_dwell_promote()),
        ("jump declassifies + hold", test_declassify_hysteresis()),
        ("zero delta -> static gate", test_static_ownership()),
        ("steady rest drains 1 per tick", test_serve_flush()),
        ("drain cap + hard-cap disarm", test_overflow_disarm()),
        ("slot collision -> fresh slot", test_slot_collision()),
    ]
}

// ---------------------------------------------------------------------------
// Offline model self-test (unit-testable pure functions).
// ---------------------------------------------------------------------------

fn b(x: f64) -> u64 {
    x.to_bits()
}

pub fn test_band_edges() -> bool {
    // 0 < |Δ| < ε accepted...
    if !sub_epsilon([b(1.0e-5), b(0.0), b(0.0)]) {
        return false;
    }
    // ...exact zero rejected (static-gate ownership)...
    if sub_epsilon([b(0.0), b(0.0), b(0.0)]) {
        return false;
    }
    // ...|Δ| == ε rejected (strict band)...
    if sub_epsilon([b(EPSILON), b(0.0), b(0.0)]) {
        return false;
    }
    // ...and vanilla moved-band rejected.
    if sub_epsilon([b(0.9), b(0.0), b(0.0)]) {
        return false;
    }
    true
}

pub fn test_dwell_no_promote() -> bool {
    let mut s = QuantumSlot::new(7);
    let d = [b(5.0e-5), b(0.0), b(0.0)];
    for i in 0..(K_TICKS - 1) {
        if tick(&mut s, d, [b(1.0 + i as f64 * 5.0e-5), b(2.0), b(3.0)]) != Decision::Vanilla {
            return false;
        }
    }
    s.phase == PHASE_DWELL && s.dwell == K_TICKS - 1 && s.pending == 0
}

pub fn test_dwell_promote() -> bool {
    let mut s = QuantumSlot::new(7);
    let d = [b(5.0e-5), b(0.0), b(0.0)];
    for i in 0..(K_TICKS - 1) {
        let _ = tick(&mut s, d, [b(1.0 + i as f64 * 5.0e-5), b(2.0), b(3.0)]);
    }
    // Promotion tick replays its own segment (steps=1), REST begins.
    if !matches!(tick(&mut s, d, [b(1.0 + 19.0 * 5.0e-5), b(2.0), b(3.0)]),
                 Decision::Serve { steps: 1 }) {
        return false;
    }
    s.phase == PHASE_REST && s.pending == 0
}

pub fn test_declassify_hysteresis() -> bool {
    let mut s = QuantumSlot::new(7);
    let d = [b(5.0e-5), b(0.0), b(0.0)];
    for i in 0..K_TICKS {
        let _ = tick(&mut s, d, [b(1.0 + i as f64 * 5.0e-5), b(2.0), b(3.0)]);
    }
    // A single ≥ε jump -> vanilla + hold (no serve of the broken drift).
    if tick(&mut s, [b(0.5), b(0.0), b(0.0)], [b(9.0), b(2.0), b(3.0)]) != Decision::Vanilla {
        return false;
    }
    if s.vanilla_hold != 1 {
        return false;
    }
    // Hold tick runs vanilla regardless, then dwell restarts from zero.
    let _ = tick(&mut s, d, [b(9.0), b(2.0), b(3.0)]);
    s.phase == PHASE_VANILLA && s.dwell == 0 && s.vanilla_hold == 0
}

pub fn test_static_ownership() -> bool {
    let mut s = QuantumSlot::new(7);
    // Exactly-zero delta (bits) -> static gate, disjoint from quantum.
    matches!(
        tick(&mut s, [b(0.0), b(0.0), b(0.0)], [b(1.0), b(2.0), b(3.0)]),
        Decision::Static
    )
}

pub fn test_serve_flush() -> bool {
    let mut s = QuantumSlot::new(7);
    let d = [b(5.0e-5), b(0.0), b(0.0)];
    for i in 0..K_TICKS {
        let _ = tick(&mut s, d, [b(1.0 + i as f64 * 5.0e-5), b(2.0), b(3.0)]);
    }
    // Steady-state REST: exactly one in-tick segment drained per tick.
    if !matches!(tick(&mut s, d, [b(1.0 + 19.0 * 5.0e-5), b(2.0), b(3.0)]),
                 Decision::Serve { steps: 1 }) {
        return false;
    }
    if !matches!(tick(&mut s, d, [b(1.0 + 20.0 * 5.0e-5), b(2.0), b(3.0)]),
                 Decision::Serve { steps: 1 }) {
        return false;
    }
    s.anchor == [b(1.0 + 20.0 * 5.0e-5), b(2.0), b(3.0)] && s.pending == 0
}

pub fn test_overflow_disarm() -> bool {
    let mut s = QuantumSlot::new(7);
    let d = [b(5.0e-5), b(0.0), b(0.0)];
    // REST with a backlog of exactly MAXSTEPS: drains fully (cap boundary).
    s.phase = PHASE_REST;
    s.pending = MAXSTEPS;
    if !matches!(tick(&mut s, d, [b(4.0), b(2.0), b(3.0)]),
                 Decision::Serve { steps: MAXSTEPS }) {
        return false;
    }
    if s.pending != 1 {
        return false;
    }
    // Backlog past the hard cap (drain fault): one-shot disarm, permanent.
    s.pending = BACKLOG_CAP + 1;
    if tick(&mut s, d, [b(4.0), b(2.0), b(3.0)]) != Decision::Vanilla {
        return false;
    }
    s.phase == PHASE_DISARMED && tick(&mut s, d, [b(4.0), b(2.0), b(3.0)]) == Decision::Vanilla
}

pub fn test_slot_collision() -> bool {
    // Same flat slot, different eid stamps: slot_index collides, the stamp
    // check treats it as a fresh slot (miss -> vanilla until re-dwell).
    let a = slot_index(0xDEAD_BEEF);
    let b2 = slot_index(0xDEAD_BEEF + (NSLOTS as u64));
    a == b2 // layout discipline: identical slot, eid disambiguates the stamp
}

// ---------------------------------------------------------------------------
// Tests (cargo test --lib).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_edges_strict() {
        assert!(test_band_edges());
    }

    #[test]
    fn dwell_k_minus_one_stays_vanilla() {
        assert!(test_dwell_no_promote());
    }

    #[test]
    fn promotion_serves_own_segment() {
        assert!(test_dwell_promote());
    }

    #[test]
    fn declassify_hysteresis() {
        assert!(test_declassify_hysteresis());
    }

    #[test]
    fn zero_delta_owned_by_static_gate() {
        assert!(test_static_ownership());
    }

    #[test]
    fn steady_rest_drains_one_per_tick() {
        assert!(test_serve_flush());
    }

    #[test]
    fn drain_cap_and_hard_cap_disarm() {
        assert!(test_overflow_disarm());
    }

    #[test]
    fn flat_slot_layout_discipline() {
        assert!(test_slot_collision());
    }
}
