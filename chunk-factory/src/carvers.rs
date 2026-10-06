//! NCF P2.8 — carvers, decompiled faithfully from Paper 1.21.10:
//!   - WorldCarver (carveEllipsoid, carveBlock, getCarveState, canReach)
//!   - CaveWorldCarver (carve/createRoom/createTunnel/shouldSkip/getThickness)
//!   - CanyonWorldCarver (carve/doCarve/initWidthFactors/updateVerticalRadius/
//!     shouldSkip)
//!   - CarvingMask (BitSet, idx = (x&15) | (z&15)<<4 | (y-minY)<<8)
//!   - NoiseBasedChunkGenerator.applyCarvers — the 17x17 neighbour walk,
//!     per-carver setLargeFeatureSeed(seed + carverIndex, chunkX, chunkZ),
//!     ONE aquifer (the CENTER chunk's) shared across every carver.
//!   - ProtoChunk.setBlockState at CARVERS: the four FINAL heightmaps are
//!     primed (full top-down scan) on the first write and updated per write;
//!     the WORLDGEN pair stays frozen at surface values.
//!
//! Carve fluid = Aquifer.computeSubstance(ctx, 0.0): null (substance > 0)
//! cancels the carve at that block; FluidStatus.at returns plain AIR above
//! fluid levels (verified — not CAVE_AIR).
//!
//! Mth.sin/cos are the 65536-entry TABLE functions (block-step precision) —
//! ported in mth.rs; carver tunnel geometry depends on them.

use crate::aquifer::NoiseBasedAquifer;
use crate::biomes::{BiomeFacts, BiomeNoise};
use crate::filler::{FillerChunk, StateTable};
use crate::jrandom::{LegacyRandomSource, RandomSource};
use crate::mth;
use crate::surface_rules::{top_material, ChunkColumns, SurfaceContext, SurfaceSystem};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// Config IR (parsed configured_carver JSON)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum FloatProviderDef {
    Constant(f32),
    Uniform { min: f32, max: f32 },
    ClampedNormal { mean: f32, deviation: f32, min: f32, max: f32 },
    Trapezoid { min: f32, max: f32, plateau: f32 },
}

impl FloatProviderDef {
    pub fn parse(v: &crate::json::Json) -> Result<Self, String> {
        // bare number = constant (canyon.json yScale: 3.0 — T19-style trap)
        if let Some(f) = v.as_f64() {
            return Ok(FloatProviderDef::Constant(f as f32));
        }
        let t = v
            .get("type")
            .and_then(|x| x.as_str())
            .ok_or("float provider without type")?;
        Ok(match t {
            "minecraft:constant" => FloatProviderDef::Constant(
                v.get("value").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
            ),
            "minecraft:uniform" => FloatProviderDef::Uniform {
                min: v.get("min_inclusive").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                max: v.get("max_exclusive").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
            },
            "minecraft:clamped_normal" => FloatProviderDef::ClampedNormal {
                mean: v.get("mean").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                deviation: v.get("deviation").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                min: v.get("min").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                max: v.get("max").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
            },
            "minecraft:trapezoid" => FloatProviderDef::Trapezoid {
                min: v.get("min").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                max: v.get("max").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
                plateau: v.get("plateau").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
            },
            _ => return Err(format!("unknown float provider {t}")),
        })
    }

    pub fn sample(&self, random: &mut dyn RandomSource) -> f32 {
        match *self {
            FloatProviderDef::Constant(v) => v,
            FloatProviderDef::Uniform { min, max } => mth::random_between(random, min, max),
            // ClampedNormalFloat is not referenced by the vanilla overworld
            // carver configs; the loud failure keeps silent divergence out.
            FloatProviderDef::ClampedNormal { .. } => {
                panic!("clamped_normal float provider: not used by the gated vanilla carvers")
            }
            FloatProviderDef::Trapezoid { min, max, plateau } => {
                mth::trapezoid_float(random, min, max, plateau)
            }
        }
    }
}

/// HeightProvider — carver `y`.
#[derive(Clone, Copy, Debug)]
pub enum HeightProviderDef {
    Constant(i32),
    Uniform { min: AnchorDef, max: AnchorDef },
}

#[derive(Clone, Copy, Debug)]
pub enum AnchorDef {
    Absolute(i32),
    AboveBottom(i32),
    BelowTop(i32),
}

impl AnchorDef {
    fn resolve(self, min_y: i32, height: i32) -> i32 {
        match self {
            AnchorDef::Absolute(y) => y,
            AnchorDef::AboveBottom(o) => min_y + o,
            AnchorDef::BelowTop(o) => height + min_y - 1 - o,
        }
    }
}

impl HeightProviderDef {
    pub fn parse(v: &crate::json::Json) -> Result<Self, String> {
        let t = v
            .get("type")
            .and_then(|x| x.as_str())
            .ok_or("height provider without type")?;
        let parse_anchor = |j: Option<&crate::json::Json>| -> Result<AnchorDef, String> {
            let j = j.ok_or("height provider missing anchor")?;
            if let Some(i) = j.get("absolute").and_then(|x| x.as_i64()) {
                return Ok(AnchorDef::Absolute(i as i32));
            }
            if let Some(i) = j.get("above_bottom").and_then(|x| x.as_i64()) {
                return Ok(AnchorDef::AboveBottom(i as i32));
            }
            if let Some(i) = j.get("below_top").and_then(|x| x.as_i64()) {
                return Ok(AnchorDef::BelowTop(i as i32));
            }
            Err("bad height anchor".into())
        };
        Ok(match t {
            "minecraft:constant" => HeightProviderDef::Constant(parse_anchor(v.get("value"))?.resolve(0, 0)),
            "minecraft:uniform" => HeightProviderDef::Uniform {
                min: parse_anchor(v.get("min_inclusive"))?,
                max: parse_anchor(v.get("max_inclusive"))?,
            },
            // biased_to_bottom / very_biased_to_bottom / trapezoid are not
            // referenced by the vanilla overworld carver configs (all three
            // use uniform); loud failure keeps silent divergence out.
            other => panic!("height provider {other}: not used by the gated vanilla carvers"),
        })
    }

    pub fn sample(&self, random: &mut dyn RandomSource, min_y: i32, height: i32) -> i32 {
        match *self {
            HeightProviderDef::Constant(v) => v,
            HeightProviderDef::Uniform { min, max } => {
                mth::random_between_inclusive(random, min.resolve(min_y, height), max.resolve(min_y, height))
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CarverKind {
    Cave,
    Canyon,
}

#[derive(Clone, Debug)]
pub struct CarverConfig {
    pub kind: CarverKind,
    pub probability: f32,
    pub y: HeightProviderDef,
    pub y_scale: FloatProviderDef,
    pub lava_level: AnchorDef,
    /// expanded replaceable block names (tags resolved by the caller)
    pub replaceable: Vec<String>,
    // cave-specific
    pub horizontal_radius_multiplier: Option<FloatProviderDef>,
    pub vertical_radius_multiplier: Option<FloatProviderDef>,
    pub floor_level: Option<FloatProviderDef>,
    // canyon-specific
    pub vertical_rotation: Option<FloatProviderDef>,
    pub shape_distance_factor: Option<FloatProviderDef>,
    pub shape_horizontal_radius_factor: Option<FloatProviderDef>,
    pub shape_thickness: Option<FloatProviderDef>,
    pub shape_vertical_radius_default_factor: f32,
    pub shape_vertical_radius_center_factor: f32,
    pub shape_width_smoothness: i32,
}

pub fn parse_configured_carver(v: &crate::json::Json, expand_tag: &dyn Fn(&str) -> Result<Vec<String>, String>) -> Result<CarverConfig, String> {
    let t = v
        .get("type")
        .and_then(|x| x.as_str())
        .ok_or("configured carver without type")?;
    let kind = match t {
        "minecraft:cave" => CarverKind::Cave,
        "minecraft:canyon" => CarverKind::Canyon,
        other => return Err(format!("unsupported carver type {other}")),
    };
    let c = v.get("config").ok_or("configured carver missing config")?;
    let replaceable_ref = c
        .get("replaceable")
        .and_then(|x| x.as_str())
        .ok_or("carver missing replaceable")?;
    let replaceable = expand_tag(replaceable_ref)?;
    let lava_anchor = c.get("lava_level").ok_or("carver missing lava_level")?;
    let lava_level = if let Some(i) = lava_anchor.get("above_bottom").and_then(|x| x.as_i64()) {
        AnchorDef::AboveBottom(i as i32)
    } else if let Some(i) = lava_anchor.get("absolute").and_then(|x| x.as_i64()) {
        AnchorDef::Absolute(i as i32)
    } else if let Some(i) = lava_anchor.get("below_top").and_then(|x| x.as_i64()) {
        AnchorDef::BelowTop(i as i32)
    } else {
        return Err("bad lava_level anchor".into());
    };
    Ok(CarverConfig {
        kind,
        probability: c.get("probability").and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
        y: HeightProviderDef::parse(c.get("y").ok_or("carver missing y")?)?,
        y_scale: FloatProviderDef::parse(c.get("yScale").ok_or("carver missing yScale")?)?,
        lava_level,
        replaceable,
        horizontal_radius_multiplier: c
            .get("horizontal_radius_multiplier")
            .map(FloatProviderDef::parse)
            .transpose()?,
        vertical_radius_multiplier: c
            .get("vertical_radius_multiplier")
            .map(FloatProviderDef::parse)
            .transpose()?,
        floor_level: c.get("floor_level").map(FloatProviderDef::parse).transpose()?,
        vertical_rotation: c
            .get("vertical_rotation")
            .map(FloatProviderDef::parse)
            .transpose()?,
        shape_distance_factor: c
            .get("shape")
            .and_then(|s| s.get("distance_factor"))
            .map(FloatProviderDef::parse)
            .transpose()?,
        shape_horizontal_radius_factor: c
            .get("shape")
            .and_then(|s| s.get("horizontal_radius_factor"))
            .map(FloatProviderDef::parse)
            .transpose()?,
        shape_thickness: c
            .get("shape")
            .and_then(|s| s.get("thickness"))
            .map(FloatProviderDef::parse)
            .transpose()?,
        shape_vertical_radius_default_factor: c
            .get("shape")
            .and_then(|s| s.get("vertical_radius_default_factor"))
            .and_then(|x| x.as_f64())
            .unwrap_or(1.0) as f32,
        shape_vertical_radius_center_factor: c
            .get("shape")
            .and_then(|s| s.get("vertical_radius_center_factor"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0) as f32,
        shape_width_smoothness: c
            .get("shape")
            .and_then(|s| s.get("width_smoothness"))
            .and_then(|x| x.as_i64())
            .unwrap_or(3) as i32,
    })
}

// ---------------------------------------------------------------------------
// CarvingMask
// ---------------------------------------------------------------------------

pub struct CarvingMask {
    pub min_y: i32,
    bits: Vec<u64>,
}

impl CarvingMask {
    pub fn new(height: i32, min_y: i32) -> Self {
        let n = (256 * height as usize).div_ceil(64);
        CarvingMask { min_y, bits: vec![0u64; n] }
    }

    #[inline]
    fn index(&self, x: i32, y: i32, z: i32) -> usize {
        (((x & 0xF) | ((z & 0xF) << 4) | ((y - self.min_y) << 8)) as usize) & !(1usize << 63)
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32) {
        let idx = self.index(x, y, z);
        self.bits[idx / 64] |= 1u64 << (idx % 64);
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> bool {
        let idx = self.index(x, y, z);
        (self.bits[idx / 64] >> (idx % 64)) & 1 == 1
    }
}

// ---------------------------------------------------------------------------
// Carver runtime
// ---------------------------------------------------------------------------

pub struct CarvingContext {
    pub min_gen_y: i32,
    pub gen_depth: i32,
}

/// Immutable kit for one applyCarvers run.
pub struct CarverKit<'a> {
    /// configured carver ref -> parsed config ("minecraft:cave" -> cfg)
    pub configs_by_ref: HashMap<String, CarverConfig>,
    pub rs: &'a crate::router::RandomState,
    pub system: &'a SurfaceSystem,
    pub rule_root: &'a crate::surface_rules::Rule,
    pub facts: &'a HashMap<String, BiomeFacts>,
    pub biome_noise: &'a BiomeNoise,
    pub min_gen_y: i32,
    pub gen_depth: i32,
    pub air: u32,
    pub water: u32,
    pub lava: u32,
}

/// Mutable per-run state.
pub struct CarveState<'c, 'k> {
    pub chunk: &'c mut FillerChunk,
    pub mask: CarvingMask,
    pub aquifer: NoiseBasedAquifer<'k>,
    pub final_heightmaps_ready: bool,
    /// per-section non-air block counts (Java LevelChunkSection
    /// nonEmptyBlockCount) for the hasOnlyAir early-return.
    pub section_non_empty: Vec<u32>,
}

impl<'c, 'k> CarveState<'c, 'k> {
    pub fn new(chunk: &'c mut FillerChunk, aquifer: NoiseBasedAquifer<'k>) -> Self {
        let chunk_height = chunk.height;
        let chunk_min_y = chunk.min_y;
        let section_non_empty = chunk
            .sections
            .iter()
            .map(|sec| {
                sec.states
                    .iter()
                    .filter(|&&s| {
                        !matches!(
                            chunk.state_table.get(s).name.as_str(),
                            "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
                        )
                    })
                    .count() as u32
            })
            .collect();
        CarveState {
            chunk,
            mask: CarvingMask::new(chunk_height, chunk_min_y),
            aquifer,
            final_heightmaps_ready: false,
            section_non_empty,
        }
    }

    /// ProtoChunk.setBlockState at CARVERS semantics: write; prime the FINAL
    /// heightmaps on first use; update all four per write; WG pair frozen.
    fn set_block_carvers(&mut self, x: i32, y: i32, z: i32, state: u32) {
        if y < self.chunk.min_y || y >= self.chunk.min_y + self.chunk.height {
            return;
        }
        let sec_idx = ((y - self.chunk.min_y) / 16) as usize;
        if sec_idx >= self.chunk.sections.len() {
            return;
        }
        let placing_air = is_air_name(self.chunk.state_table.get(state).name.as_str());
        if self.section_non_empty[sec_idx] == 0 && placing_air {
            // ProtoChunk: hasOnlyAir && state.is(AIR) -> return
            return;
        }
        let prev = self.chunk.sections[sec_idx].states
            [crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)];
        let prev_air = is_air_name(self.chunk.state_table.get(prev).name.as_str());
        self.chunk.sections[sec_idx].states
            [crate::filler::SectionData::block_index(x & 15, y & 15, z & 15)] = state;
        // maintain nonEmptyBlockCount
        if prev_air && !placing_air {
            self.section_non_empty[sec_idx] += 1;
        } else if !prev_air && placing_air {
            self.section_non_empty[sec_idx] -= 1;
        }
        if !self.final_heightmaps_ready {
            prime_final_heightmaps(self.chunk);
            self.final_heightmaps_ready = true;
        }
        update_final_heightmap(self.chunk, 2, x, y, z, state); // OCEAN_FLOOR
        update_final_heightmap(self.chunk, 3, x, y, z, state); // WORLD_SURFACE
        update_final_heightmap(self.chunk, 4, x, y, z, state); // MOTION_BLOCKING
        update_final_heightmap(self.chunk, 5, x, y, z, state); // MOTION_BLOCKING_NO_LEAVES
    }
}

#[inline]
fn is_air_name(name: &str) -> bool {
    matches!(name, "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")
}

#[inline]
fn blocks_motion(name: &str) -> bool {
    !matches!(name, "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "minecraft:water" | "minecraft:lava")
}

#[inline]
fn is_fluid_name(name: &str) -> bool {
    name == "minecraft:water" || name == "minecraft:lava"
}

/// Heightmap.primeHeightmaps over the four FINAL types, scanning top-down;
/// a column stops updating a type once its predicate matched (removal
/// semantics). AIR-only skip: the first NON-AIR block is the probe.
fn prime_final_heightmaps(chunk: &mut FillerChunk) {
    // extend the heightmaps vec to 6 slots (2 WG + 4 FINAL), init minY
    while chunk.heightmaps.len() < 6 {
        chunk.heightmaps.push(crate::filler::HeightmapData::new(
            final_kind(chunk.heightmaps.len()),
            chunk.min_y,
        ));
    }
    let top = chunk.min_y + chunk.height;
    let mut remaining: [bool; 4] = [true; 4]; // OCEAN_FLOOR, WORLD_SURFACE, MB, MB_NO_LEAVES
    for x in 0..16i32 {
        for z in 0..16i32 {
            remaining = [true; 4];
            let mut y = top - 1;
            while y >= chunk.min_y {
                let state = chunk.block(x, y, z);
                let name = chunk.state_table.get(state).name.as_str();
                if name == "minecraft:air" {
                    y -= 1;
                    continue;
                }
                // probe block found: for each remaining type whose predicate
                // matches, set height = y + 1 and remove from the list
                let bm = blocks_motion(name);
                let fl = is_fluid_name(name);
                let preds = [bm, true, bm || fl, bm || fl];
                for (k, &pred) in preds.iter().enumerate() {
                    if remaining[k] && pred {
                        let idx = (x + z * 16) as usize;
                        chunk.heightmaps[2 + k].first_available[idx] = y + 1;
                        remaining[k] = false;
                    }
                }
                if !remaining.iter().any(|&r| r) {
                    break;
                }
                y -= 1;
            }
            // types that never matched stay at minY (unprimed default)
            let idx = (x + z * 16) as usize;
            for k in 0..4 {
                if remaining[k] {
                    chunk.heightmaps[2 + k].first_available[idx] = chunk.min_y;
                }
            }
        }
    }
}

fn final_kind(idx: usize) -> crate::filler::HeightmapKind {
    // FINAL types use the same predicate machinery; the WG enum only carries
    // the two WORLDGEN kinds, so FINAL slots reuse the closest semantics via
    // explicit predicate functions in update_final_heightmap/prime.
    if idx % 2 == 0 {
        crate::filler::HeightmapKind::OceanFloorWg
    } else {
        crate::filler::HeightmapKind::WorldSurfaceWg
    }
}

/// Heightmap.update for one of the four FINAL types (predicate inline).
fn update_final_heightmap(chunk: &mut FillerChunk, hm: usize, x: i32, y: i32, z: i32, state: u32) {
    let name = chunk.state_table.get(state).name.as_str();
    let is_opaque: bool = match hm {
        2 => blocks_motion(name),                            // OCEAN_FLOOR
        3 => !is_air_name(name),                             // WORLD_SURFACE
        4 => blocks_motion(name) || is_fluid_name(name),     // MOTION_BLOCKING
        _ => blocks_motion(name) || is_fluid_name(name),   // MB_NO_LEAVES
    };
    let min_y = chunk.min_y;
    let idx = (x + z * 16) as usize;
    let first_available = chunk.heightmaps[hm].first_available[idx];
    if y <= first_available - 2 {
        return;
    }
    if is_opaque {
        if y >= first_available {
            chunk.heightmaps[hm].first_available[idx] = y + 1;
        }
    } else if first_available - 1 == y {
        let mut i = y - 1;
        loop {
            if i < min_y {
                chunk.heightmaps[hm].first_available[idx] = min_y;
                return;
            }
            let s = chunk.block(x, i, z);
            let sname = chunk.state_table.get(s).name.as_str();
            let opq: bool = match hm {
                2 => blocks_motion(sname),
                3 => !is_air_name(sname),
                4 => blocks_motion(sname) || is_fluid_name(sname),
                _ => blocks_motion(sname) || is_fluid_name(sname),
            };
            if opq {
                chunk.heightmaps[hm].first_available[idx] = i + 1;
                return;
            }
            i -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// WorldCarver core
// ---------------------------------------------------------------------------

impl<'a> CarverKit<'a> {
    /// WorldCarver.canReach — verbatim.
    fn can_reach(&self, chunk: &FillerChunk, x: f64, z: f64, branch_index: i32, branch_count: i32, width: f32) -> bool {
        let d = (chunk.chunk_min_x + 8) as f64;
        let d2 = x - d;
        let d1 = (chunk.chunk_min_z + 8) as f64;
        let d3 = z - d1;
        let d4 = (branch_count - branch_index) as f64;
        let d5 = (width + 2.0f32 + 16.0f32) as f64;
        d2 * d2 + d3 * d3 - d4 * d4 <= d5 * d5
    }

    /// WorldCarver.carveEllipsoid — verbatim.
    #[allow(clippy::too_many_arguments)]
    fn carve_ellipsoid(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        x: f64,
        y: f64,
        z: f64,
        horizontal_radius: f64,
        vertical_radius: f64,
        skip: &dyn Fn(f64, f64, f64, i32) -> bool,
    ) -> bool {
        let pos_x = state.chunk.chunk_min_x;
        let pos_z = state.chunk.chunk_min_z;
        let d = (pos_x + 8) as f64;
        let d1 = (pos_z + 8) as f64;
        let d2 = 16.0 + horizontal_radius * 2.0;
        if (x - d).abs() > d2 || (z - d1).abs() > d2 {
            return false;
        }
        let max = std::cmp::max(mth::floor(x - horizontal_radius) - pos_x - 1, 0);
        let min = std::cmp::min(mth::floor(x + horizontal_radius) - pos_x, 15);
        let max1 = std::cmp::max(mth::floor(y - vertical_radius) - 1, self.min_gen_y + 1);
        let upgrade_offset = 7i32; // chunk.isUpgrading() ? 0 : 7 — fresh worlds: 7
        let min1 = std::cmp::min(
            mth::floor(y + vertical_radius) + 1,
            self.min_gen_y + self.gen_depth - 1 - upgrade_offset,
        );
        let max2 = std::cmp::max(mth::floor(z - horizontal_radius) - pos_z - 1, 0);
        let min2 = std::cmp::min(mth::floor(z + horizontal_radius) - pos_z, 15);
        let mut flag = false;
        for i1 in max..=min {
            let block_x = pos_x + i1;
            let d3 = (block_x as f64 + 0.5 - x) / horizontal_radius;
            for i2 in max2..=min2 {
                let block_z = pos_z + i2;
                let d4 = (block_z as f64 + 0.5 - z) / horizontal_radius;
                if d3 * d3 + d4 * d4 >= 1.0 {
                    continue;
                }
                // MutableBoolean per column (created inside the i2 loop)
                let mut reached = false;
                for i3 in (max1 + 1..=min1).rev() {
                    let d5 = (i3 as f64 - 0.5 - y) / vertical_radius;
                    if skip(d3, d5, d4, i3) || state.mask.get(i1, i3, i2) {
                        continue;
                    }
                    state.mask.set(i1, i3, i2);
                    if self.carve_block(config, ctx, state, block_x, i3, block_z, &mut reached) {
                        flag = true;
                    }
                }
            }
        }
        flag
    }

    /// WorldCarver.carveBlock — verbatim.
    fn carve_block(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        x: i32,
        y: i32,
        z: i32,
        reached_surface: &mut bool,
    ) -> bool {
        let block_state = state.chunk.block(x, y, z);
        let name = state.chunk.state_table.get(block_state).name.clone();
        if name == "minecraft:grass_block" || name == "minecraft:mycelium" {
            *reached_surface = true;
        }
        if !config.replaceable.contains(&name) {
            return false;
        }
        let carve_state = {
            let lava_y = config.lava_level.resolve(self.min_gen_y, self.gen_depth);
            if y <= lava_y {
                Some(self.lava)
            } else {
                state.aquifer.compute_substance(x, y, z, 0.0, self.air, self.water, self.lava)
            }
        };
        let Some(carve_state) = carve_state else {
            return false;
        };
        let carve_name = state.chunk.state_table.get(carve_state).name.clone();
        let carve_is_fluid = is_fluid_name(&carve_name);
        state.set_block_carvers(x, y, z, carve_state);
        let sched = state.aquifer.should_schedule_fluid_update();
        if sched && carve_is_fluid {
            let packed = ((x & 15) | ((y & 15) << 4) | ((z & 15) << 8)) as u16;
            let sec_idx = ((y - state.chunk.min_y) / 16) as usize;
            if sec_idx < state.chunk.post_processing.len() {
                state.chunk.post_processing[sec_idx].push(packed);
            }
        }
        if *reached_surface {
            // checkPos = pos.below(); if DIRT -> topMaterial fix
            let below = state.chunk.block(x, y - 1, z);
            let below_name = state.chunk.state_table.get(below).name.as_str();
            if below_name == "minecraft:dirt" {
                let replacement = {
                    let mut cols = ChunkColumns { chunk: state.chunk };
                    let r = top_material(self.rule_root, ctx, &cols, x, y - 1, z, carve_is_fluid);
                    r
                };
                if let Some(new_state) = replacement {
                    let id = state.chunk.state_table.intern_canonical(&new_state);
                    let n = state.chunk.state_table.get(id).name.clone();
                    state.set_block_carvers(x, y - 1, z, id);
                    if is_fluid_name(&n) {
                        let packed = ((x & 15) | (((y - 1) & 15) << 4) | ((z & 15) << 8)) as u16;
                        let sec_idx = (((y - 1) - state.chunk.min_y) / 16) as usize;
                        if sec_idx < state.chunk.post_processing.len() {
                            state.chunk.post_processing[sec_idx].push(packed);
                        }
                    }
                }
            }
        }
        true
    }

    /// CaveWorldCarver.carve — verbatim.
    #[allow(clippy::too_many_arguments)]
    fn carve_cave(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        random: &mut dyn RandomSource,
        chunk_pos_x: i32,
        chunk_pos_z: i32,
    ) {
        let block_pos_coord = (4 * 2 - 1) * 16;
        let a = random.next_int_bound(15) + 1;
        let b = random.next_int_bound(a) + 1;
        let random_int = random.next_int_bound(b);
        for _ in 0..random_int {
            let d = ((chunk_pos_x * 16) + random.next_int_bound(16)) as f64;
            let d1 = config.y.sample(random, self.min_gen_y, self.gen_depth) as f64;
            let d2 = ((chunk_pos_z * 16) + random.next_int_bound(16)) as f64;
            let d3 = config
                .horizontal_radius_multiplier
                .unwrap_or(FloatProviderDef::Constant(1.0))
                .sample(random) as f64;
            let d4 = config
                .vertical_radius_multiplier
                .unwrap_or(FloatProviderDef::Constant(1.0))
                .sample(random) as f64;
            let d5 = config
                .floor_level
                .unwrap_or(FloatProviderDef::Constant(-1.0))
                .sample(random) as f64;
            let mut i1 = 1i32;
            if random.next_int_bound(4) == 0 {
                let d6 = config.y_scale.sample(random) as f64;
                let f = 1.0f32 + random.next_f32() * 6.0f32;
                // createRoom: d = 1.5 + sin(1.5707964f) * radius; d1 = d * ratio
                let dd = 1.5 + (mth::sin_f32(1.5707964f32) as f64) * f as f64;
                let dd1 = dd * d6;
                let skip = |rx: f64, ry: f64, rz: f64, _yy: i32| -> bool {
                    // CaveWorldCarver.shouldSkip
                    ry <= d5 || rx * rx + ry * ry + rz * rz >= 1.0
                };
                self.carve_ellipsoid(config, ctx, state, d + 1.0, d1, d2, dd, dd1, &skip);
                i1 += random.next_int_bound(4);
            }
            for _ in 0..i1 {
                let f1 = random.next_f32() * (std::f32::consts::PI * 2.0);
                let f = (random.next_f32() - 0.5f32) / 4.0f32;
                let thickness = self.get_thickness(random);
                let i3 = block_pos_coord - random.next_int_bound(block_pos_coord / 4);
                self.create_tunnel(
                    config, ctx, state, random.next_long(), d, d1, d2, d3, d4, thickness, f1, f,
                    0, i3, 1.0, d5,
                );
            }
        }
    }

    /// CaveWorldCarver.getThickness — verbatim.
    fn get_thickness(&self, random: &mut dyn RandomSource) -> f32 {
        let mut f = random.next_f32() * 2.0f32 + random.next_f32();
        if random.next_int_bound(10) == 0 {
            f *= random.next_f32() * random.next_f32() * 3.0f32 + 1.0f32;
        }
        f
    }

    /// CaveWorldCarver.createTunnel — verbatim (branching + reach check).
    #[allow(clippy::too_many_arguments)]
    fn create_tunnel(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        seed: i64,
        mut x: f64,
        mut y: f64,
        mut z: f64,
        horizontal_radius_multiplier: f64,
        vertical_radius_multiplier: f64,
        thickness: f32,
        mut yaw: f32,
        mut pitch: f32,
        branch_index: i32,
        branch_count: i32,
        horizontal_vertical_ratio: f64,
        floor_level: f64,
    ) {
        let mut random = LegacyRandomSource::new(seed);
        let i = random.next_int_bound(branch_count / 2) + branch_count / 4;
        let flag = random.next_int_bound(6) == 0;
        let mut f = 0.0f32;
        let mut f1 = 0.0f32;
        let mut i1 = branch_index;
        while i1 < branch_count {
            let d = 1.5
                + (mth::sin_f32(std::f32::consts::PI * i1 as f32 / branch_count as f32) as f64)
                    * thickness as f64;
            let d1 = d * horizontal_vertical_ratio;
            let cos = mth::cos_f32(pitch);
            x += (mth::cos_f32(yaw) * cos) as f64;
            y += mth::sin_f32(pitch) as f64;
            z += (mth::sin_f32(yaw) * cos) as f64;
            pitch *= if flag { 0.92f32 } else { 0.7f32 };
            pitch += f1 * 0.1f32;
            yaw += f * 0.1f32;
            f1 *= 0.9f32;
            f *= 0.75f32;
            f1 += (random.next_f32() - random.next_f32()) * random.next_f32() * 2.0f32;
            f += (random.next_f32() - random.next_f32()) * random.next_f32() * 4.0f32;
            if i1 == i && thickness > 1.0f32 {
                self.create_tunnel(
                    config, ctx, state, random.next_long(), x, y, z, horizontal_radius_multiplier,
                    vertical_radius_multiplier, random.next_f32() * 0.5f32 + 0.5f32,
                    yaw - 1.5707964f32, pitch / 3.0f32, i1, branch_count, 1.0, floor_level,
                );
                self.create_tunnel(
                    config, ctx, state, random.next_long(), x, y, z, horizontal_radius_multiplier,
                    vertical_radius_multiplier, random.next_f32() * 0.5f32 + 0.5f32,
                    yaw + 1.5707964f32, pitch / 3.0f32, i1, branch_count, 1.0, floor_level,
                );
                return;
            }
            if random.next_int_bound(4) == 0 {
                i1 += 1;
                continue;
            }
            if !self.can_reach(state.chunk, x, z, i1, branch_count, thickness) {
                return;
            }
            let skip = |rx: f64, ry: f64, rz: f64, _yy: i32| -> bool {
                ry <= floor_level || rx * rx + ry * ry + rz * rz >= 1.0
            };
            self.carve_ellipsoid(
                config, ctx, state, x, y, z, d * horizontal_radius_multiplier,
                d1 * vertical_radius_multiplier, &skip,
            );
            i1 += 1;
        }
    }

    /// CanyonWorldCarver.carve — verbatim.
    #[allow(clippy::too_many_arguments)]
    fn carve_canyon(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        random: &mut dyn RandomSource,
        chunk_pos_x: i32,
        chunk_pos_z: i32,
    ) {
        let i = (4 * 2 - 1) * 16;
        let d = ((chunk_pos_x * 16) + random.next_int_bound(16)) as f64;
        let i1 = config.y.sample(random, self.min_gen_y, self.gen_depth);
        let d1 = ((chunk_pos_z * 16) + random.next_int_bound(16)) as f64;
        let f = random.next_f32() * (std::f32::consts::PI * 2.0);
        let f1 = config
            .vertical_rotation
            .unwrap_or(FloatProviderDef::Constant(0.0))
            .sample(random);
        let d2 = config.y_scale.sample(random) as f64;
        let f2 = config
            .shape_thickness
            .unwrap_or(FloatProviderDef::Constant(1.0))
            .sample(random);
        let i2 = ((i as f32)
            * config
                .shape_distance_factor
                .unwrap_or(FloatProviderDef::Constant(1.0))
                .sample(random)) as i32;
        self.do_carve(config, ctx, state, random.next_long(), d, i1 as f64, d1, f2, f, f1, 0, i2, d2);
    }

    /// CanyonWorldCarver.doCarve — verbatim.
    #[allow(clippy::too_many_arguments)]
    fn do_carve(
        &self,
        config: &CarverConfig,
        ctx: &mut SurfaceContext,
        state: &mut CarveState,
        seed: i64,
        mut x: f64,
        mut y: f64,
        mut z: f64,
        thickness: f32,
        mut yaw: f32,
        mut pitch: f32,
        branch_index: i32,
        branch_count: i32,
        horizontal_vertical_ratio: f64,
    ) {
        let mut random = LegacyRandomSource::new(seed);
        let width_factors = self.init_width_factors(config, &mut random);
        let mut f = 0.0f32;
        let mut f1 = 0.0f32;
        let mut i = branch_index;
        while i < branch_count {
            let mut d = 1.5
                + (mth::sin_f32((i as f32) * std::f32::consts::PI / branch_count as f32) as f64)
                    * thickness as f64;
            let mut d1 = d * horizontal_vertical_ratio;
            d *= config
                .shape_horizontal_radius_factor
                .unwrap_or(FloatProviderDef::Constant(1.0))
                .sample(&mut random) as f64;
            d1 = self.update_vertical_radius(config, &mut random, d1, branch_count as f32, i as f32);
            let cos = mth::cos_f32(pitch);
            let sin = mth::sin_f32(pitch);
            x += (mth::cos_f32(yaw) * cos) as f64;
            y += sin as f64;
            z += (mth::sin_f32(yaw) * cos) as f64;
            pitch *= 0.7f32;
            pitch += f1 * 0.05f32;
            yaw += f * 0.05f32;
            f1 *= 0.8f32;
            f *= 0.5f32;
            f1 += (random.next_f32() - random.next_f32()) * random.next_f32() * 2.0f32;
            f += (random.next_f32() - random.next_f32()) * random.next_f32() * 4.0f32;
            if random.next_int_bound(4) == 0 {
                i += 1;
                continue;
            }
            if !self.can_reach(state.chunk, x, z, i, branch_count, thickness) {
                return;
            }
            let min_gen_y = self.min_gen_y;
            let wf = width_factors.clone();
            let skip = move |rx: f64, ry: f64, rz: f64, yy: i32| -> bool {
                // CanyonWorldCarver.shouldSkip: i = y - minGenY;
                // (rx^2 + rz^2) * widthFactors[i - 1] + ry^2 / 6.0 >= 1.0
                let idx = (yy - min_gen_y - 1) as isize;
                let w = if idx >= 0 && (idx as usize) < wf.len() {
                    wf[idx as usize]
                } else {
                    1.0
                };
                (rx * rx + rz * rz) * (w as f64) + ry * ry / 6.0 >= 1.0
            };
            self.carve_ellipsoid(config, ctx, state, x, y, z, d, d1, &skip);
            i += 1;
        }
    }

    /// CanyonWorldCarver.initWidthFactors — verbatim.
    fn init_width_factors(&self, config: &CarverConfig, random: &mut dyn RandomSource) -> Vec<f32> {
        let gen_depth = self.gen_depth as usize;
        let mut floats = vec![1.0f32; gen_depth];
        let mut f = 1.0f32;
        for i in 0..gen_depth {
            if i == 0 || random.next_int_bound(config.shape_width_smoothness) == 0 {
                f = 1.0f32 + random.next_f32() * random.next_f32();
            }
            floats[i] = f * f;
        }
        floats
    }

    /// CanyonWorldCarver.updateVerticalRadius — verbatim (f32 throughout).
    fn update_vertical_radius(
        &self,
        config: &CarverConfig,
        random: &mut dyn RandomSource,
        vertical_radius: f64,
        branch_count: f32,
        current_branch: f32,
    ) -> f64 {
        let f = 1.0f32 - mth::abs_f32(0.5f32 - current_branch / branch_count) * 2.0f32;
        let f1 = config.shape_vertical_radius_default_factor
            + config.shape_vertical_radius_center_factor * f;
        (f1 as f64) * vertical_radius * (mth::random_between(random, 0.75f32, 1.0f32) as f64)
    }
}

// ---------------------------------------------------------------------------
// NoiseBasedChunkGenerator.applyCarvers — the 17x17 driver
// ---------------------------------------------------------------------------

/// applyCarvers for one center chunk. `configs_by_ref` maps configured-carver
/// refs ("minecraft:cave") to parsed configs; `biome_carvers(nx, nz)` returns
/// the NEIGHBOR chunk corner biome's carver ref list (DIRECT climate sample,
/// no BiomeManager vote) whose INDEX drives the seed; ONE aquifer (the center
/// chunk's) is shared across every carver; the SurfaceContext is shared for
/// topMaterial dirt fixes.
#[allow(clippy::too_many_arguments)]
pub fn apply_carvers(
    chunk: &mut FillerChunk,
    seed: i64,
    kit: &CarverKit,
    ctx: &mut SurfaceContext,
    center_aquifer: NoiseBasedAquifer,
    biome_carvers: &dyn Fn(i32, i32) -> Vec<String>,
) {
    let mut state = CarveState::new(chunk, center_aquifer);
    // WorldgenRandom over LegacyRandomSource(RandomSupport.generateUniqueSeed())
    // — the unique seed is irrelevant: setLargeFeatureSeed reseeds per carver.
    let mut worldgen_random = LegacyRandomSource::new(0);
    let pos_x = state.chunk.chunk_min_x / 16;
    let pos_z = state.chunk.chunk_min_z / 16;
    for i1 in -8..=8i32 {
        for i2 in -8..=8i32 {
            let neighbor_x = pos_x + i1;
            let neighbor_z = pos_z + i2;
            let carver_refs = biome_carvers(neighbor_x, neighbor_z);
            for (i3, reference) in carver_refs.iter().enumerate() {
                worldgen_random.set_large_feature_seed(seed + i3 as i64, neighbor_x, neighbor_z);
                let Some(config) = kit.configs_by_ref.get(reference) else {
                    continue; // unsupported carver ref — cannot happen on the gated corpus
                };
                match config.kind {
                    CarverKind::Cave => {
                        if worldgen_random.next_f32() <= config.probability {
                            kit.carve_cave(config, ctx, &mut state, &mut worldgen_random, neighbor_x, neighbor_z);
                        }
                    }
                    CarverKind::Canyon => {
                        if worldgen_random.next_f32() <= config.probability {
                            kit.carve_canyon(config, ctx, &mut state, &mut worldgen_random, neighbor_x, neighbor_z);
                        }
                    }
                }
            }
        }
    }
}

