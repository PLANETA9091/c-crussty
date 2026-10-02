//! Runtime wiring for the OFF-THREAD PREPARE-STAGE lever (TASK-401-D,
//! vector offthread — lever cmp401_offthread; bridge
//! prepare/net/minecraft/world/entity/PrepareOps.java, index
//! src/prepare_index.rs, retargets in src/classfile.rs:
//! patch_push_entities_offthread / patch_target_find /
//! patch_mob_aistep / patch_avoid_canuse).
//!
//! DELIVERY (mobs_manager pattern, four target classes): byte hooks capture
//! pristine bytes at first load; the activation worker waits for boot quiet,
//! defines PrepareOps into the kernel loader, RegisterNatives
//! (prepProbe/prepInit/prepBatch/prepQuery), computes the four single-site
//! retargets from pristine bytes, flips READY per-target and retransforms.
//! Per-target fail-closed: a shape mismatch on ONE target leaves that site
//! vanilla while the others still arm (each target logs its own outcome).
//!
//! The WRITE side (batch gather) lives in RegionTickOps.java itself —
//! recompiled in-repo with the PrepareOps gate baked in (env-check at
//! <clinit>, zero work when the lever flag differs).

use std::ffi::c_void;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const TARGET_GOAL_CLASS: &str =
    "net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal";
const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const AVOID_GOAL_CLASS: &str = "net/minecraft/world/entity/ai/goal/AvoidEntityGoal";
const OPS_CLASS: &str = "net/minecraft/world/entity/PrepareOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../prepare/build/net/minecraft/world/entity/PrepareOps.class");

/// Java-side gate baked into OPS_BYTES (<clinit>: ENABLED =
/// "cmp401_offthread".equals(trim(getenv))). Exact-match lever keeps the
/// item/other families off — clean A/B on the getEntities lanes.
const GATE_LEVER: &str = "cmp401_offthread";

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

static ACTIVATED: AtomicBool = AtomicBool::new(false);

struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
    ready: AtomicBool,
}

impl Target {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
            ready: AtomicBool::new(false),
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
        self.orig.lock().map(|o| o.is_some()).unwrap_or(false)
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig.lock().ok().and_then(|mut o| o.take())
    }
    fn set_patch(&self, bytes: Arc<[u8]>) {
        if let Ok(mut p) = self.patch.lock() {
            *p = Some(bytes);
        }
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch.lock().ok().and_then(|p| p.clone())
    }
}

static T_LIVING: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static T_TARGET_GOAL: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static T_MOB: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static T_AVOID_GOAL: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn targets() -> [&'static Target; 4] {
    [
        T_LIVING.get_or_init(|| Target::new(LIVING_CLASS)),
        T_TARGET_GOAL.get_or_init(|| Target::new(TARGET_GOAL_CLASS)),
        T_MOB.get_or_init(|| Target::new(MOB_CLASS)),
        T_AVOID_GOAL.get_or_init(|| Target::new(AVOID_GOAL_CLASS)),
    ]
}

/// Register the byte hooks (call once from cplugin_init). Dormant-invisible:
/// with the lever flag unset/mismatched NOTHING is registered.
pub fn register() {
    if lever_flag() != GATE_LEVER {
        eprintln!(
            "[crussty-plugin] prepare_index: dormant (set CRUSSTY_LEVER_FLAG={GATE_LEVER} to enable)"
        );
        return;
    }
    for t in targets() {
        cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
            if !ACTIVATED.load(Ordering::Acquire) {
                // Pristine sighting: stash for the worker; never rewrite here.
                t.stash_orig(bytes);
                return None;
            }
            let cached = t.patch_bytes();
            if !t.served.swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] prepare_index: hook serve {} ({} bytes)",
                    t.name,
                    cached.as_ref().map(|c| c.len()).unwrap_or(0)
                );
            }
            cached.map(|c| c.to_vec())
        });
    }
}

/// Per-target patch computation (pure rust, length-preserving retargets).
/// Returns the patched bytes for one target, or Err (fail-closed per target).
fn compute_patch(t: &Target, original: &[u8]) -> Result<Vec<u8>, String> {
    match t.name {
        LIVING_CLASS => crate::classfile::patch_push_entities_offthread(original).map(|(b, _)| b),
        TARGET_GOAL_CLASS => crate::classfile::patch_target_find(original).map(|(b, _)| b),
        MOB_CLASS => crate::classfile::patch_mob_aistep(original).map(|(b, _)| b),
        AVOID_GOAL_CLASS => crate::classfile::patch_avoid_canuse(original).map(|(b, _)| b),
        _ => Err("unknown target".into()),
    }
}

/// Background activation: wait for kernel classes + boot quiet, define the
/// bridge, RegisterNatives, compute per-target patches, flip READY, retransform.
pub fn activate() {
    if lever_flag() != GATE_LEVER {
        return;
    }
    std::thread::spawn(|| {
        let list = targets();
        // All four classes load at boot (entity AI surfaces).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            let missing = list.iter().any(|t| cplug_sdk::classes::find_class(t.name).is_none());
            if !missing {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] prepare_index: target classes not loaded within 180s, hooks stay dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] prepare_index: boot marker not seen, hooks stay dormant");
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
                "[crussty-plugin] prepare_index: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild prepare/ via javac; hooks stay dormant"
            );
            return;
        }

        // Pristine capture for any class that predates the hook.
        for t in list {
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] prepare_index: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] prepare_index: no pristine bytes for {}, site stays vanilla",
                        t.name
                    );
                }
            }
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(living) = cplug_sdk::classes::find_class(LIVING_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(living.as_jclass(), mid, &[]);
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
                return false;
            }
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] prepare_index: define_class({OPS_CLASS}) failed");
                return false;
            };
            let names = [
                CString::new("prepProbe").expect("no NUL"),
                CString::new("prepInit").expect("no NUL"),
                CString::new("prepBatch").expect("no NUL"),
                CString::new("prepQuery").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(I)I").expect("no NUL"),
                CString::new("(III[I[D)I").expect("no NUL"),
                CString::new("(DDDDDDII[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: crate::prepare_index::prep_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: crate::prepare_index::prep_init as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: crate::prepare_index::prep_batch as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[3].as_ptr(),
                    signature: sigs[3].as_ptr(),
                    fnPtr: crate::prepare_index::prep_query as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] prepare_index: register_natives failed (code {code}) — hooks stay dormant"
                );
                return false;
            }
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] prepare_index: bridge definition failed, hooks stay dormant");
            return;
        }

        // Compute per-target patches; flip READY per target; retransform each.
        let mut armed_sites = 0usize;
        for t in list {
            let Some(original) = t.take_orig() else {
                eprintln!(
                    "[crussty-plugin] prepare_index: no pristine bytes for {} — site stays vanilla",
                    t.name
                );
                continue;
            };
            let patched = match compute_patch(t, &original) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] prepare_index: retarget rejected for {} ({e}) — site stays vanilla",
                        t.name
                    );
                    continue;
                }
            };
            eprintln!(
                "[crussty-plugin] prepare_index: computed patch for {} ({} -> {} bytes)",
                t.name,
                original.len(),
                patched.len()
            );
            t.set_patch(Arc::from(patched));
            t.ready.store(true, Ordering::Release);
            armed_sites += 1;
        }
        if armed_sites == 0 {
            eprintln!(
                "[crussty-plugin] prepare_index: zero sites armed — lever stays vanilla (no marker)"
            );
            return;
        }
        ACTIVATED.store(true, Ordering::Release);
        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp401_offthread: ARMED sites={armed_sites}/4 (living.push/target-find/mob.aiStep/avoid.canUse) gather=region-bucket-batch write=offthread-rebuild planes=1.0+8.0 pad=3.0 move_limit=4.0 r_gate=1.0 (retargeted sites serve prepared candidates + exact vanilla re-filter; ERR_RANGE per-call vanilla, ERR_STRUCT disarm)"
        );
        for t in list {
            if !t.ready.load(Ordering::Acquire) {
                continue;
            }
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!(
                "[crussty-plugin] prepare_index: {} armed, retransform rc={rc}",
                t.name
            );
        }
    });
}
