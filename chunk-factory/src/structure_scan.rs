//! NCF P5.3-pre — I8 structure-fallback prescan (owner directive 2026-10-07:
//! "Пока P5.3 не готов, пометьте чанки рядом с возможным стартом
//! террейн-адаптирующей структуры как Java-fallback; радиус брать по
//! Beardifier и по типам структур").
//!
//! The gate-P2 honest-exclusion mechanism: chunks that the (still unported)
//! Beardifier.forStructuresInChunk could touch are moved OUT of the stagediff
//! corpus (both java + rust NBTs) BEFORE the compare, counted, and reported —
//! the same protocol as the T38-A spawn exclusion (ci_gate_p2.sh step 4b).
//! This is NOT a fix: it is invariant I8 (documented Java-fallback), the
//! coverage cost is reported per cell. P5.3 (piece engine) will shrink it to 0.
//!
//! Radius derivation (conservative, from the Beardifier decompile, CFR 0.152,
//! purpur-1.21.10):
//!   1. Beardifier.compute adds a piece contribution only where
//!      getBeardContribution's kernel index (d+12) is in [0,24) — i.e. the
//!      block is within 11 blocks of the piece box (BEARD_KERNEL_RADIUS=12,
//!      isInKernelRange cuts at |d| <= 11). Junctions likewise.
//!   2. Piece boxes are bounded by the JigsawPlacement.addPieces allowed AABB:
//!      center ± max_distance_from_center.horizontal (the start piece box
//!      itself is placed before the AABB exists — its extent is covered by
//!      the +16 chunk-pad term below).
//!   3. => a chunk (cx,cz) can see a non-zero beardifier only if its 16-block
//!      range intersects [Pcenter - (maxdist+11), Pcenter + (maxdist+11)].
//!      Conservative chunk radius: R = ceil((maxdist + 16 + 11)/16) + 1
//!      (villages/outposts/trail_ruins: 8; ancient_city/trial_chambers: 10).
//!
//! Start validity (prevents over-marking from never-generating placement
//! chunks): a start at placement chunk P is treated as GENERATING iff the
//! structure's biome predicate contains the overworld climate lookup at the
//! structure's stub position (replicating Structure.findValidGenerationPoint:
//! isValidBiome at the GenerationStub position):
//!   - project_start_to_heightmap WORLD_SURFACE_WG: y = h(P) + 1 +
//!     start_height.absolute  (addPieces: i2 = pos.y + getFirstFreeHeight,
//!     h = noise-fill WORLD_SURFACE_WG height at the chunk middle = our
//!     filler's hm_surface[(8,8)] first_available - 1)
//!   - no projection + absolute start_height: y = start_height.absolute
//!     (ancient_city -27)
//!   - no projection + uniform start_height a..b: y in {a, b} — BOTH checked
//!     (over-approximation; the real y is an RNG draw during assembly)
//!   - anything else (other heightmap types, above_bottom/below_top
//!     providers): validity UNKNOWN => chunk marked unconditionally
//!     (over-approximation) and the start is reported as "unknown-validity".
//!
//! Known gaps (loud, logged, honest — NOT silently dropped):
//!   - concentric_rings placement (strongholds): NOT scanned. Ring 0 radius =
//!     4*distance = 4*32 = 128 chunks (~2 km) — outside any current gate
//!     corpus (radius 24). Revisit when the corpus grows beyond ~100 chunks
//!     from origin.
//!   - exclusion_zone placement constraints are ignored => we may mark a
//!     placement chunk that vanilla would suppress (over-approximation, safe
//!     direction).
//!   - nether_fossil: nether-biome-gated; the biome check excludes it
//!     automatically (an overworld climate lookup never lands on a nether
//!     biome in the vanilla parameter list).

use crate::climate::{quantize_coord, TargetPoint};
use crate::random_spread::RandomSpreadStructurePlacement;
use crate::router::{RandomState, WorldgenDir};
use crate::vanilla_biomes;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

/// Beardifier kernel reach: getBeardContribution returns 0 for |d| >= 12
/// (isInKernelRange(d+12) requires 0 <= d+12 < 24).
pub const BEARDIFIER_KERNEL_RADIUS: i32 = 11;

#[derive(Clone, Debug)]
pub struct FallbackStart {
    pub structure: String,
    pub set: String,
    pub cx: i32,
    pub cz: i32,
    pub radius: i32,
    /// true when the validity check could not be performed (unknown
    /// start-height/projection form) — the chunk was marked conservatively.
    pub unknown_validity: bool,
}

#[derive(Default)]
pub struct FallbackReport {
    /// Marked chunk coords (sorted by BTreeSet order).
    pub chunks: BTreeSet<(i32, i32)>,
    pub starts: Vec<FallbackStart>,
    /// placement types / structures we deliberately do not scan (loud gaps).
    pub skipped: Vec<String>,
}

/// Conservative chunk radius for one structure. Exact derivation (purpur
/// 1.21.10 decompiles):
///   - pieces are bounded by the allowed AABB of JigsawPlacement.addPieces:
///     start-center ± max_distance_from_center (exclusive +1);
///   - the Beardifier kernel reaches 11 blocks beyond a piece box;
///   - chunk C (block range [16c, 16c+16)) intersects [Xc-(maxdist+11),
///     Xc+(maxdist+11)] with Xc = 16*P+8 iff c ∈ [P-6, P+6] for maxdist 80
///     (general: R = floor((maxdist + 11 + 15) / 16)).
pub fn radius_chunks(max_distance_from_center: i32) -> i32 {
    (max_distance_from_center + BEARDIFIER_KERNEL_RADIUS + 15) / 16
}

/// WorldgenRandom.setLargeFeatureSeed equivalent is already in
/// RandomSpreadStructurePlacement::potential_chunk (P5.1, verified).

// ---------------------------------------------------------------------------
// biome predicate resolution (tags read from the extract root, NOT via
// WorldgenDir — the dir walks only worldgen/ + dimension/)
// ---------------------------------------------------------------------------

fn tag_file(root: &Path, ns: &str, path: &str) -> Option<String> {
    let p = root.join("data").join(ns).join("tags/worldgen/biome").join(format!("{path}.json"));
    std::fs::read_to_string(p).ok()
}

pub(crate) fn resolve_biome_predicate(
    root: &Path,
    value: &crate::json::Json,
    out: &mut HashSet<String>,
    visiting: &mut Vec<String>,
    unknown: &mut bool,
) {
    if let Some(tag_ref) = value.as_str() {
        let Some(ref_str) = tag_ref.strip_prefix('#') else {
            out.insert(tag_ref.to_string());
            return;
        };
        if visiting.iter().any(|v| v == ref_str) {
            return; // cycle — stop
        }
        let Some((ns, path)) = ref_str.split_once(':') else {
            *unknown = true;
            return;
        };
        visiting.push(ref_str.to_string());
        match tag_file(root, ns, path).as_deref().and_then(|t| crate::json::parse(t).ok()) {
            Some(parsed) => {
                if let Some(values) = parsed.get("values") {
                    if let Some(items) = values.as_arr() {
                        for item in items {
                            resolve_biome_predicate(root, item, out, visiting, unknown);
                        }
                    }
                }
            }
            None => {
                // missing tag file: over-approximate (unknown validity)
                *unknown = true;
            }
        }
        visiting.pop();
        return;
    }
    if let Some(items) = value.as_arr() {
        for item in items {
            resolve_biome_predicate(root, item, out, visiting, unknown);
        }
        return;
    }
    if value.is_obj() {
        // object form is not a biome list; treat as unknown
        *unknown = true;
    }
}

// ---------------------------------------------------------------------------
// start_height parsing (v1: absolute + uniform of absolutes)
// ---------------------------------------------------------------------------

enum StartHeightSpec {
    /// {"absolute": n}
    Constant(i32),
    /// {"uniform": {min_inclusive: {absolute: a}, max_inclusive: {absolute: b}}}
    UniformRange(i32, i32),
    Unknown,
}

fn height_offset(v: Option<&crate::json::Json>) -> Option<i32> {
    let v = v?;
    if v.is_obj() {
        v.get("absolute").and_then(|a| a.as_i64()).map(|a| a as i32)
    } else {
        v.as_f64().map(|n| n as i32)
    }
}

fn parse_start_height(v: Option<&crate::json::Json>) -> StartHeightSpec {
    let Some(v) = v else { return StartHeightSpec::Constant(0) };
    if let Some(a) = height_offset(Some(v)) {
        return StartHeightSpec::Constant(a);
    }
    // typed form: {"type": "minecraft:uniform", "min_inclusive": {"absolute": a},
    // "max_inclusive": {"absolute": b}} — trial_chambers ships this exact shape.
    let t = v.get("type").and_then(|x| x.as_str());
    if t == Some("minecraft:uniform") {
        let lo = height_offset(v.get("min_inclusive"));
        let hi = height_offset(v.get("max_inclusive"));
        if let (Some(a), Some(b)) = (lo, hi) {
            return StartHeightSpec::UniformRange(a, b);
        }
        return StartHeightSpec::Unknown;
    }
    if t == Some("minecraft:constant") {
        return match height_offset(v.get("value")) {
            Some(a) => StartHeightSpec::Constant(a),
            None => StartHeightSpec::Unknown,
        };
    }
    // legacy inline form: {"uniform": {min_inclusive, max_inclusive}}
    if let Some(u) = v.get("uniform") {
        let lo = height_offset(u.get("min_inclusive"));
        let hi = height_offset(u.get("max_inclusive"));
        if let (Some(a), Some(b)) = (lo, hi) {
            return StartHeightSpec::UniformRange(a, b);
        }
    }
    StartHeightSpec::Unknown
}

// ---------------------------------------------------------------------------

/// The validity stub is the START-BOX CENTER: addPieces builds the start
/// piece box at blockPos1 = the CHUNK CORNER (template origin, extending
/// +x/+z) and checks the biome at (box.center.x, stub_y, box.center.z) —
/// NOT the chunk middle. The center offset = half the start template's
/// size, which we do not know before P5.3 — so sample a conservative GRID:
///   - projected structures (villages/outposts/trail_ruins, start templates
///     small): in-chunk offsets {0,4,8,12}^2, per-column WORLD_SURFACE_WG
///     height (getFirstFreeHeight is evaluated AT the box center column);
///   - non-projected (ancient_city city_center ~84 wide => half 42,
///     trial_chambers): offsets {0,16,32,48,64}^2 at the fixed stub y.
/// Any sample matching the biome predicate marks the start valid
/// (over-approximation). Residual false-negative risk: projected start
/// templates wider than 24 (center beyond corner+12) — a named red chunk
/// exposes it; P5.3 replaces this whole layer with real piece boxes.
pub(crate) fn stub_samples(
    structure: &crate::json::Json,
    height_of: &dyn Fn(i32, i32) -> Option<i32>,
) -> (Vec<(i32, i32, i32)>, bool) {
    let proj = structure
        .get("project_start_to_heightmap")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let sh = parse_start_height(structure.get("start_height"));
    let mut out = Vec::new();
    let mut unknown = false;
    match (proj.as_deref(), sh) {
        (Some("WORLD_SURFACE_WG") | Some("WORLD_SURFACE"), StartHeightSpec::Constant(off)) => {
            for dx in [0i32, 4, 8, 12] {
                for dz in [0i32, 4, 8, 12] {
                    if let Some(h) = height_of(dx, dz) {
                        out.push((dx, dz, h + 1 + off));
                    }
                }
            }
        }
        (Some(_), _) => unknown = true,
        (None, StartHeightSpec::Constant(off)) => {
            for dx in [0i32, 16, 32, 48, 64] {
                for dz in [0i32, 16, 32, 48, 64] {
                    out.push((dx, dz, off));
                }
            }
        }
        (None, StartHeightSpec::UniformRange(a, b)) => {
            for dx in [0i32, 16, 32, 48, 64] {
                for dz in [0i32, 16, 32, 48, 64] {
                    out.push((dx, dz, a));
                    out.push((dx, dz, b));
                }
            }
        }
        (None, StartHeightSpec::Unknown) => unknown = true,
    }
    (out, unknown)
}

/// Overworld climate lookup for a STRUCTURE biome check — the stub position
/// must be QUART-SNAPPED before the climate fields are evaluated.
///
/// Oracle chain (addendum 51): Java's Structure.isValidBiome (CFR
/// Structure.java 199-202) calls
/// `getNoiseBiome(QuartPos.fromBlock(stub.x), QuartPos.fromBlock(stub.y),
/// QuartPos.fromBlock(stub.z), sampler)` — QUART coords; and the
/// Climate.Sampler evaluates the density functions at QuartPos.toBlock(quart)
/// = quart*4 (see the filler.rs quart-fill comment — that path validated
/// bit-exact against java's saved chunk biomes). Net effect: the climate
/// scalars are sampled at the stub block coords SNAPPED DOWN to the 4-block
/// quart grid, NOT at the raw stub block. Evaluating at the raw stub diverges
/// from java near biome near-ties (seed 3053459 trail_ruins @ (-10,16):
/// raw stub (-158,48,254) -> depth q=210 -> taiga -> accept; snapped
/// (-160,48,252) -> depth q=112 -> river -> reject; java rejected — river is
/// not in #has_structure/trail_ruins — and the missing BURY fill was the
/// c_-10..-8 x 14..17 java=water rust=stone gate divergence).
pub(crate) fn biome_at(rs: &RandomState, handle: &mut BiomeListHandle, bx: i32, by: i32, bz: i32) -> String {
    let bx = bx >> 2 << 2;
    let by = by >> 2 << 2;
    let bz = bz >> 2 << 2;
    let fields: [&crate::density::Df; 6] = [
        &rs.router.temperature,
        &rs.router.vegetation,
        &rs.router.continents,
        &rs.router.erosion,
        &rs.router.depth,
        &rs.router.ridges,
    ];
    let mut vals = [0.0f64; 6];
    for (fi, f) in fields.iter().enumerate() {
        vals[fi] = f.compute(&rs.bank, bx, by, bz);
    }
    let [t, hu, co, er, de, wi] = [vals[0] as f32, vals[1] as f32, vals[2] as f32, vals[3] as f32, vals[4] as f32, vals[5] as f32];
    let target = TargetPoint {
        temperature: quantize_coord(t),
        humidity: quantize_coord(hu),
        continentalness: quantize_coord(co),
        erosion: quantize_coord(er),
        depth: quantize_coord(de),
        weirdness: quantize_coord(wi),
    };
    handle.find_value(&target).to_string()
}

/// Owner of the SHARED biome search list (RandomState::biome_list — built
/// once per RandomState) plus its own search memo. Memo chaining semantics
/// are unchanged: previously the `&mut ParameterList` carried the RTree
/// last-result hint across calls; now the handle carries it (Job 441690
/// SPEED LEVER — the ~7500-leaf tree must not be rebuilt per chunk).
pub struct BiomeListHandle {
    pub list: std::sync::Arc<crate::climate::ParameterList>,
    memo: Option<usize>,
}

impl BiomeListHandle {
    pub fn new(rs: &RandomState) -> Self {
        BiomeListHandle { list: rs.biome_list(), memo: None }
    }
    pub fn find_value(&mut self, target: &TargetPoint) -> &str {
        self.list.find_value(target, &mut self.memo)
    }
}

/// Scan one batch for I8-fallback chunks — v2 (P5.3 increment 5): the
/// marking decision comes from the REAL Beardifier feed, not from the v1
/// height/stub approximation (deleted; its over-approximation was the
/// class-(a) red-cell mechanism — chunks Java actually generated were
/// considered invalid and left unexcluded).
///
/// Marked (Java-fallback, excluded from the gate compare):
///   - `unfaithful`: SOME (set, P) in the chunk's createReferences +-8
///     neighborhood had a pick trigger that may diverge from Java
///     (non-jigsaw candidate drawn / unsupported start_height / unsupported
///     pool_alias type / pool key absent from the extract);
///   - `multi`: two or more adapting starts referenced (Java sums their
///     contributions in HashMap order — irreproducible last-bit sum order).
///
/// NOT marked (returned to the gate corpus, generated with the REAL
/// Beardifier): chunks with a faithful neighborhood — zero or exactly one
/// faithfully-assembled adapting start. Loud protocol:
///   - sets with non-random_spread placement are still UNSCANNED (loud gap,
///     unchanged — strongholds/concentric_rings);
///   - the per-chunk reason rides the FallbackStart report.
pub fn scan_batch(
    dir: &WorldgenDir,
    rs: &RandomState,
    seed: i64,
    root: &Path,
    x0: i32,
    x1: i32,
    z0: i32,
    z1: i32,
) -> Result<FallbackReport, String> {
    let mut report = FallbackReport::default();

    // Loud skipped-set report (same protocol as v1): sets we deliberately do
    // not scan (non-random_spread placement) — adapting structures inside
    // them remain an honest hole in the marking.
    for ns in dir.namespaces() {
        for set_name in dir.list(&ns, "structure_set") {
            let Some(text) = dir.get(&ns, "structure_set", &set_name) else {
                continue;
            };
            let Ok(j) = crate::json::parse(text) else {
                report.skipped.push(format!("{ns}:{set_name}: unparseable"));
                continue;
            };
            let placement_ok = j
                .get("placement")
                .and_then(|p| p.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("minecraft:random_spread")
                == "minecraft:random_spread";
            if !placement_ok {
                report
                    .skipped
                    .push(format!("{ns}:{set_name}: non-random_spread placement (unscanned gap)"));
            }
        }
    }

    let mut sampler = crate::height_feed::ColumnHeightSource::new(rs, seed);
    let mut feed = crate::piece_feed::BeardFeed::new(dir, root, rs, seed);
    let mut marked = 0usize;
    for cx in x0..=x1 {
        for cz in z0..=z1 {
            let (beard, diag) = feed.build_for_chunk_diag(&mut sampler, cx, cz);
            if diag.unfaithful || diag.referenced >= 2 {
                marked += 1;
                report.chunks.insert((cx, cz));
                report.starts.push(FallbackStart {
                    structure: if diag.unfaithful && diag.referenced >= 2 {
                        "unfaithful+multi-start".to_string()
                    } else if diag.unfaithful {
                        "unfaithful-pick".to_string()
                    } else {
                        "multi-start".to_string()
                    },
                    set: format!("feed: {} pieces", beard.pieces().len()),
                    cx,
                    cz,
                    radius: 0,
                    unknown_validity: diag.unfaithful,
                });
            }
        }
    }
    eprintln!(
        "[scan-v2] {} / {} batch chunks marked (feed-driven I8 narrowing)",
        marked,
        ((x1 - x0 + 1) * (z1 - z0 + 1)).max(0)
    );
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radii_match_the_kernel_derivation() {
        // villages/outposts/trail_ruins: maxdist 80 -> floor(106/16) = 6
        assert_eq!(radius_chunks(80), 6);
        // ancient_city/trial_chambers: maxdist 116 -> floor(142/16) = 8
        assert_eq!(radius_chunks(116), 8);
    }

    #[test]
    fn stub_samples_replicate_addpieces() {
        let height_of = |_dx: i32, _dz: i32| -> Option<i32> { Some(70) };
        // villages: y = h + 1 + 0 (per grid column)
        let j = crate::json::parse(r#"{"start_height":{"absolute":0},"project_start_to_heightmap":"WORLD_SURFACE_WG"}"#).unwrap();
        let (samples, unk) = stub_samples(&j, &height_of);
        assert_eq!(samples.len(), 16);
        assert!(samples.iter().all(|(dx, dz, y)| *y == 71 && *dx <= 12 && *dz <= 12));
        assert!(!unk);
        // trail_ruins: y = h + 1 - 15
        let j = crate::json::parse(r#"{"start_height":{"absolute":-15},"project_start_to_heightmap":"WORLD_SURFACE_WG"}"#).unwrap();
        let (samples, _) = stub_samples(&j, &height_of);
        assert!(samples.iter().all(|(_, _, y)| *y == 56));
        // ancient_city: no projection, absolute -27, wide grid
        let j = crate::json::parse(r#"{"start_height":{"absolute":-27}}"#).unwrap();
        let (samples, _) = stub_samples(&j, &height_of);
        assert_eq!(samples.len(), 25);
        assert!(samples.iter().all(|(_, _, y)| *y == -27));
        // trial_chambers: uniform -40..-20 -> both endpoints
        let j = crate::json::parse(r#"{"start_height":{"type":"minecraft:uniform","max_inclusive":{"absolute":-20},"min_inclusive":{"absolute":-40}}}"#).unwrap();
        let (samples, _) = stub_samples(&j, &height_of);
        assert_eq!(samples.len(), 50);
        assert!(samples.iter().any(|(_, _, y)| *y == -40));
        assert!(samples.iter().any(|(_, _, y)| *y == -20));
        // unknown projection -> unknown validity
        let j = crate::json::parse(r#"{"start_height":{"absolute":0},"project_start_to_heightmap":"OCEAN_FLOOR_WG"}"#).unwrap();
        let (_, unk) = stub_samples(&j, &height_of);
        assert!(unk);
    }

    #[test]
    fn village_start_for_the_canonical_seed_is_found() {
        // P5.1 verified: minecraft:villages (spacing 34, salt 10387312) has a
        // placement chunk at (-14,-15) for seed 3053459 (region (-1,-1)) —
        // the T38-B blob center (addendum 19).
        let v = RandomSpreadStructurePlacement {
            spacing: 34,
            separation: 8,
            salt: 10387312,
            spread_type: crate::random_spread::SpreadType::Linear,
            frequency: 1.0,
            frequency_reduction_method: crate::random_spread::FrequencyReductionMethod::Default,
            exclusion_zone: None,
        };
        let (cx, cz) = v.potential_chunk(3053459, -1, -1).unwrap();
        assert_eq!((cx, cz), (-14, -15));
    }
}
