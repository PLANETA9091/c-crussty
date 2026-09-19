//! Runtime wiring for the CHUNK-PARSE-DIAG lever (RECON-13d, TASK-327 —
//! see entityinside/net/minecraft/world/level/chunk/storage/ChunkParseDiagOps.java).
//!
//! The TOP-1 alloc lane of the 150k scene is the chunk-parse path
//! (SerializableChunkData.parse + MapDecoder/codec machinery — 33.38% of the
//! ap-samples of run s7165, RECON-13b/13e). The lane cost is a pure function
//! of the NUMBER of parse events; the open question is whether those events
//! are REPEAT loads of already-seen chunks (cache pays) or FIRST loads of new
//! chunks (ticket churn — cache never pays). This lever instruments the parse
//! entry with a passive per-chunk census: the single `ldc "xPos"` +
//! `invokevirtual CompoundTag.getIntOr` pair inside parse is retargeted to
//! ChunkParseDiagOps.diagXIntOr — the receiver rides as the first static arg
//! (verifier-visible stack shape identical), the value is delegated 1:1, and
//! the note is a read-only map update wrapped in catch(Throwable).
//!
//! Diagnostics-not-config: the census file (CRUSSTY_PARSE_DIAG_FILE) is the
//! measurement artifact; no server behavior beyond the O(1) note changes.
//!
//! Patch computed from pristine bytes captured at first class load (the class
//! loads long before boot, so the pristine path is the no-op-retransform
//! capture, fluid_guard pattern); served via a single retransform after the
//! bridge (`ChunkParseDiagOps`) is defined into the kernel loader.
//!
//! Gate: env `CRUSSTY_PARSE_DIAG` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline. Fail-closed matrix: patcher Err (method
//! shape mismatch) -> no patch, hook stays dormant; bridge define failure ->
//! dormant; no pristine capture -> dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const TARGET_CLASS: &str =
    "net/minecraft/world/level/chunk/storage/SerializableChunkData";
const OPS_CLASS: &str = "net/minecraft/world/level/chunk/storage/ChunkParseDiagOps";

const OPS_BYTES: &[u8] = include_bytes!(
    "../entityinside/build/net/minecraft/world/level/chunk/storage/ChunkParseDiagOps.class"
);

fn enabled() -> bool {
    std::env::var("CRUSSTY_PARSE_DIAG")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);

struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

/// Poison recovery (TASK-46): locks only wrap plain Vec/Arc stores.
impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        let mut orig = self.orig.lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(bytes.to_vec());
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn set_patch(&self, bytes: Arc<[u8]>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(TARGET_CLASS))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise): the callback performs NO JNI
/// work; pristine capture at the class's own load, patch served from the
/// cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] parse_diag: dormant (set CRUSSTY_PARSE_DIAG=1 to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] parse_diag: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] parse_diag: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for boot, define the bridge into the kernel
/// loader, capture pristine bytes via no-op retransform (the target class
/// predates the hook — it loads during early boot), compute the
/// length-preserving patch, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // Kernel loader must be quiet before define/retransform (fluid_guard
        // TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] parse_diag: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] parse_diag: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] parse_diag: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

        // RESOLUTION CLOSURE GUARD: the embedded bridge bytes MUST declare
        // the receiver-prepended diagXIntOr the retarget emits; otherwise the
        // first chunk parse detonates a NoSuchMethodError. Fail-closed.
        let closure_ok = cplug_sdk::jni_util::with_attached(|_env| {
            crate::classfile::parse_diag_resolution_closure(OPS_BYTES)
        })
        .unwrap_or(false);
        if !closure_ok {
            eprintln!(
                "[crussty-plugin] parse_diag: RESOLUTION CLOSURE FAILED — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (FlushOps pattern).
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity")
            else {
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
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] parse_diag: defined {OPS_CLASS} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] parse_diag: define_class({OPS_CLASS}) failed"
                    );
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] parse_diag: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // Arm the periodic dump + shutdown hook (census artifact writer).
        let diag_file = std::env::var("CRUSSTY_PARSE_DIAG_FILE")
            .unwrap_or_else(|_| "chunk-parse-diag.txt".to_string());
        let _ = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(OPS_CLASS) else {
                crate::clear_exception(env);
                return;
            };
            let Some(method) =
                env.get_static_method_id(cls.as_jclass(), "armDumpTask", "(Ljava/lang/String;)V")
            else {
                crate::clear_exception(env);
                return;
            };
            let Some(path) = env.new_string(&diag_file) else {
                crate::clear_exception(env);
                return;
            };
            env.call_static_void_method(
                cls.as_jclass(),
                method,
                &[jni::jvalue { l: path }],
            );
            let _ = crate::clear_exception(env);
        });

        // Pristine bytes for a class that predates the hook: no-op
        // retransform capture, fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] parse_diag: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _attempt in 1..=5 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] parse_diag: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };
        let (patched, outcome) = match crate::classfile::retarget_ldc_virtual_to_static(
            &original,
            "parse",
            "(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;",
            "xPos",
            (
                "net/minecraft/nbt/CompoundTag",
                "getIntOr",
                "(Ljava/lang/String;I)I",
            ),
            (
                OPS_CLASS,
                "diagXIntOr",
                "(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;I)I",
            ),
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] parse_diag: patch rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let retargeted = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !retargeted {
            eprintln!(
                "[crussty-plugin] parse_diag: unexpected patch outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] parse_diag: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));

        crate::kernel_policy::audit_wire(OPS_CLASS, "diagXIntOr", "parse_diag v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] parse_diag: {} armed, retransform rc={rc}", t.name);
    });
}
