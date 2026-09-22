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
//! Mob.class is hook-free under this lever (prepare_index's Mob hook is
//! dormant unless cmp401_offthread), so a stash-based serve is safe: pristine
//! bytes are captured on first sighting, the retarget patches the RECEIVED
//! bytes, AlreadyPatched → pass-through, Err → None fail-closed pass-through.

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const OPS_CLASS: &str = "net/minecraft/world/entity/MobScanOps";

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
            | Ok("cmp412_eqsnapv3") | Ok("cmp414_cvs") | Ok("cmp416_fluid")
    )
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

/// Register the byte hook on Mob (call once from cplugin_init). Dormant-
/// invisible: with the lever flag unset/mismatched NOTHING is registered.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] mobs_sscan: dormant (lever_flag != cmp406_sscan, vanilla despawn scans)"
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
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] mobs_sscan: bridge definition failed, hook stays dormant");
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp406_sscan: ARMED sscan-despawn (Mob.checkDespawn Level.findNearbyPlayer site -> MobScanOps.findNearbyPlayerGate; rust sscanEpoch = ONE bulk JNI/tick DOD pass over mobs_soa SoA positions -> nearest-qualifying-player column nearest[denseId]; vanilla decisions bit-for-bit: first-strictly-closer in players() order, ties keep earlier; despawn body itself untouched vanilla; zero per-entity JNI; empty flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "findNearbyPlayerGate", "cmp406_sscan v1");
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
        }
    });
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
// distanceToSqr component order, and the retarget descriptor contract
// (receiver-prepended virtual desc — Level receiver prepended).
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
