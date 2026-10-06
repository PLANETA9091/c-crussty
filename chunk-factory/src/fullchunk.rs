//! NCF P3.4 / P4.9 — FULL-chunk NBT assembly (SerializableChunkData.write
//! form, Paper 1.21.10, DataVersion 4556). This is the payload of the stair-B
//! bridge: ONE Rust call returns these bytes; the server reads the chunk as
//! FULL.
//!
//! Faithful field set for a freshly generated chunk (no features yet —
//! Phase 4; no structures — Phase 5; light recomputed by the server on load
//! because isLightOn=false):
//!   DataVersion, xPos, yPos, zPos, LastUpdate, InhabitedTime, Status,
//!   sections[{Y, block_states{palette[,data]}, biomes{palette[,data]}}],
//!   block_entities[], block_ticks/fluid_ticks (only when non-empty),
//!   PostProcessing, Heightmaps{MOTION_BLOCKING, MOTION_BLOCKING_NO_LEAVES,
//!   OCEAN_FLOOR, WORLD_SURFACE}, structures{starts{}, references{}},
//!   isLightOn=false (omitted unless correct), starlight.light_version=10
//!   when light was computed (it was not — native path leaves light to the
//!   server; Java's own write() emits isLightOn=true with arrays).

use crate::filler::{HeightmapKind, FillerChunk};
use crate::palette::{pack_biome_section, pack_block_section};
use crate::sections::Nbt;

pub const DATA_VERSION_1_21_10: i32 = 4556;

/// Heightmap.Types serialization keys for the four FINAL types, in the
/// EnumMap order ChunkAccess serializes (write() iterates the chunk's
/// heightmaps map; only the four FINAL types are present at FULL).
fn heightmap_key(kind: usize) -> &'static str {
    match kind {
        2 => "OCEAN_FLOOR",
        3 => "WORLD_SURFACE",
        4 => "MOTION_BLOCKING",
        5 => "MOTION_BLOCKING_NO_LEAVES",
        _ => unreachable!(),
    }
}

/// Build the FULL-chunk NBT for a carvers-stage FillerChunk.
pub fn full_chunk_nbt(fc: &FillerChunk, data_version: i32) -> Nbt {
    let min_section_y = fc.min_y >> 4;

    let mut sections = Vec::new();
    for (i, _sec) in fc.sections.iter().enumerate() {
        let blocks = pack_block_section(fc, i);
        let biomes = pack_biome_section(fc, i);
        let mut block_states_fields: Vec<(String, Nbt)> = Vec::new();
        block_states_fields.push((
            "palette".to_string(),
            Nbt::List(blocks.palette.iter().map(|s| state_compound(s)).collect()),
        ));
        if let Some(data) = &blocks.data {
            block_states_fields.push(("data".to_string(), Nbt::LongArray(data.clone())));
        }
        let mut biome_fields: Vec<(String, Nbt)> = Vec::new();
        biome_fields.push((
            "palette".to_string(),
            Nbt::List(biomes.palette.iter().map(|s| Nbt::String(s.clone())).collect()),
        ));
        if let Some(data) = &biomes.data {
            biome_fields.push(("data".to_string(), Nbt::LongArray(data.clone())));
        }
        sections.push(Nbt::Comp(vec![
            ("Y".to_string(), Nbt::Byte((fc.min_y >> 4) as i8 + i as i8)),
            ("block_states".to_string(), Nbt::Comp(block_states_fields)),
            ("biomes".to_string(), Nbt::Comp(biome_fields)),
        ]));
    }

    // Heightmaps compound: the four FINAL types (WG pair never serializes)
    let mut hm_fields: Vec<(String, Nbt)> = Vec::new();
    if fc.heightmaps.len() >= 6 {
        for kind in 2..6usize {
            let hm = &fc.heightmaps[kind];
            let mut longs = vec![0i64; 37];
            for (col, &fa) in hm.first_available.iter().enumerate() {
                let stored = (fa - fc.min_y).max(0) as i64;
                longs[col / 7] |= stored << ((col % 7 * 9) as i32);
            }
            hm_fields.push((heightmap_key(kind).to_string(), Nbt::LongArray(longs)));
        }
    }

    // PostProcessing: list of list-of-short (packOffsets) — one entry per
    // section, empty lists preserved.
    let pp: Vec<Nbt> = fc
        .post_processing
        .iter()
        .map(|v| Nbt::List(v.iter().map(|&s| Nbt::Short(s as i16)).collect()))
        .collect();

    // Ticks: empty at CARVERS-native (aquifer fluid ticks are PostProcessing
    // marks, not scheduled ticks; block ticks come from features — Phase 4).
    // saveTicks writes them ONLY when non-empty.

    Nbt::Comp(vec![
        ("DataVersion".to_string(), Nbt::Int(data_version)),
        ("xPos".to_string(), Nbt::Int(fc.chunk_min_x >> 4)),
        ("yPos".to_string(), Nbt::Int(min_section_y)),
        ("zPos".to_string(), Nbt::Int(fc.chunk_min_z >> 4)),
        ("LastUpdate".to_string(), Nbt::Long(0)),
        ("InhabitedTime".to_string(), Nbt::Long(0)),
        ("Status".to_string(), Nbt::String("minecraft:full".to_string())),
        ("sections".to_string(), Nbt::List(sections)),
        ("block_entities".to_string(), Nbt::List(Vec::new())),
        ("PostProcessing".to_string(), Nbt::List(pp)),
        ("Heightmaps".to_string(), Nbt::Comp(hm_fields)),
        (
            "structures".to_string(),
            Nbt::Comp(vec![
                ("starts".to_string(), Nbt::Comp(Vec::new())),
                ("References".to_string(), Nbt::Comp(Vec::new())),
            ]),
        ),
        ("isLightOn".to_string(), Nbt::Byte(0)),
    ])
}

fn state_compound(canonical: &str) -> Nbt {
    let def = crate::filler::BlockStateDef::parse(canonical);
    let mut fields: Vec<(String, Nbt)> = vec![("Name".to_string(), Nbt::String(def.name.clone()))];
    if !def.props.is_empty() {
        let props: Vec<(String, Nbt)> = def
            .props
            .iter()
            .map(|(k, v)| (k.clone(), Nbt::String(v.clone())))
            .collect();
        fields.push(("Properties".to_string(), Nbt::Comp(props)));
    }
    Nbt::Comp(fields)
}

#[allow(dead_code)]
fn unused(_: &HeightmapKind) {}
