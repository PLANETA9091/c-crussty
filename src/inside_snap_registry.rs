//! Runtime wiring for the INSIDE-SNAP REGISTRY SIDECAR (TASK-459-57, idea
//! ID-P32, закон 11 тик-459 — lever `cmp459_snapreg` STRICT eq, DORMANT
//! scaffold).
//!
//! ПОДСИСТЕМА (развитие inside_snap, lever cmp432_inside2/cmp436_ins4):
//! заменяет per-visit `SNAPS.get(sec)` CHM-хоп в inside_snap-гейте на
//! ФЛЕТ-индексный сервинг по per-section СТАБИЛЬНЫМ int-индексам:
//!
//!   CHM<LevelChunkSection, Snap>  ->  Snap[] REG по стабильному int idx
//!   fresh-check                   ->  epoch одним int-cmp (REG_EPOCH[idx])
//!   любой miss/stale/вне-реестра  ->  FAIL-CLOSED CHM fallback (действующая
//!                                     плоскость InsideSnapOps), затем ваниль
//!
//! Состав (закон 6 — НЕ одиночная функция):
//!   1. ФЛЕТ-РЕЕСТР (java-мост InsideSnapRegistryOps): REG[idx] -> Snap
//!      (ЗЕРКАЛА тех же ссылок, что держит CHM-плоскость; sidecar сам Snap
//!      НЕ создаёт — единственный источник истины остаётся collect-план).
//!   2. РЕГИСТРАЦИЯ: стабильный монотонный int idx на СЕКЦИЮ — один раз за
//!      жизнь секции (тот же каденс, что InsideSnapOps.register сегодня);
//!      per-visit SERVE-путь идентичности НЕ трогает. v2: idx вписывается в
//!      секцию на перехваченных сайтах создания (splice-паттерн fluid_free;
//!      паттерн перехвата создания = src/entity_index.rs), identity-карта
//!      регистрации уходит.
//!   3. EPOCH FAST-GATE: REG_EPOCH[idx] int-зеркало Snap.gen (bump secWrite —
//!      инвалидационный контракт InsideSnapOps НЕ тронут); fast-gate = один
//!      int-cmp, mismatch => stale => CHM fallback (fail-closed направление:
//!      fresh-якорь — ВСЕГДА длинное builtAtGen==gen сравнение, int-wrap не
//!      может дать wrong serve, только более медленный путь).
//!   4. FALLBACK: флет-miss => InsideSnapOps.snapGet (shipped CHM-плоскость
//!      cmp432_inside2) => ваниль level.getBlockState. Секции вне
//!      перехваченных сайтов создания, незарегистрированный idx, кап — всё
//!      продолжает сервиться через fallback (карточка: «секции вне
//!      перехваченных сайтов создания — fallback»).
//!
//! ПАРИТЕТ: HIT возвращает ТОТ ЖЕ объект BlockState, что и CHM-плоскость
//! (флет-массив держит ссылки тех же Snap). selfTest — полный при арме
//! + сэмплированный 1/100 на реальных HIT (v2; карточка: selfTest count
//! раз/100): fresh-identity, epoch-mismatch -> fallback, bounds -> fallback,
//! кап-гвард.
//!
//! FAIL-CLOSED: любой Throwable в гейте => ваниль (fail-dominant, как
//! snapGet); selfTest != true => BRIDGE_READY не публикуется (probe-then-
//! patch канон); в v1 НЕТ нативных методов (sidecar чисто java —
//! snapCollect/snapProbe остаются достоянием CHM-плоскости) и НЕТ байтовых
//! хуков: ретаргеты (сайт создания секции + wide-гейт 4B->3B) — фаза v2
//! после оффлайн-оракула.
//!
//! NCDFE КАНОН (round-3, run 35902792520, канон ×93-indy): порядок дефайнов
//! $Snap -> $Lane -> InsideSnapRegistryOps ДО первого init сайдкара
//! (newarray в <clinit> сайдкара резолвит InsideSnapOps$Snap через kernel
//! loader); в <clinit> сайдкара нет indy и нет method-ref на nested-типы.
//!
//! ARM markers (server stdout):
//!   "[crussty-plugin] cmp459_snapreg: defined InsideSnapRegistryOps in kernel loader"
//!   "[crussty-plugin] cmp459_snapreg: selfTest=true BEFORE arm"
//!   "inside_snapreg: ARMED (flat registry sidecar live; ...)"
//!   (v2, после widen: entity_compose stage marker + "first FLAT serve")
//!
//! ФЛАГ-ГЕЙТ: CRUSSTY_LEVER_FLAG STRICT eq `cmp459_snapreg` ИЛИ составной
//! climb-флаг `cmp456_chunkmono_p31snap` (TASK-460-01; пустой/чужой =
//! ваниль бит-в-байт: класс не определяется, хуков нет). Legacy env
//! CRUSSTY_SNAPREG=1 принимается для A/B-реплеев (канон inside_bitmask).
//! Прогноз (карточка ID-P32): java_util −40-60% CHM-части => +0.8-1.2пп.

use jvmti_bindings::jni;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideSnapRegistryOps";
/// NCDFE canon: nested types of the CHM plane must be DEFINED before the
/// sidecar's first init (its <clinit> newarray resolves $Snap).
const SNAP_NESTED_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps$Snap";
const LANE_NESTED_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps$Lane";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapRegistryOps.class");
const SNAP_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapOps$Snap.class");
const LANE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapOps$Lane.class");

fn enabled() -> bool {
    // ID-P32 rides STRICT eq on its OWN lever id; TASK-460-01 climb-compo
    // `cmp456_chunkmono_p31snap` accepted on the chunkmono carrier; legacy env
    // accepted for A/B replays (канон inside_bitmask::enabled).
    let lever = std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == "cmp459_snapreg" || v == "cmp456_chunkmono_p31snap"
        })
        .unwrap_or(false);
    if lever {
        return true;
    }
    std::env::var("CRUSSTY_SNAPREG")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate visibility for the entity_compose stage pipeline (v2 wide-gate will
/// compose here; v1 publishes only after the bridge is defined+armed).
pub fn enabled_pub() -> bool {
    enabled()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref to the DEFINED ops class (0 = none). REQUIRED for the probe:
/// a JNI-defined class is NOT reachable via find_class (system loader on an
/// attached native thread) nor loader.loadClass (noise_fill.rs smoke-1).
static OPS_GREF: AtomicUsize = AtomicUsize::new(0);

/// Pollable gate for the entity_compose stage pipeline.
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(Ordering::Acquire)
}

/// Register (idempotent; call once from cplugin_init). NO byte hook is
/// installed here — v2 adds the section-creation retarget (entity_index.rs
/// pattern) and the wide 4B->3B gate site through entity_compose.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_snapreg: dormant (lever_flag != cmp459_snapreg and CRUSSTY_SNAPREG unset)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_snapreg: bridge owner armed, v2 retargets (section-creation + wide gate) stay unwired in scaffold"
    );
}

/// Define one nested bridge class into the kernel loader under the captured
/// loader ref. Duplicate-definition (LinkError) = fail-closed dormant: the
/// only acceptable cause is a mixed-flag run where inside_snap.rs already
/// defined the nested class — anything else stays fail-closed.
fn define_nested(env: &JniEnv, bytes: &[u8], name: &str, loader: jni::jobject) -> bool {
    match env.define_class(name, loader, bytes) {
        Some(_) => {
            eprintln!("[crussty-plugin] cmp459_snapreg: defined {name} in kernel loader");
            true
        }
        None => {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] cmp459_snapreg: define_class({name}) failed (acceptable ONLY if already defined by inside_snap.rs)"
            );
            false
        }
    }
}

/// Probe the defined bridge: selfTest() must return true, then v1()+arm()
/// flips are applied and armState() must return "ARMED" (probe-then-patch
/// canon: a disarmed bridge must never be published).
fn probe_and_arm() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
        let gref = OPS_GREF.load(Ordering::SeqCst) as jni::jclass;
        if gref.is_null() {
            return false;
        }
        let Some(test_mid) = env.get_static_method_id(gref, "selfTest", "()I") else {
            crate::clear_exception(env);
            return false;
        };
        // sdk exposes int/void/object static calls only: selfTest returns 1/0.
        let ok = env.call_static_int_method(gref, test_mid, &[]) == 1;
        if !ok {
            crate::clear_exception(env);
            eprintln!(
                "[crussty-plugin] cmp459_snapreg: selfTest=false BEFORE arm, hook stays dormant"
            );
            return false;
        }
        eprintln!("[crussty-plugin] cmp459_snapreg: selfTest=true BEFORE arm");
        // v1() then arm() — arm is the LAST step (colpush arm-order canon).
        for (method, desc) in [("v1", "()V"), ("arm", "()V")] {
            let Some(mid) = env.get_static_method_id(gref, method, desc) else {
                crate::clear_exception(env);
                return false;
            };
            env.call_static_void_method(gref, mid, &[]);
        }
        let Some(state_mid) = env.get_static_method_id(gref, "armState", "()Ljava/lang/String;")
        else {
            crate::clear_exception(env);
            return false;
        };
        let res = env.call_static_object_method(gref, state_mid, &[]);
        if res.is_null() {
            crate::clear_exception(env);
            return false;
        }
        let state = env.get_string_utf(res as jni::jstring).unwrap_or_default();
        env.delete_local_ref(res);
        state == "ARMED"
    })
    .unwrap_or(false)
}

/// Background activation: wait for the kernel Entity class, wait out boot,
/// define $Snap -> $Lane -> InsideSnapRegistryOps (NCDFE define-order),
/// probe selfTest+ARM, publish BRIDGE_READY. v1 installs NO byte hook.
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
                    "[crussty-plugin] inside_snapreg: {ENTITY_CLASS} not loaded within 180s, bridge stays undefined"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_snapreg: forcing kernel load of {ENTITY_CLASS}"
                );
                // RC7 canon (TASK-433-B; ref a3991c2): LAZY force-load — no
                // <clinit> on the poll thread before Bootstrap.
                crate::improved_noise::force_load_kernel_class_lazy(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define (boot-storm discipline).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_snapreg: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_snapreg: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] inside_snapreg: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_459_p32_blobs.sh; hook stays dormant"
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
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);

            // NCDFE define-order (canon ×93-indy): nested types of the CHM
            // plane first, sidecar last — its first init (probe) newarrays
            // the Snap[] REG and resolves $Snap through THIS loader.
            let snap_ok = define_nested(env, SNAP_BYTES, SNAP_NESTED_CLASS, gref);
            let lane_ok = define_nested(env, LANE_BYTES, LANE_NESTED_CLASS, gref);
            if !snap_ok || !lane_ok {
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    let gr = env.new_global_ref(c);
                    OPS_GREF.store(gr as usize, Ordering::SeqCst);
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] cmp459_snapreg: defined {OPS_CLASS} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] inside_snapreg: define_class({OPS_CLASS}) failed"
                    );
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_snapreg: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // PROBE-THEN-ARM: selfTest true -> v1() -> arm() -> armState()=="ARMED".
        if !probe_and_arm() {
            eprintln!(
                "[crussty-plugin] inside_snapreg: probe/arm failed, hook stays dormant (fail-closed)"
            );
            return;
        }

        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "snapGet", "inside_snapreg v1 (ID-P32 sidecar; no retarget sites in scaffold)");
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono_p31snap: ARMED snapreg sidecar (flat registry defined+selfTest+armed; v1: no retarget sites, fail-closed CHM fallback)"
        );
        eprintln!(
            "[crussty-plugin] inside_snapreg: bridge defined+selfTest+armed, BRIDGE_READY (v2: section-creation + wide-gate retargets)"
        );
    });
}
