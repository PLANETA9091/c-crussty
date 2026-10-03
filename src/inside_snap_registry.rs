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
//!   "inside_snapreg: P36 epoch pre-gate live on the REG carrier (...)"
//!   (v2, после widen: entity_compose stage marker + "first FLAT serve")
//!
//! ФЛАГ-ГЕЙТ: CRUSSTY_LEVER_FLAG STRICT eq `cmp459_snapreg` ИЛИ составной
//! climb-флаг `cmp456_chunkmono_p31snap` (TASK-460-01; пустой/чужой =
//! ваниль бит-в-байт: класс не определяется, хуков нет). Legacy env
//! CRUSSTY_SNAPREG=1 принимается для A/B-реплеев (канон inside_bitmask).
//! Прогноз (карточка ID-P32): java_util −40-60% CHM-части => +0.8-1.2пп.
//!
//! ITER-2 WIRE (TASK-529/AG-244, ветка swarm-529-244): scaffold заведён на
//! LIVE-ПУТЬ — исполняемое ядро решения wired-гейта за STRICT-eq lever
//! (гейт-список ТОЛЬКО этого модуля; чужие STRICT-OR списки не расширены
//! — закон 4 изоляция):
//!   serve_gate = P36 PRE-GATE (флет EPOCHS[slot] long-cmp ПЕРЕД полным
//!   serve-путём; 64-бит, ABA/wrap-гвард: длинное builtAtGen==gen остаётся
//!   ВСЕГДА финальным якорем свежести — int-wrap не может дать wrong
//!   serve) -> P32 serveFlat (int-mirror epoch + long anchor + content)
//!   -> FAIL-CLOSED CHM fallback (serve_chm) -> ваниль.
//! Самтесты пинуют незыблемые контракты: hit-инвариант на 10k синтетических
//! секций (serve из REG == serve из CHM, ТОТ ЖЕ объект), miss->fallback
//! (незарегистрированный idx / stale epoch / кап), epoch-wrap (gen =
//! u64::MAX-2, +5 бампов через wrap, int-coincidence +2^32 отклоняется
//! long-якорем). Байтовые ретаргеты (idx-в-секцию splice :56-58,
//! wide-gate 4B->3B) остаются iter-3 (java сайт-спека) — здесь НЕТ.

// Decision-core surface (RegistryState/serve_gate/epoch_fresh) — compose-
// поверхность iter-3 (wide-gate/snapGet-HEAD сайт-спека); до неё часть
// pub-элементов читается только самтестами — не шумим в CI (паттерн
// соседа inside_epoch_gate.rs).
#![allow(dead_code)]

use jvmti_bindings::jni;
use jvmti_bindings::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;

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

/// STRICT lever id of THIS module (закон 4 изоляция): gate lists are
/// matched STRICT eq only — no prefix/substring, чужие STRICT-OR списки
/// не расширяются (selftest lever_gate_strict_eq pins the contract).
pub const LEVER_ID: &str = "cmp459_snapreg";

/// The gate list of THIS module only: own id first, then the carrier-
/// compo ids the scaffold already accepted (TASK-460-01 / cmp466_c98ai).
const LEVER_IDS: &[&str] = &[LEVER_ID, "cmp456_chunkmono_p31snap", "cmp466_c98ai"];

/// STRICT-eq match against LEVER_IDS (pure — testable without env races).
fn lever_id_matches(v: &str) -> bool {
    LEVER_IDS.iter().any(|id| v == *id)
}

fn enabled() -> bool {
    // ID-P32 rides STRICT eq on its OWN lever id; TASK-460-01 climb-compo
    // `cmp456_chunkmono_p31snap` accepted on the chunkmono carrier; legacy env
    // accepted for A/B replays (канон inside_bitmask::enabled).
    let lever = std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| lever_id_matches(v.trim()))
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

/// Compose-visible poll (non-blocking sibling of wait_bridge_ready): the
/// sidecar may serve a visit ONLY when the STRICT lever is up AND the
/// bridge finished probe-then-arm. Iter-3 compose stage polls this before
/// routing any site to the wired serve_gate decision.
pub fn live_gate_armed() -> bool {
    enabled() && BRIDGE_READY.load(Ordering::Acquire)
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
        "[crussty-plugin] inside_snapreg: bridge owner armed, live-path wire = serve_gate (P36 EPOCHS[slot] long-cmp pre-gate -> P32 serveFlat -> fail-closed CHM fallback); v2 byte retargets stay iter-3 (java site-spec)"
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
        eprintln!(
            "[crussty-plugin] inside_snapreg: P36 epoch pre-gate live on the REG carrier (EPOCHS[slot] long-cmp before full serve; long builtAtGen==gen = final freshness anchor)"
        );
    });
}

// ======================================================================
// LIVE-PATH WIRE (iter-2, TASK-529/AG-244) — P36 epoch pre-gate + P32
// flat-serve decision core over the REG carrier
// ----------------------------------------------------------------------
// Исполняемый контракт wired-гейта, 1:1 зеркало java-моста
// InsideSnapRegistryOps (serveFlat :216-239, secRegister :142-154,
// snapAttach :160-166, epochBump :169-173) С ПРЕПЕНДЕННЫМ P36 pre-gate
// (RESEARCH-459-P36 §1: флет EPOCHS[slot] long-cmp ПЕРЕД полным serve-
// путём; REG_EPOCH — носитель ID-P36, RESEARCH-459-P32 финал). Ядро —
// compose-поверхность iter-3 (wide 4B->3B гейт / snapGet HEAD pre-gate):
// самтесты ниже пинуют ВСЕ незыблемые контракты на 10k синтетических
// секций (hit-identity REG==CHM, fail-closed fallback, wrap-гвард).
//
// Незыблемое (нарушить = провал):
//   - тот же объект Snap: REG[idx] — зеркала ССЫЛОК CHM-плоскости
//     (sidecar никогда не минтует Snap — sec_register требует
//     materialized-секцию, attach берёт ref из collect-плана);
//   - та же инвалидация: sec_write_bump только ЧИТАЕТ след secWrite
//     (InsideSnapOps.java:505-518 НЕ тронут — gen-бамп остаётся в CHM
//     плоскости);
//   - тот же collect-план: источник истины = chm (CHM-плоскость);
//   - HIT = ТОТ ЖЕ BlockState-объект, что CHM-плоскость (token identity).
// ======================================================================

/// Same tracking budget as InsideSnapOps.CAP / java REG_CAP (1<<15).
pub const REG_CAP: usize = 1 << 15;

/// Identity token of a BlockState object — model of java ref identity
/// (HIT-контракт: «тот же объект, что CHM-плоскость/ваниль»). Same token
/// == same object; different tokens == different objects.
pub type StateToken = u64;

/// Section identity — model of the LevelChunkSection ref-identity key
/// (CHM-плоскость key / SEC2IDX identity map).
pub type SecId = u64;

/// Final serve of a visit. Both arms carry the SAME object token the
/// shipped CHM plane (then vanilla) would return — parity contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Serve {
    /// Fresh snapshot serve (flat REG[idx] on the sidecar arm).
    Snap(StateToken),
    /// Miss continuation: vanilla live-palette read
    /// (LevelChunk.getBlockStateFinal — the stale/miss continuation).
    Vanilla(StateToken),
}

impl Serve {
    #[inline]
    pub fn token(self) -> StateToken {
        match self {
            Serve::Snap(t) | Serve::Vanilla(t) => t,
        }
    }
}

/// Content mode mirror (java s.states[4096] vs s.single; publish sites
/// InsideSnapOps.java:650/:690; single = bpe==0 one-object serve).
/// Хранится АТОМАРНО внутри общего SnapMirror: CHM-плоскость публикует
/// через ТОТ ЖЕ Arc-реф, который сервит sidecar (re-mint запрещён) —
/// модель volatile-записи совместимого поля (:650/:690).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapContent {
    Full,
    Single,
}

/// SnapMirror.content u8 encoding (0 = pending — java fresh.pending, :495).
const CONTENT_PENDING: u8 = 0;
const CONTENT_FULL: u8 = 1;
const CONTENT_SINGLE: u8 = 2;

fn content_u8(c: SnapContent) -> u8 {
    match c {
        SnapContent::Full => CONTENT_FULL,
        SnapContent::Single => CONTENT_SINGLE,
    }
}

/// Mirror of `InsideSnapOps.Snap` — the SAME class/ref the CHM plane
/// materializes (never re-minted by the sidecar). Volatile-field model
/// (java volatile long == AtomicU64, Acquire-load/Release-store):
/// gen (secWrite bump :510), builtAtGen (publish :655/:691; disarm -1L
/// :696 == u64::MAX in the two's-complement mirror — unreachable: gen
/// needs 2^63 real writes per section to reach the sentinel).
pub struct SnapMirror {
    pub sec: SecId,
    pub gen: AtomicU64,
    pub built_at_gen: AtomicU64,
    /// CONTENT_PENDING/CONTENT_FULL/CONTENT_SINGLE — interior-mutable:
    /// sidecar держит ТОТ ЖЕ Arc, что и CHM-плоскость, публикацию контента
    /// делает collect-плоскость (sidecar никогда не re-mint'ит).
    pub content: AtomicU8,
}

/// Deterministic content of (section, cell) at epoch gen — the live
/// palette read AND the snapshot built at that epoch are the SAME object
/// when gen == builtAtGen (seqlock discipline: bump AFTER the write,
/// rebuild re-publishes at the new gen). Single-mode sections serve ONE
/// object for every packed cell (bpe==0).
#[inline]
fn content_token_of(sec: SecId, packed: u32, gen: u64, single: bool) -> StateToken {
    let cell = if single { 0x1000_0000u64 } else { packed as u64 };
    let mut h = sec ^ (cell << 20) ^ gen.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 33;
    h.wrapping_mul(0xFF51_AFD7_ED55_8CCD) ^ (h >> 29)
}

/// P32 clampAssign mirror (java :131-136): negative/overflow => -1
/// (cap exhaustion — fail-closed, the CHM plane owns the section).
fn clamp_assign(raw: i32) -> i32 {
    if raw < 0 || raw >= REG_CAP as i32 {
        return -1;
    }
    raw
}

/// Sidecar + CHM-plane state model. The LIVE state lives in the java
/// bridge arrays; this core is the wired decision the bridge executes
/// and iter-3's compose stage diffs against. Field mirrors:
///   REG / REG_EPOCH / REG_COUNT / SEC2IDX — InsideSnapRegistryOps.java:116-128
///   EPOCHS[slot] (P36 long mirror)        — pre-gate носитель (ID-P36)
///   SNAPS CHM                             — InsideSnapOps.java:176 (source of truth)
pub struct RegistryState {
    /// REG[idx] — mirrors of the CHM-plane Snap refs (same Arc = same object).
    pub reg: Vec<Option<Arc<SnapMirror>>>,
    /// REG_EPOCH[idx] — P32 int mirror (low 32 bits of gen).
    pub reg_epoch: Vec<i32>,
    /// EPOCHS[slot] — P36 LONG mirror (full 64-bit gen); THE pre-gate state.
    pub epochs: Vec<u64>,
    /// SEC2IDX — WRITE-path identity map (once per section lifetime; the
    /// per-visit serve path never touches it).
    pub sec2idx: HashMap<SecId, i32>,
    /// REG_COUNT — monotonic high-water (slots never reused).
    pub reg_count: i32,
    /// The CHM plane (source of truth; never evicts — sec->snap immutable).
    pub chm: HashMap<SecId, Arc<SnapMirror>>,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryState {
    pub fn new() -> Self {
        Self {
            reg: (0..REG_CAP).map(|_| None).collect(),
            reg_epoch: vec![0; REG_CAP],
            epochs: vec![0; REG_CAP],
            sec2idx: HashMap::new(),
            reg_count: 0,
            chm: HashMap::new(),
        }
    }

    /// CHM plane: the collect plan materializes a Snap (register+pending;
    /// java register :490-499, publish at collect). The sidecar NEVER
    /// mints mirrors except refs attached FROM here (same Arc — same object).
    pub fn chm_materialize(&mut self, sec: SecId) -> Arc<SnapMirror> {
        let s = Arc::new(SnapMirror {
            sec,
            gen: AtomicU64::new(0),
            built_at_gen: AtomicU64::new(0),
            content: AtomicU8::new(CONTENT_PENDING), // fresh.pending = true, :495
        });
        self.chm.insert(sec, Arc::clone(&s));
        s
    }

    /// P32 secRegister mirror (java :142-154): idempotent WRITE-path
    /// registration — one int idx per section LIFETIME (the SAME cadence
    /// InsideSnapOps.register pays at first touch; the per-visit serve
    /// path never touches SEC2IDX). Sections the collect plane never
    /// materialized stay UNREGISTERED (-1): the sidecar does not mint
    /// Snap refs (source of truth = collect plan).
    pub fn sec_register(&mut self, sec: SecId) -> i32 {
        if !self.chm.contains_key(&sec) {
            return -1; // never-minted guard (card: outside intercepted sites => fallback)
        }
        if let Some(&have) = self.sec2idx.get(&sec) {
            return have;
        }
        let raw = self.reg_count; // REG_COUNT.getAndIncrement()
        self.reg_count = self.reg_count.wrapping_add(1);
        let idx = clamp_assign(raw);
        if idx < 0 {
            self.reg_count = self.reg_count.wrapping_sub(1);
            return -1; // cap: fail-closed (fallback plane owns the section)
        }
        self.sec2idx.insert(sec, idx); // race: one winner publishes, both slots valid
        idx
    }

    /// P32 snapAttach mirror (java :160-166): publish the ref, THEN stamp
    /// the mirrors (reader order ref -> epoch, RESEARCH-459-P32 §2.3).
    /// The P36 long mirror is stamped at the SAME publish point.
    pub fn attach(&mut self, idx: i32, s: &Arc<SnapMirror>) {
        if idx < 0 || idx as usize >= REG_CAP {
            return;
        }
        let gen = s.gen.load(Ordering::Acquire);
        self.reg[idx as usize] = Some(Arc::clone(s)); // publish ref...
        self.reg_epoch[idx as usize] = gen as i32; // ...then int mirror...
        self.epochs[idx as usize] = gen; // ...then the P36 long mirror
    }

    /// secWrite invalidation, sidecar side = READ-ONLY consumer of the
    /// bump (java secWrite :505-518 NOT touched — gen++ stays the CHM
    /// plane's contract, seqlock: AFTER the write). Mirrors go stale
    /// until the rebuild restamps them (fail-closed direction only).
    pub fn sec_write_bump(&mut self, sec: SecId) {
        if let Some(s) = self.chm.get_mut(&sec) {
            s.gen.fetch_add(1, Ordering::AcqRel); // volatile gen++ (java :510)
        }
    }

    /// Collect rebuild publish (java :690-691: content -> builtAtGen,
    /// reader order) + mirror re-sync (the v2 REG_EPOCH-carrier restamp —
    /// epochBump :169-173 mirror site). Single-threaded model: the java
    /// in-flight disarm (:695-697) is unreachable by construction.
    pub fn publish_rebuild(&mut self, sec: SecId, content: SnapContent) {
        let Some(s) = self.chm.get(&sec) else { return };
        let gen = s.gen.load(Ordering::Acquire);
        let Some(s) = self.chm.get_mut(&sec) else { return };
        // java publish order :650/:690 — content THEN builtAtGen (reader
        // order: content за якорем; якорь = финальный вердикт свежести).
        s.content.store(content_u8(content), Ordering::Release);
        s.built_at_gen.store(gen, Ordering::Release); // builtAtGen = g
        if let Some(&idx) = self.sec2idx.get(&sec) {
            if idx >= 0 && (idx as usize) < REG_CAP {
                self.reg_epoch[idx as usize] = gen as i32;
                self.epochs[idx as usize] = gen;
            }
        }
    }

    /// P36 PRE-GATE — ONE long-cmp on the flat EPOCHS[slot] mirror BEFORE
    /// the full serve path (RESEARCH-459-P36 §1). FULL-WIDTH 64-bit
    /// equality: a 2^32-multiple gen drift (int-wrap coincidence) can
    /// NEVER pass; any mismatch/foreign slot => false => fail-closed
    /// (fallback), never a serve. Zero-init note: slot==gen==0 is benign —
    /// a gen-0 published snapshot needs no restamp (0 IS its stamp); an
    /// unpublished one has no content and the full path rejects it.
    #[inline]
    pub fn epoch_fresh(&self, slot: i32, gen: u64) -> bool {
        if slot < 0 || slot as usize >= REG_CAP {
            return false;
        }
        self.epochs[slot as usize] == gen
    }

    /// P32 serveFlat core (java :216-239) WITHOUT the pre-gate: bounds/
    /// ref resolution is the caller's; the int-mirror epoch gate, the
    /// content gate and the LONG anchor live here. The long
    /// builtAtGen == gen compare is ALWAYS the final freshness anchor —
    /// an int-wrap coincidence can never serve stale (P32 §3/§5).
    fn serve_flat_core(s: &SnapMirror, mirror_i32: i32, packed: u32) -> Option<StateToken> {
        let gen_at_int_gate = s.gen.load(Ordering::Acquire); // volatile read 1 (java :226)
        if mirror_i32 != gen_at_int_gate as i32 {
            return None; // int-cmp epoch gate — mismatch => stale
        }
        let c = s.content.load(Ordering::Acquire);
        if c == CONTENT_PENDING {
            return None; // pending => CHM fallback owns collect triggering (:238)
        }
        let single = c == CONTENT_SINGLE;
        let gen_at_anchor = s.gen.load(Ordering::Acquire); // volatile read 2 (java :231/:235)
        if s.built_at_gen.load(Ordering::Acquire) != gen_at_anchor {
            return None; // LONG ANCHOR — final freshness verdict
        }
        Some(content_token_of(s.sec, packed, gen_at_anchor, single))
    }

    /// THE LIVE-PATH GATE — the wired decision: P36 pre-gate PREPENDED to
    /// the P32 flat serve, fail-closed CHM fallback inlined exactly like
    /// the java bridge (:194-213). Returns (final serve, flat_hit):
    /// flat_hit == true means REG[idx] served the visit (the cheap path);
    /// false means the shipped CHM plane produced the answer (bit-exact
    /// continuation). The fallback arm NEVER changes the answer — only
    /// the path (P32 §3: miss/стейл/вне-реестра/кап => fallback).
    pub fn serve_gate(&self, idx: i32, sec: SecId, packed: u32) -> (Serve, bool) {
        // ---- 1. P36 PRE-GATE: EPOCHS[slot] long-cmp BEFORE full serve ----
        if idx < 0 || idx as usize >= REG_CAP {
            return (self.serve_chm(sec, packed), false); // unregistered bounds
        }
        let Some(s) = self.reg[idx as usize].as_ref() else {
            return (self.serve_chm(sec, packed), false); // unbound slot
        };
        let gen = s.gen.load(Ordering::Acquire); // volatile epoch of the section
        if !self.epoch_fresh(idx, gen) {
            return (self.serve_chm(sec, packed), false); // stale/wrap: full path SKIPPED
        }
        // ---- 2. FULL SERVE PATH (P32 serveFlat; java :216-239) ----
        match Self::serve_flat_core(s, self.reg_epoch[idx as usize], packed) {
            Some(t) => (Serve::Snap(t), true),
            None => (self.serve_chm(sec, packed), false), // fail-closed fallback (:207)
        }
    }

    /// CHM-plane serve mirror (InsideSnapOps.serve/serve4 essence — the
    /// fail-closed fallback plane; its own miss continues vanilla).
    pub fn serve_chm(&self, sec: SecId, packed: u32) -> Serve {
        let Some(s) = self.chm.get(&sec) else {
            // Untracked section: register+collect side effects elided in the
            // model; the visit continues vanilla (snapGet miss path :212).
            return Serve::Vanilla(content_token_of(sec, packed, u64::MAX, false));
        };
        let gen = s.gen.load(Ordering::Acquire);
        let built = s.built_at_gen.load(Ordering::Acquire);
        let c = s.content.load(Ordering::Acquire);
        let single = c == CONTENT_SINGLE;
        match c {
            CONTENT_FULL | CONTENT_SINGLE if built == gen => {
                Serve::Snap(content_token_of(s.sec, packed, built, single))
            }
            // stale-miss continuation: live palette read (java :350/:463/:477)
            _ => Serve::Vanilla(content_token_of(s.sec, packed, gen, single)),
        }
    }
}

// ======================================================================
// SELFTESTS (iter-2; java bridge selfTest covers the JVM side, these pin
// the wired decision core contracts rust-side — same ids, same verdicts)
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// STRICT gate of THIS module: own id passes; carrier-compo ids the
    /// scaffold already accepted pass; EVERYTHING else (empty, prefixes,
    /// near-misses, чужие lever ids) fails — STRICT eq, no substring.
    #[test]
    fn lever_gate_strict_eq() {
        assert!(lever_id_matches("cmp459_snapreg"));
        assert!(lever_id_matches("cmp456_chunkmono_p31snap"));
        assert!(lever_id_matches("cmp466_c98ai"));
        for foreign in [
            "",
            "cmp459",
            "cmp459_snapregX",
            "cmp459_snapreg_",
            "cmp432_inside2",
            "cmp436_ins4",
            "cmp456_chunkmono",
            "CMP459_SNAPREG", // case-sensitive STRICT eq
        ] {
            assert!(!lever_id_matches(foreign), "foreign lever must fail: {foreign:?}");
        }
    }

    /// HIT-ИНВАРИАНТ (контракт iter-2): serve из REG == serve из CHM на
    /// 10k синтетических секций — each fresh section flat-serves on ALL
    /// 4096 packed cells and the answer is the SAME BlockState object the
    /// CHM plane would serve (token identity), with the SAME Snap ref
    /// mirrored (Arc::ptr_eq — the sidecar never re-mints).
    #[test]
    fn hit_invariant_10k_sections_serve_reg_eq_chm() {
        const N: usize = 10_000;
        let mut st = RegistryState::new();
        let mut idxs = Vec::with_capacity(N);
        for i in 0..N {
            let sec = 1_000_000u64 + i as u64;
            let s = st.chm_materialize(sec);
            let idx = st.sec_register(sec);
            assert!(idx >= 0, "registration must succeed below the cap");
            st.attach(idx, &s);
            st.publish_rebuild(sec, SnapContent::Full);
            idxs.push((sec, idx));
        }
        let mut flat_hits = 0u64;
        for (sec, idx) in &idxs {
            for packed in 0..4096u32 {
                let (gate, flat) = st.serve_gate(*idx, *sec, packed);
                assert!(flat, "fresh section must flat-serve (idx {idx} sec {sec})");
                assert_eq!(
                    gate,
                    st.serve_chm(*sec, packed),
                    "HIT from REG must be the SAME BlockState object the CHM plane serves"
                );
                flat_hits += 1;
            }
            assert!(Arc::ptr_eq(
                st.reg[*idx as usize].as_ref().unwrap(),
                &st.chm[sec]
            ));
        }
        assert_eq!(flat_hits, (N * 4096) as u64);
    }

    /// Тот же объект Snap + single-mode identity: REG mirrors the CHM
    /// plane refs; a single-mode (bpe==0) section serves ONE object for
    /// every packed cell on BOTH planes.
    #[test]
    fn same_object_contract_reg_mirrors_chm_refs() {
        let mut st = RegistryState::new();
        let sec = 0x5ECu64;
        let s = st.chm_materialize(sec);
        let idx = st.sec_register(sec);
        assert!(idx >= 0);
        st.attach(idx, &s);
        st.publish_rebuild(sec, SnapContent::Single);
        assert!(Arc::ptr_eq(&st.chm[&sec], &st.reg[idx as usize].as_ref().unwrap()));
        let (gate, flat) = st.serve_gate(idx, sec, 777);
        assert!(flat);
        assert_eq!(gate, st.serve_chm(sec, 777));
        assert_eq!(gate, st.serve_chm(sec, 0)); // one object everywhere
        assert_eq!(gate.token(), content_token_of(sec, 777, s.gen.load(Ordering::Acquire), true));
    }

    /// MISS->FALLBACK: (a) секция, которую collect-план не материализовал,
    /// никогда не регистрируется (sidecar не минтует Snap); (b)
    /// незарегистрированный/отрицательный/за-капом idx; (c) stale epoch
    /// (bump без restamp); (d) кап исчерпан => -1 => fallback-плоскость
    /// продолжает обслуживать. Везде: gate-ответ == ответ CHM-плоскости,
    /// flat_hit == false (fail-closed направление).
    #[test]
    fn miss_fallback_unregistered_stale_and_cap() {
        let mut st = RegistryState::new();

        // (a) never-minted guard
        let never = 0xDEADu64;
        assert_eq!(st.sec_register(never), -1);
        // (b) unregistered / negative / beyond-cap idx: bounds + unbound slot
        for idx in [-1i32, 0, REG_CAP as i32, REG_CAP as i32 + 7] {
            let (serve, flat) = st.serve_gate(idx, never, 5);
            assert!(!flat, "idx {idx} must fail closed");
            assert_eq!(serve, st.serve_chm(never, 5));
        }
        // (c) stale epoch: bump without restamp => pre-gate mismatch =>
        // full serve path SKIPPED, fallback == CHM plane answer
        let sec = 9u64;
        let s = st.chm_materialize(sec);
        let idx = st.sec_register(sec);
        assert!(idx >= 0);
        st.attach(idx, &s);
        st.publish_rebuild(sec, SnapContent::Full);
        assert!(st.serve_gate(idx, sec, 100).1, "fresh must flat-serve");
        st.sec_write_bump(sec); // secWrite contract: gen++ (java :510)
        assert!(!st.epoch_fresh(idx, st.chm[&sec].gen.load(Ordering::Acquire)));
        let (serve, flat) = st.serve_gate(idx, sec, 100);
        assert!(!flat);
        assert_eq!(serve, st.serve_chm(sec, 100)); // stale-miss continuation
        // (d) cap exhaustion => -1 => fallback plane owns the section
        st.reg_count = REG_CAP as i32;
        let capped = 11u64;
        st.chm_materialize(capped);
        assert_eq!(st.sec_register(capped), -1);
        let (serve_cap, flat_cap) = st.serve_gate(REG_CAP as i32 - 1, capped, 0);
        assert!(!flat_cap);
        assert_eq!(serve_cap, st.serve_chm(capped, 0));
        // clampAssign contract (java :131-136)
        assert_eq!(clamp_assign(-3), -1);
        assert_eq!(clamp_assign(REG_CAP as i32), -1);
        assert_eq!(clamp_assign(REG_CAP as i32 - 1), REG_CAP as i32 - 1);
        assert_eq!(clamp_assign(0), 0);
    }

    /// SITE-1 MIRROR (iter-3 java pre-gate in InsideSnapOps.snapGet): the
    /// per-thread warm slot IS the EPOCHS[slot] carrier — stamp = the attach/
    /// publish restamp point, pre-gate hit == the flat-serve verdict of the
    /// wired decision core, slot-stale (bump without restamp) == epoch_fresh
    /// false => full serve path. Pins the java pregEpoch long-cmp semantics
    /// (64-bit mirror + long builtAtGen==gen anchor) against serve_gate:
    /// miss/stale direction is fail-closed ONLY (never a wrong serve).
    #[test]
    fn pregate_site1_slot_epoch_mirror() {
        let mut st = RegistryState::new();
        let sec = 0xB1D5u64;
        let s = st.chm_materialize(sec);
        let idx = st.sec_register(sec);
        assert!(idx >= 0);
        st.attach(idx, &s); // slot stamp analog: publish ref, then epoch mirrors
        st.publish_rebuild(sec, SnapContent::Full);
        let gen = st.chm[&sec].gen.load(Ordering::Acquire);
        // warm slot: pregEpoch == gen => the ONE long-cmp passes == epoch_fresh
        assert!(st.epoch_fresh(idx, gen));
        for packed in [0u32, 1, 4095] {
            let (gate, flat) = st.serve_gate(idx, sec, packed);
            assert!(flat, "warm slot must flat-serve on every packed cell");
            assert_eq!(
                gate,
                st.serve_chm(sec, packed),
                "slot-serve must be the SAME BlockState object the CHM plane serves"
            );
        }
        // slot stale: secWrite bump without restamp => pre-gate misses =>
        // full serve path (java: pregate() => null => serve/serve4 body).
        st.sec_write_bump(sec);
        assert!(!st.epoch_fresh(idx, st.chm[&sec].gen.load(Ordering::Acquire)));
        for packed in [0u32, 1, 4095] {
            let (gate, flat) = st.serve_gate(idx, sec, packed);
            assert!(!flat, "stale slot must fall back to the full serve path");
            assert_eq!(gate, st.serve_chm(sec, packed));
        }
        // re-warm (full-path fresh serve restamps the slot): flat again
        st.publish_rebuild(sec, SnapContent::Full);
        let (gate, flat) = st.serve_gate(idx, sec, 3);
        assert!(flat);
        assert_eq!(gate, st.serve_chm(sec, 3));
    }

    /// EPOCH-WRAP ГВАРД (контракт P36): gen форсится к u64::MAX-2, +5
    /// бампов ПЕРЕСЕКАЮТ wrap (MAX-1, MAX, 0, 1, 2) — fresh-выводы
    /// корректны после каждого restamp; между restamp'ами gate fail-closed;
    /// int-wrap coincidence (gen += 2^32 — int-зеркало СОВПАДАЕТ) отбрасы-
    /// вается и P36 long-cmp, и длинным builtAtGen==gen якорем — wrong
    /// serve невозможен, только более медленный путь.
    #[test]
    fn epoch_wrap_u64_max_minus_2_plus5_bumps() {
        let mut st = RegistryState::new();
        let sec = 0xA11CEu64;
        let s = st.chm_materialize(sec);
        let idx = st.sec_register(sec);
        assert!(idx >= 0);
        st.attach(idx, &s);
        // force gen near the 64-bit wrap (java long: -3 two's-complement)
        st.chm.get_mut(&sec).unwrap().gen.store(u64::MAX - 2, Ordering::Release);
        st.publish_rebuild(sec, SnapContent::Full);
        for packed in [0u32, 1, 0xFFF, 4095] {
            let (gate, flat) = st.serve_gate(idx, sec, packed);
            assert!(flat);
            assert_eq!(gate, st.serve_chm(sec, packed));
            assert_eq!(gate.token(), content_token_of(sec, packed, u64::MAX - 2, false));
        }

        // (a) +5 bumps across the wrap, restamp each: fresh verdicts correct
        let mut g = u64::MAX - 2;
        for _ in 0..5 {
            st.sec_write_bump(sec);
            g = g.wrapping_add(1);
            assert_eq!(st.chm[&sec].gen.load(Ordering::Acquire), g);
            st.publish_rebuild(sec, SnapContent::Full);
            assert!(st.epoch_fresh(idx, g));
            let (gate, flat) = st.serve_gate(idx, sec, 0);
            assert!(flat, "restamped gen must serve fresh across the wrap");
            assert_eq!(gate.token(), content_token_of(sec, 0, g, false));
        }
        assert_eq!(g, 2, "MAX-2 +5 = wrap past MAX down to 2");
        assert_eq!(st.chm[&sec].gen.load(Ordering::Acquire), 2);

        // (b) bump WITHOUT restamp: stale => fail-closed == CHM answer
        st.sec_write_bump(sec);
        let (serve, flat) = st.serve_gate(idx, sec, 0);
        assert!(!flat);
        assert_eq!(serve, st.serve_chm(sec, 0));

        // (c) int-wrap COINCIDENCE: gen drifted by +2^32 from builtAtGen —
        // the int mirror matches (low 32 bits equal) yet BOTH the P36
        // long-cmp and the long anchor reject: no wrong serve, ever.
        let built = st.chm[&sec].built_at_gen.load(Ordering::Acquire);
        let forced = built.wrapping_add(1u64 << 32);
        st.chm.get_mut(&sec).unwrap().gen.store(forced, Ordering::Release);
        assert_eq!(
            st.reg_epoch[idx as usize],
            forced as i32,
            "int mirror MUST coincide on a 2^32-multiple drift"
        );
        assert!(
            !st.epoch_fresh(idx, forced),
            "P36 long-cmp must reject a 2^32-multiple gen drift"
        );
        assert_eq!(
            RegistryState::serve_flat_core(&st.chm[&sec], st.reg_epoch[idx as usize], 0),
            None,
            "long anchor must reject the int-coincident stale gen"
        );
        let (serve_c, flat_c) = st.serve_gate(idx, sec, 0);
        assert!(!flat_c);
        assert_eq!(serve_c, st.serve_chm(sec, 0));

        // (d) full-serve path alone (pre-gate bypassed) still cannot serve
        // stale when the int mirror matches but the long anchor fails —
        // the anchor is ALWAYS the final freshness verdict.
        st.epochs[idx as usize] = forced; // pre-gate forced to match too
        let (serve_d, flat_d) = st.serve_gate(idx, sec, 0);
        assert!(!flat_d, "long anchor closes the gate even past both mirrors");
        assert_eq!(serve_d, st.serve_chm(sec, 0));
    }
}
