//! P24 NOISE OCTAVE SCRATCH-POOL (TASK-459-64, law-11 WILD agent, v0 scaffold).
//!
//! Thread-лок пул флет-аккумуляторов для октавных проходов ImprovedNoise /
//! PerlinNoise / NormalNoise (idea ID-P24, RESEARCH-458-P.md): октавные
//! интерполяционные массивы переиспользуются между октавами/колонками одной
//! ген-колонки вместо свежих аллокаций. Цель — GC-debt relief на ген-сценах
//! (pregen/exploration; 0% soak-CPU, инертно на soak ×421-C).
//!
//! НЕ дублирует noisesimd (cmp457: ускоряет ВЫЧИСЛЕНИЕ октав) и не трогает
//! fill-family (src/noise_fill.rs whole-body свопы `fillArray`): этот lever
//! убирает ТОЛЬКО аллокацию вокруг октав. Формулы/порядок октав не меняются —
//! значения шума бит-в-байт.
//!
//! Бит-в-байт контракт (значение карточки): переиспользуемый буфер
//! ЗАПОЛНЯЕТСЯ ЦЕЛИКОМ до первого чтения. Пул никогда не обнуляет память —
//! протечка «хвостов прошлой колонки» в результат невозможна, пока вызывающий
//! переписывает весь приобретённый срез до чтения (обязательство фазы-2 в
//! ops-бридже; см. заглушку в src/improved_noise.rs).
//!
//! Гейт: env `CRUSSTY_NOISE_SCRATCH_POOL` (1/true/on/yes → on). OFF by default.
//! Gate OFF ⇒ register() не регистрирует НИЧЕГО (урок dormant-gate-leak
//! 3a270ee), activate() — no-op, acquire() деградирует до обычной аллокации.
//!
//! v0 = observation-only by construction (канон proto_blend_cache/B10):
//!  1. Никаких byte hooks, никаких java-классов, никакого retransform —
//!     NCDFE-канон: фаза-2 определяет ops-бридж EARLY на тихом
//!     activation-worker (паттерн improved_noise: define → patch → ОДИН
//!     retransform; ноль определений классов внутри byte-hook callback).
//!  2. Пул функционален на Rust-стороне (ScratchGuard + счётчики) — это и
//!     есть alloc-профиль, которого требует карточка («escape-analysis мог
//!     уже скаляризовать — сначала alloc-профиль»): фаза-2 армит сайты
//!     только при живом re-use счётчике на прегене.
//!  3. Кап пула на тред (MAX_POOLED_BYTES) против memory bloat / exhaustion /
//!     leaks (object-pool trade-offs, рисёрч-источник 3); переполнение =
//!     failure-open (обычная аллокация, никаких блокировок на горячем пути).
//!
//! Компилируется В ИЗОЛЯЦИИ (нет `crate::`-ссылок) — тот же принцип, что у
//! proto_blend_cache.rs: параллельные правки других агентов в lib.rs не
//! ломают сборку этого модуля.

// v0 scaffold: публичный API пула сознательно НЕ вызывается до фазы-2 (арм
// сайтов только после живого alloc-профиля) — dead_code гасим целиком,
// как introspection-хелперы proto_blend_cache.rs.
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// NEW env gate for this lever (OFF by default — do not "fix" this).
pub const GATE_ENV: &str = "CRUSSTY_NOISE_SCRATCH_POOL";

/// Верхняя граница памяти, которую ОДИН тред может удерживать в free-списке
/// пула (object-pool «memory bloat» риск; источник 3 рисёрча). Буферы сверх
/// капа возвращаются аллокатору (failure-open).
pub const MAX_POOLED_BYTES: usize = 1 << 20; // 1 MiB per thread

/// Максимальная длина одного пула-емого буфера (элементов f64): октавные
/// аккумуляторы NoiseChunk-колонки на порядок меньше; всё, что крупнее,
/// считается не-скретчем и в пул не возвращается.
pub const MAX_POOLABLE_LEN: usize = 1 << 16; // 64k f64 = 512 KiB

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

static ENABLED: AtomicBool = AtomicBool::new(false);
static GATE_READ: AtomicBool = AtomicBool::new(false);

/// Профильные счётчики (alloc-профиль v0; читаются `stats()`):
/// acquires = всего acquire_f64(), reuses = выдано из free-списка,
/// fresh = свежая аллокация, released = возвращено в пул (в т.ч. кап-дропы
/// не считаются), cap_dropped = отброшено из-за MAX_POOLED_BYTES/LEN.
static ACQUIRES: AtomicU64 = AtomicU64::new(0);
static REUSES: AtomicU64 = AtomicU64::new(0);
static FRESH: AtomicU64 = AtomicU64::new(0);
static RELEASED: AtomicU64 = AtomicU64::new(0);
static CAP_DROPPED: AtomicU64 = AtomicU64::new(0);

/// Free-список треда: длина → буфер ровно этой длины (октавные аккумуляторы
/// всегда одной длины внутри колонки — exact-size матч убирает resize).
struct PoolInner {
    free: HashMap<usize, Vec<f64>>,
    pooled_bytes: usize,
}

thread_local! {
    static POOL: RefCell<PoolInner> = RefCell::new(PoolInner {
        free: HashMap::new(),
        pooled_bytes: 0,
    });
}

/// Контракт владения буфером из пула. Deref/DerefMut дают срез `&[f64]` /
/// `&mut [f64]`; Drop возвращает буфер в thread-лок free-список (или
/// аллокатору при выключенном гейте / переполнении капа / oversize).
pub struct ScratchGuard {
    buf: Option<Vec<f64>>,
}

impl ScratchGuard {
    /// Длина приобретённого буфера (>= запрошенной ровно равна ей).
    pub fn len(&self) -> usize {
        self.buf.as_ref().map(|b| b.len()).unwrap_or(0)
    }
    /// Пуст ли буфер (len == 0 — легальный вырожденный случай).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Полный мутабельный срез. ЗАПОЛНЯТЬ ЦЕЛИКОМ до первого чтения
    /// (бит-в-байт контракт: пул не обнуляет память).
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        self.buf.as_mut().map(|b| b.as_mut_slice()).unwrap_or(&mut [])
    }
    /// Полный иммутабельный срез.
    pub fn as_slice(&self) -> &[f64] {
        self.buf.as_deref().unwrap_or(&[])
    }
}

impl std::ops::Deref for ScratchGuard {
    type Target = [f64];
    fn deref(&self) -> &[f64] {
        self.as_slice()
    }
}

impl std::ops::DerefMut for ScratchGuard {
    fn deref_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}

impl Drop for ScratchGuard {
    fn drop(&mut self) {
        if let Some(buf) = self.buf.take() {
            if !poolable_len(buf.len()) {
                return; // выключенный гейт / oversize — обычный free
            }
            let pooled = POOL.with(|p| {
                let mut p = p.borrow_mut();
                if p.pooled_bytes + buf.len() * 8 > MAX_POOLED_BYTES {
                    return false; // кап → failure-open, буфер уходит аллокатору
                }
                p.pooled_bytes += buf.len() * 8;
                p.insert_exact(buf);
                true
            });
            if pooled {
                RELEASED.fetch_add(1, Ordering::Relaxed);
            } else {
                CAP_DROPPED.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

impl PoolInner {
    fn insert_exact(&mut self, buf: Vec<f64>) {
        self.free.insert(buf.len(), buf);
    }
}

/// Класс скретча: пул работает только при поднятом гейте и для непустых
/// буферов разумной длины (октавные аккумуляторы NoiseChunk-колонки на
/// порядок меньше MAX_POOLABLE_LEN; всё крупнее — не скретч).
fn poolable_len(len: usize) -> bool {
    ENABLED.load(Ordering::Relaxed) && len > 0 && len <= MAX_POOLABLE_LEN
}

// ---------------------------------------------------------------------------
// Public API (Rust-side pool; фаза-2 вызывает из ops-бриджа/kernel-shapes)
// ---------------------------------------------------------------------------

/// Приобрести скретч-буфер `len` элементов f64. При выключенном гейте —
/// обычная свежая аллокация (zero-cost dormancy: логика вызывающего не
/// ветвится, ветвится только Drop-путь).
pub fn acquire_f64(len: usize) -> ScratchGuard {
    ACQUIRES.fetch_add(1, Ordering::Relaxed);
    if !poolable_len(len) {
        // Гейт выключен или len вне класса скретча: fresh, без пула.
        if gate_on() {
            FRESH.fetch_add(1, Ordering::Relaxed);
        }
        return ScratchGuard { buf: Some(vec![0.0; len]) };
    }
    let reused = POOL.with(|p| {
        let mut p = p.borrow_mut();
        if let Some(buf) = p.free.remove(&len) {
            p.pooled_bytes -= buf.len() * 8;
            return Some(buf);
        }
        None
    });
    match reused {
        Some(buf) => {
            REUSES.fetch_add(1, Ordering::Relaxed);
            ScratchGuard { buf: Some(buf) }
        }
        None => {
            FRESH.fetch_add(1, Ordering::Relaxed);
            ScratchGuard { buf: Some(vec![0.0; len]) }
        }
    }
}

/// Профильный срез (alloc-профиль v0 → решение о фазе-2): (acquires, reuses,
/// fresh, released, cap_dropped). Критерий карточки: на живой прегене
/// reuses/acquires должно быть стабильно > 0, иначе EA уже скаляризовала
/// сайты и lever закрывается как мёртвый.
pub fn stats() -> (u64, u64, u64, u64, u64) {
    (
        ACQUIRES.load(Ordering::Relaxed),
        REUSES.load(Ordering::Relaxed),
        FRESH.load(Ordering::Relaxed),
        RELEASED.load(Ordering::Relaxed),
        CAP_DROPPED.load(Ordering::Relaxed),
    )
}

fn gate_on() -> bool {
    if !GATE_READ.load(Ordering::Acquire) {
        let on = std::env::var(GATE_ENV)
            .map(|v| {
                let v = v.trim().to_ascii_lowercase();
                v == "1" || v == "true" || v == "on" || v == "yes"
            })
            .unwrap_or(false);
        ENABLED.store(on, Ordering::Release);
        GATE_READ.store(true, Ordering::Release);
    }
    ENABLED.load(Ordering::Relaxed)
}

/// env-gate (off by default), читается register/activate. Те же написания,
/// что у improved_noise/proto_blend_cache.
pub fn enabled() -> bool {
    gate_on()
}

// ---------------------------------------------------------------------------
// register() — v0: НИКАКИХ хуков (gate OFF ⇒ вообще ничего)
// ---------------------------------------------------------------------------

/// Регистрация lever'а (idempotent; один вызов из cplugin_init).
///
/// v0 observation-only: пул — чистый Rust-состояние, java-поверхности нет,
/// поэтому byte hook не нужен. Ничего не регистрируем даже при gate ON —
/// арм сайтов произойдёт в фазе-2 (NCDFE-канон: EARLY-define ops-бриджа на
/// тихом worker'е до первого serve, ноль определений в hook-callback).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] noise_scratch_pool: dormant (P24 v0 scaffold; set {GATE_ENV}=1 to arm the alloc-profile pool — no hooks, no patch)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] noise_scratch_pool: ARMED v0 observation-only (pool cap {MAX_POOLED_BYTES}B/thread, max len {MAX_POOLABLE_LEN} f64) — no java surface in v0, phase-2 retarget pending alloc-profile"
    );
}

// ---------------------------------------------------------------------------
// activate() — v0: no-op worker (gate OFF ⇒ nothing happens)
// ---------------------------------------------------------------------------

/// Фоновая активация (один вызов после inject_surface).
///
/// v0: читать нечего — java-классы не форсируются, бриджи не определяются
/// (NCDFE-канон), retransform нет. TODO(P24 Phase 2):
///   1. alloc-порог: stats() на прегене — reuses/acquires > 0 иначе lever
///      закрыт (EA-скаляризация риск карточки);
///   2. EARLY-define NoiseScratchOps в loader кернела (паттерн
///      improved_noise::activate: poll+force-load → define → один
///      retransform);
///   3. parity: A/B min-of-2/3 lockstep-прегена (закон 16), бит-в-байт
///      golden-сверка столбцов.
pub fn activate() {
    if !enabled() {
        // register() уже напечатал dormant; фоновой работы нет вовсе
        // (несоответствие register/activate прятало баги в improved_noise).
        return;
    }
    eprintln!(
        "[crussty-plugin] noise_scratch_pool: v0 activate no-op (observation-only; phase-2 defines ops bridge ONLY after a live pregen alloc-profile)"
    );
}
