//! NCF P5.3 increment 2d pilot — piece_dump: assemble village structures
//! at seed 3053459 (and friends) and dump the piece box list + junctions
//! as JSON for the java-oracle capture comparison.
//!
//! The Java oracle side (CI): a Purpur 1.21.10 server with the same worldgen
//! extract generates the same placement chunk and dumps
//! StructureStart.getPieces() boxes (piece.boundingBox + groundLevelDelta +
//! getJunctions()) — the dump here must be box-identical.
//!
//! usage: piece_dump <worldgen-dir> <extract-root> <seed> <cx> <cz>
//!         [--set minecraft:villages]
use chunk_factory::height_feed::ColumnHeightSource;
use chunk_factory::piece_feed::{
    biome_list_for, biome_tag_set, structure_start_for_chunk, DirPoolSource,
};
use chunk_factory::router::{RandomState, WorldgenDir};
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 {
        eprintln!(
            "usage: piece_dump <worldgen-dir> <extract-root> <seed> <cx> <cz> [--set minecraft:villages]"
        );
        std::process::exit(2);
    }
    let dir = WorldgenDir::load(Path::new(&args[1])).expect("worldgen dir");
    let root = Path::new(&args[2]).to_path_buf();
    let seed: i64 = args[3].parse().expect("seed");
    let cx: i32 = args[4].parse().expect("cx");
    let cz: i32 = args[5].parse().expect("cz");
    let set_key = if args.len() > 7 && args[6] == "--set" {
        args[7].clone()
    } else {
        "minecraft:villages".to_string()
    };

    // --find: scan chunks in a growing square around (cx, cz) for placement
    // chunks of the set and print them (no assembly) — the pilot locator.
    if args.len() > 6 && args[6] == "--find" {
        let dir2 = WorldgenDir::load(Path::new(&args[1])).expect("worldgen dir");
        let text = dir2
            .get("minecraft", "structure_set", &set_key.split_once(':').unwrap().1)
            .expect("set json");
        let (placement, entries) =
            chunk_factory::piece_feed::parse_structure_set_json(&text).expect("parse set");
        let radius = if args.len() > 8 { args[8].parse().unwrap_or(64) } else { 64 };
        let mut found = 0;
        for r in 0..=radius {
            for dz in (-r..=r).map(|v: i32| v) {
                for dx in (-r..=r).map(|v: i32| v) {
                    if dx.abs() != r && dz.abs() != r {
                        continue; // ring only
                    }
                    let x = cx + dx;
                    let z = cz + dz;
                    if placement.is_placement_chunk(seed, x, z) {
                        println!("PLACEMENT {x} {z} ({} entries)", entries.len());
                        found += 1;
                        if found >= 8 {
                            return;
                        }
                    }
                }
            }
        }
        println!("NO-PLACEMENT within radius {radius}");
        return;
    }

    let rs = RandomState::build_overworld(&dir, seed).expect("build_overworld");
    let mut pools = DirPoolSource::new(&dir);
    let mut sampler = ColumnHeightSource::new(&rs, seed);

    let t0 = std::time::Instant::now();
    let result = structure_start_for_chunk(
        &dir,
        &root,
        &rs,
        seed,
        cx,
        cz,
        &set_key,
        &mut sampler,
        &mut pools,
    )
    .expect("structure_start_for_chunk");
    let dt = t0.elapsed();

    // Loud notes (Java: LOGGER.warn / crash paths) — printed to stderr so
    // the JSON on stdout stays machine-comparable.
    if !pools.missing_pools.is_empty() {
        eprintln!("MISSING_POOLS {:?}", pools.missing_pools);
    }
    if !pools.missing_templates.is_empty() {
        eprintln!("MISSING_TEMPLATES {:?}", pools.missing_templates);
    }
    if !pools.unsupported.is_empty() {
        eprintln!("UNSUPPORTED {:?}", pools.unsupported);
    }

    match result {
        None => {
            println!("{{\"set\":\"{set_key}\",\"chunk\":[{cx},{cz}],\"start\":null}}");
        }
        Some((key, assembly, adapt)) => {
            let adapt_name = match adapt {
                chunk_factory::beardifier::TerrainAdjustment::None => "none",
                chunk_factory::beardifier::TerrainAdjustment::Bury => "bury",
                chunk_factory::beardifier::TerrainAdjustment::BeardThin => "beard_thin",
                chunk_factory::beardifier::TerrainAdjustment::BeardBox => "beard_box",
                chunk_factory::beardifier::TerrainAdjustment::Encapsulate => "encapsulate",
            };
            println!("{{\"set\":\"{set_key}\",\"chunk\":[{cx},{cz}],\"structure\":\"{key}\",");
            println!(" \"terrain_adaptation\":\"{adapt_name}\",");
            println!(
                " \"stub\":[{},{},{}],\"piece_count\":{},\"assembly_ms\":{},",
                assembly.stub_position.0,
                assembly.stub_position.1,
                assembly.stub_position.2,
                assembly.pieces.len(),
                dt.as_millis()
            );
            println!(" \"pieces\":[");
            for (i, p) in assembly.pieces.iter().enumerate() {
                let b = &p.bounding_box;
                let rot_name = match p.rotation {
                    chunk_factory::jigsaw::Rotation::None => "NONE",
                    chunk_factory::jigsaw::Rotation::Clockwise90 => "CLOCKWISE_90",
                    chunk_factory::jigsaw::Rotation::Clockwise180 => "CLOCKWISE_180",
                    chunk_factory::jigsaw::Rotation::Counterclockwise90 => "COUNTERCLOCKWISE_90",
                };
                let elem = match &p.element {
                    chunk_factory::jigsaw::PoolElement::Single { location, .. } => {
                        format!("LegacySingle[{location}]")
                    }
                    chunk_factory::jigsaw::PoolElement::List { elements, .. } => {
                        format!("List[{}]", elements.len())
                    }
                    chunk_factory::jigsaw::PoolElement::Feature { feature_id, .. } => {
                        format!("Feature[{feature_id}]")
                    }
                    chunk_factory::jigsaw::PoolElement::Empty => "Empty".to_string(),
                    chunk_factory::jigsaw::PoolElement::Unsupported { kind } => {
                        format!("Unsupported[{kind}]")
                    }
                };
                println!(
                    "  {{\"i\":{i},\"box\":[{},{},{},{},{},{}],\"gld\":{},\"rotation\":\"{}\",\"element\":\"{}\",\"junctions\":[",
                    b.min_x,
                    b.min_y,
                    b.min_z,
                    b.max_x,
                    b.max_y,
                    b.max_z,
                    p.ground_level_delta,
                    rot_name,
                    elem
                );
                for (j, jn) in p.junctions.iter().enumerate() {
                    let sep = if j + 1 < p.junctions.len() { "," } else { "" };
                    println!(
                        "   [{},{},{},{},\"{}\"]{sep}",
                        jn.source_x,
                        jn.source_ground_y,
                        jn.source_z,
                        jn.delta_y,
                        match jn.dest_projection {
                            chunk_factory::jigsaw::Projection::Rigid => "rigid",
                            chunk_factory::jigsaw::Projection::TerrainMatching => "terrain_matching",
                        }
                    );
                }
                let comma = if i + 1 < assembly.pieces.len() { "}," } else { "}" };
                println!("  ]{comma}");
            }
            println!(" ]}}");
        }
    }
    let _ = (biome_list_for, biome_tag_set, HashMap::<String, ()>::new); // re-export smoke
}
