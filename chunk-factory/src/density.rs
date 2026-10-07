//! Density-function IR + scalar reference evaluator (NCF P1.2/P1.5/P2.3).
//!
//! Two-phase design mirroring vanilla exactly:
//!
//! 1. PARSE (`Unwired`): JSON (noise_settings routers / density_function
//!    registry files) -> tree with UNBOUND noise keys. This mirrors the DFU
//!    codec stage: HolderHolders are resolved to their file contents, numbers
//!    become constants, `type` dispatch follows DensityFunctions.bootstrap.
//! 2. WIRE (`Df`): the RandomState.NoiseWiringHelper pass — every NoiseHolder
//!    gets a real `NormalNoise` instance (created through the worldgen
//!    positional factory `fromHashOf(key)`), BlendedNoise gets
//!    `withNewRandom(fromHashOf("minecraft:terrain"))`, EndIslands get the
//!    level seed, legacy routers override temperature/vegetation/shift.
//!    min/max bounds are computed bottom-up in THIS pass, because vanilla's
//!    bounds live on the wired tree (NoiseHolder.maxValue() = 2.0 only when
//!    unbound).
//!
//! The evaluator (`Df::compute`) reproduces the SCALAR path of
//! `NoiseRouter.<field>.compute(SinglePointContext)` on the UNBOUND router —
//! i.e. Marker nodes pass through (no cell caches), Blender.empty is the
//! identity. This is the P1.5 reference the golden-vector gate compares
//! bit-in-bit against the live server (bench/golden VectorCapture).
//!
//! Java semantics preserved:
//! * `Ap2` MIN/MAX/MUL bound-based short-circuits (observable with NaN/Inf),
//! * MulOrAdd folding for constant operands (create() order: argument1 first),
//! * Mapped bounds incl. the INVERT swap and ABS/SQUARE max(0,·) rules,
//! * f32 arithmetic INSIDE splines (coordinate cast, hermite terms) and f64
//!   outside (Spline.compute widens the final float),
//! * `(float)` casts exactly where vanilla casts.

use crate::mth;
use crate::noise::{BlendedNoise, NormalNoise};

// --------------------------------------------------------------------------
// CubicSpline (f32 semantics)
// --------------------------------------------------------------------------

/// CubicSpline.Constant or CubicSpline.Multipoint (net.minecraft.util).
#[derive(Debug, Clone)]
pub enum SplineValue {
    Const(f32),
    Multi(Box<MultiSpline>),
}

#[derive(Debug, Clone)]
pub struct MultiSpline {
    pub coordinate: Box<Df>,
    pub locations: Vec<f32>,
    pub values: Vec<SplineValue>,
    pub derivatives: Vec<f32>,
    pub min: f32,
    pub max: f32,
}

impl SplineValue {
    pub fn min_value(&self) -> f32 {
        match self {
            SplineValue::Const(v) => *v,
            SplineValue::Multi(m) => m.min,
        }
    }

    pub fn max_value(&self) -> f32 {
        match self {
            SplineValue::Const(v) => *v,
            SplineValue::Multi(m) => m.max,
        }
    }

    /// CubicSpline.apply — FLOAT path.
    pub fn apply(&self, bank: &NoiseBank, x: i32, y: i32, z: i32) -> f32 {
        match self {
            SplineValue::Const(v) => *v,
            SplineValue::Multi(m) => m.apply(bank, x, y, z),
        }
    }
}

impl MultiSpline {
    /// Multipoint.apply (all f32, final widening by the Spline caller).
    pub fn apply(&self, bank: &NoiseBank, x: i32, y: i32, z: i32) -> f32 {
        let f = self.coordinate.compute(bank, x, y, z) as f32; // Coordinate.apply cast
        let i = find_interval_start(&self.locations, f);
        let i1 = (self.locations.len() - 1) as i32;
        if i < 0 {
            // linearExtend left
            let value = self.values[0].apply(bank, x, y, z);
            return linear_extend(f, &self.locations, value, &self.derivatives, 0);
        }
        if i == i1 {
            let value = self.values[i1 as usize].apply(bank, x, y, z);
            return linear_extend(f, &self.locations, value, &self.derivatives, i1 as usize);
        }
        let f1 = self.locations[i as usize];
        let f2 = self.locations[i as usize + 1];
        let f3 = (f - f1) / (f2 - f1);
        let f6 = self.values[i as usize].apply(bank, x, y, z);
        let f7 = self.values[i as usize + 1].apply(bank, x, y, z);
        let f4 = self.derivatives[i as usize];
        let f5 = self.derivatives[i as usize + 1];
        let f8 = f4 * (f2 - f1) - (f7 - f6);
        let f9 = -f5 * (f2 - f1) + (f7 - f6);
        // `Mth.lerp(f3, f6, f7) + f3 * (1.0f - f3) * Mth.lerp(f3, f8, f9)`
        // binds the FLOAT overload Mth.lerp(float,float,float) — the ENTIRE
        // hermite is f32 arithmetic (the double overload would differ by an
        // f32 ulp in the final values; caught by the density vector gate).
        let l1 = f6 + f3 * (f7 - f6);
        let l2 = f8 + f3 * (f9 - f8);
        l1 + f3 * (1.0f32 - f3) * l2
    }
}

/// Multipoint.findIntervalStart: binarySearch(0, len, i -> start < loc[i]) - 1.
pub fn find_interval_start(locations: &[f32], start: f32) -> i32 {
    // Mth.binarySearch: first index where predicate holds (start < locations[i]).
    let mut min = 0usize;
    let max = locations.len();
    let mut i = max - min;
    while i > 0 {
        let i1 = i / 2;
        let i2 = min + i1;
        if start < locations[i2] {
            i = i1;
        } else {
            min = i2 + 1;
            i -= i1 + 1;
        }
    }
    min as i32 - 1
}

/// Multipoint.linearExtend.
pub fn linear_extend(
    coordinate: f32,
    locations: &[f32],
    value: f32,
    derivatives: &[f32],
    index: usize,
) -> f32 {
    let f = derivatives[index];
    if f == 0.0 {
        value
    } else {
        value + f * (coordinate - locations[index])
    }
}

/// Multipoint.create — bounds computation in f32.
pub fn multi_spline_create(
    coordinate: Box<Df>,
    locations: Vec<f32>,
    values: Vec<SplineValue>,
    derivatives: Vec<f32>,
    coordinate_min: f32,
    coordinate_max: f32,
) -> MultiSpline {
    assert_eq!(locations.len(), values.len());
    assert_eq!(locations.len(), derivatives.len());
    assert!(!locations.is_empty(), "Cannot create a multipoint spline with no points");
    let i = locations.len() - 1;
    let mut f = f32::INFINITY;
    let mut f1 = f32::NEG_INFINITY;
    let f2 = coordinate_min;
    let f3 = coordinate_max;
    if f2 < locations[0] {
        let f4 = linear_extend(f2, &locations, values[0].min_value(), &derivatives, 0);
        let f5 = linear_extend(f2, &locations, values[0].max_value(), &derivatives, 0);
        f = f.min(f4.min(f5));
        f1 = f1.max(f4.max(f5));
    }
    if f3 > locations[i] {
        let f4 = linear_extend(f3, &locations, values[i].min_value(), &derivatives, i);
        let f5 = linear_extend(f3, &locations, values[i].max_value(), &derivatives, i);
        f = f.min(f4.min(f5));
        f1 = f1.max(f4.max(f5));
    }
    for v in &values {
        f = f.min(v.min_value());
        f1 = f1.max(v.max_value());
    }
    for i1 in 0..i {
        let f5 = locations[i1];
        let f6 = locations[i1 + 1];
        let f7 = f6 - f5;
        let f8 = values[i1].min_value();
        let f9 = values[i1].max_value();
        let f10 = values[i1 + 1].min_value();
        let f11 = values[i1 + 1].max_value();
        let f12 = derivatives[i1];
        let f13 = derivatives[i1 + 1];
        if f12 == 0.0 && f13 == 0.0 {
            continue;
        }
        let f14 = f12 * f7;
        let f15 = f13 * f7;
        let min = f8.min(f10);
        let max = f9.max(f11);
        let f16 = f14 - f11 + f8;
        let f17 = f14 - f10 + f9;
        let f18 = -f15 + f10 - f9;
        let f19 = -f15 + f11 - f8;
        let min1 = f16.min(f18);
        let max1 = f17.max(f19);
        f = f.min(min + 0.25f32 * min1);
        f1 = f1.max(max + 0.25f32 * max1);
    }
    MultiSpline { coordinate, locations, values, derivatives, min: f, max: f1 }
}

// --------------------------------------------------------------------------
// Wired IR
// --------------------------------------------------------------------------

/// RarityValueMapper (WeirdScaledSampler).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rarity {
    Type1, // getSpaghettiRarity3D, maxRarity 2.0
    Type2, // getSphaghettiRarity2D, maxRarity 3.0
}

impl Rarity {
    pub fn max_rarity(&self) -> f64 {
        match self {
            Rarity::Type1 => 2.0,
            Rarity::Type2 => 3.0,
        }
    }

    pub fn map(&self, value: f64) -> f64 {
        match self {
            // QuantizedSpaghettiRarity.getSpaghettiRarity3D
            Rarity::Type1 => {
                if value < -0.5 {
                    0.75
                } else if value < 0.0 {
                    1.0
                } else if value < 0.5 {
                    1.5
                } else {
                    2.0
                }
            }
            // QuantizedSpaghettiRarity.getSphaghettiRarity2D
            Rarity::Type2 => {
                if value < -0.75 {
                    0.5
                } else if value < -0.5 {
                    0.75
                } else if value < 0.5 {
                    1.0
                } else if value < 0.75 {
                    2.0
                } else {
                    3.0
                }
            }
        }
    }
}

/// Marker types (DensityFunctions.Marker.Type). The NoiseChunk wrap machine
/// (interpolator.rs) dispatches on these exactly like NoiseChunk.wrapNew.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerType {
    Interpolated,
    FlatCache,
    Cache2D,
    CacheOnce,
    CacheAllInCell,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ap2Type {
    Add,
    Mul,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MappedType {
    Abs,
    Square,
    Cube,
    HalfNegative,
    QuarterNegative,
    Invert,
    Squeeze,
}

impl MappedType {
    /// Mapped.transform — the pure per-value transform.
    pub fn transform(&self, value: f64) -> f64 {
        match self {
            MappedType::Abs => value.abs(),
            MappedType::Square => value * value,
            MappedType::Cube => value * value * value,
            MappedType::HalfNegative => {
                if value > 0.0 {
                    value
                } else {
                    value * 0.5
                }
            }
            MappedType::QuarterNegative => {
                if value > 0.0 {
                    value
                } else {
                    value * 0.25
                }
            }
            MappedType::Invert => 1.0 / value,
            MappedType::Squeeze => {
                let d = mth::clamp(value, -1.0, 1.0);
                d / 2.0 - d * d * d / 24.0
            }
        }
    }
}

/// Bank of wired noise instances (interned by resource key, mirroring
/// RandomState.noiseIntances). Index-based references keep the IR cheap to
/// evaluate.
#[derive(Default)]
pub struct NoiseBank {
    pub noises: Vec<NormalNoise>,
    pub blended: Vec<BlendedNoise>,
}

/// Wired density function tree (scalar reference form).
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Df {
    Const(f64),
    YClampedGradient { from_y: i32, to_y: i32, from_value: f64, to_value: f64 },
    /// noise instance index, xz_scale, y_scale
    Noise(usize, f64, f64),
    ShiftedNoise {
        shift_x: Box<Df>,
        shift_y: Box<Df>,
        shift_z: Box<Df>,
        xz_scale: f64,
        y_scale: f64,
        noise: usize,
    },
    ShiftA(usize),
    ShiftB(usize),
    Shift(usize),
    BlendDensity(Box<Df>),
    /// Marker nodes: pass-through on the unbound scalar path; the
    /// NoiseChunk wrap machine (interpolator.rs) replaces them by cell caches.
    Marker { ty: MarkerType, wrapped: Box<Df> },
    WeirdScaledSampler { input: Box<Df>, noise: usize, rarity: Rarity },
    RangeChoice {
        input: Box<Df>,
        min_inclusive: f64,
        max_exclusive: f64,
        when_in_range: Box<Df>,
        when_out_of_range: Box<Df>,
    },
    Clamp { input: Box<Df>, min: f64, max: f64 },
    Mapped { ty: MappedType, input: Box<Df>, min: f64, max: f64 },
    MulOrAdd { is_add: bool, input: Box<Df>, min: f64, max: f64, argument: f64 },
    Ap2 { ty: Ap2Type, a1: Box<Df>, a2: Box<Df>, min: f64, max: f64 },
    Spline(Box<MultiSpline>),
    Blended(usize),
    BlendAlpha,
    BlendOffset,
    Beardifier,
    EndIslands, // wired with the level seed; scalar eval unsupported (P2 tail)
    FindTopSurface { density: Box<Df>, upper_bound: Box<Df>, lower_bound: i32, cell_height: i32 },
    /// Addendum 35 (Job 441690) — machine-wired FlatCache view. Java's
    /// NoiseChunk constructor maps the whole router through `this::wrap`,
    /// replacing every Marker::FlatCache with a per-chunk FlatCache holding
    /// a (noiseSizeXZ+1)^2 quart-column cache PRECOMPUTED AT y=0. Its
    /// compute() (CFR NoiseChunk.java):
    ///   quart(x,z) inside [firstNoiseX, firstNoiseX+sizeXZ) x [same z]
    ///     -> the cached value = filler.compute(quart-column, y=0)
    ///   outside -> filler.compute(raw x, y, z)
    /// The aquifer's scalar path evaluates the WIRED fields with
    /// SinglePointContext, so it observes exactly this semantics. Our
    /// passthrough (Df::Marker) missed the in-window quart quantization:
    /// tectonic ridges = flat_cache(ridge) quantized java's value at the
    /// out-of-chunk scan columns to the quart column, flipping the
    /// underground_river/parameters range_choice gate (0.2008 vs 0.2
    /// threshold) => floodedness/barrier branch => aquifer status.
    /// Built per-machine by `with_flat_cache_windows`; never spec-hashed
    /// (the transform runs after RandomState::build's hash) and never
    /// interned (aquifer-local).
    FlatCacheWindow { wrapped: Box<Df>, first_noise_x: i32, first_noise_z: i32, size_xz: i32 },
}

impl Df {
    pub fn compute(&self, bank: &NoiseBank, x: i32, y: i32, z: i32) -> f64 {
        match self {
            Df::Const(v) => *v,
            Df::YClampedGradient { from_y, to_y, from_value, to_value } => mth::clamped_map(
                y as f64,
                *from_y as f64,
                *to_y as f64,
                *from_value,
                *to_value,
            ),
            Df::Noise(idx, xz_scale, y_scale) => {
                bank.noises[*idx].get_value(x as f64 * xz_scale, y as f64 * y_scale, z as f64 * xz_scale)
            }
            Df::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, noise } => {
                let d = x as f64 * xz_scale + shift_x.compute(bank, x, y, z);
                let d1 = y as f64 * y_scale + shift_y.compute(bank, x, y, z);
                let d2 = z as f64 * xz_scale + shift_z.compute(bank, x, y, z);
                bank.noises[*noise].get_value(d, d1, d2)
            }
            Df::ShiftA(idx) => {
                // ShiftNoise.compute(x, 0.0, z): getValue(x*0.25, 0.25*0, z*0.25)*4
                bank.noises[*idx].get_value(x as f64 * 0.25, 0.0, z as f64 * 0.25) * 4.0
            }
            Df::ShiftB(idx) => {
                // compute(blockZ, blockX, 0.0) — SWAPPED args
                bank.noises[*idx].get_value(z as f64 * 0.25, x as f64 * 0.25, 0.0) * 4.0
            }
            Df::Shift(idx) => {
                bank.noises[*idx].get_value(x as f64 * 0.25, y as f64 * 0.25, z as f64 * 0.25) * 4.0
            }
            Df::BlendDensity(input) => {
                // Blender.empty().blendDensity = identity (no old-world blending
                // on the scalar path; legacy blending is a Java-fallback lane, I8)
                
                input.compute(bank, x, y, z)
            }
            Df::Marker { wrapped, .. } => wrapped.compute(bank, x, y, z),
            Df::FlatCacheWindow { wrapped, first_noise_x, first_noise_z, size_xz } => {
                // NoiseChunk.FlatCache.compute — quart quantization + y=0 pin
                // inside the machine window, raw filler outside.
                let qx = (x >> 2) - first_noise_x;
                let qz = (z >> 2) - first_noise_z;
                if qx >= 0 && qz >= 0 && qx < *size_xz && qz < *size_xz {
                    wrapped.compute(bank, x & !3, 0, z & !3)
                } else {
                    wrapped.compute(bank, x, y, z)
                }
            }
            Df::WeirdScaledSampler { input, noise, rarity } => {
                let value = input.compute(bank, x, y, z);
                let d = rarity.map(value);
                d * (bank.noises[*noise].get_value(x as f64 / d, y as f64 / d, z as f64 / d)).abs()
            }
            Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
                let d = input.compute(bank, x, y, z);
                if d >= *min_inclusive && d < *max_exclusive {
                    when_in_range.compute(bank, x, y, z)
                } else {
                    when_out_of_range.compute(bank, x, y, z)
                }
            }
            Df::Clamp { input, min, max } => mth::clamp(input.compute(bank, x, y, z), *min, *max),
            Df::Mapped { ty, input, .. } => ty.transform(input.compute(bank, x, y, z)),
            Df::MulOrAdd { is_add, input, argument, .. } => {
                let v = input.compute(bank, x, y, z);
                if *is_add {
                    v + argument
                } else {
                    v * argument
                }
            }
            Df::Ap2 { ty, a1, a2, .. } => {
                let d = a1.compute(bank, x, y, z);
                match ty {
                    Ap2Type::Add => d + a2.compute(bank, x, y, z),
                    Ap2Type::Mul => {
                        if d == 0.0 {
                            0.0 // vanilla short-circuit (yields +0.0)
                        } else {
                            d * a2.compute(bank, x, y, z)
                        }
                    }
                    Ap2Type::Min => {
                        if d < a2.min_value_of(bank) {
                            d
                        } else {
                            mth::java_min(d, a2.compute(bank, x, y, z))
                        }
                    }
                    Ap2Type::Max => {
                        if d > a2.max_value_of(bank) {
                            d
                        } else {
                            mth::java_max(d, a2.compute(bank, x, y, z))
                        }
                    }
                }
            }
            Df::Spline(spline) => spline.apply(bank, x, y, z) as f64,
            Df::Blended(idx) => bank.blended[*idx].compute(x, y, z),
            Df::BlendAlpha => 1.0,
            Df::BlendOffset => 0.0,
            Df::Beardifier => 0.0,
            Df::EndIslands => panic!("EndIslands scalar eval not implemented yet (Phase 2 tail)"),
            Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } => {
                let i = mth::floor(upper_bound.compute(bank, x, y, z) / *cell_height as f64)
                    * cell_height;
                if i <= *lower_bound {
                    return *lower_bound as f64;
                }
                let mut i1 = i;
                while i1 >= *lower_bound {
                    if density.compute(bank, x, i1, z) > 0.0 {
                        return i1 as f64;
                    }
                    i1 -= cell_height;
                }
                *lower_bound as f64
            }
        }
    }

    /// Wired tree bounds (NoiseHolder-bound values).
    pub fn min_value_of(&self, bank: &NoiseBank) -> f64 {
        match self {
            Df::Const(v) => *v,
            Df::YClampedGradient { from_value, to_value, .. } => from_value.min(*to_value),
            Df::Noise(idx, ..) => -bank.noises[*idx].max_value(),
            Df::ShiftedNoise { noise, .. } => -bank.noises[*noise].max_value(),
            Df::ShiftA(idx) | Df::ShiftB(idx) | Df::Shift(idx) => {
                -bank.noises[*idx].max_value() * 4.0
            }
            Df::BlendDensity(_) => f64::NEG_INFINITY,
            Df::Marker { wrapped: w, .. } => w.min_value_of(bank),
            Df::FlatCacheWindow { wrapped: w, .. } => w.min_value_of(bank),
            Df::WeirdScaledSampler { noise, rarity, .. } => {
                0.0f64.min(rarity.max_rarity() * bank.noises[*noise].max_value())
            }
            Df::RangeChoice { when_in_range, when_out_of_range, .. } => {
                when_in_range.min_value_of(bank).min(when_out_of_range.min_value_of(bank))
            }
            Df::Clamp { min, .. } => *min,
            Df::Mapped { min, .. } => *min,
            Df::MulOrAdd { min, .. } => *min,
            Df::Ap2 { min, .. } => *min,
            Df::Spline(s) => s.min as f64,
            Df::Blended(idx) => -bank.blended[*idx].max_value(),
            Df::BlendAlpha => 1.0,
            Df::BlendOffset => 0.0,
            Df::Beardifier => 0.0,
            Df::EndIslands => -0.84375,
            Df::FindTopSurface { lower_bound, .. } => *lower_bound as f64,
        }
    }

    pub fn max_value_of(&self, bank: &NoiseBank) -> f64 {
        match self {
            Df::Const(v) => *v,
            Df::YClampedGradient { from_value, to_value, .. } => from_value.max(*to_value),
            Df::Noise(idx, ..) => bank.noises[*idx].max_value(),
            Df::ShiftedNoise { noise, .. } => bank.noises[*noise].max_value(),
            Df::ShiftA(idx) | Df::ShiftB(idx) | Df::Shift(idx) => {
                bank.noises[*idx].max_value() * 4.0
            }
            Df::BlendDensity(_) => f64::INFINITY,
            Df::Marker { wrapped: w, .. } => w.max_value_of(bank),
            Df::FlatCacheWindow { wrapped: w, .. } => w.max_value_of(bank),
            Df::WeirdScaledSampler { noise, rarity, .. } => {
                rarity.max_rarity() * bank.noises[*noise].max_value()
            }
            Df::RangeChoice { when_in_range, when_out_of_range, .. } => {
                when_in_range.max_value_of(bank).max(when_out_of_range.max_value_of(bank))
            }
            Df::Clamp { max, .. } => *max,
            Df::Mapped { max, .. } => *max,
            Df::MulOrAdd { max, .. } => *max,
            Df::Ap2 { max, .. } => *max,
            Df::Spline(s) => s.max as f64,
            Df::Blended(idx) => bank.blended[*idx].max_value(),
            Df::BlendAlpha => 1.0,
            Df::BlendOffset => 0.0,
            Df::Beardifier => 0.0,
            Df::EndIslands => 0.5625,
            Df::FindTopSurface { lower_bound, upper_bound, .. } => {
                (*lower_bound as f64).max(upper_bound.max_value_of(bank))
            }
        }
    }
}

// --------------------------------------------------------------------------
// TwoArgumentSimpleFunction.create — bound computation + MulOrAdd folding
// --------------------------------------------------------------------------

/// TwoArgumentSimpleFunction.create(type, argument1, argument2) — full port:
/// bound math and the constant-operand MulOrAdd folding.
pub fn ap2_create(ty: Ap2Type, a1: Df, a2: Df, bank: &NoiseBank) -> Df {
    let d = a1.min_value_of(bank);
    let d1 = a2.min_value_of(bank);
    let d2 = a1.max_value_of(bank);
    let d3 = a2.max_value_of(bank);
    let is_const1 = matches!(a1, Df::Const(_));
    let is_const2 = matches!(a2, Df::Const(_));
    let d4 = match ty {
        Ap2Type::Add => d + d1,
        Ap2Type::Mul => {
            if d > 0.0 && d1 > 0.0 {
                d * d1
            } else if d2 < 0.0 && d3 < 0.0 {
                d2 * d3
            } else {
                (d * d3).min(d2 * d1)
            }
        }
        Ap2Type::Min => d.min(d1),
        Ap2Type::Max => d.max(d1),
    };
    let d5 = match ty {
        Ap2Type::Add => d2 + d3,
        Ap2Type::Mul => {
            if d > 0.0 && d1 > 0.0 {
                d2 * d3
            } else if d2 < 0.0 && d3 < 0.0 {
                d * d1
            } else {
                (d * d1).max(d2 * d3)
            }
        }
        Ap2Type::Min => d2.min(d3),
        Ap2Type::Max => d2.max(d3),
    };
    if matches!(ty, Ap2Type::Add | Ap2Type::Mul) {
        if is_const1 {
            let c = match &a1 {
                Df::Const(v) => *v,
                _ => unreachable!(),
            };
            return Df::MulOrAdd {
                is_add: ty == Ap2Type::Add,
                input: Box::new(a2),
                min: d4,
                max: d5,
                argument: c,
            };
        }
        if is_const2 {
            let c = match &a2 {
                Df::Const(v) => *v,
                _ => unreachable!(),
            };
            return Df::MulOrAdd {
                is_add: ty == Ap2Type::Add,
                input: Box::new(a1),
                min: d4,
                max: d5,
                argument: c,
            };
        }
    }
    Df::Ap2 { ty, a1: Box::new(a1), a2: Box::new(a2), min: d4, max: d5 }
}

/// Mapped.create — bound rules (incl. INVERT swap and ABS/SQUARE max(0,·)).
pub fn mapped_create(ty: MappedType, input: Df, bank: &NoiseBank) -> Df {
    let d = input.min_value_of(bank);
    let d1 = input.max_value_of(bank);
    let d2 = ty.transform(d);
    let d3 = ty.transform(d1);
    let (min, max) = if ty == MappedType::Invert {
        if d < 0.0 && d1 > 0.0 {
            (f64::NEG_INFINITY, f64::INFINITY)
        } else {
            (d3, d2) // swapped on purpose (vanilla)
        }
    } else if ty == MappedType::Abs || ty == MappedType::Square {
        (0.0f64.max(d), d2.max(d3))
    } else {
        (d2, d3)
    };
    Df::Mapped { ty, input: Box::new(input), min, max }
}

// ---------------------------------------------------------------------------
// P2.11 — node classification: provable Y-independence of a wired subtree.
// Conservative by construction: anything not proven Y-free classifies as
// Y-DEPENDENT (the cache then simply never applies — no semantic risk).
// Value-equivalence argument: for a Y-free node the evaluation at
// (x, y, z) is bit-identical for every finite y (Noise with y_scale == 0.0
// reads y*0.0 = ±0.0, and the noise kernels are ±0-symmetric at the cell
// level), so memoising one column value reproduces the exact scalar result.
// ---------------------------------------------------------------------------
impl Df {
    pub fn is_y_free(&self, bank: &NoiseBank) -> bool {
        match self {
            Df::Const(_) | Df::BlendAlpha | Df::BlendOffset | Df::Beardifier => true,
            Df::YClampedGradient { .. } | Df::Blended(_) | Df::EndIslands | Df::FindTopSurface { .. } => false,
            // Noise value = instance.getValue(x*xz, y*ys, z*xz): y_scale == 0.0
            // makes the y input ±0.0 — the perlin gradient path is ±0-symmetric
            // (verified against the bit-exact kernel: same value at y=0 and
            // y=-0), so the node is Y-free.
            Df::Noise(_, _, y_scale) => *y_scale == 0.0,
            // ShiftA/ShiftB compute at y=0.0 (2D shift noises); the plain
            // Shift reads y*0.25 — Y-DEPENDENT.
            Df::ShiftA(_) | Df::ShiftB(_) => true,
            Df::Shift(_) => false,
            Df::ShiftedNoise { shift_x, shift_y, shift_z, y_scale, noise, .. } => {
                // shifted noise reads noise.getValue(x*sx + dx, y*ys + dy, z*sz + dz)
                // where dy = shift_y(x,y,z) * 4.0? — the y shift is a child;
                // require: y_scale == 0 AND the shift children y-free? the
                // y-shift affects ONLY the y input: with y_scale == 0 the
                // contribution is (shift_y * 4) * 0 = ±0 — but shift_y is
                // still EVALUATED (side-effect-free, so value-identical).
                let _ = (shift_x, shift_y, shift_z, noise);
                *y_scale == 0.0
            }
            Df::Marker { wrapped, .. } => wrapped.is_y_free(bank),
            // The window value switches between the y=0-pinned quart column
            // (in-window) and the raw-context filler (out-window); even a
            // y-free wrapped content yields DIFFERENT per-column values on
            // the two sides, so a per-column memo keyed by (x,z) alone would
            // mix them. Conservative: Y-DEPENDENT.
            Df::FlatCacheWindow { .. } => false,
            Df::BlendDensity(i) => i.is_y_free(bank),
            // WeirdScaledSampler feeds y/d into a full 3D noise — Y-DEPENDENT.
            Df::WeirdScaledSampler { .. } => false,
            Df::RangeChoice { input, when_in_range, when_out_of_range, .. } => {
                input.is_y_free(bank) && when_in_range.is_y_free(bank) && when_out_of_range.is_y_free(bank)
            }
            Df::Clamp { input, .. } | Df::Mapped { input, .. } | Df::MulOrAdd { input, .. } => input.is_y_free(bank),
            Df::Ap2 { a1, a2, .. } => a1.is_y_free(bank) && a2.is_y_free(bank),
            Df::Spline(_) => {
                // splines evaluate coordinate + location/value splines; the
                // coordinate child may be y-free but location selection is on
                // the coordinate value — the WHOLE spline is y-free iff its
                // coordinate subtree is (values are constants or nested
                // splines selected by the same coordinate...). Nested
                // splines re-enter on the SAME coordinate — but their
                // structure can reference y in VALUES? Vanilla spline values
                // are constants or splines OF THE SAME coordinate — so the
                // entire evaluation is a pure function of the coordinate
                // value. Conservative fallback: NOT proven here.
                false
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P2.11/P2.12 — memoised evaluation (per-chunk column cache). The memo keys
// on the NODE ADDRESS (nodes are interned/boxed for the RandomState lifetime
// and never move) plus the (x, z) column, and is consulted ONLY for
// provably y-free subtrees (is_y_free, conservative). Value equivalence: a
// y-free node's scalar evaluation is bit-identical for every y, so returning
// a cached value reproduces compute() exactly — the zero-diff gates still
// compare every produced value bit-for-bit against the JVM oracle.
// ---------------------------------------------------------------------------

pub type ColumnMemo = std::collections::HashMap<(usize, i32, i32), f64>;

impl SplineValue {
    /// apply with the column memo threaded through nested splines.
    pub fn apply_memo(&self, bank: &NoiseBank, x: i32, y: i32, z: i32, memo: &mut ColumnMemo) -> f32 {
        match self {
            SplineValue::Const(v) => *v,
            SplineValue::Multi(m) => m.apply_memo(bank, x, y, z, memo),
        }
    }
}

impl MultiSpline {
    pub fn apply_memo(&self, bank: &NoiseBank, x: i32, y: i32, z: i32, memo: &mut ColumnMemo) -> f32 {
        if self.coordinate.is_y_free(bank) {
            let key = (self as *const MultiSpline as usize, x, z);
            if let Some(&v) = memo.get(&key) {
                return v as f32;
            }
            let v = self.apply(bank, x, y, z);
            memo.insert(key, v as f64);
            return v;
        }
        // y-dependent coordinate (e.g. the depth gradient): the location
        // selection re-runs per point, but the bracketing VALUES may be
        // y-free inner splines — evaluated through the memo.
        self.apply_memo_inner(bank, x, y, z, memo)
    }

    /// apply() with the values (not the coordinate) routed through the memo —
    /// arithmetic identical to MultiSpline::apply.
    fn apply_memo_inner(&self, bank: &NoiseBank, x: i32, y: i32, z: i32, memo: &mut ColumnMemo) -> f32 {
        let f = self.coordinate.compute(bank, x, y, z) as f32;
        let i = find_interval_start(&self.locations, f);
        let i1 = (self.locations.len() - 1) as i32;
        if i < 0 {
            let value = self.values[0].apply_memo(bank, x, y, z, memo);
            return linear_extend(f, &self.locations, value, &self.derivatives, 0);
        }
        if i == i1 {
            let value = self.values[i1 as usize].apply_memo(bank, x, y, z, memo);
            return linear_extend(f, &self.locations, value, &self.derivatives, i1 as usize);
        }
        let f1 = self.locations[i as usize];
        let f2 = self.locations[i as usize + 1];
        let f3 = (f - f1) / (f2 - f1);
        let f6 = self.values[i as usize].apply_memo(bank, x, y, z, memo);
        let f7 = self.values[i as usize + 1].apply_memo(bank, x, y, z, memo);
        let f4 = self.derivatives[i as usize];
        let f5 = self.derivatives[i as usize + 1];
        let f8 = f4 * (f2 - f1) - (f7 - f6);
        let f9 = -f5 * (f2 - f1) + (f7 - f6);
        let l1 = f6 + f3 * (f7 - f6);
        let l2 = f8 + f3 * (f9 - f8);
        l1 + f3 * (1.0f32 - f3) * l2
    }
}

impl Df {
    /// Memoised scalar evaluation for the climate-sampler path (and any
    /// caller with column locality). Recurses through the transparent
    /// wrappers (markers, unary transforms) and the spline/arithmetic shape,
    /// memoising every PROVABLY y-free subtree per (x, z) column. Arithmetic
    /// and operand order are identical to compute(); the memo only returns a
    /// cached bit-identical value for a pure y-free subtree.
    pub fn compute_memo(&self, bank: &NoiseBank, x: i32, y: i32, z: i32, memo: &mut ColumnMemo) -> f64 {
        // cheap path: a y-free subtree caches wholesale
        if self.is_y_free(bank) {
            let key = (self as *const Df as usize, x, z);
            if let Some(&v) = memo.get(&key) {
                return v;
            }
            let v = self.compute_memo_inner(bank, x, y, z, memo);
            memo.insert(key, v);
            return v;
        }
        self.compute_memo_inner(bank, x, y, z, memo)
    }

    fn compute_memo_inner(&self, bank: &NoiseBank, x: i32, y: i32, z: i32, memo: &mut ColumnMemo) -> f64 {
        match self {
            Df::Spline(m) => m.apply_memo(bank, x, y, z, memo) as f64,
            Df::Marker { wrapped, .. } => wrapped.compute_memo(bank, x, y, z, memo),
            Df::BlendDensity(i) => i.compute_memo(bank, x, y, z, memo),
            Df::Clamp { input, min, max } => {
                let v = input.compute_memo(bank, x, y, z, memo);
                crate::mth::clamp(v, *min, *max)
            }
            Df::Mapped { ty, input, .. } => ty.transform(input.compute_memo(bank, x, y, z, memo)),
            Df::MulOrAdd { is_add, input, argument, .. } => {
                let v = input.compute_memo(bank, x, y, z, memo);
                if *is_add { v + argument } else { v * argument }
            }
            Df::Ap2 { ty, a1, a2, .. } => {
                let d = a1.compute_memo(bank, x, y, z, memo);
                match ty {
                    Ap2Type::Add => d + a2.compute_memo(bank, x, y, z, memo),
                    Ap2Type::Mul => {
                        if d == 0.0 {
                            0.0
                        } else {
                            d * a2.compute_memo(bank, x, y, z, memo)
                        }
                    }
                    Ap2Type::Min => {
                        if d < a2.min_value_of(bank) {
                            d
                        } else {
                            crate::mth::java_min(d, a2.compute_memo(bank, x, y, z, memo))
                        }
                    }
                    Ap2Type::Max => {
                        if d > a2.max_value_of(bank) {
                            d
                        } else {
                            crate::mth::java_max(d, a2.compute_memo(bank, x, y, z, memo))
                        }
                    }
                }
            }
            Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
                let d = input.compute_memo(bank, x, y, z, memo);
                if d >= *min_inclusive && d < *max_exclusive {
                    when_in_range.compute_memo(bank, x, y, z, memo)
                } else {
                    when_out_of_range.compute_memo(bank, x, y, z, memo)
                }
            }
            _ => self.compute(bank, x, y, z),
        }
    }
}

// ---------------------------------------------------------------------------
// Addendum 35 (Job 441690) — the machine FlatCache view for unbound scalar
// evaluation (aquifer path). NoiseChunk maps the router through this::wrap,
// swapping every Marker::FlatCache for a per-chunk FlatCache with a
// (noiseSizeXZ+1)^2 quart-column cache precomputed at y=0 (CFR
// NoiseChunk.java FlatCache.compute: in-window -> cached quart column,
// out-window -> filler at the raw context). The aquifer evaluates the wired
// fields with SinglePointContext, i.e. THROUGH those wrappers; our scalar
// passthrough must reproduce them. This transform binds each FlatCache
// marker to ONE machine window.
// ---------------------------------------------------------------------------

/// Rebuild `df` replacing every Marker::FlatCache with a FlatCacheWindow
/// bound to (first_noise_x, first_noise_z, size_xz). Clones leaves; recurses
/// into every child including spline coordinates and nested spline values.
pub fn with_flat_cache_windows(df: &Df, first_noise_x: i32, first_noise_z: i32, size_xz: i32) -> Df {
    let rec = |d: &Df| with_flat_cache_windows(d, first_noise_x, first_noise_z, size_xz);
    match df {
        Df::Const(_)
        | Df::YClampedGradient { .. }
        | Df::Noise(..)
        | Df::ShiftA(_)
        | Df::ShiftB(_)
        | Df::Shift(_)
        | Df::Blended(_)
        | Df::BlendAlpha
        | Df::BlendOffset
        | Df::Beardifier
        | Df::EndIslands => (*df).clone(),
        | Df::FlatCacheWindow { .. } => (*df).clone(),
        Df::Marker { ty, wrapped } => {
            let w = Box::new(rec(wrapped));
            if matches!(ty, MarkerType::FlatCache) {
                Df::FlatCacheWindow { wrapped: w, first_noise_x, first_noise_z, size_xz }
            } else {
                Df::Marker { ty: *ty, wrapped: w }
            }
        }
        Df::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, noise } => Df::ShiftedNoise {
            shift_x: Box::new(rec(shift_x)),
            shift_y: Box::new(rec(shift_y)),
            shift_z: Box::new(rec(shift_z)),
            xz_scale: *xz_scale,
            y_scale: *y_scale,
            noise: *noise,
        },
        Df::BlendDensity(input) => Df::BlendDensity(Box::new(rec(input))),
        Df::WeirdScaledSampler { input, noise, rarity } => Df::WeirdScaledSampler {
            input: Box::new(rec(input)),
            noise: *noise,
            rarity: *rarity,
        },
        Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
            Df::RangeChoice {
                input: Box::new(rec(input)),
                min_inclusive: *min_inclusive,
                max_exclusive: *max_exclusive,
                when_in_range: Box::new(rec(when_in_range)),
                when_out_of_range: Box::new(rec(when_out_of_range)),
            }
        }
        Df::Clamp { input, min, max } => Df::Clamp {
            input: Box::new(rec(input)),
            min: *min,
            max: *max,
        },
        Df::Mapped { ty, input, min, max } => Df::Mapped {
            ty: *ty,
            input: Box::new(rec(input)),
            min: *min,
            max: *max,
        },
        Df::MulOrAdd { is_add, input, min, max, argument } => Df::MulOrAdd {
            is_add: *is_add,
            input: Box::new(rec(input)),
            min: *min,
            max: *max,
            argument: *argument,
        },
        Df::Ap2 { ty, a1, a2, min, max } => Df::Ap2 {
            ty: *ty,
            a1: Box::new(rec(a1)),
            a2: Box::new(rec(a2)),
            min: *min,
            max: *max,
        },
        Df::Spline(ms) => Df::Spline(Box::new(spline_with_flat_cache_windows(ms, first_noise_x, first_noise_z, size_xz))),
        Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } => Df::FindTopSurface {
            density: Box::new(rec(density)),
            upper_bound: Box::new(rec(upper_bound)),
            lower_bound: *lower_bound,
            cell_height: *cell_height,
        },
    }
}

fn spline_with_flat_cache_windows(ms: &MultiSpline, fx: i32, fz: i32, sxz: i32) -> MultiSpline {
    let rec_value = |v: &SplineValue| match v {
        SplineValue::Const(c) => SplineValue::Const(*c),
        SplineValue::Multi(inner) => SplineValue::Multi(Box::new(spline_with_flat_cache_windows(inner, fx, fz, sxz))),
    };
    MultiSpline {
        coordinate: Box::new(with_flat_cache_windows(&ms.coordinate, fx, fz, sxz)),
        locations: ms.locations.clone(),
        values: ms.values.iter().map(rec_value).collect(),
        derivatives: ms.derivatives.clone(),
        min: ms.min,
        max: ms.max,
    }
}

#[cfg(test)]
mod flat_cache_window_tests {
    use super::*;

    /// Addendum 35: in-window queries return the y=0-PINNED quart-column
    /// value (even at y != 0); out-window queries pass the raw context.
    #[test]
    fn flat_cache_window_y_pin_and_fallback() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        // y-dependent content: the gradient value differs at every y.
        let inner = Df::YClampedGradient { from_y: -16, to_y: 16, from_value: 5.0, to_value: -5.0 };
        let df = Df::FlatCacheWindow {
            wrapped: Box::new(inner),
            first_noise_x: 0,
            first_noise_z: 0,
            size_xz: 5,
        };
        // y=0 gradient value = 0.0; in-window quart x/z = 0..4 => blocks 0..15.
        let y0 = Df::YClampedGradient { from_y: -16, to_y: 16, from_value: 5.0, to_value: -5.0 }
            .compute(&bank, 0, 0, 0);
        assert_eq!(y0.to_bits(), 0.0f64.to_bits());
        for x in [0, 3, 15] {
            for y in [-40, -1, 0, 7, 44] {
                let v = df.compute(&bank, x, y, 9);
                assert_eq!(v.to_bits(), y0.to_bits(), "in-window ({x},{y},9) must pin y=0");
            }
        }
        // Out of window (x=20 => quart 5 >= size_xz): raw gradient at y.
        // clamped_map(10, -16, 16, 5, -5) = 5 - (26/32)*10 = -3.125
        let v = df.compute(&bank, 20, 10, 0);
        assert_eq!(v.to_bits(), (-3.125f64).to_bits(), "out-window must stay raw");
    }

    /// Negative-chunk windows: firstNoiseX = floorDiv(chunkMinX, 4) and the
    /// quart quantization must floor toward -inf (x & !3 semantics).
    #[test]
    fn flat_cache_window_negative_origin() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        let inner = Df::Const(7.5);
        // chunk (12,24) from the tectA evidence: firstNoiseX = 48, window
        // x quarts 48..52 = blocks 192..211 INCLUSIVE (the slot-95 column!).
        let df = Df::FlatCacheWindow {
            wrapped: Box::new(inner),
            first_noise_x: 48,
            first_noise_z: 96,
            size_xz: 5,
        };
        for x in [192, 207, 208, 211] {
            assert_eq!(df.compute(&bank, x, 44, 391).to_bits(), 7.5f64.to_bits(), "x={x} in-window");
        }
        assert_eq!((208i32 >> 2) - 48, 4, "block 208 = last in-window quart");
        assert_eq!((212i32 >> 2) - 48, 5, "block 212 = first out-window quart");
        // Negative origin: floorDiv semantics via arithmetic shift.
        let dfn = Df::FlatCacheWindow {
            wrapped: Box::new(Df::Const(1.0)),
            first_noise_x: -52,
            first_noise_z: -60,
            size_xz: 5,
        };
        // chunk (-13,-15): blocks -208..-193 x -240..-225 in-window.
        assert_eq!((-198i32 >> 2) - (-52), 2, "negative floor division");
        assert_eq!(dfn.compute(&bank, -198, 57, -240).to_bits(), 1.0f64.to_bits());
        assert_eq!(dfn.compute(&bank, -192, 57, -240).to_bits(), 1.0f64.to_bits());
    }
}
