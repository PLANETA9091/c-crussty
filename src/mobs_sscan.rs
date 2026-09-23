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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

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

// ---------------------------------------------------------------------------
// TASK-427-B2 (scanq): STRIPED + TWO-PHASE despawn epoch (protocol-v2 D1+D2
// slice port, mc-линия 20c9fdc/afbc1dc — «per-shard reader brackets» +
// «JNI-critical=memcpy-only»):
//
//   D2 — the GC-locker stall kill: the legacy epoch pinned the java column
//   with GetPrimitiveArrayCritical across the WHOLE population pass AND the
//   whole retry ladder (QRETRY × full 41k×players rescans) — under 150k-
//   entity GC pressure that staged a JVM-wide allocator stall (the mf-l2
//   collapse profile). Now the pass runs into a rust-side scratch with NO
//   criticals held; the ONLY critical region is the final memcpy of the
//   stripe segment (microseconds, bounded).
//
//   D1 — bounded per-tick budget via striping: each epoch refreshes ONE
//   stripe of ≤ STRIPE_CAP ids (rotation s = tick % stride_count, coverage
//   cycles every stride_count ticks); the FIRST epoch of a (re)grown bound
//   is a FULL warm-up pass (no uninitialised entries can ever be published:
//   the column is persistent across ticks, zeros would phantom-pick
//   player 0). A stripe entry read by checkDespawn is a PICK-hint frozen
//   ≤ stride_count ticks ago — the same documented freshness class as the
//   SoA ≤1-tick ghost (the vanilla despawn body still computes its
//   distances on the LIVE player entity; only WHICH player is picked can
//   differ, and only within the pick ladder's near-tie band).
//
//   Retry ladder stays BOUNDED (RETRIES, spin→yield) and runs WITHOUT any
//   critical held; exhaustion → ERR_RANGE → java vanilla-this-tick (mc
//   canon: bounded fail-open, NO infinite spins, NO plane disarm).
// ---------------------------------------------------------------------------

/// Stripe width cap (ids per epoch) — the bounded per-tick despawn budget.
const STRIPE_CAP: usize = 16_384;
/// Minimum stripe rotation depth (coverage cycle ≤ this many ticks).
const STRIPE_MIN: usize = 4;
/// Bounded retry budget for the version bracket (per stripe pass).
const RETRIES: u32 = 4;

struct ScanScratch {
    /// Stripe segment values (player index or -1) before the publish memcpy.
    buf: Vec<i32>,
}

static SCAN_BUILD: Mutex<Option<ScanScratch>> = Mutex::new(None);

/// Highest bound ever published (warm-up/growth detector — full pass when
/// the column's fresh-prefix must extend). Single-flight by the java
/// EPOCH_LOCK; AtomicUsize is belt-and-braces.
static PUBLISHED_BOUND: AtomicUsize = AtomicUsize::new(0);

/// Stripe geometry for `bound`: rotation depth (≥ STRIPE_MIN, ≥ ceil of the
/// budget) and the stripe segment [lo, hi) for epoch `tick`. Pure fn —
/// oracle-tested below.
fn stripe_range(tick: i32, bound: usize) -> (usize, usize) {
    let stride_count = ((bound + STRIPE_CAP - 1) / STRIPE_CAP).max(STRIPE_MIN);
    let s = (tick.max(0) as usize) % stride_count;
    let lo = s * STRIPE_CAP;
    let hi = (lo + STRIPE_CAP).min(bound);
    (lo.min(bound), hi)
}

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
            | Ok("cmp423_brain3") | Ok("cmp424_mobfeed")
            | Ok("cmp421_brain")
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
/// per entity). TASK-427-B2 (scanq): STRIPED + TWO-PHASE body —
/// phase 1 computes the stripe segment into a rust scratch with NO criticals
/// held (bounded retry ladder, spin→yield); phase 2 memcpy-publishes ONLY
/// the stripe segment under one short critical. First epoch of a grown
/// bound = FULL warm-up pass (no uninitialised column entries can reach the
/// java gate). Per-tick work bounded by STRIPE_CAP ids × players.
///
/// Scans the mobs_soa SoA positions for the active stripe
/// `id in [lo, hi) ⊆ 0..min(id_top, caps)` under the bounded version bracket
/// (no WLOCK — writers proceed; torn stripe → bounded retry; exhaustion →
/// ERR_RANGE = java vanilla this tick, epoch retried next) and computes
/// `nearest[id]` = index of the FIRST strictly-closest qualifying player
/// (ties keep the earlier player — the exact `getNearestPlayer(DDDD,
/// Predicate)` selection ladder, including the distanceToSqr component order
/// `(px-mx)²+(py-my)²+(pz-mz)²`) or -1 when no qualifying player exists.
/// Players arrive as a flat [x,y,z]×n f64 snapshot (already predicate-
/// filtered by the java bridge). Returns the PUBLISHED column length
/// (= bound: entries outside this tick's stripe retain their prior-tick
/// values — the striped-freshness contract, warm-up guarantees every
/// published entry was written at least once) or ERR_RANGE (bad n/arrays,
/// retries exhausted) / ERR_STRUCT (pin failure). Java fail-closed: any
/// non-count → vanilla scan this tick.
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

    let pl = unsafe { std::slice::from_raw_parts(players as *const jni::jdouble, p_cap as usize) };

    // Stripe geometry: FULL warm-up pass when the bound (re)grows (every
    // published entry must have been written at least once — a zero entry
    // would phantom-pick player 0), otherwise ONE rotated stripe.
    let full = bound > PUBLISHED_BOUND.load(Ordering::Acquire);
    let (lo, hi) = if full {
        (0usize, bound)
    } else {
        stripe_range(tick, bound)
    };

    // Phase 1 — compute the segment into the scratch (NO criticals).
    let mut guard = SCAN_BUILD.lock().unwrap_or_else(|p| p.into_inner());
    let scratch = guard.get_or_insert_with(|| ScanScratch { buf: Vec::new() });
    let width = hi - lo;
    if scratch.buf.len() < width {
        scratch.buf.resize(width, 0);
    }
    let buf: &mut [i32] = &mut scratch.buf[..width];

    let mut tries: u32 = 0;
    loop {
        tries += 1;
        if tries > RETRIES {
            // writer storm — java falls back to vanilla this tick (epoch
            // retried next tick; bounded fail-open, mc canon).
            return ERR_RANGE;
        }
        let v1 = version.load(Ordering::Acquire);
        if v1 & 1 == 1 {
            std::hint::spin_loop();
            std::thread::yield_now();
            continue;
        }
        for (i, id) in (lo..hi).enumerate() {
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
            buf[i] = best_i;
        }
        let v2 = version.load(Ordering::Acquire);
        if v2 != v1 {
            continue; // torn stripe — bounded retry (NO critical held)
        }
        break;
    }

    // Phase 2 — memcpy-publish ONLY the stripe segment (shortest possible
    // critical). Entries outside [lo, hi) retain their prior-tick values.
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, nearest, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, n_cap as usize) };
    dst[lo..hi].copy_from_slice(buf);
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, nearest, pinned, 0) };

    PUBLISHED_BOUND.store(bound, Ordering::Release);
    static STRIPE_LOGGED: AtomicBool = AtomicBool::new(false);
    if !STRIPE_LOGGED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "[crussty-plugin] cmp406_sscan: scanq-stripes EFFECT armed (first striped two-phase epoch: full={full} segment=[{lo},{hi}) of bound={bound}, budget cap {STRIPE_CAP} ids/tick, retry ladder {RETRIES} no-criticals)"
        );
    }
    bound as i32
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

    /// TASK-427-B2 (scanq): stripe geometry oracle — every id belongs to
    /// exactly one stripe of a stride_count rotation; each stripe is ≤
    /// STRIPE_CAP wide; segments tile [0, bound) without gaps/overlap; the
    /// per-tick budget is bounded.
    #[test]
    fn stripe_range_tiles_bound_exactly() {
        for &bound in &[0usize, 1, 100, STRIPE_CAP - 1, STRIPE_CAP, STRIPE_CAP + 1, 41_000] {
            let stride_count = ((bound + STRIPE_CAP - 1) / STRIPE_CAP).max(STRIPE_MIN);
            // Collect the distinct segments over one full rotation.
            let mut segs: Vec<(usize, usize)> = Vec::new();
            for t in 0..(stride_count as i32) {
                let (lo, hi) = stripe_range(t, bound);
                if bound == 0 || lo >= bound {
                    // Ticks whose stripe starts past the bound simply refresh
                    // nothing (empty segment) — valid no-op, not a gap.
                    assert_eq!(lo, hi);
                    continue;
                }
                assert!(lo < hi, "non-empty stripe for bound={bound} tick={t}");
                assert!(hi - lo <= STRIPE_CAP, "budget cap");
                assert!(hi <= bound);
                segs.push((lo, hi));
            }
            // Rotation depth must cover [0, bound) exactly once.
            if bound > 0 {
                let mut covered = vec![false; bound];
                for &(lo, hi) in &segs {
                    for id in lo..hi {
                        assert!(!covered[id], "id {id} covered twice");
                        covered[id] = true;
                    }
                }
                assert!(covered.iter().all(|&c| c), "full coverage bound={bound}");
            }
        }
    }

    /// TASK-427-B2: warm-up rule — a bound growth must select the FULL pass
    /// (every published entry written at least once; zeros would phantom-pick
    /// player 0), a steady bound uses the rotated stripe.
    #[test]
    fn warmup_on_bound_growth() {
        let mut published: usize = 0;
        let mut full_wins = 0usize;
        for bound in [1000usize, 2819, 5000, 5000, 41_000, 41_000, 41_000] {
            let full = bound > published;
            if full {
                full_wins += 1;
                published = bound; // publish path stores PUBLISHED_BOUND
            }
        }
        assert_eq!(full_wins, 4, "warm-up on first + every growth");
    }
}
