#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::excessive_precision, clippy::needless_range_loop)]

//! NCF P2.10 groundwork — the noise-stage chunk filler: turns the bit-exact
//! density/interpolation machinery (interpolator.rs) + aquifer.rs + the ore
//! veinifier into per-block chunk content, replicating
//! `NoiseBasedChunkGenerator.doFill` + `NoiseChunk.getInterpolatedState`
//! (blockStateRule = [aquifer.computeSubstance(ctx, cacheAllInCell(final+
//! Beardifier)), OreVeinifier]) + proto-chunk heightmap updates
//! (OCEAN_FLOOR_WG / WORLD_SURFACE_WG, unprimed update machine) + biome
//! quart fill (fillBiomesFromNoise) + ProtoChunk.markPosForPostprocessing.
//!
//! Beardifier: structures are Phase 5; the gate corpus (fresh world, spiral,
//! vanilla) is generated with an EMPTY beardifier (BeardifierMarker adds 0.0).
//! Chunks whose density is touched by structure bounding boxes will diverge —
//! they fall under I8 Java-fallback territory until Phase 5. Documented.

use crate::aquifer::{GlobalFluidPicker, NoiseBasedAquifer, OreStateIds, OreVeinifierRule};
use crate::density::Df;
use crate::interpolator::NoiseChunkSim;
use crate::router::RandomState;
use crate::xoroshiro::XoroshiroRandomSource;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Block state table (contract shared with sections.rs / stagediff)
// ---------------------------------------------------------------------------

/// A block state: registry name + properties SORTED by key (vanilla
/// NbtUtils.writeBlockState order: the state's property map is a sorted map).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockStateDef {
    pub name: String,
    pub props: Vec<(String, String)>,
}

impl BlockStateDef {
    /// Canonical string form used by stagediff: "minecraft:stone" or
    /// "minecraft:water[level=0]" (props joined "k=v" with ",", sorted).
    pub fn canonical(&self) -> String {
        if self.props.is_empty() {
            self.name.clone()
        } else {
            let ps: Vec<String> = self.props.iter().map(|(k, v)| format!("{k}={v}")).collect();
            format!("{}[{}]", self.name, ps.join(","))
        }
    }

    pub fn parse(canonical: &str) -> Self {
        let (name, props) = match canonical.find('[') {
            Some(i) => (&canonical[..i], &canonical[i + 1..canonical.len() - 1]),
            None => (canonical, ""),
        };
        let mut props: Vec<(String, String)> = if props.is_empty() {
            Vec::new()
        } else {
            props
                .split(',')
                .map(|p| {
                    let (k, v) = p.split_once('=').expect("prop k=v");
                    (k.to_string(), v.to_string())
                })
                .collect()
        };
        props.sort();
        BlockStateDef { name: name.to_string(), props }
    }
}

/// Intern table for block states (first-encounter ids; the contract only
/// requires CONTENT equality, palette order is irrelevant to stagediff).
#[derive(Default)]
pub struct StateTable {
    pub states: Vec<BlockStateDef>,
    keys: HashMap<String, u32>,
}

impl StateTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, name: &str, props: &[(&str, &str)]) -> u32 {
        let mut ps: Vec<(String, String)> = props
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ps.sort();
        let def = BlockStateDef { name: name.to_string(), props: ps };
        let key = def.canonical();
        if let Some(&id) = self.keys.get(&key) {
            return id;
        }
        let id = self.states.len() as u32;
        self.states.push(def);
        self.keys.insert(key, id);
        id
    }

    pub fn intern_canonical(&mut self, canonical: &str) -> u32 {
        let def = BlockStateDef::parse(canonical);
        let key = def.canonical();
        if let Some(&id) = self.keys.get(&key) {
            return id;
        }
        let id = self.states.len() as u32;
        self.states.push(def);
        self.keys.insert(key, id);
        id
    }

    pub fn get(&self, id: u32) -> &BlockStateDef {
        &self.states[id as usize]
    }
}

/// Biome name table (strings like "minecraft:river").
#[derive(Default)]
pub struct BiomeTable {
    pub names: Vec<String>,
    keys: HashMap<String, u32>,
}

impl BiomeTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.keys.get(name) {
            return id;
        }
        let id = self.names.len() as u32;
        self.names.push(name.to_string());
        self.keys.insert(name.to_string(), id);
        id
    }
}

/// One 16^3 section: block state ids (idx = y*256 + z*16 + x, VERIFIED
/// against PalettedContainer strategy) and biome quart ids
/// (idx = y*16 + z*4 + x). Unfilled ids are 0; id 0 is always air
/// (interned first), matching the empty proto chunk.
#[derive(Clone)]
pub struct SectionData {
    pub states: [u32; 4096],
    pub biomes: [u16; 64],
}

impl SectionData {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SectionData { states: [0; 4096], biomes: [0; 64] }
    }
    #[inline]
    pub fn block_index(x: i32, y: i32, z: i32) -> usize {
        (y * 256 + z * 16 + x) as usize
    }
    #[inline]
    pub fn biome_index(qx: i32, qy: i32, qz: i32) -> usize {
        (qy * 16 + qz * 4 + qx) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeightmapKind {
    OceanFloorWg,
    WorldSurfaceWg,
}

impl HeightmapKind {
    /// Heightmap.Types predicates: WORLD_SURFACE_WG = NOT_AIR,
    /// OCEAN_FLOOR_WG = MATERIAL_MOTION_BLOCKING (blocksMotion).
    #[inline]
    pub fn is_opaque_state(self, state: u32, table: &StateTable) -> bool {
        let def = table.get(state);
        match self {
            HeightmapKind::WorldSurfaceWg => !is_air_name(&def.name),
            HeightmapKind::OceanFloorWg => blocks_motion(&def.name),
        }
    }
}

fn is_air_name(name: &str) -> bool {
    name == "minecraft:air" || name == "minecraft:cave_air" || name == "minecraft:void_air"
}

/// BlockBehaviour.blocksMotion for the states reachable at NOISE/SURFACE
/// statuses: fluids and air do not block motion, everything else does.
fn blocks_motion(name: &str) -> bool {
    !matches!(
        name,
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "minecraft:water" | "minecraft:lava"
    )
}

/// Raw first-available heights per column (idx = x + z*16, VERIFIED:
/// Heightmap.getIndex(x,z) = x + z*16), stored ABSOLUTE
/// (vanilla stores value - minY in the bit storage; getFirstAvailable
/// returns stored + minY — we keep the absolute value directly).
pub struct HeightmapData {
    pub kind: HeightmapKind,
    pub first_available: [i32; 256],
}

impl HeightmapData {
    pub fn new(kind: HeightmapKind, min_y: i32) -> Self {
        HeightmapData { kind, first_available: [min_y; 256] }
    }

    /// Heightmap.update for one placed block, with the downward scan reading
    /// the CURRENT (partially filled) chunk array — blocks below the fill
    /// position are still air (id 0), exactly like the live proto chunk
    /// during the y-descending doFill.
    fn update(&mut self, x: i32, y: i32, z: i32, is_opaque: bool, min_y: i32, sections: &[SectionData], table: &StateTable) {
        let idx = (x + z * 16) as usize;
        let first_available = self.first_available[idx];
        if y <= first_available - 2 {
            return;
        }
        if is_opaque {
            if y >= first_available {
                self.first_available[idx] = y + 1;
            }
        } else if first_available - 1 == y {
            let mut i = y - 1;
            loop {
                if i < min_y {
                    self.first_available[idx] = min_y;
                    return;
                }
                let sec_idx = ((i - min_y) / 16) as usize;
                if sec_idx >= sections.len() {
                    i -= 1;
                    continue;
                }
                let s = sections[sec_idx].states[SectionData::block_index(x & 15, i & 15, z & 15)];
                if self.kind.is_opaque_state(s, table) {
                    self.first_available[idx] = i + 1;
                    return;
                }
                i -= 1;
            }
        }
    }
}

/// One generated chunk at NOISE status semantics.
pub struct FillerChunk {
    pub min_y: i32,
    pub height: i32,
    /// world-space min block X/Z (pos.getMinBlockX/Z) — needed by the
    /// surface/carver passes (session 6).
    pub chunk_min_x: i32,
    pub chunk_min_z: i32,
    /// height/16 sections, section index = (y - min_y) / 16.
    pub sections: Vec<SectionData>,
    pub state_table: StateTable,
    pub biome_table: BiomeTable,
    /// [0] = OCEAN_FLOOR_WG, [1] = WORLD_SURFACE_WG (the two WORLDGEN
    /// heightmaps primed at NOISE/SURFACE statuses — task 5-a fact).
    /// Session 6: slots 2..6 are the four FINAL heightmaps, primed at the
    /// first CARVERS-stage write (carvers.rs).
    pub heightmaps: Vec<HeightmapData>,
    /// Per section: packed shorts from markPosForPostprocessing
    /// (packOffsetCoordinates = (x&15) | (y&15)<<4 | (z&15)<<8).
    pub post_processing: Vec<Vec<u16>>,
}

impl FillerChunk {
    #[inline]
    pub fn section_of(&self, y: i32) -> usize {
        ((y - self.min_y) / 16) as usize
    }

    /// World-coord block state id.
    pub fn block(&self, x: i32, y: i32, z: i32) -> u32 {
        let sec = &self.sections[self.section_of(y)];
        sec.states[SectionData::block_index(x & 15, y & 15, z & 15)]
    }

    pub fn block_canonical(&self, x: i32, y: i32, z: i32) -> String {
        self.state_table.get(self.block(x, y, z)).canonical()
    }
}

/// Generate one chunk's noise-stage content (NOISE status semantics).
pub fn generate_noise_chunk(rs: &RandomState, seed: i64, cx: i32, cz: i32) -> Result<FillerChunk, String> {
    if rs.settings.legacy_random_source {
        return Err("legacy_random_source settings (nether-style) not supported by the filler yet".into());
    }
    if !rs.settings.aquifers_enabled {
        return Err("noise settings without aquifers not supported yet".into());
    }
    let min_y = rs.settings.min_y;
    let height = rs.settings.height;
    let sections_count = (height / 16) as usize;
    let min_block_x = cx * 16;
    let min_block_z = cz * 16;
    let max_block_x = min_block_x + 15;
    let max_block_z = min_block_z + 15;

    let mut table = StateTable::new();
    let mut biomes_tbl = BiomeTable::new();
    let air = table.intern("minecraft:air", &[]);
    let water = table.intern("minecraft:water", &[("level", "0")]);
    let lava = table.intern("minecraft:lava", &[("level", "0")]);
    let default_block = table.intern_canonical(&rs.settings.default_block);
    let ore_ids = OreStateIds {
        copper_ore: table.intern("minecraft:copper_ore", &[]),
        raw_copper_block: table.intern("minecraft:raw_copper_block", &[]),
        granite: table.intern("minecraft:granite", &[]),
        deepslate_iron_ore: table.intern("minecraft:deepslate_iron_ore", &[]),
        raw_iron_block: table.intern("minecraft:raw_iron_block", &[]),
        tuff: table.intern("minecraft:tuff", &[]),
    };
    let table = table; // freeze

    // aquiferRandom = random.fromHashOf("aquifer").forkPositional();
    // oreRandom   = random.fromHashOf("ore").forkPositional();
    // `random` = settings.getRandomSource().newInstance(levelSeed).forkPositional()
    // (RandomState.java lines 47-50) — rebuilt concretely, identical.
    let mut base = XoroshiroRandomSource::new(seed);
    let worldgen = base.fork_positional();
    let mut aquifer_src = worldgen.from_hash_of("minecraft:aquifer");
    let aquifer_factory = aquifer_src.fork_positional();
    let mut ore_src = worldgen.from_hash_of("minecraft:ore");
    let ore_factory = ore_src.fork_positional();

    let picker = GlobalFluidPicker { sea_level: rs.settings.sea_level };
    let mut aquifer = NoiseBasedAquifer::new(
        &rs.bank,
        &rs.router,
        aquifer_factory,
        min_y,
        height,
        min_block_x,
        max_block_x,
        min_block_z,
        max_block_z,
        picker,
    );
    let ore_rule = OreVeinifierRule { ore_random: ore_factory };

    let mut sections: Vec<SectionData> = (0..sections_count).map(|_| SectionData::new()).collect();
    let mut post_processing: Vec<Vec<u16>> = (0..sections_count).map(|_| Vec::new()).collect();
    let mut hm_ocean = HeightmapData::new(HeightmapKind::OceanFloorWg, min_y);
    let mut hm_surface = HeightmapData::new(HeightmapKind::WorldSurfaceWg, min_y);
    let ore_veins_enabled = rs.settings.ore_veins_enabled;

    // Drive the interpolation machinery exactly like doFill and materialize
    // the block rule per block. The aquifer/ore borrows all hang off the
    // immutable rs; only sections/post/hm/table mutate (none overlap rs).
    let mut sim = NoiseChunkSim::from_random_state(rs, 4, min_block_x, min_block_z);
    let aquifer_ref = &mut aquifer;
    let ore_ref = &ore_rule;
    // NCF profiling only: compiled OUT of normal builds (never in CI); set
    // stage skip flags via RUSTFLAGS="--cfg ncf_profile" cargo build --release.
    let skip_aquifer = cfg!(ncf_profile) && std::env::var("NCF_SKIP_AQUIFER").is_ok();
    let skip_veins = cfg!(ncf_profile) && std::env::var("NCF_SKIP_VEINS").is_ok();
    let skip_write = cfg!(ncf_profile) && std::env::var("NCF_SKIP_WRITE").is_ok();
    let skip_hm = cfg!(ncf_profile) && std::env::var("NCF_SKIP_HM").is_ok();
    let skip_drive = cfg!(ncf_profile) && std::env::var("NCF_SKIP_DRIVE").is_ok();
    let skip_biome = cfg!(ncf_profile) && std::env::var("NCF_SKIP_BIOME").is_ok();
    if !skip_drive {
    sim.drive_blocks(&mut |bx: i32, by: i32, bz: i32, sim: &mut NoiseChunkSim| {
        // substance = cacheAllInCell(finalDensity + BeardifierMarker) — the
        // per-cell batch-filled cache (selectCellYZ), read at this block.
        let substance = sim.substance_value();
        let mut state: Option<u32> = if skip_aquifer {
            if substance < 0.0 { Some(default_block) } else { None }
        } else {
            aquifer_ref.compute_substance(bx, by, bz, substance, air, water, lava)
        };
        if state.is_none() && ore_veins_enabled && !skip_veins {
            // OreVeinifier order: toggle, ridged, gap — bound (interpolated
            // containing) values from the sim, never scalar.
            let toggle = sim.compute_field(12);
            let ridged = sim.compute_field(13);
            let gap = sim.compute_field(14);
            state = ore_ref.compute(toggle, ridged, gap, bx, by, bz, &ore_ids);
        }
        let state = match state {
            Some(s) => s,
            None => default_block,
        };
        // doFill: `if (interpolatedState == AIR ...) continue;` — AIR blocks
        // are NOT written to the section and do NOT update the heightmaps.
        if is_air_name(table.get(state).name.as_str()) || skip_write {
            return;
        }
        let sec_idx = ((by - min_y) / 16) as usize;
        sections[sec_idx].states[SectionData::block_index(bx & 15, by & 15, bz & 15)] = state;
        // heightmaps (doFill order: OCEAN_FLOOR_WG then WORLD_SURFACE_WG)
        if skip_hm {
            return;
        }
        let op_ocean = HeightmapKind::OceanFloorWg.is_opaque_state(state, &table);
        let op_surface = HeightmapKind::WorldSurfaceWg.is_opaque_state(state, &table);
        // Heightmap.update receives SECTION-LOCAL x/z and WORLD y
        // (doFill: heightmapUnprimed.update(i11, i7, i14, ...) with
        // i11 = i10 & 0xF, i14 = i13 & 0xF).
        hm_ocean.update(bx & 15, by, bz & 15, op_ocean, min_y, &sections, &table);
        hm_surface.update(bx & 15, by, bz & 15, op_surface, min_y, &sections, &table);
        // postprocessing: aquifer.shouldScheduleFluidUpdate && fluid not empty
        if aquifer_ref.should_schedule_fluid_update() {
            let packed = ((bx & 15) | ((by & 15) << 4) | ((bz & 15) << 8)) as u16;
            post_processing[sec_idx].push(packed);
        }
    });
    } // end skip_drive

    // Biome quarts (fillBiomesFromNoise): per section, 4x4x4 quarts, order
    // i1=x, i2=y, i3=z (LevelChunkSection.fillBiomesFromNoise).
    let mut list = crate::climate::ParameterList::new(
        crate::vanilla_biomes::overworld_points()
            .into_iter()
            .map(|(p, n)| (p, n.to_string()))
            .collect(),
    );
    let q_min_x = min_block_x.div_euclid(4);
    let q_min_z = min_block_z.div_euclid(4);

    // P2.11 — the six climate fields classified once; y-free fields are
    // memoised per quart COLUMN (x,z): the scalar tree is a pure function,
    // and a y-free tree returns bit-identical values for every y, so the
    // cache reproduces the exact per-quart scalar result (see density.rs
    // is_y_free for the equivalence argument).
    let climate_fields: [&Df; 6] = [
        &rs.router.temperature,
        &rs.router.vegetation,
        &rs.router.continents,
        &rs.router.erosion,
        &rs.router.depth,
        &rs.router.ridges,
    ];
    let field_y_free: [bool; 6] = {
        let mut f = [false; 6];
        for (i, field) in climate_fields.iter().enumerate() {
            f[i] = field.is_y_free(&rs.bank);
        }
        f
    };
    let mut memo: crate::density::ColumnMemo = HashMap::new();
    if skip_biome { return Ok(FillerChunk { min_y, height, chunk_min_x: min_block_x, chunk_min_z: min_block_z, sections, state_table: table, biome_table: biomes_tbl, heightmaps: vec![hm_ocean, hm_surface], post_processing }); }
    for sy in 0..sections_count as i32 {
        let section_y = (min_y / 16) + sy;
        let q_y0 = section_y * 4; // QuartPos.fromSection
        // Java order (LevelChunkSection.fillBiomesFromNoise): x outer,
        // y, z inner — matters for the RTree ThreadLocal last-result path.
        for ix in 0..4i32 {
            for iy in 0..4i32 {
                for iz in 0..4i32 {
                    let qx = q_min_x + ix;
                    let qy = q_y0 + iy;
                    let qz = q_min_z + iz;
                    // Climate.Sampler.sample takes QUART coords and evaluates
                    // the density functions at BLOCK coords
                    // (QuartPos.toBlock = *4, Climate.java line ~146).
                    let bx = qx * 4;
                    let by = qy * 4;
                    let bz = qz * 4;
                    // quantizeCoord takes FLOAT (Climate.java line 62).
                    let fields = [
                        (&rs.router.temperature, 0usize),
                        (&rs.router.vegetation, 1),
                        (&rs.router.continents, 2),
                        (&rs.router.erosion, 3),
                        (&rs.router.depth, 4),
                        (&rs.router.ridges, 5),
                    ];
                    let mut vals = [0.0f64; 6];
                    for (fi, (field, _)) in fields.iter().enumerate() {
                        let _ = field_y_free[fi];
                        vals[fi] = field.compute_memo(&rs.bank, bx, by, bz, &mut memo);
                    }
                    let [t, hu, co, er, de, wi] = vals;
                    let t = t as f32;
                    let hu = hu as f32;
                    let co = co as f32;
                    let er = er as f32;
                    let de = de as f32;
                    let wi = wi as f32;
                    let target = crate::climate::TargetPoint {
                        temperature: crate::climate::quantize_coord(t),
                        humidity: crate::climate::quantize_coord(hu),
                        continentalness: crate::climate::quantize_coord(co),
                        erosion: crate::climate::quantize_coord(er),
                        depth: crate::climate::quantize_coord(de),
                        weirdness: crate::climate::quantize_coord(wi),
                    };
                    let biome = list.find_value(&target).to_string();
                    let id = biomes_tbl.intern(&biome);
                    sections[sy as usize].biomes[SectionData::biome_index(ix, iy, iz)] = id as u16;
                }
            }
        }
    }

    Ok(FillerChunk {
        min_y,
        height,
        chunk_min_x: min_block_x,
        chunk_min_z: min_block_z,
        sections,
        state_table: table,
        biome_table: biomes_tbl,
        heightmaps: vec![hm_ocean, hm_surface],
        post_processing,
    })
}
