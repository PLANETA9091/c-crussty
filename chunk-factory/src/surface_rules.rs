//! NCF P2.5 — surface rules engine, decompiled faithfully from Paper 1.21.10:
//!   - SurfaceRules (conditions + rules, LazyCondition epoch caches)
//!   - SurfaceSystem (buildSurface, getSurfaceDepth, getBand / clay bands,
//!     erodedBadlandsExtension, frozenOceanExtension, topMaterial)
//!   - VerticalAnchor (absolute / above_bottom / below_top)
//!
//! IR source (P1.3): the `surface_rule` tree of data/<ns>/worldgen/
//! noise_settings/<name>.json — exactly what SurfaceRules.RuleSource.CODEC
//! decodes (registry keys: bandlands/block/sequence/condition +
//! biome/noise_threshold/vertical_gradient/y_above/water/temperature/steep/
//! not/hole/above_preliminary_surface/stone_depth).
//!
//! Bit-exactness: f32/f64 split exactly as Java; wrapping int math; the lazy
//! condition caches keyed on the epoch counters (lastUpdateXZ/lastUpdateY),
//! which increment per updateXZ/updateY call — NOT per distinct position.
//! Java starts counters at Long.MIN_VALUE and condition epochs at counter-1
//! (wrap to Long.MAX_VALUE); we start counters at 0 and epochs at u64::MAX —
//! identical recompute behaviour (epoch never collides with a live counter).

use crate::biomes::{
    cold_enough_to_snow, get_biome_voted, should_melt_frozen_ocean_iceberg_slightly, BiomeFacts,
    BiomeNoise, BiomeSource,
};
use crate::filler::{HeightmapKind, StateTable};
use crate::jrandom::RandomSource;
use crate::mth;
use crate::router::RandomState;
use crate::{filler::FillerChunk, json};
use std::collections::HashMap;

pub const WHITE_TERRACOTTA: &str = "minecraft:white_terracotta";
pub const ORANGE_TERRACOTTA: &str = "minecraft:orange_terracotta";
pub const TERRACOTTA: &str = "minecraft:terracotta";
pub const YELLOW_TERRACOTTA: &str = "minecraft:yellow_terracotta";
pub const BROWN_TERRACOTTA: &str = "minecraft:brown_terracotta";
pub const RED_TERRACOTTA: &str = "minecraft:red_terracotta";
pub const LIGHT_GRAY_TERRACOTTA: &str = "minecraft:light_gray_terracotta";
pub const PACKED_ICE: &str = "minecraft:packed_ice";
pub const SNOW_BLOCK: &str = "minecraft:snow_block";

/// Context.HOW_FAR_BELOW_PRELIMINARY_SURFACE_LEVEL_TO_BUILD_SURFACE
const HOW_FAR_BELOW: i32 = 8;

// ---------------------------------------------------------------------------
// VerticalAnchor
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum VerticalAnchor {
    Absolute(i32),
    AboveBottom(i32),
    BelowTop(i32),
}

impl VerticalAnchor {
    #[inline]
    pub fn resolve_y(self, min_y: i32, height: i32) -> i32 {
        match self {
            VerticalAnchor::Absolute(y) => y,
            VerticalAnchor::AboveBottom(o) => min_y + o,
            // BelowTop.resolveY: getGenDepth() + getMinGenY() - 1 - offset
            VerticalAnchor::BelowTop(o) => height + min_y - 1 - o,
        }
    }

    pub fn parse(v: &json::Json) -> Result<Self, String> {
        if let Some(i) = v.get("absolute").and_then(|x| x.as_i64()) {
            return Ok(VerticalAnchor::Absolute(i as i32));
        }
        if let Some(i) = v.get("above_bottom").and_then(|x| x.as_i64()) {
            return Ok(VerticalAnchor::AboveBottom(i as i32));
        }
        if let Some(i) = v.get("below_top").and_then(|x| x.as_i64()) {
            return Ok(VerticalAnchor::BelowTop(i as i32));
        }
        Err("vertical anchor: none of absolute/above_bottom/below_top".into())
    }
}

// ---------------------------------------------------------------------------
// IR (parsed JSON tree) — P1.3 surface_rule half
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CaveSurface {
    Ceiling,
    Floor,
}

#[derive(Clone, Debug)]
pub enum CondDef {
    BiomeIs(Vec<String>),
    NoiseThreshold { noise: String, min: f64, max: f64 },
    VerticalGradient { random_name: String, true_at_and_below: VerticalAnchor, false_at_and_above: VerticalAnchor },
    YAbove { anchor: VerticalAnchor, surface_depth_multiplier: i32, add_stone_depth: bool },
    Water { offset: i32, surface_depth_multiplier: i32, add_stone_depth: bool },
    Temperature,
    Steep,
    Not(Box<CondDef>),
    Hole,
    AbovePreliminarySurface,
    StoneDepth { offset: i32, add_surface_depth: bool, secondary_depth_range: i32, surface_type: CaveSurface },
}

#[derive(Clone, Debug)]
pub enum RuleDef {
    Bandlands,
    Block(String),
    Sequence(Vec<RuleDef>),
    Condition { if_true: CondDef, then_run: Box<RuleDef> },
}

pub fn parse_rule(v: &json::Json) -> Result<RuleDef, String> {
    let t = v
        .get("type")
        .and_then(|x| x.as_str())
        .ok_or("surface rule without type")?;
    match t {
        "minecraft:bandlands" => Ok(RuleDef::Bandlands),
        "minecraft:block" => {
            let state = v
                .get("result_state")
                .ok_or("block rule missing result_state")?;
            parse_block_state_rule(state).map(RuleDef::Block)
        }
        "minecraft:sequence" => {
            let arr = v
                .get("sequence")
                .and_then(|x| x.as_arr())
                .ok_or("sequence rule missing sequence")?;
            let mut seq = Vec::with_capacity(arr.len());
            for e in arr {
                seq.push(parse_rule(e)?);
            }
            Ok(RuleDef::Sequence(seq))
        }
        "minecraft:condition" => {
            let if_true = parse_cond(v.get("if_true").ok_or("condition missing if_true")?)?;
            let then_run = Box::new(parse_rule(
                v.get("then_run").ok_or("condition missing then_run")?,
            )?);
            Ok(RuleDef::Condition { if_true, then_run })
        }
        _ => Err(format!("unknown surface rule type {t}")),
    }
}

fn parse_block_state_rule(v: &json::Json) -> Result<String, String> {
    let name = v
        .get("Name")
        .and_then(|x| x.as_str())
        .ok_or("block state missing Name")?;
    let mut props: Vec<(String, String)> = Vec::new();
    if let Some(json::Json::Obj(fields)) = v.get("Properties") {
        for (k, val) in fields {
            if let Some(vs) = val.as_str() {
                props.push((k.clone(), vs.to_string()));
            }
        }
    }
    props.sort();
    let mut s = name.to_string();
    if !props.is_empty() {
        let ps: Vec<String> = props.iter().map(|(k, v)| format!("{k}={v}")).collect();
        s.push_str(&format!("[{}]", ps.join(",")));
    }
    Ok(s)
}

pub fn parse_cond(v: &json::Json) -> Result<CondDef, String> {
    let t = v
        .get("type")
        .and_then(|x| x.as_str())
        .ok_or("condition without type")?;
    Ok(match t {
        "minecraft:biome" => {
            let arr = v
                .get("biome_is")
                .and_then(|x| x.as_arr())
                .ok_or("biome condition missing biome_is")?;
            let mut biomes = Vec::with_capacity(arr.len());
            for b in arr {
                let Some(s) = b.as_str() else {
                    return Err("biome_is entry not a string".into());
                };
                if s.contains(':') {
                    biomes.push(s.to_string());
                } else {
                    biomes.push(format!("minecraft:{s}"));
                }
            }
            CondDef::BiomeIs(biomes)
        }
        "minecraft:noise_threshold" => CondDef::NoiseThreshold {
            noise: v
                .get("noise")
                .and_then(|x| x.as_str())
                .ok_or("noise_threshold missing noise")?
                .to_string(),
            min: v.get("min_threshold").and_then(|x| x.as_f64()).unwrap_or(0.0),
            max: v.get("max_threshold").and_then(|x| x.as_f64()).unwrap_or(0.0),
        },
        // Paper's optionally-flat-bedrock condition: with the default
        // generateFlatBedrock=false it is EXACTLY a vanilla vertical_gradient
        // (Paper OptionallyFlatBedrockConditionSource.apply uses the raw
        // anchors when the paper config flag is off; CI boots default config).
        "minecraft:vertical_gradient" | "paper:optionally_flat_bedrock_condition_source" =>
            CondDef::VerticalGradient {
            random_name: v
                .get("random_name")
                .and_then(|x| x.as_str())
                .ok_or("vertical_gradient missing random_name")?
                .to_string(),
            true_at_and_below: VerticalAnchor::parse(
                v.get("true_at_and_below").ok_or("vertical_gradient missing true_at_and_below")?,
            )?,
            false_at_and_above: VerticalAnchor::parse(
                v.get("false_at_and_above").ok_or("vertical_gradient missing false_at_and_above")?,
            )?,
        },
        "minecraft:y_above" => CondDef::YAbove {
            anchor: VerticalAnchor::parse(v.get("anchor").ok_or("y_above missing anchor")?)?,
            surface_depth_multiplier: v
                .get("surface_depth_multiplier")
                .and_then(|x| x.as_i64())
                .unwrap_or(0) as i32,
            add_stone_depth: matches!(v.get("add_stone_depth"), Some(json::Json::Bool(true))),
        },
        "minecraft:water" => CondDef::Water {
            offset: v.get("offset").and_then(|x| x.as_i64()).unwrap_or(-1) as i32,
            surface_depth_multiplier: v
                .get("surface_depth_multiplier")
                .and_then(|x| x.as_i64())
                .unwrap_or(0) as i32,
            add_stone_depth: matches!(v.get("add_stone_depth"), Some(json::Json::Bool(true))),
        },
        "minecraft:temperature" => CondDef::Temperature,
        "minecraft:steep" => CondDef::Steep,
        "minecraft:not" => CondDef::Not(Box::new(parse_cond(
            v.get("invert").ok_or("not condition missing invert")?,
        )?)),
        "minecraft:hole" => CondDef::Hole,
        "minecraft:above_preliminary_surface" => CondDef::AbovePreliminarySurface,
        "minecraft:stone_depth" => {
            let surface_type = match v.get("surface_type").and_then(|x| x.as_str()) {
                Some("ceiling") => CaveSurface::Ceiling,
                Some("floor") => CaveSurface::Floor,
                other => return Err(format!("stone_depth bad surface_type {other:?}")),
            };
            CondDef::StoneDepth {
                offset: v.get("offset").and_then(|x| x.as_i64()).unwrap_or(0) as i32,
                add_surface_depth: matches!(
                    v.get("add_surface_depth"),
                    Some(json::Json::Bool(true))
                ),
                secondary_depth_range: v
                    .get("secondary_depth_range")
                    .and_then(|x| x.as_i64())
                    .unwrap_or(0) as i32,
                surface_type,
            }
        }
        _ => return Err(format!("unknown surface condition type {t}")),
    })
}

// ---------------------------------------------------------------------------
// Runtime rule tree (per-chunk instances with epoch caches)
// ---------------------------------------------------------------------------

/// Java: condition.lastUpdate = context.counter - 1 (wrap from MIN_VALUE ->
/// MAX_VALUE) — first test always recomputes. We use u64::MAX for the same
/// effect (counters start at 0 and never wrap).
#[derive(Clone, Copy)]
struct LazyCache {
    epoch: u64,
    value: bool,
}

impl LazyCache {
    const FRESH: Self = LazyCache { epoch: u64::MAX, value: false };
}

pub enum Cond {
    // LazyY conditions (epoch = last_update_y)
    BiomeIs { biomes: Vec<String>, cache: std::cell::Cell<LazyCache> },
    StoneDepth { offset: i32, add_surface_depth: bool, secondary_depth_range: i32, ceiling: bool, cache: std::cell::Cell<LazyCache> },
    YAbove { anchor: VerticalAnchor, mult: i32, add_stone_depth: bool, cache: std::cell::Cell<LazyCache> },
    Water { offset: i32, mult: i32, add_stone_depth: bool, cache: std::cell::Cell<LazyCache> },
    Temperature { cache: std::cell::Cell<LazyCache> },
    VerticalGradient { true_y: i32, false_y: i32, factory: Box<dyn crate::jrandom::PositionalRandomFactory>, cache: std::cell::Cell<LazyCache> },
    // LazyXZ conditions (epoch = last_update_xz)
    NoiseThreshold { noise_idx: usize, min: f64, max: f64, cache: std::cell::Cell<LazyCache> },
    Steep { cache: std::cell::Cell<LazyCache> },
    Hole { cache: std::cell::Cell<LazyCache> },
    // Non-lazy
    AbovePreliminarySurface,
    Not(Box<Cond>),
}

pub enum Rule {
    Bandlands,
    Block(String),
    Sequence(Vec<Rule>),
    Test { cond: Cond, followup: Box<Rule> },
}

impl Cond {
    fn reset_caches(&mut self) {
        match self {
            Cond::BiomeIs { cache, .. }
            | Cond::StoneDepth { cache, .. }
            | Cond::YAbove { cache, .. }
            | Cond::Water { cache, .. }
            | Cond::Temperature { cache }
            | Cond::VerticalGradient { cache, .. }
            | Cond::NoiseThreshold { cache, .. }
            | Cond::Steep { cache }
            | Cond::Hole { cache } => cache.set(LazyCache::FRESH),
            Cond::AbovePreliminarySurface => {}
            Cond::Not(inner) => inner.reset_caches(),
        }
    }
}

pub struct SurfaceRuleSet {
    pub root: Rule,
}

impl SurfaceRuleSet {
    /// ruleSource.apply(context) — built per chunk in Java (fresh condition
    /// instances); we build once per RandomState and reset caches per chunk.
    pub fn build(def: &RuleDef, rs: &mut RandomState, dir: &crate::router::WorldgenDir, table: &mut StateTable) -> Result<Self, String> {
        let root = build_rule(def, rs, dir, table)?;
        Ok(SurfaceRuleSet { root })
    }

    pub fn reset_caches(&mut self) {
        self.root.reset_caches();
    }
}

impl Rule {
    fn reset_caches(&mut self) {
        match self {
            Rule::Bandlands | Rule::Block(_) => {}
            Rule::Sequence(rules) => {
                for r in rules {
                    r.reset_caches();
                }
            }
            Rule::Test { cond, followup } => {
                cond.reset_caches();
                followup.reset_caches();
            }
        }
    }
}

fn build_rule(def: &RuleDef, rs: &mut RandomState, dir: &crate::router::WorldgenDir, table: &mut StateTable) -> Result<Rule, String> {
    Ok(match def {
        RuleDef::Bandlands => Rule::Bandlands,
        RuleDef::Block(state) => Rule::Block(state.clone()),
        RuleDef::Sequence(seq) => {
            // Java SequenceRuleSource.apply: a 1-element sequence returns the
            // single rule directly (no SequenceRule wrapper — same semantics
            // for tryApply, keep the wrapper-less form for faithfulness).
            if seq.len() == 1 {
                build_rule(&seq[0], rs, dir, table)?
            } else {
                let mut rules = Vec::with_capacity(seq.len());
                for d in seq {
                    rules.push(build_rule(d, rs, dir, table)?);
                }
                Rule::Sequence(rules)
            }
        }
        RuleDef::Condition { if_true, then_run } => Rule::Test {
            cond: build_cond(if_true, rs, dir, table)?,
            followup: Box::new(build_rule(then_run, rs, dir, table)?),
        },
    })
}

fn build_cond(def: &CondDef, rs: &mut RandomState, dir: &crate::router::WorldgenDir, _table: &mut StateTable) -> Result<Cond, String> {
    Ok(match def {
        CondDef::BiomeIs(biomes) => Cond::BiomeIs { biomes: biomes.clone(), cache: std::cell::Cell::new(LazyCache::FRESH) },
        CondDef::NoiseThreshold { noise, min, max } => {
            let idx = rs.get_or_create_noise(dir, noise)?;
            Cond::NoiseThreshold { noise_idx: idx, min: *min, max: *max, cache: std::cell::Cell::new(LazyCache::FRESH) }
        }
        CondDef::VerticalGradient { random_name, true_at_and_below, false_at_and_above } => {
            // final int i = trueAtAndBelow.resolveY(context.context); etc.
            let true_y = true_at_and_below.resolve_y(rs.settings.min_y, rs.settings.height);
            let false_y = false_at_and_above.resolve_y(rs.settings.min_y, rs.settings.height);
            // getOrCreateRandomFactory(name) = random.fromHashOf(name).forkPositional()
            // — the gradient only calls factory.at(x,y,z), so the boxed trait
            // object suffices (Xoroshiro for overworld, Legacy for legacy).
            let mut src = rs.worldgen_factory.from_hash_of(random_name);
            let factory = src.fork_positional_factory();
            Cond::VerticalGradient { true_y, false_y, factory, cache: std::cell::Cell::new(LazyCache::FRESH) }
        }
        CondDef::YAbove { anchor, surface_depth_multiplier, add_stone_depth } => Cond::YAbove {
            anchor: *anchor,
            mult: *surface_depth_multiplier,
            add_stone_depth: *add_stone_depth,
            cache: std::cell::Cell::new(LazyCache::FRESH),
        },
        CondDef::Water { offset, surface_depth_multiplier, add_stone_depth } => Cond::Water {
            offset: *offset,
            mult: *surface_depth_multiplier,
            add_stone_depth: *add_stone_depth,
            cache: std::cell::Cell::new(LazyCache::FRESH),
        },
        CondDef::Temperature => Cond::Temperature { cache: std::cell::Cell::new(LazyCache::FRESH) },
        CondDef::Steep => Cond::Steep { cache: std::cell::Cell::new(LazyCache::FRESH) },
        CondDef::Not(inner) => Cond::Not(Box::new(build_cond(inner, rs, dir, _table)?)),
        CondDef::Hole => Cond::Hole { cache: std::cell::Cell::new(LazyCache::FRESH) },
        CondDef::AbovePreliminarySurface => Cond::AbovePreliminarySurface,
        CondDef::StoneDepth { offset, add_surface_depth, secondary_depth_range, surface_type } => Cond::StoneDepth {
            offset: *offset,
            add_surface_depth: *add_surface_depth,
            secondary_depth_range: *secondary_depth_range,
            ceiling: *surface_type == CaveSurface::Ceiling,
            cache: std::cell::Cell::new(LazyCache::FRESH),
        },
    })
}

// ---------------------------------------------------------------------------
// SurfaceSystem
// ---------------------------------------------------------------------------

pub struct SurfaceSystem {
    pub surface_noise: usize,
    pub surface_secondary_noise: usize,
    pub clay_bands_offset_noise: usize,
    pub badlands_pillar_noise: usize,
    pub badlands_pillar_roof_noise: usize,
    pub badlands_surface_noise: usize,
    pub iceberg_pillar_noise: usize,
    pub iceberg_pillar_roof_noise: usize,
    pub iceberg_surface_noise: usize,
    /// clayBands[192] as canonical state strings (interned into each
    /// chunk's own table at apply time — the chunk tables are per-chunk)
    pub clay_bands: Vec<String>,
    pub sea_level: i32,
}

impl SurfaceSystem {
    pub fn new(rs: &mut RandomState, dir: &crate::router::WorldgenDir, table: &mut StateTable) -> Result<Self, String> {
        let surface_noise = rs.get_or_create_noise(dir, "minecraft:surface")?;
        let surface_secondary_noise = rs.get_or_create_noise(dir, "minecraft:surface_secondary")?;
        let clay_bands_offset_noise = rs.get_or_create_noise(dir, "minecraft:clay_bands_offset")?;
        let badlands_pillar_noise = rs.get_or_create_noise(dir, "minecraft:badlands_pillar")?;
        let badlands_pillar_roof_noise = rs.get_or_create_noise(dir, "minecraft:badlands_pillar_roof")?;
        let badlands_surface_noise = rs.get_or_create_noise(dir, "minecraft:badlands_surface")?;
        let iceberg_pillar_noise = rs.get_or_create_noise(dir, "minecraft:iceberg_pillar")?;
        let iceberg_pillar_roof_noise = rs.get_or_create_noise(dir, "minecraft:iceberg_pillar_roof")?;
        let iceberg_surface_noise = rs.get_or_create_noise(dir, "minecraft:iceberg_surface")?;
        // clayBands = generateBands(noiseRandom.fromHashOf("minecraft:clay_bands"))
        // (noiseRandom = RandomState.random — the worldgen factory itself).
        let mut band_random = rs.worldgen_factory.from_hash_of("minecraft:clay_bands");
        let clay_bands = generate_bands(&mut *band_random, table);
        Ok(SurfaceSystem {
            surface_noise,
            surface_secondary_noise,
            clay_bands_offset_noise,
            badlands_pillar_noise,
            badlands_pillar_roof_noise,
            badlands_surface_noise,
            iceberg_pillar_noise,
            iceberg_pillar_roof_noise,
            iceberg_surface_noise,
            clay_bands,
            sea_level: rs.settings.sea_level,
        })
    }

    #[inline]
    fn noise_value(rs: &RandomState, idx: usize, x: f64, y: f64, z: f64) -> f64 {
        rs.bank.noises[idx].get_value(x, y, z)
    }

    /// getSurfaceDepth: (int)(noise(x,0,z)*2.75 + 3.0 + at(x,0,z).nextDouble()*0.25)
    pub fn get_surface_depth(&self, rs: &RandomState, x: i32, z: i32) -> i32 {
        let value = Self::noise_value(rs, self.surface_noise, x as f64, 0.0, z as f64);
        let mut jitter_random = rs.worldgen_factory.at(x, 0, z);
        let jitter = jitter_random.next_f64() * 0.25;
        (value * 2.75 + 3.0 + jitter) as i32
    }

    pub fn get_surface_secondary(&self, rs: &RandomState, x: i32, z: i32) -> f64 {
        Self::noise_value(rs, self.surface_secondary_noise, x as f64, 0.0, z as f64)
    }

    /// getBand: clayBands[(y + (int)Math.round(offsetNoise(x,0,z)*4.0) + 192) % 192]
    pub fn get_band(&self, rs: &RandomState, x: i32, y: i32, z: i32) -> String {
        let v = Self::noise_value(rs, self.clay_bands_offset_noise, x as f64, 0.0, z as f64) * 4.0;
        let i = java_math_round_i32(v);
        let idx = (y + i + 192).rem_euclid(192);
        self.clay_bands[idx as usize].clone()
    }
}

/// Math.round(double) with (int) narrowing (values are small; NaN -> 0 kept).
#[inline]
pub fn java_math_round_i32(v: f64) -> i32 {
    if v.is_nan() {
        return 0;
    }
    mth::floor(v + 0.5) as i64 as i32
}

/// SurfaceSystem.generateBands — 192-entry band table. Draw order is
/// load-bearing (T1/T2 semantics). Faithful to the CFR output:
///   orange loop: for (i=0; i<192; ++i) { if ((i += nextInt(5)+1) >= 192) continue; b[i]=ORANGE; }
///   white loop:  for (i2=0, i1=0; i1<ix && i2<192; ++i1, i2 += nextInt(16)+4) { body }
fn generate_bands(random: &mut dyn RandomSource, table: &mut StateTable) -> Vec<String> {
    let _ = table; // the serial form no longer interns band states (P4.7 split)
    let terracotta = TERRACOTTA.to_string();
    let orange = ORANGE_TERRACOTTA.to_string();
    let yellow = YELLOW_TERRACOTTA.to_string();
    let brown = BROWN_TERRACOTTA.to_string();
    let red = RED_TERRACOTTA.to_string();
    let white = WHITE_TERRACOTTA.to_string();
    let light_gray = LIGHT_GRAY_TERRACOTTA.to_string();
    let mut b = vec![terracotta; 192];
    let mut i: i64 = 0;
    while i < 192 {
        i += (random.next_int_bound(5) + 1) as i64;
        if i >= 192 {
            // Java `continue` still runs ++i, but the loop condition then fails
            // identically; i is dead after the loop.
            break;
        }
        b[i as usize] = orange.clone();
        i += 1; // the for-loop ++i
    }
    make_bands(random, &mut b, 1, yellow);
    make_bands(random, &mut b, 2, brown);
    make_bands(random, &mut b, 1, red);
    let ix = next_int_between_inclusive(random, 9, 15);
    let mut i1: i64 = 0;
    let mut i2: i64 = 0;
    while i1 < ix as i64 && i2 < 192 {
        // body
        b[i2 as usize] = white.clone();
        if i2 - 1 > 0 && random.next_boolean() {
            b[(i2 - 1) as usize] = light_gray.clone();
        }
        if i2 + 1 >= 192 || !random.next_boolean() {
            // continue (increments still run below)
        } else {
            b[(i2 + 1) as usize] = light_gray.clone();
        }
        // increment clause: ++i1, i2 += nextInt(16)+4
        i1 += 1;
        i2 += (random.next_int_bound(16) + 4) as i64;
    }
    b
}

fn next_int_between_inclusive(random: &mut dyn RandomSource, min: i32, max: i32) -> i32 {
    min + random.next_int_bound(max - min + 1)
}

fn make_bands(random: &mut dyn RandomSource, bands: &mut [String], min_size: i32, state: String) {
    let random_int = next_int_between_inclusive(random, 6, 15);
    for _ in 0..random_int {
        let i1 = min_size + random.next_int_bound(3);
        let pos = random.next_int_bound(192) as i64;
        let mut i2: i64 = 0;
        while pos + i2 < 192 && i2 < i1 as i64 {
            bands[(pos + i2) as usize] = state.clone();
            i2 += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Chunk column adapter (blockColumn + heightmap queries on FillerChunk)
// ---------------------------------------------------------------------------

pub struct ChunkColumns<'c> {
    pub chunk: &'c mut FillerChunk,
}

impl<'c> ChunkColumns<'c> {
    /// ChunkAccess.getHeight(WORLD_SURFACE_WG, x, z) = getFirstAvailable - 1.
    #[inline]
    pub fn height_wg(&self, x: i32, z: i32) -> i32 {
        self.chunk.heightmaps[1].first_available[(x + z * 16) as usize] - 1
    }

    #[inline]
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u32 {
        self.chunk.block(x, y, z)
    }

    /// ProtoChunk.setBlockState at SURFACE status: write, update the WORLDGEN
    /// heightmaps with the NEW state (heightmapsAfter(SURFACE) = the pair),
    /// markPosForPostprocessing when the state carries a fluid. The two
    /// heightmap updates are order-independent (each reads only its own data).
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, state: u32) {
        if y < self.chunk.min_y || y >= self.chunk.min_y + self.chunk.height {
            return;
        }
        let sec_idx = ((y - self.chunk.min_y) / 16) as usize;
        if sec_idx >= self.chunk.sections.len() {
            return;
        }
        self.chunk.sections[sec_idx].states[crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)] = state;
        let op_ocean = HeightmapKind::OceanFloorWg.is_opaque_state(state, &self.chunk.state_table);
        let op_surface = HeightmapKind::WorldSurfaceWg.is_opaque_state(state, &self.chunk.state_table);
        self.update_hm(0, x, y, z, op_ocean);
        self.update_hm(1, x, y, z, op_surface);
        // fluids: surface rules place no fluids in vanilla trees, but the
        // contract stays: any state whose name is water/lava marks PP.
        let name = self.chunk.state_table.get(state).name.as_str();
        if name == "minecraft:water" || name == "minecraft:lava" {
            let packed = ((x & 15) | ((y & 15) << 4) | ((z & 15) << 8)) as u16;
            self.chunk.post_processing[sec_idx].push(packed);
        }
    }

    fn update_hm(&mut self, hm: usize, x: i32, y: i32, z: i32, opaque: bool) {
        // Heightmap.update receives SECTION-LOCAL x/z (doFill convention)
        let min_y = self.chunk.min_y;
        let x = x & 15;
        let z = z & 15;
        let idx = (x + z * 16) as usize;
        let first_available = self.chunk.heightmaps[hm].first_available[idx];
        if y <= first_available - 2 {
            return;
        }
        if opaque {
            if y >= first_available {
                self.chunk.heightmaps[hm].first_available[idx] = y + 1;
            }
        } else if first_available - 1 == y {
            let mut i = y - 1;
            loop {
                if i < min_y {
                    self.chunk.heightmaps[hm].first_available[idx] = min_y;
                    return;
                }
                let sec_idx = ((i - min_y) / 16) as usize;
                if sec_idx >= self.chunk.sections.len() {
                    i -= 1;
                    continue;
                }
                let s = self.chunk.sections[sec_idx].states
                    [crate::filler::SectionData::block_index(x & 15, i & 15, z & 15)];
                let is_opq = if hm == 0 {
                    HeightmapKind::OceanFloorWg.is_opaque_state(s, &self.chunk.state_table)
                } else {
                    HeightmapKind::WorldSurfaceWg.is_opaque_state(s, &self.chunk.state_table)
                };
                if is_opq {
                    self.chunk.heightmaps[hm].first_available[idx] = i + 1;
                    return;
                }
                i -= 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SurfaceRules.Context — epoch-driven state
// ---------------------------------------------------------------------------

pub struct SurfaceContext<'a> {
    pub system: &'a SurfaceSystem,
    pub rs: &'a RandomState,
    pub biome_noise: &'a BiomeNoise,
    pub facts: &'a HashMap<String, BiomeFacts>,
    pub source: &'a std::cell::RefCell<BiomeSource<'a>>,
    pub zoom_seed: i64,
    pub min_y: i32,
    pub height: i32,
    pub default_block: u32,

    pub last_update_xz: u64,
    pub last_update_y: u64,
    pub block_x: i32,
    pub block_z: i32,
    pub surface_depth: i32,
    // lastSurfaceDepth2Update (init: lastUpdateXZ - 1)
    pub last_surface_depth2_update: u64,
    pub surface_secondary: f64,
    pub last_min_surface_level_update: u64,
    pub min_surface_level: i32,
    pub last_preliminary_surface_cell_origin: i64,
    pub preliminary_surface_cache: [i32; 4],
    // per-updateY state
    pub biome: Option<String>,
    pub block_y: i32,
    pub water_height: i32,
    pub stone_depth_below: i32,
    pub stone_depth_above: i32,
}

impl<'a> SurfaceContext<'a> {
    pub fn new(
        system: &'a SurfaceSystem,
        rs: &'a RandomState,
        biome_noise: &'a BiomeNoise,
        facts: &'a HashMap<String, BiomeFacts>,
        source: &'a std::cell::RefCell<BiomeSource<'a>>,
        zoom_seed: i64,
    ) -> Self {
        SurfaceContext {
            system,
            rs,
            biome_noise,
            facts,
            source,
            zoom_seed,
            min_y: rs.settings.min_y,
            height: rs.settings.height,
            default_block: 0,
            last_update_xz: 0,
            last_update_y: 0,
            block_x: 0,
            block_z: 0,
            surface_depth: 0,
            last_surface_depth2_update: u64::MAX, // lastUpdateXZ - 1 (wrap)
            surface_secondary: 0.0,
            last_min_surface_level_update: u64::MAX,
            min_surface_level: 0,
            last_preliminary_surface_cell_origin: i64::MAX,
            preliminary_surface_cache: [0; 4],
            biome: None,
            block_y: 0,
            water_height: i32::MIN,
            stone_depth_below: 0,
            stone_depth_above: 0,
        }
    }

    /// Context.updateXZ — call sites increment the counters FIRST.
    pub fn update_xz(&mut self, block_x: i32, block_z: i32) {
        self.last_update_xz = self.last_update_xz.wrapping_add(1);
        self.last_update_y = self.last_update_y.wrapping_add(1);
        self.block_x = block_x;
        self.block_z = block_z;
        self.surface_depth = self.system.get_surface_depth(self.rs, block_x, block_z);
    }

    /// Context.updateY
    pub fn update_y(&mut self, stone_depth_above: i32, stone_depth_below: i32, water_height: i32, block_x: i32, block_y: i32, block_z: i32) {
        self.last_update_y = self.last_update_y.wrapping_add(1);
        // biome = Suppliers.memoize(() -> biomeGetter.apply(pos.set(...)))
        let biome = get_biome_voted(
            &mut self.source.borrow_mut(),
            self.zoom_seed,
            block_x,
            block_y,
            block_z,
        );
        self.biome = Some(biome);
        self.block_y = block_y;
        self.water_height = water_height;
        self.stone_depth_below = stone_depth_below;
        self.stone_depth_above = stone_depth_above;
        let _ = block_x;
        let _ = block_z;
    }

    /// Context.getSurfaceSecondary (cached per lastUpdateXZ)
    pub fn get_surface_secondary(&mut self) -> f64 {
        if self.last_surface_depth2_update != self.last_update_xz {
            self.last_surface_depth2_update = self.last_update_xz;
            self.surface_secondary =
                self.system.get_surface_secondary(self.rs, self.block_x, self.block_z);
        }
        self.surface_secondary
    }

    /// NoiseChunk.preliminarySurfaceLevel(x, z): snap to quart blocks
    /// (QuartPos.toBlock(QuartPos.fromBlock(x)) = (x>>2)<<2), floor the
    /// preliminary_surface_level router field at y=0.
    fn preliminary_surface_level(&self, x: i32, z: i32) -> i32 {
        let qx = (x >> 2) << 2;
        let qz = (z >> 2) << 2;
        let v = self
            .rs
            .router
            .preliminary_surface_level
            .compute(&self.rs.bank, qx, 0, qz);
        mth::floor(v)
    }

    /// Context.getMinSurfaceLevel (16-block surface cells, f32 lerp2)
    pub fn get_min_surface_level(&mut self) -> i32 {
        if self.last_min_surface_level_update != self.last_update_xz {
            self.last_min_surface_level_update = self.last_update_xz;
            let cell_x = self.block_x >> 4;
            let cell_z = self.block_z >> 4;
            let packed = chunk_as_long(cell_x, cell_z);
            if self.last_preliminary_surface_cell_origin != packed {
                self.last_preliminary_surface_cell_origin = packed;
                self.preliminary_surface_cache[0] = self.preliminary_surface_level(cell_x << 4, cell_z << 4);
                self.preliminary_surface_cache[1] = self.preliminary_surface_level((cell_x + 1) << 4, cell_z << 4);
                self.preliminary_surface_cache[2] = self.preliminary_surface_level(cell_x << 4, (cell_z + 1) << 4);
                self.preliminary_surface_cache[3] = self.preliminary_surface_level((cell_x + 1) << 4, (cell_z + 1) << 4);
            }
            let l = mth::lerp2(
                ((self.block_x & 0xF) as f32 / 16.0f32) as f64,
                ((self.block_z & 0xF) as f32 / 16.0f32) as f64,
                self.preliminary_surface_cache[0] as f64,
                self.preliminary_surface_cache[1] as f64,
                self.preliminary_surface_cache[2] as f64,
                self.preliminary_surface_cache[3] as f64,
            );
            // Java: Mth.floor(Mth.lerp2((float)...)) — the float casts happen
            // at the ARGUMENTS (verified line 754), lerp2 itself runs on the
            // int values as (float)(...) casts. Mth.lerp2(float delta1, float
            // delta2, ...) — DELTAS are floats, values ints widened to double
            // in the varargs? lerp2 signature: (double d, double e, double f,
            // double g, double h, double i). The call casts deltas to float,
            // then floor. Our lerp2 takes f64 — pass float-cast deltas.
            self.min_surface_level = mth::floor(l) + self.surface_depth - HOW_FAR_BELOW;
        }
        self.min_surface_level
    }
}

fn chunk_as_long(x: i32, z: i32) -> i64 {
    (x as i64 & 0xFFFF_FFFF) | ((z as i64 & 0xFFFF_FFFF) << 32)
}

// ---------------------------------------------------------------------------
// Condition / rule evaluation
// ---------------------------------------------------------------------------

impl Cond {
    fn test(&self, ctx: &mut SurfaceContext, chunk: &ChunkColumns) -> bool {
        match self {
            Cond::BiomeIs { biomes, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let v = ctx.biome.as_ref().map(|b| biomes.contains(b)).unwrap_or(false);
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::StoneDepth { offset, add_surface_depth, secondary_depth_range, ceiling, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let i = if *ceiling { ctx.stone_depth_below } else { ctx.stone_depth_above };
                let i1 = if *add_surface_depth { ctx.surface_depth } else { 0 };
                let i2 = if *secondary_depth_range == 0 {
                    0
                } else {
                    let mapped = mth::map(ctx.get_surface_secondary(), -1.0, 1.0, 0.0, *secondary_depth_range as f64);
                    mapped as i32
                };
                let v = i <= 1 + offset + i1 + i2;
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::YAbove { anchor, mult, add_stone_depth, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let anchor_y = anchor.resolve_y(ctx.min_y, ctx.height);
                let v = ctx.block_y + (if *add_stone_depth { ctx.stone_depth_above } else { 0 })
                    >= anchor_y + ctx.surface_depth * mult;
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::Water { offset, mult, add_stone_depth, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let v = ctx.water_height == i32::MIN
                    || ctx.block_y + (if *add_stone_depth { ctx.stone_depth_above } else { 0 })
                        >= ctx.water_height + offset + ctx.surface_depth * mult;
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::Temperature { cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let biome = ctx.biome.clone().unwrap_or_default();
                let v = ctx
                    .facts
                    .get(&biome)
                    .map(|f| {
                        cold_enough_to_snow(ctx.biome_noise, f, ctx.block_x, ctx.block_y, ctx.block_z, ctx.system.sea_level)
                    })
                    .unwrap_or(false);
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::VerticalGradient { true_y, false_y, factory, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_y {
                    return c.value;
                }
                let y = ctx.block_y;
                let v = if y <= *true_y {
                    true
                } else if y >= *false_y {
                    false
                } else {
                    let d = mth::map(y as f64, *true_y as f64, *false_y as f64, 1.0, 0.0);
                    let mut r = factory.at(ctx.block_x, y, ctx.block_z);
                    (r.next_f32() as f64) < d
                };
                cache.set(LazyCache { epoch: ctx.last_update_y, value: v });
                v
            }
            Cond::NoiseThreshold { noise_idx, min, max, cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_xz {
                    return c.value;
                }
                let value = ctx.rs.bank.noises[*noise_idx].get_value(ctx.block_x as f64, 0.0, ctx.block_z as f64);
                let v = value >= *min && value <= *max;
                cache.set(LazyCache { epoch: ctx.last_update_xz, value: v });
                v
            }
            Cond::Steep { cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_xz {
                    return c.value;
                }
                let i = ctx.block_x & 0xF;
                let i1 = ctx.block_z & 0xF;
                let max = std::cmp::max(i1 - 1, 0);
                let min = std::cmp::min(i1 + 1, 15);
                let height = chunk.height_wg(i, max);
                let height1 = chunk.height_wg(i, min);
                let v = if height1 >= height + 4 {
                    true
                } else {
                    let max1 = std::cmp::max(i - 1, 0);
                    let min1 = std::cmp::min(i + 1, 15);
                    let height2 = chunk.height_wg(max1, i1);
                    let height3 = chunk.height_wg(min1, i1);
                    height2 >= height3 + 4
                };
                cache.set(LazyCache { epoch: ctx.last_update_xz, value: v });
                v
            }
            Cond::Hole { cache } => {
                let c = cache.get();
                if c.epoch == ctx.last_update_xz {
                    return c.value;
                }
                let v = ctx.surface_depth <= 0;
                cache.set(LazyCache { epoch: ctx.last_update_xz, value: v });
                v
            }
            Cond::AbovePreliminarySurface => ctx.block_y >= ctx.get_min_surface_level(),
            Cond::Not(inner) => !inner.test(ctx, chunk),
        }
    }
}

impl Rule {
    #[inline]
    pub fn try_apply(&self, ctx: &mut SurfaceContext, chunk: &ChunkColumns) -> Option<String> {
        match self {
            Rule::Block(state) => Some(state.clone()),
            Rule::Bandlands => Some(
                ctx.system
                    .get_band(ctx.rs, ctx.block_x, ctx.block_y, ctx.block_z),
            ),
            Rule::Sequence(rules) => {
                for r in rules {
                    if let Some(s) = r.try_apply(ctx, chunk) {
                        return Some(s);
                    }
                }
                None
            }
            Rule::Test { cond, followup } => {
                if cond.test(ctx, chunk) {
                    followup.try_apply(ctx, chunk)
                } else {
                    None
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SurfaceSystem.buildSurface
// ---------------------------------------------------------------------------

/// SurfaceSystem.buildSurface — full column walk. `probe_biome` semantics:
/// biomeManager.getBiome(pos.set(x, useLegacy ? 0 : height+1, z)).
#[allow(clippy::too_many_arguments)]
pub fn build_surface(
    ctx: &mut SurfaceContext,
    rule: &Rule,
    chunk: &mut ChunkColumns,
    default_block: u32,
) {
    let min_block_x = chunk.chunk.min_y; // placeholder, real X/Z from chunk pos
    let _ = min_block_x;
    for i in 0..16i32 {
        for i1 in 0..16i32 {
            let x = chunk.chunk.chunk_min_x + i;
            let z = chunk.chunk.chunk_min_z + i1;
            // int i4 = chunk.getHeight(WORLD_SURFACE_WG, i, i1) + 1;
            let i4 = chunk.height_wg(i, i1) + 1;
            // biome probe at (x, useLegacy ? 0 : i4, z) — overworld: i4
            let probe = get_biome_voted(
                &mut ctx.source.borrow_mut(),
                ctx.zoom_seed,
                x,
                i4,
                z,
            );
            let probe_frozen = matches!(probe.as_str(), "minecraft:frozen_ocean" | "minecraft:deep_frozen_ocean");
            let probe_badlands = probe == "minecraft:eroded_badlands";
            if probe_badlands {
                eroded_badlands_extension(ctx, chunk, x, z, i4, default_block);
            }
            // int i5 = chunk.getHeight(WORLD_SURFACE_WG, i, i1) + 1; (re-read!)
            let i5 = chunk.height_wg(i, i1) + 1;
            ctx.update_xz(x, z);
            let mut i6 = 0i32;
            let mut i7 = i32::MIN;
            let mut i8 = i32::MAX;
            let min_y = chunk.chunk.min_y;
            let mut y = i5;
            while y >= min_y {
                let block = chunk.get_block(x, y, z);
                if is_air_id(block, &chunk.chunk.state_table) {
                    i6 = 0;
                    i7 = i32::MIN;
                    y -= 1;
                    continue;
                }
                if !is_air_id(block, &chunk.chunk.state_table) && is_fluid_id(block, &chunk.chunk.state_table) {
                    if i7 != i32::MIN {
                        y -= 1;
                        continue;
                    }
                    i7 = y + 1;
                    y -= 1;
                    continue;
                }
                if i8 >= y {
                    // scan down for the stone run bottom
                    i8 = -32512;
                    let mut i10 = y - 1;
                    while i10 >= min_y - 1 {
                        let below = chunk.get_block(x, i10, z);
                        if is_stone_id(below, &chunk.chunk.state_table) {
                            i10 -= 1;
                            continue;
                        }
                        i8 = i10 + 1;
                        break;
                    }
                }
                let stone_depth_below = y - i8 + 1;
                i6 += 1;
                ctx.update_y(i6, stone_depth_below, i7, x, y, z);
                if block == default_block {
                    if let Some(new_state) = rule.try_apply(ctx, chunk) {
                        let id = chunk.chunk.state_table.intern_canonical(&new_state);
                        chunk.set_block(x, y, z, id);
                    }
                }
                y -= 1;
            }
            if probe_frozen {
                let min_surface = ctx.get_min_surface_level();
                frozen_ocean_extension(ctx, chunk, &probe, x, z, i4, min_surface, default_block);
            }
        }
    }
}

#[inline]
pub fn is_air_state(state: u32, table: &StateTable) -> bool {
    is_air_id(state, table)
}

#[inline]
pub fn is_fluid_state(state: u32, table: &StateTable) -> bool {
    is_fluid_id(state, table)
}

#[inline]
pub fn is_stone_state(state: u32, table: &StateTable) -> bool {
    is_stone_id(state, table)
}

#[inline]
fn is_air_id(state: u32, table: &StateTable) -> bool {
    matches!(
        table.get(state).name.as_str(),
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
    )
}

#[inline]
fn is_fluid_id(state: u32, table: &StateTable) -> bool {
    matches!(table.get(state).name.as_str(), "minecraft:water" | "minecraft:lava")
}

#[inline]
fn is_stone_id(state: u32, table: &StateTable) -> bool {
    // SurfaceSystem.isStone: !isAir && fluidState.isEmpty()
    !is_air_id(state, table) && !is_fluid_id(state, table)
}

/// SurfaceSystem.erodedBadlandsExtension
fn eroded_badlands_extension(
    ctx: &mut SurfaceContext,
    chunk: &mut ChunkColumns,
    x: i32,
    z: i32,
    height: i32,
    default_block: u32,
) {
    let min = f64::min(
        (SurfaceSystem::noise_value(ctx.rs, ctx.system.badlands_surface_noise, x as f64, 0.0, z as f64) * 8.25).abs(),
        SurfaceSystem::noise_value(
            ctx.rs,
            ctx.system.badlands_pillar_noise,
            x as f64 * 0.2,
            0.0,
            z as f64 * 0.2,
        ) * 15.0,
    );
    if min <= 0.0 {
        return;
    }
    let abs = (SurfaceSystem::noise_value(
        ctx.rs,
        ctx.system.badlands_pillar_roof_noise,
        x as f64 * 0.75,
        0.0,
        z as f64 * 0.75,
    ) * 1.5)
        .abs();
    let d3 = 64.0 + f64::min(min * min * 2.5, abs.ceil() * 50.0 + 24.0);
    let floor = mth::floor(d3);
    if height > floor {
        return;
    }
    let min_y = chunk.chunk.min_y;
    // first loop: walk down while the block is NOT the default BLOCK (name
    // compare — Java .is(defaultBlock.getBlock()) ignores properties); bail
    // out entirely on WATER.
    {
        let default_name = chunk.chunk.state_table.get(default_block).name.clone();
        let mut i = floor;
        while i >= min_y {
            let block = chunk.get_block(x, i, z);
            let name = chunk.chunk.state_table.get(block).name.as_str();
            if name == default_name {
                break;
            }
            if name == "minecraft:water" {
                return;
            }
            i -= 1;
        }
    }
    // second loop: fill air with default block down from floor
    let mut i = floor;
    while i >= min_y {
        let block = chunk.get_block(x, i, z);
        if !is_air_id(block, &chunk.chunk.state_table) {
            break;
        }
        chunk.set_block(x, i, z, default_block);
        i -= 1;
    }
}

/// SurfaceSystem.frozenOceanExtension — probe biome is the column biome.
#[allow(clippy::too_many_arguments)]
fn frozen_ocean_extension(
    ctx: &mut SurfaceContext,
    chunk: &mut ChunkColumns,
    probe_biome: &str,
    x: i32,
    z: i32,
    height: i32,
    min_surface_level: i32,
    _default_block: u32,
) {
    let min = f64::min(
        (SurfaceSystem::noise_value(ctx.rs, ctx.system.iceberg_surface_noise, x as f64, 0.0, z as f64) * 8.25).abs(),
        SurfaceSystem::noise_value(
            ctx.rs,
            ctx.system.iceberg_pillar_noise,
            x as f64 * 1.28,
            0.0,
            z as f64 * 1.28,
        ) * 15.0,
    );
    if min <= 1.8 {
        return;
    }
    let abs = (SurfaceSystem::noise_value(
        ctx.rs,
        ctx.system.iceberg_pillar_roof_noise,
        x as f64 * 1.17,
        0.0,
        z as f64 * 1.17,
    ) * 1.5)
        .abs();
    let mut min1 = f64::min(min * min * 1.2, abs.ceil() * 40.0 + 14.0);
    // biome.shouldMeltFrozenOceanIcebergSlightly(pos.set(x, seaLevel, z), seaLevel)
    let facts = ctx.facts.get(probe_biome);
    let melts = facts
        .map(|f| {
            should_melt_frozen_ocean_iceberg_slightly(
                ctx.biome_noise,
                f,
                x,
                ctx.system.sea_level,
                z,
                ctx.system.sea_level,
            )
        })
        .unwrap_or(false);
    if melts {
        min1 -= 2.0;
    }
    let d3;
    if min1 > 2.0 {
        d3 = ctx.system.sea_level as f64 - min1 - 7.0;
        min1 += ctx.system.sea_level as f64;
    } else {
        min1 = 0.0;
        d3 = 0.0;
    }
    let d4 = min1;
    let mut random = ctx.rs.worldgen_factory.at(x, 0, z);
    let i = 2 + random.next_int_bound(4);
    let i1 = ctx.system.sea_level + 18 + random.next_int_bound(10);
    let mut i2 = 0i32;
    let snow_block_id = chunk.chunk.state_table.intern(SNOW_BLOCK, &[]);
    let packed_ice_id = chunk.chunk.state_table.intern(PACKED_ICE, &[]);
    let mut max = std::cmp::max(height, (min1 as i32) + 1);
    // Java loop condition:
    //   if (!(isAir && max < (int)d4 && nextDouble() > 0.01)
    //       && (!isWater || max <= (int)d3 || max >= seaLevel || d3 == 0.0
    //           || !(nextDouble() > 0.15))) continue;
    // i.e. setBlock only when !C1 && C2; the second nextDouble() draws only
    // when isWater && max > (int)d3 && max < seaLevel && d3 != 0.0.
    while max >= min_surface_level {
        let block = chunk.get_block(x, max, z);
        let is_air = is_air_id(block, &chunk.chunk.state_table);
        let c1 = is_air && max < (d4 as i32) && random.next_f64() > 0.01;
        let name = chunk.chunk.state_table.get(block).name.as_str();
        let is_water = name == "minecraft:water";
        let c2_draw_path = is_water
            && max > (d3 as i32)
            && max < ctx.system.sea_level
            && d3 != 0.0
            && random.next_f64() > 0.15;
        if !c1 && !c2_draw_path {
            if i2 <= i && max > i1 {
                chunk.set_block(x, max, z, snow_block_id);
                i2 += 1;
            } else {
                chunk.set_block(x, max, z, packed_ice_id);
            }
        }
        max -= 1;
    }
}

// ---------------------------------------------------------------------------
// topMaterial (carver dirt-fix) — SurfaceSystem.topMaterial decompiled
// ---------------------------------------------------------------------------

/// CarvingContext.topMaterial(biomeGetter, chunk, pos, hasFluid): a FRESH
/// SurfaceRules.Context, updateXZ(x, z), updateY(1, 1, hasFluid ? y+1 : MIN,
/// x, y, z), then tryApply. Returns the replacement state (or None).
pub fn top_material(
    rule: &Rule,
    ctx: &mut SurfaceContext,
    chunk: &ChunkColumns,
    x: i32,
    y: i32,
    z: i32,
    has_fluid: bool,
) -> Option<String> {
    ctx.update_xz(x, z);
    ctx.update_y(1, 1, if has_fluid { y + 1 } else { i32::MIN }, x, y, z);
    rule.try_apply(ctx, chunk)
}
