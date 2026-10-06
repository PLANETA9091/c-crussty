//! NCF P4.1 — FeatureSorter и порядок фич + decoration seeding chain.
//!
//! Port of the mapped Purpur 2535 sources (CFR 0.152, session 7):
//!   - net.minecraft.world.level.biome.FeatureSorter.buildFeaturesPerStep
//!     (the TreeMap(successor sets) + Graph.depthFirstSearch + reverse
//!     topological order, feature indices via computeIfAbsent MutableInt),
//!   - ChunkGenerator.addVanillaDecorations seeding:
//!       WorldgenRandom over XoroshiroRandomSource(generateUniqueSeed())
//!       long populationSeed = setDecorationSeed(levelSeed, minBlockX, minBlockZ)
//!       setFeatureSeed(populationSeed, index, step) = seed + index + 10000*step.
//! RNG semantics: WorldgenRandom.next(bits) on a non-Legacy inner source =
//! (int)(inner.nextLong() >>> (64-bits)); nextLong() = Legacy form
//! (next(32)<<32) + next(32) — two inner nextLong draws. Verified against
//! the decompile (WorldgenRandom.java, lines 26-50).

use crate::jrandom::RandomSource;
use crate::xoroshiro::XoroshiroRandomSource;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// DecorationStep of the world: per-biome feature holder sets are provided
/// by the datapack IR as PLACED-FEATURE id lists (interned by the caller).
pub type StepFeatureLists = Vec<Vec<Vec<usize>>>; // [source(biome)][step][feature ids]

/// buildFeaturesPerStep — returns per-step lists of placed-feature ids in
/// the vanilla topological decoration order. Sources iterate in the caller's
/// biome order (FeatureSorter receives the biome set; our IR order matches
/// the datapack parse order — the corpus pins it).
pub fn build_features_per_step(sources: &StepFeatureLists) -> Vec<Vec<usize>> {
    // FeatureData(step, featureIndex) with featureIndex = global insertion id
    let mut feature_index: HashMap<usize, usize> = HashMap::new();
    let mut next_index = 0usize;
    let mut max_steps = 0usize;
    for src in sources {
        max_steps = max_steps.max(src.len());
    }
    // TreeMap<(step, featureIndex), Set<(step, featureIndex) successors>>
    let mut graph: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> = BTreeMap::new();
    for src in sources {
        let mut list: Vec<(usize, usize)> = Vec::new(); // (step, featureIndex)
        for (i1, holder_set) in src.iter().enumerate() {
            for &pf in holder_set {
                let idx = *feature_index.entry(pf).or_insert_with(|| {
                    let v = next_index;
                    next_index += 1;
                    v
                });
                list.push((i1, idx));
            }
        }
        for i in 0..list.len() {
            let entry = graph.entry(list[i]).or_default();
            if i + 1 < list.len() {
                entry.insert(list[i + 1]);
            }
        }
    }
    // Graph.depthFirstSearch over the ordered key set, list::add on BLACK.
    let mut seen_white: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut in_progress: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut order: Vec<(usize, usize)> = Vec::new();
    for &node in graph.keys() {
        if !in_progress.is_empty() {
            // vanilla: the polluted in-progress set from a cycle crashes the
            // next top-level visit ("You somehow broke the universe") —
            // faithful: feature order cycles are fatal in Java worldgen too.
            panic!("You somehow broke the universe; DFS bork");
        }
        if seen_white.contains(&node) {
            continue;
        }
        // vanilla: `if (!Graph.depthFirstSearch(...)) continue;` — a cycle
        // returns false and SKIPS (the crash comes from the polluted set).
        let _ = dfs(&graph, &mut seen_white, &mut in_progress, &mut order, node);
    }
    order.reverse();
    // Rebuild the featureIndex -> placed-id mapping by replaying the exact
    // computeIfAbsent insertion order (sources outer, steps, holders).
    let mut feature_id_by_index: Vec<usize> = vec![usize::MAX; next_index];
    {
        let mut fi: HashMap<usize, usize> = HashMap::new();
        let mut next = 0usize;
        for src in sources {
            for holder_set in src.iter() {
                for &pf in holder_set.iter() {
                    let idx = *fi.entry(pf).or_insert_with(|| {
                        let v = next;
                        next += 1;
                        v
                    });
                    feature_id_by_index[idx] = pf;
                }
            }
        }
    }
    // Java buckets by featureData.step(): the step recorded per FeatureData
    // (first occurrence across sources).
    let mut step_of: Vec<usize> = vec![usize::MAX; next_index];
    for src in sources {
        for (i1, holder_set) in src.iter().enumerate() {
            for &pf in holder_set {
                let idx = feature_index[&pf];
                if step_of[idx] == usize::MAX {
                    step_of[idx] = i1;
                }
            }
        }
    }
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); max_steps];
    for &(_s, fidx) in &order {
        out[step_of[fidx]].push(feature_id_by_index[fidx]);
    }
    out
}

/// Graph.depthFirstSearch — 3-color DFS adding nodes on BLACK. Returns false
/// if the node was already black (skip), true if explored.
fn dfs(
    graph: &BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>,
    black: &mut BTreeSet<(usize, usize)>,
    gray: &mut BTreeSet<(usize, usize)>,
    out: &mut Vec<(usize, usize)>,
    node: (usize, usize),
) -> bool {
    if black.contains(&node) {
        return false;
    }
    if gray.contains(&node) {
        // cycle: vanilla Graph returns false WITHOUT cleaning the in-progress
        // set — the caller's next top-level visit panics (faithful crash).
        return false;
    }
    gray.insert(node);
    if let Some(succ) = graph.get(&node) {
        for &s in succ {
            // vanilla: `if (!seen.contains(t) && !search(t)) return false;`
            if black.contains(&s) {
                continue;
            }
            if !dfs(graph, black, gray, out, s) {
                return false;
            }
        }
    }
    gray.remove(&node);
    black.insert(node);
    out.push(node);
    true
}

/// WorldgenRandom-over-Xoroshiro decoration seeder (session 7 decompile).
pub struct DecorationRandom {
    src: XoroshiroRandomSource,
    pub count: u64,
}

impl DecorationRandom {
    pub fn new(unique_seed: i64) -> Self {
        DecorationRandom { src: XoroshiroRandomSource::new(unique_seed), count: 0 }
    }

    /// WorldgenRandom.next(bits) with a non-Legacy inner source.
    #[inline]
    fn next32(&mut self) -> i32 {
        self.count += 1;
        (self.src.next_long() >> 32) as i32
    }

    /// LegacyRandomSource.nextLong shape over the overridden next(32):
    /// ((long)next(32) << 32) + next(32) — two inner 64-bit draws.
    #[inline]
    fn next_long_wg(&mut self) -> i64 {
        ((self.next32() as i64) << 32).wrapping_add(self.next32() as i64)
    }

    /// setDecorationSeed — returns the population seed l.
    pub fn set_decoration_seed(&mut self, level_seed: i64, min_block_x: i32, min_block_z: i32) -> i64 {
        self.src = XoroshiroRandomSource::new(level_seed);
        self.count = 0;
        let l = self.next_long_wg() | 1;
        let l1 = self.next_long_wg() | 1;
        let l2 = (min_block_x as i64)
            .wrapping_mul(l)
            .wrapping_add((min_block_z as i64).wrapping_mul(l1))
            ^ level_seed;
        self.src = XoroshiroRandomSource::new(l2);
        l2
    }

    /// setFeatureSeed — population + index + 10000*step.
    pub fn set_feature_seed(&mut self, population_seed: i64, index: usize, step: usize) {
        let l = population_seed
            .wrapping_add(index as i64)
            .wrapping_add(10000i64.wrapping_mul(step as i64));
        self.src = XoroshiroRandomSource::new(l);
        self.count = 0;
    }

    /// RNG delegated to the inner source (Feature.place draws).
    pub fn rng(&mut self) -> &mut dyn RandomSource {
        &mut self.src
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorter_respects_first_occurrence_indices_and_steps() {
        // two biomes: A has [step0: f1, f2; step1: f3], B has [step0: f2]
        let sources: StepFeatureLists = vec![
            vec![vec![1, 2], vec![3]],
            vec![vec![2]],
        ];
        let steps = build_features_per_step(&sources);
        assert_eq!(steps.len(), 2);
        // topological: f1 before f2 before f3 (A order), f2 shared
        assert_eq!(steps[0], vec![1, 2]);
        assert_eq!(steps[1], vec![3]);
    }

    #[test]
    fn sorter_cycle_fails_loudly() {
        // cycle: step0 f1 depends on f2 (later list) which depends on f1 in
        // the second source — vanilla throws; we panic too.
        let sources: StepFeatureLists = vec![vec![vec![1, 2]], vec![vec![2, 1]]];
        let r = std::panic::catch_unwind(|| build_features_per_step(&sources));
        assert!(r.is_err(), "cycle must fail loudly");
    }

    #[test]
    fn decoration_seed_is_deterministic_and_feature_seed_offset_shape() {
        let mut a = DecorationRandom::new(12345);
        let mut b = DecorationRandom::new(999);
        let pa = a.set_decoration_seed(3053459, 1600, 1600);
        let pb = b.set_decoration_seed(3053459, 1600, 1600);
        assert_eq!(pa, pb, "unique-seed source must be overwritten by setSeed");
        // setFeatureSeed = l + index + 10000*step (shape check via determinism)
        a.set_feature_seed(pa, 3, 2);
        b.set_feature_seed(pa.wrapping_add(3).wrapping_add(10000 * 2), 0, 0);
        // both sources now identically seeded -> same draw
        let da = a.rng().next_long();
        let db = b.rng().next_long();
        assert_eq!(da, db);
    }
}
