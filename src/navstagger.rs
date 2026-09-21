//! Runtime wiring for the NAVSTAGGER lever (TASK-399-C, mega-round-3,
//! agent C — vector cmp399_navstag: N=4-phase goal re-selection scheduler).
//!
//! The [`crate::brainhook`] pattern applied to `net.minecraft.world.entity.Mob`:
//!
//! 1. Hook: once the kernel `Mob` is loaded we flip READY and retransform;
//!    the byte hook fires on the retransform and applies the ONE-BYTE
//!    `Mob.serverAiStep` patch ([`crate::classfile::patch_mob_stagger`] —
//!    `iconst_2 -> iconst_4` in the vanilla id-based goal-selection gate).
//!
//!    Vanilla 1.21.2+ already staggers goal re-selection 2-phase by id:
//!    `i = tickCount + getId(); if (i % 2 != 0 && tickCount > 1)` ->
//!    `targetSelector/goalSelector.tickRunningGoals(false)` (running goals
//!    execute — movement/attack continuity), else FULL `tick()` on both
//!    selectors (goalCleanup + goalUpdate). Widening the modulus to 4 makes
//!    the FULL re-selection run for exactly the (tickCount + id) % 4 == 0
//!    slot (~25% of mobs per tick, deterministic round-robin by id, NOT by
//!    wall-time); the other 75% keep tickRunningGoals(false) every tick.
//!    navigation.tick / customServerAiStep / controls are OUTSIDE the branch
//!    and stay every-tick vanilla — movement/attack/spawn parity by
//!    construction. Superiority deviation: goal re-evaluation (incl.
//!    goalCleanup stop()) for a cold mob is delayed by up to 4 ticks
//!    (vanilla bound: 2) — documented in LEVER-V399-C.md.
//!
//!    No bridge class, no natives: the phase machine and the every-tick
//!    running-goal path already exist in the kernel; the lever changes only
//!    N. Fail-closed: pattern mismatch -> Err -> hook serves pristine bytes
//!    -> vanilla path.
//!
//! 2. Arm gate: CRUSSTY_LEVER_FLAG=cmp399_navstag AND CRUSSTY_REGION_THREADS>=2
//!    (bank=4). Markers (server-stdout, grep-able):
//!    "[crussty-plugin] cmp399_navstag: ARMED N=4 ..." and
//!    "[crussty-plugin] cmp399_navstag: PATCHED Mob.serverAiStep ...".

use crate::classfile;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MOB_CLASS: &str = classfile::MOB_CLASS;

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("cmp399_navstag"))
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(MOB_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match classfile::patch_mob_stagger(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] cmp399_navstag: PATCHED Mob.serverAiStep iconst_2->iconst_4 (N=4 goal re-selection slot) ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!(
                    "[crussty-plugin] cmp399_navstag: patch failed: {e} — hook stays dormant (vanilla path)"
                );
                None
            }
        }
    });
}

/// Background activation: wait for the kernel Mob, flip READY and retransform
/// so the hook applies the one-byte modulus patch. Mob loads with the first
/// entity-type bootstrap on every world boot, so the poll loop is a latency
/// safety net, not a liveness requirement.
pub fn activate() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp399_navstag: dormant (set CRUSSTY_LEVER_FLAG=cmp399_navstag to enable)"
        );
        return;
    }
    let workers = crate::region_threads::workers_from_env_pub();
    if workers.is_none() {
        eprintln!(
            "[crussty-plugin] cmp399_navstag: requires CRUSSTY_REGION_THREADS>=2 (AI runs on region workers), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(MOB_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp399_navstag: {MOB_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            let sighted = cplug_sdk::classes::is_sighted(MOB_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!(
            "[crussty-plugin] cmp399_navstag: ARMED N=4 (mob goal re-selection slot (tickCount+getId())%4==0; tickRunningGoals every tick; region_workers={:?}; retransform rc={rc})",
            workers
        );
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final acceptance line (TASK-22/C1 marker convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp399_navstag: applied (Mob.serverAiStep N=2->N=4; superiority: goal re-evaluation delay <=4 ticks, LEVER-V399-C.md)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp399_navstag: NOT APPLIED after retransform (see patch-failed line above; kernel build mismatch?)"
            );
        }
    });
}
