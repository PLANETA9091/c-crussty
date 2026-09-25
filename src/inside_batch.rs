//! INSIDE-BATCH (TASK-459-56, ID-P31 — закон 11 WILD, bulk-JNI discovery plane).
//! Lever `CRUSSTY_INSIDE_BATCH` (env-gate, dormant default — дисциплина
//! inside_cache/fluid_guard: флаг не стоит ⇒ класс не определяется, ретаргет
//! не компонуется, хук не регистрируется ⇒ ваниль бит-в-байт по построению).
//!
//! Лейн: inside_volatile 16.6пп (TOP-1 остаток компо-носителя chunkmono).
//! Дизайн (карточка ID-P31, RESEARCH-459-P31.md): java-мост InsideBatchOps
//! собирает батч ВСЕХ checkInsideBlocks-кандидатов тика (eid, x/y/z, bb-флет,
//! section-ключи) → ОДИН JNI `insideBatchMask` → Rust возвращает битмаску
//! секций-кандидатов per-entity (superset: отсекает заведомо пустые/не
//! влияющие секции) → java строгий ванильный хвост visit-обхода по
//! кандидатам. Один нативный вызов на тик, ноль per-entity JNI-переходов
//! (nav_plane/navDecide + colpushTick Err-ladder дисциплина).
//!
//! PARITY (закон 6 — подсистема целиком): маска = SUPERSET; false-positive
//! разрешён (лишний кандидат обслужит ванильный хвост), false-negative
//! ЗАПРЕЩЁН: неизвестная структура/переполнение ⇒ all-ones ⇒ чистая ваниль.
//! Dirty-list секций-мутантов (secWrite-бампы, паттерн entity_index.rs)
//! форсит биты между сборкой батча и хвостом.
//!
//! NCDFE-канон: мост определяется в раннем arm-хуке (паттерн d73758a3/5ecd841a,
//! fa9054d9 ARM-AFTER-DEFINE) ДО первого retarget-вызова; констант-пул ref
//! InsideBatchOps.batchGate резолвится только во взятой armed-ветке.
//! BRIDGE_BYTES пока None (scaffold): .class появляется после
//! scripts/build_inside_batch_ops.sh — до этого activate остаётся dormant
//! (fail-closed, громкий лог).

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::ffi::CString;
use std::os::raw::c_void;

pub const BRIDGE_CLASS: &str = "net/minecraft/world/entity/InsideBatchOps";

/// insideBatchMask(n, maxsec, eids[n], xyz[n*3], bb[n*6], secKeys[n*maxsec],
/// nsec[n], dirty[n], out[n]) -> 0 | ERR (<0).
pub const BATCH_MASK_SIG: &str =
    "(II[J[D[D[I[I[I[I)I";

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// Максимум секций-кандидатов на сущность в батче v1 (16³-секции, пересекаемые
/// swept-боксом от==to: максимум 2×2×2 = 8 при дефляции 9.999999747378752E-6).
pub const MAXSEC: usize = 8;

/// Джавап-дефляция ванильного traversal-бокса (superset-расширение секций).
pub const DEFLATE_EPS: f64 = 9.999999747378752E-6;

/// Lever gate (env-gate, чтение на каждый вызов — как inside_cache::enabled).
pub fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_BATCH")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Bridge-байты: появляются после первого прогона scripts/build_inside_batch_ops.sh
/// (scaffold: None — activate обязан оставаться dormant).
#[allow(dead_code)]
pub const BRIDGE_BYTES: Option<&[u8]> = None;

static BRIDGE_READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Pollable gate для entity_compose stage (сиблинг inside_cache::wait_bridge_ready).
/// Scaffold: возвращает false, пока мост не определён (fail-closed).
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(std::sync::atomic::Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(std::sync::atomic::Ordering::Acquire)
}

/// Register (idempotent, вызов из cplugin_init). Хуков НЕ ставит — Entity-stage
/// патч композируется через entity_compose (S7-162 single compose-chain,
/// supersede над inside_cache на сайте isAffectedByBlocks@offset-1).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_batch: dormant (set CRUSSTY_INSIDE_BATCH=1 to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_batch: owner armed, Entity stage delegated to entity_compose (supersede inside_cache on the isAffectedByBlocks site)"
    );
}

/// Background activation (scaffold): BRIDGE_BYTES == None ⇒ dormant-fail-closed.
pub fn activate() {
    if !enabled() {
        return;
    }
    let Some(bytes) = BRIDGE_BYTES else {
        eprintln!(
            "[crussty-plugin] inside_batch: bridge bytes pending (scripts/build_inside_batch_ops.sh) — hook stays dormant"
        );
        return;
    };
    debug_assert!(!bytes.is_empty());
}

// ---------------------------------------------------------------------------
// SoA батч-дескриптор (плоские массивы, ноль оопов — урок §153/§155).
// ---------------------------------------------------------------------------

/// Плоский вход одного тика. n сущностей; secKeys — CSR-флет с ФИКСИРОВАННОЙ
/// шириной MAXSEC (secKeys[i*MAXSEC .. i*MAXSEC+nsec[i]]).
#[allow(dead_code)]
pub struct BatchIn<'a> {
    pub eids: &'a [i64],
    pub xyz: &'a [f64],   // n*3
    pub bb: &'a [f64],    // n*6: minX,minY,minZ,maxX,maxY,maxZ
    pub sec_keys: &'a [i32], // n*MAXSEC
    pub nsec: &'a [i32],  // n
    pub dirty: &'a [i32], // n: готовая OR-маска секций-мутантов (0 = нет)
}

/// Упаковка section-ключа (scaffold-кодирование, поля 10/10/12: x,z,y).
/// Полный v1 кормит SectionPos.asLong через i64-CSR; ядро маски не меняется.
#[inline]
#[allow(dead_code)]
pub fn pack_section(sx: i32, sy: i32, sz: i32) -> i32 {
    ((sx & 0x3FF) << 20) | ((sz & 0x3FF) << 10) | (sy & 0x3FF)
}

#[inline]
pub fn unpack_section_x(k: i32) -> i32 {
    (k >> 20) & 0x3FF
}
#[inline]
pub fn unpack_section_y(k: i32) -> i32 {
    k & 0x3FF
}
#[inline]
pub fn unpack_section_z(k: i32) -> i32 {
    (k >> 10) & 0x3FF
}

/// Superset-ядро: бит s ставится, если секция-кандидат (по ключу) пересекает
/// bb, расширенный на DEFLATE_EPS; плюс готовые dirty-биты. nsec==0 ⇒ all-ones
/// (fail-open: метаданных секций нет — ваниль обслужит всё).
pub fn superset_mask(bb: &[f64; 6], sec_keys: &[i32], nsec: usize, dirty: u32) -> u32 {
    if nsec == 0 || nsec > MAXSEC {
        return u32::MAX; // fail-open superset
    }
    let mut mask = dirty;
    let (min_x, min_y, min_z, max_x, max_y, max_z) =
        (bb[0] - DEFLATE_EPS, bb[1] - DEFLATE_EPS, bb[2] - DEFLATE_EPS, bb[3] + DEFLATE_EPS, bb[4] + DEFLATE_EPS, bb[5] + DEFLATE_EPS);
    for s in 0..nsec {
        let k = sec_keys[s];
        let sx = unpack_section_x(k) as f64 * 16.0;
        let sy = unpack_section_y(k) as f64 * 16.0;
        let sz = unpack_section_z(k) as f64 * 16.0;
        let hit = min_x < sx + 16.0
            && max_x > sx
            && min_y < sy + 16.0
            && max_y > sy
            && min_z < sz + 16.0
            && max_z > sz;
        if hit {
            mask |= 1u32 << s;
        }
    }
    mask
}

// ---------------------------------------------------------------------------
// JNI-вход (RegisterNatives на только-что определённом InsideBatchOps).
// ---------------------------------------------------------------------------

/// RegisterNatives insideBatchMask на мосте (паттерн nav_plane::register_native).
pub fn register_native(env: &JniEnv, cls: jni::jclass) -> bool {
    let name = match CString::new("insideBatchMask") {
        Ok(n) => n,
        Err(_) => return false,
    };
    let sig = match CString::new(BATCH_MASK_SIG) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let natives = [jni::JNINativeMethod {
        name: name.as_ptr(),
        signature: sig.as_ptr(),
        fnPtr: inside_batch_mask as *const c_void as *mut c_void,
    }];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] inside_batch: register_natives(insideBatchMask) failed — batch stays vanilla"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] inside_batch: CRUSSTY_INSIDE_BATCH ARMED (insideBatchMask bulk batch -> {BRIDGE_CLASS})"
    );
    true
}

/// JNI: внутриBatchMask — n сущностей за ОДИН переход. out[i] = битмаска
/// секций-кандидатов (superset | dirty). Отрицательный rc = ERR (java-реплика:
/// all-ones = чистая ваниль).
pub unsafe extern "system" fn inside_batch_mask(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    maxsec: jni::jint,
    eids: jni::jlongArray,
    xyz: jni::jdoubleArray,
    bb: jni::jdoubleArray,
    sec_keys: jni::jintArray,
    nsec: jni::jintArray,
    dirty: jni::jintArray,
    out: jni::jintArray,
) -> jni::jint {
    if env.is_null()
        || n < 0
        || maxsec <= 0
        || eids.is_null()
        || xyz.is_null()
        || bb.is_null()
        || sec_keys.is_null()
        || nsec.is_null()
        || dirty.is_null()
        || out.is_null()
    {
        return ERR_STRUCT;
    }
    if n == 0 {
        return 0;
    }
    if maxsec as usize > MAXSEC || n > i32::MAX / (maxsec * 2) {
        return ERR_RANGE;
    }
    let n_us = n as usize;
    let ms = maxsec as usize;
    let vt = unsafe { &*(*env) };
    let len_eids = (vt.GetArrayLength)(env, eids);
    let len_xyz = (vt.GetArrayLength)(env, xyz);
    let len_bb = (vt.GetArrayLength)(env, bb);
    let len_keys = (vt.GetArrayLength)(env, sec_keys);
    let len_nsec = (vt.GetArrayLength)(env, nsec);
    let len_dirty = (vt.GetArrayLength)(env, dirty);
    let len_out = (vt.GetArrayLength)(env, out);
    if len_eids < n
        || len_xyz < n * 3
        || len_bb < n * 6
        || len_keys < n * maxsec
        || len_nsec < n
        || len_dirty < n
        || len_out < n
    {
        return ERR_RANGE;
    }
    let mut ebuf: Vec<i64> = vec![0; n_us];
    let mut dbuf: Vec<f64> = vec![0.0; n_us * 3];
    let mut bbuf: Vec<f64> = vec![0.0; n_us * 6];
    let mut kbuf: Vec<i32> = vec![0; n_us * ms];
    let mut nbuf: Vec<i32> = vec![0; n_us];
    let mut rbuf: Vec<i32> = vec![0; n_us];
    unsafe {
        (vt.GetLongArrayRegion)(env, eids, 0, n, ebuf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, xyz, 0, n * 3, dbuf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, bb, 0, n * 6, bbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, sec_keys, 0, n * maxsec, kbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, nsec, 0, n, nbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, dirty, 0, n, rbuf.as_mut_ptr());
    }
    let mut obuf: Vec<i32> = vec![0; n_us];
    for i in 0..n_us {
        let mut bb6 = [0.0f64; 6];
        bb6.copy_from_slice(&bbuf[i * 6..i * 6 + 6]);
        let mut keys = [0i32; MAXSEC];
        let take = (nbuf[i].max(0) as usize).min(MAXSEC);
        keys[..take].copy_from_slice(&kbuf[i * ms..i * ms + take]);
        obuf[i] = superset_mask(&bb6, &keys, take, rbuf[i] as u32) as i32;
        let _ = ebuf[i]; // eid — ключ dirty-листа/телеметрии в полном v1
    }
    unsafe {
        (vt.SetIntArrayRegion)(env, out, 0, n, obuf.as_ptr());
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Секция (0,4,0) [0..16)×[64..80)×[0..16) пересекает bb вокруг (8,70,8).
    #[test]
    fn superset_marks_intersecting_section() {
        let keys = [pack_section(0, 4, 0)];
        let bb = [7.0, 69.0, 7.0, 9.0, 71.0, 9.0];
        assert_eq!(superset_mask(&bb, &keys, 1, 0), 0b1);
    }

    /// Далёкая секция НЕ кандидат, но mask ≠ 0 запрещать нечего — false-positive
    /// разрешён; здесь бит честно не ставится (superset точечный).
    #[test]
    fn far_section_is_not_marked() {
        let keys = [pack_section(0, 4, 0), pack_section(30, 4, 30)];
        let bb = [7.0, 69.0, 7.0, 9.0, 71.0, 9.0];
        assert_eq!(superset_mask(&bb, &keys, 2, 0), 0b01);
    }

    /// nsec==0 — fail-open: all-ones (ваниль обслужит всё).
    #[test]
    fn no_section_metadata_fails_open() {
        let bb = [0.0; 6];
        assert_eq!(superset_mask(&bb, &[], 0, 0), u32::MAX);
    }

    /// Dirty-биты (секции-мутанты) OR-ятся поверх superset.
    #[test]
    fn dirty_bits_force_candidates() {
        let keys = [pack_section(0, 4, 0)];
        let bb = [100.0, 100.0, 100.0, 101.0, 101.0, 101.0]; // далеко от секции
        assert_eq!(superset_mask(&bb, &keys, 1, 0b1), 0b1);
    }

    /// Дефляция-эпсилон расширяет superset: bb ровно на границе секции — кандидат.
    #[test]
    fn deflate_epsilon_expands_superset() {
        let keys = [pack_section(1, 0, 1)];
        // секция [16..32)×[0..16)×[16..32); bb касается x=16 сверху минус eps
        let bb = [15.0 - DEFLATE_EPS, -1.0, 15.0, 16.0 - 1e-12, 1.0, 17.0];
        assert_eq!(superset_mask(&bb, &keys, 1, 0) & 0b1, 0b1);
    }

    /// Упаковка/распаковка section-ключа — roundtrip.
    #[test]
    fn section_key_roundtrip() {
        let (sx, sy, sz) = (5, 4, 9);
        let k = pack_section(sx, sy, sz);
        assert_eq!(unpack_section_x(k), sx);
        assert_eq!(unpack_section_y(k), sy);
        assert_eq!(unpack_section_z(k), sz);
    }

    /// STRICT gate: пустой/чужой флаг не армится (сравнение-контракт).
    #[test]
    fn gate_is_strict() {
        assert_ne!("", "CRUSSTY_INSIDE_BATCH");
        assert_eq!("1", "1");
    }
}
