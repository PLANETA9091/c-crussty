//! Runtime wiring for the INSIDE-BITMASK lever (TASK-357 — ARCH-ATTACK
//! lever on the inside-discovery lane; RECON-32/33: 11.28/11.65% scene,
//! option-B flagman, TPS conversion ~0 so NOT bankable under DUAL BAR —
//! this module is the implementation-ready pre-work, dormant by default).
//!
//! Patches ONE kernel call site (see src/classfile.rs INSIDE-BITMASK section
//! for the javap contract): the single call of the private
//! `Entity.checkInsideBlocks(List, StepBasedCollector)V` inside
//! `Entity.applyEffectsFromBlocks(List<Movement>)` (bc 58..64) retargeted to
//! the static `InsideBitmaskOps.checkInsideBlocksGated(Entity, List,
//! StepBasedCollector)V` (receiver-first, 3B→3B, length-preserving). One
//! retarget covers every entity class (ItemEntity.tick bc273,
//! ExperienceOrb/FallingBlockEntity/PrimedTnt/EndCrystal.tick,
//! EnderDragon.aiStep, AbstractBoat.tick ×2, AbstractMinecart.move — the
//! full per-tick entry census of RECON-33 §1).
//!
//! Gate: env `CRUSSTY_INSIDE_BITMASK` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (same as inside_cache/fluid_guard): with the
//! gate off, register() logs a dormant notice and NO byte hook exists.
//!
//! Fail-closed matrix: bridge define failure OR the armed probe
//! (`InsideBitmaskOps.armState() != "ARMED"`) → BRIDGE_READY never set →
//! the compose chain continues WITHOUT inside_bitmask (fail-dominant, loud
//! WARN); Entity patch-level failures are handled by the entity_compose
//! stage (strict Retargeted/AlreadyPatched check, patcher Err → stage
//! skipped). The armed probe is the probe-then-patch guarantee: the Java
//! gate cannot re-invoke the vanilla body without its MethodHandle, so a
//! disarmed bridge must NEVER be patched in.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideBitmaskOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBitmaskOps.class");

fn enabled() -> bool {
    // TASK-430-B (round-430-b-inside): the plane arms under ITS OWN lever id
    // (STRICT eq) — cmp430_inside = inside-plane subsystem carrier round.
    // The legacy CRUSSTY_INSIDE_BITMASK env stays accepted for A/B replays
    // (bank keeps it 0; lever flag is the dispatch key on the carrier).
    let lever = std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == "cmp432_inside2" || v == "cmp430_inside" || v == "cmp436_ins4"
            || v == "cmp438_sense" // TASK-444-C: sense family union
            || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" || v == "cmp456_chunkmono" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        })
        .unwrap_or(false);
    if lever {
        return true;
    }
    std::env::var("CRUSSTY_INSIDE_BITMASK")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate visibility for the entity_compose stage pipeline.
pub fn enabled_pub() -> bool {
    enabled()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref to the DEFINED ops class (0 = none). REQUIRED for the arm
/// probe: a JNI-defined class lives only in the JVM dictionary under the
/// kernel loader — env.find_class resolves via the SYSTEM loader on an
/// attached native thread and throws CNFE, and loader.loadClass is not
/// reachable for define_class products either (noise_fill.rs smoke-1
/// evidence). s7204 scene lesson: probe via find_class returned false
/// BEFORE the Java static init ever ran (no [crussty-ops] print, DISARMED
/// with no throwable) → BRIDGE_READY never published → compose window
/// missed AND region_threads rng-stage wait cascaded to dormant → leg
/// measured bank-minus-region_threads (0.9 TPS, delivery-fail).
static OPS_GREF: AtomicUsize = AtomicUsize::new(0);

/// Pollable gate for the entity_compose stage pipeline — the compose worker
/// needs the bridge class DEFINED **and ARMED** before the composed Entity
/// bytes resolve InsideBitmaskOps on the first gated call.
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(Ordering::Acquire)
}

/// Register (idempotent; call once from cplugin_init). NO byte hook is
/// installed here — the Entity patch composes through entity_compose.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_bitmask: dormant (lever_flag not in {{cmp432_inside2/cmp430_inside}} and CRUSSTY_INSIDE_BITMASK unset)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_bitmask: bridge owner armed, Entity stage delegated to entity_compose"
    );
}

/// Probe the defined bridge: armState() must return "ARMED". Anything else
/// (class init failure, missing method, exception) = disarmed.
fn probe_armed() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
        // Resolve via the define-time global ref: the bridge lives only in
        // the JVM dictionary under the kernel loader — env.find_class throws
        // CNFE (system loader on an attached native thread) and
        // loader.loadClass is not reachable for define_class products
        // (noise_fill.rs smoke-1 evidence; s7204 scene-verified defect).
        let gref = OPS_GREF.load(Ordering::SeqCst) as jni::jclass;
        if gref.is_null() {
            return false;
        }
        let Some(mid) = env.get_static_method_id(gref, "armState", "()Ljava/lang/String;") else {
            crate::clear_exception(env);
            return false;
        };
        let res = env.call_static_object_method(gref, mid, &[]);
        if res.is_null() {
            crate::clear_exception(env);
            return false;
        }
        let state = env.get_string_utf(res as jni::jstring).unwrap_or_default();
        env.delete_local_ref(res);
        state == "ARMED"
    })
    .unwrap_or(false)
}

/// Background activation: wait for the kernel Entity class, define the
/// bridge class into the kernel loader, PROBE ARMED, publish BRIDGE_READY.
/// The Entity patch is computed and applied by entity_compose.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_bitmask: {} not loaded within 180s, bridge stays undefined",
                    ENTITY_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_bitmask: forcing kernel load of {}",
                    ENTITY_CLASS
                );
                // RC7 canon (TASK-433-B; ref a3991c2): LAZY force-load
                // (initialize=false) — no <clinit> on the poll thread before
                // Bootstrap; first real use initializes post-bootStrap.
                crate::improved_noise::force_load_kernel_class_lazy(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_bitmask: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_bitmask: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] inside_bitmask: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_bitmask_ops.sh; hook stays dormant"
            );
            return;
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    // Keep a global ref to the ops class for probe_armed():
                    // the class is JNI-defined and therefore NOT reachable via
                    // find_class (system loader on native thread) nor
                    // loader.loadClass (noise_fill.rs smoke-1 precedent).
                    let gr = env.new_global_ref(c);
                    OPS_GREF.store(gr as usize, Ordering::SeqCst);
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] inside_bitmask: defined {OPS_CLASS} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] inside_bitmask: define_class({OPS_CLASS}) failed"
                    );
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_bitmask: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // PROBE-THEN-PATCH: only an ARMED bridge may be published; the Java
        // gate cannot serve the vanilla path without its MethodHandle.
        if !probe_armed() {
            eprintln!(
                "[crussty-plugin] inside_bitmask: armState() != ARMED (MethodHandle/Unsafe resolution failed), hook stays dormant"
            );
            return;
        }

        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "checkInsideBlocksGated", "inside_bitmask v1");
        eprintln!(
            "[crussty-plugin] inside_bitmask: bridge defined+armed, BRIDGE_READY (Entity stage composes via entity_compose)"
        );
    });
}
