//! Runtime wiring for ITEMS-STAGGER (ROUND-396 vector B, TASK-396-B —
//! мега-раунд архитектурная замена расписания тяжёлых проверок item-тика).
//!
//! The vector: ItemEntity.tick runs three heavy checks whose work is
//! schedule-replaceable without touching despawn/pickup/baseTick/n-push:
//!   1. mergeWithNeighbours()   (vanilla already throttles to 1/40 stationary)
//!   2. Entity.move(...)        (vanilla already staggers 1/4 settled)
//!   3. Level.noCollision(...)  (per-tick squeezed-check, freshness N)
//! Each is re-scheduled on a phase-shifted cadence keyed by entity id:
//! `((tickCount / unit) + id) % N == 0` (unit = vanilla grid 40/4/1), N =
//! CRUSSTY_LEVER_ARG (default 4). First scheduled tick (spawn, tickCount<=1)
//! is always due; the vanilla moved-case passes through 1:1.
//!
//! Mechanism: ONE bridge class (ItemStaggerOps, include_bytes!) defined into
//! the kernel loader + THREE 1:1 call-site retargets inside ItemEntity.tick
//! (receiver-prepended statics, identical stack shapes — no bytecode length
//! change, no StackMapTable edits). The bridge invokes the PRIVATE vanilla
//! mergeWithNeighbours through a MethodHandle (privateLookupIn; nestmates are
//! not available to a define_class product). Fail-dominance: the bridge's
//! static initializer THROWS if the handle cannot be built; the worker
//! probes signature() and aborts BEFORE retransform — ItemEntity stays
//! 100% vanilla.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_stagger"` (exact), N from
//! `CRUSSTY_LEVER_ARG` (default 4, read by the bridge itself). Dormant lever
//! = vanilla by construction (no hook, no define, no retransform).
//!
//! Isolation: ItemEntity is NOT part of the entity_compose Entity chain
//! (parent class hook matches by exact name only); no other lever patches
//! ItemEntity. Stateless bridge — safe under region_threads=4 parallel item
//! ticking (config read-only after static init, no collections, no memo).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemStaggerOps";
const OPS_DESC_MERGE: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";
const OPS_DESC_MOVE: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V";
const OPS_DESC_NOCOLL: &str =
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z";

const OPS_BYTES: &[u8] =
    include_bytes!("../itemsstagger/build/net/minecraft/world/entity/item/ItemStaggerOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);
static OPS_GREF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_stagger")
    )
}

pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

/// Poison recovery (TASK-46 discipline): locks wrap plain stores only.
struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target {
        orig: std::sync::Mutex::new(None),
        patch: std::sync::Mutex::new(None),
    })
}

fn stash_orig(bytes: &[u8]) {
    let mut orig = target().orig.lock().unwrap_or_else(PoisonError::into_inner);
    if orig.is_none() {
        *orig = Some(bytes.to_vec());
    }
}

fn orig_bytes() -> Option<Vec<u8>> {
    target()
        .orig
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

fn set_patch(bytes: Arc<[u8]>) {
    *target().patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
}

fn patch_bytes() -> Option<Arc<[u8]>> {
    target()
        .patch
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

/// Single byte hook on ItemEntity. Pristine sighting (first load OR the
/// no-op capture retransform) is stashed only; after READY the precomputed
/// compose is served. Callback does NO JNI/classfile work (loader-lock
/// discipline, entity_compose precedent).
fn register_hook() {
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, move |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            stash_orig(bytes);
            return None;
        }
        patch_bytes().map(|c| c.to_vec())
    });
}

/// Probe the bridge: forces static init, returns its signature string.
/// Any failure (CNFE/NOSUCH/ExceptionInInitializerError) -> None (abort).
fn probe_signature() -> Option<String> {
    cplug_sdk::jni_util::with_attached(|env| {
        let gref = OPS_GREF.load(Ordering::SeqCst) ;
        if gref.is_null() {
            return None;
        }
        let Some(mid) = env.get_static_method_id(gref, "signature", "()Ljava/lang/String;") else {
            crate::clear_exception(env);
            return None;
        };
        let res = env.call_static_object_method(gref, mid, &[]);
        if res.is_null() {
            crate::clear_exception(env);
            return None;
        }
        let s = env.get_string_utf(res as jvmti_bindings::jni::jstring).unwrap_or_default();
        env.delete_local_ref(res);
        Some(s)
    })
    .unwrap_or(None)
}

/// Apply the three strict 1:1 retargets. Any outcome other than exactly one
/// retargeted site (or an already-patched single site) is a shape violation
/// -> Err (fail-dominant, no serve).
fn compose(orig: &[u8]) -> Result<Vec<u8>, String> {
    let mut bytes = orig.to_vec();
    let gates: [(&str, &str, &str, (&str, &str, &str), &str); 3] = [
        (
            "merge",
            "net/minecraft/world/entity/item/ItemEntity",
            "()V",
            (
                "net/minecraft/world/entity/item/ItemEntity",
                "mergeWithNeighbours",
                OPS_DESC_MERGE,
            ),
            OPS_DESC_MERGE,
        ),
        (
            "move",
            "net/minecraft/world/entity/MoverType",
            "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
            (
                "net/minecraft/world/entity/Entity",
                "move",
                "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
            ),
            OPS_DESC_MOVE,
        ),
        (
            "nocollision",
            "net/minecraft/world/phys/AABB",
            "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z",
            (
                "net/minecraft/world/level/Level",
                "noCollision",
                "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z",
            ),
            OPS_DESC_NOCOLL,
        ),
    ];
    // tuple above is awkward to read; do the three retargets explicitly:
    let _ = gates;

    // Gate 1: merge — from the private virtual to the bridge static.
    let merge_from = (
        "net/minecraft/world/entity/item/ItemEntity",
        "mergeWithNeighbours",
        "()V",
    );
    let merge_to = (OPS_CLASS, "mergeWithNeighbours", OPS_DESC_MERGE);
    // Gate 2: settled-move extension — from Entity.move virtual.
    let move_from = (
        "net/minecraft/world/entity/Entity",
        "move",
        "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
    );
    let move_to = (OPS_CLASS, "move", OPS_DESC_MOVE);
    // Gate 3: noCollision freshness — from Level.noCollision virtual.
    let nocoll_from = (
        "net/minecraft/world/level/Level",
        "noCollision",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z",
    );
    let nocoll_to = (OPS_CLASS, "noCollision", OPS_DESC_NOCOLL);

    for (label, from, to) in [
        ("merge", merge_from, merge_to),
        ("move", move_from, move_to),
        ("nocollision", nocoll_from, nocoll_to),
    ] {
        let (out, outcome) = crate::classfile::retarget_virtual_to_static(
            &bytes, "tick", "()V", from, to,
        )?;
        match outcome {
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {
                eprintln!("[crussty-plugin] items_stagger: gate {label} retargeted (sites=1)");
            }
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 } => {
                eprintln!("[crussty-plugin] items_stagger: gate {label} already patched (sites=1)");
            }
            other => {
                return Err(format!(
                    "gate {label}: strict site-count violated ({other:?}) — fail-dominant"
                ));
            }
        }
        // Thread the mutated buffer through the next stage.
        bytes = out;
    }
    Ok(bytes)
}

/// Background activation (fluid_bitmask/entity_compose discipline):
/// boot quiet -> ItemEntity visible -> hook registered -> bridge defined ->
/// signature probe -> pristine capture (no-op retransform) -> strict compose
/// -> READY -> retransform (serve) -> ARMED.
pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_stagger: dormant (set CRUSSTY_LEVER_FLAG=items_stagger to enable)"
        );
        return;
    }
    let n = std::env::var("CRUSSTY_LEVER_ARG").unwrap_or_else(|_| "4".to_string());
    std::thread::Builder::new()
        .name("crussty-items-stagger".into())
        .spawn(move || {
            // Boot must be quiet before any bridge define (loader-storm discipline).
            if !crate::improved_noise::wait_for_boot() {
                eprintln!("[crussty-plugin] items_stagger: boot marker not seen, lever stays dormant");
                return;
            }
            // Wait for the kernel ItemEntity class (loads with the world).
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(ITEM_ENTITY).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_stagger: {ITEM_ENTITY} not loaded within 180s — lever stays dormant"
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            // Hook first so any subsequent retransform/load stashes pristine bytes.
            register_hook();

            // Define the bridge into the kernel loader (fluid_bitmask pattern).
            let defined = cplug_sdk::jni_util::with_attached(|env| {
                let Some(cls) = cplug_sdk::classes::find_class(ITEM_ENTITY) else {
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
                env.delete_local_ref(class_cls);
                if gref.is_null() {
                    crate::describe_exception(env);
                    env.delete_local_ref(loader);
                    return false;
                }
                env.delete_local_ref(loader);
                match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                    Some(c) => {
                        let gr = env.new_global_ref(c);
                        OPS_GREF.store(gr as usize, Ordering::SeqCst);
                        env.delete_local_ref(c);
                        eprintln!(
                            "[crussty-plugin] items_stagger: defined {OPS_CLASS} in kernel loader"
                        );
                        true
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] items_stagger: define_class({OPS_CLASS}) failed");
                        false
                    }
                }
            })
            .unwrap_or(false);
            if !defined {
                eprintln!("[crussty-plugin] items_stagger: bridge definition aborted, lever stays dormant (vanilla)");
                return;
            }

            // Fail-dominant probe: static init throws if the private-handle
            // lookup failed -> probe returns None -> abort BEFORE retransform.
            match probe_signature() {
                Some(sig) => {
                    eprintln!("[crussty-plugin] items_stagger: bridge probe ok ({sig})");
                }
                None => {
                    eprintln!(
                        "[crussty-plugin] items_stagger: bridge signature probe FAILED (static init threw?) — lever stays dormant (vanilla)"
                    );
                    return;
                }
            }

            // Pristine capture: the class predates the hook (fast boot) —
            // no-op retransform fires the hook with original bytes (stash).
            for _attempt in 1..=3 {
                if orig_bytes().is_some() {
                    break;
                }
                let _ = cplug_sdk::retransform_class(ITEM_ENTITY);
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            let Some(orig) = orig_bytes() else {
                eprintln!("[crussty-plugin] items_stagger: no pristine bytes captured — lever stays dormant (vanilla)");
                return;
            };

            // Strict compose of the three gates.
            match compose(&orig) {
                Ok(patched) => {
                    set_patch(std::sync::Arc::from(patched.into_boxed_slice()));
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_stagger: compose rejected ({e}) — lever stays dormant (vanilla)"
                    );
                    return;
                }
            }

            // Serve: flip READY then retransform — the hook serves the compose.
            READY.store(true, Ordering::SeqCst);
            for _attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
                if rc == 0 {
                    ARMED.store(true, Ordering::SeqCst);
                    eprintln!(
                        "[crussty-plugin] items_stagger: ARMED gates=merge,move,nocollision N={n} (ItemEntity retransformed)"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            eprintln!(
                "[crussty-plugin] items_stagger: retransform never returned 0 — lever NOT armed (vanilla)"
            );
        })
        .ok();
}
