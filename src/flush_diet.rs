//! Runtime wiring for the FLUSH-DIET lever (S7-137, ARCH-ATTACK lever #4 —
//! the InsideBlockEffectApplier$StepBasedCollector.flushStep allocation
//! lane).
//!
//! STEP-0 (javap on the materialized purpur-1.21.10 kernel) + the alloc
//! census of run 35275967738 (X150K, first true churn ranking):
//! vanilla flushStep() moves the per-type before/after effect lists into
//! finalEffects with List.addAll; ArrayList.addAll resolves c.toArray()
//! BEFORE the emptiness check, so every empty call still allocates
//! new Object[0] via Arrays.copyOf — 336 samples (4.6% of the true
//! 25.6GB/60s churn) on
//!   advanceStep -> flushStep -> ArrayList.addAll -> ArrayList.toArray
//!     -> Arrays.copyOf -> Object[].
//! The collector runs per entity per tick (field insideEffectCollector),
//! advanceStep fires on every block step of the inside-blocks traversal —
//! ~300k advanceSteps/tick at 150k entities.
//!
//! Patches ONE kernel class: both addAll call sites inside flushStep()V
//! (invokeinterface List.addAll, 5B each) retargeted to invokestatic
//! FlushOps.fladd (3B) + 2 nop — length-preserving, verifier-identical
//! stack shape. Semantics: empty source adds nothing and returns false
//! (what ArrayList.addAll returns after its wasted toArray); non-empty
//! sources delegate to the same List.addAll — order, identity, effects
//! untouched (median-exact parity by construction).
//!
//! Patch computed from pristine bytes captured at first class load; served
//! via a single retransform after the bridge class (`FlushOps`) is defined
//! into the kernel loader.
//!
//! Gate: env `CRUSSTY_FLUSH_DIET` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (fluid_guard/alloc_diet/inside_cache): with
//! the gate off, register() logs a dormant notice and NO byte hook is
//! installed, activate() returns immediately, the module is
//! byte-indistinguishable from the pre-S7-137 plugin.
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch / field renamed)
//! → no patch, hook stays dormant; bridge define failure → dormant; no
//! pristine capture → dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const SBC_CLASS: &str =
    "net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector";
const OPS_CLASS: &str = "net/minecraft/world/entity/FlushOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/FlushOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_FLUSH_DIET")
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
    TARGET.get_or_init(|| Target::new(SBC_CLASS))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise): the callback performs NO JNI
/// work; pristine capture at the class's own load, patch served from the
/// cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] flush_diet: dormant (set CRUSSTY_FLUSH_DIET=1 to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] flush_diet: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] flush_diet: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the collector class, define the bridge
/// into the kernel loader, compute the length-preserving patch from the
/// pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // The collector class loads with the first Entity (boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] flush_diet: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] flush_diet: forcing kernel load of {}",
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

        // Kernel loader must be quiet before define/retransform (fluid_guard
        // TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] flush_diet: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] flush_diet: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] flush_diet: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_flush_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (EntityQueryOps pattern).
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
                    eprintln!("[crussty-plugin] flush_diet: defined {OPS_CLASS} in kernel loader");
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] flush_diet: define_class({OPS_CLASS}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] flush_diet: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes for a class that predates the hook (fast boot):
        // no-op retransform capture, fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] flush_diet: {} predates hook, capturing via no-op retransform",
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
                    "[crussty-plugin] flush_diet: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };
        let (patched, outcome) = match crate::classfile::patch_flush_step(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] flush_diet: patch rejected ({e}), hook stays dormant"
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
                "[crussty-plugin] flush_diet: unexpected patch outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] flush_diet: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));

        crate::kernel_policy::audit_wire(OPS_CLASS, "fladd", "flush_diet v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] flush_diet: {} armed, retransform rc={rc}", t.name);
    });
}
