//! Runtime wiring for the SINGLE ENTITY COMPOSE-CHAIN (S7-162, TASK-304).
//!
//! S7-160/S7-161 evidence (leg #5 35381522360 stdout lines 886/895): TWO
//! byte hooks on net/minecraft/world/entity/Entity (inside_cache and
//! region_threads) each serve a WHOLE-CLASS cached byte array computed
//! from their own pristine stash. `cplug_sdk::hooks::dispatch_bytes` does
//! chain callbacks (each receives the previous output), but a whole-class
//! serve REPLACES the accumulated bytes, so the later writer silently
//! SUPersedes the earlier patch: leg #5 served rng-only 205494 over the
//! inside bytes 205522 (inside_cache gate dead in the v2 bank); S7-161
//! hit the reverse order by luck (205546 = rng+batch).
//!
//! This module is the SINGLE OWNER of the Entity byte pipeline. Five
//! composable stages are applied strictly in this order on ONE
//! accumulated buffer, each stage strict about its own site count and
//! fail-dominant (a rejected stage logs loudly and the chain continues
//! WITHOUT it — vanilla behaviour for that concern; no stage ever
//! invents semantics):
//!
//!   1. inside_cache   `classfile::patch_inside_cache`          (Retargeted/AlreadyPatched)
//!   2. fluid_free     `fluid_free::compose_entity`             (Option)
//!   3. fluid_dirty    `fluid_dirty::compose_entity`            (Option)
//!   4. region rng     `classfile::patch_region_rng_entity`     (strict sites==1)
//!   5. batch ctor     `classfile::patch_entity_collector_ctor` (strict sites==1)
//!   6. traversal      `classfile::patch_entity_traversal`      (strict sites==1)
//!   7. zeroin         `classfile::patch_entity_zeroalloc`      (strict sites==3,
//!      METHOD-BODY redirects: collidedWithFluid/collidedWithShapeMovingFrom/
//!      updateFluidHeightAndDoFluidPushing -> ZeroAllocOps statics, S7-164)
//!
//! Cross-module contract (region_threads): the region lever must NOT arm
//! its parallel ticking unless the rng stage composed successfully
//! (serialized UUID seeding is a parity precondition of worker
//! parallelism). This module publishes the rng verdict BEFORE the batch
//! stage wait; `region_threads::activate` calls `wait_rng_verdict` and
//! aborts on failure — the same "Entity rng failure kills region"
//! semantics as the pre-compose code, without the deadlock (the verdict
//! needs only the region BRIDGE classes defined, not region READY).
//!
//! Ordering: the Entity retransform happens EXACTLY ONCE (here), after
//! every enabled stage's bridge had its `wait_bridge_ready` window.
//! Population inject (INJECTS-ONLY protocol) starts after the ARMED
//! markers, so ~100% of the population is constructed through whatever
//! ctor shape this pipeline composed.
//!
//! Fail-closed matrix: no pristine bytes -> dormant (Entity vanilla);
//! bridge window missed for a stage -> that stage skipped (loud WARN);
//! strict site-count violation -> that stage skipped (rng additionally
//! publishes the failure verdict that disarms region_threads); hook
//! serve before READY -> vanilla passthrough (stash only).

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";

/// rng-stage verdict for region_threads: 0 = pending, 1 = composed
/// (strict sites==1), 2 = failed/skipped (region must stay dormant).
static RNG_VERDICT: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Poison recovery (TASK-46): locks wrap plain Vec/Arc stores only.
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
    TARGET.get_or_init(|| Target::new(ENTITY_CLASS))
}

fn stage_enabled() -> bool {
    crate::inside_cache::enabled_pub()
        || crate::inside_batch::enabled_pub() // TASK-460-01 (ID-P31): stage-1 sibling/supersede
        || crate::fluid_free::enabled_pub()
        || crate::fluid_dirty::enabled_pub()
        || crate::region_threads::workers_from_env_pub().is_some()
        || crate::batch_collector::enabled_pub()
        || crate::traversal::enabled_pub()
        || crate::zero_alloc::enabled_pub()
        || crate::skip_store::enabled_pub()
        || crate::inside_snap::enabled_pub()
}

/// Register the single Entity byte hook (idempotent; call once from
/// cplugin_init). The callback performs NO JNI/class-file work
/// (loader-lock discipline — pristine capture at the class's own load,
/// patch served from the cache computed on the quiet activation worker).
pub fn register() {
    if !stage_enabled() {
        eprintln!("[crussty-plugin] entity_compose: dormant (no Entity-stage lever enabled)");
        return;
    }
    if std::env::var("CRUSSTY_FLUID_GUARD").is_ok() {
        eprintln!(
            "[crussty-plugin] entity_compose: WARN legacy fluid_guard Entity hook enabled \
             alongside — it serves whole-class bytes from its own cache and would \
             supersede the compose chain (isolation discipline: do not compose them)"
        );
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker; never rewrite here.
            eprintln!(
                "[crussty-plugin] entity_compose: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes)
                    .map(|(m, _)| m)
                    .unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed compose; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] entity_compose: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Publish the rng-stage verdict (region_threads polls this BEFORE arming).
pub fn wait_rng_verdict(timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    loop {
        match RNG_VERDICT.load(Ordering::Acquire) {
            1 => return true,
            2 => return false,
            _ => {}
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

/// Background activation: wait for the kernel Entity class + boot marker,
/// apply every enabled stage in the fixed order on ONE accumulated
/// buffer, publish the rng verdict, flip READY and retransform Entity
/// exactly once.
pub fn activate() {
    if !stage_enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // Wait for the kernel Entity class (loads at boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] entity_compose: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] entity_compose: forcing kernel load of {}",
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

        // Kernel loader must be quiet before any bridge define runs (the
        // stage bridges define themselves on their own workers; this worker
        // only waits for them) — boot-time class-loading storm discipline.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] entity_compose: boot marker not seen, hook stays dormant");
            return;
        }

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard
        // pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] entity_compose: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] entity_compose: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }
        let Some(mut bytes) = t.take_orig() else {
            return;
        };
        let orig_len = bytes.len();
        let major = crate::improved_noise::class_version(&bytes)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let mut chain: Vec<&str> = Vec::new();

        // ---- STAGE 1: inside_cache | inside_batch (discovery gate retarget) ----
        // TASK-460-01 (ID-P31): S7-162 supersede discipline — the site has
        // EXACTLY ONE owner: armed inside_batch supersedes inside_cache
        // (batchGate bridge); otherwise inside_cache owns the site as before.
        if crate::inside_batch::enabled_pub() {
            if crate::inside_batch::wait_bridge_ready(180_000) {
                match crate::classfile::patch_inside_batch(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { .. }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_batch composed (supersede inside_cache, {outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_batch");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_batch strict check violated ({outcome:?}), chain continues WITHOUT inside (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_batch patch rejected ({e}), chain continues WITHOUT inside (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_batch bridge missed its window, chain continues WITHOUT inside (fail-dominant)"
                );
            }
        } else if crate::inside_cache::enabled_pub() {
            if crate::inside_cache::wait_bridge_ready(180_000) {
                match crate::classfile::patch_inside_cache(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { .. }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_cache composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_cache strict check violated ({outcome:?}), chain continues WITHOUT inside (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_cache patch rejected ({e}), chain continues WITHOUT inside (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_cache bridge missed its window, chain continues WITHOUT inside (fail-dominant)"
                );
            }
        }

        // ---- STAGE 1b: inside_bitmask (section all-air pre-gate, TASK-357) ----
        if crate::inside_bitmask::enabled_pub() {
            if crate::inside_bitmask::wait_bridge_ready(180_000) {
                match crate::classfile::patch_inside_bitmask(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { .. }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_bitmask composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_bitmask");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_bitmask strict check violated ({outcome:?}), chain continues WITHOUT inside_bitmask (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_bitmask patch rejected ({e}), chain continues WITHOUT inside_bitmask (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_bitmask bridge missed its window, chain continues WITHOUT inside_bitmask (fail-dominant)"
                );
            }
        }

        // ---- STAGE 1c: inside_snap (cmp424_inside: snapshot gate retarget) ----
        // ТОЛЬКО ОДИН getBlockState-сайт (Entity.lambda$checkInsideBlocks$2);
        // snapshot-плоскость + секWrite-инвалидация живут в inside_snap.rs.
        if crate::inside_snap::enabled_pub() {
            if crate::inside_snap::wait_bridge_ready(180_000) {
                match crate::classfile::patch_inside_snap_gate(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_snap composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_snap");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_snap strict check violated ({outcome:?}), chain continues WITHOUT inside_snap (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_snap patch rejected ({e}), chain continues WITHOUT inside_snap (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_snap bridge missed its window, chain continues WITHOUT inside_snap (fail-dominant)"
                );
            }
        }

        // ---- STAGE 1d: inside_fluid (R468-S18 fluid-empty fastpath) ----
        // THE single collidedWithFluid site in lambda$checkInsideBlocks$2 ->
        // InsideFluidOps.gate (receiver-first 3B->3B). Disjoint from the
        // stage-1 gate site (inside_cache/inside_batch supersede pair) and
        // from the stage-1c getBlockState site (inside_snap) — three owners,
        // three sites, one compose chain (S7-162 discipline).
        if crate::inside_fluid::enabled_pub() {
            if crate::inside_fluid::wait_bridge_ready(60_000) {
                match crate::classfile::patch_inside_fluid(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_fluid composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_fluid");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_fluid strict check violated ({outcome:?}), chain continues WITHOUT inside_fluid (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_fluid patch rejected ({e}), chain continues WITHOUT inside_fluid (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_fluid bridge missed its window, chain continues WITHOUT inside_fluid (fail-dominant)"
                );
            }
        }

        // ---- STAGE 2: fluid_free (fgate wrapper retarget) ----
        if crate::fluid_free::enabled_pub() {
            if crate::fluid_free::wait_bridge_ready(60_000) {
                match crate::fluid_free::compose_entity(&bytes) {
                    Some(p) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage fluid_free composed (fgate)"
                        );
                        bytes = p;
                        chain.push("fluid_free");
                    }
                    None => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage fluid_free rejected, chain continues WITHOUT fluid_free (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: fluid_free bridge missed its window, chain continues WITHOUT fluid_free (fail-dominant)"
                );
            }
        }

        // ---- STAGE 3: fluid_dirty (scan wrapper retarget) ----
        if crate::fluid_dirty::enabled_pub() {
            if crate::fluid_dirty::wait_bridge_ready(60_000) {
                match crate::fluid_dirty::compose_entity(&bytes) {
                    Some(p) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage fluid_dirty composed (scan)"
                        );
                        bytes = p;
                        chain.push("fluid_dirty");
                    }
                    None => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage fluid_dirty rejected, chain continues WITHOUT fluid_dirty (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: fluid_dirty bridge missed its window, chain continues WITHOUT fluid_dirty (fail-dominant)"
                );
            }
        }

        // ---- STAGE 4: region rng (serialized UUID seeding) ----
        // Verdict is published BEFORE the batch wait so region_threads never
        // blocks on the batch bridge window.
        if crate::region_threads::workers_from_env_pub().is_some() {
            let mut rng_ok = false;
            if crate::region_threads::wait_bridge_ready_pub(180_000) {
                match crate::classfile::patch_region_rng_entity(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage region_rng composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("rng");
                        rng_ok = true;
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage region_rng strict site-count violated ({outcome:?}), chain continues WITHOUT rng (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage region_rng patch rejected ({e}), chain continues WITHOUT rng (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: region bridge missed its window, chain continues WITHOUT rng (fail-dominant)"
                );
            }
            RNG_VERDICT.store(if rng_ok { 1 } else { 2 }, Ordering::Release);
        } else {
            // Region lever off: nobody polls the verdict; mark n/a as failed
            // so a stray poller fails closed.
            RNG_VERDICT.store(2, Ordering::Release);
        }

        // ---- STAGE 5: batch collector ctor retarget ----
        if crate::batch_collector::enabled_pub() {
            if crate::batch_collector::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_collector_ctor(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage batch_ctor composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("batch");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage batch_ctor strict site-count violated ({outcome:?}), chain continues WITHOUT batch (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage batch_ctor patch rejected ({e}), chain continues WITHOUT batch (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: batch bridge missed its window, chain continues WITHOUT batch (fail-dominant)"
                );
            }
        }

        // ---- STAGE 6: flat traversal retarget (S7-163 lever #9) ----
        if crate::traversal::enabled_pub() {
            if crate::traversal::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_traversal(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traversal composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("traversal");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traversal strict site-count violated ({outcome:?}), chain continues WITHOUT traversal (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traversal patch rejected ({e}), chain continues WITHOUT traversal (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: traversal bridge missed its window, chain continues WITHOUT traversal (fail-dominant)"
                );
            }
        }

        // ---- STAGE 7: zero-alloc inside/fluid body-redirects (S7-164 lever #10) ----
        // Composite: ALL THREE Entity bodies must redirect or the stage is
        // skipped as a whole (fail-dominant, no half-composed stage).
        if crate::zero_alloc::enabled_pub() {
            if crate::zero_alloc::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_zeroalloc(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 3 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage zeroin composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("zeroin");
                    }
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::AlreadyPatched { sites: 3 }
                    ) => {
                        // Idempotent re-sight (stale retransform replay): the
                        // redirects are already in place — treat as composed.
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage zeroin composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("zeroin");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage zeroin strict check violated ({outcome:?}), chain continues WITHOUT zeroin (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage zeroin patch rejected ({e}), chain continues WITHOUT zeroin (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: zero_alloc bridge missed its window, chain continues WITHOUT zeroin (fail-dominant)"
                );
            }
        }

        // ---- STAGE 8: skip-store-bb value-equal putfield skip (#13-SBB, S7-166) ----
        // Single-site body redirect of Entity.setBoundingBox(AABB) to the
        // SkipStoreOps bridge (javap-verbatim normalization + bit-equal
        // store-skip; RECON-12a parity contract: 0 identity sites on bb,
        // AABB immutable, bit-equality strictly stronger than dcmp).
        if crate::skip_store::enabled_pub() {
            if crate::skip_store::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_skip_store_bb(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage sbb composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("sbb");
                    }
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                    ) => {
                        // Idempotent re-sight (stale retransform replay).
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage sbb composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("sbb");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage sbb strict check violated ({outcome:?}), chain continues WITHOUT sbb (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage sbb patch rejected ({e}), chain continues WITHOUT sbb (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: skip_store bridge missed its window, chain continues WITHOUT sbb (fail-dominant)"
                );
            }
        }

        // ---- STAGE 9: inside-diet body redirect (TASK-332 lever #12 v1) ----
        // Single-site body redirect of the private 5-arg checkInsideBlocks to
        // the InsideDietOps bridge (glue-free per-call allocations, vanilla
        // static walk). Fail-dominant like every other stage.
        if crate::inside_diet::enabled_pub() {
            if crate::inside_diet::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_inside_diet(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_diet composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_diet");
                    }
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
                    ) => {
                        // Idempotent re-sight (stale retransform replay).
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_diet composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("inside_diet");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_diet strict check violated ({outcome:?}), chain continues WITHOUT inside_diet (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage inside_diet patch rejected ({e}), chain continues WITHOUT inside_diet (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: inside_diet bridge missed its window, chain continues WITHOUT inside_diet (fail-dominant)"
                );
            }
        }

        // ---- STAGE 10: travel-diet body redirect (RECON-21, lever #14 v2a) ----
        // Single-site body redirect of the private Entity.collide(Vec3) to
        // the TravelDietOps bridge (scalar scratch-slot mirror of the vanilla
        // body — AABB/ArrayList/FloatArraySet temporaries die, products stay
        // allocations, bit-exact double order). Fail-dominant like every
        // other stage.
        if crate::travel_diet::enabled_pub() {
            if crate::travel_diet::wait_bridge_ready(120_000) {
                match crate::classfile::patch_entity_traveldiet(&bytes) {
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 2 }
                    ) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traveldiet composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("traveldiet");
                    }
                    Ok((p, outcome)) if matches!(
                        outcome,
                        crate::classfile::RetargetOutcome::AlreadyPatched { sites: 2 }
                    ) => {
                        // Idempotent re-sight (stale retransform replay).
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traveldiet composed ({outcome:?})"
                        );
                        bytes = p;
                        chain.push("traveldiet");
                    }
                    Ok((_p, outcome)) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traveldiet strict check violated ({outcome:?}), chain continues WITHOUT traveldiet (fail-dominant)"
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] entity_compose: stage traveldiet patch rejected ({e}), chain continues WITHOUT traveldiet (fail-dominant)"
                        );
                    }
                }
            } else {
                eprintln!(
                    "[crussty-plugin] entity_compose: travel_diet bridge missed its window, chain continues WITHOUT traveldiet (fail-dominant)"
                );
            }
        }

        let composed_len = bytes.len();
        t.set_patch(PatchCache {
            bytes: Arc::from(bytes),
            major,
        });

        crate::kernel_policy::audit_wire(
            ENTITY_CLASS,
            "inside-gate/fgate/scan/rngUUID/collector-ctor/traversal/zeroin/skip-store-bb/travel-diet",
            "entity_compose v5",
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!(
            "[crussty-plugin] entity_compose: ARMED chain [{}], {} -> {} bytes, retransform rc={rc}",
            chain.join("->"),
            orig_len,
            composed_len
        );
    });
}
