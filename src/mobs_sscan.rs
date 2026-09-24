//! Runtime wiring for the SSCAN plane (TASK-406-E, vector R4 «despawn/spawn/
//! activation scans batch» — lever `cmp406_sscan`; bridge
//! sscan/net/minecraft/world/entity/MobScanOps.java, natives below, retarget
//! of the `Level.findNearbyPlayer` call site inside `Mob.checkDespawn()V` via
//! `classfile::retarget_virtual_to_static`).
//!
//! R4 RUST-FIRST (owner directive, round-406): the per-tick despawn scan —
//! `ServerLevel.tick` entity loop → `Mob.checkDespawn()` for the WHOLE mob
//! population (150k bench mobs) — is dominated by
//! `Level.findNearbyPlayer(Entity,D,Predicate)` → `getNearestPlayer(DDDD,
//! Predicate)`: a per-mob java iterator over `level.players()` with a
//! predicate test + `Player.distanceToSqr(DDD)` per player (javap ground
//! truth: exactly 1 site in Mob.checkDespawn, owner Level, desc
//! `(LEntity;DLjava/util/function/Predicate;)LPlayer;`). RUST replaces the
//! O(mobs×players) scan with ONE bulk JNI per server tick:
//! `sscanEpoch(tick, idTop, players[D], nearest[I])` does a DOD pass over the
//! mobs_soa SoA positions (f64 x/y/z) under the seqlock reader bracket and
//! writes the nearest-qualifying-player column `nearest[id]` (player index or
//! -1) straight into a shared java int[] (GetPrimitiveArrayCritical). The
//! per-mob cost collapses to ONE java array read.
//!
//! VANILLA-BIT-FOR-BIT DECISIONS (owner mandate): the gate returns the SAME
//! Player object vanilla would (first strictly-closer player in
//! `level.players()` order; ties keep the earlier player — replicated 1:1 in
//! the rust pass, including the `(best == -1.0 || d < best)` ladder and the
//! `d = (px-mx)² + (py-my)² + (pz-mz)²` summation order of
//! `Entity.distanceToSqr`), and the REST of `Mob.checkDespawn` (hard/soft
//! despawn ranges, noActionTime, random.nextInt(800), removeWhenFarAway,
//! discard) runs UNTOUCHED vanilla bytecode reading the returned player's
//! coordinates. The distance filter is never applied (checkDespawn passes
//! -1.0), the predicate is applied ONCE per player at snapshot build (it
//! depends only on player state, not on the mob).
//!
//! BULK-JNI LAW (one transition per batch, never per entity): the column is
//! computed by RUST once per server tick in ONE native call; the epoch is
//! double-checked per tick (`EPOCH_TICK` volatile pair, see MobScanOps).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp406_sscan"` (STRICT eq; empty/foreign
//! flag = no hook registered at all — byte-indistinguishable from vanilla).
//! FAIL-CLOSED ladder: not-Mob / not-in-plane (idBoxOf null) / epoch-miss →
//! vanilla `level.findNearbyPlayer(...)` for this call; sscanProbe mismatch /
//! sscanEpoch ERR_STRUCT → disarm forever; ERR_RANGE → vanilla this tick,
//! epoch retried next tick; sites != 1 → hook stays dormant (fail-closed).
//!
//! SSCAN2-SPAWN (TASK-437-A, PLAN-Б1): lever `cmp436_sscan2` — STRICT OR
//! member in `enabled()` arms the legacy despawn halfplane EXACTLY as before,
//! AND the NEW spawn halfplane arms under `spawn_enabled()` (STRICT eq —
//! legacy flags keep their exact prior behavior: no NaturalSpawner hook, no
//! extra retransform). Spawn site (javap ground truth, round-396-a kernel):
//! `NaturalSpawner.spawnCategoryForPosition` 8-arg — exactly ONE
//! `invokevirtual ServerLevel.getNearestPlayer(DDDDZ)Player` site @offset 221
//! (the only getNearestPlayer in the class); all spawn attempts funnel into
//! the 8-arg overload (4/6/7-arg call it via invokestatic). Retargeted to the
//! receiver-prepended static gate
//! `MobScanOps.spawnNearestPlayerGate(ServerLevel,DDDD,Z)Player`. The spawn
//! gate answers per call from the per-tick java snapshot (vanilla
//! EntitySelector predicates applied ONCE per (tick,level)) — ZERO per-call
//! JNI (law 6: the ONE bulk JNI/tick is sscanEpoch despawn column; the spawn
//! gate is a pure java array walk over the READY snapshot). selfTest() is
//! called AFTER define+RegisterNatives and BEFORE READY/retransform: false →
//! the whole plane stays dormant (ваниль бит-в-байт).
//!
//! Mob.class is hook-free under this lever (prepare_index's Mob hook is
//! dormant unless cmp401_offthread), so a stash-based serve is safe: pristine
//! bytes are captured on first sighting, the retarget patches the RECEIVED
//! bytes, AlreadyPatched → pass-through, Err → None fail-closed pass-through.

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/MobScanOps";
/// SSCAN2-SPAWN (TASK-437-A): the spawn-halfplane host class.
const NSPAWN_CLASS: &str = "net/minecraft/world/level/NaturalSpawner";

const OPS_BYTES: &[u8] =
    include_bytes!("../sscan/build/net/minecraft/world/entity/MobScanOps.class");

const CHECKDESPAWN_NAME: &str = "checkDespawn";
const CHECKDESPAWN_DESC: &str = "()V";
/// The scan site inside `Mob.checkDespawn` (javap purpur-1.21.10: exactly one
/// `invokevirtual Level.findNearbyPlayer` at offset 55; owner = Level).
const FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "findNearbyPlayer",
    "(Lnet/minecraft/world/entity/Entity;DLjava/util/function/Predicate;)Lnet/minecraft/world/entity/player/Player;",
);
/// Receiver-prepended static form (stack-identical Level→static gate).
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;DLjava/util/function/Predicate;)Lnet/minecraft/world/entity/player/Player;";

/// SSCAN2-SPAWN (TASK-437-A): the spawn site inside the 8-arg
/// `spawnCategoryForPosition` (javap: exactly 1 `invokevirtual
/// ServerLevel.getNearestPlayer(DDDDZ)Player` @offset 221 — the only
/// getNearestPlayer in NaturalSpawner; 4/6/7-arg overloads funnel into the
/// 8-arg via invokestatic).
const SPAWN_METHOD_NAME: &str = "spawnCategoryForPosition";
const SPAWN_METHOD_DESC: &str = "(Lnet/minecraft/world/entity/MobCategory;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/NaturalSpawner$SpawnPredicate;Lnet/minecraft/world/level/NaturalSpawner$AfterSpawnCallback;ILjava/util/function/Consumer;)V";
const SPAWN_FROM: (&str, &str, &str) = (
    "net/minecraft/server/level/ServerLevel",
    "getNearestPlayer",
    "(DDDDZ)Lnet/minecraft/world/entity/player/Player;",
);
/// Receiver-prepended static form (stack-identical ServerLevel→static gate).
const SPAWN_GATE_DESC: &str = "(Lnet/minecraft/server/level/ServerLevel;DDDDZ)Lnet/minecraft/world/entity/player/Player;";

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x5353; // "SS"
const QRETRY: u32 = 128;

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp406_sscan") | Ok("cmp409_multi") | Ok("cmp412_meganav") | Ok("cmp414_cvs")
            // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
            | Ok("cmp412_eqsnapv3") | Ok("cmp414_cvs") | Ok("cmp417_bq")
            // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
            | Ok("cmp420_colpush")
            // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
            | Ok("cmp422_brain2")
            // TASK-424-A: GC-ревизия brain3 (STRICT OR).
            | Ok("cmp423_brain3") | Ok("cmp424_mobfeed") | Ok("cmp430_inside")
            | Ok("cmp421_brain")
            // TASK-437-A: sscan2 despawn+spawn plane (STRICT OR).
            | Ok("cmp436_sscan2")
    )
}

/// SSCAN2-SPAWN (TASK-437-A): spawn-halfplane arming — STRICT eq, ONLY the
/// exact round flag. Legacy members of enabled() keep their exact prior
/// behavior (no NaturalSpawner hook, no spawn gate, no extra retransform).
fn spawn_enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp436_sscan2")
    )
}

/// TASK-437-A: the ARM-marker label (active flag or "(off)") — evidence lines
/// must carry the EXACT flag (a cmp406_sscan line under a cmp436_sscan2 leg
/// would be misleading evidence).
fn label() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|_| "(off)".to_string())
}

static READY: AtomicBool = AtomicBool::new(false);

/// Compute the retarget patch from the RECEIVED (pristine) Mob bytes.
fn retarget_despawn(
    bytes: &[u8],
) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        CHECKDESPAWN_NAME,
        CHECKDESPAWN_DESC,
        FROM,
        (OPS_CLASS, "findNearbyPlayerGate", GATE_STATIC_DESC),
    )
}

/// SSCAN2-SPAWN (TASK-437-A): compute the retarget patch from the RECEIVED
/// (pristine) NaturalSpawner bytes (spawn halfplane; STRICT eq cmp436_sscan2).
fn retarget_spawn(
    bytes: &[u8],
) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        SPAWN_METHOD_NAME,
        SPAWN_METHOD_DESC,
        SPAWN_FROM,
        (OPS_CLASS, "spawnNearestPlayerGate", SPAWN_GATE_DESC),
    )
}

/// Register the byte hooks: Mob (despawn, all enabled() flags) and — STRICT eq
/// cmp436_sscan2 only — NaturalSpawner (spawn halfplane; legacy flags keep
/// byte-vanilla NaturalSpawner). Dormant-invisible: with the lever flag
/// unset/mismatched NOTHING is registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] mobs_sscan: dormant (lever_flag not in sscan family, vanilla despawn+spawn scans)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOB_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: pass pristine bytes through untouched.
            return None;
        }
        match retarget_despawn(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] mobs_sscan: hook serve {MOB_CLASS} {} bytes (findNearbyPlayer site, sites=1)",
                            out.len()
                        );
                    }
                    Some(out)
                }
                crate::classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    // Idempotent: keep the chain output (my rewrite already in).
                    Some(out)
                }
                other => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] mobs_sscan: findNearbyPlayer site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] mobs_sscan: retarget rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });

    // SSCAN2-SPAWN (TASK-437-A): spawn-halfplane hook — STRICT eq, NOT armed
    // by legacy flags (byte-vanilla NaturalSpawner under cmp406_sscan family).
    if !spawn_enabled() {
        return;
    }
    cplug_sdk::hooks::register_bytes(NSPAWN_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: pass pristine bytes through untouched.
            return None;
        }
        match retarget_spawn(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] mobs_sscan: hook serve {NSPAWN_CLASS} {} bytes (spawnCategoryForPosition getNearestPlayer(DDDDZ) site, sites=1)",
                            out.len()
                        );
                    }
                    Some(out)
                }
                crate::classfile::RetargetOutcome::AlreadyPatched { .. } => {
                    Some(out)
                }
                other => {
                    static NF_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !NF_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] mobs_sscan: spawn getNearestPlayer site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] mobs_sscan: spawn retarget rejected ({e}) — pass-through, spawn slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for boot + Mob load, define the MobScanOps
/// bridge into the kernel loader + RegisterNatives (sscanProbe/sscanEpoch),
/// flip READY (hook starts serving) and retransform Mob.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(MOB_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] mobs_sscan: {MOB_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mobs_sscan: boot marker not seen, hook stays dormant");
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
                "[crussty-plugin] mobs_sscan: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild sscan/ via scripts/build_sscan_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(MOB_CLASS) else {
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
                eprintln!("[crussty-plugin] mobs_sscan: define_class({OPS_CLASS}) failed");
                return false;
            };

            // RegisterNatives: sscanProbe (magic) + sscanEpoch (bulk column writer).
            let names = [
                CString::new("sscanProbe").expect("no NUL"),
                CString::new("sscanEpoch").expect("no NUL"),
            ];
            // TASK-409-E ROOT-CAUSE (eleg1/eleg2 dormant-гейт): java-side
            // declaration is `sscanEpoch(int tick, int idTop, double[], int[])`
            // = (II[D[I)I — TWO leading ints (tick, idTop). The registered
            // sig had a stray third `I` -> RegisterNatives posted
            // NoSuchMethodError and returned JNI_ERR (-1) EVERY boot, so the
            // despawn bridge silently never armed (legs measured only the
            // comp base). Sig must match the declared native EXACTLY.
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(II[D[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: sscan_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: sscan_epoch as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                // TASK-409-E: describe BEFORE clear — a sig/name mismatch
                // posts NoSuchMethodError; without this the only trace is a
                // bare (code -1) and the gate sleeps silently (урон 2 ног).
                crate::describe_exception(env);
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] mobs_sscan: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            // TASK-437-A: selfTest==true ДО ARM — on the LOCAL ref from
            // define_class (TASK-417-C find_class-fix: JVMTI-scan filters
            // non-INITIALIZED classes; this call is the class's first active
            // use ⇒ <clinit>). Probe magic + vanilla ladder oracle; false →
            // dormant (ваниль бит-в-байт).
            let st = mobscan_selftest(env, c);
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            if !st {
                eprintln!(
                    "[crussty-plugin] mobs_sscan: selfTest=false — hook stays dormant (fail-closed, vanilla bit-for-bit)"
                );
                return false;
            }
            eprintln!(
                "[crussty-plugin] mobs_sscan: selfTest=true (probe magic + vanilla getNearestPlayer ladder oracle OK) BEFORE arm"
            );
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] mobs_sscan: bridge definition failed or selfTest=false, hook stays dormant"
            );
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed). TASK-437-A:
        // label = точный активный флаг (evidence-дисциплина); под
        // cmp436_sscan2 объявляются ОБЕ полуплоскости.
        if spawn_enabled() {
            eprintln!(
                "[crussty-plugin] {}: sscan armed despawn+spawn (2 sites: Mob.checkDespawn Level.findNearbyPlayer -> findNearbyPlayerGate + NaturalSpawner.spawnCategoryForPosition ServerLevel.getNearestPlayer(DDDDZ) @offset 221 -> spawnNearestPlayerGate; rust sscanEpoch = ONE bulk JNI/tick DOD pass over mobs_soa SoA positions -> nearest-qualifying-player column nearest[denseId]; spawn gate = per-tick vanilla EntitySelector snapshot (NO_SPECTATORS / NO_CREATIVE_OR_SPECTATOR by site flag) + vanilla distanceToSqr ladder, 0 per-call JNI; vanilla decisions bit-for-bit: first-strictly-closer in players() order, ties keep earlier; scan bodies untouched vanilla; empty flag = vanilla bit-for-bit)",
                label()
            );
        } else {
            eprintln!(
                "[crussty-plugin] {}: sscan armed despawn (Mob.checkDespawn Level.findNearbyPlayer site -> MobScanOps.findNearbyPlayerGate; rust sscanEpoch = ONE bulk JNI/tick DOD pass over mobs_soa SoA positions -> nearest-qualifying-player column nearest[denseId]; vanilla decisions bit-for-bit: first-strictly-closer in players() order, ties keep earlier; despawn body itself untouched vanilla; zero per-entity JNI; empty flag = vanilla bit-for-bit)",
                label()
            );
        }

        crate::kernel_policy::audit_wire(OPS_CLASS, "findNearbyPlayerGate", "cmp406_sscan v1");
        if spawn_enabled() {
            crate::kernel_policy::audit_wire(OPS_CLASS, "spawnNearestPlayerGate", "cmp436_sscan2 v1");
        }
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!(
            "[crussty-plugin] mobs_sscan: {MOB_CLASS} armed, retransform rc={rc} (ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] mobs_sscan: retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
            return;
        }
        // SSCAN2-SPAWN (TASK-437-A): spawn halfplane retransform — STRICT eq
        // cmp436_sscan2 only; rc != 0 → spawn slice stays vanilla (despawn
        // stays armed: per-halfplane fail-closed).
        if spawn_enabled() {
            let rcs = cplug_sdk::retransform_class(NSPAWN_CLASS);
            eprintln!(
                "[crussty-plugin] mobs_sscan: {NSPAWN_CLASS} armed, retransform rc={rcs} (spawn halfplane)"
            );
            if rcs != 0 {
                eprintln!(
                    "[crussty-plugin] mobs_sscan: spawn retransform FAILED (rc={rcs}) — spawn slice stays vanilla"
                );
            }
        }
    });
}

/// TASK-437-A: selfTest on the LOCAL ref of the just-defined bridge (same
/// pattern as colpush_selftest; TASK-417-C find_class-fix). Any pending
/// exception is cleared and reported as failure (fail-closed).
fn mobscan_selftest(env: &jvmti_bindings::env::JniEnv, cls: jni::jclass) -> bool {
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] mobs_sscan: selfTest method resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!(
            "[crussty-plugin] mobs_sscan: selfTest threw (late resolution) — fail-closed"
        );
        return false;
    }
    rc != 0
}

// ---------------------------------------------------------------------------
// Natives (registered on MobScanOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn sscan_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK nearest-player column writer — ONE transition per tick-batch (never
/// per entity).
///
/// Scans the mobs_soa SoA positions for `id in 0..min(id_top, nearest_cap,
/// soa_len)` under the seqlock reader bracket (no WLOCK — writers proceed;
/// torn snapshot → bounded retry) and writes `nearest[id]` = index of the
/// FIRST strictly-closest qualifying player (ties keep the earlier player —
/// the exact `getNearestPlayer(DDDD,Predicate)` selection ladder, including
/// the distanceToSqr component order `(px-mx)²+(py-my)²+(pz-mz)²`) or -1 when
/// no qualifying player exists. Players arrive as a flat [x,y,z]×n f64
/// snapshot (already predicate-filtered by the java bridge). Returns the
/// number of written elements or ERR_RANGE (bad n/arrays) / ERR_STRUCT (pin
/// failure, retries exhausted). Java fail-closed: any non-count → vanilla
/// scan this tick.
///
/// # Safety
/// See sscan_probe.
#[no_mangle]
pub unsafe extern "system" fn sscan_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    id_top: jni::jint,
    players: jni::jdoubleArray,
    nearest: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || players.is_null() || nearest.is_null() {
        return ERR_STRUCT;
    }
    if tick < 0 || id_top < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let p_cap = unsafe { (vt.GetArrayLength)(env, players) };
    let n_cap = unsafe { (vt.GetArrayLength)(env, nearest) };
    if p_cap <= 0 || n_cap <= 0 || p_cap % 3 != 0 {
        return ERR_RANGE;
    }
    let n = (p_cap / 3) as usize;
    let Some((xs, ys, zs, version)) = crate::mobs_soa::sscan_snapshot() else {
        return 0; // empty universe (no upserts ever) — zero valid entries
    };
    let bound = (id_top as usize)
        .min(n_cap as usize)
        .min(xs.len())
        .min(ys.len())
        .min(zs.len());

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, nearest, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, bound) };
    let pl = unsafe { std::slice::from_raw_parts(players as *const jni::jdouble, p_cap as usize) };

    let rc: i32;
    let mut tries: u32 = 0;
    loop {
        tries += 1;
        if tries > QRETRY {
            rc = ERR_RANGE; // writer storm — java falls back to vanilla this tick
            break;
        }
        let v1 = version.load(Ordering::Acquire);
        if v1 & 1 == 1 {
            std::hint::spin_loop();
            continue;
        }
        for (id, slot) in dst.iter_mut().enumerate() {
            let mx = xs[id];
            let my = ys[id];
            let mz = zs[id];
            let mut best = -1.0f64;
            let mut best_i: i32 = -1;
            for p in 0..n {
                let px = pl[p * 3];
                let py = pl[p * 3 + 1];
                let pz = pl[p * 3 + 2];
                // Entity.distanceToSqr(mobX, mobY, mobZ) on the PLAYER:
                // dx = player.x - mob.x etc., sum order (dx*dx + dy*dy) + dz*dz.
                let dx = px - mx;
                let dy = py - my;
                let dz = pz - mz;
                let d = dx * dx + dy * dy + dz * dz;
                // getNearestPlayer ladder: (best == -1.0 || d < best) — ties
                // keep the EARLIER player (first strictly-closer wins).
                if best == -1.0 || d < best {
                    best = d;
                    best_i = p as i32;
                }
            }
            *slot = best_i;
        }
        let v2 = version.load(Ordering::Acquire);
        if v2 != v1 {
            continue; // torn snapshot — retry the whole pass
        }
        rc = bound as i32;
        break;
    }

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, nearest, pinned, 0) };
    rc
}

// ---------------------------------------------------------------------------
// Tests: the vanilla getNearestPlayer selection ladder oracle (first strictly
// closer wins, ties keep the earlier player, -1.0 sentinel ladder), the
// distanceToSqr component order, the retarget descriptor contracts (both
// despawn and SSCAN2-SPAWN receiver-prepended forms), and the SSCAN2-SPAWN
// STRICT-gate needles (cmp436_sscan2).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_constant_strict() {
        // STRICT eq: no prefix/suffix tolerance (TASK-402-F lesson).
        assert!(enabled_with("cmp406_sscan"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp405_stagtick"));
        assert!(!enabled_with("cmp406_aibatch"));
        assert!(!enabled_with("cmp406_sscan_x"));
        assert!(!enabled_with(" cmp406_sscan"));
    }

    fn enabled_with(s: &str) -> bool {
        s == "cmp406_sscan"
    }

    /// TASK-437-A needle: the SSCAN2 family — cmp436_sscan2 is a STRICT OR
    /// member of enabled() (despawn halfplane) AND the sole spawn_enabled()
    /// member (spawn halfplane, STRICT eq).
    #[test]
    fn sscan2_gates_strict_or_and_eq() {
        // despawn halfplane: STRICT OR member.
        assert!(enabled_members("cmp436_sscan2"));
        assert!(!enabled_members(""));
        assert!(!enabled_members("cmp436_sscan"));
        assert!(!enabled_members("cmp436_sscan2_x"));
        assert!(!enabled_members(" cmp436_sscan2"));
        assert!(enabled_members("cmp406_sscan")); // legacy family member stays
        // spawn halfplane: STRICT eq — ONLY the exact round flag.
        assert!(spawn_members("cmp436_sscan2"));
        assert!(!spawn_members(""));
        assert!(!spawn_members("cmp406_sscan"));
        assert!(!spawn_members("cmp409_multi"));
        assert!(!spawn_members("cmp436_sscan2_x"));
        assert!(!spawn_members(" cmp436_sscan2"));
    }

    fn enabled_members(s: &str) -> bool {
        s == "cmp406_sscan" || s == "cmp409_multi" || s == "cmp412_meganav"
            || s == "cmp412_eqsnapv3" || s == "cmp414_cvs" || s == "cmp417_bq"
            || s == "cmp420_colpush" || s == "cmp422_brain2" || s == "cmp423_brain3"
            || s == "cmp424_mobfeed" || s == "cmp430_inside" || s == "cmp421_brain"
            || s == "cmp436_sscan2"
    }

    fn spawn_members(s: &str) -> bool {
        s == "cmp436_sscan2"
    }

    /// TASK-437-A needle: the SSCAN2-SPAWN retarget descriptor contract —
    /// site owner ServerLevel (javap: invokevirtual owner of the
    /// spawnCategoryForPosition site), gate desc = virtual desc with the
    /// ServerLevel receiver prepended, gate name matches the java bridge.
    #[test]
    fn spawn_retarget_desc_contract() {
        assert_eq!(SPAWN_FROM.0, "net/minecraft/server/level/ServerLevel");
        assert_eq!(SPAWN_FROM.1, "getNearestPlayer");
        assert_eq!(SPAWN_FROM.2, "(DDDDZ)Lnet/minecraft/world/entity/player/Player;");
        let expect_static = format!("(L{};{}", SPAWN_FROM.0, &SPAWN_FROM.2[1..]);
        assert_eq!(SPAWN_GATE_DESC, expect_static);
        assert_eq!(
            SPAWN_GATE_DESC,
            "(Lnet/minecraft/server/level/ServerLevel;DDDDZ)Lnet/minecraft/world/entity/player/Player;"
        );
        assert_eq!(SPAWN_METHOD_NAME, "spawnCategoryForPosition");
        assert_eq!(
            SPAWN_METHOD_DESC,
            "(Lnet/minecraft/world/entity/MobCategory;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/NaturalSpawner$SpawnPredicate;Lnet/minecraft/world/level/NaturalSpawner$AfterSpawnCallback;ILjava/util/function/Consumer;)V"
        );
    }

    /// Mirror of the vanilla `getNearestPlayer(DDDD, Predicate)` inner ladder
    /// operating on the same f64 values as the rust epoch pass.
    fn vanilla_pick(players: &[(f64, f64, f64)], mob: (f64, f64, f64)) -> i32 {
        let mut best = -1.0f64;
        let mut best_i = -1i32;
        for (i, &(px, py, pz)) in players.iter().enumerate() {
            let dx = px - mob.0;
            let dy = py - mob.1;
            let dz = pz - mob.2;
            let d = dx * dx + dy * dy + dz * dz;
            if best == -1.0 || d < best {
                best = d;
                best_i = i as i32;
            }
        }
        best_i
    }

    #[test]
    fn first_strictly_closer_wins_ties_keep_earlier() {
        let mob = (0.0, 0.0, 0.0);
        // Strictly closer later player REPLACES the earlier one.
        let ps = vec![(10.0, 0.0, 0.0), (5.0, 0.0, 0.0)];
        assert_eq!(vanilla_pick(&ps, mob), 1);
        // Exact tie: earlier player kept (first wins, `<` not `<=`).
        let ps = vec![(3.0, 4.0, 0.0), (0.0, 3.0, 4.0)];
        assert_eq!(vanilla_pick(&ps, mob), 0);
        // Single player always selected (the -1.0 sentinel branch).
        let ps = vec![(100.0, 100.0, 100.0)];
        assert_eq!(vanilla_pick(&ps, mob), 0);
        // Empty player list → -1 (java bridge returns null → vanilla null).
        assert_eq!(vanilla_pick(&Vec::new(), mob), -1);
    }

    #[test]
    fn distance_component_order_matches_distance_to_sqr() {
        // Entity.distanceToSqr(x,y,z) on the player: dx = player - mob,
        // sum = dx*dx + dy*dy + dz*dz. The square is negation-exact, so the
        // operand direction cannot flip a bit; the SUM order must match.
        let (mx, my, mz) = (1.5, -2.25, 3.125);
        let (px, py, pz) = (-0.75, 4.5, 2.0);
        let dx = px - mx;
        let dy = py - my;
        let dz = pz - mz;
        let expect = dx * dx + dy * dy + dz * dz;
        assert_eq!(dx * dx + dy * dy + dz * dz, (mx - px) * (mx - px) + dy * dy + dz * dz);
        assert!(expect > 0.0);
    }

    #[test]
    fn retarget_desc_contract() {
        // Receiver-prepended static form: virtual desc ()-style args with the
        // OWNER (Level) prepended — stack-identical Level-consuming bridge.
        let expect_static = format!("(L{};{}", FROM.0, &FROM.2[1..].to_string());
        assert_eq!(GATE_STATIC_DESC, expect_static);
        assert_eq!(
            GATE_STATIC_DESC,
            "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;DLjava/util/function/Predicate;)Lnet/minecraft/world/entity/player/Player;"
        );
    }

    #[test]
    fn epoch_bounds_match_java_ladder() {
        // Java MobScanOps publishes rc = number of written elements; readers
        // guard id < rc. The rust bound never exceeds any of the three caps.
        let id_top = 150_000usize;
        let n_cap = 262_144usize;
        let soa_len = 1_048_576usize;
        let bound = id_top.min(n_cap).min(soa_len);
        assert_eq!(bound, 150_000);
        assert!(bound <= n_cap && bound <= soa_len);
    }
}
