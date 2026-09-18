//! Runtime wiring for the BATCH-COLLECTOR lever (S7-161, ARCH-ATTACK
//! lever #8 v2 — see entityinside/.../BatchCollector.java).
//!
//! S7-160 lesson: the lazy Unsafe swap into the final
//! `insideEffectCollector` field does not survive across ticks
//! (BatchCollector.<init>+ensure = 1441 samples with ZERO entity
//! rotation) — retired from the hot path. The persistent mechanism is a
//! CONSTRUCTOR-LEVEL retarget: the single `new StepBasedCollector; dup;
//! invokespecial <init>` site in `Entity.<init>(EntityType, Level)` is
//! rewritten to `BatchCollector` (same ()V ctor shape, subsumption-safe
//! for the putfield) by `classfile::patch_entity_collector_ctor`.
//!
//! COMPOSITION (S7-160 log evidence, lines 886/895 of leg #5): Entity is
//! patched by TWO byte hooks (inside_cache and region_threads) and the
//! region_threads hook composes from ITS OWN take_orig — the final
//! Entity bytes served at the region retransform are rng-only; the
//! inside_cache bytes (205522) are superseded (205458 -> 205494). The
//! collector retarget therefore rides the REGION-THREADS Entity chain
//! (after the S7-158d rng retarget) — the LAST writer wins and it must
//! carry the batch patch. The inside_cache/rng hook-coexistence cleanup
//! is S7-162.
//!
//! This module: (1) defines BatchCollector into the KERNEL loader at
//! boot; (2) publishes BRIDGE_READY; (3) `wait_bridge_ready` is polled
//! by region_threads BEFORE composing the Entity bytes (the class must
//! be resolvable the moment the patched ctor runs — otherwise the first
//! entity construction dies with NoClassDefFoundError; the population
//! inject runs AFTER arm-time, so the ordering is a hard gate).
//! Fail-closed: define failure -> dormant (region composes without the
//! batch patch); strict NEW-site mismatch -> region keeps rng-only and
//! logs the rejection.

use std::sync::atomic::{AtomicBool, Ordering};

const BC_CLASS: &str = "net/minecraft/world/entity/BatchCollector";

const BC_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/BatchCollector.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_BATCH_COLLECTOR")
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

/// S7-161: pollable gate for the region_threads Entity composition — the
/// patched ctor resolves `BatchCollector` the instant an entity spawns,
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
            "[crussty-plugin] batch_collector: dormant (set CRUSSTY_BATCH_COLLECTOR=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] batch_collector: requires CRUSSTY_REGION_THREADS>=2 (the ctor retarget composes through the region_threads Entity chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        // Boot discipline: same as inside_cache/region_threads (quiet
        // loader before define). Entity is guaranteed loaded at this point.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] batch_collector: boot marker not seen, hook stays dormant"
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
        let major = crate::improved_noise::class_version(BC_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] batch_collector: {BC_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
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
            let Some(c) = env.define_class(BC_CLASS, gref, BC_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] batch_collector: define_class({BC_CLASS}) failed");
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] batch_collector: defined {BC_CLASS} in kernel loader");
            BRIDGE_READY.store(true, Ordering::Release);
        } else {
            eprintln!(
                "[crussty-plugin] batch_collector: bridge definition failed, hook stays dormant"
            );
        }
    });
}
