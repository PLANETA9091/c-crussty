//! Runtime wiring for the `perlin_noise` hook (TASK-73, Session-1 of
//! docs/WORLDGEN_BATCHING_LAYER_DESIGN.md §8).
//!
//! Replaces the kernel's `PerlinNoise.getValue(DDDDDZ)D` body — the WHOLE
//! octave loop (TASK-69 recon: the method IS the loop) — with a bridge
//! (`PerlinNoiseNativeOps.getValue`) that samples the whole object through
//! ONE native crossing (`PaperNativePerlinNoise.nativeGetValue` on a handle
//! built by `nativeBuildHandle` from the live instance's state, TASK-70
//! ABI). All CPU gates are GO: G-STEP0 (0.59x octave grain), G-RECON
//! (whole-method owner shapes), G-ABI (0.825x whole-object kernel, ABI
//! decoded, parity 0/51000), G-BODY (whole-body swap through a REAL
//! retransform, 0.815x end-to-end, dispatch overhead <= noise).
//!
//! The bridge extracts the instance's private state reflectively ONCE per
//! instance (22-60 us warm) and degrades to a bit-exact Java re-implementation
//! of the kernel octave loop if the handle path fails for any reason —
//! worldgen never sees a NaN from this wiring (B.2.2 ladder inside the
//! bridge; see PerlinNoiseNativeOps.java).
//!
//! Gate: env `CRUSSTY_NATIVE_PERLIN_NOISE` — ON BY DEFAULT as of TASK-148
//! (owner optimization directive "оптимизируй"): the TASK-74 G-AB live A/B
//! win (wall −12.3%, cpu_burst −11.1% median with perfect separation, parity
//! 0/20000 bit-exact, bench/e2e/results/PERLIN_AB_2026-09-09.md) is the
//! product default now. The kernel-policy key (PROVEN_WINS entry, TASK-86)
//! still guards arming — the two-key contract is UNCHANGED and either key
//! alone remains a kill-switch. Explicit opt-out: `0/false/off/no`. Opt-out
//! (or policy KeepJava) keeps the dormant-invisible discipline: register()
//! logs a notice and NO hook is installed, activate() returns immediately,
//! and the module is byte-indistinguishable from the pre-TASK-73 plugin.
//!
//! The Java bridge references `PerlinNoise` directly, so it must live in
//! the kernel's loader (same pattern as improved_noise's
//! ImprovedNoiseNativeOps), not the bootstrap. `PaperNativePerlinNoise`
//! itself is defined into the bootstrap loader by the manifest-driven
//! native surface (src/jni_table.rs lists it with all four natives) — the
//! kernel-loader bridge resolves it parent-first.
//!
//! Two halfs (identical shape to improved_noise):
//!
//! 1. Hook: the byte hook captures the class's ORIGINAL bytes on its first
//!    sighting (the class's own load, READY=false). Once the kernel class
//!    is loaded and the server has booted, the bridge classes are defined
//!    into the kernel loader, the patch is computed on the quiet activation
//!    worker (cplug_sdk::asm::replace_body — the proven whole-body helper;
//!    the callback itself never runs class-file surgery), then retransform
//!    once — the byte hook serves the precomputed bytes back.
//!
//! 2. Self-test: after activation, drive the real bridge natives
//!    (`PaperNativePerlinNoise.nativeBuildHandle`/`nativeGetValue` through
//!    JNI) over a synthetic two-slot config: the handle builds, samples are
//!    finite, two independently built handles sample bit-identically, and
//!    both handles free cleanly.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

pub const PERLIN_CLASS: &str = "net/minecraft/world/level/levelgen/synth/PerlinNoise";
const OPS_NAME: &str = "net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps";
const OPS_HANDLE_NAME: &str = "net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Handle";
const OPS_REAPER_NAME: &str = "net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Reaper";
const NATIVE_BRIDGE: &str = "net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise";

const OPS_BYTES: &[u8] =
    include_bytes!("../noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps.class");
const OPS_HANDLE_BYTES: &[u8] =
    include_bytes!("../noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Handle.class");
const OPS_REAPER_BYTES: &[u8] =
    include_bytes!("../noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Reaper.class");

/// Whole-body replacement spec: the kernel method's seven argument slots
/// (this + 5 doubles + the fixedYMax flag) pass straight through to the
/// bridge — no field reads in the generated body (G-BODY-proven shape:
/// straight-line, trivially verifiable).
const BRIDGE_GETVALUE_DESC: &str =
    "(Lnet/minecraft/world/level/levelgen/synth/PerlinNoise;DDDDDZ)D";

/// env-gate (TASK-148: ON BY DEFAULT — the measured TASK-74 G-AB win is the
/// product default; explicit opt-out 0/false/off/no), read once at register
/// time. The kernel-policy two-key gate is unchanged and still guards arming.
fn enabled() -> bool {
    std::env::var("CRUSSTY_NATIVE_PERLIN_NOISE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v == "0" || v == "false" || v == "off" || v == "no")
        })
        .unwrap_or(true)
}

/// Kernel-policy key for the arming decision (TASK-86): the registry entry
/// `("PerlinNoise", "getValueWholeBody")` carries the G-AB live evidence.
/// short_class() normalization makes the full internal PERLIN_CLASS match.
const POLICY_KERNEL: &str = "getValueWholeBody";

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel PerlinNoise classloader, captured at activation.
/// 0 = not captured yet.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of the class — its
/// original load goes through the byte hook while READY=false, so the load
/// callback stores the pristine bytes (no retransform needed). The patched
/// bytecode is computed from them on the quiet worker thread, then cached
/// for the final retransform's callback.
static ORIG_BYTES: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
    std::sync::OnceLock::new();

/// Patched bytecode cache (TASK-26/C5 pattern): Arc<[u8]> + major parsed
/// once at patch-compute time — the serve path does no header re-parsing.
#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

static PATCH_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>> =
    std::sync::OnceLock::new();
/// One-shot flag for the per-serve log line.
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn orig_lock() -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock() -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE.get_or_init(|| std::sync::Mutex::new(None))
}
// Poison-recovery (TASK-46): every lock above uses
// `unwrap_or_else(PoisonError::into_inner)` — a panic while a guard is held
// must not turn the next hook-callback invocation into an unwind-across-JNI
// VM abort. The guards only wrap plain Vec/Arc reads/writes (no user code
// runs under the lock), so a poisoned mutex still holds valid data.

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// The callback performs NO JNI/class-file work (loader-lock discipline —
/// see improved_noise): pristine capture at the class's own load, patch
/// served from the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] perlin_noise: dormant (CRUSSTY_NATIVE_PERLIN_NOISE opt-out)"
        );
        return;
    }
    // TASK-86 two-key promotion gate: env AND kernel-policy must both Allow.
    // KeepJava (unknown / demoted / policy=strict-without-registry-entry)
    // keeps the Java path for the boot — the policy is the promotion ledger,
    // the env flag is the operator switch (B.2.2 ladder: either side refuses
    // => dormant; either side can kill-switch by itself).
    match crate::kernel_policy::decide(PERLIN_CLASS, POLICY_KERNEL) {
        crate::kernel_policy::Decision::Allow => {}
        crate::kernel_policy::Decision::KeepJava { reason } => {
            eprintln!(
                "[crussty-plugin] perlin_noise: kernel-policy KeepJava ({reason}) — staying dormant despite env gate"
            );
            return;
        }
    }
    crate::kernel_policy::audit_wire(
        PERLIN_CLASS,
        POLICY_KERNEL,
        "perlin_noise whole-body bridge arming (PerlinNoise.getValue -> PerlinNoiseNativeOps)",
    );
    cplug_sdk::hooks::register_bytes(PERLIN_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] perlin_noise: pristine sighting {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut orig = orig_lock().lock().unwrap_or_else(PoisonError::into_inner);
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = patch_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] perlin_noise: hook serve {} bytes (major {})",
                cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                cached.as_ref().map(|c| c.major).unwrap_or(0)
            );
        }
        cached.map(|c| c.bytes.to_vec())
    });
}

/// Background activation: wait for the kernel class, define the bridge into
/// its loader, compute the whole-body patch, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        // register() already logged the dormant notice; without it the byte
        // hook is NOT registered, so defining bridges / retransforming here
        // could only define classes nobody calls and confuse the log.
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(PERLIN_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] perlin_noise: {PERLIN_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                if forced_attempts < 12 || forced_attempts.is_multiple_of(12) {
                    forced_attempts += 1;
                    eprintln!(
                        "[crussty-plugin] perlin_noise: forcing kernel load of {PERLIN_CLASS} (attempt {forced_attempts})"
                    );
                }
                crate::improved_noise::force_load_kernel_class(PERLIN_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(PERLIN_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Define/retransform only after the kernel loader has gone quiet
        // (a DefineClass racing the boot-time class-loading storm deadlocks
        // the JVM inside defineClass1 — improved_noise's hardening note).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] perlin_noise: boot marker not seen, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] perlin_noise: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let max_major = cplug_sdk::jni_util::with_attached(crate::improved_noise::jvm_max_class_major).flatten();
        if let Some(max) = max_major {
            for (name, bytes) in OPS_EMBEDS {
                let bm = crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0);
                if bm > max {
                    eprintln!(
                        "[crussty-plugin] perlin_noise: embedded bridge {name} is class major {bm} but this JVM supports up to {max} — rebuild noise/ via scripts/build_noise.sh; hook stays dormant"
                    );
                    return;
                }
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let jvm_major = crate::improved_noise::jvm_class_major(env).unwrap_or(u16::MAX);
            for (name, bytes) in OPS_EMBEDS {
                if let Some((major, _)) = crate::improved_noise::class_version(bytes) {
                    if major > jvm_major {
                        eprintln!(
                            "[crussty-plugin] perlin_noise: {name} is class-file major {major} but this JVM supports up to {jvm_major} — hook stays dormant"
                        );
                        return false;
                    }
                }
            }
            let Some(cls) = cplug_sdk::classes::find_class(PERLIN_CLASS) else {
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
            let mut ok = true;
            for (name, bytes) in OPS_EMBEDS {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] perlin_noise: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] perlin_noise: define_class({name}) failed");
                        ok = false;
                    }
                }
            }
            // Pre-define the ASM helper into the kernel loader too, so the
            // byte hook never performs a define_class mid-retransform.
            if cplug_sdk::asm::ensure_defined(env, gref).is_none() {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] perlin_noise: asm helper define failed");
                ok = false;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] perlin_noise: bridge definition aborted (no env or loader)");
            return;
        }

        // The ORIGINAL bytes were captured by the byte hook during the
        // class's own load. If the kernel loaded the class before this
        // plugin registered (possible on fast boots), a no-op retransform
        // delivers the current bytes through the hook (READY=false -> stash
        // only), with the resource stream as the last-resort fallback.
        if orig_lock().lock().unwrap_or_else(PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] perlin_noise: class predates hook, capturing current bytes via no-op retransform"
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(PERLIN_CLASS);
                eprintln!(
                    "[crussty-plugin] perlin_noise: capture retransform rc={rc} (attempt {attempt})"
                );
                captured = orig_lock()
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .is_some();
                if captured {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !captured {
                eprintln!(
                    "[crussty-plugin] perlin_noise: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                let stream_bytes = resource_stream_capture();
                match stream_bytes {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] perlin_noise: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] perlin_noise: resource-stream capture failed too, hook stays dormant"
                        );
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] perlin_noise: no original bytes captured even after retransform, hook stays dormant"
            );
            return;
        };

        // Phase 2: compute the whole-body patch on the quiet thread.
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] perlin_noise: no kernel loader captured");
            return;
        }
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            let spec = cplug_sdk::asm::ReplaceBody {
                method_name: "getValue",
                method_desc: "(DDDDDZ)D",
                bridge_owner: OPS_NAME,
                bridge_name: "getValue",
                bridge_desc: BRIDGE_GETVALUE_DESC,
                args: &[
                    cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }, // this
                    cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'D' }, // x
                    cplug_sdk::asm::ArgSpec::Local { slot: 3, ty: b'D' }, // y
                    cplug_sdk::asm::ArgSpec::Local { slot: 5, ty: b'D' }, // z
                    cplug_sdk::asm::ArgSpec::Local { slot: 7, ty: b'D' }, // y0
                    cplug_sdk::asm::ArgSpec::Local { slot: 9, ty: b'D' }, // y1
                    cplug_sdk::asm::ArgSpec::Local { slot: 11, ty: b'Z' }, // fixedYMax flag
                ],
            };
            cplug_sdk::asm::replace_body(env, loader as jni::jobject, &original, &spec)
        })
        .flatten();
        let Some(patched) = patched else {
            eprintln!("[crussty-plugin] perlin_noise: patch computation failed, hook stays dormant");
            return;
        };
        let patch_major = crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] perlin_noise: computed patch for getValue(DDDDDZ)D ({} -> {} bytes), orig major {} patch major {}",
            original.len(),
            patched.len(),
            crate::improved_noise::class_version(&original).map(|(m, _)| m).unwrap_or(0),
            patch_major
        );
        *patch_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(patched),
            major: patch_major,
        });

        // Phase 3: a SINGLE retransform; the callback serves the cached
        // patch. kernel_policy: the PaperNativePerlinNoise pairing is new —
        // audit (log) it without asserting proven-ness; the G-AB evidence
        // feeds the PROVEN_WINS/whitelist update after the live A/B.
        crate::kernel_policy::audit_wire(NATIVE_BRIDGE, "nativeGetValue", "perlin_noise whole-body v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(PERLIN_CLASS);
        eprintln!("[crussty-plugin] perlin_noise: hook armed, retransform rc={rc}");
        eprintln!(
            "[crussty-plugin] perlin_noise: sighting feed: {} full class-heap scans avoided",
            cplug_sdk::classes::scans_avoided(PERLIN_CLASS)
        );

        bridge_selftest();
    });
}

const OPS_EMBEDS: [(&str, &[u8]); 3] = [
    (OPS_NAME, OPS_BYTES),
    (OPS_HANDLE_NAME, OPS_HANDLE_BYTES),
    (OPS_REAPER_NAME, OPS_REAPER_BYTES),
];

/// Last-resort pristine capture: the kernel loader's resource stream yields
/// the original class file bytes with no JVMTI event delivery involved.
fn resource_stream_capture() -> Option<Vec<u8>> {
    cplug_sdk::jni_util::with_attached(|env| {
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            return None;
        }
        let loader_cls = env.find_class("java/lang/ClassLoader")?;
        let garm = env.get_method_id(
            loader_cls,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
        )?;
        let res_name = env.new_string_utf(&format!("{PERLIN_CLASS}.class"))?;
        let stream = env.call_object_method(
            loader as jni::jobject,
            garm,
            &[jni::jvalue { l: res_name }],
        );
        if stream.is_null() {
            crate::clear_exception(env);
            env.delete_local_ref(res_name);
            env.delete_local_ref(loader_cls);
            return None;
        }
        let in_cls = env.find_class("java/io/InputStream")?;
        let rab = env.get_method_id(in_cls, "readAllBytes", "()[B")?;
        let arr = env.call_object_method(stream, rab, &[]);
        let out = if arr.is_null() {
            crate::clear_exception(env);
            None
        } else {
            let jarr = arr as jni::jbyteArray;
            let len = env.get_array_length(jarr as jni::jarray);
            let mut signed = vec![0i8; len as usize];
            env.get_byte_array_region(jarr, 0, len, &mut signed);
            Some(signed.iter().map(|&b| b as u8).collect::<Vec<u8>>())
        };
        env.delete_local_ref(arr);
        env.delete_local_ref(stream);
        env.delete_local_ref(res_name);
        env.delete_local_ref(loader_cls);
        out
    })
    .flatten()
}

/// Drive the real bridge through JNI over a synthetic two-slot PerlinNoise
/// config (TASK-70 ABI): nativeBuildHandle must return nonzero handles,
/// nativeGetValue samples must be finite, two independently built handles
/// must sample bit-identically, and both must free cleanly.
fn bridge_selftest() {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let Some(bridge) = env.find_class(NATIVE_BRIDGE) else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] perlin_noise: self-test: find_class({NATIVE_BRIDGE}) failed");
            return false;
        };
        let Some(build) = env.get_static_method_id(
            bridge,
            "nativeBuildHandle",
            "([B[B[D[D[D[DDD)J",
        ) else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };
        let Some(get_mid) = env.get_static_method_id(bridge, "nativeGetValue", "(JDDDDDZ)D") else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };
        let Some(free) = env.get_static_method_id(bridge, "nativeFreeHandle", "(J)V") else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };

        // Synthetic two-slot config (both octaves present), TASK-70 ABI.
        const N: usize = 2;
        let mut pcat = [0u8; 256 * N];
        for (i, b) in pcat.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(31).wrapping_add(7);
        }
        let mask = [1u8; N];
        let xo = [1.0f64, 2.0];
        let yo = [0.5f64, -0.5];
        let zo = [3.25f64, 0.75];
        let amps = [1.0f64, 0.5];
        const IN_F: f64 = 1.0;
        const VAL_F: f64 = 0.5;

        let new_bytes = |data: &[u8]| -> Option<jni::jbyteArray> {
            let arr = env.new_byte_array(data.len() as i32)?;
            let signed: Vec<i8> = data.iter().map(|&b| b as i8).collect();
            env.set_byte_array_region(arr, 0, data.len() as i32, &signed);
            Some(arr)
        };
        let (Some(a0), Some(a1)) = (new_bytes(&pcat), new_bytes(&mask)) else {
            env.delete_local_ref(bridge);
            return false;
        };

        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let jenv = raw;
            let call_long = fn_table.CallStaticLongMethodA;
            let call_double = fn_table.CallStaticDoubleMethodA;
            let call_void = fn_table.CallStaticVoidMethodA;
            let new_darr = fn_table.NewDoubleArray;
            let set_darr = fn_table.SetDoubleArrayRegion;

            // Double arrays via the raw vtable (the SDK exposes no
            // double-array helpers; same pattern as the CallStatic* calls).
            let d_arr = |data: &[f64]| -> jni::jdoubleArray {
                let arr = (new_darr)(jenv, data.len() as i32);
                (set_darr)(jenv, arr, 0, data.len() as i32, data.as_ptr());
                arr
            };
            let a2 = d_arr(&xo);
            let a3 = d_arr(&yo);
            let a4 = d_arr(&zo);
            let a5 = d_arr(&amps);

            let build_args = [
                jni::jvalue { l: a0 },
                jni::jvalue { l: a1 },
                jni::jvalue { l: a2 },
                jni::jvalue { l: a3 },
                jni::jvalue { l: a4 },
                jni::jvalue { l: a5 },
                jni::jvalue { d: IN_F },
                jni::jvalue { d: VAL_F },
            ];
            let h1 = (call_long)(jenv, bridge, build, build_args.as_ptr());
            let h2 = (call_long)(jenv, bridge, build, build_args.as_ptr());
            if h1 == 0 || h2 == 0 {
                eprintln!(
                    "[crussty-plugin] perlin_noise: self-test: buildHandle returned 0 ({h1}/{h2})"
                );
                return false;
            }
            // Determinism probe: two independently built handles from the
            // same config must sample bit-identically (same kernel, same
            // bits in -> same bits out).
            let pts: [(f64, f64, f64, f64, f64, bool); 3] = [
                (1.5, 2.5, 3.5, 0.0, 0.0, false),
                (-7.25, 123.5, -0.125, 0.5, -0.5, false),
                (33.0, 5.75, -2.0, 0.0, 0.0, true),
            ];
            for (x, y, z, y0, y1, flag) in pts {
                let args1 = [
                    jlong_val(h1),
                    d(x), d(y), d(z), d(y0), d(y1),
                    jni::jvalue { z: flag as u8 },
                ];
                let v1 = (call_double)(jenv, bridge, get_mid, args1.as_ptr());
                let args2 = [
                    jlong_val(h2),
                    d(x), d(y), d(z), d(y0), d(y1),
                    jni::jvalue { z: flag as u8 },
                ];
                let v2 = (call_double)(jenv, bridge, get_mid, args2.as_ptr());
                if !v1.is_finite() || !v2.is_finite() {
                    eprintln!(
                        "[crussty-plugin] perlin_noise: self-test: non-finite sample at ({x},{y},{z}): {v1}/{v2}"
                    );
                    return false;
                }
                if v1.to_bits() != v2.to_bits() {
                    eprintln!(
                        "[crussty-plugin] perlin_noise: self-test: nondeterministic sample at ({x},{y},{z}): {v1} vs {v2}"
                    );
                    return false;
                }
            }
            let args1 = [jlong_val(h1)];
            (call_void)(jenv, bridge, free, args1.as_ptr());
            let args2 = [jlong_val(h2)];
            (call_void)(jenv, bridge, free, args2.as_ptr());

            let _ = (call_void, call_long, call_double);
            env.delete_local_ref(a2 as jni::jobject);
            env.delete_local_ref(a3 as jni::jobject);
            env.delete_local_ref(a4 as jni::jobject);
            env.delete_local_ref(a5 as jni::jobject);
        }
        env.delete_local_ref(a0);
        env.delete_local_ref(a1);
        env.delete_local_ref(bridge);
        true
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] perlin_noise: self-test passed (whole-object handle round-trip through real bridge, deterministic)"
        ),
        _ => eprintln!("[crussty-plugin] perlin_noise: self-test failed/skipped"),
    }
}

fn d(v: f64) -> jni::jvalue {
    jni::jvalue { d: v }
}

fn jlong_val(v: i64) -> jni::jvalue {
    jni::jvalue { j: v }
}
