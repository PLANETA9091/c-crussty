//! Runtime wiring for the alloc-diet lever (S7-133, TASK-269 — ARCH-ATTACK
//! lever #2, the allocation lane).
//!
//! Patches TWO kernel call sites (see src/classfile.rs ALLOC-DIET section
//! for the javap contracts):
//!   1. `LivingEntity.pushEntities` — the `Level.getPushableEntities`
//!      virtual site retargeted to `EntityQueryOps.pushables` (kills the
//!      two per-call ArrayLists of the vanilla wrapper, ~45k+ calls/tick).
//!   2. `CollisionUtil.getCollisionsForBlocksOrWorldBorder` — the
//!      unconditional `new MutableBlockPos; dup; <init>` spliced to
//!      `EntityQueryOps.mutablePos` (kills ~250k ctor allocations/tick).
//!
//! Both patches are LENGTH-PRESERVING rust splices (no ASM, no StackMapTable
//! churn) computed from the pristine bytes captured at first class load;
//! served via a single retransform per class after the bridge
//! `EntityQueryOps` is defined into the kernel loader.
//!
//! Gate: env `CRUSSTY_ALLOC_DIET` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (same as fluid_guard/improved_noise): with
//! the gate off, register() logs a dormant notice and NO byte hook is
//! installed, activate() returns immediately, the module is
//! byte-indistinguishable from the pre-TASK-269 plugin.
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch) → no patch for
//! that class, hook stays dormant for BOTH (the lever is one unit);
//! bridge define failure → dormant; no pristine capture → dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const COLLISION_CLASS: &str = "ca/spottedleaf/moonrise/patches/collisions/CollisionUtil";
const OPS_CLASS: &str = "net/minecraft/world/entity/EntityQueryOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityquery/build/net/minecraft/world/entity/EntityQueryOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_ALLOC_DIET")
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

/// Per-class state: pristine capture + computed patch + serve log flag.
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

static TARGETS: std::sync::OnceLock<Vec<Target>> = std::sync::OnceLock::new();

fn targets() -> &'static [Target] {
    TARGETS.get_or_init(|| vec![Target::new(LIVING_CLASS), Target::new(COLLISION_CLASS)])
}

/// Register the byte hooks (idempotent; call once from cplugin_init).
///
/// The callbacks perform NO JNI/class-file work (loader-lock discipline —
/// see improved_noise): pristine capture at the class's own load, patch
/// served from the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] alloc_diet: dormant (set CRUSSTY_ALLOC_DIET=1 to enable)"
        );
        return;
    }
    for (idx, t) in targets().iter().enumerate() {
        let idx = idx;
        cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
            let t = &targets()[idx];
            if !READY.load(Ordering::Relaxed) {
                // Pristine sighting (the original class load): stash the
                // bytes for the worker; never rewrite here.
                eprintln!(
                    "[crussty-plugin] alloc_diet: pristine sighting {} {} bytes (major {})",
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
                    "[crussty-plugin] alloc_diet: hook serve {} {} bytes",
                    t.name,
                    cached.as_ref().map(|c| c.len()).unwrap_or(0)
                );
            }
            cached.map(|c| c.to_vec())
        });
    }
}

/// Background activation: wait for the kernel classes, define the
/// EntityQueryOps bridge into the kernel loader, compute the two
/// length-preserving patches from the pristine bytes, flip READY and
/// retransform both classes once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // Wait for BOTH kernel classes (LivingEntity loads at boot; the
        // collision util loads at the first collision query — force-load
        // if still absent at the deadline, improved_noise cadence).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for t in targets().iter() {
            let mut forced = 0usize;
            loop {
                if cplug_sdk::classes::find_class(t.name).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] alloc_diet: {} not loaded within 180s, hook stays dormant",
                        t.name
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                    && forced < 12
                {
                    forced += 1;
                    eprintln!(
                        "[crussty-plugin] alloc_diet: forcing kernel load of {} (attempt {forced})",
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
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] alloc_diet: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] alloc_diet: server booted, defining EntityQueryOps into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let ops_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if ops_major > jvm_major {
            eprintln!(
                "[crussty-plugin] alloc_diet: EntityQueryOps is class major {ops_major} but JVM supports up to {jvm_major} — rebuild entityquery/ via scripts/build_entity_query_ops.sh; hook stays dormant"
            );
            return;
        }

        // Capture the kernel loader global ref from LivingEntity.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(LIVING_CLASS) else {
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
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] alloc_diet: defined {OPS_CLASS} in kernel loader");
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] alloc_diet: define_class({OPS_CLASS}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] alloc_diet: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes: if a class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard
        // pattern.
        for t in targets().iter() {
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] alloc_diet: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] alloc_diet: no pristine bytes for {}, hook stays dormant",
                        t.name
                    );
                    return;
                }
            }
        }

        // Compute both patches from the pristine bytes (pure rust,
        // length-preserving). Any Err = kernel shape mismatch → fail closed
        // (the lever is one unit: both legs or none).
        for t in targets().iter() {
            let Some(original) = t.take_orig() else {
                return;
            };
            let major = crate::improved_noise::class_version(&original)
                .map(|(m, _)| m)
                .unwrap_or(0);
            let (patched, outcome) = if t.name == LIVING_CLASS {
                match crate::classfile::patch_push_entities(&original) {
                    Ok(pair) => pair,
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] alloc_diet: pushEntities patch rejected ({e}), hook stays dormant"
                        );
                        return;
                    }
                }
            } else {
                match crate::classfile::patch_collision_temps(&original) {
                    Ok(p) => (p, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }),
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] alloc_diet: collision-temps patch rejected ({e}), hook stays dormant"
                        );
                        return;
                    }
                }
            };
            let retargeted = matches!(
                outcome,
                crate::classfile::RetargetOutcome::Retargeted { .. }
                    | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
            );
            if !retargeted {
                eprintln!(
                    "[crussty-plugin] alloc_diet: unexpected patch outcome for {} ({outcome:?}), hook stays dormant",
                    t.name
                );
                return;
            }
            eprintln!(
                "[crussty-plugin] alloc_diet: computed patch for {} ({} -> {} bytes, {outcome:?})",
                t.name,
                original.len(),
                patched.len()
            );
            t.set_patch(PatchCache {
                bytes: Arc::from(patched),
                major,
            });
        }

        // Single retransform per class; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_CLASS, "pushables/mutablePos", "alloc_diet v1");
        READY.store(true, Ordering::Release);
        for t in targets().iter() {
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!("[crussty-plugin] alloc_diet: {} armed, retransform rc={rc}", t.name);
        }
    });
}
