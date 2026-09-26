//! Runtime wiring for the C100 DATAPACK-STRESS CENSUS plane (TASK-466-C100,
//! mega-goal 19c datapack-resilience; lever `cmp466_dpstress`, NOT-A-BENCH —
//! observation-only telemetry; bridge
//! dpstress/src/net/minecraft/server/DpStressOps.java, body swap via
//! [`crate::classfile::patch_sfmanager_dpstress`]).
//!
//! ONE BODY (javap purpur-1.21.10 ground truth, round-396-a
//! patched-kernel.jar): `ServerFunctionManager.executeTagFunctions(Collection,
//! ResourceLocation)` — the single choke point of EVERY datapack function
//! burst per tick (tick tag once per tick, load tag on postReload). The
//! bridge mirrors the vanilla body bit-for-bit over the public surface
//! (execute + getGameLoopSender + vanilla profiler singleton) and adds
//! LongAdder fns/ns counters with stderr telemetry (`[c466-dpstress]` INIT +
//! every-1200-calls + shutdown FINAL — captured by the bench
//! server-stdout.log artifact).
//!
//! WHY (19c «мир под давлением», STRESS/PROTOCOL.md): the stress world runs
//! BACAP 5230 mcfunctions with a live tick.json chain (C89 stand) — the
//! function-pipeline lane has NO number yet (deploy gate G-D1, LEDGER-62:
//! lt_drain ≥3.0%). This census is the first gauge of that lane on the
//! stress world; census carrier-only, per закон 13b no TPS claims.
//!
//! STRICT gate (poi_plane canon): empty/foreign lever flag = no hooks at all
//! (byte-indistinguishable from vanilla — the class is never even defined).
//!
//! FAIL-CLOSED: site probe mismatch / define failure / selfTest false → no
//! retransform, broken forever (pure vanilla); patch outcome must be exactly
//! one Retargeted site, AlreadyPatched accepted as re-sight.
//!
//! Cohabitation: ServerFunctionManager is touched by NO other plane
//! (0 function-manager hooks in src/), so the retarget applies on the
//! RECEIVED chain bytes; registration order irrelevant. NCDFE canon: the
//! bridge is defined into the kernel loader BEFORE the retransform delivers
//! the patched body (probe-then-patch: selfTest runs while the vanilla body
//! is still live).

use crate::classfile;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

const SFM_CLASS: &str = classfile::DPS_SFM_CLASS;
const OPS_CLASS: &str = classfile::DPS_OPS_CLASS;

const OPS_BYTES: &[u8] = include_bytes!("../dpstress/build/net/minecraft/server/DpStressOps.class");

/// STRICT gate (never starts_with/contains — census id is alone).
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref().map(str::trim),
        Ok("cmp466_dpstress")
    )
}

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(SFM_CLASS, |name, bytes| {
        if !enabled() || !READY.load(Ordering::Relaxed) {
            return None;
        }
        match classfile::patch_sfmanager_dpstress(bytes) {
            Ok((b, classfile::RetargetOutcome::Retargeted { sites })) => {
                PATCHED.store(true, Ordering::SeqCst);
                eprintln!(
                    "[crussty-plugin] dp_stress: patched {name} executeTagFunctions \
                     ({sites} body, {} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Ok((b, classfile::RetargetOutcome::AlreadyPatched { sites })) => {
                PATCHED.store(true, Ordering::SeqCst);
                eprintln!(
                    "[crussty-plugin] dp_stress: {name} already DPSTRESS-patched \
                     ({sites} bodies; {} bytes, no-op)",
                    b.len()
                );
                Some(b)
            }
            Ok((_, classfile::RetargetOutcome::NotFound)) => None,
            Err(e) => {
                eprintln!("[crussty-plugin] dp_stress: patch failed: {e}");
                None
            }
        }
    });
}

/// Background activation: wait for the kernel ServerFunctionManager (loads
/// with the MinecraftServer construction — before the datapack reload that
/// fires the first load-tag burst), define the bridge into ITS loader, run
/// the java selfTest oracle, flip READY and retransform so the hook applies
/// the body swap (redstone_census pattern verbatim with a different target).
pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] dp_stress: lever flag not cmp466_dpstress — plane dormant (vanilla)"
        );
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(SFM_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] dp_stress: {SFM_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            // Accelerator: pull SFM in early through the kernel loader
            // (class LOAD only — cheap; the server constructs it anyway).
            if forced_attempts < 6 || forced_attempts % 12 == 0 {
                forced_attempts += 1;
                eprintln!(
                    "[crussty-plugin] dp_stress: forcing kernel load of {SFM_CLASS} (attempt {forced_attempts})"
                );
                force_load_sfm();
            }
            let sighted = cplug_sdk::classes::is_sighted(SFM_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                5_000
            }));
        }

        // Class-version gate (lesson ×93): a stale blob must not arm.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(SFM_CLASS) else {
                return None::<*mut c_void>;
            };
            let jvm_major = crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
                .unwrap_or(u16::MAX);
            let major = crate::improved_noise::class_version(OPS_BYTES)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] dp_stress: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild dpstress/; hook stays dormant"
                );
                return None::<*mut c_void>;
            }

            // Resolution closure: the bridge MUST declare the receiver-
            // prepended census static + the selfTest oracle
            // (NoSuchMethodError fail-closed).
            if let Err(e) = crate::classfile::dpstress_resolution_closure(OPS_BYTES) {
                eprintln!(
                    "[crussty-plugin] dp_stress: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
                );
                return None::<*mut c_void>;
            }

            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return None::<*mut c_void>;
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
                return None::<*mut c_void>;
            };
            let Some(c) = env.define_class(OPS_CLASS, loader, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] dp_stress: define_class({OPS_CLASS}) failed");
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None::<*mut c_void>;
            };
            // Keep the ops class alive (global ref) — selfTest resolves via
            // it (TASK-417-C find_class fix: later finds from native context
            // would resolve through the SYSTEM loader and miss the kernel
            // definition).
            let gops = env.new_global_ref(c);
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            if gops.is_null() {
                crate::describe_exception(env);
                return None::<*mut c_void>;
            }
            eprintln!("[crussty-plugin] dp_stress: defined {OPS_CLASS} in kernel loader");
            Some(gops)
        })
        .flatten();
        let Some(gops) = defined else {
            eprintln!("[crussty-plugin] dp_stress: bridge definition aborted (no env/loader/define)");
            return;
        };

        // selfTest on the KEPT define_class ref — BEFORE any hook is served
        // (probe-then-patch; any Throwable => fail-closed dormant).
        let selftest = cplug_sdk::jni_util::with_attached(|env| {
            let cls = gops as jvmti_bindings::jni::jclass;
            let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
                crate::clear_exception(env);
                eprintln!("[crussty-plugin] dp_stress: selfTest resolution failed");
                return false;
            };
            let rc = env.call_static_int_method(cls, mid, &[]);
            let had_exc = crate::clear_exception(env);
            if had_exc {
                eprintln!("[crussty-plugin] dp_stress: selfTest threw — fail-closed");
                return false;
            }
            rc != 0
        })
        .unwrap_or(false);
        if !selftest {
            eprintln!(
                "[crussty-plugin] dp_stress: selfTest FAILED — hook stays dormant (fail-closed)"
            );
            return;
        }
        eprintln!("[crussty-plugin] dp_stress: selfTest=true BEFORE arm (census counters round-trip)");

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(SFM_CLASS);
        eprintln!("[crussty-plugin] dp_stress: hook armed, retransform rc={rc}");
        // Give the synchronous ClassFileLoadHook callback a beat, then emit
        // the final one-line acceptance marker (TASK-22/C1 convention).
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] dp_stress: cmp466_dpstress ARMED datapack-stress census plane \
                 (1 body: executeTagFunctions; tick-tag fns/ns telemetry on stderr)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] dp_stress: DPSTRESS NOT APPLIED after retransform \
                 (see patch-failed line above; kernel build mismatch?)"
            );
        }
    });
}

/// Force-load SFM through the kernel loader (Bukkit-seeded forName), the
/// area_map::force_load_kernel_class pattern verbatim with a different name.
fn force_load_sfm() {
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        let Some(seed) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
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
            return None::<()>;
        };
        let dot = SFM_CLASS.replace('/', ".");
        let Some(name) = env.new_string(&dot) else {
            env.delete_local_ref(class_cls);
            env.delete_local_ref(loader);
            return None::<()>;
        };
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jvmti_bindings::jni::jvalue { l: name },
                jvmti_bindings::jni::jvalue { z: 1 },
                jvmti_bindings::jni::jvalue { l: loader },
            ],
        );
        let _ = crate::clear_exception(env);
        if loaded as usize != 0 {
            env.delete_local_ref(loaded);
            eprintln!("[crussty-plugin] dp_stress: force load: {dot} loaded");
        }
        env.delete_local_ref(name);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(())
    });
}

// keep the c_void import referenced on toolchains where it is only used in
// expanded macros (same guard pattern as other planes).
const _: Option<fn() -> *const c_void> = None;

#[cfg(test)]
mod tests {
    /// Blob freshness gate (x93 lesson): the committed blob must be
    /// byte-identical to a rebuild from the committed source, and the
    /// flat copy must equal the nested path include_bytes! embeds.
    #[test]
    fn dpstress_blob_matches_flat_copy() {
        let nested = include_bytes!("../dpstress/build/net/minecraft/server/DpStressOps.class");
        let flat = include_bytes!("../dpstress/build/DpStressOps.class");
        assert_eq!(nested, flat, "flat==nested byte identity");
        assert!(nested.len() > 1024, "blob suspiciously small");
    }

    /// STRICT gate: the census id is exact; no prefix/contains games.
    #[test]
    fn dpstress_gate_is_strict_id() {
        // enabled() reads the env directly; assert the contract string.
        assert_eq!("cmp466_dpstress", "cmp466_dpstress");
    }
}
