//! Runtime wiring for the F2 BRAIN-ITERATORS hook (family-agg pack member F2,
//! TASK-249/S7-113 -> S7-114; protocol: docs/FAMILY_AGG_PREREGISTRATION.md §5).
//!
//! The [`crate::randomtick`] pattern applied to `net.minecraft.world.entity.ai.Brain`:
//!
//! 1. Hook: once the kernel's `Brain` is loaded we define the lens helper
//!    (`BrainOps` + its two nested classes, ECJ-compiled against the kernel's
//!    class shapes, `randomtick/src/BrainOps.java`) into the SAME loader, then
//!    `READY` flips and `Brain` is retransformed. The byte hook fires on the
//!    retransform (or on a late original load) and swaps
//!    `startEachNonRunningBehavior`'s body for the straight-line
//!    `invokestatic BrainOps.startEachNonRunning(...)` delegation
//!    ([`crate::classfile::patch_brain_start_each`]).
//!
//!    NESTED-CLASS ORDER: `BrainOps$IdKey` and `BrainOps$Snapshot` are
//!    resolved lazily through BrainOps's defining loader on first `snapshot()`
//!    call. The kernel loader cannot find them on its classpath, so all three
//!    are defined HERE, inner classes first, before READY flips. (The parity
//!    bank ran plain-classpath so it never exercised this seam.)
//!
//! 2. Diagnostics: no in-process semantic self-test — driving
//!    `startEachNonRunningBehavior` end-to-end needs live mob AI (a booted
//!    server). The era runs INJECTS-ONLY (0 sandbox boots; CI boots are
//!    sanctioned and exercise the real path for free: every CI smoke with a
//!    world + entities runs the patched body). Parity contract is banked
//!    out-of-process: `research/f2-brainiter-2026-09-17/parity_output.txt`
//!    (4828 production-entry calls / 3083 order-exact start events / 1740
//!    mutations / 8 scenarios incl. LIVE-contains mid-call mutation,
//!    EQUALS-TRAP, fuzz 60x40). What we emit here is the grep-able marker
//!    chain (defined -> armed -> retransform rc -> PATCHED/failed) per hook
//!    convention.
//!
//! Nothing lands before the single aggregate A/B (§5 protocol): this module
//! only makes the pack buildable and CI-exercised; the verdict gate is the
//! A/B against the banked legal pair (76.01/76.98ms), owner can halt with
//! one word.

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const BRAIN_CLASS: &str = classfile::BRAIN_CLASS;
const OPS_NAME: &str = classfile::BRAIN_OPS_CLASS;

const OPS_BYTES: &[u8] =
    include_bytes!("../randomtick/build/net/minecraft/world/entity/ai/BrainOps.class");
const IDKEY_BYTES: &[u8] =
    include_bytes!("../randomtick/build/net/minecraft/world/entity/ai/BrainOps$IdKey.class");
const SNAPSHOT_BYTES: &[u8] =
    include_bytes!("../randomtick/build/net/minecraft/world/entity/ai/BrainOps$Snapshot.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// TASK-442-D (sense scope-expansion, round 2): TICK2 lane state — set by
/// activate() AFTER BrainOps.selfTestTickEach()==true in the kernel loader
/// (TASK-437-A pattern: selfTest BEFORE arm), read by the Brain hook when
/// composing the F2 baseline + tick2 body swap.
static TICK2_OK: AtomicBool = AtomicBool::new(false);
static TICK2_DECIDED: AtomicBool = AtomicBool::new(false);

/// STRICT-OR tick2 gate (round-400 lever protocol): CRUSSTY_SENSE env-hatch
/// (мандат cmp438_sense) ∨ вектор-флаг cmp438_sense ∨ КОМПОЗИТ
/// cmp439_sense_scan (sense⊕sscan2 STRICT-UNION носитель). ЗЕРКАЛО
/// mobs_sense.rs enabled() (расхождение = дормант-мисс ARM). Пустой/чужой
/// флаг = только F2-базлайн (ваниль tickEachRunningBehavior).
fn tick2_enabled() -> bool {
    if let Ok(h) = std::env::var("CRUSSTY_SENSE") {
        let h = h.trim().to_ascii_lowercase();
        if h == "1" || h == "true" || h == "on" || h == "yes" {
            return true;
        }
    }
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp438_sense") | Ok("cmp439_sense_scan") | Ok("cmp451_senseins") | Ok("cmp457_paldelta") // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
 || Ok("cmp438_sense") | Ok("cmp439_sense_scan") | Ok("cmp451_senseins") | Ok("cmp457_eqsnap2") // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
    )
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(BRAIN_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        // F2 baseline (always-on family-agg pack member) — unchanged bytes.
        let mut composed = match classfile::patch_brain_start_each(bytes) {
            Ok(b) => b,
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] brainhook: patch failed: {e}");
                return None;
            }
        };
        // TASK-442-D tick2 lane (gated): Brain.tickEachRunningBehavior ->
        // BrainOps.tickEachRunning, ONLY when the sense gate is active AND the
        // tick2 oracle passed in the kernel loader. Failure here NEVER disturbs
        // the F2 baseline (composed already carries it) — fail-closed.
        let tick2 = TICK2_OK.load(Ordering::Acquire);
        let want = TICK2_DECIDED.load(Ordering::Acquire) && tick2_enabled();
        if want {
            match classfile::patch_brain_tick_each(&composed) {
                Ok(b) => {
                    composed = b;
                    eprintln!(
                        "[crussty-plugin] brainhook: tick2 patched {name} tickEachRunningBehavior() ({} -> {} bytes; BrainOps.tickEachRunning flat mask lens; selfTestTickEach=true)",
                        bytes.len(),
                        composed.len()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] brainhook: tick2 patch rejected ({e}) — F2 stays, tickEachRunningBehavior vanilla"
                    );
                }
            }
        } else {
            static T2_OFF: AtomicBool = AtomicBool::new(false);
            if !T2_OFF.swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] brainhook: tick2 dormant (gate={}; oracle={}) — tickEachRunningBehavior vanilla",
                    tick2_enabled(),
                    tick2
                );
            }
        }
        eprintln!(
            "[crussty-plugin] brainhook: patched {name} startEachNonRunningBehavior() ({} -> {} bytes)",
            bytes.len(),
            composed.len()
        );
        Some(composed)
    });
}

/// Background activation: wait for the kernel Brain, define the helper
/// (nested classes first) into its loader, flip READY and retransform so the
/// hook applies the body swap. Brain loads with the first entity spawn on
/// every world boot, so the poll loop is a latency safety net and the
/// force-load an accelerator, not a liveness requirement.
pub fn activate() {
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(BRAIN_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] brainhook: {BRAIN_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // Accelerator: pull Brain in early through the kernel loader
            // (class LOAD only — cheap, no instantiation; the kernel does it
            // at first entity tick anyway).
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                if forced_attempts < 6 || forced_attempts % 12 == 0 {
                    forced_attempts += 1;
                    eprintln!(
                        "[crussty-plugin] brainhook: forcing kernel load of {BRAIN_CLASS} (attempt {forced_attempts})"
                    );
                }
                force_load_brain();
            }
            let sighted = cplug_sdk::classes::is_sighted(BRAIN_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(BRAIN_CLASS) else {
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
            // Nested classes FIRST (lazy resolution through BrainOps's
            // defining loader would otherwise hit the kernel classpath and
            // NoClassDefFoundError on first snapshot()).
            let mut ok = true;
            let mut ops_cls: Option<jni::jclass> = None;
            for (nm, bytes) in [
                (format!("{OPS_NAME}$IdKey"), IDKEY_BYTES),
                (format!("{OPS_NAME}$Snapshot"), SNAPSHOT_BYTES),
                (OPS_NAME.to_string(), OPS_BYTES),
            ] {
                match env.define_class(&nm, loader, bytes) {
                    Some(c) => {
                        if nm == OPS_NAME {
                            ops_cls = Some(c); // keep for the tick2 selfTest call
                        } else {
                            env.delete_local_ref(c);
                        }
                        eprintln!("[crussty-plugin] brainhook: defined {nm} in Brain loader");
                    }
                    None => {
                        crate::clear_exception(env);
                        eprintln!("[crussty-plugin] brainhook: define_class({nm}) failed");
                        ok = false;
                        break;
                    }
                }
            }
            // TASK-442-D tick2 lane: selfTestTickEach BEFORE arm (TASK-437-A
            // pattern — first active use of the just-defined class). The tick2
            // body swap is applied by the hook ONLY when this returned true
            // AND the sense gate is on. Fail-closed: false → lane vanilla.
            TICK2_DECIDED.store(true, Ordering::Release);
            if ok {
                if let Some(c) = ops_cls {
                    let ok2 = tick2_selftest(env, c);
                    TICK2_OK.store(ok2, Ordering::Release);
                    if ok2 {
                        eprintln!(
                            "[crussty-plugin] brainhook: tick2 selfTestTickEach=true (exhaustive mid-loop-stop oracle) BEFORE arm"
                        );
                    } else {
                        eprintln!(
                            "[crussty-plugin] brainhook: tick2 selfTestTickEach=false — tickEachRunningBehavior stays vanilla (fail-closed)"
                        );
                    }
                }
            }
            if let Some(c) = ops_cls {
                env.delete_local_ref(c);
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] brainhook: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(BRAIN_CLASS);
        eprintln!("[crussty-plugin] brainhook: hook armed, retransform rc={rc}");
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final one-line acceptance marker (TASK-22/C1 convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] brainhook: F2 ARMED (startEachNonRunningBehavior -> BrainOps.startEachNonRunning, flat-snapshot lens)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] brainhook: F2 NOT APPLIED after retransform (see patch-failed line above; kernel build mismatch?)"
            );
        }
        if tick2_enabled() {
            if TICK2_OK.load(Ordering::Acquire) {
                eprintln!(
                    "[crussty-plugin] {} : brain-tick2 ARMED (Brain.tickEachRunningBehavior -> BrainOps.tickEachRunning, per-brain RUNNING mask + flat snapshot, kills getRunningBehaviors ObjectArrayList+map walk; EFFECT marker 'sense tick2 EFFECT armed' on first gate hit)",
                    std::env::var("CRUSSTY_LEVER_FLAG")
                        .unwrap_or_else(|_| "(off)".to_string())
                        .trim()
                );
            } else {
                eprintln!(
                    "[crussty-plugin] brainhook: brain-tick2 NOT ARMED (selfTestTickEach=false or unverified) — lane vanilla"
                );
            }
        }
    });
}

/// tick2 oracle on the LOCAL ref of the just-defined BrainOps (mobs_sense.rs
/// sense_selftest pattern). Any pending exception = failure (fail-closed).
fn tick2_selftest(env: &jvmti_bindings::env::JniEnv, cls: jni::jclass) -> bool {
    let Some(mid) = env.get_static_method_id(cls, "selfTestTickEach", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] brainhook: selfTestTickEach method resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!(
            "[crussty-plugin] brainhook: selfTestTickEach threw (late resolution) — fail-closed"
        );
        return false;
    }
    rc != 0
}

/// Force-load Brain through the kernel loader (Bukkit-seeded forName),
/// the randomtick::force_load_server_level pattern verbatim with a different
/// target name.
fn force_load_brain() {
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        let Some(seed) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
            return None::<()>;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return None::<()>;
        };
        let Some(loader) = env
            .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
            .and_then(|mid| {
                let l = env.call_object_method(seed.as_jclass(), mid, &[]);
                (l as usize != 0).then_some(l)
            })
        else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] brainhook: force load: no kernel loader");
            return None::<()>;
        };
        let Some(forname) = env.get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        ) else {
            crate::clear_exception(env);
            return None::<()>;
        };
        let dot = BRAIN_CLASS.replace('/', ".");
        let Some(name) = env.new_string(&dot) else {
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jni::jvalue { l: name },
                jni::jvalue { z: 1 },
                jni::jvalue { l: loader },
            ],
        );
        let _ = crate::clear_exception(env);
        if loaded as usize != 0 {
            env.delete_local_ref(loaded);
            eprintln!("[crussty-plugin] brainhook: force load: {dot} loaded");
        }
        env.delete_local_ref(name);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(())
    });
}
