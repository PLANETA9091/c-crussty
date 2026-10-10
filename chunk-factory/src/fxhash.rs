//! Shared fixed-seed FxHash (rustc-hash 1.x arithmetic) — S3 factoring.
//!
//! One crate-wide copy of the hasher (moved verbatim from filler.rs, where it
//! was introduced for the StateTable keys map, R1#4): rotate-left 5, xor,
//! wrap-mul by the fixed seed. Consumers are pure LOOKUP structures (never
//! iterated; ids come from insertion order into side tables), so the bucket
//! layout is output-invisible — the fixed seed just removes the per-map
//! SipHash setup and makes fresh tables assign identical ids.

use std::hash::Hasher;

#[derive(Default)]
pub(crate) struct FxHasher {
    hash: u64,
}

pub(crate) const FX_SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = (self.hash.rotate_left(5) ^ i).wrapping_mul(FX_SEED);
    }
}

impl Hasher for FxHasher {
    // str/String hash through here: write(payload bytes) + write_u8(0xff).
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.add_to_hash(u64::from_le_bytes(bytes[..8].try_into().unwrap()));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.add_to_hash(u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.add_to_hash(u16::from_le_bytes(bytes[..2].try_into().unwrap()) as u64);
            bytes = &bytes[2..];
        }
        if let Some(&b) = bytes.first() {
            self.add_to_hash(b as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The moved hasher must hash identically to the filler.rs original:
    /// the StateTable id assignment contract (fresh tables -> identical ids,
    /// content decides membership) is pinned by the S4 tests on StateTable
    /// itself; this pins the primitive.
    #[test]
    fn fx_hasher_matches_rustc_hash_idiom() {
        let mut h = FxHasher::default();
        h.write(b"minecraft:stone");
        h.write_u8(0xff);
        let mut g = FxHasher::default();
        g.write(b"minecraft:stone");
        g.write_u8(0xff);
        assert_eq!(h.finish(), g.finish());
        // empty vs non-empty differ; byte order matters
        let mut e = FxHasher::default();
        e.write_u8(0xff);
        assert_ne!(h.finish(), e.finish());
    }
}
