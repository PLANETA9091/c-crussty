//! stagediff — NCF task 5 gate: compare the Java staged-dump oracle (task 5-a)
//! against Rust filler output (filler.rs), block-by-block + biome-by-biome +
//! heightmap longs + postprocessing counts.
//!
//! Usage:
//!   stagediff <java_dir> <rust_dir> [--manifest] [--max N] [--full]
//!     java_dir: dir with seed_<seed>/c_<x>_<z>.nbt (task 5-a layout)
//!     rust_dir: dir with the same layout produced by `stagediff --gen`
//!   stagediff --gen <seed> <cx> <cz> <worldgen_dir> <out_file> [noise|surface]
//!     generate one chunk with the Rust filler and write it staged-schema
//!   stagediff --selftest
#![allow(clippy::type_complexity)]

use chunk_factory::filler::generate_noise_chunk;
use chunk_factory::router::{RandomState, WorldgenDir};
use chunk_factory::status_chain::{generate_carvers_chunk, generate_surface_chunk, StageKit};
use chunk_factory::sections::{filler_to_staged, parse_staged_file, state_table_from_staged, write_staged_file, StagedChunk};
use std::path::Path;
use std::process::ExitCode;

fn compare_pair(java: &StagedChunk, rust: &StagedChunk) -> Result<(), String> {
    if java.x != rust.x || java.z != rust.z {
        return Err(format!("chunk id mismatch java({},{}) rust({},{})", java.x, java.z, rust.x, rust.z));
    }
    if java.sections.len() != rust.sections.len() {
        return Err(format!("section count java={} rust={}", java.sections.len(), rust.sections.len()));
    }
    let _table = state_table_from_staged(java);
    for (si, (js, rs)) in java.sections.iter().zip(rust.sections.iter()).enumerate() {
        if js.y != rs.y {
            return Err(format!("section Y mismatch idx={si} java={} rust={}", js.y, rs.y));
        }
        for (bi, (&jd, &rd)) in js.data.iter().zip(rs.data.iter()).enumerate() {
            if jd as usize >= js.palette.len() || rd as usize >= rs.palette.len() {
                // palette oob = structural divergence
                if jd != rd {
                    return Err(format!("palette idx mismatch sec={si} i={bi} java={jd} rust={rd}"));
                }
                continue;
            }
            if js.palette[jd as usize] != rs.palette[rd as usize] {
                let y = js.y as i32 * 16 + (bi as i32) / 256;
                let rem = bi % 256;
                let z = rem / 16;
                let x = rem % 16;
                return Err(format!(
                    "block divergence sec={si} @ ({},{},{}) java={} rust={}",
                    x, y, z, js.palette[jd as usize], rs.palette[rd as usize]
                ));
            }
        }
        for (qi, (&jd, &rd)) in js.biome_data.iter().zip(rs.biome_data.iter()).enumerate() {
            let jb = js.biome_palette.get(jd as usize).cloned().unwrap_or_default();
            let rb = rs.biome_palette.get(rd as usize).cloned().unwrap_or_default();
            if jb != rb {
                let y = js.y as i32 * 4 + (qi as i32) / 16;
                let rem = qi % 16;
                let z = rem / 4;
                let x = rem % 4;
                return Err(format!("biome divergence sec={si} quart=({x},{y},{z}) java={jb} rust={rb}"));
            }
        }
    }
    // heightmaps: raw long arrays must match exactly (both sides packed)
    for (name, jraw) in &java.heightmaps {
        match rust.heightmaps.iter().find(|(n, _)| n == name) {
            Some((_, rraw)) => {
                if jraw != rraw {
                    let mut first_diff = None;
                    for (i, (a, b)) in jraw.iter().zip(rraw.iter()).enumerate() {
                        if a != b {
                            first_diff = Some(i);
                            break;
                        }
                    }
                    return Err(format!("heightmap {name} differs (first diff long idx {first_diff:?})"));
                }
            }
            None => return Err(format!("heightmap {name} missing on rust side")),
        }
    }
    Ok(())
}

fn list_staged(dir: &Path) -> Vec<(i32, i32, std::path::PathBuf)> {
    let mut out = Vec::new();
    // layout: <dir>/seed_*/c_<x>_<z>.nbt
    let mut seeds = std::fs::read_dir(dir).expect("java dir").collect::<Result<Vec<_>, _>>().expect("readdir");
    seeds.sort_by_key(|e| e.file_name());
    for seed_entry in seeds.iter() {
        let seed_dir = seed_entry.path();
        if !seed_dir.is_dir() {
            continue;
        }
        let mut files = std::fs::read_dir(&seed_dir).expect("seed dir").collect::<Result<Vec<_>, _>>().expect("readdir");
        files.sort_by_key(|e| e.file_name());
        for f in files.iter() {
            let name = f.file_name().to_string_lossy().to_string();
            if let Some(rest) = name.strip_prefix("c_").and_then(|r| r.strip_suffix(".nbt")) {
                let mut it = rest.split('_');
                if let (Some(x), Some(z)) = (it.next(), it.next()) {
                    if let (Ok(x), Ok(z)) = (x.parse::<i32>(), z.parse::<i32>()) {
                        out.push((x, z, f.path()));
                    }
                }
            }
        }
    }
    out
}

fn real_main(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|a| a == "--selftest") {
        selftest()?;
        println!("stagediff --selftest: GREEN");
        return Ok(0);
    }
    if args.first().map(|s| s == "--trace-surface").unwrap_or(false) {
        // --trace-surface <seed> <cx> <cz> <worldgen_dir> <out.tsv>
        let seed: i64 = args.get(1).ok_or("seed")?.parse().map_err(|_| "seed")?;
        let cx: i32 = args.get(2).ok_or("cx")?.parse().map_err(|_| "cx")?;
        let cz: i32 = args.get(3).ok_or("cz")?.parse().map_err(|_| "cz")?;
        let wg = args.get(4).ok_or("worldgen dir")?;
        let out = args.get(5).ok_or("out file")?;
        std::env::set_var("NCF_DATA_ROOT", wg);
        let dir = WorldgenDir::load(Path::new(wg)).map_err(|e| e.to_string())?;
        let mut rs = RandomState::build_overworld(&dir, seed).map_err(|e| e.to_string())?;
        let mut kit = StageKit::build(&mut rs, &dir).map_err(|e| e.to_string())?;
        let rows = chunk_factory::status_chain::trace_surface(&mut rs, &mut kit, &dir, seed, cx, cz)
            .map_err(|e| e.to_string())?;
        std::fs::write(out, rows).map_err(|e| e.to_string())?;
        println!("trace written to {out}");
        return Ok(0);
    }
    if args.first().map(|s| s == "--fallback-list").unwrap_or(false) {
        // --fallback-list <seed> <x0> <x1> <z0> <z1> <worldgen_dir>
        //   I8 structure-fallback prescan (T38-B pending P5.3): prints
        //   c_<x>_<z> lines (stagediff naming) for every chunk that the
        //   Beardifier could touch (terrain-adapting structures), to stdout;
        //   per-start / per-skip diagnostics go to stderr. The gate script
        //   moves BOTH corpora's files for these chunks out of the compare
        //   (same honest-exclusion protocol as the T38-A spawn exclusion).
        let seed: i64 = args.get(1).ok_or("--fallback-list: seed")?.parse().map_err(|_| "seed")?;
        let x0: i32 = args.get(2).ok_or("x0")?.parse().map_err(|_| "x0")?;
        let x1: i32 = args.get(3).ok_or("x1")?.parse().map_err(|_| "x1")?;
        let z0: i32 = args.get(4).ok_or("z0")?.parse().map_err(|_| "z0")?;
        let z1: i32 = args.get(5).ok_or("z1")?.parse().map_err(|_| "z1")?;
        let wg = args.get(6).ok_or("worldgen dir")?;
        std::env::set_var("NCF_DATA_ROOT", wg);
        let dir = WorldgenDir::load(Path::new(wg)).map_err(|e| e.to_string())?;
        let rs = RandomState::build_overworld(&dir, seed).map_err(|e| e.to_string())?;
        let t0 = std::time::Instant::now();
        let report = chunk_factory::structure_scan::scan_batch(&dir, &rs, seed, Path::new(wg), x0, x1, z0, z1)
            .map_err(|e| e.to_string())?;
        for s in &report.skipped {
            eprintln!("FALLBACK skipped {s}");
        }
        for s in &report.starts {
            eprintln!(
                "FALLBACK start structure={} set={} cx={} cz={} r={}{}",
                s.structure,
                s.set,
                s.cx,
                s.cz,
                s.radius,
                if s.unknown_validity { " (unknown-validity)" } else { "" }
            );
        }
        for (cx, cz) in &report.chunks {
            println!("c_{cx}_{cz}");
        }
        eprintln!(
            "FALLBACK summary: {} starts, {} chunks in [{x0}..{x1}]x[{z0}..{z1}] ({} of batch), scan {}ms",
            report.starts.len(),
            report.chunks.len(),
            ((x1 - x0 + 1) * (z1 - z0 + 1)) as i64,
            t0.elapsed().as_millis()
        );
        return Ok(0);
    }
    if args.first().map(|s| s == "--gen-batch").unwrap_or(false) {
        // --gen-batch <seed> <x0> <x1> <z0> <z1> <worldgen_dir> <out_dir>
        //   builds the RandomState ONCE (the real factory shape), generates
        //   the chunk range, writes staged-schema files, prints the timing.
        let seed: i64 = args.get(1).ok_or("--gen-batch: seed")?.parse().map_err(|_| "seed")?;
        let x0: i32 = args.get(2).ok_or("x0")?.parse().map_err(|_| "x0")?;
        let x1: i32 = args.get(3).ok_or("x1")?.parse().map_err(|_| "x1")?;
        let z0: i32 = args.get(4).ok_or("z0")?.parse().map_err(|_| "z0")?;
        let z1: i32 = args.get(5).ok_or("z1")?.parse().map_err(|_| "z1")?;
        let wg = args.get(6).ok_or("worldgen dir")?;
        let out = args.get(7).ok_or("out dir")?;
        let status = args
            .iter()
            .position(|a| a == "--status")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "noise".to_string());
        // tags (carver replaceable) resolve against the same extract
        std::env::set_var("NCF_DATA_ROOT", wg);
        let dir = WorldgenDir::load(Path::new(wg)).map_err(|e| e.to_string())?;
        let mut rs = RandomState::build_overworld(&dir, seed).map_err(|e| e.to_string())?;
        let mut kit = if status == "noise" {
            None
        } else {
            Some(StageKit::build(&mut rs, &dir).map_err(|e| e.to_string())?)
        };
        let seed_dir = Path::new(out).join(format!("seed_{seed}"));
        std::fs::create_dir_all(&seed_dir).map_err(|e| e.to_string())?;
        let status_key = format!("minecraft:{status}");
        let t0 = std::time::Instant::now();
        // NCF_TIMING=1: per-stage ms/chunk breakdown (Amdahl table, owner
        // directive 2026-10-07: "стадия, мс/чанк Rust, мс/чанк Java, доля").
        let timing = std::env::var("NCF_TIMING").is_ok();
        let mut stage_ms: std::collections::BTreeMap<&'static str, u128> = Default::default();
        let mut n = 0usize;
        for cx in x0..=x1 {
            for cz in z0..=z1 {
                let s0 = std::time::Instant::now();
                let fc = match (status.as_str(), kit.as_mut()) {
                    ("noise", _) => generate_noise_chunk(&rs, seed, cx, cz),
                    ("surface", Some(k)) => generate_surface_chunk(&mut rs, k, &dir, seed, cx, cz),
                    ("carvers", Some(k)) => generate_carvers_chunk(&mut rs, k, &dir, seed, cx, cz),
                    (s, _) => return Err(format!("gen-batch: unsupported status {s}")),
                }
                .map_err(|e| e.to_string())?;
                let stage = match status.as_str() {
                    "noise" => "noise_fill",
                    "surface" => "surface_rules",
                    "carvers" => "carvers",
                    _ => "gen",
                };
                let s1 = std::time::Instant::now();
                let mut st = filler_to_staged(&fc, &status_key, 4556);
                st.x = cx;
                st.z = cz;
                std::fs::write(seed_dir.join(format!("c_{cx}_{cz}.nbt")), write_staged_file(&st))
                    .map_err(|e| e.to_string())?;
                let s2 = std::time::Instant::now();
                if timing {
                    *stage_ms.entry(stage).or_default() += (s1 - s0).as_millis();
                    *stage_ms.entry("staged_nbt_write").or_default() += (s2 - s1).as_millis();
                }
                n += 1;
            }
        }
        let elapsed = t0.elapsed().as_secs_f64();
        if timing {
            for (stage, ms) in &stage_ms {
                eprintln!(
                    "TIMING stage={} total_ms={} chunks={n} ms_per_chunk={:.2}",
                    stage,
                    ms,
                    *ms as f64 / n as f64
                );
            }
        }
        println!("gen-batch[{status}]: {n} chunks in {elapsed:.2}s ({:.1} chunks/s single-core, incl. staged NBT write)", n as f64 / elapsed);
        return Ok(0);
    }
    if args.first().map(|s| s == "--gen").unwrap_or(false) {
        // --gen <seed> <cx> <cz> <worldgen_dir> <out_file> [status]
        let seed: i64 = args.get(1).ok_or("--gen: seed")?.parse().map_err(|_| "seed")?;
        let cx: i32 = args.get(2).ok_or("--gen: cx")?.parse().map_err(|_| "cx")?;
        let cz: i32 = args.get(3).ok_or("--gen: cz")?.parse().map_err(|_| "cz")?;
        let wg = args.get(4).ok_or("--gen: worldgen dir")?;
        let out = args.get(5).ok_or("--gen: out file")?;
        let status = args
            .get(6)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "noise".to_string());
        std::env::set_var("NCF_DATA_ROOT", wg);
        let dir = WorldgenDir::load(Path::new(wg)).map_err(|e| e.to_string())?;
        let mut rs = RandomState::build_overworld(&dir, seed).map_err(|e| e.to_string())?;
        let status_key = format!("minecraft:{status}");
        let fc = match status.as_str() {
            "noise" => generate_noise_chunk(&rs, seed, cx, cz),
            "surface" | "carvers" => {
                let mut kit = StageKit::build(&mut rs, &dir).map_err(|e| e.to_string())?;
                if status == "surface" {
                    generate_surface_chunk(&mut rs, &mut kit, &dir, seed, cx, cz)
                } else {
                    generate_carvers_chunk(&mut rs, &mut kit, &dir, seed, cx, cz)
                }
            }
            s => return Err(format!("--gen: unsupported status {s}")),
        }
        .map_err(|e| e.to_string())?;
        let mut st = filler_to_staged(&fc, &status_key, 4556);
        st.x = cx;
        st.z = cz;
        std::fs::write(out, write_staged_file(&st)).map_err(|e| e.to_string())?;
        println!("generated {out}");
        return Ok(0);
    }
    let java_dir = args.first().ok_or("usage: stagediff <java_dir> <rust_dir>")?;
    let rust_dir = args.get(1).ok_or("usage: stagediff <java_dir> <rust_dir>")?;
    let max: usize = args
        .iter()
        .position(|a| a == "--max")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(usize::MAX);
    let java_files = list_staged(Path::new(java_dir));
    let rust_files = list_staged(Path::new(rust_dir));
    if java_files.is_empty() {
        return Err(format!("no staged files under {java_dir}"));
    }
    let mut checked = 0usize;
    let mut equal = 0usize;
    let mut diverged = 0usize;
    let mut errs: Vec<String> = Vec::new();
    for (x, z, jpath) in java_files.iter().take(max) {
        let rpath = rust_files.iter().find(|(rx, rz, _)| rx == x && rz == z).map(|(_, _, p)| p);
        let rpath = match rpath {
            Some(p) => p,
            None => {
                diverged += 1;
                errs.push(format!("c_{x}_{z}: MISSING on rust side"));
                continue;
            }
        };
        let jb = std::fs::read(jpath).map_err(|e| e.to_string())?;
        let rb = std::fs::read(rpath).map_err(|e| e.to_string())?;
        let jsc = parse_staged_file(&jb)?;
        let rsc = parse_staged_file(&rb)?;
        checked += 1;
        match compare_pair(&jsc, &rsc) {
            Ok(()) => equal += 1,
            Err(e) => {
                diverged += 1;
                if errs.len() < 40 {
                    errs.push(format!("c_{x}_{z}: {e}"));
                }
            }
        }
    }
    println!("pairs={checked} equal={equal} diverged={diverged}");
    for e in &errs {
        println!("  {e}");
    }
    Ok(if diverged == 0 { 0 } else { 1 })
}

fn selftest() -> Result<(), String> {
    use chunk_factory::filler::{BiomeTable, FillerChunk, SectionData, StateTable};
    use chunk_factory::sections::{parse_staged_file, write_staged_file};
    let mut table = StateTable::new();
    let air = table.intern("minecraft:air", &[]);
    let stone = table.intern("minecraft:stone", &[]);
    let mut biomes = BiomeTable::new();
    let ocean = biomes.intern("minecraft:ocean");
    let mut sections = Vec::new();
    for i in 0..24 {
        let mut s = SectionData::new();
        if i < 4 {
            for v in s.states.iter_mut() {
                *v = stone;
            }
        }
        for b in s.biomes.iter_mut() {
            *b = ocean as u16;
        }
        sections.push(s);
    }
    let fc = FillerChunk {
        min_y: -64,
        height: 384,
        chunk_min_x: 112,
        chunk_min_z: 144,
        sections,
        state_table: table,
        biome_table: biomes,
        heightmaps: vec![
            chunk_factory::filler::HeightmapData {
                kind: chunk_factory::filler::HeightmapKind::OceanFloorWg,
                first_available: [-64; 256],
            },
            chunk_factory::filler::HeightmapData {
                kind: chunk_factory::filler::HeightmapKind::WorldSurfaceWg,
                first_available: [-64; 256],
            },
        ],
        post_processing: (0..24).map(|_| Vec::new()).collect(),
    };
    let mut st = filler_to_staged(&fc, "minecraft:noise", 4556);
    st.x = 7;
    st.z = 9;
    let bytes = write_staged_file(&st);
    let parsed = parse_staged_file(&bytes)?;
    // round-trip: rewrite from parsed and byte-compare (deterministic schema)
    let bytes2 = write_staged_file(&parsed);
    if bytes != bytes2 {
        return Err("selftest: round-trip bytes differ".into());
    }
    compare_pair(&parsed, &parsed).map_err(|e| format!("selftest: equal pair failed: {e}"))?;
    // flip one block -> divergence at the right coords
    let mut bad = parse_staged_file(&bytes)?;
    let _ = air;
    bad.sections[0].palette.push("minecraft:dirt".to_string());
    let last = bad.sections[0].palette.len() - 1;
    bad.sections[0].data[5 * 256 + 3 * 16 + 2] = last as u32;
    match compare_pair(&parsed, &bad) {
        Err(e) => {
            if !e.contains("block divergence sec=0 @ (2,-59,3)") {
                return Err(format!("selftest: wrong divergence report: {e}"));
            }
        }
        Ok(()) => return Err("selftest: diverged pair reported equal".into()),
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match real_main(&args) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("stagediff: {e}");
            ExitCode::FAILURE
        }
    }
}
