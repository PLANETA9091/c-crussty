//! Runtime wiring for the `improved_noise` hook.
//!
//! Replaces the kernel's `ImprovedNoise.noise(DDDDD)D` body with a bridge
//! (`ImprovedNoiseNativeOps`) that samples through the native handle
//! (`PaperNativeImprovedNoise.nativeBuildHandle`/`nativeNoise`). The
//! rewritten body reads `this`'s private `p`/`xo`/`yo`/`zo` itself (legal:
//! it is ImprovedNoise's own method) and passes them to the bridge — the
//! class file's field ACCESS FLAGS must stay untouched, because the JVM
//! rejects any field-modifier change in a retransformed class with
//! JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED.
//!
//! Gate: env `CRUSSTY_NATIVE_IMPROVED_NOISE` (1/true/on/yes → on). Off by
//! default, matching upstream Crussty CE's stance: `improved_noise` stays
//! diagnostic until a guarded runtime hook and strict server gate prove the
//! profile benefits.
//!
//! The Java bridge references `ImprovedNoise` directly, so it must live in
//! the kernel's loader (same pattern as area_map's SingleUserAreaMapOps), not
//! the bootstrap: a bootstrap-defined copy would fail to resolve the kernel
//! class and shadow it for the kernel's own (parent-first) loader.
//!
//! Two halfs:
//!
//! 1. Hook: the byte hook (registered at cplugin_init) captures the class's
//!    ORIGINAL bytes on its first sighting — the class's own load, which goes
//!    through the hook while READY=false. Once the kernel class is loaded we
//!    define the bridge classes into its loader, compute the patch from the
//!    captured bytes on the quiet activation worker, then retransform once —
//!    the byte hook serves the precomputed bytes back. The callback itself
//!    never runs the ASM helper: `ClassReader`'s COMPUTE_FRAMES resolves
//!    StackMapTable frame types through `Class.forName`, and a class
//!    definition racing live server class loads inside the redefinition
//!    callback deadlocks. Exactly one retransform, like area_map.)
//!
//! 2. Self-test: after activation, drive the REAL bridge
//!    (`PaperNativeImprovedNoise.nativeBuildHandle`/`nativeNoise` through
//!    JNI) over a synthetic 256-byte permutation: assert the handle builds,
//!    samples are finite and deterministic, and the handle frees cleanly.
//!    This exercises bridge registration + JNI marshalling + native sampling.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

pub const NOISE_CLASS: &str = "net/minecraft/world/level/levelgen/synth/ImprovedNoise";
const BRIDGE_NAME: &str = "net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps";
const BRIDGE_HANDLE_NAME: &str = "net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps$Handle";
const BRIDGE_REAPER_NAME: &str = "net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps$Reaper";
const NATIVE_BRIDGE: &str = "net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise";

const BRIDGE_BYTES: &[u8] = include_bytes!(
    "../noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.class"
);
const BRIDGE_HANDLE_BYTES: &[u8] = include_bytes!(
    "../noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps$Handle.class"
);
const BRIDGE_REAPER_BYTES: &[u8] = include_bytes!(
    "../noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps$Reaper.class"
);

/// G4 demonstrator batching helper (docs/G4_SITE_PATCH_DESIGN.md §5.1): the
/// same-descriptor retarget target for the bridge noise() call. Defined into
/// the kernel loader alongside the other embedded classes (4th embed); only
/// ever CALLED when the batch gate chain armed the site and the retarget
/// landed — with the gate off it is an inert, never-initialized class.
const BATCH_OPS_NAME: &str = "net/minecraft/world/level/levelgen/synth/ImprovedNoiseBatchOps";
const BATCH_OPS_BYTES: &[u8] = include_bytes!(
    "../noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseBatchOps.class"
);
/// Descriptor of the bridge noise() call the ASM patch emits — the retarget
/// `from`/`to` descriptor (IDENTICAL on both sides: Variant R's same-stack-
/// shape guarantee).
const BRIDGE_NOISE_DESC: &str =
    "(Lnet/minecraft/world/level/levelgen/synth/ImprovedNoise;[BDDDDDDDD)D";

/// env-gate (off by default), read once at register time
fn enabled() -> bool {
    std::env::var("CRUSSTY_NATIVE_IMPROVED_NOISE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel ImprovedNoise classloader, captured at
/// activation; the patch worker reuses it to feed the ASM helper.
/// 0 = not captured yet.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref to the DEFINED ImprovedNoiseBatchOps class (G4 demonstrator).
/// Captured directly in the define loop — resolving it later via
/// Class.forName(name, true, loader) proved unreliable live (S7-12 dormant
/// boot: "helper self-test skipped" with the exception swallowed), while the
/// define loop already HOLDS the jclass. 0 = not defined yet.
static BATCH_OPS_GREF: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of the class — its
/// original load goes through the byte hook while READY=false, so the load
/// callback stores the pristine bytes (no retransform needed). The patched
/// bytecode is computed from them on the quiet worker thread, then cached
/// for the final retransform's callback.
static ORIG_BYTES: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
    std::sync::OnceLock::new();

/// Patched bytecode cache (TASK-26/C5): the bytes live in an `Arc<[u8]>` so
/// the serve path clones a REFCOUNT out of the mutex instead of the whole
/// ~3-6KB class, and the class-file major is parsed once at patch-compute
/// time — the serve path (class-load thread) does no header re-parsing.
/// Clone is cheap: the Arc clone is a refcount bump (no class-byte memcpy).
#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

static PATCH_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>> =
    std::sync::OnceLock::new();
/// One-shot flag for the per-serve log line (TASK-26/C5): log the FIRST
/// serve, stay silent afterwards (the serve can fire more than once when a
/// re-run retransform is requested).
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn orig_lock() -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock() -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE.get_or_init(|| std::sync::Mutex::new(None))
}
// Poison-recovery (TASK-46): every lock of ORIG_BYTES / PATCH_CACHE uses
// `unwrap_or_else(PoisonError::into_inner)`. The guards only wrap plain
// Vec/Arc-snapshot reads/writes (no user code runs under the lock), so a
// poisoned mutex still holds structurally valid data. A raw .unwrap() here
// would panic on a hook-callback thread (ClassFileLoadHook runs on the JVM's
// class-load/redefinition threads) and unwind across JNI = VM abort.

/// Class-file version of `b` as (major, minor), or None if not a class file.
fn class_version(b: &[u8]) -> Option<(u16, u16)> {
    if b.len() < 8 || u32::from_be_bytes(b[0..4].try_into().ok()?) != 0xCAFE_BABE {
        return None;
    }
    Some((
        u16::from_be_bytes([b[6], b[7]]),
        u16::from_be_bytes([b[4], b[5]]),
    ))
}

/// Major class-file version the running JVM supports (Java N => 44 + N).
/// Read from the `java.class.version` system property ("65.0" on Java 21)
/// via the classloader-less bootstrap `System` class. None = unreadable.
fn jvm_class_major(env: &JniEnv) -> Option<u16> {
    let sys = env.find_class("java/lang/System")?;
    let get_prop = env.get_static_method_id(
        sys,
        "getProperty",
        "(Ljava/lang/String;)Ljava/lang/String;",
    )?;
    let key = env.new_string("java.class.version")?;
    let val = env.call_static_object_method(
        sys,
        get_prop,
        &[jni::jvalue { l: key }],
    );
    let _ = crate::clear_exception(env);
    env.delete_local_ref(key);
    env.delete_local_ref(sys);
    if val.is_null() {
        env.delete_local_ref(val);
        return None;
    }
    let s = env.get_string_utf(val);
    env.delete_local_ref(val);
    s.and_then(|s| s.split('.').next()?.parse::<u16>().ok())
}

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// The callback performs NO JNI/ASM work: classfile hooks run on the
/// redefinition thread while the JVM holds loader locks, and the ASM pipeline
/// (ClassReader/ClassWriter with COMPUTE_FRAMES) resolves StackMapTable frame
/// types through `Class.forName` — a class definition racing live server
/// class-loading deadlocks. Instead the ORIGINAL bytes are captured here (at
/// the class's own load, READY=false), patched on the quiet activation
/// worker, and this callback only serves them back from the cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] improved_noise: dormant (set CRUSSTY_NATIVE_IMPROVED_NOISE=1 to enable)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(NOISE_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] improved_noise: pristine sighting {} bytes (major {})",
                bytes.len(),
                class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            // poison-recovery: pristine-capture site on a hook-callback thread
            let mut orig = orig_lock().lock().unwrap_or_else(PoisonError::into_inner);
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Serve the precomputed patch; zero Java work on this thread beyond
        // the contract-required copy into the returned Vec (the ByteCb ABI
        // is Option<Vec<u8>>). TASK-26/C5: taking the cache out is a
        // refcount bump (no memcpy under the lock), the class version was
        // parsed once at patch-compute time, and this log line fires only
        // on the FIRST serve.
        // poison-recovery: serve site on a hook-callback thread (Arc clone
        // under the recovered lock — data stays intact after poison)
        let cached = patch_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] improved_noise: hook serve {} bytes (major {})",
                cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                cached.as_ref().map(|c| c.major).unwrap_or(0)
            );
        }
        cached.map(|c| c.bytes.to_vec())
    });
}

/// JVM's supported class-file major version ("java.class.version",
/// e.g. "65.0" on Java 21). define_class of embedded bridge bytes compiled
/// by a newer javac dies with a raw UnsupportedClassVersionError that names
/// no source — read the numbers ourselves and fail with a clear line.
fn jvm_max_class_major(env: &JniEnv) -> Option<u16> {
    let sys = env.find_class("java/lang/System")?;
    let getprop = env.get_static_method_id(
        sys,
        "getProperty",
        "(Ljava/lang/String;)Ljava/lang/String;",
    )?;
    let key = env.new_string_utf("java.class.version")?;
    let val = env.call_static_object_method(sys, getprop, &[jni::jvalue { l: key }]);
    crate::clear_exception(env);
    env.delete_local_ref(key);
    if val.is_null() {
        env.delete_local_ref(sys);
        return None;
    }
    let s = env.get_string_utf(val as jni::jstring);
    env.delete_local_ref(val);
    env.delete_local_ref(sys);
    let s = s?;
    s.split('.').next()?.parse::<u16>().ok()
}

/// Background activation: wait for the kernel class, define the bridge into
/// its loader, flip READY and retransform so the hook applies the patch.
pub fn activate() {
    if !enabled() {
        // register() already logged the dormant notice; without it the byte
        // hook is NOT registered, so defining bridges / retransforming here
        // could only define classes nobody calls and confuse the log (this
        // mismatch is what hid the stale-v69-bridge failure for so long).
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(NOISE_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] improved_noise: {NOISE_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // Retry the force-load like area_map: a single attempt races the
            // boot (Bukkit unresolvable early) and loses the hook for the run.
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                if forced_attempts < 12 || forced_attempts % 12 == 0 {
                    forced_attempts += 1;
                    eprintln!(
                        "[crussty-plugin] improved_noise: forcing kernel load of {NOISE_CLASS} (attempt {forced_attempts})"
                    );
                }
                force_load_kernel_class();
            }
            // TASK-22/C1 negative backoff: while the class name has never
            // been sighted through the ClassFileLoadHook feed, find_class
            // answers from the feed without any JVMTI scan, so a relaxed 10s
            // cadence costs nothing; once sighted, keep the 2s cadence for
            // activation latency. The 180s deadline handling above is
            // unchanged.
            let sighted = cplug_sdk::classes::is_sighted(NOISE_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Defer the define/retransform until the server is fully booted.
        // The kernel loader is hot during startup (worldgen codecs load
        // `ImprovedNoise` and friends from the main thread); a JNI
        // DefineClass into that loader races the boot-time class-loading
        // storm and deadlocks the JVM inside defineClass1. After boot the
        // kernel loader is quiet, so defining the bridge + helper there and
        // retransforming `ImprovedNoise` is safe. The byte hook is already
        // registered and passive (READY=false) until this point.
        if !wait_for_boot() {
            eprintln!(
                "[crussty-plugin] improved_noise: boot marker not seen, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] improved_noise: server booted, defining bridge into kernel loader"
        );

        // Guard: the embedded bridge bytes must not be newer than the JVM
        // (major-69 bytes from a Java 25 javac in a Java 21 kernel fail
        // define_class with a bare UnsupportedClassVersionError). Read the
        // JVM's real max and refuse early with actionable numbers.
        let max_major = cplug_sdk::jni_util::with_attached(jvm_max_class_major).flatten();
        if let Some(max) = max_major {
            let bm = class_version(BRIDGE_BYTES).map(|(m, _)| m).unwrap_or(0);
            if bm > max {
                eprintln!(
                    "[crussty-plugin] improved_noise: embedded bridge is class major {bm} but this JVM supports up to {max} — recompile noise/ with `--release 8` and rebuild; hook stays dormant"
                );
                return;
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            // Guard: the bundled bridge classes must not exceed the running
            // JVM's class-file version (a Java-25-compiled noise bridge on a
            // Java-21 kernel dies with a cryptic UnsupportedClassVersionError
            // in the server log — this turned up in CI on 2026-09-07). If the
            // artifacts are stale, disable the patch and say how to fix it.
            let jvm_major = jvm_class_major(env).unwrap_or(u16::MAX);
            for (name, bytes) in [
                (BRIDGE_NAME, BRIDGE_BYTES),
                (BRIDGE_HANDLE_NAME, BRIDGE_HANDLE_BYTES),
                (BRIDGE_REAPER_NAME, BRIDGE_REAPER_BYTES),
                (BATCH_OPS_NAME, BATCH_OPS_BYTES),
            ] {
                if let Some((major, _)) = class_version(bytes) {
                    if major > jvm_major {
                        eprintln!(
                            "[crussty-plugin] improved_noise: {name} is class-file major {major} \
                             but this JVM supports up to {jvm_major} — rebuild noise/ with \
                             'scripts/build_noise.sh' (--release), patch stays dormant"
                        );
                        return false;
                    }
                }
            }
            let Some(cls) = cplug_sdk::classes::find_class(NOISE_CLASS) else {
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
            for (name, bytes) in [
                (BRIDGE_NAME, BRIDGE_BYTES),
                (BRIDGE_HANDLE_NAME, BRIDGE_HANDLE_BYTES),
                (BRIDGE_REAPER_NAME, BRIDGE_REAPER_BYTES),
                (BATCH_OPS_NAME, BATCH_OPS_BYTES),
            ]
            {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        // G4: keep the demonstrator helper's jclass as a
                        // global ref — the self-test (and any future Rust-
                        // side plumbing) resolves it from here instead of a
                        // Class.forName round-trip through the loader.
                        if name == BATCH_OPS_NAME {
                            let g = env.new_global_ref(c);
                            BATCH_OPS_GREF.store(g as usize, Ordering::SeqCst);
                        }
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] improved_noise: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] improved_noise: define_class({name}) failed");
                        ok = false;
                    }
                }
            }
            // Pre-define the ASM helper into the kernel loader too, so the
            // byte hook (retransform callback) never performs a define_class
            // mid-retransformation (mirrors area_map: no class definition
            // inside the class-file hook callback).
            if cplug_sdk::asm::ensure_defined(env, gref).is_none() {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] improved_noise: asm helper define failed");
                ok = false;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] improved_noise: bridge definition aborted (no env or loader)");
            return;
        }

        // The ORIGINAL bytes were captured by the byte hook during the
        // class's own load (READY=false branch of register()). BUT the
        // kernel loads ImprovedNoise during early worldgen (spawn prep),
        // often BEFORE this plugin's hook even registers — in that case the
        // hook never saw the load. A retransform makes the JVM deliver the
        // class's CURRENT bytes through the hook; with READY still false the
        // hook only stores them (returns None → no bytecode change). That
        // gives us the baseline to patch, no JVMTI locks held here.
        if orig_lock().lock().unwrap_or_else(PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] improved_noise: class predates hook, capturing current bytes via no-op retransform"
            );
            // S7-8 hardening (engine TASK-08 re-land companion): retransform
            // byte delivery rides the engine's ClassFileLoadHook dispatch.
            // Pre-TASK-08 engine builds intermittently mis-named that event
            // off the VM's non-NUL-terminated name buffer and silently
            // dropped it (rc=0, zero bytes — the 2026-09-08 20:47 boot).
            // Retry a few times (each attempt re-rolls the dispatch dice),
            // then fall back to the kernel loader's resource stream, which
            // yields the original class file bytes with no JVMTI event
            // delivery involved at all.
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(NOISE_CLASS);
                eprintln!(
                    "[crussty-plugin] improved_noise: capture retransform rc={rc} (attempt {attempt})"
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
                    "[crussty-plugin] improved_noise: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                let stream_bytes = cplug_sdk::jni_util::with_attached(|env| {
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
                    let res_name = env.new_string_utf(&format!("{NOISE_CLASS}.class"))?;
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
                .flatten();
                match stream_bytes {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] improved_noise: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] improved_noise: resource-stream capture failed too, hook stays dormant"
                        );
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] improved_noise: no original bytes captured even after retransform (hook not firing?), hook stays dormant"
            );
            return;
        };

        // Phase 2: run the ASM pipeline here (quiet thread, no JVMTI locks).
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] improved_noise: no kernel loader captured");
            return;
        }
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            let spec = cplug_sdk::asm::ReplaceBody {
                method_name: "noise",
                method_desc: "(DDDDD)D",
                bridge_owner: BRIDGE_NAME,
                bridge_name: "noise",
                bridge_desc:
                    "(Lnet/minecraft/world/level/levelgen/synth/ImprovedNoise;[BDDDDDDDD)D",
                args: &[
                    // `this` (the handle cache key)
                    cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
                    // this.p / this.xo / this.yo / this.zo — read inside the
                    // patched class's own method; no access-flag changes.
                    cplug_sdk::asm::ArgSpec::ThisField { name: "p", desc: "[B" },
                    cplug_sdk::asm::ArgSpec::ThisField { name: "xo", desc: "D" },
                    cplug_sdk::asm::ArgSpec::ThisField { name: "yo", desc: "D" },
                    cplug_sdk::asm::ArgSpec::ThisField { name: "zo", desc: "D" },
                    // raw coordinates: x, y, z, yScale, yMax
                    cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'D' },
                    cplug_sdk::asm::ArgSpec::Local { slot: 3, ty: b'D' },
                    cplug_sdk::asm::ArgSpec::Local { slot: 5, ty: b'D' },
                    cplug_sdk::asm::ArgSpec::Local { slot: 7, ty: b'D' },
                    cplug_sdk::asm::ArgSpec::Local { slot: 9, ty: b'D' },
                ],
            };
            cplug_sdk::asm::replace_body(env, loader as jni::jobject, &original, &spec)
        })
        .flatten();
        let Some(patched) = patched else {
            eprintln!("[crussty-plugin] improved_noise: patch computation failed, hook stays dormant");
            return;
        };
        let (patched, retarget_note) = maybe_batch_retarget(patched);
        if let Some(note) = retarget_note {
            eprintln!("{note}");
        }
        // TASK-26/C5: parse the patched class's version ONCE here (quiet
        // activation worker) and cache it alongside the bytes — the serve
        // path never re-parses the header on the class-load thread.
        let patch_major = class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] improved_noise: computed patch for noise() ({} -> {} bytes), orig major {} patch major {}",
            original.len(),
            patched.len(),
            class_version(&original).map(|(m, _)| m).unwrap_or(0),
            patch_major
        );
        // poison-recovery: worker-side write (TASK-46) — a panic on the
        // activation worker must not poison the hook-callback serve path
        *patch_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(patched),
            major: patch_major,
        });

        // Phase 3: a SINGLE retransform; the callback serves the cached patch.
        // Kernel selection policy (src/kernel_policy.rs): from this moment
        // the hot path routes ImprovedNoise.noise() through the
        // PaperNativeImprovedNoise natives — assert (debug builds) and log
        // (audit mode) that the policy still allows them. The natives are
        // whitelisted as "live" in PROVEN_WINS, so this never fires unless a
        // future edit accidentally demotes them.
        debug_assert!(
            crate::kernel_policy::decide(NATIVE_BRIDGE, "nativeNoise").is_allowed(),
            "kernel policy refused the live improved_noise routing (nativeNoise no longer proven?)"
        );
        crate::kernel_policy::audit_wire(NATIVE_BRIDGE, "nativeNoise", "improved_noise hot-patch v2");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(NOISE_CLASS);
        eprintln!("[crussty-plugin] improved_noise: hook armed, retransform rc={rc}");
        // TASK-22/C1 acceptance: exactly one line per hook with the final
        // scans-avoided count (the poller has exited the loop by now, so
        // this is the final number for this hook's polling window).
        eprintln!(
            "[crussty-plugin] improved_noise: sighting feed: {} full class-heap scans avoided",
            cplug_sdk::classes::scans_avoided(NOISE_CLASS)
        );

        bridge_selftest();
    });
}

/// G4 demonstrator (docs/G4_SITE_PATCH_DESIGN.md §4-§5.1): arm the batch
/// site and — ONLY if the gate chain passes — retarget the bridge invokestatic
/// in the freshly computed ASM patch to the same-descriptor batching helper
/// (`ImprovedNoiseBatchOps.noise`). Runs on the quiet activation worker
/// (never a hook callback thread). Dormant by default: with the rollout gate
/// off, `site_arm` refuses silently and the served bytes are bit-identical
/// to the pre-G4 patch. Any retarget error is fail-safe: the PROVEN
/// unretargeted patch is kept (single-call ground state).
///
/// Returns the (possibly retargeted) bytes plus an optional marker line for
/// the caller to log (kept out of the hot function for readability).
fn maybe_batch_retarget(patched: Vec<u8>) -> (Vec<u8>, Option<String>) {
    let site = crate::batch_api::SiteSpec {
        site_class: NOISE_CLASS,
        site_method: "noise",
        // The kernel bridge the site routes through — G2 mask + audit_wire
        // target (mirrors the improved_noise kernel wiring below).
        kernel_class: NATIVE_BRIDGE,
        kernel_method: "nativeNoise",
        // No batch-table kernel exists for noise (BATCH_ADOPTION_MATRIX rows
        // 15/16: body-dominated LOW) — demonstrator site, marker id=none,
        // flush leg stays in the zero-op/single-call ground state.
        kernel_id: None,
        // B.3 T policy lane: no batch kernel → the default lane (16, clamped).
        default_t: crate::batch_api::threshold_for_kernel(None),
        tag: "improved_noise",
    };
    match crate::batch_api::site_arm(&site) {
        crate::batch_api::SiteArm::NotArmed => (patched, None),
        crate::batch_api::SiteArm::Armed { t } => {
            match crate::classfile::retarget_invokestatic(
                &patched,
                "noise",
                "(DDDDD)D",
                (BRIDGE_NAME, "noise", BRIDGE_NOISE_DESC),
                (BATCH_OPS_NAME, "noise", BRIDGE_NOISE_DESC),
            ) {
                Ok((new_bytes, crate::classfile::RetargetOutcome::Retargeted { sites })) => (
                    new_bytes,
                    Some(format!(
                        "[crussty-plugin] batch: site improved_noise retargeted {sites} call site(s) \
                         -> ImprovedNoiseBatchOps.noise (T={t}); flush leg = zero-op dispatcher round-trip, \
                         sampling stays bit-exact single-call (no noise batch kernel — LOW tier, honest demonstrator)"
                    )),
                ),
                Ok((_, other @ crate::classfile::RetargetOutcome::AlreadyPatched { sites })) => (
                    patched,
                    Some(format!(
                        "[crussty-plugin] batch: site improved_noise retarget skipped: already patched ({sites} site(s), {other:?})"
                    )),
                ),
                Ok((_, outcome @ crate::classfile::RetargetOutcome::NotFound)) => (
                    patched,
                    Some(format!(
                        "[crussty-plugin] batch: site improved_noise retarget skipped: bridge call site not found in the computed patch ({outcome:?}) — keeping the proven unretargeted patch"
                    )),
                ),
                Err(e) => {
                    // G4 S7-12 diagnostic: dump the failing bytes so the
                    // parse failure is analyzable offline (the dump is the
                    // ASM-patched class the JVM itself accepted for the
                    // serve path, so a parser gap — not corruption — is the
                    // working hypothesis).
                    let dump = "/tmp/crussty_g4_asm_patched.class";
                    let dump_note = match std::fs::write(dump, &patched) {
                        Ok(()) => format!(" (bytes dumped to {dump})"),
                        Err(w) => format!(" (dump to {dump} failed: {w})"),
                    };
                    (
                        patched,
                        Some(format!(
                            "[crussty-plugin] batch: site improved_noise retarget FAILED ({e}){dump_note} — keeping the proven unretargeted patch"
                        )),
                    )
                }
            }
        }
    }
}

/// Wait until the server has finished booting: `org/bukkit/Bukkit`'s static
/// `getServer()` returns a non-null CraftServer, with a short settling delay
/// after that so the kernel loader's boot-time class-loading storm has
/// fully quieted. Returns false on timeout (~120s).
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
                crate::clear_exception(env);
                return false;
            };
            let srv = env.call_static_object_method(bukkit.as_jclass(), get_server, &[]);
            let had_exc = crate::clear_exception(env);
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

fn force_load_kernel_class() {
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        let Some(seed) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
            eprintln!("[crussty-plugin] improved_noise: force load: Bukkit not found");
            return None::<()>;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return None::<()>;
        };
        let Some(loader) = env
            .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
            .and_then(|mid| {
                let l = env.call_object_method(seed.as_jclass(), mid, &[]);
                (l as usize != 0).then_some(l)
            })
        else {
            crate::clear_exception(env);
            env.delete_local_ref(class_cls);
            return None::<()>;
        };
        let Some(forname) = env.get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        ) else {
            crate::clear_exception(env);
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let dot = NOISE_CLASS.replace('/', ".");
        let Some(name) = env.new_string(&dot) else {
            crate::clear_exception(env);
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jni::jvalue { l: name },
                jni::jvalue { z: 1 /* true */ },
                jni::jvalue { l: loader },
            ],
        );
        let had_exc = crate::clear_exception(env);
        if loaded.is_null() {
            eprintln!(
                "[crussty-plugin] improved_noise: Class.forName({NOISE_CLASS}) failed (exc={had_exc})"
            );
        } else {
            eprintln!(
                "[crussty-plugin] improved_noise: Class.forName({NOISE_CLASS}) succeeded"
            );
        }
        env.delete_local_ref(loaded);
        env.delete_local_ref(name);
        env.delete_local_ref(class_cls);
        env.delete_local_ref(loader);
        Some(())
    });
}

/// Drive the real bridge through JNI over a synthetic 256-byte permutation:
/// buildHandle must return a nonzero handle, nativeNoise samples must be
/// finite and deterministic, freeHandle must cleanly release it.
fn bridge_selftest() {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let Some(bridge) = env.find_class(NATIVE_BRIDGE) else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] improved_noise: self-test: find_class({NATIVE_BRIDGE}) failed");
            return false;
        };
        let Some(build) = env.get_static_method_id(bridge, "nativeBuildHandle", "([BDDD)J") else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };
        let Some(noise_mid) = env.get_static_method_id(bridge, "nativeNoise", "(JDDDDD)D") else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };
        let Some(free) = env.get_static_method_id(bridge, "nativeFreeHandle", "(J)V") else {
            crate::clear_exception(env);
            env.delete_local_ref(bridge);
            return false;
        };

        let mut perm = [0u8; 256];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = (i as u8).wrapping_mul(31).wrapping_add(7);
        }
        let Some(arr) = env.new_byte_array(256) else {
            env.delete_local_ref(bridge);
            return false;
        };
        let signed: Vec<i8> = perm.iter().map(|&b| b as i8).collect();
        env.set_byte_array_region(arr, 0, 256, &signed);

        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let jenv = raw;
            let call_long = fn_table.CallStaticLongMethodA;
            let call_double = fn_table.CallStaticDoubleMethodA;
            let call_void = fn_table.CallStaticVoidMethodA;

            let args = [jni::jvalue { l: arr }, d(1.0), d(2.0), d(3.0), d(0.0), d(0.0)];
            let handle = (call_long)(jenv, bridge, build, args.as_ptr());
            if handle == 0 {
                eprintln!("[crussty-plugin] improved_noise: self-test: buildHandle returned 0");
                return false;
            }
            let pts: [(f64, f64, f64, f64, f64); 3] = [
                (1.5, 2.5, 3.5, 1.0, 0.0),
                (-7.25, 123.5, -0.125, 4.0, 100.0),
                (33.0, 5.75, -2.0, 0.0, 1.0),
            ];
            for (x, y, z, ys, ym) in pts {
                let args = [jlong_val(handle), d(x), d(y), d(z), d(ys), d(ym)];
                let v = (call_double)(jenv, bridge, noise_mid, args.as_ptr());
                if !v.is_finite() {
                    eprintln!(
                        "[crussty-plugin] improved_noise: self-test: non-finite sample at ({x},{y},{z})"
                    );
                    return false;
                }
            }
            let args = [jlong_val(handle)];
            (call_void)(jenv, bridge, free, args.as_ptr());
        }
        env.delete_local_ref(arr);
        env.delete_local_ref(bridge);
        true
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] improved_noise: self-test passed (native handle round-trip through real bridge)"
        ),
        _ => eprintln!("[crussty-plugin] improved_noise: self-test failed/skipped"),
    }
    // G4 (design §6 risk 6): the batching helper must be proven live BEFORE
    // real traffic — drive its flush leg once through the REAL batch bridge
    // (abiVersion gate + zero-op run()) from the KERNEL loader.
    batch_helper_selftest();
}

/// Drive `ImprovedNoiseBatchOps.selfTestFlush()` (the demonstrator's flush
/// machinery: abiVersion() + zero-op run() through the live bridge) once and
/// report. Resolves the helper through its DEFINE-TIME global ref
/// (BATCH_OPS_GREF) — a Class.forName round-trip through the loader proved
/// unreliable live (S7-12 dormant boot) and is deliberately not used. Every
/// failure path is LOUD (describe_exception): a silent skip here cost a
/// debugging cycle already.
fn batch_helper_selftest() {
    let gref = BATCH_OPS_GREF.load(Ordering::SeqCst);
    if gref == 0 {
        eprintln!(
            "[crussty-plugin] batch: helper self-test skipped (ImprovedNoiseBatchOps not defined)"
        );
        return;
    }
    let rc = cplug_sdk::jni_util::with_attached(|env| {
        let cls = gref as jni::jclass;
        let Some(selftest_mid) = env.get_static_method_id(cls, "selfTestFlush", "()I") else {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] batch: helper self-test: getStaticMethodID(selfTestFlush()I) failed"
            );
            return None::<i32>;
        };
        // A static call initializes the class on first use (JVM contract),
        // which also proves the helper links (its referenced bridge
        // signatures resolve inside the kernel loader).
        let rc = env.call_static_int_method(cls, selftest_mid, &[]);
        if crate::clear_exception(env) {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] batch: helper self-test: static call left a pending exception (cleared; see trace above)"
            );
            return None::<i32>;
        }
        Some(rc)
    });
    match rc.flatten() {
        Some(r) if r == crate::batch_api::ABI_WORD => eprintln!(
            "[crussty-plugin] batch: helper self-test passed (ImprovedNoiseBatchOps flush round-trip = abi {r})"
        ),
        Some(r) => eprintln!(
            "[crussty-plugin] batch: helper self-test DIAGNOSTIC rc={r} (flush degraded or bridge unavailable — single-call ground state holds)"
        ),
        None => eprintln!(
            "[crussty-plugin] batch: helper self-test failed (see exception trace above)"
        ),
    }
}

fn d(v: f64) -> jni::jvalue {
    jni::jvalue { d: v }
}

fn jlong_val(v: i64) -> jni::jvalue {
    jni::jvalue { j: v }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// TASK-46 poison-recovery: ORIG_BYTES / PATCH_CACHE are locked from the
    /// ClassFileLoadHook callback (pristine capture + patch serve) — a panic
    /// while either is locked must not turn the next callback invocation
    /// into an unwind-across-JNI VM abort. After deliberate poison, the
    /// exact production access patterns must recover with data intact.
    #[test]
    fn poisoned_orig_and_patch_locks_recovered() {
        // Seed PATCH_CACHE so the serve path has data that must survive the
        // poison (Arc payload is immutable; poison never corrupts it).
        *patch_lock().lock().unwrap() = Some(PatchCache {
            bytes: Arc::from(&b"patched-class-bytes"[..]),
            major: 65,
        });
        // Poison both mutexes: panic while each guard is held in a thread.
        for lock in [0usize, 1] {
            let _ = std::thread::spawn(move || match lock {
                0 => {
                    let _g = orig_lock().lock().unwrap();
                    panic!("deliberate: poison ORIG_BYTES");
                }
                _ => {
                    let _g = patch_lock().lock().unwrap();
                    panic!("deliberate: poison PATCH_CACHE");
                }
            })
            .join();
        }
        // 1) Pristine-capture site (hook callback): recover + write through.
        let mut orig = orig_lock().lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(b"orig-class-bytes".to_vec());
        }
        drop(orig);
        // 2) Serve site (hook callback): recovered Arc clone keeps the data.
        let cached = patch_lock()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let cached = cached.expect("patch data must survive poison");
        assert_eq!(&*cached.bytes, b"patched-class-bytes");
        assert_eq!(cached.major, 65);
        // 3) Worker write site: plain assignment through the recovered lock.
        *patch_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(&b"new-patch"[..]),
            major: 66,
        });
        assert_eq!(
            patch_lock()
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .as_ref()
                .map(|c| c.major),
            Some(66)
        );
        // 4) is_none() probe + clone (worker capture path) on recovered ORIG.
        assert!(!orig_lock()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_none());
        assert_eq!(
            orig_lock()
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone()
                .as_deref(),
            Some(&b"orig-class-bytes"[..])
        );
    }
}
