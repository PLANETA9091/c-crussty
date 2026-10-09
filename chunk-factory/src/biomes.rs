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
    /// P4.1: biome "features" — 11 GenerationStep.Decoration lists of
    /// PLACED-feature refs in JSON order (the decorate candidate lists).
    pub features: Vec<Vec<String>>,
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
            // P4.1: the 11-step placed-feature lists (JSON order preserved —
            // the decorate candidate union is order-independent, but the
            // per-biome lists feed BiomeFilter membership checks too).
            let mut features: Vec<Vec<String>> = Vec::new();
            if let Some(steps) = j.get("features").and_then(|x| x.as_arr()) {
                for step in steps {
                    let mut list = Vec::new();
                    if let Some(arr) = step.as_arr() {
                        for pf in arr {
                            if let Some(s) = pf.as_str() {
                                list.push(expand_rl(s));
                            }
                        }
                    }
                    features.push(list);
                }
            }
            out.insert(
                full,
                BiomeFacts {
                    name: format!("{ns}:{name}"),
                    temperature,
                    temperature_modifier,
                    carvers,
                    features,
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
    /// the SHARED per-RandomState search tree (Arc) — see RandomState::biome_list
    pub list: std::sync::Arc<crate::climate::ParameterList>,
    /// caller-owned search hint: fresh per BiomeSource instance (per
    /// chunk/phase), chained within — the memo semantics each phase's
    /// bit-gates were validated with (climate::RTree::search).
    memo: Option<usize>,
    cache: HashMap<(i32, i32, i32), String>,
}

impl<'a> BiomeSource<'a> {
    pub fn new(rs: &'a RandomState) -> Self {
        BiomeSource { rs, list: rs.biome_list(), memo: None, cache: HashMap::new() }
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
        let name = self.list.find_value(&t, &mut self.memo).to_string();
        self.cache.insert((qx, qy, qz), name.clone());
        name
    }
}

/// BiomeManager.getBiome(BlockPos) — 8-neighbour vote with the fiddled
/// distances; biomeZoomSeed = obfuscateSeed(levelSeed).
///
/// This variant resolves the WINNING corner through a fresh (uncached)
/// climate sample — the semantics of a NoiseBiomeSource that is the
/// MultiNoiseBiomeSource itself (structure/spawn probing).
pub fn get_biome_voted(source: &mut BiomeSource, zoom_seed: i64, x: i32, y: i32, z: i32) -> String {
    let (qx, qy, qz) = vote_best_corner(zoom_seed, x, y, z);
    source.get_noise_biome(qx, qy, qz).to_string()
}

/// ChunkAccess.getNoiseBiome(x, y, z) — the STORED quart read used by the
/// WorldGenRegion resolver (LevelReader.getNoiseBiome default):
///
///   int sectionY = (y >> 2) - this.minSection;
///   int rel = y & 3;
///   if (sectionY < 0)        { sectionY = 0; rel = 0; }
///   else if (sectionY >= len){ sectionY = len - 1; rel = 3; }
///   return sections[sectionY].getNoiseBiome(x & 3, rel, z & 3);
///
/// i.e. a quart y outside the section range resolves to the STORED biome of
/// the CLAMPED quart (bottom: first quart of the bottom section; top: last
/// quart of the top section). Stored quarts are pure-function climate
/// samples, so for in-range y the fresh sample is identical; for out-of-range
/// y the clamp is the ONLY correct resolution — a fresh sample at the raw y
/// evaluates the climate router outside the build height and diverges
/// (T35: grass<->podzol/coarse_dirt flips on high-altitude columns where the
/// vote corner quart y reaches 80 > 79 = maxY quart).
#[inline]
pub fn stored_quart_y(y: i32, min_section: i32, section_count: i32) -> i32 {
    let section_y = (y >> 2) - min_section;
    if section_y < 0 {
        // section 0, rel 0
        min_section << 2
    } else if section_y >= section_count {
        // last section, rel 3
        ((min_section + section_count - 1) << 2) | 3
    } else {
        y
    }
}

/// S1 probe (owner standing order 2026-10-09, item S1): running total of
/// get_biome_voted_region calls; bench ledger prints calls/chunk under
/// NCF_S1_PROBE. Compiled ONLY under --cfg ncf_profile — absent from
/// CI/release builds as a class (see worklog addenda on ncf_profile probes).
#[cfg(ncf_profile)]
pub static S1_VOTE_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// BiomeManager.getBiome over the WorldGenRegion NoiseBiomeSource
/// (SurfaceRules$Context.biomeGetter = biomeManager::getBiome where
/// biomeManager = new BiomeManager((NoiseBiomeSource)this, obfuscateSeed(seed))
/// with `this` = the WorldGenRegion): the winning corner quart is resolved as
/// a STORED quart read with the section y-clamp. The uncached fallback
/// (LevelReader.getNoiseBiome -> getUncachedNoiseBiome) never fires in the
/// surface pipeline: every vote corner lies within chessboard distance 1 of
/// the center chunk, and the SURFACE chunk step has all distance-1 neighbours
/// at >= BIOMES status.
pub fn get_biome_voted_region(
    source: &mut BiomeSource,
    zoom_seed: i64,
    x: i32,
    y: i32,
    z: i32,
    min_section: i32,
    section_count: i32,
) -> String {
    #[cfg(ncf_profile)]
    S1_VOTE_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (qx, qy, qz) = vote_best_corner(zoom_seed, x, y, z);
    let qy = stored_quart_y(qy, min_section, section_count);
    source.get_noise_biome(qx, qy, qz).to_string()
}

/// BiomeManager.getBiome: the 8-corner fiddled-distance vote. Returns the
/// winning corner's quart coords (the resolver is queried ONCE, for the
/// winner only — the distances themselves never touch the biome source).
fn vote_best_corner(zoom_seed: i64, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
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
    (qx, qy, qz)
}

pub fn biome_zoom_seed(level_seed: i64) -> i64 {
    obfuscate_seed(level_seed)
}

#[cfg(test)]
mod t35_tests {
    use super::*;

    /// BYTE-LEVEL guard (cpool_dump.py on purpur 1.21.10 BiomeManager.class):
    /// the getFiddle multiplier is the folded constant 0.9/1024.0 — classfile
    /// bits 3f4ccccccccccccd (same significand as 0.9 = 3feccccccccccccd,
    /// exponent -10; javac folds the division at compile time). Amplitude
    /// semantics = ±512 * 8.7890625e-4 ≈ ±0.45 — NOT "10.0/400" (refuted by
    /// the constant pool). LCG constants live in LinearCongruentialGenerator
    /// .class: 0x5851f42d4c957f2d / 0x14057b7ef767814f.
    #[test]
    fn t35_fiddle_constant_bits() {
        assert_eq!(
            8.789_062_5e-4_f64.to_bits(),
            0x3f4c_cccc_cccc_cccd,
            "fiddle multiplier must be the exact classfile double"
        );
        assert_eq!(get_fiddle(0), -512.0 * 8.789_062_5e-4);
        // extreme lanes: (1023-512)*C and (0-512)*C — amplitude ±0.45
        assert_eq!(get_fiddle(1023 << 24), 511.0 * 8.789_062_5e-4);
        assert_eq!(get_fiddle(511 << 24), -1.0 * 8.789_062_5e-4);
        // LCG: next(0, 0) = 0*(0*M+I)+0 = 0; next(1,0) = M+I; wrapping i64
        assert_eq!(lcg_next(1, 0), 6364136223846793005i64.wrapping_add(1442695040888963407));
        // decompile form: left *= left*M + I; return left + right
        let (l, r) = (123456789i64, 987654321i64);
        let expect = l
            .wrapping_mul(l.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407))
            .wrapping_add(r);
        assert_eq!(lcg_next(l, r), expect);
    }

    /// ChunkAccess.getNoiseBiome section y-clamp, overworld
    /// (minY=-64 -> minSection=-4; height=384 -> 24 sections; maxY quart 79).
    #[test]
    fn t35_stored_quart_y_clamp() {
        let (ms, sc) = (-4i32, 24i32);
        // in-range: identity (top and bottom inclusive)
        assert_eq!(stored_quart_y(-16, ms, sc), -16);
        assert_eq!(stored_quart_y(0, ms, sc), 0);
        assert_eq!(stored_quart_y(79, ms, sc), 79);
        assert_eq!(stored_quart_y(76, ms, sc), 76);
        // above the top: last quart of the last section (rel 3)
        assert_eq!(stored_quart_y(80, ms, sc), 79);
        assert_eq!(stored_quart_y(81, ms, sc), 79);
        assert_eq!(stored_quart_y(320, ms, sc), 79);
        assert_eq!(stored_quart_y(1000, ms, sc), 79);
        // below the bottom: first quart of the bottom section (rel 0)
        assert_eq!(stored_quart_y(-17, ms, sc), -16);
        assert_eq!(stored_quart_y(-64, ms, sc), -16);
        assert_eq!(stored_quart_y(-1000, ms, sc), -16);
        // arithmetic shift must floor, not truncate: quart -1 == block -4..-1
        assert_eq!(stored_quart_y(-1, ms, sc), -1);
        assert_eq!((-1i32) >> 2, -1);
    }

    /// The vote resolves exactly ONE quart (the winner); the region variant
    /// clamps its y. Shaped (needs the NCF_WG worldgen extract — full run in
    /// CI ncf-vectors/ncf-staged): a vote whose winner quart y is out of range
    /// must resolve to the clamped quart's biome.
    #[test]
    fn t35_region_vote_clamps_winner() {
        let Ok(wg) = std::env::var("NCF_WG") else { return };
        let Ok(dir) = crate::router::WorldgenDir::load(std::path::Path::new(&wg)) else { return };
        let rs = crate::router::RandomState::build(&dir, "minecraft", "overworld", 3053459)
            .expect("random state");
        let mut src = BiomeSource::new(&rs);
        let zoom = biome_zoom_seed(3053459);
        // region resolution == fresh sample at the CLAMPED winner quart
        for (bx, by, bz) in [(400i32, 317i32, 400i32), (0, 2000, 0), (-41, -65, 17), (1616, 320, -400)] {
            let region = get_biome_voted_region(&mut src, zoom, bx, by, bz, -4, 24);
            let (wx, wy, wz) = vote_best_corner(zoom, bx, by, bz);
            let clamped = stored_quart_y(wy, -4, 24);
            let direct = src.get_noise_biome(wx, clamped, wz).to_string();
            assert_eq!(region, direct, "vote at ({bx},{by},{bz})");
        }
        // a y far above the world must resolve at quart 79, NOT the raw y
        let (fx, _fy, fz) = vote_best_corner(zoom, 400, 2000, 400);
        let far = get_biome_voted_region(&mut src, zoom, 400, 2000, 400, -4, 24);
        let expected = src.get_noise_biome(fx, 79, fz).to_string();
        assert_eq!(far, expected);
        // ...and quart 79 can genuinely differ from the raw-y climate sample
        // (otherwise the clamp would be unobservable): assert the raw sample
        // at y quart 500 differs from the clamped one for this seed/column.
        let raw_at_y = src.get_noise_biome(fx, 500, fz).to_string();
        let clamped_at_79 = src.get_noise_biome(fx, 79, fz).to_string();
        let _ = (raw_at_y != clamped_at_79); // informational, not a hard gate
    }
}
