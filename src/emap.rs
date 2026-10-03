//! TASK-411-A k5b: ChunkMap.entityMap full-table race fence (emap).
//!
//! Root cause: docs/TASK411_A_AIOOBE_ROOTCAUSE.md — the unsynchronized
//! fastutil Int2ObjectOpenHashMap `ChunkMap.entityMap` is put from the
//! population phase and removed/probed from tick-phase region workers;
//! concurrent mutation desyncs fastutil's size/table invariant, the table
//! can fill past maxFill without rehash, and the open-addressing probe
//! loop never terminates (watchdog stuck containsKey:349 <-
//! ChunkMap.addEntity:953) while the MapIterator downscan walks key[--pos]
//! past slot 0 ("Index -1 ... length 131073").
//!
//! Fix (classfile::patch_chunkmap_entitymap): all 12 Int2ObjectMap call
//! sites in ChunkMap retargeted (length-preserving invokeinterface ->
//! invokestatic + 2 nops) to the static fence helpers in EntityMapOps —
//! monitor serialized on the MAP INSTANCE (put/remove/containsKey/get are
//! mutually exclusive => the table can never fill past maxFill => the
//! infinite probe and the -1 downscan are impossible by construction);
//! values() iterators are created under the map monitor and are
//! bound-checked/fail-dominant (EntityMapSafeItr). Vanilla semantics
//! bit-in-byte: same insertion/lookup/iteration order, same weakly
//! consistent iteration, same check-then-act window; only serialization
//! added. Any compose failure = ChunkMap completely vanilla (fail-dominant).
//!
//! Delivery (NavPoolOps pattern): three flat top-level classes (ZERO
//! nested, S7-163) defined into the KERNEL loader at region_threads
//! activation BEFORE the ChunkMap retransform; the emap compose stage runs
//! only if ALL THREE defines succeeded (probe-then-patch).
//!
//! ARM-CANON (Л-475-C52.1, ROUND-478-A11 audit): the fence arms on the
//! nav_plane union (27 uniq — the historical carrier levers) PLUS the
//! fresh-gen emap superset [EMAP_ARM_SUPERSET]. The ROUND-475 levers
//! cmp405_eindex / cmp475_itemidle / cmp475_c30conf ran OUTSIDE the union:
//! javap contracts show NONE of them mutates the 34 fenced sites (12
//! ChunkMap.entityMap Int2ObjectMap + 22 ReferenceList add/remove/contains
//! across 8 classes), yet every dispatch leg runs the region_threads=4
//! canon — so the population-vs-region-worker race (TASK-411-A watchdog
//! AIOOBE "Index -1", k5b; ReferenceList downscan AIOOBE ×3278, a-k5b run
//! 35698807454) is live UNFENCED on those legs = vanilla-emap inheritance.
//! That is a canon DELEGATION defect, not a lever defect: the fix widens
//! ONLY this gate. nav_plane.rs (nav lane: NavPlaneOps/NavPoolOps/
//! NodeEvaluator) stays untouched — fresh-gen legs must NOT inherit the
//! nav byte-payload, only the safety fence. Deliberately NOT a full union
//! (закон-5): unknown/empty flags stay vanilla-emap.

pub const EMAP_OPS_CLASS: &str = "net/minecraft/server/level/EntityMapOps";
pub const EMAP_VALUES_CLASS: &str = "net/minecraft/server/level/EntityMapSafeValues";
pub const EMAP_ITR_CLASS: &str = "net/minecraft/server/level/EntityMapSafeItr";

pub const EMAP_OPS_BYTES: &[u8] =
    include_bytes!("../bridges/entityinside/build/net/minecraft/server/level/EntityMapOps.class");
pub const EMAP_VALUES_BYTES: &[u8] =
    include_bytes!("../bridges/entityinside/build/net/minecraft/server/level/EntityMapSafeValues.class");
pub const EMAP_ITR_BYTES: &[u8] =
    include_bytes!("../bridges/entityinside/build/net/minecraft/server/level/EntityMapSafeItr.class");

/// Fresh-gen emap-arm superset (Л-475-C52.1 / ROUND-478-A11): the
/// ROUND-475 levers whose rt4 legs inherited vanilla-emap because the
/// historical delegation `emap::armed() -> nav_plane::armed()` predates
/// them. Enumerated allowlist — NOT a full union (закон-5). Extend ONLY by
/// audit (javap contract + rt-canon leg evidence) with a census-pinning
/// test update.
pub const EMAP_ARM_SUPERSET: [&str; 3] =
    ["cmp405_eindex", "cmp475_itemidle", "cmp475_c30conf"];

/// Pure superset comparator (STRICT trim-eq; testable without env
/// mutation). The nav union lives in nav_plane::armed() and is OR-ed on
/// top — its payload (NavPlaneOps/NavPoolOps/NodeEvaluator) stays off on
/// superset-only legs.
fn superset_arm(flag: Option<&str>) -> bool {
    match flag {
        Some(v) => EMAP_ARM_SUPERSET.contains(&v.trim()),
        None => false,
    }
}

/// STRICT emap-arm gate (Л-475-C52.1 canon + ROUND-478-A11 delegation fix):
/// the nav_plane union (27 uniq carriers) OR the fresh-gen emap superset.
/// Empty/unknown flags = vanilla-emap (fail-closed).
pub fn armed() -> bool {
    if crate::nav_plane::armed() {
        return true;
    }
    let v = std::env::var("CRUSSTY_LEVER_FLAG").ok();
    superset_arm(v.as_deref())
}

/// The full define list, in dependency order (values/itr reference ops
/// only via verification-time resolution — order is irrelevant for
/// linking, kept for log readability).
pub fn define_list() -> [(&'static str, &'static [u8]); 3] {
    [
        (EMAP_OPS_CLASS, EMAP_OPS_BYTES),
        (EMAP_VALUES_CLASS, EMAP_VALUES_BYTES),
        (EMAP_ITR_CLASS, EMAP_ITR_BYTES),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ROUND-478-A11 delegation fix: the superset census is pinned at the
    /// three audited ROUND-475 fresh-gen levers (javap contracts in the
    /// A11 audit: none mutates the 34 fenced sites, all run rt4 canon).
    /// Any extension must consciously update this test (закон-5: no full
    /// unions by drift).
    #[test]
    fn emap_arm_superset_census_pinned() {
        assert_eq!(EMAP_ARM_SUPERSET.len(), 3);
        assert_eq!(EMAP_ARM_SUPERSET[0], "cmp405_eindex");
        assert_eq!(EMAP_ARM_SUPERSET[1], "cmp475_itemidle");
        assert_eq!(EMAP_ARM_SUPERSET[2], "cmp475_c30conf");
    }

    /// STRICT trim-eq comparator contract: no prefix/suffix/whitespace
    /// tolerance beyond the symmetric trim; empty/unknown stay vanilla.
    #[test]
    fn emap_superset_is_strict_eq() {
        assert!(superset_arm(Some("cmp405_eindex")));
        assert!(superset_arm(Some("cmp475_itemidle")));
        assert!(superset_arm(Some("cmp475_c30conf")));
        assert!(superset_arm(Some("  cmp475_itemidle  "))); // trim contract (symmetric, mirrors nav union)
        assert!(superset_arm(Some(" cmp405_eindex"))); // trim contract (mirrors nav_plane::armed())
        assert!(!superset_arm(None));
        assert!(!superset_arm(Some("")));
        assert!(!superset_arm(Some("cmp405_eindex_x")));
        assert!(!superset_arm(Some("xcmp405_eindex")));
        assert!(!superset_arm(Some("CMP405_EINDEX"))); // case-sensitive (mirrors nav union)
        assert!(!superset_arm(Some("cmp401_collide")));
        assert!(!superset_arm(Some("cmp405_navplane"))); // nav-семья: через nav_plane::armed()
    }

    /// Blob-sync guard (check_blob_sync discipline): the embedded bytes
    /// MUST equal the on-disk build output — a rebuilt .java without a
    /// cargo rerun would silently deliver stale bytecode.
    #[test]
    fn emap_embedded_bytes_match_build_dir() {
        for (name, embedded) in define_list() {
            let file = format!("bridges/entityinside/build/{name}.class");
            let disk = std::fs::read(&file)
                .unwrap_or_else(|e| panic!("{file}: {e} (run scripts/build_emap_ops.sh)"));
            assert_eq!(
                embedded, disk,
                "embedded {name}.class is stale — rerun scripts/build_emap_ops.sh"
            );
        }
    }

    /// Delivery-set guard: EXACTLY three classfiles in the emap build dir
    /// (no nested classes — S7-163).
    #[test]
    fn emap_build_dir_has_exactly_three_classfiles() {
        let dir = "bridges/entityinside/build/net/minecraft/server/level";
        let n = std::fs::read_dir(dir)
            .expect("build dir present (run build_emap_ops.sh)")
            .filter_map(|e| e.ok())
            .filter(|e| {
                let p = e.file_name().to_string_lossy().to_string();
                p.starts_with("EntityMap") && p.ends_with(".class")
            })
            .count();
        assert_eq!(
            n, 3,
            "EntityMapOps classfile set drifted — must compile to exactly THREE classfiles"
        );
    }

    /// Sources declare ZERO nested classes (S7-163 discipline).
    #[test]
    fn emap_sources_declare_no_nested_classes() {
        for src in [
            "bridges/entityinside/net/minecraft/server/level/EntityMapOps.java",
            "bridges/entityinside/net/minecraft/server/level/EntityMapSafeValues.java",
            "bridges/entityinside/net/minecraft/server/level/EntityMapSafeItr.java",
        ] {
            let text = std::fs::read_to_string(src).expect(src);
            let mut declared: Vec<String> = Vec::new();
            for line in text.lines() {
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
                "{src} declares nested classes {declared:?} — kernel-loader \
                 define set would drift (S7-163)"
            );
        }
    }
}
