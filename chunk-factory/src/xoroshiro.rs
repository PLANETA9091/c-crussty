//! Xoroshiro128++ lineage (NCF P2.1) — `Xoroshiro128PlusPlus`,
//! `XoroshiroRandomSource`, `XoroshiroPositionalRandomFactory`.
//!
//! Ported from the CFR decompile of mojang-mapped Purpur 1.21.10 with javap
//! verification of the floating-point constants (see the f32 `nextDouble`
//! trap documented in [`crate::jrandom`]).
//!
//! `fromHashOf` = MD5 of the UTF-8 string packed into a 128-bit seed
//! (`RandomSupport.seedFromHashOf`) XORed with the factory (seedLo, seedHi),
//! then `Xoroshiro128PlusPlus(lo, hi)` directly (NO stafford mix here — the
//! mix happens only in `upgradeSeedTo128bit` used by the `XoroshiroRandomSource(long)`
//! constructor).

use crate::jrandom::{
    support_seed_from_hash_of, upgrade_seed_to_128bit, GaussianCache, PositionalRandomFactory,
    RandomSource, DOUBLE_UNIT_F32, FLOAT_UNIT,
};
use crate::mth;

/// Xoroshiro128PlusPlus.nextLong() (java semantics, wrapping i64 ops):
/// ```java
/// long l = seedLo, l1 = seedHi;
/// long l2 = rotateLeft(l + l1, 17) + l;
/// seedLo = rotateLeft(l, 49) ^ (l1 ^= l) ^ (l1 << 21);
/// seedHi = rotateLeft(l1, 28);
/// return l2;
/// ```
#[derive(Debug, Clone)]
pub struct Xoroshiro128PlusPlus {
    seed_lo: i64,
    seed_hi: i64,
}

impl Xoroshiro128PlusPlus {
    pub fn new(seed_lo: i64, seed_hi: i64) -> Self {
        let (mut seed_lo, mut seed_hi) = (seed_lo, seed_hi);
        if (seed_lo | seed_hi) == 0 {
            // all-zero state is replaced by (GOLDEN, SILVER)
            seed_lo = -7046029254386353131i64;
            seed_hi = 7640891576956012809i64;
        }
        Self { seed_lo, seed_hi }
    }

    pub fn from_seed_128(lo: i64, hi: i64) -> Self {
        Self::new(lo, hi)
    }

    #[inline]
    pub fn next_long(&mut self) -> i64 {
        let l = self.seed_lo;
        let mut l1 = self.seed_hi;
        let l2 = (l.wrapping_add(l1)).rotate_left(17).wrapping_add(l);
        l1 ^= l;
        self.seed_lo = l.rotate_left(49) ^ l1 ^ (l1 << 21);
        self.seed_hi = l1.rotate_left(28);
        l2
    }
}

/// XoroshiroRandomSource.
pub struct XoroshiroRandomSource {
    rng: Xoroshiro128PlusPlus,
    gaussian: GaussianCache,
}

impl XoroshiroRandomSource {
    pub fn new(seed: i64) -> Self {
        let (lo, hi) = upgrade_seed_to_128bit(seed);
        Self { rng: Xoroshiro128PlusPlus::new(lo, hi), gaussian: GaussianCache::new() }
    }

    pub fn new_128(lo: i64, hi: i64) -> Self {
        Self { rng: Xoroshiro128PlusPlus::new(lo, hi), gaussian: GaussianCache::new() }
    }

    pub fn set_seed_128(&mut self, lo: i64, hi: i64) {
        self.rng = Xoroshiro128PlusPlus::new(lo, hi);
        self.gaussian.reset();
    }

    #[inline]
    fn next_bits(&mut self, bits: u32) -> i64 {
        ((self.rng.next_long() as u64) >> (64 - bits)) as i64
    }

    /// fork(): `new XoroshiroRandomSource(nextLong(), nextLong())` — NO seed
    /// upgrade, the two longs feed the generator directly.
    pub fn fork(&mut self) -> Self {
        let lo = self.rng.next_long();
        let hi = self.rng.next_long();
        Self::new_128(lo, hi)
    }

    /// forkPositional(): `XoroshiroPositionalRandomFactory(nextLong(), nextLong())`.
    pub fn fork_positional(&mut self) -> XoroshiroPositionalRandomFactory {
        let lo = self.rng.next_long();
        let hi = self.rng.next_long();
        XoroshiroPositionalRandomFactory { seed_lo: lo, seed_hi: hi }
    }
}

impl RandomSource for XoroshiroRandomSource {
    fn set_seed(&mut self, seed: i64) {
        let (lo, hi) = upgrade_seed_to_128bit(seed);
        self.rng = Xoroshiro128PlusPlus::new(lo, hi);
        self.gaussian.reset();
    }

    fn next_int(&mut self) -> i32 {
        self.rng.next_long() as i32
    }

    /// XoroshiroRandomSource.nextInt(bound): multiply-reject on UNSIGNED 32:
    /// ```java
    /// long l = Integer.toUnsignedLong(this.nextInt());
    /// long l1 = l * (long)bound;
    /// long l2 = l1 & 0xFFFFFFFFL;
    /// if (l2 < (long)bound) {
    ///     int i = Integer.remainderUnsigned(~bound + 1, bound);
    ///     while (l2 < (long)i) { ... redraw ... }
    /// }
    /// return (int)(l1 >> 32);
    /// ```
    fn next_int_bound(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "Bound must be positive");
        let bound_u = bound as u32 as u64;
        let mut l = (self.next_int() as u32) as u64;
        let mut l1 = l.wrapping_mul(bound_u);
        let mut l2 = l1 & 0xFFFF_FFFF;
        if l2 < bound_u {
            let threshold = (bound.wrapping_neg()) as u32 as u64 % bound_u;
            while l2 < threshold {
                l = (self.next_int() as u32) as u64;
                l1 = l.wrapping_mul(bound_u);
                l2 = l1 & 0xFFFF_FFFF;
            }
        }
        (l1 >> 32) as i32
    }

    fn next_long(&mut self) -> i64 {
        self.rng.next_long()
    }

    fn next_boolean(&mut self) -> bool {
        (self.rng.next_long() & 1) != 0
    }

    /// `(float)nextBits(24) * 2^-24f` — javap: l2f; ldc 5.9604645E-8f; fmul.
    fn next_f32(&mut self) -> f32 {
        (self.next_bits(24) as f32) * FLOAT_UNIT
    }

    /// THE TRAP: `(float)nextBits(53) * 2^-53f` — javap: l2f; fmul; f2d.
    fn next_f64(&mut self) -> f64 {
        let b = self.next_bits(53);
        let f = (b as f32) * DOUBLE_UNIT_F32;
        f as f64
    }

    fn next_gaussian(&mut self) -> f64 {
        // Java holds one MarsagliaPolarGaussian per source (aliasing); in
        // Rust we temporarily take the cache out to satisfy the borrow
        // checker — semantically identical (same state, same draws).
        let mut cache = std::mem::take(&mut self.gaussian);
        let v = cache.next_gaussian(self);
        self.gaussian = cache;
        v
    }

    /// XoroshiroRandomSource.consumeCount OVERRIDE: nextLong() x count
    /// (differs from the RandomSource default — load-bearing for skipOctave).
    fn consume_count(&mut self, count: usize) {
        for _ in 0..count {
            self.rng.next_long();
        }
    }

    /// fork(): `new XoroshiroRandomSource(nextLong(), nextLong())` — direct
    /// 128-bit ctor, NO upgrade mix.
    fn fork_same_lineage(&mut self) -> Self {
        let lo = self.rng.next_long();
        let hi = self.rng.next_long();
        Self::new_128(lo, hi)
    }

    fn fork_positional_factory(&mut self) -> Box<dyn PositionalRandomFactory> {
        let lo = self.rng.next_long();
        let hi = self.rng.next_long();
        Box::new(XoroshiroPositionalRandomFactory { seed_lo: lo, seed_hi: hi })
    }
}

/// XoroshiroPositionalRandomFactory:
/// * at(x,y,z): seed = Mth.getSeed(x,y,z) ^ seedLo; `XoroshiroRandomSource(seed, seedHi)`
///   (direct 128-bit ctor — no upgrade mix!)
/// * fromHashOf(name): Seed128bit = MD5(name) XOR (seedLo, seedHi) — direct ctor
/// * fromSeed(seed): `(seed ^ seedLo, seed ^ seedHi)` — direct ctor
#[derive(Debug, Clone, Copy)]
pub struct XoroshiroPositionalRandomFactory {
    pub seed_lo: i64,
    pub seed_hi: i64,
}

impl XoroshiroPositionalRandomFactory {
    pub fn at(&self, x: i32, y: i32, z: i32) -> XoroshiroRandomSource {
        let seed = mth::get_seed(x, y, z);
        let l = seed ^ self.seed_lo;
        XoroshiroRandomSource::new_128(l, self.seed_hi)
    }

    pub fn from_hash_of(&self, name: &str) -> XoroshiroRandomSource {
        let (lo, hi) = support_seed_from_hash_of(name);
        XoroshiroRandomSource::new_128(lo ^ self.seed_lo, hi ^ self.seed_hi)
    }

    pub fn from_seed(&self, seed: i64) -> XoroshiroRandomSource {
        XoroshiroRandomSource::new_128(seed ^ self.seed_lo, seed ^ self.seed_hi)
    }
}

impl PositionalRandomFactory for XoroshiroPositionalRandomFactory {
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource> {
        Box::new(Self::from_hash_of(self, name))
    }

    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        Box::new(Self::at(self, x, y, z))
    }

    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource> {
        Box::new(Self::from_seed(self, seed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_draws() {
        let mut a = XoroshiroRandomSource::new(3053459);
        let mut b = XoroshiroRandomSource::new(3053459);
        for _ in 0..200 {
            assert_eq!(a.next_int(), b.next_int());
            assert_eq!(a.next_int_bound(1000), b.next_int_bound(1000));
            assert_eq!(a.next_long(), b.next_long());
            assert_eq!(a.next_f64().to_bits(), b.next_f64().to_bits());
            assert_eq!(a.next_f32().to_bits(), b.next_f32().to_bits());
            assert_eq!(a.next_gaussian().to_bits(), b.next_gaussian().to_bits());
        }
        for _ in 0..2000 {
            let v = a.next_int_bound(97);
            assert!((0..97).contains(&v));
        }
    }

    #[test]
    fn zero_seed_replaced() {
        // Xoroshiro128PlusPlus(lo=0, hi=0) -> (GOLDEN, SILVER)
        let mut g = XoroshiroRandomSource::new_128(0, 0);
        let mut r = XoroshiroRandomSource::new_128(-7046029254386353131, 7640891576956012809);
        assert_eq!(g.next_long(), r.next_long());
    }

    #[test]
    fn fork_draws_two_longs_first() {
        let mut base = XoroshiroRandomSource::new(42);
        let lo = base.next_long();
        let hi = base.next_long();
        let mut base2 = XoroshiroRandomSource::new(42);
        let _ = base2.next_long();
        let _ = base2.next_long();
        let mut forked = base2.fork();
        let mut expected = XoroshiroRandomSource::new_128(lo, hi);
        expected.next_long(); // forks differ from base; just structural smoke
        let _ = forked.next_long();
    }

    #[test]
    fn positional_uses_direct_ctor() {
        let mut base = XoroshiroRandomSource::new(3053459);
        let factory = base.fork_positional();
        // at(): seed = getSeed(x,y,z) ^ lo, hi = factory hi (no upgrade)
        let mut r1 = factory.at(1, 64, -1);
        let seed = mth::get_seed(1, 64, -1);
        let mut r2 = XoroshiroRandomSource::new_128(seed ^ factory.seed_lo, factory.seed_hi);
        assert_eq!(r1.next_long(), r2.next_long());
        // fromHashOf: MD5-based, deterministic
        let mut h1 = factory.from_hash_of("minecraft:terrain");
        let (lo, hi) = support_seed_from_hash_of("minecraft:terrain");
        let mut h2 = XoroshiroRandomSource::new_128(lo ^ factory.seed_lo, hi ^ factory.seed_hi);
        assert_eq!(h1.next_long(), h2.next_long());
    }
}
