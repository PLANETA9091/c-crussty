//! NCF P2.10-tail / P4.7 / T12 — PalettedContainer serial form ("pack"),
//! decompiled from Paper 1.21.10 (PalettedContainer.pack + Strategy + the
//! Strategy$1/$2 bit ladders):
//!
//!   blocks  (4096 entries, 4 bits/axis): bits = ceillog2(paletteSize);
//!           0 -> single-value (NO data field); 1..4 -> 4 bits stored;
//!           5..8 -> that bit count; 9+ -> global IN MEMORY but the stored
//!           bit count is still ceillog2(paletteSize) with a LOCAL palette
//!           list (verified: Configuration.Global(bitsInMemory, bits) where
//!           bits = the requested count; pack() writes bitsInStorage()).
//!   biomes  (64 entries, 2 bits/axis): bits = ceillog2(paletteSize);
//!           0 -> no data; 1..3 -> that bit count; 4+ -> same global rule.
//!   packing = SimpleBitStorage: valuesPerLong = 64 / bits, LSB-first,
//!           longs = ceil(entryCount / valuesPerLong), trailing bits zero.
//!   palette order = first-encounter during reencodeContents (idFor).
//!
//! Sections needing > 256 distinct blocks would push the palette beyond the
//! HashMapPalette cap in memory, but the SERIAL form still stores a local
//! list — so the writer below is exact for every palette size (no global-id
//! table needed for the serial form).

use crate::filler::FillerChunk;

/// Strategy$1 (blocks): getConfigurationForBitCount -> stored bits.
fn block_storage_bits(palette_size: usize) -> usize {
    let bits = ceillog2_usize(palette_size);
    match bits {
        0 => 0,
        1..=4 => 4,
        n => n,
    }
}

/// Strategy$2 (biomes): stored bits.
fn biome_storage_bits(palette_size: usize) -> usize {
    ceillog2_usize(palette_size)
}

fn ceillog2_usize(v: usize) -> usize {
    // Mth.ceillog2: 32 - numberOfLeadingZeros(value - 1) for v >= 1; v=1 -> 0
    if v <= 1 {
        0
    } else {
        32 - ((v - 1) as u32).leading_zeros() as usize
    }
}

/// SimpleBitStorage packing: valuesPerLong = 64/bits, LSB-first.
pub fn pack_bit_storage(values: &[u32], bits: usize) -> Vec<i64> {
    if bits == 0 {
        return Vec::new();
    }
    let values_per_long = 64 / bits;
    let long_count = values.len().div_ceil(values_per_long);
    let mut out = vec![0i64; long_count];
    for (i, &v) in values.iter().enumerate() {
        let long_idx = i / values_per_long;
        let shift = (i % values_per_long) * bits;
        out[long_idx] |= ((v as i64) & mask_for(bits)) << shift;
    }
    out
}

fn mask_for(bits: usize) -> i64 {
    (1i64 << bits) - 1
}

/// A section palette + packed data in the exact NBT layout.
pub struct PackedSection {
    /// canonical state strings, first-encounter order
    pub palette: Vec<String>,
    pub data: Option<Vec<i64>>,
}

/// Dense id->palette-index remap replacing the per-section
/// `HashMap<u32,u32>` (R4#4: 4096 SipHash lookups per section, 24 sections
/// per chunk => 98,304+ per chunk across the two pack paths).
///
/// The shared StateTable/BiomeTable ids are DENSE (0..len-1, first-encounter
/// intern), so the map is a flat Vec indexed by raw id with `u32::MAX` as
/// the "unseen" sentinel. The caller walks entries in the SAME order as the
/// old HashMap version and assigns palette indices on first encounter, so
/// the palette order — and therefore the NBT bytes — are identical by
/// construction. Reset per section is a tiny memset (`fill`, the shared
/// tables hold only a few hundred states) — deterministic and simpler than
/// epoch tagging.
pub struct DenseRemap {
    map: Vec<u32>,
}

impl DenseRemap {
    /// Sized to the id space of the shared table (`states.len()` /
    /// `names.len()`).
    pub fn new(id_space_size: usize) -> Self {
        Self { map: vec![u32::MAX; id_space_size] }
    }

    /// Reset for the next section: O(id_space) memset (sub-KB for the
    /// tables in play), fully deterministic.
    pub fn reset(&mut self) {
        self.map.fill(u32::MAX);
    }

    /// First-encounter remap of `id`: returns its palette index, assigning
    /// `next_index` (the caller's current `palette.len()`) on first sight.
    /// The walk order is caller-controlled, so this reproduces the HashMap
    /// version exactly — including the (unreachable through the pack paths)
    /// case of an id at or beyond the initial size, which grows the vec
    /// instead of panicking. `u32::MAX` can never collide with a real
    /// palette index: a section holds at most 4096 (blocks) / 64 (biomes)
    /// entries.
    #[inline]
    pub fn remap(&mut self, id: u32, next_index: u32) -> u32 {
        let idx = id as usize;
        if idx >= self.map.len() {
            self.map.resize(idx + 1, u32::MAX);
        }
        let slot = &mut self.map[idx];
        if *slot == u32::MAX {
            *slot = next_index;
            next_index
        } else {
            *slot
        }
    }
}

/// Id-space pack result (R3#2b): the palette as raw shared-table ids in
/// first-encounter order + the packed data longs. Callers map ids through
/// the shared tables themselves — fullchunk builds the block-state NBT
/// compound DIRECTLY from the interned BlockStateDef (no canonical String
/// is materialized at all on this path); the canonical form is only built
/// where a schema needs owned strings, once per state id per chunk.
pub struct PackedSectionIds {
    /// StateTable ids (blocks) / BiomeTable name ids (biomes), widened to
    /// u32; first-encounter order — identical to the String palettes.
    pub palette_ids: Vec<u32>,
    pub data: Option<Vec<i64>>,
}

/// `pack_block_section` into caller-owned buffers (R3#2b hoist): `remap` is
/// `reset()` here (the exact pristine all-unseen state a fresh
/// `DenseRemap::new` produces) and `data` is restored to the same 4096
/// zeros the fresh `vec![0u32; 4096]` had — the walk writes EVERY slot
/// (SectionData.states is `[u32; 4096]`), so the packed output is
/// byte-identical to the per-call-allocation version while the buffers are
/// reused across all 24 sections of a chunk.
pub fn pack_block_section_into(
    fc: &FillerChunk,
    section_idx: usize,
    remap: &mut DenseRemap,
    data: &mut Vec<u32>,
) -> PackedSectionIds {
    let sec = &fc.sections[section_idx];
    remap.reset();
    data.clear();
    data.resize(4096, 0);
    let mut palette: Vec<u32> = Vec::new();
    for (j, &s) in sec.states.iter().enumerate() {
        let next = palette.len() as u32;
        let pi = remap.remap(s, next);
        if pi == next {
            palette.push(s);
        }
        data[j] = pi;
    }
    let bits = block_storage_bits(palette.len());
    let packed = if bits == 0 {
        None
    } else {
        Some(pack_bit_storage(data, bits))
    };
    PackedSectionIds { palette_ids: palette, data: packed }
}

/// `pack_biome_section` into caller-owned buffers (same hoist contract as
/// `pack_block_section_into`; the 64-entry biome walk fully overwrites
/// `bdata` — SectionData.biomes is `[u16; 64]`).
pub fn pack_biome_section_into(
    fc: &FillerChunk,
    section_idx: usize,
    remap: &mut DenseRemap,
    bdata: &mut Vec<u32>,
) -> PackedSectionIds {
    let sec = &fc.sections[section_idx];
    remap.reset();
    bdata.clear();
    bdata.resize(64, 0);
    // separate dense remap: the biome id space is its own table (u16 ids)
    let mut palette: Vec<u16> = Vec::new();
    for (j, &b) in sec.biomes.iter().enumerate() {
        let next = palette.len() as u32;
        let pi = remap.remap(b as u32, next);
        if pi == next {
            palette.push(b);
        }
        bdata[j] = pi;
    }
    let bits = biome_storage_bits(palette.len());
    let packed = if bits == 0 {
        None
    } else {
        Some(pack_bit_storage(bdata, bits))
    };
    PackedSectionIds {
        palette_ids: palette.iter().map(|&b| b as u32).collect(),
        data: packed,
    }
}

/// Repack one 4096-block section from raw first-encounter ids.
pub fn pack_block_section(fc: &FillerChunk, section_idx: usize) -> PackedSection {
    let mut remap = DenseRemap::new(fc.state_table.states.len());
    let mut data = Vec::new();
    let packed = pack_block_section_into(fc, section_idx, &mut remap, &mut data);
    PackedSection {
        palette: packed.palette_ids.iter().map(|&s| fc.state_table.get(s).canonical()).collect(),
        data: packed.data,
    }
}

pub fn pack_biome_section(fc: &FillerChunk, section_idx: usize) -> PackedSection {
    let mut remap = DenseRemap::new(fc.biome_table.names.len());
    let mut bdata = Vec::new();
    let packed = pack_biome_section_into(fc, section_idx, &mut remap, &mut bdata);
    PackedSection {
        palette: packed
            .palette_ids
            .iter()
            .map(|&b| fc.biome_table.names[b as usize].clone())
            .collect(),
        data: packed.data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filler::{BiomeTable, FillerChunk, SectionData, StateTable};
    use crate::jrandom::{LegacyRandomSource, RandomSource};
    use std::collections::HashMap;

    /// The ORIGINAL first-encounter walk (verbatim pre-DenseRemap loop) —
    /// the reference for the equivalence tests below.
    fn reference_first_encounter(ids: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut palette: Vec<u32> = Vec::new();
        let mut index: HashMap<u32, u32> = HashMap::new();
        let mut data = vec![0u32; ids.len()];
        for (j, &s) in ids.iter().enumerate() {
            let pi = match index.get(&s) {
                Some(&p) => p,
                None => {
                    let p = palette.len() as u32;
                    palette.push(s);
                    index.insert(s, p);
                    p
                }
            };
            data[j] = pi;
        }
        (palette, data)
    }

    fn test_chunk(states: &[u32], biome_ids: &[u16], table_states: usize, table_biomes: usize) -> FillerChunk {
        let mut st = StateTable::new();
        for i in 0..table_states {
            st.intern_canonical(&format!("minecraft:test_state_{}", i));
        }
        let mut bt = BiomeTable::new();
        for i in 0..table_biomes {
            bt.intern(&format!("minecraft:test_biome_{}", i));
        }
        let mut sec = SectionData::new();
        sec.states.copy_from_slice(&states[..states.len().min(4096)]);
        for (j, &b) in biome_ids.iter().take(64).enumerate() {
            sec.biomes[j] = b;
        }
        FillerChunk {
            min_y: -64,
            height: 384,
            chunk_min_x: 0,
            chunk_min_z: 0,
            sections: vec![sec],
            state_table: st,
            biome_table: bt,
            heightmaps: Vec::new(),
            post_processing: Vec::new(),
        }
    }

    #[test]
    fn dense_remap_first_encounter_matches_hashmap() {
        // random id streams (dense id space + sparse/huge ids) must remap
        // EXACTLY like the verbatim HashMap walk: same first-encounter
        // palette order, same packed indices.
        let mut rng = LegacyRandomSource::new(0x5EED_CAFE);
        for trial in 0..64 {
            let space = match trial % 4 {
                0 => 1,
                1 => 3,
                2 => 37,
                _ => 700,
            };
            let len = 1 + (rng.next_int_bound(512).max(0) as usize);
            let mut ids = Vec::with_capacity(len);
            for _ in 0..len {
                ids.push(rng.next_int_bound(space).max(0) as u32);
            }
            let mut remap = DenseRemap::new(space as usize);
            let mut palette: Vec<u32> = Vec::new();
            let mut got = vec![0u32; len];
            for (j, &s) in ids.iter().enumerate() {
                let next = palette.len() as u32;
                let pi = remap.remap(s, next);
                if pi == next {
                    palette.push(s);
                }
                got[j] = pi;
            }
            let (want_palette, want_data) = reference_first_encounter(&ids);
            assert_eq!(palette, want_palette, "trial {trial} palette order diverged");
            assert_eq!(got, want_data, "trial {trial} data diverged");
            // reset must restore the pristine all-unseen state: after it,
            // the first id is a first encounter again (palette index 0)
            remap.reset();
            assert_eq!(remap.remap(ids[0], 0), 0, "trial {trial} reset left stale entries");
        }
    }

    #[test]
    fn dense_remap_grows_for_out_of_range_ids() {
        // the pack paths can never produce ids beyond the shared table, but
        // the growth fallback must stay deterministic and first-encounter
        // exact anyway.
        let mut remap = DenseRemap::new(2);
        assert_eq!(remap.remap(9, 0), 0); // grows to 10
        assert_eq!(remap.remap(9, 1), 0); // still the same slot
        assert_eq!(remap.remap(4, 1), 1);
        assert_eq!(remap.remap(4, 2), 1);
        assert_eq!(remap.remap(0, 2), 2);
        assert_eq!(remap.remap(0, 3), 2);
    }

    #[test]
    fn pack_block_section_matches_hashmap_reference() {
        let mut rng = LegacyRandomSource::new(3053459);
        // adversarial shapes: all-same, random, all-distinct (bits ladder
        // 0 / 4 / 9), plus random biome fills
        let cases: Vec<Vec<u32>> = vec![
            vec![5u32; 4096],
            (0..4096).map(|_| rng.next_int_bound(37).max(0) as u32).collect(),
            (0..4096).map(|i| (i % 300) as u32).collect(),
            (0..4096).map(|_| rng.next_int_bound(1).max(0) as u32).collect(),
        ];
        for (ci, states) in cases.iter().enumerate() {
            let biomes: Vec<u16> = (0..64).map(|_| rng.next_int_bound(5).max(0) as u16).collect();
            let fc = test_chunk(states, &biomes, 300, 5);
            let packed = pack_block_section(&fc, 0);
            let (want_palette, want_data) = reference_first_encounter(states);
            let want_strings: Vec<String> =
                want_palette.iter().map(|&s| fc.state_table.get(s).canonical()).collect();
            assert_eq!(packed.palette, want_strings, "case {ci} palette");
            let bits = block_storage_bits(want_palette.len());
            let want_packed = if bits == 0 { None } else { Some(pack_bit_storage(&want_data, bits)) };
            assert_eq!(packed.data, want_packed, "case {ci} data");
        }
        // biome section (separate id space) vs the same reference walk
        let states: Vec<u32> = (0..4096).map(|_| rng.next_int_bound(37).max(0) as u32).collect();
        let biomes: Vec<u16> = (0..64).map(|_| rng.next_int_bound(5).max(0) as u16).collect();
        let fc = test_chunk(&states, &biomes, 300, 5);
        let packed = pack_biome_section(&fc, 0);
        let biome_ids: Vec<u32> = biomes.iter().map(|&b| b as u32).collect();
        let (want_palette, want_data) = reference_first_encounter(&biome_ids);
        let want_strings: Vec<String> =
            want_palette.iter().map(|&b| fc.biome_table.names[b as usize].clone()).collect();
        assert_eq!(packed.palette, want_strings, "biome palette");
        let bits = biome_storage_bits(want_palette.len());
        let want_packed = if bits == 0 { None } else { Some(pack_bit_storage(&want_data, bits)) };
        assert_eq!(packed.data, want_packed, "biome data");
    }

    #[test]
    fn pack_into_reuse_matches_fresh_per_call() {
        // R3#2b hoist contract: reusing remap+data buffers across sections
        // (the into-variants reset them internally) must produce the SAME
        // palette order and packed longs as the fresh per-call wrappers —
        // a missing reset would collapse every section after the first to
        // a single-entry palette and fail this immediately.
        let mut rng = LegacyRandomSource::new(0x0BEE_F00D);
        let states: Vec<u32> = (0..4096).map(|_| rng.next_int_bound(97).max(0) as u32).collect();
        let biomes: Vec<u16> = (0..64).map(|_| rng.next_int_bound(7).max(0) as u16).collect();
        let mut fc = test_chunk(&states, &biomes, 97, 7);
        // three sections so the reuse walk actually crosses section
        // boundaries (test_chunk builds a one-section chunk)
        let extra = fc.sections[0].clone();
        fc.sections.push(extra.clone());
        fc.sections.push(extra);
        let mut remap_b = DenseRemap::new(fc.state_table.states.len());
        let mut remap_m = DenseRemap::new(fc.biome_table.names.len());
        let mut data: Vec<u32> = Vec::new();
        let mut bdata: Vec<u32> = Vec::new();
        for i in 0..3 {
            let fresh_b = pack_block_section(&fc, 0);
            let fresh_m = pack_biome_section(&fc, 0);
            let into_b = pack_block_section_into(&fc, i, &mut remap_b, &mut data);
            let into_m = pack_biome_section_into(&fc, i, &mut remap_m, &mut bdata);
            assert_eq!(fresh_b.data, into_b.data, "section {i} block data");
            assert_eq!(fresh_m.data, into_m.data, "section {i} biome data");
            let strings: Vec<String> =
                into_b.palette_ids.iter().map(|&s| fc.state_table.get(s).canonical()).collect();
            assert_eq!(fresh_b.palette, strings, "section {i} block palette");
            let names: Vec<String> = into_m
                .palette_ids
                .iter()
                .map(|&b| fc.biome_table.names[b as usize].clone())
                .collect();
            assert_eq!(fresh_m.palette, names, "section {i} biome palette");
        }
    }

    #[test]
    fn ceillog2_semantics() {
        assert_eq!(ceillog2_usize(1), 0);
        assert_eq!(ceillog2_usize(2), 1);
        assert_eq!(ceillog2_usize(3), 2);
        assert_eq!(ceillog2_usize(4), 2);
        assert_eq!(ceillog2_usize(5), 3);
        assert_eq!(ceillog2_usize(16), 4);
        assert_eq!(ceillog2_usize(17), 5);
        assert_eq!(ceillog2_usize(256), 8);
        assert_eq!(ceillog2_usize(257), 9);
    }

    #[test]
    fn block_bit_ladder() {
        // Strategy$1: size 1 -> 0 (single-value no data); 2..16 -> 4; 17..32 -> 5
        assert_eq!(block_storage_bits(1), 0);
        assert_eq!(block_storage_bits(2), 4);
        assert_eq!(block_storage_bits(16), 4);
        assert_eq!(block_storage_bits(17), 5);
        assert_eq!(block_storage_bits(256), 8);
        assert_eq!(block_storage_bits(257), 9);
    }

    #[test]
    fn biome_bit_ladder() {
        // Strategy$2: 1 -> 0; 2 -> 1; 3 -> 2; 4..8 -> 3; 9..16 -> 4
        assert_eq!(biome_storage_bits(1), 0);
        assert_eq!(biome_storage_bits(2), 1);
        assert_eq!(biome_storage_bits(3), 2);
        assert_eq!(biome_storage_bits(4), 2);
        assert_eq!(biome_storage_bits(5), 3);
        assert_eq!(biome_storage_bits(9), 4);
    }

    #[test]
    fn packing_lsb_first() {
        // 4 bits, 16 values per long: value i in nibble i
        let values: Vec<u32> = (0..4096).map(|i| (i % 16) as u32).collect();
        let packed = pack_bit_storage(&values, 4);
        assert_eq!(packed.len(), 4096 / 16);
        for (i, &v) in packed.iter().enumerate() {
            let mut expect = 0i64;
            for k in 0..16 {
                expect |= (((i * 16 + k) % 16) as i64) << (k * 4);
            }
            assert_eq!(v, expect);
        }
    }
}
