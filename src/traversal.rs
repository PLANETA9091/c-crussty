//! Runtime wiring for the FLAT-TRAVERSAL lever (S7-163, ARCH-ATTACK
//! lever #9 — see entityinside/net/minecraft/world/level/TraverseOps.java).
//!
//! The inside-pipeline traversal lane (RECON-4: forEachBlockIntersectedBetween
//! family 7.27% CPU; orchestration tail ~3050 of ~3400 samples = guava
//! AbstractIterators + betweenCornersInDirection bodies + betweenClosed
//! lambdas + LongOpenHashSet per call) is attacked by retargeting the
//! SINGLE invokestatic call site in Entity.checkInsideBlocks to the flat
//! TraverseOps.forEachFlat (identical descriptor; bit-exact visit
//! order/steps verified by TraverseLockstepHarness — 60k+ random and
//! degenerate scenarios, sequence+return bit-in-bit).
//!
//! This module: (1) defines TraverseOps into the KERNEL loader at boot
//! (same package net.minecraft.world.level as BlockGetter); (2) publishes
//! BRIDGE_READY; (3) `wait_bridge_ready` is polled by entity_compose
//! BEFORE composing the Entity bytes — the patched call site resolves
//! TraverseOps the moment checkInsideBlocks first runs, so the class must
//! be defined before the Entity retransform is served (the population
//! inject runs AFTER arm-time; ordering is a hard gate, batch_collector
//! precedent). The retarget itself lives in entity_compose stage 6
//! (single compose-chain owner, classfile::patch_entity_traversal).
//! Fail-closed: define failure -> dormant (entity_compose continues
//! without the traversal stage); strict site mismatch -> compose keeps
//! the chain without traversal and logs the rejection.

use std::sync::atomic::{AtomicBool, Ordering};

const TRAVERSE_CLASS: &str = "net/minecraft/world/level/TraverseOps";

const TRAVERSE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/level/TraverseOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_FLAT_TRAVERSAL")
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

/// Pollable gate for the entity_compose stage pipeline — the retargeted
/// call site resolves `TraverseOps` the instant checkInsideBlocks runs,
/// so the class MUST be defined before the Entity retransform is served.
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

pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] flat_traversal: dormant (set CRUSSTY_FLAT_TRAVERSAL=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] flat_traversal: requires CRUSSTY_REGION_THREADS>=2 (the retarget composes through the entity_compose chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        // Boot discipline: same as inside_cache/region_threads (quiet
        // loader before define). Entity is guaranteed loaded at this point.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] flat_traversal: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(15));

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(TRAVERSE_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] flat_traversal: {TRAVERSE_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

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
            let Some(c) = env.define_class(TRAVERSE_CLASS, gref, TRAVERSE_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] flat_traversal: define_class({TRAVERSE_CLASS}) failed"
                );
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] traverse_ops: defined {TRAVERSE_CLASS} in kernel loader");
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] flat_traversal: bridge definition failed, hook stays dormant"
            );
        }
    });
}
