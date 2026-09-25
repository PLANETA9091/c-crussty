//! COLPUSH — подсистема collide+push (push-половина) ЦЕЛИКОМ на Rust
//! (TASK-419-A, round-419 vector A — lever cmp420_colpush, закон 6 v17).
//!
//! БОТЛНЕК (BOTTLENECK-419): collide ~8.7% + MobPushOps.push/move ~7-9% java.
//! На master push-путь per-entity: mobUpsert-JNI + chain-скан + java-фильтры
//! + аллокации на КАЖДОГО living entity В КАЖДЫЙ aiStep (~40-48k/тик).
//!
//! ARCHITECTURE (закон 6 v17 — один bulk-JNI на подсистему на тик):
//! 1. ВХОДНОЙ БУФЕР: персистентные java-массивы (по плотному id mobs_soa):
//!      COL_D double[id*6+0..5] = cx, cy, cz, hx, hz, hh   (ТОЧНЫЕ полуэкстенты
//!          по осям — не max-superset; радиус-гейт 2.0 java-стороной)
//!      COL_I int[id*3+0..2]    = lid, flags, freshTick
//!    Пишутся per-entity в момент pushEntities (aiStep @850, ПОСЛЕ move ⇒
//!    end-of-tick позиция) — простые store, 0 JNI, 0 аллокаций. Flags bit0 =
//!    plane-covered (isPushable && alive && !spectator && !noPhysics &&
//!    team==null && !vehicle && !passenger && max(hx,hz,hh) <= 2.0).
//! 2. ОДИН bulk-JNI/тик `colpushTick` (main-поток, RegionTickOps.forEach ДО
//!    GO-барьера = 0 гонок; lazy tryLock-фоллбек без блокировок — конвой
//!    невозможен): rust строит активный список (fresh == tick-1 — снапшот
//!    end-of-previous-tick = ТОЧНО текущие позиции: никто ещё не тикал),
//!    uniform-grid broad-phase (ячейка 4.0, pad 1, reach ≤ hx_i+hx_j ≤ 4.0),
//!    точный AABB-overlap (строгие сравнения = эквивалент AABB.intersects)
//!    и пишет CSR списков кандидатов (по возрастанию плотного id) в
//!    java-массивы OFF[idTop+1] + IDS[cap].
//! 3. ВЫХОДНОЙ БУФЕР: CSR. JAVA ХВОСТ ОСТАЁТСЯ ВАНИЛЬНЫМ БИТ-В-БАЙТ:
//!    cramming (RNG self.random.nextInt(4), nonPass, hurtServer 6.0f),
//!    numCollisions (живые поля, живой cross-entity cap), e.push(self)
//!    (живые позиции, живое округление per-pair add, hasImpulse). Rust =
//!    чистый bulk-broadphase; вся семантика пуша — ванильная по построению.
//! 4. Плоскость mobs_soa рефрешится ИЗ СВОИХ строк одним WLOCK-брэкетом
//!    (crate::mobs_soa::colpush_plane_refresh) — sscan/ai-плоскости-носители
//!    читают свежие колонки БЕЗ per-entity JNI. Заодно дренирует eqsnap-
//!    шарды (fallback-сущности держат их пустыми).
//!
//! DELTA (задокументировано, класс ghost-контракта eqsnap master):
//! - кандидаты из end-of-previous-tick позиций (≤1 тик ghost);
//! - порядок кандидатов = возрастание плотного id (вместо live chain-порядка);
//! - passengers/vehicles/teamed/oversized исключены из плоскости (per-entity
//!   fail-closed ваниль; bench-популяция ≈ 0 таких);
//! - cramming-count по снапшоту (граничные ±1 у порога RNG-ветки).
//! Пустой флаг = бридж не определён, хук спит, LivingEntity бит-в-байт ваниль.
//!
//! ARM markers (server stdout):
//!   "[crussty-plugin] cmp420_colpush: defined net/minecraft/world/entity/ColpushOps in kernel loader"
//!   "[crussty-plugin] cmp420_colpush: ARMED (bulk push broadphase CSR; retransform rc=...)"
//!   "[crussty-plugin] cmp420_colpush: PATCHED LivingEntity.pushEntities (...)"
//!   "[crussty-plugin] cmp420_colpush: bulk EFFECT armed (first tick N, rows A, pairs B)"
//! Java-маркер (ColpushOps): "colpush: push-plane EFFECT armed (first gate hit ...)".

use crate::classfile;
use jvmti_bindings::jni;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock};

const TARGET_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/ColpushOps";

const OPS_BYTES: &[u8] = include_bytes!("../colpush/build/net/minecraft/world/entity/ColpushOps.class");

const PROBE_MAGIC: i32 = 0x435050; // "CP"
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

/// Плотный-id вселенная (общая с mobs_soa — JavaSide 1<<20).
const IDS_CAP: usize = 1 << 20;
/// Грид-таблица (open-addressed), ячейка 4.0 → 640-радиус бокс = 320×320 живых
/// ячеек + tombstones; 1<<18 как у плоскости.
const GRID_CAP: usize = 1 << 18;
/// Ячейка грида: reach пары = hx_i + hx_j ≤ 2.0 + 2.0 = 4.0 → pad 1.
const CELL: f64 = 4.0;
const H1: i64 = 0x9E37_79B9_1B48_59D1u64 as i64; // golden-ratio scramble
const H2: i64 = 0xBF58_476D_1CE4_E5B9u64 as i64;

/// Входной ряд: смещения внутри COL_D / COL_I на плотный id.
pub const ROW_D: usize = 6; // cx, cy, cz, hx, hz, hh
pub const ROW_I: usize = 3; // lid, flags, fresh
pub const FLAG_INCLUDE: i32 = 1;

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);

/// Счётчики EFFECT-маркера (первый bulk с парами).
static FIRST_BULK: AtomicBool = AtomicBool::new(true);
pub static LAST_ROWS: AtomicUsize = AtomicUsize::new(0);
pub static LAST_PAIRS: AtomicUsize = AtomicUsize::new(0);

struct BulkState {
    /// grow-once scratch: активные id, грид-цепи, CSR.
    act: Vec<i32>,
    head: Vec<i32>,
    next: Vec<i32>,
    cnt: Vec<i32>,
    off_scratch: Vec<i32>,
    csr: Vec<i32>,
}
static BULK_LOCK: Mutex<()> = Mutex::new(());
fn bulk_state() -> &'static Mutex<BulkState> {
    static S: OnceLock<Mutex<BulkState>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(BulkState {
            act: Vec::with_capacity(1 << 16),
            head: vec![0; GRID_CAP],
            next: vec![0; IDS_CAP],
            cnt: vec![0; IDS_CAP],
            off_scratch: vec![0; IDS_CAP + 1],
            csr: Vec::with_capacity(1 << 21),
        })
    })
}

fn lever_flag_matches() -> bool {
    // STRICT-OR (TASK-426-A): cmp420_colpush (свой, master-сертифицированный
    // +29.5 pair @4ab7306) ИЛИ cmp424_mobfeed (SoA-кормление — будит фид
    // pushEntities whole-body redirect под вектор-флагом A-рестарта).
    // Прошлые прочие флаги = бит-в-байт прежнее поведение.
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let t = v.trim();
            t == "cmp420_colpush" || t == "cmp424_mobfeed" || t == "cmp430_inside" || t == "cmp432_inside2" || t == "cmp436_ins4" || t == "cmp457_paldelta"
            || t == "cmp438_sense" // TASK-444-C: sense family union
            || t == "cmp451_senseins" || t == "cmp457_paldelta" || t == "cmp453_diet" || t == "cmp434_chunkpl" || t == "cmp435_chunk3" || t == "cmp437_chunk4" || t == "cmp444_chunk5" || t == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        })
        .unwrap_or(false)
}

/// Register the COMPOSING LivingEntity byte hook (idempotent; call once from
/// cplugin_init AFTER mobs_ai::register — мой хук последний в цепочке
/// LivingEntity: получатель = soa/aibatch-выход, патч переписывает ТОЛЬКО
/// тело pushEntities whole-body redirect'ом в ColpushOps). Pre-arm возвращает
/// None (прежние хуки владеют pristine-захватом и своими сервами).
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp420_colpush: dormant (lever_flag != cmp420_colpush, vanilla pushEntities)"
        );
        return;
    }
    ARMED.store(true, Ordering::Release);
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: chain output проходит насквозь (ранние хуки владеют
            // pristine-захватом; mobs_ai после меня ещё НЕ зарегистрирован —
            // я САМЫЙ последний хук LivingEntity).
            return None;
        }
        // Композиция на ПОЛУЧЕННЫЕ байты (soa+aibatch-патчи сохранены;
        // redirect заменяет только тело pushEntities — aiStep-ретаргет
        // mobs_ai и травел-патчи нетронуты). Патч идемпотентен: повторный
        // проход над уже-redirect'нутым телом даёт тот же вывод.
        match classfile::patch_push_entities_colpush(bytes) {
            Ok((out, outcome)) => match outcome {
                classfile::RetargetOutcome::Retargeted { .. }
                | classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    static SERVED: AtomicBool = AtomicBool::new(false);
                    if !SERVED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] cmp420_colpush: PATCHED {TARGET_CLASS}.pushEntities ({} bytes; whole-body redirect -> ColpushOps.pushEntities, bulk push broadphase CSR; {outcome:?})",
                            out.len()
                        );
                    }
                    Some(out)
                }
                other => {
                    static NF: AtomicBool = AtomicBool::new(false);
                    if !NF.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] cmp420_colpush: pushEntities site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR: AtomicBool = AtomicBool::new(false);
                if !ERR.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] cmp420_colpush: compose patch rejected ({e}) — pass-through, pushEntities vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for the LivingEntity load + boot-quiet + the
/// soa/stagger/ai LIVING serves (chain ordering — their stash-based serves
/// must not run AFTER my compose) → define ColpushOps + RegisterNatives →
/// resolution closure → READY (hook starts composing) → retransform
/// LivingEntity (mobs_ai pattern; the serve happens in the hook chain).
/// Background activation (TASK-420-A FIX-МАНДАТ п.1 — define ДО арма,
/// root-cause round-colpusha ROOTCAUSE-NCDFE.md: волна-419 звала
/// RegionTickOps.bulkTick→ColpushOps ДО define ⇒ HotSpot кэширует провал
/// CP-резолюции ⇒ NCDFE ×1902 навсегда).
///
/// ПОРЯДОК (queryplane-паттерн mod+register(last)+activate(last)):
///   1. wait LivingEntity load + boot-quiet;
///   2. class-version + resolution-closure gates (structural, no JNI exec);
///   3. EARLY DEFINE: define_class(ColpushOps) + RegisterNatives, БЕЗ
///      исполнения java-кода (0 резолюций ⇒ poison невозможен). 20s settle
///      убран: define пассивен, а java-гейт bulkTick (RegionTickOps
///      COLPUSH_ON) держит call site недостижимым до флипа п.7;
///   4. wait soa/stagger/ai LIVING serves (compose ordering ретрансформа);
///   5. selfTest на СОХРАНЁННОМ global ref (TASK-417-C find_class-fix:
///      JVMTI-скан фильтрует не-INITIALIZED, звать надо на ref от
///      define_class). После soa_served MobPushOps гарантированно определён
///      ⇒ selfTest не может отравиться NCDFE;
///   6. READY → retransform LivingEntity (compose serve pushEntities);
///   7. ARM ПОСЛЕДНИМ шагом: RegionTickOps.COLPUSH_ON = true (JNI
///      SetStaticBooleanField через kernel-loader Class.forName, retry до
///      120s) — вызов bulkTick возможен ТОЛЬКО после define+selfTest.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        // LivingEntity loads at boot (entity superclass).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(TARGET_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp420_colpush: {TARGET_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp420_colpush: boot marker not seen, hook stays dormant");
            return;
        }

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
                "[crussty-plugin] cmp420_colpush: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild colpush/; hook stays dormant"
            );
            return;
        }

        // S7-164-гвард: доставленный classfile обязан объявить все статические
        // входы (pushEntities-редирект + bulkTick-триггер RegionTickOps).
        if let Err(e) = classfile::colpush_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp420_colpush: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // (3) EARLY DEFINE — define_class + RegisterNatives, java-код НЕ
        // исполняется (0 lazy-резолюций). Мост в лоадере ЗАРАНЕЕ до любого
        // тика; global ref сохраняется для selfTest п.5.
        let Some(gops) = define_bridge() else {
            eprintln!("[crussty-plugin] cmp420_colpush: bridge definition failed, hook stays dormant");
            return;
        };
        eprintln!(
            "[crussty-plugin] cmp420_colpush: defined {OPS_CLASS} in kernel loader (pre-arm: RegionTickOps bulkTick gate still OFF — NCDFE structurally impossible)"
        );

        // (4) Ordering: wait for the soa/stagger/ai LivingEntity serves (their
        // stash-based serves would REPLACE my composed bytes if they ran
        // after my retransform). Timeout = proceed anyway (fail-open on
        // ordering only, composition itself stays fail-closed).
        let order_deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        loop {
            let soa = crate::mobs_ai::soa_served();
            let stag = crate::mobs_ai::stagger_served();
            let ai = crate::mobs_ai::ai_ready();
            if soa && stag && ai {
                break;
            }
            if std::time::Instant::now() > order_deadline {
                eprintln!(
                    "[crussty-plugin] cmp420_colpush: LIVING serve signals timeout (soa={soa} stagger={stag} ai={ai}) — proceeding, chain composes current bytes"
                );
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // (5) selfTest на сохранённом global ref (мандат п.1: selfTest ДО
        // арма). soa_served ⇒ MobPushOps определён ⇒ резолюции selfTest
        // безопасны; any Throwable ⇒ fail-closed dormant (never arm).
        let selftest = cplug_sdk::jni_util::with_attached(|env| colpush_selftest(env, gops))
            .unwrap_or(false);
        if !selftest {
            eprintln!(
                "[crussty-plugin] cmp420_colpush: selfTest FAILED — hook stays dormant (fail-closed; bulkTick gate stays OFF)"
            );
            return;
        }
        eprintln!("[crussty-plugin] cmp420_colpush: selfTest=true (row-layout/IDS_CAP/MobPushOps universe OK) BEFORE arm");

        // (6) READY → retransform LivingEntity: hook composes the whole-body
        // pushEntities redirect onto the received chain bytes (soa+ai srv
        // preserved) and retransform publishes them.
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(TARGET_CLASS);
        eprintln!(
            "[crussty-plugin] cmp420_colpush: ARMED (bulk push broadphase CSR; retransform rc={rc})"
        );
        std::thread::sleep(std::time::Duration::from_millis(250));
        eprintln!(
            "[crussty-plugin] cmp420_colpush: applied (pushEntities -> ColpushOps; java tail = vanilla bit-exact, rust = bulk broadphase 1 JNI/tick)"
        );

        // (7) ARM — ПОСЛЕДНИЙ шаг (define+selfTest+retransform позади):
        // флип RegionTickOps.COLPUSH_ON=true делает bulkTick call site
        // достижимым ТОЛЬКО теперь ⇒ NCDFE невозможен по построению.
        arm_regiontickops_bulk();
    });
}

/// SelfTest on the KEPT global ref of the just-defined bridge (TASK-417-C
/// find_class-fix: JVMTI-scan filters non-INITIALIZED classes — call on the
/// define_class ref; this call is the class's first active use ⇒ <clinit>).
/// Any pending exception is cleared and reported as failure (fail-closed).
fn colpush_selftest(env: &jvmti_bindings::env::JniEnv, gops: *mut c_void) -> bool {
    let cls = gops as jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] cmp420_colpush: selfTest method resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!(
            "[crussty-plugin] cmp420_colpush: selfTest threw (late resolution) — fail-closed"
        );
        return false;
    }
    rc != 0
}

/// Define the ColpushOps bridge into the kernel loader (Entity anchor) +
/// RegisterNatives (colpushProbe/colpushTick). NO java code executed here
/// (0 lazy resolutions ⇒ no NCDFE-poison risk). Returns the KEPT global ref
/// of the defined class (for the later selfTest call).
fn define_bridge() -> Option<*mut c_void> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity") else {
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
            eprintln!("[crussty-plugin] cmp420_colpush: define_class({OPS_CLASS}) failed");
            return None;
        };

        // RegisterNatives: colpushProbe/colpushTick (impl — этот модуль).
        let names = [
            CString::new("colpushProbe").expect("no NUL"),
            CString::new("colpushTick").expect("no NUL"),
        ];
        let sigs = [
            CString::new("()I").expect("no NUL"),
            CString::new("(II[D[I[I[I)I").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: colpush_probe as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: colpush_tick as *const c_void as *mut c_void,
            },
        ];
        let reg = env.register_natives(c, &natives);
        if let Err(code) = reg {
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] cmp420_colpush: register_natives failed (code {code}) — hook stays dormant"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }

        // Keep the bridge across the with_attached boundary (selfTest later).
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

/// (7) ARM: flip RegionTickOps.COLPUSH_ON = true via JNI (kernel-loader
/// Class.forName(initialize=true) → GetStaticFieldID → SetStaticBooleanField
/// → read-back verify). RegionTickOps может быть ещё не определён
/// region_threads на этот момент — retry до 120s. До флипа bulkTick call
/// site НЕДОСТИЖИМ (volatile gate default OFF) ⇒ порядок define→arm
/// гарантирован даже при самом позднем define.
fn arm_regiontickops_bulk() {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    loop {
        let ok = cplug_sdk::jni_util::with_attached(|env| {
            let Some(entity) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity")
            else {
                crate::clear_exception(env);
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(entity.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return false;
            };
            let Some(forname) = env.get_static_method_id(
                class_cls,
                "forName",
                "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
            ) else {
                crate::clear_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            let Some(name) = env.new_string("net.minecraft.world.entity.RegionTickOps") else {
                crate::clear_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            let loaded = env.call_static_object_method(
                class_cls,
                forname,
                &[
                    jvmti_bindings::jni::jvalue { l: name },
                    jvmti_bindings::jni::jvalue { z: 1 /* initialize */ },
                    jvmti_bindings::jni::jvalue { l: loader },
                ],
            );
            let had_exc = crate::clear_exception(env);
            env.delete_local_ref(name);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            if had_exc || loaded.is_null() {
                // RegionTickOps ещё не определён region_threads — ретрай.
                return false;
            }
            let Some(fid) = env.get_static_field_id(loaded as jni::jclass, "COLPUSH_ON", "Z") else {
                crate::clear_exception(env);
                eprintln!(
                    "[crussty-plugin] cmp420_colpush: ARM-флип: RegionTickOps.COLPUSH_ON field not found — bulkTick stays OFF"
                );
                env.delete_local_ref(loaded);
                return false;
            };
            // Raw SetStaticBooleanField/GetStaticBooleanField (wrapper lacks
            // the boolean static-field setters; fn-table call is the
            // improved_noise.rs:876 precedent).
            let raw = env.raw();
            unsafe {
                let fn_table = &(**raw);
                (fn_table.SetStaticBooleanField)(raw, loaded as jni::jclass, fid, 1);
            }
            let back = unsafe {
                let fn_table = &(**raw);
                (fn_table.GetStaticBooleanField)(raw, loaded as jni::jclass, fid)
            };
            env.delete_local_ref(loaded);
            if back == 0 {
                eprintln!("[crussty-plugin] cmp420_colpush: ARM-флип read-back failed — bulkTick stays OFF");
                return false;
            }
            true
        })
        .unwrap_or(false);
        if ok {
            eprintln!(
                "[crussty-plugin] cmp420_colpush: RegionTickOps.bulkTick ARMED (COLPUSH_ON=true AFTER define+selfTest — arm-before-define NCDFE root-cause eliminated)"
            );
            return;
        }
        if std::time::Instant::now() > deadline {
            eprintln!(
                "[crussty-plugin] cmp420_colpush: RegionTickOps not reachable within 120s — bulkTick trigger stays OFF (fail-closed, vanilla push)"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(2_000));
    }
}


/// # Safety
/// Called by the JVM through RegisterNatives.
#[no_mangle]
pub unsafe extern "system" fn colpush_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    PROBE_MAGIC
}

/// ОДИН bulk-JNI/тик: CSR кандидатов пуша по всей популяции.
///
/// in_d: double[id_top * ROW_D] = (cx,cy,cz,hx,hz,hh) по плотному id;
/// in_i: int[id_top * ROW_I] = (lid, flags, freshTick);
/// out_off: int[id_top+1] (CSR offsets); out_ids: int[out_ids_cap].
/// Возвращает: >=0 = суммарная длина CSR; ERR_RANGE = out_ids мал
/// (java растит и ретраит — вход персистентный, проход идемпотентен);
/// ERR_STRUCT = java дизармит навсегда.
///
/// # Safety
/// Called by the JVM through RegisterNatives; массивы живые на время вызова.
#[no_mangle]
pub unsafe extern "system" fn colpush_tick(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    id_top: jni::jint,
    in_d: jni::jdoubleArray,
    in_i: jni::jintArray,
    out_off: jni::jintArray,
    out_ids: jni::jintArray,
) -> jni::jint {
    if !ARMED.load(Ordering::Acquire) || env.is_null() {
        return ERR_STRUCT;
    }
    if in_d.is_null() || in_i.is_null() || out_off.is_null() || out_ids.is_null() {
        return ERR_STRUCT;
    }
    if tick < 1 || id_top < 0 || (id_top as usize) > IDS_CAP {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let n = id_top as usize;
    let d_cap = unsafe { (vt.GetArrayLength)(env, in_d) } as usize;
    let i_cap = unsafe { (vt.GetArrayLength)(env, in_i) } as usize;
    let off_cap = unsafe { (vt.GetArrayLength)(env, out_off) } as usize;
    let ids_cap = unsafe { (vt.GetArrayLength)(env, out_ids) } as usize;
    if d_cap < n * ROW_D || i_cap < n * ROW_I || off_cap < n + 1 {
        return ERR_STRUCT; // структурный дрейф буферов — java дизарм
    }

    // Одна партия за раз (java-side tryLock гарантирует, это страховка).
    let Ok(mut st) = bulk_state().lock() else {
        return ERR_RANGE; // конкурентный вызов — java ретраит следующим тиком
    };

    let dp = unsafe { (vt.GetPrimitiveArrayCritical)(env, in_d, std::ptr::null_mut()) };
    if dp.is_null() {
        return ERR_STRUCT;
    }
    let ip = unsafe { (vt.GetPrimitiveArrayCritical)(env, in_i, std::ptr::null_mut()) };
    if ip.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, in_d, dp, 0) };
        return ERR_STRUCT;
    }
    let d = unsafe { std::slice::from_raw_parts(dp as *const f64, n * ROW_D) };
    let i = unsafe { std::slice::from_raw_parts(ip as *const i32, n * ROW_I) };

    let want_tick = tick - 1; // снапшот end-of-previous-tick

    // ---- PASS 1: активные ряды + грид ----
    st.act.clear();
    for slot in &mut st.head[..] {
        *slot = 0;
    }
    for id in 0..n {
        let flags = i[id * ROW_I + 1];
        let fresh = i[id * ROW_I + 2];
        if fresh != want_tick || flags & FLAG_INCLUDE == 0 {
            st.cnt[id] = 0;
            continue;
        }
        let b = id * ROW_D;
        let cx = d[b];
        let cy = d[b + 1];
        let cz = d[b + 2];
        let hx = d[b + 3];
        let hz = d[b + 4];
        let hh = d[b + 5];
        if !(cx.is_finite() && cy.is_finite() && cz.is_finite())
            || !(hx.is_finite() && hz.is_finite() && hh.is_finite())
            || hx <= 0.0
            || hz <= 0.0
            || hh <= 0.0
        {
            st.cnt[id] = 0;
            continue;
        }
        st.act.push(id as i32);
        st.cnt[id] = 0;
        let g = grid_slot(i[id * ROW_I], cx, cz);
        st.next[id] = st.head[g];
        st.head[g] = id as i32 + 1;
    }

    // ---- PASS 2: степени кандидатов (точный AABB-overlap) ----
    let rows = st.act.len();
    let act_snapshot: Vec<i32> = st.act.clone();
    for &a in act_snapshot.iter() {
        let a = a as usize;
        let ba = a * ROW_D;
        let cx = d[ba];
        let cz = d[ba + 2];
        let hx = d[ba + 3];
        let hz = d[ba + 4];
        let cy = d[ba + 1];
        let hh = d[ba + 5];
        let gxa = (cx / CELL).floor() as i64;
        let gza = (cz / CELL).floor() as i64;
        let mut deg: i32 = 0;
        for gz in (gza - 1)..=(gza + 1) {
            for gx in (gxa - 1)..=(gxa + 1) {
                let g = grid_slot_by(gx, gz, i[a * ROW_I]);
                let mut cur = st.head[g];
                while cur != 0 {
                    let b2 = (cur - 1) as usize;
                    cur = st.next[b2];
                    if b2 == a {
                        continue;
                    }
                    let bb = b2 * ROW_D;
                    // Точный AABB.intersects (строгие <): |dcx| < hx_a+hx_b и т.д.
                    if (d[bb] - cx).abs() < hx + d[bb + 3]
                        && (d[bb + 2] - cz).abs() < hz + d[bb + 4]
                        && (d[bb + 1] - cy).abs() < hh + d[bb + 5]
                    {
                        deg += 1;
                    }
                }
            }
        }
        st.cnt[a] = deg;
    }

    // ---- PASS 3: prefix-sum → OFF ----
    let mut total: i64 = 0;
    // OFF в java-массиве пишем на финальном проходе; тут считаем total и
    // наполняем off_scratch.
    st.off_scratch[0] = 0;
    let act_len = st.act.len();
    for (k, &a) in act_snapshot.iter().enumerate() {
        let a = a as usize;
        total += st.cnt[a] as i64;
        if (k + 1) < act_len {
            st.off_scratch[k + 1] = total as i32;
        }
    }
    let total = total as usize;
    // Неактивные id получают OFF-пары нулевой длины (deg=0).
    // Заполнение java OFF: для каждого id — begin/end (end = begin + deg).
    // Для неактивных begin==end==... используем предыдущий префикс (нули длины).
    // Для простоты: OFF[id+1]-OFF[id] == cnt[id] (0 для неактивных), а OFF[id]
    // для неактивных = префикс на момент id. Строим один проход по id.
    if total > ids_cap || total > i32::MAX as usize {
        unsafe {
            (vt.ReleasePrimitiveArrayCritical)(env, in_i, ip, 0);
            (vt.ReleasePrimitiveArrayCritical)(env, in_d, dp, 0);
        }
        return ERR_RANGE; // java растит IDS и ретраит
    }

    let op = unsafe { (vt.GetPrimitiveArrayCritical)(env, out_off, std::ptr::null_mut()) };
    if op.is_null() {
        unsafe {
            (vt.ReleasePrimitiveArrayCritical)(env, in_i, ip, 0);
            (vt.ReleasePrimitiveArrayCritical)(env, in_d, dp, 0);
        }
        return ERR_STRUCT;
    }
    let ids_p = unsafe { (vt.GetPrimitiveArrayCritical)(env, out_ids, std::ptr::null_mut()) };
    if ids_p.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out_off, op, 0) };
        unsafe {
            (vt.ReleasePrimitiveArrayCritical)(env, in_i, ip, 0);
            (vt.ReleasePrimitiveArrayCritical)(env, in_d, dp, 0);
        }
        return ERR_STRUCT;
    }
    let off = unsafe { std::slice::from_raw_parts_mut(op as *mut i32, n + 1) };
    let ids = unsafe { std::slice::from_raw_parts_mut(ids_p as *mut i32, ids_cap) };

    // ---- PASS 4: заполнение CSR ----
    let mut cursor: usize = 0;
    off[0] = 0;
    let mut ai: usize = 0; // индекс в act
    for id in 0..n {
        let is_active = ai < rows && st.act[ai] as usize == id;
        if is_active {
            let a = id;
            let ba = a * ROW_D;
            let cx = d[ba];
            let cz = d[ba + 2];
            let hx = d[ba + 3];
            let hz = d[ba + 4];
            let cy = d[ba + 1];
            let hh = d[ba + 5];
            let gxa = (cx / CELL).floor() as i64;
            let gza = (cz / CELL).floor() as i64;
            for gz in (gza - 1)..=(gza + 1) {
                for gx in (gxa - 1)..=(gxa + 1) {
                    let g = grid_slot_by(gx, gz, i[a * ROW_I]);
                    let mut cur = st.head[g];
                    while cur != 0 {
                        let b2 = (cur - 1) as usize;
                        cur = st.next[b2];
                        if b2 == a {
                            continue;
                        }
                        let bb = b2 * ROW_D;
                        if (d[bb] - cx).abs() < hx + d[bb + 3]
                            && (d[bb + 2] - cz).abs() < hz + d[bb + 4]
                            && (d[bb + 1] - cy).abs() < hh + d[bb + 5]
                        {
                            // кандидаты по возрастанию плотного id: грид-проход
                            // идёт по цепям; для детерминизма сортируем ниже.
                            if cursor < ids_cap {
                                ids[cursor] = b2 as i32;
                            }
                            cursor += 1;
                        }
                    }
                }
            }
            ai += 1;
        }
        off[id + 1] = cursor as i32;
    }
    // Детерминизм: сортируем каждый список кандидатов по возрастанию id.
    // (Проход выше писал подряд; сортируем срезы по OFF.)
    for k in 0..n {
        let b = off[k] as usize;
        let e = off[k + 1] as usize;
        if e > b {
            let s = &mut ids[b..e];
            // Малые списки — insertion sort (в среднем 0-8 элементов).
            for m in 1..s.len() {
                let key = s[m];
                let mut p = m;
                while p > 0 && s[p - 1] > key {
                    s[p] = s[p - 1];
                    p -= 1;
                }
                s[p] = key;
            }
        }
    }

    unsafe {
        (vt.ReleasePrimitiveArrayCritical)(env, out_ids, ids_p, 0);
        (vt.ReleasePrimitiveArrayCritical)(env, out_off, op, 0);
    }

    // ---- PASS 5: рефреш плоскости mobs_soa из своих строк (1 WLOCK) ----
    // (sscan/ai-плоскости-носители читают свежие колонки; eqsnap-шарды
    // дренируются заодно.)
    let drained = crate::mobs_soa::colpush_plane_refresh(n, d, i, want_tick);

    unsafe {
        (vt.ReleasePrimitiveArrayCritical)(env, in_i, ip, 0);
        (vt.ReleasePrimitiveArrayCritical)(env, in_d, dp, 0);
    }

    LAST_ROWS.store(rows, Ordering::Relaxed);
    LAST_PAIRS.store(total, Ordering::Relaxed);
    if FIRST_BULK.swap(false, Ordering::Relaxed) {
        eprintln!(
            "[crussty-plugin] cmp420_colpush: bulk EFFECT armed (first tick {tick}, rows {rows}, pair-slots {total}, plane-refresh {drained})"
        );
    }
    total as i32
}

#[inline]
fn grid_slot(lid: i32, cx: f64, cz: f64) -> usize {
    grid_slot_by((cx / CELL).floor() as i64, (cz / CELL).floor() as i64, lid)
}

#[inline]
fn grid_slot_by(gx: i64, gz: i64, lid: i32) -> usize {
    let mut h = (gx.wrapping_mul(H1)) ^ (gz.wrapping_mul(H2)) ^ ((lid as i64).wrapping_mul(0x517C_C1B7_2722_0A95u64 as i64));
    h ^= (h >> 32);
    h ^= (h >> 17);
    (h & (GRID_CAP as i64 - 1)) as usize
}
