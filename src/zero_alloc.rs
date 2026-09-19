//! Runtime wiring for the ZERO-ALLOC-INSIDE lever (S7-164, ARCH-ATTACK
//! lever #10 — see entityinside/net/minecraft/world/level/ZeroAllocOps.java).
//!
//! The fluid-push lane (10.55% CPU = updateFluidHeightAndDoFluidPushing
//! family) and the collidedWithFluid chain (1.37% + 0.54% + 0.32% +
//! FluidState.getAABB 0.77%) are attacked by replacing the METHOD BODIES of
//! Entity.collidedWithFluid / collidedWithShapeMovingFrom /
//! updateFluidHeightAndDoFluidPushing with receiver-prepended static
//! dispatches into ZeroAllocOps (scalar, allocation-free, bit-exact double
//! math per the javap contract). Census (v3 profile, 35399980345):
//! collidedAlongVector is called ONLY from collidedWithShapeMovingFrom, and
//! collidedWithShapeMovingFrom ONLY from collidedWithFluid — so three
//! Entity body-redirects capture the whole lane without touching AABB.
//!
//! This module: (1) defines ZeroAllocOps into the KERNEL loader at boot
//! (same package net.minecraft.world.level as TraverseOps); (2) publishes
//! BRIDGE_READY; (3) `wait_bridge_ready` is polled by entity_compose
//! BEFORE composing the Entity bytes (the redirected bodies resolve
//! ZeroAllocOps the instant the methods run, so the class MUST be defined
//! before the Entity retransform is served — batch_collector/traverse_ops
//! precedent). Fail-closed: define failure -> dormant (entity_compose
//! continues without the zeroin stage).
//!
//! DELIVERY GRAPH GUARD (S7-163 leg#1 lesson): ZeroAllocOps.java MUST
//! compile to exactly ONE classfile. The cargo tests below parse the
//! source for nested static declarations and mirror-check the build dir —
//! a nested class would detonate as NoClassDefFoundError on the first
//! entity tick (the offline lockstep harness cannot catch it).

use std::sync::atomic::{AtomicBool, Ordering};

const ZERO_ALLOC_CLASS: &str = "net/minecraft/world/level/ZeroAllocOps";

const ZERO_ALLOC_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/level/ZeroAllocOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_ZERO_ALLOC")
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

/// Pollable gate for the entity_compose stage pipeline — the redirected
/// bodies resolve `ZeroAllocOps` the instant they first run, so the class
/// MUST be defined before the Entity retransform is served.
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
            "[crussty-plugin] zero_alloc_inside: dormant (set CRUSSTY_ZERO_ALLOC=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] zero_alloc_inside: requires CRUSSTY_REGION_THREADS>=2 (the redirects compose through the entity_compose chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] zero_alloc_inside: boot marker not seen, hook stays dormant"
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
        let major = crate::improved_noise::class_version(ZERO_ALLOC_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] zero_alloc_inside: {ZERO_ALLOC_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
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
            let Some(c) = env.define_class(ZERO_ALLOC_CLASS, gref, ZERO_ALLOC_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] zero_alloc_inside: define_class({ZERO_ALLOC_CLASS}) failed"
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
        if defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] zero_alloc_ops: defined {ZERO_ALLOC_CLASS} in kernel loader"
            );
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] zero_alloc_inside: bridge definition failed, hook stays dormant"
            );
        }
    });
}

#[cfg(test)]
mod zeroalloc_delivery_tests {
    /// S7-163 leg#1 delivery-graph guard, mirrored for lever #10: the
    /// ZeroAllocOps.java source MUST declare ZERO nested classes (the
    /// bridge compiles to exactly one classfile and is defined alone).
    #[test]
    fn zeroalloc_ops_source_declares_no_nested_classes() {
        let src = include_str!("../entityinside/net/minecraft/world/level/ZeroAllocOps.java");
        let mut declared: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    // nested declarations carry `static` (private static final class X)
                    if before.contains("static") && !before.contains("//") {
                        let rest = &t[i + pat.len()..];
                        let name: String = rest
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty() {
                            declared.push(name);
                        }
                    }
                    break;
                }
            }
        }
        assert!(
            declared.is_empty(),
            "ZeroAllocOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile; nested classes would crash the \
             server with NoClassDefFoundError (S7-163 leg#1 TECH-DUD)"
        );
    }

    /// Build-dir mirror: exactly one ZeroAllocOps classfile exists.
    #[test]
    fn zeroalloc_build_dir_has_exactly_one_classfile() {
        let dir = "entityinside/build/net/minecraft/world/level";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_zeroalloc_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("ZeroAllocOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(
            count, 1,
            "ZeroAllocOps classfile set drifted — ZeroAllocOps must compile to exactly ONE classfile"
        );
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn zeroalloc_embedded_bytes_match_build_dir() {
        let on_disk =
            std::fs::read("entityinside/build/net/minecraft/world/level/ZeroAllocOps.class")
                .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::ZERO_ALLOC_BYTES,
            "embedded ZeroAllocOps.class is stale — rerun scripts/build_zeroalloc_ops.sh"
        );
    }
}
