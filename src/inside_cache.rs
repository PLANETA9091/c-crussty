//! Runtime wiring for the INSIDE-CACHE lever (S7-135, TASK-271 — ARCH-ATTACK
//! lever #3, the inside-blocks/fluid discovery lane).
//!
//! Patches ONE kernel call site (see src/classfile.rs INSIDE-CACHE section
//! for the javap contract): the method-entry gate
//! `Entity.isAffectedByBlocks` INSIDE
//! `Entity.checkInsideBlocks(List, StepBasedCollector)` retargeted to the
//! static `InsideBlockOps.gate(Entity)Z` (receiver-first, 3B→3B,
//! length-preserving). For static entities (deltaMovement==0, position
//! bit-equal to the cached tick) the bridge serves the whole discovery
//! from flat primitive slot arrays (replay of the vanilla effect calls);
//! anything else falls through to `e.isAffectedByBlocks()` — the vanilla
//! body runs untouched.
//!
//! The patch is computed from the pristine bytes captured at first class
//! load; served via a single retransform after both bridge classes
//! (`InsideBlockOps`, `InsideBlockOps$Recorder`) are defined into the
//! kernel loader.
//!
//! Gate: env `CRUSSTY_INSIDE_CACHE` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (same as fluid_guard/alloc_diet): with the
//! gate off, register() logs a dormant notice and NO byte hook is
//! installed, activate() returns immediately, the module is
//! byte-indistinguishable from the pre-S7-135 plugin.
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch / collector
//! field renamed) → no patch, hook stays dormant; bridge define failure →
//! dormant; no pristine capture → dormant; bridge Unsafe resolution
//! failure (ARMED=false) → gate always returns the vanilla verdict.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps";
const RECORDER_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps$Recorder";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBlockOps.class");
const RECORDER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBlockOps$Recorder.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_CACHE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Poison recovery (TASK-46): every lock uses
/// `unwrap_or_else(PoisonError::into_inner)` — locks only wrap plain
/// Vec/Arc stores (no user code under the lock).
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
}

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
    fn set_patch(&self, cache: PatchCache) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(cache);
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .map(|c| Arc::clone(&c.bytes))
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(ENTITY_CLASS))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// The callback performs NO JNI/class-file work (loader-lock discipline —
/// see improved_noise): pristine capture at the class's own load, patch
/// served from the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_cache: dormant (set CRUSSTY_INSIDE_CACHE=1 to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the
            // bytes for the worker; never rewrite here.
            eprintln!(
                "[crussty-plugin] inside_cache: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] inside_cache: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the kernel class, define both bridge
/// classes into the kernel loader, compute the length-preserving patch
/// from the pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // Wait for the kernel Entity class (loads at boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_cache: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_cache: forcing kernel load of {}",
                    t.name
                );
                crate::improved_noise::force_load_kernel_class(t.name);
            }
            let sighted = cplug_sdk::classes::is_sighted(t.name);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] inside_cache: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_cache: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (RECORDER_CLASS, RECORDER_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] inside_cache: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_block_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        // Capture the kernel loader global ref from Entity.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            let mut ok = true;
            for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (RECORDER_CLASS, RECORDER_BYTES)] {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] inside_cache: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] inside_cache: define_class({name}) failed");
                        ok = false;
                        break;
                    }
                }
            }
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] inside_cache: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard
        // pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] inside_cache: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
                let _ = attempt;
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] inside_cache: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        // Compute the patch from the pristine bytes (pure rust,
        // length-preserving). Any Err = kernel shape mismatch → fail closed.
        let Some(original) = t.take_orig() else {
            return;
        };
        let major = crate::improved_noise::class_version(&original)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let (patched, outcome) = match crate::classfile::patch_inside_cache(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] inside_cache: patch rejected ({e}), hook stays dormant"
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
                "[crussty-plugin] inside_cache: unexpected patch outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] inside_cache: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(PatchCache {
            bytes: Arc::from(patched),
            major,
        });

        // Single retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_CLASS, "gate", "inside_cache v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] inside_cache: {} armed, retransform rc={rc}", t.name);
    });
}
