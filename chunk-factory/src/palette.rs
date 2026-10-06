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

/// Repack one 4096-block section from raw first-encounter ids.
pub fn pack_block_section(fc: &FillerChunk, section_idx: usize) -> PackedSection {
    let sec = &fc.sections[section_idx];
    let mut palette: Vec<u32> = Vec::new();
    let mut index: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    let mut data = vec![0u32; 4096];
    for (j, &s) in sec.states.iter().enumerate() {
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
    let bits = block_storage_bits(palette.len());
    let packed = if bits == 0 {
        None
    } else {
        Some(pack_bit_storage(&data, bits))
    };
    PackedSection {
        palette: palette.iter().map(|&s| fc.state_table.get(s).canonical()).collect(),
        data: packed,
    }
}

pub fn pack_biome_section(fc: &FillerChunk, section_idx: usize) -> PackedSection {
    let sec = &fc.sections[section_idx];
    let mut palette: Vec<u16> = Vec::new();
    let mut index: std::collections::HashMap<u16, u32> = std::collections::HashMap::new();
    let mut data = vec![0u32; 64];
    for (j, &b) in sec.biomes.iter().enumerate() {
        let pi = match index.get(&b) {
            Some(&p) => p,
            None => {
                let p = palette.len() as u32;
                palette.push(b);
                index.insert(b, p);
                p
            }
        };
        data[j] = pi;
    }
    let bits = biome_storage_bits(palette.len());
    let packed = if bits == 0 {
        None
    } else {
        Some(pack_bit_storage(&data, bits))
    };
    PackedSection {
        palette: palette
            .iter()
            .map(|&b| fc.biome_table.names[b as usize].clone())
            .collect(),
        data: packed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
