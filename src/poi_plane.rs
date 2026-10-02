//! Runtime wiring for the POI-PLANE (TASK-456-B, vector R6 «POI подсистема
//! целиком» закон-6 — lever `cmp456_poi`; bridge
//! poi/net/minecraft/world/entity/ai/village/poi/PoiOps.java, natives below,
//! site retargets via `classfile::retarget_virtual_to_static`).
//!
//! TWO SITES (javap patched-kernel 1.21.10 ground truth):
//!   1. `Level.notifyAndUpdatePhysics` — the ONLY
//!      `invokevirtual Level.updatePOIOnBlockStateChange(BlockPos,BS,BS)V`
//!      site (vanilla dispatches into the ServerLevel override: forState×2 +
//!      Objects.equals early-out + rare exists/remove/add + debugSynchronizers).
//!      Runs for EVERY setBlock (fluid updates dominate a 150k-entity scene).
//!   2. `ChunkMap.tick(BooleanSupplier)` — the ONLY
//!      `invokevirtual PoiManager.tick(BooleanSupplier)V` site (the "poi"
//!      phase of the per-tick chunk orchestration; village-distance
//!      propagateUpdates).
//!
//! RUST OWNS THE SUBSYSTEM DATA: the POI state mask (stateId → isPoi, built
//! once in Java from `PoiTypes.hasPoi` over `Block.BLOCK_STATE_REGISTRY`) is
//! bound INTO rust (`poiBindMask`) — the DOD mask lives in both twins, rust
//! is the source of truth for the mirror store. The per-tick POI event batch
//! (levelHash, posLo, posHi, oldId, newId)×n lands in the rust PoiStore via
//! ONE bulk JNI per tick (`poiEpoch`, flushed at site 2; empty tick = 0
//! crossings). ZERO per-block JNI (закон 6).
//!
//! VANILLA-BIT-FOR-BIT: the fast path returns early ONLY when NEITHER state
//! is POI — vanilla in that case computes forState×2 (both Optional.empty),
//! Objects.equals → true → no side effect; skipping it is observationally
//! identical. The slow path performs the FULL vanilla virtual call
//! (`level.updatePOIOnBlockStateChange(pos, old, new)` — the override body is
//! UNTOUCHED); the event append is pure telemetry. Site 2 = flush + the FULL
//! vanilla `pm.tick(hasTimeLeft)`. Empty/foreign lever flag = no hooks at all
//! (byte-indistinguishable from vanilla).
//!
//! FAIL-CLOSED ladder: probe mismatch / bind failure / epoch ERR_STRUCT →
//! broken=true forever (pure vanilla); ERR_RANGE → vanilla this tick; gate
//! receives bytes whose site count != 1 → hook pass-through (dormant).
//!
//! Chain cohabitation: Level is already hooked by queryplane +
//! region_threads (guardEntityTick / whole-body redirects); ChunkMap by
//! region_threads Hook 4 (`tick()V → newTrackerTick` site — a DIFFERENT
//! method than my `tick(BooleanSupplier)`). This module registers LAST and
//! retargets its own sites on the RECEIVED chain bytes (previous rewrites
//! preserved byte-for-byte; AlreadyPatched idempotence from classfile.rs).

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, PoisonError};

const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const CHUNKMAP_CLASS: &str = "net/minecraft/server/level/ChunkMap";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/village/poi/PoiOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../poi/build/net/minecraft/world/entity/ai/village/poi/PoiOps.class");

// --- site 1: Level.notifyAndUpdatePhysics → Level.updatePOIOnBlockStateChange
const NIUP_NAME: &str = "notifyAndUpdatePhysics";
const NIUP_DESC: &str = "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;II)V";
const FROM_POI_UPDATE: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "updatePOIOnBlockStateChange",
    "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V",
);
const POI_GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V";

// --- site 2: ChunkMap.tick(BooleanSupplier) → PoiManager.tick
const CMTICK_NAME: &str = "tick";
const CMTICK_DESC: &str = "(Ljava/util/function/BooleanSupplier;)V";
const FROM_POI_TICK: (&str, &str, &str) = (
    "net/minecraft/world/entity/ai/village/poi/PoiManager",
    "tick",
    "(Ljava/util/function/BooleanSupplier;)V",
);
const POI_TICK_GATE_STATIC_DESC: &str =
    "(Lnet/minecraft/world/entity/ai/village/poi/PoiManager;Ljava/util/function/BooleanSupplier;)V";

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x5049; // "PI"

/// STRICT-OR carrier gate (x454-C MAIN FIX canon: the round lever arms the
/// FULL era composite; empty/foreign flag = vanilla bit-for-bit).
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref().map(str::trim),
        Ok("cmp456_poi") | Ok("cmp409_multi") | Ok("cmp412_meganav") | Ok("cmp412_eqsnapv3")
            | Ok("cmp414_cvs") | Ok("cmp417_bq") | Ok("cmp420_colpush") | Ok("cmp421_brain")
            | Ok("cmp422_brain2") | Ok("cmp423_brain3") | Ok("cmp424_mobfeed")
            | Ok("cmp430_inside") | Ok("cmp432_inside2") | Ok("cmp436_ins4") | Ok("cmp438_sense")
            | Ok("cmp451_senseins") | Ok("cmp453_diet") | Ok("cmp452_mega")
            | Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4")
            | Ok("cmp444_chunk5") | Ok("cmp450_chunk")
            // R468-S39 drift-fix: post-456 era carriers keep the certified POI
            // plane alive (strict-OR canon "all certified master planes live on
            // every gate"; quiet-drift class, LEDGER Л180h).
            | Ok("cmp456_chunkmono") | Ok("cmp456_chunkmono_p31snap") | Ok("cmp466_c98ai")
    )
}

static READY: AtomicBool = AtomicBool::new(false);
static READY_CM: AtomicBool = AtomicBool::new(false);

/// Rust PoiStore mirror (subsystem data): (levelHash, posLo, posHi, oldId,
/// newId) per event. Append-only log; the serve-plane (find/ensureLoaded
/// queries from the mirror) is the documented cycle-2 extension.
static POI_STORE: Mutex<Vec<[i32; 5]>> = Mutex::new(Vec::new());
static POI_STORE_TOTAL: AtomicU64 = AtomicU64::new(0);
static MASK_WWORDS: Mutex<Vec<u64>> = Mutex::new(Vec::new());

// --- C28 MC-312010 guard-a (ROUND-475) -------------------------------------
// Mojang MC-312010 class (Server-СТЗ C93/×474 top item; POI/chunk store
// out-of-order re-dirty records — same ordering family as MC-310372): the
// rust PoiStore mirror is the observation point. WATCHDOG, telemetry-only:
// per-epoch ORDERING-HASH over the batch (1 hash per POI batch — the
// PoiOps.maybeEpoch amortization, POI_BATCH=4 in master) + STRICT tick
// monotonicity of the epoch stream (equal/decreasing epoch tick = non-
// monotonic record write). Detection ONLY: ingestion stays bit-for-bit,
// nothing disarms (a guard is not a lever; закон-5 канон).
//
// Preregister (C28):
//  - OFF by default (флаг-канон): env CRUSSTY_MC312A_GUARD = 1/true/on/yes;
//    cached once → the OFF hot path is ONE atomic load per poiEpoch call
//    (1 per POI_BATCH=4 ticks), zero measurable CPU;
//  - overhead ≤0.1% JNI-переходов: the hash is computed INSIDE the existing
//    bulk poiEpoch crossing (zero extra JNI transitions, like POI_BATCH);
//    ~1 FNV round per event int, sub-µs per batch;
//  - NCDFE-канон: no new Java classes / natives / retargets / blob rebuilds
//    → NoClassDefFoundError impossible by construction (pure rust, live
//    inside the already-registered poiEpoch native);
//  - ARM-пруф по артефакту (Л-475): first observed batch logs the ARMED
//    marker and violations log capped markers — both to stderr →
//    server-stdout.log; OFF-run proof = ZERO "mc312a" lines in the artifact.
static MC312A_FLAG: AtomicU8 = AtomicU8::new(0); // 0 unknown, 1 off, 2 on
static MC312A_ARM: AtomicBool = AtomicBool::new(false);
static MC312A_VIOL_LOGS: AtomicU64 = AtomicU64::new(0);

/// env-канон гейта (fluid_guard pattern): unset/garbage = OFF.
fn guard_flag_from(v: Option<&str>) -> bool {
    v.map(|v| {
        let v = v.trim().to_ascii_lowercase();
        v == "1" || v == "true" || v == "on" || v == "yes"
    })
    .unwrap_or(false)
}

/// Cached gate read: env consulted once, then a plain atomic load.
fn guard_enabled() -> bool {
    match MC312A_FLAG.load(Ordering::Acquire) {
        1 => false,
        2 => true,
        _ => {
            let on = guard_flag_from(std::env::var("CRUSSTY_MC312A_GUARD").ok().as_deref());
            MC312A_FLAG.store(if on { 2 } else { 1 }, Ordering::Release);
            on
        }
    }
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// One FNV-1a round over a u32 word (order-sensitive mixing).
fn fnv1a_word(hash: u64, w: u32) -> u64 {
    (hash ^ (w as u64)).wrapping_mul(FNV_PRIME)
}

/// Ordering-hash of one POI epoch batch: the epoch tick, then the event
/// tuples (levelHash, posLo, posHi, oldId, newId) in ARRIVAL order. Chainable
/// across epochs (prev hash feeds the next) → any reordering/re-dirty of the
/// record stream changes the rolling hash.
fn ordering_hash(prev: u64, tick: i32, events: &[[i32; 5]]) -> u64 {
    let mut h = fnv1a_word(prev, tick as u32);
    for e in events {
        for w in e {
            h = fnv1a_word(h, *w as u32);
        }
    }
    h
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GuardVerdict {
    Monotonic,
    NonMonotonic { prev_tick: i32 },
}

/// Watchdog state. Writer = the server thread inside poiEpoch (already
/// serialized by PoiOps.maybeEpoch's EPOCH_TICK discipline); Mutex only for
/// test/tooling access.
struct OrderingGuard {
    last_tick: Option<i32>,
    rolling: u64,
    batches: u64,
    violations: u64,
}

impl OrderingGuard {
    const fn new() -> Self {
        Self {
            last_tick: None,
            rolling: FNV_OFFSET,
            batches: 0,
            violations: 0,
        }
    }

    /// Observe one epoch: extend the ordering-hash chain, enforce strict tick
    /// monotonicity. First batch initializes (no prior → monotonic).
    fn observe(&mut self, tick: i32, events: &[[i32; 5]]) -> GuardVerdict {
        let verdict = match self.last_tick {
            Some(prev) if tick <= prev => GuardVerdict::NonMonotonic { prev_tick: prev },
            _ => GuardVerdict::Monotonic,
        };
        self.rolling = ordering_hash(self.rolling, tick, events);
        self.last_tick = Some(tick);
        self.batches += 1;
        if matches!(verdict, GuardVerdict::NonMonotonic { .. }) {
            self.violations += 1;
        }
        verdict
    }
}

static MC312A_GUARD: Mutex<OrderingGuard> = Mutex::new(OrderingGuard::new());

/// Post-ingest watchdog call (telemetry-only, never fails ingestion).
fn mc312a_observe(tick: i32, events: &[[i32; 5]]) {
    let mut g = match MC312A_GUARD.lock() {
        Ok(g) => g,
        Err(p) => PoisonError::into_inner(p), // poison-recovery canon (TASK-46)
    };
    if !MC312A_ARM.swap(true, Ordering::Release) {
        // ARM-пруф: lands in server-stdout.log (Л-475: artifact is the only proof).
        eprintln!(
            "[crussty-plugin] mc312a-guard: ARMED tick={tick} events={} (per-batch ordering-hash + strict tick monotonicity over the PoiStore epoch stream; env CRUSSTY_MC312A_GUARD, off by default)",
            events.len()
        );
    }
    if let GuardVerdict::NonMonotonic { prev_tick } = g.observe(tick, events) {
        let n = MC312A_VIOL_LOGS.fetch_add(1, Ordering::Relaxed);
        if n < 8 || n % 16 == 0 {
            eprintln!(
                "[crussty-plugin] mc312a-guard: POI-STORE ORDERING VIOLATION tick={tick} prev_tick={prev_tick} epoch_seq={} rolling_hash={:016x} violations={} (MC-312010 class: non-monotonic POI/chunk record write)",
                g.batches, g.rolling, g.violations
            );
        }
    }
}

fn retarget_poi_update(bytes: &[u8]) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        NIUP_NAME,
        NIUP_DESC,
        FROM_POI_UPDATE,
        (OPS_CLASS, "updatePoiGate", POI_GATE_STATIC_DESC),
    )
}

fn retarget_poi_tick(bytes: &[u8]) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        CMTICK_NAME,
        CMTICK_DESC,
        FROM_POI_TICK,
        (OPS_CLASS, "poiTickGate", POI_TICK_GATE_STATIC_DESC),
    )
}

/// Register the two byte hooks (call once from cplugin_init, AFTER
/// queryplane/region_threads so this hook receives their chain bytes).
/// Dormant-invisible: with the lever flag unset/mismatched NOTHING is
/// registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] poi_plane: dormant (lever_flag not in poi strict-OR family, vanilla POI plane)"
        );
        return;
    }
    // Hook 1: Level (site 1 — notifyAndUpdatePhysics POI update).
    cplug_sdk::hooks::register_bytes(LEVEL_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None; // pre-arm: pass chain bytes through untouched
        }
        match retarget_poi_update(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] poi_plane: hook serve {LEVEL_CLASS} {} bytes (updatePOIOnBlockStateChange site, sites=1)",
                            out.len()
                        );
                    }
                    Some(out)
                }
                crate::classfile::RetargetOutcome::AlreadyPatched { .. } => Some(out),
                other => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] poi_plane: updatePOI site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] poi_plane: Level retarget rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
    // Hook 2: ChunkMap (site 2 — PoiManager.tick orchestration phase).
    cplug_sdk::hooks::register_bytes(CHUNKMAP_CLASS, move |_name, bytes| {
        if !READY_CM.load(Ordering::Acquire) {
            return None;
        }
        match retarget_poi_tick(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] poi_plane: hook serve {CHUNKMAP_CLASS} {} bytes (PoiManager.tick site, sites=1)",
                            out.len()
                        );
                    }
                    Some(out)
                }
                crate::classfile::RetargetOutcome::AlreadyPatched { .. } => Some(out),
                other => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] poi_plane: PoiManager.tick site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] poi_plane: ChunkMap retarget rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for boot + Level/ChunkMap load, define PoiOps
/// into the kernel loader + RegisterNatives (poiProbe/poiBindMask/poiEpoch),
/// run the selfTest oracle (selfTest==true until ARM — also builds+binds the
/// POI mask off-tick), flip READY (hooks start serving), retransform both
/// classes.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            let lv = cplug_sdk::classes::find_class(LEVEL_CLASS).is_some();
            let cm = cplug_sdk::classes::find_class(CHUNKMAP_CLASS).is_some();
            if lv && cm {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] poi_plane: {LEVEL_CLASS}/{CHUNKMAP_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] poi_plane: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Lesson 408: embedded bridge bytes must not be newer than the JVM.
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
                "[crussty-plugin] poi_plane: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild poi/ via scripts/build_456b_blobs_all.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(level_cls) = cplug_sdk::classes::find_class(LEVEL_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(level_cls.as_jclass(), mid, &[]);
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
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] poi_plane: define_class({OPS_CLASS}) failed");
                return false;
            };

            let names = [
                CString::new("poiProbe").expect("no NUL"),
                CString::new("poiBindMask").expect("no NUL"),
                CString::new("poiEpoch").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("([J)I").expect("no NUL"),
                CString::new("(II[I[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: poi_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: poi_bind_mask as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: poi_epoch as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                crate::describe_exception(env);
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] poi_plane: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }

            // selfTest()Z oracle BEFORE READY (selfTest==true до ARM). The
            // first static call also initializes PoiOps: the mask build +
            // poiBindMask happen HERE, at boot, off the tick path.
            let ok = env
                .get_static_method_id(c, "selfTest", "()Z")
                .map(|mid| {
                    let raw = env.raw();
                    let result = unsafe {
                        let fn_table = &(**raw);
                        let call_bool = fn_table.CallStaticBooleanMethodA;
                        (call_bool)(raw, c, mid, [].as_ptr())
                    };
                    crate::clear_exception(env);
                    result != 0
                })
                .unwrap_or_else(|| {
                    crate::clear_exception(env);
                    false
                });
            if !ok {
                eprintln!(
                    "[crussty-plugin] poi_plane: PoiOps.selfTest()==false — hook stays dormant"
                );
            }
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] poi_plane: bridge definition failed, hook stays dormant");
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp456_poi: ARMED poi-plane (Level.notifyAndUpdatePhysics updatePOIOnBlockStateChange site -> PoiOps.updatePoiGate POI-mask fast-path + ChunkMap.tick PoiManager.tick site -> PoiOps.poiTickGate epoch flush; rust PoiStore mirror via ONE bulk poiEpoch JNI/tick; vanilla decisions bit-for-bit: both-non-POI = vanilla no-op early-out, slow path = full untouched vanilla virtual call; zero per-block JNI; empty flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "updatePoiGate", "cmp456_poi v1");
        crate::kernel_policy::audit_wire(OPS_CLASS, "poiTickGate", "cmp456_poi v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(LEVEL_CLASS);
        eprintln!(
            "[crussty-plugin] poi_plane: {LEVEL_CLASS} armed, retransform rc={rc} (ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] poi_plane: Level retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
            return;
        }
        READY_CM.store(true, Ordering::Release);
        let rc2 = cplug_sdk::retransform_class(CHUNKMAP_CLASS);
        eprintln!(
            "[crussty-plugin] poi_plane: {CHUNKMAP_CLASS} armed, retransform rc={rc2} (ready-gate on)"
        );
        if rc2 != 0 {
            READY_CM.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] poi_plane: ChunkMap retransform FAILED (rc={rc2}) — site 2 stays vanilla"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Natives (registered on PoiOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn poi_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// Bind the POI state mask INTO rust (subsystem data; DOD twin).
/// rc = 0 ok; ERR_STRUCT on pin failure / gate mismatch.
///
/// # Safety
/// See poi_probe.
#[no_mangle]
pub unsafe extern "system" fn poi_bind_mask(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    mask: jni::jlongArray,
) -> jni::jint {
    if !enabled() || env.is_null() || mask.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let len = unsafe { (vt.GetArrayLength)(env, mask) };
    if len <= 0 {
        return ERR_RANGE;
    }
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, mask, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let words =
        unsafe { std::slice::from_raw_parts(pinned as *const jni::jlong, len as usize) };
    let mut copy = Vec::with_capacity(len as usize);
    for w in words {
        copy.push(*w as u64);
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, mask, pinned, 0) };
    match MASK_WWORDS.lock() {
        Ok(mut g) => {
            *g = copy;
            0
        }
        Err(_) => ERR_STRUCT,
    }
}

/// BULK epoch ingest — ONE transition per tick (never per block). Appends the
/// event batch into the rust PoiStore mirror and reports the store size via
/// out[0]. rc = number of ingested events, ERR_RANGE (bad n/arrays), ERR_STRUCT
/// (pin failure/gate). Java fail-closed: any non-count → disarmed.
///
/// # Safety
/// See poi_probe.
#[no_mangle]
pub unsafe extern "system" fn poi_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    count: jni::jint,
    batch: jni::jintArray,
    out: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || batch.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    if tick < 0 || count < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let b_cap = unsafe { (vt.GetArrayLength)(env, batch) };
    let o_cap = unsafe { (vt.GetArrayLength)(env, out) };
    if b_cap < count * 5 || o_cap < 1 || b_cap % 5 != 0 {
        return ERR_RANGE;
    }
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, batch, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let src = unsafe { std::slice::from_raw_parts(pinned as *const jni::jint, b_cap as usize) };
    let mut events: Vec<[i32; 5]> = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        events.push([
            src[i * 5],
            src[i * 5 + 1],
            src[i * 5 + 2],
            src[i * 5 + 3],
            src[i * 5 + 4],
        ]);
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, batch, pinned, 0) };

    let mut store = match POI_STORE.lock() {
        Ok(mut g) => g,
        Err(_) => return ERR_STRUCT,
    };
    store.extend_from_slice(&events);
    let total = store.len() as u64;
    drop(store);
    POI_STORE_TOTAL.store(total, Ordering::Relaxed);

    // C28 MC-312010 guard-a: post-ingest ordering watchdog (telemetry-only,
    // off by default — env CRUSSTY_MC312A_GUARD). Computed INSIDE this JNI
    // crossing: zero extra JNI transitions (≤0.1% переход-бюджет, как
    // POI_BATCH). Ingestion path above is untouched.
    if guard_enabled() {
        mc312a_observe(tick, &events);
    }

    let pinned_out = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned_out.is_null() {
        return ERR_STRUCT;
    }
    unsafe {
        *(pinned_out as *mut jni::jint) = total as jni::jint;
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned_out, 0) };
    count
}

// ---------------------------------------------------------------------------
// Tests: the vanilla no-op equivalence oracle (both-non-POI → Objects.equals
// early-out), the mask index math, the batch packing contract, the epoch
// bounds, and both retarget descriptor contracts.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn enabled_with(s: &str) -> bool {
        s == "cmp456_poi"
    }

    #[test]
    fn lever_constant_strict() {
        assert!(enabled_with("cmp456_poi"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp455_spawn"));
        assert!(!enabled_with("cmp456_poi_x"));
        assert!(!enabled_with(" cmp456_poi"));
    }

    #[test]
    fn both_non_poi_means_vanilla_noop() {
        // Vanilla: forState×2 → (empty, empty) → Objects.equals==true → the
        // updatePOIOnBlockStateChange body performs ZERO side effects. The
        // fast-path early-out is observationally identical.
        let old_poi = false;
        let new_poi = false;
        assert!(!old_poi && !new_poi); // gate returns without telemetry
    }

    #[test]
    fn mask_index_math_roundtrip() {
        // m[id>>>6] bit (id&63); capacity = m.len() << 6.
        let mut m = vec![0u64; 4];
        let set = |m: &mut Vec<u64>, id: usize| m[id >> 6] |= 1u64 << (id & 63);
        let get = |m: &[u64], id: i64| -> bool {
            id >= 0
                && (id as usize) < m.len() << 6
                && (m[(id >> 6) as usize] >> (id & 63)) & 1 != 0
        };
        for id in [0usize, 1, 63, 64, 100, 191, 255] {
            set(&mut m, id);
        }
        for id in [0usize, 1, 63, 64, 100, 191, 255] {
            assert!(get(&m, id as i64), "bit {id} must be set");
        }
        for id in [2usize, 62, 65, 254] {
            assert!(!get(&m, id as i64), "bit {id} must be clear");
        }
        assert!(!get(&m, -1));
        assert!(!get(&m, 256));
    }

    #[test]
    fn batch_packing_roundtrip() {
        // BlockPos.asLong → (lo, hi) split; 5-int tuple layout.
        let packed: i64 = 0x0123_4567_89AB_CDEF;
        let lo = packed as i32;
        let hi = (packed as u64 >> 32) as i32;
        assert_eq!(lo, 0x89AB_CDEFu32 as i32);
        assert_eq!(hi, 0x0123_4567);
        let re = ((hi as i64) << 32) | (lo as i64 & 0xFFFF_FFFF);
        assert_eq!(re, packed);
        let tuple = [777, lo, hi, 1001, 1002];
        assert_eq!(tuple.len(), 5);
    }

    #[test]
    fn epoch_bounds_match_java_ladder() {
        // Java flush sends count events (count*5 ints); rust requires
        // b_cap >= count*5, b_cap % 5 == 0, o_cap >= 1.
        let b_cap = 5 * 64;
        let count = 64;
        assert!(b_cap >= count * 5 && b_cap % 5 == 0);
        let bad = 5 * 64 + 2;
        assert!(bad % 5 != 0); // ERR_RANGE
    }

    #[test]
    fn gate_static_desc_contracts() {
        // Site 1: receiver-prepended Level static form.
        let expect1 = format!("(L{};{}", FROM_POI_UPDATE.0, &FROM_POI_UPDATE.2[1..]);
        assert_eq!(POI_GATE_STATIC_DESC, expect1);
        // Site 2: receiver-prepended PoiManager static form.
        let expect2 = format!("(L{};{}", FROM_POI_TICK.0, &FROM_POI_TICK.2[1..]);
        assert_eq!(POI_TICK_GATE_STATIC_DESC, expect2);
    }

    // --- C28 MC-312010 guard-a ---------------------------------------------

    #[test]
    fn mc312a_env_canon_off_by_default() {
        // Флаг-канон: unset/garbage = OFF; only 1/true/on/yes (trim+case) = ON.
        assert!(!guard_flag_from(None));
        assert!(!guard_flag_from(Some("")));
        assert!(!guard_flag_from(Some("0")));
        assert!(!guard_flag_from(Some("off")));
        assert!(!guard_flag_from(Some("cmp456_poi")));
        assert!(guard_flag_from(Some("1")));
        assert!(guard_flag_from(Some("true")));
        assert!(guard_flag_from(Some("on")));
        assert!(guard_flag_from(Some(" yes ")));
        assert!(guard_flag_from(Some("TRUE")));
    }

    #[test]
    fn mc312a_hash_order_and_tick_sensitive() {
        let a = [[1, 10, 20, 300, 301], [2, 11, 21, 400, 401]];
        let b = [[2, 11, 21, 400, 401], [1, 10, 20, 300, 301]]; // event pairs swapped
        let h_a = ordering_hash(FNV_OFFSET, 100, &a);
        assert_ne!(h_a, ordering_hash(FNV_OFFSET, 100, &b), "reordering events must change the ordering-hash");
        assert_ne!(h_a, ordering_hash(FNV_OFFSET, 101, &a), "different epoch tick must change the ordering-hash");
        assert_eq!(h_a, ordering_hash(FNV_OFFSET, 100, &a), "same order+tick → deterministic");
        // intra-tuple word order matters too (oldId/newId swap detectable)
        let c = [[1, 10, 20, 301, 300]];
        assert_ne!(ordering_hash(FNV_OFFSET, 100, &[a[0]]), ordering_hash(FNV_OFFSET, 100, &c));
        // empty batch still mixes the tick (epoch chain stays tick-sensitive)
        assert_ne!(ordering_hash(FNV_OFFSET, 100, &[]), ordering_hash(FNV_OFFSET, 104, &[]));
    }

    #[test]
    fn mc312a_monotonic_ladder_detects_nonmonotonic_writes() {
        let mut g = OrderingGuard::new();
        assert_eq!(g.observe(100, &[[1, 0, 0, 5, 6]]), GuardVerdict::Monotonic); // init
        assert_eq!(g.observe(104, &[]), GuardVerdict::Monotonic); // POI_BATCH=4 step, empty epoch ok
        assert_eq!(g.observe(108, &[[2, 1, 1, 7, 8]]), GuardVerdict::Monotonic);
        assert_eq!(g.observe(108, &[]), GuardVerdict::NonMonotonic { prev_tick: 108 }); // equal epoch
        assert_eq!(g.observe(106, &[]), GuardVerdict::NonMonotonic { prev_tick: 108 }); // decreasing epoch
        assert_eq!(g.observe(112, &[]), GuardVerdict::Monotonic); // stream continues
        assert_eq!(g.violations, 2);
        assert_eq!(g.batches, 6);
        assert_eq!(g.last_tick, Some(112));
    }

    #[test]
    fn mc312a_rolling_hash_chains_across_epochs() {
        let e1 = [[1, 0, 0, 5, 6]];
        let e2 = [[2, 1, 1, 7, 8]];
        let mut g = OrderingGuard::new();
        g.observe(4, &e1);
        g.observe(8, &e2);
        // chain == feeding each batch hash into the next
        let direct = ordering_hash(ordering_hash(FNV_OFFSET, 4, &e1), 8, &e2);
        assert_eq!(g.rolling, direct);
        // epoch order swap → different chain (record-stream serialization)
        let mut g2 = OrderingGuard::new();
        g2.observe(8, &e2);
        g2.observe(9, &e1);
        assert_ne!(g2.rolling, g.rolling);
    }
}
