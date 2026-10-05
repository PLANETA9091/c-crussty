//! ircheck — Phase 1 parse/wire coverage gate (NCF P1.1-P1.6).
//!
//! Walks an extracted datapack `data/` tree and, for EVERY noise_settings
//! file in the requested namespace(s):
//!   1. parses the full noise_router (all density_function registry refs
//!      resolved recursively) — a parse failure is a RED gate;
//!   2. wires the router with seed 0 (RandomState.NoiseWiringHelper
//!      equivalent: noise instances, blended noise, bounds) — a wiring
//!      failure is a RED gate (honest unsupported-lane report);
//!   3. prints the canonical world-spec hash (P1.7) per file.
//!
//! Also prints inventory counts per worldgen kind so CI shows the coverage
//! surface. Exit 0 <=> all files parse AND wire.
//!
//! Usage: cargo run --bin ircheck -- <data-dir> [--ns minecraft] [--seed 0]

use chunk_factory::json;
use chunk_factory::router::{RandomState, WorldgenDir};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: ircheck <data-dir> [--ns minecraft] [--seed 0]");
        std::process::exit(2);
    }
    let root = std::path::PathBuf::from(&args[1]);
    let mut ns_filter = String::from("minecraft");
    let mut seed: i64 = 0;
    let mut it = args.iter().skip(2);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--ns" => ns_filter = it.next().expect("--ns value").clone(),
            "--seed" => seed = it.next().map(|s| s.parse().expect("seed")).expect("--seed value"),
            other => {
                eprintln!("unknown arg {other}");
                std::process::exit(2);
            }
        }
    }

    let dir = match WorldgenDir::load(&root) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("FATAL: cannot load worldgen dir: {e}");
            std::process::exit(1);
        }
    };

    println!("worldgen inventory:");
    let mut any_files = false;
    for ns in dir.namespaces() {
        if !ns_filter.is_empty() && ns != ns_filter {
            continue;
        }
        any_files = true;
        for kind in [
            "noise_settings",
            "density_function",
            "noise",
            "biome",
            "configured_feature",
            "placed_feature",
            "structure",
            "structure_set",
        ] {
            let n = dir.count(&ns, kind);
            if n > 0 {
                println!("  {ns}/{kind}: {n}");
            }
        }
    }
    if !any_files {
        eprintln!("FATAL: no worldgen files for namespace '{ns_filter}' under {}", root.display());
        std::process::exit(1);
    }

    // Every noise_settings file must parse + wire.
    let settings_files: Vec<String> = dir
        .namespaces()
        .into_iter()
        .filter(|ns| ns_filter.is_empty() || ns == &ns_filter)
        .flat_map(|ns| {
            let files = dir.list(&ns, "noise_settings");
            files.into_iter().map(move |f| format!("{ns}:{f}"))
        })
        .collect();

    let mut failures = 0usize;
    let mut wired = 0usize;
    for spec in &settings_files {
        let (ns, name) = spec.split_once(':').unwrap();
        match RandomState::build(&dir, ns, name, seed) {
            Ok(rs) => {
                wired += 1;
                println!(
                    "OK   {ns}:{name}  noise_instances={} blended={} spec_hash={:016x}",
                    rs.bank.noises.len(),
                    rs.bank.blended.len(),
                    rs.spec_hash
                );
            }
            Err(e) => {
                failures += 1;
                eprintln!("FAIL {ns}:{name}: {e}");
            }
        }
    }

    // Every density_function registry file must parse standalone too (files
    // not reachable from any router still have to be understood by the IR).
    let mut df_fail = 0usize;
    let mut df_parsed = 0usize;
    for ns in dir.namespaces() {
        if !ns_filter.is_empty() && ns != ns_filter {
            continue;
        }
        for rel in dir.list(&ns, "density_function") {
            let text = match dir.read(&ns, "density_function", &rel) {
                Some(t) => t,
                None => continue,
            };
            match json::parse(&text) {
                Ok(j) => {
                    let mut visited = std::collections::HashSet::new();
                    match chunk_factory::router::parse_df_value(&j, &dir, &mut visited) {
                        Ok(_) => df_parsed += 1,
                        Err(e) => {
                            df_fail += 1;
                            eprintln!("FAIL df {ns}:{rel}: {e}");
                        }
                    }
                }
                Err(e) => {
                    df_fail += 1;
                    eprintln!("FAIL df json {ns}:{rel}: {e}");
                }
            }
        }
    }

    println!("noise_settings: {wired} wired OK, {failures} failures");
    println!("density_function files: {df_parsed} parsed OK, {df_fail} failures");
    if failures == 0 && df_fail == 0 {
        println!("VERDICT: IR GATE PASS");
    } else {
        println!("VERDICT: IR GATE FAIL");
        std::process::exit(1);
    }
}
