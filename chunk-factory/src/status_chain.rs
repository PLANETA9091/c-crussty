//! NCF session 6 — status chain orchestration:
//!   NOISE (filler.rs) -> SURFACE (surface_rules.rs) -> CARVERS (carvers.rs)
//!
//! Replicates the Paper 1.21.10 ChunkStatus pipeline for the overworld
//! generator:
//!   - SURFACE  = NoiseBasedChunkGenerator.buildSurface (SurfaceSystem) with
//!     the noise_settings `surface_rule` tree;
//!   - CARVERS  = NoiseBasedChunkGenerator.applyCarvers — 17x17 neighbour
//!     walk; each neighbour chunk's carver list comes from the DIRECT climate
//!     biome at the chunk's min-block corner, y=0 quart (no BiomeManager
//!     vote); ONE aquifer (center chunk's) is shared across all carvers.
//!
//! Structures (beardifier) are Phase 5: the gated corpus (fresh world) has no
//! structures intersecting the probe chunks except the documented I8
//! fallback set.

use crate::biomes::{load_biome_facts, BiomeNoise, BiomeSource};
use crate::carvers::{apply_carvers, parse_configured_carver, CarverConfig, CarverKit};
use crate::filler::{generate_noise_chunk, FillerChunk, StateTable};
use crate::router::{RandomState, WorldgenDir};
use crate::surface_rules::{
    build_surface, parse_rule, ChunkColumns, SurfaceContext, SurfaceRuleSet, SurfaceSystem,
};
use std::cell::RefCell;
use std::collections::HashMap;

/// Parsed, reusable per-batch worldgen kit (RandomState-level state: noises
/// interned once, surface rule built once, biome facts loaded once).
pub struct StageKit {
    pub rule_set: SurfaceRuleSet,
    pub system: SurfaceSystem,
    pub facts: HashMap<String, crate::biomes::BiomeFacts>,
    pub biome_noise: BiomeNoise,
    pub carver_configs: HashMap<String, CarverConfig>,
    /// cached parsed configured_carver JSONs by ref
    pub default_block: String,
    /// P5.3 increment 4: the per-chunk Beardifier, installed by the batch
    /// driver BEFORE generate_surface_chunk/generate_carvers_chunk (EMPTY =
    /// pre-wiring machine). Rides in the kit so the batch driver's
    /// long-lived &RandomState borrows (feed/sampler) can coexist.
    pub beard: crate::beardifier::Beardifier,
}

impl StageKit {
    pub fn build(rs: &mut RandomState, dir: &WorldgenDir) -> Result<Self, String> {
        // surface_rule IR from the SAME noise_settings JSON (P1.3 surface half)
        let text = dir
            .get(&rs.settings_ns, "noise_settings", &rs.settings_name)
            .ok_or_else(|| format!("noise_settings not found: {}:{}", rs.settings_ns, rs.settings_name))?
            .clone();
        let j = crate::json::parse(&text).map_err(|e| e.to_string())?;
        let rule_json = j
            .get("surface_rule")
            .ok_or("noise_settings missing surface_rule")?;
        let rule_def = parse_rule(rule_json)?;
        let default_block = rs.settings.default_block.clone();
        let mut table = StateTable::new();
        table.intern_canonical(&default_block);
        let rule_set = SurfaceRuleSet::build(&rule_def, rs, dir, &mut table)?;
        let system = SurfaceSystem::new(rs, dir, &mut table)?;
        let facts = load_biome_facts(dir)?;
        let biome_noise = BiomeNoise::new();
        // configured carvers: parse ONLY the refs referenced by overworld
        // biome carver lists (nether_cave etc. would fail the overworld-only
        // parser — they are not part of the gated corpus).
        let mut carver_configs = HashMap::new();
        let mut wanted: Vec<String> = facts
            .values()
            .flat_map(|f| f.carvers.iter().cloned())
            .collect();
        wanted.sort();
        wanted.dedup();
        for full in wanted {
            let (ns, name) = full
                .split_once(':')
                .ok_or_else(|| format!("bad carver ref {full}"))?;
            let Some(text) = dir.read(ns, "configured_carver", name) else {
                return Err(format!("configured_carver not found: {full}"));
            };
            let cj = crate::json::parse(&text).map_err(|e| e.to_string())?;
            match parse_configured_carver(&cj, &expand_tag_fn) {
                Ok(cfg) => {
                    carver_configs.insert(full, cfg);
                }
                Err(e) => {
                    // A referenced-but-unsupported carver must NOT silently
                    // vanish: register a loud marker entry the fallback
                    // policy reports (P3.5/P5.6). The vanilla overworld
                    // corpus only references cave/cave_extra_underground/
                    // canyon — all supported.
                    return Err(format!("carver {full}: {e}"));
                }
            }
        }
        Ok(StageKit {
            rule_set,
            system,
            facts,
            biome_noise,
            carver_configs,
            default_block,
            beard: crate::beardifier::Beardifier::empty(),
        })
    }
}

/// Tag expansion bound to NCF_DATA_ROOT (data/minecraft/tags/block/**).
fn expand_tag_fn(reference: &str) -> Result<Vec<String>, String> {
    let name = reference.strip_prefix('#').unwrap_or(reference);
    let (ns, base) = match name.split_once(':') {
        Some((a, b)) => (a.to_string(), b.to_string()),
        None => ("minecraft".to_string(), name.to_string()),
    };
    let root = std::env::var_os("NCF_DATA_ROOT")
        .map(std::path::PathBuf::from)
        .ok_or("NCF_DATA_ROOT not set")?;
    let path = root
        .join("data")
        .join(&ns)
        .join("tags")
        .join("block")
        .join(format!("{base}.json"));
    let text = std::fs::read_to_string(&path).map_err(|e| format!("tag read {path:?}: {e}"))?;
    let j = crate::json::parse(&text).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    let mut visited = std::collections::HashSet::new();
    expand_tag_json(&j, &root, &mut out, &mut visited)?;
    Ok(out)
}

fn expand_tag_json(
    j: &crate::json::Json,
    root: &std::path::Path,
    out: &mut Vec<String>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<(), String> {
    let arr = j
        .get("values")
        .and_then(|x| x.as_arr())
        .ok_or("tag without values")?;
    for e in arr {
        let s = e.as_str().ok_or("tag entry not a string")?;
        if let Some(inner) = s.strip_prefix('#') {
            if !visited.insert(inner.to_string()) {
                continue;
            }
            let (ns, base) = match inner.split_once(':') {
                Some((a, b)) => (a.to_string(), b.to_string()),
                None => ("minecraft".to_string(), inner.to_string()),
            };
            let path = root
                .join("data")
                .join(&ns)
                .join("tags")
                .join("block")
                .join(format!("{base}.json"));
            let text = std::fs::read_to_string(&path).map_err(|e| format!("tag read {path:?}: {e}"))?;
            let inner_j = crate::json::parse(&text).map_err(|e| e.to_string())?;
            expand_tag_json(&inner_j, root, out, visited)?;
        } else if s.contains(':') {
            out.push(s.to_string());
        } else {
            out.push(format!("minecraft:{s}"));
        }
    }
    Ok(())
}

/// Generate one chunk at SURFACE status. The per-chunk Beardifier rides in
/// the kit (kit.beard; EMPTY = pre-wiring behavior — Java's marker-only
/// machine). Kept OUT of the signature so callers holding a &RandomState
/// borrow (the Beardifier feed) can coexist with the &mut RandomState here.
pub fn generate_surface_chunk(
    rs: &mut RandomState,
    kit: &mut StageKit,
    dir: &WorldgenDir,
    seed: i64,
    cx: i32,
    cz: i32,
) -> Result<FillerChunk, String> {
    let mut chunk =
        crate::filler::generate_noise_chunk_with_beardifier(rs, seed, cx, cz, kit.beard.clone())?;
    apply_surface_pass(rs, kit, dir, seed, &mut chunk)?;
    Ok(chunk)
}

/// SurfaceSystem.buildSurface for one chunk (shared RandomState/kit).
pub fn apply_surface_pass(
    rs: &mut RandomState,
    kit: &mut StageKit,
    _dir: &WorldgenDir,
    seed: i64,
    chunk: &mut FillerChunk,
) -> Result<(), String> {
    // fresh condition instances per chunk (Java ruleSource.apply(context))
    kit.rule_set.reset_caches();
    let default_block = chunk.state_table.intern_canonical(&rs.settings.default_block);
    let zoom_seed = crate::biomes::biome_zoom_seed(seed);
    let source = RefCell::new(BiomeSource::new(rs));
    {
        let mut ctx = SurfaceContext::new(&kit.system, rs, &kit.biome_noise, &kit.facts, &source, zoom_seed);
        ctx.default_block = default_block;
        let mut cols = ChunkColumns { chunk };
        build_surface(&mut ctx, &kit.rule_set.root, &mut cols, default_block);
    }
    Ok(())
}

/// Generate one chunk at CARVERS status (beardifier via kit.beard, see
/// generate_surface_chunk).
pub fn generate_carvers_chunk(
    rs: &mut RandomState,
    kit: &mut StageKit,
    dir: &WorldgenDir,
    seed: i64,
    cx: i32,
    cz: i32,
) -> Result<FillerChunk, String> {
    let mut chunk = generate_surface_chunk(rs, kit, dir, seed, cx, cz)?;
    apply_carvers_pass(rs, kit, dir, seed, &mut chunk)?;
    Ok(chunk)
}

/// applyCarvers for one chunk (shared kit; the aquifer is rebuilt per chunk
/// exactly like NoiseChunk.aquifer() — bound to THIS chunk's bounds; the
/// cache content is position-derived and deterministic, see carvers.rs).
pub fn apply_carvers_pass(
    rs: &mut RandomState,
    kit: &mut StageKit,
    _dir: &WorldgenDir,
    seed: i64,
    chunk: &mut FillerChunk,
) -> Result<(), String> {
    kit.rule_set.reset_caches();
    let default_block = chunk.state_table.intern_canonical(&rs.settings.default_block);
    let zoom_seed = crate::biomes::biome_zoom_seed(seed);
    let source = RefCell::new(BiomeSource::new(rs));
    let air = chunk.state_table.intern("minecraft:air", &[]);
    let water = chunk.state_table.intern("minecraft:water", &[("level", "0")]);
    let lava = chunk.state_table.intern("minecraft:lava", &[("level", "0")]);
    // aquifer for the CENTER chunk (min/max block bounds), same wiring as
    // filler.rs (aquiferRandom = worldgen.fromHashOf("minecraft:aquifer"))
    let mut base = crate::xoroshiro::XoroshiroRandomSource::new(seed);
    let worldgen = base.fork_positional();
    let mut aquifer_src = worldgen.from_hash_of("minecraft:aquifer");
    let aquifer_factory = aquifer_src.fork_positional();
    let picker = crate::aquifer::GlobalFluidPicker {
        sea_level: rs.settings.sea_level,
    };
    let min_block_x = chunk.chunk_min_x;
    let max_block_x = min_block_x + 15;
    let min_block_z = chunk.chunk_min_z;
    let max_block_z = min_block_z + 15;
    let aquifer = crate::aquifer::NoiseBasedAquifer::new(
        &rs.bank,
        &rs.router,
        aquifer_factory,
        rs.settings.min_y,
        rs.settings.height,
        min_block_x,
        max_block_x,
        min_block_z,
        max_block_z,
        picker,
    );
    let mut ctx = SurfaceContext::new(&kit.system, rs, &kit.biome_noise, &kit.facts, &source, zoom_seed);
    ctx.default_block = default_block;
    // biome -> carver refs for the DIRECT corner biome at y=0
    let facts = &kit.facts;
    let biome_carvers = |nx: i32, nz: i32| -> Vec<String> {
        let mut src = source.borrow_mut();
        let qx = nx * 16 >> 2;
        let qz = nz * 16 >> 2;
        let biome = src.get_noise_biome(qx, 0, qz).to_string();
        drop(src);
        facts.get(&biome).map(|f| f.carvers.clone()).unwrap_or_default()
    };
    let rule_root = &kit.rule_set.root;
    let k = CarverKit {
        configs_by_ref: kit.carver_configs.clone(),
        rs,
        system: &kit.system,
        rule_root,
        facts,
        biome_noise: &kit.biome_noise,
        min_gen_y: rs.settings.min_y,
        gen_depth: rs.settings.height,
        air,
        water,
        lava,
    };
    apply_carvers(chunk, seed, &k, &mut ctx, aquifer, &biome_carvers);
    Ok(())
}

/// Session 6 bisect rig: replicate the build_surface walk for ONE chunk and
/// emit the same TSV the Java /goldensurface capture writes (VectorCapture).
#[allow(clippy::too_many_arguments)]
pub fn trace_surface(
    rs: &mut RandomState,
    kit: &mut StageKit,
    _dir: &WorldgenDir,
    seed: i64,
    cx: i32,
    cz: i32,
) -> Result<String, String> {
    use std::fmt::Write as _;
    kit.rule_set.reset_caches();
    let mut chunk = generate_noise_chunk(rs, seed, cx, cz)?;
    let default_block = chunk.state_table.intern_canonical(&rs.settings.default_block);
    let zoom_seed = crate::biomes::biome_zoom_seed(seed);
    let source = RefCell::new(BiomeSource::new(rs));
    let mut ctx = SurfaceContext::new(&kit.system, rs, &kit.biome_noise, &kit.facts, &source, zoom_seed);
    ctx.default_block = default_block;
    let mut cols = ChunkColumns { chunk: &mut chunk };
    let mut out = String::from(
        "x\tz\ty\tbiome\tsurfaceDepth\tminSurfaceLevel\tsecondary\twaterHeight\tstoneAbove\tstoneBelow\treplacement\n",
    );
    let min_block_x = cx << 4;
    let min_block_z = cz << 4;
    let min_y = cols.chunk.min_y;
    for i in 0..16i32 {
        for i1 in 0..16i32 {
            let x = min_block_x + i;
            let z = min_block_z + i1;
            let i5 = cols.height_wg(i, i1) + 1;
            ctx.update_xz(x, z);
            let mut i6 = 0i32;
            let mut i7 = i32::MIN;
            let mut i8 = i32::MAX;
            let mut y = i5;
            while y >= min_y {
                let block = cols.get_block(x, y, z);
                if crate::surface_rules::is_air_state(block, &cols.chunk.state_table) {
                    i6 = 0;
                    i7 = i32::MIN;
                    y -= 1;
                    continue;
                }
                if crate::surface_rules::is_fluid_state(block, &cols.chunk.state_table) {
                    if i7 != i32::MIN {
                        y -= 1;
                        continue;
                    }
                    i7 = y + 1;
                    y -= 1;
                    continue;
                }
                if i8 >= y {
                    i8 = -32512;
                    let mut i10 = y - 1;
                    while i10 >= min_y - 1 {
                        let below = cols.get_block(x, i10, z);
                        if crate::surface_rules::is_stone_state(below, &cols.chunk.state_table) {
                            i10 -= 1;
                            continue;
                        }
                        i8 = i10 + 1;
                        break;
                    }
                }
                i6 += 1;
                ctx.update_y(i6, y - i8 + 1, i7, x, y, z);
                let replacement = if block == default_block {
                    kit.rule_set
                        .root
                        .try_apply(&mut ctx, &cols)
                        .map(|s| s.replace(['[', ']', '{', '}', '=', ','], ""))
                        .unwrap_or_else(|| "-".to_string())
                } else {
                    "-".to_string()
                };
                let min_surface = ctx.get_min_surface_level();
                let secondary = ctx.get_surface_secondary();
                let _ = writeln!(
                    out,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    x,
                    z,
                    y,
                    ctx.biome.clone().unwrap_or_default(),
                    ctx.surface_depth,
                    min_surface,
                    secondary,
                    ctx.water_height,
                    ctx.stone_depth_above,
                    ctx.stone_depth_below,
                    replacement,
                );
                if let Some(new_state) = if block == default_block {
                    kit.rule_set.root.try_apply(&mut ctx, &cols)
                } else {
                    None
                } {
                    let id = cols.chunk.state_table.intern_canonical(&new_state);
                    cols.set_block(x, y, z, id);
                }
                y -= 1;
            }
        }
    }
    Ok(out)
}
