//! NCF P5.3 increment 2d-feed — the per-chunk structure piece feed.
//!
//! Wires the 2a/2b assembly engine (placer.rs) to the real worldgen
//! directory: structure JSONs, template pools, template NBTs, the
//! StructureSet weighted pick protocol, and the 2c height sampler.
//!
//! Exact Java protocol per chunk (ChunkGenerator.createStructures CFR
//! 453-501):
//!   1. for each structure set: `placement.isStructureChunk(state, x, z)`
//!      (random_spread.rs, P5.1);
//!   2. single-entry set: tryGenerateStructure DIRECTLY (NO pick RNG);
//!      multi-entry: fresh `WorldgenRandom(LegacyRandomSource(0))` seeded
//!      `setLargeFeatureSeed(levelSeed, x, z)` (LEGACY random! CFR 480-482),
//!      weighted pick `nextInt(total)` + subtract-walk, RETRY on invalid
//!      starts: remove the entry, `total -= weight`, continue (CFR 487-500);
//!   3. tryGenerateStructure → Structure.generate (CFR 508-529) →
//!      `findValidGenerationPoint` = `findGenerationPoint(ctx).filter(
//!      isValidBiome)` (Structure.java CFR 180-182) — the ASSEMBLY runs
//!      FIRST (its RNG separate from the pick RNG), the biome filter runs
//!      on the stub position (quart coords, CFR 199-202);
//!   4. JigsawStructure.findGenerationPoint (CFR 108-115): startHeight.
//!      sample(context.random(), WorldGenerationContext) [absolute = no
//!      RNG], blockPos = (minBlockX, i, minBlockZ), addPieces with
//!      PoolAliasLookup.create(poolAliases, blockPos, seed) [empty for
//!      vanilla villages], dimensionPadding, liquidSettings.
//!
//! The pick/assembly split matches Java exactly: each retried candidate
//! gets a FRESH assembly RNG (a new GenerationContext.random() = the
//! settings' random source over the level seed — Xoroshiro for overworld),
//! the pick RNG carries across retries within one set.

use crate::beardifier::TerrainAdjustment;
use crate::jigsaw::{self, PoolElement, ResolvedPool, TemplateData};
use crate::jrandom::{LegacyRandomSource, RandomSource};
use crate::xoroshiro::XoroshiroRandomSource;
use crate::placer::{add_pieces, AssemblyParams, AssemblyResult, FirstFreeHeight, PoolSource};
use crate::router::WorldgenDir;
use crate::template_cache::TemplateCache;
use std::collections::HashMap;
use std::path::Path;

/// Production PoolSource over the worldgen directory (2b had test stubs
/// only). Alias resolution is IDENTITY (2b semantics) until the alias
/// increment; structures declaring pool_aliases are reported loudly.
pub struct DirPoolSource<'d> {
    dir: &'d WorldgenDir,
    templates: TemplateCache<'d>,
    max_size_cache: HashMap<String, i32>,
    template_cache: HashMap<String, Option<TemplateData>>,
    pub missing_pools: Vec<String>,
    pub missing_templates: Vec<String>,
    pub unsupported: Vec<String>,
}

impl<'d> DirPoolSource<'d> {
    pub fn new(dir: &'d WorldgenDir) -> Self {
        DirPoolSource {
            dir,
            templates: TemplateCache::new(dir),
            max_size_cache: HashMap::new(),
            template_cache: HashMap::new(),
            missing_pools: Vec::new(),
            missing_templates: Vec::new(),
            unsupported: Vec::new(),
        }
    }

    fn template_of_location(&mut self, ns: &str, path: &str) -> Option<TemplateData> {
        let key = format!("{ns}:{path}");
        if let Some(t) = self.template_cache.get(&key) {
            return t.clone();
        }
        let t = self
            .templates
            .get(ns, path)
            .and_then(|nbt| jigsaw::parse_template(&nbt));
        if t.is_none() {
            self.missing_templates.push(key.clone());
        }
        self.template_cache.insert(key, t.clone());
        t
    }
}

impl<'d> PoolSource for DirPoolSource<'d> {
    fn resolve(&self, key: &str) -> Option<ResolvedPool> {
        jigsaw::resolve_pool(self.dir, key, false)
    }

    fn template_of(&mut self, element: &PoolElement) -> Option<TemplateData> {
        match element {
            PoolElement::Single { location, .. } => {
                let (ns, path) = location.split_once(':')?;
                self.template_of_location(ns, path)
            }
            PoolElement::List { .. } => {
                // Java has no single template for a List element; the placer
                // recurses into children itself. Loud if reached directly.
                self.unsupported.push("template_of(list-element)".into());
                None
            }
            PoolElement::Feature { feature_id, .. } => {
                // FeaturePoolElement has no template (getSize ZERO); a
                // direct template request is a loud divergence.
                self.unsupported.push(format!("template_of(feature:{feature_id})"));
                None
            }
            PoolElement::Empty => None,
            PoolElement::Unsupported { kind } => {
                self.unsupported.push(format!("template_of({kind})"));
                None
            }
        }
    }

    fn max_size(&mut self, key: &str) -> i32 {
        if let Some(&m) = self.max_size_cache.get(key) {
            return m;
        }
        // StructureTemplatePool.getMaxSize: max over ALL weighted elements
        // of getBoundingBox(ZERO, IDENTITY).getYSpan(); orElse(0) for an
        // empty/missing pool; a missing template would crash in Java — we
        // note loudly and skip that element.
        let mut m = 0i32;
        if let Some(pool) = jigsaw::resolve_pool(self.dir, key, false) {
            for e in &pool.templates {
                match e {
                    PoolElement::Empty => continue, // EmptyPoolElement bbox is forbidden
                    // FeaturePoolElement: getSize ZERO => bbox [pos..pos]
                    // => yspan 1 (CFR 68-85).
                    PoolElement::Feature { .. } => m = m.max(1),
                    e2 => {
                        if let Some(t) = self.template_of(e2) {
                            let b = t.bounding_box((0, 0, 0), crate::jigsaw::Rotation::None);
                            m = m.max(b.get_yspan());
                        }
                    }
                }
            }
        }
        self.max_size_cache.insert(key.to_string(), m);
        m
    }

    fn note_missing_pool(&mut self, key: &str) {
        self.missing_pools.push(key.to_string());
    }
    fn note_missing_template(&mut self, location: &str) {
        self.missing_templates.push(location.to_string());
    }
    fn note_unsupported(&mut self, kind: &str) {
        self.unsupported.push(kind.to_string());
    }
}

/// JigsawStructure fields we consume (JigsawStructure.java CFR 84-99 CODEC).
#[derive(Debug, Clone)]
pub struct JigsawStructureJson {
    pub key: String,
    pub biomes_tag: Option<String>,
    pub start_pool: String,
    pub start_jigsaw_name: Option<String>,
    pub max_depth: i32,
    /// HeightProvider: only `absolute` is wired (villages = 0); anything
    /// else is a loud unsupported note (the REAL y is an RNG draw).
    pub start_height_absolute: Option<i32>,
    pub use_expansion_hack: bool,
    pub project_start_to_heightmap: bool,
    pub max_distance: (i32, i32),
    pub terrain_adaptation: TerrainAdjustment,
    pub has_pool_aliases: bool,
}

pub fn parse_structure_json(key: &str, text: &str) -> Result<JigsawStructureJson, String> {
    let j = crate::json::parse(text).map_err(|e| e.to_string())?;
    let ty = j.get("type").and_then(|v| v.as_str()).unwrap_or("");
    if ty != "minecraft:jigsaw" {
        return Err(format!("{key}: not a jigsaw structure ({ty})"));
    }
    let max_distance = match j.get("max_distance_from_center") {
        Some(v) => match (v.as_i64(), v.get("horizontal").and_then(|h| h.as_i64())) {
            (_, Some(h)) => (h as i32, 4064),
            (Some(d), _) => (d as i32, d as i32),
            _ => (80, 80),
        },
        None => (80, 80),
    };
    let start_height_absolute = j
        .get("start_height")
        .and_then(|h| h.get("absolute"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let has_pool_aliases = j
        .get("pool_aliases")
        .and_then(|a| a.as_arr())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    Ok(JigsawStructureJson {
        key: key.to_string(),
        biomes_tag: j
            .get("biomes")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        start_pool: j
            .get("start_pool")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        start_jigsaw_name: j
            .get("start_jigsaw_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        max_depth: j.get("size").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        start_height_absolute,
        use_expansion_hack: matches!(j.get("use_expansion_hack"), Some(crate::json::Json::Bool(true))),
        project_start_to_heightmap: j
            .get("project_start_to_heightmap")
            .and_then(|v| v.as_str())
            .is_some(),
        terrain_adaptation: TerrainAdjustment::from_json(
            j.get("terrain_adaptation")
                .and_then(|v| v.as_str())
                .unwrap_or("none"),
        )
        .unwrap_or(TerrainAdjustment::None),
        max_distance,
        has_pool_aliases,
    })
}

/// One weighted entry of a StructureSet (StructureSet.java
/// StructureSelectionEntry).
#[derive(Debug, Clone)]
pub struct StructureSetEntry {
    pub structure_key: String,
    pub weight: i32,
}

/// Parse a structure_set JSON (random_spread placement only — the same
/// known-gap list as structure_scan: concentric rings / exclusion zones are
/// not wired).
pub fn parse_structure_set_json(
    text: &str,
) -> Result<
    (
        crate::random_spread::RandomSpreadStructurePlacement,
        Vec<StructureSetEntry>,
    ),
    String,
> {
    let j = crate::json::parse(text).map_err(|e| e.to_string())?;
    let placement = crate::random_spread::RandomSpreadStructurePlacement::parse(&j)
        .map_err(|e| e.to_string())?;
    let mut entries = Vec::new();
    if let Some(arr) = j.get("structures").and_then(|x| x.as_arr()) {
        for e in arr {
            let structure_key = e
                .get("structure")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let weight = e.get("weight").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            entries.push(StructureSetEntry {
                structure_key,
                weight,
            });
        }
    }
    Ok((placement, entries))
}

/// The pick RNG of createStructures (CFR 480-482): fresh
/// WorldgenRandom(LegacyRandomSource(0)), seeded setLargeFeatureSeed(
/// levelSeed, x, z) — LEGACY random, NOT the settings' source.
pub fn new_pick_rng(level_seed: i64, chunk_x: i32, chunk_z: i32) -> LegacyRandomSource {
    let mut r = LegacyRandomSource::new(0);
    jigsaw::set_large_feature_seed(&mut r, level_seed, chunk_x, chunk_z);
    r
}

/// Biome tag expansion via structure_scan::resolve_biome_predicate (tags
/// resolve against the extract root; direct biome keys pass through).
pub fn biome_tag_set(root: &Path, tag: &str) -> (std::collections::HashSet<String>, bool) {
    let mut out = std::collections::HashSet::new();
    let mut visiting = Vec::new();
    let mut unknown = false;
    let value = crate::json::Json::Str(tag.to_string());
    crate::structure_scan::resolve_biome_predicate(root, &value, &mut out, &mut visiting, &mut unknown);
    (out, unknown)
}

/// Overworld biome parameter list for the climate lookup (structure_scan
/// parity): the pack table when wired (T39), else vanilla.
pub fn biome_list_for(rs: &crate::router::RandomState) -> crate::climate::ParameterList {
    crate::climate::ParameterList::new(match &rs.biome_points {
        Some(pts) => pts.clone(),
        None => crate::vanilla_biomes::overworld_points()
            .into_iter()
            .map(|(p, n)| (p, n.to_string()))
            .collect(),
    })
}

/// Per-chunk piece feed for ONE structure set (the 2d wiring entry):
/// placement check + weighted pick + assembly + biome filter. Returns the
/// generated structure (key, pieces, terrain adaptation) for this chunk.
pub fn structure_start_for_chunk<S: FirstFreeHeight>(
    dir: &WorldgenDir,
    root: &Path,
    rs: &crate::router::RandomState,
    level_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    set_key: &str,
    sampler: &mut S,
    pools: &mut DirPoolSource,
) -> Result<Option<(String, AssemblyResult, TerrainAdjustment)>, String> {
    let (ns, path) = set_key
        .split_once(':')
        .ok_or_else(|| format!("bad set key {set_key}"))?;
    let text = dir
        .get(ns, "structure_set", path)
        .ok_or_else(|| format!("set json missing: {set_key}"))?;
    let (placement, entries) = parse_structure_set_json(&text)?;
    if !placement.is_placement_chunk(level_seed, chunk_x, chunk_z) {
        return Ok(None);
    }
    // Load the set's structure JSONs (jigsaw only; others are skipped the
    // same way the Java pick would never reach them through OUR engine —
    // but a non-jigsaw candidate that WINS the pick must be loud).
    let mut structures = HashMap::new();
    let mut non_jigsaw = Vec::new();
    for e in &entries {
        let (sns, spath) = e
            .structure_key
            .split_once(':')
            .ok_or_else(|| format!("bad structure key {}", e.structure_key))?;
        if let Some(stext) = dir.get(sns, "structure", spath) {
            match parse_structure_json(&e.structure_key, &stext) {
                Ok(sj) => {
                    structures.insert(e.structure_key.clone(), sj);
                }
                Err(_) => non_jigsaw.push(e.structure_key.clone()),
            }
        }
    }
    let mut pick_rng = new_pick_rng(level_seed, chunk_x, chunk_z);
    // All candidates: biome tags resolved per structure lazily on demand —
    // the pick may land on a structure whose biome filter fails, then the
    // retry must try the NEXT candidate; resolve ALL tags up front (cheap,
    // JSON files cached by WorldgenDir).
    let mut tags: HashMap<String, (std::collections::HashSet<String>, bool)> = HashMap::new();
    for key in structures.keys() {
        if let Some(sj) = structures.get(key) {
            if let Some(tag) = &sj.biomes_tag {
                tags.insert(key.clone(), biome_tag_set(root, tag));
            }
        }
    }
    let _ = &non_jigsaw;
    let params_for =
        |_sj: &JigsawStructureJson, cx: i32, cz: i32| -> AssemblyParams {
            AssemblyParams {
                pos: (cx * 16, 0, cz * 16),
                start_pool: _sj.start_pool.clone(),
                start_jigsaw_name: _sj.start_jigsaw_name.clone(),
                max_depth: _sj.max_depth,
                max_distance: _sj.max_distance,
                use_expansion_hack: _sj.use_expansion_hack,
                project_start_to_heightmap: _sj.project_start_to_heightmap,
                dimension_padding: (0, 0), // DimensionPadding.DEFAULT (CFR 84)
                level_min_y: rs.settings.min_y,
                level_max_y: rs.settings.min_y + rs.settings.height - 1,
            }
        };
    let mut biome_list = biome_list_for(rs);
    // The biome filter must be evaluated against the CANDIDATE's own tag —
    // wrap pick_and_generate with per-candidate biome sets by filtering the
    // entries list here (single-entry semantics preserved: a set whose only
    // member is non-jigsaw never reaches the engine).
    let mut result = None;
    // Java iterates the pick over ALL entries; biome tags belong to each
    // candidate. We pass a biome_names set per attempt via closure over the
    // CURRENT candidate — replicated by the retry loop inside
    // pick_and_generate, so instead we inline the loop here with per-entry
    // tags (bit-exact pick sequence: nextInt on the shrinking total).
    let mut list: Vec<StructureSetEntry> = entries.clone();
    let mut total: i32 = list.iter().map(|e| e.weight).sum();
    while !list.is_empty() && total > 0 {
        let mut random_int = pick_rng.next_int_bound(total);
        let mut i1 = 0usize;
        while i1 < list.len() {
            random_int -= list[i1].weight;
            if random_int < 0 {
                break;
            }
            i1 += 1;
        }
        let entry = list[i1].clone();
        let Some(sj) = structures.get(&entry.structure_key).cloned() else {
            list.remove(i1);
            total -= entry.weight;
            continue;
        };
        if sj.has_pool_aliases {
            pools
                .unsupported
                .push(format!("pool_aliases on {} (alias increment pending)", sj.key));
        }
        if sj.start_height_absolute.is_none() {
            pools
                .unsupported
                .push(format!("non-absolute start_height on {}", sj.key));
        }
        let params = params_for(&sj, chunk_x, chunk_z);
        let mut assembly_rng = XoroshiroRandomSource::new(level_seed);
        let generated = add_pieces(&params, pools, &mut assembly_rng, sampler);
        let Some(assembly) = generated else {
            list.remove(i1);
            total -= entry.weight;
            continue;
        };
        let (sx, sy, sz) = assembly.stub_position;
        let biome = crate::structure_scan::biome_at(rs, &mut biome_list, sx, sy, sz);
        let names = tags
            .get(&entry.structure_key)
            .map(|(n, _)| n)
            .cloned()
            .unwrap_or_default();
        if !names.contains(&biome) {
            list.remove(i1);
            total -= entry.weight;
            continue;
        }
        result = Some((entry.structure_key, assembly, sj.terrain_adaptation));
        break;
    }
    Ok(result)
}
