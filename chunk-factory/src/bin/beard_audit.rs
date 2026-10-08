//! P5.3 increment 6 diagnostic — attribute gate-p2 block divergences to the
//! responsible adapting starts and dump the feed's missing-template list.
//!
//! usage: beard_audit <worldgen-dir> <extract-root> <seed> <cx> <cz> [--block X Y Z]
use chunk_factory::height_feed::ColumnHeightSource;
use chunk_factory::piece_feed::{
    structure_start_for_chunk, PickDiag, DirPoolSource,
};
use chunk_factory::router::{RandomState, WorldgenDir};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 7 {
        eprintln!(
            "usage: beard_audit <worldgen-dir> <extract-root> <seed> <cx> <cz> [--block X Y Z]"
        );
        std::process::exit(2);
    }
    let wg = &args[1];
    let root = Path::new(&args[2]).to_path_buf();
    let seed: i64 = args[3].parse().expect("seed");
    let cx: i32 = args[4].parse().expect("cx");
    let cz: i32 = args[5].parse().expect("cz");
    let block = args
        .iter()
        .position(|a| a == "--block")
        .map(|i| (args[i + 1].parse::<i32>().unwrap(), args[i + 2].parse::<i32>().unwrap(), args[i + 3].parse::<i32>().unwrap()));
    // optional: --only <set-key> — limit the audit to one structure set
    // (assemblies are expensive; nether_fossils spacing 4 floods ±8)
    let only = args
        .iter()
        .position(|a| a == "--only")
        .map(|i| args[i + 1].clone());

    std::env::set_var("NCF_DATA_ROOT", wg);
    let dir = WorldgenDir::load(Path::new(wg)).expect("worldgen dir");
    let rs = RandomState::build_overworld(&dir, seed).expect("rs");
    let mut sampler = ColumnHeightSource::new(&rs, seed);
    let mut pools = DirPoolSource::new(&dir);

    let min_block_x = cx * 16;
    let min_block_z = cz * 16;
    let max_x15 = min_block_x + 15;
    let max_z15 = min_block_z + 15;
    println!("target chunk ({cx},{cz}) column [{min_block_x}..{max_x15}] x [{min_block_z}..{max_z15}]");

    // The adapting sets: same enumeration as BeardFeed::new.
    let mut adapting_sets: Vec<String> = Vec::new();
    for ns in dir.namespaces() {
        for set_name in dir.list(&ns, "structure_set") {
            let Some(text) = dir.get(&ns, "structure_set", &set_name) else { continue };
            let Ok(j) = chunk_factory::json::parse(&text) else { continue };
            let placement_ok = j
                .get("placement")
                .and_then(|p| p.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("minecraft:random_spread")
                == "minecraft:random_spread";
            if !placement_ok {
                continue;
            }
            let has_adapting = j
                .get("structures")
                .and_then(|s| s.as_arr())
                .map(|entries| {
                    entries.iter().any(|e| {
                        e.get("structure")
                            .and_then(|v| v.as_str())
                            .and_then(|k| {
                                let (sns, spath) = k.split_once(':')?;
                                let stext = dir.get(sns, "structure", spath)?;
                                let sj = chunk_factory::json::parse(stext).ok()?;
                                sj.get("terrain_adaptation")
                                    .and_then(|v| v.as_str())
                                    .map(|a| a != "none")
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if has_adapting {
                adapting_sets.push(format!("{ns}:{set_name}"));
            }
        }
    }
    println!("adapting sets: {adapting_sets:?}");
    let adapting_sets: Vec<String> = match &only {
        Some(k) => adapting_sets.into_iter().filter(|s| s.contains(k)).collect(),
        None => adapting_sets,
    };

    let mut diag = PickDiag::default();
    // PASS 1 (verbose): every placement-pick hit in +-8 with its outcome —
    // distinguishes "no start" from "start biome-rejected / empty assembly".
    for set_key in &adapting_sets {
        let (sns, spath) = set_key.split_once(':').unwrap();
        let Some(stext) = dir.get(sns, "structure_set", spath) else { continue };
        let Ok((_placement, _entries)) = chunk_factory::piece_feed::parse_structure_set_json(&stext) else { continue };
        for px in cx - 8..=cx + 8 {
            for pz in cz - 8..=cz + 8 {
                if !_placement.is_placement_chunk(seed, px, pz) {
                    continue;
                }
                // The full isStructureChunk gate (frequency + exclusion) —
                // the prescreen alone over-places (addendum 51, seed 90210
                // outpost: legacy_type_1 rejected it on java's side).
                if !_placement.is_structure_chunk(
                    seed,
                    px,
                    pz,
                    chunk_factory::piece_feed::exclusion_other_placement(&dir, &_placement).as_ref(),
                ) {
                    if _placement.frequency < 1.0 {
                        println!(
                            "PICK set {set_key} @ ({px},{pz}) -> None (frequency {} < 1.0 rejected)",
                            _placement.frequency
                        );
                    } else {
                        println!(
                            "PICK set {set_key} @ ({px},{pz}) -> None (exclusion zone rejected)"
                        );
                    }
                    continue;
                }
                let hit = structure_start_for_chunk(
                    &dir, &root, &rs, seed, px, pz, set_key, &mut sampler, &mut pools, &mut diag,
                )
                .expect("start_for");
                match &hit {
                    Some((structure_key, assembly, adj)) => {
                        let np = assembly.pieces.len();
                        println!(
                            "PICK set {set_key} @ ({px},{pz}) -> {structure_key} adj={adj:?} pieces={np}"
                        );
                        if np == 0 {
                            println!("  (EMPTY assembly — Java would still reference it)");
                        }
                    }
                    None => println!("PICK set {set_key} @ ({px},{pz}) -> None (biome-rejected / skipped)"),
                }
            }
        }
    }
    // PASS 2: START detail for starts whose union box touches the column.
    for set_key in &adapting_sets {
        for px in cx - 8..=cx + 8 {
            for pz in cz - 8..=cz + 8 {
                let hit = structure_start_for_chunk(
                    &dir, &root, &rs, seed, px, pz, set_key, &mut sampler, &mut pools, &mut diag,
                )
                .expect("start_for");
                let Some((structure_key, assembly, adj)) = hit else { continue };
                if adj == chunk_factory::beardifier::TerrainAdjustment::None {
                    continue;
                }
                // whole-start box vs column
                let mut union: Option<chunk_factory::beardifier::InclusiveBox> = None;
                for piece in &assembly.pieces {
                    union = Some(match union {
                        Some(u) => chunk_factory::beardifier::InclusiveBox::encapsulating(&u, &piece.bounding_box),
                        None => piece.bounding_box,
                    });
                }
                let Some(sb) = union else { continue };
                let touches = sb.max_x >= min_block_x
                    && sb.min_x <= min_block_x + 15
                    && sb.max_z >= min_block_z
                    && sb.min_z <= min_block_z + 15;
                if !touches {
                    continue;
                }
                println!(
                    "START {structure_key} set {set_key} @ ({px},{pz}) adj={adj:?} pieces={} junctions={} union=[{},{},{} .. {},{},{}] stub=({}, {}, {})",
                    assembly.pieces.len(),
                    assembly.pieces.iter().map(|p| p.junctions.len()).sum::<usize>(),
                    sb.min_x, sb.min_y, sb.min_z, sb.max_x, sb.max_y, sb.max_z,
                    assembly.stub_position.0, assembly.stub_position.1, assembly.stub_position.2,
                );
                for (i, p) in assembly.pieces.iter().enumerate() {
                    let b = &p.bounding_box;
                    println!(
                        "  piece[{i}] box=[{},{},{} .. {},{},{}] gld={} proj={:?} elem={} junc={}",
                        b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z,
                        p.ground_level_delta, p.element.projection(),
                        p.element.kind(),
                        p.junctions.len(),
                    );
                    for j in &p.junctions {
                        println!("    junc @ ({},{},{})", j.source_x, j.source_ground_y, j.source_z);
                    }
                }
            }
        }
    }

    let mut missing: Vec<&String> = pools.missing_templates.iter().collect();
    missing.sort();
    missing.dedup();
    println!("missing_templates ({} distinct):", missing.len());
    for m in missing {
        println!("  {m}");
    }
    let mut unsup: Vec<&String> = pools.unsupported.iter().collect();
    unsup.sort();
    unsup.dedup();
    println!("unsupported ({} distinct):", unsup.len());
    for u in unsup {
        println!("  {u}");
    }

    if let Some((lx, ly, lz)) = block {
        // stagediff divergence coords are CHUNK-LOCAL in x/z; the piece boxes
        // are absolute — convert before attributing.
        let bx = min_block_x + lx;
        let bz = min_block_z + lz;
        let by = ly;
        // Per-piece attribution at the divergent block using the assembled
        // starts touching this column (isCloseToChunk 12 filter as in feed).
        println!("per-piece attribution at local ({lx},{ly},{lz}) = absolute ({bx},{by},{bz}):");
        for set_key in &adapting_sets {
            for px in cx - 8..=cx + 8 {
                for pz in cz - 8..=cz + 8 {
                    let hit = structure_start_for_chunk(
                        &dir, &root, &rs, seed, px, pz, set_key, &mut sampler, &mut pools, &mut diag,
                    )
                    .expect("start_for2");
                    let Some((structure_key, assembly, adj)) = hit else { continue };
                    if adj == chunk_factory::beardifier::TerrainAdjustment::None {
                        continue;
                    }
                    for p in &assembly.pieces {
                        let b = &p.bounding_box;
                        let close = b.max_x >= min_block_x - 12
                            && b.min_x <= min_block_x + 15 + 12
                            && b.max_z >= min_block_z - 12
                            && b.min_z <= min_block_z + 15 + 12;
                        if !close {
                            continue;
                        }
                        if p.element.projection() != chunk_factory::jigsaw::Projection::Rigid {
                            continue;
                        }
                        let rigid = chunk_factory::beardifier::BeardRigid::new(
                            b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z, adj,
                            p.ground_level_delta,
                        );
                        let c = chunk_factory::beardifier::beardifier_piece_contribution(&rigid, bx, by, bz);
                        if c != 0.0 {
                            println!(
                                "  {structure_key}@({px},{pz}) piece box=[{},{},{}..{},{},{}] gld={} -> {c}",
                                b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z, p.ground_level_delta,
                            );
                        }
                    }
                }
            }
        }
    }

    // --biome X Y Z: dump the 6 climate scalars (f64 + quantized) and the
    // selected biome at an ABSOLUTE block coordinate — the stub-position
    // biome oracle probe. Mirrors production structure_scan::biome_at: the
    // coords are QUART-SNAPPED before evaluation (java isValidBiome samples
    // the biome source in quart coords; addendum 51).
    if let Some(i) = args.iter().position(|a| a == "--biome") {
        let px: i32 = args[i + 1].parse().unwrap();
        let py: i32 = args[i + 2].parse().unwrap();
        let pz: i32 = args[i + 3].parse().unwrap();
        let (qx, qy, qz) = (px >> 2 << 2, py >> 2 << 2, pz >> 2 << 2);
        if (qx, qy, qz) != (px, py, pz) {
            println!("  quart-snap ({px},{py},{pz}) -> ({qx},{qy},{qz})");
        }
        let px = qx;
        let py = qy;
        let pz = qz;
        let fields: [&chunk_factory::density::Df; 6] = [
            &rs.router.temperature,
            &rs.router.vegetation,
            &rs.router.continents,
            &rs.router.erosion,
            &rs.router.depth,
            &rs.router.ridges,
        ];
        let names = ["temperature", "humidity", "continentalness", "erosion", "depth", "weirdness"];
        let mut vals = [0.0f64; 6];
        for (fi, f) in fields.iter().enumerate() {
            vals[fi] = f.compute(&rs.bank, px, py, pz);
        }
        let mut list = chunk_factory::piece_feed::biome_list_for(&rs);
        for (fi, v) in vals.iter().enumerate() {
            let q = chunk_factory::climate::quantize_coord(*v as f32);
            println!("  {:>15} = {:>20} (quantized {q})", names[fi], v);
        }
        let target = chunk_factory::climate::TargetPoint {
            temperature: chunk_factory::climate::quantize_coord(vals[0] as f32),
            humidity: chunk_factory::climate::quantize_coord(vals[1] as f32),
            continentalness: chunk_factory::climate::quantize_coord(vals[2] as f32),
            erosion: chunk_factory::climate::quantize_coord(vals[3] as f32),
            depth: chunk_factory::climate::quantize_coord(vals[4] as f32),
            weirdness: chunk_factory::climate::quantize_coord(vals[5] as f32),
        };
        let biome = list.find_value(&target);
        println!("  BIOME at ({px},{py},{pz}) = {biome}");
    }
}
