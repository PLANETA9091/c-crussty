//! Runtime wiring for the `cmp401_stagger` lever (TASK-401-I, round-401 vector I).
//!
//! STAGGERED HEAVY CHECKS: per-entity hashed 1/N staggering of per-tick
//! un-gated heavy checks, with a multi-rate vanilla invariant (visible checks
//! every tick, reference scans 1/N). Two lanes (javap ground truth on
//! purpur-1.21.10):
//!
//! 1. PUSH lane — `LivingEntity.pushEntities()V` performs an un-gated
//!    broadphase neighbor scan `Level.getPushableEntities(Entity,AABB)` EVERY
//!    tick for every LivingEntity (single site at offset 66-78). Retargeted to
//!    `PushStaggerOps.pushables(Level,Entity,AABB)List`: gate miss → immutable
//!    empty list (vanilla body exits via `list.isEmpty()`), hit → vanilla call.
//!    Scan phase = golden32(entityId) + server tickCount, N = 1/4 (env-tunable).
//!
//! 2. GOAL lane — `GoalSelector.tick()V` update phase polls
//!    `WrappedGoal.canUse()` for every non-running goal EVERY tick (single
//!    site at offset 166). Retargeted to
//!    `GoalStaggerOps.canUseGate(WrappedGoal)Z`: TargetGoal-family and
//!    Player/ServerPlayer-targeting NATG polls stay vanilla every tick
//!    (visible aggro); everything else (non-player NATG getEntitiesOfClass
//!    enumerations, stroll/remove-block/float) staggers to 1/N (miss → false,
//!    identical to a vanilla canUse()==false result).
//!
//! Both retargets are 3B→3B receiver-prepended invokestatic rewrites via
//! `classfile::retarget_virtual_to_static` (strict: exactly 1 site each,
//! anything else is fail-closed → vanilla bytes served, NOT ARMED).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp401_stagger"` (STRICT eq, round-400
//! lever protocol; полу-вооружённый мост AIOOBE урок TASK-400-D). Off by
//! default — dormant-invisible discipline: with the gate off no byte hook is
//! registered, nothing is defined or retransformed, the module is
//! byte-indistinguishable from the pre-TASK-401 plugin.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const GOALSEL_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalSelector";
const PUSH_OPS: &str = "net/minecraft/world/entity/PushStaggerOps";
const GOAL_OPS: &str = "net/minecraft/world/entity/ai/goal/target/GoalStaggerOps";
const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const WRAPPED_GOAL_CLASS: &str = "net/minecraft/world/entity/ai/goal/WrappedGoal";

const PUSH_BYTES: &[u8] =
    include_bytes!("../stagger/build/net/minecraft/world/entity/PushStaggerOps.class");
const GOAL_BYTES: &[u8] =
    include_bytes!("../stagger/build/net/minecraft/world/entity/ai/goal/target/GoalStaggerOps.class");

const GET_PUSHABLES_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";
const PUSH_STATIC_DESC: &str =
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;\
     Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";
const CANUSE_DESC: &str = "()Z";
const CANUSE_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z";

/// env gate per the round-400 lever protocol (STRICT eq — never starts_with /
/// contains: полу-вооружённый мост AIOOBE lesson TASK-400-D).
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp401_stagger") | Ok("cmp402_stagcomp") | Ok("cmp403_tickplane")
            | Ok("cmp405_stagtick")
            // TASK-406-D: композит раунда-406 — goal-стаггер (GoalSelector
            // canUseGate) активен вместе с новым AI-window срезом (мобы вне
            // ai-окна пропускают и goal-тик; мобы в окне — стаггернутые
            // canUse-поллы как в базе stagtick).
            | Ok("cmp406_aibatch")
            // TASK-406-E: композит раунда-406 — goal-стаггер (GoalSelector
            // canUseGate) активен вместе с новым despawn-scan срезом (мобы
            // вне ai-окна пропускают и goal-тик; despawn-скан батчится
            // отдельно через mobs_sscan, гейт cmp406_sscan).
            | Ok("cmp406_sscan")
            | Ok("cmp409_multi") | Ok("cmp412_meganav") | Ok("cmp412_b2p1")
    )
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// TASK-406-D: сигнал «LivingEntity retransform отработал» для mobs_ai —
/// AI-window хук компоузит поверх выходов цепочки хуков и должен
/// retransform'ить ПОСЛЕ stagger'а (последний non-None serve побеждает).
static LIVING_SERVED: AtomicBool = AtomicBool::new(false);

/// Pristine bytes per hooked class (captured at first load or via
/// resource-stream / no-op retransform fallback).
fn orig_lock(class_name: &str) -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    static LIVING: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
        std::sync::OnceLock::new();
    static GOALSEL: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
        std::sync::OnceLock::new();
    match class_name {
        GOALSEL_CLASS => GOALSEL.get_or_init(|| std::sync::Mutex::new(None)),
        _ => LIVING.get_or_init(|| std::sync::Mutex::new(None)),
    }
}

/// Register the byte hooks (idempotent; call once from cplugin_init).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] stagger: dormant (lever_flag != cmp401_stagger, vanilla push/goal checks)"
        );
        return;
    }
    for class_name in [LIVING_CLASS, GOALSEL_CLASS] {
        cplug_sdk::hooks::register_bytes(class_name, move |name, bytes| {
            if !READY.load(Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] stagger: pristine sighting {name} bytes (major {})",
                    crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
                );
                let mut orig = orig_lock(name).lock().unwrap_or_else(PoisonError::into_inner);
                if orig.is_none() {
                    *orig = Some(bytes.to_vec());
                }
                return None;
            }
            let cached = orig_lock(name)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            cached.map(|orig| match patch_for(name, &orig) {
                Ok(p) => p,
                Err(e) => {
                    // Fail-closed: serve pristine on any patch error.
                    eprintln!("[crussty-plugin] stagger: serve patch error for {name}: {e}, serving vanilla");
                    orig
                }
            })
        });
    }
}

/// Compute the retarget patch for a hooked class from pristine bytes.
fn patch_for(class_name: &str, orig: &[u8]) -> Result<Vec<u8>, String> {
    match class_name {
        LIVING_CLASS => {
            let (bytes, outcome) = crate::classfile::retarget_virtual_to_static(
                orig,
                "pushEntities",
                "()V",
                (LEVEL_CLASS, "getPushableEntities", GET_PUSHABLES_DESC),
                (PUSH_OPS, "pushables", PUSH_STATIC_DESC),
            )?;
            match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => Ok(bytes),
                other => Err(format!("push retarget strict check violated: {other:?}")),
            }
        }
        GOALSEL_CLASS => {
            let (bytes, outcome) = crate::classfile::retarget_virtual_to_static(
                orig,
                "tick",
                "()V",
                (WRAPPED_GOAL_CLASS, "canUse", CANUSE_DESC),
                (GOAL_OPS, "canUseGate", CANUSE_STATIC_DESC),
            )?;
            match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => Ok(bytes),
                other => Err(format!("goal retarget strict check violated: {other:?}")),
            }
        }
        other => Err(format!("unknown hooked class {other}")),
    }
}

/// Background activation: wait for kernel classes, define bridges into the
/// kernel loader, compute both retarget patches, flip READY and retransform.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // Wait for both hooked kernel classes (they load at boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for class_name in [LIVING_CLASS, GOALSEL_CLASS] {
            let mut forced_attempts = 0usize;
            loop {
                if cplug_sdk::classes::find_class(class_name).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] stagger: {class_name} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                    && forced_attempts < 12
                {
                    forced_attempts += 1;
                    eprintln!(
                        "[crussty-plugin] stagger: forcing kernel load of {class_name} (attempt {forced_attempts})"
                    );
                    crate::improved_noise::force_load_kernel_class(class_name);
                }
                let sighted = cplug_sdk::classes::is_sighted(class_name);
                std::thread::sleep(std::time::Duration::from_millis(if sighted {
                    2_000
                } else {
                    10_000
                }));
            }
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] stagger: boot marker not seen, hook stays dormant");
            return;
        }
        // TASK-80 crash lesson: settle after Done before any define/retransform.
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] stagger: server booted, defining bridges into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [(PUSH_OPS, PUSH_BYTES), (GOAL_OPS, GOAL_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] stagger: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild stagger/ via scripts/build_stagger_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        // Capture the kernel loader global ref from LivingEntity.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(LIVING_CLASS) else {
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
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            let mut ok = true;
            for (name, bytes) in [(PUSH_OPS, PUSH_BYTES), (GOAL_OPS, GOAL_BYTES)] {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] stagger: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] stagger: define_class({name}) failed");
                        ok = false;
                        break;
                    }
                }
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] stagger: bridge definition aborted, hook stays dormant");
            return;
        }

        // Capture pristine bytes for classes that predate the hooks (fast boots).
        for class_name in [LIVING_CLASS, GOALSEL_CLASS] {
            if orig_lock(class_name)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_none()
            {
                eprintln!(
                    "[crussty-plugin] stagger: {class_name} predates hook, resource-stream capture"
                );
                match resource_stream_capture(class_name) {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] stagger: resource-stream capture {class_name} {} bytes",
                            bytes.len()
                        );
                        *orig_lock(class_name).lock().unwrap_or_else(PoisonError::into_inner) =
                            Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] stagger: no pristine bytes for {class_name}, hook stays dormant"
                        );
                        return;
                    }
                }
            }
        }

        // Precompute both patches (strict sites==1 each).
        let mut patches: Vec<(&'static str, Arc<[u8]>)> = Vec::with_capacity(2);
        for class_name in [LIVING_CLASS, GOALSEL_CLASS] {
            let orig = orig_lock(class_name)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            let Some(orig) = orig else {
                eprintln!("[crussty-plugin] stagger: pristine bytes vanished for {class_name}");
                return;
            };
            match patch_for(class_name, &orig) {
                Ok(p) => {
                    let major = crate::improved_noise::class_version(&p)
                        .map(|(m, _)| m)
                        .unwrap_or(0);
                    eprintln!(
                        "[crussty-plugin] stagger: computed retarget for {class_name} ({} -> {} bytes, major {major})",
                        orig.len(),
                        p.len()
                    );
                    patches.push((class_name, Arc::from(p)));
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] stagger: {class_name} patch rejected ({e}) — fail-closed, hook stays dormant"
                    );
                    return;
                }
            }
        }

        // Committed: arm and retransform both classes (hooks serve patches).
        crate::kernel_policy::audit_wire(PUSH_OPS, "pushables", "cmp401_stagger v1");
        crate::kernel_policy::audit_wire(GOAL_OPS, "canUseGate", "cmp401_stagger v1");
        READY.store(true, Ordering::Release);
        let mut ok = true;
        for class_name in [LIVING_CLASS, GOALSEL_CLASS] {
            let rc = cplug_sdk::retransform_class(class_name);
            eprintln!("[crussty-plugin] stagger: retransform {class_name} rc={rc}");
            if rc != 0 {
                ok = false;
            }
        }
        if ok {
            // TASK-406-D: сигнал для mobs_ai (составление поверх soa+stagger).
            LIVING_SERVED.store(true, Ordering::Release);
            crate::mobs_ai::note_stagger_served();
            // TASK-403-C: маркер печатает ФАКТИЧЕСКИЙ флаг раунда
            // (tickplane = сегмент mob-stagger плейна).
            let flag = std::env::var("CRUSSTY_LEVER_FLAG")
                .unwrap_or_default()
                .trim()
                .to_string();
            eprintln!(
                "[crussty-plugin] {flag}: ARMED stagger (push=retargeted 1 site, goal=retargeted 1 site, N from env CRUSSTY_STAGGER_N/LEVER_ARG default 4)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp401_stagger NOT ARMED (retransform rc != 0) — vanilla behavior"
            );
        }
    });
}

/// Pristine capture fallback via the kernel loader's resource stream.
fn resource_stream_capture(class_name: &str) -> Option<Vec<u8>> {
    cplug_sdk::jni_util::with_attached(|env| {
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            return None;
        }
        let loader_cls = env.find_class("java/lang/ClassLoader")?;
        let garm = env.get_method_id(
            loader_cls,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
        )?;
        let res_name = env.new_string_utf(&format!("{class_name}.class"))?;
        let stream = env.call_object_method(
            loader as jvmti_bindings::jni::jobject,
            garm,
            &[jvmti_bindings::jni::jvalue { l: res_name }],
        );
        if stream.is_null() {
            crate::clear_exception(env);
            env.delete_local_ref(res_name);
            env.delete_local_ref(loader_cls);
            return None;
        }
        let in_cls = env.find_class("java/io/InputStream")?;
        let rab = env.get_method_id(in_cls, "readAllBytes", "()[B")?;
        let arr = env.call_object_method(stream, rab, &[]);
        let out = if arr.is_null() {
            crate::clear_exception(env);
            None
        } else {
            let jarr = arr as jvmti_bindings::jni::jbyteArray;
            let len = env.get_array_length(arr as jvmti_bindings::jni::jarray);
            let mut signed = vec![0i8; len as usize];
            env.get_byte_array_region(jarr, 0, len as i32, &mut signed);
            Some(signed.iter().map(|&b| b as u8).collect::<Vec<u8>>())
        };
        env.delete_local_ref(arr);
        env.delete_local_ref(stream);
        env.delete_local_ref(res_name);
        env.delete_local_ref(loader_cls);
        out
    })
    .flatten()
}
