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

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(BRAIN_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match classfile::patch_brain_start_each(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] brainhook: patched {name} startEachNonRunningBehavior() ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] brainhook: patch failed: {e}");
                None
            }
        }
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
            for (nm, bytes) in [
                (format!("{OPS_NAME}$IdKey"), IDKEY_BYTES),
                (format!("{OPS_NAME}$Snapshot"), SNAPSHOT_BYTES),
                (OPS_NAME.to_string(), OPS_BYTES),
            ] {
                match env.define_class(&nm, loader, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
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
    });
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
