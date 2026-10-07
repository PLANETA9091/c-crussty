//! NCF P5.3 pool-alias increment — PoolAliasLookup / PoolAliasBinding port
//! (net/minecraft/world/level/levelgen/structure/pools/alias, CFR decomp).
//!
//! Java semantics (JigsawStructure.findGenerationPoint CFR 95-99):
//!   PoolAliasLookup.create(this.poolAliases, blockPos, context.seed())
//! where blockPos = (chunkMinBlockX, startHeightY, chunkMinBlockZ) and
//! context.seed() = the LEVEL seed. create() (PoolAliasLookup.java):
//!   RandomSource randomSource = RandomSource.create(seed)          // LEGACY
//!       .forkPositional()                                          // LegacyPositionalRandomFactory(nextLong()) — CONSUMES one nextLong
//!       .at(pos);                                                  // LegacyRandomSource(Mth.getSeed(x,y,z) ^ factorySeed)
//!   aliases.forEach(binding -> binding.forEachResolved(rng, map::put));
//! The lookup map defaults to identity (map.getOrDefault(key, key)).
//!
//! Binding resolution (forEachResolved), bindings in LIST ORDER, ONE shared
//! stream:
//! * DirectPoolAlias: put(alias, target) — NO RNG.
//! * RandomPoolAlias: WeightedList.getRandom(rng):
//!   Optional.empty (NO RNG) when the list is EMPTY; otherwise
//!   rng.nextInt(totalWeight) + subtract-walk (WeightedList.Compact.get =
//!   first entry where cumulative weight > index; identical result to the
//!   Flat expansion), then put(alias, picked).
//! * RandomGroupPoolAlias: one weighted pick among GROUPS (same
//!   WeightedList.getRandom), then the chosen group's bindings resolve
//!   IN ORDER on the SAME stream.
//!
//! The alias chain is an INDEPENDENT random lineage: it starts from
//! RandomSource.create(levelSeed) — it never touches the assembly RNG
//! (Legacy(0)+setLargeFeatureSeed) nor the pick RNG.

use crate::jrandom::{LegacyRandomSource, RandomSource};
use crate::json::Json;
use std::collections::HashMap;

/// One pool_alias binding (PoolAliasBinding CFR 21-42).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolAliasBinding {
    /// DirectPoolAlias: alias -> target always.
    Direct { alias: String, target: String },
    /// RandomPoolAlias: alias -> weighted pick; (pool key, weight) in JSON order.
    Random { alias: String, targets: Vec<(String, i32)> },
    /// RandomGroupPoolAlias: weighted pick among groups; the chosen group's
    /// bindings resolve in order on the same stream.
    RandomGroup { groups: Vec<(Vec<PoolAliasBinding>, i32)> },
}

/// WeightedList.getRandom (CFR 80-86): empty list => None WITHOUT consuming
/// RNG; otherwise nextInt(totalWeight) + first entry where cumulative weight
/// > index (Compact.get; equal to the Flat entries[index] expansion).
fn weighted_pick<T: Clone>(rng: &mut dyn RandomSource, items: &[(T, i32)]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    let total: i32 = items.iter().map(|(_, w)| *w).sum();
    let mut idx = rng.next_int_bound(total);
    for (v, w) in items {
        idx -= *w;
        if idx < 0 {
            return Some(v.clone());
        }
    }
    // Unreachable for positive weights; matches Java's IllegalStateException arm.
    None
}

impl PoolAliasBinding {
    /// PoolAliasBinding CODEC dispatch on "type": minecraft:direct /
    /// minecraft:random / minecraft:random_group. Weighted lists use the
    /// {"weight": w, "data": v} record.
    pub fn parse_json(v: &Json) -> Result<PoolAliasBinding, String> {
        let ty = v
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("");
        match ty {
            "minecraft:direct" => {
                let alias = req_str(v, "alias")?;
                let target = req_str(v, "target")?;
                Ok(PoolAliasBinding::Direct { alias, target })
            }
            "minecraft:random" => {
                let alias = req_str(v, "alias")?;
                let arr = v
                    .get("targets")
                    .and_then(|t| t.as_arr())
                    .ok_or("random alias: targets[] missing")?;
                let mut targets = Vec::with_capacity(arr.len());
                for e in arr {
                    let key = req_str(e, "data")?;
                    let w = e.get("weight").and_then(|w| w.as_i64()).unwrap_or(1) as i32;
                    targets.push((key, w));
                }
                Ok(PoolAliasBinding::Random { alias, targets })
            }
            "minecraft:random_group" => {
                let arr = v
                    .get("groups")
                    .and_then(|g| g.as_arr())
                    .ok_or("random_group: groups[] missing")?;
                let mut groups = Vec::with_capacity(arr.len());
                for e in arr {
                    let w = e.get("weight").and_then(|w| w.as_i64()).unwrap_or(1) as i32;
                    let list = e
                        .get("data")
                        .and_then(|d| d.as_arr())
                        .ok_or("random_group: group data[] missing")?;
                    let mut bindings = Vec::with_capacity(list.len());
                    for b in list {
                        bindings.push(PoolAliasBinding::parse_json(b)?);
                    }
                    groups.push((bindings, w));
                }
                Ok(PoolAliasBinding::RandomGroup { groups })
            }
            other => Err(format!("pool_aliases: unsupported type {other:?}")),
        }
    }

    /// PoolAliasBinding.forEachResolved — appends to the lookup map in
    /// resolution order. Java's ImmutableMap.Builder.put CRASHES on duplicate
    /// alias keys; here the first put wins and the duplicate is reported
    /// (crash paths are fallback-owned per the NCF convention).
    fn resolve(
        &self,
        rng: &mut dyn RandomSource,
        map: &mut HashMap<String, String>,
        dup: &mut Vec<String>,
    ) {
        match self {
            PoolAliasBinding::Direct { alias, target } => put(map, dup, alias, target),
            PoolAliasBinding::Random { alias, targets } => {
                if let Some(t) = weighted_pick(rng, targets) {
                    put(map, dup, alias, &t);
                }
            }
            PoolAliasBinding::RandomGroup { groups } => {
                if let Some(bindings) = weighted_pick(rng, groups) {
                    for b in bindings {
                        b.resolve(rng, map, dup);
                    }
                }
            }
        }
    }
}

fn req_str(v: &Json, key: &str) -> Result<String, String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("pool_aliases: field {key:?} missing"))
}

fn put(map: &mut HashMap<String, String>, dup: &mut Vec<String>, alias: &str, target: &str) {
    if map.contains_key(alias) {
        dup.push(alias.to_string());
    }
    map.insert(alias.to_string(), target.to_string());
}

/// PoolAliasLookup.create (CFR 13-27): the bit-exact lineage.
/// Empty bindings => EMPTY lookup (no RNG, identity map).
pub fn build_lookup(
    bindings: &[PoolAliasBinding],
    pos: (i32, i32, i32),
    level_seed: i64,
) -> (HashMap<String, String>, Vec<String>) {
    let mut map = HashMap::new();
    let mut dup = Vec::new();
    if bindings.is_empty() {
        return (map, dup);
    }
    let mut rng = LegacyRandomSource::new(level_seed);
    let factory = rng.fork_positional_factory();
    // at(pos) = LegacyRandomSource(Mth.getSeed(x,y,z) ^ factorySeed).
    let mut alias_rng = factory.at(pos.0, pos.1, pos.2);
    for b in bindings {
        b.resolve(&mut *alias_rng, &mut map, &mut dup);
    }
    (map, dup)
}

/// PoolAliasLookup.lookup: map.getOrDefault(key, key).
pub fn lookup<'m>(map: &'m HashMap<String, String>, key: &'m str) -> &'m str {
    map.get(key).map(|s| s.as_str()).unwrap_or(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jrandom::LegacyRandomSource;

    fn parse(s: &str) -> PoolAliasBinding {
        PoolAliasBinding::parse_json(&crate::json::parse(s).unwrap()).unwrap()
    }

    #[test]
    fn parse_trial_chambers_aliases() {
        // The only vanilla pool_aliases user (1.21.10): one random_group +
        // two random bindings; data from data/minecraft/worldgen/structure/
        // trial_chambers.json.
        let Some(root) = crate::test_support::extract_root() else {
            return; // loud skip already printed by test_support
        };
        let text = std::fs::read_to_string(
            root.join("data/minecraft/worldgen/structure/trial_chambers.json"),
        )
        .expect("trial_chambers.json");
        let j = crate::json::parse(&text).unwrap();
        let arr = j.get("pool_aliases").and_then(|a| a.as_arr()).unwrap();
        assert_eq!(arr.len(), 3);
        let b0 = PoolAliasBinding::parse_json(&arr[0]).unwrap();
        let b1 = PoolAliasBinding::parse_json(&arr[1]).unwrap();
        match &b0 {
            PoolAliasBinding::RandomGroup { groups } => {
                assert!(!groups.is_empty());
                // each group is a Direct/Random list
                assert!(groups.iter().all(|(g, _)| !g.is_empty()));
            }
            other => panic!("first alias should be random_group, got {other:?}"),
        }
        match &b1 {
            PoolAliasBinding::Random { alias, targets } => {
                assert_eq!(alias, "minecraft:trial_chambers/spawner/contents/melee");
                assert!(targets.iter().all(|(_, w)| *w > 0));
            }
            other => panic!("second alias should be random, got {other:?}"),
        }
    }

    #[test]
    fn empty_bindings_identity_lookup() {
        // PoolAliasLookup.create with an empty list => EMPTY (no RNG).
        let (map, dup) = build_lookup(&[], (10, -20, -30), 3053459);
        assert!(map.is_empty() && dup.is_empty());
        assert_eq!(lookup(&map, "minecraft:any"), "minecraft:any");
    }

    #[test]
    fn direct_binding_maps_and_defaults() {
        let b = parse(r#"{"type":"minecraft:direct","alias":"minecraft:a","target":"minecraft:b"}"#);
        let (map, dup) = build_lookup(&[b], (0, 0, 0), 42);
        assert!(dup.is_empty());
        assert_eq!(lookup(&map, "minecraft:a"), "minecraft:b");
        // getOrDefault: unmapped keys pass through.
        assert_eq!(lookup(&map, "minecraft:x"), "minecraft:x");
    }

    #[test]
    fn random_binding_consumes_exactly_one_draw() {
        let b = parse(
            r#"{"type":"minecraft:random","alias":"minecraft:r","targets":[
                {"data":"minecraft:p1","weight":1},{"data":"minecraft:p2","weight":3}]}"#,
        );
        // Independent replay: the alias lineage = Legacy(levelSeed) -> fork
        // (1 next_long) -> at(pos) = Legacy(getSeed ^ factorySeed).
        let mut rng = LegacyRandomSource::new(42);
        let factory_seed = rng.next_long();
        let mut replay = LegacyRandomSource::new(crate::mth::get_seed(100, 5, -200) ^ factory_seed);
        let expect = replay.next_int_bound(4); // totalWeight = 1 + 3
        let picked = ["minecraft:p1", "minecraft:p2"][if expect < 1 { 0 } else { 1 }];
        let (map, _) = build_lookup(&[b], (100, 5, -200), 42);
        assert_eq!(lookup(&map, "minecraft:r"), picked);
    }

    #[test]
    fn random_group_shares_one_stream() {
        // Two random bindings wrapped in a single-group random_group: the
        // group pick consumes ONE nextInt, then both children resolve in
        // order on the same stream — the second child's pick must equal a
        // hand-replayed stream.
        let group = vec![
            parse(r#"{"type":"minecraft:random","alias":"minecraft:r1","targets":[{"data":"minecraft:x1","weight":1},{"data":"minecraft:x2","weight":1}]}"#),
            parse(r#"{"type":"minecraft:random","alias":"minecraft:r2","targets":[{"data":"minecraft:y1","weight":1},{"data":"minecraft:y2","weight":1}]}"#),
        ];
        let rg = PoolAliasBinding::RandomGroup { groups: vec![(group, 1)] };
        let mut rng = LegacyRandomSource::new(7);
        let factory_seed = rng.next_long();
        let mut replay = LegacyRandomSource::new(crate::mth::get_seed(3, 4, 5) ^ factory_seed);
        let g = replay.next_int_bound(1); // group pick (single group, still consumes)
        assert_eq!(g, 0);
        let a = replay.next_int_bound(2);
        let b2 = replay.next_int_bound(2);
        let exp1 = ["minecraft:x1", "minecraft:x2"][a as usize];
        let exp2 = ["minecraft:y1", "minecraft:y2"][b2 as usize];
        let (map, _) = build_lookup(&[rg], (3, 4, 5), 7);
        assert_eq!(lookup(&map, "minecraft:r1"), exp1);
        assert_eq!(lookup(&map, "minecraft:r2"), exp2);
    }
}
