//! Runtime wiring for ITEMS-STAGGER v2 (ROUND-397 vector B, TASK-397-B —
//! re-arm of the ROUND-396-B phase-scheduled merge scans after the census fix).
//!
//! The vector: ItemEntity.tick runs three heavy checks whose work is
//! schedule-replaceable without touching despawn/pickup/baseTick/n-push:
//!   1. mergeWithNeighbours()   (vanilla already throttles to 1/40 stationary)
//!   2. Entity.move(...)        (vanilla already staggers 1/4 settled)
//!   3. Level.noCollision(...)  (per-tick squeezed-check, freshness N)
//! Each is re-scheduled on a phase-shifted cadence keyed by entity id:
//! `((tickCount / unit) + id) % N == 0` (unit = vanilla grid 40/4/1), N = the
//! phase multiplier. The round-397 dispatch convention passes
//! CRUSSTY_LEVER_ARG=1 (variant selector) -> the designed N=4 schedule; an
//! explicit ARG>1 selects that N directly (bridge-side logic). First
//! scheduled tick (spawn, tickCount<=1) is always due; the vanilla moved-case
//! passes through 1:1.
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
//! CENSUS FIX vs ROUND-396-B (the fail-closed lesson): the retarget scans
//! invokevirtual (0xb6) AND invokespecial (0xb7) and matches the move site
//! OWNER-TOLERANTLY. Real kernel bytes (purpur-1.21.10 javap):
//!   * mergeWithNeighbours is PRIVATE but called via invokevirtual (JEP 181
//!     nestmates), owner=ItemEntity;
//!   * `this.move(...)` is INHERITED — javac emits the receiver's static
//!     type as the Methodref owner: `invokevirtual ItemEntity.move` (round-1
//!     expected Entity.move and found nothing -> strict compose rejected ->
//!     dormant);
//!   * Level.noCollision(Entity,AABB)Z — invokevirtual, owner=Level.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_stagger2"` (exact). Dormant lever
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

const MOVE_DESC: &str =
    "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V";
const NOCOLL_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z";

const OPS_BYTES: &[u8] =
    include_bytes!("../itemsstagger2/build/net/minecraft/world/entity/item/ItemStaggerOps.class");

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);
static OPS_GREF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_stagger2")
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
        let gref = OPS_GREF.load(Ordering::SeqCst) as jvmti_bindings::jni::jclass;
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

/// Apply the three strict 1:1 retargets (census-corrected: virtual+special
/// scan, owner-tolerant move site). Any outcome other than exactly one
/// retargeted site (or an already-patched single site) is a shape violation
/// -> Err (fail-dominant, no serve).
fn compose(orig: &[u8]) -> Result<Vec<u8>, String> {
    let mut bytes = orig.to_vec();

    // Gate 1: merge — the private nestmate call compiles to invokevirtual
    // (JEP 181); owner = ItemEntity. Kept tolerant to the special form too.
    let (out, outcome) = crate::classfile::retarget_invoke_to_static(
        &bytes,
        "tick",
        "()V",
        &[(
            "net/minecraft/world/entity/item/ItemEntity",
            "mergeWithNeighbours",
            "()V",
        )],
        (OPS_CLASS, "mergeWithNeighbours", OPS_DESC_MERGE),
    )?;
    check_gate("merge", &outcome)?;
    bytes = out;

    // Gate 2: settled-move extension — INHERITED `this.move(...)`: javac
    // emits the receiver's static type as Methodref owner (ItemEntity.move);
    // the declarator form (Entity.move) and a super-call (invokespecial) are
    // accepted for compiler-variant robustness.
    let (out, outcome) = crate::classfile::retarget_invoke_to_static(
        &bytes,
        "tick",
        "()V",
        &[
            (
                "net/minecraft/world/entity/item/ItemEntity",
                "move",
                MOVE_DESC,
            ),
            ("net/minecraft/world/entity/Entity", "move", MOVE_DESC),
        ],
        (OPS_CLASS, "move", OPS_DESC_MOVE),
    )?;
    check_gate("move", &outcome)?;
    bytes = out;

    // Gate 3: noCollision freshness — plain virtual dispatch on Level.
    let (out, outcome) = crate::classfile::retarget_invoke_to_static(
        &bytes,
        "tick",
        "()V",
        &[("net/minecraft/world/level/Level", "noCollision", NOCOLL_DESC)],
        (OPS_CLASS, "noCollision", OPS_DESC_NOCOLL),
    )?;
    check_gate("nocollision", &outcome)?;
    bytes = out;

    Ok(bytes)
}

fn check_gate(label: &str, outcome: &crate::classfile::RetargetOutcome) -> Result<(), String> {
    match outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {
            eprintln!("[crussty-plugin] items_stagger2: gate {label} retargeted (sites=1)");
            Ok(())
        }
        crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 } => {
            eprintln!("[crussty-plugin] items_stagger2: gate {label} already patched (sites=1)");
            Ok(())
        }
        other => Err(format!(
            "gate {label}: strict site-count violated ({other:?}) — fail-dominant"
        )),
    }
}

/// Background activation (fluid_bitmask/entity_compose discipline):
/// boot quiet -> ItemEntity visible -> hook registered -> bridge defined ->
/// signature probe -> pristine capture (no-op retransform) -> strict compose
/// -> READY -> retransform (serve) -> ARMED.
pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_stagger2: dormant (set CRUSSTY_LEVER_FLAG=items_stagger2 to enable)"
        );
        return;
    }
    let n = std::env::var("CRUSSTY_LEVER_ARG").unwrap_or_else(|_| "1".to_string());
    std::thread::Builder::new()
        .name("crussty-items-stagger2".into())
        .spawn(move || {
            // Boot must be quiet before any bridge define (loader-storm discipline).
            if !crate::improved_noise::wait_for_boot() {
                eprintln!("[crussty-plugin] items_stagger2: boot marker not seen, lever stays dormant");
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
                        "[crussty-plugin] items_stagger2: {ITEM_ENTITY} not loaded within 180s — lever stays dormant"
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
                            "[crussty-plugin] items_stagger2: defined {OPS_CLASS} in kernel loader"
                        );
                        true
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] items_stagger2: define_class({OPS_CLASS}) failed");
                        false
                    }
                }
            })
            .unwrap_or(false);
            if !defined {
                eprintln!("[crussty-plugin] items_stagger2: bridge definition aborted, lever stays dormant (vanilla)");
                return;
            }

            // Fail-dominant probe: static init throws if the private-handle
            // lookup failed -> probe returns None -> abort BEFORE retransform.
            match probe_signature() {
                Some(sig) => {
                    eprintln!("[crussty-plugin] items_stagger2: bridge probe ok ({sig})");
                }
                None => {
                    eprintln!(
                        "[crussty-plugin] items_stagger2: bridge signature probe FAILED (static init threw?) — lever stays dormant (vanilla)"
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
                eprintln!("[crussty-plugin] items_stagger2: no pristine bytes captured — lever stays dormant (vanilla)");
                return;
            };

            // Strict compose of the three gates (census-corrected patterns).
            match compose(&orig) {
                Ok(patched) => {
                    set_patch(std::sync::Arc::from(patched.into_boxed_slice()));
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_stagger2: compose rejected ({e}) — lever stays dormant (vanilla)"
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
                        "[crussty-plugin] items_stagger2: ARMED gates=merge,move,nocollision arg={n} (ItemEntity retransformed)"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            eprintln!(
                "[crussty-plugin] items_stagger2: retransform never returned 0 — lever NOT armed (vanilla)"
            );
        })
        .ok();
}
