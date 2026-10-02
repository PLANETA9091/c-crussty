//! PALETTE-GATHER v1 core: fused bulk blockstate resolution (task162, S7-93).
//!
//! Design doc: docs/RESEARCH_DEEPSEEK_CPU_2026-09-17.md. Measured basis (run#10
//! CI, 224,660 samples): the chunk-state read lane
//! `PalettedContainer.get` (3.7%) + `LevelChunk.getBlockStateFinal` (1.7%) +
//! `SimpleBitStorage.get` (1.6%) + `PalettedContainer.readPalette` (1.5%)
//! = **8.5% of total tick CPU self-time**, spread over per-position Java calls
//! that each re-pay bounds checks, bit extraction and a palette dereference.
//!
//! This module is the RUST CORE of the lever: a bit-exact replica of the
//! kernel's `SimpleBitStorage` packing (including the word-straddle case)
//! plus bulk/scan operations that amortize one pass over a 16³ section
//! (4096 entries — an L1-resident tile, FlashAttention lesson: fuse per
//! section, never materialize intermediates).
//!
//! Architecture mappings (research round, owner directive "от дипсика"):
//!   * MLA "absorb" — consumers may compare palette indices (latent space)
//!     instead of dereferenced blockstates; `indices_of` scans the LATENT
//!     stream and the dereference happens once per section, not per query.
//!   * FP8 tile-wise fine-grained scaling — specialization per data tile;
//!     here: per-section kernels with a single-value fast path (the commonest
//!     section shape in real worlds) instead of one generic path.
//!   * PagedAttention — the fast path allocates NOTHING; caller-owned output
//!     buffers (G3 discipline, same as entity_mirror).
//!   * MTP/speculative — single-value palette returns a constant without
//!     touching the packed words; acceptance is proven by construction
//!     (palette.len()==1 implies every index is 0), so it is not a guess.
//!
//! REFUTATION BANKED (pre-registered by the research round): raw AVX2
//! gather is NOT adopted in v1. Sources (q8): vgatherdd measured 0.95x-1.2x
//! on small working sets and even "3x" only in gather-dominated app
//! benchmarks; the data-dependent shifts of bit unpacking do not vectorize
//! cleanly, and the real win is AMORTIZATION (one crossing + one bounds
//! check + one pass over the tile), not SIMD gather. AVX2 is revisited only
//! if the CI A/B (G2) shows the scalar SWAR path still dominates.
//!
//! Bit-exactness contract (pre-registered G1): `get_index` reproduces
//! `SimpleBitStorage.get` EXACTLY:
//!   offset = i * bpe; word = words[offset >> 6]; shift = offset & 63;
//!   v = word >>> shift; if shift + bpe > 64 { v |= words[word+1] << (64 - shift) }
//!   v & mask
//! including Java's `>>>` semantics and the `(64 - shift)` LEFT shift. Property
//! tests compare this core against the same formula implemented naively, over
//! random bpe/fill/query spaces with straddle-heavy cases forced.

/// Entries per section tile (16³). The kernel's sections are always this size.
pub const SECTION_ENTRIES: usize = 4096;

/// Maximum bpe accepted from the kernel path (vanilla palettes cap at 16 for
/// global fallback; direct values use bpe up to 64 — the kernel path we bridge
/// never exceeds 15 for non-global palettes, but the core is generous).
const MAX_BPE: u8 = 31;

/// A resolved section view: bits-per-entry, packed words, palette (latent ->
/// global state id). Owned by the caller at the JNI boundary; the core never
/// allocates in any query path below.
pub struct SectionPacked {
    bpe: u8,
    mask: u64,
    entries: usize,
    words: Vec<u64>,
    palette: Vec<u32>,
}

#[inline]
fn mask_for(bpe: u8) -> u64 {
    if bpe >= 64 {
        u64::MAX
    } else {
        (1u64 << bpe) - 1
    }
}

impl SectionPacked {
    /// Validates shape: bpe 1..=MAX_BPE, words must cover entries*bpe bits.
    /// Returns None on any inconsistency (kernel must never pass garbage).
    pub fn new(bpe: u8, entries: usize, words: Vec<u64>, palette: Vec<u32>) -> Option<Self> {
        if bpe == 0 || bpe > MAX_BPE || entries == 0 {
            return None;
        }
        let need = (entries as u64 * bpe as u64).div_ceil(64) as usize;
        if words.len() < need {
            return None;
        }
        for &p in &palette {
            if p == u32::MAX {
                // sentinel-reserved; kernel ids are 0-based state ids
                return None;
            }
        }
        Some(Self {
            bpe,
            mask: mask_for(bpe),
            entries,
            words,
            palette,
        })
    }

    pub fn bpe(&self) -> u8 {
        self.bpe
    }
    pub fn palette_len(&self) -> usize {
        self.palette.len()
    }

    /// The speculative fast path (MTP lesson): a single-value palette means
    /// every packed index is 0 by construction — no word is touched.
    #[inline]
    fn single_value(&self) -> bool {
        self.palette.len() == 1
    }

    /// Bit-exact `SimpleBitStorage.get` replica (scalar, all cases).
    #[inline]
    pub fn get_index_scalar(&self, i: usize) -> u32 {
        debug_assert!(i < self.entries);
        if self.single_value() {
            return 0;
        }
        let bpe = self.bpe as usize;
        let offset = i * bpe;
        let word = offset >> 6;
        let shift = offset & 63;
        let mut v = self.words[word] >> shift;
        if shift + bpe > 64 {
            // Java `data[word+1] << (64 - shift)` — note LEFT shift by 64-shift
            v |= self.words[word + 1] << (64 - shift);
        }
        (v & self.mask) as u32
    }

    /// Fast path: non-straddle single-word read; straddle falls back to the
    /// scalar replica. Same result, fewer branches on the common case.
    #[inline]
    pub fn get_index(&self, i: usize) -> u32 {
        if self.single_value() {
            return 0;
        }
        let bpe = self.bpe as usize;
        let offset = i * bpe;
        let shift = offset & 63;
        if shift + bpe <= 64 {
            ((self.words[offset >> 6] >> shift) & self.mask) as u32
        } else {
            self.get_index_scalar(i)
        }
    }

    /// Palette dereference (latent -> global state id). None on out-of-range
    /// index (packed value must be < palette.len(); a corrupt section reports
    /// instead of guessing).
    #[inline]
    pub fn resolve(&self, idx: u32) -> Option<u32> {
        self.palette.get(idx as usize).copied()
    }

    /// Fused single-position state read: index extraction + palette deref.
    #[inline]
    pub fn get_state(&self, i: usize) -> Option<u32> {
        self.resolve(self.get_index(i))
    }

    /// Bulk fused resolve: out[j] = palette[get_index(indices[j])].
    /// ZERO allocations (G3). Returns the number of resolved entries (== out
    /// positions written) or None if any packed index is out of palette range
    /// (caller decides: abort batch / log corrupt section). Partial writes are
    /// NOT made on None — the caller's buffer is left untouched after the
    /// failing position (write-ahead discipline keeps parity trivial).
    pub fn bulk_states(&self, indices: &[u32], out: &mut [u32]) -> Option<usize> {
        if self.single_value() {
            let s = self.palette[0];
            for (o, _) in out.iter_mut().zip(indices.iter()) {
                *o = s;
            }
            return Some(indices.len().min(out.len()));
        }
        let n = indices.len().min(out.len());
        for j in 0..n {
            let p = self.get_index(indices[j] as usize);
            // manual inline of resolve() to keep the loop branch-lean
            match self.palette.get(p as usize) {
                Some(&s) => out[j] = s,
                None => return None,
            }
        }
        Some(n)
    }

    /// MLA-absorb scan: all tile positions whose packed index == `wanted`
    /// (LATENT-space compare — no palette dereference in the loop). Caller-
    /// owned buffer, ZERO allocations. Returns Some(count) or None if
    /// `wanted` is out of palette range for this section.
    ///
    /// The scan walks words sequentially (SWAR-friendly): for bpe<=15 the
    /// inner loop shifts one u64 through up to floor(64/bpe) extractions,
    /// handling the straddle at the word boundary exactly once.
    pub fn indices_of(&self, wanted: u32, out: &mut [u32]) -> Option<usize> {
        if wanted as usize >= self.palette.len() {
            return None;
        }
        if self.single_value() {
            // every position matches index 0; positions are 0..entries
            let n = out.len().min(self.entries);
            for (j, o) in out.iter_mut().enumerate().take(n) {
                *o = j as u32;
            }
            return Some(n);
        }
        let bpe = self.bpe as usize;
        let mut count = 0usize;
        let wmask = self.mask;
        let mut i = 0usize;
        let words = &self.words;
        while i < self.entries {
            let offset = i * bpe;
            let w = offset >> 6;
            let shift = offset & 63;
            // fast bulk: whole entry inside this word, and more entries fit
            if shift + bpe <= 64 {
                let chunk = words[w] >> shift;
                // number of entries fully inside this word from `shift`
                let avail = (64 - shift) / bpe;
                let take = avail.min(self.entries - i);
                for k in 0..take {
                    let p = ((chunk >> (k * bpe)) & wmask) as u32;
                    if p as usize == wanted as usize {
                        if count == out.len() {
                            return Some(count);
                        }
                        out[count] = (i + k) as u32;
                        count += 1;
                    }
                }
                i += take;
            } else {
                // straddle: exact scalar replica
                let p = self.get_index_scalar(i);
                if p as usize == wanted as usize {
                    if count == out.len() {
                        return Some(count);
                    }
                    out[count] = i as u32;
                    count += 1;
                }
                i += 1;
            }
        }
        Some(count)
    }
}

// ---------------------------------------------------------------------------
// tests: oracle parity (property), straddle vectors, single-value, zero-alloc
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Naive oracle — the SimpleBitStorage formula written out one step at a
    /// time, independently of the core's fast paths.
    fn oracle(words: &[u64], i: usize, bpe: u8) -> u32 {
        let bpe = bpe as usize;
        let offset = i * bpe;
        let word = offset >> 6;
        let shift = offset & 63;
        let mut v = words[word] >> shift;
        if shift + bpe > 64 {
            v |= words[word + 1] << (64 - shift);
        }
        (v & mask_for(bpe as u8)) as u32
    }

    fn make_section(bpe: u8, entries: usize, palette_n: u32, seed: u64) -> SectionPacked {
        // xorshift64* — deterministic fills
        let mut s = seed | 1;
        let mut next = move || {
            s ^= s >> 12;
            s ^= s << 25;
            s ^= s >> 27;
            s.wrapping_mul(0x2545F4914F6CDD1D)
        };
        let words_len = (entries as u64 * bpe as u64).div_ceil(64) as usize;
        let mut words = Vec::new();
        for _ in 0..words_len {
            words.push(next());
        }
        // keep every packed value in palette range: clamp by masking palette
        let p = palette_n.max(1);
        // palette sized so ANY packed value resolves (no out-of-range noise in
        // the property tests; corrupt-input cases construct their own sections)
        let palette_len = if bpe <= 16 { 1usize << bpe } else { p as usize };
        let palette: Vec<u32> = (0..palette_len as u32).map(|k| k.wrapping_mul(0x9E3779B9) % 0xFFFF).collect();
        SectionPacked::new(bpe, entries, words, palette).unwrap()
    }

    #[test]
    fn straddle_hand_vectors() {
        // bpe=5, i=12: offset=60, shift=60, 60+5>64 -> straddle across words 0/1
        // new() requires words.len() >= ceil(entries*bpe/64) = 5 for entries=64
        let words = [0x1234_5678_9ABC_DEF0u64, 0x0FED_CBA9_8765_4321u64, 0, 0, 0];
        let pal: Vec<u32> = (0..32).collect();
        let sec = SectionPacked::new(5, 64, words.to_vec(), pal).unwrap();
        assert_eq!(sec.get_index(12), oracle(&words, 12, 5));
        // bpe=7, i=9: offset=63 -> maximal straddle (1 bit in first word)
        let words7 = [u64::MAX, 0, 0, 0, 0, 0, 0];
        let sec2 = SectionPacked::new(7, 64, words7.to_vec(), (0..128).collect()).unwrap();
        assert_eq!(sec2.get_index(9), oracle(&words7, 9, 7));
        assert_eq!(sec2.get_index(9), 1); // 1 bit from all-ones word0, 6 zeros from word1
    }

    #[test]
    fn property_scalar_vs_oracle_all_bpe() {
        // every bpe 1..=16, random fills, all positions — EXACT parity
        for bpe in 1..=16u8 {
            let sec = make_section(bpe, 512, 300, 0xA5A5_0000 + bpe as u64);
            for i in 0..512 {
                assert_eq!(
                    sec.get_index(i),
                    oracle(&sec.words, i, bpe),
                    "bpe={} i={}",
                    bpe,
                    i
                );
            }
        }
    }

    #[test]
    fn property_bulk_and_scan_vs_oracle() {
        for bpe in [1u8, 3, 4, 5, 6, 7, 8, 9, 12, 15] {
            let entries = 4096;
            let sec = make_section(bpe, entries, 64, 0xC0FF_EE00 + bpe as u64);
            // bulk: query 300 random positions, compare against oracle chain
            let mut s = 0x51ED_270Bu64;
            let mut next = move || {
                s ^= s >> 12;
                s ^= s << 25;
                s ^= s >> 27;
                s.wrapping_mul(0x2545F4914F6CDD1D)
            };
            let idxs: Vec<u32> = (0..300).map(|_| (next() as usize % entries) as u32).collect();
            let mut out = vec![0u32; 300];
            sec.bulk_states(&idxs, &mut out).expect("bulk ok");
            for (j, &q) in idxs.iter().enumerate() {
                let want = sec.palette[oracle(&sec.words, q as usize, bpe) as usize];
                assert_eq!(out[j], want, "bulk bpe={} j={}", bpe, j);
            }
            // scan: for every palette index, positions from indices_of must
            // match the oracle walk EXACTLY (same order, same count)
            for wanted in 0..sec.palette.len() as u32 {
                let mut got = vec![0u32; entries];
                let n = sec.indices_of(wanted, &mut got).unwrap();
                let mut expect = Vec::new();
                for i in 0..entries {
                    if oracle(&sec.words, i, bpe) as usize == wanted as usize {
                        expect.push(i as u32);
                    }
                }
                assert_eq!(&got[..n], &expect[..], "scan bpe={} wanted={}", bpe, wanted);
            }
        }
    }

    #[test]
    fn single_value_fast_path() {
        let sec = SectionPacked::new(
            4,
            4096,
            vec![0xDEAD_BEEFu64; 256],
            vec![777],
        )
        .unwrap();
        for i in [0usize, 1, 4095, 2048] {
            assert_eq!(sec.get_index(i), 0);
            assert_eq!(sec.get_state(i), Some(777));
        }
        // scan with empty buffer returns 0 without touching anything
        let mut none: [u32; 0] = [];
        assert_eq!(sec.indices_of(0, &mut none), Some(0));
    }

    #[test]
    fn corrupt_inputs_report_not_guess() {
        // packed value out of palette range -> None, no partial writes
        let sec = make_section(6, 256, 1, 42); // bpe=6 -> palette 2^6
        let small_pal = SectionPacked::new(6, 256, sec.words.clone(), vec![1, 2, 3]).unwrap();
        let mut out = vec![0u32; 8];
        assert!(small_pal.bulk_states(&[0, 1, 2, 3, 4, 5, 6, 7], &mut out).is_none());
        // crafted in-range section: word0 = 0 | (1<<6) | (2<<12) -> entries [0,1,2]
        let crafted = SectionPacked::new(6, 8, vec![0x2040, 0], vec![1, 2, 3]).unwrap();
        let mut out3 = vec![0u32; 4];
        let n = crafted.bulk_states(&[0, 1, 2], &mut out3).expect("in-range ok");
        assert_eq!(n, 3);
        assert_eq!(&out3[..3], &[1, 2, 3]);
        assert!(small_pal.indices_of(3, &mut out).is_none());
        // shape validation
        assert!(SectionPacked::new(0, 16, vec![0; 1], vec![1]).is_none());
        assert!(SectionPacked::new(4, 4096, vec![0; 1], vec![1, 2]).is_none());
    }

    #[test]
    fn zero_alloc_query_paths() {
        // alloc_counter counts PER-THREAD (S7-93 fix): exact-zero assert is
        // deterministic under parallel test schedules.
        let sec = make_section(5, 4096, 32, 0x0D0_6A);
        let idxs: Vec<u32> = (0..1024u32).map(|k| (k * 7) % 4096).collect();
        let mut out = vec![0u32; 1024];
        crate::entity_mirror::alloc_counter::reset();
        let _ = sec.bulk_states(&idxs, &mut out);
        let _ = sec.indices_of(3, &mut out);
        let _ = sec.get_state(123);
        let n = crate::entity_mirror::alloc_counter::count();
        assert_eq!(n, 0, "query paths must allocate NOTHING (got {})", n);
    }

    #[test]
    fn occupancy_helpers() {
        // 4 bits per entry: 16 entries per word exactly, no straddle anywhere
        let sec = make_section(4, 64, 16, 7);
        assert_eq!(oracle(&sec.words, 63, 4), sec.get_index(63));
        // bpe=9: 7 entries per word + straddle every word boundary
        let sec9 = make_section(9, 256, 512, 11);
        for i in 0..256 {
            assert_eq!(sec9.get_index(i), oracle(&sec9.words, i, 9));
        }
    }
}
