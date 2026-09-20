//! Runtime wiring for the REGION-THREADS lever (S7-156, TASK-295 —
//! ARCH-ATTACK lever #7: region-threaded entity ticking, the last xN-class
//! lane after single-lever exhaustion, feasibility gate S7-155 GREEN).
//!
//! Byte hooks (both strict, fail-closed):
//!  1. `ServerLevel.tick(BooleanSupplier)` — its single
//!     `EntityTickList.forEach(Consumer)` call site retargeted 1:1 to
//!     `RegionTickOps.forEach(EntityTickList,Consumer)` (receiver-prepended
//!     static, identical stack shape).
//!  2. `ServerLevel$EntityCallbacks.onTickingStart/onTickingEnd` — the ONLY
//!     `EntityTickList.add/remove` call sites in the whole kernel (census
//!     S7-156: 3 classes carry EntityTickList CP-refs; ServerLevel has the
//!     field + forEach + contains, callbacks have add/remove), retargeted to
//!     `RegionTickOps.onTickingStart/onTickingEnd` (deferred FIFO during a
//!     parallel phase, drained at the barrier).
//!  3. `Level.guardEntityTick` — the ONLY worker-reachable mid-tick pump
//!     site (S7-157b, live crash 35353820223: Paper pumps main-thread
//!     mid-tick tasks per entity tick; a worker's concurrent poll of the
//!     main-thread-only ServerChunkCache$MainThreadExecutor queue raced a
//!     NoSuchElementException and killed the server 40s into the first leg).
//!     Retargeted to `RegionTickOps.midTickTasks(Level)`: workers suppress,
//!     main thread reproduces the exact vanilla virtual dispatch.
//!  4. `ChunkMap.tick()V -> newTrackerTick()` (S7-158b, live crash 35363758352:
//!     the unchecked trackerEntities sweep NPE'd on a slot nulled by a
//!     parallel worker's entity removal) -> `TrackerTickOps.newTrackerTick(
//!     ChunkMap)` — vanilla sweep byte-for-byte + removal-safe null guard.
//!  5. `Entity.<init>` Mth.createInsecureUUID site (S7-158d, live UUID-dup
//!     WARN in 35363758352: purpur entity-shared-random=true routes EVERY
//!     entity through ONE ThreadUnsafeRandom; parallel constructors aliased
//!     state) -> `RngOps.createInsecureUUID` — vanilla two-draw UUIDv4
//!     serialized per-source (static->static, same descriptor).
//!
//! Bridge: `RegionTickOps` (+ inner `Mut`), `TrackerTickOps`, `RngOps`
//! defined into the KERNEL loader before any patched bytes are served
//! (BRIDGE_READY protocol, the S7-143 LinkageError lesson).
//!
//! Gate: env `CRUSSTY_REGION_THREADS` (integer >= 2 -> on; absent/1 = dormant
//! vanilla passthrough — the Ops also re-parses the same env independently
//! at class-init, so an armed splice with WORKERS=1 degrades to the vanilla
//! forEach INSIDE the bridge: parity intact, benefit zero).
//!
//! Cohabitation (ServerLevel seam): F1 (randomtick) and F3 (tighook) also
//! hook ServerLevel. Byte hooks chain in registration order; this module
//! registers LAST (after tickhook::register) so the tick splice composes on
//! top of F1/F3 bytes. F1/F3 are one-shot retransform guards (their later
//! dispatches return None); a SECOND retransform of ServerLevel would
//! therefore revert their swaps — the bench compose chain never enables
//! F1/F3 alongside region_threads (LOUD WARN if their envs are set).
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch) -> no patch, hook
//! dormant; bridge define failure -> dormant; strict site-count mismatch
//! (Retargeted{sites != 1}) -> dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const SERVER_LEVEL: &str = "net/minecraft/server/level/ServerLevel";
const CALLBACKS_CLASS: &str = "net/minecraft/server/level/ServerLevel$EntityCallbacks";
const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const CHUNKMAP_CLASS: &str = "net/minecraft/server/level/ChunkMap";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/RegionTickOps";
const OPS_INNER_CLASS: &str = "net/minecraft/world/entity/RegionTickOps$Mut";
const OPS_GUARD_CLASS: &str = "net/minecraft/world/entity/RegionTickOps$GuardedNavigatingMobs";
const TRACKER_OPS_CLASS: &str = "net/minecraft/server/level/TrackerTickOps";
const RNG_OPS_CLASS: &str = "net/minecraft/util/RngOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/RegionTickOps.class");
const OPS_INNER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/RegionTickOps$Mut.class");
// S7-170 (TASK-349 delivery fix): nested GuardedNavigatingMobs MUST be defined
// into the kernel loader — RegionTickOps.ensureNavMobsGuarded instantiates it
// on first swap; missing define = NoClassDefFoundError on the first add flow.
const OPS_GUARD_BYTES: &[u8] = include_bytes!(
    "../entityinside/build/net/minecraft/world/entity/RegionTickOps$GuardedNavigatingMobs.class"
);
const TRACKER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/TrackerTickOps.class");
const RNG_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/util/RngOps.class");

/// S7-168 (STEAL v2 defect-fix, TASK-335): BU-DEFER bridge — sendBlockUpdated
/// канализация для воркеров (javap-контракт: handle receiver-prepended).
const BLOCKUPD_CLASS: &str = "net/minecraft/server/level/BlockUpdateOps";
const BLOCKUPD_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/BlockUpdateOps.class");

fn bu_defer_enabled() -> bool {
    std::env::var("CRUSSTY_BU_DEFER")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

fn workers_from_env() -> Option<i64> {
    std::env::var("CRUSSTY_REGION_THREADS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .filter(|w| *w >= 2)
}

fn enabled() -> bool {
    workers_from_env().is_some()
}

/// S7-160 (batch_collector): composition probe — the only swap site lives
/// in RegionTickOps.tickBucket, so the lever requires region_threads >= 2.
pub fn workers_from_env_pub() -> Option<i64> {
    workers_from_env()
}

static READY: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

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

static TARGET_SL: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static TARGET_CB: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static TARGET_LV: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static TARGET_CM: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
// S7-162: the Entity target moved to entity_compose (single compose-chain
// owner); this module no longer registers an Entity hook nor patches Entity.

fn sl_target() -> &'static Target {
    TARGET_SL.get_or_init(|| Target::new(SERVER_LEVEL))
}
fn cb_target() -> &'static Target {
    TARGET_CB.get_or_init(|| Target::new(CALLBACKS_CLASS))
}
fn lv_target() -> &'static Target {
    TARGET_LV.get_or_init(|| Target::new(LEVEL_CLASS))
}
fn cm_target() -> &'static Target {
    TARGET_CM.get_or_init(|| Target::new(CHUNKMAP_CLASS))
}

pub fn bridge_ready() -> bool {
    BRIDGE_READY.load(Ordering::Relaxed)
}

/// S7-162 (entity_compose): pollable wait for the bridge definition — the
/// rng compose stage runs on the entity_compose worker and needs the
/// bridge classes (RngOps et al) DEFINED (not region READY). region READY
/// itself is gated on the rng verdict (wait_rng_verdict) to preserve the
/// "Entity rng failure kills region" semantics without a deadlock.
pub fn wait_bridge_ready_pub(timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(Ordering::Acquire)
}

/// S7-162 (entity_compose): block until the Entity rng-stage verdict is
/// published; false = failed/skipped (region must NOT arm — serialized
/// UUID seeding is a parity precondition of worker parallelism).
pub fn wait_rng_verdict(timeout_ms: u64) -> bool {
    crate::entity_compose::wait_rng_verdict(timeout_ms)
}

/// Register both byte hooks (idempotent; call once from cplugin_init, AFTER
/// tickhook::register so this module's ServerLevel splice sits at the chain
/// tail and composes on top of F1/F3 bytes).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] region_threads: dormant (set CRUSSTY_REGION_THREADS>=2 to enable)"
        );
        return;
    }
    if std::env::var("CRUSSTY_RANDOM_TICK").is_ok() || std::env::var("CRUSSTY_TICK_BLOCK").is_ok()
    {
        eprintln!(
            "[crussty-plugin] region_threads: WARN F1/F3 ServerLevel levers enabled alongside — \
             their one-shot retransform guards revert on any later ServerLevel retransform; \
             this bench chain does not compose them (isolation discipline)"
        );
    }
    // Hook 1: ServerLevel (chain tail — composes F1/F3 bytes).
    cplug_sdk::hooks::register_bytes(SERVER_LEVEL, |_name, bytes| {
        let t = sl_target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: pristine sighting {} {} bytes",
                t.name,
                bytes.len()
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
    // Hook 2: EntityCallbacks (guard sites; no cohabitation on this class).
    cplug_sdk::hooks::register_bytes(CALLBACKS_CLASS, |_name, bytes| {
        let t = cb_target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: pristine sighting {} {} bytes",
                t.name,
                bytes.len()
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
    // Hook 3: Level (S7-157b mid-tick gate site in guardEntityTick).
    cplug_sdk::hooks::register_bytes(LEVEL_CLASS, |_name, bytes| {
        let t = lv_target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: pristine sighting {} {} bytes",
                t.name,
                bytes.len()
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
    // Hook 4: ChunkMap (S7-158b removal-safe tracker sweep site in tick()V).
    cplug_sdk::hooks::register_bytes(CHUNKMAP_CLASS, |_name, bytes| {
        let t = cm_target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: pristine sighting {} {} bytes",
                t.name,
                bytes.len()
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] region_threads: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
    // S7-162: the Entity hook (S7-158d serialized UUID seeding site) moved
    // to entity_compose::register — the single compose-chain owner. This
    // module no longer registers an Entity hook.
}

/// Background activation: wait for ServerLevel, define the RegionTickOps
/// bridge into the kernel loader, compute BOTH retargets from pristine bytes
/// (strict sites==1 each), flip READY, retransform both classes once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(move || {
        let sl = sl_target();
        let cb = cb_target();
        let lv = lv_target();
        let cm = cm_target();

        // ServerLevel loads during server bootstrap (before the first level).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(sl.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] region_threads: {} not loaded within 180s, hook stays dormant",
                    sl.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] region_threads: forcing kernel load of {}",
                    sl.name
                );
                crate::improved_noise::force_load_kernel_class(sl.name);
            }
            let sighted = cplug_sdk::classes::is_sighted(sl.name);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] region_threads: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] region_threads: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in {
            let mut list = vec![
                (OPS_CLASS, OPS_BYTES),
                (OPS_INNER_CLASS, OPS_INNER_BYTES),
                (OPS_GUARD_CLASS, OPS_GUARD_BYTES),
                (TRACKER_OPS_CLASS, TRACKER_BYTES),
                (RNG_OPS_CLASS, RNG_BYTES),
            ];
            if bu_defer_enabled() {
                list.push((BLOCKUPD_CLASS, BLOCKUPD_BYTES));
            }
            list
        } {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] region_threads: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_region_tick_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        // Capture the kernel loader global ref from ServerLevel.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(sl.name) else {
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
            let mut bridge_list: Vec<(&str, &[u8])> = vec![
                (OPS_CLASS, OPS_BYTES),
                (OPS_INNER_CLASS, OPS_INNER_BYTES),
                (OPS_GUARD_CLASS, OPS_GUARD_BYTES),
                (TRACKER_OPS_CLASS, TRACKER_BYTES),
                (RNG_OPS_CLASS, RNG_BYTES),
            ];
            if bu_defer_enabled() {
                bridge_list.push((BLOCKUPD_CLASS, BLOCKUPD_BYTES));
            }
            // ITEMS-SWEEP v2 (ROUND-397 / TASK-397-E): ItemsSweepOps is
            // CO-DEFINED with RegionTickOps — tickBucket references
            // ItemsSweepOps.sweepBucket unconditionally and the class must
            // exist whenever this bridge does. Env gate lives inside the ops
            // class (SWEEP static-init): lever off = static-read no-op,
            // dormant-invisible for the bank.
            bridge_list.push((
                crate::items_sweep::SWEEP_OPS_CLASS,
                crate::items_sweep::SWEEP_OPS_BYTES,
            ));
            let mut ok = true;
            for (name, bytes) in bridge_list {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!(
                            "[crussty-plugin] region_threads: defined {name} in kernel loader"
                        );
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!(
                            "[crussty-plugin] region_threads: define_class({name}) failed"
                        );
                        ok = false;
                        break;
                    }
                }
            }
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] region_threads: bridge definition aborted, hook stays dormant"
            );
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);

        // Pristine bytes for all four targets (hook stash or no-op
        // retransform).
        for t in [sl, cb, lv, cm] {
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] region_threads: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] region_threads: no pristine bytes for {}, hook stays dormant",
                        t.name
                    );
                    return;
                }
            }
        }

        // Compute patches (strict: Retargeted{sites:1} per class; callbacks
        // carry add+remove in one patch — 1+1).
        let Some(sl_orig) = sl.take_orig() else { return };
        let Some(cb_orig) = cb.take_orig() else { return };
        let sl_major = crate::improved_noise::class_version(&sl_orig)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let cb_major = crate::improved_noise::class_version(&cb_orig)
            .map(|(m, _)| m)
            .unwrap_or(0);

        let (sl_patched, sl_outcome) =
            match crate::classfile::patch_region_tick_serverlevel(&sl_orig) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] region_threads: ServerLevel patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
        if !matches!(
            sl_outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
        ) {
            eprintln!(
                "[crussty-plugin] region_threads: ServerLevel strict site-count violated ({sl_outcome:?}), hook stays dormant"
            );
            return;
        }
        // S7-168 BU-DEFER (STEAL v2 defect-fix): compose the sendBlockUpdated
        // body-redirect INTO the ServerLevel bytes (after the lambda$tick$4
        // retarget). Strict: exactly ONE site; closure guard ran at boot.
        let sl_patched = if bu_defer_enabled() {
            let (p2, bu_outcome) =
                match crate::classfile::patch_serverlevel_send_block_updated(&sl_patched) {
                    Ok(pair) => pair,
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] region_threads: BU-DEFER ServerLevel patch rejected ({e}), hook stays dormant"
                        );
                        return;
                    }
                };
            if !matches!(
                bu_outcome,
                crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
            ) {
                eprintln!(
                    "[crussty-plugin] region_threads: BU-DEFER strict site-count violated ({bu_outcome:?}), hook stays dormant"
                );
                return;
            }
            eprintln!(
                "[crussty-plugin] region_threads: BU-DEFER composed: sendBlockUpdated -> BlockUpdateOps.handle (sites:1)"
            );
            p2
        } else {
            sl_patched
        };
        let (cb_patched, (cb_out_add, cb_out_rem)) =
            match crate::classfile::patch_region_tick_callbacks(&cb_orig) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] region_threads: EntityCallbacks patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
        if !matches!(
            (cb_out_add, cb_out_rem),
            (
                crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
                crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
            )
        ) {
            eprintln!(
                "[crussty-plugin] region_threads: EntityCallbacks strict site-count violated ({cb_out_add:?} / {cb_out_rem:?}), hook stays dormant"
            );
            return;
        }
        let Some(lv_orig) = lv.take_orig() else { return };
        let (lv_patched, lv_outcome) =
            match crate::classfile::patch_region_tick_guardentity(&lv_orig) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] region_threads: Level patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
        if !matches!(
            lv_outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
        ) {
            eprintln!(
                "[crussty-plugin] region_threads: Level strict site-count violated ({lv_outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] region_threads: computed patches (ServerLevel {} -> {} bytes {sl_outcome:?}; EntityCallbacks {} -> {} bytes add={cb_out_add:?} remove={cb_out_rem:?}; Level {} -> {} bytes {lv_outcome:?})",
            sl_orig.len(),
            sl_patched.len(),
            cb_orig.len(),
            cb_patched.len(),
            lv_orig.len(),
            lv_patched.len()
        );
        let (sl_len, sl_len_patched) = (sl_orig.len(), sl_patched.len());
        sl.set_patch(PatchCache {
            bytes: Arc::from(sl_patched),
            major: sl_major,
        });
        let (cb_len, cb_len_patched) = (cb_orig.len(), cb_patched.len());
        cb.set_patch(PatchCache {
            bytes: Arc::from(cb_patched),
            major: cb_major,
        });
        let (lv_len, lv_len_patched) = (lv_orig.len(), lv_patched.len());
        let lv_major = crate::improved_noise::class_version(&lv_orig)
            .map(|(m, _)| m)
            .unwrap_or(0);
        lv.set_patch(PatchCache {
            bytes: Arc::from(lv_patched),
            major: lv_major,
        });

        // S7-158b: removal-safe tracker sweep (ChunkMap.tick()V call site).
        let Some(cm_orig) = cm.take_orig() else { return };
        let (cm_patched, cm_outcome) =
            match crate::classfile::patch_region_tracker_chunkmap(&cm_orig) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] region_threads: ChunkMap patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
        if !matches!(
            cm_outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
        ) {
            eprintln!(
                "[crussty-plugin] region_threads: ChunkMap strict site-count violated ({cm_outcome:?}), hook stays dormant"
            );
            return;
        }
        let cm_major = crate::improved_noise::class_version(&cm_orig)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let (cm_len, cm_len_patched) = (cm_orig.len(), cm_patched.len());
        cm.set_patch(PatchCache {
            bytes: Arc::from(cm_patched),
            major: cm_major,
        });

        // S7-162: the Entity rng patch (S7-158d) and the batch collector
        // ctor retarget (S7-161) moved to entity_compose — the single
        // compose-chain owner (hooks on one class supersede each other:
        // leg #5 886/895 evidence). This module waits for the rng-stage
        // verdict and stays dormant if it failed — serialized UUID seeding
        // is a parity precondition of worker parallelism.
        if !crate::entity_compose::wait_rng_verdict(180_000) {
            eprintln!(
                "[crussty-plugin] region_threads: Entity rng stage not composed (entity_compose verdict failed/timeout), hook stays dormant"
            );
            return;
        }

        eprintln!(
            "[crussty-plugin] region_threads: computed patches (ServerLevel {sl_len} -> {sl_len_patched} bytes {sl_outcome:?}; EntityCallbacks {cb_len} -> {cb_len_patched} bytes add={cb_out_add:?} remove={cb_out_rem:?}; Level {lv_len} -> {lv_len_patched} bytes {lv_outcome:?}; ChunkMap {cm_len} -> {cm_len_patched} bytes {cm_outcome:?}; Entity rng+chain via entity_compose)"
        );

        // Single READY flip, then retransform the four classes once
        // (Entity retransform is entity_compose's).
        crate::kernel_policy::audit_wire(
            OPS_CLASS,
            "forEach/onTickingStart/onTickingEnd/midTickTasks/trackerTick/rngUUID",
            "region_threads v4",
        );
        READY.store(true, Ordering::Release);
        let rc_sl = cplug_sdk::retransform_class(sl.name);
        let rc_cb = cplug_sdk::retransform_class(cb.name);
        let rc_lv = cplug_sdk::retransform_class(lv.name);
        let rc_cm = cplug_sdk::retransform_class(cm.name);
        eprintln!(
            "[crussty-plugin] region_threads: ARMED, retransform rc ServerLevel={rc_sl} EntityCallbacks={rc_cb} Level={rc_lv} ChunkMap={rc_cm} (Entity via entity_compose)"
        );
    });
}

#[cfg(test)]
mod blockupd_delivery_tests {
    /// S7-168 delivery-graph guard (mirror of skipstore discipline):
    /// BlockUpdateOps.java MUST declare ZERO nested classes — the bridge
    /// compiles to exactly one classfile and is defined alone into the
    /// kernel loader.
    #[test]
    fn blockupd_ops_source_declares_no_nested_classes() {
        let src =
            include_str!("../entityinside/net/minecraft/server/level/BlockUpdateOps.java");
        let mut declared: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") {
                        let rest = &t[i + pat.len()..];
                        let name: String = rest
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty() {
                            declared.push(name);
                        }
                    }
                    break;
                }
            }
        }
        assert!(
            declared.is_empty(),
            "BlockUpdateOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile; nested classes would crash the \
             server with NoClassDefFoundError (S7-163 leg#1 TECH-DUD)"
        );
    }

    /// Build-dir mirror: exactly one BlockUpdateOps classfile exists.
    #[test]
    fn blockupd_build_dir_has_exactly_one_classfile() {
        let dir = "entityinside/build/net/minecraft/server/level";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_blockupd_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("BlockUpdateOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(
            count, 1,
            "BlockUpdateOps classfile set drifted — must compile to exactly ONE classfile"
        );
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn blockupd_embedded_bytes_match_build_dir() {
        let on_disk =
            std::fs::read("entityinside/build/net/minecraft/server/level/BlockUpdateOps.class")
                .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::BLOCKUPD_BYTES,
            "embedded BlockUpdateOps.class is stale — rerun scripts/build_blockupd_ops.sh"
        );
    }

    /// Resolution closure: embedded bridge declares the receiver-prepended
    /// handle the ServerLevel retarget emits.
    #[test]
    fn blockupd_embedded_declares_all_redirect_targets() {
        if let Err(e) = crate::classfile::blockupd_resolution_closure(super::BLOCKUPD_BYTES) {
            panic!(
                "RESOLUTION CLOSURE FAILED: {e} — rebuild entityinside/ via build_blockupd_ops.sh"
            );
        }
    }

    /// Scope lock: EXACTLY the sendBlockUpdated target (single-site lever).
    #[test]
    fn blockupd_redirect_table_is_exactly_sendblockupdated() {
        let targets = crate::classfile::BLOCKUPD_REDIRECT_TARGETS;
        assert_eq!(targets.len(), 1, "S7-168 BU-DEFER is a SINGLE-SITE lever");
        assert_eq!(targets[0].0, "sendBlockUpdated");
        assert_eq!(
            targets[0].1,
            "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V"
        );
        assert_eq!(targets[0].2, "handle");
        assert_eq!(
            targets[0].3,
            "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V"
        );
    }
}
