//! Runtime wiring for the CHUNK6-SCHED lever (TASK-456-C, cmp456_chunkmono —
//! see chunksched/net/minecraft/server/level/ChunkSchedOps.java and
//! RESEARCH-456-C.md).
//!
//! Law 8 chunk/worldgen axis, scheduling mono-plane: the ServerChunkCache
//! scheduling slice is 4.6-5.2% wall on the vanilla anchors (getChunkNow
//! 0.6-0.8 self + ConcurrentLong2ReferenceChainedHashTable.getNode 1.2 shared
//! + JIT lambda 0.8-1.0 + off-main tails). The mono-plane feeds a direct-mapped
//! L1 shadow from the SINGLE fullChunks mutation point (moonrise$setFullChunk,
//! javap-verified) and serves the getChunkNow probe from it, skipping the
//! striped-table probe on hits; a Rust L2 key-mirror (chunk-granular JNI
//! events, parse/send precedent — ZERO per-entity JNI, law 6) counts drift and
//! fail-opens the fast-path on any anomaly.
//!
//! Composite fail-dominant: BOTH body-redirects (getChunkNow ->
//! ChunkSchedOps.getNow, moonrise$setFullChunk -> ChunkSchedOps.onSetFullChunk)
//! or none; arm-order = define -> natives -> selfTest -> BOTH-site patch ->
//! retransform -> arm() LAST (inside_snap lesson: no [ARMED .. patched) window).
//!
//! Parity contract (law 4): empty lever flag -> hooks never registered,
//! ServerChunkCache byte-identical vanilla; the Ops fallback body is the
//! javap-verbatim vanilla getChunkNow; stale-positive windows are impossible
//! by ordering (shadow put AFTER table put, shadow del BEFORE table del).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const TARGET_CLASS: &str = "net/minecraft/server/level/ServerChunkCache";
const OPS_CLASS: &str = "net/minecraft/server/level/ChunkSchedOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../chunksched/build/net/minecraft/server/level/ChunkSchedOps.class");

const LEVER_ID: &str = "cmp456_chunkmono";

/// MEGA-CARRIER gate (STRICT-OR, diet-fix 47ea8b20 canon): the lever carries
/// the master cert stack (ins4 ⊕ senseins ⊕ diet ⊕ chunk4 ⊕ chunk5 ⊕
/// chunkparse ⊕ noise-GEN) + this mono-plane. Every master carrier stays in
/// the list UNCHANGED; cmp456_chunkmono is the ONLY addition (mirror-drift
/// lesson ×452: prod gates and test helpers move synchronously).
fn lever_flag_matches() -> bool {
    // x466-C99: маска M_CHUNKSCHED (состав pinned lever::parity_chunksched_queryplane)
    // — было env::var + 24+-eq цепь с дублями на каждый вызов.
    crate::lever::armed(crate::lever::M_CHUNKSCHED)
}

static READY: AtomicBool = AtomicBool::new(false);

struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

static TARGET: OnceLock<Target> = OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target {
        orig: std::sync::Mutex::new(None),
        patch: std::sync::Mutex::new(None),
        served: AtomicBool::new(false),
    })
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise): the callback performs NO JNI work;
/// pristine capture at the class's own load, patch served from the cache.
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono: dormant (CRUSSTY_LEVER_FLAG mismatch, vanilla bit-in-byte)"
        );
        return;
    }
    let t = target();
    let _ = &t;
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: pristine sighting {TARGET_CLASS} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut o = t.orig.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if o.is_none() {
                *o = Some(bytes.to_vec());
            }
            return None;
        }
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: hook serve {TARGET_CLASS} patched image"
            );
        }
        t.patch
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|c| c.to_vec())
    });
}

/// Background activation: wait for the target class + boot, define the bridge
/// into the kernel loader, RegisterNatives, selfTest, compute the BOTH-or-none
/// patch from the pristine stash, flip READY, retransform once, arm() LAST.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(240);
        loop {
            if cplug_sdk::classes::find_class(TARGET_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp456_chunkmono: {TARGET_CLASS} not loaded within 240s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(30) {
                eprintln!(
                    "[crussty-plugin] cmp456_chunkmono: forcing kernel load of {TARGET_CLASS}"
                );
                crate::improved_noise::force_load_kernel_class_lazy(TARGET_CLASS);
            }
            std::thread::sleep(std::time::Duration::from_millis(5_000));
        }
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Class-version gate (lesson ×93): a stale blob must not arm.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild chunksched/; hook stays dormant"
            );
            return;
        }

        // Resolution closure: the bridge MUST declare the two receiver-prepended
        // retarget statics + the native names (NoSuchMethodError fail-closed).
        if let Err(e) = crate::classfile::chunk_sched_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the KERNEL loader + RegisterNatives.
        let Some(gops) = define_bridge() else {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: bridge definition failed, hook stays dormant"
            );
            return;
        };
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono: defined {OPS_CLASS} in kernel loader, natives registered"
        );

        // selfTest on the KEPT define_class ref (TASK-417-C find_class fix):
        // any Throwable => fail-closed dormant (never arm).
        let selftest = cplug_sdk::jni_util::with_attached(|env| selftest(env, gops))
            .unwrap_or(false);
        if !selftest {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: selfTest FAILED — hook stays dormant (fail-closed)"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono: selfTest=true BEFORE arm (schedProbe magic + shadow round-trip + native probe)"
        );

        // Patch from the pristine stash — BOTH sites or none.
        let Some(orig) = target()
            .orig
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
        else {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: no pristine bytes for {TARGET_CLASS} — DISARM (fail-closed)"
            );
            return;
        };
        match crate::classfile::patch_server_chunk_cache_sched(&orig) {
            Ok((patched, outcome)) => match outcome {
                crate::classfile::RetargetOutcome::Retargeted { sites } if sites == 2 => {
                    *target()
                        .patch
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner) =
                        Some(Arc::from(patched.as_slice()));
                    READY.store(true, Ordering::Release);
                    let rc = cplug_sdk::retransform_class(TARGET_CLASS);
                    eprintln!(
                        "[crussty-plugin] cmp456_chunkmono: PATCHED {TARGET_CLASS}.getChunkNow -> ChunkSchedOps.getNow + moonrise$setFullChunk -> ChunkSchedOps.onSetFullChunk (Retargeted {{ sites: {sites} }}; {} -> {} bytes; retransform rc={rc})",
                        orig.len(),
                        patched.len()
                    );
                }
                other => {
                    eprintln!(
                        "[crussty-plugin] cmp456_chunkmono: unexpected ServerChunkCache outcome ({other:?}) — DISARM (fail-closed)"
                    );
                    return;
                }
            },
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] cmp456_chunkmono: ServerChunkCache patch rejected ({e}) — DISARM (fail-closed)"
                );
                return;
            }
        }

        // Arm LAST (inside_snap arm-order lesson): the gate opens only after
        // the retransform is live — no [ARMED .. patched) window.
        let armed = cplug_sdk::jni_util::with_attached(|env| call_arm(env, gops)).unwrap_or(false);
        if !armed {
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: java arm() failed — gate stays vanilla (fail-closed)"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp456_chunkmono: ARMED (chunk-sched mono-plane live; scheduling slice 4.6-5.2% in scope)"
        );
    });
}

fn define_bridge() -> Option<*mut std::ffi::c_void> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(TARGET_CLASS) else {
            return None;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return None;
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
            return None;
        };
        let gref = env.new_global_ref(loader);
        if gref.is_null() {
            crate::describe_exception(env);
            env.delete_local_ref(class_cls);
            return None;
        }
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: define_class({OPS_CLASS}) failed"
            );
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        };
        let names = [
            std::ffi::CString::new("schedProbe").expect("no NUL"),
            std::ffi::CString::new("mirrorEvent").expect("no NUL"),
        ];
        let sigs = [
            std::ffi::CString::new("()J").expect("no NUL"),
            std::ffi::CString::new("(JZ)Z").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: sched_probe as *const std::ffi::c_void as *mut std::ffi::c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: mirror_event as *const std::ffi::c_void as *mut std::ffi::c_void,
            },
        ];
        let reg = env.register_natives(c, &natives);
        if let Err(code) = reg {
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] cmp456_chunkmono: register_natives failed (code {code}) — hook stays dormant"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return None;
        }
        // Keep the ops class alive (global ref) — selfTest/arm resolve via it
        // (TASK-417-C find_class fix: later finds from native context would
        // resolve through the SYSTEM loader and miss the kernel definition).
        let gops = env.new_global_ref(c);
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        if gops.is_null() {
            crate::describe_exception(env);
            return None;
        }
        Some(gops as *mut std::ffi::c_void)
    })
    .flatten()
}

fn selftest(env: &jvmti_bindings::env::JniEnv, gops: *mut std::ffi::c_void) -> bool {
    let cls = gops as jvmti_bindings::jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] cmp456_chunkmono: selfTest resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!("[crussty-plugin] cmp456_chunkmono: selfTest threw — fail-closed");
        return false;
    }
    rc != 0
}

fn call_arm(env: &jvmti_bindings::env::JniEnv, gops: *mut std::ffi::c_void) -> bool {
    let cls = gops as jvmti_bindings::jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "arm", "()V") else {
        crate::clear_exception(env);
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    !had_exc
}

// ---------------------------------------------------------------------------
// NATIVES (raw fn-table discipline — colpush/inside_snap precedent)
// ---------------------------------------------------------------------------

const PROBE_MAGIC: i64 = 0x456C_456C;

/// Rust L2 key-mirror: chunk-granular events only (setFullChunk put/remove).
/// Open-addressed u64 set; DRIFT counts dup-add / miss-del anomalies; any
/// drift fail-opens the java fast-path via ChunkSchedOps.noteDrift.
struct Mirror {
    slots: Vec<u64>,
    used: usize,
}
static MIRROR: OnceLock<Mutex<Mirror>> = OnceLock::new();
static MIRROR_DRIFT: AtomicU64 = AtomicU64::new(0);
static MIRROR_SIZE: AtomicU64 = AtomicU64::new(0);
const MIRROR_CAP: usize = 1 << 17;

fn mirror() -> &'static Mutex<Mirror> {
    MIRROR.get_or_init(|| {
        Mutex::new(Mirror {
            slots: vec![0u64; MIRROR_CAP],
            used: 0,
        })
    })
}

fn mirror_slot(key: u64) -> usize {
    let mut h = (key as u64) ^ (key >> 27);
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    (h >> 24) as usize & (MIRROR_CAP - 1)
}

/// # Safety
/// Called by the JVM through RegisterNatives.
#[no_mangle]
pub unsafe extern "system" fn sched_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jlong {
    PROBE_MAGIC
}

/// # Safety
/// Called by the JVM through RegisterNatives.
#[no_mangle]
pub unsafe extern "system" fn mirror_event(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    key: jni::jlong,
    add: jni::jboolean,
) -> jni::jboolean {
    let key = key as u64;
    let mut m = match mirror().lock() {
        Ok(m) => m,
        Err(p) => p.into_inner(),
    };
    let k = if key == 0 { u64::MAX } else { key };
    let start = mirror_slot(k);
    let mut slot = start;
    let mut found = false;
    for _ in 0..MIRROR_CAP {
        let cur = m.slots[slot];
        if cur == k {
            found = true;
            break;
        }
        if cur == 0 {
            break;
        }
        slot = (slot + 1) & (MIRROR_CAP - 1);
    }
    let mut drift = false;
    if add != 0 {
        if found {
            drift = true;
        } else {
            let mut s = start;
            for _ in 0..MIRROR_CAP {
                if m.slots[s] == 0 {
                    m.slots[s] = k;
                    m.used += 1;
                    MIRROR_SIZE.store(m.used as u64, Ordering::Release);
                    break;
                }
                s = (s + 1) & (MIRROR_CAP - 1);
            }
        }
    } else if found {
        m.slots[slot] = 0;
        m.used = m.used.saturating_sub(1);
        MIRROR_SIZE.store(m.used as u64, Ordering::Release);
    } else {
        drift = true;
        MIRROR_DRIFT.fetch_add(1, Ordering::AcqRel);
    }
    drop(m);
    if drift {
        MIRROR_DRIFT.fetch_add(1, Ordering::AcqRel);
    }
    drift as jni::jboolean
}

// ---------------------------------------------------------------------------
// DELIVERY TESTS (zero_cursor / traveldiet canon — mirror-drift lesson ×452:
// prod gates and test helpers move synchronously; offline catch of the
// x451/s7171 dormant-on-CI class of defects)
// ---------------------------------------------------------------------------
#[cfg(test)]
mod chunksched_delivery_tests {
    use super::{OPS_BYTES, TARGET_CLASS};

    /// The bridge source MUST declare ZERO nested classes (kernel-loader
    /// define, colpush NCDFE lesson: a nested class would detonate as
    /// NoClassDefFoundError on the first getChunkNow call).
    #[test]
    fn chunksched_source_declares_no_nested_classes() {
        let src = include_str!("../chunksched/net/minecraft/server/level/ChunkSchedOps.java");
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if (before.contains("static") || before.contains("private"))
                        && !before.contains("//")
                    {
                        panic!("nested declaration in bridge source: {t}");
                    }
                }
            }
        }
    }

    /// Embedded bytes must exist and be a real classfile pinned to the kernel
    /// JVM major (65); the build-step javap descriptor pins live in
    /// scripts/build_chunksched_ops.sh + scripts/check_blobs_sync.sh.
    #[test]
    fn chunksched_embedded_classfile_present_and_pinned() {
        let bytes = include_bytes!("../chunksched/build/net/minecraft/server/level/ChunkSchedOps.class");
        assert_eq!(&bytes[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([bytes[6], bytes[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
        let _ = TARGET_CLASS;
    }

    /// s7171 delivery-defect guard: the resolution closure MUST accept the
    /// embedded bridge classfile as compiled (the (JZ)V vs (JZ)Z native
    /// signature drift is exactly the failure mode this pins shut — the hook
    /// would stay dormant and the leg would silently run vanilla).
    #[test]
    fn chunksched_resolution_closure_accepts_embedded_bridge() {
        crate::classfile::chunk_sched_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded ChunkSchedOps bridge bytes");
    }

    /// BOTH-or-none patch contract: the pristine-kernel-shaped redirect MUST
    /// find exactly the two sites (getChunkNow + moonrise$setFullChunk) in the
    /// embedded blob's OWN target surface — pinned offline so a kernel drift
    /// fails here instead of DISARMing on the runner.
    #[test]
    fn chunksched_redirect_targets_pinned() {
        assert_eq!(crate::classfile::CHUNKSCHED_REDIRECT_TARGETS.len(), 2);
        assert_eq!(
            crate::classfile::CHUNKSCHED_REDIRECT_TARGETS[0].0,
            "getChunkNow"
        );
        assert_eq!(
            crate::classfile::CHUNKSCHED_REDIRECT_TARGETS[1].0,
            "moonrise$setFullChunk"
        );
    }
}
