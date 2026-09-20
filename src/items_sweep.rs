//! Runtime wiring for the ITEMS-SWEEP v2 lever (TASK-397-E, MEGA-ROUND-2,
//! vector E sweep_rearm: sort-based sweep-line batch-merge — ONE in-place
//! long[] sort + monotone X-window pass per region bucket per tick replaces
//! the per-entity mergeWithNeighbours broadphase queries; TOP-1 bottleneck
//! items 31.17% + broadphase 15.66% java on bank v4. v2 of ROUND-396-C:
//! zero-alloc primitive-key sweep (no fastutil map, no per-cell arrays),
//! candidates = ALL mergeable items (v1 grid had stride-eligible only),
//! provable superset window + exact vanilla AABB filter.
//!
//! Byte hook (strict, fail-closed): `ItemEntity.tick()V` — the ONLY
//! tick()-class `mergeWithNeighbours()V` invoke site (census: offset 470,
//! javap purpur-1.21.10) retargeted 1:1 (receiver-prepended static):
//!   tick site -> `ItemsSweepOps.tickMerge(ItemEntity)` — suppressed inside
//!   the region-parallel sweep phase (the batch sweep of this tick already
//!   applied every merge the vanilla query would find); outside the phase
//!   the pristine vanilla body runs via a privateLookup MethodHandle.
//!
//! Bridge DEFINITION: `ItemsSweepOps` is CO-DEFINED with `RegionTickOps` in
//! region_threads.rs (bridge_list push below) — tickBucket references
//! ItemsSweepOps.sweepBucket unconditionally and the bank v4 always delivers
//! RegionTickOps, so the ops class must exist whenever the bridge does. The
//! env gate lives INSIDE the ops class (SWEEP static-init reads
//! CRUSSTY_LEVER_FLAG): lever off = static-read no-op, parity by construction.
//!
//! Gate (ItemEntity patch only): env `CRUSSTY_LEVER_FLAG == "items_sweep2"`.
//! Empty/other flag = dormant vanilla passthrough. The batch sweep is
//! spatially safe because region buckets are disjoint 8x8-chunk regions and
//! the merge contact reach (0.75 blocks) never crosses slot boundaries;
//! sweepBucket self-quarantines to vanilla on any internal failure.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

pub const SWEEP_OPS_CLASS: &str = "net/minecraft/world/entity/ItemsSweepOps";
pub const SWEEP_OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemsSweepOps.class");

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";

const MERGE_VIRTUAL: (&str, &str, &str) = (ITEM_ENTITY, "mergeWithNeighbours", "()V");
const OPS_TICK_MERGE_DESC: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_sweep2")
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

/// Single retarget in one composition; strict site count (fail-closed).
fn patch_items_sweep(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    let (out, outcome) = crate::classfile::retarget_virtual_to_static(
        bytes,
        "tick",
        "()V",
        MERGE_VIRTUAL,
        (SWEEP_OPS_CLASS, "tickMerge", OPS_TICK_MERGE_DESC),
    )?;
    match outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
        other => return Err(format!("tick site strict-1 violated: {other:?}")),
    }
    Ok((out, "tick site=1".to_string()))
}

/// Register the ItemEntity byte hook (idempotent; call once from
/// cplugin_init BEFORE ItemEntity can load).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_sweep2: dormant (CRUSSTY_LEVER_FLAG != items_sweep2)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_sweep2: pristine sighting {ITEM_ENTITY} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_sweep2: hook serve {ITEM_ENTITY} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for ItemEntity + the region_threads bridge
/// (ItemsSweepOps is co-defined there), then compute the single retarget
/// from pristine bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-sweep".into())
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
                    eprintln!("[crussty-plugin] items_sweep2: forcing kernel load of {ITEM_ENTITY}");
                    crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_sweep2: {ITEM_ENTITY} not loaded within 180s, hook stays dormant"
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
                eprintln!("[crussty-plugin] items_sweep2: boot marker not seen, hook stays dormant");
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(20));

            // The ops class is co-defined with RegionTickOps (bank v4 always
            // delivers region_threads) — wait for that bridge before serving
            // patched ItemEntity bytes (lazy invokestatic resolution would
            // otherwise NCDFE on the first item tick).
            if !crate::region_threads::wait_bridge_ready_pub(180_000) {
                eprintln!(
                    "[crussty-plugin] items_sweep2: region_threads bridge never became ready (ItemsSweepOps co-define), hook stays dormant"
                );
                return;
            }

            // Pristine bytes: if the class predates the hook (fast boot),
            // capture via no-op retransform (READY=false -> stash-only).
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_sweep2: {ITEM_ENTITY} predates hook, capturing via no-op retransform"
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
                        "[crussty-plugin] items_sweep2: no pristine bytes for {ITEM_ENTITY}, hook stays dormant"
                    );
                    return;
                }
            }

            // Compute the single retarget from the pristine bytes (pure rust,
            // strict site count). Any Err = kernel shape mismatch -> fail
            // closed (vanilla).
            let Some(original) = t.take_orig() else {
                return;
            };
            let major = crate::improved_noise::class_version(&original)
                .map(|(m, _)| m)
                .unwrap_or(0);
            let (patched, outcome) = match patch_items_sweep(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_sweep2: patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] items_sweep2: computed patch for {ITEM_ENTITY} ({} -> {} bytes, {outcome})",
                original.len(),
                patched.len()
            );
            t.set_patch(PatchCache {
                bytes: Arc::from(patched),
                major,
            });

            crate::kernel_policy::audit_wire(SWEEP_OPS_CLASS, "tickMerge", "items_sweep2 v1");
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
            if rc == 0 {
                ARMED.store(true, Ordering::SeqCst);
            }
            eprintln!(
                "[crussty-plugin] items_sweep2: {ITEM_ENTITY} armed, retransform rc={rc}"
            );
        })
        .ok();
}
