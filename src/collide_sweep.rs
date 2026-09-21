//! Runtime wiring for the COLLIDE-SWEEP lever (TASK-400-F, vector
//! collidesweep — swept/batched block-collision data plane against the
//! collide lane, 8.68% java @150k × region_threads=4).
//!
//! Two redirect sites (RESEARCH-F.md for the upstream lineage — Lithium
//! PR #83/#84, Lithium ChunkAwareBlockCollisionSweeper, Paper
//! 322970283426/aff206ef815b/51bc97bd9037):
//!
//!   1. STATIC scan redirect (THIS module owns the byte hook):
//!      `CollisionUtil.getCollisionsForBlocksOrWorldBorder` body ->
//!      `CollideSweepOps.blockCollisions` (identical descriptor —
//!      redirect_static_method_body_to_static). The bridge is a bit-exact
//!      port of the Moonrise scan whose ONLY delta is the allocation-free
//!      single-AABB miss path (offset-intersect with the exact AABB.move
//!      six-add decomposition + the disassembled voxelShapeIntersect
//!      dcmpg/dcmpl ladder; moved AABB materialized on hit only, like
//!      vanilla's hit path).
//!   2. INSTANCE collide redirect (entity_compose STAGE 11 owns the Entity
//!      bytes — single-owner discipline, S7-162):
//!      private `Entity.collide(Vec3)` body -> `CollideSweepOps.collide`
//!      (receiver-prepended static; TravelDietOps v2a scalar-scratch
//!      contract) served by the compose chain when this module is armed.
//!
//! Family arbitration (fail-dominant, no half-composed stage): if
//! travel_diet is armed, STAGE 10 already redirects the same Entity body —
//! stage 11 declines with a loud WARN and the Entity stays travel-diet
//! (the CollisionUtil scan redirect is orthogonal and still arms).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp399_coll"` (lever protocol; the
//! cmp399_* family auto-activates the J items-subsystem per items_manager).
//! Off by default — dormant-invisible discipline: no hook, no bridge, no
//! patch, byte-indistinguishable from the pre-TASK-400-F plugin.
//!
//! Fail-closed matrix: bridge define failure -> READY never set -> both
//! hooks serve vanilla (stash-only); kernel shape mismatch -> patcher Err ->
//! that redirect skipped (loud WARN); travel_diet armed -> Entity stage
//! declines (scan redirect still arms); class predates hook -> pristine
//! capture via no-op retransform (fluid_guard pattern).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

const COLLISION_CLASS: &str = "ca/spottedleaf/moonrise/patches/collisions/CollisionUtil";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/CollideSweepOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/CollideSweepOps.class");

fn lever_flag() -> Option<String> {
    std::env::var("CRUSSTY_LEVER_FLAG").ok().map(|v| v.trim().to_string())
}

/// Armed only by the exact cmp399_coll flag (family semantics: any other
/// cmp399_* flag keeps this lever dormant; the J items-subsystem family gate
/// is items_manager's concern and stays independent).
pub fn enabled() -> bool {
    lever_flag().as_deref() == Some("cmp399_coll")
}

/// Gate visibility for the entity_compose stage 11.
pub fn enabled_pub() -> bool {
    enabled()
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Per-class state: pristine capture + computed patch + serve log flag.
/// Poison recovery (TASK-46): locks wrap plain Vec/Arc stores only.
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
}

impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        let mut orig = self.orig.lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(bytes.to_vec());
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn set_patch(&self, cache: PatchCache) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(cache);
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .map(|c| Arc::clone(&c.bytes))
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(COLLISION_CLASS))
}

/// Pollable gate for the entity_compose stage 11 (the Entity patch needs
/// the CollideSweepOps bridge DEFINED before the redirected body resolves
/// it on the first gated call).
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    READY.load(Ordering::Acquire)
}

/// Register the byte hook on CollisionUtil (idempotent; call once from
/// cplugin_init, AFTER alloc_diet::register so that when both levers are
/// armed the chain is alloc_diet-splice -> collide_sweep-redirect and the
/// redirect computes on the spliced bytes deterministically).
///
/// The callback performs NO JNI/class-file work (loader-lock discipline —
/// pristine capture at the class's own load, patch served from the cache
/// computed on the quiet activation worker).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] collide_sweep: dormant (CRUSSTY_LEVER_FLAG=cmp399_coll to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] cmp399_coll: collide_sweep sighted flag — hooking {COLLISION_CLASS} (scan redirect), Entity stage delegated to entity_compose stage 11"
    );
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (or upstream-patched sighting when
            // alloc_diet precedes us in the chain): stash for the worker;
            // never rewrite here.
            eprintln!(
                "[crussty-plugin] collide_sweep: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes)
                    .map(|(m, _)| m)
                    .unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed redirect; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] collide_sweep: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the kernel Entity class (the kernel
/// loader capture point), wait for boot, define CollideSweepOps into the
/// kernel loader, compute the scan redirect from the stashed bytes,
/// publish READY, retransform CollisionUtil, ARM marker.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // Wait for the kernel Entity class (loads at boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] collide_sweep: {ENTITY_CLASS} not loaded within 180s, bridge stays undefined"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] collide_sweep: forcing kernel load of {ENTITY_CLASS}"
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] collide_sweep: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] collide_sweep: server booted, defining {OPS_CLASS} into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let ops_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if ops_major > jvm_major {
            eprintln!(
                "[crussty-plugin] collide_sweep: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_collide_sweep_ops.sh; hook stays dormant"
            );
            return;
        }

        // Resolution closure (S7-164 NoSuchMethodError-storm guard): the
        // delivered classfile must declare exactly the two redirect statics.
        if let Err(e) = crate::classfile::collidesweep_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] collide_sweep: bridge resolution closure FAILED ({e}) — hook stays dormant"
            );
            return;
        }

        // Capture the kernel loader global ref from Entity.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] collide_sweep: defined {OPS_CLASS} in kernel loader");
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] collide_sweep: define_class({OPS_CLASS}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] collide_sweep: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard
        // pattern.
        let t = target();
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] collide_sweep: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] collide_sweep: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }
        let Some(bytes) = t.take_orig() else {
            return;
        };
        let orig_len = bytes.len();
        let major = crate::improved_noise::class_version(&bytes)
            .map(|(m, _)| m)
            .unwrap_or(0);

        // Compute the scan redirect (static → static, identical descriptor).
        let (patched, outcome) = match crate::classfile::patch_collision_sweep(&bytes) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] collide_sweep: scan redirect rejected ({e}) — CollisionUtil stays vanilla, Entity stage disarmed (fail-dominant)"
                );
                READY.store(true, Ordering::Release);
                return;
            }
        };
        match &outcome {
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 } => {}
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 } => {
                // Stale retransform replay — serve is idempotent.
            }
            other => {
                eprintln!(
                    "[crussty-plugin] collide_sweep: scan redirect strict check violated ({other:?}) — CollisionUtil stays vanilla, Entity stage disarmed (fail-dominant)"
                );
                READY.store(true, Ordering::Release);
                return;
            }
        }
        t.set_patch(PatchCache {
            bytes: Arc::from(patched),
            major,
        });
        crate::kernel_policy::audit_wire(
            COLLISION_CLASS,
            "scan-redirect",
            "collide_sweep v1",
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!(
            "[crussty-plugin] cmp399_coll: ARMED collide_sweep v1 scan_redirect=CollisionUtil.getCollisionsForBlocksOrWorldBorder->CollideSweepOps.blockCollisions collide_redirect=entity_compose.stage11 ({} -> {} bytes, retransform rc={rc})",
            orig_len,
            t.patch_bytes().map(|c| c.len()).unwrap_or(0)
        );
    });
}
