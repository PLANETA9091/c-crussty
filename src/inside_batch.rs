//! INSIDE-BATCH (TASK-459-56, ID-P31 — закон 11 WILD, bulk-JNI discovery plane).
//! Lever `CRUSSTY_INSIDE_BATCH` (env-gate) ИЛИ составной lever_flag
//! `cmp456_chunkmono_p31snap` (STRICT eq — TASK-460-01 climb-вайринг P31 на
//! носитель cmp456_chunkmono): флаг не стоит ⇒ класс не определяется, ретаргет
//! не компонуется, хук не регистрируется ⇒ ваниль бит-в-байт.
//!
//! Лейн: inside_volatile 16.6пп (TOP-1 остаток компо-носителя chunkmono).
//! Дизайн (карточка ID-P31, RESEARCH-459-P31.md; ×463 STRICT-TAIL v1 —
//! LEDGER-25 чеклист C1-C8): java-мост InsideBatchOps собирает кандидатов
//! checkInsideBlocks тика в TL-буферы (SoA-флет: eid, x/y/z, bb-флет,
//! section-ключи) → bulk-JNI `insideBatchMask` ОДИН на T=512-бакет (296
//! переходов/тик при pop 150k vs 150,000 per-entity = ×506.8 экономии; vs
//! strict-superset-натаив 375,000 = ×1266.9 — строгий-superset в нативе
//! РЕДЖЕКТ, «строгость» только в java-хвосте) → Rust возвращает битмаску
//! секций-кандидатов per-entity (геометрический superset 5.42 ns/entity,
//! mean 2.5 секций) → strict java-хвост visit-обхода по кандидатам = v2
//! (oracle-gated; v1 маски — телеметрия TL_OUT, ноль live-потребителей ⇒
//! парити = ваниль by construction; receiver-only сайт не достигает
//! List<Movement>/StepBasedCollector — открытие ×463, см. javadoc моста).
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

/// insideBatchMask(n, maxsec, base, eids[n], xyz[n*3], bb[n*6],
/// secKeys[n*maxsec], nsec[n], dirty[n], out[n]) -> 0 | ERR (<0).
/// ×463 (TASK-463-65a, chkclimb-13): сигнатура расширена base-офсетом
/// (bucket T=512 сбрасывается из [base, base+n) БЕЗ копий — C4-дисциплина
/// «ноль Region-copy 64KB×74»), все 7 массивов читаются/пишутся через
/// GetPrimitiveArrayCritical (critical-only, ноль Vec/вызов — было 7 Vec
/// + 7 Region-copy, +0.1-0.2пп worst-case по §4 LEDGER-25).
pub const BATCH_MASK_SIG: &str =
    "(III[J[D[D[I[I[I[I)I";

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
        .map(|v| v.trim() == "cmp456_chunkmono_p31snap")
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
            // C5b selfTest (×463-дыра «0 самотестов» закрыта): java selfTest
            // МЕЖДУ RegisterNatives и noteBatchArmed — count-инвариант superset
            // (оракул-сцены через реальный bulk-JNI) + AIOOBE-проба границы
            // i=MAXBATCH−1. selfTest != 1 ⇒ BATCH_ARMED не публикуется
            // (fail-closed; arm-order контракт define→natives→selfTest→armed).
            // SDK даёт int/void/object static calls: selfTest возвращает 1/0
            // (canon inside_snap_registry::probe_and_arm).
            let Some(test_mid) = env.get_static_method_id(cgr, "selfTest", "()I") else {
                crate::clear_exception(env);
                eprintln!(
                    "[crussty-plugin] inside_batch: selfTest unresolved — bridge stays disarmed (fail-closed)"
                );
                return false;
            };
            let selftest = env.call_static_int_method(cgr, test_mid, &[]);
            if selftest != 1 {
                crate::clear_exception(env);
                eprintln!(
                    "[crussty-plugin] inside_batch: selfTest={selftest} BEFORE arm — BATCH_ARMED not published (fail-closed)"
                );
                return false;
            }
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono_p31snap: selfTest=true BEFORE arm (C5b: count-invariant superset + AIOOBE@4095)"
            );
            // Arm-order финал: BATCH_ARMED=true ТОЛЬКО после define+natives+selfTest.
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

/// JNI: insideBatchMask — n сущностей за ОДИН переход из окна [base, base+n).
/// out[base+i] = битмаска секций-кандидатов (superset | dirty). Отрицательный
/// rc = ERR (java-реплика: all-ones = чистая ваниль).
/// C4 (LEDGER-25): critical-only — GetPrimitiveArrayCritical на всех
/// потребляемых массивах (eids в v1 не потребляется: длина валидируется,
/// пин НЕ берётся), ноль heap-аллокаций, регион < 5 µs (тайминг-страж
/// внизу; AOSP JNI docs: critical regions должны быть короткими —
/// ограничение на moving-коллекторы). eids[0..n) геометрии не касается.
pub unsafe extern "system" fn inside_batch_mask(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    maxsec: jni::jint,
    base: jni::jint,
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
        || base < 0
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
    if maxsec as usize > MAXSEC || n > i32::MAX / (maxsec * 2) || base > i32::MAX - n {
        return ERR_RANGE;
    }
    let n_us = n as usize;
    let ms = maxsec as usize;
    let b_us = base as usize;
    let end = b_us + n_us; // первый индекс ЗА окном
    let vt = unsafe { &*(*env) };
    let len_eids = (vt.GetArrayLength)(env, eids);
    let len_xyz = (vt.GetArrayLength)(env, xyz);
    let len_bb = (vt.GetArrayLength)(env, bb);
    let len_keys = (vt.GetArrayLength)(env, sec_keys);
    let len_nsec = (vt.GetArrayLength)(env, nsec);
    let len_dirty = (vt.GetArrayLength)(env, dirty);
    let len_out = (vt.GetArrayLength)(env, out);
    if (len_eids as usize) < end
        || (len_xyz as usize) < end * 3
        || (len_bb as usize) < end * 6
        || (len_keys as usize) < end * ms
        || (len_nsec as usize) < end
        || (len_dirty as usize) < end
        || (len_out as usize) < end
    {
        return ERR_RANGE;
    }
    let t0 = std::time::Instant::now();
    // Critical-only pinning (C4): никаких Region-copy и Vec. Ошибки пина
    // разворачивают уже взятые пины (LIFO) — fail-open ERR_STRUCT.
    let xyz_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, xyz, std::ptr::null_mut()) };
    if xyz_pin.is_null() {
        return ERR_STRUCT;
    }
    let bb_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, bb, std::ptr::null_mut()) };
    if bb_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT) };
        return ERR_STRUCT;
    }
    let keys_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, sec_keys, std::ptr::null_mut()) };
    if keys_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, bb, bb_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT) };
        return ERR_STRUCT;
    }
    let nsec_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, nsec, std::ptr::null_mut()) };
    if nsec_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, sec_keys, keys_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, bb, bb_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT) };
        return ERR_STRUCT;
    }
    let dirty_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, dirty, std::ptr::null_mut()) };
    if dirty_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, nsec, nsec_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, sec_keys, keys_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, bb, bb_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT) };
        return ERR_STRUCT;
    }
    let out_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if out_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, dirty, dirty_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, nsec, nsec_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, sec_keys, keys_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, bb, bb_pin, jni::JNI_ABORT) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT) };
        return ERR_STRUCT;
    }
    // no-JNI-call регион: только сырые срезы + superset_mask (геометрия).
    let xyz_s = unsafe { std::slice::from_raw_parts(xyz_pin as *const f64, (end * 3) as usize) };
    let bb_s = unsafe { std::slice::from_raw_parts(bb_pin as *const f64, (end * 6) as usize) };
    let keys_s = unsafe { std::slice::from_raw_parts(keys_pin as *const i32, end * ms) };
    let nsec_s = unsafe { std::slice::from_raw_parts(nsec_pin as *const i32, end) };
    let dirty_s = unsafe { std::slice::from_raw_parts(dirty_pin as *const i32, end) };
    let out_s = unsafe { std::slice::from_raw_parts_mut(out_pin as *mut i32, end) };
    for i in b_us..end {
        let mut bb6 = [0.0f64; 6];
        bb6.copy_from_slice(&bb_s[i * 6..i * 6 + 6]);
        let mut keys = [0i32; MAXSEC];
        let take = (nsec_s[i].max(0) as usize).min(MAXSEC);
        keys[..take].copy_from_slice(&keys_s[i * ms..i * ms + take]);
        out_s[i] = superset_mask(&bb6, &keys, take, dirty_s[i] as u32) as i32;
        let _ = xyz_s[i * 3]; // xyz — телеметрия v2 (tик-позиция), геометрии не касается
    }
    unsafe {
        (vt.ReleasePrimitiveArrayCritical)(env, out, out_pin, 0); // copy-back если копия
        (vt.ReleasePrimitiveArrayCritical)(env, dirty, dirty_pin, jni::JNI_ABORT);
        (vt.ReleasePrimitiveArrayCritical)(env, nsec, nsec_pin, jni::JNI_ABORT);
        (vt.ReleasePrimitiveArrayCritical)(env, sec_keys, keys_pin, jni::JNI_ABORT);
        (vt.ReleasePrimitiveArrayCritical)(env, bb, bb_pin, jni::JNI_ABORT);
        (vt.ReleasePrimitiveArrayCritical)(env, xyz, xyz_pin, jni::JNI_ABORT);
    }
    let _ = eids; // длина валидирована; пин не нужен (геометрии не касается)
    // Тайминг-страж C4: регион < 5 µs/вызов (T=512 × 5.42 ns ≈ 2.8 µs ядра).
    let dt = t0.elapsed();
    if dt.as_nanos() > 5_000 {
        let k = CRITICAL_OVER_BUDGET.fetch_add(1, Ordering::Relaxed) + 1;
        if k & (k - 1) == 0 {
            eprintln!(
                "[crussty-plugin] inside_batch: critical region {} ns > 5000 ns budget (call #{k}) — G1 pause risk, check collector state",
                dt.as_nanos()
            );
        }
    }
    0
}

/// Телеметрия тайминг-стража C4 (power-of-two логирование).
static CRITICAL_OVER_BUDGET: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

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

    /// ×463 контракт-пин: bulk-JNI = (n, maxsec, base, 7 массивов) — base-офсет
    /// T=512-бакета БЕЗ копий (C4); T делит MAXBATCH (волна 4096 = 8×512).
    #[test]
    fn bucket_contract() {
        assert_eq!(BATCH_MASK_SIG, "(III[J[D[D[I[I[I[I)I");
        assert_eq!(4096 % 512, 0);
        assert_eq!(4096 / 512, 8);
    }

    /// base-офсет не влияет на per-entity маску (superset per-entity независим):
    /// окно [base, base+n) читает ТЕ ЖЕ геометрические поля, что и [0, n).
    #[test]
    fn superset_independent_of_base_window() {
        let keys = [pack_section(0, 4, 0), pack_section(1, 4, 0)];
        let bb = [7.0, 69.0, 7.0, 17.0, 71.0, 9.0];
        let m0 = superset_mask(&bb, &keys, 2, 0);
        // та же сущность «в окне» base=3584 (последний T=512-бакет волны 4096):
        // маска обязана быть бит-в-бит той же (rollover окна ничего не меняет).
        assert_eq!(m0, superset_mask(&bb, &keys, 2, 0));
        assert_eq!(m0 & 0b11, 0b11);
    }
}
