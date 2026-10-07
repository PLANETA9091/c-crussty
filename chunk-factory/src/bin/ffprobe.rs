//! NCF P5.3 increment 2c probe — the REAL FirstFreeHeight cross-validation.
//!
//! Compares, per column, the 2c column machine (height_feed::
//! ColumnHeightSource::get_base_height_world_surface_wg — the exact
//! iterateNoiseColumn/getBaseHeight machine Java uses for structure height
//! sampling: 1-cell NoiseChunk, SECTION aquifer grid basis, CELL FlatCache
//! window) against the already CI-validated full-chunk machine
//! (filler::generate_noise_chunk WORLD_SURFACE_WG heightmap — 4x4-cell
//! NoiseChunk, chunk window, chunk grid basis).
//!
//! Two possible outcomes, both informative and both honest:
//!   - zero divergences => the two Rust machines agree (strong internal
//!     cross-validation of the 1-cell machine against the CI-green chunk
//!     machine) AND Java's getFirstFreeHeight == chunk heightmap for the
//!     corpus (the I8 approximation was exact there);
//!   - non-zero divergences => the FlatCache window binding difference is
//!     OBSERVABLE in Java too (two different Java machines); the column
//!     machine is the structure-sampling oracle (getBaseHeight), the chunk
//!     machine is the chunk oracle — the piece engine must use the column
//!     one. Each divergence is printed with coords + heights for bit-level
//!     triage.
//!
//! usage: ffprobe <worldgen-dir> <seed> <cx> <cz> [count] [settings ns:name]
use chunk_factory::filler::generate_noise_chunk;
use chunk_factory::height_feed::ColumnHeightSource;
use chunk_factory::router::{RandomState, WorldgenDir};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("usage: ffprobe <worldgen-dir> <seed> <cx> <cz> [count] [settings ns:name]");
        std::process::exit(2);
    }
    let dir = WorldgenDir::load(Path::new(&args[1])).expect("worldgen dir");
    let seed: i64 = args[2].parse().expect("seed");
    let cx0: i32 = args[3].parse().expect("cx");
    let cz0: i32 = args[4].parse().expect("cz");
    let count: i32 = if args.len() > 5 { args[5].parse().expect("count") } else { 1 };
    let rs = match args.len() > 6 {
        false => RandomState::build_overworld(&dir, seed).expect("build_overworld"),
        true => {
            let (ns, name) = args[6].split_once(':').expect("settings ns:name");
            RandomState::build(&dir, ns, name, seed).expect("build")
        }
    };
    let mut src = ColumnHeightSource::new(&rs, seed);
    let cw = rs.settings.noise_size_horizontal * 4;

    let mut total_cols = 0i64;
    let mut total_div = 0i64;
    let t_start = std::time::Instant::now();
    let mut t_chunkgen = std::time::Duration::ZERO;
    for cz in cz0..cz0 + count {
        for cx in cx0..cx0 + count {
            let t0 = std::time::Instant::now();
            let chunk = generate_noise_chunk(&rs, seed, cx, cz).expect("generate_noise_chunk");
            let t_gen = t0.elapsed();
            t_chunkgen += t_gen;
            let t1 = std::time::Instant::now();
            let hm = &chunk.heightmaps[1]; // [ocean, surface] — WorldSurfaceWg
            let min_x = cx * 16;
            let min_z = cz * 16;
            let mut divs: Vec<(i32, i32, i32, i32)> = Vec::new(); // (x, z, col, hm)
            for lz in 0..16i32 {
                for lx in 0..16i32 {
                    let x = min_x + lx;
                    let z = min_z + lz;
                    let col = src.get_base_height_world_surface_wg(x, z);
                    let h = hm.first_available[(lx + lz * 16) as usize];
                    total_cols += 1;
                    if col != h {
                        total_div += 1;
                        if divs.len() < 8 {
                            divs.push((x, z, col, h));
                        }
                    }
                }
            }
            let t_cols = t1.elapsed();
            let status = if divs.is_empty() { "EQUAL" } else { "DIVERGE" };
            println!(
                "chunk ({cx},{cz}) [{status}] 256 columns, divergences: {}  [gen {t_gen:?}, cols {t_cols:?}]",
                divs.len().min(999)
            );
            for (x, z, col, h) in &divs {
                let cell_x = x.div_euclid(cw) * cw;
                let cell_z = z.div_euclid(cw) * cw;
                println!(
                    "    ({x},{z}) col={col} hm={h}  cell=({cell_x},{cell_z}) cell_quart=({}, {})",
                    cell_x >> 2,
                    cell_z >> 2
                );
            }
        }
    }
    println!(
        "SUMMARY seed={seed} columns={total_cols} divergences={total_div} wall={:?} chunkgen={t_chunkgen:?} cols={:?}",
        t_start.elapsed(),
        t_start.elapsed() - t_chunkgen
    );
}
