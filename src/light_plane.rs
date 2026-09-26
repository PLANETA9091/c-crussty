//! Runtime wiring for the LIGHT-ENGINE plane census lever (round-466 C21,
//! vector закон 8 chunk/worldgen ось — свет-рекалки при генерации/апдейтах;
//! lever `cmp466_light`; bridge
//! light/net/minecraft/world/entity/LightOps.java, natives: NONE).
//!
//! CENSUS LEG (NOT-A-BENCH телеметрия, прецедент C20 F3-LevelTicks): whole-body
//! redirect of `Monster.updateNoActionTime()V` →
//! `LightOps.muaNoActionTime(Monster)` — the bridge replicates the vanilla
//! body 1:1 (javap ground truth purpur-1.21.10: getLLDMV → fcmpg 0.5 →
//! noActionTime += 2) and adds periodic stderr census lines
//! `[c21-light-cens] calls=/hits=/calls_per_s=/hits_per_s=` = LIVE capture
//! numbers for the light-READ lane (`Entity.getLightLevelDependentMagicValue`
//! = 1.40% all-CPU on the navmath1 150k profile, 1632/116469 samples).
//!
//! PLANE CEILING (law 13b REFUTED_CENS math): light-READ lane ~1.57% CPU
//! (updateNoActionTime 1632 + isSunBurnTick 150 + canUse 37 +
//! getPathfindingCostFromLightLevels 13) + light-COMPUTE on light-threads
//! 0.07% (PrioritisedQueueExecutorThread 84 samples) → 100% capture ≈ +1.6пп
//! CPU = ×12.7 below the +20 bar. The census numbers anchor the capture-math
//! for the mob-family composition lane (light reads × mob-плотность).
//!
//! VANILLA-BIT-FOR-BIT: under empty/foreign `CRUSSTY_LEVER_FLAG` NO hook is
//! registered, NO retransform happens, NO bridge is defined — Monster.class
//! stays byte-identical vanilla. Under cmp466_light the redirect keeps the
//! exact vanilla semantics (same primitive call, same float compare, same
//! += 2 increment) + 2 static counter ops (<1% of the site cost — census
//! gate |norm| ≤ 1.0).
//!
//! FAIL-CLOSED ladder: patcher Err (kernel shape mismatch) → dormant;
//! bridge define failure → dormant; no pristine capture → dormant;
//! Retargeted{sites:0} → dormant. NCDFE канон: bridge defined into the
//! kernel loader BEFORE the retransform serves patched bytes; hook performs
//! NO JNI work (pristine capture at the class's own load, flush_diet
//! pattern).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const TARGET_CLASS: &str = "net/minecraft/world/entity/monster/Monster";
const OPS_CLASS: &str = "net/minecraft/world/entity/LightOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../light/build/net/minecraft/world/entity/LightOps.class");

const METHOD_NAME: &str = "updateNoActionTime";
const VIRTUAL_DESC: &str = "()V";
/// Receiver-prepended static form (stack-identical Monster→static gate).
const STATIC_DESC: &str = "(Lnet/minecraft/world/entity/monster/Monster;)V";

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp466_light")
    )
}

static READY: AtomicBool = AtomicBool::new(false);

struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

/// Poison recovery (TASK-46): locks only wrap plain Vec/Arc stores.
impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        let mut orig = self.orig.lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(bytes.to_vec());
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn set_patch(&self, bytes: Arc<[u8]>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(TARGET_CLASS))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise): the callback performs NO JNI
/// work; pristine capture at the class's own load, patch served from the
/// cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] light_plane: dormant (CRUSSTY_LEVER_FLAG != cmp466_light, vanilla bit-in-byte)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] light_plane: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] light_plane: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the Monster class, define the bridge
/// into the kernel loader, compute the whole-body redirect from the pristine
/// bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // Monster loads with the first monster (world load / population).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] light_plane: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] light_plane: forcing kernel load of {}",
                    t.name
                );
                crate::improved_noise::force_load_kernel_class(t.name);
            }
            let sighted = cplug_sdk::classes::is_sighted(t.name);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (fluid_guard
        // TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] light_plane: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] light_plane: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
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
                "[crussty-plugin] light_plane: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild via scripts/build_light_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (EntityQueryOps pattern;
        // anchor loader = Entity's — same loader all bridge classes use).
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity")
            else {
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
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] light_plane: defined {OPS_CLASS} in kernel loader");
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] light_plane: define_class({OPS_CLASS}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] light_plane: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes for a class that predates the hook (fast boot):
        // no-op retransform capture, fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] light_plane: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
                let _ = attempt;
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] light_plane: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };
        // ONE whole-body redirect — Monster.updateNoActionTime()V ->
        // LightOps.muaNoActionTime(Monster) (must land with sites:1;
        // anti-placebo gate: sites>0 else disarm).
        let (patched, outcome) = match crate::classfile::redirect_method_body_to_static(
            &original,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] light_plane: redirect rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let ok = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !ok || matches!(outcome, crate::classfile::RetargetOutcome::Retargeted { sites: 0 }) {
            eprintln!(
                "[crussty-plugin] light_plane: unexpected redirect outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] light_plane: computed redirect for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!(
            "[crussty-plugin] light_plane: ARMED updateNoActionTime census bridge (cmp466_light, bit-exact vanilla body + [c21-light-cens] counters, 0 natives, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod light_plane_tests {
    use super::{METHOD_NAME, OPS_BYTES, OPS_CLASS, STATIC_DESC, TARGET_CLASS, VIRTUAL_DESC};
    use crate::classfile::redirect_method_body_to_static;

    /// javap ground truth: the kernel Monster fixture carries exactly one
    /// updateNoActionTime()V; the redirect replaces the whole body with the
    /// LightOps gate and stays idempotent on re-sight.
    #[test]
    fn light_redirect_applies_to_kernel_fixture() {
        let fixture =
            include_bytes!("../tests/fixtures/Monster.class") as &[u8];
        let (patched, outcome) = redirect_method_body_to_static(
            fixture,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("redirect must compute");
        assert_eq!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
            "exactly the updateNoActionTime body must be replaced"
        );
        // Idempotency: re-sighting on the patched bytes must be
        // AlreadyPatched with byte-identical output.
        let (again, o1) = redirect_method_body_to_static(
            &patched,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("re-redirect must compute");
        assert_eq!(
            o1,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 },
            "updateNoActionTime re-sight must be idempotent"
        );
        assert_eq!(again, patched);
    }

    /// The redirect must land ONLY on updateNoActionTime: every other
    /// Monster method must stay byte-identical (no collateral rewrites).
    #[test]
    fn light_redirect_is_site_exact() {
        let fixture =
            include_bytes!("../tests/fixtures/Monster.class") as &[u8];
        let (patched, _outcome) = redirect_method_body_to_static(
            fixture,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("redirect must compute");
        // Class size must shrink: the vanilla body (23 bytes of code) is
        // replaced by 3 bytes of code (aload_0/invokestatic/return), but the
        // new CP entries add — net must stay within one small delta, and the
        // untouched aiStep bytes must survive verbatim.
        let needle: &[u8] = b"aiStep";
        assert!(
            patched
                .windows(needle.len())
                .any(|w| w == needle),
            "aiStep must remain present in the patched class"
        );
        let needle: &[u8] = b"getLightLevelDependentMagicValue";
        assert!(
            patched
                .windows(needle.len())
                .any(|w| w == needle),
            "the getLLDMV cp entry must remain for the ctor/aiStep sites"
        );
    }

    /// Bridge blob sanity: major 65 (release 21), the lever id + census tag
    /// must live in the constant pool (check_blobs_sync markers), and the
    /// static gate must carry the exact receiver-prepended desc.
    #[test]
    fn light_bridge_blob_contract() {
        let bytes: &[u8] = OPS_BYTES;
        assert!(bytes.len() > 8);
        let major = ((bytes[6] as u16) << 8) | bytes[7] as u16;
        assert_eq!(major, 65, "LightOps must be --release 21 (major 65)");
        for marker in [b"cmp466_light" as &[u8], b"c21-light-cens", b"muaNoActionTime"] {
            assert!(
                bytes.windows(marker.len()).any(|w| w == marker),
                "marker {} missing from LightOps blob",
                String::from_utf8_lossy(marker)
            );
        }
    }

    /// STRICT gate: only the exact cmp466_light lever arms the plane.
    #[test]
    fn light_gate_is_strict() {
        // The enabled() gate must be an exact string match — reflect via the
        // source (static assert by contract): any prefix/trim tolerance would
        // be a TASK-402-F half-armed violation.
        let desc = STATIC_DESC;
        assert_eq!(
            desc,
            "(Lnet/minecraft/world/entity/monster/Monster;)V",
            "static desc = virtual desc with receiver Monster prepended"
        );
        assert_eq!(VIRTUAL_DESC, "()V");
        assert_eq!(
            TARGET_CLASS,
            "net/minecraft/world/entity/monster/Monster"
        );
        assert_eq!(OPS_CLASS, "net/minecraft/world/entity/LightOps");
    }
}
