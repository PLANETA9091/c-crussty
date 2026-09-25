//! Ticking-bucket AFFINITY SHUFFLE (TASK-459-CX6, WILD закон-11 idea C-X6).
//!
//! RegionTickOps (region_threads=4) partitions the vanilla entity snapshot
//! into W spatial buckets via `bucketOf(entity, w)` — at w=4 the mapping is
//! the 8-chunk-region CHECKERBOARD `((rx&1)<<1)|(rz&1)`: orthogonal region
//! neighbours are NEVER on the same worker (P=0). That maximizes spatial
//! separation of concurrent workers (push/merge race control) but destroys
//! cache locality: each worker's working set is scattered dust 16 chunks
//! apart, so the broadphase paths (moonrise EntityLookup entity sections,
//! chunk refs, movement-AABB vs 8x8-chunk region reads — broadphase lane is
//! 8.8-10.9% of tick wall per LAB_LEDGER) run cold every tick on every core.
//!
//! C-X6 flips ONLY the bucket MEMBERSHIP (never the sequence):
//!   affinity(w=4) = ((floorDiv(rx,2)&1)<<1) | (floorDiv(rz,2)&1)
//! i.e. 2x2 BLOCKS of 8-chunk regions = a 16x16-chunk tile per worker
//! (tile-checkerboard). Adjacent 8-chunk regions land on one worker with
//! P=1/2 (vs 0 vanilla) and each worker's tick set becomes spatially
//! coherent -> broadphase section/chunk-ref lines stay hot in the worker's
//! L1/L2. Folia ships the same "nearby chunks -> one region thread" premise
//! (github.com/PaperMC/Folia README); work-stealing literature prices the
//! static-affinity vs cache-locality trade explicitly (Wikipedia: work
//! stealing, PDF scheduler for shared-cache cores).
//!
//! PARITY BOUNDARY (bit-for-bit, preregistered G2): the snapshot partition
//! loop (RegionTickOps.forEach, single `int s = bucketOf(e, w)` site)
//! APPENDS in snapshot order regardless of bucket id — affinity changes
//! which bucket an entity lands in, never the order inside a bucket. The
//! intra-bucket subsequence stays a subsequence of the vanilla snapshot
//! (order-preserving partition), verified by test_order_preserved below and
//! offline by the RegionLockstepHarness oracle.
//!
//! STRICT DORMANT (lever protocol, round-400 canon): env
//! CRUSSTY_LEVER_FLAG == "cmp459_cx6" (STRICT eq — полу-вооружённый мост
//! AIOOBE lesson TASK-400-D) AND region_threads>=2. This scaffold registers
//! ZERO byte hooks: no class bytes, no defines, no retransforms (lesson-408:
//! a lever baked into SOURCES without a rebuilt tracked blob = placebo
//! sleeping gate). The Java-side contract stub lives at
//! entityinside/net/minecraft/world/entity/BucketAffinityOps.java; the
//! single future wire site is documented there (RegionTickOps.java:573).
//! With the gate off this module is byte-indistinguishable from the
//! pre-CX6 plugin.

/// Lever flag (STRICT eq, round-400 lever protocol).
pub const LEVER_FLAG: &str = "cmp459_cx6";
/// Region span of the vanilla bucketing (RegionTickOps.REGION_CHUNKS).
pub const REGION_CHUNKS: i32 = 8;

/// STRICT eq env gate (never starts_with/contains — TASK-400-D lesson).
fn parse_lever_flag(v: Option<&str>) -> bool {
    v.map(|s| s.trim() == LEVER_FLAG).unwrap_or(false)
}

fn lever_flag_val() -> Option<String> {
    std::env::var("CRUSSTY_LEVER_FLAG").ok()
}

/// region_threads workers (shared parser with the region lever; None = off).
fn region_workers() -> Option<i64> {
    crate::region_threads::workers_from_env_pub()
}

/// Full arm condition for the FUTURE byte hook (scaffold: never used to
/// register anything — see module header).
pub fn armed() -> bool {
    parse_lever_flag(lever_flag_val().as_deref()) && region_workers().is_some()
}

/// Vanilla RegionTickOps.bucketOf on REGION coords, replicated bit-for-bit
/// (i32 wrapping == java int overflow; div_euclid/rem_euclid == java
/// floorDiv/floorMod for positive divisors). rx/rz are 8-chunk region coords.
pub fn vanilla_bucket_of(rx: i32, rz: i32, w: usize) -> usize {
    match w {
        4 => (((rx & 1) << 1) | (rz & 1)) as usize,
        2 => (rx & 1) as usize,
        _ => (rx
            .wrapping_mul(668265261)
            .wrapping_add(rz.wrapping_mul(374761393))
            .rem_euclid(w as i32)) as usize,
    }
}

/// C-X6 affinity mapping on REGION coords: locality shuffle, w in {2,4}
/// only (w=4: 2x2 blocks of regions = 16x16-chunk tile per worker; w=2:
/// 2-region x-blocks). w outside {2,4} stays vanilla (out of scope).
pub fn affinity_bucket_of(rx: i32, rz: i32, w: usize) -> usize {
    match w {
        4 => (((rx.div_euclid(2) & 1) << 1) | (rz.div_euclid(2) & 1)) as usize,
        2 => (rx.div_euclid(2) & 1) as usize,
        _ => vanilla_bucket_of(rx, rz, w),
    }
}

/// G1 effect-marker math: fraction of a snapshot that changes bucket id
/// under the affinity mapping vs vanilla (harness census; >=0.10 required
/// by the preregistered ARM gate). `coords` = (rx, rz) per snapshot entity.
pub fn mapping_delta_report(coords: &[(i32, i32)], w: usize) -> Option<f64> {
    if coords.is_empty() {
        return None;
    }
    let changed = coords
        .iter()
        .filter(|&&(rx, rz)| vanilla_bucket_of(rx, rz, w) != affinity_bucket_of(rx, rz, w))
        .count();
    Some(changed as f64 / coords.len() as f64)
}

/// G2 order oracle: partition a snapshot by `map` and assert every bucket
/// sequence is strictly increasing in snapshot index (a partition that only
/// re-labels membership can never permute within a bucket). Returns the
/// bucket -> indices sequences, or None on any order violation.
pub fn bucket_sequences<F>(n: usize, w: usize, map: F) -> Option<Vec<Vec<usize>>>
where
    F: Fn(usize) -> Option<usize>,
{
    let mut seqs = vec![Vec::new(); w];
    for i in 0..n {
        let b = map(i)?;
        if b >= w {
            return None;
        }
        if let Some(last) = seqs[b].last() {
            if *last >= i {
                return None; // order violated (never happens: single writer)
            }
        }
        seqs[b].push(i);
    }
    Some(seqs)
}

/// Registration: fail-closed scaffold — with the gate off this is a no-op
/// (dormant-invisible); with the gate on this STILL registers nothing
/// (lesson-408: no rebuilt tracked blob = no arm; the real byte hook lands
/// only with the javap flat==nested-verified blob rebuild).
pub fn register() {
    if !armed() {
        return; // dormant-invisible: 0 hooks, 0 defines, 0 retransforms
    }
    // ARM requested but scaffold ships no blobs: fail-closed, no byte hook.
    // The future wiring (after blob rebuild + javap flat==nested 10/10):
    //   retarget RegionTickOps.forEach `bucketOf(e, w)` call site (1 site,
    //   RegionTickOps.java:573) -> BucketAffinityOps.affinityBucketOf.
}

/// Activation phase: same fail-closed contract as [`register`].
pub fn activate() {
    register();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(spread: i32) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for rx in -spread..spread {
            for rz in -spread..spread {
                v.push((rx, rz));
            }
        }
        v
    }

    #[test]
    fn lever_gate_strict_eq() {
        assert!(parse_lever_flag(Some("cmp459_cx6")));
        assert!(parse_lever_flag(Some(" cmp459_cx6 "))); // trim only
        assert!(!parse_lever_flag(Some("cmp459_cx6x"))); // no prefix/contains
        assert!(!parse_lever_flag(Some("cmp458_roar")));
        assert!(!parse_lever_flag(None));
        assert!(!parse_lever_flag(Some("")));
    }

    #[test]
    fn vanilla_mapping_matches_java_semantics() {
        // w=4 quadrant parity, incl. negative region coords (floorDiv).
        assert_eq!(vanilla_bucket_of(0, 0, 4), 0);
        assert_eq!(vanilla_bucket_of(1, 0, 4), 2);
        assert_eq!(vanilla_bucket_of(0, 1, 4), 1);
        assert_eq!(vanilla_bucket_of(-1, -1, 4), 3); // floorDiv(-1,8)=-1
        // w=2 x-stripes.
        assert_eq!(vanilla_bucket_of(-1, 5, 2), 1);
        // w=8 hashed fallback: bit-exact java floorMod replication.
        let j = (-5i32)
            .wrapping_mul(668265261)
            .wrapping_add(7i32.wrapping_mul(374761393))
            .rem_euclid(8);
        assert_eq!(vanilla_bucket_of(-5, 7, 8), j as usize);
    }

    #[test]
    fn affinity_locality_positive_vs_zero_vanilla() {
        // Vanilla checkerboard: orthogonal neighbours NEVER share a bucket.
        let grid = snapshot(16);
        let adj_same = |map: fn(i32, i32, usize) -> usize, w: usize| -> f64 {
            let (mut same, mut total) = (0usize, 0usize);
            for &(rx, rz) in &grid {
                for (dx, dy) in [(1, 0), (0, 1)] {
                    total += 1;
                    if map(rx, rz, w) == map(rx + dx, rz + dy, w) {
                        same += 1;
                    }
                }
            }
            same as f64 / total as f64
        };
        let v = adj_same(vanilla_bucket_of, 4);
        let a = adj_same(affinity_bucket_of, 4);
        assert_eq!(v, 0.0, "vanilla checkerboard must be fully separating");
        assert!(a > 0.4, "affinity must co-locate neighbours, got {}", a);
    }

    #[test]
    fn order_preserved_membership_only() {
        // G2: both mappings must yield strictly index-increasing per-bucket
        // sequences — shuffle touches membership only, never sequence.
        let coords = snapshot(12);
        let w = 4usize;
        let seqs_v = bucket_sequences(coords.len(), w, |i| {
            Some(vanilla_bucket_of(coords[i].0, coords[i].1, w))
        })
        .expect("order oracle vanilla");
        let seqs_a = bucket_sequences(coords.len(), w, |i| {
            Some(affinity_bucket_of(coords[i].0, coords[i].1, w))
        })
        .expect("order oracle affinity");
        for (tag, seqs) in [("vanilla", seqs_v), ("affinity", seqs_a)] {
            for (b, seq) in seqs.iter().enumerate() {
                for k in 1..seq.len() {
                    assert!(
                        seq[k - 1] < seq[k],
                        "{} bucket {} order broken at {}",
                        tag,
                        b,
                        k
                    );
                }
            }
        }
    }

    #[test]
    fn affinity_balance_not_destroyed() {
        // Locality must not collapse balance: tile-checkerboard over a
        // uniform region grid keeps every bucket within 1.2x of the mean.
        let coords = snapshot(24);
        let mut counts = [0usize; 4];
        for &(rx, rz) in &coords {
            counts[affinity_bucket_of(rx, rz, 4)] += 1;
        }
        let mean = coords.len() as f64 / 4.0;
        for c in counts {
            assert!(
                (c as f64) <= 1.2 * mean && (c as f64) >= 0.8 * mean,
                "bucket imbalance: {:?} vs mean {}",
                counts,
                mean
            );
        }
    }

    #[test]
    fn out_of_scope_workers_untouched() {
        for w in [1usize, 3, 5, 8, 16] {
            let coords = snapshot(6);
            for &(rx, rz) in &coords {
                assert_eq!(
                    affinity_bucket_of(rx, rz, w),
                    vanilla_bucket_of(rx, rz, w),
                    "w={} must stay vanilla",
                    w
                );
            }
        }
    }

    #[test]
    fn mapping_delta_marker_math() {
        // Uniform large grid: ~50% of regions change bucket at w=4 (>= the
        // preregistered 10% G1 effect-marker threshold).
        let coords = snapshot(32);
        let d = mapping_delta_report(&coords, 4).expect("non-empty");
        assert!(d > 0.10, "effect marker too small: {}", d);
        assert!(d < 0.90, "mapping must stay partial: {}", d);
        assert!(mapping_delta_report(&[], 4).is_none());
    }
}
