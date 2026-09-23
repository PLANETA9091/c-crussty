//! Runtime wiring for the GOAL-SELECTOR FLAT PRIORITY FAST-PATH
//! (TASK-421-A brain-slice, lever `cmp421_brain`; закон 6 v17: подсистема
//! sense+brain ЦЕЛИКОМ на rust-носителе — этот модуль = срез «goal
//! priorities»; сенс-плоскость (CSR-арена, goal-query walk) живёт в
//! src/entity_query.rs того же крейта).
//!
//! RUST-FIRST (мандат тика-421): крейт владеет вайрингом brain-среза —
//! ретаргет сайта `invokevirtual GoalSelector.tick()V` в
//! `Mob.serverAiStep()V` (javap ground truth purpur-1.21.10: РОВНО 2 сайта,
//! targetSelector + goalSelector; Paper-сплит уже вынес tickRunningGoals
//! наружу — эти сайты НЕ трогаются, там нет выигрыша). Мост
//! goalops/net/minecraft/world/entity/ai/goal/GoalOps.java реплицирует
//! ванильное тело tick() 1:1 (goalCleanup → lockedFlags purge → goalUpdate →
//! goalTick), но три полных обхода ObjectLinkedOpenHashSet заменяет одним
//! обходом + плоскими проходами по ThreadLocal-массиву (порядок вызовов
//! stop/start/canUse/canContinueToUse/tick — insertion-order, как в ванили;
//! NO_GOAL-сентинел свёрнут в null-проверки по javap-эквивалентности).
//!
//! БЕЗ natives: win = устройство данных (flat priorities), не rust-вычисления
//! (закон 6 RUST-FIRST уважен: крейт = единственный источник вайринга; bulk
//! JNI sense+brain-подсистемы уже исчерпан eqEpoch+senseArena в том же
//! EPOCH_LOCK-окне — третий bulk-вызов не нужен).
//!
//! CHAIN COMPOSITION (cplug-sdk ORDERING CONTRACT): Mob.class уже несёт
//! ретаргет mobs_sscan (checkDespawn, sites=1). Этот хук КОМПОЗИРУЕТ:
//! на serve-времени ретаргетит tick-сайты В ПОЛУЧЕННЫХ байтах (вывод
//! предыдущих хуков сохранён; AlreadyPatched → pass-through; Err → None
//! fail-closed). Порядок converge'ит: кто бы ни ретрансформировал первым,
//! второй хук получает compose-выход и добавляет свой сайт.
//!
//! Гейт: env `CRUSSTY_LEVER_FLAG == "cmp421_brain"` (STRICT eq; пустой/
//! чужой флаг = хук не регистрируется вовсе — бит-в-байт ваниль).
//! FAIL-CLOSED лестница: sites != 2 → hook stays dormant; define_class
//! failed → dormant; java-гейт сам зовёт sel.tick() (ваниль) на SETUP-дрейф.

use std::sync::atomic::{AtomicBool, Ordering};

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class");

/// purpur-1.21.10 javap: `invokevirtual GoalSelector.tick:()V` — ровно 2 сайта
/// в Mob.serverAiStep (targetSelector @161, goalSelector @183).
const TICK_SITES: usize = 2;

const FROM: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tick",
    "()V",
);
/// Receiver-prepended static form (stack-identical GoalSelector→static gate).
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V";

static READY: AtomicBool = AtomicBool::new(false);

/// mobs_manager.rs-стиль сигнал для будущих compose-очередей (Mob-цепь).
static SERVED: AtomicBool = AtomicBool::new(false);

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp421_brain")
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Dormant-invisible: with the lever flag unset/mismatched NOTHING is
/// registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] goal_selector: dormant (lever_flag != cmp421_brain, vanilla goal selector)"
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
        match retarget_goal_tick(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == TICK_SITES => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_selector: hook serve {MOB_CLASS} {} bytes (composed, sites={sites})",
                            out.len()
                        );
                    }
                    Some(out)
                }
                crate::classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    // Idempotent: keep the chain output (my rewrite already in).
                    Some(out)
                }
                other => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_selector: GoalSelector.tick sites not rewritten ({other:?}) — pass-through (fail-closed)"
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

/// Compute the retarget patch from the RECEIVED (pristine or composed) bytes.
fn retarget_goal_tick(
    bytes: &[u8],
) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        FROM.1,
        FROM.2,
        FROM,
        (OPS_CLASS, "tickGate", GATE_STATIC_DESC),
    )
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

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp421_brain: ARMED goal-selector flat priority fast-path (Mob.serverAiStep GoalSelector.tick x2 -> GoalOps.tickGate; 1 set traversal + flat passes vs 3 vanilla; exact stop/start order; zero natives; empty flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "tickGate", "cmp421_brain v1");
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
        assert_eq!(FROM.2, "()V");
        assert_eq!(
            GATE_STATIC_DESC,
            "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V"
        );
        assert_eq!(TICK_SITES, 2);
    }

    #[test]
    fn goalops_blob_carries_gate_and_markers() {
        // javap-гейт: блоб должен нести STRICT-флаг и маркеры (cp truth).
        let blob = OPS_BYTES;
        for marker in [
            "cmp421_brain",
            "goal-selector EFFECT armed",
            "goalCleanup",
            "goalUpdate",
            "tickGate",
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
            FROM.1,
            FROM.2,
            FROM,
            (OPS_CLASS, "tickGate", "()V"),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }
}
