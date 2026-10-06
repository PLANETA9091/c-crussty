//! Java/Minecraft math helpers, ported BIT-EXACTLY from net.minecraft.util.Mth
//! (CFR decompile of the mojang-mapped Purpur 1.21.10 jar, verified 2026-10-05)
//! plus java.lang.Math semantics the kernels rely on.
//!
//! Casting law (JLS 5.1.3, matches Rust `as`):
//!   double -> int/long in Java SATURATES and maps NaN to 0; Rust `as` has the
//!   identical contract (saturating cast, NaN -> 0). Integer arithmetic that
//!   can overflow must use wrapping_* (Java int/long wrap).

/// net.minecraft.util.Mth#floor(double):
/// ```java
/// int i = (int)value; return value < (double)i ? i - 1 : i;
/// ```
#[inline]
pub fn floor(value: f64) -> i32 {
    let i = value as i32; // Java (int) cast: saturating, NaN -> 0
    if value < i as f64 {
        // Java int arithmetic WRAPS: Mth.floor(Integer.MIN_VALUE as double
        // territory) yields Integer.MAX_VALUE, not a panic.
        i.wrapping_sub(1)
    } else {
        i
    }
}

/// net.minecraft.util.Mth#lfloor(double)
#[inline]
pub fn lfloor(value: f64) -> i64 {
    let l = value as i64;
    if value < l as f64 {
        l.wrapping_sub(1)
    } else {
        l
    }
}

/// Mth#lerp(double delta, double start, double end):
/// `start + delta * (end - start)`
#[inline]
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

/// Mth#lerp2
#[inline]
pub fn lerp2(delta1: f64, delta2: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    lerp(delta2, lerp(delta1, start1, end1), lerp(delta1, start2, end2))
}

/// Mth#lerp3
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn lerp3(
    delta1: f64,
    delta2: f64,
    delta3: f64,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp(
        delta3,
        lerp2(delta1, delta2, start1, end1, start2, end2),
        lerp2(delta1, delta2, start3, end3, start4, end4),
    )
}

/// Mth#smoothstep: `x*x*x * (x * (x*6.0 - 15.0) + 10.0)`
#[inline]
pub fn smoothstep(input: f64) -> f64 {
    input * input * input * (input * (input * 6.0 - 15.0) + 10.0)
}

/// Mth#smoothstepDerivative: `30.0 * x * x * (x - 1.0) * (x - 1.0)`
#[inline]
pub fn smoothstep_derivative(input: f64) -> f64 {
    30.0 * input * input * (input - 1.0) * (input - 1.0)
}

/// java.lang.Math#min(double,double) — EXACT semantics (NaN propagates as
/// NaN, unlike Rust's IEEE minNum `f64::min` which returns the non-NaN
/// operand; -0.0 wins over +0.0):
/// ```java
/// if (a != a) return a;                       // NaN -> NaN
/// if ((a == 0.0d) && (b == 0.0d)
///     && (Double.doubleToRawLongBits(a) | Double.doubleToRawLongBits(b)) < 0)
///     return -0.0d;
/// return (a <= b) ? a : b;
/// ```
#[inline]
pub fn java_min(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        return a;
    }
    if a == 0.0 && b == 0.0 {
        // min(+0, -0) = -0.0 (either -0 wins; both +0 -> +0)
        return if a.to_bits() != 0 || b.to_bits() != 0 { -0.0 } else { a };
    }
    if a <= b {
        a
    } else {
        b
    }
}

/// java.lang.Math#max(double,double) — exact semantics (NaN propagates,
/// +0.0 wins over -0.0).
#[inline]
pub fn java_max(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        return a;
    }
    if a == 0.0 && b == 0.0 {
        // max(+0, -0) = +0.0 (either +0 wins; both -0 -> -0)
        return if a.to_bits() == 0 || b.to_bits() == 0 { 0.0 } else { a };
    }
    if a >= b {
        a
    } else {
        b
    }
}

/// Mth#clamp(double,double,double):
/// `value < min ? min : Math.min(value, max)`  (NaN propagates via Math.min)
#[inline]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else {
        java_min(value, max)
    }
}

/// Mth#clampedLerp: `delta < 0 ? start : (delta > 1 ? end : lerp(...))`
#[inline]
pub fn clamped_lerp(start: f64, end: f64, delta: f64) -> f64 {
    if delta < 0.0 {
        start
    } else if delta > 1.0 {
        end
    } else {
        lerp(delta, start, end)
    }
}

/// Mth#inverseLerp: `(delta - start) / (end - start)`
#[inline]
pub fn inverse_lerp(delta: f64, start: f64, end: f64) -> f64 {
    (delta - start) / (end - start)
}

/// Mth#clampedMap: `clampedLerp(outMin, outMax, inverseLerp(in, inMin, inMax))`
#[inline]
pub fn clamped_map(input: f64, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64 {
    clamped_lerp(output_min, output_max, inverse_lerp(input, input_min, input_max))
}

/// Mth#map: `lerp(inverseLerp(input, inputMin, inputMax), outputMin, outputMax)`
/// — the UNCLAMPED variant (5-b: vertical_gradient, stone_depth secondary).
#[inline]
pub fn map(input: f64, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64 {
    lerp(inverse_lerp(input, input_min, input_max), output_min, output_max)
}

/// Mth#ceillog2: `32 - Integer.numberOfLeadingZeros(value - 1)` for value >= 2,
/// 0 for value == 1 (5-b: heightmap bit count).
#[inline]
pub fn ceillog2(value: i32) -> u32 {
    if value <= 1 {
        return 0;
    }
    32 - ((value - 1) as u32).leading_zeros()
}

/// Mth#square(double)
#[inline]
pub fn square(v: f64) -> f64 {
    v * v
}

/// Mth#getSeed(int x, int y, int z) — the block-position seed used by
/// positional random factories:
/// ```java
/// long l = (long)(x * 3129871) ^ (long)z * 116129781L ^ (long)y;
/// l = l * l * 42317861L + l * 11L;
/// return l >> 16;
/// ```
#[inline]
pub fn get_seed(x: i32, y: i32, z: i32) -> i64 {
    // JAVA SUBTLETY: `x * 3129871` is an INT multiply (wraps at 32 bits!) and
    // only THEN widens to long; `z * 116129781L` widens first. The int-wrap
    // path is observable for |x| > ~687 (caught by the vector gate on the
    // block position 2147483000).
    let mut l = ((x.wrapping_mul(3129871)) as i64)
        ^ ((z as i64).wrapping_mul(116129781))
        ^ (y as i64);
    l = (l
        .wrapping_mul(l)
        .wrapping_mul(42317861))
    .wrapping_add(l.wrapping_mul(11));
    l >> 16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_matches_java() {
        assert_eq!(floor(3.7), 3);
        assert_eq!(floor(-3.7), -4);
        assert_eq!(floor(3.0), 3);
        assert_eq!(floor(-3.0), -3);
        assert_eq!(floor(-0.5), -1);
        // saturation edge: (int) clamps, then the i - 1 WRAPS (Java int math)
        assert_eq!(floor(3.0e9), i32::MAX);
        assert_eq!(floor(-3.0e9), i32::MAX); // MIN wraps to MAX!
        assert_eq!(floor(f64::NAN), 0); // NaN -> 0, then 0.0 < 0.0 is false -> 0
    }

    #[test]
    fn java_min_max_nan_and_zero_signs() {
        assert!(java_min(f64::NAN, 1.0).is_nan());
        assert!(java_min(1.0, f64::NAN).is_nan());
        assert!(java_max(f64::NAN, 1.0).is_nan());
        assert_eq!(java_min(-0.0, 0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(java_min(0.0, -0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(java_max(-0.0, 0.0).to_bits(), 0);
        assert_eq!(java_max(0.0, -0.0).to_bits(), 0);
        assert_eq!(java_min(3.0, 5.0), 3.0);
        assert_eq!(java_max(3.0, 5.0), 5.0);
    }

    #[test]
    fn lfloor_matches_java() {
        assert_eq!(lfloor(-3.7), -4);
        assert_eq!(lfloor(3.7), 3);
    }

    #[test]
    fn lerp_identities() {
        assert_eq!(lerp(0.5, 1.0, 3.0), 2.0);
        assert_eq!(lerp2(0.5, 0.5, 0.0, 2.0, 4.0, 6.0), 3.0);
        assert_eq!(smoothstep(0.5), 0.5);
        assert_eq!(smoothstep_derivative(0.5), 30.0 * 0.25 * (-0.5) * (-0.5));
    }

    #[test]
    fn clamp_nan_goes_through_min_side() {
        // value < min is false for NaN; Math.min(NaN, max) = NaN
        assert!(clamp(f64::NAN, -1.0, 1.0).is_nan());
        assert_eq!(clamp(0.5, -1.0, 1.0), 0.5);
        assert_eq!(clamp(-5.0, -1.0, 1.0), -1.0);
        assert_eq!(clamp(5.0, -1.0, 1.0), 1.0);
    }

    #[test]
    fn block_position_seed() {
        // Goldens for getSeed captured from the JVM (VectorCapture random rows
        // cross-check this via XoroshiroPositionalRandomFactory.at).
        assert_eq!(get_seed(0, 0, 0), 0);
        assert_eq!(get_seed(1, 64, -1), {
            let mut l = (1i64.wrapping_mul(3129871)) ^ ((-1i64).wrapping_mul(116129781)) ^ 64i64;
            l = l.wrapping_mul(l).wrapping_mul(42317861).wrapping_add(l.wrapping_mul(11));
            l >> 16
        });
    }
}

// ---------------------------------------------------------------------------
// Mth additions for P2.8 (carvers) — sin/cos are TABLE functions in Java
// (65536 entries, block-step precision); random_between family is f32.
// ---------------------------------------------------------------------------

use std::sync::OnceLock;

/// Mth.SIN[i] = (float)Math.sin((double)i * Math.PI * 2.0 / 65536.0)
fn sin_table() -> &'static [f32; 65536] {
    static TABLE: OnceLock<[f32; 65536]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [0.0f32; 65536];
        let mut i = 0usize;
        while i < 65536 {
            t[i] = ((i as f64) * std::f64::consts::PI * 2.0 / 65536.0).sin() as f32;
            i += 1;
        }
        t
    })
}

/// Mth.sin(float): SIN[(int)(value * 10430.378f) & 0xFFFF]
#[inline]
pub fn sin_f32(value: f32) -> f32 {
    let idx = (value * 10430.378f32) as i32 & 0xFFFF;
    sin_table()[idx as usize]
}

/// Mth.cos(float): SIN[(int)(value * 10430.378f + 16384.0f) & 0xFFFF]
#[inline]
pub fn cos_f32(value: f32) -> f32 {
    let idx = (value * 10430.378f32 + 16384.0f32) as i32 & 0xFFFF;
    sin_table()[idx as usize]
}

/// Mth.abs(float)
#[inline]
pub fn abs_f32(v: f32) -> f32 {
    f32::from_bits(f32::to_bits(v) & 0x7FFF_FFFF)
}

/// Mth.randomBetween(RandomSource, float low, float high):
/// low + random.nextFloat() * (high - low)
pub fn random_between<R: crate::jrandom::RandomSource + ?Sized>(random: &mut R, low: f32, high: f32) -> f32 {
    low + random.next_f32() * (high - low)
}

/// Mth.randomBetweenInclusive(RandomSource, int min, int max):
/// min + random.nextInt(max - min + 1)
pub fn random_between_inclusive<R: crate::jrandom::RandomSource + ?Sized>(random: &mut R, min: i32, max: i32) -> i32 {
    min + random.next_int_bound(max - min + 1)
}

/// TrapezoidFloat.sample: f = max - min; g = (f - plateau) / 2; h = f - g;
/// min + nextFloat() * h + nextFloat() * g
pub fn trapezoid_float<R: crate::jrandom::RandomSource + ?Sized>(random: &mut R, min: f32, max: f32, plateau: f32) -> f32 {
    let f = max - min;
    let g = (f - plateau) / 2.0f32;
    let h = f - g;
    min + random.next_f32() * h + random.next_f32() * g
}
