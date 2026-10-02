//! Runtime wiring for the F1 BATCH-RNG hook (family-agg pack member F1,
//! TASK-247/S7-111 -> S7-112; protocol: docs/FAMILY_AGG_PREREGISTRATION.md §5).
//!
//! Two halfs, the [`crate::area_map`] pattern applied to `ServerLevel`:
//!
//! 1. Hook: once the kernel's `ServerLevel` is loaded we define the
//!    `RandomTickOps` helper (compiled against the kernel's class shapes,
//!    `randomtick/src/RandomTickOps.java`) into the SAME loader, then `READY`
//!    flips and the class is retransformed. The byte hook fires on the
//!    retransform (or on a late original load) and swaps
//!    `optimiseRandomTick`'s body for the straight-line
//!    `invokestatic RandomTickOps.run(...)` delegation
//!    ([`crate::classfile::patch_optimise_random_tick`]).
//!
//!    The helper must live in ServerLevel's loader, NOT the bootstrap: its
//!    bytecode references `ServerLevel`/`LevelChunk`/moonrise classes
//!    directly, and a bootstrap-defined copy would both fail to resolve the
//!    kernel classes and shadow them for the kernel's own (parent-first)
//!    loader.
//!
//! 2. Diagnostics: this hook has NO in-process semantic self-test — driving
//!    `optimiseRandomTick` end-to-end needs a ticking world (a booted
//!    server), and the era runs INJECTS-ONLY (0 sandbox boots; CI boots are
//!    sanctioned and exercise the real path for free: every CI smoke that
//!    loads a world runs the patched body on live chunks). The parity
//!    contract is banked out-of-process instead:
//!    `research/f1-batchrng-2026-09-17/parity_output.txt` (320,000 attempts
//!    / 8 seeds / 160,379 hit interleaves incl. nextGaussian, on REAL kernel
//!    classes, plain JVM). What we DO emit here is the grep-able marker
//!    chain (defined -> armed -> retransform rc -> PATCHED/failed) the era's
//!    hook convention requires.
//!
//! Nothing lands before the single aggregate A/B (§5 protocol): this module
//! only makes the pack buildable and CI-exercised; the verdict gate is the
//! A/B against the banked legal pair (76.01/76.98ms), owner can halt with
//! one word.

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const SERVER_LEVEL_CLASS: &str = classfile::SERVER_LEVEL_CLASS;
const OPS_NAME: &str = classfile::RANDOMTICK_OPS_CLASS;

const OPS_BYTES: &[u8] =
    include_bytes!("../randomtick/build/net/minecraft/server/level/RandomTickOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(SERVER_LEVEL_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match classfile::patch_optimise_random_tick(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] randomtick: patched {name} optimiseRandomTick() ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] randomtick: patch failed: {e}");
                None
            }
        }
    });
}

/// Background activation: wait for the kernel ServerLevel, define the helper
/// into its loader, then flip READY and retransform so the hook applies the
/// body swap. ServerLevel loads during world creation on every boot (never
/// idle-dormant like SingleUserAreaMap), so the poll loop is a latency
/// safety net and the force-load an accelerator, not a liveness requirement.
pub fn activate() {
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(SERVER_LEVEL_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] randomtick: {SERVER_LEVEL_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // Accelerator: as soon as the kernel loader is reachable through
            // Bukkit, pull ServerLevel in early (class LOAD only — cheap,
            // no instantiation; the kernel does it during world creation
            // anyway, this just front-runs the 180s deadline on fast boots).
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                if forced_attempts < 6 || forced_attempts % 12 == 0 {
                    forced_attempts += 1;
                    eprintln!(
                        "[crussty-plugin] randomtick: forcing kernel load of {SERVER_LEVEL_CLASS} (attempt {forced_attempts})"
                    );
                }
                force_load_server_level();
            }
            let sighted = cplug_sdk::classes::is_sighted(SERVER_LEVEL_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(SERVER_LEVEL_CLASS) else {
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
            let ok = match env.define_class(OPS_NAME, loader, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] randomtick: defined {OPS_NAME} in ServerLevel loader");
                    true
                }
                None => {
                    crate::clear_exception(env);
                    eprintln!("[crussty-plugin] randomtick: define_class({OPS_NAME}) failed");
                    false
                }
            };
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] randomtick: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(SERVER_LEVEL_CLASS);
        eprintln!("[crussty-plugin] randomtick: hook armed, retransform rc={rc}");
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final one-line acceptance marker (TASK-22/C1 convention: one
        // final state line per hook).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] randomtick: F1 ARMED (optimiseRandomTick -> RandomTickOps.run, bit-exact LCG batch)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] randomtick: F1 NOT APPLIED after retransform (see patch-failed line above; kernel build mismatch?)"
            );
        }
    });
}

/// Force-load ServerLevel through the kernel loader (Bukkit-seeded forName),
/// the area_map::force_load_kernel_class pattern verbatim with a different
/// target name.
fn force_load_server_level() {
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
            eprintln!("[crussty-plugin] randomtick: force load: no kernel loader");
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
        let dot = SERVER_LEVEL_CLASS.replace('/', ".");
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
            eprintln!("[crussty-plugin] randomtick: force load: {dot} loaded");
        }
        env.delete_local_ref(name);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(())
    });
}
