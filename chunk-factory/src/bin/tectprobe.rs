//! tectprobe — addendum 28 (Job 441690) class-(b) bisect: evaluate the
//! tectonic final_density SUB-chains at given coordinates and assemble the
//! root piecewise, so the first diverging piece can be solved against the
//! java oracle values from the density capture (veccheck CSV).
//!
//! At cell-corner coordinates (x%4==0, y%8==0, z%4==0) the interpolated and
//! blend_density markers are identity and the flat/cache markers are
//! value-identity, so a raw Df::compute of every sub-chain is directly
//! comparable with java's chunk-fill values.
//!
//! Usage: tectprobe <extract_root> <seed> <x,y,z>...
use chunk_factory::density::{Df, MappedType};
use chunk_factory::jrandom::RandomSource;
use chunk_factory::json;
use chunk_factory::mth;
use chunk_factory::router::{parse_df_value, RandomState, Wiring, WorldgenDir};
use chunk_factory::xoroshiro::XoroshiroRandomSource;
use std::collections::HashSet;
use std::path::Path;

fn bits(v: f64) -> String {
    format!("0x{:016x} ({:.17e})", v.to_bits(), v)
}

/// Recursive dump of the wired Df tree (structure + const leaves), so the
/// assembled piecewise model can be validated against what the router
/// ACTUALLY evaluates.
fn walk_eval(df: &Df, d: usize, max_d: usize, bank: &chunk_factory::density::NoiseBank, x: i32, y: i32, z: i32) {
    if d > max_d {
        return;
    }
    let pad = "  ".repeat(d);
    let v = df.compute(bank, x, y, z);
    let tag = format!("= {} {}", bits(v), "");
    match df {
        Df::Const(vv) => println!("{pad}Const {vv} {tag}"),
        Df::YClampedGradient { from_y, to_y, from_value, to_value } => println!(
            "{pad}YClampedGradient {from_y}..{to_y} {from_value}..{to_value} {tag}"
        ),
        Df::Noise(idx, sx, sy) => println!("{pad}Noise[{idx}] sx={sx} sy={sy} {tag}"),
        Df::ShiftedNoise { noise, xz_scale, y_scale, .. } => {
            println!("{pad}ShiftedNoise[{noise}] sx={xz_scale} sy={y_scale}");
            // children dumped by caller structure — skip (rare in walk)
        }
        Df::ShiftA(i) => println!("{pad}ShiftA[{i}] {tag}"),
        Df::ShiftB(i) => println!("{pad}ShiftB[{i}] {tag}"),
        Df::Shift(i) => println!("{pad}Shift[{i}] {tag}"),
        Df::BlendDensity(w) => {
            println!("{pad}BlendDensity {tag}");
            walk_eval(w, d + 1, max_d, bank, x, y, z);
        }
        Df::Marker { ty, wrapped } => {
            println!("{pad}Marker {ty:?} {tag}");
            walk_eval(wrapped, d + 1, max_d, bank, x, y, z);
        }
        Df::FlatCacheWindow { wrapped, first_noise_x, first_noise_z, size_xz } => {
            println!("{pad}FlatCacheWindow fx={first_noise_x} fz={first_noise_z} sxz={size_xz} {tag}");
            walk_eval(wrapped, d + 1, max_d, bank, x, y, z);
        }
        Df::WeirdScaledSampler { input, .. } => {
            println!("{pad}WeirdScaledSampler {tag}");
            walk_eval(input, d + 1, max_d, bank, x, y, z);
        }
        Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
            println!("{pad}RangeChoice [{min_inclusive},{max_exclusive}) {tag}");
            walk_eval(input, d + 1, max_d, bank, x, y, z);
            println!("{pad} in-range:");
            walk_eval(when_in_range, d + 1, max_d, bank, x, y, z);
            println!("{pad} out-range:");
            walk_eval(when_out_of_range, d + 1, max_d, bank, x, y, z);
        }
        Df::Clamp { input, min, max } => {
            println!("{pad}Clamp [{min},{max}] {tag}");
            walk_eval(input, d + 1, max_d, bank, x, y, z);
        }
        Df::Mapped { ty, input, min, max } => {
            println!("{pad}Mapped {ty:?} [{min},{max}] {tag}");
            walk_eval(input, d + 1, max_d, bank, x, y, z);
        }
        Df::MulOrAdd { is_add, input, argument, min, max } => {
            println!("{pad}MulOrAdd is_add={is_add} arg={argument} [{min},{max}] {tag}");
            walk_eval(input, d + 1, max_d, bank, x, y, z);
        }
        Df::Ap2 { ty, a1, a2, .. } => {
            println!("{pad}Ap2 {ty:?} {tag}");
            walk_eval(a1, d + 1, max_d, bank, x, y, z);
            walk_eval(a2, d + 1, max_d, bank, x, y, z);
        }
        Df::Spline(ms) => {
            println!("{pad}Spline (coordinate value below) {tag}");
            println!("{pad} locations: {:?}", &ms.locations);
            println!("{pad} derivatives: {:?}", &ms.derivatives);
            walk_eval(&ms.coordinate, d + 1, max_d, bank, x, y, z);
        }
        Df::Blended(i) => println!("{pad}Blended[{i}] {tag}"),
        Df::BlendAlpha => println!("{pad}BlendAlpha {tag}"),
        Df::BlendOffset => println!("{pad}BlendOffset {tag}"),
        Df::Beardifier => println!("{pad}Beardifier {tag}"),
        Df::EndIslands => println!("{pad}EndIslands {tag}"),
        Df::FindTopSurface { .. } => println!("{pad}FindTopSurface"),
    }
}


// ---------------------------------------------------------------------------
// --variants mode: bit-exact variant hunt for the sloped_cheese carrier.
// Pairs file: "x y z java_bits_hex" per line (extracted from veccheck
// MISMATCH lines of both tectonic captures). Each variant f32-roundtrips ONE
// named component at assembly time; the variant whose final_density matches
// java BIT-EXACTLY on every coord names the root.
// ---------------------------------------------------------------------------
fn run_variants(root: &str, seed: i64, pairs_path: &str) {
    use chunk_factory::density::{Df, MarkerType, MappedType, Ap2Type};
    use chunk_factory::jrandom::RandomSource;

    let dir = WorldgenDir::load(Path::new(root)).expect("load extract");
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("RandomState build");
    let mut wiring = Wiring::new();
    let mut xrs = XoroshiroRandomSource::new(seed);
    let factory = xrs.fork_positional_factory();

    fn wire_named(wiring: &mut Wiring, dir: &WorldgenDir, factory: &dyn chunk_factory::jrandom::PositionalRandomFactory, seed: i64, name: &str) -> Df {
        let (ns, path) = match name.split_once(':') {
            Some((a, b)) => (a.to_string(), b.to_string()),
            None => ("tectonic".to_string(), name.to_string()),
        };
        let text = dir
            .get(&ns, "density_function", &path)
            .unwrap_or_else(|| panic!("df file not found: {name}"))
            .clone();
        let j = json::parse(&text).expect("json parse");
        let mut visited = HashSet::new();
        let raw = parse_df_value(&j, dir, &mut visited).expect("parse raw");
        wiring
            .wire(&raw, factory, dir, false, seed)
            .expect("wire")
    }

    // jagged noise node: half_negative(noise(minecraft:jagged, 1500, 0))
    let jag_idx = wiring
        .noise("minecraft:jagged", factory.as_ref(), &dir, false, seed)
        .expect("wire jagged noise");
    let jag_noise = Df::Mapped {
        ty: MappedType::HalfNegative,
        input: Box::new(Df::Noise(jag_idx, 1500.0, 0.0)),
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    let depth = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:depth");
    let depth_add = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/offset/depth_additive");
    let jag_isl = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/jaggedness/islands");
    let jag_cont = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/jaggedness/continents");
    let weathering = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:mountain_ridges/weathering");
    let ridges = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:mountain_ridges/ridges");
    let cont_sel = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:continent_selector");
    let factor_cont = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/factor/continents");
    let factor_isl = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/factor/islands");
    let dune_spline = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:region/diamond/dune/spline");
    let dune_final = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:region/diamond/dune/final");
    let roughness = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/roughness");
    let ocean = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:terrain_spline/ocean");
    // inline splines over full_continents (roughness/ocean multipliers)
    let sc_text = dir
        .get("tectonic", "density_function", "sloped_cheese")
        .expect("sloped_cheese file")
        .clone();
    let sc_json = json::parse(&sc_text).expect("json parse");
    let inline_sp = |wiring: &mut Wiring, j: &json::Json| -> Df {
        let mut visited = HashSet::new();
        let raw = parse_df_value(j, &dir, &mut visited).expect("parse inline spline");
        wiring
            .wire(&raw, factory.as_ref(), &dir, false, seed)
            .expect("wire inline spline")
    };
    // navigate: argument.argument2.argument2.argument1 = roughness spline node
    let sp_rough_json = sc_json
        .get("argument").unwrap()
        .get("argument2").unwrap()
        .get("argument2").unwrap()
        .get("argument1").unwrap()
        .get("argument1").unwrap();
    let sp_ocean_json = sc_json
        .get("argument").unwrap()
        .get("argument2").unwrap()
        .get("argument2").unwrap()
        .get("argument2").unwrap()
        .get("argument1").unwrap();
    let sp_rough = inline_sp(&mut wiring, sp_rough_json);
    let sp_ocean = inline_sp(&mut wiring, sp_ocean_json);

    let noodle = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:cave/noodle");
    let caves = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:caves");
    let river = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:underground_river/total");
    let lava = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:lava_tunnel/total");
    let slope_lower = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:__constants/slope_lower");
    let slope_upper = wire_named(&mut wiring, &dir, factory.as_ref(), seed, "tectonic:__constants/slope_upper");

    // pairs
    let mut pairs: Vec<(i32, i32, i32, u64)> = Vec::new();
    for line in std::fs::read_to_string(pairs_path).expect("pairs file").lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 4 { continue; }
        pairs.push((
            t[0].parse().unwrap(),
            t[1].parse().unwrap(),
            t[2].parse().unwrap(),
            u64::from_str_radix(t[3].trim_start_matches("0x"), 16).unwrap(),
        ));
    }
    eprintln!("pairs: {}", pairs.len());

    // variant selector: which component gets the f32 roundtrip
    #[derive(Clone, Copy, PartialEq)]
    enum Var { None, Depth, DepthAdd, Weathering, Ridges, ContSel, Jag, DuneFinal, Roughness, Ocean, S1, MulQ, QuarterNeg, A, WholeSc, CaveDepth }
    let variants = [
        (Var::None, "none"),
        (Var::Depth, "depth"),
        (Var::DepthAdd, "depth_add"),
        (Var::Weathering, "weathering"),
        (Var::Ridges, "ridges"),
        (Var::ContSel, "cont_sel"),
        (Var::Jag, "jag"),
        (Var::DuneFinal, "dune_final"),
        (Var::Roughness, "roughness"),
        (Var::Ocean, "ocean"),
        (Var::S1, "S1"),
        (Var::MulQ, "mulq"),
        (Var::QuarterNeg, "quarter_neg"),
        (Var::A, "A"),
        (Var::WholeSc, "whole_sc"),
        (Var::CaveDepth, "cave_depth"),
    ];

    for (v, name) in variants {
        let mut ok = 0usize;
        let mut first_fail = String::new();
        for &(x, y, z, jb) in &pairs {
            let rt = |val: f64, on: bool| -> f64 { if on { (val as f32) as f64 } else { val } };
            // The fresh-wired components carry FRESH wiring noise indexes —
            // they MUST be evaluated against the fresh bank, not rs.bank
            // (cross-bank evaluation reads wrong noise instances: the earlier
            // -0.218-vs--0.40005 offset discrepancy was exactly this artifact).
            let bank = &wiring.bank;
            // sloped_cheese assembly with variant v
            let v_depth = rt(depth.compute(bank, x, y, z), v == Var::Depth);
            let v_dadd = rt(depth_add.compute(bank, x, y, z), v == Var::DepthAdd);
            let v_weath = rt(weathering.compute(bank, x, y, z), v == Var::Weathering);
            let v_ridge = rt(ridges.compute(bank, x, y, z), v == Var::Ridges) * -1.0;
            let v_cs = rt(cont_sel.compute(bank, x, y, z), v == Var::ContSel);
            let v_jag = rt(jag_noise.compute(bank, x, y, z), v == Var::Jag);
            let v_df = rt(dune_final.compute(bank, x, y, z), v == Var::DuneFinal);
            let v_rough = rt(roughness.compute(bank, x, y, z), v == Var::Roughness);
            let v_ocean = rt(ocean.compute(bank, x, y, z), v == Var::Ocean);
            let jag_isl_v = jag_isl.compute(bank, x, y, z);
            let jag_cont_v = jag_cont.compute(bank, x, y, z);
            let dune_sp = dune_spline.compute(bank, x, y, z);
            let sp_r = sp_rough.compute(bank, x, y, z);
            let sp_o = sp_ocean.compute(bank, x, y, z);
            let fc_v = factor_cont.compute(bank, x, y, z);
            let fi_v = factor_isl.compute(bank, x, y, z);

            let jag_cont_branch = (-0.1 * v_weath) + (v_ridge + 0.6);
            let s1 = (v_depth + v_dadd)
                + (jag_isl_v * v_jag + jag_cont_v * jag_cont_branch);
            let rch = if v_cs >= 0.9 && v_cs < 1.1 { fc_v } else { fi_v };
            let mulq_v = rt(s1 * rch, v == Var::MulQ);
            let qn = if mulq_v > 0.0 { mulq_v } else { mulq_v * 0.5 };
            let qn = rt(qn, v == Var::QuarterNeg);
            let a = rt(4.0 * qn, v == Var::A);
            let b = 0.0f64.max(dune_sp * v_df) + (sp_r * v_rough + sp_o * v_ocean);
            let sc = rt(a + b, v == Var::WholeSc);
            if name == "none" && x == 192 && y == 72 && z == 384 {
                eprintln!("DEBUG depth={} dadd={} jag_isl={} jag_cont={} weath={} ridge={} cs={} fc={} fi={} dune_sp={} dune_f={} sp_r={} rough={} sp_o={} ocean={}",
                    v_depth, v_dadd, jag_isl_v, jag_cont_v, v_weath, v_ridge, v_cs, fc_v, fi_v, dune_sp, v_df, sp_r, v_rough, sp_o, v_ocean);
                eprintln!("DEBUG jcb={} s1={} rch={} mulq={} qn={} a={} b={} sc={}", jag_cont_branch, s1, rch, mulq_v, qn, a, b, sc);
            }

            // root chain
            let v_caves = caves.compute(bank, x, y, z);
            let mbc = sc.min(v_caves);
            let up = slope_upper.compute(bank, x, y, z);
            let lo = slope_lower.compute(bank, x, y, z);
            let t1 = 1.0 + mbc;
            let t2 = up * t1;
            let t3 = -1.0 + t2;
            let t4 = -0.1 + t3;
            let t5 = lo * t4;
            let t6 = 0.1 + t5;
            let t7 = 0.64 * t6;
            let d = mth::clamp(t7, -1.0, 1.0);
            let t8 = d / 2.0 - d * d * d / 24.0;
            let v_noodle = noodle.compute(bank, x, y, z);
            let branch_a = t8.min(v_noodle);
            let v_river = river.compute(bank, x, y, z);
            let v_lava = lava.compute(bank, x, y, z);
            let branch_b = 0.0002f64.min(v_river) + v_lava;
            let fin = branch_a + branch_b;
            if fin.to_bits() == jb { ok += 1; } else if first_fail.is_empty() {
                first_fail = format!("({x},{y},{z}) got {:016x} want {:016x}", fin.to_bits(), jb);
            }
        }
        println!("variant {:<12} {}/{} match", name, ok, pairs.len());
        if !first_fail.is_empty() && ok < pairs.len() {
            println!("   first fail: {first_fail}");
        }
    }
}

// --sc: walk the router's sloped_cheese subtree (final -> Add -> Min ->
// MulOrAdd(arg=0) -> CacheOnce) with values at each node.
fn run_sc_walk(root: &str, seed: i64, coords: &[(i32,i32,i32)]) {
    let dir = WorldgenDir::load(Path::new(root)).expect("load extract");
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("RandomState build");
    // explicit path (per the walk dump):
    // Add.a1 -> Min.a1 -> Squeeze.input -> Mul(0.64).input -> Marker(Interp).wrapped
    // -> BlendDensity.input -> MulOrAdd(0.1).input -> Ap2Mul.a2 -> MulOrAdd(-0.1).input
    // -> MulOrAdd(-1).input -> Ap2Mul.a2 -> MulOrAdd(1).input -> Min(inner).a1
    // -> MulOrAdd(0).input = CacheOnce(SC)
    let mut cur = &rs.router.final_density;
    fn step<'a>(cur: &'a Df, msg: &str) -> &'a Df {
        match cur {
            Df::Ap2 { a1, a2, .. } => {
                if msg.ends_with(".a2") { a2 } else { a1 }
            }
            Df::Mapped { input, .. } | Df::MulOrAdd { input, .. } | Df::Clamp { input, .. } => input,
            Df::Marker { wrapped, .. } => wrapped,
            Df::BlendDensity(w) => w,
            other => panic!("path stuck at {msg}: {:?}", std::mem::discriminant(other)),
        }
    }
    let seq = ["Add.a1", "Min.a1", "Squeeze.in", "Mul064.in", "Marker.w", "Blend.in",
               "Mul01.in", "Ap2Mul.a2", "MulN01.in", "MulN1.in", "Ap2MulUp.a2", "Mul1.in",
               "MinIn.a1", "Mul0.in"];
    for m in seq {
        cur = step(cur, m);
    }
    let sc_node: &Df = cur;
    for (x, y, z) in coords {
        println!("=== SC subtree at ({x},{y},{z}) ===");
        walk_eval(sc_node, 0, 30, &rs.bank, *x, *y, *z);
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().expect("extract root");
    let seed: i64 = args.next().expect("seed").parse().expect("seed");
    if let Some(mode) = args.next() {
        if mode == "--variants" {
            let pairs = args.next().expect("pairs file");
            run_variants(&root, seed, &pairs);
            return;
        }
        if mode == "--sc" {
            let coords: Vec<(i32,i32,i32)> = args.map(|q| { let p: Vec<&str> = q.split(',').collect(); (p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap()) }).collect();
            run_sc_walk(&root, seed, &coords);
            return;
        }
    }
    let coords: Vec<(i32, i32, i32)> = args
        .map(|s| {
            let p: Vec<&str> = s.split(',').collect();
            (
                p[0].parse().expect("x"),
                p[1].parse().expect("y"),
                p[2].parse().expect("z"),
            )
        })
        .collect();

    let dir = WorldgenDir::load(Path::new(&root)).expect("load extract");

    // The wired router: final_density must reproduce the capture mismatch.
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("RandomState build");

    // Fresh wiring for the tectonic sub-chain files (noise construction is
    // deterministic per key/seed/factory — a fresh Wiring yields identical
    // noise values to the router's own instances).
    let mut wiring = Wiring::new();
    let mut xrs = XoroshiroRandomSource::new(seed);
    let factory = xrs.fork_positional_factory();

    let mut wiring_fn = |name: &str| -> Df {
        let (ns, path) = match name.split_once(':') {
            Some((a, b)) => (a.to_string(), b.to_string()),
            None => ("tectonic".to_string(), name.to_string()),
        };
        let text = dir
            .get(&ns, "density_function", &path)
            .unwrap_or_else(|| panic!("df file not found: {name}"))
            .clone();
        let j = json::parse(&text).expect("json parse");
        let mut visited = HashSet::new();
        let raw = parse_df_value(&j, &dir, &mut visited).expect("parse raw");
        wiring
            .wire(&raw, factory.as_ref(), &dir, false, seed)
            .expect("wire")
    };

    let base_terrain = wiring_fn("tectonic:base_terrain");
    let caves = wiring_fn("tectonic:caves");
    let noodle = wiring_fn("tectonic:cave/noodle");
    let river = wiring_fn("tectonic:underground_river/total");
    let lava = wiring_fn("tectonic:lava_tunnel/total");
    let slope_lower = wiring_fn("tectonic:__constants/slope_lower");
    let slope_upper = wiring_fn("tectonic:__constants/slope_upper");
    let depth = wiring_fn("tectonic:depth");
    let offset_final = wiring_fn("tectonic:terrain_spline/offset/final");

    for (x, y, z) in &coords {
        println!("=== ({x},{y},{z}) ===");
        let v_base = base_terrain.compute(&rs.bank, *x, *y, *z);
        let v_caves = caves.compute(&rs.bank, *x, *y, *z);
        let v_noodle = noodle.compute(&rs.bank, *x, *y, *z);
        let v_river = river.compute(&rs.bank, *x, *y, *z);
        let v_lava = lava.compute(&rs.bank, *x, *y, *z);
        let v_lo = slope_lower.compute(&rs.bank, *x, *y, *z);
        let v_up = slope_upper.compute(&rs.bank, *x, *y, *z);
        let v_depth = depth.compute(&rs.bank, *x, *y, *z);
        let v_off = offset_final.compute(&rs.bank, *x, *y, *z);
        println!("base_terrain        = {}", bits(v_base));
        println!("caves               = {}", bits(v_caves));
        println!("min(base,caves)     = {}", bits(v_base.min(v_caves)));
        println!("noodle              = {}", bits(v_noodle));
        println!("river               = {}", bits(v_river));
        println!("lava                = {}", bits(v_lava));
        println!("slope_lower         = {}", bits(v_lo));
        println!("slope_upper         = {}", bits(v_up));
        println!("depth               = {}  f32={:e}", bits(v_depth), v_depth as f32);
        println!("terrain_spline/off  = {}", bits(v_off));

        // Piecewise assembly of the root (all f64, marker-identity at cell
        // corners):
        //   A = min(squeeze(mul(0.64, add(0.1, mul(lo, add(-0.1, add(-1,
        //       mul(up, add(1, min(base, caves))))))))), noodle)
        //   B = add(min(0.0002, river), lava)
        //   final = A + B
        let mbc = v_base.min(v_caves);
        let t1 = 1.0f64 + mbc;
        let t2 = v_up * t1;
        let t3 = -1.0f64 + t2;
        let t4 = -0.1f64 + t3;
        let t5 = v_lo * t4;
        let t6 = 0.1f64 + t5; // blend_density identity (alpha=1 fresh world)
        let t7 = 0.64f64 * t6; // interpolated identity at cell corners
        let t8 = {
            let d = mth::clamp(t7, -1.0, 1.0);
            d / 2.0 - d * d * d / 24.0
        }; // squeeze (vanilla formula, = our MappedType::Squeeze)
        println!("chain t6            = {}", bits(t6));
        println!("mul 0.64            = {}", bits(t7));
        println!("squeeze             = {}", bits(t8));
        let branch_a = t8.min(v_noodle);
        println!("branch A (min)      = {}", bits(branch_a));
        let river_min = 0.0002f64.min(v_river);
        let branch_b = river_min + v_lava;
        println!("branch B            = {}", bits(branch_b));
        let assembled = branch_a + branch_b;
        println!("assembled final     = {}", bits(assembled));
        let routed = rs.router.final_density.compute(&rs.bank, *x, *y, *z);
        println!("router final        = {}", bits(routed));
        // Grid-phase hunt: where does the router's flat-cache value live?
        println!("---- offset_final at neighbor columns (hunt -0.40005) ----");
        for (qx, qz) in [(192i32, 384i32), (188, 384), (196, 384), (192, 380), (192, 388), (188, 380), (196, 380), (188, 388), (196, 388), (184, 384), (192, 376)] {
            let v = offset_final.compute(&rs.bank, qx, 72, qz);
            println!("  offset_final({qx},72,{qz}) = {}", bits(v));
        }
        println!("---- end hunt ----");
        println!("---- separate offset_final walk ----");
        walk_eval(&offset_final, 0, 20, &rs.bank, *x, *y, *z);
        println!("---- end separate offset_final ----");
        if assembled.to_bits() != routed.to_bits() {
            println!("  !! assembled != router — dumping the wired tree (depth 8):");
            walk_eval(&rs.router.final_density, 0, 20, &rs.bank, *x, *y, *z);
        }
    }
}
