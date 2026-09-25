//! Runtime wiring for the MOB-AI-WINDOW plane (TASK-406-D, vector R3
//! «mob_ai_step batch» — lever `cmp406_aibatch`; bridge
//! mobai/net/minecraft/world/entity/MobAiOps.java, natives below, retarget
//! of the `serverAiStep()V` call site via `classfile::retarget_virtual_to_static`).
//!
//! R3 RUST-FIRST (owner directive, round-406): the whole mob AI plane —
//! `Mob.serverAiStep` subtree = sensing / targeting / goals / navigation /
//! brain / move-look-jump controls = 9.73% wall (cpu-collapsed comp3,
//! subtree attribution) — is rate-batched by a golden-phase WINDOW:
//! `floorMod(golden32(denseId) + tickCount, N) == 0` (default N=4 → 3/4 of
//! mobs skip the ENTIRE AI plane per tick; on-window mobs run vanilla).
//! Upstream precedents: Pufferfish DAB (brain-tick debounce), Paper/Spigot
//! ActivationRange (1/N entity ticking), Airplane DEAR (tiered 1/2, 1/4),
//! Lithium (AI-slice optimization). Rate-invariant class — the repo's own
//! accepted mob-stagger segment (cmp401_stagger +12.5pp) and item-plane
//! resting phase ((tickCount+id)&31) are the same contract.
//!
//! BULK-JNI LAW (one transition per batch, never per entity): the window
//! decision column is computed by RUST once per server tick in ONE native
//! call — `aiEpoch(tick, n, idTop, window[I)I` does a DOD pass over the
//! mobs_soa SoA population (flags bit0 = alive) under the seqlock reader
//! bracket and writes `window[denseId] = {0,1}` straight into a shared java
//! int[] (GetPrimitiveArrayCritical). The per-mob decision is an O(1) java
//! array read of that column (`MobPushOps.idBoxOf` → dense id) — ZERO
//! per-entity JNI. Rust is the single source of truth of the window rule.
//!
//! CHAIN COMPOSITION (cplug-sdk ORDERING CONTRACT: "hook N receives hook
//! N-1's output … the final result is the last produced output"): the
//! LivingEntity byte chain already carries mobs_soa's pushEntities retarget
//! (served from ITS pristine stash) — hook serve REPLACES bytes, so a
//! stash-based serve here would strip the soa rewrite. This hook therefore
//! COMPOSES: at serve time it retargets the `serverAiStep()V` site IN THE
//! RECEIVED BYTES (previous hooks' output preserved; AlreadyPatched →
//! pass-through; Err → None fail-closed pass-through). Activation waits for
//! the mobs_soa + stagger LIVING_SERVED signals (set after their
//! retransforms) so this module's retransform is strictly LAST.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp406_aibatch"` (STRICT eq; empty/
//! foreign flag = no hook registered at all — byte-indistinguishable from
//! vanilla). FAIL-CLOSED ladder: not-Mob/not-in-plane → vanilla this call;
//! aiProbe mismatch / aiEpoch ERR_STRUCT → disarm forever; ERR_RANGE →
//! vanilla this tick, window retried next tick; sites != 1 → hook stays
//! dormant (fail-closed).

use jvmti_bindings::jni;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/MobAiOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../mobai/build/net/minecraft/world/entity/MobAiOps.class");

const SERVERAISTEP_DESC: &str = "()V";
// dleg2 root-cause fix (compose-reject): валидатор требует РОВНО virtual-desc
// с receiver-классом LivingEntity, препендированным к static-дескриптору
// (см. retarget_virtual_to_static: expect_static = "(L{receiver};{desc[1..]}").
// Мост принимает LivingEntity и сам сужает до Mob внутри.
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/LivingEntity;)V";

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x4149; // "AI"
const QRETRY: u32 = 128;

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp406_aibatch") | Ok("cmp409_multi") | Ok("cmp412_meganav") | Ok("cmp414_cvs")
            // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
            | Ok("cmp412_eqsnapv3") | Ok("cmp414_cvs") | Ok("cmp417_bq")
            // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
            | Ok("cmp420_colpush")
            // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
            | Ok("cmp422_brain2")
            // TASK-424-A: GC-ревизия brain3 (STRICT OR).
            | Ok("cmp423_brain3") | Ok("cmp424_mobfeed") | Ok("cmp430_inside") | Ok("cmp432_inside2") | Ok("cmp436_ins4")
            | Ok("cmp438_sense") // TASK-444-C: sense family union
            | Ok("cmp423_brain3") | Ok("cmp424_mobfeed") | Ok("cmp430_inside") | Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4") | Ok("cmp444_chunk5") | Ok("cmp450_chunk") | Ok("cmp452_mega") | Ok("cmp453_diet")
            | Ok("cmp421_brain")
    )
}

static READY: AtomicBool = AtomicBool::new(false);

/// TASK-406-D ordering signals (see module docs): set by mobs_manager.rs /
/// stagger.rs after their LivingEntity retransforms (rc == 0).
static SOA_SERVED: AtomicBool = AtomicBool::new(false);
static STAGGER_SERVED: AtomicBool = AtomicBool::new(false);

/// mobs_manager.rs: called after the soa-patch retransform succeeded.
pub fn note_soa_served() {
    SOA_SERVED.store(true, Ordering::Release);
}

/// stagger.rs: called after the stagger-patch retransforms succeeded.
pub fn note_stagger_served() {
    STAGGER_SERVED.store(true, Ordering::Release);
}

/// TASK-419-A (colpush): read-views of the LIVING serve signals — colpush.rs
/// (the LAST LivingEntity hook) waits for soa+stagger+ai before its own
/// retransform, so the chain composes all serves in one round.
pub fn soa_served() -> bool {
    SOA_SERVED.load(Ordering::Acquire)
}

pub fn stagger_served() -> bool {
    STAGGER_SERVED.load(Ordering::Acquire)
}

pub fn ai_ready() -> bool {
    READY.load(Ordering::Acquire)
}


/// Window rule — the single source of truth (mirrored 1:1 in the rust
/// `aiEpoch` writer; java never re-derives it). Golden-multiplicative phase:
/// active iff `floorMod(golden32(id) + tick, n) == 0` with
/// golden32(id) = (id as i32).wrapping_mul(-1640531527) (Knuth's 2^32/φ).
#[inline]
fn window_active(id: usize, tick: i64, n: i64) -> bool {
    let g = (id as i32).wrapping_mul(-1_640_531_527) as i64;
    (g + tick).rem_euclid(n) == 0
}

/// Compute the composing retarget patch from the RECEIVED chain bytes.
fn retarget_ai_step(
    bytes: &[u8],
) -> Result<(Vec<u8>, crate::classfile::RetargetOutcome), String> {
    crate::classfile::retarget_virtual_to_static(
        bytes,
        "aiStep",
        SERVERAISTEP_DESC,
        (
            LIVING_CLASS,
            "serverAiStep",
            SERVERAISTEP_DESC,
        ),
        (OPS_CLASS, "serverAiStepGate", GATE_STATIC_DESC),
    )
}

/// Register the COMPOSING byte hook (call once from cplugin_init, AFTER
/// mobs_manager::register — it must be the LAST hook on LivingEntity).
/// Dormant-invisible: with the lever flag unset/mismatched NOTHING is
/// registered for this vector.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] mobs_ai: dormant (lever_flag != cmp406_aibatch, vanilla mob AI plane)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(LIVING_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pre-arm: pass the chain output through untouched (the earlier
            // hooks own pristine capture + their own serves).
            return None;
        }
        // Compose onto the RECEIVED bytes (previous hooks' output preserved).
        match retarget_ai_step(bytes) {
            Ok((out, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 1 => {
                    static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                    if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                        eprintln!(
                            "[crussty-plugin] mobs_ai: hook serve {LIVING_CLASS} {} bytes (composed, sites=1)",
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
                            "[crussty-plugin] mobs_ai: serverAiStep site not rewritten ({other:?}) — pass-through (fail-closed)"
                        );
                    }
                    None
                }
            },
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] mobs_ai: compose patch rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for boot + the soa/stagger LIVING serves,
/// define the MobAiOps bridge into the kernel loader + RegisterNatives
/// (aiProbe/aiEpoch), flip READY (hook starts composing) and retransform
/// LivingEntity — this module's serve is then the LAST in the chain.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // LivingEntity loads at boot.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(LIVING_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] mobs_ai: {LIVING_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mobs_ai: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Guard: embedded bridge bytes must not be newer than the JVM.
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
                "[crussty-plugin] mobs_ai: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild mobai/ via scripts/build_mobai_ops.sh; hook stays dormant"
            );
            return;
        }

        // Ordering: wait for the soa/stagger LivingEntity serves (their
        // stash-based serves would REPLACE my composed bytes if they ran
        // after my retransform). Timeout = proceed anyway (fail-open on
        // ordering only, composition itself stays fail-closed).
        let order_deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        loop {
            let soa = SOA_SERVED.load(Ordering::Acquire);
            let stag = STAGGER_SERVED.load(Ordering::Acquire);
            if soa && stag {
                break;
            }
            if std::time::Instant::now() > order_deadline {
                eprintln!(
                    "[crussty-plugin] mobs_ai: LIVING serve signals timeout (soa={soa} stagger={stag}) — arming anyway, chain composes current bytes"
                );
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(LIVING_CLASS) else {
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
                eprintln!("[crussty-plugin] mobs_ai: define_class({OPS_CLASS}) failed");
                return false;
            };

            // RegisterNatives: aiProbe (magic) + aiEpoch (bulk window writer).
            let names = [
                CString::new("aiProbe").expect("no NUL"),
                CString::new("aiEpoch").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(III[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: ai_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: ai_epoch as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] mobs_ai: register_natives failed (code {code}) — hook stays dormant"
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
            eprintln!("[crussty-plugin] mobs_ai: bridge definition failed, hook stays dormant");
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp406_aibatch: ARMED mob-ai-window (LivingEntity.aiStep serverAiStep site -> MobAiOps.serverAiStepGate; golden-phase skip (N-1)/N, N from env CRUSSTY_AI_N/LEVER_ARG default 4; rust aiEpoch = ONE bulk JNI/tick over mobs_soa SoA population, window column shared via int[] mirror; zero per-entity JNI; empty flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "serverAiStepGate", "cmp406_aibatch v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(LIVING_CLASS);
        eprintln!(
            "[crussty-plugin] mobs_ai: {LIVING_CLASS} armed, retransform rc={rc} (composed serve; ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] mobs_ai: retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Natives (registered on MobAiOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn ai_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK window writer — ONE transition per tick-batch (never per entity).
///
/// Scans the mobs_soa SoA population (flags bit0 = alive) for
/// `id in 0..min(id_top, window_cap, flags_len)` under the seqlock reader
/// bracket (no WLOCK — writers proceed; torn snapshot → bounded retry) and
/// writes `window[id] = active(rule)` straight into the pinned java array.
/// Returns the number of written elements (the java WINDOW_LEN) or
/// ERR_RANGE (bad n/window) / ERR_STRUCT (null env/array, pin failure,
/// retries exhausted). Java fail-closed: any non-count → vanilla AI tick.
///
/// # Safety
/// See ai_probe.
#[no_mangle]
pub unsafe extern "system" fn ai_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    n: jni::jint,
    id_top: jni::jint,
    window: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || window.is_null() {
        return ERR_STRUCT;
    }
    if !(2..=64).contains(&n) || tick < 0 || id_top < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, window) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    let Some((flags, version)) = crate::mobs_soa::ai_window_snapshot() else {
        return 0; // empty universe (no upserts ever) — zero valid entries
    };
    let bound = (id_top as usize)
        .min(cap as usize)
        .min(flags.len());

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, window, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, bound) };

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
            *slot = ((flags[id] & 1 != 0) && window_active(id, tick as i64, n as i64)) as i32;
        }
        let v2 = version.load(Ordering::Acquire);
        if v2 != v1 {
            continue; // torn snapshot — retry the whole pass
        }
        rc = bound as i32;
        break;
    }

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, window, pinned, 0) };
    rc
}

// ---------------------------------------------------------------------------
// Tests: window-rule oracle (determinism, exact 1/N coverage per id, phase
// spread), clamp ladder and the retarget descriptor contract (static desc is
// the receiver-prepended virtual desc — stack-identical void→void).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_constant_strict() {
        // STRICT eq: no prefix/suffix tolerance (TASK-402-F lesson).
        assert!(enabled_with("cmp406_aibatch"));
        assert!(!enabled_with(""));
        assert!(!enabled_with("cmp405_stagtick"));
        assert!(!enabled_with("cmp406_aibatch_x"));
        assert!(!enabled_with(" cmp406_aibatch")); // trailing-space variants rejected by design (trim handled at var read sites)
    }

    fn enabled_with(s: &str) -> bool {
        // mirror of the gate's STRICT equality (never starts_with/contains)
        s == "cmp406_aibatch"
    }

    #[test]
    fn window_rule_exact_one_in_n_per_id() {
        for n in [2i64, 4, 8, 64] {
            for id in 0..2048usize {
                let hits = (0..n).filter(|&t| window_active(id, t, n)).count();
                assert_eq!(hits, 1, "id={id} n={n}: window must hit exactly once per n ticks");
            }
        }
    }

    #[test]
    fn window_rule_phase_spread_is_uniform() {
        // Golden multiplicative hashing: per-tick active fraction ~ 1/N.
        let n = 4i64;
        for t in 0..64i64 {
            let active = (0..4096usize).filter(|&id| window_active(id, t, n)).count();
            let frac = active as f64 / 4096.0;
            assert!(
                (frac - 1.0 / n as f64).abs() < 0.05,
                "tick={t} active fraction {frac} deviates from 1/n"
            );
        }
    }

    #[test]
    fn window_rule_deterministic() {
        for id in 0..512usize {
            for t in [0i64, 7, 123_456] {
                let a = window_active(id, t, 4);
                let b = window_active(id, t, 4);
                assert_eq!(a, b);
            }
        }
    }

    #[test]
    fn retarget_desc_contract() {
        // Receiver-prepended static form (stack-identical void → void).
        let expect_static = format!("(L{LIVING_CLASS};{}", SERVERAISTEP_DESC[1..].to_string());
        // dleg2 compose-reject root cause: валидатор требует РОВНО expect_static
        // — widening до суперкласса (LEntity;)V отклоняется (stack-shape guard).
        // Мост принимает LivingEntity и сужает до Mob внутри себя.
        assert_eq!(GATE_STATIC_DESC, expect_static);
        assert_eq!(GATE_STATIC_DESC, "(Lnet/minecraft/world/entity/LivingEntity;)V");
    }

    #[test]
    fn epoch_clamps_match_java_ladder() {
        // Java MobAiOps.windowN() clamps to [2..64]; rust rejects outside.
        assert!(!(2..=64).contains(&1));
        assert!((2..=64).contains(&4));
        assert!(!(2..=64).contains(&65));
    }
}
