//! Runtime wiring for the NAV-SUBSYSTEM lever (TASK-401-C, round-401 vector C
//! — cmp401_navsys): canonical BlockState → PathType memo with the table on
//! the rust side.
//!
//! ARCHITECTURE (items_index precedent, round-398-j-subsys2):
//! 1. Bridge: `net/minecraft/world/entity/ai/NavOps.java` (defined into the
//!    kernel loader at boot+quiet, RegisterNatives navProbe/navTypeGet/
//!    navTypePut — natives in [`crate::nav_path_type`]). The bridge contains a
//!    1:1 Java port of the vanilla
//!    `WalkNodeEvaluator.getPathTypeFromState(BlockGetter, BlockPos)` chain
//!    plus the memo lookup keyed by `Block.getId(state)` (dense registry id of
//!    an immutable canonical state ⇒ staleness impossible).
//! 2. Byte hook: `net/minecraft/world/level/pathfinder/WalkNodeEvaluator`
//!    pristine capture at first load; after the bridge is defined the hook
//!    serves the WHOLE-BODY REDIRECT of the static method to
//!    `NavOps.getPathTypeFromState` (same static descriptor — stack shape
//!    identical, crate::classfile::redirect_static_method_body_to_static).
//! 3. Fail-closed: patch pattern mismatch → pristine bytes → vanilla path;
//!    natives unbound → NavOps.NATIVES_OK=false → direct vanilla-semantics
//!    compute; rust table rejects an id → recompute, no cache. Vanilla
//!    behavior by construction.
//!
//! ARM markers (server stdout):
//!   "[crussty-plugin] cmp401_navsys: defined net/minecraft/world/entity/ai/NavOps
//!    in kernel loader + registered path-type natives"
//!   "[crussty-plugin] cmp401_navsys: ARMED (canonical BlockState PathType memo; ..."
//!   "[crussty-plugin] cmp401_navsys: PATCHED WalkNodeEvaluator.getPathTypeFromState ..."

use crate::classfile;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const NAVOPS_CLASS: &str = "net/minecraft/world/entity/ai/NavOps";
const WNE_CLASS: &str = "net/minecraft/world/level/pathfinder/WalkNodeEvaluator";
const WNE_METHOD: &str = "getPathTypeFromState";
const WNE_DESC: &str =
    "(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/pathfinder/PathType;";

const NAVOPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ai/NavOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("cmp401_navsys"))
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(WNE_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None; // already served on this load/retransform cycle
        }
        match crate::classfile::redirect_static_method_body_to_static(
            bytes,
            WNE_METHOD,
            WNE_DESC,
            NAVOPS_CLASS,
            "getPathTypeFromState",
            WNE_DESC,
        ) {
            Ok((out, outcome)) => {
                use classfile::RetargetOutcome::*;
                match outcome {
                    NotFound | AlreadyPatched { .. } => {
                        PATCHED.store(false, Ordering::SeqCst);
                        None
                    }
                    Retargeted { .. } => {
                        eprintln!(
                            "[crussty-plugin] cmp401_navsys: PATCHED {WNE_CLASS}.{WNE_METHOD} \
                             ({} -> {} bytes; whole-body redirect -> NavOps, canonical state memo)",
                            bytes.len(),
                            out.len()
                        );
                        Some(out)
                    }
                }
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!(
                    "[crussty-plugin] cmp401_navsys: patch failed: {e} — hook stays dormant (vanilla path)"
                );
                None
            }
        }
    });
}

/// Background activation: wait for a quiet boot, define the bridge into the
/// kernel loader, bind natives, then flip READY and retransform the loaded
/// WalkNodeEvaluator so the hook applies the redirect.
pub fn activate() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp401_navsys: dormant (set CRUSSTY_LEVER_FLAG=cmp401_navsys to enable)"
        );
        return;
    }
    std::thread::spawn(|| {
        // Boot discipline: same as batch_collector/items_manager (quiet loader
        // before define).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp401_navsys: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(15));

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(NAVOPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] cmp401_navsys: {NAVOPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

        let defined = define_bridge_and_natives();
        if !defined {
            eprintln!("[crussty-plugin] cmp401_navsys: bridge definition failed, hook stays dormant");
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp401_navsys: defined {NAVOPS_CLASS} in kernel loader + registered path-type natives"
        );

        READY.store(true, Ordering::Release);

        // The kernel WNE may already be loaded (world mobs pathfind during
        // boot); poll briefly and retransform so the hook composes the
        // redirect on the loaded class. If it is not loaded yet, the hook
        // fires naturally at first load.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(WNE_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp401_navsys: {WNE_CLASS} not loaded within 180s (will patch at first load)"
                );
                READY.store(false, Ordering::Release);
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }
        let rc = cplug_sdk::retransform_class(WNE_CLASS);
        eprintln!(
            "[crussty-plugin] cmp401_navsys: ARMED (canonical BlockState PathType memo; retransform rc={rc})"
        );
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp401_navsys: applied (getPathTypeFromState -> NavOps; parity: pure function of canonical state, miss path = vanilla chain port)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp401_navsys: NOT APPLIED after retransform (see patch-failed line above; kernel build mismatch?)"
            );
        }
    });
}

fn define_bridge_and_natives() -> bool {
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
        let Some(c) = env.define_class(NAVOPS_CLASS, gref, NAVOPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] cmp401_navsys: define_class({NAVOPS_CLASS}) failed");
            return false;
        };

        let names = [
            CString::new("navProbe").expect("no NUL"),
            CString::new("navTypeGet").expect("no NUL"),
            CString::new("navTypePut").expect("no NUL"),
        ];
        let sigs = [
            CString::new("()I").expect("no NUL"),
            CString::new("(I)I").expect("no NUL"),
            CString::new("(II)I").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: crate::nav_path_type::nav_probe as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: crate::nav_path_type::nav_type_get as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[2].as_ptr(),
                signature: sigs[2].as_ptr(),
                fnPtr: crate::nav_path_type::nav_type_put as *const c_void as *mut c_void,
            },
        ];
        let reg = env.register_natives(c, &natives);
        if let Err(code) = reg {
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] cmp401_navsys: register_natives failed (code {code}) — hook stays dormant"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        true
    });
    defined.unwrap_or(false)
}

