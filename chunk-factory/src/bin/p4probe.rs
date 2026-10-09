//! NCF P4 probe (WORK LIST п.4 increment) — full-corpus feature IR census.
//!
//! Loads the jar-fresh worldgen extract, parses EVERY configured_feature and
//! placed_feature JSON with the features.rs IR, and reports the honest
//! support census the P4.1-P4.3 dispatch needs:
//!   - configured features by type: supported (tier-1) vs Unsupported
//!   - placed features fully supported (modifiers + body + inner nesting)
//!   - per OVERWORLD biome: how many of its 11-step placed-feature lists are
//!     fully supported — the per-chunk native-eligibility picture
//!   - universal-blocker histogram (inc. 2): for every unsupported label
//!     (modifier / configured type / parse-fail type) the number of biomes
//!     whose step lists contain at least one such feature — the widest
//!     blocker is the next port with the biggest coverage payoff
//!
//! usage: p4probe <worldgen-dir>   (NCF_P4PROBE_LIST=1 dumps per-key verdicts)

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
    // per-key list for census diffs (env-gated: NCF_P4PROBE_LIST=1)
    if std::env::var("NCF_P4PROBE_LIST").is_ok() {
        for key in &placed_keys {
            let sup = registry
                .placed
                .get(key.as_str())
                .map(|d| is_supported(d, &registry))
                .unwrap_or(false);
            println!("PLACED {} {}", if sup { "OK" } else { "--" }, key);
        }
    }

    // per-biome eligibility census + universal-blocker histogram.
    // A "universal blocker" is an unsupported configured TYPE (or unsupported
    // modifier) that blocks biomes across the board — the next type to port
    // is the one with the widest biome reach. For each biome we collect the
    // set of blocker labels over its 11-step placed lists:
    //   - unsupported modifier type (PlacementMod::Unsupported)
    //   - unsupported configured body type (FeatureDef::Unsupported)
    //   - for parse-failed placed features, the JSON "type" of the inner
    //     configured feature (re-read from the worldgen dir)
    let facts = load_biome_facts(&dir).expect("biome facts");
    let mut eligible_biomes = Vec::new();
    let mut type_blocks_biomes: BTreeMap<String, usize> = BTreeMap::new();
    for (name, f) in &facts {
        let mut total = 0usize;
        let mut ok = 0usize;
        let mut blocked_by: Vec<String> = Vec::new();
        for step in &f.features {
            for pf in step {
                total += 1;
                match registry.placed.get(pf.as_str()) {
                    Some(def) => {
                        if is_supported(def, &registry) {
                            ok += 1;
                        } else if let Some(t) = blocker_label(def, &registry) {
                            if !blocked_by.contains(&t) {
                                blocked_by.push(t);
                            }
                        }
                    }
                    None => {
                        if let Some(t) = placed_parse_type(&dir, pf) {
                            if !blocked_by.contains(&t) {
                                blocked_by.push(t);
                            }
                        }
                    }
                }
            }
        }
        if total == ok && total > 0 {
            eligible_biomes.push(name.clone());
        }
        for t in blocked_by {
            *type_blocks_biomes.entry(t).or_default() += 1;
        }
    }
    println!(
        "biomes: {} ALL-features-supported, {} with unsupported set (of {})",
        eligible_biomes.len(),
        facts.len() - eligible_biomes.len(),
        facts.len()
    );
    if !eligible_biomes.is_empty() {
        for b in &eligible_biomes {
            println!("  fully-supported biome: {b}");
        }
    }
    let total_biomes = facts.len();
    let mut hist: Vec<(&String, &usize)> = type_blocks_biomes.iter().collect();
    hist.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("universal-blocker histogram (type: #biomes blocked of {total_biomes}):");
    for (t, n) in hist {
        println!("  {t:<32} blocks {n}/{total_biomes} biomes");
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

/// Blocker label for an unsupported placed feature: the FIRST unsupported
/// modifier type if any, else the unsupported configured body type
/// ("unsupported type X" form, matching the configured census labels).
fn blocker_label(
    def: &chunk_factory::features::PlacedFeatureDef,
    registry: &FeatureRegistry,
) -> Option<String> {
    use chunk_factory::features::{FeatureDef, PlacementMod};
    for m in &def.placement {
        if let PlacementMod::Unsupported(t) = m {
            return Some(t.clone());
        }
    }
    if let Some((short, _)) = &def.inline {
        return Some(short.clone());
    }
    match registry.configured.get(&def.feature_ref) {
        Some(FeatureDef::Unsupported(t)) => Some(format!("unsupported type {t}")),
        _ => None,
    }
}

/// Type label of a placed feature whose IR parse failed: re-read the JSON
/// from the worldgen dir and pull "feature" (ref or inline object type).
fn placed_parse_type(
    dir: &chunk_factory::router::WorldgenDir,
    pf: &str,
) -> Option<String> {
    let (ns, name) = pf.split_once(':')?;
    let text = dir.get(ns, "placed_feature", name)?;
    let j = chunk_factory::json::parse(text).ok()?;
    let inner = j.get("feature")?;
    match inner.as_str() {
        Some(ref_str) => {
            let (rns, rname) = ref_str.split_once(':').unwrap_or(("minecraft", ref_str));
            let cfg = dir.get(rns, "configured_feature", rname)?;
            let cj = chunk_factory::json::parse(cfg).ok()?;
            let ty = cj.get("type")?.as_str()?.to_string();
            let short = ty.strip_prefix("minecraft:").unwrap_or(&ty).to_string();
            Some(format!("unsupported type {short} (parse fail)"))
        }
        None => {
            let ty = inner.get("type")?.as_str()?.to_string();
            Some(ty.strip_prefix("minecraft:").unwrap_or(&ty).to_string())
        }
    }
}
