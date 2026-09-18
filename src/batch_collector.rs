//! Runtime wiring for the BATCH-COLLECTOR lever (S7-160, ARCH-ATTACK
//! lever #8 — the zero-map StepBasedCollector replacement inside
//! Entity.insideEffectCollector, see entityinside/.../BatchCollector.java).
//!
//! This module is DEFINE-ONLY: it defines the BatchCollector bridge class
//! into the KERNEL loader after boot. No kernel bytes are patched and no
//! retransform is issued — the swap itself is performed lazily per entity
//! by RegionTickOps.tickBucket (S7-160 splice) through
//! BatchCollector.ensure(Entity) when CRUSSTY_BATCH_COLLECTOR=1.
//!
//! Composition rule: the lever requires region_threads >= 2 (the only swap
//! site lives in RegionTickOps.tickBucket; the CUMULATIVE v2 baseline
//! always runs region_threads=4). With region_threads dormant the bridge
//! class is defined but never reached — dormant-invisible.
//!
//! Fail-closed matrix: define failure -> only a stderr note; the vanilla
//! collector keeps serving (ensure() is never armed because the class is
//! absent — the call site is never executed, lazy resolve never triggers
//! NoClassDefFoundError... CRITICAL DISCIPLINE: RegionTickOps gates the
//! ensure() call behind its own static BATCH_COLLECTOR flag parsed from
//! the SAME env; both flags must be on for the swap to happen, so a
//! missed define degrades to the vanilla collector silently and safely).

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

pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] batch_collector: dormant (set CRUSSTY_BATCH_COLLECTOR=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] batch_collector: requires CRUSSTY_REGION_THREADS>=2 (swap site lives in RegionTickOps.tickBucket), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        // Boot discipline: same as region_threads (quiet loader before
        // define; Entity is guaranteed loaded at this point).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] batch_collector: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(25));

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
                eprintln!(
                    "[crussty-plugin] batch_collector: define_class({BC_CLASS}) failed"
                );
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] batch_collector: defined {BC_CLASS} in kernel loader"
            );
        } else {
            eprintln!(
                "[crussty-plugin] batch_collector: bridge definition failed, hook stays dormant"
            );
        }
    });
}
