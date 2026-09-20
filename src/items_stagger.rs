//! Runtime wiring for the `items_stagger` lever (TASK-395 mega-round, agent B).
//!
//! Whole-body replacement of `ItemEntity.tick()V` — the measured TOP-1 lane
//! of the baseline profile (run 35528326290: 36051/115655 samples = 31.17%
//! of java-CPU) — with an invokestatic bridge into `ItemStaggerOps.tick`, a
//! PURE-JAVA faithful mirror of the Purpur 1.21.10 `ItemEntity.tick` +
//! `Entity.tick` bytecode (javap-verified block by block) carrying exactly
//! three stagger deltas and a merge candidate-cache:
//!
//!   1. rest/move interval (tickCount+id)%4 -> %8 for resting onGround items
//!      (same vanilla gate shape, longer period, deterministic id offset);
//!   2. on rest-skip ticks two vanilla per-tick checks are deferred to the
//!      next move-tick: noPhysics recheck + moveTowardsClosestSpace, and the
//!      second updateInWaterStateAndDoFluidPushing scan (baseTick already
//!      updated the fluid state this very tick);
//!   3. merge scan tickCount % (posChanged ? 2 : 40) -> (tickCount+id) %
//!      (posChanged ? 8 : 40) + candidate-cache of the last merge partner
//!      (alive/level/AABB/wall re-validated, no section scan) — the ≤8-tick
//!      deferral is documented in LEVER.md / RESEARCH-B.md.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG` == "items_stagger" (exact). Off by default
//! — dormant-invisible discipline (fluid_guard pattern): with the gate off
//! register() logs a dormant notice and NO byte hook is registered, activate()
//! returns immediately, and the module is byte-indistinguishable from the
//! pre-TASK-395 plugin (vanilla tick path, empty-flag parity by construction).
//!
//! Delivery pattern: FluidPushGuardHook / TASK-80 (byte-hook pristine
//! capture + replace_body retransform). The bridge references ItemEntity
//! fields/members, so it is defined into the KERNEL loader, package
//! net.minecraft.world.entity (FluidPushGuardHook package; the VarHandle
//! privateLookupIn for ItemEntity.despawnRate / Entity.despawnTime needs the
//! kernel loader). Three embedded classes: ItemStaggerOps + $MergePredicate
//! + $Cand (nested classes resolve through the defining loader).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

pub const TARGET_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const HOOK_NAME: &str = "net/minecraft/world/entity/ItemStaggerOps";
const HOOK_PREDICATE_NAME: &str = "net/minecraft/world/entity/ItemStaggerOps$MergePredicate";
const HOOK_CAND_NAME: &str = "net/minecraft/world/entity/ItemStaggerOps$Cand";

const HOOK_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemStaggerOps.class");
const HOOK_PREDICATE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemStaggerOps$MergePredicate.class");
const HOOK_CAND_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemStaggerOps$Cand.class");

/// Whole-body replacement spec: the kernel method's single argument slot
/// (this) passes straight through to the static bridge — receiver-prepended
/// G-BODY-proven shape (no field reads in the generated body).
const BRIDGE_TICK_DESC: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";
const METHOD_NAME: &str = "tick";
const METHOD_DESC: &str = "()V";

/// env-gate (off by default), read once at register time: EXACT equality per
/// the lever contract — an empty/other flag is the vanilla path.
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "items_stagger")
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel ItemEntity classloader, captured at activation.
/// 0 = not captured yet.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref to the defined bridge class (self-test driver). The JNI
/// FindClass from a native attachment resolves via the SYSTEM loader and
/// cannot see classes defined into the kernel loader's runtime package
/// (TASK-80 lesson, see fluid_guard).
static HOOK_CLASS: AtomicUsize = AtomicUsize::new(0);

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
            "[crussty-plugin] items_stagger: dormant (CRUSSTY_LEVER_FLAG != items_stagger, vanilla ItemEntity.tick)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] items_stagger: pristine sighting {} bytes (major {})",
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
                "[crussty-plugin] items_stagger: hook serve {} bytes (major {})",
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
            if cplug_sdk::classes::find_class(TARGET_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] items_stagger: {TARGET_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                && (forced_attempts < 12 || forced_attempts.is_multiple_of(12))
            {
                forced_attempts += 1;
                eprintln!(
                    "[crussty-plugin] items_stagger: forcing kernel load of {TARGET_CLASS} (attempt {forced_attempts})"
                );
                crate::improved_noise::force_load_kernel_class(TARGET_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(TARGET_CLASS);
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
                "[crussty-plugin] items_stagger: boot marker not seen, hook stays dormant"
            );
            return;
        }
        // TASK-80 crash lesson (fluid_guard): settle 20 s after the boot
        // marker before any define/retransform — replicate the empirically
        // stable window.
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] items_stagger: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let max_major = cplug_sdk::jni_util::with_attached(crate::improved_noise::jvm_max_class_major).flatten();
        if let Some(max) = max_major {
            for (name, bytes) in HOOK_EMBEDS {
                let bm = crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0);
                if bm > max {
                    eprintln!(
                        "[crussty-plugin] items_stagger: embedded bridge {name} is class major {bm} but this JVM supports up to {max} — rebuild entityinside/ ItemStaggerOps.java; hook stays dormant"
                    );
                    return;
                }
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let jvm_major = crate::improved_noise::jvm_class_major(env).unwrap_or(u16::MAX);
            for (name, bytes) in HOOK_EMBEDS {
                if let Some((major, _)) = crate::improved_noise::class_version(bytes) {
                    if major > jvm_major {
                        eprintln!(
                            "[crussty-plugin] items_stagger: {name} is class-file major {major} but this JVM supports up to {jvm_major} — hook stays dormant"
                        );
                        return false;
                    }
                }
            }
            let Some(cls) = cplug_sdk::classes::find_class(TARGET_CLASS) else {
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
            for (name, bytes) in HOOK_EMBEDS {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        if name == HOOK_NAME {
                            // Promote to a GLOBAL ref BEFORE the local ref is
                            // deleted — a stored local ref would dangle after
                            // delete_local_ref (frame-local handle table).
                            let g = env.new_global_ref(c);
                            HOOK_CLASS.store(g as usize, Ordering::SeqCst);
                        }
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] items_stagger: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] items_stagger: define_class({name}) failed");
                        ok = false;
                    }
                }
            }
            // Pre-define the ASM helper into the kernel loader too, so the
            // byte hook never performs a define_class mid-retransform.
            if cplug_sdk::asm::ensure_defined(env, gref).is_none() {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] items_stagger: asm helper define failed");
                ok = false;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] items_stagger: bridge definition aborted (no env or loader)");
            return;
        }

        // The ORIGINAL bytes were captured by the byte hook during the
        // class's own load. If the kernel loaded the class before this
        // plugin registered (possible on fast boots), a no-op retransform
        // delivers the current bytes through the hook (READY=false -> stash
        // only), with the resource stream as the last-resort fallback.
        if orig_lock().lock().unwrap_or_else(PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] items_stagger: class predates hook, capturing current bytes via no-op retransform"
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(TARGET_CLASS);
                eprintln!(
                    "[crussty-plugin] items_stagger: capture retransform rc={rc} (attempt {attempt})"
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
                    "[crussty-plugin] items_stagger: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                let stream_bytes = resource_stream_capture();
                match stream_bytes {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] items_stagger: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] items_stagger: resource-stream capture failed too, hook stays dormant"
                        );
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] items_stagger: no original bytes captured even after retransform, hook stays dormant"
            );
            return;
        };

        // Phase 2: compute the whole-body patch on the quiet thread.
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] items_stagger: no kernel loader captured");
            return;
        }
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            let spec = cplug_sdk::asm::ReplaceBody {
                method_name: METHOD_NAME,
                method_desc: METHOD_DESC,
                bridge_owner: HOOK_NAME,
                bridge_name: METHOD_NAME,
                bridge_desc: BRIDGE_TICK_DESC,
                args: &[
                    cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }, // this
                ],
            };
            cplug_sdk::asm::replace_body(env, loader as jni::jobject, &original, &spec)
        })
        .flatten();
        let Some(patched) = patched else {
            eprintln!("[crussty-plugin] items_stagger: patch computation failed, hook stays dormant");
            return;
        };
        let patch_major = crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] items_stagger: computed patch for {TARGET_CLASS}.{METHOD_NAME}{METHOD_DESC} ({} -> {} bytes), orig major {} patch major {}",
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
        // patch. kernel_policy: audit (log) the new pairing without
        // asserting proven-ness; the live A/B evidence feeds the
        // PROVEN_WINS/whitelist update after the run.
        crate::kernel_policy::audit_wire(HOOK_NAME, METHOD_NAME, "items_stagger whole-body v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(TARGET_CLASS);
        eprintln!("[crussty-plugin] items_stagger: hook armed, retransform rc={rc}");

        bridge_selftest();
    });
}

const HOOK_EMBEDS: [(&str, &[u8]); 3] = [
    (HOOK_NAME, HOOK_BYTES),
    (HOOK_PREDICATE_NAME, HOOK_PREDICATE_BYTES),
    (HOOK_CAND_NAME, HOOK_CAND_BYTES),
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
        let res_name = env.new_string_utf(&format!("{TARGET_CLASS}.class"))?;
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

/// Drive the bridge's reflective self-test: the VarHandle pair
/// (ItemEntity.despawnRate, Entity.despawnTime) must be operational in the
/// kernel loader (the whole-body mirror reads both on the hot path). No
/// synthetic ItemEntity is constructed.
fn bridge_selftest() {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        // Use the stored GLOBAL ref captured at define_class — JNI FindClass
        // from a native attachment resolves via the system loader and cannot
        // see runtime-package classes defined into the kernel loader (TASK-80
        // lesson, see fluid_guard.bridge_selftest).
        let stored = HOOK_CLASS.load(Ordering::SeqCst);
        if stored == 0 {
            eprintln!("[crussty-plugin] items_stagger: self-test: no stored hook class ref");
            return false;
        }
        let hook = stored as jni::jclass;
        let Some(mid) = env.get_static_method_id(hook, "selfTest", "()Z") else {
            crate::clear_exception(env);
            return false;
        };
        // Static boolean call via the raw vtable (the SDK exposes no
        // static-boolean helper; same pattern as fluid_guard's self-test).
        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let call_bool = fn_table.CallStaticBooleanMethodA;
            (call_bool)(raw, hook, mid, [].as_ptr()) != 0
        }
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] items_stagger: self-test passed (despawnRate/despawnTime VarHandles operational in kernel loader)"
        ),
        _ => eprintln!("[crussty-plugin] items_stagger: self-test failed/skipped"),
    }
}
