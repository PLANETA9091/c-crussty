//! NCF P4.2/P4.3/P4.5 — placement modifiers, block predicates, tier-1
//! features (ore/scattered_ore, simple_block, random_patch, disk, kelp,
//! seagrass, freeze_top_layer, spring_feature) + the tier-3 dispatch with
//! honest fallback accounting.
//!
//! Ports of the mapped Purpur 2535 sources (CFR 0.152, session 7 + inc. 2/3):
//!   - PlacedFeature.placeWithContext: positions stream folds through the
//!     modifier chain IN ORDER (flatMap per modifier, per-position draws in
//!     stream order; empty stream short-circuits the chain).
//!   - InSquarePlacement/CountPlacement/RarityFilter/HeightmapPlacement/
//!     HeightRangePlacement/RandomOffsetPlacement/BiomeFilter bodies.
//!   - StateTestingPredicate offset family (inc. 2): matching_blocks /
//!     matching_fluids / replaceable / solid at pos.offset(offset)
//!     (Vec3i.offsetCodec(16)).
//!   - SurfaceRelativeThresholdFilter + EnvironmentScanPlacement (inc. 3):
//!     the two widest placement-modifier blockers (54/53 biomes); scan is a
//!     vertical position TRANSFORMER with the CFR break-then-final-test
//!     semantics (the first NOT-allowed position is still tested).
//!   - HasSturdyFacePredicate + InsideWorldBoundsPredicate (inc. 3): the
//!     two remaining nested targets of the corpus environment_scan shapes.
//!   - MonsterRoomFeature + UnderwaterMagmaFeature + MultifaceGrowthFeature
//!     (inc. 4): the body-type histogram top (54/54/54 biomes). PARSE-true;
//!     execution tails documented on the variants (spawner/chest block
//!     entities; Column.scan + per-position rng; recursive multiface
//!     spreading).
//!   - GeodeFeature (inc. 5): the widest REMAINING histogram blocker (54
//!     biomes first-blocked). Full GeodeConfiguration codec (13 fields) over
//!     the three nested settings records (GeodeBlockSettings 8 REQUIRED
//!     fields, GeodeLayerSettings doubleRange(0.01,50) orElse 1.7/2.2/3.2/
//!     4.2, GeodeCrackSettings orElse 1.0/2.0/2 — decomp441b) with honest
//!     range-checks (IntProvider.codec(1,20)/(0,10) bounds, CHANCE_RANGE
//!     0..1, nonEmptyList inner_placements). Execution tail documented:
//!     geode-private NormalNoise.create(new LegacyRandomSource(seed), -4,
//!     [1.0]) keyed by the WORLD seed, invSqrt layer radii, crack offsets,
//!     budding-amethyst FACING/WATERLOGGED writes.
//!   - RuleBasedBlockStateProvider (inc. 6): the new histogram CO-top (53
//!     biomes) — the disk family's state_provider {fallback, rules}.
//!     getState: FIRST rule whose if_true passes wins, else the fallback;
//!     "rules" is a REQUIRED field but MAY be empty (disk_clay/disk_gravel).
//!     Opens disk_sand/disk_grass (their rule predicates — matching_blocks
//!     with offset, not(any_of(solid, matching_fluids)) with offsets — are
//!     all session-7 predicates).
//!   - LakeFeature (inc. 7): the ONLY remaining histogram top (53 biomes,
//!     lake_lava_surface + lake_lava_underground placed chains). Codec:
//!     LakeFeature.Configuration — fluid + barrier, BOTH fieldOf
//!     (REQUIRED) over BlockStateProvider.CODEC (a rule_based shape
//!     parses via the full inc.-6 provider path); no other fields, no
//!     defaults. The class is @Deprecated but still registered
//!     ("minecraft:lake"). Execution tail documented on the variant
//!     (16x16x8 ellipsoid flags, CAVE_AIR upper half, barrier layer,
//!     freeze pass).
//!   - WouldSurvivePredicate (inc. 8): the last universal BLOCK-PREDICATE
//!     blocker (43/65 biomes via block_predicate_filter — the trees/patches
//!     family; also inside random_patch inline placements). Shape: optional
//!     offset (Vec3i.offsetCodec(16), default ZERO) + REQUIRED state
//!     (BlockState.CODEC); test = state.canSurvive(level, pos+offset).
//!     Per-block survival rules (sapling light, cactus base, sugar-cane
//!     ground...) are a P4 tail — eval stays honest-false, the SHAPE is
//!     parsed verbatim. RandomPatchConfiguration orElse defaults fixed to
//!     the codec values (tries 128 / xz_spread 7 / y_spread 3).
//!   - RandomSelectorFeature ("random_selector", inc. 9) — the NEW top
//!     after inc. 8 (36/65 biomes): RandomFeatureConfiguration = weighted
//!     features tried IN ORDER (first nextFloat() < chance places and
//!     RETURNS) else the default; WeightedPlacedFeature chance =
//!     floatRange(0..1); entries and default are PlacedFeature.CODEC
//!     (string ref OR inline {feature, placement}).
//!   - RandomBooleanSelectorFeature ("random_boolean_selector", inc. 9):
//!     ONE nextBoolean() — feature_true/feature_false, both fieldOf
//!     REQUIRED, no default.
//!   - SnowAndFreezeFeature ("freeze_top_layer", inc. 2) — see the
//!     FeatureDef::FreezeTopLayer doc.
//!   - SpringFeature ("spring_feature", inc. 2) — DETERMINISTIC, no draws.
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
use crate::jrandom::RandomSource;
use crate::json::Json;
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
    /// StateTestingPredicate family: the test runs at pos.offset(offset),
    /// NOT at pos itself (Vec3i.offsetCodec(16), optional, default ZERO).
    MatchingBlocks {
        offset: [i32; 3],
        blocks: Vec<String>,
    }, // full names or "#tag"
    Replaceable {
        offset: [i32; 3],
    },
    /// SolidPredicate (deprecated but present in 1.21.10) — state.isSolid().
    Solid {
        offset: [i32; 3],
    },
    /// MatchingFluidsPredicate — the fluid of the state at pos+offset
    /// (worldgen fluids are water/lava; flowing_* are levels of the same
    /// block, so the block base name identifies the fluid).
    MatchingFluids {
        offset: [i32; 3],
        fluids: Vec<String>,
    },
    Not(Box<BlockPredicate>),
    AllOf(Vec<BlockPredicate>),
    AnyOf(Vec<BlockPredicate>),
    /// WouldSurvivePredicate (inc. 8) — state.canSurvive(level, pos+offset)
    /// at pos+offset. The STATE is parsed verbatim (canonical
    /// "ns:block[k=v,...]" string); per-block survival rules (sapling light,
    /// cactus base, sugar-cane ground, ...) are a P4 tail — eval stays
    /// honest-false until they land.
    WouldSurvive {
        offset: [i32; 3],
        state: String,
    },
    /// HasSturdyFacePredicate (inc. 3) — state.isFaceSturdy(level, pos,
    /// direction) at pos+offset. Per-block sturdy-face tables are a P4
    /// tail; eval uses the session-7 solid convention (terrain full-cube
    /// blocks are sturdy on every face, air/water/lava are not) — the
    /// direction is parsed verbatim and kept for the honest eval when the
    /// property tables land.
    HasSturdyFace {
        offset: [i32; 3],
        direction: String,
    },
    /// InsideWorldBoundsPredicate (inc. 3) — !level.isOutsideBuildHeight(
    /// pos.offset(offset)). Eval: block_at answers None exactly outside
    /// the readable region (= build height), so Some(_) == inside bounds.
    InsideWorldBounds {
        offset: [i32; 3],
    },
}

/// Vec3i.offsetCodec(16): optional "offset": [x,y,z], each component
/// clamped to ±16 by the codec (values outside are codec errors in Java —
/// the tolerant clamp keeps the same effective bound for honest shapes).
fn parse_offset(j: &Json) -> [i32; 3] {
    let Some(v) = j.get("offset") else {
        return [0, 0, 0];
    };
    let Some(arr) = v.as_arr() else {
        return [0, 0, 0];
    };
    if arr.len() != 3 {
        return [0, 0, 0];
    }
    let cl = |x: i64| x.clamp(-16, 16) as i32;
    [
        cl(arr[0].as_i64().unwrap_or(0)),
        cl(arr[1].as_i64().unwrap_or(0)),
        cl(arr[2].as_i64().unwrap_or(0)),
    ]
}

/// BlockState JSON ({Name, Properties}) -> the canonical worldgen state
/// string "namespace:block[k=v,...]" (properties sorted; the same
/// convention as the state providers). Shared by predicates (would_survive,
/// inc. 8) and the state-provider leaves.
fn parse_state_name(st: &Json) -> Result<String, String> {
    let name = st
        .get("Name")
        .and_then(|n| n.as_str())
        .ok_or("state.Name")?;
    let mut props = Vec::new();
    if let Some(Json::Obj(po)) = st.get("Properties") {
        for (k, v) in po {
            props.push(format!("{}={}", k, v.as_str().unwrap_or("")));
        }
    }
    props.sort();
    Ok(if props.is_empty() {
        expand_rl(name)
    } else {
        format!("{}[{}]", expand_rl(name), props.join(","))
    })
}

pub fn parse_predicate(j: &Json) -> Result<BlockPredicate, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("predicate type")?;
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
            BlockPredicate::MatchingBlocks {
                offset: parse_offset(j),
                blocks,
            }
        }
        "replaceable" => BlockPredicate::Replaceable {
            offset: parse_offset(j),
        },
        "solid" => BlockPredicate::Solid {
            offset: parse_offset(j),
        },
        "matching_fluids" => {
            let mut fluids = Vec::new();
            if let Some(Json::Arr(a)) = j.get("fluids") {
                for f in a {
                    fluids.push(f.as_str().ok_or("fluid name")?.to_string());
                }
            } else if let Some(f) = j.get("fluids").and_then(|f| f.as_str()) {
                fluids.push(f.to_string());
            }
            BlockPredicate::MatchingFluids {
                offset: parse_offset(j),
                fluids,
            }
        }
        "has_sturdy_face" => BlockPredicate::HasSturdyFace {
            offset: parse_offset(j),
            direction: j
                .get("direction")
                .and_then(|d| d.as_str())
                .ok_or("has_sturdy_face.direction")?
                .to_string(),
        },
        "inside_world_bounds" => BlockPredicate::InsideWorldBounds {
            offset: parse_offset(j),
        },
        "would_survive" => BlockPredicate::WouldSurvive {
            offset: parse_offset(j),
            state: parse_state_name(j.get("state").ok_or("would_survive.state")?)?,
        },
        "not" => {
            let inner = j.get("predicate").ok_or("not.predicate")?;
            BlockPredicate::Not(Box::new(parse_predicate(inner)?))
        }
        "all_of" => {
            let mut v = Vec::new();
            for p in j
                .get("predicates")
                .and_then(|p| p.as_arr())
                .ok_or("all_of.predicates")?
            {
                v.push(parse_predicate(p)?);
            }
            BlockPredicate::AllOf(v)
        }
        "any_of" => {
            let mut v = Vec::new();
            for p in j
                .get("predicates")
                .and_then(|p| p.as_arr())
                .ok_or("any_of.predicates")?
            {
                v.push(parse_predicate(p)?);
            }
            BlockPredicate::AnyOf(v)
        }
        _ => return Err(format!("unsupported block predicate: {ty}")),
    })
}

/// Fluid registry id -> the block base name that fluid appears as at
/// worldgen. flowing_water/flowing_lava are LEVELS of the same block, not
/// separate blocks; worldgen fluids are water and lava only.
fn fluid_block_of(id: &str) -> Option<String> {
    match id {
        "minecraft:water" | "minecraft:flowing_water" => Some("minecraft:water".into()),
        "minecraft:lava" | "minecraft:flowing_lava" => Some("minecraft:lava".into()),
        _ => None,
    }
}

/// Fluid of a worldgen block state: the base name (before properties)
/// identifies it; only water/lava carry fluids at generation time.
fn fluid_of_block(name: &str) -> Option<String> {
    let base = name.split('[').next().unwrap_or(name);
    match base {
        "minecraft:water" => Some("minecraft:water".into()),
        "minecraft:lava" => Some("minecraft:lava".into()),
        _ => None,
    }
}

fn name_matches(entry: &str, name: &str, tag_of: &dyn Fn(&str) -> Option<Vec<String>>) -> bool {
    if let Some(rest) = entry.strip_prefix("#") {
        tag_of(rest)
            .map(|members| members.iter().any(|m| m == name))
            .unwrap_or(false)
    } else {
        entry == name
    }
}

/// Predicate evaluation at world position (x,y,z). `block_at` resolves the
/// block state NAME at any in-region position; None = outside the readable
/// region (vanilla would throw — the native lane cannot verify and answers
/// false; the chunk verdict keeps unsupported shapes off the native lane).
/// Tag sets resolve through the caller-provided tag lookup (same machinery
/// as carvers replaceable). NO rng draws: predicates are pure (the
/// would_survive evaluation needs block survival rules — P4 tail, still
/// honest-false).
pub fn eval_predicate(
    p: &BlockPredicate,
    x: i32,
    y: i32,
    z: i32,
    block_at: &dyn Fn(i32, i32, i32) -> Option<String>,
    is_replaceable: &dyn Fn(&str) -> bool,
    tag_of: &dyn Fn(&str) -> Option<Vec<String>>,
) -> bool {
    match p {
        BlockPredicate::True => true,
        BlockPredicate::False => false,
        BlockPredicate::MatchingBlocks { offset, blocks } => {
            match block_at(x + offset[0], y + offset[1], z + offset[2]) {
                Some(name) => blocks.iter().any(|b| name_matches(b, &name, tag_of)),
                None => false,
            }
        }
        BlockPredicate::Replaceable { offset } => {
            match block_at(x + offset[0], y + offset[1], z + offset[2]) {
                Some(name) => is_replaceable(&name),
                None => false,
            }
        }
        BlockPredicate::Solid { offset } => {
            // state.isSolid() on worldgen content: a real block that is not
            // air/water/lava (the "solid, not fluid/air" convention of the
            // session-7 canSurvive ports).
            match block_at(x + offset[0], y + offset[1], z + offset[2]) {
                Some(name) => {
                    let base = name.split('[').next().unwrap_or(name.as_str());
                    !matches!(base, "minecraft:air" | "minecraft:water" | "minecraft:lava")
                }
                None => false,
            }
        }
        BlockPredicate::MatchingFluids { offset, fluids } => {
            let fluid = block_at(x + offset[0], y + offset[1], z + offset[2])
                .as_deref()
                .and_then(fluid_of_block);
            match fluid {
                Some(f) => fluids
                    .iter()
                    .filter_map(|id| fluid_block_of(id))
                    .any(|b| b == f),
                None => false,
            }
        }
        BlockPredicate::Not(inner) => {
            !eval_predicate(inner, x, y, z, block_at, is_replaceable, tag_of)
        }
        BlockPredicate::AllOf(v) => v
            .iter()
            .all(|p| eval_predicate(p, x, y, z, block_at, is_replaceable, tag_of)),
        BlockPredicate::AnyOf(v) => v
            .iter()
            .any(|p| eval_predicate(p, x, y, z, block_at, is_replaceable, tag_of)),
        BlockPredicate::HasSturdyFace { offset, .. } => {
            // isFaceSturdy on worldgen content: the session-7 solid
            // convention (terrain full-cube blocks sturdy on every face;
            // air/water/lava not). Per-block tables = P4 tail.
            match block_at(x + offset[0], y + offset[1], z + offset[2]) {
                Some(name) => {
                    let base = name.split('[').next().unwrap_or(name.as_str());
                    !matches!(base, "minecraft:air" | "minecraft:water" | "minecraft:lava")
                }
                None => false,
            }
        }
        BlockPredicate::InsideWorldBounds { offset } => {
            // isOutsideBuildHeight(y) == y outside [min_y, min_y+height).
            // block_at returns None exactly for the unreadable region, so
            // a readable position IS inside the build height (honest).
            block_at(x + offset[0], y + offset[1], z + offset[2]).is_some()
        }
        BlockPredicate::WouldSurvive { .. } => {
            // canSurvive per block (sapling light, cactus base, sugar-cane
            // ground, ...) — P4 tail; the SHAPE is parsed verbatim since
            // inc. 8, the verdict stays honest-false.
            false
        }
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
        FeatureWorld {
            chunk,
            height_kinds: HashMap::new(),
        }
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
        Some(
            self.chunk.sections[sec].states
                [crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)],
        )
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
            Some(&i) => {
                self.chunk.heightmaps[i].first_available
                    [(x & 15) as usize + ((z & 15) as usize) * 16]
                    - 1
            }
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
            tag: j
                .get("tag")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string(),
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
    Uniform {
        min: HeightAnchor,
        max: HeightAnchor,
    },
    Trapezoid {
        min: HeightAnchor,
        max: HeightAnchor,
        plateau: i32,
    },
    VeryBiasedToBottom {
        min: HeightAnchor,
        max: HeightAnchor,
        inner: i32,
    },
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
        "uniform" => Ok(HeightProvider::Uniform {
            min: min()?,
            max: max()?,
        }),
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

    pub fn sample(
        &self,
        rng: &mut dyn crate::feature_sorter::WorldgenDraws,
        min_y: i32,
        gen_depth: i32,
    ) -> i32 {
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
                val.get("min_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("intprovider min")?,
                val.get("max_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("intprovider max")?,
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
        let ty = j
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("minecraft:uniform");
        match ty.strip_prefix("minecraft:").unwrap_or(ty) {
            "uniform" => Ok(IntProvider::Uniform(min as i32, max as i32)),
            other => Err(format!(
                "unsupported int provider {other} (triangle pending decompile)"
            )),
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
    RandomOffset {
        xz: IntProvider,
        y: IntProvider,
    },
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
    SurfaceWaterDepthFilter {
        max_water_depth: i32,
    },
    /// surface_relative_threshold_filter (glow_lichen, underwater_magma,
    /// lake_lava_underground; inc. 3) — PlacementFilter: shouldPlace =
    /// getHeight(heightmap,x,z)+minInclusive <= y <= +maxInclusive (codec
    /// defaults Int::MIN / Int::MAX; long arithmetic in Java, no rng).
    SurfaceRelativeThresholdFilter {
        heightmap: String,
        min_inclusive: i32,
        max_inclusive: i32,
    },
    /// environment_scan (cave_vines, lush_caves_*, rooted_azalea_tree,
    /// spore_blossom, pine_on_snow, spruce_on_snow, lake_lava_underground;
    /// inc. 3) — vertical position TRANSFORMER (PlacementModifier, NOT a
    /// filter): from the incoming position, while allowed_search_condition
    /// holds, step direction_of_search up to max_steps times; the FIRST
    /// position where target_condition passes is emitted. CFR semantics of
    /// the loop break: the first NOT-allowed position is still tested as a
    /// target before giving up (break falls through to the final check).
    /// max_steps: intRange(1,32) required. No rng draws.
    EnvironmentScan {
        direction: ScanDir,
        target: Box<BlockPredicate>,
        allowed: BlockPredicate,
        max_steps: i32,
    },
    /// any modifier the native lane has not proven bit-exact yet
    Unsupported(String),
}

/// Direction.VERTICAL_CODEC of EnvironmentScanPlacement: up | down only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanDir {
    Up,
    Down,
}

pub fn parse_placement_mod(j: &Json) -> Result<PlacementMod, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("placement type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    Ok(match ty {
        "count" => PlacementMod::Count(IntProvider::parse(j.get("count").ok_or("count")?)?),
        "rarity_filter" => PlacementMod::RarityFilter(
            j.get("chance").and_then(|c| c.as_i64()).ok_or("chance")? as i32,
        ),
        "in_square" => PlacementMod::InSquare,
        "heightmap" => PlacementMod::Heightmap(
            j.get("heightmap")
                .and_then(|h| h.as_str())
                .ok_or("heightmap")?
                .to_string(),
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
        "block_predicate_filter" => PlacementMod::BlockPredicateFilter(parse_predicate(
            j.get("predicate").ok_or("predicate")?,
        )?),
        "noise_based_count" => PlacementMod::NoiseBasedCount {
            noise_to_count_ratio: j
                .get("noise_to_count_ratio")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as f32,
            noise_offset: j
                .get("noise_offset")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            noise_scale: j.get("noise_scale").and_then(|v| v.as_f64()).unwrap_or(0.0),
        },
        "surface_water_depth_filter" => PlacementMod::SurfaceWaterDepthFilter {
            max_water_depth: j
                .get("max_water_depth")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32,
        },
        "surface_relative_threshold_filter" => PlacementMod::SurfaceRelativeThresholdFilter {
            heightmap: j
                .get("heightmap")
                .and_then(|h| h.as_str())
                .ok_or("surface_relative_threshold.heightmap")?
                .to_string(),
            // optionalFieldOf defaults: Int.MIN_VALUE / Int.MAX_VALUE.
            min_inclusive: j
                .get("min_inclusive")
                .and_then(|v| v.as_i64())
                .unwrap_or(i32::MIN as i64) as i32,
            max_inclusive: j
                .get("max_inclusive")
                .and_then(|v| v.as_i64())
                .unwrap_or(i32::MAX as i64) as i32,
        },
        "environment_scan" => {
            let dir = j
                .get("direction_of_search")
                .and_then(|d| d.as_str())
                .ok_or("environment_scan.direction_of_search")?;
            let direction = match dir {
                "up" => ScanDir::Up,
                "down" => ScanDir::Down,
                other => {
                    return Err(format!(
                        "environment_scan direction vertical (up|down), got {other}"
                    ))
                }
            };
            let target = Box::new(parse_predicate(
                j.get("target_condition").ok_or("target_condition")?,
            )?);
            // optionalFieldOf("allowed_search_condition", alwaysTrue()).
            let allowed = match j.get("allowed_search_condition") {
                Some(p) => parse_predicate(p)?,
                None => BlockPredicate::True,
            };
            // intRange(1, 32) — REQUIRED field; out-of-range = codec error
            // (honest reject, mirrors the Java codec).
            let max_steps = j
                .get("max_steps")
                .and_then(|v| v.as_i64())
                .ok_or("environment_scan.max_steps")? as i32;
            if !(1..=32).contains(&max_steps) {
                return Err(format!(
                    "environment_scan max_steps 1..=32, got {max_steps}"
                ));
            }
            PlacementMod::EnvironmentScan {
                direction,
                target,
                allowed,
                max_steps,
            }
        }
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
            let ty = f
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
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
    Ok(PlacedFeatureDef {
        feature_ref,
        placement,
        inline,
    })
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
    SimpleBlock {
        to_place_name: String,
        props: Vec<(String, String)>,
        schedule_tick: bool,
    },
    /// RandomPatchConfiguration: the INNER feature is an INLINE placed
    /// feature (feature+placement object) or a registry ref string.
    RandomPatch {
        tries: i32,
        xz_spread: i32,
        y_spread: i32,
        inner: Box<PlacedFeatureDef>,
    },
    /// RandomFeatureConfiguration ("random_selector", inc. 9): the
    /// weighted features are tried IN ORDER — the FIRST entry with
    /// nextFloat() < chance places and RETURNS (random draw per entry,
    /// short-circuit); if none triggered, the default placed feature
    /// places. Both the weighted entries and the default are
    /// PlacedFeature.CODEC (registry string ref OR inline {feature,
    /// placement} object); chance = floatRange(0..1) fieldOf (out of
    /// range = codec error), features/default fieldOf REQUIRED.
    RandomSelector {
        features: Vec<(f32, Box<PlacedFeatureDef>)>,
        default: Box<PlacedFeatureDef>,
    },
    /// RandomBooleanFeatureConfiguration ("random_boolean_selector",
    /// inc. 9): ONE nextBoolean() draw — true picks feature_true, false
    /// picks feature_false (both PlacedFeature.CODEC fieldOf REQUIRED;
    /// there is no default/fallback).
    RandomBooleanSelector {
        feature_true: Box<PlacedFeatureDef>,
        feature_false: Box<PlacedFeatureDef>,
    },
    /// DiskConfiguration: "target" is a SINGLE block predicate and the
    /// state comes from "state_provider" (inc. 6: FULL RuleBased
    /// BlockStateProvider — first matching rule wins, else the fallback;
    /// the rules field is REQUIRED but MAY be empty). Execution tail:
    /// radius IntProvider sample + betweenClosed disk walk with
    /// per-position provider getState eval (documented, like Spring).
    Disk {
        half_height: i32,
        radius: IntProvider,
        target: BlockPredicate,
        state_provider: StateProvider,
    },
    Kelp,
    Seagrass {
        probability: f32,
    },
    /// SnowAndFreezeFeature — the registry name is "freeze_top_layer"
    /// (Feature.java registration), body class SnowAndFreezeFeature,
    /// NoneFeatureConfiguration ({}). 16x16 column scan at the
    /// MOTION_BLOCKING height; biome shouldFreeze/shouldSnow decide ICE at
    /// height-1 and SNOW at height (plus SNOWY=true on the block below when
    /// it carries the property). Block light at decoration time is 0 (the
    /// light engine is not started at FEATURES) — the <10 checks pass.
    FreezeTopLayer,
    /// SpringFeature — registry name "spring_feature". DETERMINISTIC (no rng
    /// draws): the whole body is neighbour-count checks against
    /// valid_blocks. state is the FLUID state (Name+Properties from the
    /// FluidState codec); FluidState.createLegacyBlock pinning happens at
    /// execution wiring (the material package was not in the session-7
    /// oracle). valid_blocks: HolderSet = "#tag" string, single block id
    /// string (spring_nether_open), or an explicit id array (water/lava).
    Spring {
        state_name: String,
        state_props: Vec<(String, String)>,
        requires_block_below: bool,
        rock_count: i32,
        hole_count: i32,
        valid_blocks: Vec<String>,
    },
    /// MonsterRoomFeature (inc. 4) — NoneFeatureConfiguration ({}).
    /// PARSE-true. Execution tail (documented honestly): the body needs the
    /// block-entity layer — SPAWNER NBT (randomEntityId draw over
    /// skeleton/zombie/zombie/spider) + CHEST with SIMPLE_DUNGEON loot
    /// table (StructurePiece.reorient) — wired at execution, like the
    /// FluidState.createLegacyBlock pin of Spring.
    MonsterRoom,
    /// UnderwaterMagmaFeature (inc. 4) — Column.scan(floor_search_range)
    /// for the water floor, then a radius box with per-position
    /// nextFloat() < probability draws and a water/air-neighbour validity
    /// check. Codec ranges honest-checked at parse: intRange(0,512) /
    /// intRange(0,64) / floatRange(0,1).
    UnderwaterMagma {
        floor_search_range: i32,
        placement_radius_around_floor: i32,
        placement_probability_per_valid_position: f32,
    },
    /// MultifaceGrowthFeature (inc. 4) — glow_lichen / sculk_vein. Codec:
    /// block is byNameCodec().orElse(GLOW_LICHEN) (optional, default
    /// glow_lichen); search_range intRange(1,64) orElse(10); the three
    /// can_place_on_* bools default false; chance_of_spreading
    /// floatRange(0,1) orElse(0.5); can_be_placed_on = HolderSet<Block>
    /// (id array in the corpus, "#tag" accepted). Execution tail:
    /// recursive spreading with shuffled-direction rng.
    MultifaceGrowth {
        block: String,
        search_range: i32,
        can_place_on_floor: bool,
        can_place_on_ceiling: bool,
        can_place_on_wall: bool,
        chance_of_spreading: f32,
        can_be_placed_on: Vec<String>,
    },
    /// GeodeFeature (inc. 5) — "minecraft:geode" (amethyst_geode.json is the
    /// only corpus shape). PARSE-true; the codec is fully mirrored so parse
    /// results are decompile-exact: GeodeBlockSettings 8 REQUIRED fields
    /// (5 simple providers, nonEmptyList inner_placements, 2 TagKey
    /// hashedCodec strings); GeodeLayerSettings doubleRange(0.01,50.0)
    /// orElse 1.7/2.2/3.2/4.2; GeodeCrackSettings orElse 1.0/2.0/2 with
    /// ranges 0..1 / 0..5 / 0..10; CHANCE_RANGE(0,1) scalars orElse
    /// 0.35/0.0/0.05; placements_require_layer0_alternate orElse(true);
    /// IntProvider.codec(1,20) outer_wall_distance orElse uniform(4,5),
    /// distribution_points orElse uniform(3,4), IntProvider.codec(0,10)
    /// point_offset orElse uniform(1,2); min/max_gen_offset orElse -16/16;
    /// invalid_blocks_threshold Codec.INT REQUIRED. Out-of-range = honest
    /// codec error (no clamp), like underwater_magma.
    /// Execution tail (documented honestly): place() builds a geode-private
    /// NormalNoise.create(WorldgenRandom(new LegacyRandomSource(level
    /// .getSeed())), -4, [1.0]) — DETERMINISTIC per world seed but drawn at
    /// place time; distribution_points.sample draws; layer radii are
    /// 1/sqrt(filling|inner+d|...) with d = points/maxOuterWall; crack
    /// boxes from generate_crack_chance; per-position noise*noise_multiplier
    /// sum of invSqrt(distSqr+pointOffset) vs thresholds; budding amethyst
    /// clusters with FACING/WATERLOGGED property writes + fluid schedule
    /// ticks. Wired at execution like the Spring FluidState pin.
    Geode {
        filling_provider: String,
        inner_layer_provider: String,
        alternate_inner_layer_provider: String,
        middle_layer_provider: String,
        outer_layer_provider: String,
        inner_placements: Vec<String>,
        cannot_replace: String,
        invalid_blocks: String,
        filling: f32,
        inner_layer: f32,
        middle_layer: f32,
        outer_layer: f32,
        generate_crack_chance: f32,
        base_crack_size: f32,
        crack_point_offset: i32,
        use_potential_placements_chance: f32,
        use_alternate_layer0_chance: f32,
        placements_require_layer0_alternate: bool,
        outer_wall_distance: IntProvider,
        distribution_points: IntProvider,
        point_offset: IntProvider,
        min_gen_offset: i32,
        max_gen_offset: i32,
        noise_multiplier: f32,
        invalid_blocks_threshold: i32,
    },
    /// LakeFeature (inc. 7) — "minecraft:lake" (lake_lava.json is the only
    /// corpus shape; the class is @Deprecated but still registered).
    /// Codec mirror: fluid + barrier, BOTH fieldOf (REQUIRED) over
    /// BlockStateProvider.CODEC (rule_based shapes parse via the full
    /// inc.-6 provider path). No optional fields, no defaults — a missing
    /// key is an honest codec error. Execution tail (documented honestly):
    /// origin.y <= minWorldY+4 => false, then origin.below(4); an rng
    /// ellipsoid blob fills flags[16x16x8] over nextInt(4)+4 iterations
    /// (radii nextDouble()*6+3 / *4+2 / *6+3, centers clamped inside
    /// 1+r/2 .. 15-r/2); boundary checks reject liquid in the UPPER half
    /// and non-solid non-fluid in the LOWER; placement pass uses
    /// canReplaceBlock = !FEATURES_CANNOT_REPLACE with the upper half set
    /// to CAVE_AIR (+ scheduleTick + markAboveForPostProcessing) and the
    /// lower half to the fluid state; the barrier pass (non-air barrier
    /// state) replaces solid cells except LAVA_POOL_STONE_CANNOT_REPLACE
    /// with a nextInt(2) skip at y>=4; the freeze pass puts ICE on the
    /// y=4 plane when the fluid carries FluidTags.WATER and the biome
    /// shouldFreeze. Wired at execution like the Spring FluidState pin.
    Lake {
        fluid: StateProvider,
        barrier: StateProvider,
    },
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
            let ty = j
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
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
        Ok(FeatureRegistry {
            configured,
            placed,
            dir,
        })
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
            // orElse defaults per RandomPatchConfiguration.CODEC (inc. 8
            // fix: 32/0/0 -> 128/7/3 — POSITIVE_INT tries=128, NON_NEGATIVE
            // xz_spread=7, y_spread=3).
            FeatureDef::RandomPatch {
                tries: cfg.get("tries").and_then(|t| t.as_i64()).unwrap_or(128) as i32,
                xz_spread: cfg.get("xz_spread").and_then(|t| t.as_i64()).unwrap_or(7) as i32,
                y_spread: cfg.get("y_spread").and_then(|t| t.as_i64()).unwrap_or(3) as i32,
                inner: Box::new(inner),
            }
        }
        "random_selector" => {
            let cfg = j.get("config").ok_or("random_selector.config")?;
            // WeightedPlacedFeature.CODEC.listOf().fieldOf("features") —
            // REQUIRED (an absent key is a codec error; an EMPTY list is
            // legal and just means "always the default").
            let mut features = Vec::new();
            for w in cfg
                .get("features")
                .and_then(|f| f.as_arr())
                .ok_or("random_selector.features")?
            {
                let chance = w
                    .get("chance")
                    .and_then(|c| c.as_f64())
                    .ok_or("random_selector.chance")?;
                // Codec.floatRange(0, 1): out-of-range = codec error.
                if !(0.0..=1.0).contains(&chance) {
                    return Err(format!("random_selector chance 0..=1, got {chance}"));
                }
                let feature = match w.get("feature") {
                    // PlacedFeature.CODEC: registry string ref OR inline
                    // {feature, placement} object (same dual form the
                    // random_patch inner uses).
                    Some(fj) if fj.as_str().is_some() => PlacedFeatureDef {
                        feature_ref: expand_rl(fj.as_str().unwrap()),
                        placement: Vec::new(),
                        inline: None,
                    },
                    Some(fj) => parse_placed_feature(fj)?,
                    None => return Err("random_selector.feature".into()),
                };
                features.push((chance as f32, Box::new(feature)));
            }
            let default = match cfg.get("default") {
                Some(dj) if dj.as_str().is_some() => Box::new(PlacedFeatureDef {
                    feature_ref: expand_rl(dj.as_str().unwrap()),
                    placement: Vec::new(),
                    inline: None,
                }),
                Some(dj) => Box::new(parse_placed_feature(dj)?),
                None => return Err("random_selector.default".into()),
            };
            FeatureDef::RandomSelector { features, default }
        }
        "random_boolean_selector" => {
            let cfg = j.get("config").ok_or("random_boolean_selector.config")?;
            // Both halves fieldOf (REQUIRED) — a missing key = honest
            // codec error, no defaults anywhere.
            let feature_true = match cfg.get("feature_true") {
                Some(tj) if tj.as_str().is_some() => Box::new(PlacedFeatureDef {
                    feature_ref: expand_rl(tj.as_str().unwrap()),
                    placement: Vec::new(),
                    inline: None,
                }),
                Some(tj) => Box::new(parse_placed_feature(tj)?),
                None => return Err("random_boolean_selector.feature_true".into()),
            };
            let feature_false = match cfg.get("feature_false") {
                Some(fj) if fj.as_str().is_some() => Box::new(PlacedFeatureDef {
                    feature_ref: expand_rl(fj.as_str().unwrap()),
                    placement: Vec::new(),
                    inline: None,
                }),
                Some(fj) => Box::new(parse_placed_feature(fj)?),
                None => return Err("random_boolean_selector.feature_false".into()),
            };
            FeatureDef::RandomBooleanSelector {
                feature_true,
                feature_false,
            }
        }
        "disk" => {
            let cfg = j.get("config").ok_or("disk.config")?;
            let radius = IntProvider::parse(cfg.get("radius").ok_or("radius")?)?;
            let half = cfg.get("half_height").and_then(|h| h.as_i64()).unwrap_or(0) as i32;
            // DiskConfiguration: target = SINGLE block predicate;
            // state_provider = FULL RuleBasedBlockStateProvider (inc. 6)
            let target = parse_predicate(cfg.get("target").ok_or("disk.target")?)?;
            let sp = cfg.get("state_provider").ok_or("disk.state_provider")?;
            let state_provider = parse_state_provider(sp)?;
            FeatureDef::Disk {
                half_height: half,
                radius,
                target,
                state_provider,
            }
        }
        "lake" => {
            let cfg = j.get("config").ok_or("lake.config")?;
            // LakeFeature.Configuration (CFR): fluid + barrier — BOTH
            // fieldOf (REQUIRED, no orElse defaults); the field codec is
            // BlockStateProvider.CODEC so a rule_based shape goes through
            // the FULL inc.-6 provider path (not the simple-only wrapper).
            let fluid = parse_state_provider(cfg.get("fluid").ok_or("lake.fluid")?)?;
            let barrier = parse_state_provider(cfg.get("barrier").ok_or("lake.barrier")?)?;
            FeatureDef::Lake { fluid, barrier }
        }
        "kelp" => FeatureDef::Kelp,
        "seagrass" => FeatureDef::Seagrass {
            probability: j
                .get("config")
                .and_then(|c| c.get("probability"))
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0) as f32,
        },
        "freeze_top_layer" => FeatureDef::FreezeTopLayer,
        "monster_room" => FeatureDef::MonsterRoom,
        "underwater_magma" => {
            let cfg = j.get("config").ok_or("underwater_magma.config")?;
            // intRange / floatRange codecs: REQUIRED fields, out-of-range
            // = codec error (honest reject, no silent clamp).
            let fsr = cfg
                .get("floor_search_range")
                .and_then(|v| v.as_i64())
                .ok_or("underwater_magma.floor_search_range")? as i32;
            if !(0..=512).contains(&fsr) {
                return Err(format!(
                    "underwater_magma floor_search_range 0..=512, got {fsr}"
                ));
            }
            let radius = cfg
                .get("placement_radius_around_floor")
                .and_then(|v| v.as_i64())
                .ok_or("underwater_magma.placement_radius_around_floor")?
                as i32;
            if !(0..=64).contains(&radius) {
                return Err(format!(
                    "underwater_magma placement_radius_around_floor 0..=64, got {radius}"
                ));
            }
            let prob = cfg
                .get("placement_probability_per_valid_position")
                .and_then(|v| v.as_f64())
                .ok_or("underwater_magma.placement_probability_per_valid_position")?
                as f32;
            if !(0.0..=1.0).contains(&prob) {
                return Err(format!(
                    "underwater_magma placement_probability 0..=1, got {prob}"
                ));
            }
            FeatureDef::UnderwaterMagma {
                floor_search_range: fsr,
                placement_radius_around_floor: radius,
                placement_probability_per_valid_position: prob,
            }
        }
        "multiface_growth" => {
            let cfg = j.get("config").ok_or("multiface_growth.config")?;
            // byNameCodec().orElse(GLOW_LICHEN): optional field, default
            // glow_lichen (the flatXmap MultifaceSpreadeableBlock check
            // narrows the registry to lichen-like blocks).
            let block = cfg
                .get("block")
                .and_then(|b| b.as_str())
                .unwrap_or("minecraft:glow_lichen")
                .to_string();
            let mut can_be_placed_on = Vec::new();
            match cfg
                .get("can_be_placed_on")
                .ok_or("multiface.can_be_placed_on")?
            {
                Json::Str(s) => can_be_placed_on.push(s.clone()), // "#tag" HolderSet
                Json::Arr(a) => {
                    for e in a {
                        can_be_placed_on
                            .push(e.as_str().ok_or("multiface.holder entry")?.to_string());
                    }
                }
                _ => return Err("multiface.can_be_placed_on shape".into()),
            }
            FeatureDef::MultifaceGrowth {
                block,
                search_range: cfg
                    .get("search_range")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(10) as i32,
                can_place_on_floor: cfg
                    .get("can_place_on_floor")
                    .and_then(|v| match v {
                        Json::Bool(b) => Some(*b),
                        _ => None,
                    })
                    .unwrap_or(false),
                can_place_on_ceiling: cfg
                    .get("can_place_on_ceiling")
                    .and_then(|v| match v {
                        Json::Bool(b) => Some(*b),
                        _ => None,
                    })
                    .unwrap_or(false),
                can_place_on_wall: cfg
                    .get("can_place_on_wall")
                    .and_then(|v| match v {
                        Json::Bool(b) => Some(*b),
                        _ => None,
                    })
                    .unwrap_or(false),
                chance_of_spreading: cfg
                    .get("chance_of_spreading")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5) as f32,
                can_be_placed_on,
            }
        }
        "geode" => {
            let cfg = j.get("config").ok_or("geode.config")?;
            // GeodeBlockSettings: ALL 8 fields are fieldOf (REQUIRED).
            let blocks = cfg.get("blocks").ok_or("geode.blocks")?;
            let provider = |key: &str| -> Result<String, String> {
                parse_simple_state_provider(blocks.get(key).ok_or_else(|| format!("geode.{key}"))?)
            };
            let filling_provider = provider("filling_provider")?;
            let inner_layer_provider = provider("inner_layer_provider")?;
            let alternate_inner_layer_provider = provider("alternate_inner_layer_provider")?;
            let middle_layer_provider = provider("middle_layer_provider")?;
            let outer_layer_provider = provider("outer_layer_provider")?;
            // inner_placements = ExtraCodecs.nonEmptyList(BlockState list)
            // — [] is a codec error (honest).
            let mut inner_placements = Vec::new();
            match blocks
                .get("inner_placements")
                .ok_or("geode.inner_placements")?
            {
                Json::Arr(a) => {
                    for st in a {
                        let name = st
                            .get("Name")
                            .and_then(|n| n.as_str())
                            .ok_or("geode.inner_placements.Name")?
                            .to_string();
                        let mut props = Vec::new();
                        if let Some(Json::Obj(po)) = st.get("Properties") {
                            for (k, v) in po {
                                props.push(format!("{}={}", k, v.as_str().unwrap_or("")));
                            }
                        }
                        props.sort();
                        inner_placements.push(if props.is_empty() {
                            expand_rl(&name)
                        } else {
                            format!("{}[{}]", expand_rl(&name), props.join(","))
                        });
                    }
                }
                _ => return Err("geode.inner_placements shape".into()),
            }
            if inner_placements.is_empty() {
                return Err("geode.inner_placements nonEmptyList, got []".into());
            }
            let cannot_replace = blocks
                .get("cannot_replace")
                .and_then(|t| t.as_str())
                .ok_or("geode.cannot_replace")?
                .to_string();
            let invalid_blocks = blocks
                .get("invalid_blocks")
                .and_then(|t| t.as_str())
                .ok_or("geode.invalid_blocks")?
                .to_string();
            // GeodeLayerSettings: LAYER_RANGE = Codec.doubleRange(0.01, 50),
            // all four orElse (1.7 / 2.2 / 3.2 / 4.2). "layers" itself is
            // REQUIRED (fieldOf).
            let layers = cfg.get("layers").ok_or("geode.layers")?;
            let layer = |key: &str, def: f64| -> Result<f32, String> {
                let v = layers.get(key).and_then(|v| v.as_f64()).unwrap_or(def);
                if !(0.01..=50.0).contains(&v) {
                    return Err(format!("geode layer {key} 0.01..=50.0, got {v}"));
                }
                Ok(v as f32)
            };
            let filling = layer("filling", 1.7)?;
            let inner_layer = layer("inner_layer", 2.2)?;
            let middle_layer = layer("middle_layer", 3.2)?;
            let outer_layer = layer("outer_layer", 4.2)?;
            // GeodeCrackSettings: all three orElse (1.0 / 2.0 / 2) with
            // ranges CHANCE_RANGE(0,1) / 0..5 / intRange(0,10).
            let crack = cfg.get("crack").ok_or("geode.crack")?;
            let generate_crack_chance = crack
                .get("generate_crack_chance")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0);
            if !(0.0..=1.0).contains(&generate_crack_chance) {
                return Err(format!(
                    "geode generate_crack_chance 0..=1, got {generate_crack_chance}"
                ));
            }
            let base_crack_size = crack
                .get("base_crack_size")
                .and_then(|v| v.as_f64())
                .unwrap_or(2.0);
            if !(0.0..=5.0).contains(&base_crack_size) {
                return Err(format!(
                    "geode base_crack_size 0..=5.0, got {base_crack_size}"
                ));
            }
            let crack_point_offset = crack
                .get("crack_point_offset")
                .and_then(|v| v.as_i64())
                .unwrap_or(2) as i32;
            if !(0..=10).contains(&crack_point_offset) {
                return Err(format!(
                    "geode crack_point_offset 0..=10, got {crack_point_offset}"
                ));
            }
            // Top-level scalars: CHANCE_RANGE = Codec.doubleRange(0,1).
            let chance = |key: &str, def: f64| -> Result<f32, String> {
                let v = cfg.get(key).and_then(|v| v.as_f64()).unwrap_or(def);
                if !(0.0..=1.0).contains(&v) {
                    return Err(format!("geode {key} 0..=1, got {v}"));
                }
                Ok(v as f32)
            };
            let use_potential_placements_chance = chance("use_potential_placements_chance", 0.35)?;
            let use_alternate_layer0_chance = chance("use_alternate_layer0_chance", 0.0)?;
            let noise_multiplier = chance("noise_multiplier", 0.05)?;
            let placements_require_layer0_alternate =
                match cfg.get("placements_require_layer0_alternate") {
                    Some(Json::Bool(b)) => *b,
                    None => true, // orElse(true)
                    _ => return Err("geode.placements_require_layer0_alternate".into()),
                };
            // IntProvider.codec(lo,hi): out-of-bounds = honest codec
            // error; the orElse defaults are UniformInt.of(a,b).
            let intprov =
                |key: &str, lo: i32, hi: i32, da: i32, db: i32| -> Result<IntProvider, String> {
                    let parsed = match cfg.get(key) {
                        Some(v) => IntProvider::parse(v)?,
                        None => IntProvider::Uniform(da, db),
                    };
                    let (a, b) = match parsed {
                        IntProvider::Constant(c) => (c, c),
                        IntProvider::Uniform(a, b) => (a, b),
                        IntProvider::Triangle(..) => {
                            return Err(format!("geode {key}: triangle provider"))
                        }
                    };
                    if a < lo || b > hi {
                        return Err(format!("geode {key} {lo}..={hi}, got {a}..{b}"));
                    }
                    Ok(parsed)
                };
            let outer_wall_distance = intprov("outer_wall_distance", 1, 20, 4, 5)?;
            let distribution_points = intprov("distribution_points", 1, 20, 3, 4)?;
            let point_offset = intprov("point_offset", 0, 10, 1, 2)?;
            let min_gen_offset = cfg
                .get("min_gen_offset")
                .and_then(|v| v.as_i64())
                .unwrap_or(-16) as i32;
            let max_gen_offset = cfg
                .get("max_gen_offset")
                .and_then(|v| v.as_i64())
                .unwrap_or(16) as i32;
            // Codec.INT, fieldOf — REQUIRED, any int value accepted.
            let invalid_blocks_threshold =
                cfg.get("invalid_blocks_threshold")
                    .and_then(|v| v.as_i64())
                    .ok_or("geode.invalid_blocks_threshold")? as i32;
            FeatureDef::Geode {
                filling_provider,
                inner_layer_provider,
                alternate_inner_layer_provider,
                middle_layer_provider,
                outer_layer_provider,
                inner_placements,
                cannot_replace,
                invalid_blocks,
                filling,
                inner_layer,
                middle_layer,
                outer_layer,
                generate_crack_chance: generate_crack_chance as f32,
                base_crack_size: base_crack_size as f32,
                crack_point_offset,
                use_potential_placements_chance,
                use_alternate_layer0_chance,
                placements_require_layer0_alternate,
                outer_wall_distance,
                distribution_points,
                point_offset,
                min_gen_offset,
                max_gen_offset,
                noise_multiplier,
                invalid_blocks_threshold,
            }
        }
        "spring_feature" => {
            let cfg = j.get("config").ok_or("spring.config")?;
            let st = cfg.get("state").ok_or("spring.state")?;
            let state_name = st
                .get("Name")
                .and_then(|n| n.as_str())
                .ok_or("spring.state.Name")?
                .to_string();
            let mut state_props = Vec::new();
            if let Some(Json::Obj(po)) = st.get("Properties") {
                for (k, v) in po {
                    state_props.push((k.clone(), v.as_str().unwrap_or("").to_string()));
                }
            }
            state_props.sort();
            let mut valid_blocks = Vec::new();
            match cfg.get("valid_blocks").ok_or("spring.valid_blocks")? {
                Json::Str(s) => valid_blocks.push(s.clone()),
                Json::Arr(a) => {
                    for b in a {
                        valid_blocks.push(b.as_str().ok_or("spring.block")?.to_string());
                    }
                }
                _ => return Err("spring.valid_blocks shape".into()),
            }
            FeatureDef::Spring {
                state_name,
                state_props,
                requires_block_below: cfg
                    .get("requires_block_below")
                    .and_then(|v| match v {
                        Json::Bool(b) => Some(*b),
                        _ => None,
                    })
                    .unwrap_or(true),
                rock_count: cfg.get("rock_count").and_then(|v| v.as_i64()).unwrap_or(4) as i32,
                hole_count: cfg.get("hole_count").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
                valid_blocks,
            }
        }
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

/// BlockStateProvider IR (inc. 6) — RuleBasedBlockStateProvider +
/// simple_state_provider. CFR RuleBasedBlockStateProvider.getState: the
/// FIRST rule whose if_true passes at the position wins, otherwise the
/// fallback; "rules" is a REQUIRED field but MAY be empty (disk_clay /
/// disk_gravel). Other provider types (noise/forest/...) => Err (honest
/// Unsupported).
#[derive(Debug, Clone)]
pub enum StateProvider {
    /// simple_state_provider — fixed compact state, no rng draws.
    Simple(String),
    RuleBased {
        fallback: Box<StateProvider>,
        rules: Vec<(BlockPredicate, StateProvider)>,
    },
}

pub fn parse_state_provider(j: &Json) -> Result<StateProvider, String> {
    // RuleBased wrapper: {fallback: provider, rules: [{if_true, then}, ...]}
    // — both fields fieldOf (REQUIRED); rules may be [].
    if let Some(f) = j.get("fallback") {
        let fallback = Box::new(parse_state_provider(f)?);
        let mut rules = Vec::new();
        match j.get("rules").ok_or("rule_based.rules")? {
            Json::Arr(a) => {
                for r in a {
                    let if_true = parse_predicate(r.get("if_true").ok_or("rule.if_true")?)?;
                    let then = parse_state_provider(r.get("then").ok_or("rule.then")?)?;
                    rules.push((if_true, then));
                }
            }
            _ => return Err("rule_based.rules shape".into()),
        }
        return Ok(StateProvider::RuleBased { fallback, rules });
    }
    let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    if ty != "simple_state_provider" {
        return Err(format!("unsupported state provider {ty}"));
    }
    let st = j.get("state").ok_or("state")?;
    let state = parse_state_name(st)?;
    Ok(StateProvider::Simple(state))
}

/// Compact-state leaf for callers that must stay SIMPLE (random_patch
/// to_place, geode block providers — the corpus shapes there are all
/// simple; a RuleBased shape there stays an honest Unsupported).
fn parse_simple_state_provider(j: &Json) -> Result<String, String> {
    match parse_state_provider(j)? {
        StateProvider::Simple(s) => Ok(s),
        StateProvider::RuleBased { .. } => Err("rule_based_state_provider with rules".into()),
    }
}

/// The tier-3 dispatch verdict (P4.5): which features this chunk needs vs
/// what the native lane supports — falls back (I8) on the first unsupported.
pub fn decoration_supported(
    features_needed: &[usize],
    registry: &FeatureRegistry,
    placed_ids: &[String],
) -> bool {
    for &pf in features_needed {
        let Some(def) = placed_ids
            .get(pf)
            .and_then(|k| registry.placed.get(k.as_str()))
        else {
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
            return !matches!(
                short.as_str(),
                "tree" | "random_selector" | "simple_random_selector" | "random_boolean_selector"
            );
        }
        None => match registry.configured.get(&def.feature_ref) {
            Some(cfg) => cfg,
            None => return false,
        },
    };
    match body {
        FeatureDef::Unsupported(_) => false,
        FeatureDef::RandomPatch { inner, .. } => is_placed_supported(inner, registry),
        FeatureDef::RandomSelector { features, default } => {
            features.iter().all(|(_, f)| is_placed_supported(f, registry))
                && is_placed_supported(default, registry)
        }
        FeatureDef::RandomBooleanSelector {
            feature_true,
            feature_false,
        } => {
            is_placed_supported(feature_true, registry)
                && is_placed_supported(feature_false, registry)
        }
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
            cfg.get("targets").and_then(|t| t.as_arr()).unwrap()[1]
                .get("target")
                .unwrap(),
        );
        assert!(
            matches!(rt, RuleTest::TagMatch { ref tag } if tag == "minecraft:stone_ore_replaceables")
        );
        let rt2 = parse_rule_test(
            cfg.get("targets").and_then(|t| t.as_arr()).unwrap()[0]
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
        // zero offset: the tested position IS the probe position
        let at = |x: i32, y: i32, z: i32| -> Option<String> {
            if (x, y, z) == (0, 0, 0) {
                Some("minecraft:clay".into())
            } else {
                Some("minecraft:stone".into())
            }
        };
        assert!(eval_predicate(&target, 0, 0, 0, &at, &repl, &tag_of));
        assert!(!eval_predicate(&target, 5, 5, 5, &at, &repl, &tag_of));
        // inc. 6: rules:[] parses as RuleBased with an empty rule list.
        let sp = parse_state_provider(cfg.get("state_provider").unwrap()).unwrap();
        match &sp {
            StateProvider::RuleBased { fallback, rules } => {
                assert!(rules.is_empty());
                assert!(matches!(
                    fallback.as_ref(),
                    StateProvider::Simple(s) if s == "minecraft:clay"
                ));
            }
            _ => panic!("expected RuleBased provider"),
        }
        // radius uniform parse
        let r = IntProvider::parse(cfg.get("radius").unwrap()).unwrap();
        assert!(matches!(r, IntProvider::Uniform(2, 3)));
    }

    #[test]
    fn disk_sand_rule_based_provider_verbatim() {
        // configured_feature/disk_sand.json — 1 rule: matching_blocks(air)
        // at offset [0,-1,0] -> sandstone over the sand fallback.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:disk\",\"config\":{\"half_height\":2,\
             \"radius\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":6,\"min_inclusive\":2},\
             \"state_provider\":{\"fallback\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:sand\"}},\"rules\":[\
             {\"if_true\":{\"type\":\"minecraft:matching_blocks\",\
             \"blocks\":\"minecraft:air\",\"offset\":[0,-1,0]},\
             \"then\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:sandstone\"}}}]},\
             \"target\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":[\"minecraft:dirt\",\"minecraft:grass_block\"]}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Disk {
            half_height,
            radius,
            target,
            state_provider,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Disk");
        };
        assert_eq!(half_height, 2);
        assert!(matches!(radius, IntProvider::Uniform(2, 6)));
        let StateProvider::RuleBased { fallback, rules } = &state_provider else {
            panic!("expected RuleBased provider");
        };
        assert_eq!(rules.len(), 1);
        assert!(matches!(
            fallback.as_ref(),
            StateProvider::Simple(s) if s == "minecraft:sand"
        ));
        assert!(matches!(
            rules[0].1,
            StateProvider::Simple(ref s) if s == "minecraft:sandstone"
        ));
        // rule predicate: air at offset (0,-1,0) — eval against a stub
        // world (the position BELOW the probe is air, everything else sand).
        let at = |x: i32, y: i32, z: i32| -> Option<String> {
            if (x, y, z) == (0, -1, 0) {
                Some("minecraft:air".into())
            } else {
                Some("minecraft:sand".into())
            }
        };
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        assert!(eval_predicate(&rules[0].0, 0, 0, 0, &at, &repl, &tag_of));
        // target = dirt|grass_block: the sand stub world must NOT match.
        assert!(!eval_predicate(&target, 0, 0, 0, &at, &repl, &tag_of));
    }

    #[test]
    fn disk_grass_rule_not_any_of_verbatim() {
        // configured_feature/disk_grass.json — rule if_true =
        // NOT(ANY_OF(solid above, matching_fluids water above)) ->
        // grass_block[snowy=false] over the dirt fallback.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:disk\",\"config\":{\"half_height\":2,\
             \"radius\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":6,\"min_inclusive\":2},\
             \"state_provider\":{\"fallback\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:dirt\"}},\"rules\":[\
             {\"if_true\":{\"type\":\"minecraft:not\",\"predicate\":\
             {\"type\":\"minecraft:any_of\",\"predicates\":[\
             {\"type\":\"minecraft:solid\",\"offset\":[0,1,0]},\
             {\"type\":\"minecraft:matching_fluids\",\"fluids\":\"minecraft:water\",\
             \"offset\":[0,1,0]}]}},\
             \"then\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:grass_block\",\"Properties\":{\"snowy\":\"false\"}}}}]},\
             \"target\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":[\"minecraft:dirt\",\"minecraft:mud\"]}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Disk { state_provider, .. } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Disk");
        };
        let StateProvider::RuleBased { fallback, rules } = &state_provider else {
            panic!("expected RuleBased provider");
        };
        assert_eq!(rules.len(), 1);
        assert!(matches!(
            fallback.as_ref(),
            StateProvider::Simple(s) if s == "minecraft:dirt"
        ));
        assert!(matches!(
            rules[0].1,
            StateProvider::Simple(ref s) if s == "minecraft:grass_block[snowy=false]"
        ));
        // rule predicate eval: NOT(any_of(solid@above, water@above)) —
        // true only when the block ABOVE is neither solid nor water.
        let pred = &rules[0].0;
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        let world = |above: &'static str| {
            move |x: i32, y: i32, z: i32| -> Option<String> {
                if (x, y, z) == (0, 1, 0) {
                    Some(above.to_string())
                } else {
                    Some("minecraft:dirt".into())
                }
            }
        };
        assert!(eval_predicate(
            pred,
            0,
            0,
            0,
            &world("minecraft:air"),
            &repl,
            &tag_of
        ));
        assert!(!eval_predicate(
            pred,
            0,
            0,
            0,
            &world("minecraft:stone"),
            &repl,
            &tag_of
        ));
        assert!(!eval_predicate(
            pred,
            0,
            0,
            0,
            &world("minecraft:water"),
            &repl,
            &tag_of
        ));
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
            let at = |x: i32, y: i32, z: i32| -> Option<String> {
                if (x, y, z) == (0, 0, 0) {
                    Some("minecraft:air".into())
                } else {
                    Some("minecraft:stone".into())
                }
            };
            assert!(eval_predicate(p, 0, 0, 0, &at, &repl, &tag_of));
            assert!(!eval_predicate(p, 3, 3, 3, &at, &repl, &tag_of));
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

    // ---- P4 increment 2: freeze_top_layer, spring_feature, offset
    // predicates — ALL shapes below are verbatim corpus JSON
    // (data/minecraft/worldgen/... of the 1.21.10 jar-fresh extract).

    #[test]
    fn freeze_top_layer_real_shape() {
        // configured_feature/freeze_top_layer.json + placed_feature wrapper
        let j =
            crate::json::parse("{\"type\":\"minecraft:freeze_top_layer\",\"config\":{}}").unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let def = parse_configured_def(&j, short).unwrap();
        assert!(matches!(def, FeatureDef::FreezeTopLayer));
        // placed wrapper: biome filter only — fully supported verdict
        let pj = crate::json::parse(
            "{\"feature\":\"minecraft:freeze_top_layer\",\"placement\":[\
             {\"type\":\"minecraft:biome\"}]}",
        )
        .unwrap();
        let placed = parse_placed_feature(&pj).unwrap();
        assert_eq!(placed.feature_ref, "minecraft:freeze_top_layer");
    }

    #[test]
    fn spring_water_real_shape() {
        // configured_feature/spring_water.json — array HolderSet, state with
        // Properties, defaults present (requires_block_below=true).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:spring_feature\",\"config\":{\"hole_count\":1,\
             \"requires_block_below\":true,\"rock_count\":4,\
             \"state\":{\"Name\":\"minecraft:water\",\"Properties\":{\"falling\":\"true\"}},\
             \"valid_blocks\":[\"minecraft:stone\",\"minecraft:granite\",\"minecraft:diorite\",\
             \"minecraft:andesite\",\"minecraft:deepslate\",\"minecraft:tuff\",\
             \"minecraft:calcite\",\"minecraft:dirt\",\"minecraft:snow_block\",\
             \"minecraft:powder_snow\",\"minecraft:packed_ice\"]}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Spring {
            state_name,
            state_props,
            requires_block_below,
            rock_count,
            hole_count,
            valid_blocks,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Spring");
        };
        assert_eq!(state_name, "minecraft:water");
        assert_eq!(
            state_props,
            vec![("falling".to_string(), "true".to_string())]
        );
        assert!(requires_block_below);
        assert_eq!(rock_count, 4);
        assert_eq!(hole_count, 1);
        assert_eq!(valid_blocks.len(), 11);
        // the placed wrapper (count 25 / in_square / height_range uniform
        // absolute 192 above_bottom 0 / biome) must parse fully
        let pj = crate::json::parse(
            "{\"feature\":\"minecraft:spring_water\",\"placement\":[\
             {\"type\":\"minecraft:count\",\"count\":25},{\"type\":\"minecraft:in_square\"},\
             {\"type\":\"minecraft:height_range\",\"height\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":{\"absolute\":192},\"min_inclusive\":{\"above_bottom\":0}}},\
             {\"type\":\"minecraft:biome\"}]}",
        )
        .unwrap();
        assert!(parse_placed_feature(&pj).is_ok());
    }

    #[test]
    fn spring_nether_open_single_id_valid_blocks() {
        // configured_feature/spring_nether_open.json — HolderSet DIRECT id
        // (single string, no #), requires_block_below=false.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:spring_feature\",\"config\":{\"hole_count\":1,\
             \"requires_block_below\":false,\"rock_count\":4,\
             \"state\":{\"Name\":\"minecraft:lava\",\"Properties\":{\"falling\":\"true\"}},\
             \"valid_blocks\":\"minecraft:netherrack\"}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Spring {
            requires_block_below,
            valid_blocks,
            ..
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Spring");
        };
        assert!(!requires_block_below);
        assert_eq!(valid_blocks, vec!["minecraft:netherrack"]);
    }

    #[test]
    fn matching_fluids_offset_real_shape() {
        // patch_sugar_cane.json inner any_of — matching_fluids with
        // offsets [1,-1,0] / [-1,-1,0] / [0,-1,1] / [0,-1,-1]; fluids list
        // covers still + flowing.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:any_of\",\"predicates\":[\
             {\"type\":\"minecraft:matching_fluids\",\"fluids\":[\"minecraft:water\",\
             \"minecraft:flowing_water\"],\"offset\":[1,-1,0]},\
             {\"type\":\"minecraft:matching_fluids\",\"fluids\":[\"minecraft:water\",\
             \"minecraft:flowing_water\"],\"offset\":[-1,-1,0]},\
             {\"type\":\"minecraft:matching_fluids\",\"fluids\":[\"minecraft:water\",\
             \"minecraft:flowing_water\"],\"offset\":[0,-1,1]},\
             {\"type\":\"minecraft:matching_fluids\",\"fluids\":[\"minecraft:water\",\
             \"minecraft:flowing_water\"],\"offset\":[0,-1,-1]}]}",
        )
        .unwrap();
        let p = parse_predicate(&j).unwrap();
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        // sugar-cane sits on a grass block with water to one SIDE below:
        // probe at (0,64,0), water at (1,63,0) => the [1,-1,0] arm hits
        let at = |x: i32, y: i32, z: i32| -> Option<String> {
            if x == 1 && y == 63 && z == 0 {
                Some("minecraft:water[level=0]".into())
            } else {
                Some("minecraft:grass_block[snowy=false]".into())
            }
        };
        assert!(eval_predicate(&p, 0, 64, 0, &at, &repl, &tag_of));
        // no water anywhere below/side => false
        let dry = |_: i32, _: i32, _: i32| -> Option<String> {
            Some("minecraft:grass_block[snowy=false]".into())
        };
        assert!(!eval_predicate(&p, 0, 64, 0, &dry, &repl, &tag_of));
    }

    #[test]
    fn offset_clamp_and_solid_predicate() {
        // offsetCodec(16): components clamp to ±16 (tolerant parse); the
        // test probes [17,-20,0] -> [16,-16,0].
        let j = crate::json::parse("{\"type\":\"minecraft:solid\",\"offset\":[17,-20,0]}").unwrap();
        let p = parse_predicate(&j).unwrap();
        match &p {
            BlockPredicate::Solid { offset } => assert_eq!(offset, &[16, -16, 0]),
            other => panic!("expected Solid, got {other:?}"),
        }
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        // solid at the offset position (16,-16) below the probe (0,0,0)
        let at = |x: i32, y: i32, z: i32| -> Option<String> {
            if (x, y, z) == (16, -16, 0) {
                Some("minecraft:stone".into())
            } else {
                Some("minecraft:water[level=0]".into())
            }
        };
        assert!(eval_predicate(&p, 0, 0, 0, &at, &repl, &tag_of));
        // water at the tested position => not solid
        let wet = |x: i32, y: i32, _: i32| -> Option<String> {
            if (x, y) == (16, -16) {
                Some("minecraft:water[level=0]".into())
            } else {
                Some("minecraft:stone".into())
            }
        };
        assert!(!eval_predicate(&p, 0, 0, 0, &wet, &repl, &tag_of));
        // out of region => cannot verify => false (honest)
        let none = |_: i32, _: i32, _: i32| -> Option<String> { None };
        assert!(!eval_predicate(&p, 0, 0, 0, &none, &repl, &tag_of));
    }

    #[test]
    fn surface_relative_threshold_glow_lichen_verbatim() {
        // placed_feature/glow_lichen.json — srtf with ONLY max_inclusive
        // (min defaults to Int.MIN_VALUE); OCEAN_FLOOR_WG heightmap.
        let j = crate::json::parse(
            "{\"feature\":\"minecraft:glow_lichen\",\"placement\":[\
             {\"type\":\"minecraft:count\",\"count\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":157,\"min_inclusive\":104}},\
             {\"type\":\"minecraft:height_range\",\"height\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":{\"absolute\":256},\"min_inclusive\":{\"above_bottom\":0}}},\
             {\"type\":\"minecraft:in_square\"},\
             {\"type\":\"minecraft:surface_relative_threshold_filter\",\
             \"heightmap\":\"OCEAN_FLOOR_WG\",\"max_inclusive\":-13},\
             {\"type\":\"minecraft:biome\"}]}",
        )
        .unwrap();
        let def = parse_placed_feature(&j).unwrap();
        let [.., srtf, last] = def.placement.as_slice() else {
            panic!("expected 5 mods");
        };
        assert!(matches!(last, PlacementMod::BiomeFilter));
        let PlacementMod::SurfaceRelativeThresholdFilter {
            heightmap,
            min_inclusive,
            max_inclusive,
        } = srtf
        else {
            panic!("expected srtf, got {srtf:?}");
        };
        assert_eq!(heightmap, "OCEAN_FLOOR_WG");
        assert_eq!(*min_inclusive, i32::MIN); // codec default
        assert_eq!(*max_inclusive, -13);
    }

    #[test]
    fn surface_relative_threshold_underwater_magma_chain() {
        // placed_feature/underwater_magma.json — uniform count 44..52 +
        // srtf OCEAN_FLOOR_WG max_inclusive -2 (no min).
        let j = crate::json::parse(
            "{\"feature\":\"minecraft:underwater_magma\",\"placement\":[\
             {\"type\":\"minecraft:count\",\"count\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":52,\"min_inclusive\":44}},\
             {\"type\":\"minecraft:in_square\"},\
             {\"type\":\"minecraft:height_range\",\"height\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":{\"absolute\":256},\"min_inclusive\":{\"above_bottom\":0}}},\
             {\"type\":\"minecraft:surface_relative_threshold_filter\",\
             \"heightmap\":\"OCEAN_FLOOR_WG\",\"max_inclusive\":-2},\
             {\"type\":\"minecraft:biome\"}]}",
        )
        .unwrap();
        let def = parse_placed_feature(&j).unwrap();
        assert_eq!(def.placement.len(), 5);
        assert!(def
            .placement
            .iter()
            .all(|m| !matches!(m, PlacementMod::Unsupported(_))));
    }

    #[test]
    fn environment_scan_lake_lava_verbatim() {
        // placed_feature/lake_lava_underground.json — down/32, target =
        // all_of[not matching_blocks air, inside_world_bounds offset
        // [0,-5,0]], NO allowed_search_condition (=> alwaysTrue).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:environment_scan\",\"direction_of_search\":\"down\",\
             \"max_steps\":32,\"target_condition\":{\"type\":\"minecraft:all_of\",\
             \"predicates\":[{\"type\":\"minecraft:not\",\"predicate\":\
             {\"type\":\"minecraft:matching_blocks\",\"blocks\":\"minecraft:air\"}},\
             {\"type\":\"minecraft:inside_world_bounds\",\"offset\":[0,-5,0]}]}}",
        )
        .unwrap();
        let PlacementMod::EnvironmentScan {
            direction,
            target,
            allowed,
            max_steps,
        } = parse_placement_mod(&j).unwrap()
        else {
            panic!("expected EnvironmentScan");
        };
        assert_eq!(direction, ScanDir::Down);
        assert_eq!(max_steps, 32);
        assert!(matches!(allowed, BlockPredicate::True));
        let BlockPredicate::AllOf(v) = &*target else {
            panic!("expected all_of target");
        };
        assert_eq!(v.len(), 2);
        assert!(
            matches!(&v[1], BlockPredicate::InsideWorldBounds { offset } if *offset == [0, -5, 0])
        );
    }

    #[test]
    fn environment_scan_cave_vines_verbatim() {
        // placed_feature/cave_vines.json — up/12, allowed = matching_blocks
        // air, target = has_sturdy_face direction down.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:environment_scan\",\
             \"allowed_search_condition\":{\"type\":\"minecraft:matching_blocks\",\
             \"blocks\":\"minecraft:air\"},\"direction_of_search\":\"up\",\
             \"max_steps\":12,\"target_condition\":{\"type\":\"minecraft:has_sturdy_face\",\
             \"direction\":\"down\"}}",
        )
        .unwrap();
        let PlacementMod::EnvironmentScan {
            direction,
            target,
            allowed,
            max_steps,
        } = parse_placement_mod(&j).unwrap()
        else {
            panic!("expected EnvironmentScan");
        };
        assert_eq!(direction, ScanDir::Up);
        assert_eq!(max_steps, 12);
        assert!(
            matches!(&allowed, BlockPredicate::MatchingBlocks { blocks, .. } if blocks == &vec!["minecraft:air".to_string()])
        );
        assert!(
            matches!(&*target, BlockPredicate::HasSturdyFace { direction: d, .. } if d == "down")
        );
    }

    #[test]
    fn environment_scan_codec_bounds_honest_reject() {
        // intRange(1,32) and VERTICAL_CODEC are codec errors in Java — the
        // parser mirrors them (honest reject, no silent clamp).
        let base = |dir: &str, steps: i64| {
            crate::json::parse(&format!(
                "{{\"type\":\"minecraft:environment_scan\",\"direction_of_search\":\"{dir}\",\
                 \"max_steps\":{steps},\"target_condition\":{{\"type\":\"minecraft:true\"}}}}"
            ))
            .unwrap()
        };
        assert!(parse_placement_mod(&base("up", 0)).is_err());
        assert!(parse_placement_mod(&base("up", 33)).is_err());
        assert!(parse_placement_mod(&base("north", 8)).is_err());
        assert!(parse_placement_mod(&base("down", 1)).is_ok());
        assert!(parse_placement_mod(&base("up", 32)).is_ok());
    }

    #[test]
    fn sturdy_face_inside_bounds_eval() {
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        // has_sturdy_face down at the probe position: stone => sturdy,
        // water => not, unreadable => false (honest).
        let sf = parse_predicate(
            &crate::json::parse("{\"type\":\"minecraft:has_sturdy_face\",\"direction\":\"down\"}")
                .unwrap(),
        )
        .unwrap();
        let stone = |_: i32, _: i32, _: i32| -> Option<String> { Some("minecraft:stone".into()) };
        let water =
            |_: i32, _: i32, _: i32| -> Option<String> { Some("minecraft:water[level=0]".into()) };
        let none = |_: i32, _: i32, _: i32| -> Option<String> { None };
        assert!(eval_predicate(&sf, 0, 64, 0, &stone, &repl, &tag_of));
        assert!(!eval_predicate(&sf, 0, 64, 0, &water, &repl, &tag_of));
        assert!(!eval_predicate(&sf, 0, 64, 0, &none, &repl, &tag_of));
        // inside_world_bounds offset [0,-5,0]: tests (0,59,0) — readable =>
        // inside build height; the offset position unreadable => false.
        let iwb = parse_predicate(
            &crate::json::parse("{\"type\":\"minecraft:inside_world_bounds\",\"offset\":[0,-5,0]}")
                .unwrap(),
        )
        .unwrap();
        let at = |_: i32, y: i32, _: i32| -> Option<String> {
            if y >= 0 {
                Some("minecraft:stone".into())
            } else {
                None // below the build height => unreadable
            }
        };
        assert!(eval_predicate(&iwb, 0, 64, 0, &at, &repl, &tag_of)); // 59 readable
        assert!(eval_predicate(&iwb, 0, 5, 0, &at, &repl, &tag_of)); // 0 readable
        assert!(!eval_predicate(&iwb, 0, 4, 0, &at, &repl, &tag_of)); // -1 unreadable
        assert!(!eval_predicate(&iwb, 0, 64, 0, &none, &repl, &tag_of));
    }

    #[test]
    fn monster_room_verbatim_none_config() {
        // configured_feature/monster_room.json — NoneFeatureConfiguration
        // ({}); the placed chain (count 10 / in_square / height_range
        // below_top 0 .. absolute 0 / biome) must parse fully too.
        let j = crate::json::parse("{\"type\":\"minecraft:monster_room\",\"config\":{}}").unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        assert!(matches!(
            parse_configured_def(&j, short).unwrap(),
            FeatureDef::MonsterRoom
        ));
        let pj = crate::json::parse(
            "{\"feature\":\"minecraft:monster_room\",\"placement\":[\
             {\"type\":\"minecraft:count\",\"count\":10},{\"type\":\"minecraft:in_square\"},\
             {\"type\":\"minecraft:height_range\",\"height\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":{\"below_top\":0},\"min_inclusive\":{\"absolute\":0}}},\
             {\"type\":\"minecraft:biome\"}]}",
        )
        .unwrap();
        let def = parse_placed_feature(&pj).unwrap();
        assert!(def
            .placement
            .iter()
            .all(|m| !matches!(m, PlacementMod::Unsupported(_))));
    }

    #[test]
    fn underwater_magma_config_verbatim_and_ranges() {
        // configured_feature/underwater_magma.json — floor_search_range 5,
        // placement_probability_per_valid_position 0.5, radius 1.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:underwater_magma\",\"config\":\
             {\"floor_search_range\":5,\"placement_probability_per_valid_position\":0.5,\
             \"placement_radius_around_floor\":1}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::UnderwaterMagma {
            floor_search_range,
            placement_radius_around_floor,
            placement_probability_per_valid_position,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected UnderwaterMagma");
        };
        assert_eq!(floor_search_range, 5);
        assert_eq!(placement_radius_around_floor, 1);
        assert_eq!(placement_probability_per_valid_position, 0.5);
        // codec ranges are honest codec errors (intRange/floatRange).
        let mk = |fsr: i64, radius: i64, prob: f64| {
            crate::json::parse(&format!(
                "{{\"type\":\"minecraft:underwater_magma\",\"config\":\
                 {{\"floor_search_range\":{fsr},\
                 \"placement_probability_per_valid_position\":{prob},\
                 \"placement_radius_around_floor\":{radius}}}}}"
            ))
            .unwrap()
        };
        let short = "underwater_magma";
        assert!(parse_configured_def(&mk(513, 1, 0.5), short).is_err());
        assert!(parse_configured_def(&mk(5, 65, 0.5), short).is_err());
        assert!(parse_configured_def(&mk(5, 1, 1.5), short).is_err());
        assert!(parse_configured_def(&mk(0, 64, 1.0), short).is_ok());
        // missing field = codec error too (required, no defaults).
        let miss = crate::json::parse(
            "{\"type\":\"minecraft:underwater_magma\",\"config\":\
             {\"floor_search_range\":5,\"placement_radius_around_floor\":1}}",
        )
        .unwrap();
        assert!(parse_configured_def(&miss, short).is_err());
    }

    #[test]
    fn multiface_growth_glow_lichen_verbatim() {
        // configured_feature/glow_lichen.json — explicit block, 8-entry
        // can_be_placed_on id array, ceiling+wall true / floor false,
        // chance 0.5, search 20.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:multiface_growth\",\"config\":\
             {\"block\":\"minecraft:glow_lichen\",\"can_be_placed_on\":\
             [\"minecraft:stone\",\"minecraft:andesite\",\"minecraft:diorite\",\
             \"minecraft:granite\",\"minecraft:dripstone_block\",\"minecraft:calcite\",\
             \"minecraft:tuff\",\"minecraft:deepslate\"],\
             \"can_place_on_ceiling\":true,\"can_place_on_floor\":false,\
             \"can_place_on_wall\":true,\"chance_of_spreading\":0.5,\"search_range\":20}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::MultifaceGrowth {
            block,
            search_range,
            can_place_on_floor,
            can_place_on_ceiling,
            can_place_on_wall,
            chance_of_spreading,
            can_be_placed_on,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected MultifaceGrowth");
        };
        assert_eq!(block, "minecraft:glow_lichen");
        assert_eq!(search_range, 20);
        assert!(!can_place_on_floor);
        assert!(can_place_on_ceiling);
        assert!(can_place_on_wall);
        assert_eq!(chance_of_spreading, 0.5);
        assert_eq!(can_be_placed_on.len(), 8);
    }

    #[test]
    fn multiface_growth_sculk_vein_and_codec_defaults() {
        // configured_feature/sculk_vein.json — chance 1.0, all flags true.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:multiface_growth\",\"config\":\
             {\"block\":\"minecraft:sculk_vein\",\"can_be_placed_on\":\
             [\"minecraft:stone\"],\"can_place_on_ceiling\":true,\
             \"can_place_on_floor\":true,\"can_place_on_wall\":true,\
             \"chance_of_spreading\":1.0,\"search_range\":20}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::MultifaceGrowth {
            block,
            chance_of_spreading,
            ..
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected MultifaceGrowth");
        };
        assert_eq!(block, "minecraft:sculk_vein");
        assert_eq!(chance_of_spreading, 1.0);
        // codec defaults: block orElse(glow_lichen), search_range orElse(10),
        // flags default false, chance orElse(0.5); HolderSet "#tag" form.
        let dj = crate::json::parse(
            "{\"type\":\"minecraft:multiface_growth\",\"config\":\
             {\"can_be_placed_on\":\"#minecraft:base_stone_overworld\"}}",
        )
        .unwrap();
        let FeatureDef::MultifaceGrowth {
            block,
            search_range,
            can_place_on_floor,
            can_place_on_ceiling,
            can_place_on_wall,
            chance_of_spreading,
            can_be_placed_on,
        } = parse_configured_def(&dj, short).unwrap()
        else {
            panic!("expected MultifaceGrowth");
        };
        assert_eq!(block, "minecraft:glow_lichen");
        assert_eq!(search_range, 10);
        assert!(!can_place_on_floor && !can_place_on_ceiling && !can_place_on_wall);
        assert_eq!(chance_of_spreading, 0.5);
        assert_eq!(can_be_placed_on, vec!["#minecraft:base_stone_overworld"]);
    }

    #[test]
    fn geode_amethyst_verbatim() {
        // configured_feature/amethyst_geode.json — the ONLY corpus geode
        // shape; every scalar verbatim from the extract.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:geode\",\"config\":\
             {\"blocks\":{\"alternate_inner_layer_provider\":\
             {\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:budding_amethyst\"}},\
             \"cannot_replace\":\"#minecraft:features_cannot_replace\",\
             \"filling_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:air\"}},\
             \"inner_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:amethyst_block\"}},\
             \"inner_placements\":[\
             {\"Name\":\"minecraft:small_amethyst_bud\",\"Properties\":\
             {\"facing\":\"up\",\"waterlogged\":\"false\"}},\
             {\"Name\":\"minecraft:medium_amethyst_bud\",\"Properties\":\
             {\"facing\":\"up\",\"waterlogged\":\"false\"}},\
             {\"Name\":\"minecraft:large_amethyst_bud\",\"Properties\":\
             {\"facing\":\"up\",\"waterlogged\":\"false\"}},\
             {\"Name\":\"minecraft:amethyst_cluster\",\"Properties\":\
             {\"facing\":\"up\",\"waterlogged\":\"false\"}}],\
             \"invalid_blocks\":\"#minecraft:geode_invalid_blocks\",\
             \"middle_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:calcite\"}},\
             \"outer_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:smooth_basalt\"}}},\
             \"crack\":{\"base_crack_size\":2.0,\"crack_point_offset\":2,\
             \"generate_crack_chance\":0.95},\
             \"distribution_points\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":4,\"min_inclusive\":3},\
             \"invalid_blocks_threshold\":1,\
             \"layers\":{\"filling\":1.7,\"inner_layer\":2.2,\
             \"middle_layer\":3.2,\"outer_layer\":4.2},\
             \"max_gen_offset\":16,\"min_gen_offset\":-16,\
             \"noise_multiplier\":0.05,\
             \"outer_wall_distance\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":6,\"min_inclusive\":4},\
             \"placements_require_layer0_alternate\":true,\
             \"point_offset\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":2,\"min_inclusive\":1},\
             \"use_alternate_layer0_chance\":0.083,\
             \"use_potential_placements_chance\":0.35}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Geode {
            filling_provider,
            inner_layer_provider,
            alternate_inner_layer_provider,
            middle_layer_provider,
            outer_layer_provider,
            inner_placements,
            cannot_replace,
            invalid_blocks,
            filling,
            inner_layer,
            middle_layer,
            outer_layer,
            generate_crack_chance,
            base_crack_size,
            crack_point_offset,
            use_potential_placements_chance,
            use_alternate_layer0_chance,
            placements_require_layer0_alternate,
            outer_wall_distance,
            distribution_points,
            point_offset,
            min_gen_offset,
            max_gen_offset,
            noise_multiplier,
            invalid_blocks_threshold,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Geode");
        };
        assert_eq!(filling_provider, "minecraft:air");
        assert_eq!(inner_layer_provider, "minecraft:amethyst_block");
        assert_eq!(alternate_inner_layer_provider, "minecraft:budding_amethyst");
        assert_eq!(middle_layer_provider, "minecraft:calcite");
        assert_eq!(outer_layer_provider, "minecraft:smooth_basalt");
        assert_eq!(inner_placements.len(), 4);
        // BlockState codec: Properties sorted by key (facing < waterlogged).
        assert_eq!(
            inner_placements[0],
            "minecraft:small_amethyst_bud[facing=up,waterlogged=false]"
        );
        assert_eq!(
            inner_placements[3],
            "minecraft:amethyst_cluster[facing=up,waterlogged=false]"
        );
        assert_eq!(cannot_replace, "#minecraft:features_cannot_replace");
        assert_eq!(invalid_blocks, "#minecraft:geode_invalid_blocks");
        assert_eq!(filling, 1.7);
        assert_eq!(inner_layer, 2.2);
        assert_eq!(middle_layer, 3.2);
        assert_eq!(outer_layer, 4.2);
        assert_eq!(generate_crack_chance, 0.95);
        assert_eq!(base_crack_size, 2.0);
        assert_eq!(crack_point_offset, 2);
        assert_eq!(use_potential_placements_chance, 0.35);
        assert_eq!(use_alternate_layer0_chance, 0.083);
        assert!(placements_require_layer0_alternate);
        assert_eq!(outer_wall_distance, IntProvider::Uniform(4, 6));
        assert_eq!(distribution_points, IntProvider::Uniform(3, 4));
        assert_eq!(point_offset, IntProvider::Uniform(1, 2));
        assert_eq!(min_gen_offset, -16);
        assert_eq!(max_gen_offset, 16);
        assert_eq!(noise_multiplier, 0.05);
        assert_eq!(invalid_blocks_threshold, 1);
    }

    #[test]
    fn geode_codec_defaults() {
        // orElse defaults from the CFR codec: layers 1.7/2.2/3.2/4.2, crack
        // 1.0/2.0/2, chances 0.35/0.0, placements_require... true, providers
        // uniform(4,5)/uniform(3,4)/uniform(1,2), offsets -16/16, noise 0.05.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:geode\",\"config\":\
             {\"blocks\":{\"alternate_inner_layer_provider\":\
             {\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:budding_amethyst\"}},\
             \"cannot_replace\":\"#minecraft:features_cannot_replace\",\
             \"filling_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:air\"}},\
             \"inner_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:amethyst_block\"}},\
             \"inner_placements\":[{\"Name\":\"minecraft:amethyst_cluster\"}],\
             \"invalid_blocks\":\"#minecraft:geode_invalid_blocks\",\
             \"middle_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:calcite\"}},\
             \"outer_layer_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:smooth_basalt\"}}},\
             \"crack\":{},\"layers\":{},\"invalid_blocks_threshold\":1}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Geode {
            filling,
            inner_layer,
            middle_layer,
            outer_layer,
            generate_crack_chance,
            base_crack_size,
            crack_point_offset,
            use_potential_placements_chance,
            use_alternate_layer0_chance,
            placements_require_layer0_alternate,
            outer_wall_distance,
            distribution_points,
            point_offset,
            min_gen_offset,
            max_gen_offset,
            noise_multiplier,
            ..
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Geode");
        };
        assert_eq!(
            (filling, inner_layer, middle_layer, outer_layer),
            (1.7, 2.2, 3.2, 4.2)
        );
        assert_eq!(
            (generate_crack_chance, base_crack_size, crack_point_offset),
            (1.0, 2.0, 2)
        );
        assert_eq!(use_potential_placements_chance, 0.35);
        assert_eq!(use_alternate_layer0_chance, 0.0);
        assert!(placements_require_layer0_alternate);
        assert_eq!(outer_wall_distance, IntProvider::Uniform(4, 5));
        assert_eq!(distribution_points, IntProvider::Uniform(3, 4));
        assert_eq!(point_offset, IntProvider::Uniform(1, 2));
        assert_eq!(min_gen_offset, -16);
        assert_eq!(max_gen_offset, 16);
        assert_eq!(noise_multiplier, 0.05);
    }

    #[test]
    fn geode_codec_bounds_honest_reject() {
        // Required fields missing = codec error; every codec range is an
        // honest reject (no clamp); boundary values pass. The %X% slot
        // carries per-case top-level scalars, %F% the layer filling.
        const BASE: &str = concat!(
            "{\"type\":\"minecraft:geode\",\"config\":{\"blocks\":",
            "{\"alternate_inner_layer_provider\":{\"type\":\"minecraft:",
            "simple_state_provider\",\"state\":{\"Name\":\"minecraft:",
            "budding_amethyst\"}},\"cannot_replace\":\"#minecraft:features",
            "_cannot_replace\",\"filling_provider\":{\"type\":\"minecraft:",
            "simple_state_provider\",\"state\":{\"Name\":\"minecraft:air\"}},",
            "\"inner_layer_provider\":{\"type\":\"minecraft:simple_state",
            "_provider\",\"state\":{\"Name\":\"minecraft:amethyst_block\"}},",
            "\"inner_placements\":[{\"Name\":\"minecraft:amethyst_cluster\"}],",
            "\"invalid_blocks\":\"#minecraft:geode_invalid_blocks\",",
            "\"middle_layer_provider\":{\"type\":\"minecraft:simple_state",
            "_provider\",\"state\":{\"Name\":\"minecraft:calcite\"}},",
            "\"outer_layer_provider\":{\"type\":\"minecraft:simple_state",
            "_provider\",\"state\":{\"Name\":\"minecraft:smooth_basalt\"}}},",
            "\"crack\":{},\"layers\":{\"filling\":%F%,\"inner_layer\":2.2,",
            "\"middle_layer\":3.2,\"outer_layer\":4.2},%X%,",
            "\"invalid_blocks_threshold\":1}}"
        );
        let short = "geode";
        let mk = |x: &str, filling: &str| BASE.replace("%F%", filling).replace("%X%", x);
        let parse = |x: &str, filling: &str| crate::json::parse(&mk(x, filling)).unwrap();
        // boundary values inside every range: OK.
        let ok = parse(
            "\"use_potential_placements_chance\":0.0,\
             \"use_alternate_layer0_chance\":1.0,\"noise_multiplier\":1.0,\
             \"outer_wall_distance\":{\"type\":\"minecraft:uniform\",\
             \"min_inclusive\":1,\"max_inclusive\":20},\
             \"point_offset\":{\"type\":\"minecraft:uniform\",\
             \"min_inclusive\":0,\"max_inclusive\":10},\
             \"crack\":{\"base_crack_size\":5.0,\"crack_point_offset\":0,\
             \"generate_crack_chance\":0.0}",
            "50.0",
        );
        assert!(parse_configured_def(&ok, short).is_ok());
        // layer LAYER_RANGE 0.01..=50: both out-of-range ends rejected.
        for bad_filling in ["50.01", "0.009"] {
            let j = parse(
                "\"use_potential_placements_chance\":0.35,\
                 \"use_alternate_layer0_chance\":0.0",
                bad_filling,
            );
            assert!(
                parse_configured_def(&j, short).is_err(),
                "filling {bad_filling}"
            );
        }
        // GeodeCrackSettings ranges: base 0..=5, offset 0..=10, chance 0..=1.
        let with_crack = |crack_obj: &str| {
            crate::json::parse(
                &mk(
                    "\"use_potential_placements_chance\":0.35,\
                     \"use_alternate_layer0_chance\":0.0",
                    "1.7",
                )
                .replace("\"crack\":{}", crack_obj),
            )
            .unwrap()
        };
        for bad in [
            "\"crack\":{\"base_crack_size\":5.5}",
            "\"crack\":{\"crack_point_offset\":11}",
            "\"crack\":{\"generate_crack_chance\":1.2}",
        ] {
            let j = with_crack(bad);
            assert!(parse_configured_def(&j, short).is_err(), "crack {bad}");
        }
        let bounds_ok = with_crack(
            "\"crack\":{\"base_crack_size\":0.0,\"crack_point_offset\":10,\
             \"generate_crack_chance\":1.0}",
        );
        assert!(parse_configured_def(&bounds_ok, short).is_ok());
        // CHANCE_RANGE scalars 0..=1.
        let j = parse(
            "\"use_potential_placements_chance\":1.5,\
             \"use_alternate_layer0_chance\":0.0",
            "1.7",
        );
        assert!(parse_configured_def(&j, short).is_err());
        let j = parse(
            "\"use_potential_placements_chance\":0.35,\"noise_multiplier\":-0.1,\
             \"use_alternate_layer0_chance\":0.0",
            "1.7",
        );
        assert!(parse_configured_def(&j, short).is_err());
        // IntProvider.codec bounds: outer_wall_distance 1..=20, point_offset 0..=10.
        let j = parse(
            "\"outer_wall_distance\":{\"type\":\"minecraft:uniform\",\
             \"min_inclusive\":4,\"max_inclusive\":21},\
             \"use_alternate_layer0_chance\":0.0",
            "1.7",
        );
        assert!(parse_configured_def(&j, short).is_err());
        let j = parse(
            "\"point_offset\":{\"type\":\"minecraft:uniform\",\
             \"min_inclusive\":-1,\"max_inclusive\":2},\
             \"use_alternate_layer0_chance\":0.0",
            "1.7",
        );
        assert!(parse_configured_def(&j, short).is_err());
        // nonEmptyList inner_placements: [] = codec error.
        let j = crate::json::parse(&mk("\"use_alternate_layer0_chance\":0.0", "1.7").replace(
            "\"inner_placements\":[{\"Name\":\"minecraft:amethyst_cluster\"}]",
            "\"inner_placements\":[]",
        ))
        .unwrap();
        assert!(parse_configured_def(&j, short).is_err());
        // REQUIRED scalars missing = codec error.
        let j = crate::json::parse(
            &mk("\"use_alternate_layer0_chance\":0.0", "1.7")
                .replace(",\"invalid_blocks_threshold\":1}", "}"),
        )
        .unwrap();
        assert!(parse_configured_def(&j, short).is_err());
        let j = crate::json::parse(
            "{\"type\":\"minecraft:geode\",\"config\":{\"crack\":{},\"layers\":{},\
             \"invalid_blocks_threshold\":1}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j, short).is_err());
    }

    #[test]
    fn lake_lava_verbatim() {
        // configured_feature/lake_lava.json — the ONLY corpus lake shape:
        // fluid = lava[level=0], barrier = stone (both simple_state_provider).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:lake\",\"config\":{\"barrier\":\
             {\"type\":\"minecraft:simple_state_provider\",\"state\":{\"Name\":\"minecraft:stone\"}},\
             \"fluid\":{\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:lava\",\"Properties\":{\"level\":\"0\"}}}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Lake { fluid, barrier } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Lake");
        };
        assert!(matches!(
            fluid,
            StateProvider::Simple(ref s) if s == "minecraft:lava[level=0]"
        ));
        assert!(matches!(
            barrier,
            StateProvider::Simple(ref s) if s == "minecraft:stone"
        ));
    }

    #[test]
    fn lake_codec_required_fields_and_rule_based_path() {
        // BOTH fields are fieldOf (REQUIRED): a missing key = honest codec
        // error, no orElse defaults anywhere in LakeFeature.Configuration.
        let mk = |inner: &str| {
            crate::json::parse(&format!(
                "{{\"type\":\"minecraft:lake\",\"config\":{inner}}}"
            ))
            .unwrap()
        };
        // fluid missing.
        let j = mk(
            "{\"barrier\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:stone\"}}}",
        );
        assert!(parse_configured_def(&j, "lake").is_err());
        // barrier missing.
        let j = mk(
            "{\"fluid\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:water\"}}}",
        );
        assert!(parse_configured_def(&j, "lake").is_err());
        // config itself missing.
        let j = crate::json::parse("{\"type\":\"minecraft:lake\"}").unwrap();
        assert!(parse_configured_def(&j, "lake").is_err());
        // The field codec is BlockStateProvider.CODEC: a rule_based fluid
        // parses through the FULL inc.-6 provider path (first rule wins,
        // else fallback) — the same behavior RuleBasedBlockStateProvider
        // gives the disk family.
        let j = mk(
            "{\"fluid\":{\"fallback\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:water\"}},\"rules\":[\
             {\"if_true\":{\"type\":\"minecraft:matching_blocks\",\
             \"blocks\":\"minecraft:air\"},\"then\":\
             {\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:packed_ice\"}}}]},\
             \"barrier\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:stone\"}}}",
        );
        let FeatureDef::Lake { fluid, barrier } = parse_configured_def(&j, "lake").unwrap()
        else {
            panic!("expected Lake");
        };
        let StateProvider::RuleBased { fallback, rules } = &fluid else {
            panic!("expected RuleBased fluid");
        };
        assert_eq!(rules.len(), 1);
        assert!(matches!(
            fallback.as_ref(),
            StateProvider::Simple(s) if s == "minecraft:water"
        ));
        assert!(matches!(
            rules[0].1,
            StateProvider::Simple(ref s) if s == "minecraft:packed_ice"
        ));
        assert!(matches!(
            barrier,
            StateProvider::Simple(ref s) if s == "minecraft:stone"
        ));
        // An unknown provider type stays an honest codec error.
        let j = mk(
            "{\"fluid\":{\"type\":\"minecraft:noise_threshold_provider\"},\
             \"barrier\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:stone\"}}}",
        );
        assert!(parse_configured_def(&j, "lake").is_err());
    }

    #[test]
    fn would_survive_predicate_parse_and_honest_false() {
        // verbatim from configured_feature/patch_cactus.json (the inner
        // block_predicate_filter of the random_patch): all_of(
        //   matching_blocks air,
        //   would_survive cactus[age=0] at offset ZERO).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:all_of\",\"predicates\":[\
             {\"type\":\"minecraft:matching_blocks\",\"blocks\":\"minecraft:air\"},\
             {\"type\":\"minecraft:would_survive\",\"state\":\
             {\"Name\":\"minecraft:cactus\",\"Properties\":{\"age\":\"0\"}}}]}",
        )
        .unwrap();
        let p = parse_predicate(&j).unwrap();
        let BlockPredicate::AllOf(v) = &p else {
            panic!("expected all_of");
        };
        assert_eq!(v.len(), 2);
        let BlockPredicate::WouldSurvive { offset, state } = &v[1] else {
            panic!("expected would_survive");
        };
        // offsetCodec default ZERO, state canonical string with properties.
        assert_eq!(*offset, [0, 0, 0]);
        assert_eq!(state, "minecraft:cactus[age=0]");
        // offset form (the firefly_bush neighbours use [x,-1,z] offsets).
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:would_survive\",\"offset\":[1,-1,0],\
             \"state\":{\"Name\":\"minecraft:firefly_bush\"}}",
        )
        .unwrap();
        let BlockPredicate::WouldSurvive { offset, state } = parse_predicate(&j2).unwrap()
        else {
            panic!("expected would_survive");
        };
        assert_eq!(offset, [1, -1, 0]);
        assert_eq!(state, "minecraft:firefly_bush");
        // state is fieldOf (REQUIRED): a missing key = honest codec error.
        let j3 = crate::json::parse("{\"type\":\"minecraft:would_survive\"}").unwrap();
        assert!(parse_predicate(&j3).is_err());
        // EVAL: the shape is parsed, the verdict stays honest-false (the
        // per-block canSurvive rules are a P4 tail) — even on pure air the
        // would_survive component answers false, so the all_of is false.
        let tag_of = |_: &str| -> Option<Vec<String>> { None };
        let repl = |_: &str| false;
        let at = |_: i32, _: i32, _: i32| -> Option<String> { Some("minecraft:air".into()) };
        assert!(!eval_predicate(&p, 0, 0, 0, &at, &repl, &tag_of));
    }

    #[test]
    fn random_patch_codec_defaults_and_would_survive_inner() {
        // RandomPatchConfiguration.CODEC orElse defaults (inc. 8 fix):
        // tries 128 (POSITIVE_INT), xz_spread 7, y_spread 3 (NON_NEGATIVE).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:random_patch\",\"config\":{\"feature\":\
             {\"feature\":{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:short_grass\"}}}},\
             \"placement\":[{\"type\":\"minecraft:block_predicate_filter\",\
             \"predicate\":{\"type\":\"minecraft:would_survive\",\"state\":\
             {\"Name\":\"minecraft:short_grass\"}}}]}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::RandomPatch {
            tries,
            xz_spread,
            y_spread,
            inner,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected RandomPatch");
        };
        assert_eq!((tries, xz_spread, y_spread), (128, 7, 3));
        // the inner placed feature rides through block_predicate_filter
        // with a would_survive predicate (the trees/patches family shape).
        assert_eq!(inner.placement.len(), 1);
        if let PlacementMod::BlockPredicateFilter(BlockPredicate::WouldSurvive {
            offset,
            state,
        }) = &inner.placement[0]
        {
            assert_eq!(*offset, [0, 0, 0]);
            assert_eq!(state, "minecraft:short_grass");
        } else {
            panic!("expected would_survive filter");
        }
    }

    #[test]
    fn random_selector_verbatim_ref_and_inline_forms() {
        // verbatim from configured_feature/trees_birch.json — the dominant
        // corpus form: string refs for both the weighted feature and the
        // default.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:random_selector\",\"config\":\
             {\"default\":\"minecraft:birch_bees_0002\",\
             \"features\":[{\"chance\":0.0125,\
             \"feature\":\"minecraft:fallen_birch_tree\"}]}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::RandomSelector { features, default } =
            parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected RandomSelector");
        };
        assert_eq!(features.len(), 1);
        assert_eq!(features[0].0, 0.0125);
        assert_eq!(features[0].1.feature_ref, "minecraft:fallen_birch_tree");
        assert_eq!(default.feature_ref, "minecraft:birch_bees_0002");

        // verbatim from trees_plains.json — INLINE placed objects (both the
        // default and a weighted entry): {feature: <ref>, placement: []}.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:random_selector\",\"config\":\
             {\"default\":{\"feature\":\"minecraft:oak_bees_005\",\"placement\":[]},\
             \"features\":[{\"chance\":0.33333334,\"feature\":\
             {\"feature\":\"minecraft:fancy_oak_bees_005\",\"placement\":[]}},\
             {\"chance\":0.0125,\"feature\":\"minecraft:fallen_oak_tree\"}]}}",
        )
        .unwrap();
        let FeatureDef::RandomSelector { features, default } =
            parse_configured_def(&j2, "random_selector").unwrap()
        else {
            panic!("expected RandomSelector");
        };
        assert_eq!(features.len(), 2);
        // the inline placed object {feature: <ref>, placement: []} parses
        // to a ref-carrying PlacedFeatureDef (inline: None — the INNER
        // feature is a string, not an inline configured body).
        assert_eq!(features[0].1.feature_ref, "minecraft:fancy_oak_bees_005");
        assert!(features[0].1.placement.is_empty());
        assert_eq!(features[1].1.feature_ref, "minecraft:fallen_oak_tree");
        assert_eq!(default.feature_ref, "minecraft:oak_bees_005");

        // codec honesty: default and features are fieldOf (REQUIRED);
        // chance is floatRange(0..1) — out of range = codec error.
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:random_selector\",\"config\":\
             {\"features\":[]}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j3, "random_selector").is_err());
        let j4 = crate::json::parse(
            "{\"type\":\"minecraft:random_selector\",\"config\":\
             {\"default\":\"minecraft:oak\",\"features\":\
             [{\"chance\":1.5,\"feature\":\"minecraft:oak\"}]}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j4, "random_selector").is_err());
    }

    #[test]
    fn random_boolean_selector_verbatim_both_halves_required() {
        // verbatim from configured_feature/lush_caves_clay.json — BOTH
        // halves are INLINE placed objects (feature ref + empty placement).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:random_boolean_selector\",\"config\":\
             {\"feature_false\":{\"feature\":\"minecraft:clay_pool_with_dripleaves\",\
             \"placement\":[]},\"feature_true\":{\"feature\":\
             \"minecraft:clay_with_dripleaves\",\"placement\":[]}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::RandomBooleanSelector {
            feature_true,
            feature_false,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected RandomBooleanSelector");
        };
        assert_eq!(feature_true.feature_ref, "minecraft:clay_with_dripleaves");
        assert_eq!(
            feature_false.feature_ref,
            "minecraft:clay_pool_with_dripleaves"
        );
        // codec honesty: a missing half = honest codec error (no defaults).
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:random_boolean_selector\",\"config\":\
             {\"feature_true\":{\"feature\":\"minecraft:stone\",\"placement\":[]}}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j2, "random_boolean_selector").is_err());
    }
}
