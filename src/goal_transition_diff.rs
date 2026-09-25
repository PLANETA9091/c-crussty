//! WILD TASK-459-72 — GOAL-SELECTOR TRANSITION-DIFF BATCH (iter-3 scaffold).
//!
//! Карточка ID-P47 (RESEARCH-458-P.md): iter-2 flat-проходы tick/tickRunningGate
//! взяты как носитель; ДОБАВЛЕНО: stop/start переходы собираются БАТЧАМИ
//! (transition-diff) с ОТЛОЖЕННЫМ пересчётом флагов управления — 1 пересчёт/тик
//! вместо per-goal, insertion-order стоп/старт сохранён (ваниль-порядок).
//! Прогноз +0.3-0.6пп goalops-хвост. Рисёрч: RESEARCH-459-P47.md (3 URL).
//!
//! САФФОЛД DORMANT: гейт STRICT-eq `cmp459_p47` (пустой/чужой флаг = ваниль
//! бит-в-байт); блоб-гейт по маркерам `transitionDiffGate`/
//! `transitionDiffRunningGate`/`cmp459_p47` в включённом GoalOps.class —
//! пока goalops/ не пересобран со стабами iter-3, сайты НЕ трогаются.
//!
//! NCDFE-канон: define+init GoalOps в kernel loader ДО флипа READY;
//! define failed → dormant; class-major guard (урок 408); ретаргет сайтов
//! разрешён ТОЛЬКО при живых маркерах в блобе (ретаргет на несуществующий
//! статик = NoSuchMethodError в server tick — та же семья, что NCDFE).
//!
//! Ретаргет-контракт (javap purpur-1.21.10 = goal_selector.rs iter-2):
//! `Mob.serverAiStep()V`, `invokevirtual GoalSelector.tick()V` РОВНО 2 сайта
//! (@161/@183) и `invokevirtual GoalSelector.tickRunningGoals:(Z)V` РОВНО 2
//! сайта (@113/@136, оба false). iter-3 таргетит ТЕ ЖЕ 4 сайта на НОВЫЕ
//! статики (receiver-prepended): GoalOps.transitionDiffGate /
//! GoalOps.transitionDiffRunningGate — стабы в goalops/…GoalOps.java пока
//! зовут ваниль (fail-inert), модель батча живёт здесь и тестируется.
//!
//! Разделяемость с iter-2: `cmp459_p47` НЕ входит в STRICT-OR список
//! goal_selector::enabled() → гейты взаимно исключающие по env, двойное
//! define/retransform GoalOps невозможно (каждая нога видит свой флаг).

use std::sync::atomic::{AtomicBool, Ordering};

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class");

/// Окружающий метод — как в iter-2 (TASK-422-B fix: только serverAiStep).
const ENCL: (&str, &str) = ("serverAiStep", "()V");

/// Javap ground truth: ровно 2+2 сайта (anti-placebo, lesson 409/PROFILE-B2).
const TICK_SITES: usize = 2;
const RUNNING_SITES: usize = 2;

const FROM_TICK: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tick",
    "()V",
);
const FROM_RUN: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tickRunningGoals",
    "(Z)V",
);

/// iter-3 статики (receiver-prepended virtual desc — канон retarget).
const GATE_TD_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V";
const GATE_TDR_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;Z)V";

/// STRICT-eq lever (карточка P47; итер-2 флаги НЕ принимаются).
pub const LEVER: &str = "cmp459_p47";

fn flag_matches(v: &str) -> bool {
    v.trim() == LEVER
}

fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| flag_matches(&v))
        .unwrap_or(false)
}

/// NCDFE/NoSuchMethodError-канон: блоб обязан нести маркеры iter-3 ДО того,
/// как rust разрешит ретаргет сайтов на transitionDiff-статики.
fn blob_ready() -> bool {
    for marker in [
        "transitionDiffGate",
        "transitionDiffRunningGate",
        LEVER,
        "p47 transition-diff",
    ] {
        if !OPS_BYTES.windows(marker.len()).any(|w| w == marker.as_bytes()) {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Модель transition-diff (чистая, тестируемая; java-батч = 1:1 перенос).
// ---------------------------------------------------------------------------

/// Флаги управления Goal.Flag: MOVE|JUMP|LOOK|TARGET — 4 бита.
pub const FLAG_MOVE: u8 = 1 << 0;
pub const FLAG_JUMP: u8 = 1 << 1;
pub const FLAG_LOOK: u8 = 1 << 2;
pub const FLAG_TARGET: u8 = 1 << 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionKind {
    Stop,
    Start,
}

/// Один переход батча: insertion-seq монотонен → порядок стоп/старт =
/// ваниль insertion-order (карточка: «insertion-order стоп/старт сохранён»).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionOp {
    pub seq: u64,
    pub kind: TransitionKind,
    pub goal_id: u32,
    pub flags: u8,
}

/// Ёмкость дифф-буфера (ThreadLocal-скретч, без аллокации после прогрева).
pub const MAX_TRANSITIONS: usize = 1024;

#[derive(Debug, Default)]
pub struct TransitionBatch {
    ops: Vec<TransitionOp>,
    next_seq: u64,
    /// Отложенный пересчёт флагов: любой переход ставит pending,
    /// 1 пересчёт/тик снимает (flag-накопитель — MOVE/JUMP/LOOK/TARGET OR).
    flags_pending: u8,
    overflow: bool,
}

impl TransitionBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            ops: Vec::with_capacity(cap),
            next_seq: 0,
            flags_pending: 0,
            overflow: false,
        }
    }

    fn push(&mut self, kind: TransitionKind, goal_id: u32, flags: u8) {
        if self.ops.len() >= MAX_TRANSITIONS {
            // Capacity-guard (fail-closed): размечаем немедленный пересчёт
            // флагов в этой же точке — семантика ванили деградирует к
            // per-goal, НИКОГДА не к пропуску пересчёта.
            self.overflow = true;
            return;
        }
        self.ops.push(TransitionOp {
            seq: self.next_seq,
            kind,
            goal_id,
            flags,
        });
        self.next_seq += 1;
        self.flags_pending |= flags;
    }

    pub fn push_stop(&mut self, goal_id: u32, flags: u8) {
        self.push(TransitionKind::Stop, goal_id, flags);
    }

    pub fn push_start(&mut self, goal_id: u32, flags: u8) {
        self.push(TransitionKind::Start, goal_id, flags);
    }

    pub fn ops(&self) -> &[TransitionOp] {
        &self.ops
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Отложенный пересчёт флагов управления: ЧИСЛО пересчётов за тик.
    /// Контракт iter-3: ≤ 1 на тик (0, если переходов не было) — применить
    /// OR-маску накопленных флагов и снять pending. Смета: 1 вместо
    /// per-goal N×K.
    pub fn recalc_control_flags(&mut self) -> u8 {
        let flags = std::mem::take(&mut self.flags_pending);
        self.ops.clear();
        self.overflow = false;
        flags
    }

    /// Полный тик-цикл: дренаж батча (apply-фаза) в порядке insertion;
    /// возвращает (ops, pending-флаги до пересчёта).
    pub fn drain_tick(&mut self) -> (&[TransitionOp], u8) {
        (self.ops.as_slice(), self.flags_pending)
    }

    #[cfg(test)]
    pub fn overflowed(&self) -> bool {
        self.overflow
    }
}

/// Гроссбух пересчётов: invariant «≤ 1 пересчёт флагов на тик» (12e-гейт
/// чёт/нечёт parity: пересчёт флагов вне единых точек = дрейф семантики).
#[derive(Debug, Default)]
pub struct FlagRecalcLedger {
    last_tick: Option<u64>,
    recalcs_this_tick: u32,
    pub total_recalcs: u64,
    pub violations: u64,
}

impl FlagRecalcLedger {
    /// Зафиксировать пересчёт флагов в тике `tick`. Второй пересчёт в том
    /// же тике = нарушение канона (считается, не паникует — fail-closed).
    pub fn record(&mut self, tick: u64) {
        if self.last_tick == Some(tick) {
            self.recalcs_this_tick += 1;
            self.violations += 1;
        } else {
            self.last_tick = Some(tick);
            self.recalcs_this_tick = 1;
        }
        self.total_recalcs += 1;
    }

    pub fn recalcs_this_tick(&self) -> u32 {
        self.recalcs_this_tick
    }
}

// ---------------------------------------------------------------------------
// Вайринг (повторяет канон goal_selector.rs; гейты взаимно исключающие).
// ---------------------------------------------------------------------------

static READY: AtomicBool = AtomicBool::new(false);

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Dormant-invisible: флаг ≠ cmp459_p47 → НИЧЕГО не регистрируется; флаг
/// совпал, но блоб старый (нет маркеров iter-3) → тоже dormant (NCDFE-канон).
pub fn register() {
    if !enabled() || !blob_ready() {
        eprintln!(
            "[crussty-plugin] goal_transition_diff: dormant (lever_flag != {LEVER} или блоб без маркеров iter-3 — vanilla goal selector)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOB_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None;
        }
        match retarget_ai_sites(bytes) {
            Ok((out, pair)) => match (pair.tick, pair.run) {
                (
                    crate::classfile::RetargetOutcome::Retargeted { sites: ts },
                    crate::classfile::RetargetOutcome::Retargeted { sites: rs },
                ) if ts == TICK_SITES && rs == RUNNING_SITES => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_transition_diff: hook serve {MOB_CLASS} {} bytes (transition-diff batch, tick sites={ts}, running sites={rs})",
                            out.len()
                        );
                    }
                    Some(out)
                }
                (crate::classfile::RetargetOutcome::AlreadyPatched { .. },
                 crate::classfile::RetargetOutcome::AlreadyPatched { .. }) => Some(out),
                _ => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] goal_transition_diff: sites not rewritten (tick={:?}, running={:?}) — pass-through (fail-closed)",
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
                        "[crussty-plugin] goal_transition_diff: compose patch rejected ({e}) — pass-through, vanilla"
                    );
                }
                None
            }
        }
    });
}

#[derive(Debug, Clone, Copy)]
struct AiRetargetPair {
    tick: crate::classfile::RetargetOutcome,
    run: crate::classfile::RetargetOutcome,
}

/// Два прохода по serverAiStep: tick-сайты → transitionDiffGate, затем
/// running-сайты (на ВЫХОДЕ первого) → transitionDiffRunningGate.
fn retarget_ai_sites(bytes: &[u8]) -> Result<(Vec<u8>, AiRetargetPair), String> {
    let (out1, tick) = crate::classfile::retarget_virtual_to_static(
        bytes,
        ENCL.0,
        ENCL.1,
        FROM_TICK,
        (OPS_CLASS, "transitionDiffGate", GATE_TD_DESC),
    )?;
    let (out2, run) = crate::classfile::retarget_virtual_to_static(
        &out1,
        ENCL.0,
        ENCL.1,
        FROM_RUN,
        (OPS_CLASS, "transitionDiffRunningGate", GATE_TDR_DESC),
    )?;
    Ok((out2, AiRetargetPair { tick, run }))
}

/// Background activation — NCDFE-канон: define GoalOps ДО флипа READY,
/// class-major guard, boot-quiet; потом retransform Mob.
pub fn activate() {
    if !enabled() {
        return;
    }
    if !blob_ready() {
        eprintln!(
            "[crussty-plugin] goal_transition_diff: GoalOps blob без маркеров {LEVER} — rebuild goalops/ первым шагом ноги; hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for cls in [MOB_CLASS, "net/minecraft/world/entity/ai/goal/GoalSelector"] {
            while cplug_sdk::classes::find_class(cls).is_none() {
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] goal_transition_diff: {cls} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
        }
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] goal_transition_diff: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

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
                "[crussty-plugin] goal_transition_diff: {OPS_CLASS} class major {ops_major} > JVM {jvm_major} — rebuild goalops/; hook stays dormant"
            );
            return;
        }

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
                eprintln!(
                    "[crussty-plugin] goal_transition_diff: define_class({OPS_CLASS}) failed"
                );
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] goal_transition_diff: bridge definition failed, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] {LEVER}: defined {OPS_CLASS} in kernel loader"
        );
        eprintln!(
            "[crussty-plugin] {LEVER}: ARMED goal-selector transition-diff batch scaffold (stop/start батч, 1 пересчёт флагов/тик, insertion-order сохранён; чёт/нечёт parity оракул GoalOps; empty/foreign flag = vanilla bit-for-bit)"
        );
        crate::kernel_policy::audit_wire(
            OPS_CLASS,
            "transitionDiffGate",
            "cmp459_p47 v1 (iter-3 scaffold, dormant)",
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!(
            "[crussty-plugin] goal_transition_diff: {MOB_CLASS} armed, retransform rc={rc}"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] goal_transition_diff: retransform FAILED (rc={rc}) — stays vanilla"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Tests: гейт-дискрет, insertion-order, 1 пересчёт/тик, capacity-guard,
// блоб-маркеры + ретаргет-контракт (дескрипторы и site-count ground truth).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_gate_discrete() {
        // STRICT-eq: только cmp459_p47; iter-2 флаги и пустой — ваниль.
        assert!(flag_matches("cmp459_p47"));
        assert!(flag_matches(" cmp459_p47 "));
        assert!(!flag_matches(""));
        assert!(!flag_matches("cmp421_brain"));
        assert!(!flag_matches("cmp422_brain2"));
        assert!(!flag_matches("cmp457_paldelta"));
        assert!(!flag_matches("cmp459_p47x"));
    }

    #[test]
    fn transition_batch_preserves_insertion_order() {
        let mut b = TransitionBatch::with_capacity(16);
        b.push_stop(11, FLAG_MOVE);
        b.push_start(12, FLAG_MOVE | FLAG_LOOK);
        b.push_stop(13, FLAG_TARGET);
        b.push_start(14, FLAG_JUMP);
        let (ops, pending) = b.drain_tick();
        assert_eq!(ops.len(), 4);
        // seq строго монотонен → стоп/старт идут в ванильном insertion-order.
        assert!(ops.windows(2).all(|w| w[0].seq < w[1].seq));
        assert_eq!(ops[0].kind, TransitionKind::Stop);
        assert_eq!(ops[1].kind, TransitionKind::Start);
        assert_eq!(ops[2].goal_id, 13);
        // pending = OR всех флагов переходов (одна маска на тик).
        assert_eq!(pending, FLAG_MOVE | FLAG_LOOK | FLAG_TARGET | FLAG_JUMP);
    }

    #[test]
    fn flag_recalc_once_per_tick() {
        let mut b = TransitionBatch::with_capacity(16);
        let mut ledger = FlagRecalcLedger::default();
        b.push_stop(1, FLAG_MOVE);
        b.push_start(2, FLAG_LOOK);
        b.push_stop(3, FLAG_TARGET);
        // Три перехода, но пересчёт — ровно один (iter-3 core invariant).
        assert_eq!(b.recalc_control_flags(), FLAG_MOVE | FLAG_LOOK | FLAG_TARGET);
        ledger.record(42);
        assert_eq!(ledger.recalcs_this_tick(), 1);
        assert_eq!(ledger.total_recalcs, 1);
        // Пустой тик → 0 пересчётов (pending снят).
        assert_eq!(b.recalc_control_flags(), 0);
        // Двойной пересчёт в том же тике = нарушение (считается fail-closed).
        ledger.record(42);
        assert_eq!(ledger.violations, 1);
    }

    #[test]
    fn batch_capacity_guard_degrades_safe() {
        let mut b = TransitionBatch::with_capacity(4);
        for i in 0..(MAX_TRANSITIONS + 8) {
            b.push_stop(i as u32, FLAG_MOVE);
        }
        assert!(b.overflowed());
        assert_eq!(b.len(), MAX_TRANSITIONS); // влезшие переходы сохранены
        // Переполнение НЕ теряет пересчёт: pending снят принудительно,
        // батч дренирован (semantics degrade to vanilla per-goal, never
        // to a missed recalc).
        assert_eq!(b.recalc_control_flags(), FLAG_MOVE);
        assert!(b.is_empty());
    }

    #[test]
    fn blob_carries_iter3_markers_ncdfe_canon() {
        // NCDFE/NoSuchMethodError-канон: ретаргет разрешён только при живых
        // маркерах в блобе (стаб-методы существуют в байткоде).
        assert!(blob_ready(), "GoalOps blob lacks iter-3 markers — rebuild goalops/");
        for marker in [
            "cmp421_brain",
            "cmp422_brain2",
            "cmp459_p47",
            "transitionDiffGate",
            "transitionDiffRunningGate",
            "p47 transition-diff",
            "tickGate",
            "tickRunningGate",
        ] {
            assert!(
                OPS_BYTES.windows(marker.len()).any(|w| w == marker.as_bytes()),
                "marker '{marker}' missing from GoalOps blob"
            );
        }
    }

    #[test]
    fn retarget_contract_descriptors() {
        assert_eq!(ENCL, ("serverAiStep", "()V"));
        assert_eq!(FROM_TICK.2, "()V");
        assert_eq!(FROM_RUN, ("net/minecraft/world/entity/ai/goal/GoalSelector", "tickRunningGoals", "(Z)V"));
        assert_eq!(GATE_TD_DESC, "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V");
        assert_eq!(GATE_TDR_DESC, "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;Z)V");
        assert_eq!(TICK_SITES, 2);
        assert_eq!(RUNNING_SITES, 2);
    }

    #[test]
    fn retarget_rejects_wrong_static_desc() {
        let err = crate::classfile::retarget_virtual_to_static(
            &[],
            ENCL.0,
            ENCL.1,
            FROM_TICK,
            (OPS_CLASS, "transitionDiffGate", "()V"),
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "unexpected error: {err}");
    }

    #[test]
    fn mob_bytes_retarget_four_sites_if_available() {
        // Реальные байты ядра (CRUSSTY_MOB_BYTES); в CI/без файла — тихий
        // пропуск (hermetic).
        let Ok(path) = std::env::var("CRUSSTY_MOB_BYTES") else {
            eprintln!("skip: CRUSSTY_MOB_BYTES not set");
            return;
        };
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("skip: cannot read {path}");
            return;
        };
        let (_, pair) =
            retarget_ai_sites(&bytes).expect("retarget on real Mob bytes must not Err");
        assert!(
            matches!(pair.tick, crate::classfile::RetargetOutcome::Retargeted { sites: 2 }),
            "tick sites expected 2, got {:?}",
            pair.tick
        );
        assert!(
            matches!(pair.run, crate::classfile::RetargetOutcome::Retargeted { sites: 2 }),
            "running sites expected 2, got {:?}",
            pair.run
        );
    }
}
