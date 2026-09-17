//! Runtime wiring for the F3 LEVELTICKS-READS hooks (family-agg pack member
//! F3, TASK-251/S7-115 -> S7-116; protocol: docs/FAMILY_AGG_PREREGISTRATION.md
//! §5). The [`crate::brainhook`] pattern applied TWICE:
//!
//! 1. `LevelTicks.runCollectedTicks` -> [`crate::classfile::patch_run_collected_ticks`]
//! 2. `ServerLevel.tickBlock`        -> [`crate::classfile::patch_tick_block`]
//!
//! Both helpers are ONE class (`TickBlockOps`, no nested classes — unlike
//! BrainOps) defined into the kernel loader; the drain window
//! (ThreadLocal section cache) is opened by runCollectedTicks and consulted
//! by tickBlock, so the pair only makes sense together: READY gates BOTH
//! hooks, one define arms the pack member.
//!
//! F1 COHABITATION (the ServerLevel seam): F1's hook
//! ([`crate::randomtick`]) and our hook #2 are chained on the same class
//! name (cplug-sdk `dispatch_bytes` applies every matching callback in
//! registration order). JVMTI retransformation feeds the chain the ORIGINAL
//! class bytes each time, so a tickBlock-only image would silently UNDO the
//! F1 optimiseRandomTick swap (F1's one-shot PATCHED guard makes its
//! callback return None on every later dispatch). Our ServerLevel callback
//! therefore RE-APPLIES the F1 patch (proven idempotent:
//! patch(patch(x)) == patch(x), dedup CP appends) before its own patch —
//! any dispatch order converges to the same both-patched image
//! (classfile::real_noise::f3_serverlevel_composes_with_f1).
//!
//! Diagnostics: no in-process semantic self-test — driving the drain
//! end-to-end needs a ticking world (a booted server). INJECTS-ONLY: CI
//! boots are sanctioned and exercise the real path for free; the parity
//! contract is banked out-of-process
//! (research/f3-levelticks-2026-09-17/parity_output.txt, S1-S9 incl. the
//! vanilla set-removal QUIRK and the RESOLUTIONS REF=6 -> NEW=2 win). What
//! we emit here is the grep-able marker chain (defined -> armed ->
//! retransform rc -> PATCHED/failed) per hook convention.
//!
//! Nothing lands before the single aggregate A/B (§5 protocol): this module
//! only makes the pack buildable and CI-exercised; the verdict gate is the
//! A/B against the banked legal pair (76.01/76.98ms), owner can halt with
//! one word.

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const LEVELTICKS_CLASS: &str = classfile::LEVELTICKS_CLASS;
const SERVER_LEVEL: &str = classfile::SERVER_LEVEL_CLASS;
const OPS_NAME: &str = classfile::TICKBLOCK_OPS_CLASS;

const OPS_BYTES: &[u8] =
    include_bytes!("../randomtick/build/net/minecraft/server/level/TickBlockOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED_LT: AtomicBool = AtomicBool::new(false);
static PATCHED_SL: AtomicBool = AtomicBool::new(false);

/// Register both byte hooks (idempotent; call once from cplugin_init).
/// Registration ORDER matters only for log readability: randomtick::register
/// runs first (lib.rs), so on the one dispatch where both F1 and F3 patch
/// fresh, F1 logs first — the composed image is order-independent.
pub fn register() {
    // Hook 1: LevelTicks — BOTH F3 LevelTicks bodies (runCollectedTicks drain
    // swap + collectTicks queue swap) composed in ONE callback: the retransform
    // feeds ORIGINAL bytes, so each dispatch re-applies both (idempotent).
    cplug_sdk::hooks::register_bytes(LEVELTICKS_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED_LT.swap(true, Ordering::SeqCst) {
            return None;
        }
        match (|| -> Result<Vec<u8>, String> {
            let b = classfile::patch_run_collected_ticks(bytes)?;
            classfile::patch_collect_ticks(&b)
        })() {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] tickhook: patched {name} runCollectedTicks()+collectTicks() ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED_LT.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] tickhook: LevelTicks patch failed: {e}");
                None
            }
        }
    });

    // Hook 2: ServerLevel.tickBlock — COMPOSES with F1 (see module docs):
    // re-apply optimiseRandomTick first (idempotent), then our body swap.
    cplug_sdk::hooks::register_bytes(SERVER_LEVEL, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED_SL.swap(true, Ordering::SeqCst) {
            return None;
        }
        match (|| -> Result<Vec<u8>, String> {
            // JVMTI retransform hands us the ORIGINAL bytes; F1's own
            // callback stays silent on later dispatches (one-shot guard), so
            // WE keep the optimiseRandomTick swap alive here.
            let b = classfile::patch_optimise_random_tick(bytes)?;
            classfile::patch_tick_block(&b)
        })() {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] tickhook: patched {name} tickBlock() ({} -> {} bytes, F1 composed)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED_SL.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] tickhook: tickBlock patch failed: {e}");
                None
            }
        }
    });
}

/// Background activation: wait for BOTH kernel targets (ServerLevel loads
/// during world creation, LevelTicks during its construction — plus a
/// force-load accelerator for both), define the helper into the kernel
/// loader, flip READY and retransform both classes so the hooks apply the
/// body swaps.
pub fn activate() {
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            let sl = cplug_sdk::classes::find_class(SERVER_LEVEL).is_some();
            let lt = cplug_sdk::classes::find_class(LEVELTICKS_CLASS).is_some();
            if sl && lt {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] tickhook: targets not loaded within 180s (ServerLevel={sl} LevelTicks={lt}), hook stays dormant"
                );
                return;
            }
            // Accelerator: pull both targets in early through the kernel
            // loader (class LOAD only — cheap, no instantiation; the kernel
            // loads LevelTicks while constructing the first ServerLevel
            // anyway).
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                forced_attempts += 1;
                if forced_attempts <= 6 || forced_attempts % 12 == 0 {
                    eprintln!(
                        "[crussty-plugin] tickhook: forcing kernel load (attempt {forced_attempts}, ServerLevel={sl} LevelTicks={lt})"
                    );
                }
                if !sl {
                    force_load(SERVER_LEVEL);
                }
                if !lt {
                    force_load(LEVELTICKS_CLASS);
                }
            }
            let sighted =
                cplug_sdk::classes::is_sighted(SERVER_LEVEL) || cplug_sdk::classes::is_sighted(LEVELTICKS_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(SERVER_LEVEL) else {
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
            // SINGLE class, no nested trio (unlike BrainOps) — TickBlockOps's
            // own references (LevelTicks/ServerLevel/BlockPos/Block/moonrise)
            // all resolve through the kernel loader parent.
            let ok = match env.define_class(OPS_NAME, loader, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] tickhook: defined {OPS_NAME} in kernel loader");
                    true
                }
                None => {
                    crate::clear_exception(env);
                    eprintln!("[crussty-plugin] tickhook: define_class({OPS_NAME}) failed");
                    false
                }
            };
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] tickhook: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        // Retransform both targets: LevelTicks first (hook 1), then
        // ServerLevel (hook 2 composes F1 back in). The order is not a
        // correctness requirement (the compose is order-independent), it
        // just makes the marker lines read causally.
        let rc_lt = cplug_sdk::retransform_class(LEVELTICKS_CLASS);
        eprintln!("[crussty-plugin] tickhook: LevelTicks retransform rc={rc_lt}");
        let rc_sl = cplug_sdk::retransform_class(SERVER_LEVEL);
        eprintln!("[crussty-plugin] tickhook: ServerLevel retransform rc={rc_sl}");
        // Give the synchronous ClassFileLoadHook callbacks a beat, then emit
        // the final one-line acceptance markers (TASK-22/C1 convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED_LT.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] tickhook: F3 ARMED (runCollectedTicks -> TickBlockOps.runCollectedTicks, byte-exact drain mirror + section-cache window)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] tickhook: F3 NOT APPLIED on LevelTicks (see patch-failed line above; kernel build mismatch?)"
            );
        }
        if PATCHED_SL.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] tickhook: F3 ARMED (tickBlock -> TickBlockOps.tickBlock, readBlockState lens, F1 composed)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] tickhook: F3 NOT APPLIED on ServerLevel (see patch-failed line above; kernel build mismatch?)"
            );
        }
    });
}

/// Force-load a kernel class by name through the kernel loader
/// (Bukkit-seeded forName; the randomtick::force_load_server_level pattern
/// parameterized — class LOAD only, no instantiation).
fn force_load(name: &str) {
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
            eprintln!("[crussty-plugin] tickhook: force load: no kernel loader");
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
        let dot = name.replace('/', ".");
        let Some(jname) = env.new_string(&dot) else {
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jni::jvalue { l: jname },
                jni::jvalue { z: 1 },
                jni::jvalue { l: loader },
            ],
        );
        let _ = crate::clear_exception(env);
        if loaded as usize != 0 {
            env.delete_local_ref(loaded);
            eprintln!("[crussty-plugin] tickhook: force load: {dot} loaded");
        }
        env.delete_local_ref(jname);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(())
    });
}
