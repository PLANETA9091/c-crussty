//! Runtime wiring for ITEMS_SOA (MEGA-ROUND-2, TASK-397-C — round-1 lever D
//! mechanism, NEVER previously implemented).
//!
//! TOP-1 bottleneck (run 35528326290, bank v4 fp=4, 115655 samples):
//! items/ItemEntity.tick 31.17% java + item-driven broadphase 15.66% (the
//! per-item per-tick Level.getEntitiesOfClass query inside
//! ItemEntity.mergeWithNeighbours) + fastutil 8.54% + java_util 7.01%. This
//! module redirects the SINGLE private body ItemEntity.mergeWithNeighbours()V
//! (strict sites==1, redirect_method_body_to_static — the receiver is
//! prepended, every other byte untouched) to the SoA bridge ItemSoaOps:
//! candidate enumeration for the merge scan runs over flat primitive arrays
//! (double[] x/y/z slots + int[]-linked uniform-grid buckets, dirty-set sync =
//! single-writer-per-slot self-sync + 100ms double-buffered grid flip), while
//! every merge DECISION stays the exact vanilla bytecode path on the live
//! objects (see entityinside/.../ItemSoaOps.java for the parity contract and
//! the fail-open vanilla fallback).
//!
//! Flag off (CRUSSTY_LEVER_FLAG != "items_soa") -> no hook, no define, no
//! retransform: byte-exact vanilla. Flag on but anything fails (bridge window,
//! strict site count) -> fail-dominant dormant (vanilla semantics).
//!
//! Ordering: register() hooks the ItemEntity load (pristine stash), the
//! background activate() waits for the kernel class + boot quiet, defines the
//! bridge into the kernel loader, flips READY and retransforms ItemEntity
//! exactly once. ItemEntity is owned ONLY by this module (no other lever
//! retargets it), so there is no byte-hook supersede hazard (S7-160/161
//! lesson) and no retransform-order coupling with entity_compose.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const ITEM_ENTITY_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemSoaOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/item/ItemSoaOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);

struct Target {
    orig: Mutex<Option<Vec<u8>>>,
    patch: Mutex<Option<Arc<[u8]>>>,
}

fn target() -> &'static Target {
    static T: OnceLock<Target> = OnceLock::new();
    T.get_or_init(|| Target {
        orig: Mutex::new(None),
        patch: Mutex::new(None),
    })
}

/// Arm gate: CRUSSTY_LEVER_FLAG == "items_soa" AND CRUSSTY_LEVER_ARG == "1".
/// Any other combination (including empty flag) = exact vanilla path.
pub fn enabled() -> bool {
    static ARM: OnceLock<bool> = OnceLock::new();
    *ARM.get_or_init(|| {
        let flag = std::env::var("CRUSSTY_LEVER_FLAG").unwrap_or_default();
        let arg = std::env::var("CRUSSTY_LEVER_ARG").unwrap_or_default();
        let on = flag.trim() == "items_soa" && arg.trim() == "1";
        if flag.trim() == "items_soa" && !on {
            eprintln!(
                "[crussty-plugin] item_soa: lever_flag=items_soa but lever_arg={arg:?} != \"1\" — dormant (vanilla)"
            );
        }
        on
    })
}

/// Boot-time telemetry anchor for the harness stdout greps.
pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

/// Register the single ItemEntity byte hook (idempotent, call once from
/// cplugin_init). Pre-READY sightings stash pristine bytes; post-READY serves
/// the precomputed redirect. No JNI/classfile work inside the callback
/// (loader-lock discipline).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] item_soa: dormant (set CRUSSTY_LEVER_FLAG=items_soa CRUSSTY_LEVER_ARG=1 to enable)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY_CLASS, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            let mut orig = t.orig.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if orig.is_none() {
                eprintln!(
                    "[crussty-plugin] item_soa: pristine sighting {} {} bytes (major {})",
                    ITEM_ENTITY_CLASS,
                    bytes.len(),
                    crate::improved_noise::class_version(bytes)
                        .map(|(m, _)| m)
                        .unwrap_or(0)
                );
                *orig = Some(bytes.to_vec());
            }
            return None; // vanilla passthrough until armed
        }
        let cached = t
            .patch
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait kernel ItemEntity + boot quiet, define the
/// ItemSoaOps bridge into the kernel loader, patch (strict sites==1), flip
/// READY, retransform ItemEntity exactly once. Fail-dominant: any miss keeps
/// vanilla bytes.
pub fn activate() {
    if !enabled() {
        return;
    }
    let ok = std::thread::Builder::new()
        .name("crussty-item-soa".into())
        .spawn(move || activate_worker())
        .map(|j| {
            // Keep the handle leak-free: detach — the worker logs its own verdict.
            std::mem::forget(j);
            true
        })
        .unwrap_or(false);
    if !ok {
        eprintln!("[crussty-plugin] item_soa: activate worker spawn failed — dormant");
    }
}

fn activate_worker() {
    let t = target();
    // Wait for the kernel ItemEntity class (loads at boot / first drop).
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
    loop {
        if cplug_sdk::classes::find_class(ITEM_ENTITY_CLASS).is_some() {
            break;
        }
        if std::time::Instant::now() > deadline {
            eprintln!(
                "[crussty-plugin] item_soa: {} not loaded within 180s, hook stays dormant",
                ITEM_ENTITY_CLASS
            );
            return;
        }
        if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
            crate::improved_noise::force_load_kernel_class(ITEM_ENTITY_CLASS);
        }
        let sighted = cplug_sdk::classes::is_sighted(ITEM_ENTITY_CLASS);
        std::thread::sleep(std::time::Duration::from_millis(if sighted {
            2_000
        } else {
            10_000
        }));
    }

    // Kernel loader must be quiet before the bridge define (boot-time
    // class-loading storm discipline).
    if !crate::improved_noise::wait_for_boot() {
        eprintln!("[crussty-plugin] item_soa: boot marker not seen, hook stays dormant");
        return;
    }

    // Pristine bytes: if the class predates the hook (fast boot), capture via
    // no-op retransform (READY=false -> stash-only), fluid_guard pattern.
    let orig_is_some = t
        .orig
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .is_some();
    if !orig_is_some {
        eprintln!(
            "[crussty-plugin] item_soa: {} predates hook, capturing via no-op retransform",
            ITEM_ENTITY_CLASS
        );
        for _attempt in 1..=3 {
            let _ = cplug_sdk::classes::retransform(ITEM_ENTITY_CLASS);
            let has = t
                .orig
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .is_some();
            if has {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        let has = t
            .orig
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .is_some();
        if !has {
            eprintln!(
                "[crussty-plugin] item_soa: no pristine bytes for {}, hook stays dormant",
                ITEM_ENTITY_CLASS
            );
            return;
        }
    }

    let orig = t
        .orig
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let Some(orig) = orig else {
        return;
    };

    // Define the bridge into the kernel loader (ItemEntity's own loader —
    // kernel-internal types are resolvable; define AFTER ItemEntity is live so
    // the descriptor never forces an early load).
    let defined = cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(ITEM_ENTITY_CLASS) else {
            crate::clear_exception(env);
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
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] item_soa: define_class({OPS_CLASS}) failed");
            return false;
        };
        env.delete_local_ref(c);
        true
    })
    .unwrap_or(false);
    if !defined {
        eprintln!("[crussty-plugin] item_soa: bridge define failed — dormant (vanilla)");
        return;
    }
    eprintln!(
        "[crussty-plugin] item_soa: defined {OPS_CLASS} in kernel loader ({} bytes)",
        OPS_BYTES.len()
    );

    // Strict single-site body redirect of mergeWithNeighbours()V.
    let (patched, outcome) = match crate::classfile::patch_item_soa(&orig) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[crussty-plugin] item_soa: patch rejected ({e}) — dormant (vanilla)");
            return;
        }
    };
    match outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
        | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 } => {}
        other => {
            eprintln!(
                "[crussty-plugin] item_soa: strict sites==1 violated ({other:?}) — dormant (vanilla)"
            );
            return;
        }
    }

    *t.patch
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::from(patched.into_boxed_slice()));
    READY.store(true, Ordering::SeqCst);
    let rc = cplug_sdk::classes::retransform(ITEM_ENTITY_CLASS);
    ARMED.store(rc, Ordering::SeqCst);
    eprintln!(
        "[crussty-plugin] item_soa: ARMED (mergeWithNeighbours -> ItemSoaOps), retransform rc={rc}"
    );
}

#[cfg(test)]
mod tests {
    /// MEGA-ROUND-2 (TASK-397-C): the mergeWithNeighbours redirect must be a
    /// strict single-site, idempotent, major-preserving body replacement of
    /// the real booted-kernel ItemEntity (fixture = purpur 1.21.10 class).
    #[test]
    fn merge_with_neighbours_redirect_strict_and_idempotent() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/ItemEntity.class");
        let bytes = std::fs::read(path).expect("ItemEntity fixture");
        let (out, outcome) = crate::classfile::patch_item_soa(&bytes).unwrap();
        assert!(
            matches!(outcome, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }),
            "first pass must retarget exactly one site: {outcome:?}"
        );
        // Idempotence: a second retransform cycle must detect AlreadyPatched
        // (one-shot retransform guard discipline, F1/F3 lesson).
        let (out2, outcome2) = crate::classfile::patch_item_soa(&out).unwrap();
        assert!(
            matches!(outcome2, crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }),
            "second pass must be a no-op: {outcome2:?}"
        );
        // The pristine bytes must not reference the bridge; the patched must.
        let s0 = String::from_utf8_lossy(&bytes);
        assert!(!s0.contains("ItemSoaOps"), "vanilla fixture must be bridge-free");
        let s1 = String::from_utf8_lossy(&out2);
        assert!(s1.contains("ItemSoaOps"), "patched bytes must resolve the bridge");
        // Class major preserved (kernel JVM 21 = 65).
        let (m0, _) = crate::improved_noise::class_version(&bytes).unwrap();
        let (m1, _) = crate::improved_noise::class_version(&out2).unwrap();
        assert_eq!(m0, m1, "class major must be preserved");
        // Body actually shrunk to the load+invokestatic+return stub.
        assert!(out2.len() < bytes.len(), "redirect must shrink the class");
        // Optional dump for javap eyeballing: ITEM_SOA_DUMP=/tmp/...
        if let Some(dump) = std::env::var_os("ITEM_SOA_DUMP") {
            std::fs::write(dump, &out2).unwrap();
        }
    }
}
