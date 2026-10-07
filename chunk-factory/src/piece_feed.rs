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
//! gets a FRESH GenerationContext whose makeRandom = WorldgenRandom(
//! LegacyRandomSource(0)) seeded setLargeFeatureSeed(levelSeed, chunkPos)
//! (1.21.10: addPieces does NOT reseed — the record field is consumed as
//! is); the pick RNG carries across retries within one set.

use crate::alias::{self, PoolAliasBinding};
use crate::beardifier::TerrainAdjustment;
use crate::jigsaw::{self, PoolElement, ResolvedPool, TemplateData};
use crate::jrandom::{LegacyRandomSource, RandomSource};
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

/// JigsawStructure.start_height (HeightProvider): absolute = the constant y
/// (no RNG); uniform absolute range = Mth.randomBetweenInclusive(
/// context.random(), min, max) = nextInt(max-min+1)+min — the FIRST assembly
/// RNG draw (JigsawStructure.findGenerationPoint CFR 97 precedes addPieces).
/// Anything else (baseline anchors / other provider types) is loud.
#[derive(Debug, Clone)]
pub enum StartHeight {
    Absolute(i32),
    UniformAbsolute { min: i32, max: i32 },
}

/// JigsawStructure fields we consume (JigsawStructure.java CFR 84-99 CODEC).
#[derive(Debug, Clone)]
pub struct JigsawStructureJson {
    pub key: String,
    pub biomes_tag: Option<String>,
    pub start_pool: String,
    pub start_jigsaw_name: Option<String>,
    pub max_depth: i32,
    /// HeightProvider: absolute / uniform(absolute) wired; anything else is a
    /// loud unsupported note (the REAL y is an RNG draw).
    pub start_height: Option<StartHeight>,
    pub use_expansion_hack: bool,
    pub project_start_to_heightmap: bool,
    pub max_distance: (i32, i32),
    pub terrain_adaptation: TerrainAdjustment,
    /// DimensionPadding (bottom, top); (0, 0) == DimensionPadding.ZERO.
    pub dimension_padding: (i32, i32),
    /// PoolAliasBinding list (PoolAliasLookup.create; empty = identity).
    pub pool_aliases: Vec<PoolAliasBinding>,
    /// pool_aliases entries whose "type" is not wired (loud divergence).
    pub unsupported_aliases: Vec<String>,
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
    // DimensionPadding codec (CFR 22-24): NON_NEGATIVE_INT => (v, v); record
    // {bottom, top} with 0 defaults. Absent => ZERO = (0, 0).
    let dimension_padding = match j.get("dimension_padding") {
        Some(v) => match v.as_i64() {
            Some(d) => (d as i32, d as i32),
            None => (
                v.get("bottom").and_then(|b| b.as_i64()).unwrap_or(0) as i32,
                v.get("top").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
            ),
        },
        None => (0, 0),
    };
    // start_height: {absolute: N} | {type: uniform, min_inclusive/max_inclusive
    // {absolute: N}} — vertical anchors other than absolute are loud.
    let start_height = j.get("start_height").map(|h| {
        if let Some(a) = h.get("absolute").and_then(|v| v.as_i64()) {
            Ok(StartHeight::Absolute(a as i32))
        } else if h.get("type").and_then(|t| t.as_str()) == Some("minecraft:uniform") {
            let mut anchor = |side: &str| -> Result<i32, String> {
                h.get(side)
                    .and_then(|s| s.get("absolute"))
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32)
                    .ok_or_else(|| format!("start_height {side}: non-absolute anchor"))
            };
            let min = anchor("min_inclusive")?;
            let max = anchor("max_inclusive")?;
            Ok(StartHeight::UniformAbsolute { min, max })
        } else {
            Err("start_height: unsupported provider".to_string())
        }
    });
    let unsupported_aliases = Vec::new();
    let mut pool_aliases = Vec::new();
    if let Some(arr) = j.get("pool_aliases").and_then(|a| a.as_arr()) {
        for a in arr {
            match PoolAliasBinding::parse_json(a) {
                Ok(b) => pool_aliases.push(b),
                Err(e) => {
                    return Err(format!("{key}: {e}"));
                }
            }
        }
    }
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
        start_height: match start_height {
            Some(Ok(v)) => Some(v),
            Some(Err(_)) => None, // loud via unsupported start_height handling below
            None => None,
        },
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
        dimension_padding,
        pool_aliases,
        unsupported_aliases,
    })
}

/// One weighted entry of a StructureSet (StructureSet.java
/// StructureSelectionEntry).
#[derive(Debug, Clone)]
pub struct StructureSetEntry {
    pub structure_key: String,
    pub weight: i32,
}

/// Per-(set, placement chunk) pick diagnostics (I8 narrowing v2): which
/// triggers make OUR pick chain potentially divergent from Java's.
/// A faithful pick (all candidates jigsaw with supported start_height/alias
/// types and resolvable pools) reproduces Java bit-for-bit — chunks whose
/// whole neighborhood is faithful are generated with the REAL Beardifier
/// and compared honestly by the gate; unfaithful neighborhoods stay I8-marked
/// (conservative direction).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PickDiag {
    /// drawn candidate whose structure JSON is not a supported jigsaw type
    /// (Java generates it through its own StructureType; we skip + retry)
    pub non_jigsaw_drawn: usize,
    /// drawn candidate with an unsupported start_height provider (crash-path
    /// fallback in Java; we skip the candidate entirely)
    pub unsupported_start_height: usize,
    /// drawn candidate carrying pool_alias bindings of unsupported types
    /// (Java applies them; our lookup degrades to identity for those keys)
    pub unsupported_alias: usize,
    /// the assembly hit a pool key absent from the extract (Java's registry
    /// may have it — extract protocol gap; the ALIASED-key raw fallback is
    /// Java-identical but lands in the same counter — over-marking direction)
    pub missing_pool: usize,
}

impl PickDiag {
    pub fn unfaithful(&self) -> bool {
        self.non_jigsaw_drawn > 0
            || self.unsupported_start_height > 0
            || self.unsupported_alias > 0
            || self.missing_pool > 0
    }
}

/// Overworld producible biome ids (the pack's biome table when T39 wired
/// one, else the vanilla preset) — the domain of Java's biome check.
fn overworld_biome_names(rs: &crate::router::RandomState) -> std::collections::HashSet<String> {
    match &rs.biome_points {
        Some(pts) => pts.iter().map(|(_, n)| n.clone()).collect(),
        None => crate::vanilla_biomes::overworld_points()
            .into_iter()
            .map(|(_, n)| n.to_string())
            .collect(),
    }
}

/// Can the drawn candidate EVER generate in the overworld? Java runs the
/// biome check on the real assembly stub — a candidate whose biome
/// predicate resolves (tags readable) to a set DISJOINT from every overworld
/// biome never generates anywhere in this dimension (e.g. the nether-gated
/// nether_fossil inside an overworld gate). Skipping it is then
/// Java-identical (Java draws it, assembles, biome-rejects, retries — the
/// pick stream sees exactly one nextInt either way) — harmless, no I8 mark.
/// `false` = potentially generable or unknown validity (conservative mark).
fn candidate_harmless_in_overworld(
    dir: &WorldgenDir,
    root: &Path,
    rs: &crate::router::RandomState,
    structure_key: &str,
) -> bool {
    let Some((sns, spath)) = structure_key.split_once(':') else {
        return false;
    };
    let Some(stext) = dir.get(sns, "structure", spath) else {
        return false;
    };
    let Ok(j) = crate::json::parse(stext) else {
        return false;
    };
    let Some(biomes) = j.get("biomes") else {
        return false;
    };
    let mut names = std::collections::HashSet::new();
    let mut visiting = Vec::new();
    let mut unknown = false;
    crate::structure_scan::resolve_biome_predicate(root, biomes, &mut names, &mut visiting, &mut unknown);
    if unknown {
        return false;
    }
    let ow = overworld_biome_names(rs);
    names.is_disjoint(&ow)
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
    diag: &mut PickDiag,
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
    let mut non_jigsaw_drawn = 0usize;
    let mut unsupported_start_height = 0usize;
    let mut unsupported_alias = 0usize;
    let params_for =
        |_sj: &JigsawStructureJson, cx: i32, cz: i32, start_y: i32, alias_map: std::collections::HashMap<String, String>| -> AssemblyParams {
            AssemblyParams {
                pos: (cx * 16, start_y, cz * 16),
                start_pool: _sj.start_pool.clone(),
                start_jigsaw_name: _sj.start_jigsaw_name.clone(),
                max_depth: _sj.max_depth,
                max_distance: _sj.max_distance,
                use_expansion_hack: _sj.use_expansion_hack,
                project_start_to_heightmap: _sj.project_start_to_heightmap,
                // parsed from the structure JSON (CFR DimensionPadding codec);
                // (0, 0) == DimensionPadding.ZERO (villages omit the field).
                dimension_padding: _sj.dimension_padding,
                level_min_y: rs.settings.min_y,
                level_max_y: rs.settings.min_y + rs.settings.height - 1,
                alias_map,
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
            // Non-jigsaw (or unparseable) candidate DRAWN: Java generates it
            // through its own StructureType — a divergence source, UNLESS the
            // candidate's biome predicate can never match an overworld biome
            // (nether-gated etc.) — then Java biome-rejects it identically.
            if !candidate_harmless_in_overworld(dir, root, rs, &entry.structure_key) {
                non_jigsaw_drawn += 1;
            }
            list.remove(i1);
            total -= entry.weight;
            continue;
        };
        if !sj.unsupported_aliases.is_empty() {
            if !candidate_harmless_in_overworld(dir, root, rs, &sj.key) {
                unsupported_alias += 1;
            }
        }
        for a in &sj.unsupported_aliases {
            pools.unsupported.push(format!("pool_aliases on {}: {a}", sj.key));
        }
        let Some(sh) = sj.start_height.clone() else {
            if candidate_harmless_in_overworld(dir, root, rs, &sj.key) {
                list.remove(i1);
                total -= entry.weight;
                continue;
            }
            unsupported_start_height += 1;
            pools
                .unsupported
                .push(format!("unsupported start_height on {} (skipped)", sj.key));
            list.remove(i1);
            total -= entry.weight;
            continue;
        };
        let params_pos_y;
        let alias_map;
        // Structure.GenerationContext.makeRandom (1.21.10 CFR — the ROOT of
        // the first piece_dump divergence): the assembly random is a fresh
        // WorldgenRandom(LegacyRandomSource(0L)) seeded setLargeFeatureSeed
        // (levelSeed, chunkPos.x, chunkPos.z) — constructed PER CANDIDATE
        // (Structure.generate makes a new GenerationContext), and 1.21.10
        // JigsawPlacement.addPieces does NOT reseed (line 71 consumes the
        // record's field). The xoroshiro source is NOT used here.
        let mut assembly_rng = LegacyRandomSource::new(0);
        jigsaw::set_large_feature_seed(&mut assembly_rng, level_seed, chunk_x, chunk_z);
        // JigsawStructure.findGenerationPoint CFR 96-98: startHeight.sample(
        // context.random(), ...) BEFORE addPieces' Rotation.getRandom.
        let start_y = match &sh {
            StartHeight::Absolute(v) => *v,
            StartHeight::UniformAbsolute { min, max } => {
                // Mth.randomBetweenInclusive: nextInt(max - min + 1) + min.
                assembly_rng.next_int_bound(max - min + 1) + min
            }
        };
        params_pos_y = start_y;
        // PoolAliasLookup.create(poolAliases, blockPos=(minBlockX, startY,
        // minBlockZ), context.seed()=levelSeed) — an INDEPENDENT random
        // lineage (never touches the assembly/pick streams).
        let (map, dup) = alias::build_lookup(&sj.pool_aliases, (chunk_x * 16, start_y, chunk_z * 16), level_seed);
        for d in dup {
            pools
                .note_unsupported(&format!("duplicate alias key {} on {}", d.as_str(), sj.key.as_str()));
        }
        alias_map = map;
        let params = params_for(&sj, chunk_x, chunk_z, params_pos_y, alias_map);
        let mp_before = pools.missing_pools.len();
        let generated = add_pieces(&params, pools, &mut assembly_rng, sampler);
        diag.missing_pool += pools.missing_pools.len() - mp_before;
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
    diag.non_jigsaw_drawn = non_jigsaw_drawn;
    diag.unsupported_start_height = unsupported_start_height;
    diag.unsupported_alias = unsupported_alias;
    Ok(result)
}

// ---------------------------------------------------------------------------
// P5.3 increment 3 — the per-chunk Beardifier feed (Job 441690).
// ---------------------------------------------------------------------------

/// The per-chunk Beardifier builder: replicates the exact Java protocol that
/// connects jigsaw assembly to the noise chunk's density function.
///
/// Oracle chain (purpur-1.21.10 decompiles):
/// 1. NoiseBasedChunkGenerator.createNoiseChunk (CFR 119-121):
///    NoiseChunk.forChunk(chunk, random, Beardifier.forStructuresInChunk(
///    structureManager, chunk.getPos()), ...) — the beardifier is built ONCE
///    per chunk, before any noise evaluation.
/// 2. ChunkGenerator.createReferences (CFR decomp441b 532-565): chunk C
///    references every placement chunk P in [cx-8..cx+8] x [cz-8..cz+8]
///    whose start's whole-structure bounding box intersects C's 16x16 X/Z
///    column — BoundingBox.intersects(minBlockX, minBlockZ, minBlockX+15,
///    minBlockZ+15), the 4-arg X/Z-only form with INCLUSIVE overlap
///    (maxX >= minX && minX <= maxX && maxZ >= minZ && minZ <= maxZ).
/// 3. StructureManager.startsForStructure (decomp441b 62-75): the Beardifier
///    sees exactly the starts referenced by C (loaded from the start chunk
///    at STRUCTURE_STARTS; isValid() = pieces non-empty), filtered by
///    structure.terrainAdaptation() != NONE.
/// 4. Beardifier.forStructuresInChunk (decomp441 30-70): per start, per
///    piece — keep pieces with isCloseToChunk(chunkPos, 12) (4-arg X/Z
///    intersect against [minBlockX-12, minBlockX+15+12] x [minBlockZ-12,
///    minBlockZ+15+12]); PoolElementStructurePiece contributes
///    Rigid(box, STRUCTURE-level adjustment, groundLevelDelta) only when
///    the element projection == RIGID (non-rigid pieces contribute NOTHING
///    to the piece list but their junctions are still collected); junctions
///    are filtered by the EXCLUSIVE window `sourceX <= minBlockX - 12 ||
///    sourceZ <= minBlockZ - 12 || sourceX >= minBlockX + 15 + 12 ||
///    sourceZ >= minBlockZ + 15 + 12 -> reject`; non-PoolElement pieces
///    contribute Rigid(box, adjustment, 0) (not reachable through this
///    feed — the engine assembles jigsaw structures only; loud notes).
///    Union box = encapsulating over the INCLUDED piece boxes and junction
///    point boxes, then inflatedBy(24). Empty union => Beardifier.EMPTY.
pub struct BeardFeed<'d, 'r> {
    dir: &'d WorldgenDir,
    root: &'r Path,
    rs: &'r crate::router::RandomState,
    level_seed: i64,
    /// Structure sets carrying >= 1 structure with terrain_adaptation != none
    /// (random_spread placement only, mirroring structure_scan's honest gap).
    adapting_sets: Vec<String>,
    pools: DirPoolSource<'d>,
    /// structure_start_for_chunk memo: assembly is deterministic per
    /// (set, levelSeed, placementChunk) and every chunk C within the
    /// createReferences +-8 window re-queries the same P. The pick
    /// diagnostics ride along (I8 narrowing v2).
    start_cache: HashMap<
        (String, i32, i32),
        (Option<(String, AssemblyResult, TerrainAdjustment)>, PickDiag),
    >,
}

impl<'d, 'r> BeardFeed<'d, 'r> {
    pub fn new(dir: &'d WorldgenDir, root: &'r Path, rs: &'r crate::router::RandomState, level_seed: i64) -> Self {
        let mut adapting_sets = Vec::new();
        for ns in dir.namespaces() {
            for set_name in dir.list(&ns, "structure_set") {
                let Some(text) = dir.get(&ns, "structure_set", &set_name) else {
                    continue;
                };
                let Ok(j) = crate::json::parse(text) else {
                    continue;
                };
                let placement_ok = j
                    .get("placement")
                    .and_then(|p| p.get("type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("minecraft:random_spread")
                    == "minecraft:random_spread";
                if !placement_ok {
                    // Loud honest gap (same protocol as structure_scan).
                    eprintln!(
                        "[beard-feed] {}: non-random_spread placement — not scanned",
                        set_name
                    );
                    continue;
                }
                let has_adapting = j
                    .get("structures")
                    .and_then(|s| s.as_arr())
                    .map(|entries| {
                        entries.iter().any(|e| {
                            e.get("structure")
                                .and_then(|v| v.as_str())
                                .and_then(|k| {
                                    let (sns, spath) = k.split_once(':')?;
                                    let stext = dir.get(sns, "structure", spath)?;
                                    let sj = crate::json::parse(stext).ok()?;
                                    sj.get("terrain_adaptation")
                                        .and_then(|v| v.as_str())
                                        .map(|a| a != "none")
                                })
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false);
                if has_adapting {
                    adapting_sets.push(format!("{ns}:{set_name}"));
                }
            }
        }
        BeardFeed {
            dir,
            root,
            rs,
            level_seed,
            adapting_sets,
            pools: DirPoolSource::new(dir),
            start_cache: HashMap::new(),
        }
    }

    /// Assembly diagnostics accumulated by the DirPoolSource (missing pools
    /// / templates / unsupported pieces — the narrowed-I8 inputs, directive 1).
    pub fn pools(&self) -> &DirPoolSource<'d> {
        &self.pools
    }

    /// The adapting set keys considered by this feed.
    pub fn adapting_sets(&self) -> &[String] {
        &self.adapting_sets
    }

    fn start_for<S: FirstFreeHeight>(
        &mut self,
        sampler: &mut S,
        set_key: &str,
        px: i32,
        pz: i32,
    ) -> (Option<(String, AssemblyResult, TerrainAdjustment)>, PickDiag) {
        let key = (set_key.to_string(), px, pz);
        if let Some(hit) = self.start_cache.get(&key) {
            return hit.clone();
        }
        let mut diag = PickDiag::default();
        let hit = match structure_start_for_chunk(
            self.dir, self.root, self.rs, self.level_seed, px, pz, set_key, sampler, &mut self.pools,
            &mut diag,
        ) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[beard-feed] {set_key} @ ({px},{pz}): {e}");
                None
            }
        };
        if std::env::var("NCF_BEARD_PROBE").is_ok() && diag.unfaithful() {
            eprintln!(
                "[pick-diag] {set_key} @ ({px},{pz}): non_jigsaw {} uns_sh {} uns_alias {} miss_pool {} -> {:?}",
                diag.non_jigsaw_drawn,
                diag.unsupported_start_height,
                diag.unsupported_alias,
                diag.missing_pool,
                hit.as_ref().map(|(k, _, _)| k)
            );
        }
        self.start_cache.insert(key, (hit.clone(), diag.clone()));
        (hit, diag)
    }

    /// Beardifier.forStructuresInChunk over the createReferences +-8 scan for
    /// chunk (chunk_x, chunk_z), with the I8-narrowing reach diagnostics:
    /// `referenced` = adapting starts whose box touches this chunk's column;
    /// `unfaithful` = SOME (set, P) in the +-8 neighborhood had a pick
    /// trigger that may diverge from Java (see PickDiag) — regardless of the
    /// reference outcome (an unfaithful pick can hide a Java start whose box
    /// WOULD touch this chunk); `multi` = two or more starts referenced (the
    /// Java HashMap sum-order edge).
    pub fn build_for_chunk_diag<S: FirstFreeHeight>(
        &mut self,
        sampler: &mut S,
        chunk_x: i32,
        chunk_z: i32,
    ) -> (crate::beardifier::Beardifier, ReachDiag) {
        let mut unfaithful = false;
        let min_block_x = chunk_x * 16;
        let min_block_z = chunk_z * 16;
        // (structure key, placement chunk) -> start — the Java reference map
        // is keyed by STRUCTURE; the same structure appearing via two sets
        // yields ONE start (createStructures overwrites per chunk map).
        let mut referenced: HashMap<(String, i32, i32), (AssemblyResult, TerrainAdjustment)> =
            HashMap::new();
        let sets = self.adapting_sets.clone();
        for set_key in &sets {
            for px in chunk_x - 8..=chunk_x + 8 {
                for pz in chunk_z - 8..=chunk_z + 8 {
                    let (hit, diag) = self.start_for(sampler, set_key, px, pz);
                    if diag.unfaithful() {
                        unfaithful = true;
                    }
                    let Some((structure_key, assembly, adj)) = hit else {
                        continue;
                    };
                    if adj == TerrainAdjustment::None {
                        // startsForStructure predicate: adaptation != NONE.
                        continue;
                    }
                    // Reference check: start's WHOLE-STRUCTURE box vs the
                    // chunk's 16x16 column, 4-arg X/Z intersect (inclusive).
                    let mut union: Option<crate::beardifier::InclusiveBox> = None;
                    for piece in &assembly.pieces {
                        let b = &piece.bounding_box;
                        union = Some(match union {
                            Some(u) => crate::beardifier::InclusiveBox::encapsulating(&u, b),
                            None => *b,
                        });
                    }
                    let Some(start_box) = union else { continue }; // invalid start (no pieces)
                    let touches = start_box.max_x >= min_block_x
                        && start_box.min_x <= min_block_x + 15
                        && start_box.max_z >= min_block_z
                        && start_box.min_z <= min_block_z + 15;
                    if !touches {
                        continue;
                    }
                    let map_key = (structure_key, px, pz);
                    match &referenced.get(&map_key) {
                        Some(_) => {
                            // Same structure + start chunk via two sets: Java
                            // overwrites with a bit-identical regeneration
                            // (same seed/protocol) — keep the first, note it.
                            eprintln!(
                                "[beard-feed] duplicate start {} @ ({px},{pz}) via second set — kept first (bit-identical regeneration)",
                                map_key.0
                            );
                        }
                        None => {
                            referenced.insert(map_key, (assembly, adj));
                        }
                    }
                }
            }
        }

        // forStructuresInChunk filter (step 4 of the header oracle chain).
        let mut rigids = Vec::new();
        let mut junctions = Vec::new();
        let mut union: Option<crate::beardifier::InclusiveBox> = None;
        // Deterministic order: chunk-position scan, then structure key. NOTE
        // (honest edge): Java's references map is a HashMap (ChunkAccess CFR
        // 114: Maps.newHashMap()) — when MULTIPLE adapting starts reference
        // the same chunk, the Java beardifier sums their contributions in
        // HASH order, which is not reproducible here; the f64 sum order can
        // differ in the last bits. Single-start chunks (the gate corpus norm)
        // are order-free. Loud if ever hit in a gate cell.
        let mut map_keys: Vec<_> = referenced.keys().cloned().collect();
        map_keys.sort_by(|a, b| (a.1, a.2, &a.0).cmp(&(b.1, b.2, &b.0)));
        for (structure_key, px, _pz) in &map_keys {
            let (assembly, adj) = &referenced[&(structure_key.clone(), *px, *_pz)];
            let _ = structure_key;
            for piece in &assembly.pieces {
                let b = &piece.bounding_box;
                // isCloseToChunk(chunkPos, 12): 4-arg X/Z intersect, INCLUSIVE.
                let close = b.max_x >= min_block_x - 12
                    && b.min_x <= min_block_x + 15 + 12
                    && b.max_z >= min_block_z - 12
                    && b.min_z <= min_block_z + 15 + 12;
                if !close {
                    continue;
                }
                if piece.element.projection() == crate::jigsaw::Projection::Rigid {
                    rigids.push(crate::beardifier::BeardRigid::new(
                        b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z, *adj,
                        piece.ground_level_delta,
                    ));
                    union = Some(match union {
                        Some(u) => crate::beardifier::InclusiveBox::encapsulating(&u, b),
                        None => *b,
                    });
                }
                for j in &piece.junctions {
                    // EXCLUSIVE window (CFR: sourceX <= minBlockX - 12 ||
                    // sourceX >= minBlockX + 15 + 12 rejected).
                    if j.source_x <= min_block_x - 12
                        || j.source_z <= min_block_z - 12
                        || j.source_x >= min_block_x + 15 + 12
                        || j.source_z >= min_block_z + 15 + 12
                    {
                        continue;
                    }
                    junctions.push(crate::beardifier::BeardJunction {
                        source_x: j.source_x,
                        source_ground_y: j.source_ground_y,
                        source_z: j.source_z,
                    });
                    let jb = crate::beardifier::InclusiveBox {
                        min_x: j.source_x,
                        min_y: j.source_ground_y,
                        min_z: j.source_z,
                        max_x: j.source_x,
                        max_y: j.source_ground_y,
                        max_z: j.source_z,
                    };
                    union = Some(match union {
                        Some(u) => crate::beardifier::InclusiveBox::encapsulating(&u, &jb),
                        None => jb,
                    });
                }
            }
        }
        let referenced = referenced.len();
        (
            crate::beardifier::Beardifier::new(rigids, junctions, union),
            ReachDiag { referenced, unfaithful },
        )
    }

    /// Convenience wrapper (tests / tools that only need the Beardifier).
    pub fn build_for_chunk<S: FirstFreeHeight>(
        &mut self,
        sampler: &mut S,
        chunk_x: i32,
        chunk_z: i32,
    ) -> crate::beardifier::Beardifier {
        self.build_for_chunk_diag(sampler, chunk_x, chunk_z).0
    }
}

/// I8-narrowing reach diagnostics for one chunk (see build_for_chunk_diag).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReachDiag {
    pub referenced: usize,
    pub unfaithful: bool,
}

#[cfg(test)]
mod beard_feed_tests {
    use super::*;
    use crate::beardifier::{InclusiveBox, TerrainAdjustment};
    use crate::height_feed::ColumnHeightSource;
    use crate::router::{RandomState, WorldgenDir};
    use std::path::Path;

    const SEED: i64 = 3053459;

    fn extract_dir() -> Option<(WorldgenDir, std::path::PathBuf)> {
        let root = crate::test_support::extract_root()?;
        let dir = WorldgenDir::load(&root).expect("worldgen dir");
        Some((dir, root))
    }

    /// The whole-structure union box from the GoldenDumper JSON.
    fn oracle_pieces(json: &crate::json::Json) -> Vec<InclusiveBox> {
        let mut out = Vec::new();
        for start in json.get("starts").and_then(|s| s.as_arr()).unwrap_or_default() {
            for piece in start.get("pieces").and_then(|p| p.as_arr()).unwrap_or_default() {
                let b = piece.get("box").and_then(|v| v.as_arr()).unwrap();
                let nums: Vec<i32> = b.iter().filter_map(|v| v.as_i64()).map(|v| v as i32).collect();
                out.push(InclusiveBox {
                    min_x: nums[0],
                    min_y: nums[1],
                    min_z: nums[2],
                    max_x: nums[3],
                    max_y: nums[4],
                    max_z: nums[5],
                });
            }
        }
        out
    }

    fn oracle_junctions(json: &crate::json::Json) -> Vec<(i32, i32, i32)> {
        let mut out = Vec::new();
        for start in json.get("starts").and_then(|s| s.as_arr()).unwrap_or_default() {
            for piece in start.get("pieces").and_then(|p| p.as_arr()).unwrap_or_default() {
                for j in piece.get("junctions").and_then(|p| p.as_arr()).unwrap_or_default() {
                    let nums: Vec<i32> = j.as_arr().unwrap().iter().filter_map(|v| v.as_i64()).map(|v| v as i32).collect();
                    out.push((nums[0], nums[1], nums[2]));
                }
            }
        }
        out
    }

    /// Java isCloseToChunk(chunkPos, 12) — 4-arg X/Z intersect, inclusive.
    fn is_close(b: &InclusiveBox, min_block_x: i32, min_block_z: i32) -> bool {
        b.max_x >= min_block_x - 12
            && b.min_x <= min_block_x + 15 + 12
            && b.max_z >= min_block_z - 12
            && b.min_z <= min_block_z + 15 + 12
    }

    /// Java junction window — EXCLUSIVE: source ∉ [min-12] ∪ [min+27).
    fn junction_in_window(sx: i32, sz: i32, min_block_x: i32, min_block_z: i32) -> bool {
        !(sx <= min_block_x - 12
            || sz <= min_block_z - 12
            || sx >= min_block_x + 15 + 12
            || sz >= min_block_z + 15 + 12)
    }

    #[test]
    fn beard_feed_matches_golden_oracle_chunk_4_6() {
        let Some((dir, root)) = extract_dir() else {
            return; // loud skip already printed by test_support
        };
        let rs = RandomState::build_overworld(&dir, SEED).expect("build_overworld");
        let mut sampler = ColumnHeightSource::new(&rs, SEED);
        let mut feed = BeardFeed::new(&dir, &root, &rs, SEED);
        assert!(
            feed.adapting_sets().iter().any(|s| s == "minecraft:trial_chambers"),
            "trial_chambers set must be among the adapting sets"
        );
        let beard = feed.build_for_chunk(&mut sampler, 4, 6);
        assert!(!beard.is_empty(), "chunk (4,6) hosts the trial_chambers start");

        // Oracle: ci-server/golden/pieces_4_6.json (bit-exact GoldenDumper
        // capture of the trial_chambers start @ (4,6)).
        let text = match crate::test_support::golden_dir()
            .map(|g| std::fs::read_to_string(g.join("pieces_4_6.json")).expect("read oracle json"))
        {
            Some(t) => t,
            None => return, // loud skip already printed
        };
        let j = crate::json::parse(&text).unwrap();
        let (mbx, mbz) = (64, 96);
        let expected_boxes: Vec<InclusiveBox> =
            oracle_pieces(&j).into_iter().filter(|b| is_close(b, mbx, mbz)).collect();
        let expected_glds: Vec<i32> = {
            let mut v = Vec::new();
            for start in j.get("starts").and_then(|s| s.as_arr()).unwrap() {
                for piece in start.get("pieces").and_then(|p| p.as_arr()).unwrap() {
                    let b = piece.get("box").and_then(|x| x.as_arr()).unwrap();
                    let nums: Vec<i32> =
                        b.iter().filter_map(|x| x.as_i64()).map(|x| x as i32).collect();
                    let box_ = InclusiveBox {
                        min_x: nums[0],
                        min_y: nums[1],
                        min_z: nums[2],
                        max_x: nums[3],
                        max_y: nums[4],
                        max_z: nums[5],
                    };
                    if is_close(&box_, mbx, mbz) {
                        v.push(piece.get("gld").and_then(|g| g.as_i64()).unwrap() as i32);
                    }
                }
            }
            v
        };
        assert!(!expected_boxes.is_empty());

        let got = beard.pieces();
        assert_eq!(
            got.len(),
            expected_boxes.len(),
            "rigid piece count vs oracle (close-filter)"
        );
        for (g, (b, gld)) in got.iter().zip(expected_boxes.iter().zip(expected_glds.iter())) {
            assert_eq!(g.adjustment, TerrainAdjustment::Encapsulate, "trial_chambers = encapsulate");
            assert_eq!((g.min_x, g.min_y, g.min_z), (b.min_x, b.min_y, b.min_z));
            assert_eq!((g.max_x, g.max_y, g.max_z), (b.max_x, b.max_y, b.max_z));
            assert_eq!(g.ground_level_delta, *gld, "gld for box {b:?}");
        }

        // Junctions: window-filtered, order preserved.
        let expected_j: Vec<(i32, i32, i32)> = oracle_junctions(&j)
            .into_iter()
            .filter(|(sx, _, sz)| junction_in_window(*sx, *sz, mbx, mbz))
            .collect();
        let got_j: Vec<(i32, i32, i32)> =
            beard.junctions().iter().map(|j| (j.source_x, j.source_ground_y, j.source_z)).collect();
        assert_eq!(got_j, expected_j, "junction windows/order");

        // Union box: encapsulating over included pieces + junction points,
        // inflated by 24.
        let mut u: Option<InclusiveBox> = None;
        for b in &expected_boxes {
            u = Some(match u {
                Some(p) => InclusiveBox::encapsulating(&p, b),
                None => *b,
            });
        }
        for (sx, sgy, sz) in &expected_j {
            let jb = InclusiveBox {
                min_x: *sx,
                min_y: *sgy,
                min_z: *sz,
                max_x: *sx,
                max_y: *sgy,
                max_z: *sz,
            };
            u = Some(match u {
                Some(p) => InclusiveBox::encapsulating(&p, &jb),
                None => jb,
            });
        }
        assert_eq!(beard.affected(), u.map(|b| b.inflated_by(24)).as_ref());
    }

    #[test]
    fn beard_feed_reference_semantics_neighbor_chunk() {
        // Chunk (3,6): NOT the placement chunk, but the trial_chambers start
        // box touches its 16x16 column -> createReferences marks it and the
        // feed must see the start (pieces near it included).
        let Some((dir, root)) = extract_dir() else {
            return; // loud skip already printed by test_support
        };
        let rs = RandomState::build_overworld(&dir, SEED).expect("build_overworld");
        let mut sampler = ColumnHeightSource::new(&rs, SEED);
        let mut feed = BeardFeed::new(&dir, &root, &rs, SEED);
        let beard = feed.build_for_chunk(&mut sampler, 3, 6);
        assert!(!beard.is_empty(), "(3,6) must reference the (4,6) start");

        // Oracle expectation for the trial subset at (3,6).
        let text = std::fs::read_to_string(
            crate::test_support::golden_dir().expect("golden dir").join("pieces_4_6.json"),
        )
        .unwrap();
        let j = crate::json::parse(&text).unwrap();
        let (mbx, mbz) = (48, 96);
        let expected: Vec<InclusiveBox> =
            oracle_pieces(&j).into_iter().filter(|b| is_close(b, mbx, mbz)).collect();
        let got_encapsulate: Vec<&crate::beardifier::BeardRigid> = beard
            .pieces()
            .iter()
            .filter(|p| p.adjustment == TerrainAdjustment::Encapsulate)
            .collect();
        assert_eq!(
            got_encapsulate.len(),
            expected.len(),
            "trial subset at (3,6) (other adapting starts would differ in adjustment)"
        );
        for (g, b) in got_encapsulate.iter().zip(expected.iter()) {
            assert_eq!((g.min_x, g.min_y, g.min_z, g.max_x, g.max_y, g.max_z),
                       (b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z));
        }
    }

    #[test]
    fn beard_feed_necessary_condition_and_determinism() {
        // Protocol-direction check: a NON-EMPTY beardifier at chunk C implies
        // some adapting placement chunk within the createReferences +-8
        // window (no start -> no pieces). Also: empty chunks EXIST in a scan
        // range (the biome filter kills most placements), and a fresh feed
        // reproduces the same rigids bit-for-bit (start cache determinism).
        let Some((dir, root)) = extract_dir() else {
            return; // loud skip already printed by test_support
        };
        let rs = RandomState::build_overworld(&dir, SEED).expect("build_overworld");
        let mut sampler = ColumnHeightSource::new(&rs, SEED);
        let mut feed = BeardFeed::new(&dir, &root, &rs, SEED);
        let mut empties = 0usize;
        let mut nonempty = 0usize;
        for (cx, cz) in [(30, 30), (35, 30), (40, 35), (-30, 25), (25, -40), (0, 60), (-45, -30)] {
            let beard = feed.build_for_chunk(&mut sampler, cx, cz);
            if beard.is_empty() {
                empties += 1;
            } else {
                nonempty += 1;
                let mut any_placement = false;
                for set_key in feed.adapting_sets() {
                    let Some(text) =
                        dir.get("minecraft", "structure_set", set_key.split_once(':').unwrap().1)
                    else {
                        continue;
                    };
                    let Ok(j) = crate::json::parse(text) else { continue };
                    let Ok(placement) =
                        crate::random_spread::RandomSpreadStructurePlacement::parse(&j)
                    else {
                        continue;
                    };
                    for px in cx - 8..=cx + 8 {
                        for pz in cz - 8..=cz + 8 {
                            if placement.is_placement_chunk(SEED, px, pz) {
                                any_placement = true;
                            }
                        }
                    }
                }
                assert!(
                    any_placement,
                    "non-empty beardifier at ({cx},{cz}) but no adapting placement within +-8"
                );
            }
        }
        // NOTE: no nonempty>0 assertion — the far scan may legitimately miss
        // every start; the NON-EMPTY path is exercised by the (4,6) oracle
        // test, and here only the necessary-condition IMPLICATION is checked.
        assert!(
            empties >= 3,
            "biome filter must leave several chunks beardifier-free (got {empties}/7)"
        );
        // Determinism: a fresh feed (no cache) at the trial start chunk
        // produces the identical rigid list.
        let mut feed2 = BeardFeed::new(&dir, &root, &rs, SEED);
        let mut sampler2 = ColumnHeightSource::new(&rs, SEED);
        let b1 = feed.build_for_chunk(&mut sampler, 4, 6);
        let b2 = feed2.build_for_chunk(&mut sampler2, 4, 6);
        assert_eq!(b1.pieces().len(), b2.pieces().len());
        for (p1, p2) in b1.pieces().iter().zip(b2.pieces().iter()) {
            assert_eq!(
                (p1.min_x, p1.min_y, p1.min_z, p1.max_x, p1.max_y, p1.max_z, p1.ground_level_delta),
                (p2.min_x, p2.min_y, p2.min_z, p2.max_x, p2.max_y, p2.max_z, p2.ground_level_delta)
            );
        }
        assert_eq!(b1.junctions().len(), b2.junctions().len());
    }

    #[test]
    fn beardifier_changes_substance_inside_encapsulate_box() {
        // Wiring smoke test: the substance root is add(final_density,
        // BeardifierMarker) — the fed sim MUST differ from the EMPTY sim at a
        // block inside a trial_chambers piece (ENCAPSULATE adds bury*0.8
        // >= 0.55 inside the box).
        let Some((dir, root)) = extract_dir() else {
            return; // loud skip already printed by test_support
        };
        let rs = RandomState::build_overworld(&dir, SEED).expect("build_overworld");
        let mut sampler = ColumnHeightSource::new(&rs, SEED);
        let mut feed = BeardFeed::new(&dir, &root, &rs, SEED);
        let beard = feed.build_for_chunk(&mut sampler, 4, 6);
        assert!(!beard.is_empty());

        // A block inside the first oracle piece: box [46..64]x[-27..-8]x[96..114].
        let (bx, by, bz) = (64, -20, 96);
        let probe = |beard| -> f64 {
            let mut sim = crate::interpolator::NoiseChunkSim::from_random_state(&rs, 4, 64, 96);
            sim.set_beardifier(beard);
            let mut got = None;
            sim.drive_column(bx, bz, &mut |x, y, z, sim: &mut crate::interpolator::NoiseChunkSim| {
                if y == by {
                    got = Some(sim.substance_value());
                    return true;
                }
                let _ = (x, z);
                false
            });
            got.unwrap_or_else(|| panic!("column probe ({bx},{by},{bz}) never reached"))
        };
        let empty_v = probe(crate::beardifier::Beardifier::empty());
        let fed_v = probe(beard);
        assert_ne!(
            empty_v.to_bits(),
            fed_v.to_bits(),
            "substance at ({bx},{by},{bz}) must change once the beardifier is fed (empty={empty_v}, fed={fed_v})"
        );
    }
}
