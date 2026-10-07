//! NoiseChunk cell-interpolation replica (NCF P2.3-tail) — bit-exact port of
//! net.minecraft.world.level.levelgen.NoiseChunk + the doFill driving loop
//! (NoiseBasedChunkGenerator, cfr-out 2026-10-05).
//!
//! Why an arena: Java dedups wrappers through `NoiseChunk.wrapped`, a
//! HashMap keyed by RECORD EQUALITY (structural). Two structurally equal
//! markers therefore share ONE stateful wrapper (one slice pair, one cache
//! state). The intern arena reproduces that exactly: structurally equal
//! subtrees get one index, so wrapper dedup by (marker type, inner index)
//! matches Java's record-equality dedup bit for bit. (Self-containment is
//! impossible for finite trees, so a CacheAllInCell's filler can never
//! contain its own wrapper; reads of a DIFFERENT dedup-equal cache during a
//! fill see the live field, exactly like Java.)
//!
//! Faithful machinery (values AND state):
//! * counters `interpolation_counter` / `array_interpolation_counter`
//!   (the CacheOnce epochs),
//! * NoiseInterpolator slice pairs + swapSlices + selectCellYZ corner reads
//!   + updateForY/X/Z lerp chains (Mth.lerp double semantics) + the
//!   fillingCell lerp3 path,
//! * CacheAllInCell per-cell fill via the forIndex mapping (y DESCENDING),
//! * Cache2D lastPos2D, CacheOnce lastCounter/lastValue/lastArray/
//!   lastArrayCounter (incl. the array length-change replacement on the
//!   store side; the read side would throw in Java on a length change, so we
//!   panic likewise — vanilla never hits it),
//! * FlatCache eager quart tables,
//! * the batched fillArray overrides of Ap2 (Add = both arrays then add;
//!   Mul/Min/Max = per-index forIndex), Mapped, RangeChoice, CacheOnce,
//!   Cache2D, NoiseInterpolator, CacheAllInCell, FlatCache — the CALL ORDER
//!   matters because CacheOnce/Cache2D state flows through it,
//! * Blender.empty: BlendAlpha/BlendOffset stay the constant functions
//!   (1.0/0.0); the beardifier is the zero marker (structure-free capture;
//!   structures are Phase 5).
//!
//! The driver mirrors NoiseBasedChunkGenerator.doFill: initializeForFirstCellX
//! (fillSlice(true, firstCellX)) -> per cell-X column: advanceCellX
//! (fillSlice(false, firstCellX+cx+1), cellStartBlockX) -> per cell-Z: per
//! cell-Y (DESC): selectCellYZ -> per in-cell Y (DESC): updateForY -> per
//! in-cell X: updateForX -> per in-cell Z: updateForZ; the per-block value of
//! every interpolator is its post-updateForZ `value`. swapSlices() after each
//! cell-X column (doFill line 340).
//!
//! clippy::needless_range_loop is allowed module-wide: the fillArray
//! overrides and the drive loop replicate Java's indexed loops VERBATIM
//! (index arithmetic is part of the bit-exact contract).

#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;

use crate::density::{Ap2Type, MarkerType, MappedType, MultiSpline, NoiseBank, Rarity, SplineValue};
use crate::mth;

// --------------------------------------------------------------------------
// Intern arena (structural identity, mirrors Java record equality)
// --------------------------------------------------------------------------

/// Interned IR node: children are arena indices, floats kept as raw bits so
/// structural equality is exact (NaN bits preserved).
#[derive(Debug, Clone)]
pub enum INode {
    Const(u64),
    YClampedGradient { from_y: i32, to_y: i32, from_value: u64, to_value: u64 },
    Noise(usize, u64, u64),
    ShiftedNoise { sx: usize, sy: usize, sz: usize, xz: u64, ys: u64, noise: usize },
    ShiftA(usize),
    ShiftB(usize),
    Shift(usize),
    BlendDensity(usize),
    Marker { ty: MarkerType, wrapped: usize },
    WeirdScaledSampler { input: usize, noise: usize, rarity: Rarity },
    RangeChoice { input: usize, min: u64, max: u64, in_range: usize, out_of_range: usize },
    Clamp { input: usize, min: u64, max: u64 },
    Mapped { ty: MappedType, input: usize },
    MulOrAdd { is_add: bool, input: usize, argument: u64 },
    Ap2 { ty: Ap2Type, a1: usize, a2: usize, a2_min: u64, a2_max: u64 },
    Spline(usize),
    Blended(usize),
    BlendAlpha,
    BlendOffset,
    Beardifier,
    EndIslands,
    FindTopSurface { density: usize, upper: usize, lower_bound: i32, cell_height: i32 },
}

#[derive(Debug, Clone)]
pub struct ISpline {
    pub coordinate: usize,
    pub locations: Vec<f32>,
    pub values: Vec<ISplineValue>,
    pub derivatives: Vec<f32>,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone)]
pub enum ISplineValue {
    Const(f32),
    Multi(usize),
}

#[derive(Default)]
pub struct Arena {
    pub nodes: Vec<INode>,
    pub splines: Vec<ISpline>,
    spline_memo: HashMap<u64, Vec<usize>>,
    memo: HashMap<u64, Vec<usize>>,
}

fn mix(h: &mut u64, v: u64) {
    *h ^= v;
    *h = h.wrapping_mul(0x100000001b3);
}

impl Arena {
    fn spline_hash(s: &ISpline) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        mix(&mut h, s.coordinate as u64);
        for l in &s.locations {
            mix(&mut h, l.to_bits() as u64);
        }
        for d in &s.derivatives {
            mix(&mut h, d.to_bits() as u64);
        }
        for v in &s.values {
            match v {
                ISplineValue::Const(c) => mix(&mut h, 0x5f00 | c.to_bits() as u64),
                ISplineValue::Multi(i) => mix(&mut h, 0x5f01 | (*i as u64) << 8),
            }
        }
        mix(&mut h, s.min.to_bits() as u64);
        mix(&mut h, s.max.to_bits() as u64);
        h
    }

    fn spline_eq(a: &ISpline, b: &ISpline) -> bool {
        a.coordinate == b.coordinate
            && a.locations == b.locations
            && a.derivatives == b.derivatives
            && a.min.to_bits() == b.min.to_bits()
            && a.max.to_bits() == b.max.to_bits()
            && a.values.len() == b.values.len()
            && a.values.iter().zip(&b.values).all(|(x, y)| match (x, y) {
                (ISplineValue::Const(c), ISplineValue::Const(d)) => c.to_bits() == d.to_bits(),
                (ISplineValue::Multi(i), ISplineValue::Multi(j)) => i == j,
                _ => false,
            })
    }

    fn intern(&mut self, node: INode) -> usize {
        let mut h: u64 = 0xcbf29ce484222325;
        {
            let mut tag = |t: u64| mix(&mut h, t);
            match &node {
                INode::Const(_) => tag(1),
                INode::YClampedGradient { .. } => tag(2),
                INode::Noise(..) => tag(3),
                INode::ShiftedNoise { .. } => tag(4),
                INode::ShiftA(_) => tag(5),
                INode::ShiftB(_) => tag(6),
                INode::Shift(_) => tag(7),
                INode::BlendDensity(_) => tag(8),
                INode::Marker { .. } => tag(9),
                INode::WeirdScaledSampler { .. } => tag(10),
                INode::RangeChoice { .. } => tag(11),
                INode::Clamp { .. } => tag(12),
                INode::Mapped { .. } => tag(13),
                INode::MulOrAdd { .. } => tag(14),
                INode::Ap2 { .. } => tag(15),
                INode::Spline(_) => tag(16),
                INode::Blended(_) => tag(17),
                INode::BlendAlpha => tag(18),
                INode::BlendOffset => tag(19),
                INode::Beardifier => tag(20),
                INode::EndIslands => tag(21),
                INode::FindTopSurface { .. } => tag(22),
            }
        }
        match &node {
            INode::Const(v) => mix(&mut h, *v),
            INode::YClampedGradient { from_y, to_y, from_value, to_value } => {
                mix(&mut h, *from_y as u64);
                mix(&mut h, *to_y as u64);
                mix(&mut h, *from_value);
                mix(&mut h, *to_value);
            }
            INode::Noise(a, b, c) => {
                mix(&mut h, *a as u64);
                mix(&mut h, *b);
                mix(&mut h, *c);
            }
            INode::ShiftedNoise { sx, sy, sz, xz, ys, noise } => {
                mix(&mut h, *sx as u64);
                mix(&mut h, *sy as u64);
                mix(&mut h, *sz as u64);
                mix(&mut h, *xz);
                mix(&mut h, *ys);
                mix(&mut h, *noise as u64);
            }
            INode::ShiftA(a) | INode::ShiftB(a) | INode::Shift(a) | INode::BlendDensity(a) | INode::Blended(a) => {
                mix(&mut h, *a as u64);
            }
            INode::Marker { ty, wrapped } => {
                mix(&mut h, *ty as usize as u64);
                mix(&mut h, *wrapped as u64);
            }
            INode::WeirdScaledSampler { input, noise, rarity } => {
                mix(&mut h, *input as u64);
                mix(&mut h, *noise as u64);
                mix(&mut h, *rarity as usize as u64);
            }
            INode::RangeChoice { input, min, max, in_range, out_of_range } => {
                mix(&mut h, *input as u64);
                mix(&mut h, *min);
                mix(&mut h, *max);
                mix(&mut h, *in_range as u64);
                mix(&mut h, *out_of_range as u64);
            }
            INode::Clamp { input, min, max } => {
                mix(&mut h, *input as u64);
                mix(&mut h, *min);
                mix(&mut h, *max);
            }
            INode::Mapped { ty, input } => {
                mix(&mut h, *ty as usize as u64);
                mix(&mut h, *input as u64);
            }
            INode::MulOrAdd { is_add, input, argument } => {
                mix(&mut h, *is_add as u64);
                mix(&mut h, *input as u64);
                mix(&mut h, *argument);
            }
            INode::Ap2 { ty, a1, a2, a2_min, a2_max } => {
                mix(&mut h, *ty as usize as u64);
                mix(&mut h, *a1 as u64);
                mix(&mut h, *a2 as u64);
                mix(&mut h, *a2_min);
                mix(&mut h, *a2_max);
            }
            INode::Spline(s) => mix(&mut h, Self::spline_hash(&self.splines[*s])),
            INode::BlendAlpha | INode::BlendOffset | INode::Beardifier | INode::EndIslands => {}
            INode::FindTopSurface { density, upper, lower_bound, cell_height } => {
                mix(&mut h, *density as u64);
                mix(&mut h, *upper as u64);
                mix(&mut h, *lower_bound as u64);
                mix(&mut h, *cell_height as u64);
            }
        }
        if let Some(cands) = self.memo.get(&h) {
            for &c in cands {
                if self.node_eq(&self.nodes[c], &node) {
                    return c;
                }
            }
        }
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.memo.entry(h).or_default().push(idx);
        idx
    }

    fn node_eq(&self, a: &INode, b: &INode) -> bool {
        match (a, b) {
            (INode::Const(x), INode::Const(y)) => x == y,
            (INode::YClampedGradient { from_y: a, to_y: b, from_value: c, to_value: d }, INode::YClampedGradient { from_y: e, to_y: f, from_value: g, to_value: h }) => {
                a == e && b == f && c == g && d == h
            }
            (INode::Noise(a, b, c), INode::Noise(d, e, f)) => a == d && b == e && c == f,
            (INode::ShiftedNoise { sx, sy, sz, xz, ys, noise }, INode::ShiftedNoise { sx: sx2, sy: sy2, sz: sz2, xz: xz2, ys: ys2, noise: noise2 }) => {
                sx == sx2 && sy == sy2 && sz == sz2 && xz == xz2 && ys == ys2 && noise == noise2
            }
            (INode::ShiftA(a), INode::ShiftA(b))
            | (INode::ShiftB(a), INode::ShiftB(b))
            | (INode::Shift(a), INode::Shift(b))
            | (INode::BlendDensity(a), INode::BlendDensity(b))
            | (INode::Blended(a), INode::Blended(b)) => a == b,
            (INode::Marker { ty, wrapped }, INode::Marker { ty: ty2, wrapped: w2 }) => ty == ty2 && wrapped == w2,
            (INode::WeirdScaledSampler { input, noise, rarity }, INode::WeirdScaledSampler { input: i2, noise: n2, rarity: r2 }) => {
                input == i2 && noise == n2 && rarity == r2
            }
            (INode::RangeChoice { input, min, max, in_range, out_of_range }, INode::RangeChoice { input: i2, min: mi2, max: ma2, in_range: ir2, out_of_range: or2 }) => {
                input == i2 && min == mi2 && max == ma2 && in_range == ir2 && out_of_range == or2
            }
            (INode::Clamp { input, min, max }, INode::Clamp { input: i2, min: mi2, max: ma2 }) => {
                input == i2 && min == mi2 && max == ma2
            }
            (INode::Mapped { ty, input }, INode::Mapped { ty: ty2, input: i2 }) => ty == ty2 && input == i2,
            (INode::MulOrAdd { is_add, input, argument }, INode::MulOrAdd { is_add: a2, input: i2, argument: g2 }) => {
                is_add == a2 && input == i2 && argument == g2
            }
            (INode::Ap2 { ty, a1, a2, a2_min, a2_max }, INode::Ap2 { ty: ty2, a1: b1, a2: b2, a2_min: m1, a2_max: m2 }) => {
                ty == ty2 && a1 == b1 && a2 == b2 && a2_min == m1 && a2_max == m2
            }
            (INode::Spline(a), INode::Spline(b)) => Self::spline_eq(&self.splines[*a], &self.splines[*b]),
            (INode::BlendAlpha, INode::BlendAlpha)
            | (INode::BlendOffset, INode::BlendOffset)
            | (INode::Beardifier, INode::Beardifier)
            | (INode::EndIslands, INode::EndIslands) => true,
            (INode::FindTopSurface { density, upper, lower_bound, cell_height }, INode::FindTopSurface { density: d2, upper: u2, lower_bound: l2, cell_height: c2 }) => {
                density == d2 && upper == u2 && lower_bound == l2 && cell_height == c2
            }
            _ => false,
        }
    }

    fn intern_spline(&mut self, s: ISpline) -> usize {
        let h = Self::spline_hash(&s);
        if let Some(cands) = self.spline_memo.get(&h) {
            for &c in cands {
                if Self::spline_eq(&self.splines[c], &s) {
                    return c;
                }
            }
        }
        let idx = self.splines.len();
        self.splines.push(s);
        self.spline_memo.entry(h).or_default().push(idx);
        idx
    }
}

fn bits(v: f64) -> u64 {
    v.to_bits()
}

/// Intern a Df tree bottom-up (structural sharing = Java record equality).
pub fn intern_df(df: &crate::density::Df, arena: &mut Arena, bank: &NoiseBank) -> usize {
    use crate::density::Df;
    let node = match df {
        Df::Const(v) => INode::Const(bits(*v)),
        Df::YClampedGradient { from_y, to_y, from_value, to_value } => INode::YClampedGradient {
            from_y: *from_y,
            to_y: *to_y,
            from_value: bits(*from_value),
            to_value: bits(*to_value),
        },
        Df::Noise(idx, xz, ys) => INode::Noise(*idx, bits(*xz), bits(*ys)),
        Df::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, noise } => INode::ShiftedNoise {
            sx: intern_df(shift_x, arena, bank),
            sy: intern_df(shift_y, arena, bank),
            sz: intern_df(shift_z, arena, bank),
            xz: bits(*xz_scale),
            ys: bits(*y_scale),
            noise: *noise,
        },
        Df::ShiftA(i) => INode::ShiftA(*i),
        Df::ShiftB(i) => INode::ShiftB(*i),
        Df::Shift(i) => INode::Shift(*i),
        Df::BlendDensity(i) => INode::BlendDensity(intern_df(i, arena, bank)),
        Df::Marker { ty, wrapped } => INode::Marker { ty: *ty, wrapped: intern_df(wrapped, arena, bank) },
        // FlatCacheWindow trees are aquifer-local (built after interning) —
        // arm kept total; the intern shape mirrors the Marker passthrough
        // with the window constants folded into the tag.
        Df::FlatCacheWindow { wrapped, first_noise_x, first_noise_z, size_xz } => INode::Marker {
            ty: crate::density::MarkerType::Cache2D,
            wrapped: {
                // window semantics are NOT representable in INode — but this
                // arm is unreachable on every current path (the transformed
                // trees never reach the interpolator); if it ever fires it
                // must not silently pass, so panic loudly instead.
                let _ = (wrapped, first_noise_x, first_noise_z, size_xz);
                panic!("FlatCacheWindow reached intern_df — the machine view must stay aquifer-local (addendum 35)")
            },
        },
        Df::WeirdScaledSampler { input, noise, rarity } => INode::WeirdScaledSampler {
            input: intern_df(input, arena, bank),
            noise: *noise,
            rarity: *rarity,
        },
        Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
            INode::RangeChoice {
                input: intern_df(input, arena, bank),
                min: bits(*min_inclusive),
                max: bits(*max_exclusive),
                in_range: intern_df(when_in_range, arena, bank),
                out_of_range: intern_df(when_out_of_range, arena, bank),
            }
        }
        Df::Clamp { input, min, max } => {
            INode::Clamp { input: intern_df(input, arena, bank), min: bits(*min), max: bits(*max) }
        }
        Df::Mapped { ty, input, .. } => INode::Mapped { ty: *ty, input: intern_df(input, arena, bank) },
        Df::MulOrAdd { is_add, input, argument, .. } => {
            INode::MulOrAdd { is_add: *is_add, input: intern_df(input, arena, bank), argument: bits(*argument) }
        }
        Df::Ap2 { ty, a1, a2, .. } => INode::Ap2 {
            ty: *ty,
            a1: intern_df(a1, arena, bank),
            a2: intern_df(a2, arena, bank),
            a2_min: bits(a2.min_value_of(bank)),
            a2_max: bits(a2.max_value_of(bank)),
        },
        Df::Spline(s) => INode::Spline(intern_spline(s, arena, bank)),
        Df::Blended(i) => INode::Blended(*i),
        Df::BlendAlpha => INode::BlendAlpha,
        Df::BlendOffset => INode::BlendOffset,
        Df::Beardifier => INode::Beardifier,
        Df::EndIslands => INode::EndIslands,
        Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } => INode::FindTopSurface {
            density: intern_df(density, arena, bank),
            upper: intern_df(upper_bound, arena, bank),
            lower_bound: *lower_bound,
            cell_height: *cell_height,
        },
    };
    arena.intern(node)
}

fn intern_spline(s: &MultiSpline, arena: &mut Arena, bank: &NoiseBank) -> usize {
    let is = ISpline {
        coordinate: intern_df(&s.coordinate, arena, bank),
        locations: s.locations.clone(),
        values: s
            .values
            .iter()
            .map(|v| match v {
                SplineValue::Const(c) => ISplineValue::Const(*c),
                SplineValue::Multi(m) => ISplineValue::Multi(intern_spline(m, arena, bank)),
            })
            .collect(),
        derivatives: s.derivatives.clone(),
        min: s.min,
        max: s.max,
    };
    arena.intern_spline(is)
}

// --------------------------------------------------------------------------
// Wrapped tree (the mapAll(wrap) result)
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum WKind {
    Interp(usize),     // NoiseInterpolator id
    FlatCacheW(usize), // FlatCache id
    Cache2DW(usize),   // Cache2D id
    CacheOnceW(usize), // CacheOnce id
    CellCacheW(usize), // CacheAllInCell id
}

#[derive(Debug, Clone)]
enum WNode {
    Const(u64),
    YClampedGradient { from_y: i32, to_y: i32, from_value: u64, to_value: u64 },
    Noise(usize, u64, u64),
    ShiftedNoise { sx: usize, sy: usize, sz: usize, xz: u64, ys: u64, noise: usize },
    ShiftA(usize),
    ShiftB(usize),
    Shift(usize),
    BlendDensity(usize),
    WeirdScaledSampler { input: usize, noise: usize, rarity: Rarity },
    RangeChoice { input: usize, min: u64, max: u64, in_range: usize, out_of_range: usize },
    Clamp { input: usize, min: u64, max: u64 },
    Mapped { ty: MappedType, input: usize },
    MulOrAdd { is_add: bool, input: usize, argument: u64 },
    Ap2 { ty: Ap2Type, a1: usize, a2: usize, a2_min: u64, a2_max: u64 },
    Spline(usize),
    Blended(usize),
    BlendAlpha,
    BlendOffset,
    Beardifier,
    EndIslands,
    FindTopSurface { density: usize, upper: usize, lower_bound: i32, cell_height: i32 },
    W(WKind),
}

/// Wrapped spline: the coordinate and Multi children are W indices.
struct WSpline {
    coordinate: usize,
    locations: Vec<f32>,
    values: Vec<WSplineValue>,
    derivatives: Vec<f32>,
    /// bounds kept for structural fidelity (unused at evaluation time)
    #[allow(dead_code)]
    min: f32,
    #[allow(dead_code)]
    max: f32,
}

#[derive(Debug, Clone)]
enum WSplineValue {
    Const(f32),
    Multi(usize),
}

struct InterpState {
    inner: usize,
    slice0: Vec<Vec<f64>>,
    slice1: Vec<Vec<f64>>,
    /// Java field order: [000, 001, 100, 101, 010, 011, 110, 111]
    noise: [f64; 8],
    value_xz00: f64,
    value_xz10: f64,
    value_xz01: f64,
    value_xz11: f64,
    value_z0: f64,
    value_z1: f64,
    value: f64,
}

struct CellCacheState {
    inner: usize,
    values: Vec<f64>,
}

struct Cache2DState {
    inner: usize,
    last_pos2d: i64,
    last_value: f64,
}

struct CacheOnceState {
    inner: usize,
    last_counter: u64,
    last_array_counter: u64,
    last_value: f64,
    last_array: Option<Vec<f64>>,
}

struct FlatCacheState {
    inner: usize,
    values: Vec<f64>,
    size_xz: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Provider {
    /// sliceFillingContextProvider (corner fills)
    Slice,
    /// the NoiseChunk itself (cell-cache fills)
    Cell,
}

/// Eval context: block position + the Java `context == noiseChunk` identity
/// (false = foreign SinglePointContext -> wrappers fall through to inner).
#[derive(Debug, Clone, Copy)]
struct Ctx {
    x: i32,
    y: i32,
    z: i32,
    in_chunk: bool,
}

/// The NoiseChunk simulation. Per-chunk STATE only; the wrapped tree lives
/// in the shared `SimTemplate` (P2.12 tiles by region: the template is built
/// once per RandomState and instantiated per chunk).
pub struct NoiseChunkSim<'a> {
    template: &'a SimTemplate,
    pub root_fields: Vec<usize>,
    /// P5.3 increment 3 — the per-chunk Beardifier instance (Java:
    /// NoiseChunk.forChunk receives Beardifier.forStructuresInChunk(manager,
    /// chunkPos); NoiseChunk.wrap CFR 372-373 replaces the BeardifierMarker
    /// node with it). EMPTY (zero) until the feed is wired — zero behavior
    /// change by default.
    beard: crate::beardifier::Beardifier,
    interpolators: Vec<InterpState>,
    cell_caches: Vec<CellCacheState>,
    cache2ds: Vec<Cache2DState>,
    cacheonces: Vec<CacheOnceState>,
    flat_caches: Vec<FlatCacheState>,
    pub cell_width: i32,
    pub cell_height: i32,
    pub cell_count_xz: i32,
    pub cell_count_y: i32,
    pub cell_noise_min_y: i32,
    first_cell_x: i32,
    first_cell_z: i32,
    first_noise_x: i32,
    first_noise_z: i32,
    noise_size_xz: i32,
    interpolating: bool,
    filling_cell: bool,
    cell_start_block_x: i32,
    cell_start_block_y: i32,
    cell_start_block_z: i32,
    in_cell_x: i32,
    in_cell_y: i32,
    in_cell_z: i32,
    array_index: usize,
    interpolation_counter: u64,
    array_interpolation_counter: u64,
    bank: &'a NoiseBank,
    /// task 5: NoiseChunk's blockStateRule substance = cacheAllInCell(add(
    /// finalDensity, BeardifierMarker)) — a synthetic cell cache over the
    /// wrapped final_density root, filled in selectCellYZ like Java's
    /// cellCaches list (empty beardifier: values = final per index).
    substance_cache: Vec<f64>,
    /// P2.12 cross-chunk tile cache (None = disabled).
    tile: Option<&'a crate::tile::TileCache>,
    tile_epoch: u64,
}

/// P2.12: the chunk-INDEPENDENT part of the machine — interned arena, wrapped
/// tree, y-free flags, structural hashes, spline table, wrapper-id layout.
/// Built once per RandomState (the tree depends only on (spec, seed)); each
/// chunk instantiates only mutable state (slices/caches/counters).
pub struct SimTemplate {
    pub wnodes: Vec<WNode>,
    pub node_flags: Vec<u8>,
    pub subtree_hash: Vec<u64>,
    pub root_fields: Vec<usize>,
    pub wsplines: Vec<WSpline>,
    pub interp_inners: Vec<usize>,
    pub cell_inners: Vec<usize>,
    pub c2d_inners: Vec<usize>,
    pub c1ce_inners: Vec<usize>,
    pub flat_inners: Vec<usize>,
    pub cell_volume: usize,
}

impl SimTemplate {
    /// Build the template from a bank + 15 interned roots (same order as
    /// NoiseChunkSim::new took `ifields`). Called from RandomState::build
    /// (router.rs), which interns its 15 wired fields in mapAll order.
    pub fn build(bank: &NoiseBank, ifields: [usize; 15], arena: Arena) -> Self {
        let mut b = TemplateBuilder {
            arena,
            wnodes: Vec::new(),
            wrap_memo: HashMap::new(),
            wsplines: Vec::new(),
            wspline_memo: HashMap::new(),
            node_flags: Vec::new(),
            subtree_hash: Vec::new(),
            interp_inners: Vec::new(),
            cell_inners: Vec::new(),
            c2d_inners: Vec::new(),
            c1ce_inners: Vec::new(),
            flat_inners: Vec::new(),
        };
        let mut dedup: HashMap<(MarkerType, usize), WKind> = HashMap::new();
        let mut root_fields = Vec::with_capacity(15);
        for &f in &ifields {
            let w = b.wrap(f, &mut dedup);
            root_fields.push(w);
        }
        SimTemplate {
            wnodes: b.wnodes,
            node_flags: b.node_flags,
            subtree_hash: b.subtree_hash,
            root_fields,
            wsplines: b.wsplines,
            interp_inners: b.interp_inners,
            cell_inners: b.cell_inners,
            c2d_inners: b.c2d_inners,
            c1ce_inners: b.c1ce_inners,
            flat_inners: b.flat_inners,
            // overworld cell volume; instantiate() resizes per-chunk anyway
            cell_volume: 4 * 4 * 8,
        }
    }
}

/// Construction-time builder: the old NoiseChunkSim::new + wrap machinery.
struct TemplateBuilder {
    arena: Arena,
    wnodes: Vec<WNode>,
    wrap_memo: HashMap<usize, usize>,
    wsplines: Vec<WSpline>,
    wspline_memo: HashMap<usize, usize>,
    node_flags: Vec<u8>,
    subtree_hash: Vec<u64>,
    interp_inners: Vec<usize>,
    cell_inners: Vec<usize>,
    c2d_inners: Vec<usize>,
    c1ce_inners: Vec<usize>,
    flat_inners: Vec<usize>,
}


impl<'a> NoiseChunkSim<'a> {
    /// Build from an owned bank + the 15 unwired roots. `first_block_x/z` =
    /// the chunk's min block coords; min_y/height from the (clamped) noise
    /// settings; noise_size_horizontal/vertical in QUARTS (overworld: 1, 2
    /// -> cellWidth 4, cellHeight 8).
    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        template: &'a SimTemplate,
        bank: &'a NoiseBank,
        cell_count_xz: i32,
        first_block_x: i32,
        first_block_z: i32,
        min_y: i32,
        height: i32,
        noise_size_horizontal: i32,
        noise_size_vertical: i32,
        tile: Option<&'a crate::tile::TileCache>,
        tile_epoch: u64,
    ) -> Self {
        let cell_width = noise_size_horizontal * 4; // QuartPos.toBlock
        let cell_height = noise_size_vertical * 4;
        let cell_count_y = height.div_euclid(cell_height); // Mth.floorDiv
        let cell_noise_min_y = min_y.div_euclid(cell_height);
        let first_cell_x = first_block_x.div_euclid(cell_width);
        let first_cell_z = first_block_z.div_euclid(cell_width);
        let first_noise_x = first_block_x.div_euclid(4); // QuartPos.fromBlock
        let first_noise_z = first_block_z.div_euclid(4);
        let noise_size_xz = (cell_count_xz * cell_width).div_euclid(4);

        // Per-chunk STATE ONLY — the wrapped tree (wnodes/flags/hashes/
        // splines/wrapper ids) is shared from the template (P2.12).
        let rows = (cell_count_xz + 1) as usize;
        let cols = (cell_count_y + 1) as usize;
        let mut sim = NoiseChunkSim {
            template,
            root_fields: template.root_fields.clone(),
            interpolators: template
                .interp_inners
                .iter()
                .map(|&inner| InterpState {
                    inner,
                    slice0: vec![vec![0.0; cols]; rows],
                    slice1: vec![vec![0.0; cols]; rows],
                    noise: [0.0; 8],
                    value_xz00: 0.0,
                    value_xz10: 0.0,
                    value_xz01: 0.0,
                    value_xz11: 0.0,
                    value_z0: 0.0,
                    value_z1: 0.0,
                    value: 0.0,
                })
                .collect(),
            cell_caches: template
                .cell_inners
                .iter()
                .map(|&inner| CellCacheState { inner, values: vec![0.0; template.cell_volume] })
                .collect(),
            cache2ds: template
                .c2d_inners
                .iter()
                .map(|&inner| Cache2DState {
                    inner,
                    // ChunkPos.INVALID_CHUNK_POS sentinel
                    last_pos2d: chunk_as_long(i32::MIN + 1, i32::MIN + 1),
                    last_value: 0.0,
                })
                .collect(),
            cacheonces: template
                .c1ce_inners
                .iter()
                .map(|&inner| CacheOnceState {
                    inner,
                    last_counter: 0,
                    last_array_counter: 0,
                    last_value: 0.0,
                    last_array: None,
                })
                .collect(),
            flat_caches: template
                .flat_inners
                .iter()
                .map(|&inner| FlatCacheState { inner, values: Vec::new(), size_xz: 0 })
                .collect(),
            cell_width,
            cell_height,
            cell_count_xz,
            cell_count_y,
            cell_noise_min_y,
            first_cell_x,
            first_cell_z,
            first_noise_x,
            first_noise_z,
            noise_size_xz,
            interpolating: false,
            filling_cell: false,
            cell_start_block_x: 0,
            cell_start_block_y: 0,
            cell_start_block_z: 0,
            in_cell_x: 0,
            in_cell_y: 0,
            in_cell_z: 0,
            array_index: 0,
            interpolation_counter: 0,
            array_interpolation_counter: 0,
            bank,
            beard: crate::beardifier::Beardifier::empty(),
            substance_cache: vec![0.0; (cell_width * cell_width * cell_height) as usize],
            tile,
            tile_epoch,
        };
        // FlatCache eager priming (computeValues = true) — PER-CHUNK because
        // the quart grid positions are chunk-relative. Values are identical
        // to the wrap-time priming this replaces (same compute order, same
        // pure subtrees); wrapper states that Java would not have had during
        // wrap-time priming only receive extra WRITES of identical values —
        // their outputs stay bit-identical (private state, pure reads).
        let size_xz = (noise_size_xz + 1) as usize;
        for id in 0..sim.flat_caches.len() {
            let inner = sim.flat_caches[id].inner;
            let mut values = vec![0.0; size_xz * size_xz];
            for i in 0..=noise_size_xz {
                let bx = (sim.first_noise_x + i) * 4; // QuartPos.toBlock
                for i2 in 0..=noise_size_xz {
                    let bz = (sim.first_noise_z + i2) * 4;
                    let ctx = Ctx { x: bx, y: 0, z: bz, in_chunk: false };
                    values[i as usize + i2 as usize * size_xz] = sim.compute(inner, ctx);
                }
            }
            sim.flat_caches[id].values = values;
            sim.flat_caches[id].size_xz = size_xz;
        }
        sim
    }

}

impl TemplateBuilder {
    /// NoiseChunk.wrapNew — bottom-up (mapAll maps children first).
    fn wrap(&mut self, inode: usize, dedup: &mut HashMap<(MarkerType, usize), WKind>) -> usize {
        // Wrap memo: one wrapped node per distinct interned (== structurally
        // equal) subtree. This is what makes wrapper SHARING match Java's
        // `wrapped` HashMap (record equality): the same cache_once/flat_cache/
        // interpolated marker reached through different fields shares ONE
        // stateful wrapper. Without the memo each re-visit rebuilt the subtree
        // and forked the wrapper state.
        if let Some(&w) = self.wrap_memo.get(&inode) {
            return w;
        }
        thread_local! { static WDEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
        let d = WDEPTH.with(|c| c.get());
        if d > 100000 {
            panic!("wrap recursion depth {} at inode={} node={:?}", d, inode, self.arena.nodes[inode]);
        }
        WDEPTH.with(|c| c.set(d + 1));
        struct GW;
        impl Drop for GW { fn drop(&mut self) { WDEPTH.with(|c| c.set(c.get() - 1)); } }
        let _gw = GW;
        let w = match self.arena.nodes[inode].clone() {
            INode::Const(v) => WNode::Const(v),
            INode::YClampedGradient { from_y, to_y, from_value, to_value } => {
                WNode::YClampedGradient { from_y, to_y, from_value, to_value }
            }
            INode::Noise(a, b, c) => WNode::Noise(a, b, c),
            INode::ShiftedNoise { sx, sy, sz, xz, ys, noise } => WNode::ShiftedNoise {
                sx: self.wrap(sx, dedup),
                sy: self.wrap(sy, dedup),
                sz: self.wrap(sz, dedup),
                xz,
                ys,
                noise,
            },
            // ShiftA/ShiftB/Shift/Blended carry NOISE BANK indices, not child
            // nodes — no wrap recursion through them.
            INode::ShiftA(a) => WNode::ShiftA(a),
            INode::ShiftB(a) => WNode::ShiftB(a),
            INode::Shift(a) => WNode::Shift(a),
            INode::BlendDensity(a) => WNode::BlendDensity(self.wrap(a, dedup)),
            INode::WeirdScaledSampler { input, noise, rarity } => WNode::WeirdScaledSampler {
                input: self.wrap(input, dedup),
                noise,
                rarity,
            },
            INode::RangeChoice { input, min, max, in_range, out_of_range } => WNode::RangeChoice {
                input: self.wrap(input, dedup),
                min,
                max,
                in_range: self.wrap(in_range, dedup),
                out_of_range: self.wrap(out_of_range, dedup),
            },
            INode::Clamp { input, min, max } => WNode::Clamp { input: self.wrap(input, dedup), min, max },
            INode::Mapped { ty, input } => WNode::Mapped { ty, input: self.wrap(input, dedup) },
            INode::MulOrAdd { is_add, input, argument } => {
                WNode::MulOrAdd { is_add, input: self.wrap(input, dedup), argument }
            }
            INode::Ap2 { ty, a1, a2, a2_min, a2_max } => WNode::Ap2 {
                ty,
                a1: self.wrap(a1, dedup),
                a2: self.wrap(a2, dedup),
                a2_min,
                a2_max,
            },
            INode::Spline(s) => WNode::Spline(self.wrap_spline(s, dedup)),
            INode::Blended(a) => WNode::Blended(a),
            INode::BlendAlpha => WNode::BlendAlpha, // Blender.empty -> not replaced
            INode::BlendOffset => WNode::BlendOffset,
            INode::Beardifier => WNode::Beardifier, // BeardifierMarker -> zero marker
            INode::EndIslands => WNode::EndIslands,
            INode::FindTopSurface { density, upper, lower_bound, cell_height } => WNode::FindTopSurface {
                density: self.wrap(density, dedup),
                upper: self.wrap(upper, dedup),
                lower_bound,
                cell_height,
            },
            INode::Marker { ty, wrapped } => {
                let inner = self.wrap(wrapped, dedup);
                // dedup by (type, mapped inner) — Java record equality
                if let Some(kind) = dedup.get(&(ty, inner)) {
                    WNode::W(*kind)
                } else {
                    // Template build records ONLY the wrapper id + inner —
                    // the per-chunk state is allocated in instantiate().
                    let kind = match ty {
                        MarkerType::Interpolated => {
                            let id = self.interp_inners.len();
                            self.interp_inners.push(inner);
                            WKind::Interp(id)
                        }
                        MarkerType::FlatCache => {
                            let id = self.flat_inners.len();
                            self.flat_inners.push(inner);
                            WKind::FlatCacheW(id)
                        }
                        MarkerType::Cache2D => {
                            let id = self.c2d_inners.len();
                            self.c2d_inners.push(inner);
                            WKind::Cache2DW(id)
                        }
                        MarkerType::CacheOnce => {
                            let id = self.c1ce_inners.len();
                            self.c1ce_inners.push(inner);
                            WKind::CacheOnceW(id)
                        }
                        MarkerType::CacheAllInCell => {
                            let id = self.cell_inners.len();
                            self.cell_inners.push(inner);
                            WKind::CellCacheW(id)
                        }
                    };
                    dedup.insert((ty, inner), kind);
                    WNode::W(kind)
                }
            }
        };
        // P2.12: bottom-up flags + structural hash (children already pushed;
        // classify/hash BEFORE the move into wnodes).
        let flags = self.classify_node(&w);
        let shash = self.hash_node(&w);
        self.wnodes.push(w);
        let widx = self.wnodes.len() - 1;
        self.node_flags.push(flags);
        self.subtree_hash.push(shash);
        self.wrap_memo.insert(inode, widx);
        widx
    }

    /// P2.12 y-free classification (conservative — any doubt => y-dependent).
    /// Value-level: does f(x, y, z) depend on y? Mirrors the W-machine's
    /// dispatch: pure-math nodes recurse through children; noise leaves with
    /// a y term (Noise/Shift/ShiftedNoise/WeirdScaled/Blended/YClampedGradient/
    /// Spline/FindTopSurface/EndIslands) are y-dependent; cache wrappers are
    /// y-free iff their inner subtree is (their state is private — replacing
    /// their outputs with a pure memo changes no observable value).
    fn classify_node(&self, w: &WNode) -> u8 {
        let yfree_of = |idx: usize| self.node_flags[idx] != 0;
        let yf = match w {
            // Beardifier is Y-DEPENDENT once real (the kernel index is
            // (y+12) in [0,24)); the pre-wiring zero leaf was y-free and
            // the (true, false) was sound only while it computed 0.0.
            WNode::Beardifier => (false, false),
            WNode::Const(_) | WNode::BlendAlpha | WNode::BlendOffset => (true, false),
            WNode::ShiftA(_) | WNode::ShiftB(_) => (true, true),
            WNode::YClampedGradient { .. }
            | WNode::Noise(..)
            | WNode::Shift(_)
            | WNode::ShiftedNoise { .. }
            | WNode::WeirdScaledSampler { .. }
            | WNode::Spline(_)
            | WNode::Blended(_)
            | WNode::EndIslands
            | WNode::FindTopSurface { .. } => (false, false),
            WNode::BlendDensity(i) => (yfree_of(*i), self.node_flags[*i] == 2),
            WNode::Mapped { input: i, .. } => (yfree_of(*i), self.node_flags[*i] == 2),
            WNode::MulOrAdd { input: i, .. } => (yfree_of(*i), self.node_flags[*i] == 2),
            WNode::Clamp { input: i, .. } => (yfree_of(*i), self.node_flags[*i] == 2),
            WNode::RangeChoice { input, in_range, out_of_range, .. } => (
                yfree_of(*input) && yfree_of(*in_range) && yfree_of(*out_of_range),
                self.node_flags[*input] == 2 && self.node_flags[*in_range] == 2 && self.node_flags[*out_of_range] == 2,
            ),
            WNode::Ap2 { a1, a2, .. } => (
                yfree_of(*a1) && yfree_of(*a2),
                self.node_flags[*a1] == 2 && self.node_flags[*a2] == 2,
            ),
            WNode::W(kind) => match kind {
                WKind::Interp(_) | WKind::CellCacheW(_) => (false, false),
                WKind::FlatCacheW(id) => {
                    let inner = self.flat_inners[*id];
                    // the eager table is y=0-primed per quart; the miss path
                    // (out-of-range) evaluates inner at the CURRENT y — so the
                    // node is value-y-free only if the inner subtree is.
                    (yfree_of(inner), self.node_flags[inner] == 2)
                }
                WKind::Cache2DW(id) => {
                    let inner = self.c2d_inners[*id];
                    (yfree_of(inner), self.node_flags[inner] == 2)
                }
                WKind::CacheOnceW(id) => {
                    let inner = self.c1ce_inners[*id];
                    (yfree_of(inner), self.node_flags[inner] == 2)
                }
            },
        };
        match yf {
            (false, _) => 0,
            (true, false) => 1,
            (true, true) => 2,
        }
    }

    /// P2.12 structural hash: discriminant + fields + child hashes. Two
    /// wrapped nodes with the same hash within one RandomState compute the
    /// same pure function (the Arena already relies on this family of hashes
    /// for interning correctness).
    fn hash_node(&self, w: &WNode) -> u64 {
        let child = |idx: usize| self.subtree_hash[idx];
        let mut h: u64 = 0xcbf29ce484222325;
        let mut mix = |v: u64| {
            h ^= v;
            h = h.wrapping_mul(0x100000001b3);
        };
        match w {
            WNode::Const(v) => {
                mix(1);
                mix(*v);
            }
            WNode::YClampedGradient { from_y, to_y, from_value, to_value } => {
                mix(2);
                mix(*from_y as u64);
                mix(*to_y as u64);
                mix(*from_value);
                mix(*to_value);
            }
            WNode::Noise(a, b, c) => {
                mix(3);
                mix(*a as u64);
                mix(*b);
                mix(*c);
            }
            WNode::ShiftedNoise { sx, sy, sz, xz, ys, noise } => {
                mix(4);
                mix(child(*sx));
                mix(child(*sy));
                mix(child(*sz));
                mix(*xz);
                mix(*ys);
                mix(*noise as u64);
            }
            WNode::ShiftA(a) => {
                mix(5);
                mix(*a as u64);
            }
            WNode::ShiftB(a) => {
                mix(6);
                mix(*a as u64);
            }
            WNode::Shift(a) => {
                mix(7);
                mix(*a as u64);
            }
            WNode::BlendDensity(i) => {
                mix(8);
                mix(child(*i));
            }
            WNode::WeirdScaledSampler { input, noise, rarity } => {
                mix(9);
                mix(child(*input));
                mix(*noise as u64);
                mix(rarity.max_rarity().to_bits());
            }
            WNode::RangeChoice { input, min, max, in_range, out_of_range } => {
                mix(10);
                mix(child(*input));
                mix(*min);
                mix(*max);
                mix(child(*in_range));
                mix(child(*out_of_range));
            }
            WNode::Clamp { input, min, max } => {
                mix(11);
                mix(child(*input));
                mix(*min);
                mix(*max);
            }
            WNode::Mapped { ty, input } => {
                mix(12 + *ty as u64);
                mix(child(*input));
            }
            WNode::MulOrAdd { is_add, input, argument } => {
                mix(if *is_add { 30 } else { 31 });
                mix(child(*input));
                mix(*argument);
            }
            WNode::Ap2 { ty, a1, a2, .. } => {
                mix(13 + *ty as u64);
                mix(child(*a1));
                mix(child(*a2));
            }
            WNode::Spline(s) => {
                mix(20);
                let sp = &self.wsplines[*s];
                mix(child(sp.coordinate));
                for l in &sp.locations {
                    mix(l.to_bits() as u64);
                }
                for d in &sp.derivatives {
                    mix(d.to_bits() as u64);
                }
                for v in &sp.values {
                    match v {
                        WSplineValue::Const(c) => mix(c.to_bits() as u64 | 0x8000_0000_0000_0000),
                        WSplineValue::Multi(m) => mix(self.subtree_hash_of_spline(*m)),
                    }
                }
            }
            WNode::Blended(a) => {
                mix(21);
                mix(*a as u64);
            }
            WNode::BlendAlpha => mix(22),
            WNode::BlendOffset => mix(23),
            WNode::Beardifier => mix(24),
            WNode::EndIslands => mix(25),
            WNode::FindTopSurface { density, upper, lower_bound, cell_height } => {
                mix(26);
                mix(child(*density));
                mix(child(*upper));
                mix(*lower_bound as u64);
                mix(*cell_height as u64);
            }
            WNode::W(kind) => match kind {
                WKind::Interp(id) => {
                    mix(40);
                    mix(child(self.interp_inners[*id]));
                }
                WKind::FlatCacheW(id) => {
                    mix(41);
                    mix(child(self.flat_inners[*id]));
                }
                WKind::Cache2DW(id) => {
                    mix(42);
                    mix(child(self.c2d_inners[*id]));
                }
                WKind::CacheOnceW(id) => {
                    mix(43);
                    mix(child(self.c1ce_inners[*id]));
                }
                WKind::CellCacheW(id) => {
                    mix(44);
                    mix(child(self.cell_inners[*id]));
                }
            },
        }
        h
    }

    fn subtree_hash_of_spline(&self, s: usize) -> u64 {
        // spline nodes carry their hash in subtree_hash only for W indices;
        // WSplineValue::Multi children are W-spline indices — reuse the
        // coordinate/locations/values structure hash.
        let sp = &self.wsplines[s];
        let mut h: u64 = 0x9e3779b97f4a7c15;
        let mut mix = |v: u64| {
            h ^= v;
            h = h.wrapping_mul(0x100000001b3);
        };
        mix(self.subtree_hash[sp.coordinate]);
        for l in &sp.locations {
            mix(l.to_bits() as u64);
        }
        for d in &sp.derivatives {
            mix(d.to_bits() as u64);
        }
        for v in &sp.values {
            match v {
                WSplineValue::Const(c) => mix(c.to_bits() as u64 | 0x8000_0000_0000_0000),
                WSplineValue::Multi(m) => mix(self.subtree_hash_of_spline(*m)),
            }
        }
        h
    }

    /// Wrap a spline: its coordinate subtree and Multi children must be
    /// translated from interned-I indices to W indices (memoized).
    fn wrap_spline(&mut self, s: usize, dedup: &mut HashMap<(MarkerType, usize), WKind>) -> usize {
        if let Some(&ws) = self.wspline_memo.get(&s) {
            return ws;
        }
        let is = self.arena.splines[s].clone();
        let coordinate = self.wrap(is.coordinate, dedup);
        let values = is
            .values
            .iter()
            .map(|v| match v {
                ISplineValue::Const(c) => WSplineValue::Const(*c),
                ISplineValue::Multi(i) => WSplineValue::Multi(self.wrap_spline(*i, dedup)),
            })
            .collect();
        self.wsplines.push(WSpline {
            coordinate,
            locations: is.locations,
            values,
            derivatives: is.derivatives,
            min: is.min,
            max: is.max,
        });
        let ws = self.wsplines.len() - 1;
        self.wspline_memo.insert(s, ws);
        ws
    }
}

impl<'a> NoiseChunkSim<'a> {
    // ------------------------------------------------------------------
    // compute
    // ------------------------------------------------------------------

    fn pos_now(&self) -> Ctx {
        Ctx {
            x: self.cell_start_block_x + self.in_cell_x,
            y: self.cell_start_block_y + self.in_cell_y,
            z: self.cell_start_block_z + self.in_cell_z,
            in_chunk: true,
        }
    }

    fn compute(&mut self, w: usize, ctx: Ctx) -> f64 {
        // P2.12: y-free subtree tile memo — a node classified cacheable is a
        // pure f(spec, seed, world x, z); memoizing returns bit-identical f64
        // (values stored raw, no rounding). See tile.rs soundness contract.
        if self.template.node_flags[w] == 2 && self.tile.is_some() {
            let key = crate::tile::tile_key(self.template.subtree_hash[w], ctx.x, ctx.z, self.tile_epoch);
            // SAFETY of the unwrap: checked is_some above
            let tile = self.tile.unwrap();
            if let Some(v) = tile.get(key) {
                return v;
            }
            let v = self.compute_body(w, ctx);
            tile.put(key, v);
            return v;
        }
        self.compute_body(w, ctx)
    }

    fn compute_body(&mut self, w: usize, ctx: Ctx) -> f64 {
        thread_local! { static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
        let d = DEPTH.with(|c| c.get());
        if d > 100000 {
            panic!("compute recursion depth {} at w={} node={:?}", d, w, self.template.wnodes[w]);
        }
        DEPTH.with(|c| c.set(d + 1));
        struct G;
        impl Drop for G { fn drop(&mut self) { DEPTH.with(|c| c.set(c.get() - 1)); } }
        let _g = G;
        let node = self.template.wnodes[w].clone();
        match node {
            WNode::Const(v) => f64::from_bits(v),
            WNode::YClampedGradient { from_y, to_y, from_value, to_value } => mth::clamped_map(
                ctx.y as f64,
                from_y as f64,
                to_y as f64,
                f64::from_bits(from_value),
                f64::from_bits(to_value),
            ),
            WNode::Noise(idx, xz, ys) => self.bank.noises[idx].get_value(
                ctx.x as f64 * f64::from_bits(xz),
                ctx.y as f64 * f64::from_bits(ys),
                ctx.z as f64 * f64::from_bits(xz),
            ),
            WNode::ShiftedNoise { sx, sy, sz, xz, ys, noise } => {
                let d = ctx.x as f64 * f64::from_bits(xz) + self.compute(sx, ctx);
                let d1 = ctx.y as f64 * f64::from_bits(ys) + self.compute(sy, ctx);
                let d2 = ctx.z as f64 * f64::from_bits(xz) + self.compute(sz, ctx);
                self.bank.noises[noise].get_value(d, d1, d2)
            }
            WNode::ShiftA(idx) => {
                self.bank.noises[idx].get_value(ctx.x as f64 * 0.25, 0.0, ctx.z as f64 * 0.25) * 4.0
            }
            WNode::ShiftB(idx) => {
                self.bank.noises[idx].get_value(ctx.z as f64 * 0.25, ctx.x as f64 * 0.25, 0.0) * 4.0
            }
            WNode::Shift(idx) => {
                self.bank.noises[idx].get_value(ctx.x as f64 * 0.25, ctx.y as f64 * 0.25, ctx.z as f64 * 0.25) * 4.0
            }
            WNode::BlendDensity(i) => self.compute(i, ctx), // Blender.empty identity
            WNode::WeirdScaledSampler { input, noise, rarity } => {
                let value = self.compute(input, ctx);
                let d = rarity.map(value);
                d * self.bank.noises[noise].get_value(ctx.x as f64 / d, ctx.y as f64 / d, ctx.z as f64 / d).abs()
            }
            WNode::RangeChoice { input, min, max, in_range, out_of_range } => {
                let d = self.compute(input, ctx);
                if d >= f64::from_bits(min) && d < f64::from_bits(max) {
                    self.compute(in_range, ctx)
                } else {
                    self.compute(out_of_range, ctx)
                }
            }
            WNode::Clamp { input, min, max } => {
                mth::clamp(self.compute(input, ctx), f64::from_bits(min), f64::from_bits(max))
            }
            WNode::Mapped { ty, input } => {
                let v = self.compute(input, ctx);
                self.mapped_transform(ty, v)
            }
            WNode::MulOrAdd { is_add, input, argument } => {
                let v = self.compute(input, ctx);
                if is_add {
                    v + f64::from_bits(argument)
                } else {
                    v * f64::from_bits(argument)
                }
            }
            WNode::Ap2 { ty, a1, a2, a2_min, a2_max } => {
                let d = self.compute(a1, ctx);
                match ty {
                    Ap2Type::Add => d + self.compute(a2, ctx),
                    Ap2Type::Mul => {
                        if d == 0.0 {
                            0.0
                        } else {
                            d * self.compute(a2, ctx)
                        }
                    }
                    Ap2Type::Min => {
                        if d < f64::from_bits(a2_min) {
                            d
                        } else {
                            mth::java_min(d, self.compute(a2, ctx))
                        }
                    }
                    Ap2Type::Max => {
                        if d > f64::from_bits(a2_max) {
                            d
                        } else {
                            mth::java_max(d, self.compute(a2, ctx))
                        }
                    }
                }
            }
            WNode::Spline(s) => self.spline_apply(s, ctx) as f64,
            WNode::Blended(idx) => self.bank.blended[idx].compute(ctx.x, ctx.y, ctx.z),
            WNode::BlendAlpha => 1.0,
            WNode::BlendOffset => 0.0,
            // BeardifierMarker -> the per-chunk Beardifier (NoiseChunk.wrap
            // CFR 372-373). EMPTY instance computes exactly 0.0 — identical
            // to the pre-wiring marker semantics.
            WNode::Beardifier => self.beard.compute(ctx.x, ctx.y, ctx.z),
            WNode::EndIslands => panic!("EndIslands scalar eval not implemented yet (Phase 2 tail)"),
            WNode::FindTopSurface { density, upper, lower_bound, cell_height } => {
                let i = mth::floor(self.compute(upper, ctx) / cell_height as f64) * cell_height;
                if i <= lower_bound {
                    return lower_bound as f64;
                }
                let mut i1 = i;
                while i1 >= lower_bound {
                    if self.compute(density, Ctx { x: ctx.x, y: i1, z: ctx.z, in_chunk: ctx.in_chunk }) > 0.0 {
                        return i1 as f64;
                    }
                    i1 -= cell_height;
                }
                lower_bound as f64
            }
            WNode::W(kind) => match kind {
                WKind::Interp(id) => {
                    if !ctx.in_chunk {
                        let inner = self.interpolators[id].inner;
                        return self.compute(inner, ctx);
                    }
                    if !self.interpolating {
                        panic!("Trying to sample interpolator outside the interpolation loop");
                    }
                    if self.filling_cell {
                        let n = self.interpolators[id].noise; // [000,001,100,101,010,011,110,111]
                        mth::lerp3(
                            self.in_cell_x as f64 / self.cell_width as f64,
                            self.in_cell_y as f64 / self.cell_height as f64,
                            self.in_cell_z as f64 / self.cell_width as f64,
                            n[0], n[2], n[4], n[6], n[1], n[3], n[5], n[7],
                        )
                    } else {
                        self.interpolators[id].value
                    }
                }
                WKind::FlatCacheW(id) => {
                    let (qx, qz) = (ctx.x.div_euclid(4), ctx.z.div_euclid(4));
                    let i = qx - self.first_noise_x;
                    let i1 = qz - self.first_noise_z;
                    let size_xz = self.flat_caches[id].size_xz;
                    if i >= 0 && i1 >= 0 && (i as usize) < size_xz && (i1 as usize) < size_xz {
                        self.flat_caches[id].values[i as usize + i1 as usize * size_xz]
                    } else {
                        let inner = self.flat_caches[id].inner;
                        self.compute(inner, ctx)
                    }
                }
                WKind::Cache2DW(id) => {
                    let packed = chunk_as_long(ctx.x, ctx.z);
                    if self.cache2ds[id].last_pos2d == packed {
                        self.cache2ds[id].last_value
                    } else {
                        let inner = self.cache2ds[id].inner;
                        let v = self.compute(inner, ctx);
                        let st = &mut self.cache2ds[id];
                        st.last_pos2d = packed;
                        st.last_value = v;
                        v
                    }
                }
                WKind::CacheOnceW(id) => {
                    if !ctx.in_chunk {
                        let inner = self.cacheonces[id].inner;
                        return self.compute(inner, ctx);
                    }
                    let hit_array = self.cacheonces[id]
                        .last_array
                        .as_ref()
                        .map(|_| self.cacheonces[id].last_array_counter == self.array_interpolation_counter)
                        .unwrap_or(false);
                    if hit_array {
                        return self.cacheonces[id].last_array.as_ref().unwrap()[self.array_index];
                    }
                    if self.cacheonces[id].last_counter == self.interpolation_counter {
                        return self.cacheonces[id].last_value;
                    }
                    let inner = self.cacheonces[id].inner;
                    let v = self.compute(inner, ctx);
                    let st = &mut self.cacheonces[id];
                    st.last_counter = self.interpolation_counter;
                    st.last_value = v;
                    v
                }
                WKind::CellCacheW(id) => {
                    if !ctx.in_chunk {
                        let inner = self.cell_caches[id].inner;
                        return self.compute(inner, ctx);
                    }
                    if !self.interpolating {
                        panic!("Trying to sample interpolator outside the interpolation loop");
                    }
                    let (i, i1, i2) = (self.in_cell_x, self.in_cell_y, self.in_cell_z);
                    let (cw, ch) = (self.cell_width, self.cell_height);
                    if i >= 0 && i1 >= 0 && i2 >= 0 && i < cw && i1 < ch && i2 < cw {
                        let slot = (((ch - 1 - i1) * cw + i) * cw + i2) as usize;
                        self.cell_caches[id].values[slot]
                    } else {
                        let inner = self.cell_caches[id].inner;
                        self.compute(inner, ctx)
                    }
                }
            },
        }
    }

    fn mapped_transform(&self, ty: MappedType, value: f64) -> f64 {
        match ty {
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

    fn spline_apply(&mut self, s: usize, ctx: Ctx) -> f32 {
        let (coordinate, locations_len, locations, derivatives, values) = {
            let sp = &self.template.wsplines[s];
            (sp.coordinate, sp.locations.len(), sp.locations.clone(), sp.derivatives.clone(), sp.values.clone())
        };
        let f = self.compute(coordinate, ctx) as f32;
        let i = crate::density::find_interval_start(&locations, f);
        let i1 = (locations_len - 1) as i32;
        if i < 0 {
            let value = self.spline_value(&values[0], ctx);
            return crate::density::linear_extend(f, &locations, value, &derivatives, 0);
        }
        if i == i1 {
            let value = self.spline_value(&values[i1 as usize], ctx);
            return crate::density::linear_extend(f, &locations, value, &derivatives, i1 as usize);
        }
        let f1 = locations[i as usize];
        let f2 = locations[i as usize + 1];
        let f3 = (f - f1) / (f2 - f1);
        let f6 = self.spline_value(&values[i as usize], ctx);
        let f7 = self.spline_value(&values[i as usize + 1], ctx);
        let f4 = derivatives[i as usize];
        let f5 = derivatives[i as usize + 1];
        let f8 = f4 * (f2 - f1) - (f7 - f6);
        let f9 = -f5 * (f2 - f1) + (f7 - f6);
        let l1 = f6 + f3 * (f7 - f6);
        let l2 = f8 + f3 * (f9 - f8);
        l1 + f3 * (1.0f32 - f3) * l2
    }

    fn spline_value(&mut self, v: &WSplineValue, ctx: Ctx) -> f32 {
        match v {
            WSplineValue::Const(c) => *c,
            WSplineValue::Multi(i) => self.spline_apply(*i, ctx),
        }
    }

    // ------------------------------------------------------------------
    // fillArray machinery
    // ------------------------------------------------------------------

    fn fill_array(&mut self, w: usize, array: &mut [f64], provider: Provider) {
        thread_local! { static FDEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
        let d = FDEPTH.with(|c| c.get());
        if d > 100000 {
            panic!("fill_array recursion depth {} at w={} node={:?}", d, w, self.template.wnodes[w]);
        }
        FDEPTH.with(|c| c.set(d + 1));
        struct GF;
        impl Drop for GF { fn drop(&mut self) { FDEPTH.with(|c| c.set(c.get() - 1)); } }
        let _gf = GF;
        let node = self.template.wnodes[w].clone();
        match node {
            WNode::Const(v) => {
                array.fill(f64::from_bits(v));
            }
            WNode::Ap2 { ty, a1, a2, a2_min, a2_max } => match ty {
                Ap2Type::Add => {
                    self.fill_array(a1, array, provider);
                    let mut doubles = vec![0.0; array.len()];
                    self.fill_array(a2, &mut doubles, provider);
                    for i in 0..array.len() {
                        array[i] += doubles[i];
                    }
                }
                Ap2Type::Mul => {
                    self.fill_array(a1, array, provider);
                    for i1 in 0..array.len() {
                        let d = array[i1];
                        array[i1] = if d == 0.0 { 0.0 } else { d * self.compute_for_index(a2, i1, provider) };
                    }
                }
                Ap2Type::Min => {
                    let d1 = f64::from_bits(a2_min);
                    self.fill_array(a1, array, provider);
                    for i2 in 0..array.len() {
                        let d2 = array[i2];
                        array[i2] = if d2 < d1 {
                            d2
                        } else {
                            mth::java_min(d2, self.compute_for_index(a2, i2, provider))
                        };
                    }
                }
                Ap2Type::Max => {
                    let d1 = f64::from_bits(a2_max);
                    self.fill_array(a1, array, provider);
                    for i2 in 0..array.len() {
                        let d2 = array[i2];
                        array[i2] = if d2 > d1 {
                            d2
                        } else {
                            mth::java_max(d2, self.compute_for_index(a2, i2, provider))
                        };
                    }
                }
            },
            WNode::Mapped { ty, input } => {
                self.fill_array(input, array, provider);
                for i in 0..array.len() {
                    array[i] = self.mapped_transform(ty, array[i]);
                }
            }
            WNode::RangeChoice { input, min, max, in_range, out_of_range } => {
                self.fill_array(input, array, provider);
                for i in 0..array.len() {
                    let d = array[i];
                    array[i] = if d >= f64::from_bits(min) && d < f64::from_bits(max) {
                        self.compute_for_index(in_range, i, provider)
                    } else {
                        self.compute_for_index(out_of_range, i, provider)
                    };
                }
            }
            WNode::W(WKind::CacheOnceW(id)) => {
                let counter_hit = self.cacheonces[id]
                    .last_array
                    .as_ref()
                    .map(|_| self.cacheonces[id].last_array_counter == self.array_interpolation_counter)
                    .unwrap_or(false);
                if counter_hit {
                    // Java: System.arraycopy(lastArray, 0, array, 0, array.length)
                    // — a length mismatch would throw (vanilla never hits it).
                    let last = self.cacheonces[id].last_array.as_ref().unwrap().clone();
                    assert_eq!(last.len(), array.len(), "CacheOnce lastArray length changed on the read path (Java would throw)");
                    array.copy_from_slice(&last);
                    return;
                }
                let inner = self.cacheonces[id].inner;
                self.fill_array(inner, array, provider);
                let st = &mut self.cacheonces[id];
                let counter_now = self.array_interpolation_counter;
                if let Some(last) = &mut st.last_array {
                    if last.len() == array.len() {
                        last.copy_from_slice(array);
                    } else {
                        st.last_array = Some(array.to_vec());
                    }
                } else {
                    st.last_array = Some(array.to_vec());
                }
                st.last_array_counter = counter_now;
            }
            WNode::W(WKind::Cache2DW(id)) => {
                let inner = self.cache2ds[id].inner;
                self.fill_array(inner, array, provider);
            }
            WNode::W(WKind::Interp(id)) => {
                if self.filling_cell {
                    self.provider_fill_all_directly(w, array, provider);
                } else {
                    let inner = self.interpolators[id].inner;
                    self.fill_array(inner, array, provider);
                }
            }
            WNode::W(WKind::CellCacheW(_)) | WNode::W(WKind::FlatCacheW(_)) => {
                self.provider_fill_all_directly(w, array, provider);
            }
            _ => self.provider_fill_all_directly(w, array, provider),
        }
    }

    /// The ContextProvider.forIndex used by batched fillArray overrides.
    fn compute_for_index(&mut self, w: usize, index: usize, provider: Provider) -> f64 {
        match provider {
            Provider::Slice => {
                self.cell_start_block_y = (index as i32 + self.cell_noise_min_y) * self.cell_height;
                self.interpolation_counter += 1;
                self.in_cell_y = 0;
                self.array_index = index;
            }
            Provider::Cell => {
                // NoiseChunk.forIndex — y DESCENDING mapping
                let cw = self.cell_width;
                let idx = index as i32;
                let i = idx.rem_euclid(cw);
                let i1 = idx.div_euclid(cw);
                let i2 = i1.rem_euclid(cw);
                let i3 = self.cell_height - 1 - i1.div_euclid(cw);
                self.in_cell_x = i2;
                self.in_cell_y = i3;
                self.in_cell_z = i;
                self.array_index = index;
            }
        }
        let ctx = self.pos_now();
        self.compute(w, ctx)
    }

    /// ContextProvider.fillAllDirectly for both providers.
    fn provider_fill_all_directly(&mut self, w: usize, array: &mut [f64], provider: Provider) {
        match provider {
            Provider::Slice => {
                for i in 0..array.len() {
                    self.cell_start_block_y = (i as i32 + self.cell_noise_min_y) * self.cell_height;
                    self.interpolation_counter += 1;
                    self.in_cell_y = 0;
                    self.array_index = i;
                    let ctx = self.pos_now();
                    array[i] = self.compute(w, ctx);
                }
            }
            Provider::Cell => {
                self.array_index = 0;
                let mut iy = self.cell_height - 1;
                while iy >= 0 {
                    self.in_cell_y = iy;
                    for ix in 0..self.cell_width {
                        self.in_cell_x = ix;
                        let mut iz = 0i32;
                        while iz < self.cell_width {
                            self.in_cell_z = iz;
                            iz += 1;
                            let ctx = self.pos_now();
                            array[self.array_index] = self.compute(w, ctx);
                            self.array_index += 1;
                        }
                    }
                    iy -= 1;
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // NoiseInterpolator methods + the drive loop
    // ------------------------------------------------------------------

    fn select_cell_yz(&mut self, y: i32, z: i32) {
        let ids: Vec<usize> = (0..self.interpolators.len()).collect();
        for id in ids {
            // Java order: noise000 = slice0[z][y]; noise001 = slice0[z+1][y];
            // noise100 = slice1[z][y]; noise101 = slice1[z+1][y];
            // noise010 = slice0[z][y+1]; noise011 = slice0[z+1][y+1];
            // noise110 = slice1[z][y+1]; noise111 = slice1[z+1][y+1]
            let n000 = self.interpolators[id].slice0[z as usize][y as usize];
            let n001 = self.interpolators[id].slice0[(z + 1) as usize][y as usize];
            let n100 = self.interpolators[id].slice1[z as usize][y as usize];
            let n101 = self.interpolators[id].slice1[(z + 1) as usize][y as usize];
            let n010 = self.interpolators[id].slice0[z as usize][(y + 1) as usize];
            let n011 = self.interpolators[id].slice0[(z + 1) as usize][(y + 1) as usize];
            let n110 = self.interpolators[id].slice1[z as usize][(y + 1) as usize];
            let n111 = self.interpolators[id].slice1[(z + 1) as usize][(y + 1) as usize];
            self.interpolators[id].noise = [n000, n001, n100, n101, n010, n011, n110, n111];
        }
        self.filling_cell = true;
        self.cell_start_block_y = (y + self.cell_noise_min_y) * self.cell_height;
        self.cell_start_block_z = (self.first_cell_z + z) * self.cell_width;
        self.array_interpolation_counter += 1;
        let cache_ids: Vec<usize> = (0..self.cell_caches.len()).collect();
        for cid in cache_ids {
            // The cache's own field is the array being filled (Java passes
            // cacheAllInCell.values); take/put is safe because a cache's
            // filler can never contain the cache's own wrapper (proper
            // subtree), and reads of OTHER caches see their live fields.
            let mut values = std::mem::take(&mut self.cell_caches[cid].values);
            let filler = self.cell_caches[cid].inner;
            self.fill_array(filler, &mut values, Provider::Cell);
            self.cell_caches[cid].values = values;
        }
        // substance cache (task 5): Java fills EVERY CacheAllInCell in
        // cellCaches here; our substance wrapper sits over the final_density
        // root (empty beardifier -> values = final per forIndex index).
        {
            let mut arr = std::mem::take(&mut self.substance_cache);
            self.fill_array(self.root_fields[11], &mut arr, Provider::Cell);
            self.substance_cache = arr;
        }
        self.array_interpolation_counter += 1;
        self.filling_cell = false;
    }

    /// P5.3 increment 3: install the per-chunk Beardifier (call BEFORE any
    /// drive/drive_blocks — the substance cache fills lazily per cell and
    /// reads the beardifier through the wired tree).
    pub fn set_beardifier(&mut self, beard: crate::beardifier::Beardifier) {
        self.beard = beard;
    }

    /// CacheAllInCell.compute: read the substance value for the CURRENT
    /// in-cell position (slot formula = Java's values[...] index).
    pub fn substance_value(&self) -> f64 {
        let cw = self.cell_width;
        let ch = self.cell_height;
        let (i, i1, i2) = (self.in_cell_x, self.in_cell_y, self.in_cell_z);
        let slot = (((ch - 1 - i1) * cw + i) * cw + i2) as usize;
        self.substance_cache[slot]
    }

    fn update_for_y(&mut self, cell_end_block_y: i32, y: f64) {
        self.in_cell_y = cell_end_block_y - self.cell_start_block_y;
        for st in &mut self.interpolators {
            let n = st.noise; // [000,001,100,101,010,011,110,111]
            st.value_xz00 = mth::lerp(y, n[0], n[4]);
            st.value_xz10 = mth::lerp(y, n[2], n[6]);
            st.value_xz01 = mth::lerp(y, n[1], n[5]);
            st.value_xz11 = mth::lerp(y, n[3], n[7]);
        }
    }

    fn update_for_x(&mut self, cell_end_block_x: i32, x: f64) {
        self.in_cell_x = cell_end_block_x - self.cell_start_block_x;
        for st in &mut self.interpolators {
            let (a, b, c, d) = (st.value_xz00, st.value_xz10, st.value_xz01, st.value_xz11);
            st.value_z0 = mth::lerp(x, a, b);
            st.value_z1 = mth::lerp(x, c, d);
        }
    }

    fn update_for_z(&mut self, cell_end_block_z: i32, z: f64) {
        self.in_cell_z = cell_end_block_z - self.cell_start_block_z;
        self.interpolation_counter += 1;
        for st in &mut self.interpolators {
            st.value = mth::lerp(z, st.value_z0, st.value_z1);
        }
    }

    fn swap_slices(&mut self) {
        for st in &mut self.interpolators {
            std::mem::swap(&mut st.slice0, &mut st.slice1);
        }
    }

    /// fillSlice — isSlice0 selects the target array field (true only for
    /// the initializeForFirstCellX call; every advanceCellX fills slice1 and
    /// the column-end swapSlices alternates the physical buffers).
    fn fill_slice(&mut self, is_slice0: bool, start: i32) {
        self.cell_start_block_x = start * self.cell_width;
        self.in_cell_x = 0;
        for i in 0..(self.cell_count_xz + 1) {
            let i1 = self.first_cell_z + i;
            self.cell_start_block_z = i1 * self.cell_width;
            self.in_cell_z = 0;
            self.array_interpolation_counter += 1;
            let ids: Vec<usize> = (0..self.interpolators.len()).collect();
            for id in ids {
                let mut arr = if is_slice0 {
                    std::mem::take(&mut self.interpolators[id].slice0[i as usize])
                } else {
                    std::mem::take(&mut self.interpolators[id].slice1[i as usize])
                };
                let inner = self.interpolators[id].inner;
                self.fill_array(inner, &mut arr, Provider::Slice);
                if is_slice0 {
                    self.interpolators[id].slice0[i as usize] = arr;
                } else {
                    self.interpolators[id].slice1[i as usize] = arr;
                }
            }
        }
        self.array_interpolation_counter += 1;
    }

    /// Drive the doFill loop and collect per-block values for every
    /// interpolator, in capture order (cx asc, cz asc, cy desc, inY desc,
    /// inX asc, inZ asc, interpolator 0..n). Returns
    /// (interp_count, rows) with rows = (interp, x, y, z, value).
    #[allow(clippy::type_complexity)]
    pub fn drive_and_collect(&mut self) -> (usize, Vec<(u32, i32, i32, i32, f64)>) {
        assert!(!self.interpolating, "Starting interpolation twice");
        self.interpolating = true;
        self.interpolation_counter = 0;
        // initializeForFirstCellX
        self.fill_slice(true, self.first_cell_x);
        let mut out = Vec::new();
        for cx in 0..self.cell_count_xz {
            // advanceCellX(cx)
            self.fill_slice(false, self.first_cell_x + cx + 1);
            self.cell_start_block_x = (self.first_cell_x + cx) * self.cell_width;
            for cz in 0..self.cell_count_xz {
                for cy in (0..self.cell_count_y).rev() {
                    self.select_cell_yz(cy, cz);
                    for in_y in (0..self.cell_height).rev() {
                        let by = (self.cell_noise_min_y + cy) * self.cell_height + in_y;
                        let frac_y = in_y as f64 / self.cell_height as f64;
                        self.update_for_y(by, frac_y);
                        for in_x in 0..self.cell_width {
                            let bx = self.cell_start_block_x + in_x;
                            let frac_x = in_x as f64 / self.cell_width as f64;
                            self.update_for_x(bx, frac_x);
                            for in_z in 0..self.cell_width {
                                let bz = self.cell_start_block_z + in_z;
                                let frac_z = in_z as f64 / self.cell_width as f64;
                                self.update_for_z(bz, frac_z);
                                let ctx = self.pos_now();
                                debug_assert_eq!(ctx.x, bx);
                                debug_assert_eq!(ctx.y, by);
                                debug_assert_eq!(ctx.z, bz);
                                for id in 0..self.interpolators.len() {
                                    let v = self.interpolators[id].value;
                                    out.push((id as u32, bx, by, bz, v));
                                }
                            }
                        }
                    }
                }
            }
            self.swap_slices();
        }
        self.interpolating = false;
        (self.interpolators.len(), out)
    }

    /// doFill driving with a PER-BLOCK callback (task 5, filler.rs): identical
    /// loop to `drive_and_collect`, but instead of collecting values it calls
    /// `f(bx, by, bz, values)` at every block with the per-block interpolated
    /// value of EVERY wrapped root field (root_fields order: barrier,
    /// floodedness, spread, lava, temperature, vegetation, continents,
    /// erosion, depth, ridges, preliminary_surface_level, final_density,
    /// vein_toggle, vein_ridged, vein_gap). swapSlices after each cellX
    /// column, stopInterpolation at the end — verbatim.
    /// Compute the wrapped root field `root` (index into `root_fields`) at the
    /// CURRENT interpolation position — the per-block value the Java
    /// blockStateRule density functions see at this block (task 5 filler).
    pub fn compute_field(&mut self, root: usize) -> f64 {
        let ctx = self.pos_now();
        self.compute(self.root_fields[root], ctx)
    }

    #[allow(clippy::type_complexity)]
    pub fn drive_blocks(&mut self, f: &mut dyn FnMut(i32, i32, i32, &mut Self)) {
        assert!(!self.interpolating, "Starting interpolation twice");
        self.interpolating = true;
        self.interpolation_counter = 0;
        // initializeForFirstCellX
        self.fill_slice(true, self.first_cell_x);
        for cx in 0..self.cell_count_xz {
            // advanceCellX(cx)
            self.fill_slice(false, self.first_cell_x + cx + 1);
            self.cell_start_block_x = (self.first_cell_x + cx) * self.cell_width;
            for cz in 0..self.cell_count_xz {
                for cy in (0..self.cell_count_y).rev() {
                    self.select_cell_yz(cy, cz);
                    for in_y in (0..self.cell_height).rev() {
                        let by = (self.cell_noise_min_y + cy) * self.cell_height + in_y;
                        let frac_y = in_y as f64 / self.cell_height as f64;
                        self.update_for_y(by, frac_y);
                        for in_x in 0..self.cell_width {
                            let bx = self.cell_start_block_x + in_x;
                            let frac_x = in_x as f64 / self.cell_width as f64;
                            self.update_for_x(bx, frac_x);
                            for in_z in 0..self.cell_width {
                                let bz = self.cell_start_block_z + in_z;
                                let frac_z = in_z as f64 / self.cell_width as f64;
                                self.update_for_z(bz, frac_z);
                                f(bx, by, bz, self);
                            }
                        }
                    }
                }
            }
            self.swap_slices();
        }
        self.interpolating = false;
    }

    /// P5.3 increment 2c — the iterateNoiseColumn single-column drive
    /// (NoiseBasedChunkGenerator.iterateNoiseColumn, CFR 157-199: the
    /// getBaseHeight machine behind Structure height sampling). Slice fills
    /// happen ONCE (initializeForFirstCellX + advanceCellX(0), CFR 184-185),
    /// then cells are walked TOP-DOWN (CFR 186) with ONLY the target
    /// column's fracs applied per y: updateForY(i10, d2) / updateForX(x, d)
    /// / updateForZ(z, d1) where d = Math.floorMod(x, cellWidth)/cellWidth
    /// (CFR 175-183) — in_cell_x from the RAW block coords. The callback
    /// signature matches `drive_blocks` — (bx, by, bz, sim) — and returns
    /// true to STOP (stoppingState hit at i10 => return i10 + 1, CFR
    /// 195-198); returning true leaves `interpolating` cleared
    /// (stopInterpolation, CFR 199) and the machine reusable. The machine
    /// must be the 1-cell window instantiated at cellFloor(x/cw)*cw —
    /// exactly Java's iterateNoiseColumn NoiseChunk (CFR 182). Interpolated
    /// values equal the full drive's at the same (x, z): fills are
    /// order-independent and the per-block interpolation state depends only
    /// on (cell fills, current fracs).
    pub fn drive_column(
        &mut self,
        x: i32,
        z: i32,
        f: &mut dyn FnMut(i32, i32, i32, &mut Self) -> bool,
    ) {
        assert!(!self.interpolating, "Starting interpolation twice");
        self.interpolating = true;
        self.interpolation_counter = 0;
        // initializeForFirstCellX + advanceCellX(0) — the two slice fills of
        // the 1-cell machine.
        self.fill_slice(true, self.first_cell_x);
        self.fill_slice(false, self.first_cell_x + 1);
        // advanceCellX leaves cell_start_block_x at the NEXT cell; the column
        // lives in cell 0 (drive_blocks resets it the same way, CFR 187-188).
        self.cell_start_block_x = self.first_cell_x * self.cell_width;
        let base_z = self.first_cell_z * self.cell_width;
        let frac_x = (x - self.cell_start_block_x) as f64 / self.cell_width as f64;
        let frac_z = (z - base_z) as f64 / self.cell_width as f64;
        debug_assert!((0..self.cell_width).contains(&(x - self.cell_start_block_x)));
        debug_assert!((0..self.cell_width).contains(&(z - base_z)));
        for cy in (0..self.cell_count_y).rev() {
            self.select_cell_yz(cy, 0);
            for in_y in (0..self.cell_height).rev() {
                let by = (self.cell_noise_min_y + cy) * self.cell_height + in_y;
                let frac_y = in_y as f64 / self.cell_height as f64;
                self.update_for_y(by, frac_y);
                self.update_for_x(x, frac_x);
                self.update_for_z(z, frac_z);
                if f(x, by, z, self) {
                    self.interpolating = false;
                    return;
                }
            }
        }
        self.interpolating = false;
    }
}

impl<'a> NoiseChunkSim<'a> {
    /// Convenience: build from the wired RandomState for the canon capture
    /// (chunk min block coords, clamped min_y/height, overworld quart sizes).
    /// P2.12: the wrapped-tree TEMPLATE is shared from the RandomState (built
    /// once in RandomState::build); only per-chunk state is allocated here.
    #[allow(clippy::too_many_arguments)]
    pub fn from_random_state(
        rs: &'a crate::router::RandomState,
        cell_count_xz: i32,
        first_block_x: i32,
        first_block_z: i32,
    ) -> Self {
        Self::instantiate(
            &rs.sim_template,
            &rs.bank,
            cell_count_xz,
            first_block_x,
            first_block_z,
            rs.settings.min_y,
            rs.settings.height,
            rs.settings.noise_size_horizontal,
            rs.settings.noise_size_vertical,
            if rs.tile_cache.enabled { Some(&rs.tile_cache) } else { None },
            rs.tile_epoch,
        )
    }
}

/// ChunkPos.asLong — x in the LOW 32 bits, z in the HIGH 32 bits.
fn chunk_as_long(x: i32, z: i32) -> i64 {
    (x as i64 & 0xFFFF_FFFF) | ((z as i64 & 0xFFFF_FFFF) << 32)
}
