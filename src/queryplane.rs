//! Runtime wiring for the QUERYPLANE lever (TASK-412-B/413-B, round-412-b-p1
//! — lever cmp412_b2p1, STRICT eq; bridge queryplane/net/minecraft/world/
//! entity/QueryPlaneOps.java, patches in src/classfile.rs QUERYPLANE
//! section).
//!
//! Lane: broadphase entity-query 13.91-14.23% wall на meganav-профиле
//! (ChunkEntitySlices.getEntities(AABB)/CollisionUtil census, закон 6 v16 —
//! ПОДСИСТЕМА entity-query/collision слой). Три сайта:
//!   1) Level.getEntitiesOfClass(Class,AABB,Predicate) — 1.85% wall,
//!      NearestAttackableTargetGoal.findTarget 0.81% + AvoidEntityGoal.canUse
//!      0.22% (оба — Player.class против 4 fake-players);
//!   2) Level.moonrise$getHardCollidingEntities — 2.01% wall (Entity.collide
//!      1.02 + Level.noCollision 0.92; hard-colliders в сцене не спавнятся);
//!   3) ChunkEntitySlices.addEntity — монотонный HARD_ADDS-счетчик (probe),
//!      гейт пустого fast path сайта 2.
//!
//! ЗАКОН 6 v16, bulk-JNI/тик: плоскость НЕ ДОБАВЛЯЕТ ни одного JNI-вызова
//! (fast paths чисто-java: O(players) список + empty-list константа); soa-мост
//! (MobPushOps oversized de-globalization, src/mobs_soa.rs) сохраняет свой
//! ОДИН bulk-JNI (mobQuery) на батч. Снапшот и так живёт — queryplane читает
//! готовый authoritative per-level список players().
//!
//! STRICT-ОРАКУЛ (cargo test, ниже): бит-в-байт candidate-set =
//! AABB-пересечения — vanilla AABB.intersects (javap @0-63, строгие
//! dcmpg/dcmpl сравнения, NaN → false) реплицирован в rust; секционная модель
//! регистрации ChunkEntitySlices (entity регистрируется во ВСЕ 16³-секции,
//! покрываемые его AABB) не даёт ни ложных отрицательных, ни ложных
//! положительных: финальный candidate-set section-walk == {e : e.bb ∩ box},
//! проверено на рандомизированных популяциях/боксах против fast-path модели
//! (per-кандидат intersects). Границы (касание граней) — strict `<`/`>`:
//! соприкасающиеся боксы НЕ пересекаются.
//!
//! DELIVERY (collide_batch/mobs_ai гибрид):
//!   - Level: compose-on-top hook (stagger/navplane ретаргет sendBlockUpdated
//!     живёт в той же hook-цепи ВЫШЕ — mobs_ai прецедент; pristine-стэш у
//!     первого хука, compose ПОВЕРХ полученных байт, idempotent per-call).
//!   - ChunkEntitySlices: никто больше не хукает — collide_batch паттерн
//!     (pristine stash до READY, cached serve после).
//!   - Определение QueryPlaneOps в kernel loader (якорь Entity) + selfTest()
//!     через reflection (эффект-маркер) ДО вычисления патчей.
//!
//! Fail-dominant: пустой/чужой флаг — класс не определяется, хуки не
//! регистрируются (ваниль бит-в-байт); shape-mismatch = Err/NotFound →
//! pass-through/не-serve (ваниль); java-сторона — per-plane permanent
//! disarm latches + Throwable → ванильная реплика тела на вызов.

use crate::classfile::{self, RetargetOutcome};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const LEVEL_CLASS: &str = classfile::QUERY_LEVEL_CLASS;
const SLICES_CLASS: &str = classfile::SLICES_CLASS;
const OPS_CLASS: &str = classfile::QUERY_OPS_CLASS;

const OPS_BYTES: &[u8] =
    include_bytes!("../queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class");

static READY: AtomicBool = AtomicBool::new(false);

/// STRICT OR (TASK-417-C): {cmp412_b2p1 || cmp415_mcomp || cmp416_mcomp ||
/// cmp417_bq}. STRICT OR распространяется в обратную сторону (meganav-плоскости
/// армятся всеми композитными флагами — nav_plane/tickplane/mobs_manager/
/// ItemEntityManager/MobAiOps/MobScanOps/collide_batch/stagger; на cvs-носителе
/// ТЕ ЖЕ сайты принимают cmp414_cvs — старые id нетронуты), новый композит
/// round-417-C cmp417_bq = cvs-носитель (meganav⊕eqsnap-v3⊕race-fix⊕blob-sync)
/// ⊕ queryplane AWAKE (find_class fix — см. activate()).
fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            v.trim() == "cmp412_b2p1"
                || v.trim() == "cmp415_mcomp"
                || v.trim() == "cmp416_mcomp"
                || v.trim() == "cmp417_bq"
                // TASK-419-A (colpush): колпаш-носитель — queryplane awake.
                || v.trim() == "cmp420_colpush"
                || v.trim() == "cmp421_brain" || v.trim() == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2"
        })
        .unwrap_or(false)
}


/// TASK-432-B: honest lever-id for the inside-plane composite (either the
/// round-432 flag or the round-430 carrier flag arms the same stages).
fn inside_plane_label() -> &'static str {
    match std::env::var("CRUSSTY_LEVER_FLAG").as_deref() {
        Ok("cmp432_inside2") => "cmp432_inside2",
        Ok("cmp436_ins4") => "cmp436_ins4",
        Ok("cmp440_ins4d") => "cmp440_ins4d", // TASK-442-B: свой id
        _ => "cmp430_inside",
    }
}

/// Lever id for boot markers (TASK-416-A: единый lever-id композита эры в
/// ARM/EFFECT-маркерах; легаси флаги печатают свой id). Round-417-C: cmp417_bq.
fn lever_id() -> &'static str {
    match std::env::var("CRUSSTY_LEVER_FLAG").as_deref() {
        Ok("cmp420_colpush") => "cmp420_colpush",
        // TASK-426-A: SoA-feed carrier — свой id в ARM/EFFECT-маркерах.
        Ok("cmp424_mobfeed") => "cmp424_mobfeed",
        // TASK-430-B: inside-plane subsystem round — свой id.
        Ok("cmp430_inside") | Ok("cmp432_inside2") | Ok("cmp436_ins4") | Ok("cmp440_ins4d") => inside_plane_label(),
        Ok("cmp417_bq")
            // TASK-421-A (brain): свой id в ARM-маркерах.
            | Ok("cmp421_brain") => "cmp421_brain",
        // TASK-422-B (iter-2): свой id для вектор-ног.
        Ok("cmp422_brain2") => "cmp422_brain2",
        Ok("cmp416_mcomp") => "cmp416_mcomp",
        Ok("cmp415_mcomp") => "cmp415_mcomp",
        _ => "cmp412_b2p1",
    }
}

// --- ChunkEntitySlices stash (collide_batch pattern) -----------------------

struct SliceStash {
    orig: Mutex<Option<Vec<u8>>>,
    patch: Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

fn slices() -> &'static SliceStash {
    static S: std::sync::OnceLock<SliceStash> = std::sync::OnceLock::new();
    S.get_or_init(|| SliceStash {
        orig: Mutex::new(None),
        patch: Mutex::new(None),
        served: AtomicBool::new(false),
    })
}

fn slices_stash(bytes: &[u8]) {
    let mut g = slices().orig.lock().unwrap();
    if g.is_none() {
        *g = Some(bytes.to_vec());
    }
}

/// Register the byte hooks (call once from cplugin_init). Dormant-invisible:
/// с чужим/пустым флагом хуки не регистрируются вовсе.
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp412_b2p1: queryplane dormant (lever_flag != cmp412_b2p1, vanilla entity queries)"
        );
        return;
    }
    // Level: compose-on-top (mobs_ai pattern) — chain output (navplane
    // sendBlockUpdated retarget уже стоит) патчится per-call после READY.
    cplug_sdk::hooks::register_bytes(LEVEL_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None; // pre-arm: цепь не трогаем
        }
        match compose_level(bytes) {
            Ok(Some(out)) => Some(out),
            Ok(None) => None, // fail-closed pass-through (лог once внутри)
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] cmp412_b2p1: Level compose rejected ({e}) — pass-through (fail-closed)"
                    );
                }
                None
            }
        }
    });
    // ChunkEntitySlices: stash/serve (collide_batch pattern).
    cplug_sdk::hooks::register_bytes(SLICES_CLASS, move |_name, bytes| {
        let s = slices();
        if !READY.load(Ordering::Acquire) {
            slices_stash(bytes);
            return None;
        }
        let cached = s.patch.lock().unwrap().clone();
        if !s.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: hook serve {SLICES_CLASS} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Compose both Level whole-body redirects onto RECEIVED chain bytes.
/// Ok(Some) = served (possibly unchanged when already composed);
/// Ok(None) = fail-closed pass-through; Err = shape mismatch (logged once).
fn compose_level(
    bytes: &[u8],
) -> Result<Option<Vec<u8>>, String> {
    let (geoc, o1) = classfile::patch_level_get_entities_of_class(bytes)?;
    let (hard, o2) = classfile::patch_level_get_hard_colliding_entities(&geoc)?;
    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
    let matched = |o: &RetargetOutcome| {
        matches!(
            o,
            RetargetOutcome::Retargeted { .. } | RetargetOutcome::AlreadyPatched { .. }
        )
    };
    if matched(&o1) && matched(&o2) {
        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: hook serve {LEVEL_CLASS} {} bytes (composed: getEntitiesOfClass {o1:?}, hardColliding {o2:?})",
                hard.len()
            );
        }
        return Ok(Some(hard));
    }
    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "[crussty-plugin] cmp412_b2p1: Level redirect sites not matched ({o1:?} / {o2:?}) — pass-through (fail-closed)"
        );
    }
    Ok(None)
}

/// Background activation: boot quiet → define QueryPlaneOps (+selfTest) →
/// compute patches from pristine/captured bytes → READY → retransform both.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp412_b2p1: boot marker not seen, queryplane stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

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
                "[crussty-plugin] cmp412_b2p1: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild queryplane/ via scripts/build_queryplane_ops.sh; hook stays dormant"
            );
            return;
        }

        // S7-164 resolution closure: bridge declares all three statics.
        if let Err(e) = classfile::queryplane_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the KERNEL loader (Entity anchor — same as
        // collide/nav bridges) and run the java-side selfTest() reflectively
        // (эффект-маркер: класс жив ДО всяких ретаргетов).
        //
        // TASK-417-C find_class FIX (root-cause 158889b:research/
        // RESEARCH-A-iter3.md, bottom): cplug_sdk::classes::find_class =
        // JVMTI-scan с фильтром status & JVMTI_CLASS_STATUS_INITIALIZED.
        // Класс, только что определённый define_class, ещё НЕ инициализирован
        // (init = первый статический вызов — selfTest и есть первый!),
        // поэтому скан его ОТФИЛЬТРОВЫВАЛ → find_class None → selfTest false
        // на раннере (b2p1/mc1 queryplane dormant при локально-зелёном 5/5 —
        // локально класс грузится app-loader'ом, find_class не участвует).
        // ФИКС: звать selfTest на ЛОКАЛЬНОМ ref'е из define_class
        // (инициализация происходит на этом вызове); find_class НЕ участвует
        // в selfTest-пути.
        let selftest = define_bridge_and_selftest();
        if !selftest {
            eprintln!(
                "[crussty-plugin] {}: define/selfTest failed — hook stays dormant (TASK-417-C find_class fix active: selfTest зван на local ref из define_class; false = реальный гейт/сбой)",
                lever_id()
            );
            return;
        }
        // ГРОМКИЙ ARM-МАРКЕР (TASK-416-A рецепт 3: queryplane ARMED/awake —
        // grep-able boot-доказательство, selfTest()==true ДО всяких ретаргетов).
        eprintln!(
            "[crussty-plugin] {}: ARMED queryplane awake (selfTest==true, HARD_ADDS=0; Level.getEntitiesOfClass players fast path + hard-colliding empty fast path + addEntity hard-probe; target broadphase <=7)",
            lever_id()
        );
        eprintln!(
            "[crussty-plugin] {}: defined {OPS_CLASS} in kernel loader, selfTest ok (HARD_ADDS=0)",
            lever_id()
        );

        // ChunkEntitySlices pristine capture (predates hook = no-op
        // retransform; fluid_guard pattern).
        if slices().orig.lock().unwrap().is_none() {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: {SLICES_CLASS} predates hook, capturing via no-op retransform"
            );
            for _attempt in 1..=5 {
                let _ = cplug_sdk::retransform_class(SLICES_CLASS);
                if slices().orig.lock().unwrap().is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            let Some(original) = slices().orig.lock().unwrap().clone() else {
                eprintln!(
                    "[crussty-plugin] cmp412_b2p1: no pristine bytes for {SLICES_CLASS}, hook stays dormant"
                );
                return;
            };
            let (patched, outcome) = match classfile::patch_slices_hard_probe(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] cmp412_b2p1: addEntity probe rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: PATCHED {SLICES_CLASS}.addEntity ({} -> {} bytes, {outcome:?}; invokeinterface moonrise$isHardColliding -> invokestatic isHardCollidingProbe + 2nop)",
                original.len(),
                patched.len()
            );
            *slices().patch.lock().unwrap() = Some(Arc::from(patched));
        } else {
            // Hook stashed at load time — compute from the stash.
            let original = slices().orig.lock().unwrap().as_ref().unwrap().clone();
            match classfile::patch_slices_hard_probe(&original) {
                Ok((patched, outcome)) => {
                    if !matches!(
                        outcome,
                        RetargetOutcome::Retargeted { .. }
                            | RetargetOutcome::AlreadyPatched { .. }
                    ) {
                        eprintln!(
                            "[crussty-plugin] cmp412_b2p1: unexpected addEntity outcome ({outcome:?}), hook stays dormant"
                        );
                        return;
                    }
                    eprintln!(
                        "[crussty-plugin] cmp412_b2p1: PATCHED {SLICES_CLASS}.addEntity ({} -> {} bytes, {outcome:?}; invokeinterface moonrise$isHardColliding -> invokestatic isHardCollidingProbe + 2nop)",
                        original.len(),
                        patched.len()
                    );
                    *slices().patch.lock().unwrap() = Some(Arc::from(patched));
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] cmp412_b2p1: addEntity probe rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            }
        }

        // Level: compose-on-top — патч вычисляется на лету в хуке (mobs_ai
        // прецедент); pristine-стэш у первого хука цепи (stagger).

        // ГРОМКИЙ ARM-МАРКЕР (Retargeted sites-доказательства — в log-строках
        // PATCHED/compose выше; эффект-маркеры — QueryPlaneOps first gate hit).
        READY.store(true, Ordering::Release);
        let rc_l = cplug_sdk::retransform_class(LEVEL_CLASS);
        let rc_s = cplug_sdk::retransform_class(SLICES_CLASS);
        eprintln!(
            "[crussty-plugin] cmp412_b2p1: ARMED queryplane (Level.getEntitiesOfClass + moonrise$getHardCollidingEntities whole-body redirects ⊕ ChunkEntitySlices.addEntity hard-probe; retransform rc Level={rc_l} Slices={rc_s}; STRICT oracle: candidate-set = AABB-intersections bit-in-byte; 0 added JNI, fail-dominant per-plane latches)"
        );
        std::thread::sleep(std::time::Duration::from_millis(250));
        if slices().served.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: applied (queryplane; HARD_ADDS=0 empty-hard fast path, players() O(4) player-class fast path)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp412_b2p1: slices patch NOT APPLIED after retransform (kernel build mismatch?) — vanilla behavior"
            );
        }
    });
}

/// Define QueryPlaneOps into the kernel loader AND run selfTest() on the
/// LOCAL ref returned by define_class.
///
/// TASK-417-C find_class FIX: первый статический вызов на локальном ref'е
/// инициализирует класс (<clinit> читает CRUSSTY_LEVER_FLAG → ENABLED);
/// cplug_sdk::classes::find_class НЕ участвует в selfTest-пути — его
/// JVMTI-скан фильтрует не-INITIALIZED классы и только что определённый
/// бридж терял (selfTest false on runner, локально 5/5 — app-loader путь).
fn define_bridge_and_selftest() -> bool {
    let defined = cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity") else {
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
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] {}: define_class({OPS_CLASS}) failed", lever_id());
            return false;
        };
        // selfTest на ЛОКАЛЬНОМ ref'е: CallStaticIntMethodA инициализирует
        // класс (JVMS 5.5) — <clinit> выполняется на ЭТОМ вызове, поэтому
        // ENABLED уже прочитан и selfTest()==true достижим сразу после
        // define_class. Exception (ExceptionInInitializerError и т.п.) =
        // clear + false (fail-closed dormant).
        let ok = env
            .get_static_method_id(c, "selfTest", "()Z")
            .map(|mid| env.call_static_int_method(c, mid, &[]) != 0)
            .unwrap_or_else(|| {
                crate::clear_exception(env);
                false
            });
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        ok
    });
    defined.unwrap_or(false)
}

// ---------------------------------------------------------------------------
// STRICT-ОРАКУЛ (cargo test): бит-в-байт candidate-set = AABB-пересечения.
// ---------------------------------------------------------------------------

/// Ванильная AABB.intersects(AABB) — javap patched-kernel.jar 2025-12-11,
/// делегация intersects(DDDDDD)Z: строгие сравнения
/// (minX < o.maxX) && (maxX > o.minX) && (Y) && (Z);
/// dcmpg на min-плече и dcmpl на max-плече ⇒ NaN на любой стороне = false.
/// Rust `<`/`>` на NaN = false — бит-в-байт эквивалент на всём f64 домене.
#[inline]
pub fn aabb_intersects(
    minx: f64,
    miny: f64,
    minz: f64,
    maxx: f64,
    maxy: f64,
    maxz: f64,
    ominx: f64,
    ominy: f64,
    ominz: f64,
    omaxx: f64,
    omaxy: f64,
    omaxz: f64,
) -> bool {
    minx < omaxx
        && maxx > ominx
        && miny < omaxy
        && maxy > ominy
        && minz < omaxz
        && maxz > ominz
}

/// Секция сущности/бокса: vanilla floor(v / 16.0) (PersistentEntitySection
/// менеджер). Модель регистрации ChunkEntitySlices: entity регистрируется во
/// ВСЕ секции, покрываемые его AABB (addEntity @29-34: bySection add по
/// [floor(min/16), floor(max/16)]³).
#[inline]
fn section_of(v: f64) -> i32 {
    (v / 16.0).floor() as i32
}

/// Модель ванильного section-walk: candidate-set ПОСЛЕ точного per-кандидат
/// box-теста (EntityLookup.getEntities @155-185: intersects + predicate).
/// `regs` — секции, в которых зарегистрирована каждая сущность (ids).
fn vanilla_walk_candidates(
    entities: &[(usize, [f64; 3], [f64; 3])], // (id, min, max)
    box_min: [f64; 3],
    box_max: [f64; 3],
) -> Vec<usize> {
    let mut sections: std::collections::BTreeSet<(i32, i32, i32)> = Default::default();
    for s0 in section_of(box_min[0])..=section_of(box_max[0]) {
        for s1 in section_of(box_min[1])..=section_of(box_max[1]) {
            for s2 in section_of(box_min[2])..=section_of(box_max[2]) {
                sections.insert((s0, s1, s2));
            }
        }
    }
    let mut out = Vec::new();
    for (id, min, max) in entities {
        // Сущность видима walk'у только если одна из ЕЁ секций совпадает.
        let registered: std::collections::BTreeSet<(i32, i32, i32)> = (section_of(min[0])
            ..=section_of(max[0]))
            .flat_map(|a| {
                (section_of(min[1])..=section_of(max[1])).flat_map(move |b| {
                    (section_of(min[2])..=section_of(max[2])).map(move |c| (a, b, c))
                })
            })
            .collect();
        if registered.is_disjoint(&sections) {
            continue;
        }
        if aabb_intersects(
            min[0], min[1], min[2], max[0], max[1], max[2], box_min[0], box_min[1], box_min[2],
            box_max[0], box_max[1], box_max[2],
        ) {
            out.push(*id);
        }
    }
    out.sort_unstable();
    out
}

/// Модель fast path: per-кандидат AABB-тест по авторитетному списку (players).
fn fast_path_candidates(
    entities: &[(usize, [f64; 3], [f64; 3])],
    box_min: [f64; 3],
    box_max: [f64; 3],
) -> Vec<usize> {
    let mut out = Vec::new();
    for (id, min, max) in entities {
        if aabb_intersects(
            min[0], min[1], min[2], max[0], max[1], max[2], box_min[0], box_min[1], box_min[2],
            box_max[0], box_max[1], box_max[2],
        ) {
            out.push(*id);
        }
    }
    out.sort_unstable();
    out
}

#[cfg(test)]
mod queryplane_oracle_tests {
    use super::{aabb_intersects, fast_path_candidates, vanilla_walk_candidates};

    /// Детерминированный LCG (xorshift64*) — воспроизводимость оракула.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn f64(&mut self) -> f64 {
            (self.next() % 1_000_000) as f64 / 1_000_000.0
        }
        fn range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.f64()
        }
    }

    /// ОРАКУЛ: финальный candidate-set ванильного section-walk == fast-path
    /// candidate-set == {e : e.bb ∩ box}, бит-в-байт, на рандомизированной
    /// популяции (включая мульти-секционные сущности >16 блоков и боксы
    /// на границах секций).
    #[test]
    fn strict_oracle_candidate_set_eq_aabb_intersections() {
        let mut rng = Lcg(0x4D595354_41343132); // "MYSTA412" — детерминизм
        for case in 0..256u32 {
            let mut entities: Vec<(usize, [f64; 3], [f64; 3])> = Vec::new();
            for i in 0..48 {
                let cx = rng.range(-640.0, 640.0);
                let cy = rng.range(-60.0, 200.0);
                let cz = rng.range(-640.0, 640.0);
                let hw = rng.range(0.3, 9.0); // 9.0 > 8.0 → мульти-секционные
                let hh = rng.range(0.3, 9.0);
                entities.push((
                    i,
                    [cx - hw, cy - hh, cz - hw],
                    [cx + hw, cy + hh, cz + hw],
                ));
            }
            let bx0 = rng.range(-660.0, 620.0);
            let by0 = rng.range(-70.0, 190.0);
            let bz0 = rng.range(-660.0, 620.0);
            let ext = rng.range(0.0, 48.0);
            let box_min = [bx0, by0, bz0];
            let box_max = [bx0 + ext, by0 + ext, bz0 + ext];
            let v = vanilla_walk_candidates(&entities, box_min, box_max);
            let f = fast_path_candidates(&entities, box_min, box_max);
            assert_eq!(
                v, f,
                "case {case}: candidate-set divergence (section-walk vs fast path)"
            );
        }
    }

    /// Границы семантики (javap @0-63: СТРОГИЕ `<`/`>`): касание граней —
    /// НЕ пересечение; NaN на любой стороне — false (dcmpg/dcmpl контракт).
    #[test]
    fn vanilla_boundary_semantics_pinned() {
        // [0,1] против [1,2] по X: max=1 > omin=1 ЛОЖНО → не пересекаются.
        assert!(!aabb_intersects(0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0));
        // [0,1.0001]³ против [1,2]³ по всем осям (X/Y/Z строгие `>`): все три
        // max > omin → пересекаются (1.0001/1.5/1.5 vs 1/1/1).
        assert!(aabb_intersects(
            0.0, 0.0, 0.0, 1.0001, 1.5, 1.5, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0
        ));
        // Вложенный бокс — пересечение.
        assert!(aabb_intersects(
            0.0, 0.0, 0.0, 10.0, 10.0, 10.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0
        ));
        // NaN: java dcmpg(min,NaN)=1→ifge→false; rust NaN<x = false. Эквивалент.
        assert!(!aabb_intersects(
            f64::NAN, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0
        ));
        assert!(!aabb_intersects(
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, f64::NAN, 2.0, 2.0
        ));
        // ±0.0: java dcmpl/dcmpg считают -0.0 == 0.0; rust тоже (-0.0 < 0.0
        // = false) — вырожденный плоский бокс не даёт ложного пересечения.
        assert!(!aabb_intersects(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, -0.0, -0.0, 1.0, 1.0, 1.0));
    }

    /// Экономика плоскости (sanity): 48 кандидатов против 4 игроков —
    /// fast path O(players); oracle-модель не зависит от размера популяции.
    #[test]
    fn fast_path_is_independent_of_population_size() {
        let mut rng = Lcg(0x0B2E2F1D);
        let mut pop = Vec::new();
        for i in 0..48 {
            let cx = rng.range(-640.0, 640.0);
            let cz = rng.range(-640.0, 640.0);
            pop.push((i, [cx - 0.6, 60.0, cz - 0.6], [cx + 0.6, 62.0, cz + 0.6]));
        }
        let box_min = [0.0, 60.0, 0.0];
        let box_max = [16.0, 70.0, 16.0];
        let f = fast_path_candidates(&pop, box_min, box_max);
        let v = vanilla_walk_candidates(&pop, box_min, box_max);
        assert_eq!(f, v);
    }
}

#[cfg(test)]
mod queryplane_delivery_tests {
    use super::OPS_BYTES;

    /// Delivery-graph guard (collidebatch precedent): бридж определяется
    /// В ОДИНОЧКУ в kernel loader — вложенных классов быть не должно.
    #[test]
    fn queryplane_source_declares_no_nested_classes() {
        let src = include_str!("../queryplane/net/minecraft/world/entity/QueryPlaneOps.java");
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") && !t.starts_with('*') {
                        panic!("nested declaration in bridge source: {t}");
                    }
                }
            }
        }
    }

    /// Embedded bytes = реальный classfile, major 65 (--release 21).
    #[test]
    fn queryplane_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// Резолюшн-кложура (S7-164 guard) на встроенных байтах.
    #[test]
    fn queryplane_resolution_closure_accepts_embedded_bridge() {
        crate::classfile::queryplane_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded QueryPlaneOps bytes");
    }

    /// Roundtrip сайта 1 на РЕАЛЬНОМ kernel-классе (Level): тело
    /// редиректится (Retargeted), re-sight — AlreadyPatched, чужой класс —
    /// NotFound с нетронутыми байтами.
    #[test]
    fn level_geoc_redirect_roundtrip_on_kernel_fixture() {
        const LEVEL: &[u8] = include_bytes!("../tests/fixtures/Level_real.class");
        let (out, outcome) = crate::classfile::patch_level_get_entities_of_class(LEVEL)
            .expect("redirect must apply to the real Level fixture");
        assert!(matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
        ));
        assert_ne!(out, LEVEL.to_vec(), "body must actually change");
        let (_, again) = crate::classfile::patch_level_get_entities_of_class(&out)
            .expect("repatch must not error");
        assert!(matches!(
            again,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
        // Foreign class: fail closed.
        const FOREIGN: &[u8] = include_bytes!("../tests/fixtures/PalettedContainer.class");
        let (same, nf) = crate::classfile::patch_level_get_entities_of_class(FOREIGN)
            .expect("notfound path must not error");
        assert_eq!(nf, crate::classfile::RetargetOutcome::NotFound);
        assert_eq!(same, FOREIGN.to_vec());
    }

    /// Roundtrip сайта 2 на реальном Level + КОМПОЗИЦИЯ обоих редиректов
    /// (реальный порядок активации: один класс, два метода, последовательные
    /// патчи поверх друг друга) + idempotent re-sight обоих.
    #[test]
    fn level_hard_redirect_and_composition_roundtrip() {
        const LEVEL: &[u8] = include_bytes!("../tests/fixtures/Level_real.class");
        let (o1, out1) = crate::classfile::patch_level_get_entities_of_class(LEVEL)
            .expect("site 1 must apply");
        assert!(matches!(out1, crate::classfile::RetargetOutcome::Retargeted { .. }));
        let (o2, out2) = crate::classfile::patch_level_get_hard_colliding_entities(&o1)
            .expect("site 2 must compose on site-1 bytes");
        assert!(matches!(out2, crate::classfile::RetargetOutcome::Retargeted { .. }));
        // Re-sight обоих (retransform-циклы не портят).
        let (r1, out3) = crate::classfile::patch_level_get_entities_of_class(&o2)
            .expect("re-sight 1 must not error");
        assert!(matches!(
            out3,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
        let (_, out4) = crate::classfile::patch_level_get_hard_colliding_entities(&r1)
            .expect("re-sight 2 must not error");
        assert!(matches!(
            out4,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
    }

    /// Roundtrip сайта 3 на реальном ChunkEntitySlices: ровно один сайт,
    /// 5-байтный сплайс (0xb8 idx idx 00 00), idempotent re-sight, чужой
    /// класс NotFound.
    #[test]
    fn slices_probe_roundtrip_on_kernel_fixture() {
        const SLICES: &[u8] = include_bytes!("../tests/fixtures/ChunkEntitySlices_real.class");
        let (out, outcome) = crate::classfile::patch_slices_hard_probe(SLICES)
            .expect("probe must apply to the real ChunkEntitySlices fixture");
        assert!(
            matches!(outcome, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }),
            "exactly one site expected, got {outcome:?}"
        );
        // Length: pool growth APPEND-ONLY (3 Utf8 + Methodref + cp_count —
        // больше оригинала; методы не сдвигаются, код-регион той же длины).
        assert!(out.len() > SLICES.len(), "splice must grow only the pool");
        let (_, again) = crate::classfile::patch_slices_hard_probe(&out)
            .expect("re-sight must not error");
        assert!(matches!(
            again,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
        ));
        // Foreign class: fail closed.
        const FOREIGN: &[u8] = include_bytes!("../tests/fixtures/PalettedContainer.class");
        let (same, nf) = crate::classfile::patch_slices_hard_probe(FOREIGN)
            .expect("notfound path must not error");
        assert_eq!(nf, crate::classfile::RetargetOutcome::NotFound);
        assert_eq!(same, FOREIGN.to_vec());
    }

    /// STRICT-OR wiring guard (TASK-417-C, cvs-носитель): meganav-плоскости
    /// обязаны принимать ВСЕ композитные флаги носителя
    /// (cmp412_meganav || cmp414_cvs || cmp417_bq) — source-level защита
    /// от случайного реверта STRICT OR (гейт моей плоскости при этом STRICT OR
    /// {b2p1, mcomp, cmp417_bq}, см. lever_flag_matches).
    #[test]
    fn meganav_planes_accept_both_flags_strict_or() {
        for src in [
            include_str!("../src/nav_plane.rs"),
            include_str!("../src/tickplane.rs"),
            include_str!("../src/collide_batch.rs"),
            include_str!("../src/mobs_manager.rs"),
            include_str!("../src/stagger.rs"),
        ] {
            assert!(
                src.contains("cmp414_cvs"),
                "meganav plane lost the cvs-carrier arm (cmp414_cvs)"
            );
            assert!(
                src.contains("cmp417_bq"),
                "meganav plane lost the TASK-417-C composite arm (cmp417_bq)"
            );
            assert!(
                src.contains("cmp412_meganav"),
                "meganav plane lost its own flag"
            );
        }
        for src in [
            include_str!("../entityinside/net/minecraft/world/entity/ItemEntityManager.java"),
            include_str!("../mobai/net/minecraft/world/entity/MobAiOps.java"),
            include_str!("../mobpush/net/minecraft/world/entity/MobPushOps.java"),
            include_str!("../sscan/net/minecraft/world/entity/MobScanOps.java"),
            include_str!("../entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java"),
        ] {
            assert!(
                src.contains("cmp414_cvs"),
                "java meganav gate lost the cvs-carrier arm (cmp414_cvs)"
            );
            assert!(
                src.contains("cmp417_bq"),
                "java meganav gate lost the TASK-417-C composite arm (cmp417_bq)"
            );
        }
    }

    /// Гейт моей плоскости STRICT OR c cmp417_bq (пустой/чужой = ваниль);
    /// legacy b2p1-arms нетронуты (другие носители не затронуты).
    #[test]
    fn queryplane_gate_is_strict_or_c417bq() {
        let src = include_str!("queryplane.rs");
        // (self-include: гейт фиксируем литералами контракта;
        // lever_flag_matches читается активацией.)
        assert!(src.contains("\"cmp417_bq\""), "lost the cmp417_bq arm");
        assert!(src.contains("\"cmp412_b2p1\""), "lost the legacy b2p1 arm");
        // гейт НЕ принимает meganav-флаг (негатив — по стилю сравнения гейта,
        // чтобы wiring-guard-строки выше не ложились на self-include).
        assert!(!src.contains("== \"cmp412_meganav\""));
    }
}
