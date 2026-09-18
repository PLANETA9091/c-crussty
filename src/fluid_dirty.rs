//! Runtime wiring for the FLUID-DIRTY lever (S7-151, TASK-290 — ARCH-ATTACK
//! lever #6, the fluid-scan lane ~10% total CPU on live X150K).
//!
//! Own byte hook: `LevelChunk.setBlockState(BlockPos,BlockState,I)` — its
//! single `LevelChunkSection.setBlockState(IIILBlockState)BlockState` call
//! site retargeted to `FluidPushOps.secWrite(...)` (delegate + dirty-stamp
//! bump on real fluid-state changes). The ENTITY side (both fluid-scan
//! wrapper sites → `FluidPushOps.scan`) is COMPOSED by the inside_cache
//! chain (inside_cache::activate), exactly like fluid_free's fgate — the
//! Entity bytes therefore carry inside + fluid_dirty (and fluid_free when
//! enabled) retargets from one hook serve.
//!
//! Bridge: `FluidPushOps` (+ inner `ScanOut`) defined into the KERNEL loader
//! before any patched bytes are served (BRIDGE_READY protocol, the S7-143
//! LinkageError lesson: the chain waits on wait_bridge_ready).
//!
//! Gate: env `CRUSSTY_FLUID_DIRTY` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline. Requires the inside_cache chain for the
//! entity retarget (LOUD WARN without it: LevelChunk hook still arms —
//! ledger is maintained — but no scan memoization happens; parity intact,
//! benefit zero).
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch) → no patch, hook
//! dormant; bridge define failure → dormant; compose rejection → inside-only
//! bytes (fail-dominant, loud WARN).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const LEVELCHUNK_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunk";
const OPS_CLASS: &str = "net/minecraft/world/entity/FluidPushOps";
const OPS_INNER_CLASS: &str = "net/minecraft/world/entity/FluidPushOps$ScanOut";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/FluidPushOps.class");
const OPS_INNER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/FluidPushOps$ScanOut.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_FLUID_DIRTY")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
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
    TARGET.get_or_init(|| Target::new(LEVELCHUNK_CLASS))
}

/// Whether the inside_chain may compose the scan retarget on top of the
/// inside patch (gate on AND the bridge already defined).
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

/// Entity-chain composer: apply the fluid-scan retarget on top of already
/// inside(+fluid_free)-patched Entity bytes. Called by inside_cache BEFORE
/// serving — only when bridge_ready() holds, so invokestatic
/// FluidPushOps.scan can never link against an undefined class (the
/// S7-148 NCDFE lesson, closed by construction).
pub fn compose_entity(patched_chain: &[u8]) -> Option<Vec<u8>> {
    match crate::classfile::patch_fluid_dirty_entity(patched_chain) {
        Ok((bytes, outcome)) => {
            eprintln!(
                "[crussty-plugin] fluid_dirty: entity chain composed ({outcome:?})"
            );
            Some(bytes)
        }
        Err(e) => {
            eprintln!(
                "[crussty-plugin] fluid_dirty: entity chain rejected ({e}), serving chain-only"
            );
            None
        }
    }
}

/// Register the LevelChunk byte hook (idempotent; call once from
/// cplugin_init).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] fluid_dirty: dormant (set CRUSSTY_FLUID_DIRTY=1 to enable)"
        );
        return;
    }
    if !crate::inside_cache::enabled_pub() {
        eprintln!(
            "[crussty-plugin] fluid_dirty: WARN CRUSSTY_FLUID_DIRTY requires the inside_cache chain (CRUSSTY_INSIDE_CACHE=1) for the Entity scan retarget; without it the LevelChunk ledger still arms but NO memoization happens — proceed only for isolation runs"
        );
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] fluid_dirty: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] fluid_dirty: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the LevelChunk class, define the
/// FluidPushOps bridge into the kernel loader, compute the secWrite
/// retarget from pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // LevelChunk loads with the first chunk (boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] fluid_dirty: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] fluid_dirty: forcing kernel load of {}",
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
            eprintln!("[crussty-plugin] fluid_dirty: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] fluid_dirty: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (OPS_INNER_CLASS, OPS_INNER_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] fluid_dirty: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_fluid_push_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        // Capture the kernel loader global ref from LevelChunk.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(t.name) else {
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
            for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (OPS_INNER_CLASS, OPS_INNER_BYTES)] {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] fluid_dirty: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] fluid_dirty: define_class({name}) failed");
                        ok = false;
                        break;
                    }
                }
            }
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] fluid_dirty: bridge definition aborted, hook stays dormant");
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard
        // pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] fluid_dirty: {} predates hook, capturing via no-op retransform",
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
                    "[crussty-plugin] fluid_dirty: no pristine bytes for {}, hook stays dormant",
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
        let (patched, outcome) = match crate::classfile::patch_fluid_dirty_levelchunk(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] fluid_dirty: patch rejected ({e}), hook stays dormant"
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
                "[crussty-plugin] fluid_dirty: unexpected patch outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] fluid_dirty: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(PatchCache {
            bytes: Arc::from(patched),
            major,
        });

        // Single retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_CLASS, "secWrite", "fluid_dirty v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] fluid_dirty: {} armed, retransform rc={rc}", t.name);
    });
}
