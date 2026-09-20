//! Runtime wiring for the ITEMS-INDEX lever (ROUND-396 / TASK-396-A, vector A,
//! lever_flag="items_index") — ARCHITECTURE SWAP for the TOP-1 bottleneck:
//! items/ItemEntity.tick 31.17% java + item-driven broadphase 15.66% (bank v4
//! baseline, 115655 samples).
//!
//! Vanilla cost: every ItemEntity, EVERY tick, runs mergeWithNeighbours ->
//! Level.getEntitiesOfClass(ItemEntity.class, AABB.inflate(merge,...), pred)
//! — a full O(section-population) broadphase scan (EntitySectionStorage
//! forEachAccessibleNonEmptySection + EntitySection/ClassInstanceMultiMap
//! full dump) for a 0.5-block AABB. ~70% of the bench population is items,
//! so this dominates both the items lane and the broadphase lane.
//!
//! Replacement (new data structure + algorithm, not a diet): a 1.0-grid
//! hashed index of ItemEntity buckets (ConcurrentHashMap<Long,ArrayList> +
//! per-entity current-bucket side table) maintained by the items' own ticked
//! move. Two STRICT receiver-prepended static retargets, both computed from
//! pristine bytes with strict sites==1 (region_threads discipline):
//!   1. ItemEntity.mergeWithNeighbours()V: the single
//!      Level.getEntitiesOfClass(Class;AABB;Predicate)List site ->
//!      ItemMergeIndexOps.getMergeCandidates(Level;Class;AABB;Predicate)List.
//!      Same candidate set by construction: bucket- SUPERSET enumeration +
//!      EXACT vanilla filter (box.intersects(e.getBoundingBox()) + the very
//!      Predicate instance vanilla passed).
//!   2. ItemEntity.tick()V: the single ItemEntity.move(MoverType;Vec3)V
//!      site (CP owner ItemEntity, tick offset 269) ->
//!      ItemMergeIndexOps.moveIndexed(ItemEntity;MoverType;Vec3)V = exact
//!      vanilla move + reconcile(). tick runs move (269) BEFORE merge (471),
//!      so the querying item is index-exact at its own query; positions only
//!      change through this move, so all indexed items are exact at every
//!      query (bounded documented deviation: <=1 tick for items spawned or
//!      teleported between ticks).
//!
//! Bridge: ItemMergeIndexOps defined into the KERNEL loader before any
//! patched bytes are served (S7-143 BRIDGE_READY lesson). Single classfile
//! (no nested classes; lambda = invokedynamic, S7-163 lesson).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_index"` (MEGA-ROUND generic gate,
//! run_world3.sh:459 export; lever_arg="1"). Absent/different flag = exact
//! vanilla path (no bytes, no bridge — parity by construction). The ops
//! class re-checks the same env at class-init (belt-and-braces).
//!
//! Cohabitation: NO bank-active lever registers a byte hook on ItemEntity
//! (census: Entity=fluid_guard/entity_compose chain, ServerLevel=F1/F3/
//! region_threads, Level=tickhook/region_threads) — this module is the sole
//! owner of the ItemEntity byte pipeline. Fails closed: any patch/define
//! error leaves the hook dormant.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const ITEM_ENTITY: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemMergeIndexOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../items/build/net/minecraft/world/entity/item/ItemMergeIndexOps.class");

// Vanilla CP refs (javap patched-kernel.jar ItemEntity):
//   #562 = Methodref net/minecraft/world/level/Level.getEntitiesOfClass:
//          (Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;
//   #410 = Methodref net/minecraft/world/entity/item/ItemEntity.move:
//          (Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
const MERGE_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "getEntitiesOfClass",
    "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
);
const MERGE_TO: (&str, &str, &str) = (
    "net/minecraft/world/entity/item/ItemMergeIndexOps",
    "getMergeCandidates",
    "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
);
const MOVE_FROM: (&str, &str, &str) = (
    "net/minecraft/world/entity/item/ItemEntity",
    "move",
    "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
);
const MOVE_TO: (&str, &str, &str) = (
    "net/minecraft/world/entity/item/ItemMergeIndexOps",
    "moveIndexed",
    "(Lnet/minecraft/world/entity/item/ItemEntity;Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
);

static ARMED: AtomicBool = AtomicBool::new(false);

pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok("items_index"))
}

struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

impl Target {
    const fn new(name: &'static str) -> Self {
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
        self.orig.lock().unwrap_or_else(PoisonError::into_inner).is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
    fn set_patch(&self, bytes: Vec<u8>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(Arc::from(bytes));
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

static TARGET_IE: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn ie_target() -> &'static Target {
    TARGET_IE.get_or_init(|| Target::new(ITEM_ENTITY))
}

/// Byte hook on ItemEntity: pristine capture on first sighting, patched
/// bytes served once READY (idempotent on later sightings/retransforms).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_index: dormant (CRUSSTY_LEVER_FLAG != items_index)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY, |_name, bytes| {
        let t = ie_target();
        if !ARMED.load(Ordering::Relaxed) {
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_index: pristine sighting {} {} bytes",
                    t.name,
                    bytes.len()
                );
            }
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_index: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Compute the two strict retargets (merge-query + post-move reconcile) from
/// pristine bytes. Both must hit exactly one site; anything else is a shape
/// mismatch -> Err (fail closed, hook dormant).
pub fn patch_item_entity_merge_index(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let (b1, o1) = crate::classfile::retarget_virtual_to_static(
        bytes,
        "mergeWithNeighbours",
        "()V",
        MERGE_FROM,
        MERGE_TO,
    )?;
    if !matches!(o1, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("merge-query site-count violated ({o1:?})"));
    }
    let (b2, o2) = crate::classfile::retarget_virtual_to_static(
        &b1,
        "tick",
        "()V",
        MOVE_FROM,
        MOVE_TO,
    )?;
    if !matches!(o2, crate::classfile::RetargetOutcome::Retargeted { sites: 1 }) {
        return Err(format!("move site-count violated ({o2:?})"));
    }
    Ok(b2)
}

/// Background activation (region_threads pattern): wait for ItemEntity,
/// define the ops bridge into the kernel loader, compute the strict patch,
/// flip ARMED, retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-index".into())
        .spawn(move || {
            let t = ie_target();

            // ItemEntity loads during bootstrap (entity type registration) or
            // first world entity tick; force-load fallback mirrors region_threads.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(t.name).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_index: {} not loaded within 180s, hook stays dormant",
                        t.name
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    eprintln!(
                        "[crussty-plugin] items_index: forcing kernel load of {}",
                        t.name
                    );
                    crate::improved_noise::force_load_kernel_class(t.name);
                }
                let sighted = cplug_sdk::classes::is_sighted(t.name);
                std::thread::sleep(std::time::Duration::from_millis(if sighted {
                    2_000
                } else {
                    10_000
                }));
            }

            if !crate::improved_noise::wait_for_boot() {
                eprintln!(
                    "[crussty-plugin] items_index: boot marker not seen, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
            eprintln!(
                "[crussty-plugin] items_index: server booted, defining bridge into kernel loader"
            );

            // Embedded-bytes vs JVM major guard (region_threads discipline).
            let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
                crate::improved_noise::jvm_class_major(env)
            })
            .flatten()
            .unwrap_or(u16::MAX);
            let ops_major = crate::improved_noise::class_version(OPS_BYTES)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if ops_major > jvm_major {
                eprintln!(
                    "[crussty-plugin] items_index: ops class major {ops_major} > JVM {jvm_major} — rebuild items/ via scripts/build_item_merge_ops.sh; hook stays dormant"
                );
                return;
            }

            // Define the bridge into the kernel loader (capture loader from
            // ItemEntity itself).
            let defined = cplug_sdk::jni_util::with_attached(|env| {
                let Some(cls) = cplug_sdk::classes::find_class(t.name) else {
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
                match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!(
                            "[crussty-plugin] items_index: defined {OPS_CLASS} in kernel loader"
                        );
                        true
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!(
                            "[crussty-plugin] items_index: define_class({OPS_CLASS}) failed"
                        );
                        false
                    }
                }
            });
            if !defined.unwrap_or(false) {
                eprintln!(
                    "[crussty-plugin] items_index: bridge definition aborted, hook stays dormant"
                );
                return;
            }

            // Pristine bytes (hook stash or no-op retransform capture).
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_index: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] items_index: no pristine bytes for {}, hook stays dormant",
                        t.name
                    );
                    return;
                }
            }

            let Some(orig) = t.take_orig() else { return };
            let patched = match patch_item_entity_merge_index(&orig) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] items_index: ItemEntity patch rejected ({e}), hook stays dormant"
                    );
                    return;
                }
            };
            eprintln!(
                "[crussty-plugin] items_index: computed patch (ItemEntity {} -> {} bytes; merge-query sites:1; move sites:1)",
                orig.len(),
                patched.len()
            );
            t.set_patch(patched);

            crate::kernel_policy::audit_wire(
                OPS_CLASS,
                "getMergeCandidates/moveIndexed",
                "items_index v1 (ROUND-396-A)",
            );
            ARMED.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!(
                "[crussty-plugin] items_index: ARMED, retransform rc ItemEntity={rc}"
            );
        })
        .ok();
}

#[cfg(test)]
mod items_index_tests {
    /// Source discipline: the bridge MUST declare ZERO nested classes — the
    /// kernel-loader delivery defines exactly ONE classfile (S7-163 leg#1
    /// TECH-DUD lesson). The single lambda compiles to invokedynamic.
    #[test]
    fn itemmerge_ops_source_declares_no_nested_classes() {
        let src = include_str!("../items/net/minecraft/world/entity/item/ItemMergeIndexOps.java");
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
                            .take_while(|c| c.is_alphanumeric() || c == '_')
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
            "ItemMergeIndexOps.java declares nested classes {declared:?} — kernel-loader \
             delivery defines exactly ONE classfile"
        );
    }

    /// Build-dir mirror: exactly one ItemMergeIndexOps classfile exists.
    #[test]
    fn itemmerge_build_dir_has_exactly_one_classfile() {
        let dir = "items/build/net/minecraft/world/entity/item";
        let mut count = 0;
        let rd = std::fs::read_dir(dir).expect("build dir present (run build_item_merge_ops.sh)");
        for e in rd.flatten() {
            let p = e.path().to_string_lossy().to_string();
            if p.contains("ItemMergeIndexOps") && p.ends_with(".class") {
                count += 1;
            }
        }
        assert_eq!(count, 1, "ItemMergeIndexOps classfile set drifted — rerun scripts/build_item_merge_ops.sh");
    }

    /// The embedded bytes ARE the built classfile (no stale embed).
    #[test]
    fn itemmerge_embedded_bytes_match_build_dir() {
        let on_disk =
            std::fs::read("items/build/net/minecraft/world/entity/item/ItemMergeIndexOps.class")
                .expect("built classfile present");
        assert_eq!(
            on_disk,
            super::OPS_BYTES,
            "embedded ItemMergeIndexOps.class is stale — rerun scripts/build_item_merge_ops.sh"
        );
    }

    /// Resolution closure: the embedded bridge declares BOTH retarget handles
    /// (getMergeCandidates + moveIndexed) with the exact receiver-prepended
    /// descriptors the byte patch emits.
    #[test]
    fn itemmerge_embedded_declares_all_redirect_targets() {
        let targets: &[(&str, &str, &str, &str)] = &[
            (
                "class",
                super::MERGE_TO.0,
                super::MERGE_TO.1,
                super::MERGE_TO.2,
            ),
            ("class", super::MOVE_TO.0, super::MOVE_TO.1, super::MOVE_TO.2),
        ];
        if let Err(e) = crate::classfile::itemmerge_resolution_closure(super::OPS_BYTES) {
            panic!("RESOLUTION CLOSURE FAILED: {e} — rebuild items/ via build_item_merge_ops.sh (targets {targets:?})");
        }
    }

    /// Scope lock: the retarget table is EXACTLY the two designed sites.
    #[test]
    fn itemmerge_redirect_table_is_exactly_two_sites() {
        assert_eq!(super::MERGE_FROM.1, "getEntitiesOfClass");
        assert_eq!(super::MERGE_FROM.2, "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;");
        assert_eq!(super::MERGE_TO.1, "getMergeCandidates");
        assert_eq!(
            super::MERGE_TO.2,
            "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;"
        );
        assert_eq!(super::MOVE_FROM.1, "move");
        assert_eq!(
            super::MOVE_FROM.2,
            "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V"
        );
        assert_eq!(super::MOVE_TO.1, "moveIndexed");
        assert_eq!(
            super::MOVE_TO.2,
            "(Lnet/minecraft/world/entity/item/ItemEntity;Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V"
        );
    }
}
