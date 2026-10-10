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

use crate::filler::{BlockStateDef, FillerChunk, HeightmapKind};
use crate::palette::{DenseRemap, pack_biome_section_into, pack_block_section_into};
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

    // Remaps + u32 walk buffers hoisted across ALL sections (R3#2b): the
    // into-variants restore the exact fresh-allocation state each call
    // (remap = all-unseen memset via DenseRemap::reset, data = fully
    // overwritten since SectionData.states is [u32; 4096] / biomes
    // [u16; 64]), so the first-encounter palette order — and the NBT bytes
    // — are identical by construction.
    let mut remap_blocks = DenseRemap::new(fc.state_table.states.len());
    let mut remap_biomes = DenseRemap::new(fc.biome_table.names.len());
    let mut block_data: Vec<u32> = Vec::new();
    let mut biome_data: Vec<u32> = Vec::new();
    let mut sections = Vec::new();
    for (i, _sec) in fc.sections.iter().enumerate() {
        let blocks = pack_block_section_into(fc, i, &mut remap_blocks, &mut block_data);
        let biomes = pack_biome_section_into(fc, i, &mut remap_biomes, &mut biome_data);
        let mut block_states_fields: Vec<(String, Nbt)> = Vec::new();
        block_states_fields.push((
            "palette".to_string(),
            Nbt::List(
                blocks
                    .palette_ids
                    .iter()
                    .map(|&s| state_compound(fc.state_table.get(s)))
                    .collect(),
            ),
        ));
        if let Some(data) = blocks.data {
            block_states_fields.push(("data".to_string(), Nbt::LongArray(data)));
        }
        let mut biome_fields: Vec<(String, Nbt)> = Vec::new();
        biome_fields.push((
            "palette".to_string(),
            Nbt::List(
                biomes
                    .palette_ids
                    .iter()
                    .map(|&b| Nbt::String(fc.biome_table.names[b as usize].clone()))
                    .collect(),
            ),
        ));
        if let Some(data) = biomes.data {
            biome_fields.push(("data".to_string(), Nbt::LongArray(data)));
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

/// NbtUtils.writeBlockState form, built DIRECTLY from the interned
/// BlockStateDef (R3#2: no canonical re-format + re-parse per palette
/// entry). `Properties` keeps `def.props` in STORED order — the table
/// sorts props on every intern path (filler.rs intern/intern_canonical),
/// which is exactly the order the canonical string (and the old
/// parse-round-trip, whose parse also sorts) produced; the identity is
/// guarded by filler.rs `parse_canonical_is_identity`. Byte-identical NBT
/// by construction.
fn state_compound(def: &BlockStateDef) -> Nbt {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filler::{BiomeTable, SectionData, StateTable};
    use crate::jrandom::{LegacyRandomSource, RandomSource};
    use crate::sections::write_gzipped_nbt;

    /// The PRE-I-D-nbt section builder, verbatim (93bb547): fresh
    /// DenseRemap + fresh data vec per section (the pack_*_section
    /// wrappers), canonical-String palette, state compounds rebuilt via
    /// BlockStateDef::parse(canonical), data cloned into the LongArray.
    /// The reference for the byte-identity tripwire below.
    fn old_state_compound(canonical: &str) -> Nbt {
        let def = crate::filler::BlockStateDef::parse(canonical);
        let mut fields: Vec<(String, Nbt)> =
            vec![("Name".to_string(), Nbt::String(def.name.clone()))];
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

    fn old_sections(fc: &FillerChunk) -> Vec<Nbt> {
        let mut sections = Vec::new();
        for (i, _sec) in fc.sections.iter().enumerate() {
            let blocks = crate::palette::pack_block_section(fc, i);
            let biomes = crate::palette::pack_biome_section(fc, i);
            let mut block_states_fields: Vec<(String, Nbt)> = Vec::new();
            block_states_fields.push((
                "palette".to_string(),
                Nbt::List(blocks.palette.iter().map(|s| old_state_compound(s)).collect()),
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
        sections
    }

    /// I2 tripwire: the hoisted-buffer + direct-from-BlockStateDef path
    /// must produce byte-identical section NBT to the pre-change pipeline
    /// for every section. The synthetic chunk exercises: prop-carrying
    /// states interned with UNSORTED props (the table sorts — the direct
    /// def compound must match what parse∘canonical would emit), propless
    /// states, an all-air single-value section (bits=0, no data field), a
    /// single-state section, and random multi-palette sections.
    #[test]
    fn full_chunk_nbt_sections_byte_identical_to_pre_hoist_path() {
        let mut st = StateTable::new();
        st.intern_canonical("minecraft:air");
        st.intern("minecraft:water", &[("level", "0")]);
        st.intern(
            "minecraft:oak_leaves",
            &[("persistent", "false"), ("distance", "7"), ("waterlogged", "false")],
        );
        st.intern(
            "minecraft:chest",
            &[("waterlogged", "true"), ("facing", "north"), ("type", "single")],
        );
        st.intern("minecraft:chain", &[("axis", "y"), ("waterlogged", "true")]);
        for n in [
            "minecraft:stone",
            "minecraft:deepslate",
            "minecraft:dirt",
            "minecraft:gravel",
            "minecraft:tuff",
            "minecraft:packed_ice",
            "minecraft:sculk",
        ] {
            st.intern_canonical(n);
        }
        let n_states = st.states.len() as i32;
        let mut bt = BiomeTable::new();
        for n in [
            "minecraft:plains",
            "minecraft:river",
            "minecraft:ocean",
            "minecraft:frozen_peaks",
            "minecraft:dripstone_caves",
            "minecraft:stony_shore",
        ] {
            bt.intern(n);
        }
        let n_biomes = bt.names.len() as i32;

        let mut rng = LegacyRandomSource::new(0xC0FF_EE01);
        let mut fc = FillerChunk {
            min_y: -64,
            height: 384,
            chunk_min_x: -48,
            chunk_min_z: 112,
            sections: Vec::new(),
            state_table: st,
            biome_table: bt,
            heightmaps: Vec::new(),
            post_processing: Vec::new(),
        };
        for s in 0..24 {
            let mut sec = SectionData::new();
            match s {
                // section 0 stays all-air: single-value palette, NO data field
                1 => {
                    let id = rng.next_int_bound(n_states).max(0) as u32;
                    sec.states.fill(id);
                    let b = rng.next_int_bound(n_biomes).max(0) as u16;
                    sec.biomes.fill(b);
                }
                _ => {
                    for v in sec.states.iter_mut() {
                        *v = rng.next_int_bound(n_states).max(0) as u32;
                    }
                    for b in sec.biomes.iter_mut() {
                        *b = rng.next_int_bound(n_biomes).max(0) as u16;
                    }
                }
            }
            fc.sections.push(sec);
        }

        let root = full_chunk_nbt(&fc, DATA_VERSION_1_21_10);
        let new_sections = match root.get("sections") {
            Some(Nbt::List(ls)) => ls,
            _ => panic!("full chunk root missing sections"),
        };
        let old_sections = old_sections(&fc);
        assert_eq!(new_sections.len(), old_sections.len(), "24 sections expected");
        for (i, (n, o)) in new_sections.iter().zip(old_sections.iter()).enumerate() {
            let nb = write_gzipped_nbt(n);
            let ob = write_gzipped_nbt(o);
            assert_eq!(nb, ob, "section {i} NBT bytes diverged from the pre-hoist path");
        }
    }
}
