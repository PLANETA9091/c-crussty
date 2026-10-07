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

use crate::climate::{quantize_coord, ParameterList, TargetPoint};
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

fn resolve_biome_predicate(
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
fn stub_samples(
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

/// Overworld climate lookup at a block coord — mirrors filler.rs's
/// fillBiomesFromNoise path (quantize_coord on the f32 casts).
fn biome_at(rs: &RandomState, list: &mut ParameterList, bx: i32, by: i32, bz: i32) -> String {
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
    list.find_value(&target).to_string()
}

/// Scan one batch for structure-fallback chunks. `root` = the worldgen
/// extract (biome tags are read from <root>/data/<ns>/tags/worldgen/biome).
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
    let mut biome_list = ParameterList::new(match &rs.biome_points {
        Some(pts) => pts.clone(),
        None => vanilla_biomes::overworld_points()
            .into_iter()
            .map(|(p, n)| (p, n.to_string()))
            .collect(),
    });
    // noise-fill WORLD_SURFACE_WG heightmap per chunk (idx = x + z*16),
    // cached; heights feed the stub-grid for projected structures.
    let mut height_cache: HashMap<(i32, i32), Option<std::sync::Arc<[i32; 256]>>> = HashMap::new();
    let mut seen_starts: HashSet<(String, String, i32, i32)> = HashSet::new();

    for ns in dir.namespaces() {
        for set_name in dir.list(&ns, "structure_set") {
            let set_key = format!("{ns}:{set_name}");
            let Some(text) = dir.get(&ns, "structure_set", &set_name) else {
                continue;
            };
            let j = match crate::json::parse(text) {
                Ok(j) => j,
                Err(_) => {
                    report.skipped.push(format!("{set_key}: unparseable"));
                    continue;
                }
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
                    .push(format!("{set_key}: non-random_spread placement (known gap)"));
                continue;
            }
            let placement = match RandomSpreadStructurePlacement::parse(&j) {
                Ok(p) => p,
                Err(e) => {
                    report.skipped.push(format!("{set_key}: {e}"));
                    continue;
                }
            };
            let Some(structures) = j.get("structures") else {
                continue;
            };
            let Some(entries) = structures.as_arr() else {
                continue;
            };
            for entry in entries {
                let Some(skey) = entry.get("structure").and_then(|v| v.as_str()) else {
                    continue;
                };
                let Some((sns, spath)) = skey.split_once(':') else {
                    continue;
                };
                let Some(stext) = dir.get(sns, "structure", spath) else {
                    report.skipped.push(format!("{skey}: structure json missing"));
                    continue;
                };
                let sj = match crate::json::parse(stext) {
                    Ok(j) => j,
                    Err(_) => {
                        report.skipped.push(format!("{skey}: unparseable structure"));
                        continue;
                    }
                };
                let adapt = sj
                    .get("terrain_adaptation")
                    .and_then(|v| v.as_str())
                    .unwrap_or("none");
                if adapt == "none" {
                    continue;
                }
                let maxdist = match sj.get("max_distance_from_center") {
                    Some(v) => match (v.as_i64(), v.get("horizontal").and_then(|h| h.as_i64())) {
                        (_, Some(h)) => h as i32,
                        (Some(d), _) => d as i32,
                        _ => 80,
                    },
                    None => 80,
                };
                let r = radius_chunks(maxdist);
                // biome predicate
                let mut biomes: HashSet<String> = HashSet::new();
                let mut visiting: Vec<String> = Vec::new();
                let mut pred_unknown = false;
                if let Some(b) = sj.get("biomes") {
                    resolve_biome_predicate(root, b, &mut biomes, &mut visiting, &mut pred_unknown);
                } else {
                    pred_unknown = true;
                }
                let (lo_x, hi_x) = (x0 - r, x1 + r);
                let (lo_z, hi_z) = (z0 - r, z1 + r);
                for rx in lo_x.div_euclid(placement.spacing)..=hi_x.div_euclid(placement.spacing) {
                    for rz in lo_z.div_euclid(placement.spacing)..=hi_z.div_euclid(placement.spacing) {
                        let Some((px, pz)) = placement.potential_chunk_for_region(seed, rx, rz) else {
                            continue;
                        };
                        if px < lo_x || px > hi_x || pz < lo_z || pz > hi_z {
                            continue;
                        }
                        // validity: biome at the start-box-center grid
                        let mut valid = true;
                        let mut unknown_validity = pred_unknown;
                        if !pred_unknown {
                            // projected grids stay inside the candidate chunk:
                            // fetch its heightmap once, read columns from it
                            let hm: Option<std::sync::Arc<[i32; 256]>> = {
                                let needs_height = sj
                                    .get("project_start_to_heightmap")
                                    .and_then(|v| v.as_str())
                                    .is_some();
                                if needs_height {
                                    height_cache
                                        .entry((px, pz))
                                        .or_insert_with(|| {
                                            match crate::filler::generate_noise_chunk(rs, seed, px, pz)
                                            {
                                                Ok(fc) => {
                                                    let mut a = Box::new([0i32; 256]);
                                                    a.copy_from_slice(
                                                        &fc.heightmaps[1].first_available,
                                                    );
                                                    Some(std::sync::Arc::new(*a))
                                                }
                                                Err(_) => None,
                                            }
                                        })
                                        .clone()
                                } else {
                                    None
                                }
                            };
                            let height_of = |dx: i32, dz: i32| -> Option<i32> {
                                let col = ((px * 16 + dx) & 15) as usize
                                    + (((pz * 16 + dz) & 15) * 16) as usize;
                                hm.as_ref().map(|a| a[col] - 1)
                            };
                            let (samples, unk) = stub_samples(&sj, &height_of);
                            unknown_validity |= unk;
                            if samples.is_empty() {
                                valid = true; // unknown validity — over-approx
                            } else {
                                valid = samples.iter().any(|(dx, dz, y)| {
                                    let b = biome_at(
                                        rs,
                                        &mut biome_list,
                                        px * 16 + dx,
                                        *y,
                                        pz * 16 + dz,
                                    );
                                    biomes.contains(&b)
                                });
                            }
                        }
                        if !valid {
                            continue;
                        }
                        // dedupe: the same (set, structure, P) can be reached
                        // from overlapping region windows when R > spacing
                        let dedupe_key = (set_key.clone(), skey.to_string(), px, pz);
                        if !seen_starts.insert(dedupe_key) {
                            continue;
                        }
                        for cx in (px - r).max(x0)..=(px + r).min(x1) {
                            for cz in (pz - r).max(z0)..=(pz + r).min(z1) {
                                report.chunks.insert((cx, cz));
                            }
                        }
                        report.starts.push(FallbackStart {
                            structure: skey.to_string(),
                            set: set_key.clone(),
                            cx: px,
                            cz: pz,
                            radius: r,
                            unknown_validity,
                        });
                    }
                }
            }
        }
    }
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
        };
        let (cx, cz) = v.potential_chunk(3053459, -1, -1).unwrap();
        assert_eq!((cx, cz), (-14, -15));
    }
}
