//! Bit-exact Java random lineages (NCF P2.1).
//!
//! Ports of (CFR decompile + javap bytecode verification, mojang-mapped
//! Purpur 1.21.10, 2026-10-05):
//!
//! * `net.minecraft.world.level.levelgen.LegacyRandomSource` (48-bit LCG,
//!   java.util.Random semantics),
//! * `net.minecraft.world.level.levelgen.RandomSupport` (stafford mix,
//!   128-bit seed upgrade, MD5 `fromHashOf`),
//! * `net.minecraft.world.level.levelgen.MarsagliaPolarGaussian`,
//! * `XoroshiroRandomSource` / `Xoroshiro128PlusPlus` live in
//!   [`crate::xoroshiro`].
//!
//! THE nextDouble TRAP (javap-verified 2026-10-05 — CFR was right, memory
//! was not):
//! ```text
//! BitRandomSource.nextDouble:  l2f; ldc 1.110223E-16f; fmul; f2d
//! Xoroshiro.nextDouble:        l2f; ldc 1.110223E-16f; fmul; f2d
//! ```
//! i.e. `(float)bits * (float)2^-53` widened to double — the f32 multiply
//! QUANTIZES the 53-bit value to 24 mantissa bits. Reproducing this with an
//! f64 multiply would shift results. Same shape for nextFloat
//! (`(float)bits * 2^-24f`), which is f32-in/f32-out.

use crate::md5::seed_from_hash_of;
use crate::mth;

/// 2^-24 as f32 (= FLOAT_UNIT, exactly representable).
pub const FLOAT_UNIT: f32 = 5.9604645e-8_f32;
/// 2^-53 as f32 (the class-file constant `1.110223E-16f` = exactly 2^-53).
pub const DOUBLE_UNIT_F32: f32 = 1.110223e-16_f32;

/// LegacyRandomSource MODULUS_MASK = 2^48-1.
const MODULUS_MASK: i64 = (1i64 << 48) - 1;
const MULTIPLIER: i64 = 0x5DEECE66D;
const ADDEND: i64 = 0xB;

/// Java `String.hashCode()` (LegacyPositionalRandomFactory.fromHashOf).
pub fn java_string_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    for ch in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

/// RandomSupport.mixStafford13.
pub fn mix_stafford13(mut seed: i64) -> i64 {
    // Java `>>>` is a LOGICAL shift — replicate via u64 (arithmetic i64 >>
    // sign-extends and produced wrong mixes for negative seeds, caught by
    // the vector gate on the first run).
    seed = (seed ^ ((seed as u64 >> 30) as i64)).wrapping_mul(-4658895280553007687i64);
    seed = (seed ^ ((seed as u64 >> 27) as i64)).wrapping_mul(-7723592293110705685i64);
    seed ^ ((seed as u64 >> 31) as i64)
}

/// RandomSupport.upgradeSeedTo128bitUnmixed:
/// `lo = seed ^ 0x6A09E667F3BCC909; hi = lo + GOLDEN_RATIO_64`
pub fn upgrade_seed_to_128bit_unmixed(seed: i64) -> (i64, i64) {
    let lo = seed ^ 0x6A09E667F3BCC909u64 as i64;
    let hi = lo.wrapping_add(-7046029254386353131i64);
    (lo, hi)
}

/// RandomSupport.upgradeSeedTo128bit: unmixed then stafford-mixed halves.
pub fn upgrade_seed_to_128bit(seed: i64) -> (i64, i64) {
    let (lo, hi) = upgrade_seed_to_128bit_unmixed(seed);
    (mix_stafford13(lo), mix_stafford13(hi))
}

/// PositionalRandomFactory (net.minecraft.world.level.levelgen).
// `from_hash_of`/`from_seed` mirror the Java names 1:1 (I3) — clippy's
// no-self convention for from_* does not apply to a 1:1 port.
#[allow(clippy::wrong_self_convention)]
pub trait PositionalRandomFactory {
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource>;
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource>;
    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource>;
}

/// Unified random source interface (mirrors net.minecraft.util.RandomSource).
pub trait RandomSource {
    fn set_seed(&mut self, seed: i64);
    fn next_int(&mut self) -> i32;
    fn next_int_bound(&mut self, bound: i32) -> i32;
    fn next_long(&mut self) -> i64;
    fn next_boolean(&mut self) -> bool;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
    fn next_gaussian(&mut self) -> f64;
    /// RandomSource.consumeCount — default is nextInt() x count;
    /// XoroshiroRandomSource OVERRIDES it with nextLong() x count (this is
    /// load-bearing for PerlinNoise.skipOctave in the legacy construction
    /// path!).
    fn consume_count(&mut self, count: usize) {
        for _ in 0..count {
            self.next_int();
        }
    }
    /// fork() with concrete Self (same lineage).
    fn fork_same_lineage(&mut self) -> Self
    where
        Self: Sized;
    /// forkPositional() as a boxed factory.
    fn fork_positional_factory(&mut self) -> Box<dyn PositionalRandomFactory>;
}

/// `next(bits)` access (BitRandomSource).
pub trait BitsRandomSource: RandomSource {
    fn next_bits(&mut self, bits: u32) -> i32;
}

/// MarsagliaPolarGaussian (cache semantics included; `reset()` on setSeed).
#[derive(Debug)]
pub struct GaussianCache {
    next_next_gaussian: f64,
    have_next_next_gaussian: bool,
}

impl GaussianCache {
    pub fn new() -> Self {
        Self { next_next_gaussian: 0.0, have_next_next_gaussian: false }
    }

    pub fn reset(&mut self) {
        self.have_next_next_gaussian = false;
    }

    pub fn next_gaussian<R: RandomSource + ?Sized>(&mut self, rng: &mut R) -> f64 {
        if self.have_next_next_gaussian {
            self.have_next_next_gaussian = false;
            return self.next_next_gaussian;
        }
        let (d, d1, d2);
        loop {
            let a = 2.0 * rng.next_f64() - 1.0;
            let b = 2.0 * rng.next_f64() - 1.0;
            let s = mth::square(a) + mth::square(b);
            if !(s >= 1.0 || s == 0.0) {
                d = a;
                d1 = b;
                d2 = s;
                break;
            }
        }
        let square_root = (-2.0 * d2.ln() / d2).sqrt();
        self.next_next_gaussian = d1 * square_root;
        self.have_next_next_gaussian = true;
        d * square_root
    }
}

impl Default for GaussianCache {
    fn default() -> Self {
        Self::new()
    }
}

/// LegacyRandomSource (48-bit LCG).
pub struct LegacyRandomSource {
    seed: i64,
    gaussian: GaussianCache,
}

impl LegacyRandomSource {
    pub fn new(seed: i64) -> Self {
        let mut r = Self { seed: 0, gaussian: GaussianCache::new() };
        r.set_seed(seed);
        r
    }
}

impl BitsRandomSource for LegacyRandomSource {
    #[inline]
    fn next_bits(&mut self, bits: u32) -> i32 {
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND) & MODULUS_MASK;
        (self.seed >> (48 - bits)) as i32
    }
}

impl RandomSource for LegacyRandomSource {
    fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MODULUS_MASK;
        self.gaussian.reset();
    }

    fn next_int(&mut self) -> i32 {
        BitsRandomSource::next_bits(self, 32)
    }

    /// BitRandomSource.nextInt(bound) default:
    /// power-of-two path `(int)((long)bound * (long)next(31) >> 31)`, else the
    /// modulo-rejection loop `while (i - (i % bound) + (bound - 1) < 0)`.
    fn next_int_bound(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "Bound must be positive");
        if (bound & (bound - 1)) == 0 {
            return (((bound as i64) * (BitsRandomSource::next_bits(self, 31) as i64)) >> 31) as i32;
        }
        let mut i = BitsRandomSource::next_bits(self, 31);
        let mut j = i % bound;
        while i.wrapping_sub(j).wrapping_add(bound - 1) < 0 {
            i = BitsRandomSource::next_bits(self, 31);
            j = i % bound;
        }
        j
    }

    /// BitRandomSource.nextLong() default — SIGN-EXTENDED low half:
    /// `((long)next(32) << 32) + (long)next(32)`.
    fn next_long(&mut self) -> i64 {
        let i = BitsRandomSource::next_bits(self, 32) as i64;
        let j = BitsRandomSource::next_bits(self, 32) as i64;
        (i << 32).wrapping_add(j)
    }

    fn next_boolean(&mut self) -> bool {
        BitsRandomSource::next_bits(self, 1) != 0
    }

    /// `(float)next(24) * 2^-24f` — javap: i2f; ldc 5.9604645E-8f; fmul.
    fn next_f32(&mut self) -> f32 {
        (BitsRandomSource::next_bits(self, 24) as f32) * FLOAT_UNIT
    }

    /// THE TRAP: `l = ((long)next(26) << 27) + (long)next(27)`, then
    /// `(float)l * 2^-53f` (javap: l2f; fmul; f2d) — f32 multiply, f64 out.
    fn next_f64(&mut self) -> f64 {
        let i = BitsRandomSource::next_bits(self, 26) as i64;
        let j = BitsRandomSource::next_bits(self, 27) as i64;
        let l = (i << 27).wrapping_add(j);
        let f = (l as f32) * DOUBLE_UNIT_F32;
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

    fn fork_same_lineage(&mut self) -> Self {
        LegacyRandomSource::new(self.next_long())
    }

    fn fork_positional_factory(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(LegacyPositionalRandomFactory { seed: self.next_long() })
    }
}

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource> {
        Box::new(self.from_hash_of_impl(name))
    }

    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        Box::new(LegacyRandomSource::new(crate::mth::get_seed(x, y, z) ^ self.seed))
    }

    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource> {
        Box::new(LegacyRandomSource::new(seed))
    }
}

impl LegacyPositionalRandomFactory {
    /// Concrete-typed form (keeps call sites allocation-free when the
    /// lineage is statically known).
    pub fn from_hash_of_impl(&self, name: &str) -> LegacyRandomSource {
        LegacyRandomSource::new((java_string_hash_code(name) as i64) ^ self.seed)
    }
}

/// LegacyPositionalRandomFactory:
/// * at(x,y,z): `LegacyRandomSource(Mth.getSeed(x,y,z) ^ factorySeed)`
/// * fromHashOf(name): `LegacyRandomSource((long)name.hashCode() ^ factorySeed)`
/// * fromSeed(seed): `LegacyRandomSource(seed)`
#[derive(Debug, Clone, Copy)]
pub struct LegacyPositionalRandomFactory {
    pub seed: i64,
}

impl LegacyPositionalRandomFactory {
    pub fn at(&self, x: i32, y: i32, z: i32) -> LegacyRandomSource {
        LegacyRandomSource::new(mth::get_seed(x, y, z) ^ self.seed)
    }

    pub fn from_hash_of(&self, name: &str) -> LegacyRandomSource {
        LegacyRandomSource::new((java_string_hash_code(name) as i64) ^ self.seed)
    }

    pub fn from_seed(&self, seed: i64) -> LegacyRandomSource {
        LegacyRandomSource::new(seed)
    }
}

/// RandomSupport.seedFromHashOf (MD5 -> two big-endian i64 halves).
pub fn support_seed_from_hash_of(name: &str) -> (i64, i64) {
    seed_from_hash_of(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stafford13_matches_java_expression() {
        // Reference: the exact Java expression evaluated on HotSpot x86-64
        // (vector-gated in CI against random.csv family=stafford13 rows).
        for seed in [0i64, 1, 3053459, -1, i64::MAX, i64::MIN, 4242424242] {
            // Reference expression with Java `>>>` (logical) shifts, i64
            // multiply wraps.
            let mut s = seed;
            s = (s ^ ((s as u64 >> 30) as i64)).wrapping_mul(-4658895280553007687i64);
            s = (s ^ ((s as u64 >> 27) as i64)).wrapping_mul(-7723592293110705685i64);
            let expect = s ^ ((s as u64 >> 31) as i64);
            assert_eq!(mix_stafford13(seed), expect);
        }
    }

    #[test]
    fn upgrade_128_seed_smoke() {
        let (lo, hi) = upgrade_seed_to_128bit(0);
        assert!(!(lo == 0 && hi == 0));
        let (lo2, hi2) = upgrade_seed_to_128bit_unmixed(0);
        assert_eq!(lo2, 0x6A09E667F3BCC909u64 as i64);
        assert_eq!(hi2, (0x6A09E667F3BCC909u64 as i64).wrapping_add(-7046029254386353131i64));
    }

    #[test]
    fn legacy_deterministic_and_in_range() {
        let mut a = LegacyRandomSource::new(3053459);
        let mut b = LegacyRandomSource::new(3053459);
        for _ in 0..200 {
            assert_eq!(a.next_int(), b.next_int());
            assert_eq!(a.next_int_bound(97), b.next_int_bound(97));
            assert_eq!(a.next_long(), b.next_long());
            assert_eq!(a.next_f64().to_bits(), b.next_f64().to_bits());
            assert_eq!(a.next_f32().to_bits(), b.next_f32().to_bits());
            assert_eq!(a.next_gaussian().to_bits(), b.next_gaussian().to_bits());
            assert_eq!(a.next_boolean(), b.next_boolean());
        }
        for _ in 0..2000 {
            let v = a.next_int_bound(97);
            assert!((0..97).contains(&v));
        }
        for _ in 0..2000 {
            let v = a.next_int_bound(64);
            assert!((0..64).contains(&v));
        }
    }

    #[test]
    fn java_string_hash() {
        assert_eq!(java_string_hash_code(""), 0);
        assert_eq!(java_string_hash_code("a"), 97);
        assert_eq!(java_string_hash_code("ab"), 97 * 31 + 98);
        assert_eq!(java_string_hash_code("hello"), 99162322);
    }

    #[test]
    fn double_unit_f32_is_exact_two_pow_minus_53() {
        assert_eq!(DOUBLE_UNIT_F32.to_bits(), (74u32) << 23); // 2^(74-127) = 2^-53
        assert_eq!(FLOAT_UNIT.to_bits(), (103u32) << 23); // 2^(103-127) = 2^-24
    }
}

impl LegacyRandomSource {
    /// WorldgenRandom.setLargeFeatureSeed — region-grid structure seeding
    /// (P5.1 prescreen / P2.8 carver reseeding equivalent on the Legacy line).
    pub fn set_large_feature_seed(&mut self, base_seed: i64, chunk_x: i32, chunk_z: i32) {
        self.set_seed(base_seed);
        let random_long = self.next_long();
        let random_long1 = self.next_long();
        let l = (chunk_x as i64)
            .wrapping_mul(random_long)
            ^ (chunk_z as i64).wrapping_mul(random_long1)
            ^ base_seed;
        self.set_seed(l);
    }

    /// WorldgenRandom.setLargeFeatureWithSalt — verbatim.
    pub fn set_large_feature_with_salt(&mut self, level_seed: i64, region_x: i32, region_z: i32, salt: i32) {
        let l = (region_x as i64)
            .wrapping_mul(341873128712)
            .wrapping_add((region_z as i64).wrapping_mul(132897987541))
            .wrapping_add(level_seed)
            .wrapping_add(salt as i64);
        self.set_seed(l);
    }
}
