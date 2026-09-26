//! Runtime wiring for the INSIDE-FLUID lever (ROUND-468-S18, R468-S18
//! fluid-empty fastpath on the checkInsideBlocks visit lambda).
//!
//! S18 decomposition of the inside-blocks lane (RESEARCH doc /home/z/rounds/
//! ROUND-468/S18.md): the visit-lambda census (RECON-4 leg#5, cpu-collapsed,
//! 9286-sample traversal lane) attributes collidedWithFluid 1671 samples =
//! 32.6% of the visit-lambda body (5126) = 1.31% CPU — the TOP vanilla
//! sub-lane of the TOP-1 inside remainder (12-16.6пп carrier) that is NOT
//! already captured (getBlockState -> inside_snap; advanceStep ->
//! batch_collector; orchestration -> flat_traversal BANNED; alloc family ->
//! alloc_diet BANNED) and NOT the forbidden inside_bitmask plane (закон 5).
//!
//! The lever retargets the SINGLE Entity.collidedWithFluid invokevirtual call
//! site inside Entity.lambda$checkInsideBlocks$2 to the static
//! InsideFluidOps.gate(Entity,FluidState,BlockPos,Vec3,Vec3)Z (receiver-first
//! 3B->3B, length-preserving). Bit-exact contract (javap, patched kernel):
//! Fluid.getAABB returns aconst_null for isEmpty(), and
//! Entity.collidedWithFluid maps null -> false — so the gate's
//! `!fluid.isEmpty()` prefix short-circuits exactly the dead null path and
//! leaves every non-empty input to the untouched vanilla body (no cache, no
//! state, no order change: the second getFluidState consumer in the lambda
//! runs on the same boolean).
//!
//! Delivery (S7-162 single compose-chain): this module OWNS THE BRIDGE only —
//! it defines InsideFluidOps into the kernel loader and publishes
//! BRIDGE_READY; the Entity byte patch itself is the entity_compose stage
//! 1d. Fail-closed: define failure -> BRIDGE_READY never set -> the compose
//! chain continues WITHOUT the stage (vanilla call site, zero risk).
//! NCDFE canon (fa9054d9 ARM-AFTER-DEFINE): the bridge is defined BEFORE the
//! composed Entity bytes resolve the new static ref.

use std::sync::atomic::{AtomicBool, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideFluidOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideFluidOps.class");

pub fn enabled() -> bool {
    if std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp468_s18fluid")
        .unwrap_or(false)
    {
        return true;
    }
    std::env::var("CRUSSTY_INSIDE_FLUID")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

pub fn enabled_pub() -> bool {
    enabled()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

/// Pollable gate for the entity_compose stage pipeline — the composed Entity
/// bytes resolve `InsideFluidOps` on the first fluid check of the visit
/// lambda, so the class MUST be defined before the retransform is served.
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

/// Register (idempotent; call from cplugin_init). No byte hook here — the
/// Entity patch composes through entity_compose (stage 1d).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_fluid: dormant (set CRUSSTY_INSIDE_FLUID=1 or lever_flag=cmp468_s18fluid to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_fluid: bridge owner armed, Entity stage delegated to entity_compose (S7-162 single compose-chain)"
    );
}

pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // Wait for the kernel Entity class (kernel loader capture point).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_fluid: {} not loaded within 180s, bridge stays undefined",
                    ENTITY_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_fluid: forcing kernel load of {}",
                    ENTITY_CLASS
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define (boot-storm discipline).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] inside_fluid: boot marker not seen, hook stays dormant");
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
        let major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] inside_fluid: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_fluid_ops.sh; hook stays dormant"
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
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] inside_fluid: define_class({OPS_CLASS}) failed — hook stays dormant"
                );
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_fluid: bridge definition aborted, hook stays dormant (fail-closed)"
            );
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "gate", "inside_fluid v1 (R468-S18 fluid-empty fastpath; entity_compose stage 1d)");
        eprintln!(
            "[crussty-plugin] inside_fluid: bridge defined, BRIDGE_READY (Entity stage composes via entity_compose)"
        );
    });
}

#[cfg(test)]
mod insidefluid_delivery_tests {
    use super::OPS_BYTES;

    /// Delivery-graph guard (inside_diet pattern): the bridge Java source
    /// MUST declare ZERO nested classes — the classfile is defined alone
    /// into the kernel loader; a nested class would detonate as
    /// NoClassDefFoundError on the first entity tick.
    #[test]
    fn insidefluid_source_declares_no_nested_classes() {
        let src = include_str!("../entityinside/net/minecraft/world/entity/InsideFluidOps.java");
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") && !before.contains('*')
                    {
                        panic!("nested declaration in bridge source: {t}");
                    }
                }
            }
        }
    }

    /// The embedded bytes must be real classfiles pinned to major 65.
    #[test]
    fn insidefluid_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }
}
