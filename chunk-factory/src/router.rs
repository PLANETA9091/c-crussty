//! Worldgen datapack loading, density-function parsing and RandomState
//! wiring (NCF P1.1/P1.2/P1.7).
//!
//! Specification source decision (P1.1, recorded): the SOURCE OF TRUTH is the
//! datapack JSON — for vanilla that is `data/minecraft/worldgen/**` inside the
//! server jar, exactly what DFU codecs read when `RandomState.create(...)`
//! builds the router. Re-extracting the IR from live registries would couple
//! the factory to JVM internals; JSON files are byte-stable, versionable and
//! CI-extractable. Datapack packs (Terralith/Tectonic) drop into the same
//! `data/<namespace>/worldgen/...` layout, so one loader covers both.
//!
//! Wiring mirrors `RandomState.<init>` + `NoiseWiringHelper`:
//!   worldgenFactory = XoroshiroRandomSource(levelSeed).forkPositional()
//!     (Legacy lineage when settings.legacy_random_source = true)
//!   NoiseHolder(key)   -> NormalNoise interned per key via
//!                         factory.fromHashOf("minecraft:<key>") (Noises.instantiate)
//!   old_blended_noise  -> withNewRandom(factory.fromHashOf("minecraft:terrain"));
//!                         instances interned by param tuple (vanilla caches
//!                         by record equality — one draw per distinct params)
//!   end_islands        -> level seed (scalar eval unsupported yet)
//!   legacy routers     -> temperature/vegetation become legacy nether noises
//!                         (LegacyRandomSource(levelSeed+0/+1), -7 [1,1]),
//!                         shift becomes NormalNoise(0, [0.0]) on
//!                         fromHashOf("minecraft:offset")

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use crate::density::{
    ap2_create, MarkerType, mapped_create, multi_spline_create, Ap2Type, Df, MappedType, MultiSpline, NoiseBank,
    Rarity, SplineValue,
};
use crate::json::{self, Json};
use crate::jrandom::{LegacyRandomSource, PositionalRandomFactory, RandomSource};
use crate::noise::{BlendedNoise, NormalNoise};
use crate::xoroshiro::XoroshiroRandomSource;

// --------------------------------------------------------------------------
// WorldgenDir — the extracted datapack tree
// --------------------------------------------------------------------------

pub struct WorldgenDir {
    /// namespace -> kind -> relative path -> raw JSON text
    files: BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>>,
}

impl WorldgenDir {
    /// Load `root` = a directory containing `data/<ns>/worldgen/...` (the
    /// shape you get by unzipping a server jar's data/ dir or a datapack).
    pub fn load(root: &Path) -> Result<Self, String> {
        let data = root.join("data");
        if !data.is_dir() {
            return Err(format!("no data/ dir under {}", root.display()));
        }
        let mut files: BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>> =
            BTreeMap::new();
        let mut stack = vec![data.clone()];
        while let Some(dir) = stack.pop() {
            let rd = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;
            for entry in rd.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p);
                    continue;
                }
                if p.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                let rel = p.strip_prefix(&data).map_err(|e| e.to_string())?;
                let parts: Vec<&std::ffi::OsStr> = rel.iter().collect();
                // data/<ns>/worldgen/<kind>/<path...>.json
                if parts.len() < 4 || parts[1] != std::ffi::OsStr::new("worldgen") {
                    continue;
                }
                let ns = parts[0].to_string_lossy().to_string();
                let kind = parts[2].to_string_lossy().to_string();
                let mut rel_path = std::path::PathBuf::new();
                for seg in &parts[3..] {
                    rel_path.push(seg);
                }
                let rel_path = rel_path.with_extension("").to_string_lossy().to_string();
                let text = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
                files
                    .entry(ns)
                    .or_default()
                    .entry(kind)
                    .or_default()
                    .insert(rel_path, text);
            }
        }
        Ok(Self { files })
    }
    pub fn namespaces(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    /// Names of files under data/<ns>/worldgen/<kind>/ (path incl. slashes,
    /// without .json).
    pub fn list(&self, ns: &str, kind: &str) -> Vec<String> {
        self.files
            .get(ns)
            .and_then(|k| k.get(kind))
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Raw JSON text of one file.
    pub fn read(&self, ns: &str, kind: &str, name: &str) -> Option<String> {
        self.get(ns, kind, name).cloned()
    }


    pub fn count(&self, ns: &str, kind: &str) -> usize {
        self.files.get(ns).and_then(|k| k.get(kind)).map(|m| m.len()).unwrap_or(0)
    }

    fn get(&self, ns: &str, kind: &str, name: &str) -> Option<&String> {
        self.files.get(ns)?.get(kind)?.get(name)
    }
}

// --------------------------------------------------------------------------
// Noise parameters (worldgen/noise/*.json)
// --------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct NoiseParams {
    pub first_octave: i32,
    pub amplitudes: Vec<f64>,
}

impl NoiseParams {
    fn parse(v: &Json) -> Result<Self, String> {
        let first_octave = v
            .get("firstOctave")
            .and_then(|j| j.as_i64())
            .ok_or("noise params missing firstOctave")? as i32;
        let amps = v
            .get("amplitudes")
            .and_then(|j| j.as_arr())
            .ok_or("noise params missing amplitudes")?;
        let amplitudes = amps.iter().map(|a| a.as_f64().unwrap_or(0.0)).collect();
        Ok(Self { first_octave, amplitudes })
    }
}

// --------------------------------------------------------------------------
// Raw (unwired) density function tree
// --------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Raw {
    Const(f64),
    YClampedGradient { from_y: i32, to_y: i32, from_value: f64, to_value: f64 },
    Noise { key: String, xz_scale: f64, y_scale: f64 },
    ShiftedNoise {
        shift_x: Box<Raw>,
        shift_y: Box<Raw>,
        shift_z: Box<Raw>,
        xz_scale: f64,
        y_scale: f64,
        key: String,
    },
    ShiftA(String),
    ShiftB(String),
    Shift(String),
    BlendDensity(Box<Raw>),
    Marker { ty: MarkerType, wrapped: Box<Raw> },
    WeirdScaledSampler { input: Box<Raw>, key: String, rarity: Rarity },
    RangeChoice {
        input: Box<Raw>,
        min_inclusive: f64,
        max_exclusive: f64,
        when_in_range: Box<Raw>,
        when_out_of_range: Box<Raw>,
    },
    Clamp { input: Box<Raw>, min: f64, max: f64 },
    Mapped { ty: MappedType, input: Box<Raw> },
    Ap2 { ty: Ap2Type, a1: Box<Raw>, a2: Box<Raw> },
    Spline { coordinate: Box<Raw>, points: Vec<(f32, Raw, f32)> },
    OldBlendedNoise {
        xz_scale: f64,
        y_scale: f64,
        xz_factor: f64,
        y_factor: f64,
        smear_scale_multiplier: f64,
    },
    BlendAlpha,
    BlendOffset,
    Beardifier,
    EndIslands,
    FindTopSurface { density: Box<Raw>, upper_bound: Box<Raw>, lower_bound: i32, cell_height: i32 },
}

/// Parse one density-function JSON value (number | string ref | typed object).
pub fn parse_df_value(
    v: &Json,
    dir: &WorldgenDir,
    visited: &mut HashSet<String>,
) -> Result<Raw, String> {
    match v {
        Json::Num(n, _) => Ok(Raw::Const(*n)),
        Json::Str(s) => {
            // registry reference "minecraft:overworld/continents"
            let (ns, path) = split_rl(s)?;
            if !visited.insert(s.clone()) {
                return Err(format!("cyclic density_function reference: {s}"));
            }
            let text = dir
                .get(&ns, "density_function", &path)
                .ok_or_else(|| format!("density_function file not found: {s}"))?;
            let j = json::parse(text).map_err(|e| e.to_string())?;
            // The file content may be a bare number (zero.json = `0.0`) —
            // route through parse_df_value, not parse_df_typed.
            let raw = parse_df_value(&j, dir, visited)?;
            visited.remove(s);
            Ok(raw)
        }
        Json::Obj(_) => parse_df_typed(v, dir, visited),
        other => Err(format!("unexpected density function JSON: {other:?}")),
    }
}

fn split_rl(s: &str) -> Result<(String, String), String> {
    match s.split_once(':') {
        Some((ns, path)) => Ok((ns.to_string(), path.to_string())),
        None => Ok(("minecraft".to_string(), s.to_string())),
    }
}

fn num(v: &Json, key: &str) -> Result<f64, String> {
    v.get(key).and_then(|j| j.as_f64()).ok_or(format!("missing number field '{key}'"))
}

fn i32_field(v: &Json, key: &str) -> Result<i32, String> {
    v.get(key).and_then(|j| j.as_i64()).map(|x| x as i32).ok_or(format!("missing int field '{key}'"))
}

fn child<'a>(v: &'a Json, key: &str) -> Result<&'a Json, String> {
    v.get(key).ok_or(format!("missing field '{key}'"))
}

fn parse_df_typed(v: &Json, dir: &WorldgenDir, visited: &mut HashSet<String>) -> Result<Raw, String> {
    let ty = v
        .get("type")
        .and_then(|j| j.as_str())
        .ok_or("density function object missing 'type'")?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    let mut r = |k: &str| -> Result<Raw, String> {
        let j = child(v, k)?;
        parse_df_value(j, dir, visited)
    };
    Ok(match ty {
        "constant" => Raw::Const(num(v, "argument")?),
        "y_clamped_gradient" => Raw::YClampedGradient {
            from_y: i32_field(v, "from_y")?,
            to_y: i32_field(v, "to_y")?,
            from_value: num(v, "from_value")?,
            to_value: num(v, "to_value")?,
        },
        "noise" => Raw::Noise {
            key: rl_str(child(v, "noise")?)?,
            xz_scale: num(v, "xz_scale")?,
            y_scale: num(v, "y_scale")?,
        },
        "shifted_noise" => Raw::ShiftedNoise {
            shift_x: Box::new(r("shift_x")?),
            shift_y: Box::new(r("shift_y")?),
            shift_z: Box::new(r("shift_z")?),
            xz_scale: num(v, "xz_scale")?,
            y_scale: num(v, "y_scale")?,
            key: rl_str(child(v, "noise")?)?,
        },
        // ShiftA/ShiftB/Shift codecs are singleArgumentCodec(NoiseHolder)
        // — the JSON field is "argument" (the vanilla data files say so).
        "shift_a" => Raw::ShiftA(rl_str(child(v, "argument")?)?),
        "shift_b" => Raw::ShiftB(rl_str(child(v, "argument")?)?),
        "shift" => Raw::Shift(rl_str(child(v, "argument")?)?),
        "blend_density" => Raw::BlendDensity(Box::new(r("argument")?)),
        "interpolated" | "flat_cache" | "cache_2d" | "cache_once" | "cache_all_in_cell" => {
            let ty = match ty {
                "interpolated" => MarkerType::Interpolated,
                "flat_cache" => MarkerType::FlatCache,
                "cache_2d" => MarkerType::Cache2D,
                "cache_once" => MarkerType::CacheOnce,
                _ => MarkerType::CacheAllInCell,
            };
            Raw::Marker { ty, wrapped: Box::new(r("argument")?) }
        }
        "weird_scaled_sampler" => {
            let mapper = child(v, "rarity_value_mapper")?.as_str().unwrap_or("");
            let rarity = match mapper {
                "type_1" => Rarity::Type1,
                "type_2" => Rarity::Type2,
                other => return Err(format!("unknown rarity_value_mapper '{other}'")),
            };
            Raw::WeirdScaledSampler {
                input: Box::new(r("input")?),
                key: rl_str(child(v, "noise")?)?,
                rarity,
            }
        }
        "range_choice" => Raw::RangeChoice {
            input: Box::new(r("input")?),
            min_inclusive: num(v, "min_inclusive")?,
            max_exclusive: num(v, "max_exclusive")?,
            when_in_range: Box::new(r("when_in_range")?),
            when_out_of_range: Box::new(r("when_out_of_range")?),
        },
        "clamp" => Raw::Clamp {
            input: Box::new(r("input")?),
            min: num(v, "min")?,
            max: num(v, "max")?,
        },
        "abs" => Raw::Mapped { ty: MappedType::Abs, input: Box::new(r("argument")?) },
        "square" => Raw::Mapped { ty: MappedType::Square, input: Box::new(r("argument")?) },
        "cube" => Raw::Mapped { ty: MappedType::Cube, input: Box::new(r("argument")?) },
        "half_negative" => Raw::Mapped { ty: MappedType::HalfNegative, input: Box::new(r("argument")?) },
        "quarter_negative" => {
            Raw::Mapped { ty: MappedType::QuarterNegative, input: Box::new(r("argument")?) }
        }
        "invert" => Raw::Mapped { ty: MappedType::Invert, input: Box::new(r("argument")?) },
        "squeeze" => Raw::Mapped { ty: MappedType::Squeeze, input: Box::new(r("argument")?) },
        "add" | "mul" | "min" | "max" => {
            let t = match ty {
                "add" => Ap2Type::Add,
                "mul" => Ap2Type::Mul,
                "min" => Ap2Type::Min,
                _ => Ap2Type::Max,
            };
            Raw::Ap2 { ty: t, a1: Box::new(r("argument1")?), a2: Box::new(r("argument2")?) }
        }
        "spline" => parse_spline_value(child(v, "spline")?, dir, visited)?,
        "old_blended_noise" => Raw::OldBlendedNoise {
            xz_scale: num(v, "xz_scale")?,
            y_scale: num(v, "y_scale")?,
            xz_factor: num(v, "xz_factor")?,
            y_factor: num(v, "y_factor")?,
            smear_scale_multiplier: num(v, "smear_scale_multiplier")?,
        },
        "blend_alpha" => Raw::BlendAlpha,
        "blend_offset" => Raw::BlendOffset,
        "beardifier" => Raw::Beardifier,
        "end_islands" => Raw::EndIslands,
        "find_top_surface" => Raw::FindTopSurface {
            density: Box::new(r("density")?),
            upper_bound: Box::new(r("upper_bound")?),
            lower_bound: i32_field(v, "lower_bound")?,
            cell_height: i32_field(v, "cell_height")?,
        },
        other => return Err(format!("unknown density function type 'minecraft:{other}'")),
    })
}

fn rl_str(v: &Json) -> Result<String, String> {
    match v {
        Json::Str(s) => Ok(s.clone()),
        Json::Obj(_) => {
            // inline NoiseHolder is not possible in the vanilla JSON shape —
            // noise fields are always registry refs.
            Err("noise field must be a resource-location string".into())
        }
        other => Err(format!("unexpected noise field: {other:?}")),
    }
}

/// CubicSpline codec: number -> Constant, object {coordinate, points} -> Multipoint.
fn parse_spline_value(v: &Json, dir: &WorldgenDir, visited: &mut HashSet<String>) -> Result<Raw, String> {
    match v {
        Json::Num(n, _) => {
            // Constant spline — represent as a spline over a constant coordinate
            // with one point (apply must return the constant regardless of ctx).
            Ok(Raw::Spline {
                coordinate: Box::new(Raw::Const(0.0)),
                points: vec![(0.0, Raw::Const(*n), 0.0)],
            })
        }
        Json::Obj(o) => {
            // Multipoint: {coordinate: <df>, points: [{location, value, derivative}]}
            if o.contains_key("coordinate") && o.contains_key("points") {
                let coordinate = Box::new(parse_df_value(child(v, "coordinate")?, dir, visited)?);
                let mut points = Vec::new();
                for p in child(v, "points")?.as_arr().ok_or("spline points must be array")? {
                    let location = num(p, "location")? as f32;
                    let value = parse_spline_value(child(p, "value")?, dir, visited)?;
                    let derivative = num(p, "derivative")? as f32;
                    points.push((location, value, derivative));
                }
                Ok(Raw::Spline { coordinate, points })
            } else {
                // nested spline object forms all carry the shape above in the
                // vanilla datapack; anything else is an honest parse error.
                Err(format!("unrecognized spline JSON shape: keys={:?}", o.keys().collect::<Vec<_>>()))
            }
        }
        other => Err(format!("unexpected spline JSON: {other:?}")),
    }
}

// --------------------------------------------------------------------------
// Wiring (RandomState.NoiseWiringHelper equivalent)
// --------------------------------------------------------------------------

pub struct Wiring {
    pub bank: NoiseBank,
    noise_index: HashMap<String, usize>,
    blended_index: HashMap<(u64, u64, u64, u64, u64), usize>,
    /// wiring order of blended param tuples (for the spec hash)
    pub blended_order: Vec<(u64, u64, u64, u64, u64)>,
}

impl Wiring {
    pub fn new() -> Self {
        Self {
            bank: NoiseBank::default(),
            noise_index: HashMap::new(),
            blended_index: HashMap::new(),
            blended_order: Vec::new(),
        }
    }

    /// Wire a noise key: interned NormalNoise per key, with the legacy
    /// overrides applied by key name (value-exact — see module docs). Noise
    /// parameters are fetched LAZILY: the legacy shift override ignores the
    /// registry parameters entirely (NoiseParameters(0, 0.0) hardcoded), so a
    /// legacy router must not require noise/offset.json to parse.
    ///
    /// Public since NCF task 5-b: `RandomState::get_or_create_noise` (the
    /// SurfaceSystem wiring — minecraft:surface, clay_bands_offset, badlands
    /// pillars, icebergs) reuses this exact interning path.
    pub fn noise(&mut self, key: &str, factory: &dyn PositionalRandomFactory, dir: &WorldgenDir, legacy: bool, level_seed: i64) -> Result<usize, String> {
        if let Some(idx) = self.noise_index.get(key) {
            return Ok(*idx);
        }
        let (ns, name) = split_rl(key)?;
        let instance = if legacy && (name == "temperature" && ns == "minecraft") {
            NormalNoise::create_legacy_nether(
                &mut LegacyRandomSource::new(level_seed),
                -7,
                &[1.0, 1.0],
            )
        } else if legacy && (name == "vegetation" && ns == "minecraft") {
            NormalNoise::create_legacy_nether(
                &mut LegacyRandomSource::new(level_seed + 1),
                -7,
                &[1.0, 1.0],
            )
        } else if legacy && (name == "offset" && ns == "minecraft") {
            // Noises.SHIFT = "minecraft:offset"; NoiseParameters(0, 0.0)
            let mut rng = factory.from_hash_of(key);
            NormalNoise::create(rng.as_mut(), 0, &[0.0])
        } else {
            let params = self.noise_params(dir, key)?;
            let mut rng = factory.from_hash_of(key);
            NormalNoise::create(rng.as_mut(), params.first_octave, &params.amplitudes)
        };
        let idx = self.bank.noises.len();
        self.bank.noises.push(instance);
        self.noise_index.insert(key.to_string(), idx);
        Ok(idx)
    }

    fn blended(&mut self, params: (f64, f64, f64, f64, f64), factory: &dyn PositionalRandomFactory, legacy: bool, level_seed: i64) -> Result<usize, String> {
        let kbits = (
            params.0.to_bits(),
            params.1.to_bits(),
            params.2.to_bits(),
            params.3.to_bits(),
            params.4.to_bits(),
        );
        if let Some(idx) = self.blended_index.get(&kbits) {
            return Ok(*idx);
        }
        // RandomState: random = legacy ? newLegacyInstance(0) : random.fromHashOf("minecraft:terrain")
        let mut rng: Box<dyn RandomSource> = if legacy {
            Box::new(LegacyRandomSource::new(level_seed))
        } else {
            factory.from_hash_of("minecraft:terrain")
        };
        let instance = BlendedNoise::new(
            rng.as_mut(),
            params.0,
            params.1,
            params.2,
            params.3,
            params.4,
        );
        let idx = self.bank.blended.len();
        self.bank.blended.push(instance);
        self.blended_index.insert(kbits, idx);
        self.blended_order.push(kbits);
        Ok(idx)
    }

    pub fn wire(&mut self, raw: &Raw, factory: &dyn PositionalRandomFactory, dir: &WorldgenDir, legacy: bool, level_seed: i64) -> Result<Df, String> {
        Ok(match raw {
            Raw::Const(v) => Df::Const(*v),
            Raw::YClampedGradient { from_y, to_y, from_value, to_value } => Df::YClampedGradient {
                from_y: *from_y,
                to_y: *to_y,
                from_value: *from_value,
                to_value: *to_value,
            },
            Raw::Noise { key, xz_scale, y_scale } => {
                let idx = self.noise(key, factory, dir, legacy, level_seed)?;
                Df::Noise(idx, *xz_scale, *y_scale)
            }
            Raw::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, key } => {
                let sx = Box::new(self.wire(shift_x, factory, dir, legacy, level_seed)?);
                let sy = Box::new(self.wire(shift_y, factory, dir, legacy, level_seed)?);
                let sz = Box::new(self.wire(shift_z, factory, dir, legacy, level_seed)?);
                let idx = self.noise(key, factory, dir, legacy, level_seed)?;
                Df::ShiftedNoise { shift_x: sx, shift_y: sy, shift_z: sz, xz_scale: *xz_scale, y_scale: *y_scale, noise: idx }
            }
            Raw::ShiftA(key) | Raw::ShiftB(key) | Raw::Shift(key) => {
                let idx = self.noise(key, factory, dir, legacy, level_seed)?;
                match raw {
                    Raw::ShiftA(_) => Df::ShiftA(idx),
                    Raw::ShiftB(_) => Df::ShiftB(idx),
                    _ => Df::Shift(idx),
                }
            }
            Raw::BlendDensity(input) => {
                Df::BlendDensity(Box::new(self.wire(input, factory, dir, legacy, level_seed)?))
            }
            Raw::Marker { ty, wrapped } => Df::Marker {
                ty: *ty,
                wrapped: Box::new(self.wire(wrapped, factory, dir, legacy, level_seed)?),
            },
            Raw::WeirdScaledSampler { input, key, rarity } => {
                let input = Box::new(self.wire(input, factory, dir, legacy, level_seed)?);
                let idx = self.noise(key, factory, dir, legacy, level_seed)?;
                Df::WeirdScaledSampler { input, noise: idx, rarity: *rarity }
            }
            Raw::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
                Df::RangeChoice {
                    input: Box::new(self.wire(input, factory, dir, legacy, level_seed)?),
                    min_inclusive: *min_inclusive,
                    max_exclusive: *max_exclusive,
                    when_in_range: Box::new(self.wire(when_in_range, factory, dir, legacy, level_seed)?),
                    when_out_of_range: Box::new(self.wire(when_out_of_range, factory, dir, legacy, level_seed)?),
                }
            }
            Raw::Clamp { input, min, max } => Df::Clamp {
                input: Box::new(self.wire(input, factory, dir, legacy, level_seed)?),
                min: *min,
                max: *max,
            },
            Raw::Mapped { ty, input } => {
                let wired = self.wire(input, factory, dir, legacy, level_seed)?;
                mapped_create(*ty, wired, &self.bank)
            }
            Raw::Ap2 { ty, a1, a2 } => {
                let w1 = self.wire(a1, factory, dir, legacy, level_seed)?;
                let w2 = self.wire(a2, factory, dir, legacy, level_seed)?;
                ap2_create(*ty, w1, w2, &self.bank)
            }
            Raw::Spline { coordinate, points } => {
                let coord = Box::new(self.wire(coordinate, factory, dir, legacy, level_seed)?);
                let cmin = coord.min_value_of(&self.bank) as f32;
                let cmax = coord.max_value_of(&self.bank) as f32;
                let mut locations = Vec::with_capacity(points.len());
                let mut values = Vec::with_capacity(points.len());
                let mut derivatives = Vec::with_capacity(points.len());
                for (loc, val, der) in points {
                    locations.push(*loc);
                    values.push(self.wire_spline_value(val, factory, dir, legacy, level_seed)?);
                    derivatives.push(*der);
                }
                let multi = multi_spline_create(coord, locations, values, derivatives, cmin, cmax);
                Df::Spline(Box::new(multi))
            }
            Raw::OldBlendedNoise { xz_scale, y_scale, xz_factor, y_factor, smear_scale_multiplier } => {
                let idx = self.blended(
                    (*xz_scale, *y_scale, *xz_factor, *y_factor, *smear_scale_multiplier),
                    factory,
                    legacy,
                    level_seed,
                )?;
                Df::Blended(idx)
            }
            Raw::BlendAlpha => Df::BlendAlpha,
            Raw::BlendOffset => Df::BlendOffset,
            Raw::Beardifier => Df::Beardifier,
            Raw::EndIslands => Df::EndIslands,
            Raw::FindTopSurface { density, upper_bound, lower_bound, cell_height } => Df::FindTopSurface {
                density: Box::new(self.wire(density, factory, dir, legacy, level_seed)?),
                upper_bound: Box::new(self.wire(upper_bound, factory, dir, legacy, level_seed)?),
                lower_bound: *lower_bound,
                cell_height: *cell_height,
            },
        })
    }

    fn wire_spline_value(&mut self, raw: &Raw, factory: &dyn PositionalRandomFactory, dir: &WorldgenDir, legacy: bool, level_seed: i64) -> Result<SplineValue, String> {
        // Spline JSON nests splines inside 'value'; constants were folded to
        // a 1-point spline at parse time, so wire them back to Const.
        if let Raw::Spline { coordinate, points } = raw {
            if points.len() == 1 {
                if let (Raw::Const(0.0), (0.0, Raw::Const(v), 0.0)) =
                    (coordinate.as_ref(), &points[0])
                {
                    return Ok(SplineValue::Const(*v as f32));
                }
            }
            let _ = coordinate;
        }
        if let Raw::Spline { coordinate, points } = raw {
            let coord = Box::new(self.wire(coordinate, factory, dir, legacy, level_seed)?);
            let cmin = coord.min_value_of(&self.bank) as f32;
            let cmax = coord.max_value_of(&self.bank) as f32;
            let mut locations = Vec::with_capacity(points.len());
            let mut values = Vec::with_capacity(points.len());
            let mut derivatives = Vec::with_capacity(points.len());
            for (loc, val, der) in points {
                locations.push(*loc);
                values.push(self.wire_spline_value(val, factory, dir, legacy, level_seed)?);
                derivatives.push(*der);
            }
            let multi = multi_spline_create(coord, locations, values, derivatives, cmin, cmax);
            return Ok(SplineValue::Multi(Box::new(multi)));
        }
        // A bare non-spline raw inside a spline value slot cannot happen with
        // the vanilla codec (values are CubicSpline-typed).
        Err("non-spline raw inside spline value slot".into())
    }

    fn noise_params(&self, dir: &WorldgenDir, key: &str) -> Result<NoiseParams, String> {
        let (ns, name) = split_rl(key)?;
        let text = dir
            .get(&ns, "noise", &name)
            .ok_or_else(|| format!("noise parameters not found: {key}"))?
            .clone();
        let j = json::parse(&text).map_err(|e| e.to_string())?;
        NoiseParams::parse(&j)
    }
}

impl Default for Wiring {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------
// Router + RandomState
// --------------------------------------------------------------------------

/// NoiseRouter record — the 15 fields in vanilla order.
pub struct Router {
    pub barrier: Df,
    pub fluid_level_floodedness: Df,
    pub fluid_level_spread: Df,
    pub lava: Df,
    pub temperature: Df,
    pub vegetation: Df,
    pub continents: Df,
    pub erosion: Df,
    pub depth: Df,
    pub ridges: Df,
    pub preliminary_surface_level: Df,
    pub final_density: Df,
    pub vein_toggle: Df,
    pub vein_ridged: Df,
    pub vein_gap: Df,
}

/// Basic NoiseGeneratorSettings facts the factory needs.
pub struct NoiseSettingsInfo {
    pub legacy_random_source: bool,
    pub sea_level: i32,
    pub aquifers_enabled: bool,
    pub ore_veins_enabled: bool,
    pub min_y: i32,
    pub height: i32,
    /// noise.size_horizontal / size_vertical in QUARTS (overworld: 1, 2 ->
    /// cellWidth 4 / cellHeight 8). Defaults mirror NoiseSettings.CODEC.
    pub noise_size_horizontal: i32,
    pub noise_size_vertical: i32,
    /// default_block canonical form ("minecraft:stone" or with [k=v,...]).
    pub default_block: String,
}

pub struct RandomState {
    pub bank: NoiseBank,
    pub router: Router,
    pub settings: NoiseSettingsInfo,
    pub level_seed: i64,
    /// `RandomState.random` — the worldgen positional factory the SurfaceSystem
    /// ctor receives (surface depth jitter, clay bands seed) and the base of
    /// every `getOrCreateRandomFactory` (vertical_gradient random_name). The
    /// factory is stateless (position-derived draws), so storing it changes no
    /// draw order (5-b).
    pub worldgen_factory: Box<dyn PositionalRandomFactory>,
    /// noise instance index -> resource key (wiring order; for vector tools)
    pub noise_key_by_index: Vec<String>,
    /// Canonical world-spec hash (P1.7): FNV-1a over the normalized settings
    /// text + router tree shape (values in hex-bit form).
    pub spec_hash: u64,
}

impl RandomState {
    /// RandomState.create equivalent: parse noise_settings JSON -> raw router
    /// -> wire with level_seed.
    pub fn build(dir: &WorldgenDir, settings_ns: &str, settings_name: &str, level_seed: i64) -> Result<Self, String> {
        let text = dir
            .get(settings_ns, "noise_settings", settings_name)
            .ok_or_else(|| format!("noise_settings not found: {settings_ns}:{settings_name}"))?;
        let j = json::parse(text).map_err(|e| e.to_string())?;

        let legacy_random_source = j
            .get("legacy_random_source")
            .and_then(|x| match x {
                Json::Bool(b) => Some(*b),
                _ => None,
            })
            .unwrap_or(false);
        let noise = j.get("noise").ok_or("noise_settings missing 'noise'")?;
        let default_block_json = j.get("default_block").ok_or("noise_settings missing 'default_block'")?;
        let default_block_name = default_block_json
            .get("Name")
            .and_then(|x| x.as_str())
            .ok_or("default_block missing 'Name'")?
            .to_string();
        let mut default_block_props: Vec<(String, String)> = Vec::new();
        if let Some(Json::Obj(props)) = default_block_json.get("Properties") {
            for (k, v) in props {
                if let Some(vs) = v.as_str() {
                    default_block_props.push((k.clone(), vs.to_string()));
                }
            }
        }
        default_block_props.sort();
        let mut default_block = default_block_name;
        if !default_block_props.is_empty() {
            let ps: Vec<String> = default_block_props.iter().map(|(k, v)| format!("{k}={v}")).collect();
            default_block.push_str(&format!("[{}]", ps.join(",")));
        }
        let settings = NoiseSettingsInfo {
            legacy_random_source,
            sea_level: j.get("sea_level").and_then(|x| x.as_i64()).unwrap_or(63) as i32,
            aquifers_enabled: matches!(j.get("aquifers_enabled"), Some(Json::Bool(true))),
            ore_veins_enabled: matches!(j.get("ore_veins_enabled"), Some(Json::Bool(true))),
            min_y: noise.get("min_y").and_then(|x| x.as_i64()).unwrap_or(-64) as i32,
            height: noise.get("height").and_then(|x| x.as_i64()).unwrap_or(384) as i32,
            noise_size_horizontal: noise.get("size_horizontal").and_then(|x| x.as_i64()).unwrap_or(1) as i32,
            noise_size_vertical: noise.get("size_vertical").and_then(|x| x.as_i64()).unwrap_or(2) as i32,
            default_block,
        };

        let router_json = j.get("noise_router").ok_or("noise_settings missing 'noise_router'")?;
        let mut visited = HashSet::new();
        let raw_router = parse_router_raw(router_json, dir, &mut visited)?;

        // worldgen random: settings.getRandomSource().newInstance(levelSeed).forkPositional()
        let factory: Box<dyn PositionalRandomFactory> = if legacy_random_source {
            let mut rs = LegacyRandomSource::new(level_seed);
            rs.fork_positional_factory()
        } else {
            let mut rs = XoroshiroRandomSource::new(level_seed);
            rs.fork_positional_factory()
        };

        let mut wiring = Wiring::new();
        let router = wire_router(&raw_router, &mut wiring, factory.as_ref(), dir, legacy_random_source, level_seed)?;

        // spec hash over canonical form: settings flags + each field's shape
        let mut hash_text = String::new();
        hash_text.push_str(&format!(
            "NCF-spec-v1|legacy={legacy_random_source}|aquifers={}|oreveins={}|miny={}|height={}|seed={level_seed}\n",
            settings.aquifers_enabled, settings.ore_veins_enabled, settings.min_y, settings.height
        ));
        // idx -> noise key (shape must hash KEYS, not indices — different
        // settings files can produce identical index assignments)
        let mut keys: Vec<String> = vec![String::new(); wiring.noise_index.len()];
        for (k, &i) in &wiring.noise_index {
            keys[i] = k.clone();
        }
        let blended_params: Vec<(u64, u64, u64, u64, u64)> =
            wiring.blended_order.to_vec();
        for f in router_fields(&router) {
            write_shape(f, &keys, &blended_params, &mut hash_text);
            hash_text.push('\n');
        }
        let spec_hash = fnv1a64(hash_text.as_bytes());

        let mut key_by_idx: Vec<(usize, String)> =
            wiring.noise_index.iter().map(|(k, &i)| (i, k.clone())).collect();
        key_by_idx.sort();
        let noise_key_by_index = key_by_idx.into_iter().map(|(_, k)| k).collect();

        Ok(Self {
            bank: wiring.bank,
            router,
            settings,
            level_seed,
            worldgen_factory: factory,
            noise_key_by_index,
            spec_hash,
        })
    }

    /// `RandomState.getOrCreateNoise(key)` (5-b): intern a NormalNoise for an
    /// OUT-OF-ROUTER noise key (SurfaceSystem ctor noises). Mirror of the Java
    /// method: `noiseIntances.computeIfAbsent(key, k -> Noises.instantiate(...))`
    /// = NormalNoise.create(random.fromHashOf(key), params). NOTE the legacy
    /// temperature/vegetation/shift overrides do NOT apply here in Java either
    /// (they live in NoiseWiringHelper.visitNoise, router fields only).
    pub fn get_or_create_noise(&mut self, dir: &WorldgenDir, key: &str) -> Result<usize, String> {
        // Already interned (router field or a previous surface noise)?
        // noise_key_by_index holds exactly one key per bank.noises index.
        if let Some(pos) = self.noise_key_by_index.iter().position(|k| k == key) {
            return Ok(pos);
        }
        let (ns, name) = split_rl(key)?;
        let text = dir
            .get(&ns, "noise", &name)
            .ok_or_else(|| format!("noise parameters not found: {key}"))?;
        let j = json::parse(text).map_err(|e| e.to_string())?;
        let p = j.get("firstOctave").and_then(|x| x.as_i64()).ok_or("noise params missing firstOctave")? as i32;
        let amps = j
            .get("amplitudes")
            .and_then(|x| x.as_arr())
            .ok_or("noise params missing amplitudes")?;
        let amplitudes: Vec<f64> = amps.iter().map(|a| a.as_f64().unwrap_or(0.0)).collect();
        let mut rng = self.worldgen_factory.from_hash_of(key);
        let instance = NormalNoise::create(rng.as_mut(), p, &amplitudes);
        self.bank.noises.push(instance);
        self.noise_key_by_index.push(key.to_string());
        Ok(self.bank.noises.len() - 1)
    }
}

pub fn wire_router(
    raw: &RawRouter,
    wiring: &mut Wiring,
    factory: &dyn PositionalRandomFactory,
    dir: &WorldgenDir,
    legacy: bool,
    level_seed: i64,
) -> Result<Router, String> {
    Ok(Router {
        barrier: wiring.wire(&raw.barrier, factory, dir, legacy, level_seed)?,
        fluid_level_floodedness: wiring.wire(&raw.fluid_level_floodedness, factory, dir, legacy, level_seed)?,
        fluid_level_spread: wiring.wire(&raw.fluid_level_spread, factory, dir, legacy, level_seed)?,
        lava: wiring.wire(&raw.lava, factory, dir, legacy, level_seed)?,
        temperature: wiring.wire(&raw.temperature, factory, dir, legacy, level_seed)?,
        vegetation: wiring.wire(&raw.vegetation, factory, dir, legacy, level_seed)?,
        continents: wiring.wire(&raw.continents, factory, dir, legacy, level_seed)?,
        erosion: wiring.wire(&raw.erosion, factory, dir, legacy, level_seed)?,
        depth: wiring.wire(&raw.depth, factory, dir, legacy, level_seed)?,
        ridges: wiring.wire(&raw.ridges, factory, dir, legacy, level_seed)?,
        preliminary_surface_level: wiring.wire(&raw.preliminary_surface_level, factory, dir, legacy, level_seed)?,
        final_density: wiring.wire(&raw.final_density, factory, dir, legacy, level_seed)?,
        vein_toggle: wiring.wire(&raw.vein_toggle, factory, dir, legacy, level_seed)?,
        vein_ridged: wiring.wire(&raw.vein_ridged, factory, dir, legacy, level_seed)?,
        vein_gap: wiring.wire(&raw.vein_gap, factory, dir, legacy, level_seed)?,
    })
}

/// RawRouter mirrors NoiseRouter's 15 fields pre-wiring.
pub struct RawRouter {
    pub barrier: Raw,
    pub fluid_level_floodedness: Raw,
    pub fluid_level_spread: Raw,
    pub lava: Raw,
    pub temperature: Raw,
    pub vegetation: Raw,
    pub continents: Raw,
    pub erosion: Raw,
    pub depth: Raw,
    pub ridges: Raw,
    pub preliminary_surface_level: Raw,
    pub final_density: Raw,
    pub vein_toggle: Raw,
    pub vein_ridged: Raw,
    pub vein_gap: Raw,
}

pub fn parse_router_raw(v: &Json, dir: &WorldgenDir, visited: &mut HashSet<String>) -> Result<RawRouter, String> {
    Ok(RawRouter {
        barrier: parse_df_value(child(v, "barrier")?, dir, visited)?,
        fluid_level_floodedness: parse_df_value(child(v, "fluid_level_floodedness")?, dir, visited)?,
        fluid_level_spread: parse_df_value(child(v, "fluid_level_spread")?, dir, visited)?,
        lava: parse_df_value(child(v, "lava")?, dir, visited)?,
        temperature: parse_df_value(child(v, "temperature")?, dir, visited)?,
        vegetation: parse_df_value(child(v, "vegetation")?, dir, visited)?,
        continents: parse_df_value(child(v, "continents")?, dir, visited)?,
        erosion: parse_df_value(child(v, "erosion")?, dir, visited)?,
        depth: parse_df_value(child(v, "depth")?, dir, visited)?,
        ridges: parse_df_value(child(v, "ridges")?, dir, visited)?,
        preliminary_surface_level: parse_df_value(child(v, "preliminary_surface_level")?, dir, visited)?,
        final_density: parse_df_value(child(v, "final_density")?, dir, visited)?,
        vein_toggle: parse_df_value(child(v, "vein_toggle")?, dir, visited)?,
        vein_ridged: parse_df_value(child(v, "vein_ridged")?, dir, visited)?,
        vein_gap: parse_df_value(child(v, "vein_gap")?, dir, visited)?,
    })
}

fn router_fields(r: &Router) -> Vec<&Df> {
    vec![
        &r.barrier,
        &r.fluid_level_floodedness,
        &r.fluid_level_spread,
        &r.lava,
        &r.temperature,
        &r.vegetation,
        &r.continents,
        &r.erosion,
        &r.depth,
        &r.ridges,
        &r.preliminary_surface_level,
        &r.final_density,
        &r.vein_toggle,
        &r.vein_ridged,
        &r.vein_gap,
    ]
}

/// Canonical shape text of a wired node (values in bit-exact hex, structure
/// in type tags) — the P1.7 world-spec hash input.
fn write_shape(df: &Df, keys: &[String], blended: &[(u64, u64, u64, u64, u64)], out: &mut String) {
    match df {
        Df::Const(v) => out.push_str(&format!("const:{:016x}", v.to_bits())),
        Df::YClampedGradient { from_y, to_y, from_value, to_value } => out.push_str(&format!(
            "yclamp:{from_y}:{to_y}:{:016x}:{:016x}",
            from_value.to_bits(),
            to_value.to_bits()
        )),
        Df::Noise(i, xz, ys) => {
            out.push_str(&format!(
                "noise:{}:{:016x}:{:016x}",
                keys.get(*i).map(|s| s.as_str()).unwrap_or("?"),
                xz.to_bits(),
                ys.to_bits()
            ))
        }
        Df::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, noise } => {
            out.push_str("shiftednoise:");
            write_shape(shift_x, keys, blended, out);
            out.push(',');
            write_shape(shift_y, keys, blended, out);
            out.push(',');
            write_shape(shift_z, keys, blended, out);
            out.push_str(&format!(
                ";{:016x}:{:016x}:{}",
                xz_scale.to_bits(),
                y_scale.to_bits(),
                keys.get(*noise).map(|s| s.as_str()).unwrap_or("?")
            ));
        }
        Df::ShiftA(i) => out.push_str(&format!("shifta:{}", keys.get(*i).map(|s| s.as_str()).unwrap_or("?"))),
        Df::ShiftB(i) => out.push_str(&format!("shiftb:{}", keys.get(*i).map(|s| s.as_str()).unwrap_or("?"))),
        Df::Shift(i) => out.push_str(&format!("shift:{}", keys.get(*i).map(|s| s.as_str()).unwrap_or("?"))),
        Df::BlendDensity(i) => {
            out.push_str("blenddensity:");
            write_shape(i, keys, blended, out)
        }
        Df::Marker { ty, wrapped } => {
            let t = match ty {
                MarkerType::Interpolated => "interpolated",
                MarkerType::FlatCache => "flat_cache",
                MarkerType::Cache2D => "cache2d",
                MarkerType::CacheOnce => "cacheonce",
                MarkerType::CacheAllInCell => "cacheallincell",
            };
            out.push_str("marker:");
            out.push_str(t);
            out.push(':');
            write_shape(wrapped, keys, blended, out)
        }
        Df::WeirdScaledSampler { input, noise, rarity } => {
            out.push_str(&format!(
                "weird:{}:{rarity:?}:",
                keys.get(*noise).map(|s| s.as_str()).unwrap_or("?")
            ));
            write_shape(input, keys, blended, out)
        }
        Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
            out.push_str("rangechoice:");
            write_shape(input, keys, blended, out);
            out.push_str(&format!(";{:016x}:{:016x};", min_inclusive.to_bits(), max_exclusive.to_bits()));
            write_shape(when_in_range, keys, blended, out);
            out.push(',');
            write_shape(when_out_of_range, keys, blended, out);
        }
        Df::Clamp { input, min, max } => {
            out.push_str(&format!("clamp:{:016x}:{:016x}:", min.to_bits(), max.to_bits()));
            write_shape(input, keys, blended, out)
        }
        Df::Mapped { ty, input, min, max } => {
            out.push_str(&format!("mapped:{ty:?}:{:016x}:{:016x}:", min.to_bits(), max.to_bits()));
            write_shape(input, keys, blended, out)
        }
        Df::MulOrAdd { is_add, input, min, max, argument } => {
            out.push_str(&format!(
                "muloradd:{}:{:016x}:{:016x}:{:016x}:",
                if *is_add { "add" } else { "mul" },
                min.to_bits(),
                max.to_bits(),
                argument.to_bits()
            ));
            write_shape(input, keys, blended, out)
        }
        Df::Ap2 { ty, a1, a2, min, max } => {
            out.push_str(&format!("ap2:{ty:?}:{:016x}:{:016x}:", min.to_bits(), max.to_bits()));
            write_shape(a1, keys, blended, out);
            out.push(',');
            write_shape(a2, keys, blended, out);
        }
        Df::Spline(m) => write_spline_shape(m, keys, blended, out),
        Df::Blended(i) => {
            let p = blended
                .get(*i)
                .map(|t| format!("{:016x}:{:016x}:{:016x}:{:016x}:{:016x}", t.0, t.1, t.2, t.3, t.4))
                .unwrap_or_else(|| "?".into());
            out.push_str(&format!("blended:{p}"))
        }
        Df::BlendAlpha => out.push_str("blendalpha"),
        Df::BlendOffset => out.push_str("blendoffset"),
        Df::Beardifier => out.push_str("beardifier"),
        Df::EndIslands => out.push_str("endislands"),
        Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } => {
            out.push_str(&format!("findtopsurface:{lower_bound}:{cell_height}:"));
            write_shape(density, keys, blended, out);
            out.push(',');
            write_shape(upper_bound, keys, blended, out);
        }
    }
}

fn write_spline_shape(m: &MultiSpline, keys: &[String], blended: &[(u64, u64, u64, u64, u64)], out: &mut String) {
    out.push_str("spline:");
    write_shape(&m.coordinate, keys, blended, out);
    out.push_str(";[");
    for ((l, v), d) in m.locations.iter().zip(&m.values).zip(&m.derivatives) {
        out.push_str(&format!("{:08x}:{:08x}:", l.to_bits(), d.to_bits()));
        match v {
            SplineValue::Const(c) => out.push_str(&format!("c{:08x}", c.to_bits())),
            SplineValue::Multi(inner) => write_spline_shape(inner.as_ref(), keys, blended, out),
        }
        out.push(')');
    }
    out.push_str(&format!("];{:08x}:{:08x}", m.min.to_bits(), m.max.to_bits()));
}

/// FNV-1a 64-bit.
pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in data {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
