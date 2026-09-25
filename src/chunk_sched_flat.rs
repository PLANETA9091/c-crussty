//! Scaffold of the CHUNK-TICK ELIGIBILITY FLAT plane (TASK-459-59, WILD law-11,
//! tick-459; idea ID-P22 — RESEARCH-459-P22.md; axis chunk/worldgen).
//!
//! Plane contract: the java bridge collects FLAT chunk-tick eligibility
//! predicates for one tick (packed chunkXZ keys, dist/flags, playerXZ
//! proximity) into primitive arrays, makes ONE bulk JNI crossing into this
//! library, and receives an eligible-bitmask (9216 chunks = 144 longs =
//! 1152 bytes = one JNI block). Java then runs the STRICT tail in VANILLA
//! order (Moonrise ReferenceList raw-order of
//! ServerChunkCache.iterateTickingChunksFaster / ChunkMap.collectSpawningChunks):
//! the mask is a PRE-gate only — bit=1 (or unknown) always re-checks the
//! vanilla predicate in the tail; a bit=0 skip is allowed only for EXACT bits.
//! v1 scope = distance-gate ONLY (B5 spawnable = playerTicking chunks, B6
//! hasAnyNearbyNarrow area-map bit — both exact per kernel javap K6/K7);
//! one-sided superset bits B2/B3 (random-tick sections / fluid) are NOT in v1
//! (false-positive price = parity break).
//!
//! Kernel ground truth (RESEARCH-459-L02 §3, javap patched-kernel.jar):
//!   K2 ServerChunkCache.moonrise$setFullChunk(IILevelChunk)V — the SINGLE
//!      fullChunks mutation point = bit-exact mask feed point;
//!   K3 iterateTickingChunksFaster()V — flat raw-array iteration + mid-tick
//!      tasks every 8 (i&7) — order/index density preserved by the tail;
//!   K6 ChunkMap.collectSpawningChunks — idle playerTicking pass eaten by B5;
//!   K7 DistanceManager implements ChunkTickDistanceManager — O(1)
//!      moonrise$hasAnyNearbyNarrow(II)Z per chunk feeds B6.
//! Web ground truth: Moonrise README (chunk system rewrite / random ticking
//! optimisations, "without changing Vanilla behavior"),
//! MC-310372 (cross-chunk tick processing order is parity-critical ⇒ the tail
//! must not reorder), minecraft.wiki/w/Tick (1.21.5 25w06a removed the legacy
//! 128-block random-tick gate ⇒ the v1 distance gate mirrors the KERNEL
//! predicates, not legacy-128).
//!
//! Δ math: scheduling lane 4.6-5.2% of CPU (post-C2 residual 4.3-4.9%),
//! capture 25-40% → +1.1…+2.0pp (center +1.4pp); ceiling +4.6pp ⇒ stack
//! layer of the chk-14 climb (with P31/P32), never a standalone carrier.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG` == "cmp459_chunksched-mask" (STRICT eq;
//! empty/foreign flag = vanilla bit-in-bit). DORMANT by default: with the
//! flag off, register() logs a dormant notice and NO byte hook exists.
//!
//! NCDFE canon (round-459 discipline): ChunkSchedOps is define_class'd in the
//! EARLY arm hook (cplugin_init_impl → register(), before any retarget and
//! before the first tick) — a retargeted call site can never observe a
//! not-yet-defined bridge class. Dormant lever = class defined + unused
//! (dormant-invisible). Wiring stage replaces the placeholder blob below with
//! include_bytes!("../chunksched/build/net/minecraft/server/level/ChunkSchedOps.class")
//! and adds the selfTest oracle + feed hooks (chunk_send.rs pattern).
//!
//! Fail-closed matrix: empty blob (scaffold state) OR define failure OR
//! selfTest()!=true ⇒ BRIDGE_READY never set, no hook exists, loud WARN.
//!
//! JNI discipline (law 6): exactly ONE bulk crossing per tick
//! (ChunkSchedOps.evaluateEligibility — signature fixed in the java stub,
//! zero call sites in the scaffold), java tail does no native calls.
//!
//! ARM marker (wiring stage): "[crussty-plugin] cmp459_chunksched-mask: ARMED
//! chunk-tick eligibility flat plane". Grep marker: "cmp459_chunksched-mask".

const LEVER_ID: &str = "cmp459_chunksched-mask";
const OPS_CLASS: &str = "net/minecraft/server/level/ChunkSchedOps";

/// Scaffold placeholder: the compiled bridge blob lands at the wiring stage
/// (chunksched/build/... — ci.yml javac step builds it from the java stub).
/// Empty slice = fail-closed: even a set lever flag stays DORMANT until the
/// real classfile is wired (no define, no hook, vanilla bit-in-bit).
const OPS_BYTES: &[u8] = &[];

fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == LEVER_ID)
        .unwrap_or(false)
}

/// Early arm-hook entry (NCDFE canon): called from cplugin_init_impl BEFORE
/// any retarget exists and before the first tick. Scaffold behavior:
///   * lever off  → dormant notice, nothing defined, vanilla path untouched;
///   * lever on   → blob present? define + selfTest + ARMED (wiring stage);
///                  blob empty (scaffold) → fail-closed dormant + loud WARN.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla chunk-tick eligibility)"
        );
        return;
    }
    if OPS_BYTES.is_empty() {
        // Scaffold state: plane contracted, blob not wired yet. Fail-closed.
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: scaffold blob empty (ChunkSchedOps.class not wired) — hook stays dormant"
        );
        return;
    }
    // Wiring stage (TASK-459-59 follow-up): define_class(OPS_CLASS, OPS_BYTES)
    // -> static selfTest()Z oracle -> strict-tail byte hooks (K2 feed, K3/K6
    // pre-gate). See chunk_send.rs for the activation template.
    eprintln!(
        "[crussty-plugin] {LEVER_ID}: define_class({OPS_CLASS}) path reached — wiring stage pending"
    );
}
