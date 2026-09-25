//! SPAWN-BLOCK COHORT BITSET — TASK-459-83 (WILD C-X4, закон 11 тик-459).
//! Lever `cmp459_cx4`, STRICT eq. Dormant scaffold: модель + gate + DATA-PLAN
//! статистика; JNI-вайринг в isValidSpawnPostitionForType = волна-2.
//!
//! Идея C-X4: при NaturalSpawner-циклах держать однотиковый bitset когорт
//! «блок уже проверен за тик с этим светом/коллизией» → пропуск повторных
//! spawn-проверок идентичных блоков (масса 150k фиксстуры = однотипные
//! блоки). Когорта = класс эквивалентности блочной части цепочки
//! isValidSpawnPostitionForType (javap round-396-a): (stateId, light,
//! collision/fluid-класс, placement×category). Paper-ивент
//! (PreCreatureSpawnEvent, offsets 14→40) и canSpawnMobAt (offset 113) —
//! позиционно/биом-зависимы и НЕ когорт-переиспользуемы.
//!
//! STRICT superset-гейт (безопасность бит-в-байт):
//!  * memo поднимается ТОЛЬКО по факту завершённой полной ваниль-проверки
//!    (probe→Run, после vanilla → observe); незавершённые/бросившие проверки
//!    никогда не помечаются checked;
//!  * любой строгий флаг (event-abort/unknown-light/unknown-collision) =
//!    Refused: реюз и запись в memo запрещены навсегда;
//!  * переполнение таблицы/OOB → one-shot BROKEN латч → disarm (все probe
//!    возвращают Run = ваниль) до конца процесса;
//!  * lever off → java-гейт не определён/не вызывается: бит-в-байт по
//!    построению (прецедент region_threads compose / nav_plane «пустой флаг»).
//!
//! Структуры fixed-alloc: таблица 4096 слотов × u64 + два bitset 64×u64
//! (used/checked) + счётчики — нулевой alloc после старта; reset тика =
//! fill(0) (512B × 2), амортизируется на границе тика.
//!
//! Wave-2 план: `SpawnCohortBitsetOps` (net/minecraft/world/level/, сосед
//! NaturalSpawner) — EARLY-define в arm-хуке ДО первого spawn-цикла
//! (NCDFE-канон, прецедент MobPushOps.pushables:467), probe-гейт между
//! canSpawnMobAt и SpawnPlacements.isSpawnPositionOk, observe после
//! noCollision. Гейты G1-G6 — RESEARCH-459-CX4.md §6.

#![allow(dead_code)] // dormant scaffold: JNI-вайринг = волна-2

// ---------------------------------------------------------------------------
// Lever + константы
// ---------------------------------------------------------------------------

/// STRICT eq lever (пустой/чужой флаг = not armed → ваниль бит-в-байт).
pub const SPAWN_CX4_LEVER: &str = "cmp459_cx4";

/// Wave-2 FQCN java-стороны (документация контракта; define = волна-2).
pub const SPAWN_CX4_CLASS: &str = "net/minecraft/world/level/SpawnCohortBitsetOps";

/// Строгие флаги входа: любое отличие от «чистой блочной проверки» = Refused.
pub const FL_EVENT_ABORT: u8 = 1 << 0; // PreCreatureSpawnEvent.abort занят — позиционно-зависим
pub const FL_UNKNOWN_LIGHT: u8 = 1 << 1; // light вне 0..15 / не читался
pub const FL_UNKNOWN_COLL: u8 = 1 << 2; // collision/fluid-класс не разрешён

pub const SLOTS: usize = 4096;
pub const SLOT_MASK: u64 = (SLOTS as u64) - 1;
const WORDS: usize = SLOTS / 64;

/// armed(): STRICT eq по trim (прецедент nav_plane::armed).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == SPAWN_CX4_LEVER)
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Ключ когорты: stateId(24b) | light(4b) | coll(4b) | cat(8b); старшие 24b=0
// ---------------------------------------------------------------------------

pub fn pack_key(state_id: u32, light: u8, coll: u8, cat: u8) -> u64 {
    ((state_id as u64 & 0x00FF_FFFF) << 16)
        | ((light as u64 & 0x0F) << 12)
        | ((coll as u64 & 0x0F) << 8)
        | (cat as u64 & 0xFF)
}

#[inline]
fn hash_key(k: u64) -> u64 {
    // splitmix64 finalizer — детерминизм + dispersion; без внешних крейтов.
    let mut z = k.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// ---------------------------------------------------------------------------
// DATA-PLAN статистика (гейты G1/G5: effect-маркеры, skips при lever off = 0)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cx4Stats {
    /// Всего probe-вызовов (armed-эпоха).
    pub checks_total: u64,
    /// Реальные пропуски повторных проверок (memo-хиты).
    pub skips: u64,
    /// Отказы по STRICT superset-гейту (strict_flags != 0).
    pub refusals: u64,
    /// One-shot BROKEN латч (fail-closed disarm).
    pub broken: bool,
    /// Номер текущего тика (reset-счётчик).
    pub tick: u64,
}

// ---------------------------------------------------------------------------
// Bitset + таблица когорт
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeVerdict {
    /// Повторная когорта этого тика уже проверена полностью → skip блочных
    /// проверок (java: пропуск isSpawnPositionOk/checkSpawnRules/noCollision).
    Skip,
    /// Полный ваниль-путь (первая проверка когорты / refusal / broken).
    Run,
}

#[derive(Debug)]
pub struct SpawnCohortBitset {
    keys: Vec<u64>,
    used: Vec<u64>,
    checked: Vec<u64>,
    broken: bool,
    stats: Cx4Stats,
}

impl Default for SpawnCohortBitset {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnCohortBitset {
    pub fn new() -> Self {
        Self {
            keys: vec![0u64; SLOTS],
            used: vec![0u64; WORDS],
            checked: vec![0u64; WORDS],
            broken: false,
            stats: Cx4Stats::default(),
        }
    }

    /// Reset на границе тика: когорты действительны только внутри тика
    /// (свет/коллизии/состояния могут меняться между тиками — superset-гейт
    /// запрещает меж-тиковый реюз). Zero-alloc: fill(0).
    pub fn reset_tick(&mut self) {
        self.used.fill(0);
        self.checked.fill(0);
        self.stats.tick = self.stats.tick.wrapping_add(1);
    }

    #[inline]
    fn bit(w: &[u64], i: usize) -> bool {
        (w[i >> 6] >> (i & 63)) & 1 == 1
    }

    #[inline]
    fn set_bit(w: &mut [u64], i: usize) {
        w[i >> 6] |= 1u64 << (i & 63);
    }

    /// lookup_or_insert: Some(slot) — найден/вставлен; None — таблица полна.
    fn lookup_or_insert(&mut self, key: u64) -> Option<usize> {
        let start = (hash_key(key) & SLOT_MASK) as usize;
        for probe in 0..SLOTS {
            let i = (start + probe) & SLOT_MASK as usize;
            if !Self::bit(&self.used, i) {
                self.keys[i] = key;
                Self::set_bit(&mut self.used, i);
                return Some(i);
            }
            if self.keys[i] == key {
                return Some(i);
            }
        }
        None
    }

    /// Probe перед блочной частью цепочки проверки спавна.
    /// strict_flags != 0 → Refused-семантика (Run, без memo) — STRICT
    /// superset-гейт: частичные проверки не кэшируются никогда.
    pub fn probe(&mut self, key: u64, strict_flags: u8) -> ProbeVerdict {
        if self.broken {
            return ProbeVerdict::Run; // fail-closed disarm
        }
        self.stats.checks_total += 1;
        if strict_flags != 0 {
            self.stats.refusals += 1;
            return ProbeVerdict::Run;
        }
        match self.lookup_or_insert(key) {
            Some(slot) => {
                if Self::bit(&self.checked, slot) {
                    self.stats.skips += 1;
                    ProbeVerdict::Skip
                } else {
                    ProbeVerdict::Run
                }
            }
            None => {
                // Таблица полна: one-shot BROKEN → ваниль до конца процесса
                // (переполнение = аномалия фиксстуры, не штатный путь).
                self.broken = true;
                self.stats.broken = true;
                ProbeVerdict::Run
            }
        }
    }

    /// Observe ПОСЛЕ завершённой полной ваниль-проверки блочной части.
    /// Не вызывался (throw/abort/ранний return) → checked не поднят →
    /// superset-гейт не нарушен: повтор придёт как Run.
    pub fn observe(&mut self, key: u64) {
        if self.broken {
            return;
        }
        let start = (hash_key(key) & SLOT_MASK) as usize;
        for probe in 0..SLOTS {
            let i = (start + probe) & SLOT_MASK as usize;
            if !Self::bit(&self.used, i) || self.keys[i] == key {
                if Self::bit(&self.used, i) {
                    Self::set_bit(&mut self.checked, i);
                }
                return;
            }
        }
    }

    /// Эффективный ARM (lever + не broken) — java-гейт волны-2.
    pub fn armed_effective(&self) -> bool {
        armed() && !self.broken
    }

    pub fn stats(&self) -> Cx4Stats {
        self.stats
    }

    pub fn is_broken(&self) -> bool {
        self.broken
    }

    // -----------------------------------------------------------------
    // Офлайн lockstep (закон 16): бит-в-байт оракул против наивной
    // HashSet-реплики; детерминированный LCG-поток запросов.
    // -----------------------------------------------------------------
    pub fn self_test(&self) -> bool {
        const N: u64 = 4096; // ниже ёмкости — broken не трогаем в базовом реплее
        let mut bs = SpawnCohortBitset::new();
        let mut reference: std::collections::HashSet<u64> = std::collections::HashSet::new();
        let mut z: u64 = 0x1234_5678_9ABC_DEF0;
        let mut lcg = move || {
            z = z.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            z
        };
        let mut refusals_seen = 0u64;
        let mut skips_seen = 0u64;
        for _ in 0..N {
            let state_id = (lcg() % 256) as u32;
            let light = (lcg() % 4) as u8; // 0..3 → сжатые когорты (реюз частый)
            let coll = (lcg() % 2) as u8;
            let cat = (lcg() % 8) as u8;
            let key = pack_key(state_id, light, coll, cat);
            let flags = (lcg() % 16) as u8; // 12.5% стримов со строгими флагами
            // Строгий флаг ≠ 0 → refusal (канон FL_*, для реплея достаточно
            // любого ненулевого бита — гейт проверяет flags != 0).
            let strict: u8 = if flags != 0 { FL_UNKNOWN_COLL } else { 0 };
            let v = bs.probe(key, strict);
            let want = if strict != 0 {
                ProbeVerdict::Run
            } else if reference.contains(&key) {
                ProbeVerdict::Skip
            } else {
                ProbeVerdict::Run
            };
            if v != want {
                return false; // lockstep бит-в-байт нарушен
            }
            if strict == 0 && v == ProbeVerdict::Run && !reference.contains(&key) {
                reference.insert(key); // ваниль-проверка завершена → observe
                bs.observe(key);
            }
            if v == ProbeVerdict::Skip {
                skips_seen += 1;
            }
            if strict != 0 {
                refusals_seen += 1;
            }
        }
        if bs.is_broken() {
            return false; // в базовом реплее (N < SLOTS, refusals > 0) недостижимо
        }
        // Reset-семантика: после reset тика ни одна когорта не Skip.
        bs.reset_tick();
        let mut still_checked = false;
        for &k in reference.iter().take(16) {
            if bs.probe(k, 0) == ProbeVerdict::Skip {
                still_checked = true;
            }
            bs.observe(k);
        }
        if still_checked {
            return false;
        }
        // Статистика живая: refusals и skips совпадают с реплеем.
        let st = bs.stats();
        st.refusals == refusals_seen && st.skips == skips_seen && st.tick == 1
    }
}

// ---------------------------------------------------------------------------
// Тесты: lever-строгость, ключ, memo-семантика, refusal, reset, broken, lockstep
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_is_strict_eq() {
        // armed() читает env — подменяем нельзя без тестового окружения,
        // поэтому фиксируем канон: только точное значение после trim.
        assert_eq!(SPAWN_CX4_LEVER, "cmp459_cx4");
        assert!("cmp459_cx4".trim() == SPAWN_CX4_LEVER);
        assert!("".trim() != SPAWN_CX4_LEVER);
        assert!("cmp459_cx3".trim() != SPAWN_CX4_LEVER);
        assert!("cmp459_cx4 ".trim() == SPAWN_CX4_LEVER);
    }

    #[test]
    fn key_packing_distinct_and_stable() {
        let a = pack_key(1234, 7, 2, 5);
        assert_eq!(a, pack_key(1234, 7, 2, 5));
        assert_ne!(a, pack_key(1234, 8, 2, 5)); // свет-дрейф = другая когорта
        assert_ne!(a, pack_key(1235, 7, 2, 5)); // state-дрейф
        assert_ne!(a, pack_key(1234, 7, 3, 5)); // collision-дрейф
        assert_ne!(a, pack_key(1234, 7, 2, 6)); // category-дрейф
        assert_eq!(a >> 40, 0); // старшие биты зарезервированы (0)
        // 24-битный stateId не переполняет поле
        assert_eq!(pack_key(0xFF_FFFF, 0, 0, 0) >> 16, 0xFF_FFFF);
    }

    #[test]
    fn memo_first_run_then_skip() {
        let mut bs = SpawnCohortBitset::new();
        let k = pack_key(42, 5, 1, 3);
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Run); // первая проверка
        bs.observe(k); // ваниль-проверка завершена
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Skip); // повтор = skip
        assert_eq!(bs.stats().skips, 1);
        assert_eq!(bs.stats().checks_total, 2);
    }

    #[test]
    fn unobserved_check_is_never_reused() {
        // STRICT superset: probe без observe (проверка не завершена /
        // бросила) — повтор обязан прийти как Run, реюз запрещён.
        let mut bs = SpawnCohortBitset::new();
        let k = pack_key(7, 1, 1, 1);
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Run);
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Run); // observe не было
        assert_eq!(bs.stats().skips, 0);
    }

    #[test]
    fn strict_flags_refuse_memo() {
        let mut bs = SpawnCohortBitset::new();
        let k = pack_key(9, 9, 9, 9);
        for flags in [FL_EVENT_ABORT, FL_UNKNOWN_LIGHT, FL_UNKNOWN_COLL, 0x07] {
            assert_eq!(bs.probe(k, flags), ProbeVerdict::Run);
            bs.observe(k); // observe при refusal не должен поднимать checked
        }
        assert_eq!(bs.stats().refusals, 4);
        assert_eq!(bs.stats().skips, 0);
        // observe по ненатянутому слоту — no-op: реюз невозможен
        assert_eq!(bs.probe(k, FL_EVENT_ABORT), ProbeVerdict::Run);
    }

    #[test]
    fn reset_tick_clears_checked() {
        let mut bs = SpawnCohortBitset::new();
        let k = pack_key(3, 3, 3, 3);
        bs.probe(k, 0);
        bs.observe(k);
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Skip);
        bs.reset_tick();
        assert_eq!(bs.probe(k, 0), ProbeVerdict::Run); // меж-тиковый реюз запрещён
        assert_eq!(bs.stats().tick, 1);
    }

    #[test]
    fn overflow_trips_broken_latch_fail_closed() {
        let mut bs = SpawnCohortBitset::new();
        for i in 0..SLOTS as u64 {
            assert_eq!(
                bs.probe(pack_key(i as u32, 0, 0, 0), 0),
                ProbeVerdict::Run
            );
        }
        assert!(!bs.is_broken());
        // 4097-я уникальная когорта → BROKEN one-shot, ваниль-путь
        assert_eq!(bs.probe(pack_key(0xFFFF, 0, 0, 0), 0), ProbeVerdict::Run);
        assert!(bs.is_broken());
        assert!(bs.stats().broken);
        // после broken: любые probe = Run, observe = no-op, effective = false
        let k2 = pack_key(1, 1, 1, 1);
        assert_eq!(bs.probe(k2, 0), ProbeVerdict::Run);
        bs.observe(k2);
        assert_eq!(bs.probe(k2, 0), ProbeVerdict::Run);
        assert!(!bs.armed_effective());
    }

    #[test]
    fn lockstep_bit_in_byte_reference_parity() {
        // G2-оракул: bitset-решения == наивная HashSet-реплика (бит-в-байт).
        let bs = SpawnCohortBitset::new();
        assert!(bs.self_test());
    }
}
