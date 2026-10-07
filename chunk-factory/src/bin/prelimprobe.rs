//! prelimprobe — addendum 34 (Job 441690) surface-scan bisect: tree-level
//! judge for the aquifer status divergence (slot95, tectonic cells).
//!
//! NoiseChunk.preliminarySurfaceLevel evaluates the ROUTER's
//! preliminary_surface_level field at y=0 on quart-aligned columns —
//! including the 13 out-of-chunk candidate columns of every computeFluid
//! scan (Aquifer.java: candidate rects span grid±16..17 blocks, i.e. OUTSIDE
//! the veccheck capture window, which is chunk-aligned and in-chunk only).
//!
//! This probe evaluates the field twice at the given columns:
//!   OPT — the production pipeline tree (RandomState::build, which runs
//!         optimize_router_fields = P2.14 interval cutoff),
//!   RAW — the same JSON wired WITHOUT the optimizer (wire_router directly).
//! Bit differences = an unsound interval fold mis-firing on the scan
//! columns (bounds math in trusted_bounds differing from the Java
//! constructor math in TwoArgumentSimpleFunction.create).
//!
//! Usage: prelimprobe <extract_root> <seed> <x0> <x1> <z0> <z1>
//! Columns iterate step 4 over [x0..x1]x[z0..z1], aligned x&!3 / z&!3, y=0.
use chunk_factory::jrandom::{PositionalRandomFactory, RandomSource};
use chunk_factory::json;
use chunk_factory::router::{
    parse_router_raw, wire_router, RandomState, Wiring, WorldgenDir,
};
use chunk_factory::xoroshiro::XoroshiroRandomSource;
use std::collections::HashSet;
use std::path::Path;

fn bits(v: f64) -> String {
    format!("0x{:016x} ({:.17e})", v.to_bits(), v)
}

/// --trace mode: at ONE quart column, reproduce the Java
/// NoiseChunk.computePreliminarySurfaceLevel scalar eval step by step:
/// the FindTopSurface scan (upper bound, every sampled y, the density child
/// value), plus a y-freeness probe of every flat_cache-wrapped subtree
/// (Java's wired FlatCache returns the y=0 CACHED column value inside the
/// machine window — a y-dependent content diverges from a raw passthrough
/// at scan y != 0).
fn run_trace(root: &str, seed: i64, x: i32, z: i32) {
    use chunk_factory::density::{Ap2Type, Df, MappedType};

    let dir = WorldgenDir::load(Path::new(root)).expect("load extract");
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("RandomState build");
    let prelim = &rs.router.preliminary_surface_level;
    let bank = &rs.bank;

    let x = x & !3;
    let z = z & !3;
    println!("=== trace column ({x},0,{z}) ===");

    // 1) flat-cache y-freeness probe over the whole tree
    fn probe_yfree(df: &Df, bank: &chunk_factory::density::NoiseBank, x: i32, z: i32, path: &str, depth: usize) {
        if depth > 24 {
            return;
        }
        match df {
            Df::Marker { ty, wrapped } => {
                if matches!(ty, chunk_factory::density::MarkerType::FlatCache) {
                    let v0 = wrapped.compute(bank, x, 0, z);
                    let v64 = wrapped.compute(bank, x, 64, z);
                    let v128 = wrapped.compute(bank, x, 128, z);
                    let yfree = v0.to_bits() == v64.to_bits() && v0.to_bits() == v128.to_bits();
                    println!(
                        "flat_cache {path}: y0={} y64={} y128={} y-free={yfree}",
                        bits(v0), bits(v64), bits(v128)
                    );
                }
                probe_yfree(wrapped, bank, x, z, path, depth + 1);
            }
            Df::Ap2 { a1, a2, .. } => {
                probe_yfree(a1, bank, x, z, &format!("{path}/a1"), depth + 1);
                probe_yfree(a2, bank, x, z, &format!("{path}/a2"), depth + 1);
            }
            Df::MulOrAdd { input, .. } => probe_yfree(input, bank, x, z, &format!("{path}/ma"), depth + 1),
            Df::Mapped { input, .. } => probe_yfree(input, bank, x, z, &format!("{path}/mp"), depth + 1),
            Df::Clamp { input, .. } => probe_yfree(input, bank, x, z, &format!("{path}/cl"), depth + 1),
            Df::RangeChoice { input, when_in_range, when_out_of_range, .. } => {
                probe_yfree(input, bank, x, z, &format!("{path}/rc"), depth + 1);
                probe_yfree(when_in_range, bank, x, z, &format!("{path}/in"), depth + 1);
                probe_yfree(when_out_of_range, bank, x, z, &format!("{path}/out"), depth + 1);
            }
            Df::BlendDensity(w) => probe_yfree(w, bank, x, z, &format!("{path}/bd"), depth + 1),
            Df::WeirdScaledSampler { input, .. } => probe_yfree(input, bank, x, z, &format!("{path}/ws"), depth + 1),
            Df::Spline(ms) => {
                probe_yfree(&ms.coordinate, bank, x, z, &format!("{path}/spline"), depth + 1);
            }
            _ => {}
        }
    }
    probe_yfree(prelim, bank, x, z, "prelim", 0);

    // 2) FindTopSurface scan reproduction
    if let Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } = prelim {
        let up = upper_bound.compute(bank, x, 0, z);
        let i = chunk_factory::mth::floor(up / *cell_height as f64) * cell_height;
        println!("upper_bound = {} -> scan start i = {i}, lower = {lower_bound}", bits(up));
        let mut i1 = i;
        let mut steps = 0usize;
        while i1 >= *lower_bound && steps < 64 {
            let d = density.compute(bank, x, i1, z);
            println!("  density({x},{i1},{z}) = {} >0: {}", bits(d), d > 0.0);
            if d > 0.0 {
                break;
            }
            i1 -= cell_height;
            steps += 1;
        }
        println!("  => FindTopSurface returns {i1} (Mth.floor = {})", chunk_factory::mth::floor(prelim.compute(bank, x, 0, z)));
    } else {
        println!("(prelim tree is not a FindTopSurface at top level: {})", bits(prelim.compute(bank, x, 0, z)));
    }

    // 3) sub-chain y-freeness: wire the tectonic pieces directly and probe
    //    them at this column across y. offset/final is flat_cache(cache_2d(..))
    //    in the JSON — if our wiring DROPPED the marker, its wired eval is
    //    y-dependent, while Java's wired FlatCache pins y=0 in-window.
    let mut wiring = Wiring::new();
    let mut xrs = XoroshiroRandomSource::new(seed);
    let factory = xrs.fork_positional_factory();
    let mut wiring_fn = |name: &str| -> Df {
        use chunk_factory::router::parse_df_value;
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
    for name in [
        "tectonic:terrain_spline/offset/final",
        "tectonic:noise/full_continents",
        "tectonic:depth",
        "tectonic:continent_selector",
        "tectonic:noise/raw_continents",
        "tectonic:terrain_spline/factor/continents",
        "tectonic:terrain_spline/factor/islands",
        "tectonic:noise/continent/erosion_folded",
        "tectonic:noise/continent/ridges",
    ] {
        let df = {
            use chunk_factory::router::parse_df_value;
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
        let bank = &wiring.bank;
        let v0 = df.compute(bank, x, 0, z);
        let v64 = df.compute(bank, x, 64, z);
        let v80 = df.compute(bank, x, 80, z);
        let v128 = df.compute(bank, x, 128, z);
        let yfree = v0.to_bits() == v64.to_bits() && v0.to_bits() == v128.to_bits();
        println!("subchain {name}: y0={} y64={} y80={} y128={} y-free={yfree}", bits(v0), bits(v64), bits(v80), bits(v128));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = &args[0];
    let seed: i64 = args[1].parse().expect("seed");
    if args[2] == "--trace" {
        let x: i32 = args[3].parse().expect("x");
        let z: i32 = args[4].parse().expect("z");
        run_trace(root, seed, x, z);
        return;
    }
    let x0: i32 = args[2].parse().expect("x0");
    let x1: i32 = args[3].parse().expect("x1");
    let z0: i32 = args[4].parse().expect("z0");
    let z1: i32 = args[5].parse().expect("z1");

    let dir = WorldgenDir::load(Path::new(root)).expect("load extract");

    // OPT: production pipeline (wired + optimized).
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("RandomState build");
    let opt = &rs.router.preliminary_surface_level;

    // RAW: same settings JSON, wired WITHOUT the optimizer.
    let text = dir
        .get("minecraft", "noise_settings", "overworld")
        .expect("noise_settings overworld");
    let j = json::parse(text).expect("json parse");
    let legacy_random_source = matches!(j.get("legacy_random_source"), Some(chunk_factory::json::Json::Bool(true)));
    let router_json = j.get("noise_router").expect("noise_router");
    let mut visited = HashSet::new();
    let raw_router = parse_router_raw(router_json, &dir, &mut visited).expect("parse router raw");
    let mut wiring = Wiring::new();
    let mut xrs = XoroshiroRandomSource::new(seed);
    let factory = xrs.fork_positional_factory();
    let raw_routed = wire_router(
        &raw_router,
        &mut wiring,
        factory.as_ref(),
        &dir,
        legacy_random_source,
        seed,
    )
    .expect("wire router raw");
    let raw = &raw_routed.preliminary_surface_level;
    let raw_bank = &wiring.bank;

    let mut checked = 0usize;
    let mut diffs = 0usize;
    let mut z = z0 & !3;
    while z <= z1 {
        let mut x = x0 & !3;
        while x <= x1 {
            let vo = opt.compute(&rs.bank, x, 0, z);
            let vr = raw.compute(raw_bank, x, 0, z);
            checked += 1;
            if vo.to_bits() != vr.to_bits() {
                diffs += 1;
                println!("DIFF ({x},0,{z}) opt={} raw={}", bits(vo), bits(vr));
            }
            x += 4;
        }
        z += 4;
    }
    println!("prelimprobe: checked={checked} opt-vs-raw diffs={diffs}");
    if diffs == 0 {
        println!("prelimprobe: ZERO-DIFF — folds are value-neutral on these columns");
    }
}
