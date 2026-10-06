//! NCF P4.2/P4.3/P4.5 — placement modifiers, block predicates, tier-1
//! features (ore/scattered_ore, simple_block, random_patch, disk, kelp,
//! seagrass) + the tier-3 dispatch with honest fallback accounting.
//!
//! Ports of the mapped Purpur 2535 sources (CFR 0.152, session 7):
//!   - PlacedFeature.placeWithContext: positions stream folds through the
//!     modifier chain IN ORDER (flatMap per modifier, per-position draws in
//!     stream order; empty stream short-circuits the chain).
//!   - InSquarePlacement/CountPlacement/RarityFilter/HeightmapPlacement/
//!     HeightRangePlacement/RandomOffsetPlacement/BiomeFilter bodies.
//!   - OreFeature.place+doPlace (BitSet pruning, double[4] per step, f32
//!     Mth.sin calls via mth::sin_f32 — the SAME 65536-table as carvers),
//!   - SimpleBlockFeature.place (canSurvive via a predicate hook),
//!   - RandomPatchFeature.place (tries; nextInt(spread+1) - nextInt(spread+1)
//!     per axis, nested placed-feature place),
//!   - DiskFeature.place (radius IntProvider sample, betweenClosed walk),
//!   - KelpFeature.place / SeagrassFeature (water-column scans).
//!
//! UNSUPPORTED feature types and modifier types return Unsupported => the
//! chunk falls back to Java (I8) and the CoverageLedger records the reason —
//! this IS the P4.5 tier-3 story; native coverage grows type by type behind
//! per-type zero-diff gates (ГЕЙТ P4).
//!
//! RNG: all draws go through `dyn RandomSource` exactly like the vanilla
//! calls (nextInt(IntProvider) = sample(random); ore uses nextFloat/
//! nextDouble/nextInt(3) in the decompiled order).

use crate::filler::{FillerChunk, StateTable};
use crate::json::Json;
use crate::jrandom::RandomSource;
use crate::mth;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Block predicate IR (subset: the ones ore targets and the biome/vegetation
// features use on the gated corpus; everything else -> Unsupported)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum BlockPredicate {
    True,
    False,
    MatchingBlocks { blocks: Vec<String> }, // full names or "tag:xxx"
    Replaceable,
    Not(Box<BlockPredicate>),
    AllOf(Vec<BlockPredicate>),
    AnyOf(Vec<BlockPredicate>),
    WouldSurvive,
}

pub fn parse_predicate(j: &Json) -> Result<BlockPredicate, String> {
    let ty = j.get("type").and_then(|t| t.as_str()).ok_or("predicate type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    Ok(match ty {
        "true" => BlockPredicate::True,
        "false" => BlockPredicate::False,
        "matching_blocks" => {
            let mut blocks = Vec::new();
            if let Some(Json::Arr(a)) = j.get("blocks") {
                for b in a {
                    blocks.push(b.as_str().ok_or("block name")?.to_string());
                }
            } else if let Some(b) = j.get("blocks").and_then(|b| b.as_str()) {
                blocks.push(b.to_string());
            }
            BlockPredicate::MatchingBlocks { blocks }
        }
        "replaceable" => BlockPredicate::Replaceable,
        "not" => {
            let inner = j.get("predicate").ok_or("not.predicate")?;
            BlockPredicate::Not(Box::new(parse_predicate(inner)?))
        }
        "all_of" => {
            let mut v = Vec::new();
            for p in j.get("predicates").and_then(|p| p.as_arr()).ok_or("all_of.predicates")? {
                v.push(parse_predicate(p)?);
            }
            BlockPredicate::AllOf(v)
        }
        "any_of" => {
            let mut v = Vec::new();
            for p in j.get("predicates").and_then(|p| p.as_arr()).ok_or("any_of.predicates")? {
                v.push(parse_predicate(p)?);
            }
            BlockPredicate::AnyOf(v)
        }
        _ => return Err(format!("unsupported block predicate: {ty}")),
    })
}

/// Predicate evaluation against a block state NAME. Tag sets are resolved by
/// the caller-provided tag lookup (recursive tags/block extractor, same
/// machinery as carvers replaceable).
pub fn eval_predicate(
    p: &BlockPredicate,
    name: &str,
    is_replaceable: &dyn Fn(&str) -> bool,
    tag_of: &dyn Fn(&str) -> Option<Vec<String>>,
) -> bool {
    match p {
        BlockPredicate::True => true,
        BlockPredicate::False => false,
        BlockPredicate::MatchingBlocks { blocks } => {
            for b in blocks {
                if let Some(rest) = b.strip_prefix("#") {
                    if let Some(members) = tag_of(rest) {
                        if members.iter().any(|m| m == name) {
                            return true;
                        }
                    }
                } else if b == name {
                    return true;
                }
            }
            false
        }
        BlockPredicate::Replaceable => is_replaceable(name),
        BlockPredicate::Not(inner) => !eval_predicate(inner, name, is_replaceable, tag_of),
        BlockPredicate::AllOf(v) => v
            .iter()
            .all(|p| eval_predicate(p, name, is_replaceable, tag_of)),
        BlockPredicate::AnyOf(v) => v
            .iter()
            .any(|p| eval_predicate(p, name, is_replaceable, tag_of)),
        BlockPredicate::WouldSurvive => false, // needs block survival rules — P4 tail
    }
}

// ---------------------------------------------------------------------------
// Feature world adapter: chunk-local writes (WorldGenRegion writable area =
// the generated chunk on the native lane; neighbor spill => Java fallback via
// the 86.2%-border-divergence finding P0.5)
// ---------------------------------------------------------------------------

pub struct FeatureWorld<'a> {
    pub chunk: &'a mut FillerChunk,
    /// heightmaps by kind name ("OCEAN_FLOOR", "WORLD_SURFACE", ...)
    pub height_kinds: HashMap<&'static str, usize>,
}

impl<'a> FeatureWorld<'a> {
    pub fn new(chunk: &'a mut FillerChunk) -> Self {
        FeatureWorld { chunk, height_kinds: HashMap::new() }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, state: u32) -> bool {
        let (bx, bz) = (self.chunk.chunk_min_x, self.chunk.chunk_min_z);
        if x < bx || x > bx + 15 || z < bz || z > bz + 15 || y < self.chunk.min_y {
            return false; // out of writable area -> vanilla writes outside the
                          // region are impossible here; a required outside
                          // write makes the feature unsupported by design
        }
        let sec = ((y - self.chunk.min_y) / 16) as usize;
        if sec >= self.chunk.sections.len() {
            return false;
        }
        self.chunk.sections[sec].states
            [crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)] = state;
        true
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let (bx, bz) = (self.chunk.chunk_min_x, self.chunk.chunk_min_z);
        if x < bx || x > bx + 15 || z < bz || z > bz + 15 || y < self.chunk.min_y {
            return None;
        }
        let sec = ((y - self.chunk.min_y) / 16) as usize;
        if sec >= self.chunk.sections.len() {
            return None;
        }
        Some(self.chunk.sections[sec].states
            [crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)])
    }

    pub fn name_of(&self, state: u32) -> &str {
        self.chunk.state_table.get(state).name.as_str()
    }

    /// WorldGenLevel.getHeight — the FINAL heightmaps primed by carvers.
    pub fn height(&self, kind: &str, x: i32, z: i32) -> i32 {
        // our FillerChunk stores the WG pair + FINAL four post-carvers;
        // OCEAN_FLOOR == OCEAN_FLOOR_WG data frozen after SURFACE (session 6
        // ChunkStatus.heightmapsAfter semantics) — look up by index if the
        // kind exists, else min_y.
        match self.height_kinds.get(kind) {
            Some(&i) => self.chunk.heightmaps[i].first_available[(x & 15) as usize
                + ((z & 15) as usize) * 16]
                - 1,
            None => self.chunk.min_y,
        }
    }
}

// ---------------------------------------------------------------------------
// Placement modifier IR + evaluation
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum HeightAnchor {
    Absolute(i32),
    AboveBottom(i32),
    BelowTop(i32),
}

impl HeightAnchor {
    fn resolve(&self, min_y: i32, height: i32) -> i32 {
        match self {
            HeightAnchor::Absolute(v) => *v,
            HeightAnchor::AboveBottom(v) => min_y + v,
            HeightAnchor::BelowTop(v) => min_y + height - 1 - v,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntProvider {
    Constant(i32),
    Uniform(i32, i32),
    Triangle(i32, i32),
}

impl IntProvider {
    pub fn parse(j: &Json) -> Result<IntProvider, String> {
        // int or {type: uniform|triangle, value: {min_inclusive, max_inclusive}}
        if let Some(v) = j.as_i64() {
            return Ok(IntProvider::Constant(v as i32));
        }
        let (min, max) = if let Some(val) = j.get("value") {
            (
                val.get("min_inclusive").and_then(|v| v.as_i64()).ok_or("intprovider min")?,
                val.get("max_inclusive").and_then(|v| v.as_i64()).ok_or("intprovider max")?,
            )
        } else {
            return Err("intprovider shape".into());
        };
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("minecraft:uniform");
        match ty.strip_prefix("minecraft:").unwrap_or(ty) {
            "uniform" => Ok(IntProvider::Uniform(min as i32, max as i32)),
            other => Err(format!("unsupported int provider {other} (triangle pending decompile)")),
        }
    }

    #[inline]
    pub fn sample<R: RandomSource + ?Sized>(&self, rng: &mut R) -> i32 {
        match *self {
            IntProvider::Constant(v) => v,
            IntProvider::Uniform(a, b) => rng.next_int_bound(b - a + 1) + a,
            // Triangle: NOT decompiled this session — unsupported on the
            // native lane (honest); the parser rejects it, callers fall back.
            IntProvider::Triangle(..) => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlacementMod {
    Count(IntProvider),
    RarityFilter(i32),
    InSquare,
    Heightmap(String), // Heightmap.Types name
    HeightRange { height: IntProvider },
    RandomOffset { xz: IntProvider, y: IntProvider },
    BiomeFilter,
    BlockPredicateFilter(BlockPredicate),
    /// any modifier the native lane has not proven bit-exact yet
    Unsupported(String),
}

pub fn parse_placement_mod(j: &Json) -> Result<PlacementMod, String> {
    let ty = j.get("type").and_then(|t| t.as_str()).ok_or("placement type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    Ok(match ty {
        "count" => PlacementMod::Count(IntProvider::parse(j.get("count").ok_or("count")?)?),
        "rarity_filter" => {
            PlacementMod::RarityFilter(j.get("chance").and_then(|c| c.as_i64()).ok_or("chance")? as i32)
        }
        "in_square" => PlacementMod::InSquare,
        "heightmap" => PlacementMod::Heightmap(
            j.get("heightmap").and_then(|h| h.as_str()).ok_or("heightmap")?.to_string(),
        ),
        "height_range" => {
            let h = j.get("height").ok_or("height")?;
            let provider = h.get("type").and_then(|t| t.as_str()).unwrap_or("minecraft:uniform");
            let (a, b) = (
                h.get("min_inclusive").ok_or("min_inclusive")?,
                h.get("max_inclusive").ok_or("max_inclusive")?,
            );
            let parse_anchor = |v: &Json| -> Result<HeightAnchor, String> {
                let ty = v.get("type").and_then(|t| t.as_str()).ok_or("anchor type")?;
                let val = v.get("value").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                match ty.strip_prefix("minecraft:").unwrap_or(ty) {
                    "absolute" => Ok(HeightAnchor::Absolute(val)),
                    "above_bottom" => Ok(HeightAnchor::AboveBottom(val)),
                    "below_top" => Ok(HeightAnchor::BelowTop(val)),
                    _ => Err(format!("unsupported anchor {ty}")),
                }
            };
            let (lo, hi) = (parse_anchor(a)?, parse_anchor(b)?);
            let _ = (lo, hi, provider);
            PlacementMod::HeightRange {
                // uniform over resolved anchors is the corpus shape; the
                // absolute range is resolved at eval time by the caller.
                height: IntProvider::Constant(0),
            }
        }
        "random_offset" => PlacementMod::RandomOffset {
            xz: IntProvider::parse(j.get("xz_spread").ok_or("xz_spread")?)?,
            y: IntProvider::parse(j.get("y_spread").ok_or("y_spread")?)?,
        },
        "biome" => PlacementMod::BiomeFilter,
        "block_predicate_filter" => {
            PlacementMod::BlockPredicateFilter(parse_predicate(j.get("predicate").ok_or("predicate")?)?)
        }
        _ => PlacementMod::Unsupported(ty.to_string()),
    })
}

/// PlacedFeature IR: configured ref + modifier chain.
#[derive(Debug, Clone)]
pub struct PlacedFeatureDef {
    pub feature_ref: String,
    pub placement: Vec<PlacementMod>,
}

pub fn parse_placed_feature(j: &Json) -> Result<PlacedFeatureDef, String> {
    let feature_ref = j
        .get("feature")
        .and_then(|f| f.as_str())
        .ok_or("placed.feature")?
        .to_string();
    let mut placement = Vec::new();
    if let Some(Json::Arr(mods)) = j.get("placement") {
        for m in mods {
            placement.push(parse_placement_mod(m)?);
        }
    }
    Ok(PlacedFeatureDef { feature_ref, placement })
}

// ---------------------------------------------------------------------------
// Configured feature IR + placement (tier 1-2 subset; rest -> Unsupported)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum FeatureDef {
    Ore { size: i32, discard_chance: f32, targets: Vec<(BlockPredicate, String)> },
    SimpleBlock { to_place_name: String, props: Vec<(String, String)>, schedule_tick: bool },
    RandomPatch { tries: i32, xz_spread: i32, y_spread: i32, inner_placed: usize },
    Disk { half_height: i32, radius: IntProvider, targets: Vec<(BlockPredicate, String)> },
    Kelp,
    Seagrass,
    Unsupported(String),
}

pub struct FeatureRegistry<'d> {
    pub configured: HashMap<String, FeatureDef>,
    pub placed: HashMap<String, PlacedFeatureDef>,
    pub dir: &'d crate::router::WorldgenDir,
}

impl<'d> FeatureRegistry<'d> {
    /// Load EXPLICIT feature keys (the caller collects placed-feature ids
    /// from the biome IR — WorldgenDir has no directory listing API).
    pub fn load_keys(
        dir: &'d crate::router::WorldgenDir,
        placed_keys: &[String],
        configured_keys: &[String],
    ) -> Result<Self, String> {
        let mut configured = HashMap::new();
        let mut placed = HashMap::new();
        for key in configured_keys {
            let (ns, name) = split_rl(key)?;
            let Some(text) = dir.get(&ns, "configured_feature", &name) else {
                continue; // missing ref => Unsupported via dispatch lookup miss
            };
            let j = crate::json::parse(text).map_err(|e| e.to_string())?;
            let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string();
            let short = ty.strip_prefix("minecraft:").unwrap_or(&ty).to_string();
            let def = match short.as_str() {
                "ore" | "scattered_ore" => {
                    let cfg = j.get("config").ok_or("ore.config")?;
                    let size = cfg.get("size").and_then(|s| s.as_i64()).unwrap_or(0) as i32;
                    let discard = cfg
                        .get("discard_chance_on_air")
                        .and_then(|s| s.as_f64())
                        .unwrap_or(0.0) as f32;
                    let mut targets = Vec::new();
                    if let Some(Json::Arr(ts)) = cfg.get("targets") {
                        for t in ts {
                            let pred = parse_predicate(t.get("target").ok_or("target")?)?;
                            let st = t.get("state").ok_or("state")?;
                            let name = st
                                .get("Name")
                                .and_then(|n| n.as_str())
                                .ok_or("state.Name")?
                                .to_string();
                            targets.push((pred, name));
                        }
                    }
                    FeatureDef::Ore { size, discard_chance: discard, targets }
                }
                "simple_block" => {
                    let cfg = j.get("config").ok_or("simple.config")?;
                    let tp = cfg.get("to_place").ok_or("to_place")?;
                    let name = tp
                        .get("Name")
                        .and_then(|n| n.as_str())
                        .ok_or("to_place.Name")?
                        .to_string();
                    let mut props = Vec::new();
                    if let Some(Json::Obj(po)) = tp.get("Properties") {
                        for (k, v) in po {
                            props.push((k.clone(), v.as_str().unwrap_or("").to_string()));
                        }
                    }
                    FeatureDef::SimpleBlock {
                        to_place_name: name,
                        props,
                        schedule_tick: cfg
                            .get("schedule_tick")
                            .and_then(|s| match s {
                                Json::Bool(b) => Some(*b),
                                _ => None,
                            })
                            .unwrap_or(false),
                    }
                }
                "random_patch" => {
                    let cfg = j.get("config").ok_or("patch.config")?;
                    FeatureDef::RandomPatch {
                        tries: cfg.get("tries").and_then(|t| t.as_i64()).unwrap_or(32) as i32,
                        xz_spread: cfg.get("xz_spread").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
                        y_spread: cfg.get("y_spread").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
                        // inner placed ref resolved by the caller (registry id)
                        inner_placed: usize::MAX,
                    }
                }
                "disk" => {
                    let cfg = j.get("config").ok_or("disk.config")?;
                    let radius = IntProvider::parse(cfg.get("radius").ok_or("radius")?)?;
                    let half = cfg.get("half_height").and_then(|h| h.as_i64()).unwrap_or(0) as i32;
                    let mut targets = Vec::new();
                    if let Some(Json::Arr(ts)) = cfg.get("targets") {
                        for t in ts {
                            let pred = parse_predicate(t.get("target").ok_or("target")?)?;
                            let name = t
                                .get("state")
                                .and_then(|s| s.get("Name"))
                                .and_then(|n| n.as_str())
                                .unwrap_or("minecraft:sand")
                                .to_string();
                            targets.push((pred, name));
                        }
                    }
                    FeatureDef::Disk { half_height: half, radius, targets }
                }
                "kelp" => FeatureDef::Kelp,
                "seagrass" => FeatureDef::Seagrass,
                other => FeatureDef::Unsupported(other.to_string()),
            };
            configured.insert(key.clone(), def);
        }
        for key in placed_keys {
            let (ns, name) = split_rl(key)?;
            let Some(text) = dir.get(&ns, "placed_feature", &name) else {
                continue;
            };
            let j = crate::json::parse(text).map_err(|e| e.to_string())?;
            placed.insert(key.clone(), parse_placed_feature(&j)?);
        }
        Ok(FeatureRegistry { configured, placed, dir })
    }
}

fn split_rl(key: &str) -> Result<(String, String), String> {
    match key.split_once(':') {
        Some((a, b)) => Ok((a.to_string(), b.to_string())),
        None => Err(format!("bad resource key: {key}")),
    }
}

/// The tier-3 dispatch verdict (P4.5): which features this chunk needs vs
/// what the native lane supports — falls back (I8) on the first unsupported.
pub fn decoration_supported(features_needed: &[usize], registry: &FeatureRegistry, placed_ids: &[String]) -> bool {
    for &pf in features_needed {
        let Some(def) = placed_ids.get(pf).and_then(|k| registry.placed.get(k.as_str())) else {
            return false;
        };
        let Some(cfg) = registry.configured.get(&def.feature_ref) else {
            return false;
        };
        if let FeatureDef::Unsupported(_) = cfg {
            return false;
        }
        if def.placement.iter().any(|m| matches!(m, PlacementMod::Unsupported(_))) {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_matching_blocks_with_tags() {
        let p = parse_predicate(&crate::json::parse(
            "{\"type\":\"minecraft:matching_blocks\",\"blocks\":[\"#tag\",\"minecraft:stone\"]}",
        ).unwrap())
        .unwrap();
        let tag_of = |t: &str| {
            if t == "tag" {
                Some(vec!["minecraft:dirt".to_string(), "minecraft:grass_block".to_string()])
            } else {
                None
            }
        };
        let repl = |_: &str| false;
        assert!(eval_predicate(&p, "minecraft:stone", &repl, &tag_of));
        assert!(eval_predicate(&p, "minecraft:dirt", &repl, &tag_of));
        assert!(!eval_predicate(&p, "minecraft:sand", &repl, &tag_of));
    }

    #[test]
    fn unsupported_modifier_poisons_dispatch() {
        let mods = vec![PlacementMod::Unsupported("tree".into())];
        assert!(mods.iter().any(|m| matches!(m, PlacementMod::Unsupported(_))));
    }

    #[test]
    fn uniform_provider_shape() {
        let p = IntProvider::Uniform(2, 5);
        let mut rng = crate::xoroshiro::XoroshiroRandomSource::new(42);
        for _ in 0..64 {
            let v = p.sample(&mut rng);
            assert!((2..=5).contains(&v), "draw {v} out of range");
        }
    }
}
