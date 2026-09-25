//! HILBERT STORAGE ORDER (ID-H07, TASK-459-77 — закон 11 WILD, STRICT dormant).
//!
//! ПОДСИСТЕМА (закон 6 — НЕ одиночная функция): пре-сортировка индексов
//! entity-слотов ВНУТРИ секции (16³ EntitySection) в Hilbert-порядке
//! координат для локальности кэша последовательного storage-скана.
//!
//! КЛЮЧЕВОЙ КОНТРАКТ ПАРИТЕТА (карточка ID-H07, свободна из ×457): порядок
//! ВЫДАЧИ обязан остаться ванильным ⇒ H07 НЕ трогает выдачу и НЕ меняет
//! последовательность вызовов — реордерится только ФИЗИЧЕСКИЙ порядок
//! ХРАНЕНИЯ слотов (reorder storage, не выдачу). Выдача per-call = то же
//! мульти-множество сущностей, те же счётчики, тот же ранний выход count;
//! меняется лишь то, какой слот лежит на каком индексе — а ванильный скан
//! ходит по индексам последовательно и получает пространственно-локальные
//! сущности подряд (Hilbert-кривая: locality-preserving 1D↔2D mapping).
//!
//! Джавап-контракт цели (LEDGER-459-L11): EntityCollectionBySection
//! .getEntities = plain count early-out + y-clamp + storage-скан с null-
//! дырами — последовательный обход массива, ИДЕАЛЬНЫЙ потребитель пре-
//! сортировки. Reader-core broadphase-лейна = 4.0% CPU (getEntities 1120 +
//! getHardColliding 926 + intersects 871 сэмплов из 103062; лейн 9.36%).
//!
//! ГРАУНД-ТРУТ (RESEARCH-459-H07.md, ≥2 URL): geo-index 0.4.0 (agent-J
//! компилит: RTreeBuilder + HilbertSort, leaf indices = hilbert-порядок) —
//! packed immutable R-tree, bulk-load only; flatbush = «efficient
//! implementation of the packed Hilbert R-tree»; Hilbert R-tree (wiki) —
//! space-filling curve навязывает линейный порядок для кластеризации;
//! Hilbert curve (wiki) — «preserves locality fairly well».
//!
//! ГРАНУЛЯРНОСТЬ КЛЮЧА: 2D-Hilbert по (x,z) внутри секции (SECTION_BITS=4,
//! как в geo-index — 2D-only, y НЕ индексируется), y — минорный ключ
//! (4 бита хвоста). 2D выбран ИЗ-ЗА контракта цель-семейства: broadphase-
//! боксы прунятся по x/z, вертикальный дрейф не ограничен маржой.
//!
//! ПАРИТЕТ-СТРАЖ (bit-в-байт оракул будущей lockstep-цензы):
//!   - порядок = чистая ПЕРЕСТАНОВКА (биекция) тех же слотов;
//!   - мульти-множество ключей после переукладки == до (same multiset);
//!   - стабильная сортировка: одинаковые ключи сохраняют относительный
//!     порядок (двойники-координаты не меняются местами);
//!   - java-зеркало HilbertStorageOps.java (stub, vanilla-free) — будущая
//!     бит-в-байт сверка плана rust vs java ДО любого define (NCDFE-канон).
//!
//! STRICT DORMANT: lever `CRUSSTY_HILBERT_STORAGE` (env), OFF by default;
//! register() пишет dormant-notice и возвращается; НЕТ JNI, НЕТ define,
//! НЕТ чтения байтов кернела, НЕТ хуков — модуль невидим для ванильного
//! рантайма по построению. Wiring-фаза (отдельный коммит): java-лестница
//! секционного реордера + lockstep-ценз, до них — scaffold-stop.

// Scaffold-поверхность (план/гарды/статы) оживает в wiring-фазе —
// dead_code до неё ожидаем, не шумим в CI.
#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

/// Lever-гейт подсистемы (env, чтение на вызов — дисциплина inside_cache).
pub const LEVER_ENV: &str = "CRUSSTY_HILBERT_STORAGE";

/// Секция 16³: битов на ось внутри секции.
pub const SECTION_BITS: u32 = 4;

/// Минорный хвост y-ключа (16 уровней секции).
const Y_MINOR_BITS: u32 = 4;

// ── Статистика (diag; LiveSet-паттерн entity_index.rs) ─────────────────────
static SECTIONS_PLANNED: AtomicU64 = AtomicU64::new(0);
static SLOTS_PLANNED: AtomicU64 = AtomicU64::new(0);
static GUARD_REJECTS: AtomicU64 = AtomicU64::new(0);

/// (sections_planned, slots_planned, guard_rejects).
pub fn stats() -> (u64, u64, u64) {
    (
        SECTIONS_PLANNED.load(Ordering::Relaxed),
        SLOTS_PLANNED.load(Ordering::Relaxed),
        GUARD_REJECTS.load(Ordering::Relaxed),
    )
}

pub fn enabled() -> bool {
    std::env::var(LEVER_ENV)
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Register (idempotent; call once from cplugin_init). НИКАКОГО define/хука:
/// scaffold-stop — wiring-фаза отдельно, до неё модуль только считает свои
/// планы, если его спросят тесты.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] hilbert_storage_order: dormant (set CRUSSTY_HILBERT_STORAGE=1 to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] hilbert_storage_order: scaffold armed-flat (H07; storage-plan only, reorder ladder lands in wiring phase)"
    );
}

// ── Hilbert-математика (2D, канон xy2d — Wikipedia/Hilbert curve) ──────────

/// Поворот/отражение квадранта (канон rot() из d2xy-реализации).
fn hilbert_rot(n: u32, x: &mut u32, z: &mut u32, rx: u32, rz: u32) {
    if rz == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *z = n - 1 - *z;
        }
        let t = *x;
        *x = *z;
        *z = t;
    }
}

/// Координаты внутри сетки `1<<bits` → Hilbert-индекс d (биекция
/// [0,n)² → [0,n²)). bits ≤ 15 (s-цикл — старший бит координат).
pub fn hilbert_xz2d(x: u32, z: u32, bits: u32) -> u64 {
    debug_assert!((1..=15).contains(&bits), "hilbert bits out of scaffold range");
    let n = 1u32 << bits;
    let mut x = x;
    let mut z = z;
    let mut d: u64 = 0;
    let mut s = n >> 1;
    while s > 0 {
        let rx = u32::from((x & s) != 0);
        let rz = u32::from((z & s) != 0);
        d += (s as u64) * (s as u64) * (((3 * rx) ^ rz) as u64);
        hilbert_rot(n, &mut x, &mut z, rx, rz);
        s >>= 1;
    }
    d
}

/// Полный ключ слота секции: Hilbert(x,z) мажор, y минор.
/// y в пределах 0..16 секции; старшие биты y игнорируются (16³-гранулярность).
pub fn section_slot_key(x: u16, y: u16, z: u16) -> u64 {
    (hilbert_xz2d(x as u32, z as u32, SECTION_BITS) << Y_MINOR_BITS) | ((y & 0xF) as u64)
}

// ── План пре-сортировки секции ─────────────────────────────────────────────

/// План реордера ХРАНЕНИЯ одной секции: `order[r] = слот-индекс`, который
/// после переукладки занимает ранг r последовательного скана. Сама укладка
/// (кому куда переехать) — wiring-фаза; здесь — чистая математика + гарды.
pub struct SectionPlan {
    pub order: Vec<u32>,
}

impl SectionPlan {
    /// Переукладка слотов по плану (чистая перестановка: то же
    /// мульти-множество значений, `slots[order[r]]` на ранге r).
    /// Используется selfTest/цензой; прод-выдача её НЕ вызывает.
    pub fn permute<T: Copy>(&self, slots: &[T]) -> Vec<T> {
        debug_assert!(is_bijection(&self.order, slots.len()));
        self.order.iter().map(|&i| slots[i as usize]).collect()
    }
}

/// Биекция-страж: длина совпадает, каждый индекс в границах, без дублей.
pub fn is_bijection(order: &[u32], len: usize) -> bool {
    if order.len() != len {
        return false;
    }
    let mut seen = vec![false; len];
    for &i in order {
        let i = i as usize;
        if i >= len || seen[i] {
            return false;
        }
        seen[i] = true;
    }
    true
}

/// Мульти-множество значений переукладки == исходное (паритет per-call
/// выдачи: same entities, same counts). Строгий вариант для u64-ключей
/// (прод-контракт; произвольный T — через тестовое permute+sort).
pub fn multiset_preserved(slots: &[u64], plan: &SectionPlan) -> bool {
    if !is_bijection(&plan.order, slots.len()) {
        return false;
    }
    let mut a = slots.to_vec();
    a.sort_unstable();
    let mut b = plan.permute(slots);
    b.sort_unstable();
    a == b
}

/// План секции по координатам слотов (x,y,z внутри 16³): стабильная
/// сортировка индексов по section_slot_key. Одинаковые ключи (двойники-
/// координаты) сохраняют относительный порядок — перестановка остаётся
/// чистым реордером ХРАНЕНИЯ без переопределения порядка among twins.
pub fn build_section_plan(coords: &[(u16, u16, u16)]) -> SectionPlan {
    let keys: Vec<u64> = coords
        .iter()
        .map(|&(x, y, z)| section_slot_key(x, y, z))
        .collect();
    let mut order: Vec<u32> = (0..coords.len() as u32).collect();
    order.sort_by_key(|&i| keys[i as usize]);

    // Fail-closed: план-гард обязан держать биекцию всегда; иначе — reject
    // (план не публикуется, счётчик растёт, статус = без плана = ваниль).
    if !is_bijection(&order, coords.len()) {
        GUARD_REJECTS.fetch_add(1, Ordering::Relaxed);
        return SectionPlan { order: Vec::new() };
    }
    SECTIONS_PLANNED.fetch_add(1, Ordering::Relaxed);
    SLOTS_PLANNED.fetch_add(coords.len() as u64, Ordering::Relaxed);
    SectionPlan { order }
}

// ── SelfTest (cargo test / будущая offline-цеза) ───────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hilbert_is_bijection_and_local() {
        let bits = 4u32;
        let n = 1usize << bits;
        // Биекция: все 256 клеток → 256 различных d.
        let mut seen = vec![false; n * n];
        for x in 0..n as u32 {
            for z in 0..n as u32 {
                let d = hilbert_xz2d(x, z, bits) as usize;
                assert!(d < n * n && !seen[d]);
                seen[d] = true;
            }
        }
        // Локальность: соседние d дают клетки с чебышёв-дистанцией 1.
        let mut pos = vec![(0u32, 0u32); n * n];
        for x in 0..n as u32 {
            for z in 0..n as u32 {
                pos[hilbert_xz2d(x, z, bits) as usize] = (x, z);
            }
        }
        for w in pos.windows(2) {
            let (ax, az) = w[0];
            let (bx, bz) = w[1];
            let dx = (ax as i64 - bx as i64).abs();
            let dz = (az as i64 - bz as i64).abs();
            assert!(dx <= 1 && dz <= 1, "hilbert jump {dx}x{dz}");
        }
    }

    #[test]
    fn plan_is_pure_permutation_and_sorted() {
        // Детерминированный LCG-сэмпл секции (~120 слотов, плотность бенча).
        let mut st: u64 = 0x455F_4830_7713_5EED; // "H07" seed
        let mut coords = Vec::with_capacity(120);
        for _ in 0..120 {
            st = st
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let x = ((st >> 33) & 0xF) as u16;
            let y = ((st >> 37) & 0xF) as u16;
            let z = ((st >> 41) & 0xF) as u16;
            coords.push((x, y, z));
        }
        let keys: Vec<u64> = coords
            .iter()
            .map(|&(x, y, z)| section_slot_key(x, y, z))
            .collect();
        let plan = build_section_plan(&coords);
        assert!(is_bijection(&plan.order, coords.len()));
        // Мульти-множество ключей сохранено (паритет выдачи).
        assert!(multiset_preserved(&keys, &plan));
        // План отсортирован по ключу (Hilbert-порядок скана).
        let perm = plan.permute(&keys);
        assert!(perm.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn y_is_minor_key_twins_stable() {
        // Одинаковые (x,z), разные y: минорный порядок y.
        for x in 0..16u16 {
            for z in 0..16u16 {
                let ks: Vec<u64> = (0..16u16).map(|y| section_slot_key(x, y, z)).collect();
                let mut sorted = ks.clone();
                sorted.sort_unstable();
                assert_eq!(ks, sorted, "y-minor broken at {x},{z}");
                // Разные y дают разные ключи.
                assert_ne!(section_slot_key(x, 0, z), section_slot_key(x, 1, z));
            }
        }
    }
}
