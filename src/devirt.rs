//! Runtime wiring for the CMP399-DEVIRT lever (TASK-399-G, MEGA-ROUND-3,
//! agent G — JIT-friendly devirtualization of the post-J tick hot path).
//!
//! Two independent, javap-verified retargets (see src/classfile.rs
//! "TASK-399-G cmp399_devirt" block for the byte contracts):
//!
//!   D1  `SynchedEntityData.get` body fusion — the single chokepoint of the
//!       SynchedEntityData read chain (~2.5-5% java self in the round-j2b
//!       profile: getHealth/isAlive, tickBody/getItem, getSharedFlag/
//!       isSprinting, isNoGravity, isNoAi, getAirSupply...). Replaces the
//!       pristine `getItem(key).getValue()` double-virtual body with a
//!       direct `itemsById[accessor.id()].value` read chain — bit-identical
//!       semantics (same array, same index, same plain `value` field; the
//!       vanilla DataItem.getValue body IS `getfield value`), zero
//!       volatile-order change, one retransform, EVERY reader benefits.
//!
//!   D2  `GoalSelector.goalContainsAnyFlags` body redirect to the identical
//!       static on the already-defined ItemEntityManager bridge (J lever
//!       class; 0 new classes, 0 NCDFE window — the bridge must exist or
//!       the stage is skipped fail-closed). The vanilla site is the
//!       profile's vtable-stub hotspot on WrappedGoal.getFlags (0.33%);
//!       relocating the body gives C2 a fresh call site whose type profile
//!       starts at one receiver type -> devirtualization + inlining.
//!
//! Delivery: whole-class byte hooks (pristine capture at first load,
//! patch served from the cache after READY; vanilla passthrough before).
//! Fail-closed matrix: no pristine bytes -> dormant; patch rejection ->
//! that stage dormant (vanilla behaviour); ItemEntityManager bridge
//! undefined at D2 arm time -> D2 skipped (D1 unaffected).
//!
//! Gate: CRUSSTY_LEVER_FLAG == "cmp399_devirt" (strict — other agents'
//! cmp399_* flags do not arm THIS module's sites; items_manager arms its
//! own bridge on the shared cmp399_* prefix per the composition contract).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const SED_CLASS: &str = "net/minecraft/network/syncher/SynchedEntityData";
const GS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalSelector";
const IM_CLASS: &str = "net/minecraft/world/entity/ItemEntityManager";

/// Poison recovery (TASK-46): locks wrap plain Vec/Arc stores only.
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
    ready: AtomicBool,
}

#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
}

impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
            ready: AtomicBool::new(false),
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
    fn set_patch(&self, bytes: Vec<u8>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(bytes.into_boxed_slice()),
        });
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .map(|c| Arc::clone(&c.bytes))
    }
    fn ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }
}

static TARGET_SED: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static TARGET_GS: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target_sed() -> &'static Target {
    TARGET_SED.get_or_init(|| Target::new(SED_CLASS))
}
fn target_gs() -> &'static Target {
    TARGET_GS.get_or_init(|| Target::new(GS_CLASS))
}

fn flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp399_devirt")
        .unwrap_or(false)
}

/// Register the two whole-class byte hooks (idempotent; call once from
/// cplugin_init, BEFORE any kernel class loads — pristine capture at the
/// class's own load; the callbacks do NO class-file work on the loader
/// thread: stash before READY, serve the cached patch after).
pub fn register() {
    if !flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp399_devirt: dormant (set CRUSSTY_LEVER_FLAG=cmp399_devirt to enable)"
        );
        return;
    }
    for t in [target_sed(), target_gs()] {
        let t: &'static Target = t;
        cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
            if !t.ready() {
                eprintln!(
                    "[crussty-plugin] cmp399_devirt: pristine sighting {} {} bytes (major {})",
                    t.name,
                    bytes.len(),
                    crate::improved_noise::class_version(bytes)
                        .map(|(m, _)| m)
                        .unwrap_or(0)
                );
                t.stash_orig(bytes);
                return None;
            }
            // Serve the precomputed patch; the clone is an Arc refcount bump.
            let cached = t.patch_bytes();
            if !t.served.swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] cmp399_devirt: hook serve {} {} bytes",
                    t.name,
                    cached.as_ref().map(|c| c.len()).unwrap_or(0)
                );
            }
            cached.map(|c| c.to_vec())
        });
    }
    eprintln!(
        "[crussty-plugin] cmp399_devirt: byte hooks registered on {SED_CLASS} + {GS_CLASS}"
    );
}

/// Wait until `name` is loaded in the JVM (find_class poll); after
/// `force_after_ms` force a kernel load (both classes are side-effect-free
/// statics — LOGGER/flag enums only — and the same pattern entity_compose
/// uses for Entity).
fn wait_for_class(name: &str, deadline: std::time::Instant, force_after_ms: u64) -> bool {
    let started = std::time::Instant::now();
    loop {
        if cplug_sdk::classes::find_class(name).is_some() {
            return true;
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        if force_after_ms > 0
            && started.elapsed() > std::time::Duration::from_millis(force_after_ms)
        {
            eprintln!("[crussty-plugin] cmp399_devirt: forcing kernel load of {name}");
            crate::improved_noise::force_load_kernel_class(name);
        }
        let sighted = cplug_sdk::classes::is_sighted(name);
        std::thread::sleep(std::time::Duration::from_millis(if sighted {
            2_000
        } else {
            5_000
        }));
    }
}

/// Capture pristine bytes for a target: either the hook already stashed
/// them at class load, or pull them via no-op retransform (READY=false ->
/// stash-only).
fn capture_pristine(t: &'static Target) -> bool {
    if t.orig_is_some() {
        return true;
    }
    for _attempt in 1..=3 {
        let _ = cplug_sdk::retransform_class(t.name);
        if t.orig_is_some() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    t.orig_is_some()
}

/// Compose one stage: take the pristine bytes, run the patcher, arm the
/// hook (patch + READY) and retransform exactly once. Returns true on ARM.
fn arm_stage(
    t: &'static Target,
    patch: impl Fn(&[u8]) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String>,
    strict: fn(&crate::classfile::RetargetOutcome) -> bool,
    stage: &str,
) -> bool {
    let Some(orig) = t.take_orig() else {
        eprintln!("[crussty-plugin] cmp399_devirt: {stage} no pristine bytes, stage dormant");
        return false;
    };
    let orig_len = orig.len();
    match patch(&orig) {
        Ok((p, outcome)) if strict(&outcome) => {
            let composed_len = p.len();
            t.set_patch(p);
            t.ready.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!(
                "[crussty-plugin] cmp399_devirt: stage {stage} ARMED ({outcome:?}), {orig_len} -> {composed_len} bytes, retransform rc={rc}"
            );
            true
        }
        Ok((_p, outcome)) => {
            eprintln!(
                "[crussty-plugin] cmp399_devirt: stage {stage} strict check violated ({outcome:?}), stage stays dormant (vanilla)"
            );
            false
        }
        Err(e) => {
            eprintln!(
                "[crussty-plugin] cmp399_devirt: stage {stage} patch rejected ({e}), stage stays dormant (vanilla)"
            );
            false
        }
    }
}

/// Background activation: wait for boot-quiet, arm D1 (SynchedEntityData
/// fusion), then arm D2 (goal redirect) behind the ItemEntityManager
/// bridge-definition gate. Emits the composition-contract ARM marker.
pub fn activate() {
    if !flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp399_devirt: boot marker not seen, hook stays dormant");
            return;
        }
        // Kernel loader quiet before any define/retransform runs (boot-time
        // class-loading storm discipline).
        std::thread::sleep(std::time::Duration::from_secs(15));

        let mut sites = 0usize;

        // ---- D1: SynchedEntityData.get fusion ----
        let sed = target_sed();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        if wait_for_class(SED_CLASS, deadline, 60_000) && capture_pristine(sed) {
            if arm_stage(
                sed,
                crate::classfile::patch_synched_data_get,
                |o| {
                    matches!(
                        o,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                    )
                },
                "synched_data_get",
            ) {
                sites += 1;
            }
        } else {
            eprintln!(
                "[crussty-plugin] cmp399_devirt: {SED_CLASS} not captured within 180s, D1 dormant (vanilla)"
            );
        }

        // ---- D2: GoalSelector.goalContainsAnyFlags redirect ----
        // Fail-closed gate: the redirected body resolves
        // ItemEntityManager.goalContainsAnyFlags on FIRST EXECUTION — the
        // bridge (items_manager, shared cmp399_* prefix) must be DEFINED
        // before the patched GoalSelector can run a goal tick.
        let gs = target_gs();
        let im_deadline = std::time::Instant::now() + std::time::Duration::from_secs(150);
        let mut bridge_up = false;
        while std::time::Instant::now() < im_deadline {
            if cplug_sdk::classes::find_class(IM_CLASS).is_some() {
                bridge_up = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_500));
        }
        if !bridge_up {
            eprintln!(
                "[crussty-plugin] cmp399_devirt: {IM_CLASS} bridge undefined within 150s, D2 SKIPPED (fail-closed; goal loop stays vanilla)"
            );
        } else if let Err(e) = crate::classfile::devirt_resolution_closure(IM_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp399_devirt: D2 resolution closure failed ({e}), D2 SKIPPED (fail-closed)"
            );
        } else {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
            if wait_for_class(GS_CLASS, deadline, 30_000) && capture_pristine(gs) {
                if arm_stage(
                    gs,
                    crate::classfile::patch_goal_contains_flags,
                    |o| {
                        matches!(
                            o,
                            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                                | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                        )
                    },
                    "goal_contains_flags",
                ) {
                    sites += 1;
                }
            } else {
                eprintln!(
                    "[crussty-plugin] cmp399_devirt: {GS_CLASS} not captured within 120s, D2 dormant (vanilla)"
                );
            }
        }

        crate::kernel_policy::audit_wire(
            "net/minecraft/network/syncher/SynchedEntityData.get + net/minecraft/world/entity/ai/goal/GoalSelector.goalContainsAnyFlags",
            "devirt-fusion/body-redirect",
            "cmp399_devirt v1",
        );
        eprintln!("[crussty-plugin] cmp399_devirt: ARMED sites={sites}");
    });
}

/// The embedded ItemEntityManager bridge bytes (the D2 target static lives
/// there — J's lever class, recompiled with the goalContainsAnyFlags
/// helper; verified against the redirect descriptor at arm time).
const IM_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemEntityManager.class");

/// Test/audit accessor for the embedded bridge bytes (classfile
/// devirt_resolution_closure checks the D2 member against these).
pub fn im_bytes() -> &'static [u8] {
    IM_BYTES
}
