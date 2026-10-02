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

/// Nested classes compiled out of TraverseOps.java. EVERY nested classfile
/// produced by the build MUST be listed here and defined into the kernel
/// loader in the same activation — a plain classpath resolves nested
/// classes implicitly, but the kernel loader does NOT, so a missing entry
/// detonates as NoClassDefFoundError on the first entity tick (S7-163
/// leg#1 TECH-DUD root cause: TraverseOps$LongTable was compiled but
/// never defined; the offline lockstep harness could not catch it).
const TRAVERSE_NESTED: &[(&str, &[u8])] = &[(
    "net/minecraft/world/level/TraverseOps$LongTable",
    include_bytes!(
        "../entityinside/build/net/minecraft/world/level/TraverseOps$LongTable.class"
    ),
)];

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
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            };
            env.delete_local_ref(c);
            // Nested classes MUST be defined before BRIDGE_READY: the retarget
            // makes TraverseOps resolvable, and its first use executes the
            // nested-class NEW, which resolves against THIS loader only.
            for (name, bytes) in TRAVERSE_NESTED {
                let Some(nc) = env.define_class(name, gref, bytes) else {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] flat_traversal: define_class({name}) failed"
                    );
                    env.delete_local_ref(loader);
                    env.delete_local_ref(class_cls);
                    return false;
                };
                env.delete_local_ref(nc);
                eprintln!(
                    "[crussty-plugin] traverse_ops: defined nested {name} in kernel loader"
                );
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] traverse_ops: defined {TRAVERSE_CLASS} (+{} nested) in kernel loader",
                TRAVERSE_NESTED.len()
            );
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] flat_traversal: bridge definition failed, hook stays dormant"
            );
        }
    });
}

#[cfg(test)]
mod nested_delivery_tests {
    /// S7-163 leg#1 TECH-DUD guard: every nested class declared in
    /// TraverseOps.java MUST be embedded in TRAVERSE_NESTED, otherwise the
    /// kernel-loader delivery crashes the server with NoClassDefFoundError
    /// on the first entity tick (a plain classpath resolves nested classes
    /// implicitly, so the offline lockstep harness cannot catch this).
    #[test]
    fn every_nested_class_of_traverse_ops_is_embedded() {
        let src = include_str!("../entityinside/net/minecraft/world/level/TraverseOps.java");
        let mut declared: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    // nested declarations carry `static` (private static final class X)
                    if before.contains("static") {
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
            !declared.is_empty(),
            "no nested classes parsed from TraverseOps.java — parser out of date"
        );
        for name in declared {
            let expected = format!("net/minecraft/world/level/TraverseOps${name}");
            assert!(
                super::TRAVERSE_NESTED.iter().any(|(n, _)| *n == expected),
                "nested class {expected} is compiled but NOT embedded in TRAVERSE_NESTED — \
                 add it to src/traversal.rs or the server will crash with \
                 NoClassDefFoundError (S7-163 leg#1 TECH-DUD)"
            );
        }
    }

    /// The build script must produce exactly the top-level classfile plus
    /// the embedded nested set — no surprise additional nesting may appear
    /// unlisted (build-time mirror of the source-parse guard).
    #[test]
    fn build_dir_classfiles_match_embedded_set() {
        let dir = "entityinside/build/net/minecraft/world/level";
        let expected_top = format!("{dir}/TraverseOps.class");
        let mut expected: Vec<String> = vec![expected_top];
        for (name, _) in super::TRAVERSE_NESTED {
            let simple = name.rsplit('/').next().unwrap(); // "TraverseOps$LongTable"
            expected.push(format!("{dir}/{simple}.class"));
        }
        expected.sort();
        let mut actual: Vec<String> = Vec::new();
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_traverse_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("TraverseOps") && p.ends_with(".class") {
                actual.push(p);
            }
        }
        actual.sort();
        assert_eq!(expected, actual,
            "TraverseOps classfile set drifted — update TRAVERSE_NESTED in src/traversal.rs");
    }
}
