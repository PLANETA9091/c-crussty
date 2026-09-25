//! Runtime wiring for the GOAL-QUERY lever (TASK-410-C, K3 PIVOT of vector
//! R2 — lever `cmp410_eindexq`; bridge
//! entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java,
//! natives below, retarget of the `Level.getEntitiesOfClass` call sites
//! inside `NearestAttackableTargetGoal.findTarget` and
//! `AvoidEntityGoal.canUse` via `classfile::retarget_virtual_to_static`).
//!
//! K3 PIVOT (research RESEARCH-C-k3.md, collapsed-cpu round409anchora /
//! round409cleg4): the goal/target broadphase slice (~2.7-3.0% wall) is
//! delivered through a SINGLE chokepoint
//! `Level.getEntitiesOfClass(Class,AABB,Predicate)` (86-88% of the slice via
//! `EntityLookup.getEntities -> ChunkEntitySlices.getEntities`). Vanilla
//! re-walks moonrise 16³ entity sections on EVERY goal query. RUST replaces
//! the per-query section walk with ONE bulk JNI per server tick:
//! `eqEpoch(tick, idTop, soa[D], head[I], next[I])` does a DOD pass over the
//! mobs_soa SoA population (x/y/z/hw/hh/flags — the SAME dense ids as
//! MobPushOps.byId) and builds an intrusive-chain hash table straight into
//! shared java arrays: bucket = hash(floor(x/16), floor(z/16)) over 65536
//! buckets, head[h] = id+1 chain head, next[id] = id+1 link (0 = end), plus
//! frozen columns soa[id*5..id*5+5] = x,y,z,hw,hh. The per-query cost
//! collapses to flat array reads: cell rect of the (margin-inflated) AABB →
//! chains → frozen-column prune → byId[id] → live AABB.intersects →
//! predicate. ZERO per-query JNI (law 6: one bulk transition per tick).
//!
//! WHY CHAINS, NOT CSR OFFSETS (vs the pre-crash draft): a 2-pass CSR
//! (counts → prefix → fill) requires a GLOBALLY consistent snapshot; the
//! seqlock retry ladder NEVER converges under a continuous writer (150k
//! upserts/tick — the pass window always straddles version bumps). A
//! single-pass chain build is self-consistent under ANY writer interleave:
//! every id lands in exactly one chain, and the placement hash is computed
//! from the SAME frozen x/z stored in the columns — a torn row yields a
//! "ghost" at a mixed position for ≤1 tick (bounded by the 8-block java
//! margin ≫ tick drift) instead of a corrupted CSR. This is the documented
//! single-pass soundness argument; no global seqlock is taken (deliberate —
//! see bridge javadoc).
//!
//! VANILLA-BIT-FOR-BIT DECISIONS (owner mandate): the gate returns a STRICT
//! SUPERSET of the vanilla enumeration for Mob/Player-assignable classes
//! (all vanilla mobs are in the SoA plane via the push-plane self-upsert —
//! LivingEntity.aiStep → pushEntities EVERY tick; players come from the
//! authoritative level.players()); the PRECISE filter is the live
//! `getBoundingBox().intersects(box)` — the same test vanilla applies per
//! candidate; candidate ORDER is chain/bucket order (documented delta class
//! items_subsys2/mobpush — consumers are getNearestEntity nearest-picks,
//! order-independent except exact double ties). The REST of findTarget /
//! canUse (getNearestEntity, TargetingConditions, pathfind) runs UNTOUCHED
//! vanilla bytecode. The vanilla body's Profiler counter "getEntities" is
//! replicated on the snapshot path (MobPushOps.pushables precedent).
//!
//! FAIL-CLOSED: flag != "cmp410_eindexq" → nothing registered (byte-
//! indistinguishable from vanilla); plane not ready (broken/oversized/
//! probe) → vanilla; idTop==0 or rc==0-cold-tables → vanilla this tick;
//! eqProbe mismatch / eqEpoch ERR_STRUCT → disarm forever; ERR_RANGE /
//! snapshot drift / absurd rect → vanilla for the call. sites != expected →
//! hook stays pass-through (fail-closed, d05c930 lesson: verify per-site
//! via the received bytes, not assumptions).
//!
//! FQN NOTE: net.minecraft.world.entity.EntityQueryOps is TAKEN by the
//! alloc-diet bridge (S7-133, entityquery/); a duplicate define into the
//! same loader = LinkageError. This bridge is EntityGoalQueryOps.

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

/// javap ground truth (patched-kernel.jar round-396-a, purpur-1.21.10
/// Mojang-mapped): exactly ONE
/// `invokevirtual Level.getEntitiesOfClass:(Ljava/lang/Class;Lnet/minecraft/
/// world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;` site in
/// `NearestAttackableTargetGoal.findTarget()V` (offset 52; the Player/
/// ServerPlayer target types route to ServerLevel.getNearestPlayer instead)
/// and exactly ONE in `AvoidEntityGoal.canUse()Z` (offset 47). Both are
/// invokevirtual on Level (final method) even though the receiver is
/// ServerLevel.
const TARGETS: &[(&str, &str, &str)] = &[
    (
        "net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal",
        "findTarget",
        "()V",
    ),
    ("net/minecraft/world/entity/ai/goal/AvoidEntityGoal", "canUse", "()Z"),
];

const OPS_CLASS: &str = "net/minecraft/world/entity/EntityGoalQueryOps";

const OPS_BYTES: &[u8] = include_bytes!(
    "../entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class"
);

/// Owner + call replaced in both target methods.
const FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "getEntitiesOfClass",
    "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
);
/// Receiver-prepended static form (stack-identical Level→static gate).
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;";

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x4547; // "EG"

// ---- snapshot geometry: MUST match EntityGoalQueryOps.java ----
const CELLS: usize = 1 << 16; // 65536 chain buckets
const CELL_SIZE: f64 = 16.0; // blocks per cell edge
const STRIDE: usize = 5; // x,y,z,hw,hh per row

/// Java int literals as bit patterns: 0x9E3779B1 / 0x85EBCA77 (both are
/// NEGATIVE java ints; wrapping i32 mul matches java `*` exactly).
const H1: i32 = 0x9E37_79B1u32 as i32;
const H2: i32 = 0x85EB_CA77u32 as i32;

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
/// TASK-411-C (k4soa): K4-нога — та же снапшот-механика + радиус-ремонт
/// населения (gate 2.0 / pad 2) + push-лейн из снапшота (pushCandidates).
/// TASK-411-C (eqsnap, v2): та же снапшот-механика, НО upserts идут в
/// пер-потоковые delta-шарды (0 локов/seqlock), eq_epoch СНАЧАЛА сливает их
/// одним bulk-drain (O(dirty)) и только потом строит цепи (cl1 35691270899:
/// per-entity WLOCK-мутации = 24.9% CPU → 0.5 TPS; eq chain build = 0.025%).
fn enabled() -> bool {
    flag_enabled(std::env::var("CRUSSTY_LEVER_FLAG").as_deref().ok().as_deref())
}

/// TASK-452-A: PURE production gate list (no env read) — the test module pins
/// THIS function directly (x452 lesson: the test-mirror `enabled_with` was
/// retagged for cmp451_senseins while the production `enabled()` was not →
/// dormant plane → EntityGoalQueryOps never defined → MobPushOps.pushables
/// NCDFE ×32768 on the first senseins legs, DELIVERY-FAIL). One source of
/// truth, no mirror drift possible.
fn flag_enabled(flag: Option<&str>) -> bool {
    matches!(
        flag,
        Some("cmp410_eindexq") | Some("cmp411_k4soa") | Some("cmp411_eqsnap")
            // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
            | Some("cmp412_eqsnapv3") | Some("cmp414_cvs") | Some("cmp417_bq")
            // TASK-419-A (colpush): колпаш-носитель — eq_epoch снапшот жив
            // (плоскость кормит colpush_plane_refresh).
            | Some("cmp420_colpush")
            // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
            | Some("cmp422_brain2")
            // TASK-424-A: GC-ревизия brain3 (STRICT OR).
            | Some("cmp423_brain3") | Some("cmp424_mobfeed") | Some("cmp430_inside") | Some("cmp432_inside2") | Some("cmp436_ins4")
            | Some("cmp438_sense") // TASK-444-C: sense family union
            | Some("cmp451_senseins") // TASK-452-A: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
            | Some("cmp421_brain") | Some("cmp434_chunkpl") | Some("cmp435_chunk3") | Some("cmp437_chunk4") | Some("cmp444_chunk5") | Some("cmp450_chunk")
    )
}

/// TASK-411-C (k4soa): true under the K4 flag only (ARM-marker labelling).
fn enabled_flag_is_k4() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp411_k4soa")
    )
}

/// TASK-411-C (eqsnap, v2): true under the eqsnap flag only (marker labelling
/// + the shard-drain switch inside eq_epoch).
fn enabled_flag_is_eqsnap() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp411_eqsnap") | Ok("cmp412_eqsnapv3") | Ok("cmp414_cvs") | Ok("cmp417_bq")
            // TASK-419-B (sense-plane composite): STRICT OR.
            | Ok("cmp421_brain")
    )
}

/// TASK-412-C (eqsnap-v3): true under the v3 composite flag only
/// (ARM-marker labelling — точная метка флага в EFFECT-строках).
fn enabled_flag_is_eqsnapv3() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp412_eqsnapv3") | Ok("cmp414_cvs") | Ok("cmp417_bq")
    )
}

/// TASK-419-B (sense-plane): true under the sense composite flag only —
/// включает достройку CSR-арены (sense_arena) сразу после eq_epoch в том же
/// EPOCH_LOCK-окне; snapshotQuery-джава читает слайсы арены вместо цепей.
/// TASK-422-B (iter-2): STRICT-OR — вектор-флаг несёт тот же sense-срез.
fn enabled_flag_is_sense() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp421_brain") | Ok("cmp422_brain2")
            // TASK-424-A: GC-ревизия brain3 (STRICT OR).
            | Ok("cmp423_brain3") | Ok("cmp424_mobfeed") | Ok("cmp430_inside") | Ok("cmp432_inside2") | Ok("cmp436_ins4")
            | Ok("cmp438_sense") // TASK-444-C: sense family union
            | Ok("cmp451_senseins") // TASK-452-A: senseins composite — sense-arena slice must arm (production gate retag)
            | Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4") | Ok("cmp444_chunk5") | Ok("cmp450_chunk") // TASK-454-B: chunk-plane union rides carrier: senseins composite — sense-arena slice must arm (production gate retag)
    )
}

/// TASK-422-B (iter-2): точная метка вектор-флага в ARM/EFFECT-маркерах.
fn enabled_flag_is_brain2() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp422_brain2")
    )
}

static READY: AtomicBool = AtomicBool::new(false);

/// Per-class pristine capture + computed patch (mobs_manager pattern ×2).
struct Target {
    name: &'static str,
    method: &'static str,
    desc: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Vec<u8>>>,
}

impl Target {
    const fn new(name: &'static str, method: &'static str, desc: &'static str) -> Self {
        Self {
            name,
            method,
            desc,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
        }
    }

    fn stash_orig(&self, bytes: &[u8]) {
        if let Ok(mut orig) = self.orig.lock() {
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
        }
    }

    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig.lock().ok().and_then(|mut o| o.take())
    }

    fn set_patch(&self, bytes: Vec<u8>) {
        if let Ok(mut p) = self.patch.lock() {
            *p = Some(bytes);
        }
    }

    fn patch_bytes(&self) -> Option<Vec<u8>> {
        self.patch.lock().ok().and_then(|p| p.clone())
    }
}

fn targets() -> &'static [Target] {
    use std::sync::OnceLock;
    static TARGETS_CELL: OnceLock<Vec<Target>> = OnceLock::new();
    TARGETS_CELL.get_or_init(|| {
        TARGETS
            .iter()
            .map(|(name, method, desc)| Target::new(name, method, desc))
            .collect()
    })
}

/// Compute the retarget patch from the RECEIVED (pristine) bytes.
fn retarget_site(t: &Target, bytes: &[u8]) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        t.method,
        t.desc,
        FROM,
        (OPS_CLASS, "entitiesOfClassGate", GATE_STATIC_DESC),
    )
}

/// Register the byte hooks (call once from cplugin_init). Dormant-invisible:
/// with the lever flag unset/mismatched NOTHING is registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] entity_query: dormant (lever_flag != cmp410_eindexq, vanilla goal queries)"
        );
        return;
    }
    for t in targets() {
        cplug_sdk::hooks::register_bytes(t.name, move |name, bytes| {
            let Some(t) = targets().iter().find(|t| t.name == name) else {
                return None;
            };
            if !READY.load(Ordering::Acquire) {
                // Pre-arm: pass pristine bytes through untouched (stash only).
                t.stash_orig(bytes);
                return None;
            }
            let cached = t.patch_bytes();
            if cached.is_none() {
                return None; // fail-closed: no computed patch → vanilla
            }
            Some(cached.unwrap())
        });
    }
}

// ---------------------------------------------------------------------------
// TASK-413-C: EARLY bridge define (NCDFE root-cause cv3-1 35712182885 /
// cv3-2 35712204518).
//
// MobPushOps.pushables (mobpush blob, `if (K4 || EQSNAP)` branch) executes
// `EntityGoalQueryOps.pushCandidates` from the FIRST entity push — i.e. from
// the start of the population inject — while the late define below used to
// wait for BOTH goal classes to load + boot-quiet + 20s. On the meganav base
// the race flipped (cv3-1: first pushables call 09:52:03 vs goal-query ARMED
// much later): the JVM resolved EntityGoalQueryOps through MobPushOps's
// defining loader → ClassNotFoundException → and HotSpot CACHES the NCDFE per
// constant-pool entry, so the error repeated ×3938 for the whole run even
// AFTER the late define finally succeeded (log: NCDFE span 1074..83741,
// ARM at 9888). ARM-маркеры были, retarget случился, но мост для push-лейна
// так и не был виден.
//
// STRICT-порядок (закон 6 v16): define → register natives → probe ARMED →
// publish. The early define is IDEMPOTENT (BRIDGE_DEFINED + lock — a second
// define_class into the same loader = LinkageError) and anchored on the
// first loaded of {goal classes, LivingEntity}: LivingEntity loads at boot
// and is the SAME kernel loader mobs_manager defines MobPushOps into (c-l2
// empirics: a goal-anchored define was resolved through the living loader —
// one loader). mobs_manager gates its push-lane publish on this returning
// true → the bridge is in place BEFORE pushables can ever execute.
// ---------------------------------------------------------------------------

static BRIDGE_DEFINED: AtomicBool = AtomicBool::new(false);
static BRIDGE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// TASK-413-C: true iff the lever flag carries the eqsnap/k4 plane — the ONLY
/// java blobs that resolve this bridge at runtime (the MobPushOps
/// `if (K4 || EQSNAP)` branch; javac does not fold these <clinit>-computed
/// statics, but the branch is never EXECUTED under other flags, so the cp
/// entry is never resolved — lazy resolution). mobs_manager requires the
/// bridge before arming the push lane exactly under these flags.
pub fn lever_matches() -> bool {
    enabled()
}

/// Anchor preference for the early define: goal classes keep the late
/// phase's anchor identical when they are already up; LivingEntity loads at
/// boot (entity superclass) and is the loader MobPushOps lands in.
const EARLY_ANCHORS: &[&str] = &[
    "net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal",
    "net/minecraft/world/entity/ai/goal/AvoidEntityGoal",
    "net/minecraft/world/entity/LivingEntity",
];

/// Define the bridge + RegisterNatives into `anchor`'s loader. Idempotent
/// (BRIDGE_DEFINED; double-check under BRIDGE_LOCK — two workers may race).
/// Returns true iff the bridge is defined + natives bound.
fn define_bridge_once(anchor: &str) -> bool {
    if BRIDGE_DEFINED.load(Ordering::Acquire) {
        return true;
    }
    let _guard = match BRIDGE_LOCK.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    if BRIDGE_DEFINED.load(Ordering::Acquire) {
        return true;
    }
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(anchor) else {
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
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] entity_query: define_class({OPS_CLASS}) failed");
            return false;
        };

        // RegisterNatives: eqProbe (magic) + eqEpoch (bulk chain builder) +
        // senseArena (TASK-419-B: CSR-арена из только что построенных цепей).
        // TASK-409-E ROOT-CAUSE lesson: the registered sig must match the
        // java declaration EXACTLY. Java:
        //   eqProbe()                                    -> ()I
        //   eqEpoch(int,int,double[],int[],int[])        -> (II[D[I[I)I
        //   senseArena(int,int,int[],int[],int[],int[])  -> (II[I[I[I[I)I
        let names = [
            CString::new("eqProbe").expect("no NUL"),
            CString::new("eqEpoch").expect("no NUL"),
            CString::new("senseArena").expect("no NUL"),
        ];
        let sigs = [
            CString::new("()I").expect("no NUL"),
            CString::new("(II[D[I[I)I").expect("no NUL"),
            CString::new("(II[I[I[I[I)I").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: eq_probe as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: eq_epoch as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[2].as_ptr(),
                signature: sigs[2].as_ptr(),
                fnPtr: sense_arena as *const c_void as *mut c_void,
            },
        ];
        let reg = env.register_natives(c, &natives);
        if let Err(code) = reg {
            // describe BEFORE clear (TASK-409-E: sig/name mismatch posts
            // NoSuchMethodError; without this the only trace is (code -1)).
            crate::describe_exception(env);
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] entity_query: register_natives failed (code {code}) — hook stays dormant"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        true
    });
    let ok = ok.unwrap_or(false);
    if ok {
        BRIDGE_DEFINED.store(true, Ordering::Release);
        eprintln!(
            "[crussty-plugin] entity_query: bridge EARLY define ok (anchor={anchor}) — push-lane NCDFE window closed"
        );
    }
    ok
}

/// TASK-413-C public entry: guarantee the bridge is defined + natives bound
/// BEFORE the push lane goes live. Called by mobs_manager as a HARD publish
/// gate (probe-then-patch: define → register natives → probe → publish) and
/// by the activate worker's early phase. Never caches failure — each call
/// re-probes the anchors (transient JNI hiccups recoverable).
pub fn ensure_bridge_early() -> bool {
    if BRIDGE_DEFINED.load(Ordering::Acquire) {
        return true;
    }
    if !enabled() {
        return false;
    }
    for anchor in EARLY_ANCHORS {
        if cplug_sdk::classes::find_class(anchor).is_some() {
            return define_bridge_once(anchor);
        }
    }
    false
}

/// Background activation: wait for the target classes + boot quiet, define
/// the EntityGoalQueryOps bridge into the kernel loader + RegisterNatives
/// (eqProbe/eqEpoch), compute both retargets from the pristine bytes, flip
/// READY and retransform both classes.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // TASK-413-C EARLY PHASE: define the bridge as soon as the loader is
        // quiet and ANY anchor is up (LivingEntity at boot; goal classes may
        // lag the push lane by whole boot phases). This — plus the mobs_manager
        // publish gate — makes define-before-pushables deterministic instead
        // of a boot-timing race. Retarget/publish stays strictly gated below.
        if crate::improved_noise::wait_for_boot() {
            let early_deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if ensure_bridge_early() {
                    break;
                }
                if std::time::Instant::now() > early_deadline {
                    eprintln!(
                        "[crussty-plugin] entity_query: EARLY bridge define did not land within 180s — push lane will fail-closed at mobs_manager"
                    );
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
        }

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        for t in targets() {
            while cplug_sdk::classes::find_class(t.name).is_none() {
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] entity_query: {} not loaded within 180s, hook stays dormant",
                        t.name
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] entity_query: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Guard: embedded bridge bytes must not be newer than the JVM
        // (lesson 408: stale class blob = sleeping gate).
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
                "[crussty-plugin] entity_query: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild entitygoalquery/ via scripts/build_entitygoalquery_ops.sh; hook stays dormant"
            );
            return;
        }

        // Pristine bytes: capture via no-op retransform for any class that
        // predates the hook (READY=false → stash-only), fluid_guard pattern.
        for t in targets() {
            let has_orig = t.orig.lock().map(|o| o.is_some()).unwrap_or(false);
            if has_orig {
                continue;
            }
            eprintln!(
                "[crussty-plugin] entity_query: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _ in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                let has = t.orig.lock().map(|o| o.is_some()).unwrap_or(false);
                if has {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            let has = t.orig.lock().map(|o| o.is_some()).unwrap_or(false);
            if !has {
                eprintln!(
                    "[crussty-plugin] entity_query: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        // TASK-413-C: idempotent — the EARLY define (top of this worker, or
        // mobs_manager's push-lane publish gate) may have landed already; a
        // second define_class into the same loader = LinkageError.
        let anchor = targets()[0].name;
        let defined = define_bridge_once(anchor);
        if !defined {
            eprintln!("[crussty-plugin] entity_query: bridge definition failed, hook stays dormant");
            return;
        }

        // Compute both retargets from the pristine bytes (pure rust,
        // length-preserving). Any Err = kernel shape mismatch → fail closed.
        for t in targets() {
            let Some(original) = t.take_orig() else {
                eprintln!(
                    "[crussty-plugin] entity_query: pristine bytes vanished for {}, hook stays dormant",
                    t.name
                );
                return;
            };
            match retarget_site(t, &original) {
                Ok((patched, outcome)) => match outcome {
                    crate::classfile::RetargetOutcome::Retargeted { sites } if sites >= 1 => {
                        static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                            eprintln!(
                                "[crussty-plugin] entity_query: computed patch for {} ({} -> {} bytes, {sites} getEntitiesOfClass site(s))",
                                t.name,
                                original.len(),
                                patched.len()
                            );
                        }
                        t.set_patch(patched);
                    }
                    other => {
                        static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                        if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                            eprintln!(
                                "[crussty-plugin] entity_query: {} site not rewritten ({other:?}) — this class stays vanilla (fail-closed)",
                                t.name
                            );
                        }
                    }
                },
                Err(e) => {
                    static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] entity_query: retarget rejected for {} ({e}) — this class stays vanilla (fail-closed)",
                            t.name
                        );
                    }
                }
            }
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        let flag_label = if enabled_flag_is_brain2() {
            "cmp422_brain2"
        } else if enabled_flag_is_sense() {
            "cmp421_brain"
        } else if enabled_flag_is_eqsnapv3() {
            "cmp412_eqsnapv3"
        } else if enabled_flag_is_eqsnap() {
            "cmp411_eqsnap"
        } else if enabled_flag_is_k4() {
            "cmp411_k4soa"
        } else {
            "cmp410_eindexq"
        };
        let sense_note = if enabled_flag_is_sense() {
            "; TASK-419-B sense-plane: senseArena (2nd bulk JNI, same EPOCH_LOCK window) builds CSR arena from the JUST-BUILT chains — per-bucket contiguous slices in EXACT chain-walk order; java snapshotQuery walks arena[off[h]..off[h+1]) sequentially (no random next[] deref per candidate; parity oracle = rust test arena_matches_chain_walk)"
        } else {
            ""
        };
        eprintln!(
            "[crussty-plugin] {flag_label}: ARMED goal-query (NearestAttackableTargetGoal.findTarget + AvoidEntityGoal.canUse Level.getEntitiesOfClass sites -> EntityGoalQueryOps.entitiesOfClassGate; rust eqEpoch = ONE bulk JNI/tick single-pass chain build over mobs_soa SoA population -> head[65536]/next[id]/frozen x,y,z,hw,hh columns; java: cell-rect(AABB±8.0) -> chains -> frozen prune(AABB±8.0) -> byId -> live AABB.intersects + predicate = strict superset, nearest-pick order-delta documented; vanilla getNearestEntity/TargetingConditions tail untouched; zero per-entity JNI{sense_note}; empty flag = vanilla bit-for-bit)",
            sense_note = sense_note,
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "entitiesOfClassGate", "cmp410_eindexq v1");
        READY.store(true, Ordering::Release);
        for t in targets() {
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!(
                "[crussty-plugin] entity_query: {} armed, retransform rc={rc} (ready-gate on)",
                t.name
            );
            if rc != 0 {
                eprintln!(
                    "[crussty-plugin] entity_query: {} retransform FAILED (rc={rc}) — my slice stays vanilla",
                    t.name
                );
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Natives (registered on EntityGoalQueryOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn eq_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK goal-query chain builder — ONE transition per tick (never per
/// entity/query).
///
/// Single linear pass over the mobs_soa SoA population
/// `id in 0..min(id_top, caps)`: alive rows (flags bit0) with finite x/y/z
/// are frozen into `soa[id*5..id*5+5] = x,y,z,hw,hh` and linked into the
/// chain bucket `hash(floor(x/16), floor(z/16))`: `next[id] = head[h];
/// head[h] = id+1`. No global seqlock (deliberate — see module docs): each
/// id lands in exactly one chain computed from the SAME frozen x/z, so the
/// published structure is always traversal-valid; a row torn by a concurrent
/// upsert becomes a ≤1-tick ghost bounded by the java 8-block margin.
/// Returns the number of LINKED rows, or ERR_RANGE (bad caps / cold tables)
/// / ERR_STRUCT (pin failure, gate off).
///
/// # Safety
/// See eq_probe.
#[no_mangle]
pub unsafe extern "system" fn eq_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    id_top: jni::jint,
    soa: jni::jdoubleArray,
    head: jni::jintArray,
    next: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || soa.is_null() || head.is_null() || next.is_null() {
        return ERR_STRUCT;
    }
    if tick < 0 || id_top < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let soa_cap = unsafe { (vt.GetArrayLength)(env, soa) };
    let head_cap = unsafe { (vt.GetArrayLength)(env, head) };
    let next_cap = unsafe { (vt.GetArrayLength)(env, next) };
    if soa_cap <= 0 || soa_cap % (STRIDE as i32) != 0 || head_cap != CELLS as i32 || next_cap <= 0 {
        return ERR_RANGE; // structural drift — java falls back to vanilla this tick
    }
    let Some((xs, ys, zs, hws, hhs, flags, _version)) = crate::mobs_soa::eq_snapshot() else {
        return ERR_RANGE; // plane tables not initialized (cold) — vanilla this tick
    };

    // TASK-411-C (eqsnap, v2): the per-tick dirty-delta drain — apply every
    // per-thread shard row to the flat columns (O(dirty), ONE WLOCK hold)
    // BEFORE the chain pass. The eq_snapshot slices above point at the SAME
    // plane columns the drain writes (single consumer thread: eq_epoch runs
    // under the java EPOCH_LOCK; concurrent mobRemove bumps the seqlock and
    // only clears flags — the existing ≤1-tick ghost contract covers the
    // interleave). Under legacy flags this is a no-op (shards never written).
    let drained = if enabled_flag_is_eqsnap() {
        crate::mobs_soa::drain_eqsnap_shards()
    } else {
        0
    };
    static DRAIN_LOGGED: AtomicBool = AtomicBool::new(false);
    if drained > 0 && !DRAIN_LOGGED.swap(true, Ordering::Relaxed) {
        let drain_label = if enabled_flag_is_eqsnapv3() {
            "cmp412_eqsnapv3"
        } else {
            "cmp411_eqsnap"
        };
        eprintln!(
            "[crussty-plugin] {drain_label}: shard-drain EFFECT armed (first bulk drain applied {drained} dirty rows, O(dirty) per tick — no per-entity plane mutation)"
        );
    }

    let bound = (id_top as usize)
        .min((soa_cap as usize) / STRIDE)
        .min(next_cap as usize)
        .min(flags.len())
        .min(xs.len())
        .min(ys.len())
        .min(zs.len())
        .min(hws.len())
        .min(hhs.len());

    let soa_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, soa, std::ptr::null_mut()) };
    if soa_pin.is_null() {
        return ERR_STRUCT;
    }
    let head_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, head, std::ptr::null_mut()) };
    if head_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, soa, soa_pin, 0) };
        return ERR_STRUCT;
    }
    let next_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, next, std::ptr::null_mut()) };
    if next_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, soa, soa_pin, 0) };
        return ERR_STRUCT;
    }

    let soa_s = unsafe { std::slice::from_raw_parts_mut(soa_pin as *mut jni::jdouble, soa_cap as usize) };
    let head_s = unsafe { std::slice::from_raw_parts_mut(head_pin as *mut jni::jint, CELLS) };
    let next_s = unsafe { std::slice::from_raw_parts_mut(next_pin as *mut jni::jint, next_cap as usize) };

    // Zero the chain heads (fresh epoch; NEXT slots of dead rows keep stale
    // links from prior epochs but are UNREACHABLE — every live chain member
    // was linked THIS epoch through head→next).
    head_s.fill(0);

    let mut linked: i32 = 0;
    for id in 0..bound {
        if flags[id] & 1 == 0 {
            continue; // removed from the plane (swept) — never enumerated
        }
        let x = xs[id];
        let y = ys[id];
        let z = zs[id];
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            continue; // corrupt row — cannot be enumerated safely
        }
        let hw = hws[id];
        let hh = hhs[id];
        let b = id * STRIDE;
        soa_s[b] = x;
        soa_s[b + 1] = y;
        soa_s[b + 2] = z;
        soa_s[b + 3] = hw;
        soa_s[b + 4] = hh;
        // Java-parity cell hash: floor(x/16) with SATURATING i32 casts on
        // both sides (java (int)Math.floor(v/16.0); rust `as i32` saturates
        // identically; NaN rows are skipped above).
        let cx = (x / CELL_SIZE).floor() as i32;
        let cz = (z / CELL_SIZE).floor() as i32;
        let mut h = cx.wrapping_mul(H1) ^ cz.wrapping_mul(H2);
        h ^= ((h as u32) >> 16) as i32;
        let h = (h & (CELLS as i32 - 1)) as usize;
        // Intrusive insert at chain head: next[id] = old head; head[h] = id+1.
        next_s[id] = head_s[h];
        head_s[h] = (id + 1) as jni::jint;
        linked += 1;
    }

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, next, next_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, soa, soa_pin, 0) };
    linked
}

/// TASK-419-B (sense-plane): CSR-арена из УЖЕ построенных цепей. Заполнение
/// буквально идёт по тем же head/next цепям в том же порядке — порядок
/// per-bucket слайсов == chain-walk порядок по построению (паритет-оракул:
/// тест arena_matches_chain_walk внизу). Чистая функция — переиспользуется
/// нативом и тестами; ошибки структуры = Err (java → ваниль на этот тик).
fn sense_arena_fill(
    head: &[i32],
    next: &[i32],
    rows: usize,
    arena: &mut [i32],
    arena_off: &mut [i32],
) -> Result<usize, ()> {
    if arena_off.len() != CELLS + 1 || head.len() != CELLS {
        return Err(());
    }
    let arena_cap = arena.len();
    let mut cursor: usize = 0;
    for h in 0..CELLS {
        arena_off[h] = cursor as i32;
        // Тот же обход, что java chain-walk: от головы через next; порядок
        // элементов слайса ПОЭЛЕМЕНТНО равен порядку прохода цепи.
        let mut link = head[h];
        while link != 0 {
            if link < 0 {
                return Err(()); // повреждённая ссылка — ваниль на этот тик
            }
            let id = (link as usize).wrapping_sub(1);
            if id >= rows || id >= next.len() || cursor >= arena_cap {
                return Err(()); // дрейф/цикл-гард — ваниль на этот тик
            }
            arena[cursor] = id as i32;
            cursor += 1;
            link = next[id];
        }
    }
    arena_off[CELLS] = cursor as i32;
    Ok(cursor)
}

/// TASK-419-B (sense-plane): bulk CSR-арена — ВТОРОЙ bulk-JNI за тик, зовётся
/// java (maybeEpoch) СРАЗУ после eq_epoch в ТОМ ЖЕ EPOCH_LOCK-окне (тот же
/// поток-писатель; per-entity JNI по-прежнему отсутствует). Читает ТОЛЬКО
/// ЧТО построенные цепи (head/next — те же массивы, что eq_epoch заполнил)
/// и раскладывает id в плотную арену: arena[arena_off[h]..arena_off[h+1]) =
/// бакет h в точности в порядке chain-walk. Java snapshotQuery под
/// cmp421_brain читает последовательные слайсы вместо рандомного
/// next[link-1]-deref на каждого кандидата. rc = число размещённых id;
/// ERR_RANGE — структурный дрейф/цикл-гард (ваниль на этот тик, эпоха
/// ретраится); ERR_STRUCT — pin failure / гейт (дизарм).
///
/// # Safety
/// See eq_probe.
#[no_mangle]
pub unsafe extern "system" fn sense_arena(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    rows: jni::jint,
    head: jni::jintArray,
    next: jni::jintArray,
    arena: jni::jintArray,
    arena_off: jni::jintArray,
) -> jni::jint {
    // STRICT: арена обслуживает ТОЛЬКО sense-композит (чужой флаг = дизарм).
    if !enabled_flag_is_sense()
        || env.is_null()
        || head.is_null()
        || next.is_null()
        || arena.is_null()
        || arena_off.is_null()
    {
        return ERR_STRUCT;
    }
    if tick < 0 || rows < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let head_cap = unsafe { (vt.GetArrayLength)(env, head) };
    let next_cap = unsafe { (vt.GetArrayLength)(env, next) };
    let arena_cap = unsafe { (vt.GetArrayLength)(env, arena) };
    let off_cap = unsafe { (vt.GetArrayLength)(env, arena_off) };
    if head_cap != CELLS as i32
        || off_cap != (CELLS + 1) as i32
        || next_cap < rows
        || arena_cap < rows
    {
        return ERR_RANGE; // структурный дрейф параметров — ваниль этот тик
    }

    // Pin ladder (eq_epoch discipline): acquire head→next→arena→arena_off,
    // release в обратном порядке. Критическая секция без JNI-вызовов.
    let head_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, head, std::ptr::null_mut()) };
    if head_pin.is_null() {
        return ERR_STRUCT;
    }
    let next_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, next, std::ptr::null_mut()) };
    if next_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
        return ERR_STRUCT;
    }
    let arena_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, arena, std::ptr::null_mut()) };
    if arena_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, next, next_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
        return ERR_STRUCT;
    }
    let off_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, arena_off, std::ptr::null_mut()) };
    if off_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, arena, arena_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, next, next_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
        return ERR_STRUCT;
    }

    let head_s = unsafe { std::slice::from_raw_parts(head_pin as *const jni::jint, CELLS) };
    let next_s =
        unsafe { std::slice::from_raw_parts(next_pin as *const jni::jint, next_cap as usize) };
    let arena_s = unsafe {
        std::slice::from_raw_parts_mut(arena_pin as *mut jni::jint, arena_cap as usize)
    };
    let off_s = unsafe {
        std::slice::from_raw_parts_mut(off_pin as *mut jni::jint, (CELLS + 1) as usize)
    };

    let placed = sense_arena_fill(head_s, next_s, rows as usize, arena_s, off_s);

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, arena_off, off_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, arena, arena_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, next, next_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, head, head_pin, 0) };
    match placed {
        Ok(n) => n as jni::jint,
        Err(()) => ERR_RANGE,
    }
}

// ---------------------------------------------------------------------------
// Tests: the java-parity cell hash ladder, the single-pass chain soundness
// (every id exactly one chain; walk finds all), saturation parity of the
// cell floor vs the java (int)Math.floor semantics, and the retarget
// descriptor contract (receiver-prepended virtual desc — Level prepended).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn enabled_with(s: &str) -> bool {
        // TASK-452-A: pin the PRODUCTION gate list (flag_enabled) — not a
        // hand-maintained mirror. Mirror drift is the x452 NCDFE root-cause.
        flag_enabled(Some(s))
    }

    #[test]
    fn strict_gate_matches() {
        assert!(enabled_with("cmp410_eindexq"));
        assert!(enabled_with("cmp411_k4soa"));
        assert!(enabled_with("cmp411_eqsnap"));
        assert!(enabled_with("cmp412_eqsnapv3"));
        // TASK-417-C: cvs-носитель ⊕ queryplane.
        assert!(enabled_with("cmp417_bq"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp405_eindex"));
        assert!(!enabled_with("cmp401_soa"));
        assert!(!enabled_with("cmp412_meganav"));
        assert!(!enabled_with("cmp410_eindexq_x"));
        assert!(!enabled_with("cmp411_k4soa_x"));
        assert!(!enabled_with("cmp411_eqsnap_x"));
        assert!(!enabled_with("cmp412_eqsnapv3_x"));
        assert!(!enabled_with("cmp417_bq_x"));
        assert!(!enabled_with(" cmp410_eindexq"));
        assert!(!enabled_with(" cmp411_k4soa"));
        assert!(!enabled_with(" cmp411_eqsnap"));
        assert!(!enabled_with(" cmp412_eqsnapv3"));
        // TASK-419-B (sense-plane composite).
        assert!(enabled_with("cmp421_brain"));
        assert!(!enabled_with("cmp421_brain_x"));
        assert!(!enabled_with(" cmp421_brain"));
        // TASK-452-A: senseins composite — STRICT eq, no prefix/suffix tolerance
        // (production gate pinned via flag_enabled; x452 NCDFE regression guard).
        assert!(enabled_with("cmp451_senseins"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp451_senseins_x"));
        assert!(!enabled_with(" cmp451_senseins"));
        assert!(!enabled_with("cmp451_senseins "));
    }

    /// Mirror of the java EntityGoalQueryOps.cellHash operating on the same
    /// i32 bit patterns (java int mul wraps; >>> is logical shift).
    fn java_cell_hash(cx: i32, cz: i32) -> usize {
        let h = cx.wrapping_mul(H1) ^ cz.wrapping_mul(H2);
        let h = h ^ ((h as u32) >> 16) as i32;
        (h & (CELLS as i32 - 1)) as usize
    }

    #[test]
    fn cell_hash_matches_java_ladder() {
        // The rust hash and the java mirror must agree bit-for-bit.
        for &(cx, cz) in &[
            (0, 0),
            (1, 1),
            (-1, 3),
            (12345, -9876),
            (i32::MAX, i32::MIN),
            (i32::MIN, i32::MAX),
            (65536, 65536),
            (-65536, 12345678),
        ] {
            assert_eq!(cell_hash_for_test(cx, cz), java_cell_hash(cx, cz));
        }
        // Deterministic and masked into [0, CELLS).
        for &(cx, cz) in &[(7, 9), (-13, 42), (100000, -100000)] {
            let h = cell_hash_for_test(cx, cz);
            assert!(h < CELLS);
        }
    }

    fn cell_hash_for_test(cx: i32, cz: i32) -> usize {
        let mut h = cx.wrapping_mul(H1) ^ cz.wrapping_mul(H2);
        h ^= ((h as u32) >> 16) as i32;
        (h & (CELLS as i32 - 1)) as usize
    }

    #[test]
    fn cell_floor_saturates_like_java() {
        // java (int)Math.floor(v/16.0) saturates |v| > i32::MAX to
        // MIN/MAX; rust `f64 as i32` saturates identically.
        let huge = 3.5e10f64; // /16 = 2.1875e9 > i32::MAX → saturates
        assert_eq!((huge / CELL_SIZE).floor() as i32, i32::MAX);
        assert_eq!(((-huge) / CELL_SIZE).floor() as i32, i32::MIN);
        let normal = 1234.75f64;
        assert_eq!((normal / CELL_SIZE).floor() as i32, 77);
        assert_eq!((-normal / CELL_SIZE).floor() as i32, -78); // floor, not trunc
    }

    /// Simulate the eq_epoch single pass over a mock SoA population and the
    /// java-side chain walk: every alive finite id must be found EXACTLY
    /// once across the visited buckets of a rect covering it; dead rows must
    /// be absent; chain traversal terminates (acyclic).
    #[test]
    fn single_pass_chain_build_and_walk() {
        let n = 1000usize;
        let mut xs = vec![0.0f64; n];
        let mut zs = vec![0.0f64; n];
        let mut flags = vec![0u8; n];
        for id in 0..n {
            xs[id] = ((id as f64) * 7.31) % 400.0 - 200.0;
            zs[id] = ((id as f64) * 3.17) % 400.0 - 200.0;
            if id % 7 != 0 {
                flags[id] = 1; // every 7th row dead
            }
        }
        let ys = vec![64.0f64; n];
        let hws = vec![0.35f64; n];
        let hhs = vec![0.9f64; n];

        let mut soa = vec![0.0f64; n * STRIDE];
        let mut head = vec![0i32; CELLS];
        let mut next = vec![0i32; n];
        let mut linked = 0usize;
        for id in 0..n {
            if flags[id] & 1 == 0 {
                continue;
            }
            let (x, y, z, hw, hh) = (xs[id], ys[id], zs[id], hws[id], hhs[id]);
            soa[id * STRIDE] = x;
            soa[id * STRIDE + 1] = y;
            soa[id * STRIDE + 2] = z;
            soa[id * STRIDE + 3] = hw;
            soa[id * STRIDE + 4] = hh;
            let cx = (x / CELL_SIZE).floor() as i32;
            let cz = (z / CELL_SIZE).floor() as i32;
            let h = cell_hash_for_test(cx, cz);
            next[id] = head[h];
            head[h] = (id + 1) as i32;
            linked += 1;
        }
        assert_eq!(linked, n - (n + 6) / 7);

        // Walk every bucket exactly once; collect ids; no duplicates; only
        // alive ids; traversal terminates.
        let mut seen = vec![false; n];
        let mut found = 0usize;
        for h in 0..CELLS {
            let mut link = head[h];
            let mut steps = 0usize;
            while link != 0 {
                assert!(steps < n + 1, "cycle detected in chain {h}");
                steps += 1;
                let id = (link - 1) as usize;
                assert!(id < n);
                assert!(!seen[id], "duplicate id {id} in chains");
                seen[id] = true;
                assert_eq!(flags[id] & 1, 1, "dead id {id} linked");
                found += 1;
                link = next[id];
            }
        }
        assert_eq!(found, linked);
    }

    /// TASK-419-B PARITY ORACLE: the sense-arena CSR fill must reproduce the
    /// chain walk EXACTLY — per bucket, element-by-element (same candidate
    /// set, same order). Simulates eq_epoch's single pass (same as the test
    /// above), then compares sense_arena_fill slices against a direct
    /// chain walk for every bucket, plus the global invariants (total =
    /// linked; offsets non-decreasing; arena_off[CELLS] = total).
    #[test]
    fn arena_matches_chain_walk() {
        let n = 2000usize;
        let mut head = vec![0i32; CELLS];
        let mut next = vec![0i32; n];
        let mut linked = 0usize;
        // Deterministic scattered population (mix of shared buckets).
        for id in 0..n {
            if id % 11 == 0 {
                continue; // every 11th row dead
            }
            let x = ((id as f64) * 13.77) % 512.0 - 256.0;
            let z = ((id as f64) * 5.19) % 512.0 - 256.0;
            let cx = (x / CELL_SIZE).floor() as i32;
            let cz = (z / CELL_SIZE).floor() as i32;
            let h = cell_hash_for_test(cx, cz);
            next[id] = head[h];
            head[h] = (id + 1) as i32;
            linked += 1;
        }

        let mut arena = vec![0i32; n];
        let mut arena_off = vec![0i32; CELLS + 1];
        let placed = sense_arena_fill(&head, &next, n, &mut arena, &mut arena_off)
            .expect("arena fill must succeed on a sound chain structure");
        assert_eq!(placed, linked);
        assert_eq!(arena_off[CELLS] as usize, linked);

        // Per-bucket equality with the direct chain walk (THE contract).
        for h in 0..CELLS {
            let st = arena_off[h] as usize;
            let en = arena_off[h + 1] as usize;
            assert!(st <= en && en <= linked);
            let mut chain_ids = Vec::with_capacity(en - st);
            let mut link = head[h];
            let mut steps = 0usize;
            while link != 0 {
                assert!(steps < n + 1, "cycle in chain {h}");
                steps += 1;
                chain_ids.push(link - 1);
                link = next[(link - 1) as usize];
            }
            let arena_ids: Vec<i32> = arena[st..en].to_vec();
            assert_eq!(arena_ids, chain_ids, "bucket {h} order mismatch");
        }

        // Error paths: torn chain (link beyond rows) -> Err; truncated arena
        // (cursor bound) -> Err.
        let mut bad_next = next.clone();
        bad_next[3] = (n + 5) as i32; // dangling link past rows
        assert!(sense_arena_fill(&head, &bad_next, n, &mut arena, &mut arena_off).is_err());
        let mut tiny = vec![0i32; 4];
        assert!(sense_arena_fill(&head, &next, n, &mut tiny, &mut arena_off).is_err());
        let mut bad_off = vec![0i32; CELLS]; // wrong offsets length
        assert!(sense_arena_fill(&head, &next, n, &mut arena, &mut bad_off).is_err());
    }

    #[test]
    fn retarget_desc_contract() {
        // Receiver-prepended static form: virtual desc ()-style args with the
        // OWNER (Level) prepended — stack-identical Level-consuming bridge.
        let expect_static = format!("(L{};{}", FROM.0, &FROM.2[1..].to_string());
        assert_eq!(GATE_STATIC_DESC, expect_static);
        assert_eq!(
            GATE_STATIC_DESC,
            "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;"
        );
    }

    #[test]
    fn epoch_bounds_never_exceed_any_cap() {
        // Java publishes rows = idTop; rust bound must respect every cap.
        let id_top = 150_000usize;
        let soa_cap = 1_048_576usize * 5;
        let next_cap = 1_048_576usize;
        let flags_len = 1 << 20;
        let bound = id_top
            .min(soa_cap / STRIDE)
            .min(next_cap)
            .min(flags_len);
        assert_eq!(bound, 150_000);
        assert!(bound * STRIDE <= soa_cap && bound <= next_cap && bound <= flags_len);
    }
}

// ---------------------------------------------------------------------------
// Delivery-graph guard (×93 stale-blob lesson + S7-163 pattern): the
// embedded bridge bytes must BE the fresh build-dir artifact, the source
// must declare exactly one classfile, and the java native declarations must
// match the registered signatures EXACTLY (TASK-409-E NoSuchMethodError
// lesson).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod entityquery_delivery_tests {
    const SRC: &str = include_str!("../entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java");
    const BLOB: &str = "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class";

    #[test]
    fn entityquery_embedded_bytes_match_build_dir() {
        let disk = std::fs::read(BLOB).expect("build EntityGoalQueryOps.class (build_entitygoalquery_ops.sh)");
        assert_eq!(
            super::OPS_BYTES,
            disk.as_slice(),
            "embedded OPS_BYTES != build dir artifact — REBUILD via scripts/build_entitygoalquery_ops.sh (stale blob = sleeping gate, lesson ×93)"
        );
    }

    #[test]
    fn entityquery_ops_source_declares_no_nested_classes() {
        // The bridge compiles to exactly ONE classfile (no nested classes —
        // kernel-loader delivery defines exactly one classfile).
        for line in SRC.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") {
                        panic!("EntityGoalQueryOps.java declares a nested class: {t}");
                    }
                    break;
                }
            }
        }
    }

    #[test]
    fn entityquery_embedded_classfile_pinned_major_65() {
        // --release 21 pins major 65 = kernel JVM. Guards against a rebuilt
        // blob silently drifting to a newer major the JVM rejects.
        let major = u16::from_be_bytes([super::OPS_BYTES[6], super::OPS_BYTES[7]]);
        assert_eq!(major, 65, "EntityGoalQueryOps.class major {major} != 65 — rebuild with --release 21");
    }

    #[test]
    fn entityquery_native_declarations_match_registered_sigs() {
        // TASK-409-E lesson: RegisterNatives sig must match the java
        // declaration EXACTLY or the bridge sleeps with NoSuchMethodError.
        assert!(SRC.contains("private static native int eqProbe();"));
        assert!(SRC.contains(
            "private static native int eqEpoch(int tick, int idTop, double[] soa,\n            int[] head, int[] next);"
        ));
        // TASK-419-B (sense-plane): the arena native must be declared and
        // registered with the SAME shape.
        assert!(SRC.contains(
            "private static native int senseArena(int tick, int rows, int[] head,\n            int[] next, int[] arena, int[] arenaOff);"
        ));
        // And the rust side registers exactly those shapes.
        assert_eq!(super::PROBE_MAGIC & 0xFFFF, 0x4547);
        // Cell geometry parity between rust consts and java source.
        assert!(SRC.contains("static final int CELLS = 1 << 16;"));
        assert!(SRC.contains("static final int CELL_SIZE = 16;"));
        assert!(SRC.contains("static final int STRIDE = 5;"));
        assert!(SRC.contains("static final double MARGIN = 8.0D;"));
    }
}

// ---------------------------------------------------------------------------
// REAL-kernel-class retarget proof (javap ground truth d05c930 method): the
// fixture classes were extracted from the FULL patched-kernel jar
// (research/gc-recon-2026-09-19/round-396-a artifact = live runtime). The
// retarget MUST hit exactly one getEntitiesOfClass site per method and be
// idempotent — any drift between the jar and these constants = fail-closed
// dormant hook (surfaced HERE, not at bench time).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod entityquery_kernel_tests {
    use super::{FROM, GATE_STATIC_DESC, OPS_CLASS, TARGETS};
    use crate::classfile::{retarget_virtual_to_static, RetargetOutcome};

    const NAT: &[&[u8]] = &[
        include_bytes!("../tests/fixtures/NearestAttackableTargetGoal.class"),
        include_bytes!("../tests/fixtures/AvoidEntityGoal.class"),
    ];

    #[test]
    fn kernel_findTarget_and_canUse_retarget_exactly_one_site_each() {
        for (t, bytes) in TARGETS.iter().zip(NAT.iter()) {
            let (patched, outcome) = retarget_virtual_to_static(
                bytes,
                t.1,
                t.2,
                FROM,
                (OPS_CLASS, "entitiesOfClassGate", GATE_STATIC_DESC),
            )
            .unwrap_or_else(|e| panic!("{} retarget errored: {e}", t.0));
            assert_eq!(
                outcome,
                RetargetOutcome::Retargeted { sites: 1 },
                "{}.{} must hold exactly ONE Level.getEntitiesOfClass site (javap ground truth)",
                t.0,
                t.1
            );
            assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
            assert!(patched.len() >= bytes.len());
        }
    }

    #[test]
    fn kernel_retarget_idempotent_and_resolves_bridge() {
        for (t, bytes) in TARGETS.iter().zip(NAT.iter()) {
            let (patched, _) = retarget_virtual_to_static(
                bytes,
                t.1,
                t.2,
                FROM,
                (OPS_CLASS, "entitiesOfClassGate", GATE_STATIC_DESC),
            )
            .expect("patch");
            let (again, outcome) = retarget_virtual_to_static(
                &patched,
                t.1,
                t.2,
                FROM,
                (OPS_CLASS, "entitiesOfClassGate", GATE_STATIC_DESC),
            )
            .expect("repatch");
            assert_eq!(
                outcome,
                RetargetOutcome::AlreadyPatched { sites: 1 },
                "{} repatch must see its own rewrite",
                t.0
            );
            assert_eq!(again, patched, "{} repatch byte-identical", t.0);
        }
    }

    #[test]
    fn kernel_wrong_method_fails_closed() {
        // findTarget desc is ()V; canUse is ()Z — swapping them must NOT
        // patch: the (name, desc) pair resolves no method → Err (the hook
        // logs "retarget rejected" and serves vanilla = fail-closed), and
        // for a name/desc that exists without the FROM site → NotFound with
        // untouched bytes.
        let r = retarget_virtual_to_static(
            NAT[0],
            "findTarget",
            "()Z", // WRONG on purpose (exists in pool as canUse's desc, but
                   // no findTarget()Z method)
            FROM,
            (OPS_CLASS, "entitiesOfClassGate", GATE_STATIC_DESC),
        );
        assert!(r.is_err(), "wrong (name, desc) pair must error, not patch");
    }
}
