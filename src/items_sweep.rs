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
                        "[crussty-plugin] items_sweep2: PATCH-FAIL {ITEM_ENTITY} not loaded within 180s — lever stays vanilla (fail-closed)"
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
                    "[crussty-plugin] items_sweep2: PATCH-FAIL boot marker not seen — lever stays vanilla (fail-closed)"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(20));

            // The ops class is co-defined with RegionTickOps (bank v4 always
            // delivers region_threads) — wait for that bridge before serving
            // patched ItemEntity bytes (lazy invokestatic resolution would
            // otherwise NCDFE on the first item tick).
            if !crate::region_threads::wait_bridge_ready_pub(180_000) {
                eprintln!(
                    "[crussty-plugin] items_sweep2: PATCH-FAIL region_threads bridge never became ready (ItemsSweepOps co-define) — lever stays vanilla (fail-closed)"
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
                        "[crussty-plugin] items_sweep2: PATCH-FAIL no pristine bytes for {ITEM_ENTITY} — lever stays vanilla (fail-closed)"
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
            let orig_len = original.len();
            let (patched, outcome) = match patch_items_sweep(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    // OWNER-MANDATED LOUD FAILURE (TASK-397-E2 re-arm): a
                    // rejected patch must never pass silently — the marker
                    // token PATCH-FAIL is the CI grep for silent-fail.
                    eprintln!(
                        "[crussty-plugin] items_sweep2: PATCH-FAIL {ITEM_ENTITY} {orig_len} bytes: {e} — lever stays vanilla (fail-closed)"
                    );
                    return;
                }
            };
            let patch_len = patched.len();
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
            // OWNER-MANDATED LOUD SUCCESS (TASK-397-E2 re-arm): ONE greppable
            // ARMED line carrying the class-size change (pristine -> patched).
            // A leg whose log lacks this line for the live class is NOT armed.
            if rc == 0 {
                eprintln!(
                    "[crussty-plugin] items_sweep2: ARMED {ITEM_ENTITY} {orig_len} -> {patch_len} bytes ({outcome}; merge site -> {SWEEP_OPS_CLASS}.tickMerge)"
                );
            } else {
                eprintln!(
                    "[crussty-plugin] items_sweep2: PATCH-FAIL {ITEM_ENTITY} retransform rc={rc} (patch {orig_len} -> {patch_len} bytes computed) — lever stays vanilla (fail-closed)"
                );
            }
        })
        .ok();
}

#[cfg(test)]
mod tests {
    /// TASK-397-E2 (d): the patcher must transform EXACTLY the real kernel
    /// ItemEntity bytes (pristine 28904 bytes, major 65, purpur-1.21.10
    /// patched-kernel). Extract once with:
    ///   python3 -c "import zipfile;open('/tmp/ItemEntity_pristine.class','wb')\
    ///     .write(zipfile.ZipFile('<patched-kernel.jar>')\
    ///     .read('net/minecraft/world/entity/item/ItemEntity.class'))"
    /// then run: ITEM_ENTITY_CLASS=/tmp/ItemEntity_pristine.class cargo test items_sweep
    /// Skips (green) when the fixture path is absent.
    #[test]
    fn patch_real_kernel_item_entity_bytes() {
        let Ok(path) = std::env::var("ITEM_ENTITY_CLASS") else {
            eprintln!("skip: ITEM_ENTITY_CLASS not set");
            return;
        };
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("skip: cannot read {path}: {e}");
                return;
            }
        };
        assert_eq!(bytes.len(), 28904, "pristine ItemEntity size drift");
        assert_eq!(
            crate::improved_noise::class_version(&bytes).map(|(m, _)| m),
            Some(65),
            "pristine ItemEntity major drift"
        );
        let (out, outcome) =
            super::patch_items_sweep(&bytes).expect("patch must succeed on real kernel bytes");
        assert_eq!(
            outcome,
            "tick site=1",
            "strict single-site contract violated"
        );
        assert_ne!(out.len(), bytes.len(), "patched size must differ");
        // The retargeted site must name the ops bridge entrypoint.
        let hay: &[u8] = &out;
        for needle in ["ItemsSweepOps".as_bytes(), b"tickMerge"] {
            assert!(
                hay.windows(needle.len()).any(|w| w == needle),
                "patched bytes missing {:?}",
                String::from_utf8_lossy(needle)
            );
        }
        // Idempotent purity: same input -> same output (pure function).
        let (out2, outcome2) = super::patch_items_sweep(&bytes).unwrap();
        assert_eq!(out, out2);
        assert_eq!(outcome, outcome2);
    }
}
