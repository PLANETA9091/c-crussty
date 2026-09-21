//! Runtime wiring for the `cmp401_ossport` lever (TASK-401-F, vector mob-ossport).
//!
//! Ports the Lithium `unpushable_cramming` push-lane mechanism (CaffeineMC/lithium,
//! develop branch — see /home/z/rounds/ROUND-401/RESEARCH-F.md) onto the measured
//! TOP-1 bottleneck: getEntities 9.45% java — the mob-side per-entity AABB scans.
//! The push lane (LivingEntity.pushEntities -> Level.getPushableEntities ->
//! Level.getEntities) is the single largest getEntities caller in the 150k
//! population profiles (round-jmob2/round-footprint collapsed stacks).
//!
//! Shape (item_merge precedent, TASK-396-H): whole-body replacement of
//! `Level.getPushableEntities(Entity, AABB)` with a single invokestatic into
//! `MobOssOps.getPushableEntities(Level, Entity, AABB)` — a PURE-JAVA bridge
//! defined into the kernel loader (same package family as Level). The bridge
//! reproduces the vanilla body (same EntitySelector.pushableBy predicate, same
//! EntityLookup walk, same profiler counter, same PlatformHooks.addToGetEntities
//! tail) and swaps ONLY the container/indirection: upstream alwaysFalse
//! fast-fail + reused per-thread collect list + direct storage walk, with the
//! exact-vanilla fallback on any unmet precondition.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp401_ossport"` (round-396 lever protocol).
//! Off by default — dormant-invisible discipline: with the gate off no byte
//! hook is registered, nothing is defined or retransformed.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

pub const ITEM_CLASS: &str = "net/minecraft/world/level/Level";
const OPS_NAME: &str = "net/minecraft/world/entity/MobOssOps";
const METHOD_NAME: &str = "getPushableEntities";
const METHOD_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";

const OPS_BYTES: &[u8] =
    include_bytes!("../mob/build/net/minecraft/world/entity/MobOssOps.class");

/// Whole-body replacement spec: (this=Level, Entity, AABB) -> List.
const BRIDGE_DESC: &str =
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";

/// env gate per the round-396 lever protocol (NOT a CRUSSTY_ITEMS_OSS flag).
fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok("cmp401_ossport"))
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel Level classloader, captured at activation.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref to the defined bridge class (self-test driver). JNI FindClass
/// from a native attachment resolves via the SYSTEM loader and cannot see
/// classes defined into the kernel loader (TASK-80 lesson).
static OPS_CLASS: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of the class (byte hook
/// while READY=false), else via no-op retransform / resource stream.
static ORIG_BYTES: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
    std::sync::OnceLock::new();

/// Patched bytecode cache (TASK-26/C5 pattern): Arc<[u8]> + major parsed once.
#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}
static PATCH_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>> =
    std::sync::OnceLock::new();
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn orig_lock() -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock() -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE.get_or_init(|| std::sync::Mutex::new(None))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// The callback performs NO JNI/class-file work (loader-lock discipline —
/// see improved_noise): pristine capture at the class's own load, patch
/// served from the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] mob_oss: dormant (lever_flag != cmp401_ossport, vanilla mob push)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(ITEM_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] mob_oss: pristine sighting {} bytes (major {})",
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
                "[crussty-plugin] mob_oss: hook serve {} bytes (major {})",
                cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                cached.as_ref().map(|c| c.major).unwrap_or(0)
            );
        }
        cached.map(|c| c.bytes.to_vec())
    });
}

/// Background activation: wait for ItemEntity, define ItemMergeOps into the
/// kernel loader, compute the whole-body patch, flip READY and retransform.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(ITEM_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] mob_oss: {ITEM_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                && (forced_attempts < 12 || forced_attempts.is_multiple_of(12))
            {
                forced_attempts += 1;
                eprintln!(
                    "[crussty-plugin] mob_oss: forcing kernel load of {ITEM_CLASS} (attempt {forced_attempts})"
                );
                crate::improved_noise::force_load_kernel_class(ITEM_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ITEM_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mob_oss: boot marker not seen, hook stays dormant");
            return;
        }
        // TASK-80 crash lesson: settle after Done before any define/retransform.
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] mob_oss: server booted, defining ItemMergeOps into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let max_major = cplug_sdk::jni_util::with_attached(
            crate::improved_noise::jvm_max_class_major,
        )
        .flatten();
        if let Some(max) = max_major {
            let bm = crate::improved_noise::class_version(OPS_BYTES)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if bm > max {
                eprintln!(
                    "[crussty-plugin] mob_oss: embedded bridge is class major {bm} but this JVM supports up to {max} — rebuild items/ via scripts/build_items_oss.sh; hook stays dormant"
                );
                return;
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let jvm_major = crate::improved_noise::jvm_class_major(env).unwrap_or(u16::MAX);
            if let Some((major, _)) = crate::improved_noise::class_version(OPS_BYTES) {
                if major > jvm_major {
                    eprintln!(
                        "[crussty-plugin] mob_oss: ItemMergeOps is class-file major {major} but this JVM supports up to {jvm_major} — hook stays dormant"
                    );
                    return false;
                }
            }
            let Some(cls) = cplug_sdk::classes::find_class(ITEM_CLASS) else {
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
            match env.define_class(OPS_NAME, gref, OPS_BYTES) {
                Some(c) => {
                    // Promote to a GLOBAL ref BEFORE the local ref is deleted
                    // (frame-local handle table would dangle).
                    let g = env.new_global_ref(c);
                    OPS_CLASS.store(g as usize, Ordering::SeqCst);
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] mob_oss: defined {OPS_NAME} in kernel loader"
                    );
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] mob_oss: define_class({OPS_NAME}) failed");
                    ok = false;
                }
            }
            // Pre-define the ASM helper into the kernel loader too, so the
            // byte hook never performs a define_class mid-retransform.
            if cplug_sdk::asm::ensure_defined(env, gref).is_none() {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] mob_oss: asm helper define failed");
                ok = false;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] mob_oss: bridge definition aborted (no env or loader)");
            return;
        }

        // Capture pristine bytes if the class predates the hook (fast boots).
        if orig_lock().lock().unwrap_or_else(PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] mob_oss: class predates hook, capturing current bytes via no-op retransform"
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(ITEM_CLASS);
                eprintln!(
                    "[crussty-plugin] mob_oss: capture retransform rc={rc} (attempt {attempt})"
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
                    "[crussty-plugin] mob_oss: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                match resource_stream_capture() {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] mob_oss: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] mob_oss: resource-stream capture failed too, hook stays dormant"
                        );
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] mob_oss: no original bytes captured even after retransform, hook stays dormant"
            );
            return;
        };

        // Phase 2: compute the whole-body patch on the quiet thread.
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] mob_oss: no kernel loader captured");
            return;
        }
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            let spec = cplug_sdk::asm::ReplaceBody {
                method_name: METHOD_NAME,
                method_desc: METHOD_DESC,
                bridge_owner: OPS_NAME,
                bridge_name: METHOD_NAME,
                bridge_desc: BRIDGE_DESC,
                args: &[
                cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
                cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'L' },
                cplug_sdk::asm::ArgSpec::Local { slot: 2, ty: b'L' },
            ],
            };
            cplug_sdk::asm::replace_body(env, loader as jvmti_bindings::jni::jobject, &original, &spec)
        })
        .flatten();
        let Some(patched) = patched else {
            eprintln!("[crussty-plugin] mob_oss: patch computation failed, hook stays dormant");
            return;
        };
        let patch_major =
            crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] mob_oss: computed patch for {METHOD_NAME}{METHOD_DESC} ({} -> {} bytes), orig major {} patch major {}",
            original.len(),
            patched.len(),
            crate::improved_noise::class_version(&original).map(|(m, _)| m).unwrap_or(0),
            patch_major
        );
        *patch_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(patched),
            major: patch_major,
        });

        // Phase 3: a SINGLE retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_NAME, METHOD_NAME, "mob_oss lithium cmp401_ossport v1");
        eprintln!(
            "[crussty-plugin] mob_oss: cmp401_ossport ARMED (Level.getPushableEntities -> MobOssOps, lithium unpushable_cramming adaptation)"
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(ITEM_CLASS);
        eprintln!("[crussty-plugin] mob_oss: hook armed, retransform rc={rc}");

        bridge_selftest();
    });
}

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
        let res_name = env.new_string_utf(&format!("{ITEM_CLASS}.class"))?;
        let stream = env.call_object_method(
            loader as jvmti_bindings::jni::jobject,
            garm,
            &[jvmti_bindings::jni::jvalue { l: res_name }],
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
            let jarr = arr as jvmti_bindings::jni::jbyteArray;
            let len = env.get_array_length(arr as jvmti_bindings::jni::jarray);
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

/// Drive the bridge's reflective self-test: the MethodHandle pair (private
/// isMergable/tryToMerge) must resolve in the kernel loader BEFORE the patch
/// serves — ItemMergeOps.selfTest() initializes and reports.
fn bridge_selftest() {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let stored = OPS_CLASS.load(Ordering::SeqCst);
        if stored == 0 {
            eprintln!("[crussty-plugin] mob_oss: self-test: no stored ops class ref");
            return false;
        }
        let ops = stored as jvmti_bindings::jni::jclass;
        let Some(mid) = env.get_static_method_id(ops, "selfTest", "()Z") else {
            crate::clear_exception(env);
            return false;
        };
        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let call_bool = fn_table.CallStaticBooleanMethodA;
            (call_bool)(raw, ops, mid, [].as_ptr()) != 0
        }
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] mob_oss: self-test passed (MethodHandles resolved in kernel loader)"
        ),
        _ => eprintln!("[crussty-plugin] mob_oss: self-test failed/skipped"),
    }
}
