//! Runtime wiring for the MOB-PUSH lever (TASK-400-J, vector mobpush —
//! lever cmp399_mobpush; bridge mobpush/net/minecraft/world/entity/MobPushOps.java,
//! grid src/mobs_grid.rs, retarget in src/classfile.rs patch_push_entities_mob).
//!
//! ARCHITECTURE: the mob push lane (`LivingEntity.pushEntities` →
//! `Level.getPushableEntities` → moonrise whole-16³-section scan) is
//! re-broadphased onto a dedicated sharded seqlock grid of 1.0-cells
//! (pattern-B port from items_index cmp399_shard; SEPARATE instance — the
//! item-shard mode is NOT activated by this flag, items stay vanilla → clean
//! A/B on the push lane). Positions are pushed to rust per-tick by the bridge
//! itself (self-upsert at query time = end-of-move position, see MobPushOps
//! javadoc); candidates come back to java in ONE native query and are
//! re-validated exactly (level + AABB + vanilla EntitySelector.pushableBy).
//! The vanilla tail of pushEntities (cramming/numCollisions/doPush) is
//! untouched bytecode — upstream Paper «optimize entity pushes» port.
//!
//! DELIVERY (alloc_diet pattern): byte hook on LivingEntity captures pristine
//! bytes at first load; the activation worker defines MobPushOps into the
//! kernel loader + RegisterNatives (mobProbe/mobUpsert/mobRemove/mobQuery),
//! computes the length-preserving single-site retarget
//! getPushableEntities→MobPushOps.pushables, flips READY and retransforms
//! LivingEntity once.
//!
//! Fail-closed: flag != "cmp399_mobpush" → no hook installed at all (byte-
//! indistinguishable from the pre-TASK-400-J plugin); define/registration/
//! patch failure → READY stays false → vanilla; native error codes → the
//! Java bridge falls back per-call (ERR_RANGE) or disarms (ERR_STRUCT).

use std::ffi::c_void;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/MobPushOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../mobpush/build/net/minecraft/world/entity/MobPushOps.class");

/// Java-side gate baked into OPS_BYTES (<clinit>: ENABLED =
/// "cmp399_mobpush".equals(trim(getenv))). The exact-match lever keeps the
/// item-shard family (cmp399_shard) and the item subsystem off — the mobs
/// grid is a separate instance.
const GATE_LEVER: &str = "cmp399_mobpush";

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

static READY: AtomicBool = AtomicBool::new(false);

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Per-class state: pristine capture + computed patch + serve log flag.
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
}

impl Target {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        if let Ok(mut orig) = self.orig.lock() {
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig
            .lock()
            .map(|o| o.is_some())
            .unwrap_or(false)
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .ok()
            .and_then(|mut o| o.take())
    }
    fn set_patch(&self, cache: PatchCache) {
        if let Ok(mut p) = self.patch.lock() {
            *p = Some(cache);
        }
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .ok()
            .and_then(|p| p.as_ref().map(|c| Arc::clone(&c.bytes)))
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(LIVING_CLASS))
}

/// Register the byte hook (call once from cplugin_init). Dormant-invisible:
/// with the lever flag unset/mismatched NOTHING is registered — the plugin
/// stays byte-indistinguishable from vanilla for this vector.
pub fn register() {
    if lever_flag() != GATE_LEVER {
        eprintln!(
            "[crussty-plugin] mobs_grid: dormant (set CRUSSTY_LEVER_FLAG={GATE_LEVER} to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Acquire) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker; never rewrite here.
            eprintln!(
                "[crussty-plugin] mobs_grid: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] mobs_grid: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the kernel class + boot quiet, define the
/// MobPushOps bridge into the kernel loader, RegisterNatives, compute the
/// single-site retarget from the pristine bytes, flip READY, retransform.
pub fn activate() {
    if lever_flag() != GATE_LEVER {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // LivingEntity loads at boot (entity superclass); wait it out.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(t.name).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] mobs_grid: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mobs_grid: boot marker not seen, hook stays dormant");
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
                "[crussty-plugin] mobs_grid: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild mobpush/ via scripts/build_mobpush_ops.sh; hook stays dormant"
            );
            return;
        }

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] mobs_grid: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _ in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] mobs_grid: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(t.name) else {
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
                eprintln!("[crussty-plugin] mobs_grid: define_class({OPS_CLASS}) failed");
                return false;
            };

            // RegisterNatives: mobProbe/mobUpsert/mobRemove/mobQuery
            // (impl — src/mobs_grid.rs). Провал регистрации → armed()=false
            // (probeOnce не пройдёт magic) → ванильный путь.
            let names = [
                CString::new("mobProbe").expect("no NUL"),
                CString::new("mobUpsert").expect("no NUL"),
                CString::new("mobRemove").expect("no NUL"),
                CString::new("mobQuery").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(IIDDD)I").expect("no NUL"),
                CString::new("(I)I").expect("no NUL"),
                CString::new("(DDDDDDI[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_upsert as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_remove as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[3].as_ptr(),
                    signature: sigs[3].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_query as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] mobs_grid: register_natives failed (code {code}) — hook stays dormant"
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
            eprintln!(
                "[crussty-plugin] mobs_grid: bridge definition failed, hook stays dormant"
            );
            return;
        }

        // Compute the single-site retarget from the pristine bytes (pure
        // rust, length-preserving). Any Err = kernel shape mismatch → fail
        // closed (lever stays vanilla).
        let Some(original) = t.take_orig() else {
            eprintln!("[crussty-plugin] mobs_grid: pristine bytes vanished, hook stays dormant");
            return;
        };
        let major = crate::improved_noise::class_version(&original)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let (patched, outcome) = match crate::classfile::patch_push_entities_mob(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] mobs_grid: pushEntities retarget rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let retargeted = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
        );
        if !retargeted {
            eprintln!(
                "[crussty-plugin] mobs_grid: unexpected patch outcome for {} ({outcome:?}), hook stays dormant",
                t.name
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] mobs_grid: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(PatchCache {
            bytes: Arc::from(patched),
            major,
        });

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp399_mobpush: ARMED shards=64 seqlock-reads=per-cell-version writer=global-mutex shard_cap=16384 max_ids=1048576 cell=1.0 pad=1.0 radius_gate=1.0 (rust mobs_grid sharded; pushEntities tail untouched vanilla; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
        );

        // Single retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_CLASS, "pushables", "mobs_grid v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] mobs_grid: {} armed, retransform rc={rc}", t.name);
    });
}
