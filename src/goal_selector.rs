//! Runtime wiring for the GOAL-SELECTOR FLAT PRIORITY FAST-PATH — iter-2,
//! слайс подсистемы ЦЕЛИКОМ (TASK-421-A → TASK-422-B; lever STRICT-OR
//! `cmp421_brain` ∨ `cmp422_brain2`; закон 6 v17: крейт владеет вайрингом
//! brain-среза).
//!
//! RUST-FIRST: ретаргет сайтов в `Mob.serverAiStep()V` (javap ground truth
//! purpur-1.21.10, `protected final void serverAiStep()V`):
//!   - `invokevirtual GoalSelector.tick()V` — РОВНО 2 сайта (@161 targetSelector,
//!     @183 goalSelector; чётные тики);
//!   - `invokevirtual GoalSelector.tickRunningGoals(Z)V` — РОВНО 2 сайта
//!     (@113/@136; нечётные тики, Paper-сплит, оба false).
//!
//! TASK-422-B FIX (root-cause плацебо mga421, PROFILE-B2): предыдущая версия
//! сканировала ОКРУЖАЮЩИЙ метод `tick()V` (FROM.1/FROM.2 передавались как
//! метод-содержатель) — в Mob.class ЕСТЬ собственный `public void tick()V`
//! (LivingEntity.tick + updateControlFlags) БЕЗ этих сайтов → NotFound →
//! fail-closed → плечо cmp421_brain работало ванильно при ARM-маркере
//! (grep GoalOps в cpu-collapsed mga421 = 0; EFFECT-маркера нет).
//! Окружающий метод = `serverAiStep()V` — только там живут 4 сайта.
//!
//! Мост goalops/.../GoalOps.java: tickGate (флет-приоритеты: 1 обход set +
//! плоские проходы vs 3 ванильных, порядок stop/start/canUse/canContinueToUse/
//! tick — insertion-order) + tickRunningGate (нечётные тики: 1 плоский проход
//! по снапшоту vs итератор; isRunning читается ЖИВЫМ — семантика ванили).
//! БЕЗ natives: win = устройство данных (flat priorities), не rust-вычисления.
//!
//! CHAIN COMPOSITION (cplug-sdk ORDERING CONTRACT): Mob.class уже несёт
//! ретаргет mobs_sscan (checkDespawn, sites=1). Этот хук КОМПОЗИРУЕТ: на
//! serve-времени ретаргетит 4 сайта В ПОЛУЧЕННЫХ байтах. Порядок converge'ит.
//!
//! Гейт: env `CRUSSTY_LEVER_FLAG == "cmp421_brain" || == "cmp422_brain2" || v == "cmp423_brain3"`
//! (STRICT-OR по образцу 40adceb; пустой/чужой флаг = ваниль бит-в-байт).
//! FAIL-CLOSED лестница: sites != (2,2) → hook stays dormant; define_class
//! failed → dormant; java-гейт сам зовёт ваниль на SETUP-дрейф.

use std::sync::atomic::{AtomicBool, Ordering};

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class");

/// ОКРУЖАЮЩИЙ метод, в коде которого ретаргетятся сайты (TASK-422-B FIX —
/// ранее ошибочно сканировался Mob.tick()V: NotFound, плечо спало).
const ENCL: (&str, &str) = ("serverAiStep", "()V");

/// purpur-1.21.10 javap Mob.serverAiStep: `invokevirtual GoalSelector.tick:()V`
/// — РОВНО 2 сайта (@161 targetSelector, @183 goalSelector).
const TICK_SITES: usize = 2;
/// ...и `invokevirtual GoalSelector.tickRunningGoals:(Z)V` — РОВНО 2 сайта
/// (@113/@136, нечётные тики, оба аргумента false).
const RUNNING_SITES: usize = 2;

const FROM_TICK: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tick",
    "()V",
);
/// Receiver-prepended static form (stack-identical GoalSelector→static gate).
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V";

const FROM_RUN: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tickRunningGoals",
    "(Z)V",
);
const GATE_RUNNING_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;Z)V";

static READY: AtomicBool = AtomicBool::new(false);

/// mobs_manager.rs-стиль сигнал для будущих compose-очередей (Mob-цепь).
static SERVED: AtomicBool = AtomicBool::new(false);

/// STRICT-OR gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт. Носитель-флаг
/// cmp421_brain (мега-ноги тика-421/422) ∨ вектор-флаг cmp422_brain2.
fn enabled() -> bool {
    match std::env::var("CRUSSTY_LEVER_FLAG") {
        Ok(v) => {
            let v = v.trim();
            v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp434_chunkpl"
        }
        Err(_) => false,
    }
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Dormant-invisible: with the lever flag unset/mismatched NOTHING is
/// registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] goal_selector: dormant (lever_flag ∉ {{cmp421_brain, cmp422_brain2}}, vanilla goal selector)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOB_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: pass the chain output through untouched (earlier hooks
            // own pristine capture + their own serves; Mob chain = sscan
            // first, this module composes on top at its own serve time).
            return None;
        }
        // Compose onto the RECEIVED bytes (previous hooks' output preserved).
        match retarget_ai_sites(bytes) {
            Ok((out, pair)) => match (pair.tick, pair.run) {
                (
                    crate::classfile::RetargetOutcome::Retargeted { sites: ts },
                    crate::classfile::RetargetOutcome::Retargeted { sites: rs },
                ) if ts == TICK_SITES && rs == RUNNING_SITES => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_selector: hook serve {MOB_CLASS} {} bytes (composed, tick sites={ts}, running sites={rs})",
                            out.len()
                        );
                    }
                    Some(out)
                }
                (crate::classfile::RetargetOutcome::AlreadyPatched { .. },
                 crate::classfile::RetargetOutcome::AlreadyPatched { .. }) => {
                    // Idempotent: keep the chain output (my rewrite already in).
                    Some(out)
                }
                _ => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_selector: serverAiStep sites not rewritten (tick={:?}, running={:?}) — pass-through (fail-closed)",
                            pair.tick, pair.run
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] goal_selector: compose patch rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Исход обоих ретаргетов одного serve (tick-сайты + running-сайты).
#[derive(Debug, Clone, Copy)]
struct AiRetargetPair {
    tick: crate::classfile::RetargetOutcome,
    run: crate::classfile::RetargetOutcome,
}

/// Compute the retarget patch from the RECEIVED (pristine or composed) bytes.
/// TASK-422-B: ДВА прохода по ОДНОМУ окружающему методу `serverAiStep()V`:
/// сначала tick-сайты (→ GoalOps.tickGate), затем running-сайты на ВЫХОДЕ
/// первого прохода (→ GoalOps.tickRunningGate). Любой Err всплывает
/// (fail-closed, хук остаётся ванильным).
fn retarget_ai_sites(
    bytes: &[u8],
) -> Result<(Vec<u8>, AiRetargetPair), String> {
    let (out1, tick) = crate::classfile::retarget_virtual_to_static(
        bytes,
        ENCL.0,
        ENCL.1,
        FROM_TICK,
        (OPS_CLASS, "tickGate", GATE_STATIC_DESC),
    )?;
    let (out2, run) = crate::classfile::retarget_virtual_to_static(
        &out1,
        ENCL.0,
        ENCL.1,
        FROM_RUN,
        (OPS_CLASS, "tickRunningGate", GATE_RUNNING_DESC),
    )?;
    Ok((out2, AiRetargetPair { tick, run }))
}

/// Background activation: wait for Mob + GoalSelector to load, boot quiet,
/// define the GoalOps bridge into the kernel loader (NO natives — pure-java
/// fast-path), flip READY and retransform Mob (this module's serve composes
/// onto the sscan output already in the class).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for cls in [MOB_CLASS, "net/minecraft/world/entity/ai/goal/GoalSelector"] {
            while cplug_sdk::classes::find_class(cls).is_none() {
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] goal_selector: {cls} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] goal_selector: boot marker not seen, hook stays dormant");
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
                "[crussty-plugin] goal_selector: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild goalops/ via scripts/build_goalops_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge (no natives) into the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(anchor) = cplug_sdk::classes::find_class(
                "net/minecraft/world/entity/ai/goal/GoalSelector",
            ) else {
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
                eprintln!("[crussty-plugin] goal_selector: define_class({OPS_CLASS}) failed");
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] goal_selector: bridge definition failed, hook stays dormant");
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp421_brain: defined {OPS_CLASS} in kernel loader"
        );

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed; EFFECT-маркеры
        // кладёт GoalOps на первом плоском тике — вердикты только по ним,
        // урок тика-409/PROFILE-B2).
        eprintln!(
            "[crussty-plugin] cmp422_brain2: ARMED goal-selector flat priority fast-path (Mob.serverAiStep GoalSelector.tick x2 -> GoalOps.tickGate + GoalSelector.tickRunningGoals x2 -> GoalOps.tickRunningGate; 1 set traversal + flat passes vs 3 vanilla, 1 flat pass on odd ticks; exact stop/start order; zero natives; empty/foreign flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "tickGate", "cmp422_brain2 v2 (iter-2, fix serverAiStep) + tickRunningGate");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!(
            "[crussty-plugin] goal_selector: {MOB_CLASS} armed, retransform rc={rc} (composed serve; ready-gate on)"
        );
        if rc == 0 {
            SERVED.store(true, Ordering::Release);
        } else {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] goal_selector: retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
        }
    });
}

/// mobs_sscan-стиль сигнал: Mob-ретрансформация с моим сайтом сошла.
#[allow(dead_code)] // compose-очередь будущих Mob-хуков (парность с mobs_ai::ai_ready)
pub fn goal_served() -> bool {
    SERVED.load(Ordering::Acquire)
}

// ---------------------------------------------------------------------------
// Tests: strict gate + retarget descriptor contract (receiver-prepended
// virtual desc) + the javap ground-truth site count expectation.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_gate_matches() {
        // (env-гейт читается динамически; здесь проверяем чистую функцию
        // дескрипторного контракта — env-мутации в параллельных тестах
        // недетерминированы, STRICT-семантика покрыта java-гейтом.)
        // TASK-422-B FIX regression: окружающий метод — ИМЕННО serverAiStep
        // (скан tick()V давал NotFound на реальном Mob.class: плацебо-плечо).
        assert_eq!(ENCL, ("serverAiStep", "()V"));
        assert_eq!(FROM_TICK.2, "()V");
        assert_eq!(
            GATE_STATIC_DESC,
            "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V"
        );
        assert_eq!(FROM_RUN, ("net/minecraft/world/entity/ai/goal/GoalSelector", "tickRunningGoals", "(Z)V"));
        // receiver-prepended: virtual (Z)V + receiver GoalSelector.
        assert_eq!(
            GATE_RUNNING_DESC,
            "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;Z)V"
        );
        assert_eq!(TICK_SITES, 2);
        assert_eq!(RUNNING_SITES, 2);
    }

    #[test]
    fn goalops_blob_carries_gate_and_markers() {
        // javap-гейт: блоб должен нести STRICT-флаги (ОБА — STRICT-OR) и
        // маркеры обоих гейтов (cp truth).
        let blob = OPS_BYTES;
        for marker in [
            "cmp421_brain",
            "cmp422_brain2",
            "goal-selector EFFECT armed",
            "goal-selector running EFFECT armed",
            "goalCleanup",
            "goalUpdate",
            "goalTick",
            "tickGate",
            "tickRunningGate",
        ] {
            assert!(
                blob.windows(marker.len()).any(|w| w == marker.as_bytes()),
                "marker '{marker}' missing from GoalOps blob"
            );
        }
    }

    #[test]
    fn retarget_contract_rejects_wrong_desc() {
        // Валидатор compose должен отвергнуть НЕ receiver-prepended desc.
        let err = crate::classfile::retarget_virtual_to_static(
            &[],
            ENCL.0,
            ENCL.1,
            FROM_TICK,
            (OPS_CLASS, "tickGate", "()V"),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }

    #[test]
    fn retarget_running_rejects_wrong_desc() {
        // running-гейт: неверный static desc отвергается ДО скана сайтов.
        let err = crate::classfile::retarget_virtual_to_static(
            &[],
            ENCL.0,
            ENCL.1,
            FROM_RUN,
            (OPS_CLASS, "tickRunningGate", "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V"),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }

    #[test]
    fn mob_bytes_retarget_four_sites_if_available() {
        // Реальные байты ядра (ground-truth проф: /tmp/crussty-mob-bytes/
        // Mob.class из patched-kernel.jar round-396-a; в CI/sans файла —
        // тихий пропуск, тест остаётся hermetic).
        let Ok(path) = std::env::var("CRUSSTY_MOB_BYTES") else {
            eprintln!("skip: CRUSSTY_MOB_BYTES not set (no real kernel Mob.class in env)");
            return;
        };
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("skip: cannot read {path}");
            return;
        };
        let (_, pair) = retarget_ai_sites(&bytes).expect("retarget on real Mob bytes must not Err");
        assert!(
            matches!(pair.tick, crate::classfile::RetargetOutcome::Retargeted { sites: 2 }),
            "tick sites expected 2, got {:?} — ENCL/FROM contract broken",
            pair.tick
        );
        assert!(
            matches!(pair.run, crate::classfile::RetargetOutcome::Retargeted { sites: 2 }),
            "running sites expected 2, got {:?} — ENCL/FROM contract broken",
            pair.run
        );
    }
}
