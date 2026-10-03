//! SELECTOR-BULK R1 (ROUND-487 C65, lever `cmp487_sbulk1`) — Rust-side
//! bulk-enum API CONTRACT for the R1 wiring leg, layered on the C07 skeleton
//! (`selector_bulk.rs` @802ab5b5: sbProbe/sbEnumerate/sbStats natives).
//!
//! R1 (LEDGER Л-486-C07 plan): body-redirect of
//! `EntitySelector.getEntities(EntitySelector, Level, AABB)` (86.9% of the
//! dp lane ≈ 47.7% ALL-CPU, BOTTLENECK ×487 #1) →
//! `SelectorBulkOps.getEntitiesGated` via retarget_invokestatic
//! (classfile.rs:1507), STRICT-eq lever. R2 (`EntityLookup.getEntities`)
//! arms ONLY if measured R1 capture < 55% — the capture math and the arm
//! predicate are pinned here (`R2_CAPTURE_MIN`, `RetargetSpec::R2`).
//!
//! THIS TICK = contract заготовка, DORMANT by construction:
//!   - types: plane layout, native sig table, retarget specs, fill report
//!     + THRESH-estimate over the per-fill grid histogram, phase machine;
//!   - counters: extended monotonic fail-dominant family (G6) with the
//!     vanilla-fallback REASON breakdown (gate/cold/thresh/structural/err);
//!   - boundaries: THRESH/MARGIN cross-checked against the C07 skeleton,
//!     OUT_CAP_MAX / MAX_PLANE_ROWS / HIST_CELL / cold-version semantics.
//! ZERO new `#[no_mangle]` exports, ZERO define/retarget/retransform, ZERO
//! JVMTI/JNI calls: the exported-symbol surface stays bit-identical to C07
//! (sbProbe/sbEnumerate/sbStats only) ⇒ 0-delta vs PIN holds trivially and
//! the NCDFE T1=0 gate has nothing to observe. JNI PUBLISHING (native
//! RegisterNatives table + define + retarget) lands on the NEXT tick as the
//! CI-доба wiring leg, after the C01 base-rep arbiter confirms the dp-base
//! band (0.3±0.0 anchor; the ×486-C07 run 36490915319 0.5-anomaly must be
//! base-rep'd first).
//!
//! LAW-5 BOUNDARY (unchanged, re-pinned): NOT RECON-39 — the plane is a
//! per-tick rebuild with fail-dominant structural fallback + monotonic
//! counters (legal batching/layout class, eqEpoch precedent); NOT RECON-40 —
//! zero GC buttons. Residual predicates ALWAYS on live entities (strict-
//! superset oracle Л146-G2, MARGIN over-admission only).

// CONTRACT-DORMANT: every item below is consumed by the wiring tick (JNI
// publishing leg) and the offline test harness — not by the running vanilla
// path this tick. Dead-code silence is the intended state (repo convention:
// chunk_send.rs/tickplane.rs/batch_api.rs pre-rendered contracts).
#![allow(dead_code)]

use std::sync::atomic::{AtomicI64, Ordering};

// ---------------------------------------------------------------------------
// Gate (STRICT-eq lever protocol, round-400 canon): NEW R1 lever id — the C07
// skeleton gate (cmp486_sbulk1 + CRUSSTY_SELECTOR_BULK=1) stays as-is and
// keeps owning the exported natives; this contract leg arms independently so
// the wiring tick can pin the exact bytes it shipped against THIS contract.
// ---------------------------------------------------------------------------

/// R1 lever marker (STRICT-eq, single id — swarx-4 lesson).
pub const R1_LEVER_FLAG: &str = "cmp487_sbulk1";

/// Subsystem arm env for the R1 contract leg (separate from the C07
/// CRUSSTY_SELECTOR_BULK arm: two independent switches, never OR-ed).
pub const R1_ARM_ENV: &str = "CRUSSTY_SBLK_R1";

/// Compo retag-мёрж (Л175, AG-36 w528): TASK-528-COMPO arms the R1 contract
/// leg together with the window plane under ONE flag (AG-86 fork lesson:
/// STRICT-eq lists cannot be unioned via env alone).
pub const R1_COMPO_FLAG: &str = "cmp528_compo";

fn r1_enabled() -> bool {
    r1_enabled_with(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref().ok().as_deref(),
        std::env::var(R1_ARM_ENV).as_deref().ok().as_deref(),
    )
}

/// PURE production gate (test module pins THIS function).
pub fn r1_enabled_with(lever: Option<&str>, arm: Option<&str>) -> bool {
    matches!(lever, Some(R1_LEVER_FLAG) | Some(R1_COMPO_FLAG))
        && matches!(arm, Some("1"))
}

// ---------------------------------------------------------------------------
// Boundaries — MUST match selector_bulk.rs AND SelectorBulkOps.java exactly.
// ---------------------------------------------------------------------------

/// THRESH=512 (LEDGER Л1333 preregister) — cross-checked against the C07
/// skeleton constant at compile time by the test module (parity pin).
pub const SB_THRESH: i32 = crate::selector_bulk::SB_THRESH;

/// MARGIN=4 superset over-admission — same cross-check.
pub const SB_MARGIN: f64 = crate::selector_bulk::SB_MARGIN;

/// Candidate buffer cap: java allocates `int[min(rows, 1<<16)]`
/// (SelectorBulkOps.getEntitiesGated) — the wiring tick must not drift.
pub const OUT_CAP_MAX: usize = 1 << 16;

/// Hard per-tick fill cap (rows). A universe above this = fill FAILS
/// fail-dominant (vanilla tick, R1_PLANE_FILL_FAILS++), never a truncated
/// plane silently entering queries (under-admission is forbidden — G2).
pub const MAX_PLANE_ROWS: usize = 1 << 21; // 2,097,152

/// Per-fill grid histogram cell (blocks) for the THRESH estimate. Cells
/// PARTITION the plane bounding box: every row counts into exactly ONE cell
/// (partition invariant, test-pinned) ⇒ the estimate is a true lower bound of
/// rows under the expanded box → the THRESH gate never over-routes to bulk.
pub const HIST_CELL: f64 = 64.0;

/// Histogram grid side cap (cells): 256² = 65,536 i32 cells = 256 KB max —
/// keeps the per-fill histogram build O(rows) + O(side²) memset trivially
/// cheap vs the O(N) scans it decimates.
pub const HIST_MAX_SIDE: usize = 256;

/// structuralVersion semantics: 0 = "never filled" (COLD plane — any query on
/// a cold plane routes vanilla, R1_VANILLA_COLD_PLANE++); the first fill
/// publishes 1; versions are STRICTLY monotonic +1 per fill (no reuse, no
/// wrap inside a soak: i64 counter, G6).
pub const STRUCT_VERSION_COLD: i64 = 0;

/// R2 arm predicate threshold (LEDGER Л-486-C07): EntityLookup.getEntities
/// retargets ONLY if measured R1 capture < 55% — never both lanes hot.
pub const R2_CAPTURE_MIN: f64 = 0.55;

/// ERR codes — mirror of the C07 native contract (kept local so this module
/// documents the full boundary without touching skeleton internals).
pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

// ---------------------------------------------------------------------------
// Types — the wiring-tick API contract (pure; zero runtime surface).
// ---------------------------------------------------------------------------

/// SoA plane column contract (java-owned arrays; rust pins the layout).
/// Row `i` semantics: dense plane id == row index; fill order == vanilla
/// section walk order (order-parity canon — DP-PARITY-3 sequence-hash pins it
/// end-to-end); `ids[i]` maps row i → live entity on the java side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneCol {
    /// `[J` entity ids (dense plane id column).
    Ids,
    /// `[D` center-x column.
    Xs,
    /// `[D` center-z column.
    Zs,
    /// `[I` dense type ordinal (0 = unassigned; `type_ordinals` contract).
    TypeOrd,
    /// `[I` section order stamp (MC-310372 section-order preservation; R2).
    SectionOrder,
}

impl PlaneCol {
    /// Exact JNI array-type descriptor of the column (contract pin for the
    /// RegisterNatives/fill code — TASK-409-E: sig drift = NoSuchMethodError).
    pub fn jni_arr(self) -> &'static str {
        match self {
            PlaneCol::Ids => "[J",
            PlaneCol::Xs => "[D",
            PlaneCol::Zs => "[D",
            PlaneCol::TypeOrd => "[I",
            PlaneCol::SectionOrder => "[I",
        }
    }
}

/// Native signature table (EXACT match to the C07 rust natives and the
/// SelectorBulkOps.java `private static native` declarations). The wiring
/// tick passes these strings to RegisterNatives VERBATIM.
pub struct NativeSig {
    pub name: &'static str,
    pub desc: &'static str,
}

/// `sbProbe()I` — magic 0x5342 handshake.
pub const SIG_PROBE: NativeSig = NativeSig { name: "sbProbe", desc: "()I" };
/// `sbEnumerate([D[D[IDDDDIII[I)I` — (xs, zs, typeOrd, minX, maxX, minZ,
/// maxZ, typeQ, limit, structuralFresh, out) → candidate count / ERR_*.
pub const SIG_ENUMERATE: NativeSig =
    NativeSig { name: "sbEnumerate", desc: "([D[D[IDDDDIII[I)I" };
/// `sbStats([J)I` — monotonic counter readback (4 slots).
pub const SIG_STATS: NativeSig = NativeSig { name: "sbStats", desc: "([J)I" };

/// Retarget spec for one body-redirect site (retarget_invokestatic,
/// classfile.rs:1507 machinery). FQNs/descriptors are CP-EXACT contract pins:
/// the offline javap pass (CI, scripts/cert458n) verifies them against the
/// built kernel BEFORE the define/retarget lands (local sandbox is JRE-only —
/// CANON-COMMANDER.md:24; a pin mismatch = the wiring leg must NOT ship).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetargetSpec {
    /// Site class holding the redirected static call.
    pub from_class: &'static str,
    /// Site method (body-redirect: the METHOD stays, its body gains the
    /// gated bulk entry with bit-in-bit vanilla fallback).
    pub from_method: &'static str,
    /// Erased site descriptor (List<? extends Entity> → java/util/List).
    pub from_desc: &'static str,
    /// Gated entry class/method (SelectorBulkOps.getEntitiesGated).
    pub to_class: &'static str,
    pub to_method: &'static str,
}

/// R1 — the ONLY unconditional retarget of the wiring tick.
pub const RETARGET_R1: RetargetSpec = RetargetSpec {
    from_class: "net/minecraft/world/entity/selector/EntitySelector",
    from_method: "getEntities",
    from_desc: "(Lnet/minecraft/world/entity/EntitySelector;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;",
    to_class: "net/minecraft/world/entity/selector/SelectorBulkOps",
    to_method: "getEntitiesGated",
};

/// R2 — CONDITIONAL lane (x494 РЕПИН: phantom moonrise FQN
/// `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup`
/// .getEntities(AABB)List = 0× in jar md5 83b6f9c9 (Л-492-C22 forensics) —
/// REPLACED by the javap-verified LevelEntityGetter receiver-prepended trio
/// (see scripts/javap_pins_verify_cmp493.sh, 7/7 PASS). Arms only under
/// `r2_allowed(capture_r1)`; CP-EXACT descs below are the retarget anchors.
pub const RETARGET_R2_T: RetargetSpec = RetargetSpec {
    from_class: "net/minecraft/world/level/entity/LevelEntityGetter",
    from_method: "get",
    from_desc: "(Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/util/AbortableIterationConsumer;)V",
    to_class: "net/minecraft/world/entity/selector/SelectorBulkOps",
    to_method: "getEntitiesGated",
};

pub const RETARGET_R2_E: RetargetSpec = RetargetSpec {
    from_class: "net/minecraft/world/level/entity/LevelEntityGetter",
    from_method: "get",
    from_desc: "(Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Consumer;)V",
    to_class: "net/minecraft/world/entity/selector/SelectorBulkOps",
    to_method: "getEntitiesGated",
};

pub const RETARGET_R2_C: RetargetSpec = RetargetSpec {
    from_class: "net/minecraft/world/level/entity/LevelEntityGetter",
    from_method: "get",
    from_desc: "(Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/util/AbortableIterationConsumer;)V",
    to_class: "net/minecraft/world/entity/selector/SelectorBulkOps",
    to_method: "getEntitiesGated",
};

/// R1-funnel (x494 РЕПИН): Level.getEntities(TypeTest, AABB, Predicate, List,
/// int) declared in Level, CSEL.addEntities hot-site routes here via
/// inheritance (javap -c: 2 invokevirtual в EntitySelector). Receiver-
/// prepended static-retarget = 6-arg form.
pub const RETARGET_R1_FUNNEL: RetargetSpec = RetargetSpec {
    from_class: "net/minecraft/world/level/Level",
    from_method: "getEntities",
    from_desc: "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;Ljava/util/List;I)V",
    to_class: "net/minecraft/world/entity/selector/SelectorBulkOps",
    to_method: "getEntitiesGated",
};

/// R2 arm predicate: capture is the measured fraction of dp-lane CPU routed
/// through R1 (spark raw-poll attribution, dp-стенд канон). ONLY a measured
/// value below `R2_CAPTURE_MIN` opens R2 — never a projection.
pub fn r2_allowed(capture_r1: f64) -> bool {
    capture_r1.is_finite() && capture_r1 < R2_CAPTURE_MIN
}

/// Wiring phase machine (monotone, fail-dominant). Every step is taken only
/// on explicit success; a failure HOLDS the phase (retry next wiring leg) —
/// the machine NEVER skips a step and NEVER rolls back visible state; the
/// only downward move is the explicit kill-switch `Phase::disarm_all`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    /// Nothing defined/retargeted (this tick's state — 0-delta vs PIN).
    Dormant,
    /// SelectorBulkOps EARLY-defined (NCDFE T1=0 gate: define BEFORE any
    /// retarget — a site must never resolve the class during retransform).
    Defined,
    /// R1 body-redirect live (retransform accepted, mirror-drift monitor on).
    Retargeted,
    /// First fresh plane fill published (structuralVersion ≥ 1): queries may
    /// enter the THRESH gate. Any structural drift DEMOTES routing to
    /// vanilla at QUERY time (java-side fail-dominant) — the phase itself
    /// stays Armed (fill/repair is per-tick, not a wiring event).
    Armed,
}

impl Phase {
    /// One wiring step: `ok=true` promotes EXACTLY one phase; `ok=false`
    /// holds (fail-dominant). Never skips, never self-rolls-back.
    pub fn advance(self, ok: bool) -> Phase {
        if !ok {
            return self;
        }
        match self {
            Phase::Dormant => Phase::Defined,
            Phase::Defined => Phase::Retargeted,
            Phase::Retargeted => Phase::Armed,
            Phase::Armed => Phase::Armed,
        }
    }

    /// Kill-switch (operator/G6-story only): full vanilla bit-in-bit.
    pub fn disarm_all(self) -> Phase {
        Phase::Dormant
    }
}

/// Per-tick fill report (java → contract at arm/readback; pure mirror for
/// estimate + freshness math). The histogram partitions the plane bounding
/// box `[min_px, min_px + side*HIST_CELL) × [min_pz, ...)` into `side²` i32
/// cells, row-per-cell partition invariant.
#[derive(Clone, Debug, Default)]
pub struct SbFillReport {
    /// Rows filled this tick (0 = cold/empty plane → queries route vanilla).
    pub rows: usize,
    /// structuralVersion published by this fill (≥1; strictly monotonic).
    pub structural_version: i64,
    /// Plane bounding-box origin (min corner of the histogram grid).
    pub min_px: f64,
    pub min_pz: f64,
    /// Histogram grid side in cells (≤ HIST_MAX_SIDE; 0 = no histogram).
    pub hist_side: usize,
    /// `hist_side²` cell counts (row-major: cell (cx, cz) = hist[cz*side+cx]).
    pub hist: Vec<i32>,
}

impl SbFillReport {
    /// THRESH-gate estimate: candidate-row estimate for the query box after
    /// MARGIN expansion = sum of histogram cells whose windows intersect the
    /// expanded box. Cell math is INTEGER-indexed (floor((c - origin)/CELL),
    /// clamped to [0, side-1]) so java re-derives bit-identical selections —
    /// NO f64 tolerance anywhere (parity canon). Partition invariant ⇒ the
    /// result is a lower bound of the rows the flat filter would admit into
    /// these cells → the gate never over-routes bulk on estimate.
    ///
    /// Fail-dominant direction: ANY doubt (non-finite coords, degenerate
    /// histogram) → estimate 0 ⇒ `estimate ≤ SB_THRESH` ⇒ VANILLA walk.
    /// (Both routes are CORRECT — live validation runs on either path — the
    /// doubt direction only picks the cheap vanilla walk, never the fast
    /// path, which is the fail-dominant canon.)
    pub fn estimate_candidates(
        &self,
        min_x: f64,
        max_x: f64,
        min_z: f64,
        max_z: f64,
    ) -> i64 {
        if self.rows == 0
            || self.hist_side == 0
            || self.hist.len() != self.hist_side * self.hist_side
            || min_x > max_x
            || min_z > max_z
            || !min_x.is_finite()
            || !max_x.is_finite()
            || !min_z.is_finite()
            || !max_z.is_finite()
            || !self.min_px.is_finite()
            || !self.min_pz.is_finite()
        {
            return 0; // doubt ⇒ ≤ THRESH ⇒ vanilla (fail-dominant)
        }
        // Expanded box, then local coords, then integer cell windows. Both
        // axes clamp to the grid; an empty intersection (box fully outside,
        // including fully negative or fully beyond the last cell) ⇒ estimate
        // 0 — a correct lower bound, never a forced cell-0 selection.
        let s = self.hist_side;
        let last = (s - 1) as f64;
        let x0 = cell_lo(min_x - SB_MARGIN, self.min_px).max(0.0);
        let x1 = cell_hi(max_x + SB_MARGIN, self.min_px, s).min(last);
        if x0 > x1 {
            return 0;
        }
        let z0 = cell_lo(min_z - SB_MARGIN, self.min_pz).max(0.0);
        let z1 = cell_hi(max_z + SB_MARGIN, self.min_pz, s).min(last);
        if z0 > z1 {
            return 0;
        }
        let mut acc: i64 = 0;
        for cz in (z0 as usize)..=(z1 as usize) {
            for cx in (x0 as usize)..=(x1 as usize) {
                acc += self.hist[cz * s + cx] as i64;
            }
        }
        acc.min(self.rows as i64)
    }

    /// Structural freshness vs a previously seen version: STRICTLY monotonic
    /// (≥1 fresh, no reuse). `seen` = version the caller last observed; a
    /// caller must route vanilla unless `fresh(seen)` AND `rows > 0`.
    pub fn fresh(&self, seen: i64) -> bool {
        self.structural_version > STRUCT_VERSION_COLD && self.structural_version > seen
    }
}

/// floor((coord - origin) / HIST_CELL) as f64 cell index (caller clamps).
fn cell_floor(coord: f64, origin: f64) -> f64 {
    ((coord - origin) / HIST_CELL).floor()
}

fn cell_lo(expanded_min: f64, origin: f64) -> f64 {
    cell_floor(expanded_min, origin)
}

/// Inclusive upper cell index touched by `expanded_max` (clamped later).
fn cell_hi(expanded_max: f64, origin: f64, side: usize) -> f64 {
    cell_floor(expanded_max, origin).min((side - 1) as f64)
}

/// Dense type-ordinal assignment contract (C89 dense-window class): keys are
/// the entity-type resource-location strings SORTED LEXICOGRAPHICALLY by the
/// caller (java passes the same sorted set both sides see) → ordinals are
/// DENSE `1..=n` in that order, deterministic and parity-stable across legs.
/// `0` is RESERVED for "unassigned" (never emitted here) so the flat filter's
/// `type_q >= 0` comparison keeps its C07 semantics.
pub fn type_ordinals(sorted_keys: &[&str]) -> Vec<i32> {
    (1..=sorted_keys.len() as i32).collect()
}

/// Plane buffer grow policy for the fill pass: power-of-two ≥ n, clamped to
/// [`PLANE_MIN_CAP`, `MAX_PLANE_ROWS`]. Over-cap request = fill fails
/// fail-dominant (caller bumps R1_PLANE_FILL_FAILS, routes vanilla tick).
pub const PLANE_MIN_CAP: usize = 4096;

pub fn plane_cap(n: usize) -> usize {
    if n == 0 {
        return PLANE_MIN_CAP;
    }
    let mut cap = PLANE_MIN_CAP;
    while cap < n && cap < MAX_PLANE_ROWS {
        cap <<= 1;
    }
    cap
}

// ---------------------------------------------------------------------------
// Counters — extended monotonic fail-dominant family (G6: never reset; the
// arm story is readable from stdout/artifact after any soak). The C07
// skeleton owns the 4 native-level counters; this leg adds the WIRING-level
// family, most importantly the vanilla-fallback REASON breakdown — the
// artifact must answer WHY a query went vanilla, not just that it did.
// ---------------------------------------------------------------------------

/// Vanilla-fallback reason (exhaustive; every non-bulk query lands in
/// exactly one bucket — exhaustiveness is compiler- + test-pinned).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaReason {
    /// Gate off (lever/arm mismatch) — wiring problem, not a traffic story.
    GateOff,
    /// Cold plane (rows == 0 or structuralVersion == COLD).
    ColdPlane,
    /// THRESH estimate ≤ 512 → vanilla is cheaper by preregister.
    Thresh,
    /// Structural drift between fill and query (version/array-shape).
    Structural,
    /// Rust native returned ERR_* (fail-dominant by contract).
    RustErr,
}

impl VanillaReason {
    /// Counter slot (stable artifact order; NOT a HashMap — parity).
    pub fn slot(self) -> usize {
        match self {
            VanillaReason::GateOff => 0,
            VanillaReason::ColdPlane => 1,
            VanillaReason::Thresh => 2,
            VanillaReason::Structural => 3,
            VanillaReason::RustErr => 4,
        }
    }
}

const R1_SLOTS: usize = 5;

static R1_PLANE_FILLS: AtomicI64 = AtomicI64::new(0);
static R1_PLANE_FILL_FAILS: AtomicI64 = AtomicI64::new(0);
static R1_RETARGET_ATTEMPTS: AtomicI64 = AtomicI64::new(0);
static R1_RETARGET_OK: AtomicI64 = AtomicI64::new(0);
static R1_VANILLA: [AtomicI64; R1_SLOTS] = [
    AtomicI64::new(0),
    AtomicI64::new(0),
    AtomicI64::new(0),
    AtomicI64::new(0),
    AtomicI64::new(0),
];
static R1_CANDIDATES_EM: AtomicI64 = AtomicI64::new(0);
static R1_LIVE_VALIDATED: AtomicI64 = AtomicI64::new(0);
static R1_LIVE_DROPPED: AtomicI64 = AtomicI64::new(0);

/// Single entry point for vanilla-routing bookkeeping (fail-dominant canon:
/// one bump, one bucket, no silent paths).
pub fn note_vanilla(reason: VanillaReason) {
    R1_VANILLA[reason.slot()].fetch_add(1, Ordering::Relaxed);
}

pub fn note_plane_fill(ok: bool) {
    let c = if ok { &R1_PLANE_FILLS } else { &R1_PLANE_FILL_FAILS };
    c.fetch_add(1, Ordering::Relaxed);
}

pub fn note_retarget(ok: bool) {
    R1_RETARGET_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
    if ok {
        R1_RETARGET_OK.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn note_candidates(emitted: i64, validated: i64, dropped: i64) {
    R1_CANDIDATES_EM.fetch_add(emitted, Ordering::Relaxed);
    R1_LIVE_VALIDATED.fetch_add(validated, Ordering::Relaxed);
    R1_LIVE_DROPPED.fetch_add(dropped, Ordering::Relaxed);
}

/// Counter readback for selftest/artifact (monotonic, never reset).
/// Order: [0] plane_fills, [1] plane_fill_fails, [2] retarget_attempts,
/// [3] retarget_ok, [4..9] vanilla{gate_off,cold,thresh,structural,rust_err},
/// [9] candidates_emitted, [10] live_validated, [11] live_dropped.
pub fn r1_stats() -> [i64; 12] {
    [
        R1_PLANE_FILLS.load(Ordering::Relaxed),
        R1_PLANE_FILL_FAILS.load(Ordering::Relaxed),
        R1_RETARGET_ATTEMPTS.load(Ordering::Relaxed),
        R1_RETARGET_OK.load(Ordering::Relaxed),
        R1_VANILLA[0].load(Ordering::Relaxed),
        R1_VANILLA[1].load(Ordering::Relaxed),
        R1_VANILLA[2].load(Ordering::Relaxed),
        R1_VANILLA[3].load(Ordering::Relaxed),
        R1_VANILLA[4].load(Ordering::Relaxed),
        R1_CANDIDATES_EM.load(Ordering::Relaxed),
        R1_LIVE_VALIDATED.load(Ordering::Relaxed),
        R1_LIVE_DROPPED.load(Ordering::Relaxed),
    ]
}

// ---------------------------------------------------------------------------
// Activation: CONTRACT-DORMANT. Nothing here touches the JVM; the arm note
// fires ONLY under the R1 STRICT gate so vanilla runs stay byte-quiet.
// ---------------------------------------------------------------------------

pub fn activate() {
    if !r1_enabled() {
        return;
    }
    eprintln!(
        "[crussty-plugin] sb_r1: R1-CONTRACT DORMANT (lever {R1_LEVER_FLAG}, THRESH={SB_THRESH}, MARGIN={SB_MARGIN}, OUT_CAP_MAX={OUT_CAP_MAX}, MAX_PLANE_ROWS={MAX_PLANE_ROWS}, R2 capture<{R2_CAPTURE_MIN}) — types/counters/boundaries compiled, 0 natives/0 define/0 retarget this tick (wiring = CI-доба next tick, after C01 base-rep arbiter; Л-486-C07 plan)"
    );
}

// ---------------------------------------------------------------------------
// Tests (offline harness layer 1: contract parity + fail-dominant pins).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- gate ----
    #[test]
    fn r1_gate_is_strict_eq_and_independent() {
        assert_eq!(R1_LEVER_FLAG, "cmp487_sbulk1");
        assert_eq!(R1_ARM_ENV, "CRUSSTY_SBLK_R1");
        // marker alone never arms; foreign markers never arm; success pair.
        assert!(!r1_enabled_with(Some("cmp487_sbulk1"), None));
        assert!(!r1_enabled_with(Some("cmp486_sbulk1"), Some("1")));
        assert!(!r1_enabled_with(None, Some("1")));
        assert!(r1_enabled_with(Some("cmp487_sbulk1"), Some("1")));
        // independence from the C07 skeleton gate: sibling lever id differs.
        assert_ne!(R1_LEVER_FLAG, crate::selector_bulk::LEVER_FLAG);
    }

    // ---- boundary parity vs C07 skeleton ----
    #[test]
    fn boundaries_match_c07_skeleton_and_java_contract() {
        assert_eq!(SB_THRESH, 512);
        assert_eq!(SB_THRESH, crate::selector_bulk::SB_THRESH);
        assert_eq!(SB_MARGIN, 4.0);
        assert_eq!(SB_MARGIN, crate::selector_bulk::SB_MARGIN);
        assert_eq!(ERR_STRUCT, -1);
        assert_eq!(ERR_RANGE, -2);
        assert_eq!(OUT_CAP_MAX, 1 << 16); // java new int[min(rows, 1<<16)]
        assert_eq!(STRUCT_VERSION_COLD, 0);
        assert_eq!(R2_CAPTURE_MIN, 0.55);
        assert!(MAX_PLANE_ROWS >= OUT_CAP_MAX);
        assert!(HIST_CELL > 0.0 && HIST_MAX_SIDE >= 1);
    }

    // ---- native sig table (TASK-409-E: exact strings) ----
    #[test]
    fn native_sigs_are_exact() {
        assert_eq!(SIG_PROBE.name, "sbProbe");
        assert_eq!(SIG_PROBE.desc, "()I");
        assert_eq!(SIG_ENUMERATE.name, "sbEnumerate");
        // ([D[D I DDDD III [I)I — 3 int scalars (typeQ, limit, structuralFresh).
        assert_eq!(SIG_ENUMERATE.desc, "([D[D[IDDDDIII[I)I");
        assert_eq!(SIG_STATS.name, "sbStats");
        assert_eq!(SIG_STATS.desc, "([J)I");
        // Plane columns pin their JNI array types.
        assert_eq!(PlaneCol::Ids.jni_arr(), "[J");
        assert_eq!(PlaneCol::Xs.jni_arr(), "[D");
        assert_eq!(PlaneCol::Zs.jni_arr(), "[D");
        assert_eq!(PlaneCol::TypeOrd.jni_arr(), "[I");
        assert_eq!(PlaneCol::SectionOrder.jni_arr(), "[I");
    }

    // ---- retarget specs (CP-EXACT pins) + R2 capture predicate ----
    #[test]
    fn retarget_specs_and_r2_capture_gate() {
        assert_eq!(RETARGET_R1.from_class, "net/minecraft/world/entity/selector/EntitySelector");
        assert_eq!(RETARGET_R1.from_method, "getEntities");
        assert_eq!(
            RETARGET_R1.from_desc,
            "(Lnet/minecraft/world/entity/EntitySelector;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;"
        );
        assert_eq!(RETARGET_R1.to_method, "getEntitiesGated");
        // x494 РЕПИН: phantom moonrise R2 replaced by verified LevelEntityGetter trio
        for spec in [&RETARGET_R2_T, &RETARGET_R2_E, &RETARGET_R2_C, &RETARGET_R1_FUNNEL] {
            assert_eq!(spec.to_class, RETARGET_R1.to_class);
            assert_eq!(spec.to_method, "getEntitiesGated");
            assert!(spec.from_desc.starts_with("(Lnet/minecraft/"));
        }
        assert_eq!(RETARGET_R2_T.from_class, "net/minecraft/world/level/entity/LevelEntityGetter");
        assert_eq!(RETARGET_R2_E.from_method, "get");
        assert!(RETARGET_R1_FUNNEL.from_desc.ends_with("Ljava/util/List;I)V"));
        // R2 opens ONLY below 55% measured capture; boundary excluded.
        assert!(!r2_allowed(0.55));
        assert!(!r2_allowed(0.60));
        assert!(r2_allowed(0.5499));
        assert!(!r2_allowed(f64::NAN)); // doubt = no R2
    }

    // ---- phase machine (monotone, fail-hold, no skip) ----
    #[test]
    fn phase_machine_is_monotone_and_fail_dominant() {
        let mut p = Phase::Dormant;
        p = p.advance(false);
        assert_eq!(p, Phase::Dormant); // fail holds
        p = p.advance(true);
        assert_eq!(p, Phase::Defined);
        p = p.advance(false);
        assert_eq!(p, Phase::Defined); // retarget failure holds at Defined
        p = p.advance(true);
        assert_eq!(p, Phase::Retargeted);
        p = p.advance(true);
        assert_eq!(p, Phase::Armed);
        p = p.advance(true);
        assert_eq!(p, Phase::Armed); // saturates
        assert_eq!(p.disarm_all(), Phase::Dormant); // explicit kill-switch only
        // ordering is a total order (Ord derive pin).
        assert!(Phase::Dormant < Phase::Defined && Phase::Retargeted < Phase::Armed);
    }

    // ---- THRESH estimate over the fill histogram ----
    #[test]
    fn fill_estimate_is_cell_exact_and_fail_dominant() {
        // 2×2 grid of HIST_CELL=64: origin (0,0); cells cover [0,64)².
        let rep = SbFillReport {
            rows: 10,
            structural_version: 1,
            min_px: 0.0,
            min_pz: 0.0,
            hist_side: 2,
            // cell layout: (0,0)=3 (1,0)=4 (0,1)=1 (1,1)=2 — total 10 = rows
            hist: vec![3, 4, 1, 2],
        };
        // Box fully inside cell (0,0), no margin spill: [1,2]×[1,2] → 3.
        assert_eq!(rep.estimate_candidates(1.0, 2.0, 1.0, 2.0), 3);
        // Margin (4) expansion of [60,61]×[60,61] → [56,65]×[56,65] spills
        // into cells (1,0),(0,1),(1,1): 3+4+1+2 = 10 — wait: [56,65] touches
        // cell 0 ([0,64)) AND cell 1 ([64,128)) on both axes → all 4 cells.
        assert_eq!(rep.estimate_candidates(60.0, 61.0, 60.0, 61.0), 10);
        // Far-away box outside the grid → 0 (correct lower bound).
        assert_eq!(rep.estimate_candidates(1e6, 1e6 + 1.0, 1e6, 1e6 + 1.0), 0);
        // Degenerate/non-finite → 0 ⇒ ≤ THRESH ⇒ vanilla (fail-dominant).
        assert_eq!(rep.estimate_candidates(f64::NAN, 2.0, 1.0, 2.0), 0);
        assert_eq!(rep.estimate_candidates(2.0, 1.0, 1.0, 2.0), 0);
        let cold = SbFillReport::default();
        assert_eq!(cold.estimate_candidates(0.0, 1.0, 0.0, 1.0), 0);
        // Freshness: cold never fresh; versions strictly increase.
        assert!(!cold.fresh(0));
        assert!(rep.fresh(0));
        assert!(!rep.fresh(1)); // no reuse — strictly monotonic
    }

    // ---- type ordinals + plane cap ----
    #[test]
    fn ordinals_dense_sorted_and_cap_pow2_clamped() {
        assert_eq!(type_ordinals(&["a", "b", "c"]), vec![1, 2, 3]);
        assert!(type_ordinals(&[]).is_empty());
        assert_eq!(plane_cap(0), PLANE_MIN_CAP);
        assert_eq!(plane_cap(1), PLANE_MIN_CAP);
        assert_eq!(plane_cap(4097), 8192);
        assert_eq!(plane_cap(1 << 21), MAX_PLANE_ROWS);
        assert_eq!(plane_cap(usize::MAX), MAX_PLANE_ROWS); // over-cap → clamp
    }

    // ---- counters: monotonic + exhaustive reason buckets ----
    #[test]
    fn counters_monotonic_and_reasons_exhaustive() {
        let before = r1_stats();
        note_plane_fill(true);
        note_plane_fill(false);
        note_retarget(true);
        note_retarget(false);
        for r in [
            VanillaReason::GateOff,
            VanillaReason::ColdPlane,
            VanillaReason::Thresh,
            VanillaReason::Structural,
            VanillaReason::RustErr,
        ] {
            note_vanilla(r);
        }
        note_candidates(7, 5, 2);
        let after = r1_stats();
        assert!(after.len() == 12);
        for i in 0..12 {
            assert!(after[i] >= before[i], "G6: counter {i} regressed");
        }
        // exact single-bump deltas
        assert_eq!(after[0] - before[0], 1); // fills ok
        assert_eq!(after[1] - before[1], 1); // fills fail
        assert_eq!(after[2] - before[2], 2); // retarget attempts
        assert_eq!(after[3] - before[3], 1); // retarget ok
        for i in 4..9 {
            assert_eq!(after[i] - before[i], 1, "reason slot {i} bumped once");
        }
        assert_eq!(after[9] - before[9], 7);
        assert_eq!(after[10] - before[10], 5);
        assert_eq!(after[11] - before[11], 2);
        // validated + dropped == emitted (conservation pin)
        assert_eq!(after[10] + after[11], after[9]);
    }
}
