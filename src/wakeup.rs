//! Runtime wiring for the EVENT-DRIVEN AI WAKEUP-LIST (TASK-399-I, lever
//! flag "cmp399_wakeup").
//!
//! The [`crate::brainhook`] pattern applied to `net.minecraft.world.entity.Mob`:
//!
//! 1. Hook: once the kernel's `Mob` is loaded we define the helper
//!    (`AiWakeupOps`, ECJ-compiled against the kernel's class shapes,
//!    `entityinside/net/minecraft/world/entity/AiWakeupOps.java`) into the SAME
//!    loader, then `READY` flips and `Mob` is retransformed. The byte hook
//!    fires on the retransform (or on a late original load) and retargets the
//!    FOUR LDC(W)-anchored GoalSelector call sites inside `serverAiStep`
//!    ([`crate::classfile::retarget_ldcw_virtual_to_static`], strict 2+2):
//!    `GoalSelector.tick()` -> `invokestatic AiWakeupOps.tickSel(Mob,GoalSelector)V`
//!    and `GoalSelector.tickRunningGoals(Z)` ->
//!    `AiWakeupOps.tickRunning(Mob,GoalSelector,Z)V` (receiver-prepended
//!    statics, verifier-visible stack shape unchanged — no code shift, no
//!    StackMapTable edit).
//!
//! 2. Semantics: see AiWakeupOps.java — stationary mobs (no running MOVE/JUMP
//!    goal, no running target goal, no path, no target, no recent hurt)
//!    drop out of the goal cycles until a wake event (hurt / setTarget /
//!    external moveTo — all polled O(1) from VANILLA fields, zero new patch
//!    points) or the deterministic id-hash round-robin timer (<= 40 ticks)
//!    fires. Active/aggro mobs tick vanilla 100% of the time. sensing /
//!    navigation / customServerAiStep (Brain mobs) / controls / noActionTime
//!    (despawn) are NOT in the retarget scope and stay vanilla.
//!
//! 3. Fail-closed: lever flag mismatch -> dormant; Mob not loaded within the
//!    deadline -> dormant; define_class(AiWakeupOps) failure -> READY never
//!    set -> vanilla Mob; retarget site-count violation -> original bytes
//!    served (vanilla). The retargeted bytes cannot reach the JVM unless the
//!    helper class is already defined in the Mob loader, so the invokestatic
//!    resolution cannot fail at runtime.
//!
//! Marker chain (hook convention): defined -> hook armed, retransform rc ->
//! PATCHED -> "[crussty-plugin] cmp399_wakeup: ARMED ...".

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/AiWakeupOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/AiWakeupOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("cmp399_wakeup"))
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp399_wakeup: dormant (set CRUSSTY_LEVER_FLAG=cmp399_wakeup to enable)"
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
                    "[crussty-plugin] cmp399_wakeup: retargeted {name} serverAiStep 2x tick + 2x tickRunningGoals -> AiWakeupOps ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] cmp399_wakeup: retarget failed: {e}");
                None
            }
        }
    });
}

/// Apply the strict 2x tick + 2x tickRunningGoals wakeup retarget chain on the
/// hook-delivered Mob bytes. Any site-count violation aborts the whole lever
/// (original bytes stay vanilla — fail-closed).
fn wakeup_patch_chain(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut cur: Vec<u8> = bytes.to_vec();
    for (from, to, want) in [
        (
            classfile::WAKEUP_FROM_TICK,
            classfile::WAKEUP_TO_TICK,
            2usize,
        ),
        (
            classfile::WAKEUP_FROM_RUNNING,
            classfile::WAKEUP_TO_RUNNING,
            2usize,
        ),
    ] {
        let mut left = want;
        while left > 0 {
            let (b, outcome) = classfile::retarget_ldcw_virtual_to_static(
                &cur,
                classfile::WAKEUP_METHOD,
                classfile::WAKEUP_METHOD_DESC,
                &classfile::WAKEUP_ANCHORS,
                from,
                to,
            )?;
            match outcome {
                classfile::RetargetOutcome::Retargeted { .. } => {
                    cur = b;
                    left -= 1;
                }
                classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    // Partial/re-served state: treat remaining sites as done
                    // (idempotency is guarded by the PATCHED swap above).
                    left = 0;
                }
                classfile::RetargetOutcome::NotFound => {
                    return Err(format!(
                        "wakeup: {}/{} site missing in {}{} (want {} more)",
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

/// Background activation: wait for the kernel Mob, define AiWakeupOps into its
/// loader, flip READY and retransform so the hook applies the 4-site retarget.
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
                    "[crussty-plugin] cmp399_wakeup: {MOB_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // TASK-399 FIX (верхний агент, boot-NPE root-cause): force_load_mob()
            // форсил Entity.<clinit> через Bukkit PluginClassLoader ДО инициализации
            // миров — clinit звал getEngineByName("rhino") в loader-контексте без
            // rhino ⇒ Entity.scriptEngine = null навсегда ⇒ PurpurWorldConfig.
            // skeletonSettings NPE на boot (run 35556475043). Mob sightings
            // происходят натурально во время server-init (прецеденты brainhook/
            // items_manager) — пасивный полл достаточен, force-load удалён.
            // force_load_mob();
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
                else {
                    crate::clear_exception(env);
                    env.delete_local_ref(class_cls);
                    return false;
                };
            let Some(c) = env.define_class(OPS_CLASS, loader, OPS_BYTES) else {
                crate::clear_exception(env);
                eprintln!("[crussty-plugin] cmp399_wakeup: define_class({OPS_CLASS}) failed");
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            eprintln!("[crussty-plugin] cmp399_wakeup: defined {OPS_CLASS} in Mob loader");
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] cmp399_wakeup: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!("[crussty-plugin] cmp399_wakeup: hook armed, retransform rc={rc}");
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final one-line acceptance marker (TASK-22/C1 convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp399_wakeup: ARMED (Mob.serverAiStep 2x tick + 2x tickRunningGoals -> AiWakeupOps; wakeup-list: sleep when stationary, wake on hurt/setTarget/moveTo or <=40t id-hash timer)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp399_wakeup: NOT APPLIED after retransform (see retarget-failed line above; kernel build mismatch?)"
            );
        }
    });
}

/// Force-load Mob through the kernel loader (Bukkit-seeded forName) — the
/// brainhook::force_load_brain pattern verbatim with a different target name.
fn force_load_mob() {
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        let Some(seed) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
            return None::<()>;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return None::<()>;
        };
        let Some(loader) = env
            .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
            .and_then(|mid| {
                let l = env.call_object_method(seed.as_jclass(), mid, &[]);
                (l as usize != 0).then_some(l)
            })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return None::<()>;
            };
        let Some(forname) = env.get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        ) else {
            crate::clear_exception(env);
            return None::<()>;
        };
        let dot = MOB_CLASS.replace('/', ".");
        let Some(name) = env.new_string(&dot) else {
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jni::jvalue { l: name },
                jni::jvalue { z: 1 },
                jni::jvalue { l: loader },
            ],
        );
        let _ = crate::clear_exception(env);
        if loaded as usize != 0 {
            env.delete_local_ref(loaded);
            eprintln!("[crussty-plugin] cmp399_wakeup: force load: {dot} loaded");
        }
        env.delete_local_ref(name);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(())
    });
}
