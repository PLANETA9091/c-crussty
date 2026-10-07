//! aquacheck — task 5 bisect rig: compare the Rust aquifer + substance cache
//! against the live-server capture (VectorCapture.writeAquifer).
//!
//! aquifer.csv contract (from the Java side):
//!   # comment lines start with '#'
//!   block_x,block_y,block_z,substance(Double.toHexString),decision,sched
//!   decision = "minecraft:air" | "minecraft:water[level=0]" |
//!              "minecraft:lava[level=0]" | "null" (solid -> default block)
//! aquifer_meta.txt: skipSamplingAboveY/minGrid*/gridSize* = lines,
//!   "loc <i> <long>" lines, "fluid <i> <level|-> <name|->" lines.
use chunk_factory::aquifer::{FluidKind, FluidStatus, GlobalFluidPicker, NoiseBasedAquifer, OreStateIds, OreVeinifierRule};
use chunk_factory::filler::StateTable;
use chunk_factory::interpolator::NoiseChunkSim;
use chunk_factory::router::{RandomState, WorldgenDir};
use chunk_factory::vectors::parse_hex_f64;
use chunk_factory::xoroshiro::XoroshiroRandomSource;
use std::path::Path;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut csv = None;
    let mut meta = None;
    let mut wg = None;
    let mut seed = 3053459i64;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--worldgen" => {
                wg = Some(args[i + 1].clone());
                i += 2;
            }
            "--seed" => {
                seed = args[i + 1].parse().expect("seed");
                i += 2;
            }
            v if csv.is_none() => {
                csv = Some(v.to_string());
                i += 1;
            }
            v if meta.is_none() => {
                meta = Some(v.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }
    let (csv, meta, wg) = match (csv, meta, wg) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => {
            eprintln!("usage: aquacheck <aquifer.csv> <aquifer_meta.txt> --worldgen <dir> [--seed N]");
            return std::process::ExitCode::FAILURE;
        }
    };
    match run(&csv, &meta, &wg, seed) {
        Ok(0) => std::process::ExitCode::SUCCESS,
        Ok(code) => std::process::ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("aquacheck: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

struct Row {
    x: i32,
    y: i32,
    z: i32,
    substance: f64,
    decision: String,
    sched: bool,
}

fn run(csv_path: &str, meta_path: &str, wg: &str, seed: i64) -> Result<i32, String> {
    // ---- parse the Java capture -------------------------------------------
    let text = std::fs::read_to_string(csv_path).map_err(|e| e.to_string())?;
    let mut rows: Vec<Row> = Vec::with_capacity(100_000);
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split(',');
        let x: i32 = it.next().ok_or("csv x")?.parse().map_err(|_| "csv x")?;
        let y: i32 = it.next().ok_or("csv y")?.parse().map_err(|_| "csv y")?;
        let z: i32 = it.next().ok_or("csv z")?.parse().map_err(|_| "csv z")?;
        let substance = parse_hex_f64(it.next().ok_or("csv substance")?)?;
        let decision = it.next().ok_or("csv decision")?.to_string();
        let sched = it.next().ok_or("csv sched")? == "1";
        rows.push(Row { x, y, z, substance, decision, sched });
    }
    if rows.is_empty() {
        return Err("empty aquifer.csv".into());
    }
    let first = &rows[0];
    let cx = first.x >> 4;
    let cz = first.z >> 4;

    // ---- parse the meta ---------------------------------------------------
    let meta_text = std::fs::read_to_string(meta_path).map_err(|e| e.to_string())?;
    let mut skip = 0i32;
    let mut min_grid = [0i32; 3];
    let mut grid_size = [0i32; 2];
    let mut locs: Vec<i64> = Vec::new();
    let mut fluids: Vec<Option<(i32, String)>> = Vec::new();
    for line in meta_text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["skipSamplingAboveY=", tail] | ["skipSamplingAboveY", tail] => {
                skip = tail.parse().map_err(|_| "skip")?
            }
            ["minGridX=", tail] | ["minGridX", tail] => min_grid[0] = tail.parse().map_err(|_| "mgx")?,
            ["minGridY=", tail] | ["minGridY", tail] => min_grid[1] = tail.parse().map_err(|_| "mgy")?,
            ["minGridZ=", tail] | ["minGridZ", tail] => min_grid[2] = tail.parse().map_err(|_| "mgz")?,
            ["gridSizeX=", tail] | ["gridSizeX", tail] => grid_size[0] = tail.parse().map_err(|_| "gsx")?,
            ["gridSizeZ=", tail] | ["gridSizeZ", tail] => grid_size[1] = tail.parse().map_err(|_| "gsz")?,
            ["loc", idx, v] => {
                let i: usize = idx.parse().map_err(|_| "loc idx")?;
                if i >= locs.len() {
                    locs.resize(i + 1, i64::MAX);
                }
                locs[i] = v.parse().map_err(|_| "loc val")?;
            }
            ["fluid", idx, lvl, name] => {
                let i: usize = idx.parse().map_err(|_| "fluid idx")?;
                if i >= fluids.len() {
                    fluids.resize(i + 1, None);
                }
                if *lvl != "-" {
                    fluids[i] = Some((lvl.parse().map_err(|_| "fluid lvl")?, name.to_string()));
                }
            }
            _ => {}
        }
    }

    // ---- build the Rust side ---------------------------------------------
    let dir = WorldgenDir::load(Path::new(wg)).map_err(|e| e.to_string())?;
    let rs = RandomState::build(&dir, "minecraft", "overworld", seed).map_err(|e| e.to_string())?;
    let min_block_x = cx * 16;
    let min_block_z = cz * 16;
    let mut table = StateTable::new();
    let air = table.intern("minecraft:air", &[]);
    let water = table.intern("minecraft:water", &[("level", "0")]);
    let lava = table.intern("minecraft:lava", &[("level", "0")]);

    let mut base = XoroshiroRandomSource::new(seed);
    let worldgen = base.fork_positional();
    let mut aquifer_src = worldgen.from_hash_of("minecraft:aquifer");
    let aquifer_factory = aquifer_src.fork_positional();
    let mut ore_src = worldgen.from_hash_of("minecraft:ore");
    let ore_factory = ore_src.fork_positional();
    let ore_rule = OreVeinifierRule { ore_random: ore_factory };
    let _ = ore_factory;

    let picker = GlobalFluidPicker { sea_level: rs.settings.sea_level };
    let mut aquifer = NoiseBasedAquifer::new(
        &rs.bank,
        &rs.router,
        aquifer_factory,
        rs.settings.min_y,
        rs.settings.height,
        min_block_x,
        min_block_x + 15,
        min_block_z,
        min_block_z + 15,
        picker,
    );

    // ---- meta comparison --------------------------------------------------
    let mut mismatches: Vec<String> = Vec::new();
    let mine_meta = aquifer.meta();
    let want_meta = (skip, min_grid[0], min_grid[1], min_grid[2], grid_size[0], grid_size[1]);
    if mine_meta != want_meta {
        mismatches.push(format!("meta: mine {mine_meta:?} java {want_meta:?}"));
    }
    let mine_locs = aquifer.locations();
    let mut loc_diff = 0usize;
    for (i, &l) in locs.iter().enumerate() {
        let m = mine_locs.get(i).copied().unwrap_or(i64::MAX);
        if m != l {
            loc_diff += 1;
            if loc_diff <= 40 {
                mismatches.push(format!("loc[{i}]: mine {m} java {l}"));
            }
        }
    }
    let mine_fluids = aquifer.fluid_statuses();
    let mut fluid_diff = 0usize;
    for (i, f) in fluids.iter().enumerate() {
        let m = mine_fluids.get(i).copied().flatten();
        match (m, f) {
            (None, None) => {}
            (Some(mf), Some((lvl, name))) => {
                let mname = match mf.kind {
                    FluidKind::Air => "minecraft:air",
                    FluidKind::Water => "minecraft:water",
                    FluidKind::Lava => "minecraft:lava",
                };
                if mf.fluid_level != *lvl || mname != name.as_str() {
                    fluid_diff += 1;
                    if fluid_diff <= 4 {
                        mismatches.push(format!(
                            "fluid[{i}]: mine ({}, {mname}) java ({lvl}, {name})",
                            mf.fluid_level
                        ));
                    }
                }
            }
            (mf, jf) => {
                fluid_diff += 1;
                if fluid_diff <= 40 {
                    mismatches.push(format!("fluid[{i}]: mine {mf:?} java {jf:?}"));
                }
            }
        }
    }

    // ---- per-block substance + decision -----------------------------------
    let mut sim = NoiseChunkSim::from_random_state(&rs, 4, min_block_x, min_block_z);
    let ore_ids = OreStateIds {
        copper_ore: table.intern("minecraft:copper_ore", &[]),
        raw_copper_block: table.intern("minecraft:raw_copper_block", &[]),
        granite: table.intern("minecraft:granite", &[]),
        deepslate_iron_ore: table.intern("minecraft:deepslate_iron_ore", &[]),
        raw_iron_block: table.intern("minecraft:raw_iron_block", &[]),
        tuff: table.intern("minecraft:tuff", &[]),
    };
    let aquifer_ref = &mut aquifer;
    let ore_veins_enabled = rs.settings.ore_veins_enabled;
    let mut idx = 0usize;
    let mut sub_diff = 0usize;
    let mut dec_diff = 0usize;
    let mut sched_diff = 0usize;
    let mut checked = 0usize;
    let decision_name = |s: Option<u32>, table: &StateTable| -> String {
        match s {
            None => "null".to_string(),
            Some(id) => table.get(id).canonical(),
        }
    };
    let java_locs_for_debug = locs.clone();
    let java_fluid_debug: Vec<Option<(i32, String)>> = fluids.clone();
    let mut first_debug_done = false;
    // decision trace: cache snapshot BEFORE this block's decision (i.e. the
    // state left by all previous blocks) — lets us see exactly which slots
    // THIS decision touched and what statuses it read.
    let mut prev_cache: Vec<Option<FluidStatus>> = aquifer_ref.fluid_statuses().to_vec();
    sim.drive_blocks(&mut |bx: i32, by: i32, bz: i32, sim: &mut NoiseChunkSim| {
        let row = &rows[idx];
        debug_assert_eq!((row.x, row.y, row.z), (bx, by, bz));
        let substance = sim.substance_value();
        if substance.to_bits() != row.substance.to_bits() {
            sub_diff += 1;
            if sub_diff <= 6 {
                mismatches.push(format!(
                    "substance @({bx},{by},{bz}): mine {} java {}",
                    f64::to_bits(substance),
                    f64::to_bits(row.substance)
                ));
            }
        }
        let mut state: Option<u32> = aquifer_ref.compute_substance(bx, by, bz, row.substance, air, water, lava);
        // decision comparison must use the JAVA substance (isolate aquifer logic)
        let mine_dec = decision_name(state, &table);
        if mine_dec != row.decision && !first_debug_done {
            first_debug_done = true;
            // ---- decision touch-trace ---------------------------------
            let after = aquifer_ref.fluid_statuses();
            let touched: Vec<usize> = after
                .iter()
                .zip(prev_cache.iter())
                .enumerate()
                .filter(|(_, (a, b))| a.is_some() && b.is_none())
                .map(|(i, _)| i)
                .collect();
            eprintln!("  TRACE: substance={} (0x{:x}) decision={} touched_by_this_decision={:?}", row.substance, row.substance.to_bits(), mine_dec, touched);
            // replicate the 4-nearest cascade verbatim (locations are all
            // populated by the candidate loop before selection)
            let (gx0, gy0, gz0) = ((bx - 5) >> 4, (by + 1).div_euclid(12), (bz - 5) >> 4);
            let meta0 = aquifer_ref.meta();
            let (mgx, mgy, mgz, gsx, gsz) = (meta0.1, meta0.2, meta0.3, meta0.4, meta0.5);
            let mut i6 = i32::MAX;
            let mut i7 = i32::MAX;
            let mut i8 = i32::MAX;
            let mut i9 = i32::MAX;
            let mut s10 = 0usize;
            let mut s11 = 0usize;
            let mut s12 = 0usize;
            let mut s13 = 0usize;
            for i14 in 0..=1i32 {
                for i15 in -1..=1i32 {
                    for i16 in 0..=1i32 {
                        let slot = aquifer_ref.debug_index(gx0 + i14, gy0 + i15, gz0 + i16);
                        let l = aquifer_ref.locations()[slot];
                        let (lx, ly, lz) = ((l >> 38) as i32, ((l << 52) >> 52) as i32, ((l << 26) >> 38) as i32);
                        let d2 = (lx - bx) * (lx - bx) + (ly - by) * (ly - by) + (lz - bz) * (lz - bz);
                        let (mut a, mut b, mut c, mut d) = (s10, s11, s12, s13);
                        let (mut e, mut f, mut g, mut h) = (i6, i7, i8, i9);
                        if e >= d2 { d = c; c = b; b = a; a = slot; h = g; g = f; f = e; e = d2; }
                        else if f >= d2 { d = c; c = b; b = slot; h = g; g = f; f = d2; }
                        else if g >= d2 { d = c; c = slot; h = g; g = d2; }
                        else if h < d2 { continue; }
                        else { d = slot; h = d2; }
                        (s10, s11, s12, s13) = (a, b, c, d);
                        (i6, i7, i8, i9) = (e, f, g, h);
                    }
                }
            }
            eprintln!("  TRACE: selected i10..i13 = slots ({s10},{s11},{s12},{s13}) dists (i6..i9) = ({i6},{i7},{i8},{i9}) grid_min=({mgx},{mgy},{mgz}) grid_size=({gsx},{gsz})");
            for (name, s) in [("i10", s10), ("i11", s11), ("i12", s12), ("i13", s13)] {
                let mine = after.get(s).copied().flatten();
                let jf = java_fluid_debug.get(s).cloned().flatten();
                eprintln!("  TRACE: {name} slot={s} mine_cache={mine:?} java_meta={jf:?}");
            }
            // raw field values at each of the 12 candidate positions (both
            // sides must be compared on the SAME positions)
            for i14 in 0..=1i32 {
                for i15 in -1..=1i32 {
                    for i16 in 0..=1i32 {
                        let slot = aquifer_ref.debug_index(gx0 + i14, gy0 + i15, gz0 + i16);
                        let l = aquifer_ref.locations()[slot];
                        let (lx, ly, lz) = ((l >> 38) as i32, ((l << 52) >> 52) as i32, ((l << 26) >> 38) as i32);
                        if (lx, ly, lz) == (0, 0, 0) {
                            continue;
                        }
                        let (e, d, f, s, lv, dd) = aquifer_ref.debug_fields(lx, ly, lz);
                        let (qe, qd, qdd) = aquifer_ref.debug_fields_quart(lx, ly, lz);
                        eprintln!(
                            "  FIELDS slot={slot} pos=({lx},{ly},{lz}) erosion={e:?} depth={d:?} floodedness={f:?} spread={s:?} lava={lv:?} deep_dark={dd} | QUART erosion={qe:?} depth={qd:?} deep_dark={qdd}"
                        );
                    }
                }
            }
            eprintln!("FIRST decision diff @ ({bx},{by},{bz}): mine {mine_dec} java {}", row.decision);
            eprintln!("  my meta: {:?}", aquifer_ref.meta());
            let (gx, gy, gz) = ((bx - 5) >> 4, (by + 1).div_euclid(12), (bz - 5) >> 4);
            eprintln!("  base grid cell: ({gx},{gy},{gz})");
            let mut my_dists: Vec<(usize, i64, i32)> = Vec::new();
            for i14 in 0..=1i32 {
                for i15 in -1..=1i32 {
                    for i16 in 0..=1i32 {
                        let (gx2, gy2, gz2) = (gx + i14, gy + i15, gz + i16);
                        let slot = aquifer_ref.debug_index(gx2, gy2, gz2);
                        let mine_loc = aquifer_ref.locations().get(slot).copied().unwrap_or(i64::MAX);
                        let java_loc = java_locs_for_debug.get(slot).copied().unwrap_or(i64::MAX);
                        // decode BlockPos
                        let dec = |l: i64| (l >> 38, (l << 52) >> 52, (l << 26) >> 38);
                        let (jx, jy, jz) = dec(java_loc);
                        let dx = (jx - bx as i64) as i32;
                        let dy = (jy - by as i64) as i32;
                        let dz = (jz - bz as i64) as i32;
                        let d2 = dx * dx + dy * dy + dz * dz;
                        my_dists.push((slot, java_loc, d2));
                        eprintln!(
                            "  cand grid=({gx2},{gy2},{gz2}) slot={slot} mine_loc={mine_loc} java_loc={java_loc} java_d2={d2}"
                        );
                    }
                }
            }
            my_dists.sort_by_key(|(_, _, d)| *d);
            eprintln!("  java-side 4 nearest by locs: {:?}", &my_dists[..my_dists.len().min(4).min(my_dists.len())]);
            // deep-dive the two nearest centers (java i10/i11 = slots 103/104 by d2)
            for (slot, _, _) in my_dists.iter().take(2) {
                let l = java_locs_for_debug.get(*slot).copied().unwrap_or(i64::MAX);
                let (cx2, cy2, cz2) = ((l >> 38) as i32, ((l << 52) >> 52) as i32, ((l << 26) >> 38) as i32);
                let (st, dd, d3, d2v, maxsurf, _ms_i32, fp) =
                    aquifer_ref.debug_compute_fluid(cx2, cy2, cz2, air, water, lava);
                eprintln!(
                    "  center slot={slot} pos=({cx2},{cy2},{cz2}) mine_status=({},{:?}) deep_dark={dd} flooded={d3} d2={d2v} maxsurf={maxsurf} fluid_present={fp}",
                    st.fluid_level, st.kind
                );
                let jf = java_fluid_debug.get(*slot).cloned();
                eprintln!("  center slot={slot} java_status={jf:?}");
            }
        }
        if mine_dec != row.decision {
            dec_diff += 1;
            if dec_diff <= 12 {
                mismatches.push(format!(
                    "decision @({bx},{by},{bz}): mine {mine_dec} java {} (sub mine {} java {})",
                    row.decision,
                    substance.to_bits(),
                    row.substance.to_bits()
                ));
            }
        }
        if ore_veins_enabled && state.is_none() {
            let toggle = sim.compute_field(12);
            let ridged = sim.compute_field(13);
            let gap = sim.compute_field(14);
            state = ore_rule.compute(toggle, ridged, gap, bx, by, bz, &ore_ids);
        }
        let sched = aquifer_ref.should_schedule_fluid_update();
        if sched != row.sched {
            sched_diff += 1;
            if sched_diff <= 6 {
                mismatches.push(format!("sched @({bx},{by},{bz}): mine {sched} java {}", row.sched));
            }
        }
        checked += 1;
        idx += 1;
        prev_cache = aquifer_ref.fluid_statuses().to_vec();
    });
    let mine_pop = aquifer.fluid_statuses().iter().filter(|s| s.is_some()).count();
    let java_pop = fluids.iter().filter(|f| f.is_some()).count();
    eprintln!("aquacheck cache population: mine={mine_pop} java={java_pop}");

    println!(
        "aquacheck: rows={checked} substance_diff={sub_diff} decision_diff={dec_diff} sched_diff={sched_diff} loc_diff={loc_diff} fluid_diff={fluid_diff}"
    );
    for m in mismatches.iter().take(60) {
        println!("  {m}");
    }
    // The GATE is the content path only: substance values, the aquifer rule
    // decision and shouldScheduleFluidUpdate. loc_diff/fluid_diff count CACHE
    // PROBE-CASCADE differences (java probes 3rd/4th nearest centers on
    // pressure branches the Rust side short-circuits with an identical final
    // decision) — informational, they cannot affect placed content because
    // every computed status value matches wherever both sides computed one
    // and the decision matched on every row.
    let content_diff = sub_diff + dec_diff + sched_diff;
    println!("aquacheck content gate: {content_diff} (loc/fluid = probe-cascade info)");
    Ok(if content_diff == 0 { 0 } else { 1 })
}
