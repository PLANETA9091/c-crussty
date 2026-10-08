//! NCF P5.3 increment 2c — the REAL FirstFreeHeight sampler.
//!
//! The placer's `FirstFreeHeight` trait so far had only test stubs (2b).
//! The production sampler is Java's `ChunkGenerator.getFirstFreeHeight(x, z)`
//! == `getBaseHeight(x, z, Heightmap.Types.WORLD_SURFACE_WG, level, random)`
//! == `iterateNoiseColumn(..., stoppingState = NOT_AIR)` (NoiseBasedChunk
//! Generator CFR 137-139 + 157-199): a DEDICATED 1-cell NoiseChunk per call
//! (CFR 182: `new NoiseChunk(1, random, i6, i7, clampedSettings, Beardifier
//! Marker, ...)` with i6 = floorDiv(x, cellWidth)*cellWidth), scanning the
//! column TOP-DOWN through the plain noise pipeline (NO surface rules, NO
//! beardifier — BeardifierMarker == our 0.0) and returning the first y whose
//! state is not air, +1 (CFR 195-198); `level.getMinY()` when the column is
//! all air (CFR 139 orElse).
//!
//! Machine-exactness details (all replicated here):
//!   - the block rule per sample = MaterialRuleList over
//!     [aquifer.computeSubstance(ctx, cacheAllInCell(final+BeardifierMarker)),
//!      oreVeins? OreVeinifier.create(...)] with `null -> defaultBlock`
//!     (NoiseChunk ctor CFR 157-166 + iterateNoiseColumn CFR 191-192);
//!   - the 1-cell machine's AQUIFER: grid basis = the SECTION containing the
//!     cell (NoiseChunk ctor CFR 132-140), FlatCache window = the CELL
//!     (noiseSizeXZ = 1 quart, size 2) — two different bases, see
//!     NoiseBasedAquifer::new_for_column;
//!   - the sim is driven like iterateNoiseColumn: two slice fills once,
//!     then per y only the target column's fracs (interpolator::
//!     NoiseChunkSim::drive_column).
//!
//! NEVER consumes the structure RNG (Java: no RandomSource is touched by
//! getBaseHeight) — safe to call inside JigsawPlacement assembly.

use crate::aquifer::{GlobalFluidPicker, NoiseBasedAquifer, OreStateIds, OreVeinifierRule};
use crate::filler::StateTable;
use crate::interpolator::NoiseChunkSim;
use crate::placer::FirstFreeHeight;
use crate::router::RandomState;
use crate::xoroshiro::{XoroshiroPositionalRandomFactory, XoroshiroRandomSource};

/// Per-column WORLD_SURFACE_WG first-free-height source over a wired
/// `RandomState`. Rebuilds the 1-cell machine per call — Java does the same
/// (a fresh NoiseChunk per getBaseHeight call, no shared state).
pub struct ColumnHeightSource<'a> {
    rs: &'a RandomState,
    aquifer_factory: XoroshiroPositionalRandomFactory,
    ore_factory: XoroshiroPositionalRandomFactory,
    table: StateTable,
    air: u32,
    water: u32,
    lava: u32,
    default_block: u32,
    ore_ids: OreStateIds,
    ore_veins_enabled: bool,
    picker: GlobalFluidPicker,
}

impl<'a> ColumnHeightSource<'a> {
    /// aquiferRandom/oreRandom exactly as filler.rs (RandomState.java 47-50:
    /// `random.fromHashOf("aquifer").forkPositional()` over the settings'
    /// random source forked positional from the level seed).
    pub fn new(rs: &'a RandomState, level_seed: i64) -> Self {
        let mut base = XoroshiroRandomSource::new(level_seed);
        let worldgen = base.fork_positional();
        let mut aquifer_src = worldgen.from_hash_of("minecraft:aquifer");
        let aquifer_factory = aquifer_src.fork_positional();
        let mut ore_src = worldgen.from_hash_of("minecraft:ore");
        let ore_factory = ore_src.fork_positional();
        let mut table = StateTable::new();
        let air = table.intern("minecraft:air", &[]);
        let water = table.intern("minecraft:water", &[("level", "0")]);
        let lava = table.intern("minecraft:lava", &[("level", "0")]);
        let default_block = table.intern_canonical(&rs.settings.default_block);
        let ore_ids = OreStateIds {
            copper_ore: table.intern("minecraft:copper_ore", &[]),
            raw_copper_block: table.intern("minecraft:raw_copper_block", &[]),
            granite: table.intern("minecraft:granite", &[]),
            deepslate_iron_ore: table.intern("minecraft:deepslate_iron_ore", &[]),
            raw_iron_block: table.intern("minecraft:raw_iron_block", &[]),
            tuff: table.intern("minecraft:tuff", &[]),
        };
        ColumnHeightSource {
            rs,
            aquifer_factory,
            ore_factory,
            table,
            air,
            water,
            lava,
            default_block,
            ore_ids,
            ore_veins_enabled: rs.settings.ore_veins_enabled,
            picker: GlobalFluidPicker { sea_level: rs.settings.sea_level },
        }
    }

    /// getBaseHeight(x, z, WORLD_SURFACE_WG, level, random): first non-air
    /// state from the top of the clamped noise column, +1; `settings.min_y`
    /// (the clamped level's minY) when the column is all air.
    pub fn get_base_height_world_surface_wg(&mut self, x: i32, z: i32) -> i32 {
        let cw = self.rs.settings.noise_size_horizontal * 4; // NoiseChunk cellWidth
        let i6 = x.div_euclid(cw) * cw; // iterateNoiseColumn CFR 174: i6 = i2 * cellWidth
        let i7 = z.div_euclid(cw) * cw;
        let min_y = self.rs.settings.min_y;
        let height = self.rs.settings.height;
        let mut aquifer = NoiseBasedAquifer::new_for_column(
            &self.rs.bank,
            &self.rs.router,
            &self.rs.prelim_surface_cache,
            self.aquifer_factory,
            min_y,
            height,
            i6,
            i7,
            self.picker,
        );
        let ore_rule = OreVeinifierRule { ore_random: self.ore_factory };
        // iterateNoiseColumn CFR 182: NoiseChunk(1, random, i6, i7, ...) —
        // ONE cell wide, anchored at the cell containing (x, z); the clamped
        // settings' min_y/height; no tile cache (Java has none here).
        let mut sim = NoiseChunkSim::instantiate(
            &self.rs.sim_template,
            &self.rs.bank,
            1,
            i6,
            i7,
            min_y,
            height,
            self.rs.settings.noise_size_horizontal,
            self.rs.settings.noise_size_vertical,
            None,
            0,
        );
        let mut hit: Option<i32> = None;
        let (air, water, lava) = (self.air, self.water, self.lava);
        let default_block = self.default_block;
        let ore_veins_enabled = self.ore_veins_enabled;
        let ore_ids = &self.ore_ids;
        let table = &self.table;
        let ore_ref = &ore_rule;
        let aquifer_ref = &mut aquifer;
        sim.drive_column(x, z, &mut |bx, by, bz, sim| {
            // blockStateRule.calculate: first non-null of
            //   [aquifer.computeSubstance(ctx, substance), ore?]
            // else defaultBlock (NoiseChunk CFR 157-166 + 191-192).
            let substance = sim.substance_value();
            let mut state =
                aquifer_ref.compute_substance(bx, by, bz, substance, air, water, lava);
            if state.is_none() && ore_veins_enabled {
                // OreVeinifier order: toggle, ridged, gap (filler.rs parity).
                let toggle = sim.compute_field(12);
                let ridged = sim.compute_field(13);
                let gap = sim.compute_field(14);
                state = ore_ref.compute(toggle, ridged, gap, bx, by, bz, ore_ids);
            }
            let state = match state {
                Some(s) => s,
                None => default_block,
            };
            // stoppingState = NOT_AIR (Heightmap.Types.WORLD_SURFACE_WG,
            // Heightmap.java CFR 142); hit => OptionalInt.of(i10 + 1).
            let name = table.get(state).name.as_str();
            if name != "minecraft:air" && name != "minecraft:cave_air" && name != "minecraft:void_air" {
                hit = Some(by + 1);
                return true;
            }
            false
        });
        hit.unwrap_or(min_y)
    }
}

impl<'a> FirstFreeHeight for ColumnHeightSource<'a> {
    fn first_free_height(&mut self, x: i32, z: i32) -> i32 {
        self.get_base_height_world_surface_wg(x, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aquifer::{column_aquifer_grid_basis, column_flat_window};
    use crate::density::{Df, MarkerType, NoiseBank};
    use crate::interpolator::{intern_df, Arena, NoiseChunkSim, SimTemplate};

    // ---- 2c aquifer basis helpers (Java CFR oracle math) ----------------

    #[test]
    fn column_grid_basis_is_the_section_of_the_cell() {
        // SectionPos.blockToSectionCoord(i) = i >> 4 (arithmetic); min =
        // section << 4, max = min + 15 (NoiseChunk ctor CFR 132-140).
        assert_eq!(column_aquifer_grid_basis(8, 12), (0, 15, 0, 15));
        assert_eq!(column_aquifer_grid_basis(-4, -8), (-16, -1, -16, -1));
        assert_eq!(column_aquifer_grid_basis(-16, 16), (-16, -1, 16, 31));
        assert_eq!(column_aquifer_grid_basis(-1, 1), (-16, -1, 0, 15));
    }

    #[test]
    fn column_flat_window_is_the_cell() {
        // QuartPos.fromBlock(firstNoise) = firstNoise >> 2; size = 1 + 1 = 2
        // (noiseSizeXZ of a 1-cell machine = QuartPos.fromBlock(cellWidth) = 1).
        assert_eq!(column_flat_window(8, 12), (2, 3, 2));
        assert_eq!(column_flat_window(-4, -8), (-1, -2, 2));
        assert_eq!(column_flat_window(0, 0), (0, 0, 2));
    }

    // ---- drive_column protocol == drive_blocks (1-cell machine) ---------

    /// Synthetic machine: CellCache(Interpolated(YClampedGradient)) per root —
    /// the same wrapper SHAPE as the real final_density root (CacheAllInCell
    /// over the interpolated tree), no noise bank needed. The interpolated
    /// value is y-dependent (trilerp of the gradient cell corners), the
    /// substance cache is filled per cell — both meaningful signals for the
    /// protocol comparison.
    fn synthetic_template() -> SimTemplate {
        let bank = NoiseBank::default();
        let mut arena = Arena::default();
        let grad = Df::YClampedGradient {
            from_y: -64,
            to_y: 320,
            from_value: 1.0,
            to_value: -1.0,
        };
        let tree = Df::Marker {
            ty: MarkerType::CacheAllInCell,
            wrapped: Box::new(Df::Marker {
                ty: MarkerType::Interpolated,
                wrapped: Box::new(grad),
            }),
        };
        let idx = intern_df(&tree, &mut arena, &bank);
        SimTemplate::build(&bank, [idx; 15], arena)
    }

    #[test]
    fn drive_column_matches_drive_blocks_values() {
        let template = synthetic_template();
        let bank = NoiseBank::default();
        // Cells around the origin, incl. negative coords and cell corners.
        let cases: &[(i32, i32, i32, i32)] = &[
            (0, 0, 0, 0),
            (0, 0, 3, 3),
            (0, 0, 1, 2),
            (1, 1, 4, 4),
            (1, 1, 7, 7),
            (1, 1, 5, 6),
            (-1, -2, -4, -8),
            (-1, -2, -1, -5),
            (-3, 2, -12, 10),
        ];
        for &(cx, cz, x, z) in cases {
            let i6 = cx * 4;
            let i7 = cz * 4;
            let mut sim_full =
                NoiseChunkSim::instantiate(&template, &bank, 1, i6, i7, -64, 384, 1, 2, None, 0);
            let mut full: Vec<(i32, f64, f64)> = Vec::new();
            sim_full.drive_blocks(&mut |bx, by, bz, s| {
                if bx == x && bz == z {
                    let v = s.compute_field(0);
                    full.push((by, v, s.substance_value()));
                }
            });
            let mut sim_col =
                NoiseChunkSim::instantiate(&template, &bank, 1, i6, i7, -64, 384, 1, 2, None, 0);
            let mut col: Vec<(i32, f64, f64)> = Vec::new();
            sim_col.drive_column(x, z, &mut |bx, by, bz, s| {
                assert_eq!((bx, bz), (x, z), "drive_column visits only the target column");
                let v = s.compute_field(0);
                col.push((by, v, s.substance_value()));
                false
            });
            assert_eq!(full.len(), 384, "one y per block of the column at ({x},{z})");
            assert_eq!(full, col, "drive_column protocol divergence at ({x},{z})");
        }
    }

    #[test]
    fn drive_column_early_stop_is_resumable() {
        let template = synthetic_template();
        let bank = NoiseBank::default();
        let mut sim =
            NoiseChunkSim::instantiate(&template, &bank, 1, 4, 4, -64, 384, 1, 2, None, 0);
        let mut count = 0;
        sim.drive_column(5, 6, &mut |_bx, by, _bz, _s| {
            count += 1;
            by <= 200 // stoppingState hit: first y at/below 200
        });
        assert!(count < 384, "early stop must skip the rest of the column (got {count})");
        // stopInterpolation: the machine must be reusable after an early stop.
        let mut count2 = 0;
        sim.drive_column(5, 6, &mut |_bx, _by, _bz, _s| {
            count2 += 1;
            false
        });
        assert_eq!(count2, 384);
    }
}
