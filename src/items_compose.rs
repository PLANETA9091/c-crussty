//! Runtime wiring for the ITEMS-COMPOSE lever (MEGA-ROUND-2 / TASK-397-A,
//! lever_flag="items_compose_ai") — COMPOSITION of the two round-1 ARMED
//! item levers on ONE ItemEntity byte pipeline:
//!
//!   - vector A (items_index, ROUND-396-A, +2.9% ARMED): 1.0-grid hashed
//!     ItemEntity merge-candidate index replaces the per-scan
//!     O(section-population) broadphase dump
//!     `Level.getEntitiesOfClass(ItemEntity.class, AABB.inflate(merge), pred)`
//!     — the index decides WHERE the neighbours are (bucket-superset
//!     enumeration + EXACT vanilla filter chain).
//!   - vector I (items_wakeup, ROUND-396-I, +4.0% ARMED): event-driven
//!     wakeup scheduling of the merge scans — the scan body runs ONLY on
//!     E1 first sighting / E2 displacement > 0.25 since last scan / E3
//!     one-shot wake / E4 post-merge neighbour wake (plus the unconditional
//!     post-teleport scan) — wakeup decides WHEN to scan at all.
//!
//! The levers are orthogonal (WHERE vs WHEN) and compose into FOUR strict
//! receiver-prepended static retargets, all computed from pristine bytes
//! with strict sites==1 (region_threads discipline; every site individually
//! proven ARMED in round-1 legs):
//!   1. `ItemEntity.mergeWithNeighbours()V`: the single
//!      `Level.getEntitiesOfClass(Class;AABB;Predicate)List` site
//!      (mergeWithNeighbours:67) -> `ItemsComposeOps.getMergeCandidates`.
//!      Served from the index, it runs ONLY when a wakeup event let the
//!      scan through (contract C1: the wakeup gate arms the index).
//!   2. `ItemEntity.tick()V`: the single `ItemEntity.move(MoverType;Vec3)V`
//!      site (tick:269, invokevirtual #410, CP-owner ItemEntity — verified
//!      by javap on the bank kernel, NOT invokespecial there) ->
//!      `ItemsComposeOps.moveIndexed` = exact vanilla move + O(1) reconcile
//!      (index position-exactness for OTHER kverying items + E4-wake reads).
//!      tick runs move BEFORE merge, so the querying item is index-exact at
//!      its own scan.
//!   3. `ItemEntity.tick()V`: the single `mergeWithNeighbours()V` site
//!      (tick:471) -> `ItemsComposeOps.mergeWithNeighbours(ItemEntity)` —
//!      E1/E2/E3 gate, then the REAL vanilla body via reflection delegate
//!      (zero logic drift).
//!   4. `ItemEntity.teleport(TeleportTransition)`: the single
//!      `mergeWithNeighbours()V` site (teleport:29) ->
//!      `ItemsComposeOps.mergeAfterTeleport(ItemEntity)` — unconditional
//!      scan, vanilla semantics kept exactly on site 2 of the wakeup lever.
//!
//! E4 neighbour wake enumerates the SAME grid buckets (superset AABB +
//! exact filter) — the composed path contains ZERO broadphase dumps
//! (contract C3; the standalone wakeup lever still used
//! getEntitiesOfClass there).
//!
//! Bridge: ItemsComposeOps defined into the KERNEL loader before any
//! patched bytes are served (BRIDGE_READY protocol, S7-143 LinkageError
//! lesson); the class self-tests its reflection delegate via JNI
//! (`selfTest()`) BEFORE the retransform is issued (no delegate = no
//! retransform = vanilla preserved by construction). Any post-retransform
//! delegate failure degrades to a skipped scan (never a fabricated merge,
//! never a crash).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_compose_ai"` (MEGA-ROUND generic
//! lever gate, run_world3.sh export; lever_arg="1"). Empty/other flag =
//! dormant vanilla passthrough — parity by construction. The ops class
//! re-checks the same env at class-init (belt-and-braces).
//!
//! Cohabitation: sole owner of the ItemEntity byte pipeline (round-1
//! census: no bank-active lever hooks ItemEntity bytes). Fails closed:
//! patcher Err (kernel shape mismatch), non-strict site count, bridge
//! define failure, selfTest failure -> no retransform, hook dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemsComposeOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../items_compose/build/net/minecraft/world/entity/item/ItemsComposeOps.class");

// The four retarget tables (all four CP triples verified by javap against
// the bank-v4 patched kernel: major 65, #410 move, #462 mergeWithNeighbours,
// #562 getEntitiesOfClass; ALL invoked via invokevirtual in tick/teleport/
// mergeWithNeighbours).
const MERGE_QUERY_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "getEntitiesOfClass",
    "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
);
const MERGE_QUERY_TO: (&str, &str, &str) = (
    OPS_CLASS,
    "getMergeCandidates",
    "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
);
const MOVE_FROM: (&str, &str, &str) = (
    ITEM_ENTITY,
    "move",
    "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
);
const MOVE_TO: (&str, &str, &str) = (
    OPS_CLASS,
    "moveIndexed",
    "(Lnet/minecraft/world/entity/item/ItemEntity;Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
);
const MERGE_VIRTUAL: (&str, &str, &str) = (ITEM_ENTITY, "mergeWithNeighbours", "()V");
const OPS_MERGE_DESC: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";
const OPS_TELEPORT_NAME: &str = "mergeAfterTeleport";
const TELEPORT_DESC: &str =
    "(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;";

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("items_compose_ai")
    )
}

static READY: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);

pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

/// Poison recovery (TASK-46): locks only wrap plain Option stores.
struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

impl Target {
    fn new() -> Self {
        Self {
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
    fn set_patch(&self, bytes: Vec<u8>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(Arc::from(bytes));
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(Target::new)
}

/// The FOUR retargets in one composition; strict site counts (fail-closed).
/// Each sub-retarget is byte-identical to a round-1 proven leg: (1)+(2) are
/// vector A's sites, (3)+(4) are vector I's sites.
fn patch_items_compose(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    // (1) merge-query site inside the vanilla scan body -> grid index.
    let (b1, o1) = crate::classfile::retarget_virtual_to_static(
        bytes,
        "mergeWithNeighbours",
        "()V",
        MERGE_QUERY_FROM,
        MERGE_QUERY_TO,
    )?;
    if !matches!(o1, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("merge-query site strict-1 violated ({o1:?})"));
    }
    // (2) tick's move site -> vanilla move + index reconcile.
    let (b2, o2) = crate::classfile::retarget_virtual_to_static(
        &b1,
        "tick",
        "()V",
        MOVE_FROM,
        MOVE_TO,
    )?;
    if !matches!(o2, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("move site strict-1 violated ({o2:?})"));
    }
    // (3) tick's scan site -> wakeup-gated scan.
    let (b3, o3) = crate::classfile::retarget_virtual_to_static(
        &b2,
        "tick",
        "()V",
        MERGE_VIRTUAL,
        (OPS_CLASS, "mergeWithNeighbours", OPS_MERGE_DESC),
    )?;
    if !matches!(o3, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("tick merge site strict-1 violated ({o3:?})"));
    }
    // (4) teleport's scan site -> unconditional scan (vanilla parity).
    let (out, o4) = crate::classfile::retarget_virtual_to_static(
        &b3,
        "teleport",
        TELEPORT_DESC,
        MERGE_VIRTUAL,
        (OPS_CLASS, OPS_TELEPORT_NAME, OPS_MERGE_DESC),
    )?;
    if !matches!(o4, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("teleport merge site strict-1 violated ({o4:?})"));
    }
    Ok((out, "merge-query+move+tick+teleport sites=1+1+1+1".to_string()))
}

/// Register the ItemEntity byte hook (idempotent; call once from
/// cplugin_init BEFORE ItemEntity can load).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_compose: dormant (CRUSSTY_LEVER_FLAG != items_compose_ai)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_compose: pristine sighting {ITEM_ENTITY} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_compose: hook serve {ITEM_ENTITY} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for boot + ItemEntity, define the
/// ItemsComposeOps bridge into the kernel loader, JNI-verify selfTest(),
/// compute the four retargets from pristine bytes, flip READY, retransform.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-compose".into())
        .spawn(move || {
            let t = target();
            // ItemEntity loads with the first entity chunk; force-load to
            // beat the population fixture (boot-time class-loading storm
            // discipline: force + wait_for_boot like every wiring module).
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(ITEM_ENTITY).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    eprintln!(
                        "[crussty-plugin] items_compose: forcing kernel load of {ITEM_ENTITY}"
                    );
                    crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_compose: {ITEM_ENTITY} not loaded within 180s, hook stays dormant"
                    );
                    return;
                }
                let sighted = cplug_sdk::classes::is_sighted(ITEM_ENTITY);
                std::thread::sleep(std::time::Duration::from_millis(if sighted {
                    2_000
                } else {
                    10_000
                }));
            }

            if !crate::improved_noise::wait_for_boot() {
                eprintln!(
                    "[crussty-plugin] items_compose: boot marker not seen, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(15));

            // Embedded bridge bytes must not be newer than the JVM.
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
                    "[crussty-plugin] items_compose: ops class major {ops_major} but JVM supports up to {jvm_major} — rebuild items_compose/ via scripts/build_items_compose_ops.sh; hook stays dormant"
                );
                return;
            }

            // Define the bridge into the KERNEL loader (captured from
            // ItemEntity itself — same-loader naming, the NCDFE lesson),
            // then JNI-verify selfTest() BEFORE any retransform.
            let defined = cplug_sdk::jni_util::with_attached(|env| {
                let Some(cls) = cplug_sdk::classes::find_class(ITEM_ENTITY) else {
                    crate::clear_exception(env);
                    return None;
                };
                let Some(class_cls) = env.find_class("java/lang/Class") else {
                    crate::clear_exception(env);
                    return None;
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
                    return None;
                };
                let gref = env.new_global_ref(loader);
                if gref.is_null() {
                    crate::describe_exception(env);
                    env.delete_local_ref(loader);
                    env.delete_local_ref(class_cls);
                    return None;
                }
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] items_compose: define_class({OPS_CLASS}) failed"
                    );
                    return None;
                };
                // Pre-retransform contract: the ops class must resolve its
                // vanilla-scan delegate and its index structures.
                let verdict = env
                    .get_static_method_id(c, "selfTest", "()Ljava/lang/String;")
                    .and_then(|mid| {
                        let r = env.call_static_object_method(c, mid, &[]);
                        if r.is_null() {
                            None
                        } else {
                            let s = env.get_string_utf(r as jni::jstring);
                            env.delete_local_ref(r);
                            s
                        }
                    });
                env.delete_local_ref(c);
                match verdict.as_deref() {
                    Some("OK") => {
                        eprintln!(
                            "[crussty-plugin] items_compose: defined {OPS_CLASS} in kernel loader (selfTest OK)"
                        );
                        Some(true)
                    }
                    other => {
                        eprintln!(
                            "[crussty-plugin] items_compose: selfTest rejected ({other:?}) — hook stays dormant"
                        );
                        Some(false)
                    }
                }
            });
            if defined.flatten() != Some(true) {
                eprintln!(
                    "[crussty-plugin] items_compose: bridge definition aborted, hook stays dormant"
                );
                return;
            }

            // Pristine bytes: if the class predates the hook (fast boot),
            // capture via no-op retransform (READY=false -> stash-only).
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_compose: {ITEM_ENTITY} predates hook, capturing via no-op retransform"
                );
                for _attempt in 1..=3 {
                    let _ = cplug_sdk::retransform_class(ITEM_ENTITY);
                    if t.orig_is_some() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                if !t.orig_is_some() {
                    eprintln!(
                        "[crussty-plugin] items_compose: no pristine bytes for {ITEM_ENTITY}, hook stays dormant"
                    );
                    return;
                }
            }

            // Compute the four retargets from the pristine bytes (pure rust,
            // strict site counts). Any Err = kernel shape mismatch -> fail
            // closed (vanilla).
            let Some(original) = t.take_orig() else {
                return;
            };
            let (patched, outcome) = match patch_items_compose(&original) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_compose: patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] items_compose: computed patch for {ITEM_ENTITY} ({} -> {} bytes, {outcome})",
                original.len(),
                patched.len()
            );
            t.set_patch(patched);

            crate::kernel_policy::audit_wire(
                OPS_CLASS,
                "getMergeCandidates/moveIndexed/mergeWithNeighbours/mergeAfterTeleport",
                "items_compose_ai v1 (TASK-397-A: A index + I wakeup)",
            );
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
            if rc == 0 {
                ARMED.store(true, Ordering::SeqCst);
            }
            eprintln!(
                "[crussty-plugin] items_compose: {ITEM_ENTITY} armed, retransform rc={rc}"
            );
        })
        .ok();
}

#[cfg(test)]
mod items_compose_tests {
    /// Source discipline: the bridge MUST declare ZERO nested classes — the
    /// kernel-loader delivery defines exactly ONE classfile (S7-163 lesson).
    /// The single lambda compiles to invokedynamic.
    #[test]
    fn itemscompose_ops_source_declares_no_nested_classes() {
        let src = include_str!("../items_compose/net/minecraft/world/entity/item/ItemsComposeOps.java");
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
            "ItemsComposeOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile"
        );
    }

    /// Build-dir mirror: exactly one ItemsComposeOps classfile exists.
    #[test]
    fn itemscompose_build_dir_has_exactly_one_classfile() {
        let dir = "items_compose/build/net/minecraft/world/entity/item";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_items_compose_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("ItemsComposeOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(
            count, 1,
            "ItemsComposeOps classfile set drifted — rerun scripts/build_items_compose_ops.sh"
        );
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn itemscompose_embedded_bytes_match_build_dir() {
        let on_disk =
            std::fs::read("items_compose/build/net/minecraft/world/entity/item/ItemsComposeOps.class")
                .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::OPS_BYTES,
            "embedded ItemsComposeOps.class is stale — rerun scripts/build_items_compose_ops.sh"
        );
    }

    /// Resolution closure: the embedded bridge declares ALL FOUR retarget
    /// handles with the exact receiver-prepended descriptors the byte patch
    /// emits.
    #[test]
    fn itemscompose_embedded_declares_all_redirect_targets() {
        if let Err(e) = crate::classfile::itemscompose_resolution_closure(super::OPS_BYTES) {
            panic!("RESOLUTION CLOSURE FAILED: {e} — rebuild items_compose/ via build_items_compose_ops.sh");
        }
    }

    /// Scope lock: the retarget table is EXACTLY the four designed sites.
    #[test]
    fn itemscompose_redirect_table_is_exactly_four_sites() {
        assert_eq!(super::MERGE_QUERY_FROM.1, "getEntitiesOfClass");
        assert_eq!(
            super::MERGE_QUERY_FROM.2,
            "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;"
        );
        assert_eq!(super::MERGE_QUERY_TO.1, "getMergeCandidates");
        assert_eq!(super::MOVE_FROM.1, "move");
        assert_eq!(
            super::MOVE_FROM.2,
            "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V"
        );
        assert_eq!(super::MOVE_TO.1, "moveIndexed");
        assert_eq!(super::MERGE_VIRTUAL.1, "mergeWithNeighbours");
        assert_eq!(super::MERGE_VIRTUAL.2, "()V");
        assert_eq!(super::OPS_MERGE_DESC, "(Lnet/minecraft/world/entity/item/ItemEntity;)V");
        assert_eq!(super::OPS_TELEPORT_NAME, "mergeAfterTeleport");
    }

    /// Patch-shape proof against the REAL bank-v4 kernel bytes (committed
    /// fixture extracted from research/gc-recon-2026-09-19/
    /// run-s7204-bitmask/patched-kernel.jar, major 65): all four retargets
    /// must hit exactly one site each, and the patch must be
    /// idempotency-safe (a second pass over the patched bytes reports
    /// AlreadyPatched, never a double rewrite).
    #[test]
    fn itemscompose_patches_real_kernel_bytes_strict_four_sites() {
        let fixture = include_bytes!("../items_compose/test/ItemEntity.kernel65.class");
        let (patched, outcome) = super::patch_items_compose(fixture)
            .expect("all four retargets strict-1 on real kernel bytes");
        assert!(outcome.contains("1+1+1+1"), "outcome {outcome}");
        assert!(patched.len() > fixture.len(), "pool grew for the four static refs");

        // Idempotency: the patched bytes re-sighted must NOT re-patch —
        // each retarget now resolves to its `to` (AlreadyPatched), which
        // the strict-1 gate rejects loudly instead of double-rewriting.
        let second = super::patch_items_compose(&patched);
        assert!(
            second.is_err(),
            "re-patching patched bytes must fail strict-1 (AlreadyPatched), not double-patch"
        );
    }
}
