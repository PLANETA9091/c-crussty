#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::too_many_arguments, clippy::manual_range_contains, clippy::excessive_precision, clippy::type_complexity, clippy::needless_range_loop)]

//! NCF P2.6 + P2.7 — NoiseBasedAquifer + OreVeinifier, bit-exact replica of
//! Purpur 1.21.10 (mojang-mapped `net.minecraft.world.level.levelgen.Aquifer`
//! and `OreVeinifier`, CFR 0.152).
//!
//! Semantics notes (verified against the decompile, see worklog session 5):
//! - `similarity(a, b) = 1.0 - (b - a) / 25.0` (f64), FLOWING_UPDATE_SIMULARITY
//!   = similarity(square(10), square(12)) = -0.76.
//! - Block states are represented as `u32` ids from `filler::StateTable`
//!   (air / water[level=0] / lava[level=0] interned by the caller).
//! - Aquifer noises (barrier / floodedness / spread / lava / erosion / depth)
//!   and the preliminary surface level contain NO `interpolated` markers
//!   (verified by walking the overworld JSON with reference resolution), so
//!   the unbound scalar `Df::compute` produces the same values as the NoiseChunk
//!   wrapped versions (T24) — caches only memoize.
//! - The ore-vein density fields DO contain `interpolated` nodes
//!   (vein_toggle root, two inside vein_ridged) — the caller must pass the
//!   NoiseChunk-interpolated per-block values (from interpolator.rs drive),
//!   never scalar computes, for those three fields.
//! - DEBUG_* SharedConstants flags are false in production; those branches
//!   are omitted (documented deviation = none).

use crate::density::Df;
use crate::jrandom::RandomSource;
use crate::mth;
use crate::xoroshiro::XoroshiroPositionalRandomFactory;
use std::collections::HashMap;

/// FluidKind — the only fluids the overworld aquifer picks (water/lava) plus
/// air for the disabled picker.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluidKind {
    Air,
    Water,
    Lava,
}

/// Aquifer.FluidStatus record (fluidLevel, fluidType).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FluidStatus {
    pub fluid_level: i32,
    pub kind: FluidKind,
}

impl FluidStatus {
    /// FluidStatus.at(y): y < level ? fluidType : AIR.
    #[inline]
    pub fn at(&self, y: i32, air: u32, water: u32, lava: u32) -> u32 {
        if y < self.fluid_level {
            match self.kind {
                FluidKind::Air => air,
                FluidKind::Water => water,
                FluidKind::Lava => lava,
            }
        } else {
            air
        }
    }
}

/// The overworld global fluid picker (NoiseBasedChunkGenerator.createFluidPicker):
/// lava status at level -54, water status at sea level; y < min(-54, seaLevel)
/// -> lava else water. (default_fluid is minecraft:water for every noise
/// settings that ships with aquifers enabled; documented.)
#[derive(Clone, Copy)]
pub struct GlobalFluidPicker {
    pub sea_level: i32,
}

impl GlobalFluidPicker {
    #[inline]
    pub fn compute_fluid(&self, y: i32) -> FluidStatus {
        // (x, z) unused by the vanilla overworld picker.
        if y < self.sea_level.min(-54) {
            FluidStatus { fluid_level: -54, kind: FluidKind::Lava }
        } else {
            FluidStatus { fluid_level: self.sea_level, kind: FluidKind::Water }
        }
    }
}

const X_RANGE: i32 = 10;
const Y_RANGE: i32 = 9;
const Z_RANGE: i32 = 10;
/// FLOWING_UPDATE_SIMULARITY = similarity(square(10), square(12)).
const FLOWING_UPDATE_SIMULARITY: f64 = -0.76; // 1.0 - (144 - 100)/25.0
/// SURFACE_SAMPLING_OFFSETS_IN_CHUNKS (Aquifer.java line 95).
const SURFACE_SAMPLING_OFFSETS_IN_CHUNKS: [(i32, i32); 13] = [
    (0, 0),
    (-2, -1),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-3, 0),
    (-2, 0),
    (-1, 0),
    (1, 0),
    (-2, 1),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[inline]
fn similarity(first: i32, second: i32) -> f64 {
    1.0 - (second - first) as f64 / 25.0
}

/// BlockPos.asLong: x 26 bits @38, z 26 bits @12, y 12 bits @0 (java Blocks).
#[inline]
pub fn block_pos_as_long(x: i32, y: i32, z: i32) -> i64 {
    ((x as i64 & 0x3FF_FFFF) << 38) | ((z as i64 & 0x3FF_FFFF) << 12) | (y as i64 & 0xFFF)
}
#[inline]
fn block_pos_get_x(l: i64) -> i32 {
    (l >> 38) as i32
}
#[inline]
fn block_pos_get_y(l: i64) -> i32 {
    ((l << 52) >> 52) as i32
}
#[inline]
fn block_pos_get_z(l: i64) -> i32 {
    ((l << 26) >> 38) as i32
}

/// P5.3 increment 2c — the two DIFFERENT bases of the iterateNoiseColumn
/// machine's aquifer (NoiseBasedChunkGenerator.iterateNoiseColumn CFR 182 ->
/// NoiseChunk ctor CFR 132-140 -> Aquifer ctor CFR 106-111), exposed for
/// oracle tests. Java semantics:
///   - GRID basis = the SECTION containing the cell-aligned firstNoise:
///     `SectionPos.blockToSectionCoord(i) = i >> 4` (arithmetic), min block
///     = section << 4, max = min + 15;
///   - FlatCache WINDOW = the CELL: origin `QuartPos.fromBlock(firstNoise)
///     = firstNoise >> 2`, size `noiseSizeXZ + 1 = 1 + 1 = 2`.
pub fn column_aquifer_grid_basis(
    first_cell_block_x: i32,
    first_cell_block_z: i32,
) -> (i32, i32, i32, i32) {
    let sx = (first_cell_block_x >> 4) << 4;
    let sz = (first_cell_block_z >> 4) << 4;
    (sx, sx + 15, sz, sz + 15)
}

pub fn column_flat_window(first_cell_block_x: i32, first_cell_block_z: i32) -> (i32, i32, i32) {
    (first_cell_block_x >> 2, first_cell_block_z >> 2, 2)
}

#[inline]
fn grid_x(x: i32) -> i32 {
    x >> 4
}
#[inline]
fn from_grid_x(grid_x: i32, offset: i32) -> i32 {
    (grid_x << 4) + offset
}
#[inline]
fn grid_y(y: i32) -> i32 {
    y.div_euclid(12)
}
#[inline]
fn from_grid_y(grid_y: i32, offset: i32) -> i32 {
    grid_y * 12 + offset
}
#[inline]
fn grid_z(z: i32) -> i32 {
    z >> 4
}
#[inline]
fn from_grid_z(grid_z: i32, offset: i32) -> i32 {
    (grid_z << 4) + offset
}

/// OverworldBiomeBuilder.isDeepDarkRegion (javap-verified constants):
/// erosion < (double)(float)-0.225 && depth > (double)(float)0.9.
///
/// NOISECHUNK-WRAP SEMANTICS (worklog addendum 26, bit-exact probe-verified):
/// the aquifer's erosion/depth fields come from noiseRouter1 =
/// noiseRouter.mapAll(NoiseChunk::wrap) — the router's erosion field is a
/// `flat_cache`-rooted subtree and depth contains `flat_cache(overworld/offset)`,
/// and NoiseChunk.wrap replaces flat_cache markers with NoiseChunk.FlatCache
/// whose values are precomputed AT QUART-ALIGNED COLUMNS (QuartPos.toBlock(
/// QuartPos.fromBlock(x)) = x & !3) for the machine's 5x5 quart window
/// (firstNoiseX = machineMinX >> 2, sizeXZ = 4 + 1); OUTSIDE that window
/// FlatCache.compute falls back to the raw per-block evaluate. The wrapped
/// content is y-free (climate splines / y_scale=0 noises), so only the x/z
/// quantization matters. Both veccheck (unwrapped passthrough) and the
/// interpolated final_density (cell-corner aligned ⇒ idempotent) are
/// unaffected; ONLY the aquifer's status layer queries these fields at
/// ARBITRARY blocks — the resid T38-B divergence root.

/// NoiseBasedAquifer replica. The density fields are OWNED machine views:
/// each is the optimized router tree with every Marker::FlatCache bound to
/// THIS machine's window (Df::FlatCacheWindow, addendum 35) — the scalar
/// path then reproduces Java's NoiseChunk.FlatCache quart quantization
/// (in-window = y=0-pinned quart column, out-window = raw filler).
pub struct NoiseBasedAquifer<'a> {
    bank: &'a crate::density::NoiseBank,
    barrier_noise: Df,
    fluid_level_floodedness_noise: Df,
    fluid_level_spread_noise: Df,
    lava_noise: Df,
    erosion: Df,
    depth: Df,
    preliminary_surface_level: Df,
    positional_random_factory: XoroshiroPositionalRandomFactory,
    aquifer_cache: Vec<Option<FluidStatus>>,
    aquifer_location_cache: Vec<i64>,
    global_fluid_picker: GlobalFluidPicker,
    should_schedule_fluid_update: bool,
    skip_sampling_above_y: i32,
    /// NoiseChunk.FlatCache window origin: QuartPos.fromBlock(machineMinX/Z).
    first_noise_x: i32,
    first_noise_z: i32,
    min_grid_x: i32,
    min_grid_y: i32,
    min_grid_z: i32,
    grid_size_x: i32,
    grid_size_z: i32,
    /// NoiseChunk.preliminarySurfaceLevelCache (Long2IntOpenHashMap).
    prelim_cache: HashMap<(i32, i32), i32>,
}

impl<'a> NoiseBasedAquifer<'a> {
    /// Aquifer.create -> new NoiseBasedAquifer(...). `min_block_x/z` are the
    /// chunk min block coords; `min_y`/`height` the (clamped) noise settings.
    /// Grid basis AND FlatCache window both derive from the chunk bounds
    /// (the full-chunk NoiseChunk: noiseSizeXZ = 4 quarts, window size 5).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bank: &'a crate::density::NoiseBank,
        router: &'a crate::router::Router,
        aquifer_factory: XoroshiroPositionalRandomFactory,
        min_y: i32,
        height: i32,
        min_block_x: i32,
        max_block_x: i32,
        min_block_z: i32,
        max_block_z: i32,
        global_fluid_picker: GlobalFluidPicker,
    ) -> Self {
        let flat_size_xz = ((max_block_x - min_block_x + 1) >> 2) + 1;
        let fx = min_block_x >> 2;
        let fz = min_block_z >> 2;
        Self::new_impl(
            bank,
            router,
            aquifer_factory,
            min_y,
            height,
            min_block_x,
            max_block_x,
            min_block_z,
            max_block_z,
            fx,
            fz,
            flat_size_xz,
            global_fluid_picker,
        )
    }

    /// P5.3 increment 2c — the iterateNoiseColumn 1-cell machine's aquifer
    /// (NoiseBasedChunkGenerator.iterateNoiseColumn CFR 182 feeding the
    /// NoiseChunk ctor CFR 132-140). Java uses TWO DIFFERENT bases here:
    ///   - the AQUIFER GRID basis is the SECTION containing the cell-aligned
    ///     firstNoiseX/Z: Aquifer.create(this, new ChunkPos(
    ///     SectionPos.blockToSectionCoord(firstNoiseX), ...)) — 16-block grid
    ///     bounds from that section (Aquifer ctor CFR 106-111);
    ///   - the FlatCache WINDOW for the wrapped router fields (barrier,
    ///     floodedness, erosion, depth, ... the aquifer's own bindings per
    ///     addendum 35) is the CELL itself: noiseSizeXZ = QuartPos.fromBlock
    ///     (1 * cellWidth) = 1, FlatCache size = 2, origin = firstNoise >> 2.
    ///     Erosion/depth/floodedness queries at aquifer FLUID positions
    ///     (fromGridX(g, r), r in 0..9, Aquifer CFR 155-160) can land inside
    ///     the section quarts but OUTSIDE the cell quarts — Java's 1-cell
    ///     machine computes those DIRECTLY (out-of-window fallback), while a
    ///     section-sized window would quart-snap them. `first_cell_block_x/z`
    ///     are the CELL-ALIGNED coords (i6/i7 = floorDiv(x, cw)*cw).
    #[allow(clippy::too_many_arguments)]
    pub fn new_for_column(
        bank: &'a crate::density::NoiseBank,
        router: &'a crate::router::Router,
        aquifer_factory: XoroshiroPositionalRandomFactory,
        min_y: i32,
        height: i32,
        first_cell_block_x: i32,
        first_cell_block_z: i32,
        global_fluid_picker: GlobalFluidPicker,
    ) -> Self {
        let (gx0, gx1, gz0, gz1) = column_aquifer_grid_basis(first_cell_block_x, first_cell_block_z);
        let (fx, fz, size) = column_flat_window(first_cell_block_x, first_cell_block_z);
        Self::new_impl(
            bank,
            router,
            aquifer_factory,
            min_y,
            height,
            gx0,
            gx1,
            gz0,
            gz1,
            fx, // FlatCache origin = QuartPos.fromBlock(firstNoiseX)
            fz,
            size, // noiseSizeXZ (1) + 1
            global_fluid_picker,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_impl(
        bank: &'a crate::density::NoiseBank,
        router: &'a crate::router::Router,
        aquifer_factory: XoroshiroPositionalRandomFactory,
        min_y: i32,
        height: i32,
        min_block_x: i32,
        max_block_x: i32,
        min_block_z: i32,
        max_block_z: i32,
        fx: i32,
        fz: i32,
        flat_size_xz: i32,
        global_fluid_picker: GlobalFluidPicker,
    ) -> Self {
        let min_grid_x = grid_x(min_block_x - 5);
        let i = grid_x(max_block_x - 5) + 1;
        let grid_size_x = i - min_grid_x + 1;
        let min_grid_y = grid_y(min_y + 1) - 1;
        let i1 = grid_y(min_y + height + 1) + 1;
        let grid_size_y = i1 - min_grid_y + 1;
        let min_grid_z = grid_z(min_block_z - 5);
        let i3 = grid_z(max_block_z - 5) + 1;
        let grid_size_z = i3 - min_grid_z + 1;
        let cache_len = (grid_size_x as usize) * (grid_size_y as usize) * (grid_size_z as usize);
        // Addendum 35: bind every Marker::FlatCache in the aquifer-relevant
        // fields to THIS machine's window. size_xz = NoiseChunk.noiseSizeXZ+1
        // = (chunk width in blocks >> 2) + 1 = 5 for 16-wide chunks; the 2c
        // column machine passes the CELL window (origin = firstNoise >> 2,
        // size 2) via new_for_column.
        let bind = |df: &Df| {
            crate::density::with_flat_cache_windows(df, fx, fz, flat_size_xz)
        };
        let mut aquifer = NoiseBasedAquifer {
            bank,
            barrier_noise: bind(&router.barrier),
            fluid_level_floodedness_noise: bind(&router.fluid_level_floodedness),
            fluid_level_spread_noise: bind(&router.fluid_level_spread),
            lava_noise: bind(&router.lava),
            erosion: bind(&router.erosion),
            depth: bind(&router.depth),
            preliminary_surface_level: bind(&router.preliminary_surface_level),
            positional_random_factory: aquifer_factory,
            aquifer_cache: vec![None; cache_len],
            aquifer_location_cache: vec![i64::MAX; cache_len],
            global_fluid_picker,
            should_schedule_fluid_update: false,
            skip_sampling_above_y: 0,
            first_noise_x: fx,
            first_noise_z: fz,
            min_grid_x,
            min_grid_y,
            min_grid_z,
            grid_size_x,
            grid_size_z,
            prelim_cache: HashMap::new(),
        };
        let i5 = {
            let max_prelim = aquifer.max_preliminary_surface_level(
                from_grid_x(min_grid_x, 0),
                from_grid_z(min_grid_z, 0),
                from_grid_x(i, 9),
                from_grid_z(i3, 9),
            );
            aquifer.adjust_surface_level(max_prelim)
        };
        let i6 = grid_y(i5 + 12) + 1; // `gridY(i5 + 12) - -1`
        aquifer.skip_sampling_above_y = from_grid_y(i6, 11) - 1;
        aquifer
    }

    fn index(&self, grid_x: i32, grid_y: i32, grid_z: i32) -> usize {
        let i = (grid_x - self.min_grid_x) as usize;
        let i1 = (grid_y - self.min_grid_y) as usize;
        let i2 = (grid_z - self.min_grid_z) as usize;
        ((i1 * self.grid_size_z as usize) + i2) * (self.grid_size_x as usize) + i
    }

    /// NoiseChunk.maxPreliminarySurfaceLevel — max over the rect, step 4
    /// (quart sampling), inclusive bounds.
    fn max_preliminary_surface_level(&mut self, min_x: i32, min_z: i32, max_x: i32, max_z: i32) -> i32 {
        let mut i = i32::MIN;
        let mut z = min_z;
        while z <= max_z {
            let mut x = min_x;
            while x <= max_x {
                let i3 = self.preliminary_surface_level(x, z);
                if i3 > i {
                    i = i3;
                }
                x += 4;
            }
            z += 4;
        }
        i
    }

    /// NoiseChunk.preliminarySurfaceLevel — quantize to quarts, cache, compute
    /// the (unbound == bound) prelim field at y=0 and Mth.floor it.
    pub fn preliminary_surface_level(&mut self, x: i32, z: i32) -> i32 {
        // QuartPos.toBlock(QuartPos.fromBlock(x)) = x & !3 (round down to 4).
        let x = x & !3;
        let z = z & !3;
        if let Some(&v) = self.prelim_cache.get(&(x, z)) {
            return v;
        }
        let raw = self
            .preliminary_surface_level
            .compute(self.bank, x, 0, z);
        let v = mth::floor(raw);
        self.prelim_cache.insert((x, z), v);
        v
    }

    fn adjust_surface_level(&self, level: i32) -> i32 {
        level + 8
    }

    /// (Superseded, addendum 35: the wrap_coords helper is gone — its
    /// semantics are implemented exactly by Df::FlatCacheWindow inside the
    /// transformed trees, including the y=0 pin that the old helper lacked.)

    /// OverworldBiomeBuilder.isDeepDarkRegion through the NoiseChunk-wrapped
    /// erosion/depth fields. Addendum 35: the FlatCache window semantics now
    /// live INSIDE the transformed field trees (Df::FlatCacheWindow), so the
    /// evaluation is a plain compute — no ad-hoc coordinate shifting.
    #[inline]
    fn is_deep_dark_region(&self, x: i32, y: i32, z: i32) -> bool {
        let e = self.erosion.compute(self.bank, x, y, z);
        if !(e < -0.224_999_994_039_535_52) {
            return false;
        }
        let d = self.depth.compute(self.bank, x, y, z);
        d > 0.899_999_976_158_142_1
    }

    #[inline]
    pub fn should_schedule_fluid_update(&self) -> bool {
        self.should_schedule_fluid_update
    }

    /// Debug: full computeFluid breakdown for the aquacheck bisect rig.
    #[allow(clippy::type_complexity)]
    pub fn debug_compute_fluid(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        air: u32,
        water: u32,
        lava: u32,
    ) -> (FluidStatus, bool, f64, f64, f64, i32, bool) {
        // (final status, deep_dark, floodedness(d3), d2, max_surface_level, fluid_present)
        let fluid_status = self.global_fluid_picker.compute_fluid(y);
        let mut i = i32::MAX;
        let i1 = y + 12;
        let i2 = y - 12;
        let mut flag = false;
        #[allow(unused_assignments)]
        let mut deep_dark = false;
        let mut d3 = f64::NAN;
        let mut d2 = f64::NAN;
        for (ox, oz) in SURFACE_SAMPLING_OFFSETS_IN_CHUNKS {
            let i3 = x + ox * 16;
            let i4 = z + oz * 16;
            let i5 = self.preliminary_surface_level(i3, i4);
            let i6 = self.adjust_surface_level(i5);
            let flag1 = ox == 0 && oz == 0;
            if flag1 && i2 > i6 {
                return (fluid_status, false, f64::NAN, f64::NAN, i as f64, i, false);
            }
            let flag2 = i1 > i6;
            if flag2 || flag1 {
                let probe = self.global_fluid_picker.compute_fluid(i6);
                if probe.at(i6, air, water, lava) != air {
                    if flag1 {
                        flag = true;
                    }
                    if flag2 {
                        return (probe, false, f64::NAN, f64::NAN, i as f64, i, flag);
                    }
                }
            }
            i = i.min(i5);
        }
        deep_dark = self.is_deep_dark_region(x, y, z);
        let level = if deep_dark {
            -32512
        } else {
            let ii = i + 8 - y;
            d2 = if flag { mth::clamped_map(ii as f64, 0.0, 64.0, 1.0, 0.0) } else { 0.0 };
            d3 = mth::clamp(self.fluid_level_floodedness_noise.compute(self.bank, x, y, z), -1.0, 1.0);
            let d4 = mth::map(d2, 1.0, 0.0, -0.3, 0.8);
            let d5 = mth::map(d2, 1.0, 0.0, -0.8, 0.4);
            let d = d3 - d5;
            let d1 = d3 - d4;
            if d1 > 0.0 {
                fluid_status.fluid_level
            } else if d > 0.0 {
                self.compute_randomized_fluid_surface_level(x, y, z, i)
            } else {
                -32512
            }
        };
        let kind = self.compute_fluid_type(x, y, z, fluid_status, level);
        (FluidStatus { fluid_level: level, kind }, deep_dark, d3, d2, i as f64, i, flag)
    }

    /// Debug: the grid->slot index mapping (aquacheck bisect rig).
    pub fn debug_index(&self, grid_x: i32, grid_y: i32, grid_z: i32) -> usize {
        self.index(grid_x, grid_y, grid_z)
    }

    /// Debug: raw aquifer-relevant field values at a position
    /// (erosion, depth, floodedness, spread, lava, is_deep_dark verdict).
    /// `flat_cache`-wrapped fields are ALSO evaluated at the quart-aligned
    /// column (NoiseChunk.FlatCache semantics) for the bisect rig.
    pub fn debug_fields(&self, x: i32, y: i32, z: i32) -> (f64, f64, f64, f64, f64, bool) {
        let e = self.erosion.compute(self.bank, x, y, z);
        let d = self.depth.compute(self.bank, x, y, z);
        let f = self.fluid_level_floodedness_noise.compute(self.bank, x, y, z);
        let s = self.fluid_level_spread_noise.compute(self.bank, x, y, z);
        let l = self.lava_noise.compute(self.bank, x, y, z);
        let dd = self.is_deep_dark_region(x, y, z);
        (e, d, f, s, l, dd)
    }

    /// Debug: quart-aligned (NoiseChunk.FlatCache-style) erosion/depth at the
    /// column of (x, z) — the values the REAL NoiseChunk-wrapped aquifer sees.
    pub fn debug_fields_quart(&self, x: i32, y: i32, z: i32) -> (f64, f64, bool) {
        let qx = x & !3;
        let qz = z & !3;
        let e = self.erosion.compute(self.bank, qx, y, qz);
        let d = self.depth.compute(self.bank, qx, y, qz);
        let dd = e < -0.224_999_994_039_535_52 && d > 0.899_999_976_158_142_1;
        (e, d, dd)
    }

    /// Meta snapshot for the aquacheck bisect rig (task 5).
    pub fn meta(&self) -> (i32, i32, i32, i32, i32, i32) {
        (
            self.skip_sampling_above_y,
            self.min_grid_x,
            self.min_grid_y,
            self.min_grid_z,
            self.grid_size_x,
            self.grid_size_z,
        )
    }
    pub fn locations(&self) -> &[i64] {
        &self.aquifer_location_cache
    }
    pub fn fluid_statuses(&self) -> &[Option<FluidStatus>] {
        &self.aquifer_cache
    }

    /// Aquifer.computeSubstance. Returns Some(state id) = the block to place,
    /// None = solid (caller places the default block).
    pub fn compute_substance(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        substance: f64,
        air: u32,
        water: u32,
        lava: u32,
    ) -> Option<u32> {
        if substance > 0.0 {
            self.should_schedule_fluid_update = false;
            return None;
        }
        let fluid_status = self.global_fluid_picker.compute_fluid(y);
        if y > self.skip_sampling_above_y {
            self.should_schedule_fluid_update = false;
            return Some(fluid_status.at(y, air, water, lava));
        }
        if fluid_status.at(y, air, water, lava) == lava {
            // fluidStatus.at(y).is(LAVA)
            self.should_schedule_fluid_update = false;
            return Some(lava);
        }
        let i3 = grid_x(x - 5);
        let i4 = grid_y(y + 1);
        let i5 = grid_z(z - 5);
        // four-distance selection (order + >= semantics replicated verbatim)
        let mut i6: i32 = i32::MAX;
        let mut i7: i32 = i32::MAX;
        let mut i8: i32 = i32::MAX;
        let mut i9: i32 = i32::MAX;
        let mut i10: usize = 0;
        let mut i11: usize = 0;
        let mut i12: usize = 0;
        let mut i13: usize = 0;
        for i14 in 0..=1i32 {
            for i15 in -1..=1i32 {
                for i16 in 0..=1i32 {
                    let i17 = i3 + i14;
                    let i18 = i4 + i15;
                    let i19 = i5 + i16;
                    let index = self.index(i17, i18, i19);
                    let cached = self.aquifer_location_cache[index];
                    let l1 = if cached != i64::MAX {
                        cached
                    } else {
                        let mut random_source = self.positional_random_factory.at(i17, i18, i19);
                        let l = block_pos_as_long(
                            from_grid_x(i17, random_source.next_int_bound(X_RANGE)),
                            from_grid_y(i18, random_source.next_int_bound(Y_RANGE)),
                            from_grid_z(i19, random_source.next_int_bound(Z_RANGE)),
                        );
                        self.aquifer_location_cache[index] = l;
                        l
                    };
                    let i20 = block_pos_get_x(l1) - x;
                    let i21 = block_pos_get_y(l1) - y;
                    let i22 = block_pos_get_z(l1) - z;
                    let i23 = i20 * i20 + i21 * i21 + i22 * i22;
                    if i6 >= i23 {
                        i13 = i12;
                        i12 = i11;
                        i11 = i10;
                        i10 = index;
                        i9 = i8;
                        i8 = i7;
                        i7 = i6;
                        i6 = i23;
                        continue;
                    }
                    if i7 >= i23 {
                        i13 = i12;
                        i12 = i11;
                        i11 = index;
                        i9 = i8;
                        i8 = i7;
                        i7 = i23;
                        continue;
                    }
                    if i8 >= i23 {
                        i13 = i12;
                        i12 = index;
                        i9 = i8;
                        i8 = i23;
                        continue;
                    }
                    if i9 < i23 {
                        continue;
                    }
                    i13 = index;
                    i9 = i23;
                }
            }
        }
        let aquifer_status = self.get_aquifer_status(i10, air, water, lava);
        let d = similarity(i6, i7);
        // blockState = aquiferStatus.at(y); blockState1 = blockState (no
        // DEBUG_DISABLE_FLUID_GENERATION).
        let block_state = aquifer_status.at(y, air, water, lava);
        if d <= 0.0 {
            let aquifer_status1 = self.get_aquifer_status(i11, air, water, lava);
            self.should_schedule_fluid_update =
                d >= FLOWING_UPDATE_SIMULARITY && aquifer_status != aquifer_status1;
            return Some(block_state);
        }
        if block_state == water
            && self
                .global_fluid_picker
                .compute_fluid(y - 1)
                .at(y - 1, air, water, lava)
                == lava
        {
            self.should_schedule_fluid_update = true;
            return Some(block_state);
        }
        // MutableDouble(Double.NaN) — barrier memo per block.
        let mut barrier_memo: Option<f64> = None;
        let aquifer_status2 = self.get_aquifer_status(i11, air, water, lava);
        let d1 = d * self.calculate_pressure(x, y, z, &mut barrier_memo, aquifer_status, aquifer_status2, air, water, lava);
        if substance + d1 > 0.0 {
            self.should_schedule_fluid_update = false;
            return None;
        }
        let aquifer_status3 = self.get_aquifer_status(i12, air, water, lava);
        let d2 = similarity(i6, i8);
        if d2 > 0.0 {
            let d3 = d * d2 * self.calculate_pressure(x, y, z, &mut barrier_memo, aquifer_status, aquifer_status3, air, water, lava);
            if substance + d3 > 0.0 {
                self.should_schedule_fluid_update = false;
                return None;
            }
        }
        let d32 = similarity(i7, i8);
        if d32 > 0.0 {
            let d4 = d * d32 * self.calculate_pressure(x, y, z, &mut barrier_memo, aquifer_status2, aquifer_status3, air, water, lava);
            if substance + d4 > 0.0 {
                self.should_schedule_fluid_update = false;
                return None;
            }
        }
        let flag = aquifer_status != aquifer_status2;
        let flag1 = d32 >= FLOWING_UPDATE_SIMULARITY && aquifer_status2 != aquifer_status3;
        let flag2 = d2 >= FLOWING_UPDATE_SIMULARITY && aquifer_status != aquifer_status3;
        self.should_schedule_fluid_update = if !flag && !flag1 && !flag2 {
            d2 >= FLOWING_UPDATE_SIMULARITY
                && similarity(i6, i9) >= FLOWING_UPDATE_SIMULARITY
                && aquifer_status != self.get_aquifer_status(i13, air, water, lava)
        } else {
            true
        };
        Some(block_state)
    }

    fn get_aquifer_status(&mut self, packed_pos: usize, air: u32, water: u32, lava: u32) -> FluidStatus {
        if let Some(fs) = self.aquifer_cache[packed_pos] {
            return fs;
        }
        let l = self.aquifer_location_cache[packed_pos];
        let fs = self.compute_fluid(block_pos_get_x(l), block_pos_get_y(l), block_pos_get_z(l), air, water, lava);
        self.aquifer_cache[packed_pos] = Some(fs);
        fs
    }

    /// NoiseBasedAquifer.computeFluid — the 13-offset surface scan.
    fn compute_fluid(&mut self, x: i32, y: i32, z: i32, air: u32, water: u32, lava: u32) -> FluidStatus {
        let fluid_status = self.global_fluid_picker.compute_fluid(y);
        let mut i = i32::MAX;
        let i1 = y + 12;
        let i2 = y - 12;
        let mut flag = false;
        for (ox, oz) in SURFACE_SAMPLING_OFFSETS_IN_CHUNKS {
            // SectionPos.sectionToBlockCoord(v) = v * 16
            let i3 = x + ox * 16;
            let i4 = z + oz * 16;
            let i5 = self.preliminary_surface_level(i3, i4);
            let i6 = self.adjust_surface_level(i5);
            let flag1 = ox == 0 && oz == 0;
            if flag1 && i2 > i6 {
                return fluid_status;
            }
            let flag2 = i1 > i6;
            if flag2 || flag1 {
                let probe = self.global_fluid_picker.compute_fluid(i6);
                if probe.at(i6, air, water, lava) != air {
                    if flag1 {
                        flag = true;
                    }
                    if flag2 {
                        return probe;
                    }
                }
            }
            i = i.min(i5);
        }
        let i7 = self.compute_surface_level(x, y, z, fluid_status, i, flag);
        let kind = self.compute_fluid_type(x, y, z, fluid_status, i7);
        FluidStatus { fluid_level: i7, kind }
    }

    fn compute_surface_level(
        &self,
        x: i32,
        y: i32,
        z: i32,
        fluid_status: FluidStatus,
        max_surface_level: i32,
        fluid_present: bool,
    ) -> i32 {
        let (d, d1);
        if self.is_deep_dark_region(x, y, z) {
            d = -1.0;
            d1 = -1.0;
        } else {
            let i = max_surface_level + 8 - y;
            let d2: f64 = if fluid_present { mth::clamped_map(i as f64, 0.0, 64.0, 1.0, 0.0) } else { 0.0 };
            let d3 = mth::clamp(self.fluid_level_floodedness_noise.compute(self.bank, x, y, z), -1.0, 1.0);
            let d4 = mth::map(d2, 1.0, 0.0, -0.3, 0.8);
            let d5 = mth::map(d2, 1.0, 0.0, -0.8, 0.4);
            d = d3 - d5;
            d1 = d3 - d4;
        }
        if d1 > 0.0 {
            fluid_status.fluid_level
        } else if d > 0.0 {
            self.compute_randomized_fluid_surface_level(x, y, z, max_surface_level)
        } else {
            if std::env::var("NCF_AQUA_DEBUG").as_deref() == Ok("1") {
                let dd = self.is_deep_dark_region(x, y, z);
                let d3q = mth::clamp(
                    self.fluid_level_floodedness_noise.compute(self.bank, x & !3, y, z & !3),
                    -1.0,
                    1.0,
                );
                eprintln!(
                    "NCF_AQUA_DEBUG -32512 at ({x},{y},{z}) deep_dark={dd} d={d} d1={d1} msl={max_surface_level} fp={fluid_present} d3_here={} d3_quart={d3q}",
                    mth::clamp(self.fluid_level_floodedness_noise.compute(self.bank, x, y, z), -1.0, 1.0),
                );
            }
            -32512
        }
    }

    fn compute_randomized_fluid_surface_level(&self, x: i32, y: i32, z: i32, max_surface_level: i32) -> i32 {
        let i2 = x.div_euclid(16);
        let i3 = y.div_euclid(40);
        let i4 = z.div_euclid(16);
        let i5 = i3 * 40 + 20;
        let d = self.fluid_level_spread_noise.compute(self.bank, i2, i3, i4) * 10.0;
        // Mth.quantize(value, factor) = floor(value / factor) * factor
        let i7 = mth::floor(d / 3.0) * 3;
        max_surface_level.min(i5 + i7)
    }

    fn compute_fluid_type(&self, x: i32, y: i32, z: i32, fluid_status: FluidStatus, surface_level: i32) -> FluidKind {
        let mut kind = fluid_status.kind;
        if surface_level <= -10 && surface_level != -32512 && kind != FluidKind::Lava {
            let i2 = x.div_euclid(64);
            let i3 = y.div_euclid(40);
            let i4 = z.div_euclid(64);
            let d = self.lava_noise.compute(self.bank, i2, i3, i4);
            if f64::abs(d) > 0.3 {
                kind = FluidKind::Lava;
            }
        }
        kind
    }

    /// NoiseBasedAquifer.calculatePressure. `barrier_memo` = the MutableDouble
    /// NaN memo shared across the (up to three) calls per block.
    fn calculate_pressure(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        barrier_memo: &mut Option<f64>,
        first_fluid: FluidStatus,
        second_fluid: FluidStatus,
        air: u32,
        water: u32,
        lava: u32,
    ) -> f64 {
        // Java checks firstFluid.at(blockY).is(LAVA/WATER) — the state AT THE
        // BLOCK'S Y (at(y) = y < level ? type : AIR) — NOT the raw kind. For
        // sentinel statuses (level -32512) at(y) is AIR above the level.
        let first_state = first_fluid.at(y, air, water, lava);
        let second_state = second_fluid.at(y, air, water, lava);
        let first_is_lava = first_state == lava;
        let first_is_water = first_state == water;
        let second_is_lava = second_state == lava;
        let second_is_water = second_state == water;
        if (first_is_lava && second_is_water) || (first_is_water && second_is_lava) {
            return 2.0;
        }
        let abs = (first_fluid.fluid_level - second_fluid.fluid_level).abs();
        if abs == 0 {
            return 0.0;
        }
        let d = 0.5 * ((first_fluid.fluid_level + second_fluid.fluid_level) as f64);
        let d1 = (y as f64) + 0.5 - d;
        let d2 = abs as f64 / 2.0;
        let d9 = d2 - f64::abs(d1);
        // d1 > 0 ? (d9 > 0 ? d9/1.5 : d9/2.5) : ((3.0 + d9) > 0 ? (3.0+d9)/3.0 : (3.0+d9)/10.0)
        let d11 = if d1 > 0.0 {
            let d10 = d9;
            if d10 > 0.0 {
                d10 / 1.5
            } else {
                d10 / 2.5
            }
        } else {
            let d10 = 3.0 + d9;
            if d10 > 0.0 {
                d10 / 3.0
            } else {
                d10 / 10.0
            }
        };
        let d12 = if d11 >= -2.0 && d11 <= 2.0 {
            match barrier_memo {
                Some(v) => *v,
                None => {
                    let v = self.barrier_noise.compute(self.bank, x, y, z);
                    *barrier_memo = Some(v);
                    v
                }
            }
        } else {
            0.0
        };
        2.0 * (d12 + d11)
    }
}

/// OreVeinifier.VeinType.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VeinType {
    Copper,
    Iron,
}

impl VeinType {
    /// (ore, rawOreBlock, filler) state ids are resolved by the caller.
    #[inline]
    pub fn bounds(&self) -> (i32, i32) {
        match self {
            // COPPER(minY=0, maxY=50), IRON(minY=-60, maxY=-8)
            VeinType::Copper => (0, 50),
            VeinType::Iron => (-60, -8),
        }
    }
}

/// OreVeinifier.create's BlockStateFiller, hoisted into an explicit struct.
/// `toggle`/`ridged`/`gap` values MUST be the NoiseChunk-interpolated per-block
/// values for toggle/ridged (they contain `interpolated` markers); `gap` has
/// none, but the caller passes the bound value anyway (identical).
pub struct OreVeinifierRule {
    pub ore_random: XoroshiroPositionalRandomFactory,
}

impl OreVeinifierRule {
    pub fn compute(
        &self,
        toggle: f64,
        ridged: f64,
        gap: f64,
        x: i32,
        y: i32,
        z: i32,
        ids: &OreStateIds,
    ) -> Option<u32> {
        let vein_type: VeinType = if toggle > 0.0 { VeinType::Copper } else { VeinType::Iron };
        let abs = f64::abs(toggle);
        let (min_y, max_y) = vein_type.bounds();
        let i1 = max_y - y;
        let i2 = y - min_y;
        if i2 < 0 || i1 < 0 {
            return None;
        }
        let min = i1.min(i2);
        let d1 = mth::clamped_map(min as f64, 0.0, 20.0, -0.2, 0.0);
        // (double)0.4f
        if abs + d1 < 0.400_000_005_960_464_5 {
            return None;
        }
        let mut random_source = self.ore_random.at(x, y, z);
        // (double)0.7f
        if random_source.next_f32() > 0.699_999_988_079_071 {
            return None;
        }
        if ridged >= 0.0 {
            return None;
        }
        let d2 = mth::clamped_map(
            abs,
            0.400_000_005_960_464_5,  // (double)0.4f
            0.600_000_023_841_857_9,  // (double)0.6f
            0.100_000_001_490_116_12, // (double)0.1f
            0.300_000_011_920_928_96, // (double)0.3f
        );
        // (double)-0.3f
        if (random_source.next_f32() as f64) < d2 && gap > -0.300_000_011_920_928_96 {
            return Some(if random_source.next_f32() < 0.019_999_999_552_965_164 {
                // (double)0.02f
                ids.raw_ore(vein_type)
            } else {
                ids.ore(vein_type)
            });
        }
        Some(ids.filler(vein_type))
    }
}

/// The six ore-vein block state ids (interned by the caller).
pub struct OreStateIds {
    pub copper_ore: u32,
    pub raw_copper_block: u32,
    pub granite: u32,
    pub deepslate_iron_ore: u32,
    pub raw_iron_block: u32,
    pub tuff: u32,
}

impl OreStateIds {
    #[inline]
    pub fn ore(&self, t: VeinType) -> u32 {
        match t {
            VeinType::Copper => self.copper_ore,
            VeinType::Iron => self.deepslate_iron_ore,
        }
    }
    #[inline]
    pub fn raw_ore(&self, t: VeinType) -> u32 {
        match t {
            VeinType::Copper => self.raw_copper_block,
            VeinType::Iron => self.raw_iron_block,
        }
    }
    #[inline]
    pub fn filler(&self, t: VeinType) -> u32 {
        match t {
            VeinType::Copper => self.granite,
            VeinType::Iron => self.tuff,
        }
    }
}
