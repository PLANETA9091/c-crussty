//! Runtime wiring scaffold for the GOAL canUse PRE-GATE: SENSE-MEMO plane
//! (TASK-459-67, idea ID-P42; lever STRICT `cmp459_p42`; закон 11: агент =
//! вайринг/диспатч/write-through — этот модуль = SCAFFOLD итерации-0:
//! план ретаргета + pure-математика сигнатуры/инвалидации + dormant-гейт).
//!
//! ИДЕЯ (RESEARCH-459-P42.md): memo-хэш требуемых сенсов на goal.
//! В ванили (javap purpur-1.21.10) фаза `goalUpdate` внутри
//! `GoalSelector.tick()V` для КАЖДОЙ не-running цели пере-спрашивает
//! `Goal.canUse()` каждый чётный тик (Paper-сплит; нечётные тики —
//! tickRunningGoals(false), canUse там не зовётся). Для targeting-целей
//! canUse = findTarget → getNearestEntity — sense-чтение полного скана.
//!
//! PRE-GATE: если sense-сигнатура цели не изменилась с последней оценки →
//! `canUse` пропускается, переиспользуется memoized boolean
//! (**superset-инвалидация по sense-событиям**: ВСЕ пишущие сайты сенсов
//! перехвачены sense-плоскостью — SenseOps.nearestEntityGate chokepoint +
//! bulk `senseEpoch` JNI, поднимающий глобальный VERSION-seqlock; «эпоха не
//! двигалась» ⊇ «canUse-входы не менялись»). ЛОЖНЫЙ-ОТРИЦАТЕЛЬНЫЙ ЗАПРЕЩЁН:
//! скрытые мутации сенсов вне плоскости ловит selfTest-инвариант
//! (1/N-тик форс-реоценка поверх memo; расхождение → disarm навсегда);
//! STRICT-off до оракула.
//!
//! ПЛАН РЕТАРГЕТА (итерация-1, НЕ в этом коммите): сайт
//! `invokevirtual Goal.canUse()Z` ВНУТРИ `GoalSelector.tick()V` (фаза
//! goalUpdate) → static
//! `GoalOps.canUseMemoGate(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z`
//! — receiver-prepended stack-identical (канон goal_selector.rs /
//! retarget_virtual_to_static). Сайты tickGate/tickRunningGate (2+2 в
//! serverAiStep) НЕ трогаются — плоскость сидит на один уровень глубже.
//!
//! NCDFE-канон (×448 / bridge_prelist): javap ground truth ДО ретаргета;
//! блоб --release 21 = major 65 (LOADABILITY); define только после
//! find_class(GoalSelector) + boot-marker (не из null-loader фазы);
//! fail-closed pass-through на любой Err; dormant = байт-в-байт ваниль.
//!
//! Гейт: env `CRUSSTY_LEVER_FLAG == "cmp459_p42"` (изолированный флаг — в
//! семейства-носители НЕ добавлен; пустой/чужой флаг = ваниль бит-в-байт).

/// Мост-класс (общий с goal_selector; memo-методы добавлены АДДИТИВНО,
/// tickGate/tickRunningGate не тронуты).
pub const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalOps";

/// Окружающий метод целевого сайта итерации-1: фаза goalUpdate живёт здесь.
pub const ENCL: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/goal/GoalSelector",
    "tick",
    "()V",
);

/// Целевой сайт (v1): invokevirtual Goal.canUse()Z в goalUpdate →
/// GoalOps.canUseMemoGate(WrappedGoal)Z. Receiver-prepended static form.
pub const GATE_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z";

/// STRICT-гейт плоскости (изолированный флаг; НИКАКИХ носителей-OR —
/// полурасставленный мост = невалидная нога, TASK-402-F).
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp459_p42")
    )
}

/// Диагностика для ARM-маркеров (evidence-дисциплина: grep до вердикта).
pub fn memo_ready() -> bool {
    enabled()
}

// ---------------------------------------------------------------------------
// PURE-математика плоскости (тестируется без JVM; java-сторона держит
// байт-в-байт ту же арифметику — расхождение = selfTest disarm).
// ---------------------------------------------------------------------------

/// FNV-1a 64: единый mixing для sense-сигнатур (rust-тест ↔ java-хелпер).
pub const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
pub const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub fn fnv1a(seed: u64, word: u64) -> u64 {
    let mut h = seed;
    for b in word.to_le_bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

/// Per-goal memo-слот (v1 — rust-зеркало для тестов/oracle; живой слот будет
/// держать java-сторона: long[] на GoalSelector-инстанс, ключ = WrappedGoal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoSlot {
    /// FNV-1a(sense-эпоха ⊕ требуемые сенсы) на последней РЕАЛЬНОЙ оценке canUse.
    pub sig: u64,
    /// memoized ответ canUse последней реальной оценки.
    pub memo_use: bool,
    /// Тик последней реальной оценки (selfTest-расписание 1/N).
    pub last_eval_tick: i64,
}

/// Сигнатура требуемых сенсов цели: (сенс-домен, epoch-версия домена).
/// ВАЖНО (ложный-отрицательный): каждое изменение любой пары ДОЛЖНО поднимать
/// epoch соответствующего домена на java-стороне (sense-плоскость) — пропуск
/// сайта записи в плоскости = запрещённый ложный-отрицательный, ловится
/// selfTest'ом → disarm.
pub fn sense_signature(seed: u64, domains: &[(u32, u64)]) -> u64 {
    let mut h = seed;
    for &(dom, epoch) in domains {
        h = fnv1a(h, u64::from(dom));
        h = fnv1a(h, epoch);
    }
    h
}

/// SUPerset-инвалидация (единственное решение пре-гейта):
/// Some(memo_use) — сенсы не менялись, canUse пропускается (memo-ответ);
/// None — сигнатура дрейфнула/слот холодный → реальная оценка canUse
/// (вызов ванили, запись слота — на java-стороне).
pub fn pre_gate(slot: Option<MemoSlot>, sig: u64) -> Option<bool> {
    match slot {
        Some(s) if s.sig == sig => Some(s.memo_use),
        _ => None,
    }
}

/// SelfTest-расписание: форс-реоценка 1/`period` тиков поверх memo
/// (расхождение memo_use ↔ свежий canUse = disarm навсегда, java-лог WARN).
pub fn selftest_due(slot: MemoSlot, tick: i64, period: i64) -> bool {
    period > 0 && slot.last_eval_tick > 0 && (tick - slot.last_eval_tick) >= period
}

// ---------------------------------------------------------------------------
// Вайринг (scaffold: только dormant-регистрация; ретаргет — итерация-1,
// по канону goal_selector::register + activate: find_class → boot-marker →
// define blob (major-гейт) → retransform, compose на ПРИНЯТЫХ байтах).
// ---------------------------------------------------------------------------

/// Register (idempotent; call once from cplugin_init). Dormant-invisible:
/// с чужим/пустым флагом НЕ РЕГИСТРИРУЕТ ничего — байт-в-байт ваниль.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] goal_sense_memo: dormant (lever_flag != cmp459_p42, vanilla canUse)"
        );
        return;
    }
    // Итерация-1: здесь будет cplug_sdk::hooks::register_bytes(ENCL.0, ...)
    // с retarget_virtual_to_static(ENCL, FROM=(Goal,canUse,()Z),
    // (OPS_CLASS,"canUseMemoGate",GATE_DESC)) и счётчиком сайта == 1.
    // Scaffold: STRICT-off до оракула — ничего не ретаргетим.
    eprintln!(
        "[crussty-plugin] goal_sense_memo: ARMED-SOFF (scaffold, cmp459_p42; retarget NOT wired until GoalOps oracle — fail-closed vanilla)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 0x5345_4e53_454d_4f31; // "SENSEMO1"

    #[test]
    fn fnv1a_matches_known_vector() {
        // FNV-1a 64: offset basis + один word — стабильность/детерминизм.
        assert_eq!(fnv1a(FNV_OFFSET, 0), {
            let mut h = FNV_OFFSET;
            for b in 0u64.to_le_bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(FNV_PRIME);
            }
            h
        });
        assert_eq!(fnv1a(1, 7), fnv1a(1, 7));
        assert_ne!(fnv1a(1, 7), fnv1a(1, 8));
    }

    #[test]
    fn superset_invalidation_holds_until_epoch_bump() {
        let sig = sense_signature(SEED, &[(1, 100), (2, 7)]);
        let slot = MemoSlot { sig, memo_use: true, last_eval_tick: 40 };
        // Эпохи доменов не двигались → canUse пропускается, memo-ответ true.
        assert_eq!(pre_gate(Some(slot), sig), Some(true));
        // Любой домен-эпоха дрейфнула (sense-событие) → реальная оценка.
        assert_eq!(pre_gate(Some(slot), sense_signature(SEED, &[(1, 101), (2, 7)])), None);
        assert_eq!(pre_gate(Some(slot), sense_signature(SEED, &[(1, 100), (2, 8)])), None);
        // Новый домен в требуемых сенсах → другая сигнатура → оценка.
        assert_eq!(pre_gate(Some(slot), sense_signature(SEED, &[(1, 100), (2, 7), (3, 0)])), None);
        // Холодный слот → оценка.
        assert_eq!(pre_gate(None, sig), None);
        // Порядок доменов значим (канонизирован на записи слота).
        assert_ne!(
            sense_signature(SEED, &[(1, 100), (2, 7)]),
            sense_signature(SEED, &[(2, 7), (1, 100)])
        );
    }

    #[test]
    fn selftest_schedule_is_periodic_and_cold_safe() {
        let slot = MemoSlot { sig: 1, memo_use: false, last_eval_tick: 100 };
        assert!(selftest_due(slot, 100 + 64, 64));
        assert!(!selftest_due(slot, 100 + 63, 64));
        // Холодный слот (никогда не оценивался) — не дует selfTest.
        let cold = MemoSlot { sig: 1, memo_use: false, last_eval_tick: 0 };
        assert!(!selftest_due(cold, 10_000, 64));
        // Период <= 0 — selfTest выключен (не стреляет).
        assert!(!selftest_due(slot, 100 + 64, 0));
    }
}
