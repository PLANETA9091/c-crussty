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
//!   - IntProvider family (inc. 10): weighted_list / clamped /
//!     clamped_normal / biased_to_bottom / constant — the TRUE root of the
//!     "random_selector (parse fail) 36" top (count modifiers of every
//!     trees_* chain). WeightedListInt sample = nextInt(totalWeight) +
//!     in-order walk; BiasedToBottom = min + nextInt(nextInt(range+1)+1)
//!     (two draws); ClampedNormal = (int)clamp(mean + (float)nextGaussian()
//!     * deviation) — the MarsagliaPolar cache rides the SOURCE
//!     (IntProviderDraws::next_gaussian_wg, reset on re-seed like Java).
//!     getMinValue/getMaxValue mirrors drive the geode codec-range checks.
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

#[derive(Debug, Clone, PartialEq)]
pub enum IntProvider {
    Constant(i32),
    Uniform(i32, i32),
    /// BiasedToBottomInt (inc. 10): min + nextInt(nextInt(range+1)+1) —
    /// TWO nextInt draws (the inner one picks the "magnitude").
    BiasedToBottom(i32, i32),
    /// ClampedNormalInt (inc. 10): (int)clamp(mean + (float)nextGaussian()
    /// * deviation, min, max) — the Mth.normal f32 form; the gaussian
    /// rides the source-side MarsagliaPolar state (IntProviderDraws).
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min: i32,
        max: i32,
    },
    /// ClampedInt (inc. 10): clamp(source.sample, min, max).
    Clamped {
        source: Box<IntProvider>,
        min: i32,
        max: i32,
    },
    /// WeightedListInt (inc. 10): the 36-biome tree-chain blocker — count
    /// modifiers of every trees_* placed chain (plus tree branch_count and
    /// block_column layers/height internals). WeightedList.nonEmptyCodec(
    /// IntProvider.CODEC): (weight NON_NEGATIVE_INT, provider) entries,
    /// sample = nextInt(totalWeight) then the in-order accumulated-weight
    /// walk (Flat/Compact selectors agree on the same item).
    WeightedList(Vec<(i32, IntProvider)>),
}

impl IntProvider {
    pub fn parse(j: &Json) -> Result<IntProvider, String> {
        // int literal or a {type: ...} object — the full 1.21.10
        // IntProviderType family (inc. 10: weighted_list / clamped /
        // clamped_normal / biased_to_bottom added; there is NO triangle
        // int provider in 1.21.10 — IntProviderType has exactly constant,
        // uniform, biased_to_bottom, clamped, clamped_normal,
        // weighted_list).
        if let Some(v) = j.as_i64() {
            return Ok(IntProvider::Constant(v as i32));
        }
        let ty = j
            .get("type")
            .and_then(|t| t.as_str())
            .map(|t| t.strip_prefix("minecraft:").unwrap_or(t).to_string());
        match ty.as_deref() {
            Some("constant") => {
                // ConstantInt.CODEC = Codec.INT.fieldOf("value").
                let v = j.get("value").and_then(|v| v.as_i64()).ok_or("constant.value")?;
                Ok(IntProvider::Constant(v as i32))
            }
            Some("biased_to_bottom") => {
                let (a, b) = Self::min_max(j)?;
                if b < a {
                    return Err(format!("biased_to_bottom max < min ({a}..{b})"));
                }
                Ok(IntProvider::BiasedToBottom(a, b))
            }
            Some("clamped") => {
                // ClampedInt.CODEC: source = IntProvider.CODEC fieldOf,
                // both bounds Codec.INT fieldOf; validate max >= min.
                let source = Box::new(IntProvider::parse(j.get("source").ok_or("clamped.source")?)?);
                let min = j
                    .get("min_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("clamped.min_inclusive")? as i32;
                let max = j
                    .get("max_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("clamped.max_inclusive")? as i32;
                if max < min {
                    return Err(format!("clamped max < min ({min}..{max})"));
                }
                Ok(IntProvider::Clamped { source, min, max })
            }
            Some("clamped_normal") => {
                // ClampedNormalInt.CODEC: mean/deviation Codec.FLOAT, the
                // bounds Codec.INT; validate max >= min.
                let mean = j.get("mean").and_then(|v| v.as_f64()).ok_or("clamped_normal.mean")? as f32;
                let deviation = j
                    .get("deviation")
                    .and_then(|v| v.as_f64())
                    .ok_or("clamped_normal.deviation")? as f32;
                let min = j
                    .get("min_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("clamped_normal.min_inclusive")? as i32;
                let max = j
                    .get("max_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("clamped_normal.max_inclusive")? as i32;
                if max < min {
                    return Err(format!("clamped_normal max < min ({min}..{max})"));
                }
                Ok(IntProvider::ClampedNormal { mean, deviation, min, max })
            }
            Some("weighted_list") => {
                // WeightedList.nonEmptyCodec — an EMPTY/missing
                // distribution = codec error; entries are {data, weight}.
                let dist = j
                    .get("distribution")
                    .and_then(|d| d.as_arr())
                    .ok_or("weighted_list.distribution")?;
                if dist.is_empty() {
                    return Err("weighted_list distribution empty".into());
                }
                let mut items = Vec::with_capacity(dist.len());
                for w in dist {
                    let weight = w
                        .get("weight")
                        .and_then(|v| v.as_i64())
                        .ok_or("weighted_list.weight")?;
                    if weight < 0 {
                        return Err(format!("weighted_list weight < 0 ({weight})"));
                    }
                    let data = IntProvider::parse(w.get("data").ok_or("weighted_list.data")?)?;
                    items.push((weight as i32, data));
                }
                Ok(IntProvider::WeightedList(items))
            }
            // uniform (explicit) — and the tolerant ABSENT-type form: the
            // bounds sit at the top level; the nested {value: {min,max}}
            // shape is tolerated too (pre-inc.-10 behavior preserved).
            None | Some("uniform") => {
                let (a, b) = Self::min_max(j)?;
                if b < a {
                    return Err(format!("uniform max < min ({a}..{b})"));
                }
                Ok(IntProvider::Uniform(a, b))
            }
            other => Err(format!("unsupported int provider {other:?}")),
        }
    }

    /// The {value: {min_inclusive, max_inclusive}} nested shape first, else
    /// the top-level min_inclusive/max_inclusive fields.
    fn min_max(j: &Json) -> Result<(i32, i32), String> {
        if let Some(val) = j.get("value") {
            return Ok((
                val.get("min_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("intprovider min")? as i32,
                val.get("max_inclusive")
                    .and_then(|v| v.as_i64())
                    .ok_or("intprovider max")? as i32,
            ));
        }
        match (
            j.get("min_inclusive").and_then(|v| v.as_i64()),
            j.get("max_inclusive").and_then(|v| v.as_i64()),
        ) {
            (Some(a), Some(b)) => Ok((a as i32, b as i32)),
            _ => Err("intprovider shape".into()),
        }
    }

    /// Java getMinValue/getMaxValue (the codec range validators use these;
    /// ClampedInt narrows the source range, WeightedListInt takes the
    /// min-of-mins / max-of-maxes over the entries).
    pub fn min_value(&self) -> i32 {
        match self {
            IntProvider::Constant(v) => *v,
            IntProvider::Uniform(a, _) => *a,
            IntProvider::BiasedToBottom(a, _) => *a,
            IntProvider::ClampedNormal { min, .. } => *min,
            IntProvider::Clamped { source, min, .. } => (*min).max(source.min_value()),
            IntProvider::WeightedList(items) => items
                .iter()
                .map(|(_, p)| p.min_value())
                .min()
                .unwrap_or(0),
        }
    }

    pub fn max_value(&self) -> i32 {
        match self {
            IntProvider::Constant(v) => *v,
            IntProvider::Uniform(_, b) => *b,
            IntProvider::BiasedToBottom(_, b) => *b,
            IntProvider::ClampedNormal { max, .. } => *max,
            IntProvider::Clamped { source, max, .. } => (*max).min(source.max_value()),
            IntProvider::WeightedList(items) => items
                .iter()
                .map(|(_, p)| p.max_value())
                .max()
                .unwrap_or(0),
        }
    }

    #[inline]
    pub fn sample(&self, rng: &mut dyn crate::feature_sorter::IntProviderDraws) -> i32 {
        match self {
            IntProvider::Constant(v) => *v,
            IntProvider::Uniform(a, b) => rng.next_int_bound_wg(b - a + 1) + a,
            // BiasedToBottomInt.sample: min + nextInt(nextInt(range+1)+1).
            IntProvider::BiasedToBottom(a, b) => {
                let inner = rng.next_int_bound_wg(b - a + 1) + 1;
                a + rng.next_int_bound_wg(inner)
            }
            // Mth.normal = mean + (float)nextGaussian() * deviation (the
            // gaussian is cast to f32 FIRST), then clamp in f32 and the
            // Java (int) cast = trunc toward zero.
            IntProvider::ClampedNormal { mean, deviation, min, max } => {
                let normal = *mean + rng.next_gaussian_wg() as f32 * deviation;
                let c = normal.clamp(*min as f32, *max as f32);
                c as i32
            }
            IntProvider::Clamped { source, min, max } => source.sample(rng).clamp(*min, *max),
            // WeightedRandom: nextInt(totalWeight), then the in-order
            // accumulated-weight walk (Flat and Compact selectors return
            // the same item for the same roll).
            IntProvider::WeightedList(items) => {
                let total: i64 = items.iter().map(|(w, _)| *w as i64).sum();
                let mut r = rng.next_int_bound_wg(total as i32) as i64;
                for (w, p) in items {
                    r -= *w as i64;
                    if r < 0 {
                        return p.sample(rng);
                    }
                }
                0 // unreachable for validated non-negative weights (total 0 = broken shape)
            }
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
    /// noise_threshold_count (flower_cherry, flower_plains, patch_grass_meadow,
    /// patch_grass, patch_tall_grass_2, wildflowers_meadow; inc. 13) —
    /// RepeatingPlacement: count() = Biome.BIOME_INFO_NOISE (the TEMPERATURE
    /// noise parameter) sampled at x/200.0, z/200.0; value < noiseLevel ->
    /// belowNoise, else aboveNoise (IntStream.range(0, count) downstream —
    /// NO rng draws). Codec (CFR): noise_level DOUBLE fieldOf +
    /// below_noise/above_noise PLAIN Codec.INT fieldOf — NO intRange
    /// validation in 1.21.10 (any int is codec-legal; only MISSING keys are
    /// codec errors).
    NoiseThresholdCount {
        noise_level: f64,
        below_noise: i32,
        above_noise: i32,
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
        "noise_threshold_count" => {
            // CFR NoiseThresholdCountPlacement.CODEC: three fieldOf REQUIRED
            // fields; below/above via PLAIN Codec.INT (no intRange — an
            // out-of-range value is codec-legal, only a missing key errs).
            PlacementMod::NoiseThresholdCount {
                noise_level: j
                    .get("noise_level")
                    .and_then(|v| v.as_f64())
                    .ok_or("noise_threshold_count.noise_level")?,
                below_noise: j
                    .get("below_noise")
                    .and_then(|v| v.as_i64())
                    .ok_or("noise_threshold_count.below_noise")? as i32,
                above_noise: j
                    .get("above_noise")
                    .and_then(|v| v.as_i64())
                    .ok_or("noise_threshold_count.above_noise")? as i32,
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
    /// INLINE configured feature body (random_patch inner shape): the FULL
    /// {type, config} object — resolved without a registry lookup.
    pub inline: Option<(String, Json)>,
}

pub fn parse_placed_feature(j: &Json) -> Result<PlacedFeatureDef, String> {
    // feature ref: registry string OR inline configured object {type, config}
    let (feature_ref, inline) = match j.get("feature") {
        Some(Json::Str(s)) => (expand_rl(s), None),
        Some(f) if f.get("type").is_some() => {
            // inline configured feature: park the WHOLE body {type, config}
            // (inc. 13: the strict selector-element verdict re-parses it
            // through the same dispatch as named features).
            let ty = f
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
            let short = ty.strip_prefix("minecraft:").unwrap_or(&ty).to_string();
            (String::new(), Some((short, f.clone())))
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
    /// SimpleRandomFeatureConfiguration ("simple_random_selector", inc. 13):
    /// features = ExtraCodecs.nonEmptyHolderSet(PlacedFeature.LIST_CODEC)
    /// fieldOf REQUIRED — a LIST of placed features (each = registry string
    /// ref OR inline {feature, placement} object); a single ref/object is
    /// also a legal HolderSet shape; an EMPTY list = codec error. Execution
    /// tail: ONE nextInt(features.size()) uniform draw over the set IN
    /// ORDER, then the chosen placed feature places with the SAME random
    /// source and origin (draw-exact port at the stagediff features-status
    /// step). Verdict honesty (inc. 13, uniform since inc. 14): every
    /// element is verdicted STRICTLY — an inline element body goes through
    /// the SAME parse_configured_def dispatch as named features; inc. 14
    /// retired the inc.8/9 inline trust-hole entirely, so this is no longer
    /// a selector-only exception but THE one verdict path. A "#tag"
    /// holder-set is an honest unsupported (zero corpus instances).
    SimpleRandomSelector {
        features: Vec<Box<PlacedFeatureDef>>,
    },
    /// BlockColumnConfiguration ("block_column", inc. 15): CODEC field
    /// order layers -> direction -> allowed_placement -> prioritize_tip,
    /// ALL fieldOf REQUIRED; layer codec order height -> provider, height =
    /// IntProvider.NON_NEGATIVE_CODEC (codec(0, i32::MAX) — the range
    /// mirror owns the honest Err), provider = BlockStateProvider.CODEC
    /// (FULL family path: simple/weighted/rule_based/randomized_int; the
    /// noise families stay honest Err via parse_state_provider); direction
    /// = Direction.CODEC (the 6 canonical names, honest Err otherwise);
    /// allowed_placement = BlockPredicate.CODEC; prioritize_tip =
    /// Codec.BOOL (missing key = honest codec error). Execution tail
    /// (documented like Disk/Spring, implemented at the stagediff
    /// features-status step): heights sampled IN LAYER ORDER (one
    /// height.sample per layer — WeightedList entries ride the
    /// inc. 10 semantics), total==0 -> false with NO placement; then
    /// allowed_placement walks the direction from origin+1 up to total,
    /// the FIRST failure truncates the height array (prioritize_tip ?
    /// tip layers kept first : base layers kept first — truncate()) and
    /// breaks; then per layer in order, per block:
    /// setBlock(provider.getState(random, pos)) moving along the
    /// direction; returns true.
    BlockColumn {
        layers: Vec<(IntProvider, StateProvider)>,
        direction: String,
        allowed_placement: BlockPredicate,
        prioritize_tip: bool,
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
    /// FallenTreeFeature (inc. 11) — "minecraft:fallen_tree" (1.21.5+;
    /// the 5 corpus shapes are fallen_{birch,spruce,oak,super_birch,jungle}
    /// _tree). FallenTreeConfiguration codec: ALL FOUR fields fieldOf
    /// (REQUIRED, no orElse defaults): trunk_provider = FULL
    /// BlockStateProvider path (weighted_state_provider added this inc.),
    /// log_length = IntProvider.codec(0, 16) — the min/max mirror must sit
    /// inside the bound (geode-closure semantics; out-of-range = honest
    /// codec error), stump_decorators + log_decorators =
    /// TreeDecorator.CODEC.listOf() (REQUIRED, MAY be empty).
    /// place() ALWAYS returns true. Execution tail (documented honestly,
    /// like MonsterRoom/Lake): stump at origin (identity modifier, one
    /// trunk_provider draw), direction = Util.getRandom(HORIZONTAL.faces)
    /// = ONE nextInt(4); i = log_length.sample(random) - 2; start pos =
    /// origin.relative(dir, 2 + nextInt(2)) then UP once with up to 6
    /// mayPlaceOn probes downward (validTreePos = air ||
    /// REPLACEABLE_BY_TREES + isFaceSturdy below — NO rng); the
    /// canPlaceEntireFallenLog walk (validTreePos + solid-gap tolerance 2)
    /// draws NOTHING; placeFallenLog sets i log blocks with axis =
    /// dir.getAxis() then runs decorators — attached_to_logs iterates log
    /// positions in Util.shuffledCopy order (Fisher-Yates, len-1 nextInt
    /// draws), per position ONE Util.getRandom direction draw + ONE
    /// nextFloat() <= probability gate with the provider draw only AFTER
    /// the gate; trunk_vine = 4 x nextInt(3) per log pos (W/E/N/S).
    FallenTree {
        trunk_provider: StateProvider,
        log_length: IntProvider,
        stump_decorators: Vec<TreeDecorator>,
        log_decorators: Vec<TreeDecorator>,
    },
    /// TreeFeature (inc. 12, step 2 of the tree family) — "minecraft:tree".
    /// TreeConfiguration.CODEC (1.21.10) mirror, fields in codec order:
    /// trunk_provider (BlockStateProvider — FULL provider path),
    /// trunk_placer (TrunkPlacer dispatch), foliage_provider
    /// (BlockStateProvider), foliage_placer (FoliagePlacer dispatch),
    /// root_placer (optionalFieldOf — PRESENT = honest Err, the corpus
    /// root shapes are mangrove-only), dirt_provider (BlockStateProvider),
    /// minimum_size (FeatureSize dispatch), decorators
    /// (TreeDecorator.CODEC.listOf() REQUIRED — may be empty),
    /// ignore_vines BOOL orElse(false), force_dirt BOOL orElse(false).
    /// Verdict subset this increment: straight_trunk_placer +
    /// blob_foliage_placer + two_layers_feature_size + the decorators
    /// beehive/cocoa/leave_vine/place_on_ground/trunk_vine/
    /// attached_to_logs — that flips the 16 straight+blob corpus configs
    /// (birch/oak/super_birch/jungle/swamp_oak families); every other
    /// placer/feature-size/decorator type is an honest Err (the 23 other
    /// tree configs keep verdict false with the precise reason label).
    /// Execution tail (documented honestly, like FallenTree — the
    /// draw-exact port is the stagediff features-status gate): TreeFeature
    ///.place sets dirt below (dirt_provider), trunk placer places logs
    /// upward (height = base + nextInt(rand_a+1) + nextInt(rand_b+1)
    /// draws, doPlace pruning with the placer's valid-chunks checks),
    /// foliage placer walks its layers (per-layer radius/offset draws,
    /// provider draw per block), then decorators run in LIST order with
    /// per-position gates (see each TreeDecorator variant's tail).
    Tree {
        trunk_provider: StateProvider,
        trunk_placer: TrunkPlacer,
        foliage_provider: StateProvider,
        foliage_placer: FoliagePlacer,
        dirt_provider: StateProvider,
        minimum_size: FeatureSize,
        decorators: Vec<TreeDecorator>,
        ignore_vines: bool,
        force_dirt: bool,
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
        "simple_random_selector" => {
            let cfg = j.get("config").ok_or("simple_random_selector.config")?;
            let fj = cfg
                .get("features")
                .ok_or("simple_random_selector.features")?;
            // "#tag" holder-set: honest unsupported (corpus carries none).
            if fj.as_str().is_some() && fj.as_str().unwrap_or("").starts_with('#') {
                return Err(format!(
                    "simple_random_selector tag holder-set unsupported: {}",
                    fj.as_str().unwrap_or("")
                ));
            }
            // HolderSet shapes: a LIST (the corpus form), a single ref
            // string, or a single inline object. nonEmptyHolderSet: an
            // empty list = codec error.
            let items: Vec<&Json> = match fj.as_arr() {
                Some(a) => {
                    if a.is_empty() {
                        return Err("simple_random_selector features non-empty".into());
                    }
                    a.iter().collect()
                }
                None => vec![fj],
            };
            let mut features = Vec::new();
            for it in items {
                // PlacedFeature.CODEC dual form (same shapes the
                // random_selector weighted entries use).
                let f = if let Some(s) = it.as_str() {
                    PlacedFeatureDef {
                        feature_ref: expand_rl(s),
                        placement: Vec::new(),
                        inline: None,
                    }
                } else {
                    parse_placed_feature(it)?
                };
                features.push(Box::new(f));
            }
            FeatureDef::SimpleRandomSelector { features }
        }
        "block_column" => {
            let cfg = j.get("config").ok_or("block_column.config")?;
            // codec field order: layers -> direction -> allowed_placement
            // -> prioritize_tip (the first failing field owns the honest
            // Err label).
            let layers_j = cfg
                .get("layers")
                .and_then(|l| l.as_arr())
                .ok_or("block_column.layers")?;
            let mut layers = Vec::new();
            for l in layers_j {
                // Layer codec order: height -> provider.
                let height = int_provider_in_range(l, "height", 0, i32::MAX)?;
                let provider = parse_state_provider(l.get("provider").ok_or("layer.provider")?)?;
                layers.push((height, provider));
            }
            let dir = cfg
                .get("direction")
                .and_then(|d| d.as_str())
                .ok_or("block_column.direction")?;
            let direction = dir.strip_prefix("minecraft:").unwrap_or(dir).to_string();
            if !matches!(
                direction.as_str(),
                "up" | "down" | "north" | "south" | "west" | "east"
            ) {
                return Err(format!("unsupported direction {direction}"));
            }
            let allowed_placement = parse_predicate(
                cfg.get("allowed_placement")
                    .ok_or("block_column.allowed_placement")?,
            )?;
            let prioritize_tip = match cfg.get("prioritize_tip") {
                Some(Json::Bool(b)) => *b,
                _ => return Err("block_column.prioritize_tip".into()),
            };
            FeatureDef::BlockColumn {
                layers,
                direction,
                allowed_placement,
                prioritize_tip,
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
                    // IntProvider.codec(lo,hi): the provider's min/max
                    // range must sit inside the bound (Java semantics via
                    // the getMinValue/getMaxValue mirrors).
                    let (a, b) = (parsed.min_value(), parsed.max_value());
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
        "fallen_tree" => {
            let cfg = j.get("config").ok_or("fallen_tree.config")?;
            // ALL FOUR fields fieldOf (REQUIRED) — a missing key is an
            // honest codec error, no orElse defaults anywhere.
            let trunk_provider =
                parse_state_provider(cfg.get("trunk_provider").ok_or("fallen_tree.trunk_provider")?)?;
            let log_length =
                IntProvider::parse(cfg.get("log_length").ok_or("fallen_tree.log_length")?)?;
            // IntProvider.codec(0, 16): the provider's min/max mirror must
            // sit inside the bound (same honesty as the geode closures).
            let (lo, hi) = (log_length.min_value(), log_length.max_value());
            if lo < 0 || hi > 16 {
                return Err(format!("fallen_tree log_length 0..=16, got {lo}..{hi}"));
            }
            let stump_decorators = parse_tree_decorators(cfg, "stump_decorators")?;
            let log_decorators = parse_tree_decorators(cfg, "log_decorators")?;
            FeatureDef::FallenTree {
                trunk_provider,
                log_length,
                stump_decorators,
                log_decorators,
            }
        }
        "tree" => {
            let cfg = j.get("config").ok_or("tree.config")?;
            // Fields decoded in TreeConfiguration.CODEC order — the FIRST
            // failing field owns the honest error label (DFU group
            // semantics).
            let trunk_provider = parse_state_provider(
                cfg.get("trunk_provider").ok_or("tree.trunk_provider")?,
            )?;
            let trunk_placer =
                parse_trunk_placer(cfg.get("trunk_placer").ok_or("tree.trunk_placer")?)?;
            let foliage_provider = parse_state_provider(
                cfg.get("foliage_provider").ok_or("tree.foliage_provider")?,
            )?;
            let foliage_placer =
                parse_foliage_placer(cfg.get("foliage_placer").ok_or("tree.foliage_placer")?)?;
            // optionalFieldOf("root_placer"): ABSENT = None; PRESENT with
            // an unsupported root placer shape = honest codec error (the
            // corpus root shapes are mangrove-only).
            if cfg.get("root_placer").is_some() {
                return Err("unsupported root placer".into());
            }
            let dirt_provider =
                parse_state_provider(cfg.get("dirt_provider").ok_or("tree.dirt_provider")?)?;
            let minimum_size =
                parse_feature_size(cfg.get("minimum_size").ok_or("tree.minimum_size")?)?;
            let decorators = parse_tree_decorators(cfg, "decorators")?;
            let bool_field = |key: &str| -> Result<bool, String> {
                match cfg.get(key) {
                    Some(Json::Bool(b)) => Ok(*b),
                    None => Ok(false), // orElse(false)
                    _ => Err(format!("tree.{key}")),
                }
            };
            let ignore_vines = bool_field("ignore_vines")?;
            let force_dirt = bool_field("force_dirt")?;
            FeatureDef::Tree {
                trunk_provider,
                trunk_placer,
                foliage_provider,
                foliage_placer,
                dirt_provider,
                minimum_size,
                decorators,
                ignore_vines,
                force_dirt,
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
    /// weighted_state_provider (inc. 11) — WeightedStateProvider.CODEC:
    /// entries = WeightedList.nonEmptyCodec(BlockState.CODEC)
    /// .comapFlatMap(create).fieldOf("entries") — REQUIRED and the
    /// nonEmptyList + create() double-check makes EMPTY entries an honest
    /// codec error ("WeightedStateProvider with no states"); each entry =
    /// Weighted.codec: data (fieldOf REQUIRED) + weight
    /// (ExtraCodecs.NON_NEGATIVE_INT fieldOf REQUIRED — 0 is legal at
    /// runtime, only an IDE warning). Execution tail: getState =
    /// weightedList.getRandomOrThrow(random) — nextInt(totalWeight) +
    /// in-order accumulated walk (WeightedListInt semantics, inc. 10).
    Weighted(Vec<(String, i64)>),
    /// randomized_int_state_provider (inc. 15) —
    /// RandomizedIntStateProvider.CODEC field order: property (Codec.STRING
    /// fieldOf) -> source (BlockStateProvider.CODEC fieldOf, FULL provider
    /// path) -> values (IntProvider.CODEC fieldOf). Execution tail:
    /// getState = source.getState(random, pos), then — ONLY if the state
    /// has the property — trySetValue(property, values.sample(random))
    /// (the sample happens on the hasProperty branch only).
    RandomizedInt {
        property: String,
        source: Box<StateProvider>,
        values: IntProvider,
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
    match ty {
        "simple_state_provider" => {
            let st = j.get("state").ok_or("state")?;
            let state = parse_state_name(st)?;
            Ok(StateProvider::Simple(state))
        }
        "weighted_state_provider" => {
            let entries = j
                .get("entries")
                .and_then(|e| e.as_arr())
                .ok_or("weighted_state_provider.entries")?;
            if entries.is_empty() {
                return Err("WeightedStateProvider with no states".into());
            }
            let mut items = Vec::new();
            for e in entries {
                let st = e.get("data").ok_or("weighted.data")?;
                let state = parse_state_name(st)?;
                let weight = e
                    .get("weight")
                    .and_then(|w| w.as_i64())
                    .ok_or("weighted.weight")?;
                if weight < 0 {
                    return Err(format!("weighted weight NON_NEGATIVE, got {weight}"));
                }
                items.push((state, weight));
            }
            Ok(StateProvider::Weighted(items))
        }
        "randomized_int_state_provider" => {
            // RandomizedIntStateProvider.CODEC field order: property ->
            // source -> values (all fieldOf REQUIRED).
            let property = j
                .get("property")
                .and_then(|p| p.as_str())
                .ok_or("randomized_int.property")?
                .to_string();
            let source =
                Box::new(parse_state_provider(j.get("source").ok_or("randomized_int.source")?)?);
            let values = IntProvider::parse(j.get("values").ok_or("randomized_int.values")?)?;
            Ok(StateProvider::RandomizedInt {
                property,
                source,
                values,
            })
        }
        other => Err(format!("unsupported state provider {other}")),
    }
}

/// TreeDecorator IR (inc. 11) — the two corpus shapes of the
/// fallen_tree decorators; other decorator types are an honest Err.
#[derive(Debug, Clone)]
pub enum TreeDecorator {
    /// attached_to_logs (AttachedToLogsDecorator.CODEC) — probability =
    /// floatRange(0,1) fieldOf REQUIRED; block_provider =
    /// BlockStateProvider.CODEC fieldOf REQUIRED (FULL provider path);
    /// directions = ExtraCodecs.nonEmptyList(Direction.CODEC.listOf())
    /// fieldOf REQUIRED (empty = codec error; names are the 6 canonical
    /// Direction enum names). Execution tail documented on
    /// FeatureDef::FallenTree.
    AttachedToLogs {
        probability: f32,
        block_provider: StateProvider,
        directions: Vec<String>,
    },
    /// trunk_vine (TrunkVineDecorator.CODEC = MapCodec.unit — NO fields).
    TrunkVine,
    /// beehive (BeehiveDecorator.CODEC, inc. 12): probability =
    /// floatRange(0,1) fieldOf REQUIRED. Execution tail: per curated log
    /// position a nextFloat() <= probability gate, then the BEEHIVE block
    /// with the BeehiveBlockEntity NBT (2-3 bees — honey_level 5, facing
    /// draw) — wired at execution like the Spring FluidState pin.
    Beehive {
        probability: f32,
    },
    /// cocoa (CocoaDecorator.CODEC 1.21.10): probability = floatRange(0,1)
    /// fieldOf REQUIRED (the older vertical/wall quantity fields are GONE
    /// in 1.21.10). Execution tail: per curated log face a nextFloat() <=
    /// probability gate then a cocoa block with a nextInt(3) age draw.
    Cocoa {
        probability: f32,
    },
    /// leave_vine (LeaveVineDecorator.CODEC): probability = floatRange(0,1)
    /// fieldOf REQUIRED. Execution tail: per curated leaf position four
    /// directional nextFloat() <= probability gates (N/S/W/E vines).
    LeaveVine {
        probability: f32,
    },
    /// place_on_ground (PlaceOnGroundDecorator.CODEC, 1.21.5+ leaf-litter
    /// trees): tries = POSITIVE_INT fieldOf orElse(128), radius =
    /// NON_NEGATIVE_INT fieldOf orElse(2), height = NON_NEGATIVE_INT
    /// fieldOf orElse(1), block_state_provider = BlockStateProvider.CODEC
    /// fieldOf REQUIRED (weighted shapes go through the full provider
    /// path). Execution tail: tries iterations over a radius/height box
    /// around the curated leaves' ground projection, per-placed-block
    /// provider draw.
    PlaceOnGround {
        tries: i64,
        radius: i64,
        height: i64,
        block_provider: StateProvider,
    },
}

fn is_canonical_direction(s: &str) -> bool {
    matches!(s, "down" | "up" | "north" | "south" | "west" | "east")
}

fn parse_tree_decorator(j: &Json) -> Result<TreeDecorator, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("tree decorator type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    match ty {
        "trunk_vine" => Ok(TreeDecorator::TrunkVine),
        "beehive" | "cocoa" | "leave_vine" => {
            // all three share the {probability: floatRange(0,1) fieldOf}
            // single-field codec shape.
            let probability = j
                .get("probability")
                .and_then(|p| p.as_f64())
                .ok_or("tree decorator probability")?;
            if !(0.0..=1.0).contains(&probability) {
                return Err(format!("{ty} probability 0..=1, got {probability}"));
            }
            let probability = probability as f32;
            Ok(match ty {
                "beehive" => TreeDecorator::Beehive { probability },
                "cocoa" => TreeDecorator::Cocoa { probability },
                _ => TreeDecorator::LeaveVine { probability },
            })
        }
        "place_on_ground" => {
            let tries = j.get("tries").and_then(|t| t.as_i64()).unwrap_or(128);
            if tries < 1 {
                return Err(format!("place_on_ground tries POSITIVE, got {tries}"));
            }
            let radius = j.get("radius").and_then(|t| t.as_i64()).unwrap_or(2);
            if radius < 0 {
                return Err(format!("place_on_ground radius NON_NEGATIVE, got {radius}"));
            }
            let height = j.get("height").and_then(|t| t.as_i64()).unwrap_or(1);
            if height < 0 {
                return Err(format!("place_on_ground height NON_NEGATIVE, got {height}"));
            }
            let block_provider = parse_state_provider(
                j.get("block_state_provider")
                    .ok_or("place_on_ground.block_state_provider")?,
            )?;
            Ok(TreeDecorator::PlaceOnGround {
                tries,
                radius,
                height,
                block_provider,
            })
        }
        "attached_to_logs" => {
            let probability = j
                .get("probability")
                .and_then(|p| p.as_f64())
                .ok_or("attached_to_logs.probability")?;
            if !(0.0..=1.0).contains(&probability) {
                return Err(format!(
                    "attached_to_logs probability 0..=1, got {probability}"
                ));
            }
            let block_provider = parse_state_provider(
                j.get("block_provider")
                    .ok_or("attached_to_logs.block_provider")?,
            )?;
            let dirs = j
                .get("directions")
                .and_then(|d| d.as_arr())
                .ok_or("attached_to_logs.directions")?;
            if dirs.is_empty() {
                return Err("attached_to_logs directions nonEmptyList".into());
            }
            let mut directions = Vec::new();
            for d in dirs {
                let s = d.as_str().ok_or("direction name")?;
                if !is_canonical_direction(s) {
                    return Err(format!("unknown direction {s}"));
                }
                directions.push(s.to_string());
            }
            Ok(TreeDecorator::AttachedToLogs {
                probability: probability as f32,
                block_provider,
                directions,
            })
        }
        other => Err(format!("unsupported tree decorator {other}")),
    }
}

fn parse_tree_decorators(j: &Json, key: &str) -> Result<Vec<TreeDecorator>, String> {
    match j.get(key).and_then(|d| d.as_arr()) {
        Some(arr) => arr.iter().map(parse_tree_decorator).collect(),
        None => Err(key.to_string()),
    }
}

/// TrunkPlacer IR (inc. 12) — the verdict subset. TrunkPlacer.CODEC
/// dispatches by type; the base parts are base_height intRange(0,32)
/// fieldOf + height_rand_a intRange(0,24) fieldOf + height_rand_b
/// intRange(0,24) fieldOf (StraightTrunkPlacer.CODEC = trunkPlacerParts
/// ONLY). Other placer types are an honest Err.
#[derive(Debug, Clone)]
pub enum TrunkPlacer {
    /// straight_trunk_placer. Draw tail (TrunkPlacer.placeTrunk, verdict
    /// summary): tree height = base_height + nextInt(height_rand_a + 1) +
    /// nextInt(height_rand_b + 1) draws, logs set upward with the
    /// trunk_provider draws, dirt below, decorators in LIST order after.
    Straight {
        base_height: i32,
        height_rand_a: i32,
        height_rand_b: i32,
    },
}

fn parse_trunk_placer(j: &Json) -> Result<TrunkPlacer, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("trunk placer type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    match ty {
        "straight_trunk_placer" => {
            let base = j
                .get("base_height")
                .and_then(|v| v.as_i64())
                .ok_or("trunk.base_height")?;
            let ra = j
                .get("height_rand_a")
                .and_then(|v| v.as_i64())
                .ok_or("trunk.height_rand_a")?;
            let rb = j
                .get("height_rand_b")
                .and_then(|v| v.as_i64())
                .ok_or("trunk.height_rand_b")?;
            if !(0..=32).contains(&base) {
                return Err(format!("trunk base_height 0..=32, got {base}"));
            }
            if !(0..=24).contains(&ra) {
                return Err(format!("trunk height_rand_a 0..=24, got {ra}"));
            }
            if !(0..=24).contains(&rb) {
                return Err(format!("trunk height_rand_b 0..=24, got {rb}"));
            }
            Ok(TrunkPlacer::Straight {
                base_height: base as i32,
                height_rand_a: ra as i32,
                height_rand_b: rb as i32,
            })
        }
        other => Err(format!("unsupported trunk placer {other}")),
    }
}

/// FoliagePlacer IR (inc. 12) — the verdict subset. FoliagePlacer.CODEC
/// dispatches by type; the base parts are radius IntProvider.codec(0,16)
/// fieldOf + offset IntProvider.codec(0,16) fieldOf. BlobFoliagePlacer
/// adds height intRange(0,16) fieldOf. Other placer types are an honest
/// Err.
#[derive(Debug, Clone)]
pub enum FoliagePlacer {
    /// blob_foliage_placer. Draw tail (BlobFoliagePlacer.placeOnLine,
    /// verdict summary): per foliage layer a radius/offset draw pair,
    /// blob walk with provider draws per set block.
    Blob {
        radius: IntProvider,
        offset: IntProvider,
        height: i32,
    },
}

fn int_provider_in_range(
    j: &Json,
    key: &str,
    lo: i32,
    hi: i32,
) -> Result<IntProvider, String> {
    // IntProvider.codec(lo,hi): dispatch-or-bare-int decode, then the
    // min/max mirror must sit inside the bound (IntProvider.validate).
    let v = j.get(key).ok_or(key)?;
    let p = IntProvider::parse(v)?;
    let (a, b) = (p.min_value(), p.max_value());
    if a < lo || b > hi {
        return Err(format!("{key} {lo}..={hi}, got {a}..{b}"));
    }
    Ok(p)
}

fn parse_foliage_placer(j: &Json) -> Result<FoliagePlacer, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("foliage placer type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    match ty {
        "blob_foliage_placer" => {
            let radius = int_provider_in_range(j, "radius", 0, 16)?;
            let offset = int_provider_in_range(j, "offset", 0, 16)?;
            let height = j
                .get("height")
                .and_then(|v| v.as_i64())
                .ok_or("blob.height")?;
            if !(0..=16).contains(&height) {
                return Err(format!("blob height 0..=16, got {height}"));
            }
            Ok(FoliagePlacer::Blob {
                radius,
                offset,
                height: height as i32,
            })
        }
        other => Err(format!("unsupported foliage placer {other}")),
    }
}

/// FeatureSize IR (inc. 12) — the verdict subset. FeatureSize.CODEC
/// dispatches by type; TwoLayersFeatureSize.CODEC = limit intRange(0,81)
/// orElse(1) + lower_size intRange(0,16) orElse(0) + upper_size
/// intRange(0,16) orElse(1) + minClippedHeightCodec (optional
/// intRange(0,80) -> Option). ThreeLayers and other types = honest Err.
#[derive(Debug, Clone)]
pub enum FeatureSize {
    TwoLayers {
        limit: i32,
        lower_size: i32,
        upper_size: i32,
        min_clipped_height: Option<i32>,
    },
}

fn parse_feature_size(j: &Json) -> Result<FeatureSize, String> {
    let ty = j
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("feature size type")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    match ty {
        "two_layers_feature_size" => {
            let limit = j.get("limit").and_then(|v| v.as_i64()).unwrap_or(1);
            let lower = j.get("lower_size").and_then(|v| v.as_i64()).unwrap_or(0);
            let upper = j.get("upper_size").and_then(|v| v.as_i64()).unwrap_or(1);
            if !(0..=81).contains(&limit) {
                return Err(format!("two_layers limit 0..=81, got {limit}"));
            }
            if !(0..=16).contains(&lower) {
                return Err(format!("two_layers lower_size 0..=16, got {lower}"));
            }
            if !(0..=16).contains(&upper) {
                return Err(format!("two_layers upper_size 0..=16, got {upper}"));
            }
            let min_clipped_height = match j.get("min_clipped_height").and_then(|v| v.as_i64()) {
                Some(h) => {
                    if !(0..=80).contains(&h) {
                        return Err(format!("min_clipped_height 0..=80, got {h}"));
                    }
                    Some(h as i32)
                }
                None => None,
            };
            Ok(FeatureSize::TwoLayers {
                limit: limit as i32,
                lower_size: lower as i32,
                upper_size: upper as i32,
                min_clipped_height,
            })
        }
        other => Err(format!("unsupported feature size {other}")),
    }
}

/// Compact-state leaf for callers that must stay SIMPLE (random_patch
/// to_place, geode block providers — the corpus shapes there are all
/// simple; RuleBased / Weighted shapes stay an honest Unsupported there).
fn parse_simple_state_provider(j: &Json) -> Result<String, String> {
    match parse_state_provider(j)? {
        StateProvider::Simple(s) => Ok(s),
        StateProvider::RuleBased { .. } => Err("rule_based_state_provider with rules".into()),
        StateProvider::Weighted(_) => Err("weighted_state_provider in simple context".into()),
        StateProvider::RandomizedInt { .. } => {
            Err("randomized_int_state_provider in simple context".into())
        }
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

/// Composite/leaf support verdict for a RESOLVED configured body (named
/// lookup or strict inline re-parse). Composites recurse through
/// is_placed_supported. Inc. 14: the OLD composites' inline trust-hole
/// (inc. 8/9: only the 4 composite names conservatively false, everything
/// else trusted) is RETIRED — every inline body, old composite or selector
/// element, rides the same strict re-parse; is_selector_element_supported
/// collapsed into is_placed_supported (one verdict path).
fn is_def_supported(body: &FeatureDef, registry: &FeatureRegistry) -> bool {
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
        FeatureDef::SimpleRandomSelector { features } => features
            .iter()
            .all(|f| is_placed_supported(f, registry)),
        _ => true,
    }
}

/// Full support verdict for one placed feature (modifiers + configured body
/// + the bodies reachable through random_patch nesting). Uniform strict
/// verdict since inc. 14: an INLINE body is re-parsed through the same
/// parse_configured_def dispatch as named features (dispatch-miss types and
/// parse-err shapes are honest false) — the inc. 8/9 trust rule (4
/// composite names conservatively false, everything else trusted) is
/// RETIRED, so this single function now also serves the
/// simple_random_selector element verdicts (with the element's OWN
/// placement modifiers checked first, which the inc. 13 selector-only path
/// skipped).
pub fn is_placed_supported(def: &PlacedFeatureDef, registry: &FeatureRegistry) -> bool {
    if def
        .placement
        .iter()
        .any(|m| matches!(m, PlacementMod::Unsupported(_)))
    {
        return false;
    }
    // INLINE configured body (inc. 14): the OLD trust rule (inc. 8/9 — 4
    // composite names conservatively false, everything else trusted) is
    // RETIRED; every inline body now rides the uniform strict path (the
    // inc. 13 selector-element verdict): re-parse through the same
    // parse_configured_def dispatch as named features — dispatch-miss and
    // parse-err shapes are honest false. The parked body carries the FULL
    // {type, config} shape (inc. 13), so no info is lost.
    if let Some((short, body)) = &def.inline {
        return match parse_configured_def(body, short) {
            Ok(d) => is_def_supported(&d, registry),
            Err(_) => false,
        };
    }
    match registry.configured.get(&def.feature_ref) {
        Some(cfg) => is_def_supported(cfg, registry),
        None => false,
    }
}

/// Blocker label of one placed def (modifier arm + inline arm + named arm).
pub fn placed_blocker_label(def: &PlacedFeatureDef, registry: &FeatureRegistry) -> Option<String> {
    for m in &def.placement {
        if let PlacementMod::Unsupported(t) = m {
            return Some(t.clone());
        }
    }
    if let Some((short, body)) = &def.inline {
        // inc. 14: strict deep label (the inc. 13 selector-element mirror)
        // — parse-err -> the error text, dispatch-miss -> "unsupported type
        // X", composite -> deepest recursion; replaces the old
        // bare-parked-short trust-rule label.
        return match parse_configured_def(body, short) {
            Err(e) => Some(e),
            Ok(FeatureDef::Unsupported(t)) => Some(format!("unsupported type {t}")),
            Ok(d) => def_blocker_label(&d, registry),
        };
    }
    registry
        .configured
        .get(&def.feature_ref)
        .and_then(|cfg| def_blocker_label(cfg, registry))
}

/// Blocker label of a RESOLVED configured body (composite recursion shared
/// by the named and strict-inline label paths).
pub fn def_blocker_label(body: &FeatureDef, registry: &FeatureRegistry) -> Option<String> {
    match body {
        FeatureDef::Unsupported(t) => Some(format!("unsupported type {t}")),
        FeatureDef::RandomPatch { inner, .. } => placed_blocker_label(inner, registry),
        FeatureDef::RandomSelector { features, default } => features
            .iter()
            .find_map(|(_, f)| placed_blocker_label(f, registry))
            .or_else(|| placed_blocker_label(default, registry)),
        FeatureDef::RandomBooleanSelector {
            feature_true,
            feature_false,
        } => placed_blocker_label(feature_true, registry)
            .or_else(|| placed_blocker_label(feature_false, registry)),
        FeatureDef::SimpleRandomSelector { features } => features
            .iter()
            .find_map(|f| placed_blocker_label(f, registry)),
        _ => None,
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

    #[test]
    fn simple_random_selector_verbatim_warm_ocean() {
        // verbatim from configured_feature/warm_ocean_vegetation.json — the
        // HolderSet LIST form with three INLINE placed elements.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{\"features\":[\
             {\"feature\":{\"type\":\"minecraft:coral_tree\",\"config\":{}},\"placement\":[]},\
             {\"feature\":{\"type\":\"minecraft:coral_claw\",\"config\":{}},\"placement\":[]},\
             {\"feature\":{\"type\":\"minecraft:coral_mushroom\",\"config\":{}},\"placement\":[]}]}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::SimpleRandomSelector { features } =
            parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected SimpleRandomSelector");
        };
        assert_eq!(features.len(), 3);
        for f in &features {
            assert_eq!(f.feature_ref, "");
            assert!(f.placement.is_empty());
            assert!(f.inline.is_some());
        }
        let (s0, _) = features[0].inline.as_ref().unwrap();
        assert_eq!(s0, "coral_tree");

        // codec honesty: nonEmptyHolderSet — an EMPTY list = codec error;
        // a missing features key = codec error; a "#tag" holder-set = honest
        // unsupported (zero corpus instances, label documents the shape).
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{\"features\":[]}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j2, "simple_random_selector").is_err());
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j3, "simple_random_selector").is_err());
        let j4 = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":\
             {\"features\":\"#minecraft:corals\"}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j4, "simple_random_selector")
            .unwrap_err()
            .contains("tag holder-set unsupported"));
    }

    #[test]
    fn simple_random_selector_strict_element_verdicts() {
        // STRICT element verdicts (inc. 13): dispatch-miss inner types and
        // parse-err shapes are honest FALSE with precise labels; a fully
        // supported inline body is honest TRUE. The registry is EMPTY (temp
        // dir) — every verdict below rides the inline strict path, no named
        // lookup is involved.
        let tmp = std::env::temp_dir().join(format!("ncf_p4_sel_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("data/minecraft/worldgen")).unwrap();
        let dir = crate::router::WorldgenDir::load(&tmp).unwrap();
        let registry = FeatureRegistry {
            configured: HashMap::new(),
            placed: HashMap::new(),
            dir: &dir,
        };
        let _ = std::fs::remove_dir_all(&tmp);

        // coral dispatch-miss -> Unsupported -> honest false.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{\"features\":[\
             {\"feature\":{\"type\":\"minecraft:coral_tree\",\"config\":{}},\"placement\":[]}]}}",
        )
        .unwrap();
        let def = parse_configured_def(&j, "simple_random_selector").unwrap();
        assert!(!is_def_supported(&def, &registry));
        assert_eq!(
            def_blocker_label(&def, &registry).as_deref(),
            Some("unsupported type coral_tree")
        );

        // parse-err shape (weighted to_place in the compact simple_block
        // leaf, verbatim dripleaf.json element shape) -> honest false with
        // the parse-err text as the label.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{\"features\":[\
             {\"feature\":{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:weighted_state_provider\",\"entries\":[\
             {\"data\":{\"Name\":\"minecraft:stone\"},\"weight\":1}]}}},\
             \"placement\":[]}]}}",
        )
        .unwrap();
        let def2 = parse_configured_def(&j2, "simple_random_selector").unwrap();
        assert!(!is_def_supported(&def2, &registry));
        assert_eq!(
            def_blocker_label(&def2, &registry).as_deref(),
            Some("weighted_state_provider in simple context")
        );

        // fully supported inline element body (plain simple_block) ->
        // honest true, no label.
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:simple_random_selector\",\"config\":{\"features\":[\
             {\"feature\":{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:stone\"}}}},\"placement\":[]}]}}",
        )
        .unwrap();
        let def3 = parse_configured_def(&j3, "simple_random_selector").unwrap();
        assert!(is_def_supported(&def3, &registry));
        assert!(def_blocker_label(&def3, &registry).is_none());
    }

    #[test]
    fn uniform_strict_inline_verdict_after_trust_retirement() {
        // inc. 14: the OLD composites' inline trust rule (inc. 8/9) is
        // RETIRED — an inline body verdicted through is_placed_supported
        // now rides the same strict re-parse as simple_random_selector
        // elements (inc. 13). The registry is EMPTY (temp dir): every
        // verdict below rides the inline path only.
        let tmp = std::env::temp_dir().join(format!("ncf_p4_trust_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("data/minecraft/worldgen")).unwrap();
        let dir = crate::router::WorldgenDir::load(&tmp).unwrap();
        let registry = FeatureRegistry {
            configured: HashMap::new(),
            placed: HashMap::new(),
            dir: &dir,
        };
        let _ = std::fs::remove_dir_all(&tmp);

        // OLD behavior trusted this body TRUE (non-composite short). Under
        // the uniform strict verdict it is a parse-err shape (weighted
        // to_place in the compact simple_block leaf) -> honest false with
        // the parse-err label.
        let mk = |body: &str| {
            crate::json::parse(&format!("{{\"feature\":{body},\"placement\":[]}}")).unwrap()
        };
        let weighted = mk(
            "{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:weighted_state_provider\",\"entries\":[\
             {\"data\":{\"Name\":\"minecraft:stone\"},\"weight\":1}]}}}",
        );
        let def = parse_placed_feature(&weighted).unwrap();
        assert!(!is_placed_supported(&def, &registry));
        assert_eq!(
            placed_blocker_label(&def, &registry).as_deref(),
            Some("weighted_state_provider in simple context")
        );

        // dispatch-miss inner type -> honest false, "unsupported type X"
        // (the old trust rule returned the bare short as the label).
        let coral = mk("{\"type\":\"minecraft:coral_claw\",\"config\":{}}");
        let def2 = parse_placed_feature(&coral).unwrap();
        assert!(!is_placed_supported(&def2, &registry));
        assert_eq!(
            placed_blocker_label(&def2, &registry).as_deref(),
            Some("unsupported type coral_claw")
        );

        // a fully supported inline body (plain simple_block) -> honest
        // true, no label — the strict path does not over-block.
        let plain = mk(
            "{\"type\":\"minecraft:simple_block\",\"config\":\
             {\"to_place\":{\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:stone\"}}}}",
        );
        let def3 = parse_placed_feature(&plain).unwrap();
        assert!(is_placed_supported(&def3, &registry));
        assert!(placed_blocker_label(&def3, &registry).is_none());
    }

    #[test]
    fn block_column_verbatim_patch_cactus() {
        // verbatim inner body of configured_feature/patch_cactus.json (two
        // layers: biased_to_bottom 1..3 cactus[age=0], weighted_list
        // [0w3,1w1] cactus_flower) + honest codec errors in field order.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:block_column\",\"config\":\
             {\"allowed_placement\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":\"minecraft:air\"},\
             \"direction\":\"up\",\
             \"layers\":[\
              {\"height\":{\"type\":\"minecraft:biased_to_bottom\",\"max_inclusive\":3,\"min_inclusive\":1},\
               \"provider\":{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{\"Name\":\"minecraft:cactus\",\"Properties\":{\"age\":\"0\"}}}},\
              {\"height\":{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
                 {\"data\":0,\"weight\":3},{\"data\":1,\"weight\":1}]},\
               \"provider\":{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{\"Name\":\"minecraft:cactus_flower\"}}}],\
             \"prioritize_tip\":false}}",
        )
        .unwrap();
        let def = parse_configured_def(&j, "block_column").unwrap();
        let FeatureDef::BlockColumn {
            layers,
            direction,
            allowed_placement,
            prioritize_tip,
        } = &def
        else {
            panic!("expected BlockColumn");
        };
        assert_eq!(direction, "up");
        assert!(!prioritize_tip);
        assert_eq!(layers.len(), 2);
        assert!(matches!(layers[0].0, IntProvider::BiasedToBottom(1, 3)));
        assert!(matches!(
            layers[0].1,
            StateProvider::Simple(ref s) if s == "minecraft:cactus[age=0]"
        ));
        assert!(matches!(
            &layers[1].0,
            IntProvider::WeightedList(w)
                if w[0] == (3, IntProvider::Constant(0)) && w[1] == (1, IntProvider::Constant(1))
        ));
        assert!(matches!(
            layers[1].1,
            StateProvider::Simple(ref s) if s == "minecraft:cactus_flower"
        ));
        // allowed_placement: matching_blocks air (single-string form).
        let BlockPredicate::MatchingBlocks { blocks, .. } = allowed_placement else {
            panic!("expected MatchingBlocks");
        };
        assert_eq!(blocks, &["minecraft:air".to_string()]);

        // honest codec errors — the first failing field owns the label.
        let err =
            |body: &str| {
                parse_configured_def(&crate::json::parse(body).unwrap(), "block_column")
                    .unwrap_err()
            };
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"direction\":\"up\",\
                 \"layers\":[],\"allowed_placement\":{\"type\":\"minecraft:true\"}}}",
            ),
            "block_column.prioritize_tip"
        );
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"layers\":[],\
                 \"allowed_placement\":{\"type\":\"minecraft:true\"},\"prioritize_tip\":false}}",
            ),
            "block_column.direction"
        );
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"direction\":\"sideways\",\
                 \"layers\":[],\"allowed_placement\":{\"type\":\"minecraft:true\"},\
                 \"prioritize_tip\":false}}",
            ),
            "unsupported direction sideways"
        );
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"direction\":\"up\",\
                 \"layers\":[],\"prioritize_tip\":false}}",
            ),
            "block_column.allowed_placement"
        );
        // NON_NEGATIVE height range mirror: a bare -1 is honest Err.
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"direction\":\"up\",\
                 \"layers\":[{\"height\":-1,\"provider\":{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{\"Name\":\"minecraft:stone\"}}}],\
                 \"allowed_placement\":{\"type\":\"minecraft:true\"},\"prioritize_tip\":false}}",
            ),
            "height 0..=2147483647, got -1..-1"
        );
        // unsupported provider family inside a layer.
        assert_eq!(
            err("{\"type\":\"minecraft:block_column\",\"config\":{\"direction\":\"up\",\
                 \"layers\":[{\"height\":1,\"provider\":{\"type\":\"minecraft:noise_provider\",\
                 \"input\":\"minecraft:temperature\",\"scale\":0.1}}],\
                 \"allowed_placement\":{\"type\":\"minecraft:true\"},\"prioritize_tip\":false}}",
            ),
            "unsupported state provider noise_provider"
        );
    }

    #[test]
    fn block_column_verbatim_cave_vine_randomized_int() {
        // verbatim shape of configured_feature/cave_vine.json — direction
        // down, prioritize_tip true, WeightedList heights, and the
        // randomized_int_state_provider family (inc. 15). On an EMPTY
        // registry the strict inline verdict is honest TRUE (leaf body).
        let body = "{\"type\":\"minecraft:block_column\",\"config\":\
             {\"allowed_placement\":{\"type\":\"minecraft:matching_blocks\",\"blocks\":\"minecraft:air\"},\
             \"direction\":\"down\",\
             \"layers\":[\
              {\"height\":{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
                 {\"data\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":19,\"min_inclusive\":0},\"weight\":2},\
                 {\"data\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":2,\"min_inclusive\":0},\"weight\":3},\
                 {\"data\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":6,\"min_inclusive\":0},\"weight\":10}]},\
               \"provider\":{\"type\":\"minecraft:weighted_state_provider\",\"entries\":[\
                 {\"data\":{\"Name\":\"minecraft:cave_vines_plant\",\"Properties\":{\"berries\":\"false\"}},\"weight\":4},\
                 {\"data\":{\"Name\":\"minecraft:cave_vines_plant\",\"Properties\":{\"berries\":\"true\"}},\"weight\":1}]}},\
              {\"height\":1,\
               \"provider\":{\"type\":\"minecraft:randomized_int_state_provider\",\
                 \"property\":\"age\",\
                 \"source\":{\"type\":\"minecraft:weighted_state_provider\",\"entries\":[\
                   {\"data\":{\"Name\":\"minecraft:cave_vines\",\"Properties\":{\"age\":\"0\",\"berries\":\"false\"}},\"weight\":4},\
                   {\"data\":{\"Name\":\"minecraft:cave_vines\",\"Properties\":{\"age\":\"0\",\"berries\":\"true\"}},\"weight\":1}]},\
                 \"values\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":25,\"min_inclusive\":23}}}],\
             \"prioritize_tip\":true}}";
        let j = crate::json::parse(body).unwrap();
        let def = parse_configured_def(&j, "block_column").unwrap();
        let FeatureDef::BlockColumn {
            layers,
            direction,
            prioritize_tip,
            ..
        } = &def
        else {
            panic!("expected BlockColumn");
        };
        assert_eq!(direction, "down");
        assert!(*prioritize_tip);
        assert_eq!(layers.len(), 2);
        assert!(matches!(
            &layers[0].0,
            IntProvider::WeightedList(w)
                if w.len() == 3 && w[0] == (2, IntProvider::Uniform(0, 19))
                    && w[1] == (3, IntProvider::Uniform(0, 2))
                    && w[2] == (10, IntProvider::Uniform(0, 6))
        ));
        assert!(matches!(
            &layers[0].1,
            StateProvider::Weighted(w)
                if w.len() == 2
                    && w[0].0 == "minecraft:cave_vines_plant[berries=false]"
                    && w[1].0 == "minecraft:cave_vines_plant[berries=true]"
        ));
        assert!(matches!(layers[1].0, IntProvider::Constant(1)));
        let StateProvider::RandomizedInt {
            property,
            source,
            values,
        } = &layers[1].1
        else {
            panic!("expected RandomizedInt");
        };
        assert_eq!(property, "age");
        assert!(matches!(values, IntProvider::Uniform(23, 25)));
        assert!(matches!(
            source.as_ref(),
            StateProvider::Weighted(w) if w.len() == 2
        ));

        // the strict inline verdict (empty registry, inline path only).
        let tmp = std::env::temp_dir().join(format!("ncf_p4_bcol_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("data/minecraft/worldgen")).unwrap();
        let dir = crate::router::WorldgenDir::load(&tmp).unwrap();
        let registry = FeatureRegistry {
            configured: HashMap::new(),
            placed: HashMap::new(),
            dir: &dir,
        };
        let _ = std::fs::remove_dir_all(&tmp);
        assert!(is_def_supported(&def, &registry));
        let wrapped = crate::json::parse(&format!("{{\"feature\":{body},\"placement\":[]}}"))
            .unwrap();
        let placed = parse_placed_feature(&wrapped).unwrap();
        assert!(is_placed_supported(&placed, &registry));
        assert!(placed_blocker_label(&placed, &registry).is_none());
    }

    #[test]
    fn noise_threshold_count_verbatim_flower_cherry() {
        // verbatim from placed_feature/flower_cherry.json — plain Codec.INT
        // for below/above (NO range validation), DOUBLE noise_level.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:noise_threshold_count\",\"noise_level\":-0.8,\
             \"below_noise\":5,\"above_noise\":10}",
        )
        .unwrap();
        match parse_placement_mod(&j).unwrap() {
            PlacementMod::NoiseThresholdCount {
                noise_level,
                below_noise,
                above_noise,
            } => {
                assert_eq!(noise_level, -0.8);
                assert_eq!(below_noise, 5);
                assert_eq!(above_noise, 10);
            }
            other => panic!("expected NoiseThresholdCount, got {other:?}"),
        }
        // honest codec errors: each of the three keys is fieldOf REQUIRED.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:noise_threshold_count\",\"below_noise\":5,\"above_noise\":10}",
        )
        .unwrap();
        assert!(parse_placement_mod(&j2).is_err());
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:noise_threshold_count\",\"noise_level\":-0.8,\"above_noise\":10}",
        )
        .unwrap();
        assert!(parse_placement_mod(&j3).is_err());
        let j4 = crate::json::parse(
            "{\"type\":\"minecraft:noise_threshold_count\",\"noise_level\":-0.8,\"below_noise\":5}",
        )
        .unwrap();
        assert!(parse_placement_mod(&j4).is_err());
        // plain Codec.INT: a negative count is codec-legal (honest mirror of
        // the 1.21.10 codec — no intRange anywhere on this modifier).
        let j5 = crate::json::parse(
            "{\"type\":\"minecraft:noise_threshold_count\",\"noise_level\":0.0,\
             \"below_noise\":-3,\"above_noise\":-7}",
        )
        .unwrap();
        assert!(matches!(
            parse_placement_mod(&j5).unwrap(),
            PlacementMod::NoiseThresholdCount { .. }
        ));
    }

    #[test]
    fn weighted_list_count_verbatim_trees_birch() {
        // verbatim from placed_feature/trees_birch.json — the count modifier
        // that held up the whole 36-biome tree-chain top ("intprovider
        // shape" before inc. 10).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
             {\"data\":10,\"weight\":9},{\"data\":11,\"weight\":1}]}",
        )
        .unwrap();
        let p = IntProvider::parse(&j).unwrap();
        let IntProvider::WeightedList(items) = &p else {
            panic!("expected weighted_list");
        };
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], (9, IntProvider::Constant(10)));
        assert_eq!(items[1], (1, IntProvider::Constant(11)));
        // Java getMinValue/getMaxValue: min-of-mins / max-of-maxes.
        assert_eq!((p.min_value(), p.max_value()), (10, 11));
        // sample: ONE nextInt(total=10) draw, the picked entry is a
        // Constant (no further draws) — the draw count must be exactly 1
        // for both possible picks.
        let mut dr = crate::feature_sorter::DecorationRandom::new(3053459);
        let before = dr.count_wg();
        let v = p.sample(&mut dr);
        assert_eq!(dr.count_wg() - before, 1);
        assert!((10..=11).contains(&v));

        // verbatim from configured_feature/cherry.json — trunk_placer
        // branch_count rides the same weighted_list shape.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
             {\"data\":{\"type\":\"minecraft:uniform\",\"min_inclusive\":2,\
             \"max_inclusive\":4},\"weight\":1}]}",
        )
        .unwrap();
        let IntProvider::WeightedList(items2) = IntProvider::parse(&j2).unwrap() else {
            panic!("expected weighted_list");
        };
        assert_eq!(items2.len(), 1);
        assert!(matches!(items2[0].1, IntProvider::Uniform(2, 4)));

        // codec honesty: nonEmptyCodec — an EMPTY distribution = error;
        // weight is NON_NEGATIVE_INT; a missing data key = error.
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:weighted_list\",\"distribution\":[]}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j3).is_err());
        let j4 = crate::json::parse(
            "{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
             {\"data\":1,\"weight\":-1}]}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j4).is_err());
        let j5 = crate::json::parse(
            "{\"type\":\"minecraft:weighted_list\",\"distribution\":[\
             {\"weight\":1}]}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j5).is_err());
    }

    #[test]
    fn int_provider_family_verbatim_and_honest_errors() {
        // verbatim from placed_feature/glowstone_extra.json (count):
        // biased_to_bottom 0..9 — sample = min + nextInt(nextInt(range+1)+1)
        // with EXACTLY two nextInt draws.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:biased_to_bottom\",\"max_inclusive\":9,\"min_inclusive\":0}",
        )
        .unwrap();
        let p = IntProvider::parse(&j).unwrap();
        assert_eq!((p.min_value(), p.max_value()), (0, 9));
        let mut dr = crate::feature_sorter::DecorationRandom::new(3053459);
        let before = dr.count_wg();
        let v = p.sample(&mut dr);
        assert_eq!(dr.count_wg() - before, 2);
        assert!((0..=9).contains(&v));

        // verbatim from placed_feature/flower_forest_flowers.json (count):
        // clamped over a uniform -1..3 clamped to 0..3.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:clamped\",\"max_inclusive\":3,\"min_inclusive\":0,\
             \"source\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":3,\
             \"min_inclusive\":-1}}",
        )
        .unwrap();
        let IntProvider::Clamped { source, min, max } = IntProvider::parse(&j2).unwrap() else {
            panic!("expected clamped");
        };
        assert_eq!((min, max), (0, 3));
        assert_eq!((source.min_value(), source.max_value()), (-1, 3));
        // Java getMinValue/getMaxValue on ClampedInt NARROW the source
        // range: max(min, -1) = 0, min(max, 3) = 3.
        let pc = IntProvider::parse(&j2).unwrap();
        assert_eq!((pc.min_value(), pc.max_value()), (0, 3));

        // verbatim from placed_feature/pointed_dripstone.json (xz_spread):
        // clamped_normal mean 0.0, deviation 3.0, -10..10 — ONE gaussian
        // draw (through the MarsagliaPolar source-side cache).
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:clamped_normal\",\"deviation\":3.0,\
             \"max_inclusive\":10,\"mean\":0.0,\"min_inclusive\":-10}",
        )
        .unwrap();
        let IntProvider::ClampedNormal { mean, deviation, min, max } = IntProvider::parse(&j3)
            .unwrap()
        else {
            panic!("expected clamped_normal");
        };
        assert_eq!((mean, deviation, min, max), (0.0, 3.0, -10, 10));
        let mut dr2 = crate::feature_sorter::DecorationRandom::new(3053459);
        let v2 = IntProvider::parse(&j3).unwrap().sample(&mut dr2);
        assert!((-10..=10).contains(&v2));
        // determinism: same seed -> same value.
        let mut dr3 = crate::feature_sorter::DecorationRandom::new(3053459);
        assert_eq!(IntProvider::parse(&j3).unwrap().sample(&mut dr3), v2);

        // verbatim ConstantInt form ({type: constant, value: N}).
        let j4 = crate::json::parse("{\"type\":\"minecraft:constant\",\"value\":5}").unwrap();
        assert_eq!(IntProvider::parse(&j4).unwrap(), IntProvider::Constant(5));

        // honest codec errors: max < min on every bounded type.
        let j5 = crate::json::parse(
            "{\"type\":\"minecraft:biased_to_bottom\",\"max_inclusive\":-2,\"min_inclusive\":3}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j5).is_err());
        let j6 = crate::json::parse(
            "{\"type\":\"minecraft:clamped\",\"max_inclusive\":0,\"min_inclusive\":3,\
             \"source\":{\"type\":\"minecraft:uniform\",\"max_inclusive\":1,\"min_inclusive\":0}}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j6).is_err());
        let j7 = crate::json::parse(
            "{\"type\":\"minecraft:clamped_normal\",\"deviation\":1.0,\"max_inclusive\":0,\
             \"mean\":0.0,\"min_inclusive\":3}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j7).is_err());
        // an unknown provider type stays an honest error.
        let j8 = crate::json::parse(
            "{\"type\":\"minecraft:multiplied\",\"value\":1}",
        )
        .unwrap();
        assert!(IntProvider::parse(&j8).is_err());
    }

    #[test]
    fn fallen_tree_verbatim_birch_and_oak() {
        // verbatim from configured_feature/fallen_birch_tree.json — the
        // uniform log_length 5..8, the weighted_state_provider 2:1
        // red/brown mushroom block_provider, directions ["up"], empty
        // stump_decorators.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:fallen_tree\",\"config\":\
             {\"log_decorators\":[{\"type\":\"minecraft:attached_to_logs\",\
             \"block_provider\":{\"type\":\"minecraft:weighted_state_provider\",\
             \"entries\":[{\"data\":{\"Name\":\"minecraft:red_mushroom\"},\"weight\":2},\
             {\"data\":{\"Name\":\"minecraft:brown_mushroom\"},\"weight\":1}]},\
             \"directions\":[\"up\"],\"probability\":0.1}],\
             \"log_length\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":8,\"min_inclusive\":5},\
             \"stump_decorators\":[],\
             \"trunk_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:birch_log\",\
             \"Properties\":{\"axis\":\"y\"}}}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::FallenTree {
            trunk_provider,
            log_length,
            stump_decorators,
            log_decorators,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected FallenTree");
        };
        assert_eq!(log_length, IntProvider::Uniform(5, 8));
        assert!(matches!(
            &trunk_provider,
            StateProvider::Simple(s) if s == "minecraft:birch_log[axis=y]"
        ));
        assert!(stump_decorators.is_empty());
        assert_eq!(log_decorators.len(), 1);
        let TreeDecorator::AttachedToLogs {
            probability,
            block_provider,
            directions,
        } = &log_decorators[0]
        else {
            panic!("expected AttachedToLogs");
        };
        assert_eq!(*probability, 0.1);
        assert_eq!(directions, &vec!["up".to_string()]);
        assert!(matches!(
            block_provider,
            StateProvider::Weighted(ref items)
                if *items
                    == vec![
                        ("minecraft:red_mushroom".to_string(), 2i64),
                        ("minecraft:brown_mushroom".to_string(), 1i64),
                    ]
        ));

        // verbatim from fallen_oak_tree.json — trunk_vine stump decorator
        // (type-only object, MapCodec.unit) + log_length 4..7.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:fallen_tree\",\"config\":\
             {\"log_decorators\":[],\"log_length\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":7,\"min_inclusive\":4},\
             \"stump_decorators\":[{\"type\":\"minecraft:trunk_vine\"}],\
             \"trunk_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:oak_log\",\
             \"Properties\":{\"axis\":\"y\"}}}}}",
        )
        .unwrap();
        let FeatureDef::FallenTree {
            log_length: ll2,
            stump_decorators: sd2,
            ..
        } = parse_configured_def(&j2, "fallen_tree").unwrap()
        else {
            panic!("expected FallenTree oak");
        };
        assert_eq!(ll2, IntProvider::Uniform(4, 7));
        assert_eq!(sd2.len(), 1);
        assert!(matches!(sd2[0], TreeDecorator::TrunkVine));
    }

    #[test]
    fn fallen_tree_honest_codec_errors() {
        // IntProvider.codec(0, 16): a log_length whose mirror leaves the
        // bound is an honest codec error (uniform 0..17).
        let j = crate::json::parse(
            "{\"type\":\"minecraft:fallen_tree\",\"config\":\
             {\"log_decorators\":[],\"log_length\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":17,\"min_inclusive\":0},\"stump_decorators\":[],\
             \"trunk_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:oak_log\"}}}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j, "fallen_tree").is_err());
        // stump_decorators fieldOf REQUIRED — a missing key = codec error.
        let j2 = crate::json::parse(
            "{\"type\":\"minecraft:fallen_tree\",\"config\":\
             {\"log_decorators\":[],\"log_length\":{\"type\":\"minecraft:uniform\",\
             \"max_inclusive\":7,\"min_inclusive\":4},\"trunk_provider\":\
             {\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:oak_log\"}}}}",
        )
        .unwrap();
        assert!(parse_configured_def(&j2, "fallen_tree").is_err());
        // attached_to_logs: probability floatRange(0,1), directions
        // nonEmptyList, weight NON_NEGATIVE — each an honest error.
        let mk = |inner: &str| {
            crate::json::parse(&format!(
                "{{\"type\":\"minecraft:fallen_tree\",\"config\":\
                 {{\"log_decorators\":[{inner}],\"log_length\":\
                 {{\"type\":\"minecraft:uniform\",\"max_inclusive\":7,\
                 \"min_inclusive\":4}},\"stump_decorators\":[],\
                 \"trunk_provider\":{{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{{\"Name\":\"minecraft:oak_log\"}}}}}}}}"
            ))
            .unwrap()
        };
        assert!(parse_configured_def(
            &mk("{\"type\":\"minecraft:attached_to_logs\",\"probability\":1.5,\
                 \"block_provider\":{\"type\":\"minecraft:weighted_state_provider\",\
                 \"entries\":[{\"data\":{\"Name\":\"minecraft:vine\"},\"weight\":1}]},\
                 \"directions\":[\"up\"]}"),
            "fallen_tree",
        )
        .is_err());
        assert!(parse_configured_def(
            &mk("{\"type\":\"minecraft:attached_to_logs\",\"probability\":0.1,\
                 \"block_provider\":{\"type\":\"minecraft:weighted_state_provider\",\
                 \"entries\":[{\"data\":{\"Name\":\"minecraft:vine\"},\"weight\":1}]},\
                 \"directions\":[]}"),
            "fallen_tree",
        )
        .is_err());
        assert!(parse_configured_def(
            &mk("{\"type\":\"minecraft:attached_to_logs\",\"probability\":0.1,\
                 \"block_provider\":{\"type\":\"minecraft:weighted_state_provider\",\
                 \"entries\":[{\"data\":{\"Name\":\"minecraft:vine\"},\"weight\":-1}]},\
                 \"directions\":[\"up\"]}"),
            "fallen_tree",
        )
        .is_err());
        // empty weighted entries = "WeightedStateProvider with no states".
        let j3 = crate::json::parse(
            "{\"type\":\"minecraft:weighted_state_provider\",\"entries\":[]}",
        )
        .unwrap();
        assert!(parse_state_provider(&j3).is_err());
        // an unknown tree decorator type stays an honest error.
        assert!(parse_configured_def(
            &mk("{\"type\":\"minecraft:leave_vine\"}"),
            "fallen_tree",
        )
        .is_err());
    }

    #[test]
    fn tree_verbatim_birch_bees_0002() {
        // verbatim from configured_feature/birch_bees_0002.json — the
        // trees_birch selector default that held up the birch chain.
        let j = crate::json::parse(
            "{\"type\":\"minecraft:tree\",\"config\":{\"decorators\":[\
             {\"type\":\"minecraft:beehive\",\"probability\":0.002}],\
             \"dirt_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:dirt\"}},\
             \"foliage_placer\":{\"type\":\"minecraft:blob_foliage_placer\",\
             \"height\":3,\"offset\":0,\"radius\":2},\
             \"foliage_provider\":{\"type\":\"minecraft:simple_state_provider\",\
             \"state\":{\"Name\":\"minecraft:birch_leaves\",\"Properties\":\
             {\"distance\":\"7\",\"persistent\":\"false\",\
             \"waterlogged\":\"false\"}}},\"force_dirt\":false,\
             \"ignore_vines\":true,\"minimum_size\":{\"type\":\
             \"minecraft:two_layers_feature_size\",\"limit\":1,\
             \"lower_size\":0,\"upper_size\":1},\"trunk_placer\":\
             {\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":5,\
             \"height_rand_a\":2,\"height_rand_b\":0},\"trunk_provider\":\
             {\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {\"Name\":\"minecraft:birch_log\",\"Properties\":{\"axis\":\"y\"}}}}}",
        )
        .unwrap();
        let ty = j.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let short = ty.strip_prefix("minecraft:").unwrap_or(ty);
        let FeatureDef::Tree {
            trunk_provider,
            trunk_placer,
            foliage_provider,
            foliage_placer,
            dirt_provider,
            minimum_size,
            decorators,
            ignore_vines,
            force_dirt,
        } = parse_configured_def(&j, short).unwrap()
        else {
            panic!("expected Tree");
        };
        assert!(matches!(
            &trunk_provider,
            StateProvider::Simple(s) if s == "minecraft:birch_log[axis=y]"
        ));
        assert!(matches!(
            &foliage_provider,
            StateProvider::Simple(s)
                if s == "minecraft:birch_leaves[distance=7,persistent=false,waterlogged=false]"
        ));
        assert!(matches!(
            &dirt_provider,
            StateProvider::Simple(s) if s == "minecraft:dirt"
        ));
        assert!(matches!(
            trunk_placer,
            TrunkPlacer::Straight {
                base_height: 5,
                height_rand_a: 2,
                height_rand_b: 0
            }
        ));
        // bare-int radius/offset decode through the IntProvider Either arm.
        assert!(matches!(
            foliage_placer,
            FoliagePlacer::Blob {
                radius: IntProvider::Constant(2),
                offset: IntProvider::Constant(0),
                height: 3
            }
        ));
        assert!(matches!(
            minimum_size,
            FeatureSize::TwoLayers {
                limit: 1,
                lower_size: 0,
                upper_size: 1,
                min_clipped_height: None
            }
        ));
        assert_eq!(decorators.len(), 1);
        assert!(matches!(
            decorators[0],
            TreeDecorator::Beehive {
                probability
            } if probability == 0.002
        ));
        assert!(ignore_vines);
        assert!(!force_dirt);

        // verbatim decorator shapes from swamp_oak (leave_vine 0.25),
        // jungle_tree (cocoa 0.2 + trunk_vine) and oak_leaf_litter
        // (place_on_ground with weighted leaf_litter provider, orElse
        // tries/radius/height omitted in corpus).
        let mk_dec = |inner: &str| {
            let full = format!(
                "{{\"type\":\"minecraft:tree\",\"config\":{{\"decorators\":[{inner}],\
                 \"dirt_provider\":{{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{{\"Name\":\"minecraft:dirt\"}}}},\"foliage_placer\":\
                 {{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
                 \"offset\":0,\"radius\":2}},\"foliage_provider\":{{\"type\":\
                 \"minecraft:simple_state_provider\",\"state\":{{\"Name\":\
                 \"minecraft:oak_leaves\"}}}},\"minimum_size\":{{\"type\":\
                 \"minecraft:two_layers_feature_size\",\"limit\":1,\"lower_size\":0,\
                 \"upper_size\":1}},\"trunk_placer\":{{\"type\":\
                 \"minecraft:straight_trunk_placer\",\"base_height\":4,\
                 \"height_rand_a\":2,\"height_rand_b\":0}},\"trunk_provider\":\
                 {{\"type\":\"minecraft:simple_state_provider\",\"state\":\
                 {{\"Name\":\"minecraft:oak_log\"}}}}}}}}"
            );
            crate::json::parse(&full).unwrap()
        };
        let j2 = mk_dec(
            "{\"type\":\"minecraft:leave_vine\",\"probability\":0.25},\
             {\"type\":\"minecraft:cocoa\",\"probability\":0.2},\
             {\"type\":\"minecraft:trunk_vine\"},\
             {\"type\":\"minecraft:place_on_ground\",\"block_state_provider\":\
             {\"type\":\"minecraft:weighted_state_provider\",\"entries\":[\
             {\"data\":{\"Name\":\"minecraft:leaf_litter\",\"Properties\":\
             {\"facing\":\"north\",\"segment_amount\":\"3\"}},\"weight\":1}]}},\
             {\"type\":\"minecraft:trunk_vine\",\"probability\":1.0}",
        );
        let FeatureDef::Tree { decorators: d2, .. } =
            parse_configured_def(&j2, "tree").unwrap()
        else {
            panic!("expected Tree decorators");
        };
        assert_eq!(d2.len(), 5);
        assert!(matches!(
            d2[0],
            TreeDecorator::LeaveVine {
                probability
            } if probability == 0.25
        ));
        assert!(matches!(
            d2[1],
            TreeDecorator::Cocoa {
                probability
            } if probability == 0.2
        ));
        assert!(matches!(d2[2], TreeDecorator::TrunkVine));
        assert!(matches!(
            d2[3],
            TreeDecorator::PlaceOnGround {
                tries: 128,
                radius: 2,
                height: 1,
                ..
            }
        ));
        assert!(matches!(
            d2[4],
            TreeDecorator::TrunkVine // MapCodec.unit: extra keys ignored
        ));
    }

    #[test]
    fn tree_honest_verdict_labels() {
        // the 23 non-straight+blob tree configs keep verdict FALSE with the
        // precise codec-order label: fancy trunk errors first (fancy_oak).
        let mk = |trunk: &str, foliage: &str, extra: &str| {
            let full = format!(
                "{{\"type\":\"minecraft:tree\",\"config\":{{\"decorators\":[],\
                 \"dirt_provider\":{{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{{\"Name\":\"minecraft:dirt\"}}}},\"foliage_placer\":\
                 {foliage},\"foliage_provider\":{{\"type\":\
                 \"minecraft:simple_state_provider\",\"state\":{{\"Name\":\
                 \"minecraft:oak_leaves\"}}}}{extra},\"minimum_size\":\
                 {{\"type\":\"minecraft:two_layers_feature_size\",\"limit\":1,\
                 \"lower_size\":0,\"upper_size\":1}},\"trunk_placer\":{trunk},\
                 \"trunk_provider\":{{\"type\":\"minecraft:simple_state_provider\",\
                 \"state\":{{\"Name\":\"minecraft:oak_log\"}}}}}}}}"
            );
            crate::json::parse(&full).unwrap()
        };
        let e = parse_configured_def(
            &mk(
                "{\"type\":\"minecraft:fancy_trunk_placer\",\"base_height\":4,\
                 \"height_rand_a\":2,\"height_rand_b\":0}",
                "{\"type\":\"minecraft:fancy_foliage_placer\",\"offset\":0,\
                 \"radius\":1}",
                "",
            ),
            "tree",
        )
        .unwrap_err();
        assert_eq!(e, "unsupported trunk placer fancy_trunk_placer");
        // spruce: trunk parses, the foliage placer owns the label.
        let e2 = parse_configured_def(
            &mk(
                "{\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":6,\
                 \"height_rand_a\":2,\"height_rand_b\":1}",
                "{\"type\":\"minecraft:spruce_foliage_placer\",\"offset\":0,\
                 \"radius\":2,\"trunk_height\":1}",
                "",
            ),
            "tree",
        )
        .unwrap_err();
        assert_eq!(e2, "unsupported foliage placer spruce_foliage_placer");
        // dark_oak: the TRUNK placer errors first (codec order) — the
        // three_layers_feature_size / pale_moss decorators are never
        // reached for the dark_oak family.
        let j3 = mk(
            "{\"type\":\"minecraft:dark_oak_trunk_placer\",\"base_height\":4,\
             \"height_rand_a\":2,\"height_rand_b\":0}",
            "{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":2}",
            "",
        );
        let e3 = parse_configured_def(&j3, "tree").unwrap_err();
        assert_eq!(e3, "unsupported trunk placer dark_oak_trunk_placer");
        // root_placer PRESENT = honest Err (mangrove shapes).
        let j4 = mk(
            "{\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":4,\
             \"height_rand_a\":2,\"height_rand_b\":0}",
            "{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":2}",
            ",\"root_placer\":{\"type\":\"minecraft:mangrove_root_placer\"}",
        );
        assert_eq!(
            parse_configured_def(&j4, "tree").unwrap_err(),
            "unsupported root placer"
        );
        // range honesty: blob radius 17 leaves IntProvider.codec(0,16).
        let j5 = mk(
            "{\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":4,\
             \"height_rand_a\":2,\"height_rand_b\":0}",
            "{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":17}",
            "",
        );
        assert_eq!(
            parse_configured_def(&j5, "tree").unwrap_err(),
            "radius 0..=16, got 17..17"
        );
        // straight base_height 33 leaves intRange(0,32).
        let j6 = mk(
            "{\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":33,\
             \"height_rand_a\":2,\"height_rand_b\":0}",
            "{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":2}",
            "",
        );
        assert_eq!(
            parse_configured_def(&j6, "tree").unwrap_err(),
            "trunk base_height 0..=32, got 33"
        );
        // decorators REQUIRED (fieldOf) — a missing key is an honest error.
        let full7 = format!(
            "{{\"type\":\"minecraft:tree\",\"config\":{{\"dirt_provider\":\
             {{\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {{\"Name\":\"minecraft:dirt\"}}}},\"foliage_placer\":\
             {{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":2}},\"foliage_provider\":{{\"type\":\
             \"minecraft:simple_state_provider\",\"state\":{{\"Name\":\
             \"minecraft:oak_leaves\"}}}},\"minimum_size\":{{\"type\":\
             \"minecraft:two_layers_feature_size\",\"limit\":1,\"lower_size\":0,\
             \"upper_size\":1}},\"trunk_placer\":{{\"type\":\
             \"minecraft:straight_trunk_placer\",\"base_height\":4,\
             \"height_rand_a\":2,\"height_rand_b\":0}},\"trunk_provider\":\
             {{\"type\":\"minecraft:simple_state_provider\",\"state\":\
             {{\"Name\":\"minecraft:oak_log\"}}}}}}}}"
        );
        let j8 = crate::json::parse(&full7).unwrap();
        assert_eq!(
            parse_configured_def(&j8, "tree").unwrap_err(),
            "decorators"
        );
        // ignore_vines defaults to false via orElse — a bare-int blob shape
        // must still parse (both orElse bools absent).
        let j9 = mk(
            "{\"type\":\"minecraft:straight_trunk_placer\",\"base_height\":4,\
             \"height_rand_a\":2,\"height_rand_b\":0}",
            "{\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\
             \"offset\":0,\"radius\":2}",
            "",
        );
        let FeatureDef::Tree {
            ignore_vines,
            force_dirt,
            ..
        } = parse_configured_def(&j9, "tree").unwrap()
        else {
            panic!("expected Tree");
        };
        assert!(!ignore_vines && !force_dirt);
        // beehive probability 1.5 = honest Err through the shared
        // single-field decorator arm (plain literal, single braces).
        let j10 = crate::json::parse(
            "{\"type\":\"minecraft:tree\",\"config\":{\"decorators\":[\n             {\"type\":\"minecraft:beehive\",\"probability\":1.5}],\n             \"dirt_provider\":{\"type\":\"minecraft:simple_state_provider\",\n             \"state\":{\"Name\":\"minecraft:dirt\"}},\"foliage_placer\":\n             {\"type\":\"minecraft:blob_foliage_placer\",\"height\":3,\n             \"offset\":0,\"radius\":2},\"foliage_provider\":{\"type\":\n             \"minecraft:simple_state_provider\",\"state\":{\"Name\":\n             \"minecraft:oak_leaves\"}},\"minimum_size\":{\"type\":\n             \"minecraft:two_layers_feature_size\",\"limit\":1,\"lower_size\":0,\n             \"upper_size\":1},\"trunk_placer\":{\"type\":\n             \"minecraft:straight_trunk_placer\",\"base_height\":4,\n             \"height_rand_a\":2,\"height_rand_b\":0},\"trunk_provider\":\n             {\"type\":\"minecraft:simple_state_provider\",\"state\":\n             {\"Name\":\"minecraft:oak_log\"}}}}",
        )
        .unwrap();
        assert_eq!(
            parse_configured_def(&j10, "tree").unwrap_err(),
            "beehive probability 0..=1, got 1.5"
        );
    }
}
