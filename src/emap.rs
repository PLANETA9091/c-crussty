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

pub const EMAP_OPS_CLASS: &str = "net/minecraft/server/level/EntityMapOps";
pub const EMAP_VALUES_CLASS: &str = "net/minecraft/server/level/EntityMapSafeValues";
pub const EMAP_ITR_CLASS: &str = "net/minecraft/server/level/EntityMapSafeItr";

pub const EMAP_OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/EntityMapOps.class");
pub const EMAP_VALUES_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/EntityMapSafeValues.class");
pub const EMAP_ITR_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/EntityMapSafeItr.class");

/// STRICT eq lever gate — the k5b nav lane lever (same as navpool) AND-gated
/// with the TASK-413-A RACE-FENCE sub-gate: the emap+refsync fence arms only
/// when the lane lever is armed AND race_fence_on() (see below). Everything
/// else on the lane (navplane read plane, navpool, region-threads,
/// batch-collector, banking levers) is untouched by the sub-gate — that is
/// the exact A/B cut.
pub fn armed() -> bool {
    crate::nav_plane::armed() && race_fence_on()
}

/// TASK-413-A RACE-FENCE A/B sub-gate (quantify the fence CPU price, law 3):
/// leg1 = fence ON (this branch default), leg2 = fence OFF — SAME lever
/// `cmp405_navplane`, SAME bank inputs; the ONLY difference is this gate.
///
/// Semantics (fail-safe to ARMED — the fence is the validated spawn-race
/// fix, it must never silently vanish):
///  - compile-time default RACE_FENCE_DEFAULT below. The nofence dispatch
///    leg is a ONE-COMMIT flip of this const (branch-minus-fence): the
///    workflow has NO spare input slot for a race_fence input (25-input
///    GitHub limit, world-bench-parallel.yml) and no generic env passthrough;
///  - env override CRUSSTY_RACEFENCE: value "0" / "false" / "off"
///    (ASCII-insensitive) = OFF; anything else / unset = the default.
pub const RACE_FENCE_DEFAULT: bool = false; // TASK-413-A leg2 (nofence) capture point — re-armed in the next commit; the round-413-a-nofence alias pins THIS sha.

/// Pure decision kernel (unit-testable without touching process env):
/// (default_on, observed CRUSSTY_RACEFENCE value) -> fence on.
pub fn race_fence_decide(default_on: bool, env: Option<&str>) -> bool {
    if !default_on {
        return false;
    }
    match env {
        Some(v) => {
            let v = v.trim();
            !(v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("off"))
        }
        None => true,
    }
}

/// The live sub-gate (read once per armed() call; process env is
/// boot-stable in the harness, no caching needed).
pub fn race_fence_on() -> bool {
    race_fence_decide(
        RACE_FENCE_DEFAULT,
        std::env::var("CRUSSTY_RACEFENCE").ok().as_deref(),
    )
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

    /// TASK-413-A gate semantics: fail-safe to ARMED.
    #[test]
    fn race_fence_gate_semantics() {
        // default ON (the shipping branch): only explicit 0/false/off kills.
        assert!(race_fence_decide(true, None));
        assert!(race_fence_decide(true, Some("1")));
        assert!(race_fence_decide(true, Some("")));
        assert!(race_fence_decide(true, Some(" 1 ")));
        assert!(!race_fence_decide(true, Some("0")));
        assert!(!race_fence_decide(true, Some(" false ")));
        assert!(!race_fence_decide(true, Some("OFF")));
        assert!(!race_fence_decide(true, Some("Off")));
        // default OFF (the nofence leg): env cannot resurrect it.
        assert!(!race_fence_decide(false, None));
        assert!(!race_fence_decide(false, Some("1")));
        assert!(!race_fence_decide(false, Some("0")));
    }

    /// The gate is an AND on the lane lever — nothing else.
    #[test]
    fn armed_gate_is_lever_and_fence() {
        // Mirror of the armed() body: lever && race fence.
        let lever = crate::nav_plane::armed();
        let fence = race_fence_on();
        assert_eq!(armed(), lever && fence);
    }

    /// Blob-sync guard (check_blob_sync discipline): the embedded bytes
    /// MUST equal the on-disk build output — a rebuilt .java without a
    /// cargo rerun would silently deliver stale bytecode.
    #[test]
    fn emap_embedded_bytes_match_build_dir() {
        for (name, embedded) in define_list() {
            let file = format!("entityinside/build/{name}.class");
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
        let dir = "entityinside/build/net/minecraft/server/level";
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
            "entityinside/net/minecraft/server/level/EntityMapOps.java",
            "entityinside/net/minecraft/server/level/EntityMapSafeValues.java",
            "entityinside/net/minecraft/server/level/EntityMapSafeItr.java",
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
