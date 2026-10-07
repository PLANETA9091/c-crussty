//! T40 probe: verify WHAT the extract contains for the pack chain and which
//! biome our scalars resolve at given quart coords.
//!
//! Usage: t40probe <extract_root> <seed> <qx,qy,qz>...
use chunk_factory::climate::{quantize_coord, ParameterList, TargetPoint};
use chunk_factory::router::{RandomState, WorldgenDir};
use chunk_factory::vanilla_biomes;
use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().expect("root");
    let seed: i64 = args.next().expect("seed").parse().expect("seed");
    let quarts: Vec<(i32, i32, i32)> = args
        .map(|s| {
            let p: Vec<&str> = s.split(',').collect();
            (
                p[0].parse().expect("qx"),
                p[1].parse().expect("qy"),
                p[2].parse().expect("qz"),
            )
        })
        .collect();

    let dir = WorldgenDir::load(Path::new(&root)).expect("load");

    // --- 1. WHAT is in the extract (pack or vanilla versions?) ------------
    let temp = dir.get("minecraft", "noise", "temperature").expect("noise/temperature");
    println!("noise/temperature.json  = {}", temp.trim());
    let veg = dir.get("minecraft", "noise", "vegetation").expect("noise/vegetation");
    println!("noise/vegetation.json   = {}", veg.trim());
    let ero = dir.get("minecraft", "noise", "erosion").expect("noise/erosion");
    println!("noise/erosion.json      = {}", ero.trim());
    let ns = dir
        .get("minecraft", "noise_settings", "overworld")
        .expect("noise_settings/overworld");
    println!(
        "noise_settings/overworld: overlay marker preliminary_surface_level = {}",
        ns.contains("preliminary_surface_level")
    );
    let ero_df = dir
        .get("minecraft", "density_function", "overworld/erosion")
        .expect("df overworld/erosion");
    println!(
        "df overworld/erosion: pack range_choice chain = {}",
        ero_df.contains("range_choice")
    );
    let eco_df = dir
        .get("minecraft", "density_function", "overworld/effective_continentalness")
        .expect("df effective_continentalness");
    println!(
        "df effective_continentalness: pack version = {}",
        eco_df.contains("terralith") || eco_df.contains("range_choice")
    );

    // --- 2. build_overworld + evaluate ------------------------------------
    let rs = RandomState::build_overworld(&dir, seed).expect("build_overworld");
    let mut list = ParameterList::new(match &rs.biome_points {
        Some(p) => p.clone(),
        None => vanilla_biomes::overworld_points()
            .into_iter()
            .map(|(pt, n)| (pt, n.to_string()))
            .collect(),
    });
    let names: std::collections::BTreeSet<&String> = match &rs.biome_points {
        Some(p) => p.iter().map(|(_, n)| n).collect(),
        None => std::collections::BTreeSet::new(),
    };
    println!(
        "table: biome_points={} distinct={}",
        rs.biome_points.as_ref().map(|p| p.len()).unwrap_or(0),
        names.len()
    );
    let tcnt = rs.noise_key_by_index.iter().filter(|k| k.contains("temperature")).count();
    println!(
        "wired noises: {} total; temperature-family keys present: {tcnt}",
        rs.noise_key_by_index.len()
    );

    for (qx, qy, qz) in quarts {
        let (bx, by, bz) = (qx * 4, qy * 4, qz * 4);
        let r = &rs.router;
        let t = TargetPoint {
            temperature: quantize_coord(r.temperature.compute(&rs.bank, bx, by, bz) as f32),
            humidity: quantize_coord(r.vegetation.compute(&rs.bank, bx, by, bz) as f32),
            continentalness: quantize_coord(r.continents.compute(&rs.bank, bx, by, bz) as f32),
            erosion: quantize_coord(r.erosion.compute(&rs.bank, bx, by, bz) as f32),
            depth: quantize_coord(r.depth.compute(&rs.bank, bx, by, bz) as f32),
            weirdness: quantize_coord(r.ridges.compute(&rs.bank, bx, by, bz) as f32),
        };
        let winner = list.find_value(&t).to_string();
        println!(
            "quart ({qx},{qy},{qz}) block ({bx},{by},{bz}): temp={} hum={} cont={} ero={} dep={} weird={} -> {winner}",
            t.temperature, t.humidity, t.continentalness, t.erosion, t.depth, t.weirdness
        );
    }
}
