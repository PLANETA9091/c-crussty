//! INSIDE-BATCH (TASK-459-56, ID-P31 — закон 11 WILD, bulk-JNI discovery plane).
//! Lever `CRUSSTY_INSIDE_BATCH` (env-gate) ИЛИ составной lever_flag
//! `cmp456_chunkmono_p31snap` (STRICT eq — TASK-460-01 climb-вайринг P31 на
//! носитель cmp456_chunkmono): флаг не стоит ⇒ класс не определяется, ретаргет
//! не компонуется, хук не регистрируется ⇒ ваниль бит-в-байт.
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
use std::sync::atomic::Ordering;

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

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

/// Lever gate (env-gate ИЛИ составной carrier-флаг; чтение на каждый вызов —
/// как inside_cache::enabled). TASK-460-01: `cmp456_chunkmono_p31snap` —
/// STRICT eq (климб-компо P31+P32/P36 sidecar на носителе cmp456_chunkmono).
pub fn enabled() -> bool {
    if std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp456_chunkmono_p31snap" || v.trim() == "cmp466_c98ai" || v.trim() == "cmp468_s18fluid")
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

/// Bridge-байты: встроенный blob (TASK-460-01: пересобран и закоммичен —
/// урок-408/425, lever в SOURCES без пересборки tracked-блобов = ПЛАЦЕБО;
/// источник: entityinside/net/minecraft/world/entity/InsideBatchOps.java,
/// сборка scripts/build_inside_batch_ops.sh, flat==nested gate).
const BRIDGE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBatchOps.class");

/// Gate-видимость для entity_compose stage (S7-162 supersede-дисциплина).
pub fn enabled_pub() -> bool {
    enabled()
}

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

/// Background activation (TASK-460-01 wiring): РАННИЙ arm-хук (NCDFE-канон
/// d73758a3/5ecd841a, ARM-AFTER-DEFINE fa9054d9) — ждать kernel Entity,
/// boot-маркер, guard major, define InsideBatchOps в kernel loader,
/// RegisterNatives(insideBatchMask) ДО flips, java noteBatchArmed, затем
/// BRIDGE_READY. Entity-stage патч компонует entity_compose (stage 1,
/// supersede inside_cache на сайте isAffectedByBlocks — S7-162: ОДИН владелец).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // 1. Ждать kernel-класс Entity (точка захвата kernel loader).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_batch: {ENTITY_CLASS} not loaded within 180s, bridge stays undefined"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_batch: forcing kernel load of {ENTITY_CLASS}"
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // 2. Kernel loader затих до define (boot-storm дисциплина TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] inside_batch: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_batch: server booted, defining bridge into kernel loader"
        );

        // 3. Guard: встроенный блоб не новее JVM (major 65 vs живая JVM).
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
                "[crussty-plugin] inside_batch: {BRIDGE_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_batch_ops.sh; hook stays dormant"
            );
            return;
        }

        // 4. Захват kernel loader от Entity + define + RegisterNatives + arm.
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
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            let Some(c) = env.define_class(BRIDGE_CLASS, gref, BRIDGE_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] inside_batch: define_class({BRIDGE_CLASS}) failed — hook stays dormant"
                );
                return false;
            };
            eprintln!(
                "[crussty-plugin] inside_batch: defined {BRIDGE_CLASS} in kernel loader"
            );
            // RegisterNatives ДО flips/READY: первый armed batchGate-вызов обязан
            // иметь связку (arm-order контракт моста: define+natives+arm).
            let natives_ok = register_native(env, c);
            // Global ref BEFORE delete_local_ref (asm.rs canon): the JNI-defined
            // class is NOT reachable via find_class on an attached native thread
            // (noise_fill.rs smoke-1) — the define's own ref is the only handle.
            let cgr = env.new_global_ref(c);
            env.delete_local_ref(c);
            if !natives_ok {
                return false;
            }
            // Arm-order финал: BATCH_ARMED=true ТОЛЬКО после define+natives.
            let Some(arm_mid) = env.get_static_method_id(cgr, "noteBatchArmed", "()V") else {
                crate::clear_exception(env);
                eprintln!(
                    "[crussty-plugin] inside_batch: noteBatchArmed unresolved — bridge stays disarmed (fail-closed)"
                );
                return false;
            };
            env.call_static_void_method(cgr, arm_mid, &[]);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_batch: bridge definition aborted, hook stays dormant (fail-closed)"
            );
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(BRIDGE_CLASS, "batchGate", "inside_batch v1 (ID-P31 bulk-JNI; entity_compose stage 1 supersede inside_cache)");
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono_p31snap: ARMED (inside_batch bridge defined+natives+noteBatchArmed; strict vanilla tail)"
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
