//! Runtime wiring for the ITEMS-WAKEUP lever (TASK-396-I, MEGA-ROUND-1,
//! vector I: event-driven wakeup-list for ItemEntity merge scans — the
//! TOP-1 bottleneck, items 31.17% + broadphase 15.66% java on bank v4).
//!
//! Byte hook (strict, fail-closed): `ItemEntity` — the ONLY class carrying
//! `mergeWithNeighbours()V` call sites (census: tick() offset 471 +
//! teleport() offset 29; javap purpur-1.21.10). Both sites retargeted 1:1
//! (receiver-prepended static, identical stack shape):
//!   1. tick site -> `ItemsWakeupOps.mergeWithNeighbours(ItemEntity)` —
//!      gated scan (E1 first sighting / E2 movement > 0.25 blocks since the
//!      last scan / E3 one-shot active bit / E4 post-merge neighbour wake;
//!      the vanilla body is invoked via reflection delegate — zero logic
//!      drift, no re-implementation).
//!   2. teleport site -> `ItemsWakeupOps.mergeAfterTeleport(ItemEntity)` —
//!      unconditional scan (vanilla semantics kept exactly on site 2).
//!
//! Bridge: `ItemsWakeupOps` defined into the KERNEL loader before any
//! patched bytes are served (BRIDGE_READY protocol, S7-143 LinkageError
//! lesson). The class self-tests its reflection delegate at static init and
//! degrades to skip-on-fail (never a fabricated merge, never a crash); the
//! patch itself only arms when both retargets are strict (sites == 1).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_wakeup"` (MEGA-ROUND generic
//! lever gate, run_world3.sh export). Empty/other flag = dormant vanilla
//! passthrough — parity by construction. Architecturally distinct from the
//! REFUTED fluid_dirty-memo (S7-153): per-ENTITY event SCHEDULING of the
//! broadphase query, no result memoization, no per-section chunk stamps.
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch) -> no patch, hook
//! dormant; bridge define failure -> dormant; non-strict site count ->
//! dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemsWakeupOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../items_wakeup/build/net/minecraft/world/entity/item/ItemsWakeupOps.class");

const MERGE_VIRTUAL: (&str, &str, &str) = (ITEM_ENTITY, "mergeWithNeighbours", "()V");
const OPS_MERGE_DESC: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";
const TELEPORT_DESC: &str =
    "(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;";

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_wakeup")
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

/// Both retargets in one composition; strict site counts (fail-closed).
fn patch_items_wakeup(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    let (tick_bytes, tick_outcome) = crate::classfile::retarget_virtual_to_static(
        bytes,
        "tick",
        "()V",
        MERGE_VIRTUAL,
        (OPS_CLASS, "mergeWithNeighbours", OPS_MERGE_DESC),
    )?;
    match tick_outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
        other => return Err(format!("tick site strict-1 violated: {other:?}")),
    }
    let (out, tp_outcome) = crate::classfile::retarget_virtual_to_static(
        &tick_bytes,
        "teleport",
        TELEPORT_DESC,
        MERGE_VIRTUAL,
        (OPS_CLASS, "mergeAfterTeleport", OPS_MERGE_DESC),
    )?;
    match tp_outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
        other => return Err(format!("teleport site strict-1 violated: {other:?}")),
    }
    Ok((out, "tick+teleport sites=1+1".to_string()))
}

/// Register the ItemEntity byte hook (idempotent; call once from
/// cplugin_init BEFORE ItemEntity can load).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_wakeup: dormant (CRUSSTY_LEVER_FLAG != items_wakeup)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_wakeup: pristine sighting {ITEM_ENTITY} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_wakeup: hook serve {ITEM_ENTITY} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for boot + ItemEntity, define the
/// ItemsWakeupOps bridge into the kernel loader, compute both retargets
/// from pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-wakeup".into())
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
                    eprintln!("[crussty-plugin] items_wakeup: forcing kernel load of {ITEM_ENTITY}");
                    crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_wakeup: {ITEM_ENTITY} not loaded within 180s, hook stays dormant"
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
                eprintln!("[crussty-plugin] items_wakeup: boot marker not seen, hook stays dormant");
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
                    "[crussty-plugin] items_wakeup: ops class major {ops_major} but JVM supports up to {jvm_major} — rebuild items_wakeup/ via scripts/build_items_wakeup_ops.sh; hook stays dormant"
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
                            "[crussty-plugin] items_wakeup: defined {OPS_CLASS} in kernel loader"
                        );
                        true
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!(
                            "[crussty-plugin] items_wakeup: define_class({OPS_CLASS}) failed"
                        );
                        false
                    }
                }
            });
            if !defined.unwrap_or(false) {
                eprintln!("[crussty-plugin] items_wakeup: bridge definition aborted, hook stays dormant");
                return;
            }

            // Pristine bytes: if the class predates the hook (fast boot),
            // capture via no-op retransform (READY=false -> stash-only).
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_wakeup: {ITEM_ENTITY} predates hook, capturing via no-op retransform"
                );
                for attempt in 1..=3 {
                    let _ = attempt;
                    let _ = cplug_sdk::retransform_class(ITEM_ENTITY);
                    if t.orig_is_some() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                if !t.orig_is_some() {
                    eprintln!(
                        "[crussty-plugin] items_wakeup: no pristine bytes for {ITEM_ENTITY}, hook stays dormant"
                    );
                    return;
                }
            }

            // Compute both retargets from the pristine bytes (pure rust,
            // strict site counts). Any Err = kernel shape mismatch -> fail
            // closed (vanilla).
            let Some(original) = t.take_orig() else {
                return;
            };
            let major = crate::improved_noise::class_version(&original)
                .map(|(m, _)| m)
                .unwrap_or(0);
            let (patched, outcome) = match patch_items_wakeup(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_wakeup: patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] items_wakeup: computed patch for {ITEM_ENTITY} ({} -> {} bytes, {outcome})",
                original.len(),
                patched.len()
            );
            t.set_patch(PatchCache {
                bytes: Arc::from(patched),
                major,
            });

            crate::kernel_policy::audit_wire(OPS_CLASS, "mergeWithNeighbours", "items_wakeup v1");
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
            if rc == 0 {
                ARMED.store(true, Ordering::SeqCst);
            }
            eprintln!(
                "[crussty-plugin] items_wakeup: {ITEM_ENTITY} armed, retransform rc={rc}"
            );
        })
        .ok();
}
