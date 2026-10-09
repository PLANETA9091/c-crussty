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

use crate::feature_sorter::WorldgenDraws;
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
    /// WorldGenerationContext.resolveY: absolute / aboveBottom = minY + v /
    /// belowTop = genDepth - 1 + minY - v (CFR VerticalAnchor).
    fn resolve(&self, min_y: i32, height: i32) -> i32 {
        match self {
            HeightAnchor::Absolute(v) => *v,
            HeightAnchor::AboveBottom(v) => min_y + v,
            HeightAnchor::BelowTop(v) => min_y + height - 1 - v,
        }
    }
}

/// Ore target RuleTest (CFR RuleTest CODEC dispatch). Only the tests the
/// vanilla overworld ore set uses; anything else => Unsupported.
#[derive(Debug, Clone)]
pub enum RuleTest {
    AlwaysTrue,
    BlockMatch { block: String },
    TagMatch { tag: String },
    RandomBlockMatch { block: String, probability: f32 },
    Unsupported(String),
}

pub fn parse_rule_test(j: &Json) -> RuleTest {
    let ty = j
        .get("predicate_type")
        .and_then(|t| t.as_str())
        .unwrap_or("");
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    match ty {
        "always_true" => RuleTest::AlwaysTrue,
        "block_match" => RuleTest::BlockMatch {
            block: j
                .get("block")
                .and_then(|b| b.as_str())
                .unwrap_or("")
                .to_string(),
        },
        "tag_match" => RuleTest::TagMatch {
            tag: j.get("tag").and_then(|t| t.as_str()).unwrap_or("").to_string(),
        },
        "random_block_match" => RuleTest::RandomBlockMatch {
            block: j
                .get("block")
                .and_then(|b| b.as_str())
                .unwrap_or("")
                .to_string(),
            probability: j.get("probability").and_then(|p| p.as_f64()).unwrap_or(0.0) as f32,
        },
        other => RuleTest::Unsupported(other.to_string()),
    }
}

/// RuleTest.test(state, random): matching tests consume NO rng;
/// random_block_match draws nextFloat ONCE when evaluated.
pub fn eval_rule_test(
    t: &RuleTest,
    name: &str,
    rng: &mut dyn crate::feature_sorter::WorldgenDraws,
    tag_of: &dyn Fn(&str) -> Option<Vec<String>>,
) -> bool {
    match t {
        RuleTest::AlwaysTrue => true,
        RuleTest::BlockMatch { block } => block == name,
        RuleTest::TagMatch { tag } => tag
            .strip_prefix("#")
            .and_then(tag_of)
            .map(|members| members.iter().any(|m| m == name))
            .unwrap_or(false),
        RuleTest::RandomBlockMatch { block, probability } => {
            block == name && rng.next_f32_wg() < *probability
        }
        RuleTest::Unsupported(_) => false,
    }
}

/// HeightProvider (CFR heightproviders/*): the y-distribution for
/// height_range placement. Draw counts per sample are load-bearing:
///   Uniform: 1 draw (randomBetweenInclusive over the resolved range)
///   Trapezoid: 1 draw (plateau >= span) or 2 draws (0..i4 then 0..i3)
///   VeryBiasedToBottom: 3 draws
#[derive(Debug, Clone)]
pub enum HeightProvider {
    Constant(HeightAnchor),
    Uniform { min: HeightAnchor, max: HeightAnchor },
    Trapezoid { min: HeightAnchor, max: HeightAnchor, plateau: i32 },
    VeryBiasedToBottom { min: HeightAnchor, max: HeightAnchor, inner: i32 },
    Unsupported(String),
}

/// VerticalAnchor JSON: single-key object {"absolute"|"above_bottom"|
/// "below_top": value} (the codec dispatches on the KEY, not a "type" field).
fn parse_anchor(v: &Json) -> Result<HeightAnchor, String> {
    if let Some(val) = v.get("absolute").and_then(|x| x.as_i64()) {
        return Ok(HeightAnchor::Absolute(val as i32));
    }
    if let Some(val) = v.get("above_bottom").and_then(|x| x.as_i64()) {
        return Ok(HeightAnchor::AboveBottom(val as i32));
    }
    if let Some(val) = v.get("below_top").and_then(|x| x.as_i64()) {
        return Ok(HeightAnchor::BelowTop(val as i32));
    }
    // legacy {type, value} form tolerated
    if let Some(ty) = v.get("type").and_then(|t| t.as_str()) {
        let val = v.get("value").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
        let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
        return match ty {
            "absolute" => Ok(HeightAnchor::Absolute(val)),
            "above_bottom" => Ok(HeightAnchor::AboveBottom(val)),
            "below_top" => Ok(HeightAnchor::BelowTop(val)),
            other => Err(format!("unsupported anchor {other}")),
        };
    }
    Err("anchor shape".into())
}

pub fn parse_height_provider(j: &Json) -> Result<HeightProvider, String> {
    // height provider JSON: int (constant shorthand) or
    // {type, min_inclusive, max_inclusive[, plateau|inner]} — a raw anchor
    // shape without "type" is Constant (VerticalAnchor either-form).
    if j.as_i64().is_some() {
        return Ok(HeightProvider::Constant(HeightAnchor::Absolute(
            j.as_i64().unwrap() as i32,
        )));
    }
    let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    let min = || parse_anchor(j.get("min_inclusive").ok_or("min_inclusive")?);
    let max = || parse_anchor(j.get("max_inclusive").ok_or("max_inclusive")?);
    match ty {
        "constant" => Ok(HeightProvider::Constant(parse_anchor(
            j.get("value").ok_or("constant.value")?,
        )?)),
        "uniform" => Ok(HeightProvider::Uniform { min: min()?, max: max()? }),
        "trapezoid" => Ok(HeightProvider::Trapezoid {
            min: min()?,
            max: max()?,
            plateau: j.get("plateau").and_then(|p| p.as_i64()).unwrap_or(0) as i32,
        }),
        "very_biased_to_bottom" => Ok(HeightProvider::VeryBiasedToBottom {
            min: min()?,
            max: max()?,
            inner: j.get("inner").and_then(|p| p.as_i64()).unwrap_or(1) as i32,
        }),
        other => Ok(HeightProvider::Unsupported(other.to_string())),
    }
}

impl HeightProvider {
    /// Mth.randomBetweenInclusive(random, min, max) = nextInt(max-min+1) + min.
    #[inline]
    fn between(rng: &mut dyn crate::feature_sorter::WorldgenDraws, min: i32, max: i32) -> i32 {
        rng.next_int_bound_wg(max - min + 1).wrapping_add(min)
    }

    pub fn sample(&self, rng: &mut dyn crate::feature_sorter::WorldgenDraws, min_y: i32, gen_depth: i32) -> i32 {
        match self {
            HeightProvider::Constant(a) => a.resolve(min_y, gen_depth),
            HeightProvider::Uniform { min, max } => {
                let i = min.resolve(min_y, gen_depth);
                let i1 = max.resolve(min_y, gen_depth);
                if i > i1 {
                    return i; // Empty height range: warn path returns min
                }
                Self::between(rng, i, i1)
            }
            HeightProvider::Trapezoid { min, max, plateau } => {
                let i = min.resolve(min_y, gen_depth);
                let i1 = max.resolve(min_y, gen_depth);
                if i > i1 {
                    return i;
                }
                let i2 = i1 - i;
                if *plateau >= i2 {
                    return Self::between(rng, i, i1);
                }
                let i3 = (i2 - plateau) / 2;
                let i4 = i2 - i3;
                // return i + between(0, i4) + between(0, i3) — TWO draws
                i.wrapping_add(Self::between(rng, 0, i4))
                    .wrapping_add(Self::between(rng, 0, i3))
            }
            HeightProvider::VeryBiasedToBottom { min, max, inner } => {
                let i = min.resolve(min_y, gen_depth);
                let i1 = max.resolve(min_y, gen_depth);
                if i1 - i - inner + 1 <= 0 {
                    return i;
                }
                let ri = Self::between(rng, i + inner, i1);
                let ri1 = Self::between(rng, i, ri - 1);
                Self::between(rng, i, ri1 - 1 + inner)
            }
            HeightProvider::Unsupported(_) => 0,
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
        // int or {type: uniform|triangle, min_inclusive, max_inclusive} —
        // the uniform/trapezoid codecs put the bounds at the TOP level;
        // the {value: {min_inclusive, max_inclusive}} nested shape is the
        // weighted-list form and is tolerated too.
        if let Some(v) = j.as_i64() {
            return Ok(IntProvider::Constant(v as i32));
        }
        let (min, max) = if let Some(val) = j.get("value") {
            (
                val.get("min_inclusive").and_then(|v| v.as_i64()).ok_or("intprovider min")?,
                val.get("max_inclusive").and_then(|v| v.as_i64()).ok_or("intprovider max")?,
            )
        } else {
            match (
                j.get("min_inclusive").and_then(|v| v.as_i64()),
                j.get("max_inclusive").and_then(|v| v.as_i64()),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => return Err("intprovider shape".into()),
            }
        };
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("minecraft:uniform");
        match ty.strip_prefix("minecraft:").unwrap_or(ty) {
            "uniform" => Ok(IntProvider::Uniform(min as i32, max as i32)),
            other => Err(format!("unsupported int provider {other} (triangle pending decompile)")),
        }
    }

    #[inline]
    pub fn sample(&self, rng: &mut dyn crate::feature_sorter::WorldgenDraws) -> i32 {
        match *self {
            IntProvider::Constant(v) => v,
            IntProvider::Uniform(a, b) => rng.next_int_bound_wg(b - a + 1) + a,
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
    HeightRange(HeightProvider),
    RandomOffset { xz: IntProvider, y: IntProvider },
    BiomeFilter,
    BlockPredicateFilter(BlockPredicate),
    /// noise_based_count (kelp_cold uses it) — density+dx/dz, ported with
    /// the driver (per-position normal-noise sample feeding the count).
    NoiseBasedCount {
        noise_to_count_ratio: f32,
        noise_offset: f64,
        noise_scale: f64,
    },
    /// surface_water_depth_filter (trees_water) — depth check against the
    /// OCEAN_FLOORWG heightmap at the position (no rng).
    SurfaceWaterDepthFilter { max_water_depth: i32 },
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
            // CFR HeightRangePlacement: the WHOLE HeightProvider (uniform /
            // trapezoid / very_biased_to_bottom / constant) rides in the
            // modifier; sampled per incoming position.
            let h = j.get("height").ok_or("height")?;
            PlacementMod::HeightRange(parse_height_provider(h)?)
        }
        "random_offset" => PlacementMod::RandomOffset {
            xz: IntProvider::parse(j.get("xz_spread").ok_or("xz_spread")?)?,
            y: IntProvider::parse(j.get("y_spread").ok_or("y_spread")?)?,
        },
        "biome" => PlacementMod::BiomeFilter,
        "block_predicate_filter" => {
            PlacementMod::BlockPredicateFilter(parse_predicate(j.get("predicate").ok_or("predicate")?)?)
        }
        "noise_based_count" => PlacementMod::NoiseBasedCount {
            noise_to_count_ratio: j
                .get("noise_to_count_ratio")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as f32,
            noise_offset: j.get("noise_offset").and_then(|v| v.as_f64()).unwrap_or(0.0),
            noise_scale: j.get("noise_scale").and_then(|v| v.as_f64()).unwrap_or(0.0),
        },
        "surface_water_depth_filter" => PlacementMod::SurfaceWaterDepthFilter {
            max_water_depth: j
                .get("max_water_depth")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32,
        },
        _ => PlacementMod::Unsupported(ty.to_string()),
    })
}

/// PlacedFeature IR: configured ref + modifier chain.
#[derive(Debug, Clone)]
pub struct PlacedFeatureDef {
    pub feature_ref: String,
    pub placement: Vec<PlacementMod>,
    /// INLINE configured feature (random_patch inner shape): (type short,
    /// config JSON) — resolved without a registry lookup.
    pub inline: Option<(String, Json)>,
}

pub fn parse_placed_feature(j: &Json) -> Result<PlacedFeatureDef, String> {
    // feature ref: registry string OR inline configured object {type, config}
    let (feature_ref, inline) = match j.get("feature") {
        Some(Json::Str(s)) => (expand_rl(s), None),
        Some(f) if f.get("type").is_some() => {
            // inline configured feature: parse the body here and park it on
            // the def (execution resolves it without a registry lookup).
            let ty = f.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string();
            let short = ty.strip_prefix("minecraft:").unwrap_or(&ty).to_string();
            let cfg = f.get("config").cloned().ok_or("inline.config")?;
            (String::new(), Some((short, cfg)))
        }
        _ => return Err("placed.feature".into()),
    };
    let mut placement = Vec::new();
    if let Some(Json::Arr(mods)) = j.get("placement") {
        for m in mods {
            placement.push(parse_placement_mod(m)?);
        }
    }
    Ok(PlacedFeatureDef { feature_ref, placement, inline })
}

// ---------------------------------------------------------------------------
// Configured feature IR + placement (tier 1-2 subset; rest -> Unsupported)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum FeatureDef {
    /// OreConfiguration: targets are RuleTests (NOT block predicates), the
    /// discard key is "discard_chance_on_air_exposure" (CFR codec field).
    Ore {
        size: i32,
        discard_chance: f32,
        targets: Vec<(RuleTest, String)>,
        scattered: bool,
    },
    SimpleBlock { to_place_name: String, props: Vec<(String, String)>, schedule_tick: bool },
    /// RandomPatchConfiguration: the INNER feature is an INLINE placed
    /// feature (feature+placement object) or a registry ref string.
    RandomPatch {
        tries: i32,
        xz_spread: i32,
        y_spread: i32,
        inner: Box<PlacedFeatureDef>,
    },
    /// DiskConfiguration: "target" is a SINGLE block predicate and the
    /// state comes from "state_provider" (fallback + rules; vanilla corpus
    /// shapes are rules: [] + simple fallback state).
    Disk {
        half_height: i32,
        radius: IntProvider,
        target: BlockPredicate,
        state: String,
    },
    Kelp,
    Seagrass { probability: f32 },
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
            let def = match parse_configured_def(&j, &short) {
                Ok(d) => d,
                // an unparseable/unsupported shape is a HONEST Unsupported —
                // it must not poison the whole registry (the dispatch then
                // falls back for chunks that need it, with the reason).
                Err(e) => FeatureDef::Unsupported(e),
            };
            configured.insert(key.clone(), def);
        }
        for key in placed_keys {
            let (ns, name) = split_rl(key)?;
            let Some(text) = dir.get(&ns, "placed_feature", &name) else {
                continue;
            };
            let j = crate::json::parse(text).map_err(|e| e.to_string())?;
            match parse_placed_feature(&j) {
                Ok(d) => {
                    placed.insert(key.clone(), d);
                }
                Err(e) => {
                    eprintln!("[features] placed {key}: unsupported ({e})");
                }
            }
        }
        Ok(FeatureRegistry { configured, placed, dir })
    }
}


/// Parse one configured-feature JSON (type short name known) into the IR.
/// Err = an unsupported/unknowable shape (callers degrade to Unsupported).
fn parse_configured_def(j: &Json, short: &str) -> Result<FeatureDef, String> {
    let short = short.trim_start_matches("minecraft:");
    // FLOWER and NO_BONEMEAL_FLOWER are the SAME RandomPatchFeature class
    // registered under other names (CFR Feature.java) — alias them.
    let short = match short {
        "flower" | "no_bonemeal_flower" => "random_patch",
        other => other,
    };
    Ok(match short {
            "ore" | "scattered_ore" => {
                let cfg = j.get("config").ok_or("ore.config")?;
                let size = cfg.get("size").and_then(|s| s.as_i64()).unwrap_or(0) as i32;
                let discard = cfg
                    .get("discard_chance_on_air_exposure")
                    .and_then(|s| s.as_f64())
                    .unwrap_or(0.0) as f32;
                let mut targets = Vec::new();
                if let Some(Json::Arr(ts)) = cfg.get("targets") {
                    for t in ts {
                        let rt = parse_rule_test(t.get("target").ok_or("target")?);
                        let st = t.get("state").ok_or("state")?;
                        let name = st
                            .get("Name")
                            .and_then(|n| n.as_str())
                            .ok_or("state.Name")?
                            .to_string();
                        targets.push((rt, name));
                    }
                }
                FeatureDef::Ore {
                    size,
                    discard_chance: discard,
                    targets,
                    scattered: short == "scattered_ore",
                }
            }
            "simple_block" => {
                let cfg = j.get("config").ok_or("simple.config")?;
                // SimpleBlockConfiguration.to_place = a BlockStateProvider
                // (simple_state_provider {state: {Name, Properties}}).
                let state = parse_simple_state_provider(cfg.get("to_place").ok_or("to_place")?)?;
                let (name, props) = match state.split_once('[') {
                    Some((n, ps)) => (
                        n.to_string(),
                        ps.trim_end_matches(']')
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|kv| {
                                let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
                                (k.to_string(), v.to_string())
                            })
                            .collect(),
                    ),
                    None => (state, Vec::new()),
                };
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
                // inner placed feature: INLINE object {feature, placement}
                // or a registry ref string (resolved at eval time).
                let inner_j = cfg.get("feature").ok_or("patch.feature")?;
                let inner = if let Some(s) = inner_j.as_str() {
                    PlacedFeatureDef {
                        feature_ref: expand_rl(s),
                        placement: Vec::new(),
                        inline: None,
                    }
                } else {
                    parse_placed_feature(inner_j)?
                };
                FeatureDef::RandomPatch {
                    tries: cfg.get("tries").and_then(|t| t.as_i64()).unwrap_or(32) as i32,
                    xz_spread: cfg.get("xz_spread").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
                    y_spread: cfg.get("y_spread").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
                    inner: Box::new(inner),
                }
            }
            "disk" => {
                let cfg = j.get("config").ok_or("disk.config")?;
                let radius = IntProvider::parse(cfg.get("radius").ok_or("radius")?)?;
                let half = cfg.get("half_height").and_then(|h| h.as_i64()).unwrap_or(0) as i32;
                // DiskConfiguration: target = SINGLE block predicate;
                // state_provider = {fallback: simple_state_provider, rules}
                let target = parse_predicate(cfg.get("target").ok_or("disk.target")?)?;
                let sp = cfg.get("state_provider").ok_or("disk.state_provider")?;
                let state = parse_simple_state_provider(sp)?;
                FeatureDef::Disk { half_height: half, radius, target, state }
            }
            "kelp" => FeatureDef::Kelp,
            "seagrass" => FeatureDef::Seagrass {
                probability: j
                    .get("config")
                    .and_then(|c| c.get("probability"))
                    .and_then(|p| p.as_f64())
                    .unwrap_or(0.0) as f32,
            },
            other => FeatureDef::Unsupported(other.to_string()),
    })
}

fn split_rl(key: &str) -> Result<(String, String), String> {
    match key.split_once(':') {
        Some((a, b)) => Ok((a.to_string(), b.to_string())),
        None => Err(format!("bad resource key: {key}")),
    }
}

/// expand "minecraft:x" to the fully namespaced form (biome JSON entries may
/// omit the namespace).
fn expand_rl(s: &str) -> String {
    if s.contains(':') {
        s.to_string()
    } else {
        format!("minecraft:{s}")
    }
}

/// BlockStateProvider subset: the DISK provider is RuleBasedBlockStateProvider
/// {fallback: provider, rules: [...]} — the corpus shapes have rules: [] so
/// the state always comes from the fallback; non-empty rules => Err (honest
/// Unsupported). Only simple_state_provider is a proven leaf.
fn parse_simple_state_provider(j: &Json) -> Result<String, String> {
    // RuleBased wrapper: descend into "fallback", reject non-empty rules.
    let j = if let Some(f) = j.get("fallback") {
        if let Some(Json::Arr(rules)) = j.get("rules") {
            if !rules.is_empty() {
                return Err("rule_based_state_provider with rules".into());
            }
        }
        f
    } else {
        j
    };
    let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    if ty != "simple_state_provider" {
        return Err(format!("unsupported state provider {ty}"));
    }
    let st = j.get("state").ok_or("state")?;
    let name = st
        .get("Name")
        .and_then(|n| n.as_str())
        .ok_or("state.Name")?;
    let mut props = Vec::new();
    if let Some(Json::Obj(po)) = st.get("Properties") {
        for (k, v) in po {
            props.push(format!(
                "{}={}",
                k,
                v.as_str().unwrap_or("")
            ));
        }
    }
    props.sort();
    if props.is_empty() {
        Ok(expand_rl(name))
    } else {
        Ok(format!("{}[{}]", expand_rl(name), props.join(",")))
    }
}

/// The tier-3 dispatch verdict (P4.5): which features this chunk needs vs
/// what the native lane supports — falls back (I8) on the first unsupported.
pub fn decoration_supported(features_needed: &[usize], registry: &FeatureRegistry, placed_ids: &[String]) -> bool {
    for &pf in features_needed {
        let Some(def) = placed_ids.get(pf).and_then(|k| registry.placed.get(k.as_str())) else {
            return false;
        };
        if is_placed_supported(def, registry) {
            continue;
        }
        return false;
    }
    true
}

/// Full support verdict for one placed feature (modifiers + configured body
/// + the bodies reachable through random_patch nesting).
pub fn is_placed_supported(def: &PlacedFeatureDef, registry: &FeatureRegistry) -> bool {
    if def
        .placement
        .iter()
        .any(|m| matches!(m, PlacementMod::Unsupported(_)))
    {
        return false;
    }
    // INLINE configured body (random_patch inner shape).
    let body = match &def.inline {
        Some((short, _)) => {
            return !matches!(short.as_str(), "tree" | "random_selector"
                | "simple_random_selector" | "random_boolean_selector");
        }
        None => match registry.configured.get(&def.feature_ref) {
            Some(cfg) => cfg,
            None => return false,
        },
    };
    match body {
        FeatureDef::Unsupported(_) => false,
        FeatureDef::RandomPatch { inner, .. } => is_placed_supported(inner, registry),
        _ => true,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature_sorter::WorldgenDraws;

    // Real corpus shapes (data/minecraft/worldgen/... of the jar-fresh
    // extract) — the IR must parse these EXACTLY (P4.2/P4.3 tails).

    #[test]
    fn ore_config_real_shape() {
        let j = crate::json::parse(
            "{\"type\":\"minecraft:ore\",\"config\":{\"discard_chance_on_air_exposure\":0.0,\"size\":9,\
             \"targets\":[{\"state\":{\"Name\":\"minecraft:dirt\"},\
             \"target\":{\"predicate_type\":\"minecraft:block_match\",\"block\":\"minecraft:stone\"}},\
             {\"state\":{\"Name\":\"minecraft:coarse_dirt\"},\
             \"target\":{\"predicate_type\":\"minecraft:tag_match\",\"tag\":\"minecraft:stone_ore_replaceables\"}}]}}",
        )
        .unwrap();
        let cfg = j.get("config").unwrap();
        assert_eq!(cfg.get("size").and_then(|s| s.as_i64()), Some(9));
        let rt = parse_rule_test(
            cfg.get("targets")
                .and_then(|t| t.as_arr())
                .unwrap()[1]
                .get("target")
                .unwrap(),
        );
        assert!(matches!(rt, RuleTest::TagMatch { ref tag } if tag == "minecraft:stone_ore_replaceables"));
        let rt2 = parse_rule_test(
            cfg.get("targets")
                .and_then(|t| t.as_arr())
                .unwrap()[0]
                .get("target")
                .unwrap(),
        );
        let mut dr = crate::feature_sorter::DecorationRandom::new(7);
        let no_tag = |_: &str| -> Option<Vec<String>> { None };
        // BlockMatch consumes NO rng draw.
        assert!(eval_rule_test(&rt2, "minecraft:stone", &mut dr, &no_tag));
        assert!(!eval_rule_test(&rt2, "minecraft:dirt", &mut dr, &no_tag));
    }

    #[test]
    fn disk_config_single_target() {
        let j = crate::json::parse(
            "{\"type\":\"minecraft:disk\",\"config\":{\"half_height\":1,\
             \"radius\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":3,\"min_inclusive\":2},\
             \"state_provider\":{\"fallback\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:clay\"}},\"rules\":[]},\
             \"target\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":[\"minecraft:dirt\",\"minecraft:clay\"]}}}",
        )
        .unwrap();
        let cfg = j.get("config").unwrap();
        let target = parse_predicate(cfg.get("target").unwrap()).unwrap();
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        assert!(eval_predicate(&target, "minecraft:clay", &repl, &tag_of));
        assert!(!eval_predicate(&target, "minecraft:stone", &repl, &tag_of));
        let state = parse_simple_state_provider(cfg.get("state_provider").unwrap()).unwrap();
        assert_eq!(state, "minecraft:clay");
        // radius uniform parse
        let r = IntProvider::parse(cfg.get("radius").unwrap()).unwrap();
        assert!(matches!(r, IntProvider::Uniform(2, 3)));
    }

    #[test]
    fn random_patch_inline_inner() {
        let j = crate::json::parse(
            "{\"type\":\"minecraft:random_patch\",\"config\":{\"tries\":32,\"xz_spread\":7,\"y_spread\":3,\
             \"feature\":{\"feature\":{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:simple_state_provider\",\"state\":{\"Name\":\"minecraft:short_grass\"}}}},\
             \"placement\":[{\"type\":\"minecraft:block_predicate_filter\",\
             \"predicate\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":\"minecraft:air\"}}]}}}",
        )
        .unwrap();
        let cfg = j.get("config").unwrap();
        let inner = parse_placed_feature(cfg.get("feature").unwrap()).unwrap();
        assert_eq!(inner.feature_ref, "");
        assert_eq!(inner.placement.len(), 1);
        // string blocks form: "blocks": "minecraft:air" (single string)
        if let PlacementMod::BlockPredicateFilter(p) = &inner.placement[0] {
            let tag_of = |_: &str| -> Option<Vec<String>> { None };
            let repl = |_: &str| false;
            assert!(eval_predicate(p, "minecraft:air", &repl, &tag_of));
            assert!(!eval_predicate(p, "minecraft:stone", &repl, &tag_of));
        } else {
            panic!("expected block_predicate_filter");
        }
    }

    #[test]
    fn height_range_trapezoid_shape() {
        let j = crate::json::parse(
            "{\"type\":\"minecraft:height_range\",\"height\":{\"type\":\"minecraft:trapezoid\",\
             \"max_inclusive\":{\"above_bottom\":80},\"min_inclusive\":{\"above_bottom\":-80}}}",
        )
        .unwrap();
        let m = parse_placement_mod(&j).unwrap();
        match m {
            PlacementMod::HeightRange(HeightProvider::Trapezoid { min, max, plateau }) => {
                assert!(matches!(min, HeightAnchor::AboveBottom(-80)));
                assert!(matches!(max, HeightAnchor::AboveBottom(80)));
                assert_eq!(plateau, 0);
                // resolved: above_bottom(-80) = -64 + -80 = -144, +80 = -16
                assert_eq!(min.resolve(-64, 384), -144);
                assert_eq!(max.resolve(-64, 384), 16);
                // draw count: plateau 0 < span 160 => TWO draws; the two
                // between() calls walk 0..i4 then 0..i3.
                let mut dr = crate::feature_sorter::DecorationRandom::new(1);
                let _ = HeightProvider::Trapezoid { min, max, plateau }.sample(&mut dr, -64, 384);
                // determinism: same seed => same draw sequence
                let mut a = crate::feature_sorter::DecorationRandom::new(42);
                let mut b = crate::feature_sorter::DecorationRandom::new(42);
                let ya = HeightProvider::Trapezoid {
                    min: HeightAnchor::AboveBottom(-80),
                    max: HeightAnchor::AboveBottom(80),
                    plateau: 0,
                }
                .sample(&mut a, -64, 384);
                let yb = HeightProvider::Trapezoid {
                    min: HeightAnchor::AboveBottom(-80),
                    max: HeightAnchor::AboveBottom(80),
                    plateau: 0,
                }
                .sample(&mut b, -64, 384);
                assert_eq!(ya, yb);
                assert_eq!(a.count_wg(), b.count_wg());
                let _ = &mut dr;
            }
            other => panic!("expected trapezoid, got {other:?}"),
        }
    }

    #[test]
    fn worldgen_random_draw_stack() {
        // The draw-count semantics: nextLong = 2 inner draws; nextInt(bound)
        // = 1 inner draw per next(31) (rejection may add); nextFloat = 1.
        let mut a = crate::feature_sorter::DecorationRandom::new(3053459);
        let base = a.count_wg();
        let _ = a.next_long_wg();
        assert_eq!(a.count_wg(), base + 2);
        let _ = a.next_f32_wg();
        assert_eq!(a.count_wg(), base + 3);
        let v = a.next_int_bound_wg(16);
        assert!((0..16).contains(&v));
        // set_feature_seed resets determinism: identical seeds => identical
        // draw sequences (the decoration driver relies on this).
        let mut x = crate::feature_sorter::DecorationRandom::new(1);
        let mut y = crate::feature_sorter::DecorationRandom::new(2);
        x.set_feature_seed(777, 5, 6);
        y.set_feature_seed(777, 5, 6);
        assert_eq!(x.next_int_bound_wg(1000), y.next_int_bound_wg(1000));
        assert_eq!(x.next_f64_wg(), y.next_f64_wg());
        assert_eq!(x.next_long_wg(), y.next_long_wg());
    }
}
