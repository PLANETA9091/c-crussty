//! Runtime wiring for the FLUID-FREE-SECTION lever (S7-143 re-implementation
//! of the lost S7-139/91fcd70; design: research/fluid-free-2026-09-18/
//! DESIGN.md, S7-138 — lane: Entity.updateFluidHeightAndDoFluidPushing = 56%
//! of the top-1 CPU PalettedContainer.get clients).
//!
//! Two byte hooks cooperate through the inside_chain protocol:
//!
//!   1. SECTION HOOK (this module) — LevelChunkSection loads with the
//!      first chunk; the appended 2-field splice (patch_section_ff:
//!      crusstyFf B / crusstyFfGen I, PUBLIC VOLATILE, zero-init, no code
//!      edits) is served from pristine bytes captured at first sight.
//!
//!   2. ENTITY CHAIN (inside_cache.rs owns the Entity bytes) — after
//!      patch_inside_cache computes its retarget, the chain applies
//!      patch_fluid_gate on top (both updateFluidHeightAndDoFluidPushing
//!      wrapper sites -> FluidOps.fgate). The bridge class (FluidOps) is
//!      defined into the kernel loader BEFORE the retargeted bytes are
//!      ever served: inside_cache waits on wait_bridge_ready() and
//!      degrades inside-only (fail-dominant) if the bridge misses its
//!      window — the LinkageError race is closed by construction.
//!
//! FluidOps is compiled against the PURE kernel (scripts/build_fluid_ops.sh)
//! and reaches the injected fields via Unsafe offsets resolved lazily
//! (50 retries, then permanent vanilla fail-closed). Verdict cache is
//! validated against the demux MUTATION epoch (PalettedContainer.crusstyGen,
//! ±2 per mutation) — WITHOUT the demux lever the epoch field does not
//! exist, resolution fails, and the whole lever stays inert: a loud WARN is
//! printed (CRUSSTY_FLUID_FREE requires CRUSSTY_PALETTED_DEMUX=1).
//!
//! Gate: env `CRUSSTY_FLUID_FREE` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (flush_diet/inside_cache/alloc_diet).
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch) -> no patch, hook
//! dormant; bridge define failure -> dormant; section class missing -> hook
//! dormant after the 180s deadline; Unsafe resolution failure -> vanilla.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const SECTION_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunkSection";
const OPS_CLASS: &str = "net/minecraft/world/entity/FluidOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/FluidOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_FLUID_FREE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

fn demux_enabled() -> bool {
    std::env::var("CRUSSTY_PALETTED_DEMUX")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

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
    TARGET.get_or_init(|| Target::new(SECTION_CLASS))
}

/// Whether the inside_chain may compose the fluid gate on top of the inside
/// patch (gate on AND the bridge already defined).
pub fn bridge_ready() -> bool {
    BRIDGE_READY.load(Ordering::Relaxed)
}

/// Gate visibility for the inside_chain (does NOT check the bridge — the
/// chain itself waits on wait_bridge_ready).
pub fn enabled_pub() -> bool {
    enabled()
}

/// Bounded wait for the bridge definition (inside_chain protocol).
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while !bridge_ready() {
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    true
}

/// Entity-chain composer: apply the fluid gate retarget on top of already
/// inside-patched Entity bytes. Called by inside_cache BEFORE serving —
/// only when bridge_ready() holds, so invokestatic FluidOps.fgate can never
/// link against an undefined class.
pub fn compose_entity(patched_inside: &[u8]) -> Option<Vec<u8>> {
    match crate::classfile::patch_fluid_gate(patched_inside) {
        Ok((bytes, outcome)) => {
            eprintln!(
                "[crussty-plugin] fluid_free: entity chain composed ({outcome:?})"
            );
            Some(bytes)
        }
        Err(e) => {
            eprintln!(
                "[crussty-plugin] fluid_free: entity chain rejected ({e}), serving inside-only"
            );
            None
        }
    }
}

/// Register the section byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] fluid_free: dormant (set CRUSSTY_FLUID_FREE=1 to enable)"
        );
        return;
    }
    if !demux_enabled() {
        eprintln!(
            "[crussty-plugin] fluid_free: WARN CRUSSTY_FLUID_FREE requires CRUSSTY_PALETTED_DEMUX=1 (the verdict epoch is the demux mutation counter); without it the lever is inert — proceed only for isolation runs"
        );
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] fluid_free: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] fluid_free: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the section class, define the FluidOps
/// bridge into the kernel loader, compute the field splice from pristine
/// bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // The section class loads with the first chunk (boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] fluid_free: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] fluid_free: forcing kernel load of {}",
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

        // Kernel loader must be quiet before define/retransform (TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] fluid_free: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] fluid_free: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] fluid_free: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_fluid_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (EntityQueryOps pattern;
        // loader captured from the kernel Entity class).
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
            let ok = match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] fluid_free: defined {OPS_CLASS} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] fluid_free: define_class({OPS_CLASS}) failed"
                    );
                    false
                }
            };
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] fluid_free: bridge definition aborted, hook stays dormant");
            return;
        }
        // Bridge is servable: the entity chain may now compose its retarget.
        BRIDGE_READY.store(true, Ordering::Release);

        // Pristine section bytes: capture via no-op retransform if the class
        // predates the hook (fluid_guard pattern).
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] fluid_free: {} predates hook, capturing via no-op retransform",
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
                    "[crussty-plugin] fluid_free: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        // Compute the append-only field splice (pure rust, no code edits).
        let Some(original) = t.take_orig() else {
            return;
        };
        let patched = match crate::classfile::patch_section_ff(&original) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] fluid_free: patch rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        eprintln!(
            "[crussty-plugin] fluid_free: computed section splice ({} -> {} bytes)",
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));

        // Single retransform; the callback serves the cached splice.
        crate::kernel_policy::audit_wire(OPS_CLASS, "gate", "fluid_free v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] fluid_free: {} armed, retransform rc={rc}", t.name);
    });
}
