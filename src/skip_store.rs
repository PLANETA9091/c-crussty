//! Runtime wiring for the SKIP-STORE-BB lever (S7-166, #13-SBB — see
//! entityinside/net/minecraft/world/level/SkipStoreOps.java).
//!
//! The card-set/remset lane is the TOP-1 G1 consumer (16.39% self-CPU,
//! RECON-11) and its driver is the old->young store-firehose; the
//! boundingBox/move family is the top allocation producer (36.92% of
//! allocation samples, RECON-12). The vanilla Entity.setBoundingBox(AABB)
//! body ALWAYS allocates a fresh AABB and ALWAYS putfields it into the
//! old-gen Entity — one young allocation + one remembered card per call,
//! even when the value is identical. This lever redirects that ONE method
//! body (entity_compose stage 8) to SkipStoreOps.setBoundingBox, which
//! repeats the javap-verbatim normalization ladder (dcmpg/dcmpl, 64.0
//! clamp, NaN-verbatim — RECON-12a contract) and SKIPS the store when the
//! current field already bit-matches. Parity: bit-equality is STRICTLY
//! stronger than the preregistered dcmp-equality (-0.0/+0.0 never skips;
//! NaN canonicalizes — value-indistinguishable downstream); AABB is
//! immutable; 0 identity sites on bb across 10 fixture classes (RECON-12a).
//! setDeltaMovement is NOT touched (5 identity sites, move() guard window
//! = parity risk, RECON-12a).
//!
//! This module: (1) defines SkipStoreOps into the KERNEL loader at boot;
//! (2) publishes BRIDGE_READY; (3) `wait_bridge_ready` is polled by
//! entity_compose BEFORE composing the Entity bytes. Fail-closed: define
//! failure -> dormant (entity_compose continues without the sbb stage).
//!
//! DELIVERY GRAPH GUARD: SkipStoreOps.java MUST compile to exactly ONE
//! classfile and MUST declare the receiver-prepended
//! setBoundingBox(Entity;AABB)V static — both enforced by cargo tests.

use std::sync::atomic::{AtomicBool, Ordering};

const SKIP_STORE_CLASS: &str = "net/minecraft/world/level/SkipStoreOps";

pub const SKIP_STORE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/level/SkipStoreOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_SKIP_STORE_BB")
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
/// setBoundingBox body resolves SkipStoreOps the instant it first runs, so
/// the class MUST be defined before the Entity retransform is served.
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
            "[crussty-plugin] skip_store_bb: dormant (set CRUSSTY_SKIP_STORE_BB=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] skip_store_bb: requires CRUSSTY_REGION_THREADS>=2 (the redirect composes through the entity_compose chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] skip_store_bb: boot marker not seen, hook stays dormant"
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
        let major = crate::improved_noise::class_version(SKIP_STORE_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] skip_store_bb: {SKIP_STORE_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

        // RESOLUTION CLOSURE GUARD (S7-164 leg#1 TECH-DUD discipline): the
        // embedded bridge bytes MUST declare the (name, descriptor) the
        // stage-8 redirect will emit; otherwise the first entity tick
        // detonates a NoSuchMethodError storm. Fail-closed -> dormant.
        if let Err(e) = crate::classfile::skipstore_resolution_closure(SKIP_STORE_BYTES) {
            eprintln!(
                "[crussty-plugin] skip_store_bb: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
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
            let Some(c) = env.define_class(SKIP_STORE_CLASS, gref, SKIP_STORE_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] skip_store_bb: define_class({SKIP_STORE_CLASS}) failed"
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
                "[crussty-plugin] skip_store_ops: defined {SKIP_STORE_CLASS} in kernel loader"
            );
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] skip_store_bb: bridge definition failed, hook stays dormant"
            );
        }
    });
}

#[cfg(test)]
mod skipstore_delivery_tests {
    /// S7-163 leg#1 delivery-graph guard, mirrored for #13-SBB: the
    /// SkipStoreOps.java source MUST declare ZERO nested classes (the
    /// bridge compiles to exactly one classfile and is defined alone).
    #[test]
    fn skipstore_ops_source_declares_no_nested_classes() {
        let src = include_str!("../entityinside/net/minecraft/world/level/SkipStoreOps.java");
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
            "SkipStoreOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile; nested classes would crash the \
             server with NoClassDefFoundError (S7-163 leg#1 TECH-DUD)"
        );
    }

    /// Build-dir mirror: exactly one SkipStoreOps classfile exists.
    #[test]
    fn skipstore_build_dir_has_exactly_one_classfile() {
        let dir = "entityinside/build/net/minecraft/world/level";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_skipstore_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("SkipStoreOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(
            count, 1,
            "SkipStoreOps classfile set drifted — SkipStoreOps must compile to exactly ONE classfile"
        );
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn skipstore_embedded_bytes_match_build_dir() {
        let on_disk =
            std::fs::read("entityinside/build/net/minecraft/world/level/SkipStoreOps.class")
                .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::SKIP_STORE_BYTES,
            "embedded SkipStoreOps.class is stale — rerun scripts/build_skipstore_ops.sh"
        );
    }

    /// #13-SBB resolution-closure guard: the embedded bridge classfile must
    /// declare EVERY (name, descriptor) the entity_compose stage-8 redirect
    /// emits.
    #[test]
    fn skipstore_embedded_declares_all_redirect_targets() {
        if let Err(e) = crate::classfile::skipstore_resolution_closure(super::SKIP_STORE_BYTES) {
            panic!(
                "RESOLUTION CLOSURE FAILED: {e} — rebuild entityinside/ via build_skipstore_ops.sh"
            );
        }
    }

    /// #13-SBB scope lock (RECON-12a): the redirect graph must contain
    /// EXACTLY the setBoundingBox target — setDeltaMovement is parity-risky
    /// (5 identity sites, the move() guard window) and MUST NOT silently
    /// join the redirect table.
    #[test]
    fn skipstore_redirect_table_is_exactly_setboundingbox() {
        let targets = crate::classfile::SSB_REDIRECT_TARGETS;
        assert_eq!(targets.len(), 1, "#13-SBB is a SINGLE-SITE lever");
        assert_eq!(targets[0].0, "setBoundingBox");
        assert_eq!(targets[0].1, "(Lnet/minecraft/world/phys/AABB;)V");
        assert_eq!(targets[0].2, "setBoundingBox");
        assert_eq!(
            targets[0].3,
            "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V"
        );
    }
}
