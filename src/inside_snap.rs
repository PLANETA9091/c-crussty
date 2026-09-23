//! INSIDE-SNAP (TASK-424-B origin; round-430 carrier; TASK-432-B deepening — lever cmp432_inside2 STRICT-OR cmp430_inside, закон 6 v17).
//!
//! ПОДСИСТЕМА: per-section BlockState[4096] снапшоты для inside-лейна
//! (PROFILE-B: адресуемый срез = volatile-чтения PalettedContainer ≈1.3-1.6%
//! total CPU: PalettedContainer.get 0.487 + SimpleBitStorage.get 0.126 +
//! readPalette 0.099 + LevelChunk.getBlockStateFinal 0.498 + части
//! LevelChunk.getBlockState/Level.getBlockState self).
//!
//! Состав подсистемы (закон 6 — НЕ одиночная функция):
//!   1. СНАПШОТ-ПЛОСКОСТЬ (java-мост InsideSnapOps): CHM<LevelChunkSection, Snap>,
//!      HIT-сервинг = palette[(y&15)<<8|(z&15)<<4|(x&15)] — ТОТ ЖЕ объект, что
//!      возвращает ванильный readPalette (FastPaletteData.moonrise$getPalette()).
//!   2. ИНВАЛИДАЦИЯ (event-driven): ретаргет ЕДИНСТВЕННОГО сайта
//!      LevelChunkSection.setBlockState в LevelChunk.setBlockState ->
//!      InsideSnapOps.secWrite (FluidPushOps.secWrite-паттерн): делегат + bump
//!      ген-эпохи секции на реальном изменении (ref-compare old != newState).
//!   3. ГЕЙТ: ретаргет ЕДИНСТВЕННОГО сайта Level.getBlockState в
//!      Entity.lambda$checkInsideBlocks$2 -> InsideSnapOps.snapGet
//!      (receiver-first 3B->3B, длина сохранена); miss-путь = ванильный метод;
//!      композирует через entity_compose stage 1c (S7-162: ОДИН Entity-хук).
//!   4. RUST-ЯДРО: ОДИН bulk-JNI на сбор (flat words/bpe dirty-секций ->
//!      бит-в-байт декод -> flat индексы -> java-скэттер в BlockState[4096]).
//!      Инкрементально: steady-state ≈ 0 сборов (PENDING-триггер).
//!
//! BIT-EXACT КОНТРАКТ ДЕКОДА (javap SimpleBitStorage @4789ca7): этот кернел
//! хранит MODULO-лейаут (magic/mulBits из BETTER_MAGIC, ветки straddle НЕТ):
//!   word = i / valuesPerLong;  shift = (i % valuesPerLong) * bits;
//!   value = (data[word] >>> shift) & mask;  valuesPerLong = 64 / bits;
//!   data.length = (size + valuesPerLong - 1) / valuesPerLong.
//! Это ДРУГОЙ формат, чем continuous-формула palette_gather (offset = i*bpe со
//! straddle-веткой) — palette_gather этим модулем НЕ используется (его тесты
//! фиксируют контракт другого формата; чужой код не трогаем). Property-тесты
//! ниже сверяют декод с независимым oracle-ом этой формулы.
//!
//! FAIL-CLOSED: ERR_STRUCT из натива -> java DISARM (ARMED=false) навсегда;
//! LevelChunk-патч не лёг (NotFound/Err) -> DISARM (снапшоты без инвалидации =
//! навсегда-стейл = запрещено); любой Throwable в гейте -> ваниль; капы памяти
//! (CAP/FULL_CAP) -> miss -> ваниль. ФЛАГ-ГЕЙТ: CRUSSTY_LEVER_FLAG STRICT eq
//! cmp430_inside (пустой = ваниль бит-в-байт: бридж не определяется, хуки не
//! регистрируются).
//!
//! ARM markers (server stdout; порядок = stale-window контракт):
//!   "[crussty-plugin] cmp432_inside2: defined InsideSnapOps in kernel loader"
//!   "[crussty-plugin] cmp432_inside2: selfTest=true BEFORE arm"
//!   "[crussty-plugin] cmp432_inside2: PATCHED LevelChunk.setBlockState (Retargeted { sites: 1 })"
//!   "[crussty-plugin] cmp432_inside2: ARMED (snapshot gate + secWrite invalidation live BEFORE gate)"
//!   entity_compose: "stage inside_snap composed (Retargeted { sites: 1 })"
//!   java: "inside_snap: ARMED ..." + "inside_snap: first gate HIT served"

use crate::classfile;
use jvmti_bindings::jni;
use std::ffi::CString;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

const TARGET_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunk";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps";
const SNAP_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps$Snap";
const LANE_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps$Lane";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapOps.class");
const SNAP_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapOps$Snap.class");
// ROUND-3 NCDFE FIX (run 35902792520 root-cause): the inside2 serve-fastpath
// added the nested class `InsideSnapOps$Lane` but the bridge-define step never
// defined it into the kernel loader — clinit (or first resolution) of
// InsideSnapOps hit NoClassDefFoundError: [Lnet/.../InsideSnapOps$Lane; ->
// ExceptionInInitializerError -> the class stayed erroneous FOREVER -> every
// composed Entity gate call threw NCDFE (200k storm, fail-closed empty world).
// $Lane must be defined alongside $Snap (blob installed by
// build_432b_blobs.sh, define_class here, java side also de-indy'd: no
// Lane-typed resolution is reachable from <clinit> any more).
const LANE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideSnapOps$Lane.class");

const PROBE_MAGIC: i32 = 0x42534E50; // "BSNP"
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

static READY: AtomicBool = AtomicBool::new(false);

/// entity_compose stage polling: bridge classes DEFINED (stage waits on this).
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

/// Poison recovery (TASK-46 discipline): locks wrap plain stores only.
struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
}
static TARGET: OnceLock<Target> = OnceLock::new();
fn target() -> &'static Target {
    TARGET.get_or_init(|| Target {
        orig: std::sync::Mutex::new(None),
        patch: std::sync::Mutex::new(None),
    })
}

pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled_pub() {
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

fn lever_flag_matches() -> bool {
    // TASK-432-B STRICT-OR: глубокая внутри-плоскость round-432 (cmp432_inside2)
    // ИЛИ несущий round-430 (cmp430_inside, A/B ре-плей); пустой/чужой = ваниль
    // бит-в-байт. TASK-436-B: serve-plane closure round (cmp436_ins4) rides
    // STRICT-OR поверх cmp432_inside2.
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == "cmp432_inside2" || v == "cmp430_inside" || v == "cmp436_ins4"
        })
        .unwrap_or(false)
}

/// TASK-436-B: cmp436_ins4 selects the V4 serve body (per-claim lane snap
/// arrays + untracked-miss closure + cached minSecY + lane hint). The V2
/// serve() stays byte-for-byte as the control path (V4=false default).
fn v4_requested() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp436_ins4")
        .unwrap_or(false)
}

/// entity_compose stage gate (pub).
pub fn enabled_pub() -> bool {
    lever_flag_matches()
}

/// Register (idempotent; call once from cplugin_init BEFORE entity_compose):
/// the LevelChunk byte hook (stash-serve — fluid_dirty pattern; fluid_dirty
/// owns the same class only when ITS lever is on, disjoint from mine: STRICT
/// flag eq). The Entity gate retarget composes through entity_compose stage 1c
/// (single Entity hook discipline, S7-162 leg #5 lesson).
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp432_inside2: dormant (lever_flag not in {{cmp432_inside2/cmp430_inside}}, vanilla inside lane)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Acquire) {
            let mut orig = t.orig.lock().unwrap_or_else(PoisonError::into_inner);
            if orig.is_none() {
                eprintln!(
                    "[crussty-plugin] cmp432_inside2: pristine sighting {TARGET_CLASS} {} bytes (major {})",
                    bytes.len(),
                    crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
                );
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        let cached = t.patch.lock().unwrap_or_else(PoisonError::into_inner).clone();
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait LevelChunk + boot-quiet -> class-version gate ->
/// resolution closure -> define bridge + RegisterNatives -> selfTest (kept ref)
/// -> java arm() -> compute LevelChunk patch (pristine stash) -> READY ->
/// retransform LevelChunk (secWrite site live). The Entity gate retarget is
/// applied by the entity_compose stage on ITS retransform.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        // LevelChunk loads with the first chunk (boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(TARGET_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp432_inside2: {TARGET_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] cmp432_inside2: forcing kernel load of {TARGET_CLASS}"
                );
                // RC7 canon (TASK-433-B; ref a3991c2): LAZY force-load
                // (initialize=false). The initializing variant fired pre-
                // Bootstrap on the poll thread and poisoned BuiltInRegistries
                // (inside2 run 35894909390: NCDFE=473412, selfTest resolution
                // failed, empty world). Lazy define still lands pristine bytes
                // for the transform engine; <clinit> stays with the main
                // thread's post-bootStrap first use.
                crate::improved_noise::force_load_kernel_class_lazy(TARGET_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(TARGET_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp432_inside2: boot marker not seen, hook stays dormant");
            return;
        }

        // Class-version gate (lesson ×93: stale/miscompiled blob must not arm).
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [
            (OPS_CLASS, OPS_BYTES),
            (SNAP_CLASS, SNAP_BYTES),
            (LANE_CLASS, LANE_BYTES),
        ] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] cmp432_inside2: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ blobs; hook stays dormant"
                );
                return;
            }
        }

        // Resolution closure: the bridge must declare every static the two
        // retargets + the native table resolve (structural, no JNI exec).
        if let Err(e) = classfile::inside_snap_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp432_inside2: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define all three classes into the KERNEL loader + RegisterNatives.
        let Some(gops) = define_bridge() else {
            eprintln!(
                "[crussty-plugin] cmp432_inside2: bridge definition failed, hook stays dormant"
            );
            return;
        };
        eprintln!(
            "[crussty-plugin] cmp432_inside2: defined {OPS_CLASS} in kernel loader (+ Snap + Lane), natives registered"
        );

        // TASK-436-B: flip the V4 serve body BEFORE selfTest/arm — fail-closed:
        // any resolution/invocation failure leaves the bridge dormant (never a
        // half-flipped serve path). V2 stays byte-for-byte under cmp432_inside2.
        if v4_requested() {
            let flipped = cplug_sdk::jni_util::with_attached(|env| call_v4(env, gops))
                .unwrap_or(false);
            if !flipped {
                eprintln!(
                    "[crussty-plugin] cmp436_ins4: v4() flip FAILED — hook stays dormant (fail-closed)"
                );
                return;
            }
            eprintln!("[crussty-plugin] cmp436_ins4: V4 serve body FLIPPED (per-claim snap arrays + untracked-miss closure + minSecY cache + lane hint)");
        }

        // selfTest on the KEPT define_class ref (TASK-417-C find_class fix):
        // any Throwable => fail-closed dormant (never arm).
        let selftest = cplug_sdk::jni_util::with_attached(|env| selftest(env, gops))
            .unwrap_or(false);
        if !selftest {
            eprintln!(
                "[crussty-plugin] cmp432_inside2: selfTest FAILED — hook stays dormant (fail-closed)"
            );
            return;
        }
        // ROUND-3 CONTAINMENT (run 35902792520 second lesson): BRIDGE_READY is
        // published ONLY after a GREEN selfTest. It used to be published right
        // after define_bridge — so entity_compose composed the inside_snap
        // Entity stage (retarget lambda$checkInsideBlocks$2 -> InsideSnapOps)
        // even though the class was already erroneous from the failed clinit:
        // the storm. With this ordering a failed selfTest leaves the stage
        // uncomposed (fail-dominant skip, vanilla inside_snap lane) instead of
        // arming a poisoned retarget.
        BRIDGE_READY.store(true, Ordering::Release);
        eprintln!("[crussty-plugin] cmp432_inside2: selfTest=true BEFORE arm (probe + bpe4/bpe15 modulo-layout roundtrips)");

        // ARM-ORDER (TASK-424-B stale-window fix): the java gate is armed LAST —
        // AFTER the secWrite invalidation retransform is LIVE. Arming first would
        // open an [ARMED .. LevelChunk-retransform) window where a section could
        // be published and then written WITHOUT a gen bump (patch not live yet)
        // = forever-stale serve. With this order no snapshot can exist before
        // invalidation is live (serve() registers only after ARMED, so SNAPS is
        // empty while the gate is still vanilla).
        // LevelChunk patch from the pristine stash (secWrite invalidation site)
        // and the single retransform. Entity gate composes via entity_compose.
        let Some(orig) = target()
            .orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
        else {
            eprintln!(
                "[crussty-plugin] cmp432_inside2: no pristine bytes for {TARGET_CLASS} — DISARM (fail-closed: snapshots without invalidation go permanently stale)"
            );
            cplug_sdk::jni_util::with_attached(|env| call_disarm(env, gops));
            return;
        };
        match classfile::patch_inside_snap_levelchunk(&orig) {
            Ok((patched, outcome)) => match outcome {
                classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    *target()
                        .patch
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner) = Some(Arc::from(patched.as_slice()));
                    READY.store(true, Ordering::Release);
                    let rc = cplug_sdk::retransform_class(TARGET_CLASS);
                    eprintln!(
                        "[crussty-plugin] cmp432_inside2: PATCHED {TARGET_CLASS}.setBlockState -> InsideSnapOps.secWrite (Retargeted {{ sites: {sites} }}; {} -> {} bytes; retransform rc={rc})",
                        orig.len(),
                        patched.len()
                    );
                }
                other => {
                    eprintln!(
                        "[crussty-plugin] cmp432_inside2: unexpected LevelChunk outcome ({other:?}) — DISARM (fail-closed: snapshots without invalidation go permanently stale)"
                    );
                    cplug_sdk::jni_util::with_attached(|env| call_disarm(env, gops));
                    return;
                }
            },
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] cmp432_inside2: LevelChunk patch rejected ({e}) — DISARM (fail-closed)"
                );
                cplug_sdk::jni_util::with_attached(|env| call_disarm(env, gops));
                return;
            }
        }

        // Invalidation is LIVE (retransform returned). NOW flip the gate: the
        // entity_compose stage may have already retargeted the Entity lambda —
        // snapGet sees ARMED=false and falls through to vanilla until here.
        let armed = cplug_sdk::jni_util::with_attached(|env| call_arm(env, gops)).unwrap_or(false);
        if !armed {
            eprintln!(
                "[crussty-plugin] cmp432_inside2: java arm() failed — gate stays vanilla (fail-closed)"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp432_inside2: ARMED (snapshot gate + secWrite invalidation live BEFORE gate; steady-state ≈0 collects)"
        );
    });
}

fn define_bridge() -> Option<*mut c_void> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
            return None;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return None;
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
            return None;
        };
        let gref = env.new_global_ref(loader);
        if gref.is_null() {
            crate::describe_exception(env);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] cmp432_inside2: define_class({OPS_CLASS}) failed");
            return None;
        };
        let names = [
            CString::new("snapProbe").expect("no NUL"),
            CString::new("snapCollect").expect("no NUL"),
        ];
        let sigs = [
            CString::new("()I").expect("no NUL"),
            CString::new("(I[I[J[I)I").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: snap_probe as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: snap_collect as *const c_void as *mut c_void,
            },
        ];
        let reg = env.register_natives(c, &natives);
        if let Err(code) = reg {
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] cmp432_inside2: register_natives failed (code {code}) — hook stays dormant"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }
        // Snap companion (no natives).
        if let Some(s) = env.define_class(SNAP_CLASS, gref, SNAP_BYTES) {
            env.delete_local_ref(s);
        } else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] cmp432_inside2: define_class({SNAP_CLASS}) failed");
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }
        // Lane companion (round-3 NCDFE fix, no natives): the serve-fastpath
        // lanes live in this nested class; it MUST be loadable from the same
        // kernel loader as InsideSnapOps or the very first Lane[] resolution
        // poisons InsideSnapOps permanently (run 35902792520).
        if let Some(l) = env.define_class(LANE_CLASS, gref, LANE_BYTES) {
            env.delete_local_ref(l);
        } else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] cmp432_inside2: define_class({LANE_CLASS}) failed");
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }
        let gops = env.new_global_ref(c);
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        if gops.is_null() {
            crate::describe_exception(env);
            return None;
        }
        Some(gops as *mut c_void)
    })
    .flatten()
}

fn selftest(env: &jvmti_bindings::env::JniEnv, gops: *mut c_void) -> bool {
    let cls = gops as jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] cmp432_inside2: selfTest resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!("[crussty-plugin] cmp432_inside2: selfTest threw — fail-closed");
        return false;
    }
    rc != 0
}

fn call_arm(env: &jvmti_bindings::env::JniEnv, gops: *mut c_void) -> bool {
    let cls = gops as jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "arm", "()V") else {
        crate::clear_exception(env);
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    !had_exc
}

/// TASK-436-B: flip the java V4 serve path (cmp436_ins4 only, pre-selfTest).
fn call_v4(env: &jvmti_bindings::env::JniEnv, gops: *mut c_void) -> bool {
    let cls = gops as jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "v4", "()V") else {
        crate::clear_exception(env);
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    !had_exc
}

fn call_disarm(env: &jvmti_bindings::env::JniEnv, gops: *mut c_void) {
    let cls = gops as jni::jclass;
    if let Some(f) = env.get_static_field_id(cls, "ARMED", "Z") {
        let raw = env.raw();
        unsafe {
            let ft = &**raw;
            (ft.SetStaticBooleanField)(raw as *mut jni::JNIEnv, cls, f, 0);
        }
    } else {
        crate::clear_exception(env);
    }
}

// ---------------------------------------------------------------------------
// NATIVES (raw fn-table discipline — colpush precedent)
// ---------------------------------------------------------------------------

/// # Safety
/// Called by the JVM through RegisterNatives.
#[no_mangle]
pub unsafe extern "system" fn snap_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    PROBE_MAGIC
}

struct CollectScratch {
    meta: Vec<i32>,
    words: Vec<i64>,
    out: Vec<i32>,
}
static SCRATCH: OnceLock<Mutex<CollectScratch>> = OnceLock::new();
fn scratch() -> &'static Mutex<CollectScratch> {
    SCRATCH.get_or_init(|| {
        Mutex::new(CollectScratch {
            meta: vec![0i32; 3 * 64],
            words: vec![0i64; 1 << 16],
            out: vec![0i32; 4096 * 64],
        })
    })
}

/// ОДИН bulk-JNI на сбор: meta[k*3]={bpe, wordsOff, wordsLen}; words flat;
/// out[k*4096+i] = palette index (kernel MODULO layout, bit-exact).
/// Возвращает nsec; ERR_STRUCT = java дизармит навсегда; ERR_RANGE = out мал.
///
/// # Safety
/// Called by the JVM through RegisterNatives; массивы живы на время вызова.
#[no_mangle]
pub unsafe extern "system" fn snap_collect(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    nsec: jni::jint,
    jmeta: jni::jintArray,
    jwords: jni::jlongArray,
    jout: jni::jintArray,
) -> jni::jint {
    if env.is_null() || nsec <= 0 || nsec > 4096 {
        return ERR_STRUCT;
    }
    if jmeta.is_null() || jwords.is_null() || jout.is_null() {
        return ERR_STRUCT;
    }
    let vt = &**env;
    let meta_cap = (vt.GetArrayLength)(env, jmeta as jni::jarray) as usize;
    let words_cap = (vt.GetArrayLength)(env, jwords as jni::jarray) as usize;
    let out_cap = (vt.GetArrayLength)(env, jout as jni::jarray) as usize;
    let need_meta = (nsec as usize) * 3;
    if meta_cap < need_meta {
        return ERR_STRUCT;
    }
    if out_cap < (nsec as usize) * 4096 {
        return ERR_RANGE;
    }
    let mut st = scratch()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if st.meta.len() < need_meta {
        st.meta.resize(need_meta, 0);
    }
    (vt.GetIntArrayRegion)(env, jmeta, 0, need_meta as jni::jsize, st.meta.as_mut_ptr());
    if (words_cap) > st.words.len() {
        st.words.resize(words_cap, 0);
    }
    (vt.GetLongArrayRegion)(env, jwords, 0, words_cap as jni::jsize, st.words.as_mut_ptr());
    // validate meta shapes + total words
    let mut total_words = 0usize;
    for k in 0..nsec as usize {
        let bpe = st.meta[k * 3] as u32;
        let off = st.meta[k * 3 + 1] as usize;
        let len = st.meta[k * 3 + 2] as usize;
        let vpl = if bpe == 0 { 0usize } else { (64 / bpe) as usize };
        let need = if bpe == 0 {
            0
        } else {
            (4096usize + vpl - 1) / vpl
        };
        if len < need || off.saturating_add(len) > words_cap {
            return ERR_STRUCT;
        }
        total_words = total_words.saturating_add(off).saturating_add(len);
    }
    if st.out.len() < (nsec as usize) * 4096 {
        st.out.resize((nsec as usize) * 4096, 0);
    }
    for k in 0..nsec as usize {
        let bpe = st.meta[k * 3] as u32;
        let off = st.meta[k * 3 + 1] as usize;
        let len = st.meta[k * 3 + 2] as usize;
        let words = &st.words[off..off + len];
        let words_u64: &[u64] = unsafe { std::slice::from_raw_parts(words.as_ptr() as *const u64, words.len()) };
        let out = &mut st.out[k * 4096..(k + 1) * 4096];
        if decode_section_modulo(bpe, words_u64, out).is_err() {
            return ERR_STRUCT;
        }
    }
    (vt.SetIntArrayRegion)(
        env,
        jout,
        0,
        ((nsec as usize) * 4096) as jni::jsize,
        st.out.as_ptr(),
    );
    nsec
}

/// BIT-EXACT kernel SimpleBitStorage layout (javap @4789ca7): MODULO packing.
/// word w holds entries [w*vpl, w*vpl+vpl) at bit shifts 0, bpe, ..., (vpl-1)*bpe.
/// data.length = (size + vpl - 1) / vpl. bpe == 0 (ZeroBitStorage) => all zeros.
pub fn decode_section_modulo(bpe: u32, words: &[u64], out: &mut [i32]) -> Result<(), ()> {
    if out.len() < 4096 {
        return Err(());
    }
    if bpe == 0 {
        for v in out[..4096].iter_mut() {
            *v = 0;
        }
        return Ok(());
    }
    if bpe > 31 {
        return Err(());
    }
    let vpl = (64 / bpe) as usize;
    let need = (4096usize + vpl - 1) / vpl;
    if words.len() < need {
        return Err(());
    }
    let mask = if bpe >= 64 { u64::MAX } else { (1u64 << bpe) - 1 };
    for w in 0..need {
        let word = words[w];
        let base = w * vpl;
        let n_in = if base + vpl > 4096 { 4096 - base } else { vpl };
        for j in 0..n_in {
            out[base + j] = ((word >> (j * bpe as usize)) & mask) as i32;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// tests: modulo-layout oracle (bit-exact with javap SimpleBitStorage @4789ca7)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Independent oracle: the javap formula one step at a time.
    fn oracle(words: &[u64], i: usize, bpe: u32) -> u64 {
        let vpl = (64 / bpe) as usize;
        let word = i / vpl;
        let shift = (i % vpl) * bpe as usize;
        ((words[word] >> shift) & ((1u64 << bpe) - 1)) as u64
    }

    fn words_len(bpe: u32, size: usize) -> usize {
        let vpl = (64 / bpe) as usize;
        (size + vpl - 1) / vpl
    }

    #[test]
    fn modulo_matches_oracle_all_bpe() {
        let mut s = 0x9E37_79B9_1B48_59D1u64;
        let mut next = move || {
            s ^= s >> 12;
            s ^= s << 25;
            s ^= s >> 27;
            s.wrapping_mul(0x2545F4914F6CDD1D)
        };
        for bpe in [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15] {
            let n = words_len(bpe, 4096);
            let words: Vec<u64> = (0..n).map(|_| next()).collect();
            let mut out = vec![0i32; 4096];
            decode_section_modulo(bpe, &words, &mut out).expect("decode ok");
            for i in 0..4096 {
                assert_eq!(out[i] as u64, oracle(&words, i, bpe), "bpe={} i={}", bpe, i);
            }
        }
    }

    #[test]
    fn selftest_vectors_reproduced() {
        // the java selfTest bpe=4 vector: entry i = (i*7)&15
        let words: Vec<u64> = (0..256)
            .map(|w| {
                let mut acc = 0u64;
                for j in 0..16 {
                    let i = w * 16 + j;
                    acc |= (((i * 7) & 15) as u64) << (j * 4);
                }
                acc
            })
            .collect();
        let mut out = vec![0i32; 4096];
        decode_section_modulo(4, &words, &mut out).unwrap();
        for i in 0..4096 {
            assert_eq!(out[i], ((i * 7) & 15) as i32);
        }
        // bpe=15: vpl=4, 1024 words
        let words15: Vec<u64> = (0..1024)
            .map(|w| {
                let mut acc = 0u64;
                for j in 0..4 {
                    let i = w * 4 + j;
                    acc |= (((i * 11 + 3) & 32767) as u64) << (j * 15);
                }
                acc
            })
            .collect();
        let mut out15 = vec![0i32; 4096];
        decode_section_modulo(15, &words15, &mut out15).unwrap();
        for i in 0..4096 {
            assert_eq!(out15[i], ((i * 11 + 3) & 32767) as i32);
        }
    }

    #[test]
    fn zero_bpe_and_shape_guards() {
        let mut out = vec![7i32; 4096];
        decode_section_modulo(0, &[], &mut out).unwrap();
        assert!(out.iter().all(|&v| v == 0));
        // short words: rejected
        assert!(decode_section_modulo(4, &vec![0u64; 10], &mut out).is_err());
        // bpe 32+: rejected (kernel palettes never reach it; fail-closed)
        assert!(decode_section_modulo(32, &vec![0u64; 64], &mut out).is_err());
        // exact kernel data.length: bpe=4 -> 256, bpe=15 -> 1024, bpe=5 -> 342
        assert_eq!(words_len(4, 4096), 256);
        assert_eq!(words_len(15, 4096), 1024);
        assert_eq!(words_len(5, 4096), 342);
    }

    #[test]
    fn bpe5_tail_word_partial() {
        // bpe=5: vpl=12, 342 words for 4096 entries; last word holds 4096-341*12=4 entries
        let bpe = 5u32;
        let n = words_len(bpe, 4096);
        assert_eq!(n, 342);
        let mut words = vec![0u64; n];
        for w in 0..n {
            let mut acc = 0u64;
            let base = w * (64 / bpe as usize);
            let cnt = (64 / bpe as usize).min(4096 - base);
            for j in 0..cnt {
                let i = base + j;
                acc |= (((i * 3) & 31) as u64) << (j * bpe as usize);
            }
            words[w] = acc;
        }
        let mut out = vec![0i32; 4096];
        decode_section_modulo(bpe, &words, &mut out).unwrap();
        for i in 0..4096 {
            assert_eq!(out[i] as u64, oracle(&words, i, bpe));
            assert_eq!(out[i], ((i * 3) & 31) as i32);
        }
    }
}
