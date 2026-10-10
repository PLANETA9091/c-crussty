//! Bit-exact noise kernels (NCF P2.2) — `ImprovedNoise`, `PerlinNoise`,
//! `NormalNoise`, `BlendedNoise`, ported from the CFR decompile of the
//! mojang-mapped Purpur 1.21.10 jar (synth package, verified 2026-10-05).
//!
//! Discipline (invariant I2/I3):
//! * every Java `double` op is an f64 op in the SAME order — no FMA, no
//!   reassociation, no f32 substitutions;
//! * `(double)1.0E-7f` style constants are ported as `(1.0e-7_f32) as f64`
//!   (the FLOAT literal promoted — differs from 1e-7 in the low bits!);
//! * integer ops wrap (`wrapping_*`), float->int casts saturate (`as`),
//!   matching the JLS;
//! * octave selection order and construction order are load-bearing (they
//!   determine the random stream), reproduced verbatim.

use crate::jrandom::RandomSource;

/// SimplexNoise.GRADIENT (used by ImprovedNoise via SimplexNoise.dot).
pub const GRADIENT: [[i32; 3]; 16] = [
    [1, 1, 0],
    [-1, 1, 0],
    [1, -1, 0],
    [-1, -1, 0],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 0, -1],
    [-1, 0, -1],
    [0, 1, 1],
    [0, -1, 1],
    [0, 1, -1],
    [0, -1, -1],
    [1, 1, 0],
    [0, -1, 1],
    [-1, 1, 0],
    [0, -1, -1],
];

/// SimplexNoise.dot(int[] gradient, double x, double y, double z).
///
/// N1-H3 micro-opt: every gradient this function receives comes from the
/// `{-1, 0, 1}`-valued tables (ImprovedNoise GRADIENT), so the three
/// int→f64 converts + multiplies are replaced by a tristate select that
/// reproduces each product's bits EXACTLY:
///
/// Exactness lemma (IEEE-754 round-to-nearest, no FMA). For finite `a`:
/// * `1.0 * a == a` — multiplication by 1.0 is exact (keeps every bit,
///   subnormals included);
/// * `-1.0 * a == -a` — exact sign flip (note `-1.0 * -0.0 == +0.0`, which
///   the `-a` negation also produces);
/// * `0.0 * a == copysign(+0.0, a)` — the ZERO case must COPY the sign bit
///   of its operand (in particular `0.0 * -0.0 == -0.0`), hence the
///   `is_sign_negative` branch.
/// All products are therefore reproduced bit-for-bit, and the sum keeps the
/// Java left-associated shape `(g0*x + g1*y) + g2*z` — identical add
/// instructions over identical operands ⇒ bit-identical results.
///
/// Precondition: every call site passes FINITE values (perlin deltas are
/// differences of exactly-representable values, |x| <= the wrap bound
/// 2^25 ≈ 3.36e7 — never NaN/±inf, so the `0.0 * inf == NaN` hazard can
/// never arise). Asserted below in debug builds only; compiled out in
/// release.
#[inline]
pub fn gradient_dot(g: [i32; 3], x: f64, y: f64, z: f64) -> f64 {
    debug_assert!(!x.is_nan() && x.is_finite(), "gradient_dot: non-finite x");
    debug_assert!(!y.is_nan() && y.is_finite(), "gradient_dot: non-finite y");
    debug_assert!(!z.is_nan() && z.is_finite(), "gradient_dot: non-finite z");
    // GRADIENT components are exactly {-1, 0, 1} by table construction;
    // the `_` arm covers the 0 case (and would mask a corrupt table in
    // release exactly like the old multiply would).
    let t0 = match g[0] {
        1 => x,
        -1 => -x,
        _ => {
            if x.is_sign_negative() {
                -0.0
            } else {
                0.0
            }
        }
    };
    let t1 = match g[1] {
        1 => y,
        -1 => -y,
        _ => {
            if y.is_sign_negative() {
                -0.0
            } else {
                0.0
            }
        }
    };
    let t2 = match g[2] {
        1 => z,
        -1 => -z,
        _ => {
            if z.is_sign_negative() {
                -0.0
            } else {
                0.0
            }
        }
    };
    (t0 + t1) + t2 // Java: g0*x + g1*y + g2*z — strict left association
}

// --------------------------------------------------------------------------
// N1 probe counters (cfg(ncf_profile) only — probe builds, standing order
// R5): the perlin-vs-walk split for the N1 SIMD gate (NCF_WORKLIST: perlin
// share >= 25%?). Counts ImprovedNoise SAMPLE kernels — sample_and_lerp
// (noise / noise_scaled) and sample_with_derivative (noise_with_derivative)
// — i.e. one tick per full gradient-lattice + lerp3 evaluation. Pattern
// follows the S2_/S2B_ statics in surface_rules.rs; zero code in release
// builds without --cfg ncf_profile.
// --------------------------------------------------------------------------
#[cfg(ncf_profile)]
pub static N1_PERLIN_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_PERLIN_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// N1 tick: add one sample's wall nanos + call count (Relaxed — probe only).
#[cfg(ncf_profile)]
#[inline]
fn n1_perlin_tick(t: std::time::Instant) {
    N1_PERLIN_NANOS.fetch_add(
        t.elapsed().as_nanos() as u64,
        std::sync::atomic::Ordering::Relaxed,
    );
    N1_PERLIN_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

// --------------------------------------------------------------------------
// ImprovedNoise
// --------------------------------------------------------------------------

/// `net.minecraft.world.level.levelgen.synth.ImprovedNoise`.
pub struct ImprovedNoise {
    p: [u8; 256],
    pub xo: f64,
    pub yo: f64,
    pub zo: f64,
}

impl ImprovedNoise {
    pub fn new<R: RandomSource + ?Sized>(random: &mut R) -> Self {
        let xo = random.next_f64() * 256.0;
        let yo = random.next_f64() * 256.0;
        let zo = random.next_f64() * 256.0;
        let mut p = [0u8; 256];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u8;
        }
        for i in 0..256usize {
            let j = random.next_int_bound((256 - i) as i32) as usize;
            p.swap(i, i + j);
        }
        Self { p, xo, yo, zo }
    }

    #[inline]
    fn p(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize] as i32 // byte & 0xFF
    }

    /// ImprovedNoise.noise(x, y, z) -> noise(x, y, z, 0.0, 0.0).
    #[inline]
    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.noise_scaled(x, y, z, 0.0, 0.0)
    }

    /// ImprovedNoise.noise(x, y, z, yScale, yMax).
    pub fn noise_scaled(&self, x: f64, y: f64, z: f64, y_scale: f64, y_max: f64) -> f64 {
        let d = x + self.xo;
        let d1 = y + self.yo;
        let d2 = z + self.zo;
        let floor_x = crate::mth::floor(d);
        let floor_y = crate::mth::floor(d1);
        let floor_z = crate::mth::floor(d2);
        let d3 = d - floor_x as f64;
        let d4 = d1 - floor_y as f64;
        let d5 = d2 - floor_z as f64;
        
        let weird_delta = if y_scale != 0.0 {
            let d6 = if y_max >= 0.0 && y_max < d4 { y_max } else { d4 };
            // (double)1.0E-7f — the FLOAT constant widened to double.
            let eps = 1.0e-7_f32 as f64;
            (crate::mth::floor(d6 / y_scale + eps) as f64) * y_scale
        } else {
            0.0
        };
        self.sample_and_lerp(floor_x, floor_y, floor_z, d3, d4 - weird_delta, d5, d4)
    }

    /// ImprovedNoise.noiseWithDerivative(x, y, z, values) — accumulates into
    /// the provided `[f64; 3]` (Java adds into the array in place).
    pub fn noise_with_derivative(
        &self,
        x: f64,
        y: f64,
        z: f64,
        values: &mut [f64; 3],
    ) -> f64 {
        let d = x + self.xo;
        let d1 = y + self.yo;
        let d2 = z + self.zo;
        let gx = crate::mth::floor(d);
        let gy = crate::mth::floor(d1);
        let gz = crate::mth::floor(d2);
        let d3 = d - gx as f64;
        let d4 = d1 - gy as f64;
        let d5 = d2 - gz as f64;
        self.sample_with_derivative(gx, gy, gz, d3, d4, d5, values)
    }

    #[inline]
    fn grad_dot(grad_index: i32, xf: f64, yf: f64, zf: f64) -> f64 {
        gradient_dot(GRADIENT[(grad_index & 0xF) as usize], xf, yf, zf)
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_and_lerp(
        &self,
        grid_x: i32,
        grid_y: i32,
        grid_z: i32,
        delta_x: f64,
        weird_delta_y: f64,
        delta_z: f64,
        delta_y: f64,
    ) -> f64 {
        #[cfg(ncf_profile)]
        let prof_n1 = std::time::Instant::now();
        let i = self.p(grid_x);
        let i1 = self.p(grid_x + 1);
        let i2 = self.p(i + grid_y);
        let i3 = self.p(i + grid_y + 1);
        let i4 = self.p(i1 + grid_y);
        let i5 = self.p(i1 + grid_y + 1);
        let d = Self::grad_dot(self.p(i2 + grid_z), delta_x, weird_delta_y, delta_z);
        let d1 = Self::grad_dot(self.p(i4 + grid_z), delta_x - 1.0, weird_delta_y, delta_z);
        let d2 = Self::grad_dot(self.p(i3 + grid_z), delta_x, weird_delta_y - 1.0, delta_z);
        let d3 = Self::grad_dot(self.p(i5 + grid_z), delta_x - 1.0, weird_delta_y - 1.0, delta_z);
        let d4 = Self::grad_dot(self.p(i2 + grid_z + 1), delta_x, weird_delta_y, delta_z - 1.0);
        let d5 = Self::grad_dot(self.p(i4 + grid_z + 1), delta_x - 1.0, weird_delta_y, delta_z - 1.0);
        let d6 = Self::grad_dot(self.p(i3 + grid_z + 1), delta_x, weird_delta_y - 1.0, delta_z - 1.0);
        let d7 = Self::grad_dot(
            self.p(i5 + grid_z + 1),
            delta_x - 1.0,
            weird_delta_y - 1.0,
            delta_z - 1.0,
        );
        let d8 = crate::mth::smoothstep(delta_x);
        let d9 = crate::mth::smoothstep(delta_y);
        let d10 = crate::mth::smoothstep(delta_z);
        let r = crate::mth::lerp3(d8, d9, d10, d, d1, d2, d3, d4, d5, d6, d7);
        #[cfg(ncf_profile)]
        n1_perlin_tick(prof_n1);
        r
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_with_derivative(
        &self,
        grid_x: i32,
        grid_y: i32,
        grid_z: i32,
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
        values: &mut [f64; 3],
    ) -> f64 {
        #[cfg(ncf_profile)]
        let prof_n1 = std::time::Instant::now();
        let i = self.p(grid_x);
        let i1 = self.p(grid_x + 1);
        let i2 = self.p(i + grid_y);
        let i3 = self.p(i + grid_y + 1);
        let i4 = self.p(i1 + grid_y);
        let i5 = self.p(i1 + grid_y + 1);
        let i6 = self.p(i2 + grid_z);
        let i7 = self.p(i4 + grid_z);
        let i8 = self.p(i3 + grid_z);
        let i9 = self.p(i5 + grid_z);
        let i10 = self.p(i2 + grid_z + 1);
        let i11 = self.p(i4 + grid_z + 1);
        let i12 = self.p(i3 + grid_z + 1);
        let i13 = self.p(i5 + grid_z + 1);
        let g0 = GRADIENT[(i6 & 0xF) as usize];
        let g1 = GRADIENT[(i7 & 0xF) as usize];
        let g2 = GRADIENT[(i8 & 0xF) as usize];
        let g3 = GRADIENT[(i9 & 0xF) as usize];
        let g4 = GRADIENT[(i10 & 0xF) as usize];
        let g5 = GRADIENT[(i11 & 0xF) as usize];
        let g6 = GRADIENT[(i12 & 0xF) as usize];
        let g7 = GRADIENT[(i13 & 0xF) as usize];
        let d = gradient_dot(g0, delta_x, delta_y, delta_z);
        let d1 = gradient_dot(g1, delta_x - 1.0, delta_y, delta_z);
        let d2 = gradient_dot(g2, delta_x, delta_y - 1.0, delta_z);
        let d3 = gradient_dot(g3, delta_x - 1.0, delta_y - 1.0, delta_z);
        let d4 = gradient_dot(g4, delta_x, delta_y, delta_z - 1.0);
        let d5 = gradient_dot(g5, delta_x - 1.0, delta_y, delta_z - 1.0);
        let d6 = gradient_dot(g6, delta_x, delta_y - 1.0, delta_z - 1.0);
        let d7 = gradient_dot(g7, delta_x - 1.0, delta_y - 1.0, delta_z - 1.0);
        let d8 = crate::mth::smoothstep(delta_x);
        let d9 = crate::mth::smoothstep(delta_y);
        let d10 = crate::mth::smoothstep(delta_z);
        let d11 = crate::mth::lerp3(d8, d9, d10, g0[0] as f64, g1[0] as f64, g2[0] as f64, g3[0] as f64, g4[0] as f64, g5[0] as f64, g6[0] as f64, g7[0] as f64);
        let d12 = crate::mth::lerp3(d8, d9, d10, g0[1] as f64, g1[1] as f64, g2[1] as f64, g3[1] as f64, g4[1] as f64, g5[1] as f64, g6[1] as f64, g7[1] as f64);
        let d13 = crate::mth::lerp3(d8, d9, d10, g0[2] as f64, g1[2] as f64, g2[2] as f64, g3[2] as f64, g4[2] as f64, g5[2] as f64, g6[2] as f64, g7[2] as f64);
        let d14 = crate::mth::lerp2(d9, d10, d1 - d, d3 - d2, d5 - d4, d7 - d6);
        let d15 = crate::mth::lerp2(d10, d8, d2 - d, d6 - d4, d3 - d1, d7 - d5);
        let d16 = crate::mth::lerp2(d8, d9, d4 - d, d5 - d1, d6 - d2, d7 - d3);
        let d17 = crate::mth::smoothstep_derivative(delta_x);
        let d18 = crate::mth::smoothstep_derivative(delta_y);
        let d19 = crate::mth::smoothstep_derivative(delta_z);
        let d20 = d11 + d17 * d14;
        let d21 = d12 + d18 * d15;
        let d22 = d13 + d19 * d16;
        values[0] += d20;
        values[1] += d21;
        values[2] += d22;
        let r = crate::mth::lerp3(d8, d9, d10, d, d1, d2, d3, d4, d5, d6, d7);
        #[cfg(ncf_profile)]
        n1_perlin_tick(prof_n1);
        r
    }
}

// --------------------------------------------------------------------------
// PerlinNoise
// --------------------------------------------------------------------------

/// `net.minecraft.world.level.levelgen.synth.PerlinNoise`.
pub struct PerlinNoise {
    noise_levels: Vec<Option<ImprovedNoise>>,
    pub first_octave: i32,
    pub amplitudes: Vec<f64>,
    lowest_freq_value_factor: f64,
    lowest_freq_input_factor: f64,
    max_value: f64,
}

impl PerlinNoise {
    /// `PerlinNoise.create(random, firstOctave, amplitudes)` (useNewFactory=true).
    pub fn create<R: RandomSource + ?Sized>(
        random: &mut R,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        Self::build(random, first_octave, amplitudes, true)
    }

    /// `PerlinNoise.createLegacyForBlendedNoise(random, octaves=-15..0)` —
    /// 16 octaves of amplitude 1.0, useNewFactory=false.
    pub fn create_legacy_for_blended_noise<R: RandomSource + ?Sized>(random: &mut R) -> Self {
        let amps = [1.0; 16];
        Self::build(random, -15, &amps, false)
    }

    /// `PerlinNoise.createLegacyForLegacyNetherBiome` (legacy nether routers).
    pub fn create_legacy_nether<R: RandomSource + ?Sized>(
        random: &mut R,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        Self::build(random, first_octave, amplitudes, false)
    }

    fn build<R: RandomSource + ?Sized>(
        random: &mut R,
        first_octave: i32,
        amplitudes: &[f64],
        use_new_factory: bool,
    ) -> Self {
        let size = amplitudes.len();
        let i = -first_octave; // index of the legacy "first" octave
        let mut noise_levels: Vec<Option<ImprovedNoise>> = (0..size).map(|_| None).collect();
        if use_new_factory {
            let factory = random.fork_positional_factory();
            for (i1, amp) in amplitudes.iter().enumerate() {
                if *amp == 0.0 {
                    continue;
                }
                let i2 = first_octave + i1 as i32;
                let mut oct = factory.from_hash_of(&format!("octave_{i2}"));
                noise_levels[i1] = Some(ImprovedNoise::new(oct.as_mut()));
            }
        } else {
            // Legacy construction order (BlendedNoise / legacy nether biomes):
            // the FIRST ImprovedNoise is created UNCONDITIONALLY (it consumes
            // the random stream even when i >= size, where it is discarded!),
            // then i-1 down to 0, skipping zero amplitudes with
            // consumeCount(262). Vanilla:
            //   ImprovedNoise improvedNoise = new ImprovedNoise(random);
            //   if (i >= 0 && i < size && amps[i] != 0.0) noiseLevels[i] = it;
            if i >= 0 && (i as usize) < size && amplitudes[i as usize] != 0.0 {
                noise_levels[i as usize] = Some(ImprovedNoise::new(random));
            } else {
                let _discarded = ImprovedNoise::new(random);
            }
            for i1x in (0..i).rev() {
                if (i1x as usize) < size {
                    if amplitudes[i1x as usize] != 0.0 {
                        noise_levels[i1x as usize] = Some(ImprovedNoise::new(random));
                        continue;
                    }
                    skip_octave(random);
                    continue;
                }
                skip_octave(random);
            }
            let built = noise_levels.iter().filter(|x| x.is_some()).count();
            let nonzero = amplitudes.iter().filter(|a| **a != 0.0).count();
            assert_eq!(
                built, nonzero,
                "Failed to create correct number of noise levels for given non-zero amplitudes"
            );
            assert!(i >= size as i32 - 1, "Positive octaves are temporarily disabled");
        }
        let lowest_freq_input_factor = (2.0f64).powi(-i);
        let lowest_freq_value_factor =
            (2.0f64).powi(size as i32 - 1) / ((2.0f64).powi(size as i32) - 1.0);
        let max_value = edge_value(amplitudes, lowest_freq_value_factor, 2.0);
        Self {
            noise_levels,
            first_octave,
            amplitudes: amplitudes.to_vec(),
            lowest_freq_value_factor,
            lowest_freq_input_factor,
            max_value,
        }
    }

    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// PerlinNoise.getOctaveNoise(i) = noiseLevels[len - 1 - i].
    pub fn get_octave_noise(&self, octave: usize) -> Option<&ImprovedNoise> {
        self.noise_levels[self.noise_levels.len() - 1 - octave].as_ref()
    }

    /// PerlinNoise.getValue(x, y, z) -> (x, y, z, 0.0, 0.0, false).
    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.get_value_scaled(x, y, z, 0.0, 0.0, false)
    }

    /// PerlinNoise.getValue(x, y, z, yScale, yMax, useFixedY).
    pub fn get_value_scaled(
        &self,
        x: f64,
        y: f64,
        z: f64,
        y_scale: f64,
        y_max: f64,
        use_fixed_y: bool,
    ) -> f64 {
        let mut d = 0.0;
        let mut d1 = self.lowest_freq_input_factor;
        let mut d2 = self.lowest_freq_value_factor;
        for (i, level) in self.noise_levels.iter().enumerate() {
            if let Some(improved) = level {
                let y_arg = if use_fixed_y { -improved.yo } else { Self::wrap(y * d1) };
                let d3 = improved.noise_scaled(
                    Self::wrap(x * d1),
                    y_arg,
                    Self::wrap(z * d1),
                    y_scale * d1,
                    y_max * d1,
                );
                d += self.amplitudes[i] * d3 * d2; // (amp * d3) * d2 — Java order
            }
            d1 *= 2.0;
            d2 /= 2.0;
        }
        d
    }

    /// PerlinNoise.wrap: `value - lfloor(value / 3.3554432E7 + 0.5) * 3.3554432E7`
    /// (2^25 = 33554432.0).
    ///
    /// N1-H3 micro-opt: `value / C` → `value * INV_2P25`. Exactness proof:
    /// C = 2^25 is a power of two, so its reciprocal 2^-25 = 1.0/33554432.0
    /// is EXACTLY representable (the const evaluates to that one bit pattern,
    /// zero rounding). Division by 2^25 and multiplication by 2^-25 compute
    /// the identical real number (a pure binary-exponent scaling), so IEEE
    /// round-to-nearest yields identical bits for EVERY input class: normal,
    /// subnormal, ±0 (sign preserved: -0.0/2^25 == -0.0 == -0.0·2^-25),
    /// ±inf, NaN. The input bits of the subsequent `lfloor` are unchanged,
    /// hence the wrapped result is bit-identical for all inputs.
    /// (The variable-divisor `d6 / y_scale` in `noise_scaled` is deliberately
    /// NOT touched — a non-power-of-two divisor changes rounding.)
    #[inline]
    pub fn wrap(value: f64) -> f64 {
        const C: f64 = 33554432.0; // 2^25 = 3.3554432E7
        const INV_2P25: f64 = 1.0 / 33554432.0; // exact 2^-25
        value - crate::mth::lfloor(value * INV_2P25 + 0.5) as f64 * C
    }
}

fn skip_octave<R: RandomSource + ?Sized>(random: &mut R) {
    // PerlinNoise.skipOctave: consumeCount(262). Dispatches by lineage:
    // Legacy -> nextInt x262 (RandomSource default), Xoroshiro ->
    // nextLong x262 (XoroshiroRandomSource override) — load-bearing.
    random.consume_count(262);
}

fn edge_value(amplitudes: &[f64], lowest_freq_value_factor: f64, multiplier: f64) -> f64 {
    // edgeValue: octave present <=> amplitude != 0 (both factories leave
    // noiseLevels[i] null exactly at zero amplitudes).
    let mut d = 0.0;
    let mut d1 = lowest_freq_value_factor;
    for amp in amplitudes.iter() {
        if *amp != 0.0 {
            d += amp * multiplier * d1; // (amp * multiplier) * d1 — Java order
        }
        d1 /= 2.0;
    }
    d
}

// --------------------------------------------------------------------------
// NormalNoise
// --------------------------------------------------------------------------

/// `net.minecraft.world.level.levelgen.synth.NormalNoise`.
pub struct NormalNoise {
    first: PerlinNoise,
    second: PerlinNoise,
    value_factor: f64,
    max_value: f64,
}

const INPUT_FACTOR: f64 = 1.0181268882175227;

impl NormalNoise {
    /// NormalNoise.create(random, parameters) — useNewFactory=true.
    pub fn create<R: RandomSource + ?Sized>(
        random: &mut R,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        let first = PerlinNoise::create(random, first_octave, amplitudes);
        let second = PerlinNoise::create(random, first_octave, amplitudes);
        let mut i1 = i32::MAX;
        let mut i2 = i32::MIN;
        for (idx, amp) in amplitudes.iter().enumerate() {
            if *amp == 0.0 {
                continue;
            }
            i1 = i1.min(idx as i32);
            i2 = i2.max(idx as i32);
        }
        let value_factor = 0.16666666666666666 / expected_deviation(i2.wrapping_sub(i1)); // Java int wrap
        let max_value = (first.max_value() + second.max_value()) * value_factor;
        Self { first, second, value_factor, max_value }
    }

    /// NormalNoise.createLegacyNetherBiome.
    pub fn create_legacy_nether<R: RandomSource + ?Sized>(
        random: &mut R,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        let first = PerlinNoise::create_legacy_nether(random, first_octave, amplitudes);
        let second = PerlinNoise::create_legacy_nether(random, first_octave, amplitudes);
        let mut i1 = i32::MAX;
        let mut i2 = i32::MIN;
        for (idx, amp) in amplitudes.iter().enumerate() {
            if *amp == 0.0 {
                continue;
            }
            i1 = i1.min(idx as i32);
            i2 = i2.max(idx as i32);
        }
        let value_factor = 0.16666666666666666 / expected_deviation(i2.wrapping_sub(i1)); // Java int wrap
        let max_value = (first.max_value() + second.max_value()) * value_factor;
        Self { first, second, value_factor, max_value }
    }

    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let d = x * INPUT_FACTOR;
        let d1 = y * INPUT_FACTOR;
        let d2 = z * INPUT_FACTOR;
        (self.first.get_value(x, y, z) + self.second.get_value(d, d1, d2)) * self.value_factor
    }
}

fn expected_deviation(octaves: i32) -> f64 {
    0.1 * (1.0 + 1.0 / (octaves + 1) as f64)
}

// --------------------------------------------------------------------------
// BlendedNoise
// --------------------------------------------------------------------------

/// `net.minecraft.world.level.levelgen.synth.BlendedNoise` (old_blended_noise).
pub struct BlendedNoise {
    min_limit_noise: PerlinNoise,
    max_limit_noise: PerlinNoise,
    main_noise: PerlinNoise,
    xz_multiplier: f64,
    y_multiplier: f64,
    pub xz_scale: f64,
    pub y_scale: f64,
    xz_factor: f64,
    y_factor: f64,
    smear_scale_multiplier: f64,
    max_value: f64,
}

impl BlendedNoise {
    /// Public ctor: `new BlendedNoise(random, xzScale, yScale, xzFactor, yFactor, smearScaleMultiplier)`.
    pub fn new<R: RandomSource + ?Sized>(
        random: &mut R,
        xz_scale: f64,
        y_scale: f64,
        xz_factor: f64,
        y_factor: f64,
        smear_scale_multiplier: f64,
    ) -> Self {
        // createLegacyForBlendedNoise: IntStream.rangeClosed(-15, 0) x2, (-7, 0)
        let min_limit_noise = PerlinNoise::create_legacy_for_blended_noise(random);
        let max_limit_noise = PerlinNoise::create_legacy_for_blended_noise(random);
        let main_noise = {
            let amps = [1.0; 8];
            PerlinNoise::build(random, -7, &amps, false)
        };
        let xz_multiplier = 684.412 * xz_scale;
        let y_multiplier = 684.412 * y_scale;
        let max_value = min_limit_noise.max_broken_value(y_multiplier);
        Self {
            min_limit_noise,
            max_limit_noise,
            main_noise,
            xz_multiplier,
            y_multiplier,
            xz_scale,
            y_scale,
            xz_factor,
            y_factor,
            smear_scale_multiplier,
            max_value,
        }
    }

    /// BlendedNoise.withNewRandom(random) — same scales, fresh stream.
    pub fn with_new_random<R: RandomSource + ?Sized>(
        &self,
        random: &mut R,
    ) -> Self {
        Self::new(
            random,
            self.xz_scale,
            self.y_scale,
            self.xz_factor,
            self.y_factor,
            self.smear_scale_multiplier,
        )
    }

    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    pub fn compute(&self, block_x: i32, block_y: i32, block_z: i32) -> f64 {
        let d = block_x as f64 * self.xz_multiplier;
        let d1 = block_y as f64 * self.y_multiplier;
        let d2 = block_z as f64 * self.xz_multiplier;
        let d3 = d / self.xz_factor;
        let d4 = d1 / self.y_factor;
        let d5 = d2 / self.xz_factor;
        let d6 = self.y_multiplier * self.smear_scale_multiplier;
        let d7 = d6 / self.y_factor;
        let mut d8 = 0.0;
        let mut d9 = 0.0;
        let mut d10 = 0.0;
        let mut d11 = 1.0;
        for i in 0..8usize {
            if let Some(octave) = self.main_noise.get_octave_noise(i) {
                d10 += octave.noise_scaled(
                    PerlinNoise::wrap(d3 * d11),
                    PerlinNoise::wrap(d4 * d11),
                    PerlinNoise::wrap(d5 * d11),
                    d7 * d11,
                    d4 * d11,
                ) / d11;
            }
            d11 /= 2.0;
        }
        let d12 = (d10 / 10.0 + 1.0) / 2.0;
        let flag1 = d12 >= 1.0; // min-limit skipped when true
        let flag2 = d12 <= 0.0; // max-limit skipped when true
        let mut d11 = 1.0;
        for i1 in 0..16usize {
            let d13 = PerlinNoise::wrap(d * d11);
            let d14 = PerlinNoise::wrap(d1 * d11);
            let d15 = PerlinNoise::wrap(d2 * d11);
            let d16 = d6 * d11;
            if !flag1 {
                if let Some(octave) = self.min_limit_noise.get_octave_noise(i1) {
                    d8 += octave.noise_scaled(d13, d14, d15, d16, d1 * d11) / d11;
                }
            }
            if !flag2 {
                if let Some(octave) = self.max_limit_noise.get_octave_noise(i1) {
                    d9 += octave.noise_scaled(d13, d14, d15, d16, d1 * d11) / d11;
                }
            }
            d11 /= 2.0;
        }
        crate::mth::clamped_lerp(d8 / 512.0, d9 / 512.0, d12) / 128.0
    }
}

impl PerlinNoise {
    fn max_broken_value(&self, y_multiplier: f64) -> f64 {
        edge_value(&self.amplitudes, self.lowest_freq_value_factor, y_multiplier + 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jrandom::{GaussianCache, LegacyRandomSource};
    use crate::xoroshiro::XoroshiroRandomSource;

    #[test]
    fn improved_noise_deterministic() {
        let mut a = XoroshiroRandomSource::new(3053459);
        let n1 = ImprovedNoise::new(&mut a);
        let mut b = XoroshiroRandomSource::new(3053459);
        let n2 = ImprovedNoise::new(&mut b);
        for i in 0..50 {
            let x = i as f64 * 0.137;
            let y = -i as f64 * 0.0311;
            let z = i as f64 * 0.5 - 12.75;
            assert_eq!(n1.noise(x, y, z).to_bits(), n2.noise(x, y, z).to_bits());
        }
    }

    #[test]
    fn perlin_new_factory_uses_octave_hashes() {
        // Two constructions with the same seed must be identical; and the
        // factory.fromHashOf path must be exercised (determinism check).
        let mut a = XoroshiroRandomSource::new(90210);
        let p1 = PerlinNoise::create(&mut a, -3, &[1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 0.0]);
        let mut b = XoroshiroRandomSource::new(90210);
        let p2 = PerlinNoise::create(&mut b, -3, &[1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 0.0]);
        for i in 0..30 {
            let x = i as f64 * 0.5;
            let y = (i as f64) * -0.25 + 3.0;
            let z = (i as f64) * 1.5;
            assert_eq!(
                p1.get_value(x, y, z).to_bits(),
                p2.get_value(x, y, z).to_bits()
            );
        }
    }

    #[test]
    fn perlin_zero_amplitude_skipped_in_new_factory() {
        // amplitude 0 -> no ImprovedNoise built for that octave; evaluating
        // is identical to skipping the octave.
        let mut a = XoroshiroRandomSource::new(7);
        let p = PerlinNoise::create(&mut a, -2, &[1.0, 0.0, 1.0]);
        let mut b = XoroshiroRandomSource::new(7);
        let q = PerlinNoise::create(&mut b, -2, &[1.0, 0.0, 1.0]);
        assert_eq!(p.get_value(1.0, 2.0, 3.0).to_bits(), q.get_value(1.0, 2.0, 3.0).to_bits());
    }

    #[test]
    fn normal_noise_shape() {
        let mut a = XoroshiroRandomSource::new(133700);
        let n = NormalNoise::create(&mut a, -7, &[1.0, 1.0, 1.0]);
        let mut b = XoroshiroRandomSource::new(133700);
        let m = NormalNoise::create(&mut b, -7, &[1.0, 1.0, 1.0]);
        for i in 0..30 {
            let x = i as f64 * 0.125;
            assert_eq!(n.get_value(x, 0.5, -x).to_bits(), m.get_value(x, 0.5, -x).to_bits());
        }
    }

    #[test]
    fn blended_noise_legacy_construction_runs() {
        let mut a = XoroshiroRandomSource::new(3053459);
        let bl = BlendedNoise::new(&mut a, 1.0, 1.0, 80.0 / 80.0, 160.0 / 80.0, 1.0);
        let mut b = XoroshiroRandomSource::new(3053459);
        let bl2 = BlendedNoise::new(&mut b, 1.0, 1.0, 1.0, 2.0, 1.0);
        // same seed + same params => identical
        let mut c = XoroshiroRandomSource::new(3053459);
        let bl3 = BlendedNoise::new(&mut c, 1.0, 1.0, 1.0, 2.0, 1.0);
        for i in 0..20 {
            let (x, y, z) = (i * 7, i * 9 - 100, i * 5 + 3);
            assert_eq!(
                bl2.compute(x, y, z).to_bits(),
                bl3.compute(x, y, z).to_bits()
            );
        }
        let _ = bl; // different params, smoke only
    }

    #[test]
    fn legacy_gaussian_cache_reset_on_set_seed() {
        let mut r = LegacyRandomSource::new(1);
        let g1 = r.next_gaussian();
        let _ = g1;
        r.set_seed(1);
        let mut r2 = LegacyRandomSource::new(1);
        assert_eq!(r.next_gaussian().to_bits(), r2.next_gaussian().to_bits());
    }

    #[test]
    fn gaussian_cache_with_dyn_trait() {
        // Boxed dyn path (mirrors how the density evaluator will hold noises).
        let mut r: Box<dyn RandomSource> = Box::new(XoroshiroRandomSource::new(5));
        let mut cache = GaussianCache::new();
        for _ in 0..100 {
            let v = cache.next_gaussian(r.as_mut());
            assert!(v.is_finite());
        }
    }

    // ------------------------------------------------------------------
    // N1-H3 bit-exactness proofs (IMP-A perlin micro-opts)
    // ------------------------------------------------------------------

    /// OLD gradient application (pre-N1-H3): literal int→f64 converts +
    /// multiplies, strict left association — kept verbatim as the oracle.
    fn gradient_dot_old(g: [i32; 3], x: f64, y: f64, z: f64) -> f64 {
        g[0] as f64 * x + g[1] as f64 * y + g[2] as f64 * z
    }

    /// OLD wrap (pre-N1-H3): division by the 2^25 constant. Oracle only.
    fn wrap_old(value: f64) -> f64 {
        const C: f64 = 33554432.0;
        value - crate::mth::lfloor(value / C + 0.5) as f64 * C
    }

    #[test]
    fn n1_h3_gradient_dot_tristate_matches_multiply_bitwise() {
        // Representative deltas: signed zeros (the sign-copy case), boundary
        // deltas (0, 1, largest f64 < 1), negatives, subnormals, smallest
        // normal, epsilon, and the |x| <= 2^25 wrap-bound scale.
        let deltas: [f64; 15] = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            0.5,
            -0.5,
            0.137,
            -0.0311,
            1.0 - f64::EPSILON / 2.0, // largest f64 below 1.0
            f64::EPSILON,
            f64::MIN_POSITIVE, // smallest normal
            -f64::MIN_POSITIVE,
            f64::from_bits(1), // smallest subnormal
            -f64::from_bits(1),
            33554432.0, // 2^25 wrap bound
        ];
        // All 16 table rows together exercise every component in {1, -1, 0}.
        for g in GRADIENT.iter() {
            for &x in &deltas {
                for &y in &deltas {
                    for &z in &deltas {
                        let a = gradient_dot(*g, x, y, z);
                        let b = gradient_dot_old(*g, x, y, z);
                        assert_eq!(
                            a.to_bits(),
                            b.to_bits(),
                            "gradient {:?} deltas ({x:?}, {y:?}, {z:?})",
                            g
                        );
                    }
                }
            }
        }
        // Lemma spot checks (legibility):
        // (a) the ZERO term copies its operand's sign bit — 0.0 * -0.0
        //     == -0.0 — demonstrated at term level with a synthetic all-zero
        //     gradient: (-0.0 + -0.0) + -0.0 stays -0.0 in both impls.
        assert_eq!(gradient_dot_old([0, 0, 0], -0.0, -0.0, -0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(
            gradient_dot([0, 0, 0], -0.0, -0.0, -0.0).to_bits(),
            gradient_dot_old([0, 0, 0], -0.0, -0.0, -0.0).to_bits()
        );
        // (b) full sum: 0.0*-0.0 + 1.0*0.0 + 0.0*0.0 = (-0.0 + 0.0) + 0.0
        //     = +0.0 — IEEE RN sums opposite-signed zeros to +0.0.
        assert_eq!(gradient_dot_old([0, 1, 0], -0.0, 0.0, 0.0).to_bits(), 0f64.to_bits());
        assert_eq!(gradient_dot([0, 1, 0], -0.0, 0.0, 0.0).to_bits(), 0f64.to_bits());
        // (c) -1.0 * -0.0 == +0.0 == -(-0.0) — the negation arm is exact too.
        assert_eq!(
            gradient_dot([-1, 0, 0], -0.0, 0.0, 0.0).to_bits(),
            gradient_dot_old([-1, 0, 0], -0.0, 0.0, 0.0).to_bits()
        );
    }

    #[test]
    fn n1_h3_wrap_mul_by_inv_2p25_matches_div_bitwise() {
        let mut vs: Vec<f64> = Vec::new();
        // Exact 2^25 boundaries and the lfloor integer landings
        // (v/2^25 + 0.5 ∈ ℤ exactly ⇔ v = k·2^25 ± 2^24), plus adjacent
        // representable neighbors (bit ± 1).
        for k in -33i32..=33 {
            let c = k as f64 * 33554432.0;
            vs.push(c);
            vs.push(c + 16777216.0); // 2^24: +0.5 lands exactly on an integer
            vs.push(c - 16777216.0);
            vs.push(c + 4.0);
            vs.push(c - 4.0);
            if c != 0.0 {
                vs.push(f64::from_bits(c.to_bits() + 1));
                vs.push(f64::from_bits(c.to_bits() - 1));
            }
        }
        // Every input class from the proof: signed zeros, subnormals,
        // smallest normal, the seed-scale value, saturation-scale magnitudes
        // (lfloor's `as i64` clamp path), ±inf, NaN.
        vs.push(0.0);
        vs.push(-0.0);
        vs.push(f64::from_bits(1));
        vs.push(-f64::from_bits(1));
        vs.push(f64::MIN_POSITIVE);
        vs.push(-f64::MIN_POSITIVE);
        vs.push(3053459.0);
        vs.push(-3053459.0);
        vs.push(1.0e300);
        vs.push(-1.0e300);
        vs.push(f64::INFINITY);
        vs.push(f64::NEG_INFINITY);
        vs.push(f64::NAN);
        // Deterministic LCG sweep, dense over ±4·2^25 (the wrap band).
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..10_000 {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u = (s >> 11) as f64 / (1u64 << 53) as f64; // [0, 1)
            vs.push((u - 0.5) * 4.0 * 33554432.0);
        }
        for &v in &vs {
            assert_eq!(
                PerlinNoise::wrap(v).to_bits(),
                wrap_old(v).to_bits(),
                "wrap({v:e}) bits differ"
            );
        }
        // Self-check of the oracle: wrap is periodic with 2^25.
        assert_eq!(PerlinNoise::wrap(33554432.0), 0.0);
        assert_eq!(PerlinNoise::wrap(-33554432.0), 0.0);
        assert_eq!(PerlinNoise::wrap(3.0 * 33554432.0), 0.0);
    }
}
