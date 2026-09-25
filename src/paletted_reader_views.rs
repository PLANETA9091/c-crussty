//! C-X5 READER VIEWS — paletted zero-copy reader-view prototype (P-phase 1,
//! STRICT dormant). TASK-459-84, WILD закон-11 тик-459.
//!
//! Идея C-X5: массовые чтения `PalettedContainer.get(int)` (4.48% ваниль,
//! round-a32-457 cpu-collapsed) + `SimpleBitStorage.get(int)` (1.50%) = ~6.0%
//! paletted-лейн — на БАТЧ-путях (codec parse/serialize, `getAll`,
//! countPaletteSizes, materialize-циклы 4096/16384) заменяются reader-view
//! объектом: ОДНА выборка `container.data → (storage, palette, bits)` на серию
//! запросов, дальше N чтений по вьюхе без per-call volatile-рида `data`, без
//! interface-dispatch и без palette-индирекции для raw-index потребителей.
//! Одиночные чтения — ваниль (не трогаем). javap-грунт (patched-kernel.jar):
//! `PalettedContainer.get` = getfield data (volatile) + invokeinterface
//! BitStorage.get + invokevirtual readPalette НА КАЖДЫЙ вызов;
//! `SimpleBitStorage.getRaw()` = `getfield data; areturn` — живой long[] без
//! клонов (zero-copy лег); `PalettedContainer$Data` — иммутабельный кортеж
//! (configuration, storage, palette, moonrise$palette[]), resize публикуется
//! заменой volatile-ссылки data ⇒ сэмплированная пара (storage, palette)
//! консистентна внутри серии.
//!
//! ФАЗА-1 = observation-only scaffold (этот файл):
//!   1. Гейт STRICT eq `CRUSSTY_LEVER_FLAG == cmp459_cx5` — пустой/чужой
//!      флаг ⇒ register()/activate() = no-op (dormant discipline 3a270ee),
//!      байт-хук НЕ регистрируется, классов не определяется.
//!   2. Rust-референс бит-в-бит парити: packed-layout packing-цикл (транскрипт
//!      конструктора SimpleBitStorage(II[I)) ⇒ view div/mod-математика
//!      (`(data[idx/vpl] >>> (idx%vpl*bits)) & mask`) ≡ ground truth на
//!      лестнице bits 1..=15 × sizes {64, 256, 4096, 16384} (~312k точек).
//!      Эквивалентность magic-деления SimpleBitStorage (magic*idx>>>20 ≡
//!      idx/vpl) проверяется ОТДЕЛЬНО офлайн-пробой против живых кернел-
//!      классов (jvmti-недоступно в юнит-тестах); результаты — в
//!      RESEARCH-459-CX5.md §2.
//!   3. V1-ловушка (RESEARCH §6): вьюха консистентна против resize (Data
//!      иммутабелен), но НЕ против in-place мутаций storage (getAndSet пишет
//!      data[k] на месте) ⇒ волна-2 обязана доказать hold батч-путей
//!      (снапшот/секционная блокировка); fail-closed = identity re-check
//!      `container.data == sampled` до/после серии, иначе ваниль-цикл.
//!   4. NCDFE-канон: java-хелпер волны-2 определяется в kernel loader ДО
//!      первого use; ZERO определений внутри retransform (см. классовый
//!      док `src/improved_noise.rs`). Компаньон-stub:
//!      `paletted/net/minecraft/world/level/chunk/PalettedReaderViewOps.java`
//!      (JDK-only, reflection к кернелу, javac-компилируемый на CI).
//!
//! Компилируется ИЗОЛИРОВАННО (без crate::-ссылок) — как noise2d_cache.

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

/// Lever id (STRICT eq; grep-якорь вайринга волны-2).
pub const LEVER: &str = "cmp459_cx5";
/// Env-ключ STRICT-гейта.
pub const GATE_ENV: &str = "CRUSSTY_LEVER_FLAG";

static REGISTERED: AtomicBool = AtomicBool::new(false);
static ACTIVATED: AtomicBool = AtomicBool::new(false);

/// STRICT eq: вьюха активна ТОЛЬКО при точном lever id. Пустой/чужой флаг =
/// ваниль (dormant-invisible).
pub fn enabled() -> bool {
    std::env::var(GATE_ENV).map(|v| v.trim() == LEVER).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Модель SimpleBitStorage (ваниль-транскрипт)
// ---------------------------------------------------------------------------

/// Транскрипт packed-layout конструктора `SimpleBitStorage(int bits, int size,
/// int[] values)` (javap 135..): пакует блоками по `values_per_long`, value на
/// позиции i+j живёт на бит-офсете j*bits (LSB-first, без пересечения границы
/// long — инварианта protocol/Chunk-format).
pub fn pack_layout(values: &[u64], bits: u32) -> Vec<u64> {
    let vpl = 64 / bits;
    let mask = (1u64 << bits) - 1;
    let mut data = vec![0u64; values.len().div_ceil(vpl as usize)];
    let mut i = 0usize;
    let mut k = 0usize;
    while i < values.len() {
        let mut acc: u64 = 0;
        let mut j = vpl as isize - 1;
        while j >= 0 {
            let idx = i + j as usize;
            let v = if idx < values.len() { values[idx] & mask } else { 0 };
            acc = (acc << bits) | v;
            j -= 1;
        }
        data[k] = acc;
        k += 1;
        i += vpl as usize;
    }
    data
}

/// Ground truth чтения по раскладке: то, что возвращает ванильный
/// `get(int)` по определению (magic-трюк — лишь быстрый делитель).
#[inline]
pub fn layout_get(data: &[u64], bits: u32, idx: usize) -> u32 {
    let vpl = (64 / bits) as usize;
    let mask = (1u64 << bits) - 1;
    ((data[idx / vpl] >> (((idx % vpl) as u32) * bits)) & mask) as u32
}

// ---------------------------------------------------------------------------
// Reader view (C-X5) — одна выборка, N чтений
// ---------------------------------------------------------------------------

/// Сэмплированная вьюха: Java-эквивалент держит ссылки (long[] data, bits,
/// mask, vpl) + identity исходного `container.data` для re-check. Zero-copy:
/// никакой клона long[] (ванильный `getRaw()` возвращает живой массив).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReaderView<'a> {
    pub data: &'a [u64],
    pub bits: u32,
    pub mask: u64,
    pub values_per_long: u32,
}

impl<'a> ReaderView<'a> {
    /// «Одна выборка storage→(palette,биты)»: в реальной Java-вьюхе это
    /// `container.data` (volatile рид РОВНО один на серию) → storage()
    /// instanceof SimpleBitStorage → getRaw()/getBits(). Здесь — от модели.
    pub fn sample(data: &'a [u64], bits: u32) -> ReaderView<'a> {
        ReaderView {
            data,
            bits,
            mask: (1u64 << bits) - 1,
            values_per_long: 64 / bits,
        }
    }

    /// raw palette-индекс без palette-индирекции (serialize-потребитель).
    #[inline]
    pub fn raw_index(&self, idx: usize) -> u32 {
        let vpl = self.values_per_long as usize;
        ((self.data[idx / vpl] >> (((idx % vpl) as u32) * self.bits)) & self.mask) as u32
    }

    /// Батч-серия: raw_index по всем size позициям; identity re-check
    /// `container.data == sampled` выполняет волна-2 ДО и ПОСЛЕ серии
    /// (fail-closed → ваниль-цикл); здесь модель серии без re-check.
    pub fn read_series(&self, out: &mut Vec<u32>) {
        out.clear();
        out.reserve(self.data.len() * self.values_per_long as usize);
        for idx in 0..self.data.len() * self.values_per_long as usize {
            out.push(self.raw_index(idx));
        }
    }
}

// ---------------------------------------------------------------------------
// Бит-в-бит selftest (лестница bits × sizes)
// ---------------------------------------------------------------------------

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        // численно стабильный LCG (deterministic, без внешних крейтов)
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 11
    }
}

/// Полная парити-проверка: pack_layout ⇒ layout_get ≡ view.raw_index на
/// каждой точке; серийная вьюха ≡ поштучной. Возвращает число проверок.
pub fn bit_exact_selftest() -> usize {
    let mut rng = Lcg(0xC5_5EED_u64 ^ 0x9E37_79B9_7F4A_7C15);
    let mut checks = 0usize;
    let mut series: Vec<u32> = Vec::new();
    for bits in 1u32..=15 {
        for &size in &[64usize, 256, 4096, 16384] {
            let mask = (1u64 << bits) - 1;
            let values: Vec<u64> = (0..size).map(|_| rng.next() & mask).collect();
            let data = pack_layout(&values, bits);
            // ground truth по раскладке
            for (idx, &v) in values.iter().enumerate() {
                assert_eq!(layout_get(&data, bits, idx), v as u32, "layout bits={} idx={}", bits, idx);
            }
            checks += size;
            // вьюха: одна выборка, серия raw_index
            let view = ReaderView::sample(&data, bits);
            for (idx, &v) in values.iter().enumerate() {
                assert_eq!(view.raw_index(idx), v as u32, "view bits={} idx={}", bits, idx);
            }
            checks += size;
            // серийное чтение ≡ поштучному (первые 64 точки для скорости)
            view.read_series(&mut series);
            for idx in 0..64.min(size) {
                assert_eq!(series[idx], values[idx] as u32, "series bits={} idx={}", bits, idx);
            }
            checks += 64.min(size);
        }
    }
    checks
}

/// Эквивалентность magic-деления (транскрипт кернел-цепочки):
/// BETTER_MAGIC[bits] = (int) IntegerUtil.getUnsignedDivisorMagic(64/bits, 20)
/// (javap <clinit> 1174..1211), concurrentutil: getUnsignedDivisorMagic(d,20)
/// = ((1<<20)-1)/d + 1 (github.com/Spottedleaf/ConcurrentUtil IntegerUtil
/// L192-194); mulBits = (64/bits)*bits (javap ctor 180..193). Кернел каппит
/// size ≤ 4096 ("Size > 4096 not supported", javap 197..215) ⇒ проверяем
/// весь поддерживаемый диапазон: bits 1..=15 × idx<4096, wrapping-i32.
pub fn magic_division_selftest() -> usize {
    let mut pts = 0usize;
    for bits in 1u32..=15u32 {
        let vpl = 64 / bits;
        let magic = (((1u64 << 20) - 1) / vpl as u64 + 1) as u32; // concurrentutil транскрипт
        let mul_bits = (64 / bits) * bits;
        for i in 0..4096u32 {
            let j = magic.wrapping_mul(i);
            let k = j >> 20; // iushr
            let l = (j & 0xFFFFF).wrapping_mul(mul_bits) >> 20;
            assert_eq!(k as usize, i as usize / vpl as usize, "magic k bits={} idx={}", bits, i);
            assert_eq!(l, (i % vpl) * bits, "magic l bits={} idx={}", bits, i);
            pts += 1;
        }
    }
    pts
}

/// Специальный selftest деградации: bits=0 запрещено (1..32 контракт
/// Validate.inclusiveBetween(1,32)), vpl не может быть 0 — модель гарантирует
/// паникой на sample(0), проверяем ожидаемое поведение.
#[test]
fn view_rejects_bits_zero() {
    let data = [0u64; 1];
    let res = std::panic::catch_unwind(|| ReaderView::sample(&data, 0));
    assert!(res.is_err(), "bits=0 must be rejected (division by zero guard)");
}

#[test]
fn bit_exact_parity_ladder() {
    let checks = bit_exact_selftest();
    assert!(checks > 300_000, "ladder must cover ~312k points, got {}", checks);
}

#[test]
fn magic_division_parity() {
    // 61440 точек: bits 1..15 × idx<4096 (кернел-кап), wrapping-i32 семантика
    let pts = magic_division_selftest();
    assert_eq!(pts, 15 * 4096);
}

#[test]
fn gate_is_strict_eq() {
    // гейт выключен при пустом/чужом флаге — на CI CRUSSTY_LEVER_FLAG не ставится
    if std::env::var(GATE_ENV).is_err() {
        assert!(!enabled());
    }
}

// ---------------------------------------------------------------------------
// register / activate — волна-2 no-op при выключенном гейте
// ---------------------------------------------------------------------------

/// lib.rs cplugin_init wiring. Гейт OFF ⇒ no-op (ничего не регистрируется).
pub fn register() {
    if !enabled() {
        return;
    }
    // ФАЗА-1: observation-only — байт-хука нет, классов не определяем.
    REGISTERED.store(true, Ordering::SeqCst);
    eprintln!("paletted_reader_views: registered (observation-only, lever {})", LEVER);
}

/// lib.rs inject_surface wiring. Гейт OFF ⇒ no-op.
pub fn activate() {
    if !enabled() {
        return;
    }
    // ФАЗА-1: только парити-модель + счётчики; вайринг батч-путей — волна-2
    // (NCDFE-канон: PalettedReaderViewOps определяется EARLY в kernel loader).
    let checks = bit_exact_selftest();
    ACTIVATED.store(true, Ordering::SeqCst);
    eprintln!("paletted_reader_views: ARMED lever {} parity checks {}", LEVER, checks);
}

#[cfg(test)]
mod probe_states {
    use super::*;
    #[test]
    fn flags_default_off() {
        // STRICT dormant: без гейта register/activate обязаны быть no-op
        if !enabled() {
            register();
            activate();
            assert!(!REGISTERED.load(Ordering::SeqCst));
            assert!(!ACTIVATED.load(Ordering::SeqCst));
        }
    }
}
