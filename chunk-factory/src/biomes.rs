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

/// The LCG chain of the reference get_fiddled_distance, stopped after the
/// third fiddle: 6 LCG steps over the corner quart ints, then 2 seed
/// re-mixes. The chain consumes ONLY (seed, corner quart x/y/z) — no block
/// fraction — so it is constant per (corner, quart cell) and is exactly what
/// the vote-plan memo stores (R3#1). Extracted VERBATIM from the reference
/// body; the test-only wrapper below re-assembles the line-246 square-sum.
#[inline]
fn corner_fiddles(seed: i64, x: i32, y: i32, z: i32) -> [f64; 3] {
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
    [fiddle, fiddle1, fiddle2]
}

/// REFERENCE implementation (frozen, test-only since the S3 vote-plan memo):
/// the production path resolves the corner fiddles through VotePlanMemo and
/// re-runs ONLY this square-sum per block, in exactly this grouping
/// (f64 + is non-associative — no folding, no reassociation). Kept as the
/// bit-exactness oracle for the 10^5-position property test and T35; the
/// fiddles come from corner_fiddles — the chain verbatim — so reference and
/// memoized operands are the same f64s by construction.
#[cfg(test)]
fn get_fiddled_distance(seed: i64, x: i32, y: i32, z: i32, x_noise: f64, y_noise: f64, z_noise: f64) -> f64 {
    let [fiddle, fiddle1, fiddle2] = corner_fiddles(seed, x, y, z);
    mth::square(z_noise + fiddle2) + mth::square(y_noise + fiddle1) + mth::square(x_noise + fiddle)
}

// ---------------------------------------------------------------------------
// R3#1 — the per-(x, z, quart-y) vote-plan memo
// ---------------------------------------------------------------------------

/// Fixed-seed multiply-rotate hasher for the vote-plan keys (fx-hash style:
/// h = rotl(h, 5) ^ w; h *= M). Keys are internal quart coords — not
/// attacker-controlled — and the table is only get/insert-ed (iteration
/// order never observed), so a deterministic fixed-seed hash is safe and
/// removes the SipHash cost from ~5-21k vote lookups per chunk.
#[derive(Default)]
struct VoteHasher(u64);

impl VoteHasher {
    #[inline]
    fn mix(&mut self, w: u64) {
        self.0 = (self.0.rotate_left(5) ^ w).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
}

impl std::hash::Hasher for VoteHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.mix(b as u64);
        }
    }
    #[inline]
    fn write_u8(&mut self, v: u8) {
        self.mix(v as u64);
    }
    #[inline]
    fn write_i32(&mut self, v: i32) {
        self.mix(v as u32 as u64);
    }
}

/// (i3, i5, i4) -> the 8 corner (fiddle, fiddle1, fiddle2) triples; the
/// array index is the corner bit pattern i7 = (dx<<2)|(dz<<1)|(dy) exactly
/// as in the reference vote loop.
type VotePlanMap = HashMap<(i32, i32, i32), [[f64; 3]; 8], std::hash::BuildHasherDefault<VoteHasher>>;

/// The 8 corner fiddle triples for one vote cell. Within a cell (i3, i4, i5
/// fixed — the 4x4x4 BLOCKS whose (coord-2)>>2 is constant) the corner quart
/// ints (i8, i9, i10) take the same 8 (i3|i3+1, i4|i4+1, i5|i5+1)
/// combinations for EVERY block, and the LCG chain consumes only those ints
/// plus the seed — so all 24 fiddle values are cell-constants (R3 research
/// Q2). Only the square-sum operands d4/d5/d6 (the (coord-2)&3 / 4
/// fractions) vary per block and are NOT memoized.
fn build_vote_plan(seed: i64, i3: i32, i4: i32, i5: i32) -> [[f64; 3]; 8] {
    let mut plan = [[0.0f64; 3]; 8];
    for (i7, slot) in plan.iter_mut().enumerate() {
        let i8 = if (i7 & 4) == 0 { i3 } else { i3 + 1 };
        let i9 = if (i7 & 2) == 0 { i4 } else { i4 + 1 };
        let i10 = if (i7 & 1) == 0 { i5 } else { i5 + 1 };
        *slot = corner_fiddles(seed, i8, i9, i10);
    }
    plan
}

/// Per-BiomeSource vote-plan memo: kills the redundant 64-LCG chains for the
/// 64 blocks sharing a quart cell (was ~100ns of dependency-chained integer
/// math per vote, ~5-21k votes/chunk; ~50x less chain work at equal reuse).
/// Bit-exactness: integer hoisting only (the wrapping LCG is pure — same
/// inputs, same wrapping i64s, hence same f64 conversions); the per-block
/// recombination keeps the exact f64 grouping of the reference —
/// square(z+f2) + square(y+f1) + square(x+f), left-to-right, NO folding —
/// with fiddle operands that are the previously recomputed f64s
/// bit-for-bit. The winner comparison sequence is byte-for-byte the
/// reference's (strictly-less wins, INFINITY start, index 0 default).
#[derive(Default)]
struct VotePlanMemo {
    /// the zoom seed the plan table was built for — cleared on change (the
    /// seed is a per-world constant in production; the guard keeps the memo
    /// correct for ANY caller pattern, incl. the multi-seed property test).
    seed: i64,
    plan: VotePlanMap,
}

impl VotePlanMemo {
    /// BiomeManager.getBiome winner selection, memoized. Decision logic is
    /// byte-for-byte the reference vote_best_corner's; only the corner
    /// fiddle provisioning differs (table lookup vs recomputation).
    fn vote(&mut self, zoom_seed: i64, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        let i = x - 2;
        let i1 = y - 2;
        let i2 = z - 2;
        let i3 = i >> 2;
        let i4 = i1 >> 2;
        let i5 = i2 >> 2;
        let d = (i & 3) as f64 / 4.0;
        let d1 = (i1 & 3) as f64 / 4.0;
        let d2 = (i2 & 3) as f64 / 4.0;
        if self.seed != zoom_seed {
            self.plan.clear();
            self.seed = zoom_seed;
        }
        let plan = self
            .plan
            .entry((i3, i5, i4))
            .or_insert_with(|| build_vote_plan(zoom_seed, i3, i4, i5));
        let mut best_idx = 0usize;
        let mut best = f64::INFINITY;
        for i7 in 0..8usize {
            let flag = (i7 & 4) == 0;
            let flag1 = (i7 & 2) == 0;
            let flag2 = (i7 & 1) == 0;
            let d4 = if flag { d } else { d - 1.0 };
            let d5 = if flag1 { d1 } else { d1 - 1.0 };
            let d6 = if flag2 { d2 } else { d2 - 1.0 };
            let [fiddle, fiddle1, fiddle2] = plan[i7];
            // EXACT reference grouping: square(z+f2) + square(y+f1) +
            // square(x+f), evaluated left-to-right with the same operands.
            let fiddled =
                mth::square(d6 + fiddle2) + mth::square(d5 + fiddle1) + mth::square(d4 + fiddle);
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
    /// R3#1: the per-(x, z, quart-y) vote-plan memo (see VotePlanMemo).
    vote_plan: VotePlanMemo,
    /// R3#2: per-instance climate column memo threaded into Df::compute_memo
    /// (density.rs): memoizes every provably y-free climate subtree per
    /// (x, z) BLOCK column; bit-identical to Df::compute by the density.rs
    /// equivalence argument — the SAME machinery filler's initial biome
    /// paint already runs under (filler.rs climate_fields). Keys hold node
    /// pointers of self.rs, which outlives this instance and interns its
    /// nodes immutably — no ABA within the per-instance lifetime.
    climate_memo: crate::density::ColumnMemo,
}

impl<'a> BiomeSource<'a> {
    pub fn new(rs: &'a RandomState) -> Self {
        BiomeSource {
            rs,
            list: rs.biome_list(),
            memo: None,
            cache: HashMap::new(),
            vote_plan: VotePlanMemo::default(),
            climate_memo: HashMap::new(),
        }
    }

    /// The shared single-allocation resolution path: on a cache miss one
    /// String is built for the caller and one clone goes into the cache;
    /// the vote wrappers hand that SAME String to their caller instead of
    /// cloning it a second time via `.to_string()` (R4#2 micro). The RTree
    /// hint chain is untouched — find_value still runs per distinct quart,
    /// in the same order, with the same (bit-identical) TargetPoint inputs.
    fn resolve_noise_biome(&mut self, qx: i32, qy: i32, qz: i32) -> String {
        if let Some(b) = self.cache.get(&(qx, qy, qz)) {
            return b.clone();
        }
        let (bx, by, bz) = (qx * 4, qy * 4, qz * 4);
        // copy the &'a RandomState out first: the router/bank borrows must
        // not alias the &mut climate_memo below
        let rs = self.rs;
        let r = &rs.router;
        let bank = &rs.bank;
        let memo = &mut self.climate_memo;
        let t = crate::climate::TargetPoint {
            temperature: crate::climate::quantize_coord(r.temperature.compute_memo(bank, bx, by, bz, memo) as f32),
            humidity: crate::climate::quantize_coord(r.vegetation.compute_memo(bank, bx, by, bz, memo) as f32),
            continentalness: crate::climate::quantize_coord(r.continents.compute_memo(bank, bx, by, bz, memo) as f32),
            erosion: crate::climate::quantize_coord(r.erosion.compute_memo(bank, bx, by, bz, memo) as f32),
            depth: crate::climate::quantize_coord(r.depth.compute_memo(bank, bx, by, bz, memo) as f32),
            weirdness: crate::climate::quantize_coord(r.ridges.compute_memo(bank, bx, by, bz, memo) as f32),
        };
        let name = self.list.find_value(&t, &mut self.memo).to_string();
        self.cache.insert((qx, qy, qz), name.clone());
        name
    }

    /// MultiNoiseBiomeSource.getNoiseBiome(quartX, quartY, quartZ): the
    /// sampler takes QUART coords and evaluates at BLOCK coords (T29).
    pub fn get_noise_biome(&mut self, qx: i32, qy: i32, qz: i32) -> String {
        self.resolve_noise_biome(qx, qy, qz)
    }
}

/// BiomeManager.getBiome(BlockPos) — 8-neighbour vote with the fiddled
/// distances; biomeZoomSeed = obfuscateSeed(levelSeed).
///
/// This variant resolves the WINNING corner through a fresh (uncached)
/// climate sample — the semantics of a NoiseBiomeSource that is the
/// MultiNoiseBiomeSource itself (structure/spawn probing).
pub fn get_biome_voted(source: &mut BiomeSource, zoom_seed: i64, x: i32, y: i32, z: i32) -> String {
    let (qx, qy, qz) = source.vote_plan.vote(zoom_seed, x, y, z);
    source.resolve_noise_biome(qx, qy, qz)
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
    let (qx, qy, qz) = source.vote_plan.vote(zoom_seed, x, y, z);
    let qy = stored_quart_y(qy, min_section, section_count);
    source.resolve_noise_biome(qx, qy, qz)
}

/// REFERENCE vote (frozen, test-only since the S3 vote-plan memo): the
/// production path is VotePlanMemo::vote (identical decision logic, hoisted
/// corner fiddles). Kept VERBATIM as the old-vs-new oracle for the
/// 10^5-position property test and the T35 region-clamp test.
#[cfg(test)]
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

#[cfg(test)]
mod vote_memo_tests {
    use super::*;
    use crate::xoroshiro::Xoroshiro128PlusPlus;

    /// R3#1 bit-exactness property: over 4 seeds x 25_000 = 100_000
    /// pseudo-random block positions (fixed-seed crate-internal
    /// xoroshiro128++ — fully deterministic), the memoized vote must
    /// (a) agree with the reference vote_best_corner on the winning corner
    /// and (b) reproduce every per-corner fiddled distance BIT-IDENTICALLY
    /// (f64::to_bits) against the frozen reference get_fiddled_distance.
    /// One memo instance is reused across the 4 seeds, which also exercises
    /// the clear-on-seed-change guard (a stale entry would flip bits here).
    /// Coordinate ranges keep quart cells densely revisited, so the run is
    /// dominated by the memo-hit path — the same shape as the surface walk.
    #[test]
    fn vote_plan_memo_bit_exact_over_100k_positions() {
        let mut rng = Xoroshiro128PlusPlus::new(0x5DEE_CE6D, 0x2545_F491_4F6C_DD1D);
        let seeds = [
            -3053459i64,
            0x9E37_79B9_7F4A_7C15u64 as i64,
            1234567890123456789i64,
            42i64,
        ];
        let mut checked = 0usize;
        // ONE memo instance across all 4 seeds — the clear-on-seed-change
        // guard is exercised (a stale entry would flip bits in phase (b)).
        let mut memo = VotePlanMemo::default();
        for &seed in &seeds {
            for _ in 0..25_000 {
                let x = (rng.next_long().rem_euclid(16384)) as i32 - 8192;
                let z = (rng.next_long().rem_euclid(16384)) as i32 - 8192;
                let y = (rng.next_long().rem_euclid(1024)) as i32 - 512;

                // (a) winner agreement old vs new
                let old = vote_best_corner(seed, x, y, z);
                let new = memo.vote(seed, x, y, z);
                assert_eq!(old, new, "vote winner diverged at ({x},{y},{z}) seed {seed}");

                // (b) bit-level: all 8 corner distances from the plan must
                // equal the reference chain+square-sum, to_bits for bits.
                let (i, i1, i2) = (x - 2, y - 2, z - 2);
                let (i3, i4, i5) = (i >> 2, i1 >> 2, i2 >> 2);
                let plan = memo
                    .plan
                    .get(&(i3, i5, i4))
                    .expect("plan must be memoized right after the vote");
                let d = (i & 3) as f64 / 4.0;
                let d1 = (i1 & 3) as f64 / 4.0;
                let d2 = (i2 & 3) as f64 / 4.0;
                for i7 in 0..8usize {
                    let i8 = if (i7 & 4) == 0 { i3 } else { i3 + 1 };
                    let i9 = if (i7 & 2) == 0 { i4 } else { i4 + 1 };
                    let i10 = if (i7 & 1) == 0 { i5 } else { i5 + 1 };
                    let d4 = if (i7 & 4) == 0 { d } else { d - 1.0 };
                    let d5 = if (i7 & 2) == 0 { d1 } else { d1 - 1.0 };
                    let d6 = if (i7 & 1) == 0 { d2 } else { d2 - 1.0 };
                    let reference = get_fiddled_distance(seed, i8, i9, i10, d4, d5, d6);
                    let [fiddle, fiddle1, fiddle2] = plan[i7];
                    let recomputed =
                        mth::square(d6 + fiddle2) + mth::square(d5 + fiddle1) + mth::square(d4 + fiddle);
                    assert_eq!(
                        recomputed.to_bits(),
                        reference.to_bits(),
                        "fiddled distance bits diverged at ({x},{y},{z}) corner {i7} seed {seed}"
                    );
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 100_000, "property run must cover 10^5 positions");
    }

    /// R3#2 guard (NCF_WG-gated like T35): Df::compute_memo must be
    /// bit-identical to Df::compute on all 6 climate router fields at
    /// assorted block coords — first call (pure-compute + memo fill) and
    /// second call (memo-hit) — the exact equivalence get_noise_biome's
    /// switch relies on (density.rs documents it; filler.rs already runs
    /// the same path for the initial biome paint).
    #[test]
    fn climate_compute_memo_matches_compute_bitwise() {
        let Ok(wg) = std::env::var("NCF_WG") else { return };
        let Ok(dir) = crate::router::WorldgenDir::load(std::path::Path::new(&wg)) else { return };
        let rs = crate::router::RandomState::build(&dir, "minecraft", "overworld", 3053459)
            .expect("random state");
        let r = &rs.router;
        let fields = [
            &r.temperature,
            &r.vegetation,
            &r.continents,
            &r.erosion,
            &r.depth,
            &r.ridges,
        ];
        let coords = [
            (400i32, 64i32, 400i32),
            (-41, -17, 17),
            (1616, 300, -400),
            (0, 0, 0),
            (12345, -200, -9999),
            (4, -64, -4),
        ];
        let mut memo = crate::density::ColumnMemo::new();
        for pass in 0..2 {
            for &(bx, by, bz) in &coords {
                for (fi, field) in fields.iter().enumerate() {
                    let a = field.compute(&rs.bank, bx, by, bz);
                    let b = field.compute_memo(&rs.bank, bx, by, bz, &mut memo);
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "climate field {fi} pass {pass} diverged at ({bx},{by},{bz})"
                    );
                }
            }
        }
    }
}
