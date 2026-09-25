//! CHUNK-SERIAL LRU DECOMPRESS-ARENA (TASK-459-82, идея C-X3 — wild-нога
//! закона 11, тик-459; СВОЯ идея, не карточка).
//!
//! Идея: при СЕРИЙНОМ чтении чанков (topup-волны worldbench, соседние чанки
//! при population/promotion, повторные открытия того же региона-сектора)
//! ванильный путь на каждое чтение делает re-decompress payload'а, даже если
//! тот же (chunk, section) уже разжат в этой нити МИКРОСЕКУНДЫ назад — декомпресс
//! детерминирован, а serial-паттерн чтений (сосед по XZ из того же region-файла,
//! повторный topup после неудачной promote) многократно дёргает одну и ту же
//! пару (loc-entry → inflate). C-X3 держит per-thread LRU-арену последних
//! N РАЗЖАТЫХ секций: HIT → байты из арены (CRC32-контроль), MISS → ванильный
//! decompress как сегодня.
//!
//! Контракт паритета (закон 4, бит-в-байт):
//!  - HIT отдаёт ТОЧНО те байты, что вернул бы decompress: слот хранит
//!    payload + CRC32IEEE байт; на get CRC пересчитывается по выдаче —
//!    любое расхождение → CRC_FAIL → слот отравлен → MISS (vanilla path);
//!  - epoch (версия данных: регион-файл mtime/тучность — java-сторона) не
//!    совпал → MISS (устаревшая арена не обслуживает);
//!  - любая структурная ошибка (range, layout, переполнение) → BROKEN
//!    one-shot latch → fail-closed ваниль навсегда (mobs_soa ERR-лестница);
//!  - пустой/чужой lever: natives НЕ регистрируются, java-guard
//!    arenaArmed ставится ТОЛЬКО rust-регистрацией → ваниль бит-в-байт.
//!
//! NCDFE-канон: java-класс ChunkSerialArenaOps дефайнится ДО RegisterNatives;
//! java добирается до натива только через one-shot armed-гвард
//! (ROOTCAUSE-NCDFE: fail-inert на pre-resolved indy недопустим).
//!
//! СКАФФОЛД (12e): арена + CRC32 + LRU + JNI-surface + self_test + тесты —
//! здесь; wiring в реальный read-путь (ChunkSerializer.read → IOWorker,
//! site-жавап, javac-пересборка блоба) — следующая нога, см.
//! RESEARCH-459-CX3.md. STRICT dormant: lever cmp459_cx3 STRICT eq.

#![allow(dead_code)]

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const ARENA_CLASS: &str =
    "net/minecraft/world/level/chunk/storage/ChunkSerialArenaOps";

/// Gate answers: 1 = HIT (out заполнен), 0 = MISS (ваниль), <0 = ERR.
pub const ARENA_HIT: i32 = 1;
pub const ARENA_MISS: i32 = 0;
pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// Слотов per-thread (последние N секций; 12 × ≤16KiB тип-секция ≈ 192KiB).
pub const ARENA_SLOTS: usize = 12;
/// Бюджет per-thread (байт): секции крупнее SKIP — не кэшируются (cap_dropped).
pub const ARENA_BYTES_CAP: usize = 512 * 1024;
const SLOT_SKIP: usize = ARENA_BYTES_CAP / ARENA_SLOTS;

/// BROKEN latch: любая структурная ошибка → все ответы MISS/ERR → java
/// ваниль навсегда (one-shot, fail-closed).
static BROKEN: AtomicBool = AtomicBool::new(false);

/// DATA-PLAN счётчики (scaffold-фаза: observability для первой ARM-ноги).
static GETS: AtomicU64 = AtomicU64::new(0);
static PUTS: AtomicU64 = AtomicU64::new(0);
static HITS: AtomicU64 = AtomicU64::new(0);
static MISSES: AtomicU64 = AtomicU64::new(0);
static CRC_FAIL: AtomicU64 = AtomicU64::new(0);
static EVICTS: AtomicU64 = AtomicU64::new(0);
static CAP_DROPPED: AtomicU64 = AtomicU64::new(0);

pub fn stats() -> (u64, u64, u64, u64, u64, u64, u64) {
    (
        GETS.load(Ordering::Relaxed),
        PUTS.load(Ordering::Relaxed),
        HITS.load(Ordering::Relaxed),
        MISSES.load(Ordering::Relaxed),
        CRC_FAIL.load(Ordering::Relaxed),
        EVICTS.load(Ordering::Relaxed),
        CAP_DROPPED.load(Ordering::Relaxed),
    )
}

fn broken() -> bool {
    BROKEN.load(Ordering::Acquire)
}

fn break_arena(why: &str) {
    if !BROKEN.swap(true, Ordering::AcqRel) {
        eprintln!(
            "[crussty-plugin] cx3-arena: BROKEN latch set ({why}) — fail-closed vanilla forever"
        );
    }
}

// ---------------------------------------------------------------- CRC32IEEE

/// Таблица CRC32 (IEEE, poly 0xEDB88320), software-реализация без зависимостей
/// (бинтится в cdylib; x86 crc32q для байтовых потоков не годится — полином
/// другой). Табличный байт-за-раз; для 8-16KiB секций хватает с запасом.
const CRC_TABLE: [u32; 256] = build_crc_table();

const fn build_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}

pub fn crc32_ieee(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c = CRC_TABLE[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

// ------------------------------------------------------------------- LRU

/// Ключ слота: точная тройка (cx, cz, sy). 32+32+8=72 бита не влезают в u64
/// без потерь, поэтому НЕ пакуем — слот хранит тройку целиком, сравнение
/// точное (12 слотов × линейный скан — копейки). sy ∈ [0, 255] — секции выше
/// не существуют в ванильных мирах; вне диапазона → MISS (не кэшируем).
#[inline]
fn valid_section(sy: i32) -> bool {
    (0..=255).contains(&sy)
}

struct ArenaSlot {
    cx: i32,
    cz: i32,
    sy: i32,
    valid: bool,
    epoch: u64,
    crc: u32,
    stamp: u64, // LRU-часы
    buf: Vec<u8>,
}

impl ArenaSlot {
    fn new() -> Self {
        ArenaSlot { cx: 0, cz: 0, sy: 0, valid: false, epoch: 0, crc: 0, stamp: 0, buf: Vec::new() }
    }
}

struct SerialArena {
    slots: Vec<ArenaSlot>,
    clock: u64,
    live_bytes: usize,
}

thread_local! {
    static ARENA: RefCell<SerialArena> = RefCell::new(SerialArena {
        slots: (0..ARENA_SLOTS).map(|_| ArenaSlot::new()).collect(),
        clock: 0,
        live_bytes: 0,
    });
}

#[inline]
fn key_matches(slot: &ArenaSlot, cx: i32, cz: i32, sy: i32, epoch: u64) -> bool {
    slot.valid && slot.cx == cx && slot.cz == cz && slot.sy == sy && slot.epoch == epoch
}

/// GET: HIT → копирует payload в out, возвращает len (i32); MISS → 0;
/// структурный сбой → ERR_* + BROKEN latch. CRC контролирует байты:
/// пересчёт по факту выдачи, mismatch → отравить слот → MISS.
fn arena_get_inner(cx: i32, cz: i32, sy: i32, epoch: u64, out: &mut [u8]) -> i32 {
    if broken() {
        return ARENA_MISS;
    }
    if !valid_section(sy) {
        MISSES.fetch_add(1, Ordering::Relaxed);
        return ARENA_MISS;
    }
    ARENA.with(|a| {
        let mut arena = a.borrow_mut();
        arena.clock = arena.clock.wrapping_add(1);
        let clock = arena.clock;
        let Some(idx) = arena.slots.iter().position(|s| key_matches(s, cx, cz, sy, epoch)) else {
            MISSES.fetch_add(1, Ordering::Relaxed);
            return ARENA_MISS;
        };
        let slot = &mut arena.slots[idx];
        let crc_actual = crc32_ieee(&slot.buf);
        if crc_actual != slot.crc {
            // Байты арены искажены — отравить и обслужить ванилью.
            CRC_FAIL.fetch_add(1, Ordering::Relaxed);
            slot.valid = false;
            MISSES.fetch_add(1, Ordering::Relaxed);
            return ARENA_MISS;
        }
        if out.len() < slot.buf.len() {
            // out-буфер java-стороны меньше payload — структурный
            // конфликт контракта, fail-closed.
            break_arena("out buffer smaller than payload");
            return ERR_RANGE;
        }
        out[..slot.buf.len()].copy_from_slice(&slot.buf);
        let len = slot.buf.len() as i32;
        slot.stamp = clock; // touch: LRU refresh
        HITS.fetch_add(1, Ordering::Relaxed);
        len
    })
}

/// PUT: кладёт разжатые байты (после ВАНИЛЬНОГО decompress), evict LRU при
/// переполнении. Возвращает 1 = stored, 0 = skip (dormant/cap/empty), <0 err.
fn arena_put_inner(cx: i32, cz: i32, sy: i32, epoch: u64, data: &[u8]) -> i32 {
    if broken() || data.is_empty() {
        return ARENA_MISS;
    }
    if !valid_section(sy) {
        return ARENA_MISS;
    }
    if data.len() > SLOT_SKIP {
        CAP_DROPPED.fetch_add(1, Ordering::Relaxed);
        return ARENA_MISS;
    }
    ARENA.with(|a| {
        let mut arena = a.borrow_mut();
        arena.clock = arena.clock.wrapping_add(1);
        let clock = arena.clock;
        let crc = crc32_ieee(data);

        // Обновить существующий слот, если ключ уже в арене.
        if let Some(idx) = arena.slots.iter().position(|s| s.valid && s.cx == cx && s.cz == cz && s.sy == sy) {
            let old_len = arena.slots[idx].buf.len();
            let slot = &mut arena.slots[idx];
            slot.buf.clear();
            slot.buf.extend_from_slice(data);
            slot.crc = crc;
            slot.epoch = epoch;
            slot.stamp = clock;
            slot.valid = true;
            drop(slot);
            arena.live_bytes += data.len().saturating_sub(old_len);
            PUTS.fetch_add(1, Ordering::Relaxed);
            return ARENA_HIT;
        }

        // Свободный слот или evict LRU (минимальный stamp среди валидных).
        let idx = match arena.slots.iter().position(|s| !s.valid) {
            Some(i) => i,
            None => {
                let mut best = 0usize;
                let mut best_stamp = u64::MAX;
                for (i, s) in arena.slots.iter().enumerate() {
                    if s.stamp < best_stamp {
                        best_stamp = s.stamp;
                        best = i;
                    }
                }
                EVICTS.fetch_add(1, Ordering::Relaxed);
                arena.live_bytes = arena
                    .live_bytes
                    .saturating_sub(arena.slots[best].buf.len());
                best
            }
        };
        let slot = &mut arena.slots[idx];
        slot.buf.clear();
        slot.buf.extend_from_slice(data);
        slot.cx = cx;
        slot.cz = cz;
        slot.sy = sy;
        slot.epoch = epoch;
        slot.crc = crc;
        slot.stamp = clock;
        slot.valid = true;
        arena.live_bytes += data.len();
        PUTS.fetch_add(1, Ordering::Relaxed);
        ARENA_HIT
    })
}

fn arena_reset_inner() -> usize {
    ARENA.with(|a| {
        let mut arena = a.borrow_mut();
        let live = arena.live_bytes;
        for s in arena.slots.iter_mut() {
            s.valid = false;
            s.buf.clear();
        }
        arena.live_bytes = 0;
        live
    })
}

// ------------------------------------------------------------------ lever

/// STRICT eq: natives регистрируются ТОЛЬКО при cmp459_cx3 (STRICT dormant).
fn lever_enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp459_cx3")
        .unwrap_or(false)
}

/// RegisterNatives(chunkSerialArenaGet/Put) на только что дефайнутом
/// ChunkSerialArenaOps, затем one-shot java-guard (define-ДО-arm — NCDFE
/// канон). Wiring в activation-worker (entity_index_manager паттерн) —
/// следующая нога.
pub fn register_arena(env: &JniEnv, cls: jni::jclass) -> bool {
    if !lever_enabled() {
        return false;
    }
    let Ok(name_g) = CString::new("chunkSerialArenaGet") else {
        return false;
    };
    let Ok(name_p) = CString::new("chunkSerialArenaPut") else {
        return false;
    };
    let Ok(sig_g) = CString::new("(IIIIJ[B)I") else {
        return false;
    };
    let Ok(sig_p) = CString::new("(IIIIJ[B)I") else {
        return false;
    };
    let natives = [
        jni::JNINativeMethod {
            name: name_g.as_ptr(),
            signature: sig_g.as_ptr(),
            fnPtr: chunk_serial_arena_get as *const c_void as *mut c_void,
        },
        jni::JNINativeMethod {
            name: name_p.as_ptr(),
            signature: sig_p.as_ptr(),
            fnPtr: chunk_serial_arena_put as *const c_void as *mut c_void,
        },
    ];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] cx3-arena: register_natives failed — gate stays asleep"
        );
        return false;
    }
    let Some(mid) = env.get_static_method_id(cls, "arenaArmed", "()V") else {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] cx3-arena: arenaArmed()V missing — gate stays asleep"
        );
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    if crate::clear_exception(env) {
        eprintln!(
            "[crussty-plugin] cx3-arena: arenaArmed()V threw — gate stays asleep"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] cx3-arena: cmp459_cx3 ARMED (serial LRU decompress arena → {ARENA_CLASS})"
    );
    true
}

/// JNI: chunkSerialArenaGet(cx, cz, sy, epoch) -> len>0 HIT / 0 MISS /
/// <0 ERR. Java-сторона вызывает ПОСЛЕ того, как ванильный путь открыл
/// loc-entry и ПЕРЕД inflate. Scaffold-фаза: консервативный MISS
/// (parity by default); out-буфер java-стороны + serve-путь — следующая
/// нога (см. RESEARCH-459-CX3.md, wiring-план).
pub unsafe extern "system" fn chunk_serial_arena_get(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    cx: jni::jint,
    cz: jni::jint,
    sy: jni::jint,
    epoch: jni::jlong,
) -> jni::jint {
    if env.is_null() {
        return ERR_STRUCT;
    }
    let _env = unsafe { JniEnv::from_raw(env) };
    GETS.fetch_add(1, Ordering::Relaxed);
    if broken() {
        return ARENA_MISS;
    }
    // Scaffold-фаза: реальный out-буфер приходит java-стороной следующей
    // ногой (re-usable byte[]); здесь консервативный MISS (parity by default).
    let _ = (env, cx, cz, sy, epoch);
    MISSES.fetch_add(1, Ordering::Relaxed);
    ARENA_MISS
}

/// JNI: chunkSerialArenaPut(cx, cz, sy, epoch, data) — java кладёт ТОЛЬКО ЧТО
/// разжатые ванилью байты (post-inflate hook). Ошибки молча проглатываются:
/// арена никогда не ломает ванильный read.
pub unsafe extern "system" fn chunk_serial_arena_put(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    cx: jni::jint,
    cz: jni::jint,
    sy: jni::jint,
    epoch: jni::jlong,
    data: jni::jbyteArray,
) -> jni::jint {
    if env.is_null() {
        return ERR_STRUCT;
    }
    let env = unsafe { JniEnv::from_raw(env) };
    if data.is_null() {
        return ARENA_MISS;
    }
    let len = env.get_array_length(data as jni::jarray);
    if len <= 0 {
        return ARENA_MISS;
    }
    let mut buf = vec![0i8; len as usize];
    env.get_byte_array_region(data, 0, len, &mut buf);
    let bytes: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    arena_put_inner(cx, cz, sy, epoch as u64, &bytes)
}

// --------------------------------------------------------------- self_test

/// Самотест (ARM-фаза, entity_index-паттерн): roundtrip bit-exact + LRU
/// порядок + cap-dropped + fail-closed. Возвращает true только если ВСЕ
/// инварианты сошлись; иначе BROKEN → vanilla forever.
pub fn self_test() -> bool {
    if !lever_enabled() {
        return true; // dormant: самотест тривиально пройден, gate спит
    }
    let cx = 7i32;
    let cz = -13i32;
    let epoch = 42u64;
    let payload: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();

    // INV1: MISS на пустой арене.
    let mut out = vec![0u8; payload.len()];
    if arena_get_inner(cx, cz, 3, epoch, &mut out) != ARENA_MISS {
        break_arena("selftest INV1");
        return false;
    }
    // INV2: put → get bit-exact.
    if arena_put_inner(cx, cz, 3, epoch, &payload) != ARENA_HIT {
        break_arena("selftest INV2");
        return false;
    }
    if arena_get_inner(cx, cz, 3, epoch, &mut out) != payload.len() as i32
        || out[..payload.len()] != payload[..]
    {
        break_arena("selftest INV2-exact");
        return false;
    }
    // INV3: epoch bump → MISS (устаревшее не обслуживаем).
    if arena_get_inner(cx, cz, 3, epoch + 1, &mut out) != ARENA_MISS {
        break_arena("selftest INV3");
        return false;
    }
    // INV4: волна из ARENA_SLOTS+1 новых put'ов — LRU обязан вытеснить ≥1.
    for s in 1..=(ARENA_SLOTS as i32 + 1) {
        let p = vec![s as u8; 64];
        let _ = arena_put_inner(cx + s, cz, 0, epoch, &p);
    }
    // INV5: cap-drop oversized payload.
    let big = vec![0u8; SLOT_SKIP + 1];
    if arena_put_inner(cx, cz, 200, epoch, &big) != ARENA_MISS {
        break_arena("selftest INV5");
        return false;
    }
    // INV6: после серийной волны оригинальный ключ (cx,cz,3) вне волны INV4
    // (cx+s ≥ cx+1) → должен остаться bit-в-байт.
    if arena_get_inner(cx, cz, 3, epoch, &mut out) != payload.len() as i32
        || out[..payload.len()] != payload[..]
    {
        break_arena("selftest INV6-exact");
        return false;
    }
    let _ = arena_reset_inner();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_ieee_known_vectors() {
        assert_eq!(crc32_ieee(b""), 0x0000_0000);
        assert_eq!(crc32_ieee(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32_ieee(b"The quick brown fox jumps over the lazy dog"), 0x414F_A339);
    }

    #[test]
    fn section_range_guard() {
        assert!(valid_section(0));
        assert!(valid_section(255));
        assert!(!valid_section(256));
        assert!(!valid_section(-1));
        // Вне диапазона get/put = MISS (не кэшируем).
        let mut out = vec![0u8; 16];
        assert_eq!(arena_get_inner(0, 0, 256, 1, &mut out), ARENA_MISS);
        assert_eq!(arena_put_inner(0, 0, -1, 1, &[1u8]), ARENA_MISS);
    }

    #[test]
    fn arena_roundtrip_bit_exact() {
        let p: Vec<u8> = (0..1024u32).map(|i| (i % 253) as u8).collect();
        let mut out = vec![0u8; p.len()];
        assert_eq!(arena_get_inner(1, 2, 5, 9, &mut out), ARENA_MISS);
        assert_eq!(arena_put_inner(1, 2, 5, 9, &p), ARENA_HIT);
        assert_eq!(arena_get_inner(1, 2, 5, 9, &mut out), p.len() as i32);
        assert_eq!(out[..p.len()], p[..]);
    }

    #[test]
    fn arena_epoch_bump_is_miss() {
        let p = vec![7u8; 128];
        let mut out = vec![0u8; 128];
        assert_eq!(arena_put_inner(3, 4, 1, 100, &p), ARENA_HIT);
        assert_eq!(arena_get_inner(3, 4, 1, 100, &mut out), 128);
        assert_eq!(arena_get_inner(3, 4, 1, 101, &mut out), ARENA_MISS);
    }

    #[test]
    fn arena_lru_eviction() {
        let mut out = vec![0u8; 64];
        for s in 0..(ARENA_SLOTS as i32) {
            let p = vec![s as u8; 64];
            assert_eq!(arena_put_inner(100 + s, 0, 0, 1, &p), ARENA_HIT);
        }
        // Все 12 на месте (каждый get обновляет stamp → порядок LRU
        // детерминирован: слот 0 — самый старый).
        for s in 0..(ARENA_SLOTS as i32) {
            assert_eq!(arena_get_inner(100 + s, 0, 0, 1, &mut out), 64);
        }
        // 13-й put вытесняет LRU (слот 0 — его stamp старейший после
        // порядка touch'ей выше).
        let extra = vec![99u8; 64];
        assert_eq!(arena_put_inner(500, 500, 0, 1, &extra), ARENA_HIT);
        assert_eq!(arena_get_inner(500, 500, 0, 1, &mut out), 64);
        assert_eq!(arena_get_inner(100, 0, 0, 1, &mut out), ARENA_MISS);
        for s in 1..(ARENA_SLOTS as i32) {
            assert_eq!(arena_get_inner(100 + s, 0, 0, 1, &mut out), 64);
        }
    }

    #[test]
    fn arena_cap_drop_oversized() {
        let big = vec![0u8; SLOT_SKIP + 1];
        assert_eq!(arena_put_inner(9, 9, 9, 1, &big), ARENA_MISS);
    }

    #[test]
    fn arena_self_test_passes_dormant() {
        // dormant-путь: lever пуст → true без arm.
        assert!(self_test());
    }
}
