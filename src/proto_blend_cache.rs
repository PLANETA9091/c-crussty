//! Runtime wiring for the `blend_cache` hook — B10 PROTOTYPE (gate OFF).
//!
//! Target: the NoiseChunk per-column blend-offset/alpha cache — the path
//! behind P500's biggest measured win (NoiseChunkBlendCache 244x,
//! oldEmptyBlenderSummary 67µs vs newEmptyBlenderSummary 274.5ns, see
//! docs/HOOK_BLEND_CACHE.md and bench/p500/results/P500_REPORT.md §21).
//!
//! Unlike area_map/improved_noise this module is **observation-only by
//! construction**: a wrong patch here changes terrain generation, which is
//! FORBIDDEN (project rule: no gameplay changes). Therefore:
//!
//! 1. Gate: env `CRUSSTY_NATIVE_BLEND_CACHE` (1/true/on/yes → on), OFF by
//!    default. Gate OFF ⇒ byte hook NOT registered, activate() a no-op
//!    (same dormant-discipline as improved_noise after fix 3a270ee).
//! 2. Even gate ON, the byte hook only CAPTURES pristine NoiseChunk bytes
//!    (`PATCH_ENABLED = false` ⇒ it can never serve a patch — the classfile
//!    patcher and ops-bridge classes do not exist yet).
//! 3. The activation worker runs the read-only parity self-test (10k random
//!    samples, old vs new summary kernels through the real injected P500
//!    bridge) and a patch-point probe; results go to the log. A patch would
//!    only ever be served after docs/HOOK_BLEND_CACHE.md §3.4's checklist
//!    passes (TODO(B10 Phase 2/3) blocks below).
//!
//! Self-test design: call BOTH `oldEmptyBlenderSummary` and
//! `newEmptyBlenderSummary` (found by name from src/jni_table.rs rows
//! 159–160, sig `(II[J)I`) over 10k random inputs with FRESH dst arrays per
//! call (kernels may mutate inputs), compare return value + all written
//! longs BIT-EXACT; zero mismatches allowed (docs §3.5 for the tolerance
//! rules and the future double-parity rules).
//!
//! Deliberate duplication of helpers (wait_for_boot / class_version /
//! force-load pattern) from src/improved_noise.rs and src/area_map.rs: this
//! module must compile IN ISOLATION (no `crate::` references) while the
//! main agent wires it, and while other agents' edits land in lib.rs.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Kernel class owning the per-column blend cache (Mojang-mapped internal
/// name; Paper 1.21.x is Mojang-mapped). TODO(B10 Phase 2): confirm exact
/// name/method surface with `javap -p -c` against the running kernel before
/// any patch work (docs/HOOK_BLEND_CACHE.md §3.3/§3.4).
pub const NOISE_CHUNK_CLASS: &str = "net/minecraft/world/level/levelgen/NoiseChunk";
/// Neighbor classes for the Phase-2 probe (existence check only, no patch).
pub const BLENDER_CLASS: &str = "net/minecraft/world/level/levelgen/blending/Blender";
pub const BLENDING_DATA_CLASS: &str =
    "net/minecraft/world/level/levelgen/blending/BlendingData";

/// P500 bench bridge class — BOOTSTRAP loader, already defined +
/// RegisterNatives'd by lib.rs inject_surface (found by name from
/// src/jni_table.rs: MAIN_JNI_TABLE rows for PaperNativeNoiseChunkBlendCache).
const NATIVE_BRIDGE: &str = "PaperNativeNoiseChunkBlendCache";
const OLD_SUMMARY: &str = "oldEmptyBlenderSummary";
const NEW_SUMMARY: &str = "newEmptyBlenderSummary";
const SUMMARY_SIG: &str = "(II[J)I";

/// NEW env gate for this hook (OFF by default — do not "fix" this).
pub const GATE_ENV: &str = "CRUSSTY_NATIVE_BLEND_CACHE";

/// Parity harness parameters (docs/HOOK_BLEND_CACHE.md §3.5).
const PARITY_SAMPLES: usize = 10_000;
/// Yield between batches so a boot-time worker never starves a core.
const PARITY_BATCH: usize = 500;
/// dst array size; G21 used long[64] and P500 kernels may write up to that.
const DST_LEN: i32 = 64;
/// Report the first K parity offenders with full inputs.
const MAX_REPORTED_FAILURES: usize = 5;

/// HARD OFF in this prototype. Even with the env gate set, no patched
/// bytecode is ever served until the Phase-2/3 pipeline (ops bridge classes,
/// classfile patcher, §3.4 checklist) exists. Flipping this bool alone does
/// NOT enable a patch: the serve branch below must be implemented first.
const PATCH_ENABLED: bool = false;

/// Candidate patch points to PROBE (existence check only). Names are the
/// Mojang-mapping guesses from docs/HOOK_BLEND_CACHE.md §3.3 — a candidate
/// that does not resolve on the live class means "redesign", never "force".
const CANDIDATE_PATCH_METHODS: &[(&str, &str)] = &[
    (
        "blendOffset",
        "(Lnet/minecraft/world/level/levelgen/DensityFunction$FunctionContext;)D",
    ),
    (
        "blendAlpha",
        "(Lnet/minecraft/world/level/levelgen/DensityFunction$FunctionContext;)D",
    ),
];

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

static READY: AtomicBool = AtomicBool::new(false);
/// Set once the parity self-test passed (10k/10k, bit-exact). A failed run
/// is FINAL for this process (no retry, hook stays dormant).
static PARITY_OK: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// Pristine NoiseChunk bytes captured by the byte hook (observation-only in
/// this prototype; consumed by the Phase-3 patch computation).
static ORIG_BYTES: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();
static PATCH_CACHE: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();

fn orig_lock() -> &'static Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| Mutex::new(None))
}
#[allow(dead_code)]
fn patch_lock() -> &'static Mutex<Option<Vec<u8>>> {
    PATCH_CACHE.get_or_init(|| Mutex::new(None))
}

/// Wiring diagnostics for the main agent's status logs.
/// Introspection for operators/tests: parity verdict of the last self-test.
#[allow(dead_code)]
pub fn parity_passed() -> bool {
    PARITY_OK.load(Ordering::Relaxed)
}

/// True once a Phase-3 patch has actually been served (always false in this
/// prototype — PATCH_ENABLED=false).
/// Introspection: has the (gated) patch actually been served.
#[allow(dead_code)]
pub fn patched() -> bool {
    PATCHED.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Gate
// ---------------------------------------------------------------------------

/// env-gate (off by default), read at register/activate time.
/// Same accepted spellings as improved_noise.
pub fn enabled() -> bool {
    std::env::var(GATE_ENV)
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Class-file version of `b` as (major, minor), or None if not a class file.
/// (Copy of improved_noise::class_version — kept local for isolation.)
fn class_version(b: &[u8]) -> Option<(u16, u16)> {
    if b.len() < 8 || u32::from_be_bytes(b[0..4].try_into().ok()?) != 0xCAFE_BABE {
        return None;
    }
    Some((
        u16::from_be_bytes([b[6], b[7]]),
        u16::from_be_bytes([b[4], b[5]]),
    ))
}

// ---------------------------------------------------------------------------
// register() — byte hook (gate OFF ⇒ nothing happens)
// ---------------------------------------------------------------------------

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// Gate OFF: log dormant and DO NOT register anything (the dormant-gate-leak
/// lesson from improved_noise 3a270ee — a registered-but-passive hook hides
/// activation bugs and surprises log readers).
///
/// Gate ON: the hook is observation-only. `PATCH_ENABLED=false` ⇒ it NEVER
/// returns replacement bytes; it only captures the pristine NoiseChunk bytes
/// for the Phase-3 patch computation and logs the sighting.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] blend_cache: dormant (prototype; set {GATE_ENV}=1 to run the read-only parity harness — no patch ships)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(NOISE_CHUNK_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting: stash once, never rewrite here.
            eprintln!(
                "[crussty-plugin] blend_cache: pristine sighting {name} ({} bytes, major {})",
                bytes.len(),
                class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut orig = orig_lock().lock().unwrap();
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Phase 3 (not in this prototype): serve the precomputed patch.
        // TODO(B10 Phase 3): implement ONLY after docs/HOOK_BLEND_CACHE.md
        // §3.4 checklist passes; compute the patched class on the quiet
        // activation worker (ASM `replace_body` pattern, see
        // src/improved_noise.rs Phase 2) and cache it in PATCH_CACHE. Zero
        // class definitions may happen inside this callback (COMPUTE_FRAMES
        // deadlock, see improved_noise.rs header).
        if PATCH_ENABLED {
            let cached = patch_lock().lock().unwrap().clone();
            if cached.is_none() {
                return None;
            }
            PATCHED.store(true, Ordering::SeqCst);
            return cached;
        }
        None
    });
}

// ---------------------------------------------------------------------------
// activate() — background worker (gate OFF ⇒ nothing happens)
// ---------------------------------------------------------------------------

/// Background activation: wait for boot, run the read-only parity self-test
/// through the injected P500 bridge, probe candidate patch points, log the
/// verdict. Defines NO classes and retransforms NOTHING in this prototype.
pub fn activate() {
    if !enabled() {
        // register() already logged the dormant notice; mirror improved_noise:
        // no background work at all when gated off.
        return;
    }
    std::thread::spawn(|| {
        // TODO(B10 Phase 2): also wait for the kernel classes here
        // (cplug_sdk::classes::find_class(NOISE_CHUNK_CLASS) poll +
        // force-load pattern from area_map::activate) BEFORE defining the
        // ops bridge into the kernel loader. The parity harness itself only
        // needs the bootstrap-loader P500 bridge, so Phase 1 skips kernel
        // class polling entirely — zero touching of worldgen classes.
        if !wait_for_boot() {
            eprintln!(
                "[crussty-plugin] blend_cache: boot marker not seen, hook stays dormant"
            );
            return;
        }

        // 1) Parity self-test (read-only vs the kernel: two native calls).
        let ok = cplug_sdk::jni_util::with_attached(blend_parity_selftest)
            .unwrap_or(false);
        PARITY_OK.store(ok, Ordering::Release);
        if !ok {
            eprintln!(
                "[crussty-plugin] blend_cache: parity FAILED, hook stays dormant (no patch will be served this process)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] blend_cache: parity OK ({PARITY_SAMPLES} samples, bit-exact)"
            );
        }

        // 2) Patch-point probe (existence check of §3.3 candidates only).
        cplug_sdk::jni_util::with_attached(|env| {
            log_candidate_sighting(env);
            Some(())
        });

        // 3) Patch step — disabled by construction in this prototype.
        if !ok {
            return;
        }
        if !PATCH_ENABLED {
            eprintln!(
                "[crussty-plugin] blend_cache: observation-only prototype complete (PATCH_ENABLED=false, nothing patched)"
            );
            return;
        }
        // TODO(B10 Phase 2): define ops/bridge classes into the KERNEL loader
        // (area_map::activate define_class pattern; `--release 8` artifacts,
        // class-version guard like improved_noise::activate).
        // TODO(B10 Phase 3): compute the patch from ORIG_BYTES on this quiet
        // thread (cplug_sdk::asm::replace_body with ThisField/Local ArgSpecs),
        // then READY.store(true) + ONE cplug_sdk::retransform_class
        // (NOISE_CHUNK_CLASS) + the §4-style double-parity runtime check.
        let _ = READY.load(Ordering::Relaxed);
    });
}

// ---------------------------------------------------------------------------
// Parity self-test — 10k random samples, old vs new summary kernels
// ---------------------------------------------------------------------------

/// Drive `oldEmptyBlenderSummary` and `newEmptyBlenderSummary` through the
/// REAL injected bridge over `PARITY_SAMPLES` random inputs and compare
/// bit-exactly. Returns true on zero mismatches (docs §3.5 rules).
fn blend_parity_selftest(env: &JniEnv) -> bool {
    let Some(cls) = env.find_class(NATIVE_BRIDGE) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        eprintln!(
            "[crussty-plugin] blend_cache: self-test: find_class({NATIVE_BRIDGE}) failed (surface not injected?)"
        );
        return false;
    };
    let Some(old_mid) = env.get_static_method_id(cls, OLD_SUMMARY, SUMMARY_SIG) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        env.delete_local_ref(cls);
        eprintln!("[crussty-plugin] blend_cache: self-test: {OLD_SUMMARY} unresolved");
        return false;
    };
    let Some(new_mid) = env.get_static_method_id(cls, NEW_SUMMARY, SUMMARY_SIG) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        env.delete_local_ref(cls);
        eprintln!("[crussty-plugin] blend_cache: self-test: {NEW_SUMMARY} unresolved");
        return false;
    };

    // Fixed-seed xorshift64 (area_map-style LCG) — reproducible offenders.
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    let mut old_buf = vec![0i64; DST_LEN as usize];
    let mut new_buf = vec![0i64; DST_LEN as usize];
    let mut failures = 0usize;

    for sample in 0..PARITY_SAMPLES {
        // Argument space mirrors the bench group G21 (p0=N=256, p1=31):
        // n ∈ 0..=256, p ∈ 0..=63. Fresh dst arrays per call — P500 learned
        // that kernels may mutate their inputs.
        let n = (next() % 257) as i32;
        let p = (next() % 64) as i32;

        let Some(dst_old) = env.new_long_array(DST_LEN) else {
            env.delete_local_ref(cls);
            return false;
        };
        let Some(dst_new) = env.new_long_array(DST_LEN) else {
            env.delete_local_ref(dst_old);
            env.delete_local_ref(cls);
            return false;
        };

        let r_old = call_summary(env, cls, old_mid, n, p, dst_old);
        let exc_old = cplug_sdk::jni_util::clear_exception(env);
        let r_new = call_summary(env, cls, new_mid, n, p, dst_new);
        let exc_new = cplug_sdk::jni_util::clear_exception(env);

        let mut ok = true;
        if exc_old || exc_new {
            ok = false;
            if failures < MAX_REPORTED_FAILURES {
                eprintln!(
                    "[crussty-plugin] blend_cache: SELF-TEST FAIL #{sample}: exception (old={exc_old}, new={exc_new}) at n={n} p={p}"
                );
            }
        } else if r_old != r_new {
            ok = false;
            if failures < MAX_REPORTED_FAILURES {
                eprintln!(
                    "[crussty-plugin] blend_cache: SELF-TEST FAIL #{sample}: return mismatch n={n} p={p}: old={r_old} new={r_new}"
                );
            }
        } else {
            env.get_long_array_region(dst_old, 0, DST_LEN, &mut old_buf);
            env.get_long_array_region(dst_new, 0, DST_LEN, &mut new_buf);
            let r = r_old.clamp(0, DST_LEN) as usize;
            let min_w = r.min(old_buf.len()).min(new_buf.len());
            if let Some(i) = (0..min_w).find(|&i| old_buf[i] != new_buf[i]) {
                ok = false;
                if failures < MAX_REPORTED_FAILURES {
                    eprintln!(
                        "[crussty-plugin] blend_cache: SELF-TEST FAIL #{sample}: payload mismatch n={n} p={p} at long[{i}]: old=0x{:016x} new=0x{:016x}",
                        old_buf[i] as u64, new_buf[i] as u64
                    );
                }
            }
        }
        if !ok {
            failures += 1;
        }

        // Determinism probe (docs §3.5): every 1000th sample, re-run the
        // native path twice on identical inputs and require identical output.
        if sample % 1000 == 0 && !exc_new {
            let r_new2 = call_summary(env, cls, new_mid, n, p, dst_new);
            let exc2 = cplug_sdk::jni_util::clear_exception(env);
            if exc2 || r_new2 != r_new {
                failures += 1;
                if failures < MAX_REPORTED_FAILURES {
                    eprintln!(
                        "[crussty-plugin] blend_cache: SELF-TEST FAIL #{sample}: nondeterministic native path (r={r_new} vs {r_new2})"
                    );
                }
            }
        }

        env.delete_local_ref(dst_old);
        env.delete_local_ref(dst_new);

        if failures > PARITY_SAMPLES / 10 {
            // >10% broken: bail out early, the verdict is already clear.
            // (MAX_REPORTED_FAILURES only caps the per-sample log spam.)
            eprintln!(
                "[crussty-plugin] blend_cache: self-test aborted at sample {sample} ({} failures)",
                failures
            );
            env.delete_local_ref(cls);
            return false;
        }

        // Yield between batches; old-path cost ~67µs/call makes this loop
        // ~1s total, but politeness at boot is free.
        if sample % PARITY_BATCH == PARITY_BATCH - 1 {
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }

    env.delete_local_ref(cls);
    if failures == 0 {
        true
    } else {
        eprintln!(
            "[crussty-plugin] blend_cache: self-test FAILED {failures}/{PARITY_SAMPLES}"
        );
        false
    }
}

/// One `(II[J)I` summary call through the bridge.
fn call_summary(
    env: &JniEnv,
    cls: jni::jclass,
    mid: jni::jmethodID,
    n: i32,
    p: i32,
    dst: jni::jlongArray,
) -> i32 {
    env.call_static_int_method(
        cls,
        mid,
        &[
            jni::jvalue { i: n },
            jni::jvalue { i: p },
            jni::jvalue { l: dst },
        ],
    )
}

// ---------------------------------------------------------------------------
// Patch-point probe (existence check only — resolves method IDs and logs)
// ---------------------------------------------------------------------------

/// Probe the §3.3 candidate methods against the LIVE NoiseChunk class and
/// log which resolve. Never patches anything; NoSuchMethodError pending
/// exceptions are cleared per probe. Also confirms the blend-related
/// neighbor classes are loaded (Blender / BlendingData).
fn log_candidate_sighting(env: &JniEnv) {
    let Some(cls) = cplug_sdk::classes::find_class(NOISE_CHUNK_CLASS) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        eprintln!(
            "[crussty-plugin] blend_cache: probe: {NOISE_CHUNK_CLASS} not loaded (worldgen idle?) — candidates unverified"
        );
        return;
    };
    for (name, desc) in CANDIDATE_PATCH_METHODS {
        let hit = env.get_method_id(cls.as_jclass(), name, desc).is_some();
        let _ = cplug_sdk::jni_util::clear_exception(env);
        eprintln!(
            "[crussty-plugin] blend_cache: probe: NoiseChunk.{name} {desc} -> {}",
            if hit { "RESOLVED" } else { "absent (verify with javap)" }
        );
    }
    for other in [BLENDER_CLASS, BLENDING_DATA_CLASS] {
        let hit = cplug_sdk::classes::find_class(other).is_some();
        eprintln!(
            "[crussty-plugin] blend_cache: probe: {other} -> {}",
            if hit { "loaded" } else { "not loaded" }
        );
    }
}

// ---------------------------------------------------------------------------
// Boot gate (copy of improved_noise::wait_for_boot — isolation over DRY)
// ---------------------------------------------------------------------------

/// Wait until the server has finished booting: `org/bukkit/Bukkit`'s static
/// `getServer()` returns a non-null CraftServer, with a short settling delay
/// after that. Returns false on timeout (~120s).
fn wait_for_boot() -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    loop {
        let booted = cplug_sdk::jni_util::with_attached(|env| {
            let Some(bukkit) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
                return false;
            };
            let Some(get_server) = env.get_static_method_id(
                bukkit.as_jclass(),
                "getServer",
                "()Lorg/bukkit/Server;",
            ) else {
                let _ = cplug_sdk::jni_util::clear_exception(env);
                return false;
            };
            let srv = env.call_static_object_method(bukkit.as_jclass(), get_server, &[]);
            let had_exc = cplug_sdk::jni_util::clear_exception(env);
            if srv.is_null() || had_exc {
                false
            } else {
                env.delete_local_ref(srv);
                true
            }
        });
        if booted.unwrap_or(false) {
            std::thread::sleep(std::time::Duration::from_secs(10));
            return true;
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

// ---------------------------------------------------------------------------
// Wiring notes for the main agent (do NOT edit src/lib.rs from B10):
//
//   mod proto_blend_cache;                    // next to mod area_map;
//   ...
//   proto_blend_cache::register();            // in cplugin_init, after
//                                             // improved_noise::register();
//   ...
//   proto_blend_cache::activate();            // at the end of inject_surface,
//                                             // after improved_noise::activate();
//
// Both calls are no-ops unless CRUSSTY_NATIVE_BLEND_CACHE is set, so wiring
// the prototype cannot change default behavior.
// ---------------------------------------------------------------------------
