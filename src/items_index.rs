//! Runtime wiring for the ITEMS-INDEX lever (TASK-395 MEGA-ROUND, agent A —
//! `items_index`: секционно-резидентный spatial-hash индекс item-мердж-
//! кандидатов).
//!
//! Replaces ONLY the candidate scan of the vanilla item merge:
//! `ItemEntity.mergeWithNeighbours()V` calls
//! `Level.getEntitiesOfClass(ItemEntity.class, inflatedBox, predicate)` — the
//! retarget swaps that single call site to
//! `ItemMergeIndexOps.mergeCandidates(ItemEntity, double, AABB, Predicate)`
//! with an IDENTICAL stack shape (same already-inflated box computed by the
//! VERBATIM copied bytecode incl. paper onlyMergeItemsHorizontally, same
//! predicate instance from the vanilla invokedynamic, List return). The loop
//! body (isMergable, fixItemsMergingThroughWalls clip, tryToMerge, isRemoved
//! break) and the second call site (teleport) stay vanilla bytes.
//!
//! Index (bridge class, defined into the kernel loader): per-Level 0.5-grid
//! fastutil Long2ObjectOpenHashMap striped over 64 ReentrantLock shards
//! (region_threads>=2 safe), lazy query-proportional maintenance — refresh at
//! query, compact-on-touch (removed/aged/superseded), exact vanilla
//! post-filters (predicate + CURRENT box intersection) per candidate.
//!
//! Patch pipeline mirrors flush_diet: byte hook captures pristine ItemEntity
//! bytes at first load; the activation worker waits for boot, defines the
//! ops trio into the kernel loader, computes the length-preserving patch
//! (classfile::patch_itementity_merge — fail-closed on any shape mismatch),
//! then serves via a single retransform.
//!
//! Gate: `CRUSSTY_LEVER_FLAG == "items_index"` (run_world3.sh exports
//! CRUSSTY_LEVER_FLAG from the workflow lever_flag input). Any other value —
//! dormant-invisible: ItemEntity bytes untouched, vanilla path by
//! construction.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const TARGET_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemMergeIndexOps";
const OPS_CELL_CLASS: &str = "net/minecraft/world/entity/item/ItemMergeIndexOps$Cell";
const OPS_SHARD_CLASS: &str = "net/minecraft/world/entity/item/ItemMergeIndexOps$Shard";

const OPS_BYTES: &[u8] = include_bytes!(
    "../itemsindex/build/net/minecraft/world/entity/item/ItemMergeIndexOps.class"
);
const OPS_CELL_BYTES: &[u8] = include_bytes!(
    "../itemsindex/build/net/minecraft/world/entity/item/ItemMergeIndexOps$Cell.class"
);
const OPS_SHARD_BYTES: &[u8] = include_bytes!(
    "../itemsindex/build/net/minecraft/world/entity/item/ItemMergeIndexOps$Shard.class"
);

fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("items_index"))
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
/// ItemEntity has NO other lever owner in the compose chain — this module is
/// the sole byte-hook on the class. Loader-lock discipline: the callback does
/// NO JNI work; pristine capture at the class's own load, patch served from
/// the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_index: dormant (set CRUSSTY_LEVER_FLAG=items_index to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_index: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] items_index: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for ItemEntity to load, wait for a quiet boot,
/// define the ops trio into the kernel loader, compute the length-preserving
/// patch from the pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-index".into())
        .spawn(move || {
            let t = target();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(t.name).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_index: {} not loaded within 180s, hook stays dormant",
                        t.name
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    eprintln!(
                        "[crussty-plugin] items_index: forcing kernel load of {}",
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
                eprintln!(
                    "[crussty-plugin] items_index: boot marker not seen, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(20));
            eprintln!(
                "[crussty-plugin] items_index: server booted, defining ops trio into kernel loader"
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
                    "[crussty-plugin] items_index: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild itemsindex/ via scripts/build_items_index_ops.sh; hook stays dormant"
                );
                return;
            }

            // Define the ops trio into the kernel loader (Entity loader anchor,
            // fluid_bitmask/flush_diet pattern).
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
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                let mut defined = 0usize;
                for (name, bytes) in [
                    (OPS_CLASS, OPS_BYTES),
                    (OPS_CELL_CLASS, OPS_CELL_BYTES),
                    (OPS_SHARD_CLASS, OPS_SHARD_BYTES),
                ] {
                    match env.define_class(name, gref, bytes) {
                        Some(c) => {
                            env.delete_local_ref(c);
                            defined += 1;
                            eprintln!(
                                "[crussty-plugin] items_index: defined {name} in kernel loader"
                            );
                        }
                        None => {
                            crate::describe_exception(env);
                            eprintln!(
                                "[crussty-plugin] items_index: define_class({name}) failed"
                            );
                        }
                    }
                }
                defined == 3
            });
            if !defined.unwrap_or(false) {
                eprintln!(
                    "[crussty-plugin] items_index: ops definition aborted, hook stays dormant"
                );
                return;
            }

            // Pristine bytes for a class that predates the hook (fast boot):
            // no-op retransform capture, fluid_guard pattern.
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_index: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] items_index: no pristine bytes for {}, hook stays dormant",
                        t.name
                    );
                    return;
                }
            }

            let Some(original) = t.take_orig() else {
                return;
            };
            let (patched, outcome) = match crate::classfile::patch_itementity_merge(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_index: patch rejected ({e}), hook stays dormant"
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
                    "[crussty-plugin] items_index: unexpected patch outcome ({outcome:?}), hook stays dormant"
                );
                return;
            }
            eprintln!(
                "[crussty-plugin] items_index: computed patch for {} ({} -> {} bytes, {outcome:?})",
                t.name,
                original.len(),
                patched.len()
            );
            t.set_patch(Arc::from(patched));

            crate::kernel_policy::audit_wire(OPS_CLASS, "mergeCandidates", "items_index v1");
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!("[crussty-plugin] items_index: {} armed, retransform rc={rc}", t.name);
        })
        .ok();
}
