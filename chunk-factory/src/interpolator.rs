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

#[derive(Debug, Clone)]
struct InterpState {
    inner: usize,
    /// R2#3: flat slice storage — one contiguous buffer per slice, indexed
    /// `[z * (cell_count_y + 1) + y]` (rows = cell_count_xz + 1, cols =
    /// cell_count_y + 1). Replaces Vec<Vec<f64>>: the 8 selectCellYZ corner
    /// reads per interpolator per cell become single-offset reads into one
    /// contiguous buffer, and fill_slice rows are contiguous sub-slices.
    /// Values, fill order and the per-row fill_array call structure (the
    /// array_interpolation_counter epochs CacheOnce observes) are unchanged.
    slice0: Vec<f64>,
    slice1: Vec<f64>,
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

#[derive(Clone)]
struct CellCacheState {
    inner: usize,
    values: Vec<f64>,
}

#[derive(Clone)]
struct Cache2DState {
    inner: usize,
    last_pos2d: i64,
    last_value: f64,
}

#[derive(Clone)]
struct CacheOnceState {
    inner: usize,
    last_counter: u64,
    last_array_counter: u64,
    last_value: f64,
    last_array: Option<Vec<f64>>,
}

#[derive(Clone)]
struct FlatCacheState {
    inner: usize,
    values: Vec<f64>,
    size_xz: usize,
}

/// R2 free-win 5: frac table capacity — vanilla NoiseSettings clamps
/// size_horizontal/size_vertical to [1, 4] (codec intRange), so cell
/// width/height = 4*size span 4..=16.
const MAX_FRAC: usize = 16;

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
/// NP1: Clone — a fill WORKER needs a private snapshot of the whole per-chunk
/// machine (all fields are owned Vecs/arrays/primitives plus the shared
/// `&'a` template/bank/tile refs, which are Copy). The worker mutates its
/// clone for its row range; the parent copies back exactly what the serial
/// end state requires (see fill_slice_parallel).
#[derive(Clone)]
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
    /// R2#2: per-chunk frac tables for the filling-cell lerp3 — the three
    /// `in_cell as f64 / cell as f64` divisions hoisted out of the per-node
    /// path (identical operands => identical quotient bits, I2-safe by
    /// construction). Valid whenever `filling_cell` is set: in-cell coords
    /// are then always within [0, cell_width) x [0, cell_height) x
    /// [0, cell_width) (both forIndex and fillAllDirectly guarantee it).
    /// R2 free-win 5: fixed-size frac tables (const MAX_FRAC slots) instead
    /// of Vec — the hot lerp3 reads lose the slice header/len load. Vanilla
    /// clamps NoiseSettings size_horizontal/size_vertical to [1, 4] (codec
    /// intRange), so cells span 4..=16 and MAX_FRAC = 16 covers every
    /// loadable world (overworld 4x8, end 8x4); instantiate asserts it once
    /// per machine. Slots at indices >= cell_{width,height} stay 0.0 and are
    /// never read: filling_cell guarantees in_cell_* within [0, cell_*).
    frac_x: [f64; MAX_FRAC],
    frac_y: [f64; MAX_FRAC],
    frac_z: [f64; MAX_FRAC],
    /// De-alloc: reusable secondary buffer for the fillArray Ap2::Add
    /// override (Java allocates `doubles = new double[...]` per visit).
    /// Taken/put around each use so nested Ap2::Add levels still get
    /// private storage (identical per-level buffers, identical writes).
    ap2_scratch: Vec<f64>,
    /// P2.12 cross-chunk tile cache (None = disabled).
    tile: Option<&'a crate::tile::TileCache>,
    tile_epoch: u64,
    /// R2#1: per-node LAST-SLOT front memo in front of the tile cache —
    /// one (subtree_hash, x, z, value) entry per W-node id, sentinel
    /// (0, i32::MIN, i32::MIN, 0.0). Slice fills evaluate the same (node,
    /// world column) once per y corner (overworld: 49x per column); every
    /// repeat otherwise pays 3 fnv mixes + a RefCell probe for a
    /// bit-identical hit. FRONT memo only — tile cache semantics
    /// (persistence, eviction, epoch, counters) are untouched.
    last_tile: Vec<(u64, i32, i32, f64)>,
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
                    // R2#3: flat slices — rows * cols f64, calloc-backed.
                    slice0: vec![0.0; rows * cols],
                    slice1: vec![0.0; rows * cols],
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
            // R2#2 + R2 free-win 5: the hoisted filling-cell fracs — `i as
            // f64 / cell as f64` with the exact operands the per-node path
            // used, computed once into fixed tables (assert once per machine:
            // vanilla clamps the settings to cells 4..=16 <= MAX_FRAC).
            frac_x: {
                assert!(cell_width as usize <= MAX_FRAC, "cell_width {cell_width} exceeds frac table MAX_FRAC");
                let mut a = [0.0; MAX_FRAC];
                for (i, v) in a.iter_mut().enumerate().take(cell_width as usize) {
                    *v = i as f64 / cell_width as f64;
                }
                a
            },
            frac_y: {
                assert!(cell_height as usize <= MAX_FRAC, "cell_height {cell_height} exceeds frac table MAX_FRAC");
                let mut a = [0.0; MAX_FRAC];
                for (i, v) in a.iter_mut().enumerate().take(cell_height as usize) {
                    *v = i as f64 / cell_height as f64;
                }
                a
            },
            frac_z: {
                let mut a = [0.0; MAX_FRAC];
                for (i, v) in a.iter_mut().enumerate().take(cell_width as usize) {
                    *v = i as f64 / cell_width as f64;
                }
                a
            },
            ap2_scratch: Vec::new(),
            tile,
            tile_epoch,
            // R2#1: per-node last-slot front memo, sentinel-initialized
            // (world block coords never reach i32::MIN, and a real
            // (0, i32::MIN, i32::MIN) triple is impossible).
            last_tile: vec![(0u64, i32::MIN, i32::MIN, 0.0f64); template.wnodes.len()],
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

// --------------------------------------------------------------------------
// N1 probe counters (noise/interpolator machine residue, standing order R5,
// cfg(ncf_profile) only — absent from release/CI builds as a class, same as
// the S2_* class in surface_rules.rs). These numbers gate the future PKG-D
// SoA-SIMD decision on the substance fill:
//   N1_TILE_HITS / N1_TILE_MISSES — y-free subtree tile-cache probe outcomes,
//     counted at the NoiseChunkSim::compute call site (tile.rs internals
//     deliberately untouched — counting lives on the caller side).
//   N1_FILL_NODE_VISITS — W-node visits (fill_array entries) while the
//     substance cache fills (select_cell_yz, the final_density
//     CacheAllInCell): the per-node dispatch the SoA rewrite would remove.
//   N1_FILL_ELEMS — substance-fill elements (cw*cw*ch per cell = 128
//     overworld; 98,304/chunk at 768 cells).
#[cfg(ncf_profile)]
pub static N1_TILE_HITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_TILE_MISSES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_FILL_NODE_VISITS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_FILL_ELEMS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_FILL_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
// N1 probe (R3-N1B): slice-fill decomposition — COARSE clock around the
// whole fill_slice (5-10 calls/chunk, no inflation), leaf-call counters
// split y-free (tile-cacheable, node_flags==2) vs y-dependent (the SoA
// batchable mass), and a coarse clock around the whole drive_blocks call
// (filler.rs, 1 call/chunk). Decision rule (worklist N1): L4 slice-leaf
// SoA batching is alive iff SLICE_NANOS >= ~2.5 ms AND YDEP leaves are a
// major share of noise-leaf calls.
#[cfg(ncf_profile)]
pub static N1_SLICE_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_SLICE_LEAF_YDEP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_SLICE_LEAF_YFREE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N1_DRIVE_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
// N2 probe (NP2 go/no-go, standing order R5, cfg(ncf_profile) only): the
// parallel fill_slice arm split — clone / scope / main-rows / worker-rows /
// merge — plus an OPTIONAL per-unit table (fill_slice_rows unit =
// (row, interpolator) take+fill_array+put-back) behind a second env gate.
// All clocks are plain Instant deltas summed into Relaxed atomics (unit
// clocks run on BOTH threads; atomics sum per chunk). Zero effect on normal
// builds: every line here is compiled out without --cfg ncf_profile.
//   N2_FILL_CALLS      — fill_slice_parallel entries (5/chunk overworld)
//   N2_CLONE_NANOS     — the worker replica clone (once per fill call)
//   N2_SCOPE_NANOS     — the whole thread::scope (spawn..join..merge)
//   N2_MAIN_ROWS_NANOS — parent-side fill_slice_rows (lower half)
//   N2_WORKER_ROWS_NANOS — worker-side fill_slice_rows (upper half)
//   N2_MERGE_NANOS     — copy-back + counter/scalar/memo merge block
// Decision rule (worklist NP2): keep pushing parallelism only if the fixed
// fork/join overhead (clone + scope - max(main, worker)) is small vs the
// parallel slice mass, and clone cost does not dominate a 5-call chunk.
#[cfg(ncf_profile)]
pub static N2_FILL_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N2_CLONE_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N2_SCOPE_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N2_MAIN_ROWS_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N2_WORKER_ROWS_NANOS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(ncf_profile)]
pub static N2_MERGE_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Per-unit slots, indexed `row * interps_len + id` (overworld: 5 rows x
/// 8 interps = 40 used of 64; the bench guards every index). Optional —
/// see N2_UNIT_PROBE below.
#[cfg(ncf_profile)]
pub const N2_UNIT_LEN: usize = 64;
#[cfg(ncf_profile)]
pub static N2_UNIT_NANOS: [std::sync::atomic::AtomicU64; N2_UNIT_LEN] =
    [const { std::sync::atomic::AtomicU64::new(0) }; N2_UNIT_LEN];
#[cfg(ncf_profile)]
pub static N2_UNIT_IC_DELTA: [std::sync::atomic::AtomicU64; N2_UNIT_LEN] =
    [const { std::sync::atomic::AtomicU64::new(0) }; N2_UNIT_LEN];
/// interpolators.len() seen by the last unit-probed fill_slice_rows call —
/// the bench needs it to split the flat unit slots back into (row, id).
#[cfg(ncf_profile)]
pub static N2_INTERPS_LEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// N2 per-unit gate, read ONCE per process: only the exact value "1"
/// enables the per-unit clocks in fill_slice_rows (~4-6 µs/chunk of clock
/// pairs at 200 units/chunk — always OFF unless asked for). Independent of
/// NCF_PAR_FILL (units are clocked on the serial path too) and of
/// NCF_N2_PROBE (printout gate, bench side only).
#[cfg(ncf_profile)]
static N2_UNIT_PROBE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

/// N2 per-unit probe gate (probe only; cfg(ncf_profile) builds).
#[cfg(ncf_profile)]
#[inline]
pub fn n2_unit_probe_enabled() -> bool {
    *N2_UNIT_PROBE.get_or_init(|| std::env::var("NCF_N2_UNIT_PROBE").as_deref() == Ok("1"))
}

#[cfg(ncf_profile)]
thread_local! {
    static N1_IN_SLICE_FILL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
/// N1 leaf tick on the slice-fill path (probe only).
#[cfg(ncf_profile)]
#[inline]
fn n1_slice_leaf_tick(y_free: bool) {
    if N1_IN_SLICE_FILL.with(|c| c.get()) {
        if y_free {
            N1_SLICE_LEAF_YFREE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        } else {
            N1_SLICE_LEAF_YDEP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

// Substance-fill phase marker (probe only): true exactly while the
// select_cell_yz substance fill walks the tree. thread_local — like FDEPTH —
// because a NoiseChunkSim is driven from a single thread (&mut self).
#[cfg(ncf_profile)]
thread_local! {
    static N1_IN_SUBSTANCE_FILL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

// --------------------------------------------------------------------------
// NP1: noise-parallel slice-fill gate (NCF_PAR_FILL, default OFF)
// --------------------------------------------------------------------------

/// NP1 gate, read ONCE per process: only the exact value "1" enables the
/// parallel fill arm; unset / "0" / anything else keeps the serial path
/// (default OFF = byte- and codepath-identical). Never read per call.
static PAR_FILL: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

#[cfg(test)]
static PAR_FORCE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// NP1 gate. In test builds the PAR_FORCE atomic (see force_par_fill)
/// overrides the env — a OnceLock fed from the env cannot be re-armed
/// per test, and tests must drive BOTH arms in one process.
#[inline]
fn par_fill_enabled() -> bool {
    #[cfg(test)]
    {
        match PAR_FORCE.load(std::sync::atomic::Ordering::Relaxed) {
            1 => return true,
            2 => return false,
            _ => {}
        }
    }
    *PAR_FILL.get_or_init(|| std::env::var("NCF_PAR_FILL").map(|v| v == "1").unwrap_or(false))
}

/// Test-only gate override: 0 = follow NCF_PAR_FILL, 1 = force parallel,
/// 2 = force serial. pub(crate) so the interpolator tests (and only tests)
/// can pin the arm under test.
#[cfg(test)]
pub(crate) fn force_par_fill(mode: u8) {
    PAR_FORCE.store(mode, std::sync::atomic::Ordering::Relaxed);
}

// --------------------------------------------------------------------------
// R2#1: debug-only re-entry guard (the old per-node TLS depth check)
// --------------------------------------------------------------------------

// The old compute_body/fill_array carried a thread-local depth counter paid
// on EVERY node visit (get + set + Drop). The W-tree is a finite acyclic DAG
// (structural interning makes self-containment impossible — module header),
// so per-node recursion is bounded by tree depth and the guard was purely
// defensive. It now lives at the top-level entry points (where recursion
// into the machine originates) and at fill_array's own entry — and only in
// debug builds; release builds compile it out entirely (N1-H6 closed the
// last unconditional holdout: fill_array's per-visit FDEPTH, below).
#[cfg(debug_assertions)]
thread_local! {
    static ENTRY_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(debug_assertions)]
struct EntryGuard;

#[cfg(debug_assertions)]
impl Drop for EntryGuard {
    fn drop(&mut self) {
        ENTRY_DEPTH.with(|c| c.set(c.get() - 1));
    }
}

#[cfg(debug_assertions)]
fn enter_interp_entry(site: &'static str) -> EntryGuard {
    let d = ENTRY_DEPTH.with(|c| c.get());
    assert!(d <= 100000, "interp entry re-entry depth {d} at {site}");
    ENTRY_DEPTH.with(|c| c.set(d + 1));
    EntryGuard
}

// N1-H6: fill_array's own per-visit guard — same story, same shape as the
// EntryGuard above. It used to run UNCONDITIONALLY on every fill_array
// visit (TLS get + set + Drop, release included), contradicting the header
// claim; the whole machinery is now cfg(debug_assertions) and release
// builds compile it out entirely. Recursion safety in release rests on the
// same argument as for the compute path: the W-tree is a finite acyclic
// DAG, so recursion depth is bounded by tree depth.
#[cfg(debug_assertions)]
thread_local! {
    static FDEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(debug_assertions)]
struct FillGuard;

#[cfg(debug_assertions)]
impl Drop for FillGuard {
    fn drop(&mut self) {
        FDEPTH.with(|c| c.set(c.get() - 1));
    }
}

#[cfg(debug_assertions)]
fn enter_fill_entry(w: usize, node: &WNode) -> FillGuard {
    let d = FDEPTH.with(|c| c.get());
    if d > 100000 {
        panic!("fill_array recursion depth {d} at w={w} node={node:?}");
    }
    FDEPTH.with(|c| c.set(d + 1));
    FillGuard
}

impl<'a> NoiseChunkSim<'a> {
    // ------------------------------------------------------------------
    // compute
    // ------------------------------------------------------------------

    #[inline]
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
            let hash = self.template.subtree_hash[w];
            // R2#1 per-node LAST-SLOT front memo: a slice fill evaluates
            // the same (node, world column) once per y corner (overworld:
            // 49x per column) and every repeat paid 3 fnv mixes + a
            // RefCell probe (+ hit/miss counter writes) for a
            // bit-identical f64. The slot collapses repeats to 3 integer
            // compares. FRONT memo only: tile semantics (persistence,
            // eviction, epoch, counters) are untouched — a slot miss
            // falls through to the tile probe exactly as before, and
            // every tile outcome (hit OR freshly computed miss after
            // put) refreshes the slot. Soundness: the tile contract
            // already makes f(w, x, z) a pure function for flag-2 nodes,
            // so the last (hash, x, z) -> value per node id is
            // bit-identical; a same-(w,x,z) re-entry through a
            // DESCENDANT is impossible (finite acyclic DAG — module
            // header), so no in-flight call can observe a stale slot.
            // (ncf_profile note: front-memo hits bypass the tile counters,
            // so hit/miss ratios shift — diagnostic-only, cfg-gated.)
            let slot = self.last_tile[w];
            if slot.0 == hash && slot.1 == ctx.x && slot.2 == ctx.z {
                return slot.3;
            }
            let key = crate::tile::tile_key(hash, ctx.x, ctx.z, self.tile_epoch);
            // SAFETY of the unwrap: checked is_some above
            let tile = self.tile.unwrap();
            if let Some(v) = tile.get(key) {
                #[cfg(ncf_profile)]
                {
                    // N1 probe: tile-cache outcome (counted at the call
                    // site; tile.rs internals untouched).
                    N1_TILE_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                self.last_tile[w] = (hash, ctx.x, ctx.z, v);
                return v;
            }
            #[cfg(ncf_profile)]
            {
                N1_TILE_MISSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            let v = self.compute_body(w, ctx);
            tile.put(key, v);
            self.last_tile[w] = (hash, ctx.x, ctx.z, v);
            return v;
        }
        self.compute_body(w, ctx)
    }

    fn compute_body(&mut self, w: usize, ctx: Ctx) -> f64 {
        // R2#1: the per-node TLS DEPTH guard (get + set + Drop on EVERY node
        // visit) is gone. The W-tree is a finite acyclic DAG (interning
        // guarantees no cycles — module header), so the guard was purely
        // defensive; a debug-only re-entry guard survives at the top-level
        // entry points (enter_interp_entry), release builds compile it out.
        // R2#1: match on a REFERENCE — the old per-visit
        // `self.template.wnodes[w].clone()` moved a 48-byte node per visit.
        // The template ref is copied out first, so `node` never borrows
        // `self` and every arm keeps its verbatim arithmetic.
        let template = self.template;
        let node = &template.wnodes[w];
        match node {
            WNode::Const(v) => f64::from_bits(*v),
            WNode::YClampedGradient { from_y, to_y, from_value, to_value } => mth::clamped_map(
                ctx.y as f64,
                *from_y as f64,
                *to_y as f64,
                f64::from_bits(*from_value),
                f64::from_bits(*to_value),
            ),
            WNode::Noise(idx, xz, ys) => {
                #[cfg(ncf_profile)]
                n1_slice_leaf_tick(template.node_flags[w] == 2);
                self.bank.noises[*idx].get_value(
                    ctx.x as f64 * f64::from_bits(*xz),
                    ctx.y as f64 * f64::from_bits(*ys),
                    ctx.z as f64 * f64::from_bits(*xz),
                )
            }
            WNode::ShiftedNoise { sx, sy, sz, xz, ys, noise } => {
                #[cfg(ncf_profile)]
                n1_slice_leaf_tick(template.node_flags[w] == 2);
                let d = ctx.x as f64 * f64::from_bits(*xz) + self.compute(*sx, ctx);
                let d1 = ctx.y as f64 * f64::from_bits(*ys) + self.compute(*sy, ctx);
                let d2 = ctx.z as f64 * f64::from_bits(*xz) + self.compute(*sz, ctx);
                self.bank.noises[*noise].get_value(d, d1, d2)
            }
            WNode::ShiftA(idx) => {
                #[cfg(ncf_profile)]
                n1_slice_leaf_tick(template.node_flags[w] == 2);
                self.bank.noises[*idx].get_value(ctx.x as f64 * 0.25, 0.0, ctx.z as f64 * 0.25) * 4.0
            }
            WNode::ShiftB(idx) => {
                #[cfg(ncf_profile)]
                n1_slice_leaf_tick(template.node_flags[w] == 2);
                self.bank.noises[*idx].get_value(ctx.z as f64 * 0.25, ctx.x as f64 * 0.25, 0.0) * 4.0
            }
            WNode::Shift(idx) => {
                #[cfg(ncf_profile)]
                n1_slice_leaf_tick(template.node_flags[w] == 2);
                self.bank.noises[*idx].get_value(ctx.x as f64 * 0.25, ctx.y as f64 * 0.25, ctx.z as f64 * 0.25) * 4.0
            }
            WNode::BlendDensity(i) => self.compute(*i, ctx), // Blender.empty identity
            WNode::WeirdScaledSampler { input, noise, rarity } => {
                let value = self.compute(*input, ctx);
                let d = rarity.map(value);
                d * self.bank.noises[*noise].get_value(ctx.x as f64 / d, ctx.y as f64 / d, ctx.z as f64 / d).abs()
            }
            WNode::RangeChoice { input, min, max, in_range, out_of_range } => {
                let d = self.compute(*input, ctx);
                if d >= f64::from_bits(*min) && d < f64::from_bits(*max) {
                    self.compute(*in_range, ctx)
                } else {
                    self.compute(*out_of_range, ctx)
                }
            }
            WNode::Clamp { input, min, max } => {
                mth::clamp(self.compute(*input, ctx), f64::from_bits(*min), f64::from_bits(*max))
            }
            WNode::Mapped { ty, input } => {
                let v = self.compute(*input, ctx);
                self.mapped_transform(*ty, v)
            }
            WNode::MulOrAdd { is_add, input, argument } => {
                let v = self.compute(*input, ctx);
                if *is_add {
                    v + f64::from_bits(*argument)
                } else {
                    v * f64::from_bits(*argument)
                }
            }
            WNode::Ap2 { ty, a1, a2, a2_min, a2_max } => {
                let d = self.compute(*a1, ctx);
                match ty {
                    Ap2Type::Add => d + self.compute(*a2, ctx),
                    Ap2Type::Mul => {
                        if d == 0.0 {
                            0.0
                        } else {
                            d * self.compute(*a2, ctx)
                        }
                    }
                    Ap2Type::Min => {
                        if d < f64::from_bits(*a2_min) {
                            d
                        } else {
                            mth::java_min(d, self.compute(*a2, ctx))
                        }
                    }
                    Ap2Type::Max => {
                        if d > f64::from_bits(*a2_max) {
                            d
                        } else {
                            mth::java_max(d, self.compute(*a2, ctx))
                        }
                    }
                }
            }
            WNode::Spline(s) => self.spline_apply(*s, ctx) as f64,
            WNode::Blended(idx) => self.bank.blended[*idx].compute(ctx.x, ctx.y, ctx.z),
            WNode::BlendAlpha => 1.0,
            WNode::BlendOffset => 0.0,
            // BeardifierMarker -> the per-chunk Beardifier (NoiseChunk.wrap
            // CFR 372-373). EMPTY instance computes exactly 0.0 — identical
            // to the pre-wiring marker semantics.
            WNode::Beardifier => self.beard.compute(ctx.x, ctx.y, ctx.z),
            WNode::EndIslands => panic!("EndIslands scalar eval not implemented yet (Phase 2 tail)"),
            WNode::FindTopSurface { density, upper, lower_bound, cell_height } => {
                let i = mth::floor(self.compute(*upper, ctx) / (*cell_height) as f64) * (*cell_height);
                if i <= *lower_bound {
                    return *lower_bound as f64;
                }
                let mut i1 = i;
                while i1 >= *lower_bound {
                    if self.compute(*density, Ctx { x: ctx.x, y: i1, z: ctx.z, in_chunk: ctx.in_chunk }) > 0.0 {
                        return i1 as f64;
                    }
                    i1 -= *cell_height;
                }
                *lower_bound as f64
            }
            WNode::W(kind) => match kind {
                WKind::Interp(id) => {
                    if !ctx.in_chunk {
                        let inner = self.interpolators[*id].inner;
                        return self.compute(inner, ctx);
                    }
                    if !self.interpolating {
                        panic!("Trying to sample interpolator outside the interpolation loop");
                    }
                    if self.filling_cell {
                        let n = self.interpolators[*id].noise; // [000,001,100,101,010,011,110,111]
                        // R2#2 + R2 free-win 5: the three per-visit fracs
                        // come from the fixed per-chunk tables — same
                        // operands/quotient bits as the original divisions
                        // (zero float change); fixed-capacity storage drops
                        // the Vec header/len load and, with it, the bounds
                        // check.
                        // SAFETY: filling_cell guarantees in_cell_x/z in
                        // [0, cell_width) and in_cell_y in [0, cell_height)
                        // (forIndex + fillAllDirectly loop bounds), and
                        // instantiate asserts cell_width/cell_height <=
                        // MAX_FRAC — so every index below is < MAX_FRAC.
                        let (fx, fy, fz) = unsafe { (
                            *self.frac_x.get_unchecked(self.in_cell_x as usize),
                            *self.frac_y.get_unchecked(self.in_cell_y as usize),
                            *self.frac_z.get_unchecked(self.in_cell_z as usize),
                        ) };
                        mth::lerp3(
                            fx, fy, fz,
                            n[0], n[2], n[4], n[6], n[1], n[3], n[5], n[7],
                        )
                    } else {
                        self.interpolators[*id].value
                    }
                }
                WKind::FlatCacheW(id) => {
                    let (qx, qz) = (ctx.x.div_euclid(4), ctx.z.div_euclid(4));
                    let i = qx - self.first_noise_x;
                    let i1 = qz - self.first_noise_z;
                    let size_xz = self.flat_caches[*id].size_xz;
                    if i >= 0 && i1 >= 0 && (i as usize) < size_xz && (i1 as usize) < size_xz {
                        self.flat_caches[*id].values[i as usize + i1 as usize * size_xz]
                    } else {
                        let inner = self.flat_caches[*id].inner;
                        self.compute(inner, ctx)
                    }
                }
                WKind::Cache2DW(id) => {
                    let packed = chunk_as_long(ctx.x, ctx.z);
                    if self.cache2ds[*id].last_pos2d == packed {
                        self.cache2ds[*id].last_value
                    } else {
                        let inner = self.cache2ds[*id].inner;
                        let v = self.compute(inner, ctx);
                        let st = &mut self.cache2ds[*id];
                        st.last_pos2d = packed;
                        st.last_value = v;
                        v
                    }
                }
                WKind::CacheOnceW(id) => {
                    if !ctx.in_chunk {
                        let inner = self.cacheonces[*id].inner;
                        return self.compute(inner, ctx);
                    }
                    let hit_array = self.cacheonces[*id]
                        .last_array
                        .as_ref()
                        .map(|_| self.cacheonces[*id].last_array_counter == self.array_interpolation_counter)
                        .unwrap_or(false);
                    if hit_array {
                        return self.cacheonces[*id].last_array.as_ref().unwrap()[self.array_index];
                    }
                    if self.cacheonces[*id].last_counter == self.interpolation_counter {
                        return self.cacheonces[*id].last_value;
                    }
                    let inner = self.cacheonces[*id].inner;
                    let v = self.compute(inner, ctx);
                    let st = &mut self.cacheonces[*id];
                    st.last_counter = self.interpolation_counter;
                    st.last_value = v;
                    v
                }
                WKind::CellCacheW(id) => {
                    if !ctx.in_chunk {
                        let inner = self.cell_caches[*id].inner;
                        return self.compute(inner, ctx);
                    }
                    if !self.interpolating {
                        panic!("Trying to sample interpolator outside the interpolation loop");
                    }
                    let (i, i1, i2) = (self.in_cell_x, self.in_cell_y, self.in_cell_z);
                    let (cw, ch) = (self.cell_width, self.cell_height);
                    if i >= 0 && i1 >= 0 && i2 >= 0 && i < cw && i1 < ch && i2 < cw {
                        let slot = (((ch - 1 - i1) * cw + i) * cw + i2) as usize;
                        self.cell_caches[*id].values[slot]
                    } else {
                        let inner = self.cell_caches[*id].inner;
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
        // De-alloc: copy the template ref out first (same pattern as
        // compute_body) so the spline slices borrow the SHARED template
        // ('a) and not `self` — the three per-eval Vec clones are gone.
        // The immutable sp borrows cross the child self.compute calls
        // unchanged; reads are bit-identical (same Vecs, same order).
        let template = self.template;
        let sp = &template.wsplines[s];
        let locations = &sp.locations;
        let derivatives = &sp.derivatives;
        let values = &sp.values;
        let f = self.compute(sp.coordinate, ctx) as f32;
        let i = crate::density::find_interval_start(locations, f);
        let i1 = (locations.len() - 1) as i32;
        if i < 0 {
            let value = self.spline_value(&values[0], ctx);
            return crate::density::linear_extend(f, locations, value, derivatives, 0);
        }
        if i == i1 {
            let value = self.spline_value(&values[i1 as usize], ctx);
            return crate::density::linear_extend(f, locations, value, derivatives, i1 as usize);
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
        // N1-H6: debug-only re-entry guard — the EntryGuard pattern applied
        // to the fill path. Was: unconditional TLS depth get/set/Drop on
        // EVERY fill_array visit (release included). Release builds compile
        // it out entirely (see enter_fill_entry above for the recursion
        // argument); debug builds keep identical overflow detection and an
        // identical panic message.
        #[cfg(debug_assertions)]
        let _gf = enter_fill_entry(w, &self.template.wnodes[w]);
        #[cfg(ncf_profile)]
        {
            // N1 probe: W-node visits while the substance cache fills (see
            // the N1 counter block below). fill_array IS the node visit —
            // one entry per WNode dispatched, recursion included.
            if N1_IN_SUBSTANCE_FILL.with(|c| c.get()) {
                N1_FILL_NODE_VISITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
        // De-alloc: match on a REFERENCE into the shared template (copied
        // ref, same pattern as compute_body) — the per-visit 48-byte
        // `wnodes[w].clone()` is gone; every arm keeps its verbatim logic.
        let template = self.template;
        let node = &template.wnodes[w];
        match node {
            WNode::Const(v) => {
                array.fill(f64::from_bits(*v));
            }
            WNode::Ap2 { ty, a1, a2, a2_min, a2_max } => match ty {
                Ap2Type::Add => {
                    self.fill_array(*a1, array, provider);
                    // De-alloc: reuse the machine's scratch buffer instead of
                    // a fresh `vec![0.0; len]` per visit. Same size, same
                    // zero init, fully overwritten by the a2 fill, same add
                    // loop -> bit-identical; take/put keeps nested Ap2::Add
                    // levels on private buffers.
                    let mut doubles = std::mem::take(&mut self.ap2_scratch);
                    doubles.clear();
                    doubles.resize(array.len(), 0.0);
                    self.fill_array(*a2, &mut doubles, provider);
                    for i in 0..array.len() {
                        array[i] += doubles[i];
                    }
                    self.ap2_scratch = doubles;
                }
                Ap2Type::Mul => {
                    self.fill_array(*a1, array, provider);
                    for i1 in 0..array.len() {
                        let d = array[i1];
                        array[i1] = if d == 0.0 { 0.0 } else { d * self.compute_for_index(*a2, i1, provider) };
                    }
                }
                Ap2Type::Min => {
                    let d1 = f64::from_bits(*a2_min);
                    self.fill_array(*a1, array, provider);
                    for i2 in 0..array.len() {
                        let d2 = array[i2];
                        array[i2] = if d2 < d1 {
                            d2
                        } else {
                            mth::java_min(d2, self.compute_for_index(*a2, i2, provider))
                        };
                    }
                }
                Ap2Type::Max => {
                    let d1 = f64::from_bits(*a2_max);
                    self.fill_array(*a1, array, provider);
                    for i2 in 0..array.len() {
                        let d2 = array[i2];
                        array[i2] = if d2 > d1 {
                            d2
                        } else {
                            mth::java_max(d2, self.compute_for_index(*a2, i2, provider))
                        };
                    }
                }
            },
            WNode::Mapped { ty, input } => {
                self.fill_array(*input, array, provider);
                for i in 0..array.len() {
                    array[i] = self.mapped_transform(*ty, array[i]);
                }
            }
            WNode::RangeChoice { input, min, max, in_range, out_of_range } => {
                self.fill_array(*input, array, provider);
                for i in 0..array.len() {
                    let d = array[i];
                    array[i] = if d >= f64::from_bits(*min) && d < f64::from_bits(*max) {
                        self.compute_for_index(*in_range, i, provider)
                    } else {
                        self.compute_for_index(*out_of_range, i, provider)
                    };
                }
            }
            WNode::W(WKind::CacheOnceW(id)) => {
                let counter_hit = self.cacheonces[*id]
                    .last_array
                    .as_ref()
                    .map(|_| self.cacheonces[*id].last_array_counter == self.array_interpolation_counter)
                    .unwrap_or(false);
                if counter_hit {
                    // Java: System.arraycopy(lastArray, 0, array, 0, array.length)
                    // — a length mismatch would throw (vanilla never hits it).
                    // De-alloc: copy straight from the stored array (take/put
                    // shape of select_cell_yz is unnecessary here — no &mut
                    // overlap) — the per-hit Vec clone is gone.
                    let last = self.cacheonces[*id].last_array.as_ref().unwrap();
                    assert_eq!(last.len(), array.len(), "CacheOnce lastArray length changed on the read path (Java would throw)");
                    array.copy_from_slice(last);
                    return;
                }
                let inner = self.cacheonces[*id].inner;
                self.fill_array(inner, array, provider);
                let st = &mut self.cacheonces[*id];
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
                let inner = self.cache2ds[*id].inner;
                self.fill_array(inner, array, provider);
            }
            WNode::W(WKind::Interp(id)) => {
                if self.filling_cell {
                    self.provider_fill_all_directly(w, array, provider);
                } else {
                    let inner = self.interpolators[*id].inner;
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
                //
                // N1-H7 fast path for cw==4 && ch==8 — the only dimensions
                // the overworld machine ever instantiates: NoiseSettings
                // size_horizontal=1 / size_vertical=2 quarts -> 4/8 blocks
                // (router.rs defaults, instantiate() multiplies by 4),
                // filler.rs builds the per-chunk Sim via from_random_state
                // with exactly those settings, height_feed hard-codes 1,2,
                // and the SimTemplate cell_volume is 4*4*8.
                //
                // Validity of the shift/mask rewrite: `index: usize` is
                // provably within [0, array.len()) — every Provider::Cell
                // fill drives it over 0..array.len() with array.len() ==
                // cell_width*cell_width*cell_height (cell-cache values and
                // substance_cache; the per-element compute_for_index callers
                // in Ap2/RangeChoice reuse the same array) — hence `idx =
                // index as i32` is non-negative and far below i32::MAX. For
                // a non-negative dividend and the power-of-two divisor 4:
                // rem_euclid == `& 3` and div_euclid == `>> 2` exactly
                // (quotient and remainder are non-negative, so euclidean ==
                // truncating == shift/mask), and the derived i1 is
                // non-negative again, so its div/rem rewrite is equally
                // exact. Identical integers feed identical in_cell_x/y/z
                // state, so pos_now()/compute outputs are bit-identical
                // (pure integer identity — no float op involved). The
                // general path below stays for any other dimensions.
                let cw = self.cell_width;
                let ch = self.cell_height;
                let idx = index as i32;
                if cw == 4 && ch == 8 {
                    let i = idx & 3; // idx.rem_euclid(4)
                    let i1 = idx >> 2; // idx.div_euclid(4), idx >= 0
                    let i2 = i1 & 3; // i1.rem_euclid(4)
                    let i3 = 7 - (i1 >> 2); // ch - 1 - i1.div_euclid(4)
                    self.in_cell_x = i2;
                    self.in_cell_y = i3;
                    self.in_cell_z = i;
                } else {
                    let i = idx.rem_euclid(cw);
                    let i1 = idx.div_euclid(cw);
                    let i2 = i1.rem_euclid(cw);
                    let i3 = ch - 1 - i1.div_euclid(cw);
                    self.in_cell_x = i2;
                    self.in_cell_y = i3;
                    self.in_cell_z = i;
                }
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
        // De-alloc: iterate the id range directly (the range captures len()
        // once, like the collected Vec did) — no per-cell Vec allocation.
        // R2#3: flat slices — the 8 double-indirect corner reads become
        // single-offset reads at [z*cols + y] (and the z+1 / y+1
        // neighbors), one contiguous buffer per slice.
        let cols = (self.cell_count_y + 1) as usize;
        for id in 0..self.interpolators.len() {
            // Java order: noise000 = slice0[z][y]; noise001 = slice0[z+1][y];
            // noise100 = slice1[z][y]; noise101 = slice1[z+1][y];
            // noise010 = slice0[z][y+1]; noise011 = slice0[z+1][y+1];
            // noise110 = slice1[z][y+1]; noise111 = slice1[z+1][y+1]
            let st = &mut self.interpolators[id];
            let zoff = z as usize * cols;
            let yu = y as usize;
            let n000 = st.slice0[zoff + yu];
            let n001 = st.slice0[zoff + cols + yu];
            let n100 = st.slice1[zoff + yu];
            let n101 = st.slice1[zoff + cols + yu];
            let n010 = st.slice0[zoff + yu + 1];
            let n011 = st.slice0[zoff + cols + yu + 1];
            let n110 = st.slice1[zoff + yu + 1];
            let n111 = st.slice1[zoff + cols + yu + 1];
            st.noise = [n000, n001, n100, n101, n010, n011, n110, n111];
        }
        self.filling_cell = true;
        self.cell_start_block_y = (y + self.cell_noise_min_y) * self.cell_height;
        self.cell_start_block_z = (self.first_cell_z + z) * self.cell_width;
        self.array_interpolation_counter += 1;
        // De-alloc: id range iteration — no per-cell Vec allocation.
        for cid in 0..self.cell_caches.len() {
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
            #[cfg(ncf_profile)]
            {
                // N1 probe: arm the substance-fill phase marker and count
                // the elements this fill writes (cw*cw*ch per cell).
                N1_IN_SUBSTANCE_FILL.with(|c| c.set(true));
                N1_FILL_ELEMS.fetch_add(arr.len() as u64, std::sync::atomic::Ordering::Relaxed);
            }
            #[cfg(ncf_profile)]
            let n1_t0 = std::time::Instant::now();
            self.fill_array(self.root_fields[11], &mut arr, Provider::Cell);
            #[cfg(ncf_profile)]
            {
                N1_FILL_NANOS.fetch_add(
                    n1_t0.elapsed().as_nanos() as u64,
                    std::sync::atomic::Ordering::Relaxed,
                );
                N1_IN_SUBSTANCE_FILL.with(|c| c.set(false));
            }
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
        #[cfg(ncf_profile)]
        let n1_t0 = std::time::Instant::now();
        self.cell_start_block_x = start * self.cell_width;
        self.in_cell_x = 0;
        // R2#3: flat slice storage — a row is the contiguous sub-slice
        // flat[row..row + cols]. The buffer is taken OUT of self so
        // fill_array keeps a private &mut (same take/put shape as the old
        // per-row Vec), and the PER-ROW fill_array calls are preserved
        // verbatim: the array_interpolation_counter increments are
        // call-order-observable through the CacheOnce lastArray epochs —
        // rows are NOT batched.
        //
        // NP1: the row loop is the only parallel section (phase 1). The
        // serial path is the IDENTICAL instruction sequence via one
        // #[inline]-able call to fill_slice_rows (same body, moved verbatim);
        // the parallel arm forks ONE worker on the upper row range. The
        // trailing array_interpolation_counter bump stays HERE (parent-side)
        // in both arms.
        let rows = 0..(self.cell_count_xz + 1) as usize;
        if par_fill_enabled() && self.cell_count_xz >= 2 && rows.len() >= 2 {
            self.fill_slice_parallel(is_slice0, start, rows);
        } else {
            self.fill_slice_rows(is_slice0, start, rows);
        }
        self.array_interpolation_counter += 1;
        #[cfg(ncf_profile)]
        {
            N1_SLICE_NANOS.fetch_add(
                n1_t0.elapsed().as_nanos() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
    }

    /// NP1: the VERBATIM per-row body of fill_slice's row loop (the serial
    /// path executes exactly today's instruction sequence; the parallel arm
    /// runs disjoint row ranges of it on parent + one worker). Each row:
    /// cell_start_block_z/in_cell_z updates, one array_interpolation_counter
    /// bump, then the per-interpolator take / fill_array / put of
    /// flat[row..row+cols].
    ///
    /// The N1_IN_SLICE_FILL probe marker is armed HERE (not in fill_slice)
    /// so a NP1 worker thread — which runs this fn with its OWN TLS —
    /// buckets its leaf ticks into the slice-fill counters exactly like the
    /// parent. cfg(ncf_profile) only; zero effect on values.
    fn fill_slice_rows(&mut self, is_slice0: bool, _start: i32, rows: std::ops::Range<usize>) {
        #[cfg(ncf_profile)]
        N1_IN_SLICE_FILL.with(|c| c.set(true));
        let cols = (self.cell_count_y + 1) as usize;
        // N2 per-unit probe: read the gate + interp count ONCE per call (this
        // fn runs on BOTH threads — the atomics sum per chunk). Gate OFF =
        // only a branch + a field read per unit (~0.2 µs/chunk).
        #[cfg(ncf_profile)]
        let n2_unit = n2_unit_probe_enabled();
        #[cfg(ncf_profile)]
        let n2_interps_len = self.interpolators.len();
        #[cfg(ncf_profile)]
        if n2_unit {
            N2_INTERPS_LEN.store(
                n2_interps_len as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        for i in rows {
            let i1 = self.first_cell_z + i as i32;
            self.cell_start_block_z = i1 * self.cell_width;
            self.in_cell_z = 0;
            self.array_interpolation_counter += 1;
            // De-alloc: id range iteration — no per-slice Vec allocation.
            let row = i * cols;
            for id in 0..self.interpolators.len() {
                // N2 unit clock: take+fill_array+put-back for ONE (row, id)
                // unit. Instant is only constructed when the gate is on (the
                // 4-6 µs/chunk cost must not leak into unprobed runs).
                #[cfg(ncf_profile)]
                let n2_u0 = if n2_unit {
                    Some(std::time::Instant::now())
                } else {
                    None
                };
                #[cfg(ncf_profile)]
                let n2_ic0 = self.interpolation_counter;
                let mut flat = if is_slice0 {
                    std::mem::take(&mut self.interpolators[id].slice0)
                } else {
                    std::mem::take(&mut self.interpolators[id].slice1)
                };
                let inner = self.interpolators[id].inner;
                self.fill_array(inner, &mut flat[row..row + cols], Provider::Slice);
                if is_slice0 {
                    self.interpolators[id].slice0 = flat;
                } else {
                    self.interpolators[id].slice1 = flat;
                }
                #[cfg(ncf_profile)]
                if let Some(n2_u0) = n2_u0 {
                    let n2_idx = i * n2_interps_len + id;
                    if n2_idx < N2_UNIT_LEN {
                        N2_UNIT_NANOS[n2_idx].fetch_add(
                            n2_u0.elapsed().as_nanos() as u64,
                            std::sync::atomic::Ordering::Relaxed,
                        );
                        N2_UNIT_IC_DELTA[n2_idx].fetch_add(
                            (self.interpolation_counter - n2_ic0) as u64,
                            std::sync::atomic::Ordering::Relaxed,
                        );
                    }
                }
            }
        }
        #[cfg(ncf_profile)]
        N1_IN_SLICE_FILL.with(|c| c.set(false));
    }

    /// NP1: the fork/join parallel arm for fill_slice's rows — main thread
    /// keeps the larger lower half, ONE cloned worker takes the upper half
    /// (std::thread::scope, the region.rs write_region_parallel pattern).
    ///
    /// Template precondition (NP1 phase-1 contract — re-verify before
    /// enabling NCF_PAR_FILL for other templates): all cache_2d/flat_cache
    /// inners in the ACTIVE template are y-free IN VALUE (JSON-verified for
    /// the overworld template), cellCaches are empty (cell_inners == []),
    /// and NO RNG exists on the fill path (every draw is constructor-time or
    /// position-derived in the drive callback). Under those conditions every
    /// fill unit is position-pure: the f64 written for (row i, element j)
    /// is f(node, ctx(X, Y_j, Z_i)) — independent of which replica or thread
    /// computes it and of visit order ACROSS replicas (I2-safe by
    /// construction; zero float-op changes).
    ///
    /// Value transparency of the per-replica memos — why they need NO merge:
    /// * last_tile front memo + TileCache: pure-function values keyed by
    ///   (node, world column). A stale/cold slot only re-routes the read
    ///   through a tile probe or a recompute, which returns the identical
    ///   bits; different post-call memo CONTENT between serial and parallel
    ///   is therefore unobservable in every later value.
    /// * Cache2D (last_pos2d/last_value): position-keyed pure memo — a hit
    ///   requires an exact (x, z) match and returns f(that position), which
    ///   is bit-identical to a recompute. No epoch, cannot false-hit.
    /// * CacheOnce (last_counter/last_value, last_array/last_array_counter):
    ///   epoch-keyed pure memo. Every replica starts at the fork snapshot
    ///   (stored epochs <= fork epoch) and its own epochs are strictly
    ///   monotone, so a replica can never observe an epoch FUTURE to its
    ///   own state — no false hits, only recomputes of identical bits.
    /// The copy-backs below exist to make the PARENT's post-call machine
    /// state byte-identical to the serial end state: later phases
    /// (select_cell_yz epoch arithmetic, CacheOnce checks, drive-callback
    /// reads) must see the serial machine, not merely an equivalent one.
    fn fill_slice_parallel(&mut self, is_slice0: bool, start: i32, rows: std::ops::Range<usize>) {
        #[cfg(ncf_profile)]
        N2_FILL_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mid = (rows.start + rows.end + 1) / 2; // main takes the larger lower half
        let fork_ic = self.interpolation_counter;
        let fork_aic = self.array_interpolation_counter;
        // Fork: snapshot clone for the worker. The clone OWNS everything
        // mutable; only the shared `&'a` refs (SimTemplate/NoiseBank/
        // TileCache) cross the scope boundary — all Sync (tile.rs NP1).
        // The closure captures NO reference into self's interior.
        #[cfg(ncf_profile)]
        let n2_t_clone = std::time::Instant::now();
        let mut worker = self.clone();
        #[cfg(ncf_profile)]
        N2_CLONE_NANOS.fetch_add(
            n2_t_clone.elapsed().as_nanos() as u64,
            std::sync::atomic::Ordering::Relaxed,
        );
        let is_slice0_c = is_slice0;
        let start_c = start;
        let rows_end = rows.end;
        #[cfg(ncf_profile)]
        let n2_t_scope = std::time::Instant::now();
        std::thread::scope(|scope| {
            let h = scope.spawn(move || {
                #[cfg(ncf_profile)]
                let n2_t_worker = std::time::Instant::now();
                worker.fill_slice_rows(is_slice0_c, start_c, mid..rows_end);
                #[cfg(ncf_profile)]
                N2_WORKER_ROWS_NANOS.fetch_add(
                    n2_t_worker.elapsed().as_nanos() as u64,
                    std::sync::atomic::Ordering::Relaxed,
                );
                worker // return the owned clone
            });
            #[cfg(ncf_profile)]
            let n2_t_main = std::time::Instant::now();
            self.fill_slice_rows(is_slice0, start, rows.start..mid);
            #[cfg(ncf_profile)]
            N2_MAIN_ROWS_NANOS.fetch_add(
                n2_t_main.elapsed().as_nanos() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
            let done = h.join().expect("np1 fill worker");
            #[cfg(ncf_profile)]
            let n2_t_merge = std::time::Instant::now();
            // Parent deltas while the worker ran (ic = per-element Slice
            // bumps, aic = per-row bumps). Serial totals = fork + both
            // deltas; the parent already holds its own.
            let d_ic = self.interpolation_counter - fork_ic;
            let d_aic = self.array_interpolation_counter - fork_aic;
            // ---- (a) slice rows the worker owned: memcpy f64 -------------
            let cols = (self.cell_count_y + 1) as usize;
            let lo = mid * cols;
            let hi = rows_end * cols;
            for id in 0..self.interpolators.len() {
                let (dst, src) = if is_slice0 {
                    (&mut self.interpolators[id].slice0, &done.interpolators[id].slice0)
                } else {
                    (&mut self.interpolators[id].slice1, &done.interpolators[id].slice1)
                };
                dst[lo..hi].copy_from_slice(&src[lo..hi]);
            }
            // ---- (b) COUNTER MERGE ---------------------------------------
            // Parent ran rows.start..mid, worker ran mid..rows_end; the sum
            // of both deltas equals the serial delta exactly (each side's
            // per-row/per-element bump pattern is replica-independent). The
            // trailing bump stays parent-side in fill_slice.
            self.interpolation_counter += done.interpolation_counter - fork_ic;
            self.array_interpolation_counter += done.array_interpolation_counter - fork_aic;
            // ---- (c) FINAL SCALAR STATE ----------------------------------
            // Every scalar the row body mutates (directly or through
            // fill_array's Provider::Slice context): copy the worker's end
            // value — its rows are the serial TAIL, so its end state is the
            // serial end state (the ic/aic epochs it embeds are adjusted by
            // the parent deltas measured above).
            self.cell_start_block_z = done.cell_start_block_z;
            self.in_cell_z = done.in_cell_z;
            self.cell_start_block_y = done.cell_start_block_y;
            self.in_cell_y = done.in_cell_y;
            self.array_index = done.array_index;
            // CacheOnce memo states: the freshest store decides. If the
            // worker stored (store epoch > fork epoch — its rows are the
            // tail), its store IS the serial-last store: copy content and
            // shift the epoch by the parent's delta so it equals the serial
            // epoch bit for bit. If it never stored, the parent's own stores
            // (rows start..mid) are the serial-last stores — keep them.
            for id in 0..self.cacheonces.len() {
                let d = &done.cacheonces[id];
                let s = &mut self.cacheonces[id];
                if d.last_counter > fork_ic {
                    s.last_counter = d.last_counter + d_ic;
                    s.last_value = d.last_value;
                }
                if d.last_array_counter > fork_aic {
                    s.last_array = d.last_array.clone();
                    s.last_array_counter = d.last_array_counter + d_aic;
                }
            }
            // Cache2D: position-keyed pure memo (no epoch) — value-
            // transparent in ANY state, so no copy-back (see fork comment).
            // ---- (d) memo replicas: last_tile / ap2_scratch NOT merged ---
            // last_tile: value-transparent (pure-function slots; a stale
            // slot recomputes identical bits — see fork comment).
            // ap2_scratch: scratch cleared+resized before every use, and its
            // len/capacity are unobservable in values.
            #[cfg(ncf_profile)]
            N2_MERGE_NANOS.fetch_add(
                n2_t_merge.elapsed().as_nanos() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        #[cfg(ncf_profile)]
        N2_SCOPE_NANOS.fetch_add(
            n2_t_scope.elapsed().as_nanos() as u64,
            std::sync::atomic::Ordering::Relaxed,
        );
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
#[inline]
fn chunk_as_long(x: i32, z: i32) -> i64 {
    (x as i64 & 0xFFFF_FFFF) | ((z as i64 & 0xFFFF_FFFF) << 32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::{RandomState, WorldgenDir};

    /// The PAR_FORCE override is process-global — serialize the flips so the
    /// NP1 tests' serial/parallel drives each run under their own forced
    /// mode. (Unrelated tests that drive sims concurrently may observe the
    /// forced arm; their outputs are arm-invariant, so they stay green.)
    static GATE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn bits(v: &[f64]) -> Vec<u64> {
        v.iter().map(|x| x.to_bits()).collect()
    }

    /// One serial drive + one parallel drive on identical FRESH machines
    /// built from the same RandomState (NP1: cell_count_xz = 4, the real
    /// filler shape; rows = 5). Returns both sims and both drive outputs.
    fn drive_serial_and_parallel(
        rs: &RandomState,
        bx: i32,
        bz: i32,
    ) -> (
        NoiseChunkSim<'_>,
        Vec<(u32, i32, i32, i32, f64)>,
        NoiseChunkSim<'_>,
        Vec<(u32, i32, i32, i32, f64)>,
    ) {
        let _g = GATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        force_par_fill(2); // serial, regardless of env
        let mut serial = NoiseChunkSim::from_random_state(rs, 4, bx, bz);
        let (_n_s, out_s) = serial.drive_and_collect();
        force_par_fill(1); // parallel arm forced on
        let mut par = NoiseChunkSim::from_random_state(rs, 4, bx, bz);
        let (_n_p, out_p) = par.drive_and_collect();
        force_par_fill(0); // back to env
        (serial, out_s, par, out_p)
    }

    /// T1 (shaped on NCF_WG, skips gracefully when the extract is absent —
    /// same pattern as router.rs): the NP1 parallel slice-fill arm must be
    /// BIT-EXACT against the serial path. Full overworld noise drive on
    /// identical machines; compares every collected drive value, all
    /// interpolator slice0/slice1 buffers and the substance cache by f64
    /// BITS (I2), at two chunk origins.
    #[test]
    fn t1_par_fill_rows_bit_identical() {
        let Ok(wg) = std::env::var("NCF_WG") else { return };
        let Ok(dir) = WorldgenDir::load(std::path::Path::new(&wg)) else { return };
        let rs = RandomState::build_overworld(&dir, 3053459).expect("build_overworld");
        for (bx, bz) in [(0, 0), (112, 144)] {
            let (serial, out_s, par, out_p) = drive_serial_and_parallel(&rs, bx, bz);
            assert_eq!(out_s.len(), out_p.len(), "drive output length ({bx},{bz})");
            for (a, b) in out_s.iter().zip(out_p.iter()) {
                assert_eq!(a.0, b.0);
                assert_eq!(a.1, b.1);
                assert_eq!(a.2, b.2);
                assert_eq!(a.3, b.3);
                assert_eq!(
                    a.4.to_bits(),
                    b.4.to_bits(),
                    "drive value diverged at ({},{},{}) interp {} chunk ({bx},{bz})",
                    a.1,
                    a.2,
                    a.3,
                    a.0
                );
            }
            for id in 0..serial.interpolators.len() {
                assert_eq!(
                    bits(&serial.interpolators[id].slice0),
                    bits(&par.interpolators[id].slice0),
                    "slice0 interp {id} chunk ({bx},{bz})"
                );
                assert_eq!(
                    bits(&serial.interpolators[id].slice1),
                    bits(&par.interpolators[id].slice1),
                    "slice1 interp {id} chunk ({bx},{bz})"
                );
            }
            assert_eq!(
                bits(&serial.substance_cache),
                bits(&par.substance_cache),
                "substance cache chunk ({bx},{bz})"
            );
        }
    }

    /// T2 (shaped on NCF_WG): the counter-merge design — interpolation_
    /// counter and array_interpolation_counter must be EQUAL between the
    /// serial and the parallel run (they are the CacheOnce epochs). Also
    /// pins the final scalar state the row body mutates and the CacheOnce
    /// memo state (the copy-back shifts the worker's stored epochs by the
    /// parent deltas, reconstructing the serial state exactly).
    #[test]
    fn t2_par_fill_counters_identical() {
        let Ok(wg) = std::env::var("NCF_WG") else { return };
        let Ok(dir) = WorldgenDir::load(std::path::Path::new(&wg)) else { return };
        let rs = RandomState::build_overworld(&dir, 3053459).expect("build_overworld");
        let (serial, _out_s, par, _out_p) = drive_serial_and_parallel(&rs, 0, 0);
        assert_eq!(
            serial.interpolation_counter, par.interpolation_counter,
            "interpolation_counter"
        );
        assert_eq!(
            serial.array_interpolation_counter, par.array_interpolation_counter,
            "array_interpolation_counter"
        );
        // final scalar state the row body mutates (copy-back (c))
        assert_eq!(serial.cell_start_block_z, par.cell_start_block_z);
        assert_eq!(serial.in_cell_z, par.in_cell_z);
        assert_eq!(serial.cell_start_block_y, par.cell_start_block_y);
        assert_eq!(serial.in_cell_y, par.in_cell_y);
        assert_eq!(serial.array_index, par.array_index);
        // CacheOnce memo state: epoch-shifted copy-back must reconstruct the
        // serial state exactly (last_array included, by f64 bits).
        for id in 0..serial.cacheonces.len() {
            let a = &serial.cacheonces[id];
            let b = &par.cacheonces[id];
            assert_eq!(a.last_counter, b.last_counter, "cacheonce {id} last_counter");
            assert_eq!(
                a.last_array_counter, b.last_array_counter,
                "cacheonce {id} last_array_counter"
            );
            assert_eq!(a.last_value.to_bits(), b.last_value.to_bits(), "cacheonce {id} last_value");
            match (&a.last_array, &b.last_array) {
                (None, None) => {}
                (Some(x), Some(y)) => assert_eq!(bits(x), bits(y), "cacheonce {id} last_array"),
                _ => panic!("cacheonce {id} last_array presence divergence"),
            }
        }
    }

    /// T3: gate semantics of the force override (the env OnceLock shape
    /// itself is process-global — its exact-string behavior is verified by
    /// the NCF_PAR_FILL=0/1 stagediff A/B runs).
    #[test]
    fn t3_par_gate_force_semantics() {
        let _g = GATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        force_par_fill(1);
        assert!(par_fill_enabled());
        force_par_fill(2);
        assert!(!par_fill_enabled());
        force_par_fill(0);
        // mode 0 mirrors the env gate
        assert_eq!(
            par_fill_enabled(),
            std::env::var("NCF_PAR_FILL").map(|v| v == "1").unwrap_or(false)
        );
    }
}
