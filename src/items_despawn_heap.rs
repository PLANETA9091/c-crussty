//! TASK-397-H (MEGA-ROUND-2, lever_flag="items_despawn_heap"): event-driven
//! despawn/pickup lifetime subsystem for ItemEntity (RESEARCH-H.md).
//!
//! The vanilla per-tick tail gate `if (!isClientSide && age >= despawnRate)
//! {ItemDespawnEvent / discard}` inside ItemEntity.tick() AND
//! ItemEntity.inactiveTick() (the two and only lifetime evaluation points of
//! an item — 103k items x every tick on the bank scene) is REPLACED by a
//! heap-scheduled due lookup: a flat long[] min-heap (no boxing) schedules
//! each item's despawn tick ONCE (`due = sighting + (despawnRate - age)`,
//! exact vanilla algebra — the first tail where the age field reaches
//! despawnRate; overdue items are due immediately); the per-tick hot path is
//! a single flat-array read + compare. The despawn DECISION itself stays vanilla: at the due tick the
//! bridge returns false and the SURVIVING vanilla bytes (the untouched
//! `age >= despawnRate` field check + ItemDespawnEvent + discard/cancel
//! block) run exactly as vanilla — the bridge can only make the vanilla gate
//! RUN earlier/later, never change its math. Any ops failure fails open to
//! that surviving vanilla check.
//!
//! Byte surgery (strict, fail-closed, same-length — classfile::
//! splice_item_lifetime_tail): the first 7 bytes of both tails (aload_0 +
//! level()/isClientSide() virtual probe) become
//! `invokestatic ItemLifetimeOps.tailGate(ItemEntity)Z` + nops; zero branch
//! -target drift, zero StackMapTable edits.
//!
//! Deviations (documented in docs/TASK-397-H.md, all bounded, all dead in
//! the bench window): (1) pickupDelay stays vanilla (bench pins 32767);
//! (2) writer-shortened lifetimes (merge-max/makeFakeItem) despawn on their
//! own heap due (lateness <= remaining life); (3) isClientSide probe dropped
//! (dedicated kernel: always false; the surviving vanilla check re-decides).
//!
//! Gate: env CRUSSTY_LEVER_FLAG == "items_despawn_heap" (empty/other = exact
//! vanilla path, parity by construction). Java side re-parses the same env
//! at bridge static-init and prints the [S7-H] ARMED marker.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemLifetimeOps";
const OPS_METHOD: &str = "tailGate";

const OPS_BYTES: &[u8] = include_bytes!(
    "../items_despawn_heap/build/net/minecraft/world/entity/item/ItemLifetimeOps.class"
);

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_despawn_heap")
    )
}

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);

pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Poison recovery (TASK-46): locks only wrap plain Option stores.
struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
}

impl Target {
    fn new() -> Self {
        Self {
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
        self.orig.lock().unwrap_or_else(PoisonError::into_inner).is_some()
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
    TARGET.get_or_init(Target::new)
}

/// Both tail splices in one composition; strict site counts (fail-closed).
fn patch_despawn_heap(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    let (tick_bytes, tick_outcome) = crate::classfile::splice_item_lifetime_tail(
        bytes,
        "tick",
        OPS_CLASS,
        OPS_METHOD,
    )?;
    match tick_outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
        other => return Err(format!("tick tail strict-1 violated: {other:?}")),
    }
    let (out, it_outcome) = crate::classfile::splice_item_lifetime_tail(
        &tick_bytes,
        "inactiveTick",
        OPS_CLASS,
        OPS_METHOD,
    )?;
    match it_outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
        other => return Err(format!("inactiveTick tail strict-1 violated: {other:?}")),
    }
    Ok((out, "tick+inactiveTick tails=1+1".to_string()))
}

/// Register the ItemEntity byte hook (idempotent; call once from
/// cplugin_init BEFORE ItemEntity can load).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_despawn_heap: dormant (CRUSSTY_LEVER_FLAG != items_despawn_heap)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_despawn_heap: pristine sighting {ITEM_ENTITY} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_despawn_heap: hook serve {ITEM_ENTITY} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for boot + ItemEntity, define the
/// ItemLifetimeOps bridge into the kernel loader, splice both tails from
/// pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-despawn-heap".into())
        .spawn(move || {
            let t = target();
            // ItemEntity loads with the first entity chunk; force-load to
            // beat the population fixture (boot-time class-loading storm
            // discipline: force + wait_for_boot like every wiring module).
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(ITEM_ENTITY).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    eprintln!(
                        "[crussty-plugin] items_despawn_heap: forcing kernel load of {ITEM_ENTITY}"
                    );
                    crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_despawn_heap: {ITEM_ENTITY} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                let sighted = cplug_sdk::classes::is_sighted(ITEM_ENTITY);
                std::thread::sleep(std::time::Duration::from_millis(if sighted {
                    2_000
                } else {
                    10_000
                }));
            }

            if !crate::improved_noise::wait_for_boot() {
                eprintln!(
                    "[crussty-plugin] items_despawn_heap: boot marker not seen, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(20));

            // Embedded bridge bytes must not be newer than the JVM.
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
                    "[crussty-plugin] items_despawn_heap: ops class major {ops_major} but JVM supports up to {jvm_major} — rebuild items_despawn_heap/ via scripts/build_item_lifetime_ops.sh; hook stays dormant"
                );
                return;
            }

            // Resolution closure: the bridge MUST declare the exact static the
            // splice emits, else the first item tick detonates a
            // NoSuchMethodError and the lever must stay dormant.
            if !crate::classfile::item_lifetime_ops_resolution_closure(OPS_BYTES, OPS_METHOD) {
                eprintln!(
                    "[crussty-plugin] items_despawn_heap: bridge closure check failed ({OPS_CLASS}.{OPS_METHOD} missing), hook stays dormant"
                );
                return;
            }

            // Define the bridge into the KERNEL loader (captured from
            // ItemEntity itself — same-loader naming, the NCDFE lesson).
            let defined = cplug_sdk::jni_util::with_attached(|env| {
                let Some(cls) = cplug_sdk::classes::find_class(ITEM_ENTITY) else {
                    crate::clear_exception(env);
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
                match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!(
                            "[crussty-plugin] items_despawn_heap: defined {OPS_CLASS} in kernel loader"
                        );
                        true
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!(
                            "[crussty-plugin] items_despawn_heap: define_class({OPS_CLASS}) failed"
                        );
                        false
                    }
                }
            });
            if !defined.unwrap_or(false) {
                eprintln!(
                    "[crussty-plugin] items_despawn_heap: bridge definition aborted, hook stays dormant"
                );
                return;
            }

            // Pristine bytes: if the class predates the hook (fast boot),
            // capture via no-op retransform (READY=false -> stash-only).
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_despawn_heap: {ITEM_ENTITY} predates hook, capturing via no-op retransform"
                );
                for _ in 1..=3 {
                    let _ = cplug_sdk::retransform_class(ITEM_ENTITY);
                    if t.orig_is_some() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                if !t.orig_is_some() {
                    eprintln!(
                        "[crussty-plugin] items_despawn_heap: no pristine bytes for {ITEM_ENTITY}, hook stays dormant"
                    );
                    return;
                }
            }

            // Compute both splices from the pristine bytes (pure rust, strict
            // site counts). Any Err = kernel shape mismatch -> fail closed.
            let Some(original) = t.take_orig() else {
                return;
            };
            let major = crate::improved_noise::class_version(&original)
                .map(|(m, _)| m)
                .unwrap_or(0);
            let (patched, outcome) = match patch_despawn_heap(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_despawn_heap: patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] items_despawn_heap: computed patch for {ITEM_ENTITY} ({} -> {} bytes, {outcome})",
                original.len(),
                patched.len()
            );
            t.set_patch(PatchCache {
                bytes: Arc::from(patched),
                major,
            });

            crate::kernel_policy::audit_wire(OPS_CLASS, OPS_METHOD, "items_despawn_heap v1");
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
            if rc == 0 {
                ARMED.store(true, Ordering::SeqCst);
            }
            eprintln!(
                "[crussty-plugin] items_despawn_heap: {ITEM_ENTITY} armed, retransform rc={rc}"
            );
        })
        .ok();
}
