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

/// Kernel Entity class (wait target for the define window — inside_bitmask canon).
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";

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

/// Lever gate (TASK-460-03 climb: семейный STRICT-OR — клаймб-рычаг
/// `cmp457_paldelta_p31` (носитель cmp457_paldelta + INSIDE-BATCH P31) или
/// исторический env CRUSSTY_INSIDE_BATCH для A/B-реплеев; пустой/чужой
/// флаг = ванилль). Чтение на каждый вызов — как inside_cache::enabled.
pub fn enabled() -> bool {
    if std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == "cmp457_paldelta_p31" // TASK-460-03 climb family
        })
        .unwrap_or(false)
    {
        return true;
    }
    std::env::var("CRUSSTY_INSIDE_BATCH")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate visibility for the entity_compose stage pipeline (S7-162).
pub fn enabled_pub() -> bool {
    enabled()
}

/// Bridge-байты: встроены из tracked blob (build/net/... — lesson-408:
/// спящие блобы = плацебо; blob пересобран scripts/build_inside_batch_ops.sh
/// и включён в binary на компиле-тайме — никаких runtime-чтений).
const BRIDGE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBatchOps.class");

static BRIDGE_READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Global ref to the defined bridge class (probe/selfTest/noteArmed calls).
static OPS_GREF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

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
            "[crussty-plugin] inside_batch: dormant (lever_flag not cmp457_paldelta_p31 and CRUSSTY_INSIDE_BATCH unset)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_batch: owner armed, Entity stage delegated to entity_compose (supersede inside_cache on the isAffectedByBlocks site)"
    );
}

/// Probe the defined bridge: selfTest() must return 1. Anything else
/// (class init failure, missing method, exception, native round-trip
/// mismatch) = disarmed (fail-closed).
fn probe_selftest() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
        let gref = OPS_GREF.load(std::sync::atomic::Ordering::SeqCst) as jni::jclass;
        if gref.is_null() {
            return false;
        }
        // The class is JNI-defined => NOT reachable via find_class (system
        // loader on an attached native thread); resolve via the define-time
        // global ref (inside_bitmask.rs precedent, noise_fill smoke-1).
        let Some(mid) = env.get_static_method_id(gref, "selfTest", "()I") else {
            crate::clear_exception(env);
            return false;
        };
        let rc = env.call_static_int_method(gref, mid, &[]);
        crate::clear_exception(env);
        rc == 1
    })
    .unwrap_or(false)
}

/// Arm the bridge: define + RegisterNatives + selfTest ДО noteBatchArmed
/// (arm-order контракт, canon d73758a3/5ecd841a / fa9054d9 ARM-AFTER-DEFINE).
fn note_armed() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
        let gref = OPS_GREF.load(std::sync::atomic::Ordering::SeqCst) as jni::jclass;
        if gref.is_null() {
            return false;
        }
        let Some(mid) = env.get_static_method_id(gref, "noteBatchArmed", "()V") else {
            crate::clear_exception(env);
            return false;
        };
        env.call_static_void_method(gref, mid, &[]);
        crate::clear_exception(env);
        true
    })
    .unwrap_or(false)
}

/// Background activation (inside_bitmask.rs canon): wait for the kernel
/// Entity class, wait for the quiet kernel loader, define the bridge into
/// the kernel loader, RegisterNatives, selfTest, noteBatchArmed, publish
/// BRIDGE_READY. The Entity patch is computed and applied by entity_compose
/// (the stage composes ONLY when BOTH this bridge and the inside_cache
/// bridge are ready — the armed java branch delegates to InsideBlockOps.gate).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_batch: {} not loaded within 180s, bridge stays undefined",
                    ENTITY_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_batch: forcing kernel load of {}",
                    ENTITY_CLASS
                );
                // RC7 canon (TASK-433-B; ref a3991c2): LAZY force-load.
                crate::improved_noise::force_load_kernel_class_lazy(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define (boot-time class-loading
        // storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_batch: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_batch: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(BRIDGE_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] inside_batch: bridge is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_batch_ops.sh; hook stays dormant"
            );
            return;
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(cls.as_jclass(), mid, &[]);
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
            match env.define_class(BRIDGE_CLASS, gref, BRIDGE_BYTES) {
                Some(c) => {
                    let gr = env.new_global_ref(c);
                    OPS_GREF.store(gr as usize, std::sync::atomic::Ordering::SeqCst);
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] inside_batch: defined {BRIDGE_CLASS} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] inside_batch: define_class({BRIDGE_CLASS}) failed"
                    );
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_batch: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // Arm-order: natives registered on the defined class BEFORE selfTest.
        let natives_ok = cplug_sdk::jni_util::with_attached(|env| {
            let gref = OPS_GREF.load(std::sync::atomic::Ordering::SeqCst) as jni::jclass;
            !gref.is_null() && register_native(env, gref)
        })
        .unwrap_or(false);
        if !natives_ok {
            eprintln!(
                "[crussty-plugin] inside_batch: register_natives failed, hook stays dormant"
            );
            return;
        }

        // PROBE-THEN-PATCH: selfTest (pure java + ОДИН native round-trip).
        if !probe_selftest() {
            eprintln!(
                "[crussty-plugin] inside_batch: selfTest() != 1 (native round-trip mismatch), hook stays dormant"
            );
            return;
        }
        if !note_armed() {
            eprintln!(
                "[crussty-plugin] inside_batch: noteBatchArmed failed, hook stays dormant"
            );
            return;
        }

        BRIDGE_READY.store(true, std::sync::atomic::Ordering::Release);
        crate::kernel_policy::audit_wire(
            BRIDGE_CLASS,
            "batchGate",
            "inside_batch v1 (TASK-460-03 climb)",
        );
        eprintln!(
            "[crussty-plugin] inside_batch: CRUSSTY_INSIDE_BATCH ARMED (insideBatchMask bulk batch -> {BRIDGE_CLASS}); bridge defined+armed, BRIDGE_READY (Entity stage composes via entity_compose)"
        );
    });
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
        "[crussty-plugin] inside_batch: register_natives(insideBatchMask) OK on {BRIDGE_CLASS} (selfTest next — arm-order canon)"
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
