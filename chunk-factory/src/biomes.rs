//! NCF P2.5 — biome facts the surface rules and carvers need:
//!   1. climate settings per biome (temperature, temperature_modifier) parsed
//!      from data/<ns>/worldgen/biome/*.json;
//!   2. Biome.warmEnoughToRain / coldEnoughToSnow /
//!      shouldMeltFrozenOceanIcebergSlightly with the height-adjusted
//!      temperature (TEMPERATURE_NOISE above seaLevel+17) and the FROZEN
//!      temperature modifier (FROZEN_TEMPERATURE_NOISE + BIOME_INFO_NOISE);
//!   3. BiomeManager.getBiome — the 8-neighbour fiddled-distance vote
//!      (obfuscateSeed = sha256 of the level seed; LinearCongruentialGenerator
//!      fiddle), resolving through the climate sampler (quart in, block out).
//!
//! All semantics decompiled from Paper 1.21.10 (BiomeManager.java, Biome.java,
//! Biome$TemperatureModifier.java, LinearCongruentialGenerator.java).

use crate::climate::ParameterList;
use crate::router::RandomState;
use crate::sha256::obfuscate_seed;
use crate::simplex::PerlinSimplexNoise;
use crate::{jrandom, mth};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Per-biome climate facts
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TemperatureModifier {
    None,
    Frozen,
}

#[derive(Clone)]
pub struct BiomeFacts {
    /// biome registry name ("minecraft:plains")
    pub name: String,
    pub temperature: f32,
    pub temperature_modifier: TemperatureModifier,
    /// biome "carvers" list — configured_carver refs in JSON order
    pub carvers: Vec<String>,
}

/// Load facts for every data/<ns>/worldgen/biome/*.json in the tree.
pub fn load_biome_facts(dir: &crate::router::WorldgenDir) -> Result<HashMap<String, BiomeFacts>, String> {
    let mut out = HashMap::new();
    for ns in dir.namespaces() {
        for name in dir.list(&ns, "biome") {
            let text = dir
                .read(&ns, "biome", &name)
                .ok_or_else(|| format!("biome read failed: {ns}:{name}"))?;
            let j = crate::json::parse(&text).map_err(|e| e.to_string())?;
            let full = format!("minecraft:{name}");
            let temperature = j
                .get("temperature")
                .and_then(|x| x.as_f64())
                .unwrap_or(0.5) as f32;
            let temperature_modifier = match j.get("temperature_modifier").and_then(|x| x.as_str()) {
                Some("frozen") => TemperatureModifier::Frozen,
                _ => TemperatureModifier::None,
            };
            let mut carvers = Vec::new();
            if let Some(arr) = j.get("carvers").and_then(|x| x.as_arr()) {
                for c in arr {
                    if let Some(s) = c.as_str() {
                        carvers.push(expand_rl(s));
                    }
                }
            }
            out.insert(
                full,
                BiomeFacts {
                    name: format!("{ns}:{name}"),
                    temperature,
                    temperature_modifier,
                    carvers,
                },
            );
        }
    }
    Ok(out)
}

fn expand_rl(s: &str) -> String {
    if s.contains(':') {
        s.to_string()
    } else {
        format!("minecraft:{s}")
    }
}

// ---------------------------------------------------------------------------
// Static noise fields (Biome.java lines 94-97)
// ---------------------------------------------------------------------------

pub struct BiomeNoise {
    /// TEMPERATURE_NOISE = PerlinSimplexNoise(WorldgenRandom(Legacy(1234)), [0])
    pub temperature_noise: PerlinSimplexNoise,
    /// FROZEN_TEMPERATURE_NOISE = ... Legacy(3456), [-2,-1,0]
    pub frozen_temperature_noise: PerlinSimplexNoise,
    /// BIOME_INFO_NOISE = ... Legacy(2345), [0]
    pub biome_info_noise: PerlinSimplexNoise,
}

impl BiomeNoise {
    pub fn new() -> Self {
        let mut r1 = jrandom::LegacyRandomSource::new(1234);
        let temperature_noise = PerlinSimplexNoise::new(&mut r1, &[0]);
        let mut r2 = jrandom::LegacyRandomSource::new(3456);
        let frozen_temperature_noise = PerlinSimplexNoise::new(&mut r2, &[-2, -1, 0]);
        let mut r3 = jrandom::LegacyRandomSource::new(2345);
        let biome_info_noise = PerlinSimplexNoise::new(&mut r3, &[0]);
        BiomeNoise {
            temperature_noise,
            frozen_temperature_noise,
            biome_info_noise,
        }
    }
}

impl Default for BiomeNoise {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Biome temperature semantics (f32 exactly as decompiled)
// ---------------------------------------------------------------------------

/// Biome.TemperatureModifier.modifyTemperature
fn modify_temperature(
    noise: &BiomeNoise,
    modifier: TemperatureModifier,
    pos_x: i32,
    pos_z: i32,
    temperature: f32,
) -> f32 {
    match modifier {
        TemperatureModifier::None => temperature,
        TemperatureModifier::Frozen => {
            let d = noise.frozen_temperature_noise.get_value(
                pos_x as f64 * 0.05,
                pos_z as f64 * 0.05,
                false,
            ) * 7.0;
            let value = noise.biome_info_noise.get_value(
                pos_x as f64 * 0.2,
                pos_z as f64 * 0.2,
                false,
            );
            let d1 = d + value;
            if d1 < 0.3 {
                let value1 = noise.biome_info_noise.get_value(
                    pos_x as f64 * 0.09,
                    pos_z as f64 * 0.09,
                    false,
                );
                if value1 < 0.8 {
                    return 0.2f32;
                }
            }
            temperature
        }
    }
}

/// Biome.getHeightAdjustedTemperature
pub fn get_temperature(noise: &BiomeNoise, facts: &BiomeFacts, pos_x: i32, pos_y: i32, pos_z: i32, sea_level: i32) -> f32 {
    let f = modify_temperature(noise, facts.temperature_modifier, pos_x, pos_z, facts.temperature);
    let i = sea_level + 17;
    if pos_y > i {
        // float f1 = (float)(TEMPERATURE_NOISE.getValue((double)((float)pos.getX() / 8.0f),
        //     (double)((float)pos.getZ() / 8.0f), false) * 8.0);
        let x8 = (pos_x as f32 / 8.0f32) as f64;
        let z8 = (pos_z as f32 / 8.0f32) as f64;
        let f1 = (noise.temperature_noise.get_value(x8, z8, false) * 8.0) as f32;
        // return f - (f1 + (float)pos.getY() - (float)i) * 0.05f / 40.0f;
        f - (f1 + pos_y as f32 - i as f32) * 0.05f32 / 40.0f32
    } else {
        f
    }
}

pub fn warm_enough_to_rain(noise: &BiomeNoise, facts: &BiomeFacts, pos_x: i32, pos_y: i32, pos_z: i32, sea_level: i32) -> bool {
    get_temperature(noise, facts, pos_x, pos_y, pos_z, sea_level) >= 0.15f32
}

pub fn cold_enough_to_snow(noise: &BiomeNoise, facts: &BiomeFacts, pos_x: i32, pos_y: i32, pos_z: i32, sea_level: i32) -> bool {
    !warm_enough_to_rain(noise, facts, pos_x, pos_y, pos_z, sea_level)
}

pub fn should_melt_frozen_ocean_iceberg_slightly(noise: &BiomeNoise, facts: &BiomeFacts, pos_x: i32, pos_y: i32, pos_z: i32, sea_level: i32) -> bool {
    get_temperature(noise, facts, pos_x, pos_y, pos_z, sea_level) > 0.1f32
}

// ---------------------------------------------------------------------------
// BiomeManager — the fiddled 8-neighbour vote
// ---------------------------------------------------------------------------

/// LinearCongruentialGenerator.next(left, right):
///   left *= left * 6364136223846793005L + 1442695040888963407L; return left + right;
#[inline]
fn lcg_next(left: i64, right: i64) -> i64 {
    let l = left.wrapping_mul(left.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407));
    l.wrapping_add(right)
}

#[inline]
fn get_fiddle(seed: i64) -> f64 {
    (((seed >> 24) & 0x3FF) - 512) as f64 * 8.789_062_5e-4
}

#[inline]
fn get_fiddled_distance(seed: i64, x: i32, y: i32, z: i32, x_noise: f64, y_noise: f64, z_noise: f64) -> f64 {
    let mut l = lcg_next(seed, x as i64);
    l = lcg_next(l, y as i64);
    l = lcg_next(l, z as i64);
    l = lcg_next(l, x as i64);
    l = lcg_next(l, y as i64);
    l = lcg_next(l, z as i64);
    let fiddle = get_fiddle(l);
    l = lcg_next(l, seed);
    let fiddle1 = get_fiddle(l);
    l = lcg_next(l, seed);
    let fiddle2 = get_fiddle(l);
    mth::square(z_noise + fiddle2) + mth::square(y_noise + fiddle1) + mth::square(x_noise + fiddle)
}

/// The biome-source resolution: quart coords in, biome name out. Backed by
/// the RandomState climate machinery (quantize + RTree), cached per call site.
pub struct BiomeSource<'a> {
    pub rs: &'a RandomState,
    pub list: ParameterList,
    cache: HashMap<(i32, i32, i32), String>,
}

impl<'a> BiomeSource<'a> {
    pub fn new(rs: &'a RandomState) -> Self {
        let list = ParameterList::new(
            crate::vanilla_biomes::overworld_points()
                .into_iter()
                .map(|(p, n)| (p, n.to_string()))
                .collect(),
        );
        BiomeSource { rs, list, cache: HashMap::new() }
    }

    /// MultiNoiseBiomeSource.getNoiseBiome(quartX, quartY, quartZ): the
    /// sampler takes QUART coords and evaluates at BLOCK coords (T29).
    pub fn get_noise_biome(&mut self, qx: i32, qy: i32, qz: i32) -> String {
        if let Some(b) = self.cache.get(&(qx, qy, qz)) {
            return b.clone();
        }
        let (bx, by, bz) = (qx * 4, qy * 4, qz * 4);
        let r = &self.rs.router;
        let t = crate::climate::TargetPoint {
            temperature: crate::climate::quantize_coord(r.temperature.compute(&self.rs.bank, bx, by, bz) as f32),
            humidity: crate::climate::quantize_coord(r.vegetation.compute(&self.rs.bank, bx, by, bz) as f32),
            continentalness: crate::climate::quantize_coord(r.continents.compute(&self.rs.bank, bx, by, bz) as f32),
            erosion: crate::climate::quantize_coord(r.erosion.compute(&self.rs.bank, bx, by, bz) as f32),
            depth: crate::climate::quantize_coord(r.depth.compute(&self.rs.bank, bx, by, bz) as f32),
            weirdness: crate::climate::quantize_coord(r.ridges.compute(&self.rs.bank, bx, by, bz) as f32),
        };
        let name = self.list.find_value(&t).to_string();
        self.cache.insert((qx, qy, qz), name.clone());
        name
    }
}

/// BiomeManager.getBiome(BlockPos) — 8-neighbour vote with the fiddled
/// distances; biomeZoomSeed = obfuscateSeed(levelSeed).
pub fn get_biome_voted(source: &mut BiomeSource, zoom_seed: i64, x: i32, y: i32, z: i32) -> String {
    let i = x - 2;
    let i1 = y - 2;
    let i2 = z - 2;
    let i3 = i >> 2;
    let i4 = i1 >> 2;
    let i5 = i2 >> 2;
    let d = (i & 3) as f64 / 4.0;
    let d1 = (i1 & 3) as f64 / 4.0;
    let d2 = (i2 & 3) as f64 / 4.0;
    let mut best_idx = 0usize;
    let mut best = f64::INFINITY;
    for i7 in 0..8usize {
        let flag = (i7 & 4) == 0;
        let flag1 = (i7 & 2) == 0;
        let flag2 = (i7 & 1) == 0;
        let i8 = if flag { i3 } else { i3 + 1 };
        let i9 = if flag1 { i4 } else { i4 + 1 };
        let i10 = if flag2 { i5 } else { i5 + 1 };
        let d4 = if flag { d } else { d - 1.0 };
        let d5 = if flag1 { d1 } else { d1 - 1.0 };
        let d6 = if flag2 { d2 } else { d2 - 1.0 };
        let fiddled = get_fiddled_distance(zoom_seed, i8, i9, i10, d4, d5, d6);
        // Java: if (!(d3 > fiddledDistance)) continue; -> strictly-less wins
        if best > fiddled {
            best_idx = i7;
            best = fiddled;
        }
    }
    let qx = if (best_idx & 4) == 0 { i3 } else { i3 + 1 };
    let qy = if (best_idx & 2) == 0 { i4 } else { i4 + 1 };
    let qz = if (best_idx & 1) == 0 { i5 } else { i5 + 1 };
    source.get_noise_biome(qx, qy, qz).to_string()
}

pub fn biome_zoom_seed(level_seed: i64) -> i64 {
    obfuscate_seed(level_seed)
}
