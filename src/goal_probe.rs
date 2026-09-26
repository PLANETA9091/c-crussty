//! GOAL-SELECTOR BRANCH-PREDICATE PROBE (ROUND-468 S88, WILD/ЛАБ RESEARCH-нога
//! — диагностический пасс-тру, НЕ оптимизация; lever STRICT-eq
//! `cmp468_s88probe`).
//!
//! СКОУП S88: моб-ИИ бранч-предикаты. javap ground truth (patched-kernel
//! round-396-a = живой рантайм, purpur-1.21.10):
//!   - `GoalSelector.tick()V`: фазы goalCleanup (bc12-92) → lockedFlags
//!     purge (bc73-87) → goalUpdate (bc107-268) → tail
//!     `tickRunningGoals(true)` (bc277). Цепочка предикатов goalUpdate на
//!     каждую НЕ-running цель: isRunning (bc137) → goalContainsAnyFlags
//!     (disabledFlags, bc144-151) → goalCanBeReplacedForAllFlags
//!     (bc155-162) → WrappedGoal.canUse (bc166) → флаг-луп блокировок
//!     (bc193-261). ИЗМЕРЯЕМЫЕ сайты: canUse — РОВНО 1 сайт (bc166);
//!     canBeReplacedBy — РОВНО 1 сайт (bc59) внутри private-static
//!     goalCanBeReplacedForAllFlags.
//!
//! ЧТО ДЕЛАЕТ: ретаргетит оба сайта на пасс-тру мост GoalProbeOps (чистые
//! счётчики ИСХОДОВ бранчей + сэмплированный наносек-cost canUse; вызов
//! ванильного виртуального метода и возврат его результата 1:1, throwable
//! пробрасывается, порядок вычислений не меняется, RNG не трогается) —
//! это даёт ИЗМЕРЕННЫЕ частоты исходов для capture-матем гипотезы
//! реордеринга предикатов (ваниль-безопасный класс = только pure-проверки;
//! RNG-несущие canUse/canContinueToUse переставлять нельзя).
//!
//! ГЕЙТ: env CRUSSTY_LEVER_FLAG == "cmp468_s88probe" (STRICT-eq, закон
//! 402-F: пустой/чужой флаг = ни сайта, ни моста — ваниль бит-в-байт).
//! FAIL-CLOSED лестница: sites != 1 → hook dormant; define_class failed →
//! dormant; retransform rc != 0 → READY сброшен.
//!
//! НЕ конфликтует с goal_selector.rs (cmp421_brain-семья):lever
//! взаимоисключающий (один env), классы ретаргета разные (Mob vs
//! GoalSelector), мост другой (GoalOps vs GoalProbeOps).

use std::sync::atomic::{AtomicBool, Ordering};

const GS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalSelector";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalProbeOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../goalprobe/build/net/minecraft/world/entity/ai/goal/GoalProbeOps.class");

/// Окружающий метод сайта canUse: `GoalSelector.tick()V` (javap: РОВНО 1
/// сайт invokevirtual WrappedGoal.canUse на bc166, фаза goalUpdate).
const ENCL_CU: (&str, &str) = ("tick", "()V");
/// FROM: receiver WrappedGoal (НЕ Goal — javap: invokevirtual #242
/// Method WrappedGoal.canUse:()Z, aload_3 = WrappedGoal loop var).
const FROM_CANUSE: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/WrappedGoal",
    "canUse",
    "()Z",
);
/// Receiver-prepended static форма (WrappedGoal → static bridge).
const PROBE_CANUSE_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z";

/// Окружающий метод сайта canBeReplacedBy: private-static
/// `GoalSelector.goalCanBeReplacedForAllFlags(WrappedGoal,Map)Z` (javap:
/// РОВНО 1 сайт invokevirtual WrappedGoal.canBeReplacedBy на bc59).
const ENCL_CB: (&str, &str) = (
    "goalCanBeReplacedForAllFlags",
    "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;Ljava/util/Map;)Z",
);
const FROM_CBRB: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/WrappedGoal",
    "canBeReplacedBy",
    "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z",
);
/// Receiver + 1 аргумент (incoming WrappedGoal) → static bridge.
const PROBE_CBRB_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z";

/// javap ground truth: РОВНО 1 сайт в каждом из двух методов.
const CANUSE_SITES: usize = 1;
const CBRB_SITES: usize = 1;

static READY: AtomicBool = AtomicBool::new(false);

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    match std::env::var("CRUSSTY_LEVER_FLAG") {
        Ok(v) => v.trim() == "cmp468_s88probe",
        Err(_) => false,
    }
}

/// Оба ретаргета одного serve (canUse + canBeReplacedBy сайты).
#[derive(Debug, Clone, Copy)]
struct ProbeRetargetPair {
    canuse: crate::classfile::RetargetOutcome,
    cbrb: crate::classfile::RetargetOutcome,
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Dormant-invisible: с чужим/пустым флагом НИЧЕГО не регистрируется.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] goal_probe: dormant (lever_flag != cmp468_s88probe, vanilla goal selector)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(GS_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: GoalSelector грузится на буте раньше активации —
            // пропускаем pristine, составляем ретаргет только на
            // retransform-serve после define моста.
            return None;
        }
        match retarget_probe_sites(bytes) {
            Ok((out, pair)) => match (pair.canuse, pair.cbrb) {
                (
                    crate::classfile::RetargetOutcome::Retargeted { sites: cu },
                    crate::classfile::RetargetOutcome::Retargeted { sites: cb },
                ) if cu == CANUSE_SITES && cb == CBRB_SITES => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_probe: hook serve {GS_CLASS} {} bytes (canUse sites={cu}, cbrb sites={cb})",
                            out.len()
                        );
                    }
                    Some(out)
                }
                (
                    crate::classfile::RetargetOutcome::AlreadyPatched { .. },
                    crate::classfile::RetargetOutcome::AlreadyPatched { .. },
                ) => {
                    // Idempotent: мой ретаргет уже в байтах.
                    Some(out)
                }
                (cu, cb) => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_probe: sites not rewritten (canUse={cu:?}, cbrb={cb:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] goal_probe: compose patch rejected ({e}) — pass-through, vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Compute the retarget patch from the RECEIVED bytes: pass 1 = canUse сайт
/// в tick()V, pass 2 = canBeReplacedBy сайт в goalCanBeReplacedForAllFlags
/// НА ВЫХОДЕ первого прохода. Любой Err всплывает (fail-closed).
fn retarget_probe_sites(bytes: &[u8]) -> Result<(Vec<u8>, ProbeRetargetPair), String> {
    let (out1, canuse) = crate::classfile::retarget_virtual_to_static(
        bytes,
        ENCL_CU.0,
        ENCL_CU.1,
        FROM_CANUSE,
        (OPS_CLASS, "canUseProbe", PROBE_CANUSE_DESC),
    )?;
    let (out2, cbrb) = crate::classfile::retarget_virtual_to_static(
        &out1,
        ENCL_CB.0,
        ENCL_CB.1,
        FROM_CBRB,
        (OPS_CLASS, "canBeReplacedProbe", PROBE_CBRB_DESC),
    )?;
    Ok((out2, ProbeRetargetPair { canuse, cbrb }))
}

/// Background activation: wait for GoalSelector load, boot quiet, define
/// GoalProbeOps into the kernel loader (NO natives), flip READY, retransform
/// GoalSelector (этот класс НЕ патчится другими хуками — мой serve
/// единственный, compose-очередь не нужна).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(GS_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] goal_probe: {GS_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] goal_probe: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Guard: embedded bridge bytes must not be newer than the JVM
        // (lesson 408: stale class blob = sleeping gate).
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let ops_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if ops_major > jvm_major {
            eprintln!(
                "[crussty-plugin] goal_probe: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild goalprobe/ via scripts/build_goalprobe_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge (no natives) into the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(anchor) = cplug_sdk::classes::find_class(GS_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(anchor.as_jclass(), mid, &[]);
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
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] goal_probe: define_class({OPS_CLASS}) failed");
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] goal_probe: bridge definition failed, hook stays dormant");
            return;
        }
        eprintln!("[crussty-plugin] cmp468_s88probe: defined {OPS_CLASS} in kernel loader");

        // ГРОМКИЙ ARM-МАРКЕР (EFFECT-маркеры кладёт GoalProbeOps флашами
        // раз в 2с — вердикты только по ним, урок тика-409/PROFILE-B2).
        eprintln!(
            "[crussty-plugin] cmp468_s88probe: ARMED goal-selector branch-predicate probe (GoalSelector.tick WrappedGoal.canUse x1 -> GoalProbeOps.canUseProbe + goalCanBeReplacedForAllFlags WrappedGoal.canBeReplacedBy x1 -> GoalProbeOps.canBeReplacedProbe; pass-through counters, vanilla outcome bit-for-bit, zero natives)"
        );

        crate::kernel_policy::audit_wire(
            OPS_CLASS,
            "canUseProbe",
            "cmp468_s88probe S88 branch-predicate probe + canBeReplacedProbe",
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(GS_CLASS);
        eprintln!(
            "[crussty-plugin] goal_probe: {GS_CLASS} armed, retransform rc={rc} (ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!("[crussty-plugin] goal_probe: retransform FAILED (rc={rc}) — stays vanilla");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_contract_matches() {
        assert_eq!(ENCL_CU, ("tick", "()V"));
        assert_eq!(
            ENCL_CB,
            (
                "goalCanBeReplacedForAllFlags",
                "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;Ljava/util/Map;)Z"
            )
        );
        assert_eq!(FROM_CANUSE.2, "()Z");
        // receiver-prepended: virtual ()Z + receiver WrappedGoal.
        assert_eq!(
            PROBE_CANUSE_DESC,
            "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z"
        );
        assert_eq!(
            PROBE_CBRB_DESC,
            "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z"
        );
        assert_eq!(CANUSE_SITES, 1);
        assert_eq!(CBRB_SITES, 1);
    }

    #[test]
    fn goalprobe_blob_carries_gate_and_markers() {
        // javap-гейт: блоб должен нести STRICT-флаг и EFFECT-маркеры (cp truth).
        let blob = OPS_BYTES;
        for marker in [
            "cmp468_s88probe",
            "GOALPROBE EFFECT armed",
            "canUseProbe",
            "canBeReplacedProbe",
            "GOALPROBE canUse evals=",
            "GOALPROBE creplaced evals=",
        ] {
            assert!(
                blob.windows(marker.len()).any(|w| w == marker.as_bytes()),
                "marker '{marker}' missing from GoalProbeOps blob"
            );
        }
    }

    #[test]
    fn retarget_contract_rejects_wrong_desc() {
        // Валидатор compose должен отвергнуть НЕ receiver-prepended desc.
        let err = crate::classfile::retarget_virtual_to_static(
            &[],
            ENCL_CU.0,
            ENCL_CU.1,
            FROM_CANUSE,
            (OPS_CLASS, "canUseProbe", "()Z"),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }

    #[test]
    fn retarget_cbrb_rejects_wrong_desc() {
        let err = crate::classfile::retarget_virtual_to_static(
            &[],
            ENCL_CB.0,
            ENCL_CB.1,
            FROM_CBRB,
            (
                OPS_CLASS,
                "canBeReplacedProbe",
                "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z",
            ),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }
}
