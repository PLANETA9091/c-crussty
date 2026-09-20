//! TASK-396-F (MEGA-ROUND-1, vector F — lever_flag="items_mono"):
//! JIT-devirtualization of the item-entity tick lane.
//!
//! Architecture (RESEARCH-F.md — /home/z/rounds/ROUND-396/RESEARCH-F.md):
//! the ONLY megamorphic dispatch of the entity-tick loop is the single
//! `invokevirtual Entity.tick()V` inside `ServerLevel.tickNonPassenger`
//! (javap census, fixture e2992d63: exactly 1 site; ItemEntity ~70% of the
//! 150k-entity bench population, items lane 31.17% java = TOP-1). The lever
//! retargets that ONE instruction to the static
//! `RegionTickOps.entityTick(Entity)V` type-test split:
//!   instanceof ItemEntity -> monomorphic invokevirtual ItemEntity.tick
//!                            (C2 direct call + inline of the whole
//!                            tickNonPassenger -> entityTick -> item tick chain)
//!   else                  -> byte-identical vanilla virtual dispatch
//! Tick order, tick semantics and every surrounding vanilla/moonrise check
//! (lambda$tick$4 gates, guardEntityTick pump wrap, ensureTickThread,
//! currentlyTickingEntity, ActivationRange) are untouched — a pure
//! call-structure replacement, not a diet (JIT evidence: Shipilëv Quark #16,
//! Black Magic Method Dispatch, HotSpot inlining wiki — URLs in RESEARCH-F).
//!
//! This module is the FLAG OWNER + observability ONLY: the byte patch is
//! composed inside region_threads::activate (the single ServerLevel
//! patch-chain owner, strict Retargeted{1}), and the bridge body lives in
//! RegionTickOps — already defined into the kernel loader by region_threads,
//! so there are ZERO new bridge classes and ZERO NCDFE surface. The Java
//! side reads the same env at bridge static-init and prints the [S7-F]
//! ARMED marker (dual-sided audit trail in the bench log).
//!
//! Dormant-invisible: with CRUSSTY_LEVER_FLAG != "items_mono" the kernel
//! bytes stay bit-for-bit vanilla (the compose step is skipped entirely).

/// Lever gate: armed iff `CRUSSTY_LEVER_FLAG == "items_mono"` (exact match;
/// run_world3.sh:459 exports the world-bench-parallel `lever_flag` input
/// verbatim). The same env is re-parsed independently by the Java bridge
/// (RegionTickOps.parseItemsMono) — the two must agree by construction.
pub fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "items_mono")
        .unwrap_or(false)
}

/// Activation: observability only (the compose patch happens in
/// region_threads::activate, which consults enabled()). Called from lib.rs
/// before region_threads::activate so the log order reads arm -> compose.
pub fn activate() {
    if enabled() {
        eprintln!(
            "[crussty-plugin] items_mono: LEVER ARMED (CRUSSTY_LEVER_FLAG=items_mono, lever_arg={:?}) — \
             tickNonPassenger Entity.tick -> RegionTickOps.entityTick type-test split \
             (ItemEntity monomorphic lane)",
            std::env::var("CRUSSTY_LEVER_ARG").unwrap_or_default()
        );
    } else {
        eprintln!(
            "[crussty-plugin] items_mono: dormant (set CRUSSTY_LEVER_FLAG=items_mono to enable)"
        );
    }
}
