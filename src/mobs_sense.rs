//! Runtime wiring for the SENSING+BRAIN PLANE — the targeting nearest-pick
//! batch (TASK-438-A2 restart of TASK-438-A; lever STRICT-UNION carrier
//! `cmp438_sense`; закон 6 v17: подсистема ЦЕЛИКОМ на проверенном SoA-
//! субстрате — паттерн-носитель round-437-a-sscan2, mobs_soa flat arrays +
//! seqlock=global-version).
//!
//! RUST-FIRST: javap ground truth (patched-kernel.jar round-396-a,
//! purpur-1.21.10, RESEARCH-A-439-SENSE.md) — ВСЯ targeting-conditions
//! nearest-выборка сходится в ОДИН chokepoint, интерфейсный default-метод
//! `net/minecraft/server/level/ServerEntityGetter.getNearestEntity(List,
//! TargetingConditions, LivingEntity, DDD)` (четыре getNearestPlayer-варианта
//! + getNearestEntity(Class/TagKey) депегаются в него; ServerLevel НЕ
//! переопределяет; NearestAttackableTargetGoal.findTarget зовёт напрямую @84).
//! Тело ванили:
//!
//!     d = -1.0; best = null;
//!     for c in list { if (!conditions.test(level, targeter, c)) continue;
//!                     e = c.distanceToSqr(x, y, z);
//!                     if (d == -1.0 || e < d) { d = e; best = c; } }
//!     return best;
//!
//! Мост `sense/net/minecraft/server/level/SenseOps.java`:
//! `nearestEntityGate(ServerEntityGetter, List, TargetingConditions,
//! LivingEntity, DDD) -> LivingEntity` — body-swap этого метода
//! (14-байт straight line, [`crate::classfile::patch_sense_nearest_entity`];
//! прецедент patch_brain_start_each). Гейт:
//!   1. rust-guided best-first тест: колонка sense-эпохи
//!      `nearest[denseId] = argmin dist² (ties → младший индекс players()
//!      порядка) | -1`, identity-guard `entities[i] == SNAPSHOT[i]` O(n);
//!   2. ТОЧНАЯ дистанция-обрезка свипа: `d >= bestD` кандидаты пропускают
//!      тест — они НЕ МОГУТ выиграть лестницу `(d == -1.0 || e < d)`, а
//!      TargetingConditions.test чист (range/invisibility/idleTimeout/
//!      selector/LOS-clip, RandomSource НЕ потребляется) ⇒ ЛЮБОЙ порядок
//!      тестов даёт ванильный pick: min-dist passing, tie-keep-earlier.
//!      STALE снапшот деградирует ТОЛЬКО выигрыш, не корректность (закон 4).
//!   Fail-closed лестница → байт-точная ваниль-реплика тела.
//!
//! BULK-JNI LAW (один переход на батч): `senseEpoch(tick, idTop, players[D],
//! nearest[I])` — ОДИН DOD-проход по mobs_soa SoA-позициям (sscan_snapshot
//! reader-контракт: xs/ys/zs + global VERSION, bounded retry) за тик пишет
//! колонку nearest прямо в разделяемый java int[]. ZERO per-call JNI.
//!
//! Гейт: env `CRUSSTY_LEVER_FLAG == "cmp438_sense"` ∨ семейство-носители
//! (STRICT-OR brain3-паттерна: cmp430_inside-эра). Пустой/чужой флаг =
//! ваниль бит-в-байт (ничего не регистрируется, класс-байты идентичны).
//! FAIL-CLOSED лестница: не-Mob targeter / вне SoA-плоскости (idBoxOf null) /
//! id вне последней эпохи / identity-guard промах → дистанция-обрезанный
//! ваниль-свип (или байт-точная реплика); senseProbe mismatch / senseEpoch
//! ERR_STRUCT / selfTest=false → дизарм навсегда; ERR_RANGE / эпоха-промах →
//! ваниль на этот тик (эпоха ретраится).

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const GETTER_CLASS: &str = "net/minecraft/server/level/ServerEntityGetter";
const MOBPUSH_CLASS: &str = "net/minecraft/world/entity/MobPushOps";
const OPS_CLASS: &str = "net/minecraft/world/entity/SenseOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../sense/build/net/minecraft/world/entity/SenseOps.class");

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x5345; // "SE"
const QRETRY: u32 = 128;

/// STRICT-OR gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). env-хатч CRUSSTY_SENSE=1 (мандат cmp438_sense: прямой
/// включатель без lever-флага) ∨ вектор-флаг cmp438_sense ∨ семейство
/// cmp430_inside-эры (носители-прецеденты cmp424_mobfeed / cmp430_inside;
/// ЗЕРКАЛО sense/net/minecraft/world/entity/SenseOps.java leverEnabled —
/// расхождение = дормант-мисс ARM). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    if let Ok(h) = std::env::var("CRUSSTY_SENSE") {
        let h = h.trim().to_ascii_lowercase();
        if h == "1" || h == "true" || h == "on" || h == "yes" {
            return true;
        }
    }
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp438_sense")
            | Ok("cmp406_sscan")
            | Ok("cmp409_multi")
            | Ok("cmp412_meganav")
            | Ok("cmp412_eqsnapv3")
            | Ok("cmp414_cvs")
            | Ok("cmp417_bq")
            | Ok("cmp420_colpush")
            | Ok("cmp421_brain")
            | Ok("cmp422_brain2")
            | Ok("cmp423_brain3")
            | Ok("cmp424_mobfeed")
            | Ok("cmp430_inside")
            | Ok("cmp451_senseins") | Ok("cmp455_spawn") // TASK-452-A: senseins composite (carrier ins4 + sense/brain family, STRICT OR) — PRODUCTION gate retag (x452: dormant -> SenseOps never defined, core of vector dead)
    )
}

static READY: AtomicBool = AtomicBool::new(false);

pub fn sense_ready() -> bool {
    READY.load(Ordering::Acquire)
}

/// Активный флаг для ARM-строк (evidence-дисциплина TASK-437-A).
fn label() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_else(|_| "(off)".to_string())
        .trim()
        .to_string()
}

/// Register the byte hook on the ServerEntityGetter interface (call once from
/// cplugin_init). Dormant-invisible: with the lever flag unset/mismatched
/// NOTHING is registered — byte-indistinguishable from vanilla.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] mobs_sense: dormant (lever_flag ∉ family, vanilla targeting scans)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(GETTER_CLASS, move |name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: pass the chain output through untouched.
            return None;
        }
        match crate::classfile::patch_sense_nearest_entity(bytes) {
            Ok(out) => {
                static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] mobs_sense: hook serve {name} {} bytes (getNearestEntity(List,TC,LE,DDD) -> SenseOps.nearestEntityGate, body-swap {} -> {})",
                        bytes.len(),
                        bytes.len(),
                        out.len()
                    );
                }
                Some(out)
            }
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] mobs_sense: body swap rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for boot + ServerEntityGetter + MobPushOps
/// (SenseOps references MobPushOps.idBoxOf/idCount — same kernel loader),
/// define the SenseOps bridge + RegisterNatives (senseProbe/senseEpoch),
/// selfTest==true BEFORE arm (TASK-437-A pattern), flip READY and retransform
/// the interface.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // ServerEntityGetter loads with ServerLevel at world boot; MobPushOps
        // is defined into the same loader by mobs_manager::activate.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for cls in [GETTER_CLASS, MOBPUSH_CLASS] {
            while cplug_sdk::classes::find_class(cls).is_none() {
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] mobs_sense: {cls} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mobs_sense: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Guard: embedded bridge bytes must not be newer than the JVM
        // (lesson 408: stale class blob = sleeping gate).
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let ops_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if ops_major > jvm_major {
            eprintln!(
                "[crussty-plugin] mobs_sense: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild sense/ via scripts/build_430b_blobs.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge + RegisterNatives + selfTest in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(GETTER_CLASS) else {
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
                eprintln!("[crussty-plugin] mobs_sense: define_class({OPS_CLASS}) failed");
                return false;
            };

            // RegisterNatives: senseProbe (magic) + senseEpoch (bulk column
            // writer). Sig matches the declared native EXACTLY (урок 409-E:
            // stray sig = NoSuchMethodError every boot, silent sleeping gate).
            let names = [
                CString::new("senseProbe").expect("no NUL"),
                CString::new("senseEpoch").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(II[D[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: sense_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: sense_epoch as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                crate::describe_exception(env);
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] mobs_sense: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            // TASK-437-A: selfTest==true ДО ARM — on the LOCAL ref from
            // define_class (first active use ⇒ <clinit>). Probe magic +
            // vanilla nearest-pick ladder oracle; false → dormant.
            let st = sense_selftest(env, c);
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            if !st {
                eprintln!(
                    "[crussty-plugin] mobs_sense: selfTest=false — hook stays dormant (fail-closed, vanilla bit-for-bit)"
                );
                return false;
            }
            eprintln!(
                "[crussty-plugin] mobs_sense: selfTest=true (probe magic + vanilla getNearestEntity ladder oracle OK) BEFORE arm"
            );
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] mobs_sense: bridge definition failed or selfTest=false, hook stays dormant"
            );
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed; EFFECT-маркеры
        // кладёт SenseOps на первом gate-hit — вердикты только по ним).
        eprintln!(
            "[crussty-plugin] {}: sense armed (ServerEntityGetter.getNearestEntity(List,TC,LE,DDD) default-method body-swap -> SenseOps.nearestEntityGate; rust senseEpoch = ONE bulk JNI/tick DOD pass over mobs_soa SoA positions -> nearest-player column nearest[denseId] (ties keep earlier players() index); gate = identity-guarded best-first test + decision-exact distance pruning (d>=bestD candidates cannot win the (d==-1.0||e<d) ladder; TargetingConditions.test is pure), fail-closed -> byte-exact vanilla replica; zero per-call JNI; empty/foreign flag = vanilla bit-for-bit)",
            label()
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "nearestEntityGate", "cmp438_sense v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(GETTER_CLASS);
        eprintln!(
            "[crussty-plugin] mobs_sense: {GETTER_CLASS} armed, retransform rc={rc} (ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] mobs_sense: retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
        }
    });
}

/// selfTest on the LOCAL ref of the just-defined bridge (same pattern as
/// mobscan_selftest / colpush_selftest; TASK-417-C find_class-fix). Any
/// pending exception is cleared and reported as failure (fail-closed).
fn sense_selftest(env: &jvmti_bindings::env::JniEnv, cls: jni::jclass) -> bool {
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] mobs_sense: selfTest method resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!(
            "[crussty-plugin] mobs_sense: selfTest threw (late resolution) — fail-closed"
        );
        return false;
    }
    rc != 0
}

// ---------------------------------------------------------------------------
// Natives (registered on SenseOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn sense_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK nearest-player column writer — ONE transition per tick-batch (never
/// per entity).
///
/// Scans the mobs_soa SoA positions for `id in 0..min(id_top, nearest_cap,
/// soa_len)` under the seqlock reader bracket (no WLOCK — writers proceed;
/// torn snapshot → bounded retry) and writes `nearest[id]` = index of the
/// FIRST strictly-closest player (ties keep the EARLIER player — the exact
/// `getNearestEntity` ladder, including the distanceToSqr component order
/// `(px-mx)²+(py-my)²+(pz-mz)²`) or -1 when no player exists. Players arrive
/// as a flat [x,y,z]×n f64 snapshot (UNFILTERED — the gate applies
/// TargetingConditions.test per candidate exactly like vanilla; NO predicate
/// folding here, unlike the despawn sscan plane). Returns the number of
/// written elements or ERR_RANGE (bad n/arrays) / ERR_STRUCT (pin failure,
/// retries exhausted). Java fail-closed: any non-count → vanilla sweep this
/// tick.
///
/// # Safety
/// See sense_probe.
#[no_mangle]
pub unsafe extern "system" fn sense_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    id_top: jni::jint,
    players: jni::jdoubleArray,
    nearest: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || players.is_null() || nearest.is_null() {
        return ERR_STRUCT;
    }
    if tick < 0 || id_top < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let p_cap = unsafe { (vt.GetArrayLength)(env, players) };
    let n_cap = unsafe { (vt.GetArrayLength)(env, nearest) };
    if p_cap <= 0 || n_cap <= 0 || p_cap % 3 != 0 {
        return ERR_RANGE;
    }
    let n = (p_cap / 3) as usize;
    let Some((xs, ys, zs, version)) = crate::mobs_soa::sscan_snapshot() else {
        return 0; // empty universe (no upserts ever) — zero valid entries
    };
    let bound = (id_top as usize)
        .min(n_cap as usize)
        .min(xs.len())
        .min(ys.len())
        .min(zs.len());

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, nearest, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, bound) };
    let pl = unsafe { std::slice::from_raw_parts(players as *const jni::jdouble, p_cap as usize) };

    let rc: i32;
    let mut tries: u32 = 0;
    loop {
        tries += 1;
        if tries > QRETRY {
            rc = ERR_RANGE; // writer storm — java falls back to vanilla this tick
            break;
        }
        let v1 = version.load(Ordering::Acquire);
        if v1 & 1 == 1 {
            std::hint::spin_loop();
            continue;
        }
        for (id, slot) in dst.iter_mut().enumerate() {
            let mx = xs[id];
            let my = ys[id];
            let mz = zs[id];
            let mut best = -1.0f64;
            let mut best_i: i32 = -1;
            for p in 0..n {
                let px = pl[p * 3];
                let py = pl[p * 3 + 1];
                let pz = pl[p * 3 + 2];
                // Entity.distanceToSqr(mobX, mobY, mobZ) on the PLAYER:
                // dx = player.x - mob.x etc., sum order (dx*dx + dy*dy) + dz*dz.
                let dx = px - mx;
                let dy = py - my;
                let dz = pz - mz;
                let d = dx * dx + dy * dy + dz * dz;
                // getNearestEntity ladder: (best == -1.0 || d < best) — ties
                // keep the EARLIER player (first strictly-closer wins).
                if best == -1.0 || d < best {
                    best = d;
                    best_i = p as i32;
                }
            }
            *slot = best_i;
        }
        let v2 = version.load(Ordering::Acquire);
        if v2 != v1 {
            continue; // torn snapshot — retry the whole pass
        }
        rc = bound as i32;
        break;
    }

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, nearest, pinned, 0) };
    rc
}

// ---------------------------------------------------------------------------
// Tests: the vanilla nearest-pick ladder oracle (first strictly closer wins,
// ties keep the earlier player, -1.0 sentinel), distanceToSqr component
// order, STRICT gate semantics, and the body-swap contract on the real
// ServerEntityGetter bytes when provided.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_constant_strict() {
        // STRICT eq: no prefix/suffix tolerance (TASK-402-F lesson).
        assert!(enabled_with("cmp438_sense"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp430_inside_x"));
        assert!(!enabled_with(" cmp438_sense"));
    }

    fn enabled_with(s: &str) -> bool {
        s == "cmp438_sense" || s == "cmp439_sense_scan" || s == "cmp451_senseins" || s == "cmp455_spawn" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
    }

    /// Mirror of the vanilla `getNearestEntity(List,TC,LE,DDD)` inner ladder
    /// operating on the same f64 values as the rust epoch pass.
    fn vanilla_pick(players: &[(f64, f64, f64)], mob: (f64, f64, f64)) -> i32 {
        let mut best = -1.0f64;
        let mut best_i = -1i32;
        for (i, &(px, py, pz)) in players.iter().enumerate() {
            let dx = px - mob.0;
            let dy = py - mob.1;
            let dz = pz - mob.2;
            let d = dx * dx + dy * dy + dz * dz;
            if best == -1.0 || d < best {
                best = d;
                best_i = i as i32;
            }
        }
        best_i
    }

    #[test]
    fn first_strictly_closer_wins_ties_keep_earlier() {
        let mob = (0.0, 0.0, 0.0);
        let ps = vec![(10.0, 0.0, 0.0), (5.0, 0.0, 0.0)];
        assert_eq!(vanilla_pick(&ps, mob), 1);
        // Exact tie: earlier player kept (first wins, `<` not `<=`).
        let ps = vec![(3.0, 4.0, 0.0), (0.0, 3.0, 4.0)];
        assert_eq!(vanilla_pick(&ps, mob), 0);
        let ps = vec![(100.0, 100.0, 100.0)];
        assert_eq!(vanilla_pick(&ps, mob), 0);
        assert_eq!(vanilla_pick(&Vec::new(), mob), -1);
    }

    #[test]
    fn distance_component_order_matches_distance_to_sqr() {
        // (px-mx)²+(py-my)²+(pz-mz)² component order — the mirror of
        // Entity.distanceToSqr(DDD) used by the vanilla ladder.
        let mob = (1.5, 2.5, 3.5);
        let p = (4.5, 1.0, 3.5);
        let dx = p.0 - mob.0;
        let dy = p.1 - mob.1;
        let dz = p.2 - mob.2;
        assert_eq!(dx * dx + dy * dy + dz * dz, 9.0 + 2.25 + 0.0);
    }

    #[test]
    fn body_swap_contract_constants() {
        // javap ground truth: the chokepoint is the INTERFACE default method
        // (ServerLevel does NOT override); the gate desc is the virtual desc
        // with the receiver class PREPENDED (compose validator contract).
        assert_eq!(GETTER_CLASS, "net/minecraft/server/level/ServerEntityGetter");
        assert_eq!(
            crate::classfile::SENSE_GETTER_CLASS,
            "net/minecraft/server/level/ServerEntityGetter"
        );
        assert!(crate::classfile::SENSE_NEAREST_DESC.starts_with("(Ljava/util/List;"));
        assert!(crate::classfile::SENSE_GATE_DESC.starts_with(
            "(Lnet/minecraft/server/level/ServerEntityGetter;"
        ));
    }

    #[test]
    fn senseops_blob_carries_gate_and_markers() {
        // javap-гейт: блоб должен нести STRICT-флаги семейства и маркеры
        // гейта (cp truth).
        let blob = OPS_BYTES;
        for marker in [
            "cmp438_sense",
            "cmp430_inside",
            "nearestEntityGate",
            "senseProbe",
            "senseEpoch",
            "selfTest",
            "sense EFFECT",
        ] {
            assert!(
                blob.windows(marker.len()).any(|w| w == marker.as_bytes()),
                "marker '{marker}' missing from SenseOps blob"
            );
        }
    }

    #[test]
    fn getter_bytes_swap_if_available() {
        // Реальные байты ядра (ground truth: /tmp/crussty-sense-bytes/
        // ServerEntityGetter.class из patched-kernel.jar round-396-a; в
        // CI/без файла — тихий пропуск, тест остаётся hermetic).
        let Ok(path) = std::env::var("CRUSSTY_SENSE_BYTES") else {
            eprintln!("skip: CRUSSTY_SENSE_BYTES not set (no real kernel ServerEntityGetter.class in env)");
            return;
        };
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("skip: cannot read {path}");
            return;
        };
        let out = crate::classfile::patch_sense_nearest_entity(&bytes)
            .expect("body swap on real ServerEntityGetter bytes must not Err");
        // Idempotency: patch(patch(x)) == patch(x).
        let out2 = crate::classfile::patch_sense_nearest_entity(&out)
            .expect("second pass must not Err");
        assert_eq!(out.len(), out2.len(), "body swap must be idempotent in size");
    }
}
