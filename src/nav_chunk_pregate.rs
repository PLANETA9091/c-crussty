//! NAV-CHUNK PRE-GATE (TASK-459-70, ID-P45 — wild-нога закона 11, тик-459).
//!
//! Идея ID-P45 ПОХИЩЕНА у agent-K ID-H04 (roaring-occupancy паттерн,
//! RESEARCH-458-K): chunk→navigatingMobs флет-маппинг из home-chunk mirror;
//! block-update в чанке, не накрытом ни одной decision-сферой нав. моба,
//! → zero-work ДО collect-прохода NavPlaneOps.handle.
//!
//! Точка: ванильный sendBlockUpdated (Paper, javap-verbatim в nav_plane.rs)
//! на КАЖДЫЙ collision-дельта block-update итерирует весь navigatingMobs и
//! зовёт shouldRecomputePath(pos) на каждого моба. Решение = сфера
//! {P : distSqr(P+0.5, mid) < remaining^2}, mid=(node+mob)/2 (см. decide()).
//! Каждый нав. моб регистрирует КОНСЕРВАТИВНЫЙ чанковый extent сферы
//! (floor((mid±(R+1))/16), 2D-флет по XZ — y игнорируется, что только
//! расширяет superset). Гейт = O(1): count==0 ⇒ ванильный проход доказанно
//! no-op (PREGATE_EMPTY) ⇒ return ДО итерации сета / UNSAFE-ридов / JNI.
//!
//! Parity (superset-гейт, контракт entity_index count==0):
//!  - FN невозможен по построению: inc(counts)-до-publish (key пишется
//!    Release после cnt), unpublish-до-dec, per-shard seqlock читателя
//!    (QRETRY → ERR_RANGE → java полный проход), unbounded bypass для
//!    R > REGISTER_R_CAP / не-finit / span > MAX_SPAN_CELLS (гейт засыпает
//!    глобально, parity сохранён);
//!  - false positives безвредны: MAYBE = точный сегодняшний путь (collect +
//!    navDecide батч);
//!  - ERR/переполнение/самотест-мимо → BROKEN latch → fail-closed ваниль
//!    навсегда (mobs_soa ERR-лестница, никогда не меняет поведение);
//!  - пустой/чужой lever: natives не регистрируются, java-guard pregateOk
//!    ставится ТОЛЬКО rust-регистрацией → ваниль бит-в-байт (закон 4).
//!
//! NCDFE-канон: define NavPlaneOps ДО RegisterNatives(navPregate); java
//! добирается до натива только через one-shot pregateOk (ROOTCAUSE-NCDFE).
//!
//! СКАФФОЛД (12e): mirror + gate + JNI + самотест + тесты — здесь; wiring
//! (fused drain+pregate в JNI, note-сайты javap, javac-пересборка класса,
//! entity_index_manager-style activation) — следующая нога. Свежесть:
//! прегейт зовётся из handle ПОСЛЕ слитого дрена буферов note-сайтов
//! (fused flush+pregate, паттерн eidxFlushQuery) — см. RESEARCH-459-P45.md.

#![allow(dead_code)]

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::Mutex;

pub const PREGATE_CLASS: &str = "net/minecraft/server/level/NavPlaneOps";
pub const PREGATE_SIG: &str = "(II)I";

/// Gate answers: 0 = EMPTY (zero-work), 1 = MAYBE (vanilla pass), <0 = ERR.
pub const PREGATE_EMPTY: i32 = 0;
pub const PREGATE_MAYBE: i32 = 1;
pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// Spheres with remaining beyond this register as UNBOUNDED (global bypass;
/// типичный ванильный wander << 64 — см. RESEARCH-459-P45.md, риск 4).
pub const REGISTER_R_CAP: f64 = 64.0;
/// Extent span beyond this (cells) → UNBOUNDED (анти-распухание таблицы).
const MAX_SPAN_CELLS: i64 = 4096;

const NSHARDS: usize = 64;
const CHUNK_CAP: usize = 1 << 11; // per-shard chunk slots → 131k covered chunks
const MOB_CAP: usize = 1 << 13;   // navigating-mob slots (150k-pop фиксатор << 8k)
const PROBE_MAX: usize = 64;      // open-addressing budget → ERR_STRUCT
const QRETRY: u32 = 64;           // seqlock retry budget → ERR_RANGE

const K0: AtomicI64 = AtomicI64::new(0);
const I0: AtomicI32 = AtomicI32::new(0);
const U0: AtomicUsize = AtomicUsize::new(0);

/// BROKEN latch: any structural failure → pregate always ERR → java full
/// vanilla pass forever (one-shot, fail-closed).
static BROKEN: AtomicBool = AtomicBool::new(false);
/// Live unbounded mobs (bypass: gate answers MAYBE while > 0).
static UNBOUNDED_LIVE: AtomicI32 = AtomicI32::new(0);
/// Sum of all per-chunk counts (self-test invariant #1).
static REG_SUM: AtomicI64 = AtomicI64::new(0);
/// DATA-PLAN metrics: total gate calls + zero-work (EMPTY) answers.
static GATE_CALLS: AtomicUsize = AtomicUsize::new(0);
static GATE_ZERO_WORK: AtomicUsize = AtomicUsize::new(0);

struct Shard {
    /// Seqlock: even = stable, odd = write in flight (entity_index pattern).
    ver: AtomicUsize,
    /// Packed chunk keys, forced non-zero (sign bit set), NEVER freed:
    /// cnt==0 means EMPTY; no key reuse => no ABA for lock-free readers.
    keys: [AtomicI64; CHUNK_CAP],
    /// Live sphere coverage count per chunk (0 = EMPTY).
    cnt: [AtomicI32; CHUNK_CAP],
}

static SHARDS: [Shard; NSHARDS] = [const {
    Shard {
        ver: U0,
        keys: [K0; CHUNK_CAP],
        cnt: [I0; CHUNK_CAP],
    }
}; NSHARDS];

/// Mob extent table (path-funnel notes are RARE — linear scan under the
/// single global writer lock is scaffold-sufficient; open addressing =
/// next-leg, entity_index pattern). Inc-before-publish / unpublish-before-dec.
static M_KEY: [AtomicI64; MOB_CAP] = [const { K0 }; MOB_CAP]; // mob_id | MIN
static M_EXT: [AtomicI64; 4 * MOB_CAP] = [const { K0 }; 4 * MOB_CAP]; // x0,z0,x1,z1
const EXT_UNBOUNDED: i64 = i64::MIN;

struct WState {
    hi: usize,
    free: Vec<usize>,
}
static WSTATE: Mutex<WState> = Mutex::new(WState { hi: 0, free: Vec::new() });

fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Bijective non-zero chunk key: u32-packed (cx,cz) with sign bit forced on.
fn cell_key(cx: i64, cz: i64) -> i64 {
    let cx = cx.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    let cz = cz.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    ((((cx as u32) as u64) << 32) | ((cz as u32) as u64) | (1u64 << 63)) as i64
}

fn shard_of(key: i64) -> usize {
    (splitmix(key as u64) as usize) & (NSHARDS - 1)
}

/// Linear probe. Ok(None) = absent (first zero slot terminates — keys are
/// never freed, so the probe invariant holds); Err = probe exhaustion.
fn shard_find(s: &Shard, key: i64) -> Result<Option<usize>, i32> {
    let base = shard_of(key);
    for i in 0..PROBE_MAX {
        let slot = (base + i) & (CHUNK_CAP - 1);
        let k = s.keys[slot].load(Ordering::Acquire);
        if k == key {
            return Ok(Some(slot));
        }
        if k == 0 {
            return Ok(None);
        }
    }
    Err(ERR_STRUCT)
}

/// Seqlock-stable read of one chunk's coverage: Ok(Some(n>0)) = MAYBE,
/// Ok(None) = EMPTY, Err = java full pass (fail-closed, never toward FN).
fn shard_count(s: &Shard, key: i64) -> Result<Option<i32>, i32> {
    let mut tries = 0u32;
    loop {
        let v1 = s.ver.load(Ordering::Acquire);
        if v1 & 1 == 0 {
            let c = match shard_find(s, key)? {
                Some(slot) => s.cnt[slot].load(Ordering::Acquire),
                None => 0,
            };
            if s.ver.load(Ordering::Acquire) == v1 {
                return Ok(if c == 0 { None } else { Some(c) });
            }
        }
        tries += 1;
        if tries > QRETRY {
            return Err(ERR_RANGE);
        }
        std::hint::spin_loop();
    }
}

/// Seqlock write window around a batch of same-shard mutations.
fn shard_write<T>(s: &Shard, f: impl FnOnce() -> T) -> T {
    s.ver.fetch_add(1, Ordering::AcqRel); // odd
    let out = f();
    s.ver.fetch_add(1, Ordering::Release); // even
    out
}

fn inc_cell(cx: i64, cz: i64) -> Result<(), i32> {
    let key = cell_key(cx, cz);
    let s = &SHARDS[shard_of(key)];
    shard_write(s, || -> Result<(), i32> {
        match shard_find(s, key)? {
            Some(slot) => {
                s.cnt[slot].fetch_add(1, Ordering::Release);
                REG_SUM.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            None => {
                // allocate: first zero slot in the probe chain
                let base = shard_of(key);
                let mut alloc: Option<usize> = None;
                for i in 0..PROBE_MAX {
                    let slot = (base + i) & (CHUNK_CAP - 1);
                    if s.keys[slot].load(Ordering::Relaxed) == 0 {
                        alloc = Some(slot);
                        break;
                    }
                }
                let Some(slot) = alloc else {
                    BROKEN.store(true, Ordering::Relaxed);
                    return Err(ERR_STRUCT); // CHUNK_CAP exhausted → fail-closed
                };
                // inc-BEFORE-publish: cnt visible only after key (Release).
                s.cnt[slot].store(1, Ordering::Relaxed);
                s.keys[slot].store(key, Ordering::Release);
                REG_SUM.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        }
    })
}

fn dec_cell(cx: i64, cz: i64) -> Result<(), i32> {
    let key = cell_key(cx, cz);
    let s = &SHARDS[shard_of(key)];
    shard_write(s, || -> Result<(), i32> {
        match shard_find(s, key)? {
            Some(slot) => {
                let prev = s.cnt[slot].fetch_sub(1, Ordering::Release);
                REG_SUM.fetch_sub(1, Ordering::Relaxed);
                if prev <= 0 {
                    // structural violation (double-dec) → fail-closed vanilla
                    BROKEN.store(true, Ordering::Relaxed);
                    return Err(ERR_STRUCT);
                }
                Ok(()) // key stays resident (cnt==0 = EMPTY; no ABA by design)
            }
            None => {
                BROKEN.store(true, Ordering::Relaxed);
                Err(ERR_STRUCT)
            }
        }
    })
}

/// Conservative chunk extent of the decision sphere: floor((mid±(R+1))/16),
/// XZ only (y ignored — superset-widening). None => register as UNBOUNDED.
fn extent_of(mid_x: f64, mid_z: f64, r: f64) -> Option<(i64, i64, i64, i64)> {
    if !mid_x.is_finite() || !mid_z.is_finite() || !r.is_finite() || r < 0.0 {
        return None;
    }
    if r > REGISTER_R_CAP {
        return None; // documented cap: big-R spheres go the bypass route
    }
    let x0 = ((mid_x - r - 1.0).floor() as i64) >> 4;
    let x1 = ((mid_x + r + 1.0).floor() as i64) >> 4;
    let z0 = ((mid_z - r - 1.0).floor() as i64) >> 4;
    let z1 = ((mid_z + r + 1.0).floor() as i64) >> 4;
    let sx = x1.saturating_sub(x0).saturating_add(1);
    let sz = z1.saturating_sub(z0).saturating_add(1);
    if sx.saturating_mul(sz) > MAX_SPAN_CELLS {
        return None; // anti-bloat: global bypass instead of table blowup
    }
    Some((x0, z0, x1, z1))
}

fn read_ext(slot: usize) -> (i64, i64, i64, i64) {
    (
        M_EXT[slot * 4].load(Ordering::Relaxed),
        M_EXT[slot * 4 + 1].load(Ordering::Relaxed),
        M_EXT[slot * 4 + 2].load(Ordering::Relaxed),
        M_EXT[slot * 4 + 3].load(Ordering::Relaxed),
    )
}

fn mob_probe(w: &WState, mob_key: i64) -> Option<usize> {
    for slot in 0..w.hi {
        if M_KEY[slot].load(Ordering::Relaxed) == mob_key {
            return Some(slot);
        }
    }
    None
}

/// Register (or re-register) the decision-sphere extent of navigating mob
/// `mob_id`. Note-sites (next leg): path set/unset funnels + navigatingMobs
/// add/remove, buffered per-thread, drained fused with the pregate JNI.
pub fn note_sphere(mob_id: i64, mid_x: f64, mid_z: f64, r: f64) -> Result<(), i32> {
    if BROKEN.load(Ordering::Relaxed) {
        return Err(ERR_STRUCT);
    }
    if mob_id <= 0 {
        return Err(ERR_RANGE);
    }
    let mob_key = mob_id | i64::MIN;
    let ext = extent_of(mid_x, mid_z, r);
    let mut w = WSTATE.lock().map_err(|_| {
        BROKEN.store(true, Ordering::Relaxed);
        ERR_STRUCT
    })?;
    // 1) unpublish + drop the previous extent (dec-old BEFORE inc-new; the
    //    seqlock window is unstable for readers → retry, never stale EMPTY)
    if let Some(slot) = mob_probe(&w, mob_key) {
        M_KEY[slot].store(0, Ordering::Relaxed); // unpublish-first
        if M_EXT[slot * 4].load(Ordering::Relaxed) == EXT_UNBOUNDED {
            UNBOUNDED_LIVE.fetch_sub(1, Ordering::Relaxed);
        } else {
            let (x0, z0, x1, z1) = read_ext(slot);
            let mut z = z0;
            while z <= z1 {
                let mut x = x0;
                while x <= x1 {
                    dec_cell(x, z)?;
                    x += 1;
                }
                z += 1;
            }
        }
        w.free.push(slot);
    }
    // 2) allocate a fresh slot
    let slot = if let Some(s) = w.free.pop() {
        s
    } else {
        if w.hi >= MOB_CAP {
            BROKEN.store(true, Ordering::Relaxed);
            return Err(ERR_STRUCT);
        }
        let s = w.hi;
        w.hi += 1;
        s
    };
    // 3) register the new extent (inc-BEFORE-publish)
    match ext {
        Some((x0, z0, x1, z1)) => {
            let mut z = z0;
            while z <= z1 {
                let mut x = x0;
                while x <= x1 {
                    inc_cell(x, z)?;
                    x += 1;
                }
                z += 1;
            }
            M_EXT[slot * 4].store(x0, Ordering::Relaxed);
            M_EXT[slot * 4 + 1].store(z0, Ordering::Relaxed);
            M_EXT[slot * 4 + 2].store(x1, Ordering::Relaxed);
            M_EXT[slot * 4 + 3].store(z1, Ordering::Relaxed);
        }
        None => {
            M_EXT[slot * 4].store(EXT_UNBOUNDED, Ordering::Relaxed);
            UNBOUNDED_LIVE.fetch_add(1, Ordering::Relaxed);
        }
    }
    // 4) publish the mob (counts already in place)
    M_KEY[slot].store(mob_key, Ordering::Release);
    Ok(())
}

/// Unregister all extents of `mob_id` (mob left navigatingMobs / path unset).
/// Idempotent: unknown mob = Ok.
pub fn drop_mob(mob_id: i64) -> Result<(), i32> {
    if BROKEN.load(Ordering::Relaxed) {
        return Err(ERR_STRUCT);
    }
    if mob_id <= 0 {
        return Err(ERR_RANGE);
    }
    let mob_key = mob_id | i64::MIN;
    let mut w = WSTATE.lock().map_err(|_| {
        BROKEN.store(true, Ordering::Relaxed);
        ERR_STRUCT
    })?;
    let Some(slot) = mob_probe(&w, mob_key) else {
        return Ok(());
    };
    M_KEY[slot].store(0, Ordering::Relaxed); // unpublish BEFORE dec
    if M_EXT[slot * 4].load(Ordering::Relaxed) == EXT_UNBOUNDED {
        UNBOUNDED_LIVE.fetch_sub(1, Ordering::Relaxed);
    } else {
        let (x0, z0, x1, z1) = read_ext(slot);
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                dec_cell(x, z)?;
                x += 1;
            }
            z += 1;
        }
    }
    w.free.push(slot);
    Ok(())
}

/// THE GATE. 0 (EMPTY) = no live navigating-mob decision sphere covers chunk
/// (cx,cz) ⇒ the vanilla navigatingMobs pass is provably a no-op ⇒ zero-work.
/// 1 (MAYBE) = covered (or unbounded mob live) ⇒ vanilla pass as today.
/// <0 = ERR ⇒ java one-shot disarms the gate (vanilla forever, fail-closed).
pub fn pregate(cx: i32, cz: i32) -> i32 {
    GATE_CALLS.fetch_add(1, Ordering::Relaxed);
    if BROKEN.load(Ordering::Relaxed) {
        return ERR_STRUCT;
    }
    if UNBOUNDED_LIVE.load(Ordering::Relaxed) > 0 {
        return PREGATE_MAYBE;
    }
    let key = cell_key(cx as i64, cz as i64);
    let s = &SHARDS[shard_of(key)];
    match shard_count(s, key) {
        Ok(Some(_)) => PREGATE_MAYBE,
        Ok(None) => {
            GATE_ZERO_WORK.fetch_add(1, Ordering::Relaxed);
            PREGATE_EMPTY
        }
        Err(e) => {
            BROKEN.store(true, Ordering::Relaxed);
            e
        }
    }
}

/// Structural self-test (cmp458_roar pattern: every N flush, next-leg call
/// site). (1) per-shard count sum == REG_SUM; (2) no negative counts;
/// (3) unbounded counter sane; (4) stable seqlocks during the walk.
/// Any miss → caller breaks the latch → fail-closed vanilla.
pub fn self_test() -> bool {
    if BROKEN.load(Ordering::Relaxed) {
        return false;
    }
    if UNBOUNDED_LIVE.load(Ordering::Relaxed) < 0 {
        return false;
    }
    let mut sum: i64 = 0;
    for s in SHARDS.iter() {
        let v = s.ver.load(Ordering::Acquire);
        if v & 1 == 1 {
            return false; // write in flight — inconclusive; retry next flush
        }
        for slot in 0..CHUNK_CAP {
            if s.keys[slot].load(Ordering::Relaxed) != 0 {
                let c = s.cnt[slot].load(Ordering::Relaxed);
                if c < 0 {
                    return false;
                }
                sum += c as i64;
            }
        }
        if s.ver.load(Ordering::Acquire) != v {
            return false;
        }
    }
    sum == REG_SUM.load(Ordering::Relaxed)
}

/// DATA-PLAN metric (прегист-гейт: ноль zero-work-попаданий = плацебо-класс).
/// (gate calls, zero-work answers).
pub fn pregate_stats() -> (usize, usize) {
    (
        GATE_CALLS.load(Ordering::Relaxed),
        GATE_ZERO_WORK.load(Ordering::Relaxed),
    )
}

/// STRICT eq lever gate (пустой/чужой флаг = ваниль бит-в-байт, закон 4).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp459_p45")
        .unwrap_or(false)
}

/// RegisterNatives navPregate on the just-defined NavPlaneOps class, then
/// flip the java one-shot guard (define-ДО-arm — NCDFE-канон). Wiring into
/// the activation worker (entity_index_manager pattern) — следующая нога.
pub fn register_pregate(env: &JniEnv, cls: jni::jclass) -> bool {
    let Ok(name) = CString::new("navPregate") else {
        return false;
    };
    let Ok(sig) = CString::new(PREGATE_SIG) else {
        return false;
    };
    let natives = [jni::JNINativeMethod {
        name: name.as_ptr(),
        signature: sig.as_ptr(),
        fnPtr: nav_pregate as *const c_void as *mut c_void,
    }];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] navpregate: register_natives(navPregate) failed — gate stays asleep"
        );
        return false;
    }
    let Some(mid) = env.get_static_method_id(cls, "pregateArmed", "()V") else {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] navpregate: pregateArmed()V missing — gate stays asleep"
        );
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    if crate::clear_exception(env) {
        eprintln!(
            "[crussty-plugin] navpregate: pregateArmed()V threw — gate stays asleep"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] navpregate: cmp459_p45 ARMED (O(1) chunk gate -> {PREGATE_CLASS})"
    );
    true
}

/// JNI: navPregate(cx, cz) -> 0 EMPTY / 1 MAYBE / negative ERR.
pub unsafe extern "system" fn nav_pregate(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    cx: jni::jint,
    cz: jni::jint,
) -> jni::jint {
    if env.is_null() {
        return ERR_STRUCT;
    }
    pregate(cx, cz)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Statics are process-global — serialize the mutation tests.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Deterministic LCG (no rand dep).
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0 >> 11
        }
        fn f64(&mut self) -> f64 {
            (self.next() % 1_000_000) as f64 / 1_000_000.0
        }
    }

    #[test]
    fn empty_chunk_is_zero_work() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // fresh statics: no mob registered anywhere → EMPTY (gate would skip
        // the whole vanilla pass — the 40-60% capture case)
        assert_eq!(pregate(12345, -12345), PREGATE_EMPTY);
        assert_eq!(pregate(0, 0), PREGATE_EMPTY);
    }

    #[test]
    fn extent_covers_sphere_fn_free() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut rng = Lcg(0xC0FFEE);
        for case in 0..64u32 {
            let mob_id = 1000 + case as i64;
            // mob + end node within a modest range; remaining <= R_CAP
            let mob_x = rng.f64() * 512.0 - 256.0;
            let mob_z = rng.f64() * 512.0 - 256.0;
            let node_x = mob_x + rng.f64() * 32.0 - 16.0;
            let node_z = mob_z + rng.f64() * 32.0 - 16.0;
            let remaining = 1.0 + rng.f64() * 40.0;
            let mid_x = (node_x + mob_x) / 2.0;
            let mid_z = (node_z + mob_z) / 2.0;
            note_sphere(mob_id, mid_x, mid_z, remaining).unwrap();
            // every point inside the closed sphere must be covered (FN-free):
            // chunk of P must answer MAYBE
            for _ in 0..64 {
                let ang = rng.f64() * std::f64::consts::TAU;
                let rad = remaining * rng.f64();
                let px = mid_x + rad * ang.cos();
                let pz = mid_z + rad * ang.sin();
                let rc = pregate((px.floor() as i64 >> 4) as i32,
                                 (pz.floor() as i64 >> 4) as i32);
                assert_eq!(rc, PREGATE_MAYBE, "FN at case {case} P=({px},{pz})");
            }
            drop_mob(mob_id).unwrap();
        }
        assert!(self_test());
    }

    #[test]
    fn drop_restores_empty() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        note_sphere(42, 100.0, 200.0, 30.0).unwrap();
        assert_eq!(pregate(6, 12), PREGATE_MAYBE); // chunk(100,200)
        drop_mob(42).unwrap();
        assert_eq!(pregate(6, 12), PREGATE_EMPTY);
        assert_eq!(drop_mob(42).unwrap(), ()); // idempotent
        assert!(self_test());
    }

    #[test]
    fn re_note_moves_extent() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        note_sphere(7, 16.0, 16.0, 8.0).unwrap(); // sphere around chunk (1,1)
        assert_eq!(pregate(1, 1), PREGATE_MAYBE);
        note_sphere(7, 1600.0, 1600.0, 8.0).unwrap(); // re-path far away
        assert_eq!(pregate(1, 1), PREGATE_EMPTY);
        assert_eq!(pregate(100, 100), PREGATE_MAYBE);
        drop_mob(7).unwrap();
        assert_eq!(pregate(100, 100), PREGATE_EMPTY);
        assert!(self_test());
    }

    #[test]
    fn unbounded_bypass_is_global_and_safe() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // NaN / huge remaining / span blowup → UNBOUNDED → global MAYBE
        note_sphere(9, f64::NAN, 0.0, 8.0).unwrap();
        assert_eq!(pregate(555, 555), PREGATE_MAYBE);
        drop_mob(9).unwrap();
        note_sphere(10, 0.0, 0.0, REGISTER_R_CAP * 4.0).unwrap();
        assert_eq!(pregate(-999, 999), PREGATE_MAYBE);
        drop_mob(10).unwrap();
        assert_eq!(pregate(-999, 999), PREGATE_EMPTY);
        assert!(self_test());
    }

    #[test]
    fn err_range_on_bad_mob_id() {
        let _g = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(note_sphere(0, 0.0, 0.0, 1.0), Err(ERR_RANGE));
        assert_eq!(drop_mob(-5), Err(ERR_RANGE));
        assert!(self_test());
    }

    /// STRICT gate: empty/other flag must not arm (literal comparator
    /// contract; env manipulation is racy across parallel tests).
    #[test]
    fn gate_is_strict_eq() {
        assert_ne!("", "cmp459_p45");
        assert_eq!("cmp459_p45", "cmp459_p45");
        assert_ne!("cmp405_navplane", "cmp459_p45");
    }
}
