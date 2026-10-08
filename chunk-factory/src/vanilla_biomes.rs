//! Vanilla overworld biome parameter table — faithful Rust port of
//! net.minecraft.world.level.biome.OverworldBiomeBuilder (CFR decompile of
//! the mojang-mapped Purpur 1.21.10 jar; reference kept at
//! chunk-factory/docs_overworld_biome_builder.java).
//!
//! Every float span goes through Climate.quantizeCoord (`(long)(f*10000.0f)`,
//! F32 multiply) via climate::Parameter — the f32 literals below are the
//! exact Java literals (e.g. 0.93333334f, 0.26666668f, -0.2225f).
//!
//! Emission ORDER is the gate: ParameterList::build's stable sorts and the
//! RTree's child order depend on the point order, and the live server's
//! list (captured into climate_points.csv by the golden harness) is built by
//! the same code path.
//!
//! Naming note: the CFR decompile names the 6th addSurfaceBiome parameter
//! "depth", but it is the WEIRDNESS slice span (the inland slices are
//! weirdness ranges); the body's `Climate.Parameter.point(0.0f)` is the real
//! DEPTH column, emitted twice (depth 0.0 and 1.0) per surface call. The
//! trailing float of each call is the OFFSET.

use crate::climate::{Parameter, ParameterPoint};

fn span(min: f32, max: f32) -> Parameter {
    Parameter::span(min, max)
}

fn pt(v: f32) -> Parameter {
    Parameter::point(v)
}

/// span(Parameter, Parameter) overload — min.min() .. max.max().
fn span_params(a: &Parameter, b: &Parameter) -> Parameter {
    Parameter { min: a.min, max: b.max }
}

struct Builder {
    points: Vec<(ParameterPoint, &'static str)>,
    full_range: Parameter,
    temperatures: [Parameter; 5],
    humidities: [Parameter; 5],
    erosions: [Parameter; 7],
    frozen_range: Parameter,
    unfrozen_range: Parameter,
    mushroom_fields_c: Parameter,
    deep_ocean_c: Parameter,
    ocean_c: Parameter,
    coast_c: Parameter,
    inland_c: Parameter,
    near_inland_c: Parameter,
    mid_inland_c: Parameter,
    far_inland_c: Parameter,
}

fn biome(name: &'static str) -> &'static str {
    // Biomes.X -> registered name (all vanilla biome paths are lowercase
    // snake of the constant name).
    name
}

impl Builder {
    fn new() -> Self {
        Builder {
            points: Vec::new(),
            full_range: span(-1.0, 1.0),
            temperatures: [
                span(-1.0, -0.45),
                span(-0.45, -0.15),
                span(-0.15, 0.2),
                span(0.2, 0.55),
                span(0.55, 1.0),
            ],
            humidities: [
                span(-1.0, -0.35),
                span(-0.35, -0.1),
                span(-0.1, 0.1),
                span(0.1, 0.3),
                span(0.3, 1.0),
            ],
            erosions: [
                span(-1.0, -0.78),
                span(-0.78, -0.375),
                span(-0.375, -0.2225),
                span(-0.2225, 0.05),
                span(0.05, 0.45),
                span(0.45, 0.55),
                span(0.55, 1.0),
            ],
            frozen_range: span(-1.0, -0.45), // temperatures[0] (same span)
            unfrozen_range: Parameter { min: 0, max: 0 }, // filled below
            mushroom_fields_c: span(-1.2, -1.05),
            deep_ocean_c: span(-1.05, -0.455),
            ocean_c: span(-0.455, -0.19),
            coast_c: span(-0.19, -0.11),
            inland_c: span(-0.11, 0.55),
            near_inland_c: span(-0.11, 0.03),
            mid_inland_c: span(0.03, 0.3),
            far_inland_c: span(0.3, 1.0),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn surface(
        points: &mut Vec<(ParameterPoint, &'static str)>,
        temperature: &Parameter,
        humidity: &Parameter,
        continentalness: &Parameter,
        erosion: &Parameter,
        weirdness: &Parameter,
        offset: f32,
        key: &'static str,
    ) {
        for depth in [0.0f32, 1.0f32] {
            points.push((
                ParameterPoint {
                    temperature: *temperature,
                    humidity: *humidity,
                    continentalness: *continentalness,
                    erosion: *erosion,
                    depth: pt(depth),
                    weirdness: *weirdness,
                    offset: crate::climate::quantize_coord(offset),
                },
                key,
            ));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn underground(
        points: &mut Vec<(ParameterPoint, &'static str)>,
        temperature: &Parameter,
        humidity: &Parameter,
        continentalness: &Parameter,
        erosion: &Parameter,
        weirdness: &Parameter,
        offset: f32,
        key: &'static str,
    ) {
        points.push((
            ParameterPoint {
                temperature: *temperature,
                humidity: *humidity,
                continentalness: *continentalness,
                erosion: *erosion,
                depth: span(0.2, 0.9),
                weirdness: *weirdness,
                offset: crate::climate::quantize_coord(offset),
            },
            key,
        ));
    }

    #[allow(clippy::too_many_arguments)]
    fn bottom(
        points: &mut Vec<(ParameterPoint, &'static str)>,
        temperature: &Parameter,
        humidity: &Parameter,
        continentalness: &Parameter,
        erosion: &Parameter,
        weirdness: &Parameter,
        offset: f32,
        key: &'static str,
    ) {
        points.push((
            ParameterPoint {
                temperature: *temperature,
                humidity: *humidity,
                continentalness: *continentalness,
                erosion: *erosion,
                depth: pt(1.1),
                weirdness: *weirdness,
                offset: crate::climate::quantize_coord(offset),
            },
            key,
        ));
    }

    // ---- pickers (exact port) ------------------------------------------

    fn middle_biomes(&self, t: usize, h: usize) -> &'static str {
        const M: [[&str; 5]; 5] = [
            ["minecraft:snowy_plains", "minecraft:snowy_plains", "minecraft:snowy_plains", "minecraft:snowy_taiga", "minecraft:taiga"],
            ["minecraft:plains", "minecraft:plains", "minecraft:forest", "minecraft:taiga", "minecraft:old_growth_spruce_taiga"],
            ["minecraft:flower_forest", "minecraft:plains", "minecraft:forest", "minecraft:birch_forest", "minecraft:dark_forest"],
            ["minecraft:savanna", "minecraft:savanna", "minecraft:forest", "minecraft:jungle", "minecraft:jungle"],
            ["minecraft:desert", "minecraft:desert", "minecraft:desert", "minecraft:desert", "minecraft:desert"],
        ];
        M[t][h]
    }

    fn middle_biomes_variant(&self, t: usize, h: usize) -> Option<&'static str> {
        const V: [[Option<&str>; 5]; 5] = [
            [Some("minecraft:ice_spikes"), None, Some("minecraft:snowy_taiga"), None, None],
            [None, None, None, None, Some("minecraft:old_growth_pine_taiga")],
            [Some("minecraft:sunflower_plains"), None, None, Some("minecraft:old_growth_birch_forest"), None],
            [None, None, Some("minecraft:plains"), Some("minecraft:sparse_jungle"), Some("minecraft:bamboo_jungle")],
            [None, None, None, None, None],
        ];
        V[t][h]
    }

    fn plateau_biomes(&self, t: usize, h: usize) -> &'static str {
        const P: [[&str; 5]; 5] = [
            ["minecraft:snowy_plains", "minecraft:snowy_plains", "minecraft:snowy_plains", "minecraft:snowy_taiga", "minecraft:snowy_taiga"],
            ["minecraft:meadow", "minecraft:meadow", "minecraft:forest", "minecraft:taiga", "minecraft:old_growth_spruce_taiga"],
            ["minecraft:meadow", "minecraft:meadow", "minecraft:meadow", "minecraft:meadow", "minecraft:pale_garden"],
            ["minecraft:savanna_plateau", "minecraft:savanna_plateau", "minecraft:forest", "minecraft:forest", "minecraft:jungle"],
            ["minecraft:badlands", "minecraft:badlands", "minecraft:badlands", "minecraft:wooded_badlands", "minecraft:wooded_badlands"],
        ];
        P[t][h]
    }

    fn plateau_biomes_variant(&self, t: usize, h: usize) -> Option<&'static str> {
        const V: [[Option<&str>; 5]; 5] = [
            [Some("minecraft:ice_spikes"), None, None, None, None],
            [Some("minecraft:cherry_grove"), None, Some("minecraft:meadow"), Some("minecraft:meadow"), Some("minecraft:old_growth_pine_taiga")],
            [Some("minecraft:cherry_grove"), Some("minecraft:cherry_grove"), Some("minecraft:forest"), Some("minecraft:birch_forest"), None],
            [None, None, None, None, None],
            [Some("minecraft:eroded_badlands"), Some("minecraft:eroded_badlands"), None, None, None],
        ];
        V[t][h]
    }

    fn shattered_biomes(&self, t: usize, h: usize) -> Option<&'static str> {
        const S: [[Option<&str>; 5]; 5] = [
            [Some("minecraft:windswept_gravelly_hills"), Some("minecraft:windswept_gravelly_hills"), Some("minecraft:windswept_hills"), Some("minecraft:windswept_forest"), Some("minecraft:windswept_forest")],
            [Some("minecraft:windswept_gravelly_hills"), Some("minecraft:windswept_gravelly_hills"), Some("minecraft:windswept_hills"), Some("minecraft:windswept_forest"), Some("minecraft:windswept_forest")],
            [Some("minecraft:windswept_hills"), Some("minecraft:windswept_hills"), Some("minecraft:windswept_hills"), Some("minecraft:windswept_forest"), Some("minecraft:windswept_forest")],
            [None, None, None, None, None],
            [None, None, None, None, None],
        ];
        S[t][h]
    }

    fn oceans_deep(&self, t: usize) -> &'static str {
        const O: [&str; 5] = [
            "minecraft:deep_frozen_ocean",
            "minecraft:deep_cold_ocean",
            "minecraft:deep_ocean",
            "minecraft:deep_lukewarm_ocean",
            "minecraft:warm_ocean",
        ];
        O[t]
    }

    fn oceans_shallow(&self, t: usize) -> &'static str {
        const O: [&str; 5] = [
            "minecraft:frozen_ocean",
            "minecraft:cold_ocean",
            "minecraft:ocean",
            "minecraft:lukewarm_ocean",
            "minecraft:warm_ocean",
        ];
        O[t]
    }

    fn pick_middle_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if param.max < 0 {
            return self.middle_biomes(t, h);
        }
        match self.middle_biomes_variant(t, h) {
            Some(v) => v,
            None => self.middle_biomes(t, h),
        }
    }

    fn pick_badlands_biome(&self, h: usize, param: &Parameter) -> &'static str {
        if h < 2 {
            if param.max < 0 {
                biome("minecraft:badlands")
            } else {
                biome("minecraft:eroded_badlands")
            }
        } else if h < 3 {
            biome("minecraft:badlands")
        } else {
            biome("minecraft:wooded_badlands")
        }
    }

    fn pick_middle_biome_or_badlands_if_hot(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if t == 4 {
            self.pick_badlands_biome(h, param)
        } else {
            self.pick_middle_biome(t, h, param)
        }
    }

    fn pick_slope_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if t >= 3 {
            return self.pick_plateau_biome(t, h, param);
        }
        if h <= 1 {
            biome("minecraft:snowy_slopes")
        } else {
            biome("minecraft:grove")
        }
    }

    fn pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if t == 0 {
            self.pick_slope_biome(t, h, param)
        } else {
            self.pick_middle_biome_or_badlands_if_hot(t, h, param)
        }
    }

    fn maybe_pick_windswept_savanna_biome(&self, t: usize, h: usize, param: &Parameter, key: &'static str) -> &'static str {
        if t > 1 && h < 4 && param.max >= 0 {
            biome("minecraft:windswept_savanna")
        } else {
            key
        }
    }

    fn pick_beach_biome(&self, t: usize, _h: usize) -> &'static str {
        if t == 0 {
            biome("minecraft:snowy_beach")
        } else if t == 4 {
            biome("minecraft:desert")
        } else {
            biome("minecraft:beach")
        }
    }

    fn pick_shattered_coast_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        let k = if param.max >= 0 {
            self.pick_middle_biome(t, h, param)
        } else {
            self.pick_beach_biome(t, h)
        };
        self.maybe_pick_windswept_savanna_biome(t, h, param, k)
    }

    fn pick_plateau_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if param.max >= 0 {
            if let Some(v) = self.plateau_biomes_variant(t, h) {
                return v;
            }
        }
        self.plateau_biomes(t, h)
    }

    fn pick_peak_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        if t <= 2 {
            return if param.max < 0 {
                biome("minecraft:jagged_peaks")
            } else {
                biome("minecraft:frozen_peaks")
            };
        }
        if t == 3 {
            biome("minecraft:stony_peaks")
        } else {
            self.pick_badlands_biome(h, param)
        }
    }

    fn pick_shattered_biome(&self, t: usize, h: usize, param: &Parameter) -> &'static str {
        match self.shattered_biomes(t, h) {
            Some(v) => v,
            None => self.pick_middle_biome(t, h, param),
        }
    }

    // ---- slices (exact port, ORDER MATTERS) -----------------------------

    fn add_off_coast_biomes(&mut self) {
        Self::surface(&mut self.points, 
            &self.full_range,
            &self.full_range,
            &self.mushroom_fields_c,
            &self.full_range,
            &self.full_range,
            0.0,
            biome("minecraft:mushroom_fields"),
        );
        for i in 0..5 {
            let temperature = self.temperatures[i];
            let deep = self.oceans_deep(i);
            Self::surface(&mut self.points, 
                &temperature,
                &self.full_range,
                &self.deep_ocean_c,
                &self.full_range,
                &self.full_range,
                0.0,
                deep,
            );
            let shallow = self.oceans_shallow(i);
            Self::surface(&mut self.points, 
                &temperature,
                &self.full_range,
                &self.ocean_c,
                &self.full_range,
                &self.full_range,
                0.0,
                shallow,
            );
        }
    }

    fn add_peaks(&mut self, param: &Parameter) {
        for i in 0..5 {
            for i1 in 0..5 {
                let temperature = self.temperatures[i];
                let humidity = self.humidities[i1];
                let rk = self.pick_middle_biome(i, i1, param);
                let rk1 = self.pick_middle_biome_or_badlands_if_hot(i, i1, param);
                let rk2 = self.pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, i1, param);
                let rk3 = self.pick_plateau_biome(i, i1, param);
                let rk4 = self.pick_shattered_biome(i, i1, param);
                let rk5 = self.maybe_pick_windswept_savanna_biome(i, i1, param, rk4);
                let rk6 = self.pick_peak_biome(i, i1, param);
                let c = span_params(&self.coast_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[0], param, 0.0, rk6);
                let c = span_params(&self.coast_c, &self.near_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[1], param, 0.0, rk2);
                let c = span_params(&self.mid_inland_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[1], param, 0.0, rk6);
                let c = span_params(&self.coast_c, &self.near_inland_c);
                let e = span_params(&self.erosions[2], &self.erosions[3]);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &e, param, 0.0, rk);
                let c = span_params(&self.mid_inland_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[2], param, 0.0, rk3);
                Self::surface(&mut self.points, &temperature, &humidity, &self.mid_inland_c, &self.erosions[3], param, 0.0, rk1);
                Self::surface(&mut self.points, &temperature, &humidity, &self.far_inland_c, &self.erosions[3], param, 0.0, rk3);
                let c = span_params(&self.coast_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[4], param, 0.0, rk);
                let c = span_params(&self.coast_c, &self.near_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[5], param, 0.0, rk5);
                let c = span_params(&self.mid_inland_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[5], param, 0.0, rk4);
                let c = span_params(&self.coast_c, &self.far_inland_c);
                Self::surface(&mut self.points, &temperature, &humidity, &c, &self.erosions[6], param, 0.0, rk);
            }
        }
    }

    fn add_high_slice(&mut self, param: &Parameter) {
        for i in 0..5 {
            for i1 in 0..5 {
                let temperature = self.temperatures[i];
                let humidity = self.humidities[i1];
                let rk = self.pick_middle_biome(i, i1, param);
                let rk1 = self.pick_middle_biome_or_badlands_if_hot(i, i1, param);
                let rk2 = self.pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, i1, param);
                let rk3 = self.pick_plateau_biome(i, i1, param);
                let rk4 = self.pick_shattered_biome(i, i1, param);
                let rk5 = self.maybe_pick_windswept_savanna_biome(i, i1, param, rk);
                let rk6 = self.pick_slope_biome(i, i1, param);
                let rk7 = self.pick_peak_biome(i, i1, param);
                Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &self.erosions[0], param, 0.0, rk6);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[0], param, 0.0, rk7);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &self.erosions[1], param, 0.0, rk2);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[1], param, 0.0, rk6);
                let e = span_params(&self.erosions[2], &self.erosions[3]);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.near_inland_c), &e, param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[2], param, 0.0, rk3);
                Self::surface(&mut self.points, &temperature, &humidity, &self.mid_inland_c, &self.erosions[3], param, 0.0, rk1);
                Self::surface(&mut self.points, &temperature, &humidity, &self.far_inland_c, &self.erosions[3], param, 0.0, rk3);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.far_inland_c), &self.erosions[4], param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.near_inland_c), &self.erosions[5], param, 0.0, rk5);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[5], param, 0.0, rk4);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.far_inland_c), &self.erosions[6], param, 0.0, rk);
            }
        }
    }

    fn add_mid_slice(&mut self, param: &Parameter) {
        Self::surface(&mut self.points, &self.full_range, &self.full_range, &self.coast_c, &span_params(&self.erosions[0], &self.erosions[2]), param, 0.0, biome("minecraft:stony_shore"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[1], &self.temperatures[2]), &self.full_range, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:swamp"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[3], &self.temperatures[4]), &self.full_range, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:mangrove_swamp"));
        for i in 0..5 {
            for i1 in 0..5 {
                let temperature = self.temperatures[i];
                let humidity = self.humidities[i1];
                let rk = self.pick_middle_biome(i, i1, param);
                let rk1 = self.pick_middle_biome_or_badlands_if_hot(i, i1, param);
                let rk2 = self.pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, i1, param);
                let rk3 = self.pick_shattered_biome(i, i1, param);
                let rk4 = self.pick_plateau_biome(i, i1, param);
                let rk5 = self.pick_beach_biome(i, i1);
                let rk6 = self.maybe_pick_windswept_savanna_biome(i, i1, param, rk);
                let rk7 = self.pick_shattered_coast_biome(i, i1, param);
                let rk8 = self.pick_slope_biome(i, i1, param);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[0], param, 0.0, rk8);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.mid_inland_c), &self.erosions[1], param, 0.0, rk2);
                Self::surface(&mut self.points, &temperature, &humidity, &self.far_inland_c, &self.erosions[1], param, 0.0, if i == 0 { rk8 } else { rk4 });
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &self.erosions[2], param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &self.mid_inland_c, &self.erosions[2], param, 0.0, rk1);
                Self::surface(&mut self.points, &temperature, &humidity, &self.far_inland_c, &self.erosions[2], param, 0.0, rk4);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.near_inland_c), &self.erosions[3], param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[3], param, 0.0, rk1);
                if param.max < 0 {
                    Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[4], param, 0.0, rk5);
                    Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[4], param, 0.0, rk);
                } else {
                    Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.coast_c, &self.far_inland_c), &self.erosions[4], param, 0.0, rk);
                }
                Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[5], param, 0.0, rk7);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &self.erosions[5], param, 0.0, rk6);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[5], param, 0.0, rk3);
                if param.max < 0 {
                    Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[6], param, 0.0, rk5);
                } else {
                    // decompile line 261: coastContinentalness ALONE (not a
                    // span) — verified against the live server table
                    Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[6], param, 0.0, rk);
                }
                if i == 0 {
                    Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, rk);
                }
            }
        }
    }

    fn add_low_slice(&mut self, param: &Parameter) {
        Self::surface(&mut self.points, &self.full_range, &self.full_range, &self.coast_c, &span_params(&self.erosions[0], &self.erosions[2]), param, 0.0, biome("minecraft:stony_shore"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[1], &self.temperatures[2]), &self.full_range, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:swamp"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[3], &self.temperatures[4]), &self.full_range, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:mangrove_swamp"));
        for i in 0..5 {
            for i1 in 0..5 {
                let temperature = self.temperatures[i];
                let humidity = self.humidities[i1];
                let rk = self.pick_middle_biome(i, i1, param);
                let rk1 = self.pick_middle_biome_or_badlands_if_hot(i, i1, param);
                let rk2 = self.pick_middle_biome_or_badlands_if_hot_or_slope_if_cold(i, i1, param);
                let rk3 = self.pick_beach_biome(i, i1);
                let rk4 = self.maybe_pick_windswept_savanna_biome(i, i1, param, rk);
                let rk5 = self.pick_shattered_coast_biome(i, i1, param);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, rk1);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, rk2);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &span_params(&self.erosions[2], &self.erosions[3]), param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &span_params(&self.erosions[2], &self.erosions[3]), param, 0.0, rk1);
                Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &span_params(&self.erosions[3], &self.erosions[4]), param, 0.0, rk3);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[4], param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[5], param, 0.0, rk5);
                Self::surface(&mut self.points, &temperature, &humidity, &self.near_inland_c, &self.erosions[5], param, 0.0, rk4);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &self.erosions[5], param, 0.0, rk);
                Self::surface(&mut self.points, &temperature, &humidity, &self.coast_c, &self.erosions[6], param, 0.0, rk3);
                if i == 0 {
                    Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.near_inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, rk);
                }
            }
        }
    }

    fn add_valleys(&mut self, param: &Parameter) {
        Self::surface(&mut self.points, &self.frozen_range, &self.full_range, &self.coast_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0,
            if param.max < 0 { biome("minecraft:stony_shore") } else { biome("minecraft:frozen_river") });
        Self::surface(&mut self.points, &self.unfrozen_range, &self.full_range, &self.coast_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0,
            if param.max < 0 { biome("minecraft:stony_shore") } else { biome("minecraft:river") });
        Self::surface(&mut self.points, &self.frozen_range, &self.full_range, &self.near_inland_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, biome("minecraft:frozen_river"));
        Self::surface(&mut self.points, &self.unfrozen_range, &self.full_range, &self.near_inland_c, &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, biome("minecraft:river"));
        Self::surface(&mut self.points, &self.frozen_range, &self.full_range, &span_params(&self.coast_c, &self.far_inland_c), &span_params(&self.erosions[2], &self.erosions[5]), param, 0.0, biome("minecraft:frozen_river"));
        Self::surface(&mut self.points, &self.unfrozen_range, &self.full_range, &span_params(&self.coast_c, &self.far_inland_c), &span_params(&self.erosions[2], &self.erosions[5]), param, 0.0, biome("minecraft:river"));
        Self::surface(&mut self.points, &self.frozen_range, &self.full_range, &self.coast_c, &self.erosions[6], param, 0.0, biome("minecraft:frozen_river"));
        Self::surface(&mut self.points, &self.unfrozen_range, &self.full_range, &self.coast_c, &self.erosions[6], param, 0.0, biome("minecraft:river"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[1], &self.temperatures[2]), &self.full_range, &span_params(&self.inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:swamp"));
        Self::surface(&mut self.points, &span_params(&self.temperatures[3], &self.temperatures[4]), &self.full_range, &span_params(&self.inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:mangrove_swamp"));
        Self::surface(&mut self.points, &self.frozen_range, &self.full_range, &span_params(&self.inland_c, &self.far_inland_c), &self.erosions[6], param, 0.0, biome("minecraft:frozen_river"));
        for i in 0..5 {
            for i1 in 0..5 {
                let temperature = self.temperatures[i];
                let humidity = self.humidities[i1];
                let rk = self.pick_middle_biome_or_badlands_if_hot(i, i1, param);
                Self::surface(&mut self.points, &temperature, &humidity, &span_params(&self.mid_inland_c, &self.far_inland_c), &span_params(&self.erosions[0], &self.erosions[1]), param, 0.0, rk);
            }
        }
    }

    fn add_underground_biomes(&mut self) {
        Self::underground(&mut self.points, &self.full_range, &self.full_range, &span(0.8, 1.0), &self.full_range, &self.full_range, 0.0, biome("minecraft:dripstone_caves"));
        Self::underground(&mut self.points, &self.full_range, &span(0.7, 1.0), &self.full_range, &self.full_range, &self.full_range, 0.0, biome("minecraft:lush_caves"));
        Self::bottom(&mut self.points, &self.full_range, &self.full_range, &self.full_range, &span_params(&self.erosions[0], &self.erosions[1]), &self.full_range, 0.0, biome("minecraft:deep_dark"));
    }

    fn add_inland_biomes(&mut self) {
        self.add_mid_slice(&span(-1.0, -0.93333334));
        self.add_high_slice(&span(-0.93333334, -0.7666667));
        self.add_peaks(&span(-0.7666667, -0.56666666));
        self.add_high_slice(&span(-0.56666666, -0.4));
        self.add_mid_slice(&span(-0.4, -0.26666668));
        self.add_low_slice(&span(-0.26666668, -0.05));
        self.add_valleys(&span(-0.05, 0.05));
        self.add_low_slice(&span(0.05, 0.26666668));
        self.add_mid_slice(&span(0.26666668, 0.4));
        self.add_high_slice(&span(0.4, 0.56666666));
        self.add_peaks(&span(0.56666666, 0.7666667));
        self.add_high_slice(&span(0.7666667, 0.93333334));
        self.add_mid_slice(&span(0.93333334, 1.0));
    }
}

/// The vanilla overworld parameter table in OverworldBiomeBuilder emission
/// order (minecraft:-prefixed registered biome names).
pub fn overworld_points() -> Vec<(ParameterPoint, &'static str)> {
    let mut b = Builder::new();
    b.unfrozen_range = span_params(&b.temperatures[1], &b.temperatures[4]);
    b.add_off_coast_biomes();
    b.add_inland_biomes();
    b.add_underground_biomes();
    b.points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate::ParameterList;

    #[test]
    fn table_shape_and_no_duplicates() {
        let points = overworld_points();
        // Vanilla emits thousands of points (every surface call emits TWO
        // depth rows per (t,h) pair across 13 weirdness slices). The exact
        // count is gated row-by-row against the live server via
        // climate_points.csv; here we pin the structural invariants.
        assert!(points.len() >= 7000 && points.len() <= 8200, "count = {}", points.len());
        // biome names are all minecraft:-prefixed
        for (p, name) in &points {
            assert!(name.starts_with("minecraft:"), "{name}");
            assert!(p.offset >= 0);
            // depth quantization matches the surface/underground rules
            assert!(
                p.depth == Parameter::point(0.0)
                    || p.depth == Parameter::point(1.0)
                    || p.depth == Parameter::span(0.2, 0.9)
                    || p.depth == Parameter::point(1.1),
                "unexpected depth span {p:?}"
            );
        }
    }

    #[test]
    fn search_agrees_with_brute_force_on_real_table() {
        // The RTree search returns A minimizer, not necessarily the same leaf
        // the linear scan picks when several points TIE on fitness (Java has
        // the same property: RTree traversal + the ThreadLocal last-result
        // seed decide which of the tied leaves comes back). So the tree is
        // validated on FITNESS VALUE (must equal the linear minimum), and on
        // the biome name ONLY when that minimum is unique.
        let list = ParameterList::new(
            overworld_points().into_iter().map(|(p, n)| (p, n.to_string())).collect(),
        );
        let mut memo: Option<usize> = None;
        let points = overworld_points();
        let mut checked = 0usize;
        let mut unique_checked = 0usize;
        for qx in [-40i32, -13, -3, 0, 3, 13, 40] {
            for qy in [-8i32, 0, 8, 24, 48] {
                let t = TargetPoint_of(qx * 137, qy * 91, qx * 53 + 7);
                let leaf = list.tree.search(&t, &mut memo);
                let got_fit = points[leaf].0.fitness(&t);
                // linear minimum
                let mut min_fit = i64::MAX;
                let mut min_count = 0usize;
                let mut min_biome = "";
                for (p, n) in &points {
                    let f = p.fitness(&t);
                    if f < min_fit {
                        min_fit = f;
                        min_count = 1;
                        min_biome = n;
                    } else if f == min_fit {
                        min_count += 1;
                    }
                }
                assert_eq!(got_fit, min_fit, "tree missed the minimum at {qx},{qy}");
                if min_count == 1 {
                    assert_eq!(list.names[leaf], min_biome, "unique argmin wrong at {qx},{qy}");
                    unique_checked += 1;
                }
                checked += 1;
            }
        }
        assert!(checked > 0 && unique_checked > 0);
    }

    #[allow(non_snake_case)]
    fn TargetPoint_of(a: i32, b: i32, c: i32) -> crate::climate::TargetPoint {
        use crate::climate::TargetPoint;
        let f = |v: i32| crate::climate::quantize_coord((v % 2000) as f32 / 1000.0);
        TargetPoint {
            temperature: f(a),
            humidity: f(b),
            continentalness: f(c),
            erosion: f(a + b),
            depth: f(b - c),
            weirdness: f(a * 3 + 1),
        }
    }
}
