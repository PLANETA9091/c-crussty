//! veccheck — the golden-vector zero-diff gate (NCF P0.3 tail / P1 gate /
//! P2.1-P2.3 evidence).
//!
//! Compares Rust bit-exact ports against CSV vectors captured from the REAL
//! Paper 1.21.10 classes (bench/golden `VectorCapture`, command
//! `/goldenvec`). Bits, not epsilons: every float is compared via its exact
//! bit pattern (hex-float CSV, see `crate::vectors`).
//!
//! Usage:
//!   cargo run --bin veccheck -- <vectors-dir> --worldgen <dir> --seed <seed> --settings overworld
//!
//! Exit 0 <=> every row matches bit-exactly. Any mismatch -> exit 1 (CI red),
//! first mismatches printed with full row context.
//!
//! CSV shapes (written by VectorCapture):
//!   random.csv : family,src,seed_lo,seed_hi,op,arg,index,value
//!   noise.csv  : family,impl,seed,seed_hi,params,x,y,z,v0,v1,v2,v3
//!                (x/y/z hex floats, or decimal ints for `blended`)
//!   density.csv: field,x,y,z,value

use chunk_factory::jrandom::{
    mix_stafford13, upgrade_seed_to_128bit, LegacyPositionalRandomFactory, LegacyRandomSource,
    RandomSource,
};
use chunk_factory::noise::{BlendedNoise, ImprovedNoise, NormalNoise, PerlinNoise};
use chunk_factory::router::{RandomState, Router, WorldgenDir};
use chunk_factory::xoroshiro::{XoroshiroPositionalRandomFactory, XoroshiroRandomSource};
use chunk_factory::vectors::load_csv;

use std::path::PathBuf;

struct Mismatches {
    count: usize,
    shown: usize,
    limit: usize,
    label: String,
}

impl Mismatches {
    fn new(label: &str, limit: usize) -> Self {
        Self { count: 0, shown: 0, limit, label: label.to_string() }
    }

    fn hit(&mut self, msg: String) {
        self.count += 1;
        if self.shown < self.limit {
            eprintln!("  MISMATCH [{}] {}", self.label, msg);
            self.shown += 1;
        }
    }
}

fn parse_param_map(params: &str) -> Vec<(String, String)> {
    params
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            (k.to_string(), v.to_string())
        })
        .collect()
}

fn param<'a>(map: &'a [(String, String)], key: &str) -> &'a str {
    map.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str()).unwrap_or("")
}

fn amps_of(map: &[(String, String)], key: &str) -> Vec<f64> {
    param(map, key)
        .split('|')
        .filter(|t| !t.is_empty())
        .map(|t| chunk_factory::vectors::parse_hex_f64(t).unwrap())
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: veccheck <vectors-dir> [--worldgen <dir>] [--seed <seed>] [--settings <name>] [--ns <ns>]");
        std::process::exit(2);
    }
    let vec_dir = PathBuf::from(&args[1]);
    let mut worldgen: Option<PathBuf> = None;
    let mut seed: i64 = 3053459;
    let mut settings = String::from("overworld");
    let mut ns = String::from("minecraft");
    let mut mode = String::from("all"); // all | interp | climate | climate-table
    let mut it = args.iter().skip(2);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--worldgen" => worldgen = it.next().map(PathBuf::from),
            "--seed" => seed = it.next().map(|s| s.parse().expect("seed")).expect("--seed value"),
            "--settings" => settings = it.next().expect("--settings value").clone(),
            "--ns" => ns = it.next().expect("--ns value").clone(),
            "--mode" => mode = it.next().expect("--mode value").clone(),
            other => {
                eprintln!("unknown arg {other}");
                std::process::exit(2);
            }
        }
    }
    let run_standard = mode == "all";
    let run_interp = mode == "all" || mode == "interp";
    let run_climate = mode == "all" || mode == "climate";
    let run_climate_table = mode == "all" || mode == "climate-table";

    let mut total_checked = 0usize;
    let mut total_mismatch = 0usize;

    // ------------------------------------------------------------------
    // random.csv — family,src,seed_lo,seed_hi,op,arg,index,value
    // ------------------------------------------------------------------
    let random_path = vec_dir.join("random.csv");
    if run_standard && random_path.exists() {
        let rows = load_csv(&random_path).expect("random.csv");
        let mut mm = Mismatches::new("random", 12);
        let mut checked = 0usize;

        // Groups preserve file order. For family=random the arg column is NOT
        // part of the identity (the draw pattern interleaves ops on ONE
        // stream); for pos rows the arg (position/name) IS (each at()/
        // fromHashOf yields a fresh child stream).
        let mut groups: Vec<(String, String, i64, i64, String, Vec<usize>)> = Vec::new();
        for (ri, row) in rows.iter().enumerate() {
            let family = row.c(0).to_string();
            let arg_id = if family == "random" { String::new() } else { row.c(5).to_string() };
            let lo = row.i64(2).unwrap_or(0);
            let hi = row.i64(3).unwrap_or(0);
            match groups
                .iter_mut()
                .find(|g| g.0 == family && g.1 == row.c(1) && g.2 == lo && g.3 == hi && g.4 == arg_id)
            {
                Some(g) => g.5.push(ri),
                None => groups.push((family, row.c(1).to_string(), lo, hi, arg_id, vec![ri])),
            }
        }

        for (family, src, seed_lo, seed_hi, arg, row_ids) in &groups {
            match family.as_str() {
                "stafford" => {
                    for &ri in row_ids {
                        let expect = rows[ri].i64(7).unwrap();
                        let got = mix_stafford13(*seed_lo);
                        checked += 1;
                        if got != expect {
                            mm.hit(format!(
                                "stafford13({seed_lo}) = {got} (0x{got:016x}), java {expect} (0x{expect:016x})"
                            ));
                        }
                    }
                }
                "upgrade128" => {
                    for &ri in row_ids {
                        let row = &rows[ri];
                        let expect = row.i64(7).unwrap();
                        let (lo, hi) = upgrade_seed_to_128bit(*seed_lo);
                        let got = if row.c(1) == "upgrade128_lo" { lo } else { hi };
                        checked += 1;
                        if got != expect {
                            mm.hit(format!("upgrade128({seed_lo}) half = {got}, java {expect}"));
                        }
                    }
                }
                "random" => {
                    let mut rng: Box<dyn RandomSource> = match src.as_str() {
                        "legacy" => Box::new(LegacyRandomSource::new(*seed_lo)),
                        "xoroshiro" => Box::new(XoroshiroRandomSource::new(*seed_lo)),
                        other => {
                            mm.hit(format!("unknown random src '{other}'"));
                            continue;
                        }
                    };
                    for &ri in row_ids {
                        let row = &rows[ri];
                        let op = row.c(4).to_string();
                        let argn = row.i64(5).unwrap_or(0);
                        let value_s = row.c(7).to_string();
                        checked += 1;
                        match op.as_str() {
                            "nextInt" => {
                                let got = rng.next_int();
                                if got.to_string() != value_s {
                                    mm.hit(format!("nextInt = {got}, java {value_s}"));
                                }
                            }
                            "nextIntBound" => {
                                let got = rng.next_int_bound(argn as i32);
                                if got.to_string() != value_s {
                                    mm.hit(format!("nextIntBound({argn}) = {got}, java {value_s}"));
                                }
                            }
                            "nextLong" => {
                                let got = rng.next_long();
                                if got.to_string() != value_s {
                                    mm.hit(format!("nextLong = {got}, java {value_s}"));
                                }
                            }
                            "nextBoolean" => {
                                let got = rng.next_boolean();
                                if (if got { "true" } else { "false" }) != value_s {
                                    mm.hit(format!("nextBoolean = {got}, java {value_s}"));
                                }
                            }
                            "nextFloat" => {
                                let got = rng.next_f32();
                                let expect = row.f32(7).unwrap();
                                if got.to_bits() != expect.to_bits() {
                                    mm.hit(format!(
                                        "nextFloat = 0x{:08x}, java 0x{:08x}",
                                        got.to_bits(),
                                        expect.to_bits()
                                    ));
                                }
                            }
                            "nextDouble" => {
                                let got = rng.next_f64();
                                let expect = row.f64(7).unwrap();
                                if got.to_bits() != expect.to_bits() {
                                    mm.hit(format!(
                                        "nextDouble = 0x{:016x}, java 0x{:016x}",
                                        got.to_bits(),
                                        expect.to_bits()
                                    ));
                                }
                            }
                            "nextGaussian" => {
                                let got = rng.next_gaussian();
                                let expect = row.f64(7).unwrap();
                                if got.to_bits() != expect.to_bits() {
                                    mm.hit(format!(
                                        "nextGaussian = {got:e}, java {expect:e}"
                                    ));
                                }
                            }
                            other => mm.hit(format!("unknown op '{other}'")),
                        }
                    }
                }
                "pos" => {
                    let mut child_rng: Box<dyn RandomSource> = match src.as_str() {
                        "legacy_at" => {
                            let mut p = arg.split('|');
                            let x: i32 = p.next().unwrap().parse().unwrap();
                            let y: i32 = p.next().unwrap().parse().unwrap();
                            let z: i32 = p.next().unwrap().parse().unwrap();
                            let f = LegacyPositionalRandomFactory { seed: *seed_lo };
                            Box::new(f.at(x, y, z))
                        }
                        "xoroshiro_at" => {
                            let mut p = arg.split('|');
                            let x: i32 = p.next().unwrap().parse().unwrap();
                            let y: i32 = p.next().unwrap().parse().unwrap();
                            let z: i32 = p.next().unwrap().parse().unwrap();
                            let f = XoroshiroPositionalRandomFactory { seed_lo: *seed_lo, seed_hi: *seed_hi };
                            Box::new(f.at(x, y, z))
                        }
                        "legacy_fromhash" => {
                            let f = LegacyPositionalRandomFactory { seed: *seed_lo };
                            Box::new(f.from_hash_of_impl(arg))
                        }
                        "xoroshiro_fromhash" => {
                            let f = XoroshiroPositionalRandomFactory { seed_lo: *seed_lo, seed_hi: *seed_hi };
                            Box::new(f.from_hash_of(arg))
                        }
                        other => {
                            mm.hit(format!("unknown pos src '{other}'"));
                            continue;
                        }
                    };
                    for &ri in row_ids {
                        let row = &rows[ri];
                        let op = row.c(4).to_string();
                        let value_s = row.c(7).to_string();
                        checked += 1;
                        match op.as_str() {
                            "nextIntBound" => {
                                // capture used nextInt(97) for all pos rows
                                let got = child_rng.next_int_bound(97);
                                if got.to_string() != value_s {
                                    mm.hit(format!("pos nextIntBound(97) = {got}, java {value_s}"));
                                }
                            }
                            "nextLong" => {
                                let got = child_rng.next_long();
                                if got.to_string() != value_s {
                                    mm.hit(format!("pos nextLong = {got}, java {value_s}"));
                                }
                            }
                            "nextGaussian" => {
                                let got = child_rng.next_gaussian();
                                let expect = row.f64(7).unwrap();
                                if got.to_bits() != expect.to_bits() {
                                    mm.hit(format!("pos nextGaussian = {got:e}, java {expect:e}"));
                                }
                            }
                            "nextDouble" => {
                                let got = child_rng.next_f64();
                                let expect = row.f64(7).unwrap();
                                if got.to_bits() != expect.to_bits() {
                                    mm.hit(format!(
                                        "pos nextDouble bits 0x{:016x}, java 0x{:016x}",
                                        got.to_bits(),
                                        expect.to_bits()
                                    ));
                                }
                            }
                            other => mm.hit(format!("unknown pos op '{other}'")),
                        }
                    }
                }
                _ => {}
            }
        }
        println!("random.csv      : checked {checked}, mismatches {}", mm.count);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    // ------------------------------------------------------------------
    // noise.csv — family,impl,seed,seed_hi,params,x,y,z,v0,v1,v2,v3
    // ------------------------------------------------------------------
    let noise_path = vec_dir.join("noise.csv");
    if run_standard && noise_path.exists() {
        let rows = load_csv(&noise_path).expect("noise.csv");
        let mut mm = Mismatches::new("noise", 12);
        let mut checked = 0usize;
        for row in &rows {
            let imp = row.c(1);
            let s = row.i64(2).unwrap();
            let params = row.c(4).to_string();
            let x = row.f64(5).unwrap();
            let y = row.f64(6).unwrap();
            let z = row.f64(7).unwrap();
            let (xi, yi, zi) = (x as i32, y as i32, z as i32);
            let pmap = parse_param_map(&params);
            checked += 1;
            match imp {
                "improved" => {
                    let mut rng = XoroshiroRandomSource::new(s);
                    let n = ImprovedNoise::new(&mut rng);
                    let got = n.noise(x, y, z);
                    let expect = row.f64(8).unwrap();
                    if got.to_bits() != expect.to_bits() {
                        mm.hit(format!(
                            "improved({s}) at {x},{y},{z} bits 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                "improved_deriv" => {
                    let mut rng = XoroshiroRandomSource::new(s);
                    let n = ImprovedNoise::new(&mut rng);
                    let mut vals = [0.0f64; 3];
                    let got = n.noise_with_derivative(x, y, z, &mut vals);
                    let expect = row.f64(8).unwrap();
                    let mut ok = got.to_bits() == expect.to_bits();
                    for (k, v) in vals.iter().enumerate() {
                        let e = row.f64(9 + k).unwrap();
                        ok &= v.to_bits() == e.to_bits();
                    }
                    if !ok {
                        mm.hit(format!(
                            "improved_deriv({s}) at {x},{y},{z} got {vals:?} value 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                "perlin" | "perlin_fixed_y" => {
                    let fo: i32 = param(&pmap, "fo").parse().unwrap();
                    let amps = amps_of(&pmap, "amps");
                    let mut rng = XoroshiroRandomSource::new(s);
                    let n = PerlinNoise::create(&mut rng, fo, &amps);
                    let got = if imp == "perlin" {
                        n.get_value(x, y, z)
                    } else {
                        let ys = chunk_factory::vectors::parse_hex_f64(param(&pmap, "yscale")).unwrap();
                        let ym = chunk_factory::vectors::parse_hex_f64(param(&pmap, "ymax")).unwrap();
                        n.get_value_scaled(x, y, z, ys, ym, false)
                    };
                    let expect = row.f64(8).unwrap();
                    if got.to_bits() != expect.to_bits() {
                        mm.hit(format!(
                            "{imp}({s},{params}) at {x},{y},{z} got 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                "normal" | "normal_legacy" => {
                    let fo: i32 = param(&pmap, "fo").parse().unwrap();
                    let amps = amps_of(&pmap, "amps");
                    let mut rng = XoroshiroRandomSource::new(s);
                    let n = if imp == "normal" {
                        NormalNoise::create(&mut rng, fo, &amps)
                    } else {
                        NormalNoise::create_legacy_nether(&mut rng, fo, &amps)
                    };
                    let got = n.get_value(x, y, z);
                    let expect = row.f64(8).unwrap();
                    if got.to_bits() != expect.to_bits() {
                        mm.hit(format!(
                            "{imp}({s},{params}) at {x},{y},{z} got 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                "blended" => {
                    let p = |k: &str| chunk_factory::vectors::parse_hex_f64(param(&pmap, k)).unwrap();
                    let mut rng = XoroshiroRandomSource::new(s);
                    let n = BlendedNoise::new(&mut rng, p("xz"), p("ys"), p("xzf"), p("yf"), p("smear"));
                    let got = n.compute(xi, yi, zi);
                    let expect = row.f64(8).unwrap();
                    if got.to_bits() != expect.to_bits() {
                        mm.hit(format!(
                            "blended({s}) at {xi},{yi},{zi} got 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                "rs_noise" => {
                    let dir = worldgen.as_ref().expect("rs_noise rows require --worldgen");
                    let dir = WorldgenDir::load(dir).expect("worldgen dir");
                    let key = params.clone(); // noise key, e.g. minecraft:jagged
                    let rs = RandomState::build(&dir, &ns, &settings, s).expect("RandomState build");
                    let idx = rs
                        .noise_key_by_index
                        .iter()
                        .position(|k| k == &key)
                        .unwrap_or_else(|| panic!("noise key {key} not wired"));
                    let got = rs.bank.noises[idx].get_value(x, y, z);
                    let expect = row.f64(8).unwrap();
                    if got.to_bits() != expect.to_bits() {
                        mm.hit(format!(
                            "rs_noise({key},seed {s}) at {x},{y},{z} got 0x{:016x} java 0x{:016x}",
                            got.to_bits(),
                            expect.to_bits()
                        ));
                    }
                }
                other => mm.hit(format!("unknown noise impl '{other}'")),
            }
        }
        println!("noise.csv       : checked {checked}, mismatches {}", mm.count);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    // ------------------------------------------------------------------
    // density.csv — field,x,y,z,value
    // ------------------------------------------------------------------
    let density_path = vec_dir.join("density.csv");
    if run_standard && density_path.exists() {
        let rows = load_csv(&density_path).expect("density.csv");
        let dir = worldgen.as_ref().expect("density rows require --worldgen");
        let dir = WorldgenDir::load(dir).expect("worldgen dir");
        let rs = RandomState::build(&dir, &ns, &settings, seed).expect("RandomState build");
        let mut mm = Mismatches::new("density", 100000);
        let mut checked = 0usize;
        for row in &rows {
            let field = row.c(0);
            let x = row.i32(1).unwrap();
            let y = row.i32(2).unwrap();
            let z = row.i32(3).unwrap();
            let expect = row.f64(4).unwrap();
            let df = router_field(&rs.router, field);
            let got = match df {
                Some(f) => f.compute(&rs.bank, x, y, z),
                None => {
                    mm.hit(format!("unknown field '{field}'"));
                    continue;
                }
            };
            checked += 1;
            if got.to_bits() != expect.to_bits() {
                mm.hit(format!(
                    "{field} at {x},{y},{z} got 0x{:016x} ({got:e}) java 0x{:016x} ({expect:e})",
                    got.to_bits(),
                    expect.to_bits()
                ));
            }
        }
        println!("density.csv     : checked {checked}, mismatches {}", mm.count);
        println!("spec_hash       : {:016x}", rs.spec_hash);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    // ------------------------------------------------------------------
    // interp.csv — NoiseChunk cell interpolation rows (P2.3-tail)
    // ------------------------------------------------------------------
    let interp_path = vec_dir.join("interp.csv");
    if run_interp && interp_path.exists() {
        let dir = worldgen.as_ref().expect("interp rows require --worldgen");
        let dir = WorldgenDir::load(dir).expect("worldgen dir");
        let rs = RandomState::build(&dir, &ns, &settings, seed).expect("RandomState build");
        let mut sim = chunk_factory::interpolator::NoiseChunkSim::from_random_state(&rs, 4, 1600, 1600);

        let text = std::fs::read_to_string(&interp_path).expect("read interp.csv");
        let mut header_seed: Option<i64> = None;
        let mut header_interp_count: Option<usize> = None;
        let mut cell_w_header: Option<i32> = None;
        let mut cell_h_header: Option<i32> = None;
        // T38-B: the capture coords ride in the `# firstNoise=<bx>,<bz>`
        // header (writeInterp has always written them). Absent = the canon
        // corpus at 1600,1600 (backward compatible with old captures).
        let mut header_base: Option<(i32, i32)> = None;
        for line in text.lines() {
            let t = line.trim();
            if !t.starts_with('#') {
                continue;
            }
            if let Some(v) = t.strip_prefix("# worldSeed=") {
                header_seed = v.parse().ok();
            }
            if let Some(v) = t.strip_prefix("# interpCount=") {
                header_interp_count = v.parse().ok();
            }
            if let Some(v) = t.strip_prefix("# firstNoise=") {
                let mut it = v.split(',');
                let bx = it.next().and_then(|s| s.parse().ok());
                let bz = it.next().and_then(|s| s.parse().ok());
                if let (Some(bx), Some(bz)) = (bx, bz) {
                    header_base = Some((bx, bz));
                }
            }
            if let Some(v) = t.strip_prefix("# cellWidth=") {
                // "# cellWidth=4 cellHeight=8 cellCountXZ=4 cellCountY=48 cellNoiseMinY=-8"
                let mut it = v.split_whitespace();
                cell_w_header = it.next().and_then(|x| x.parse().ok());
                for kv in it {
                    if let Some(h) = kv.strip_prefix("cellHeight=") {
                        cell_h_header = h.parse().ok();
                    }
                }
            }
        }
        if let Some((bx, bz)) = header_base {
            sim = chunk_factory::interpolator::NoiseChunkSim::from_random_state(&rs, 4, bx, bz);
        }
        if let Some(ws) = header_seed {
            assert_eq!(ws, seed, "interp.csv worldSeed != --seed");
        }
        if let Some(w) = cell_w_header {
            assert_eq!(w, sim.cell_width, "cellWidth mismatch");
        }
        if let Some(h) = cell_h_header {
            assert_eq!(h, sim.cell_height, "cellHeight mismatch");
        }

        let (interp_count, rows) = sim.drive_and_collect();
        if let Some(want) = header_interp_count {
            assert_eq!(want, interp_count, "interpolator count mismatch vs capture");
        }
        // replay the CSV rows IN ORDER against the sim rows
        let rows = rows.into_iter().map(|(i, x, y, z, v)| (i, x, y, z, v.to_bits()));
        let mut rows = rows.collect::<Vec<_>>().into_iter();
        let mut mm = Mismatches::new("interp", 50);
        let mut checked = 0usize;
        for line in text.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = t.split(',').collect();
            if cols.len() != 5 {
                mm.hit(format!("bad row shape: {t}"));
                continue;
            }
            let i: u32 = cols[0].parse().unwrap();
            let x: i32 = cols[1].parse().unwrap();
            let y: i32 = cols[2].parse().unwrap();
            let z: i32 = cols[3].parse().unwrap();
            let expect = chunk_factory::vectors::parse_hex_f64(cols[4]).unwrap();
            match rows.next() {
                Some((gi, gx, gy, gz, gbits)) => {
                    checked += 1;
                    let coords_ok = i == gi && x == gx && y == gy && z == gz;
                    let value_ok = expect.is_nan() && f64::from_bits(gbits).is_nan()
                        || expect.to_bits() == gbits;
                    if !coords_ok || !value_ok {
                        mm.hit(format!(
                            "row {checked}: java ({i},{x},{y},{z},{}) rust ({gi},{gx},{gy},{gz},0x{gbits:016x})", cols[4]
                        ));
                    }
                }
                None => {
                    mm.hit(format!("rust produced FEWER rows than java at row {checked}"));
                    break;
                }
            }
        }
        if rows.next().is_some() {
            mm.hit("rust produced MORE rows than java".to_string());
        }
        println!("interp.csv       : interp_count {interp_count}, checked {checked}, mismatches {}", mm.count);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    // ------------------------------------------------------------------
    // climate.csv — biome selection over captured targets (P2.4)
    // ------------------------------------------------------------------
    let climate_path = vec_dir.join("climate.csv");
    if run_climate && climate_path.exists() {
        let mut list = chunk_factory::climate::ParameterList::new(
            chunk_factory::vanilla_biomes::overworld_points()
                .into_iter()
                .map(|(p, n)| (p, n.to_string()))
                .collect(),
        );
        let rows = load_csv(&climate_path).expect("climate.csv");
        let mut mm = Mismatches::new("climate", 12);
        let mut checked = 0usize;
        for row in &rows {
            let target = chunk_factory::climate::TargetPoint {
                temperature: row.i64(3).unwrap(),
                humidity: row.i64(4).unwrap(),
                continentalness: row.i64(5).unwrap(),
                erosion: row.i64(6).unwrap(),
                depth: row.i64(7).unwrap(),
                weirdness: row.i64(8).unwrap(),
            };
            let got = list.find_value(&target).to_string();
            let expect = row.c(9).to_string();
            checked += 1;
            if got != expect {
                mm.hit(format!("q=({},{},{}) got {got}, java {expect}", row.c(0), row.c(1), row.c(2)));
            }
        }
        println!("climate.csv      : checked {checked}, mismatches {}", mm.count);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    // ------------------------------------------------------------------
    // climate_points.csv — the ported table vs the LIVE server list (P2.4)
    // ------------------------------------------------------------------
    let points_path = vec_dir.join("climate_points.csv");
    if run_climate_table && points_path.exists() {
        let points = chunk_factory::vanilla_biomes::overworld_points();
        let rows = load_csv(&points_path).expect("climate_points.csv");
        let mut mm = Mismatches::new("climate-table", 12);
        let mut checked = 0usize;
        let n = rows.len().max(points.len());
        for idx in 0..n {
            match (rows.get(idx), points.get(idx)) {
                (Some(row), Some((p, name))) => {
                    checked += 1;
                    let got = [
                        p.temperature.min, p.temperature.max,
                        p.humidity.min, p.humidity.max,
                        p.continentalness.min, p.continentalness.max,
                        p.erosion.min, p.erosion.max,
                        p.depth.min, p.depth.max,
                        p.weirdness.min, p.weirdness.max,
                        p.offset,
                    ];
                    for (k, g) in got.iter().enumerate() {
                        let e = row.i64(k).unwrap();
                        if g != &e {
                            mm.hit(format!("row {idx} col {k}: rust {g}, java {e}"));
                            break;
                        }
                    }
                    if row.c(13) != *name {
                        mm.hit(format!("row {idx}: rust {name}, java {}", row.c(13)));
                    }
                }
                _ => {
                    mm.hit(format!("row count mismatch: rust {}, java {}", points.len(), rows.len()));
                    break;
                }
            }
        }
        println!("climate_points   : rust {} rows, java {} rows, checked {checked}, mismatches {}", points.len(), rows.len(), mm.count);
        total_checked += checked;
        total_mismatch += mm.count;
    }

    println!("TOTAL: {total_checked} checked, {total_mismatch} mismatches");
    if total_mismatch > 0 {
        println!("VERDICT: ZERO-DIFF GATE FAIL");
        std::process::exit(1);
    }
    println!("VERDICT: ZERO-DIFF GATE PASS (bit-exact)");
}

fn router_field<'a>(r: &'a Router, name: &str) -> Option<&'a chunk_factory::density::Df> {
    Some(match name {
        "barrier" => &r.barrier,
        "fluid_level_floodedness" => &r.fluid_level_floodedness,
        "fluid_level_spread" => &r.fluid_level_spread,
        "lava" => &r.lava,
        "temperature" | "sampler.temperature" => &r.temperature,
        "vegetation" | "sampler.vegetation" => &r.vegetation,
        "continents" | "sampler.continents" => &r.continents,
        "erosion" | "sampler.erosion" => &r.erosion,
        "depth" | "sampler.depth" => &r.depth,
        "ridges" | "sampler.ridges" => &r.ridges,
        "preliminary_surface_level" => &r.preliminary_surface_level,
        "final_density" => &r.final_density,
        "vein_toggle" => &r.vein_toggle,
        "vein_ridged" => &r.vein_ridged,
        "vein_gap" => &r.vein_gap,
        _ => return None,
    })
}
