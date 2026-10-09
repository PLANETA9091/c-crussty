//! NCF P4 probe (WORK LIST п.4 increment) — full-corpus feature IR census.
//!
//! Loads the jar-fresh worldgen extract, parses EVERY configured_feature and
//! placed_feature JSON with the features.rs IR, and reports the honest
//! support census the P4.1-P4.3 dispatch needs:
//!   - configured features by type: supported (tier-1) vs Unsupported
//!   - placed features fully supported (modifiers + body + inner nesting)
//!   - per OVERWORLD biome: how many of its 11-step placed-feature lists are
//!     fully supported — the per-chunk native-eligibility picture
//!
//! usage: p4probe <worldgen-dir>

use chunk_factory::biomes::load_biome_facts;
use chunk_factory::features::{FeatureDef, FeatureRegistry, PlacementMod};
use chunk_factory::router::WorldgenDir;
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: p4probe <worldgen-dir>");
        std::process::exit(2);
    }
    let dir = WorldgenDir::load(Path::new(&args[1])).expect("worldgen dir");
    // full keys: the registry parses namespaced refs
    let mut configured_keys = Vec::new();
    let mut placed_keys = Vec::new();
    for ns in dir.namespaces() {
        for name in dir.list(&ns, "configured_feature") {
            configured_keys.push(format!("{ns}:{name}"));
        }
        for name in dir.list(&ns, "placed_feature") {
            placed_keys.push(format!("{ns}:{name}"));
        }
    }
    let registry =
        FeatureRegistry::load_keys(&dir, &placed_keys, &configured_keys).expect("registry");

    // configured census by type
    let mut by_type: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut supported_configured = 0usize;
    for key in &configured_keys {
        let def = registry.configured.get(key.as_str());
        let (ty, ok) = match def {
            Some(FeatureDef::Unsupported(t)) => (t.clone(), false),
            Some(_) => ("<supported>".to_string(), true),
            None => ("<missing>".to_string(), false),
        };
        let e = by_type.entry(ty).or_insert((0, 0));
        e.1 += 1;
        if ok {
            e.0 += 1;
            supported_configured += 1;
        }
    }
    println!("configured: {}/{} fully supported", supported_configured, configured_keys.len());
    for (ty, (ok, total)) in &by_type {
        if ty == "<supported>" {
            continue;
        }
        println!("  unsupported type {ty:<24} {ok}/{total}");
    }

    // placed census
    let mut supported_placed = 0usize;
    let mut unsupported_mods: BTreeMap<String, usize> = BTreeMap::new();
    for key in &placed_keys {
        let Some(def) = registry.placed.get(key.as_str()) else {
            *unsupported_mods.entry("<missing>".into()).or_default() += 1;
            continue;
        };
        if is_supported(def, &registry) {
            supported_placed += 1;
        } else {
            for m in &def.placement {
                if let PlacementMod::Unsupported(t) = m {
                    *unsupported_mods.entry(t.clone()).or_default() += 1;
                }
            }
        }
    }
    println!("placed: {}/{} fully supported", supported_placed, placed_keys.len());
    for (t, n) in &unsupported_mods {
        println!("  blocker {t:<28} {n}");
    }

    // per-biome eligibility census
    let facts = load_biome_facts(&dir).expect("biome facts");
    let mut eligible_biomes = Vec::new();
    let mut partial_biomes = Vec::new();
    for (name, f) in &facts {
        let mut total = 0usize;
        let mut ok = 0usize;
        for step in &f.features {
            for pf in step {
                total += 1;
                if let Some(def) = registry.placed.get(pf.as_str()) {
                    if is_supported(def, &registry) {
                        ok += 1;
                    }
                }
            }
        }
        if total == ok && total > 0 {
            eligible_biomes.push(name.clone());
        } else {
            partial_biomes.push((name.clone(), ok, total));
        }
    }
    println!(
        "biomes: {} ALL-features-supported, {} with unsupported set (of {})",
        eligible_biomes.len(),
        partial_biomes.len(),
        facts.len()
    );
    if !eligible_biomes.is_empty() {
        for b in &eligible_biomes {
            println!("  fully-supported biome: {b}");
        }
    }

    // draw-stack sanity via the DecorationRandom (compile-time wiring check)
    use chunk_factory::feature_sorter::WorldgenDraws;
    let mut dr = chunk_factory::feature_sorter::DecorationRandom::new(3053459);
    let l = dr.set_decoration_seed(3053459, 1600, 1600);
    dr.set_feature_seed(l, 3, 6);
    let d = dr.next_int_bound_wg(16);
    println!("decor-seed probe: population={l} first_draw={d}");
}

fn is_supported(
    def: &chunk_factory::features::PlacedFeatureDef,
    registry: &FeatureRegistry,
) -> bool {
    chunk_factory::features::is_placed_supported(def, registry)
}
