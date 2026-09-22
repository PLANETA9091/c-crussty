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

/// NAV-POOL (TASK-410-A k5, cmp405_navplane STRICT eq): A* node-pool bridge
/// — prepare launders the node map to the fresh shape instead of clearing,
/// getNode drops the per-call lambda (get + position check + new-on-miss).
/// Same package as NodeEvaluator (protected `nodes` field access).
const NAVPOOL_CLASS: &str = "net/minecraft/world/level/pathfinder/NavPoolOps";
const NAVPOOL_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/level/pathfinder/NavPoolOps.class");
const NODE_EVALUATOR_CLASS: &str = "net/minecraft/world/level/pathfinder/NodeEvaluator";

/// REFSYNC (TASK-412-A, cmp405_navplane lane via crate::emap::armed()): the
/// SEVEN additional ReferenceList mutator-fence targets (ServerLevel is
/// composed separately in the sl compose chain; ChunkMap itself carries no
/// add/remove/contains sites — only its inner TrackedEntity, listed here).
/// Census + helpers: classfile::patch_referencelist_callsites /
/// EntityMapOps.refList* (a-k5b leg evidence: Reference2IntOpenHashMap.find
/// infinite probe <- ReferenceList.add <- entityStartLoaded, main-thread
/// injection vs region-worker unload removes).
const REFSYNC_CLASSES: [&str; 7] = [
    "io/papermc/paper/threadedregions/EntityScheduler$EntitySchedulerTickList",
    "ca/spottedleaf/moonrise/paper/util/BaseChunkSystemHooks",
    "ca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk",
    "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/server/ServerEntityLookup",
    "org/bukkit/craftbukkit/CraftWorld",
    "net/minecraft/server/level/ChunkHolder",
    "net/minecraft/server/level/ChunkMap$TrackedEntity",
];

/// ENTITYMAP FENCE (TASK-411-A k5b, cmp405_navplane lane): the three flat
/// bridge classes (ZERO nested) are owned by crate::emap; the ChunkMap
/// compose appends the 12-site fence to the tracker-patched bytes.

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
// TASK-410-A k5: NodeEvaluator target (navpool compose owner; fail-open).
static TARGET_NE: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
// TASK-412-A: the seven ReferenceList fence targets (refsync compose).
static TARGET_RS: std::sync::OnceLock<Vec<Target>> = std::sync::OnceLock::new();

fn rs_targets() -> &'static [Target] {
    TARGET_RS.get_or_init(|| REFSYNC_CLASSES.iter().map(|n| Target::new(n)).collect())
}
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
fn ne_target() -> &'static Target {
    TARGET_NE.get_or_init(|| Target::new(NODE_EVALUATOR_CLASS))
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
    // Hook 5 (TASK-410-A k5, cmp405_navplane STRICT eq): NodeEvaluator —
    // the A* node-pool compose target. Registered ONLY when armed; empty/
    // other flag = no hook, no definition, no retarget (vanilla bit-in-byte).
    if crate::nav_pool::armed() {
        cplug_sdk::hooks::register_bytes(NODE_EVALUATOR_CLASS, |_name, bytes| {
            let t = ne_target();
            if !READY.load(Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] navpool: pristine sighting {} {} bytes",
                    t.name,
                    bytes.len()
                );
                t.stash_orig(bytes);
                return None;
            }
            let cached = t.patch_bytes();
            if !t.served.swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] navpool: hook serve {} {} bytes",
                    t.name,
                    cached.as_ref().map(|c| c.len()).unwrap_or(0)
                );
            }
            cached.map(|c| c.to_vec())
        });
    }
    // Hook 6 (TASK-412-A, cmp405_navplane lane): the seven ReferenceList
    // mutator-fence targets. Registered ONLY when armed; empty/other flag
    // = no hook, no capture, no retarget (vanilla bit-in-byte).
    if crate::emap::armed() {
        for t in rs_targets() {
            cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
                if !READY.load(Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] refsync: pristine sighting {} {} bytes",
                        t.name,
                        bytes.len()
                    );
                    t.stash_orig(bytes);
                    return None;
                }
                let cached = t.patch_bytes();
                if !t.served.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] refsync: hook serve {} {} bytes",
                        t.name,
                        cached.as_ref().map(|c| c.len()).unwrap_or(0)
                    );
                }
                cached.map(|c| c.to_vec())
            });
        }
    }
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
        // TASK-413-A A/B proof marker (grep-able, one-shot): the race-fence
        // sub-gate state at activation. leg1 (fence) must log racefence=ON,
        // leg2 (nofence) racefence=OFF — the emap/refsync compose and hooks
        // follow this gate verbatim; navplane/navpool do not.
        eprintln!(
            "[crussty-plugin] region_threads: racefence={} (emap+refsync A/B sub-gate; default={} lane-lever={})",
            if crate::emap::race_fence_on() { "ON" } else { "OFF" },
            crate::emap::RACE_FENCE_DEFAULT,
            crate::nav_plane::armed(),
        );
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
            if crate::nav_pool::armed() {
                list.push((NAVPOOL_CLASS, NAVPOOL_BYTES));
            }
            // TASK-411-A: emap fence classes join the major-version
            // pre-check when the lane lever is armed.
            if crate::emap::armed() {
                for (n, b) in crate::emap::define_list() {
                    list.push((n, b));
                }
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
                return (false, false);
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return (false, false);
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
                return (false, false);
            };
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return (false, false);
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
            // NAV-PLANE (TASK-405-A, cmp405_navplane STRICT eq): define the
            // NavPlaneOps bridge ONLY when armed; empty flag = not defined,
            // not registered, not retargeted -> vanilla bit-in-byte.
            if crate::nav_plane::armed() {
                bridge_list.push((crate::nav_plane::NAV_CLASS, crate::nav_plane::NAV_BYTES));
                // NAV-POOL (TASK-410-A k5, same lever): the pool bridge is
                // part of the nav vector — defined+registered under the
                // exact same STRICT eq gate.
                bridge_list.push((NAVPOOL_CLASS, NAVPOOL_BYTES));
            }
            // ENTITYMAP FENCE (TASK-411-A k5b, same lever): define the
            // three flat fence classes BEFORE the ChunkMap retransform —
            // the fence helpers must resolve when the first fenced call
            // site executes.
            if crate::emap::armed() {
                for (n, b) in crate::emap::define_list() {
                    bridge_list.push((n, b));
                }
            }
            let mut ok = true;
            let mut emap_defined = !crate::emap::armed();
            for (name, bytes) in bridge_list {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        if name == crate::nav_plane::NAV_CLASS {
                            // RegisterNatives navDecide BEFORE flipping any
                            // READY latch: first armed handle() call must
                            // bind, else the java ERR ladder goes vanilla.
                            if !crate::nav_plane::register_native(env, c) {
                                ok = false;
                            }
                        }
                        if name == NAVPOOL_CLASS {
                            // RegisterNatives navPoolTick BEFORE READY: the
                            // first armed prepare() call must bind (telemetry
                            // failure is swallowed java-side, but the bind
                            // must be in place for the ARM/EFFECT markers).
                            if !crate::nav_pool::register_native(env, c) {
                                ok = false;
                            }
                        }
                        if crate::emap::armed()
                            && crate::emap::define_list().iter().any(|(n, _)| *n == name)
                        {
                            // Probe-then-patch: the define IS the emap ARM
                            // probe — a failed define leaves the fence
                            // uncomposed (vanilla entityMap).
                            emap_defined = true;
                        }
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
                        if crate::emap::armed()
                            && crate::emap::define_list().iter().any(|(n, _)| *n == name)
                        {
                            emap_defined = false;
                        }
                        break;
                    }
                }
            }
            (ok, emap_defined)
        });
        let Some((ok_defined, emap_defined)) = defined else {
            eprintln!(
                "[crussty-plugin] region_threads: bridge definition aborted, hook stays dormant"
            );
            return;
        };
        if !ok_defined {
            eprintln!(
                "[crussty-plugin] region_threads: bridge definition aborted, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] region_threads: emap fence define probe = {}",
            if emap_defined { "OK (armed, will compose)" } else { "FAILED (entityMap stays vanilla)" }
        );
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
        // NAV-PLANE compose (TASK-405-A, cmp405_navplane STRICT eq):
        // independent of bu_defer so the armed delta vs the vanilla anchor is
        // PURELY the nav-batch read plane (same javap-verbatim body, batched
        // shouldRecomputePath decisions). Strict: exactly ONE site.
        let sl_patched = if crate::nav_plane::armed() {
            let (p3, nav_outcome) =
                match crate::classfile::patch_serverlevel_send_block_updated_navplane(&sl_patched)
                {
                    Ok(pair) => pair,
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] navplane: ServerLevel patch rejected ({e}), lever stays vanilla"
                        );
                        return;
                    }
                };
            if !matches!(
                nav_outcome,
                crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
            ) {
                eprintln!(
                    "[crussty-plugin] navplane: strict site-count violated ({nav_outcome:?}), lever stays vanilla"
                );
                return;
            }
            p3
        } else {
            sl_patched
        };
        // REFSYNC compose (TASK-412-A, cmp405_navplane lane): the ServerLevel
        // ReferenceList sites (add x2 remove x2 — currentlyTicking entity
        // bookkeeping) join the compose chain AFTER the navplane retarget.
        // Strict: exactly 4 sites; any violation leaves ServerLevel's
        // ReferenceList calls vanilla (fail-dominant, chain stops here).
        let sl_patched = if crate::emap::armed() && emap_defined {
            match crate::classfile::patch_referencelist_callsites(&sl_patched) {
                Ok((p, o))
                    if matches!(
                        o,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 4 }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 4 }
                    ) =>
                {
                    eprintln!(
                        "[crussty-plugin] refsync: ServerLevel ReferenceList fence composed ({o:?})"
                    );
                    p
                }
                Ok((_, o)) => {
                    eprintln!(
                        "[crussty-plugin] refsync: ServerLevel strict site-count violated ({o:?}), hook stays dormant"
                    );
                    return;
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] refsync: ServerLevel patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            }
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
        // ENTITYMAP FENCE (TASK-411-A k5b): compose the 12-site fence ONTO
        // the tracker-patched bytes (one set_patch carries BOTH — the
        // single-retransform discipline: hooks on one class are composed
        // on the bytes, never staged as competing retransforms).
        // Fail-dominant: probe failed / patch rejected / strict census
        // violated -> tracker-only bytes, entityMap stays vanilla.
        let mut cm_final = cm_patched.clone();
        let mut emap_status = "vanilla (lever off or define failed)".to_string();
        if crate::emap::armed() && emap_defined {
            match crate::classfile::patch_chunkmap_entitymap(&cm_patched) {
                Ok((p, o)) => {
                    let ok = matches!(
                        o,
                        crate::classfile::RetargetOutcome::Retargeted { sites: 12 }
                            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 12 }
                    );
                    if ok {
                        cm_final = p;
                        emap_status = format!("{o:?} (idempotent)");
                        eprintln!(
                            "[crussty-plugin] emap: entityMap fence composed on ChunkMap ({o:?}) — 12 sites monitor-fenced"
                        );
                    } else {
                        eprintln!(
                            "[crussty-plugin] emap: fence outcome {o:?} violates strict census:12, entityMap stays vanilla"
                        );
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] emap: fence patch rejected ({e}), entityMap stays vanilla"
                    );
                }
            }
        }
        let (cm_len, cm_len_patched) = (cm_orig.len(), cm_final.len());
        cm.set_patch(PatchCache {
            bytes: Arc::from(cm_final),
            major: cm_major,
        });

        // NAV-POOL (TASK-410-A k5, cmp405_navplane STRICT eq): compose the
        // A* node-pool into NodeEvaluator (prepare + getNode, strict
        // sites:2). Fail-OPEN for this stage only: capture/patch failure
        // leaves the region hook intact and the pool vanilla this boot.
        let mut ne_patched = false;
        if crate::nav_pool::armed() {
            let ne = ne_target();
            'navpool: {
                if !ne.orig_is_some() {
                    eprintln!(
                        "[crussty-plugin] navpool: {} not sighted yet, forcing kernel load",
                        ne.name
                    );
                    crate::improved_noise::force_load_kernel_class(ne.name);
                    for _attempt in 1..=3 {
                        let _ = cplug_sdk::retransform_class(ne.name);
                        if ne.orig_is_some() {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(250));
                    }
                }
                let Some(ne_orig) = ne.take_orig() else {
                    eprintln!(
                        "[crussty-plugin] navpool: NodeEvaluator not capturable, pool stays vanilla this boot"
                    );
                    break 'navpool;
                };
                let (ne_p, ne_outcome) =
                    match crate::classfile::patch_nodeevaluator_navpool(&ne_orig) {
                        Ok(pair) => pair,
                        Err(e) => {
                            eprintln!(
                                "[crussty-plugin] navpool: NodeEvaluator patch rejected ({e}), pool stays vanilla"
                            );
                            break 'navpool;
                        }
                    };
                if !matches!(
                    ne_outcome,
                    crate::classfile::RetargetOutcome::Retargeted { sites: 2 }
                ) {
                    eprintln!(
                        "[crussty-plugin] navpool: strict site-count violated ({ne_outcome:?}), pool stays vanilla"
                    );
                    break 'navpool;
                }
                let ne_major = crate::improved_noise::class_version(&ne_orig)
                    .map(|(m, _)| m)
                    .unwrap_or(0);
                eprintln!(
                    "[crussty-plugin] navpool: NodeEvaluator composed ({} -> {} bytes {ne_outcome:?})",
                    ne_orig.len(),
                    ne_p.len()
                );
                ne.set_patch(PatchCache {
                    bytes: Arc::from(ne_p),
                    major: ne_major,
                });
                ne_patched = true;
            }
        }

        // REFSYNC (TASK-412-A, cmp405_navplane lane): fence the ReferenceList
        // mutator sites across the SEVEN additional kernel classes. Each
        // class is independent (fail-dominant per class: capture/patch/census
        // failure leaves THAT class vanilla, the rest still fenced).
        let mut refsync_armed = 0usize;
        let mut refsync_failed = 0usize;
        if crate::emap::armed() && emap_defined {
            for t in rs_targets() {
                'refsync: {
                    if !t.orig_is_some() {
                        eprintln!(
                            "[crussty-plugin] refsync: {} not sighted yet, forcing kernel load",
                            t.name
                        );
                        crate::improved_noise::force_load_kernel_class(t.name);
                        for _attempt in 1..=3 {
                            let _ = cplug_sdk::retransform_class(t.name);
                            if t.orig_is_some() {
                                break;
                            }
                            std::thread::sleep(std::time::Duration::from_millis(250));
                        }
                    }
                    let Some(orig) = t.take_orig() else {
                        eprintln!(
                            "[crussty-plugin] refsync: {} not capturable, class stays vanilla",
                            t.name
                        );
                        refsync_failed += 1;
                        break 'refsync;
                    };
                    let want = crate::classfile::refsync_want_sites(t.name);
                    let (p, o) = match crate::classfile::patch_referencelist_callsites(&orig) {
                        Ok(pair) => pair,
                        Err(e) => {
                            eprintln!(
                                "[crussty-plugin] refsync: {} patch rejected ({e}), class stays vanilla",
                                t.name
                            );
                            refsync_failed += 1;
                            break 'refsync;
                        }
                    };
                    let ok = matches!(
                        o,
                        crate::classfile::RetargetOutcome::Retargeted { sites: s }
                            if s == want
                    ) || matches!(
                        o,
                        crate::classfile::RetargetOutcome::AlreadyPatched { sites: s }
                            if s == want
                    );
                    if !ok {
                        eprintln!(
                            "[crussty-plugin] refsync: {} strict site-count violated ({o:?}, want {want}), class stays vanilla",
                            t.name
                        );
                        refsync_failed += 1;
                        break 'refsync;
                    }
                    let major = crate::improved_noise::class_version(&orig)
                        .map(|(m, _)| m)
                        .unwrap_or(0);
                    eprintln!(
                        "[crussty-plugin] refsync: {} composed ({} -> {} bytes {o:?})",
                        t.name,
                        orig.len(),
                        p.len()
                    );
                    t.set_patch(PatchCache {
                        bytes: Arc::from(p),
                        major,
                    });
                    refsync_armed += 1;
                }
            }
        }
        let refsync_status = if !crate::emap::armed() || !emap_defined {
            "vanilla (lever off or define failed)".to_string()
        } else {
            format!(
                "fenced {refsync_armed}/7 classes + ServerLevel ({} failed classes vanilla)",
                refsync_failed
            )
        };

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
        if ne_patched {
            crate::kernel_policy::audit_wire(
                NAVPOOL_CLASS,
                "prepare/getNode (A* node-pool, fresh-shape laundering)",
                "nav_pool v1",
            );
        }
        if refsync_armed > 0 {
            crate::kernel_policy::audit_wire(
                crate::emap::EMAP_OPS_CLASS,
                "ReferenceList fence (refListAdd/refListRemove/refListContains monitor-serialized)",
                "refsync v1",
            );
        }
        if emap_status.starts_with("Retargeted") || emap_status.starts_with("AlreadyPatched") {
            crate::kernel_policy::audit_wire(
                crate::emap::EMAP_OPS_CLASS,
                "entityMap fence (containsKey/put/remove/get/values monitor-serialized)",
                "emap v1",
            );
        }
        READY.store(true, Ordering::Release);
        let rc_sl = cplug_sdk::retransform_class(sl.name);
        let rc_cb = cplug_sdk::retransform_class(cb.name);
        let rc_lv = cplug_sdk::retransform_class(lv.name);
        let rc_cm = cplug_sdk::retransform_class(cm.name);
        let rc_ne = if ne_patched {
            cplug_sdk::retransform_class(ne_target().name)
        } else {
            -1
        };
        let mut rc_rs = String::new();
        for t in rs_targets() {
            if t.patch_bytes().is_some() {
                let rc = cplug_sdk::retransform_class(t.name);
                rc_rs.push_str(&format!(" {}={}", t.name.rsplit('/').next().unwrap_or(t.name), rc));
            }
        }
        eprintln!(
            "[crussty-plugin] region_threads: ARMED, retransform rc ServerLevel={rc_sl} EntityCallbacks={rc_cb} Level={rc_lv} ChunkMap={rc_cm} NodeEvaluator={rc_ne} (Entity via entity_compose; navpool {}; emap {emap_status}; refsync {refsync_status}; retransform:{rc_rs})",
            if ne_patched { "ARMED" } else { "vanilla" }
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

#[cfg(test)]
mod navpool_delivery_tests {
    /// TASK-410-A k5 delivery-graph guard (mirror of blockupd discipline):
    /// NavPoolOps.java MUST declare ZERO nested classes — the bridge
    /// compiles to exactly one classfile and is defined alone into the
    /// kernel loader.
    #[test]
    fn navpool_ops_source_declares_no_nested_classes() {
        let src = include_str!(
            "../entityinside/net/minecraft/world/level/pathfinder/NavPoolOps.java"
        );
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
            "NavPoolOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile; nested classes would crash the \
             server with NoClassDefFoundError (S7-163 leg#1 TECH-DUD)"
        );
    }

    /// Build-dir mirror: exactly one NavPoolOps classfile exists.
    #[test]
    fn navpool_build_dir_has_exactly_one_classfile() {
        let dir = "entityinside/build/net/minecraft/world/level/pathfinder";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_navpool_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("NavPoolOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(
            count, 1,
            "NavPoolOps classfile set drifted — must compile to exactly ONE classfile"
        );
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn navpool_embedded_bytes_match_build_dir() {
        let on_disk = std::fs::read(
            "entityinside/build/net/minecraft/world/level/pathfinder/NavPoolOps.class",
        )
        .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::NAVPOOL_BYTES,
            "embedded NavPoolOps.class is stale — rerun scripts/build_navpool_ops.sh"
        );
    }

    /// Resolution closure: the embedded bridge declares BOTH receiver-
    /// prepended targets the NodeEvaluator retarget emits.
    #[test]
    fn navpool_embedded_declares_all_redirect_targets() {
        if let Err(e) = crate::classfile::navpool_resolution_closure(super::NAVPOOL_BYTES) {
            panic!(
                "RESOLUTION CLOSURE FAILED: {e} — rebuild entityinside/ via build_navpool_ops.sh"
            );
        }
    }

    /// Scope lock: EXACTLY prepare+getNode (the two-site pool lever).
    #[test]
    fn navpool_redirect_table_is_exactly_prepare_getnode() {
        let targets = crate::classfile::NAVPOOL_REDIRECT_TARGETS;
        assert_eq!(targets.len(), 2, "TASK-410-A k5 is a TWO-SITE lever");
        assert_eq!(targets[0].0, "prepare");
        assert_eq!(
            targets[0].1,
            "(Lnet/minecraft/world/level/PathNavigationRegion;Lnet/minecraft/world/entity/Mob;)V"
        );
        assert_eq!(targets[0].2, "prepare");
        assert_eq!(
            targets[0].3,
            "(Lnet/minecraft/world/level/pathfinder/NodeEvaluator;Lnet/minecraft/world/level/PathNavigationRegion;Lnet/minecraft/world/entity/Mob;)V"
        );
        assert_eq!(targets[1].0, "getNode");
        assert_eq!(targets[1].1, "(III)Lnet/minecraft/world/level/pathfinder/Node;");
        assert_eq!(targets[1].2, "getNode");
        assert_eq!(
            targets[1].3,
            "(Lnet/minecraft/world/level/pathfinder/NodeEvaluator;III)Lnet/minecraft/world/level/pathfinder/Node;"
        );
    }

    /// The laundering field-set must cover EVERY mutable Node field
    /// (javap: heapIdx/closed/g/h/f/cameFrom/walkedDistance/costMalus/type;
    /// x/y/z/hash are final — identity, never laundered).
    #[test]
    fn navpool_laundering_covers_every_mutable_node_field() {
        let src = include_str!(
            "../entityinside/net/minecraft/world/level/pathfinder/NavPoolOps.java"
        );
        for field in [
            "heapIdx", "closed", ".g =", ".h =", ".f =", "cameFrom", "walkedDistance",
            "costMalus", ".type =",
        ] {
            assert!(
                src.contains(field),
                "NavPoolOps.prepare laundering misses mutable Node field {field} — \
                 stale values would leak across searches (parity break)"
            );
        }
        // The vanilla fallback MUST stay in place beyond the retention bound.
        assert!(
            src.contains("nodes.clear()"),
            "MAP_CAP overflow path must fall back to the vanilla clear"
        );
    }
}
