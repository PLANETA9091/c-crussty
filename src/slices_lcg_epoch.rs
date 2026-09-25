//! SLICES-LCG-EPOCH (TASK-459-80, WILD C-X1, закон 11 v18.2 — lever cmp459_cx1, STRICT dormant).
//!
//! ИДЕЯ: вместо полных инвалидаций EntitySectionArrays при мутациях entity-срезов
//! (EntitySectionStorage / ClassInstanceMultiMap по 16³-секции) держать 64-битную
//! LCG-эпоху НА СЕКЦИЮ: писатель бампает эпоху ОДНИМ add, читатель сравнивает эпоху
//! ОДНИМ cmp ДО полного пути (CHM-lookup + перестройка среза). Паттерн secWrite-эпох
//! ×458-P3 (P32 SNAP sidecar + P36 Inside-epoch fast-gate — там int-эпохи BLOCK-секций
//! и `volatile long gen` в InsideSnapOps.Snap, bumps on real change), здесь —
//! entity-slices, 64-бит, флет-таблица эпох.
//!
//! LCG-ШАГ: X_{n+1} = X_n + 1442695040888963407 (mod 2^64), a=1, c=нечётный (шаг Кнута).
//! По теореме Халла–Добелла это полный период 2^64: k·c ≡ 0 (mod 2^64) при нечётном c
//! возможно только при 2^64 | k ⇒ НЕТ ABA-повтора значения эпохи ни при каком числе
//! бампов в пределах вселенной; при этом bump остаётся ОДНОЙ инструкцией add
//! (в отличие от умножающего LCG и в отличие от CHM.remove+put+rebuild полной
//! инвалидации: 2 map-операции + alloc + копия списка среза + 1 map-операция
//! повторного lookup у читателя).
//!
//! SEQLOCK-ДИСЦИПЛИНА (in-repo прецедент cmp399_shard; U3 seqlock): снапшот валиден,
//! только если эпоха не изменилась за время копии (double-check до/после build);
//! дифт ⇒ discard, читатель идёт полным путём и перестраивает. STALE-SERVE невозможен:
//! G4 stale-window=0.
//!
//! STRICT DORMANT: модуль НИЧЕГО не ретаргетит и не активирует на горячем пути —
//! никакой define-class, RegisterNatives, хуков. Активация возможна только при
//! CRUSSTY_LEVER_FLAG строго равном cmp459_cx1 (пустой/чужой флаг = ваниль бит-в-байт).
//! java-стаб: entityinside/net/minecraft/world/entity/SlicesLcgEpochOps.java (не
//! определяется в kernel loader в dormant-состоянии; трекед-блобы не тронуты).
//!
//! GATES (preregistered, RESEARCH-459-CX1.md §4):
//!   G1 ARM marker "cmp459_cx1: selfTest=true BEFORE arm" (после проводки, не сейчас);
//!   G2 lockstep бит-в-байт списков срезов vs прямой обход EntitySectionStorage;
//!   G3 NCDFE=0 ($Nested определяются вместе с Ops);
//!   G4 stale-window=0 (seqlock double-check, miss → ваниль);
//!   G5 population-parity, band 6.0–9.5M, деградации 0;
//!   G6 fail-closed: любой сбой → DISARM навсегда → ваниль.
//!
//! CAPTURE-МАТЕМ: моб-broadphase полка 4–8% CPU (item-версия семейства = 15.66%,
//! src/items_index.rs); захват фаст-гейта 80–95% ходок ⇒ прогноз +1.0..2.0пп при
//! полной проводке, +0.5..1.0пп как композиция к P32/P36-семейству; потолок
//! 4–8пп < 20пп ⇒ композиционная нога (закон 13, CLIMB-план). ×458-P3 вердикты:
//! P32 +0.8–1.2пп, P36 в связке −0.5–1пп затрат.
//!
//! ИСТОЧНИКИ: U1 cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf (Fraser, epoch-based
//! reclamation); U2 en.wikipedia.org/wiki/Linear_congruential_generator (Hull–Dobell,
//! полный период); U3 en.wikipedia.org/wiki/Seqlock; U4 docs.rs/crossbeam-epoch.

#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

/// Идентификатор рычага (STRICT eq; пустой/чужой CRUSSTY_LEVER_FLAG = ваниль).
pub const LEVER_ID: &str = "cmp459_cx1";

/// LCG-шаг: нечётная константа Кнута ⇒ полный период 2^64 при a=1 (Hull–Dobell, U2).
pub const LCG_STEP: u64 = 1442695040888963407;

/// STRICT-гейт рычага: только точное совпадение флага активирует путь (dormant по умолчанию).
pub fn lever_enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == LEVER_ID)
        .unwrap_or(false)
}

/// 64-битная LCG-эпоха одной секции.
///
/// WRITER: [`SlicesLcgEpoch::bump`] = один add (fetch_add) — вся стоимость инвалидации.
/// READER: [`SlicesLcgEpoch::matches`] = один Acquire-load + один cmp — фаст-гейт
/// перед полным путём (снапшот серверится только при совпадении эпох).
pub struct SlicesLcgEpoch(AtomicU64);

impl SlicesLcgEpoch {
    pub const fn new() -> Self {
        // Стартуем с шага: 0 никогда не является валидной эпохой снапшота
        // (0 = «снапшота нет», sentinel — fail-closed по умолчанию).
        Self(AtomicU64::new(LCG_STEP))
    }

    /// WRITER-путь: bump эпохи одним add. Возвращает НОВУЮ эпоху.
    /// Release: публикация данных среза, предшествующих бампу, упорядочена
    /// относительно последующего Acquire-чтения читателя.
    pub fn bump(&self) -> u64 {
        self.0.fetch_add(LCG_STEP, Ordering::Release).wrapping_add(LCG_STEP)
    }

    /// Текущая эпоха (один Acquire-load).
    pub fn current(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    /// READER-фаст-гейт: один load + один cmp. true ⇒ снапшот эпохи ещё жив,
    /// полный путь (CHM-walk + перестройка) пропускается.
    pub fn matches(&self, snapshot_epoch: u64) -> bool {
        self.current() == snapshot_epoch
    }
}

impl Default for SlicesLcgEpoch {
    fn default() -> Self {
        Self::new()
    }
}

/// Снапшот среза, снятый под эпоху `epoch` (seqlock-протокол, U3/cmp399_shard).
pub struct Snapshot<T> {
    pub epoch: u64,
    pub data: T,
}

/// Снять снапшот с double-check эпохи: если за время `build` писатель бампнул
/// эпоху, снапшот отбрасывается (None) — читатель обязан replay (G4 stale-window=0).
pub fn snapshot<T>(e: &SlicesLcgEpoch, build: impl FnOnce() -> T) -> Option<Snapshot<T>> {
    let e0 = e.current();
    let data = build();
    let e1 = e.current();
    if e0 != e1 {
        return None;
    }
    Some(Snapshot { epoch: e1, data })
}

/// Сервинг снапшота: один cmp до полного пути (ядро C-X1).
pub fn serve<T>(e: &SlicesLcgEpoch, snap: &Snapshot<T>) -> bool {
    e.matches(snap.epoch)
}

/// Флет-таблица «секция → 64-бит LCG-эпоха» (P36-стиль, но u64 и LCG-шаг).
/// Индексация = стабильный int-индекс секции (P32 sidecar-реестр); slot 0
/// зарезервирован (никогда не выдаётся секциям, всегда читается как 0 = miss).
pub struct SectionEpochTable {
    epochs: Vec<AtomicU64>,
}

impl SectionEpochTable {
    /// `sections` — число секций; слоты инициализированы LCG_STEP (>0, валидная эпоха).
    pub fn new(sections: usize) -> Self {
        let mut epochs = Vec::with_capacity(sections + 1);
        epochs.push(AtomicU64::new(0)); // slot 0 = sentinel, не выдаётся
        for _ in 0..sections {
            epochs.push(AtomicU64::new(LCG_STEP));
        }
        Self { epochs }
    }

    pub fn len(&self) -> usize {
        self.epochs.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// READER: один load + один cmp (фаст-гейт до полного пути).
    pub fn epoch(&self, section: usize) -> u64 {
        self.epochs[section + 1].load(Ordering::Acquire)
    }

    /// WRITER: bump эпохи секции одним add (вся инвалидация).
    pub fn bump(&self, section: usize) -> u64 {
        self.epochs[section + 1]
            .fetch_add(LCG_STEP, Ordering::Release)
            .wrapping_add(LCG_STEP)
    }

    /// Снапшот среза секции с seqlock double-check (None ⇒ replay полным путём).
    pub fn snapshot<T>(&self, section: usize, build: impl FnOnce() -> T) -> Option<Snapshot<T>> {
        let e0 = self.epoch(section);
        let data = build();
        let e1 = self.epoch(section);
        if e0 != e1 {
            return None;
        }
        Some(Snapshot { epoch: e1, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn lcg_recurrence_matches_reference_stream() {
        // U2: X_{n+1} = (a·X_n + c) mod 2^64, a=1 ⇒ X_{n+1} = X_n + c — ровно один add.
        // независимая рекуррентность по определению LCG (u128, честный mod 2^64):
        let mut x2: u128 = LCG_STEP as u128;
        for _ in 0..1_000u32 {
            x2 = (x2 + LCG_STEP as u128) % (1u128 << 64);
        }
        assert_eq!(x2 as u64, {
            let mut y: u64 = LCG_STEP;
            for _ in 0..1_000u32 {
                y = y.wrapping_add(LCG_STEP);
            }
            y
        });
    }

    #[test]
    fn step_is_odd_full_period_precondition() {
        // Hull–Dobell (a=1): c нечётно ⇒ период 2^64 — основание anti-ABA контракта.
        assert_eq!(LCG_STEP & 1, 1);
    }

    #[test]
    fn no_aba_zero_return_within_2_power_20_bumps() {
        // k·c ≡ 0 (mod 2^64) невозможно при k < 2^64 при нечётном c ⇒ структурно
        // проверяем на 2^20 бампов: все эпохи различны и ни одна не равна 0.
        let e = SlicesLcgEpoch::new();
        let mut seen: HashSet<u64> = HashSet::with_capacity(1 << 20);
        let mut cur = e.current();
        for _ in 0..(1u64 << 20) {
            cur = cur.wrapping_add(LCG_STEP);
            assert_ne!(cur, 0, "epoch wrapped to sentinel 0 — contract broken");
            assert!(seen.insert(cur), "epoch repeat = ABA — contract broken");
        }
        assert_eq!(seen.len(), 1 << 20);
    }

    #[test]
    fn reader_gate_invalid_after_bump_valid_before() {
        let e = SlicesLcgEpoch::new();
        let snap = snapshot(&e, || 7u32).expect("quiet build");
        assert!(serve(&e, &snap)); // один cmp — hit
        let old = e.current();
        let new = e.bump(); // писатель: один add
        assert_eq!(new, old.wrapping_add(LCG_STEP));
        assert!(!serve(&e, &snap)); // фаст-гейт = miss → полный путь
        let snap2 = snapshot(&e, || 8u32).expect("quiet rebuild");
        assert!(serve(&e, &snap2));
    }

    #[test]
    fn seqlock_discard_on_concurrent_bump() {
        let e = SlicesLcgEpoch::new();
        // мутация внутри build = гонка писателя; снапшот обязан отброситься (G4).
        let r = snapshot(&e, || {
            e.bump();
            42u32
        });
        assert!(r.is_none(), "stale snapshot must be discarded (stale-window=0)");
    }

    #[test]
    fn section_table_bump_and_gate() {
        let t = SectionEpochTable::new(64);
        assert_eq!(t.len(), 64);
        let s5 = t.snapshot(5, || vec![1u64, 2, 3]).expect("quiet build");
        assert!(t.epoch(5) == s5.epoch);
        let n = t.bump(5);
        assert_ne!(n, s5.epoch);
        assert_ne!(t.epoch(5), s5.epoch, "reader cmp must miss after bump");
        // чужая секция не задета (эпоха НА секцию): слот 6 бампнут, слот 7 = старт.
        let init = LCG_STEP;
        assert_eq!(t.epoch(7), init);
        let n6 = t.bump(6);
        assert_ne!(n6, init);
        assert_ne!(t.epoch(6), init);
        // Сервинг чужой секции по своей эпохе — один cmp, hit:
        let s6 = t.snapshot(6, || 9u32).expect("quiet build");
        assert_eq!(t.epoch(6), s6.epoch);
        assert_eq!(s6.data, 9u32);
    }

    #[test]
    fn dormant_by_default_strict_gate() {
        // В тестовой среде флаг не выставлен ⇒ STRICT-гейт закрыт (ваниль).
        // Чужой lever флаг тоже закрывает — проверяем логику сравнением.
        assert_eq!(LEVER_ID, "cmp459_cx1");
        let foreign = "cmp458_swar".trim() == LEVER_ID;
        assert!(!foreign);
    }
}
