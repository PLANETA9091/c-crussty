//! Runtime wiring for the noise BATCH FILL hooks (TASK-108 v1,
//! docs/BATCH_BRIDGE_DESIGN.md + design-doc §6 G-ABI-2 outcome).
//!
//! Two whole-body swap targets:
//!   * `DensityFunctions$Noise.fillArray` — javap: a bare
//!     `ctx.fillAllDirectly(this, out)`; per point the full stack
//!     NoiseHolder.getValue -> NormalNoise -> 2x PerlinNoise octaves.
//!   * `DensityFunctions$ShiftNoise.fillArray` — the INTERFACE default
//!     (same bare body), inherited by ShiftA/ShiftB/ShiftedNoise.
//!
//! Both bodies are replaced with one invokestatic into
//! `NormalNoiseBatchOps` (compiled noise/ bridge, embedded here): the
//! bridge resolves the striped {h1,h2,vf} handle triple for the wrapped
//! NormalNoise, runs the RECORDING pass through the provider's own
//! vanilla fillAllDirectly (parity-by-construction incl. NoiseChunk$1's
//! stateful forIndex), then ONE native crossing
//! (nativeFillScaledPositions / nativeFillShiftA / nativeFillShiftB —
//! G-ABI-2 decoded bit-exact, already registered by jni_table.rs rows
//! 282-291 on every boot). Fallbacks collapse to ctx.fillAllDirectly =
//! the exact vanilla body; compute() is never touched.
//!
//! Gate: env `CRUSSTY_NATIVE_NOISE_FILL` (1/true/on/yes -> on) AND the
//! kernel-policy two-key rule (src/kernel_policy.rs) — the entry
//! ("DensityFunctions$Noise", "noiseFillArrayWholeBody") is not in
//! PROVEN_WINS yet, so A/B arms run with CRUSSTY_KERNEL_POLICY=off
//! (documented purpose: benchmarking an unproven kernel). Off by
//! default: dormant = byte-identical classes (hopper-jar rule).
//!
//! Two halfs (identical shape to improved_noise/perlin_noise):
//!  1. Byte hooks capture each target's ORIGINAL bytes on first sighting;
//!     patches are computed on the quiet activation worker; exactly one
//!     retransform per target.
//!  2. Self-test: after both retransforms, NormalNoiseBatchOps.selfTest()
//!     drives the REAL bridged fillArray vs the vanilla per-point leaf
//!     across sizes {1,17,256} for Noise/ShiftA/ShiftB + the proto-holder
//!     fallback. Raw-bits equality mandatory; the bench refuses to arm on
//!     a SELFTEST FAIL line.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const FILL_CLASS: &str = "net/minecraft/world/level/levelgen/DensityFunctions$Noise";
const SHIFT_CLASS: &str = "net/minecraft/world/level/levelgen/DensityFunctions$ShiftNoise";
const OPS_NAME: &str = "net/minecraft/world/level/levelgen/NormalNoiseBatchOps";

const FILLARRAY_DESC: &str = "([DLnet/minecraft/world/level/levelgen/DensityFunction$ContextProvider;)V";

const POLICY_KERNEL: &str = "noiseFillArrayWholeBody";

const OPS_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps.class"
));
const OPS_HANDLE_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Handle.class"
));
const OPS_REAPER_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Reaper.class"
));
const OPS_RECORDER_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Recorder.class"
));
const OPS_RECORDER_TL_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$RecorderTL.class"
));
const OPS_REC_OUT_TL_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$RecOutTL.class"
));
const OPS_CENSUS_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Census.class"
));
const OPS_TEST_PROVIDER_BYTES: &[u8] = include_bytes!(concat!(
    "../noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps$TestProvider.class"
));

const OPS_EMBEDS: [(&str, &[u8]); 8] = [
    (OPS_NAME, OPS_BYTES),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Handle",
        OPS_HANDLE_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Reaper",
        OPS_REAPER_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Recorder",
        OPS_RECORDER_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$RecorderTL",
        OPS_RECORDER_TL_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$RecOutTL",
        OPS_REC_OUT_TL_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$Census",
        OPS_CENSUS_BYTES,
    ),
    (
        "net/minecraft/world/level/levelgen/NormalNoiseBatchOps$TestProvider",
        OPS_TEST_PROVIDER_BYTES,
    ),
];

/// One whole-body swap target: the class to patch and the same-package
/// bridge method the body becomes (args: this, out[], ctx — slots 0/1/2).
struct Target {
    class: &'static str,
    bridge_name: &'static str,
    bridge_desc: &'static str,
    label: &'static str,
}

const TARGETS: [Target; 2] = [
    Target {
        class: FILL_CLASS,
        bridge_name: "fillNoise",
        bridge_desc: "(Lnet/minecraft/world/level/levelgen/DensityFunctions$Noise;[DLnet/minecraft/world/level/levelgen/DensityFunction$ContextProvider;)V",
        label: "Noise",
    },
    Target {
        class: SHIFT_CLASS,
        bridge_name: "fillShift",
        bridge_desc: "(Lnet/minecraft/world/level/levelgen/DensityFunctions$ShiftNoise;[DLnet/minecraft/world/level/levelgen/DensityFunction$ContextProvider;)V",
        label: "ShiftNoise",
    },
];

/// env-gate (off by default), read once at register time
fn enabled() -> bool {
    std::env::var("CRUSSTY_NATIVE_NOISE_FILL")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: [AtomicBool; 2] = [AtomicBool::new(false), AtomicBool::new(false)];
/// Global ref to the kernel classloader (captured at activation).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of each target.
static ORIG_BYTES: [std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>>; 2] =
    [std::sync::OnceLock::new(), std::sync::OnceLock::new()];

/// Patched bytecode cache per target (TASK-26/C5 pattern: Arc + major).
#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

static PATCH_CACHE: [std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>>; 2] =
    [std::sync::OnceLock::new(), std::sync::OnceLock::new()];
/// One-shot flag per target for the per-serve log line.
static SERVE_LOGGED: [AtomicBool; 2] = [AtomicBool::new(false), AtomicBool::new(false)];

fn orig_lock(i: usize) -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES[i].get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock(i: usize) -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE[i].get_or_init(|| std::sync::Mutex::new(None))
}
// Poison-recovery (TASK-46): every lock of ORIG_BYTES / PATCH_CACHE uses
// `unwrap_or_else(PoisonError::into_inner)` — hook-callback threads must
// never unwind across JNI (VM abort).

/// Register the byte hooks (idempotent; call once from cplugin_init).
/// The callbacks perform NO JNI/ASM work (loader-lock discipline): pristine
/// capture at the class's own load, patch served from the precomputed cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] noise_fill: dormant (set CRUSSTY_NATIVE_NOISE_FILL=1 to enable)"
        );
        return;
    }
    // TASK-86 two-key gate: env AND kernel-policy must both Allow. The
    // fill family is NOT in the proven registry yet — A/B arms set
    // CRUSSTY_KERNEL_POLICY=off (the documented benchmarking override);
    // strict/audit modes keep this dormant by design.
    for (i, t) in TARGETS.iter().enumerate() {
        match crate::kernel_policy::decide(t.class, POLICY_KERNEL) {
            crate::kernel_policy::Decision::Allow => {}
            crate::kernel_policy::Decision::KeepJava { reason } => {
                eprintln!(
                    "[crussty-plugin] noise_fill: kernel-policy KeepJava for {} ({reason}) — staying dormant despite env gate",
                    t.label
                );
                return;
            }
        }
        crate::kernel_policy::audit_wire(
            t.class,
            POLICY_KERNEL,
            "noise_fill batch whole-body arming (fillArray -> NormalNoiseBatchOps)",
        );
        let idx = i;
        cplug_sdk::hooks::register_bytes(t.class, move |_name, bytes| {
            if !READY[idx].load(Ordering::Relaxed) {
                // Pristine sighting: stash the bytes for the worker.
                eprintln!(
                    "[crussty-plugin] noise_fill: pristine sighting {} ({} bytes, major {})",
                    TARGETS[idx].label,
                    bytes.len(),
                    crate::improved_noise::class_version(bytes)
                        .map(|(m, _)| m)
                        .unwrap_or(0)
                );
                let mut orig = orig_lock(idx).lock().unwrap_or_else(PoisonError::into_inner);
                if orig.is_none() {
                    *orig = Some(bytes.to_vec());
                }
                return None;
            }
            // Serve the precomputed patch (Arc refcount bump under the lock).
            let cached = patch_lock(idx)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            if !SERVE_LOGGED[idx].swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] noise_fill: hook serve {} ({} bytes, major {})",
                    TARGETS[idx].label,
                    cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                    cached.as_ref().map(|c| c.major).unwrap_or(0)
                );
            }
            cached.map(|c| c.bytes.to_vec())
        });
    }
}

/// Background activation: wait for both kernel classes, define the ops
/// bridge into the kernel loader, compute both patches, flip READY and
/// retransform each target once, then run the live bit-exact self-test.
pub fn activate() {
    if !enabled() {
        // register() logged the dormant notice; no hook -> nothing to do.
        return;
    }
    std::thread::spawn(|| {
        // Phase 0: wait for BOTH targets (the ShiftNoise interface can
        // lag the Noise record by seconds; poll both).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(240);
        let mut forced = 0usize;
        loop {
            let all = TARGETS
                .iter()
                .all(|t| cplug_sdk::classes::find_class(t.class).is_some());
            if all {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] noise_fill: kernel classes not loaded within 240s, hooks stay dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(230) {
                forced += 1;
                if forced <= 12 || forced % 12 == 0 {
                    for t in TARGETS.iter() {
                        if cplug_sdk::classes::find_class(t.class).is_none() {
                            eprintln!(
                                "[crussty-plugin] noise_fill: forcing kernel load of {} (attempt {forced})",
                                t.class
                            );
                            crate::improved_noise::force_load_kernel_class(t.class);
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(if forced > 0 { 2 } else { 10 }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] noise_fill: boot marker not seen, hooks stay dormant");
            return;
        }
        eprintln!(
            "[crussty-plugin] noise_fill: server booted, defining NormalNoiseBatchOps into kernel loader"
        );

        // Guard: embedded bridge bytes must not exceed the JVM's class-file
        // version (UnsupportedClassVersionError hardening, mirrors perlin).
        let max_major =
            cplug_sdk::jni_util::with_attached(crate::improved_noise::jvm_max_class_major).flatten();
        if let Some(max) = max_major {
            for (name, bytes) in OPS_EMBEDS {
                let bm = crate::improved_noise::class_version(bytes)
                    .map(|(m, _)| m)
                    .unwrap_or(0);
                if bm > max {
                    eprintln!(
                        "[crussty-plugin] noise_fill: {name} is class-file major {bm} but this JVM supports up to {max} — rebuild noise/ via scripts/build_noise.sh; hooks stay dormant"
                    );
                    return;
                }
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(FILL_CLASS) else {
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
                        eprintln!("[crussty-plugin] noise_fill: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] noise_fill: define_class({name}) failed");
                        ok = false;
                    }
                }
            }
            if cplug_sdk::asm::ensure_defined(env, gref).is_none() {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] noise_fill: asm helper define failed");
                ok = false;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] noise_fill: bridge definition aborted (no env or loader)");
            return;
        }

        // Capture originals for any target the hook never saw (fast boots:
        // the class may predate hook registration) — no-op retransform
        // retries, then the loader resource stream as the last resort.
        for (i, t) in TARGETS.iter().enumerate() {
            if orig_lock(i)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_some()
            {
                continue;
            }
            eprintln!(
                "[crussty-plugin] noise_fill: {} predates hook, capturing current bytes via no-op retransform",
                t.label
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(t.class);
                eprintln!(
                    "[crussty-plugin] noise_fill: capture retransform {} rc={rc} (attempt {attempt})",
                    t.label
                );
                captured = orig_lock(i)
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .is_some();
                if captured {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !captured {
                let stream_bytes = resource_stream_capture(t.class);
                match stream_bytes {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] noise_fill: resource-stream capture {} ({} bytes)",
                            t.label,
                            bytes.len()
                        );
                        *orig_lock(i).lock().unwrap_or_else(PoisonError::into_inner) =
                            Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] noise_fill: resource-stream capture failed for {}, hook stays dormant",
                            t.label
                        );
                    }
                }
            }
        }

        // Phase 2: compute both patches on the quiet thread.
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] noise_fill: no kernel loader captured");
            return;
        }
        let mut ready_all = true;
        for (i, t) in TARGETS.iter().enumerate() {
            let original = orig_lock(i)
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            let Some(original) = original else {
                eprintln!(
                    "[crussty-plugin] noise_fill: no original bytes for {}, hook stays dormant",
                    t.label
                );
                ready_all = false;
                continue;
            };
            let patched = cplug_sdk::jni_util::with_attached(|env| {
                let spec = cplug_sdk::asm::ReplaceBody {
                    method_name: "fillArray",
                    method_desc: FILLARRAY_DESC,
                    bridge_owner: OPS_NAME,
                    bridge_name: t.bridge_name,
                    bridge_desc: t.bridge_desc,
                    args: &[
                        cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }, // this
                        cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'[' }, // out
                        cplug_sdk::asm::ArgSpec::Local { slot: 2, ty: b'L' }, // ctx
                    ],
                };
                cplug_sdk::asm::replace_body(env, loader as jni::jobject, &original, &spec)
            })
            .flatten();
            let Some(patched) = patched else {
                eprintln!(
                    "[crussty-plugin] noise_fill: patch computation failed for {}, hook stays dormant",
                    t.label
                );
                ready_all = false;
                continue;
            };
            let patch_major =
                crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
            eprintln!(
                "[crussty-plugin] noise_fill: computed patch for {} fillArray ({} -> {} bytes), orig major {} patch major {}",
                t.label,
                original.len(),
                patched.len(),
                crate::improved_noise::class_version(&original)
                    .map(|(m, _)| m)
                    .unwrap_or(0),
                patch_major
            );
            *patch_lock(i).lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
                bytes: Arc::from(patched),
                major: patch_major,
            });
            READY[i].store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(t.class);
            eprintln!(
                "[crussty-plugin] noise_fill: {} hook armed, retransform rc={rc}",
                t.label
            );
        }
        if !ready_all {
            return;
        }

        // Phase 3: live bit-exact self-test BEFORE any bench (design gate 1).
        selftest();
    });
}

/// Last-resort pristine capture via the kernel loader's resource stream
/// (no JVMTI event delivery involved; mirrors perlin_noise).
fn resource_stream_capture(target: &str) -> Option<Vec<u8>> {
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
        let res_name = env.new_string_utf(&format!("{target}.class"))?;
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

/// Drive NormalNoiseBatchOps.selfTest() and log the verdict LOUDLY.
/// The bench harness greps for `SELFTEST PASS` and refuses armed runs on
/// anything else (design gate: bit-exact before any A/B).
fn selftest() {
    let result = cplug_sdk::jni_util::with_attached(|env| -> Option<String> {
        let cls = env.find_class(OPS_NAME)?;
        let mid = env.get_static_method_id(cls, "selfTest", "()Ljava/lang/String;")?;
        let val = env.call_static_object_method(cls, mid, &[]);
        let _ = crate::clear_exception(env);
        env.delete_local_ref(cls);
        if val.is_null() {
            return None;
        }
        let s = env.get_string_utf(val as jni::jstring);
        env.delete_local_ref(val);
        s
    })
    .flatten();
    match result {
        Some(line) => eprintln!("[crussty-plugin] noise_fill: {line}"),
        None => eprintln!(
            "[crussty-plugin] noise_fill: CRUSSTY_NOISE_FILL SELFTEST FAIL exception=selfTest call failed"
        ),
    }
}
