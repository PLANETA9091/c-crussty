//! NCF P2.5 — SimplexNoise + PerlinSimplexNoise, decompiled faithfully from
//! Paper 1.21.10 (net.minecraft.world.level.levelgen.synth.SimplexNoise /
//! PerlinSimplexNoise). Needed by the surface rules' `temperature` condition:
//! Biome.getHeightAdjustedTemperature queries TEMPERATURE_NOISE
//! (PerlinSimplexNoise(LegacyRandomSource(1234L), [0])) when blockY >
//! seaLevel + 17; FROZEN_TEMPERATURE_NOISE ([-2,-1,0], seed 3456L) backs the
//! FrozenOcean temperatureModifier.
//!
//! Bit-exactness rules (I2): no FMA, wrapping int ops, Mth.floor semantics.

use crate::jrandom::RandomSource;
use crate::mth;

const GRADIENT: [[i32; 3]; 16] = [
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

/// sqrt(3) — Java computes Math.sqrt(3.0) at class-init; the correctly
/// rounded f64 is the literal below (identical bits).
const SQRT_3: f64 = 1.7320508075688772;
/// 0.5 * (sqrt(3) - 1)
const F2: f64 = 0.5 * (SQRT_3 - 1.0);
/// (3.0 - sqrt(3)) / 6.0
const G2: f64 = (3.0 - SQRT_3) / 6.0;

pub struct SimplexNoise {
    p: [i32; 512],
    pub xo: f64,
    pub yo: f64,
    pub zo: f64,
}

impl SimplexNoise {
    pub fn new(random: &mut dyn RandomSource) -> Self {
        let xo = random.next_f64() * 256.0;
        let yo = random.next_f64() * 256.0;
        let zo = random.next_f64() * 256.0;
        let mut p = [0i32; 512];
        let mut i = 0usize;
        while i < 256 {
            p[i] = i as i32;
            i += 1;
        }
        for ix in 0..256usize {
            let random_int = random.next_int_bound(256 - ix as i32) as usize;
            let i1 = p[ix];
            p[ix] = p[random_int + ix];
            p[random_int + ix] = i1;
        }
        SimplexNoise { p, xo, yo, zo }
    }

    #[inline]
    fn p_at(&self, index: i32) -> i32 {
        self.p[(index & 0xFF) as usize]
    }

    #[inline]
    fn dot(gradient: &[i32; 3], x: f64, y: f64, z: f64) -> f64 {
        gradient[0] as f64 * x + gradient[1] as f64 * y + gradient[2] as f64 * z
    }

    #[inline]
    fn corner_noise_3d(gradient_index: usize, x: f64, y: f64, z: f64, offset: f64) -> f64 {
        let mut d = offset - x * x - y * y - z * z;
        if d < 0.0 {
            0.0
        } else {
            d *= d;
            d * d * Self::dot(&GRADIENT[gradient_index], x, y, z)
        }
    }

    /// 2D simplex (SimplexNoise.getValue(x, y)).
    pub fn get_value_2d(&self, x: f64, y: f64) -> f64 {
        let d = (x + y) * F2;
        let floor = mth::floor(x + d);
        let floor1 = mth::floor(y + d);
        // Java (CFR): d1 = (double)(floor + floor1) * G2; d2 = (double)floor - d1;
        let d1 = (floor as f64 + floor1 as f64) * G2;
        let d2 = floor as f64 - d1;
        let d3 = floor1 as f64 - d1;
        let d4 = x - d2;
        let d5 = y - d3;
        let (i, i1): (i32, i32) = if d4 > d5 { (1, 0) } else { (0, 1) };
        let d6 = d4 - i as f64 + G2;
        let d7 = d5 - i1 as f64 + G2;
        let d8 = d4 - 1.0 + 2.0 * G2;
        let d9 = d5 - 1.0 + 2.0 * G2;
        let i2 = floor & 0xFF;
        let i3 = floor1 & 0xFF;
        let i4 = self.p_at(i2 + self.p_at(i3)) % 12;
        let i5 = self.p_at(i2 + i + self.p_at(i3 + i1)) % 12;
        let i6 = self.p_at(i2 + 1 + self.p_at(i3 + 1)) % 12;
        let c0 = Self::corner_noise_3d(i4 as usize, d4, d5, 0.0, 0.5);
        let c1 = Self::corner_noise_3d(i5 as usize, d6, d7, 0.0, 0.5);
        let c2 = Self::corner_noise_3d(i6 as usize, d8, d9, 0.0, 0.5);
        70.0 * (c0 + c1 + c2)
    }

    /// 3D simplex (SimplexNoise.getValue(x, y, z)) — decompile-verbatim tail:
    /// four corners with grad indices i9/i10/i11/i12, offset 0.6, sum * 32.0.
    pub fn get_value_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let d1 = (x + y + z) * 0.3333333333333333;
        let floor = mth::floor(x + d1);
        let floor1 = mth::floor(y + d1);
        let floor2 = mth::floor(z + d1);
        let d3 = (floor as f64 + floor1 as f64 + floor2 as f64) * 0.16666666666666666;
        let d4 = floor as f64 - d3;
        let d5 = floor1 as f64 - d3;
        let d6 = floor2 as f64 - d3;
        let d7 = x - d4;
        let d8 = y - d5;
        let d9 = z - d6;
        let (i, i1, i2, i3, i4, i5): (i32, i32, i32, i32, i32, i32) = if d7 >= d8 {
            if d8 >= d9 {
                (1, 0, 0, 1, 1, 0)
            } else if d7 >= d9 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if d8 < d9 {
            (0, 0, 1, 0, 1, 1)
        } else if d7 < d9 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };
        let d10 = d7 - i as f64 + 0.16666666666666666;
        let d11 = d8 - i1 as f64 + 0.16666666666666666;
        let d12 = d9 - i2 as f64 + 0.16666666666666666;
        let d13 = d7 - i3 as f64 + 0.3333333333333333;
        let d14 = d8 - i4 as f64 + 0.3333333333333333;
        let d15 = d9 - i5 as f64 + 0.3333333333333333;
        let d16 = d7 - 1.0 + 0.5;
        let d17 = d8 - 1.0 + 0.5;
        let d18 = d9 - 1.0 + 0.5;
        let i6 = floor & 0xFF;
        let i7 = floor1 & 0xFF;
        let i8 = floor2 & 0xFF;
        let i9 = self.p_at(i6 + self.p_at(i7 + self.p_at(i8))) % 12;
        let i10 = self.p_at(i6 + i + self.p_at(i7 + i1 + self.p_at(i8 + i2))) % 12;
        let i11 = self.p_at(i6 + i3 + self.p_at(i7 + i4 + self.p_at(i8 + i5))) % 12;
        let i12 = self.p_at(i6 + 1 + self.p_at(i7 + 1 + self.p_at(i8 + 1))) % 12;
        let c0 = Self::corner_noise_3d(i9 as usize, d7, d8, d9, 0.6);
        let c1 = Self::corner_noise_3d(i10 as usize, d10, d11, d12, 0.6);
        let c2 = Self::corner_noise_3d(i11 as usize, d13, d14, d15, 0.6);
        let c3 = Self::corner_noise_3d(i12 as usize, d16, d17, d18, 0.6);
        32.0 * (c0 + c1 + c2 + c3)
    }
}

/// PerlinSimplexNoise — decompile-verbatim. `octaves` = the Java
/// List<Integer> of EXPECTED octave indices (NOT amplitudes): TEMPERATURE_NOISE
/// passes [0], FROZEN_TEMPERATURE_NOISE passes [-2,-1,0].
pub struct PerlinSimplexNoise {
    noise_levels: Vec<Option<SimplexNoise>>,
    highest_freq_input_factor: f64,
    highest_freq_value_factor: f64,
}

impl PerlinSimplexNoise {
    pub fn new(random: &mut dyn RandomSource, octaves: &[i32]) -> Self {
        let first = *octaves.iter().min().unwrap();
        let last = *octaves.iter().max().unwrap();
        let i = -first; // -octaves.firstInt()
        let i2 = i + last + 1; // total level slots
        assert!(i2 >= 1, "Total number of octaves needs to be >= 1");
        let base = SimplexNoise::new(random);
        let mut noise_levels: Vec<Option<SimplexNoise>> = (0..i2).map(|_| None).collect();
        let i3 = last;
        // `if (i1 >= 0 && i1 < i2 && octaves.contains(0)) noiseLevels[i1] = simplexNoise;`
        // — stores THE SAME instance; when the condition fails the constructed
        // instance's draws are still consumed but the value source is dropped.
        if last >= 0 && (last as usize) < i2 as usize && octaves.contains(&0) {
            noise_levels[last as usize] = Some(SimplexNoise {
                p: base.p,
                xo: base.xo,
                yo: base.yo,
                zo: base.zo,
            });
        }
        for i4 in (last + 1)..i2 {
            if i4 >= 0 && octaves.contains(&(i3 - i4)) {
                noise_levels[i4 as usize] = Some(SimplexNoise::new(random));
            } else {
                random.consume_count(262);
            }
        }
        if last > 0 {
            // long l = (long)(simplexNoise.getValue(xo, yo, zo) * 9.223372036854776E18)
            // — simplexNoise is the ORIGINAL instance (2D value at its offsets)
            let l = (base.get_value_2d(base.xo, base.yo) * 9.223372036854776E18) as i64;
            let mut wr = crate::jrandom::LegacyRandomSource::new(l);
            for i5 in (0..i3).rev() {
                if (i5 as usize) < i2 as usize && octaves.contains(&(i3 - i5)) {
                    noise_levels[i5 as usize] = Some(SimplexNoise::new(&mut wr));
                } else {
                    wr.consume_count(262);
                }
            }
        }
        let highest_freq_input_factor = 2.0f64.powi(last);
        let highest_freq_value_factor = 1.0 / (2.0f64.powi(i2) - 1.0);
        PerlinSimplexNoise {
            noise_levels,
            highest_freq_input_factor,
            highest_freq_value_factor,
        }
    }

    /// getValue(x, y, useNoiseOffsets)
    pub fn get_value(&self, x: f64, y: f64, use_noise_offsets: bool) -> f64 {
        let mut d = 0.0f64;
        let mut d1 = self.highest_freq_input_factor;
        let mut d2 = self.highest_freq_value_factor;
        for level in &self.noise_levels {
            if let Some(sn) = level {
                d += sn.get_value_2d(
                    x * d1 + if use_noise_offsets { sn.xo } else { 0.0 },
                    y * d1 + if use_noise_offsets { sn.yo } else { 0.0 },
                ) * d2;
            }
            d1 /= 2.0;
            d2 *= 2.0;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jrandom::LegacyRandomSource;

    #[test]
    fn simplex_deterministic_and_bounded() {
        let mut r = LegacyRandomSource::new(1234);
        let s = SimplexNoise::new(&mut r);
        let a = s.get_value_2d(17.25, -3.5);
        let b = s.get_value_2d(17.25, -3.5);
        assert_eq!(a.to_bits(), b.to_bits());
        assert!(a > -100.0 && a < 100.0);
        let c = s.get_value_3d(1.5, 2.5, -3.5);
        assert_eq!(c.to_bits(), s.get_value_3d(1.5, 2.5, -3.5).to_bits());
    }

    #[test]
    fn temperature_noise_single_octave() {
        // TEMPERATURE_NOISE = PerlinSimplexNoise(WorldgenRandom(Legacy(1234)), [0]).
        // WorldgenRandom wraps a Legacy and delegates next(int) to it for
        // Legacy inner sources; SimplexNoise uses nextDouble/nextIntBound, so
        // driving the Legacy directly is draw-identical here.
        let mut lr = LegacyRandomSource::new(1234);
        let pn = PerlinSimplexNoise::new(&mut lr, &[0]);
        let v = pn.get_value(10.0 / 8.0, -4.0 / 8.0, false);
        assert_eq!(v.to_bits(), pn.get_value(10.0 / 8.0, -4.0 / 8.0, false).to_bits());
        // [-2,-1,0] path exercises the negative-octave chain too
        let mut lr2 = LegacyRandomSource::new(3456);
        let pn2 = PerlinSimplexNoise::new(&mut lr2, &[-2, -1, 0]);
        let w = pn2.get_value(3.25, -1.5, true);
        assert!(w.is_finite());
    }
}
