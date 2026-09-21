//! Runtime wiring for the EVENT-DRIVEN AI WAKEUP (TASK-401-G, lever flag
//! "cmp401_wakeup"). Extension of the TASK-399-I precedent
//! (branch round-399-i-wakeup, 4 goal-selector sites) to NINE LDC(W)-anchored
//! call sites in `net.minecraft.world.entity.Mob.serverAiStep`:
//!
//!   2x GoalSelector.tickRunningGoals(Z) -> AiWakeupOps.tickRunning(Mob,sel,Z)
//!   2x GoalSelector.tick()              -> AiWakeupOps.tickSel(Mob,sel)
//!   1x Sensing.tick()                   -> AiWakeupOps.tickSensing(Mob,sensing)
//!   1x PathNavigation.tick()            -> AiWakeupOps.tickNav(Mob,nav)
//!   1x MoveControl.tick()               -> AiWakeupOps.tickMove(Mob,ctl)
//!   1x LookControl.tick()               -> AiWakeupOps.tickLook(Mob,ctl)
//!   1x JumpControl.tick()               -> AiWakeupOps.tickJump(Mob,ctl)
//!
//! (receiver-prepended statics, verifier-visible stack shape unchanged — no
//! code shift, no StackMapTable edit). customServerAiStep ("mob tick", Brain)
//! and the noActionTime despawn path are NOT in scope and stay vanilla.
//!
//! BOOT-NPE PRECEDENT FIX (TASK-399-I, 4140600): NO force-load of Mob through
//! the Bukkit-seeded loader — Entity.<clinit> in a foreign loader context left
//! Entity.scriptEngine null forever -> PurpurWorldConfig NPE at boot. Mob is
//! sighted naturally during server-init (brainhook/items_manager precedent);
//! this module passive-polls `is_sighted` only.
//!
//! Fail-closed chain: lever flag mismatch -> dormant; Mob not loaded within
//! the deadline -> dormant; define_class(AiWakeupOps) failure -> READY never
//! set -> vanilla Mob; ANY of the 9 retarget site counts violated -> original
//! bytes served (vanilla). The retargeted bytes cannot reach the JVM unless
//! the helper class is already defined in the Mob loader, so the invokestatic
//! resolution cannot fail at runtime.
//!
//! Marker chain (hook convention): defined -> hook armed, retransform rc ->
//! PATCHED -> "[crussty-plugin] cmp401_wakeup: ARMED ...".

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/AiWakeupOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/AiWakeupOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// STRICT eq() on THIS lever's flag only (TASK-400-D gate-lesson: never widen
/// to a flag family — a half-armed sibling bridge AIOOBE(-1,16385) killed two
/// round legs).
fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("cmp401_wakeup"))
        .unwrap_or(false)
}

type Ref3 = (&'static str, &'static str, &'static str);

/// The 9-site wakeup chain: (anchors, from, to, want-sites). Any violation of
/// a site count aborts the whole lever (fail-closed vanilla).
fn wakeup_chain() -> [(&'static [&'static str], Ref3, Ref3, usize); 7] {
    [
        (
            &classfile::WAKEUP_ANCHORS,
            classfile::WAKEUP_FROM_TICK,
            classfile::WAKEUP_TO_TICK,
            2,
        ),
        (
            &classfile::WAKEUP_ANCHORS,
            classfile::WAKEUP_FROM_RUNNING,
            classfile::WAKEUP_TO_RUNNING,
            2,
        ),
        (
            &["sensing"],
            classfile::WAKEUP_FROM_SENSING,
            classfile::WAKEUP_TO_SENSING,
            1,
        ),
        (
            &["navigation"],
            classfile::WAKEUP_FROM_NAV,
            classfile::WAKEUP_TO_NAV,
            1,
        ),
        (&["move"], classfile::WAKEUP_FROM_MOVE, classfile::WAKEUP_TO_MOVE, 1),
        (&["look"], classfile::WAKEUP_FROM_LOOK, classfile::WAKEUP_TO_LOOK, 1),
        (&["jump"], classfile::WAKEUP_FROM_JUMP, classfile::WAKEUP_TO_JUMP, 1),
    ]
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp401_wakeup: dormant (set CRUSSTY_LEVER_FLAG=cmp401_wakeup to enable)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOB_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match wakeup_patch_chain(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] cmp401_wakeup: retargeted {name} serverAiStep 9 sites -> AiWakeupOps ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] cmp401_wakeup: retarget failed: {e}");
                None
            }
        }
    });
}

/// Apply the 9-site wakeup retarget chain on the hook-delivered Mob bytes.
/// Any site-count violation aborts the whole lever (original bytes stay
/// vanilla — fail-closed).
fn wakeup_patch_chain(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut cur: Vec<u8> = bytes.to_vec();
    for (anchors, from, to, want) in wakeup_chain() {
        let mut left = want;
        while left > 0 {
            let (b, outcome) = classfile::retarget_ldcw_virtual_to_static(
                &cur,
                classfile::WAKEUP_METHOD,
                classfile::WAKEUP_METHOD_DESC,
                anchors,
                from,
                to,
            )?;
            match outcome {
                classfile::RetargetOutcome::Retargeted { .. } => {
                    cur = b;
                    left -= 1;
                }
                classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    // Partial/re-served state: remaining sites count as done
                    // (idempotency guarded by the PATCHED swap in register()).
                    left = 0;
                }
                classfile::RetargetOutcome::NotFound => {
                    return Err(format!(
                        "wakeup: {}/{}/{} site missing in {}{} (want {} more)",
                        from.0,
                        from.1,
                        from.2,
                        classfile::WAKEUP_METHOD,
                        classfile::WAKEUP_METHOD_DESC,
                        left
                    ));
                }
            }
        }
    }
    Ok(cur)
}

/// Background activation: wait for the kernel Mob (passive sighting poll —
/// the TASK-399-I boot-NPE precedent FORBIDS force-load through the Bukkit
/// loader), define AiWakeupOps into its loader, flip READY and retransform so
/// the hook applies the 9-site retarget chain.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(MOB_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp401_wakeup: {MOB_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // PASSIVE POLL ONLY — never force_load_mob() (boot-NPE precedent,
            // run 35556475043): Mob sightings happen naturally during
            // server-init (brainhook / items_manager precedents).
            let sighted = cplug_sdk::classes::is_sighted(MOB_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                5_000
            }));
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(MOB_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
                .or_else(|| {
                    crate::clear_exception(env);
                    env.delete_local_ref(class_cls);
                    None
                })
            else {
                return false;
            };
            let Some(c) = env.define_class(OPS_CLASS, loader, OPS_BYTES) else {
                crate::clear_exception(env);
                eprintln!("[crussty-plugin] cmp401_wakeup: define_class({OPS_CLASS}) failed");
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            eprintln!("[crussty-plugin] cmp401_wakeup: defined {OPS_CLASS} in Mob loader");
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] cmp401_wakeup: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!("[crussty-plugin] cmp401_wakeup: hook armed, retransform rc={rc}");
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final one-line acceptance marker (TASK-22/C1 convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp401_wakeup: ARMED (Mob.serverAiStep 9 sites -> AiWakeupOps: 2x tickSel + 2x tickRunningGoals + sensing/nav/move/look/jump skips; sleep when stationary far from players, wake on hurt/setTarget/moveTo/player<=16b or <=40t id-hash timer)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp401_wakeup: NOT APPLIED after retransform (see retarget-failed line above; kernel build mismatch?)"
            );
        }
    });
}
