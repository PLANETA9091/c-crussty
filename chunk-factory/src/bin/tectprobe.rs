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

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().expect("extract root");
    let seed: i64 = args.next().expect("seed").parse().expect("seed");
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
