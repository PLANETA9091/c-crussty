//! Runtime wiring for the TRAVEL-DIET v2a COLLIDE-DIET lever (RECON-21,
//! ARCH-ATTACK lever #14 — see entityinside/net/minecraft/world/entity/
//! TravelDietOps.java).
//!
//! The travel-collide lane (7.71% CPU on the s7178 anchor; travel-chain
//! alloc 10.07% of the s7177 alloc window) is attacked by replacing the
//! METHOD BODY of the private Entity.collide(Vec3) with a receiver-
//! prepended static dispatch into TravelDietOps — a verbatim javap mirror
//! of the vanilla body (RECON-21 contract) whose AABB/ArrayList/
//! FloatArraySet temporaries are replaced by thread-local scratch slots
//! fully overwritten before every use (stack-slot replacement, NOT a
//! cache — ZeroAllocOps law). The RESULT objects stay allocations
//! (products, not temporaries).
//!
//! This module: (1) defines TravelDietOps into the KERNEL loader at boot
//! (package net.minecraft.world.entity, same as InsideDietOps); (2)
//! publishes BRIDGE_READY; (3) `wait_bridge_ready` is polled by
//! entity_compose BEFORE composing the Entity bytes (the redirected body
//! resolves TravelDietOps the instant it runs, so the class MUST be
//! defined before the Entity retransform is served — batch_collector/
//! zero_alloc precedent). Fail-closed: define failure -> dormant
//! (entity_compose continues without the traveldiet stage).
//!
//! DELIVERY GRAPH GUARD (S7-163 leg#1 lesson): TravelDietOps.java MUST
//! compile to exactly ONE classfile (cargo test parses the source for
//! nested declarations; the anonymous-ThreadLocal strike of the first
//! build is the live precedent — scratch is an Object[] via
//! withInitial(method reference)).

use std::sync::atomic::{AtomicBool, Ordering};

const TRAVEL_DIET_CLASS: &str = "net/minecraft/world/entity/TravelDietOps";

const TRAVEL_DIET_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/TravelDietOps.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_TRAVEL_DIET")
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
/// body resolves `TravelDietOps` the instant it first runs, so the class
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
            "[crussty-plugin] travel_diet: dormant (set CRUSSTY_TRAVEL_DIET=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] travel_diet: requires CRUSSTY_REGION_THREADS>=2 (the redirect composes through the entity_compose chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] travel_diet: boot marker not seen, hook stays dormant"
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
        let major = crate::improved_noise::class_version(TRAVEL_DIET_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] travel_diet: {TRAVEL_DIET_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

        // RESOLUTION CLOSURE GUARD (S7-164 leg#1 TECH-DUD): the embedded
        // bridge bytes MUST declare the (name, descriptor) the traveldiet
        // redirect emits; otherwise the first entity tick detonates a
        // NoSuchMethodError storm. Fail-closed here -> dormant + loud log.
        if let Err(e) = crate::classfile::traveldiet_resolution_closure(TRAVEL_DIET_BYTES) {
            eprintln!(
                "[crussty-plugin] travel_diet: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
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
            let Some(c) = env.define_class(TRAVEL_DIET_CLASS, gref, TRAVEL_DIET_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] travel_diet: define_class({TRAVEL_DIET_CLASS}) failed"
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
                "[crussty-plugin] travel_diet: defined {TRAVEL_DIET_CLASS} in kernel loader"
            );
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] travel_diet: bridge definition failed, hook stays dormant"
            );
            return;
        }

        // ---- v2b: LivingEntity travelInFluid arm (RECON-21 section 4) ----
        // The bridge is defined; wait for the LivingEntity kernel class,
        // capture pristine bytes, compute the single-site body redirect,
        // then arm the serve hook and retransform once (alloc_diet
        // activation cadence).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced = 0usize;
        loop {
            if cplug_sdk::classes::find_class(LIVING_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] travel_diet: {LIVING_CLASS} not loaded within 180s, living arm stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                && forced < 12
            {
                forced += 1;
                eprintln!(
                    "[crussty-plugin] travel_diet: forcing kernel load of {LIVING_CLASS} (attempt {forced})"
                );
                crate::improved_noise::force_load_kernel_class(LIVING_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(LIVING_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }
        // Pristine capture: if LivingEntity predates the hook (fast boot),
        // no-op retransform while LIVING_READY=false (stash-only callback).
        if living_orig().is_none() {
            eprintln!(
                "[crussty-plugin] travel_diet: {LIVING_CLASS} predates hook, capturing via no-op retransform"
            );
            for _ in 1..=3 {
                let _ = cplug_sdk::retransform_class(LIVING_CLASS);
                if living_orig().is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if living_orig().is_none() {
                eprintln!(
                    "[crussty-plugin] travel_diet: no pristine bytes for {LIVING_CLASS}, living arm stays dormant"
                );
                return;
            }
        }
        let original = living_take_orig().expect("pristine living bytes checked");
        let major = crate::improved_noise::class_version(&original)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let (patched, outcome) = match crate::classfile::patch_livingentity_traveldiet(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] travel_diet: living patch rejected ({e}), living arm stays dormant"
                );
                return;
            }
        };
        if !matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ) {
            eprintln!(
                "[crussty-plugin] travel_diet: unexpected living patch outcome ({outcome:?}), living arm stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] travel_diet: computed living patch ({outcome:?}, {} -> {} bytes)",
            original.len(),
            patched.len()
        );
        *LIVING.patch.lock().unwrap_or_else(|p| p.into_inner()) = Some(LivingPatch {
            bytes: std::sync::Arc::from(patched),
            major,
        });
        crate::kernel_policy::audit_wire(
            TRAVEL_DIET_CLASS,
            "travelInFluid",
            "travel_diet v2b",
        );
        LIVING_READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(LIVING_CLASS);
        eprintln!(
            "[crussty-plugin] travel_diet: living arm armed, retransform rc={rc}"
        );
    });
}

// ---- v2b LivingEntity serve hook (alloc_diet Target pattern, minimal) ----

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";

struct LivingPatch {
    bytes: std::sync::Arc<[u8]>,
    #[allow(dead_code)]
    major: u16,
}

struct LivingTarget {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<LivingPatch>>,
}

static LIVING: LivingTarget = LivingTarget {
    orig: std::sync::Mutex::new(None),
    patch: std::sync::Mutex::new(None),
};
static LIVING_READY: AtomicBool = AtomicBool::new(false);

fn living_orig() -> Option<Vec<u8>> {
    LIVING
        .orig
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .clone()
}

fn living_take_orig() -> Option<Vec<u8>> {
    LIVING
        .orig
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .take()
}

/// Byte hook on LivingEntity: pristine capture at the class's own load,
/// patched bytes served after LIVING_READY (bridge defined + patch
/// computed). Registration MUST precede any kernel class load (cplugin_init).
pub fn register_living() {
    if !enabled() {
        return;
    }
    cplug_sdk::hooks::register_bytes(LIVING_CLASS, move |_name, bytes| {
        if !LIVING_READY.load(Ordering::Acquire) {
            let mut guard = LIVING.orig.lock().unwrap_or_else(|p| p.into_inner());
            if guard.is_none() {
                eprintln!(
                    "[crussty-plugin] travel_diet: pristine sighting {LIVING_CLASS} {} bytes (major {})",
                    bytes.len(),
                    crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
                );
                *guard = Some(bytes.to_vec());
            }
            return None;
        }
        let cached = LIVING
            .patch
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .as_ref()
            .map(|p| p.bytes.clone());
        cached.map(|c| c.to_vec())
    });
}

#[cfg(test)]
mod traveldiet_delivery_tests {
    /// S7-163 delivery-graph guard, mirrored for lever #14 v2a: the
    /// TravelDietOps.java source MUST declare ZERO nested classes (the
    /// first build strike — anonymous ThreadLocal — is the live precedent).
    #[test]
    fn traveldiet_ops_source_declares_no_nested_classes() {
        let src = include_str!("../entityinside/net/minecraft/world/entity/TravelDietOps.java");
        let mut declared: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
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
            "TravelDietOps declares nested types {declared:?} — kernel-loader delivery detonates NoClassDefFoundError"
        );
    }

    /// The embedded classfile must be the compiled TravelDietOps bridge and
    /// must satisfy the resolution closure the entity_compose redirect
    /// emits (collide with Entity receiver prepended).
    #[test]
    fn traveldiet_embedded_classfile_satisfies_redirect_closure() {
        crate::classfile::traveldiet_resolution_closure(super::TRAVEL_DIET_BYTES)
            .expect("embedded TravelDietOps.class misses the redirect target");
    }
}
