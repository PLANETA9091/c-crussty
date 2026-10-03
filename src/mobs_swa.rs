//! AG-243 / TASK-458-I — SWAR batch-AABB broadphase kernel, iter-1 (DORMANT).
//!
//! Lever: cmp458_swar. Research: RESEARCH-458-I §0-§1 (Box2D "SIMD for
//! Collision" wide-SIMD canon; Sweep-and-prune theorem: x-interval overlap is
//! a NECESSARY condition of AABB overlap => a min_x-window is a superset).
//!
//! This file ships the rust-side iter-1 kernel only:
//!   (a) candidate window build via binary search over the min_x-sorted SoA;
//!   (b) portable SWAR 8-wide batch-AABB mask on u64 (8 boxes per register,
//!       bit operations only, NO simd intrinsics — Box2D scalar-fallback law);
//!   (c) scalar fallback with the identical contract (bit-for-bit mask);
//!   (d) monotone SAP push-tail: insertion-sort pass over a nearly-sorted
//!       array, O(n + inversions) per tick.
//!
//! DORMANT: nothing here is called from production paths (law 4: STRICT-eq
//! isolation). The java-bridge / NCDFE wiring is iter-2 (see result.md).

/// Axis-aligned box, f32 columns (flat f32 column layout, no fixed-point
/// conversion needed at iter-1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Box2 {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl Box2 {
    /// True strict overlap on x/z only. y is NOT pruned: vertical drift is not
    /// bounded by the x/z margin (pushCandidates parity, RESEARCH-458-I §1).
    /// Ordered comparisons => NaN yields false on every lane (fail-closed).
    #[inline]
    pub fn intersects_xz(&self, o: &Box2) -> bool {
        self.min_x <= o.max_x
            && self.max_x >= o.min_x
            && self.min_z <= o.max_z
            && self.max_z >= o.min_z
    }

    /// This box inflated by `m` on x/z (push margin, MARGIN=8.0 canon).
    #[inline]
    pub fn inflate_xz(&self, m: f32) -> Box2 {
        Box2 {
            min_x: self.min_x - m,
            max_x: self.max_x + m,
            min_z: self.min_z - m,
            max_z: self.max_z + m,
            ..*self
        }
    }
}

/// Structure-of-arrays for the broadphase lane. `min_x` is kept sorted
/// ascending by the monotone SAP tail; all other columns are parallel.
#[derive(Clone, Debug, Default)]
pub struct SwaSoa {
    pub min_x: Vec<f32>,
    pub max_x: Vec<f32>,
    pub min_z: Vec<f32>,
    pub max_z: Vec<f32>,
    /// Live entity ids parallel to the columns (order follows min_x sort).
    pub ids: Vec<u32>,
}

impl SwaSoa {
    #[inline]
    pub fn len(&self) -> usize {
        self.min_x.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min_x.is_empty()
    }
}

/// IEEE-754 monotone map f32 -> ordered u32 key (total order, -0 < +0).
/// Used by the sort so the binary search and the insertion pass compare with
/// one deterministic ordering.
#[inline]
pub fn f32_ordered_key(x: f32) -> u32 {
    let b = x.to_bits();
    if b & 0x8000_0000 != 0 {
        !b
    } else {
        b | 0x8000_0000
    }
}

/// (a) Candidate window build: two lower_bounds on the min_x-sorted column.
/// Returns the half-open index range [lo, hi) of boxes whose min_x lies in
/// [x_lo, x_hi). CALLER CONTRACT: `x_lo` must be sized by the max box span
/// (query.min_x - margin - max_span) so long boxes starting left of x_lo
/// cannot overlap the inflated query (superset precondition).
pub fn window_bounds(soa: &SwaSoa, x_lo: f32, x_hi: f32) -> (usize, usize) {
    let lo_key = f32_ordered_key(x_lo);
    let hi_key = f32_ordered_key(x_hi);
    let lower_bound = |from: usize, to: usize, key: u32| -> usize {
        let (mut a, mut b) = (from, to);
        while a < b {
            let mid = a + (b - a) / 2;
            if f32_ordered_key(soa.min_x[mid]) < key {
                a = mid + 1;
            } else {
                b = mid;
            }
        }
        a
    };
    let lo = lower_bound(0, soa.len(), lo_key);
    let hi = lower_bound(lo, soa.len(), hi_key);
    (lo, hi)
}

/// (b) Portable SWAR 8-wide batch-AABB mask on u64.
///
/// 8 candidate lanes are packed into one u64 accumulator: lane i's verdict
/// occupies bit i, x and z axis verdicts are AND-combined in the same
/// register — pure bit operations, no simd intrinsics (Box2D canon: "a
/// surprising number of users without AVX2 capable CPUs" => portable kernel
/// first; AVX2 movemask is an iter-2 opt-in behind the same contract).
/// Ordered f32 comparisons keep NaN lanes at 0 (fail-closed).
///
/// Lanes [base, base + lanes) with `lanes <= 8`; out-of-range lane bits are 0.
#[inline]
pub fn swar_mask8(soa: &SwaSoa, base: usize, lanes: usize, q: &Box2) -> u64 {
    debug_assert!(lanes <= 8 && base + lanes <= soa.len());
    let mut acc: u64 = 0;
    for i in 0..lanes {
        let idx = base + i;
        // x axis: two ordered compares per lane, verdict as 0/1 bit
        let x_ok = (soa.min_x[idx] <= q.max_x) as u64 & ((soa.max_x[idx] >= q.min_x) as u64);
        // z axis into the same register (AND-combine, SWAR-style reduction)
        let z_ok = (soa.min_z[idx] <= q.max_z) as u64 & ((soa.max_z[idx] >= q.min_z) as u64);
        acc |= (x_ok & z_ok) << i;
    }
    acc
}

/// Full-window SWAR pass: mask over window [wlo, whi) chunked in 8-lane
/// registers. Returns (offset, mask) chunks; identical chunking to the
/// scalar fallback so the two are comparable chunk-for-chunk.
pub fn swar_window_mask(soa: &SwaSoa, wlo: usize, whi: usize, q: &Box2) -> Vec<(usize, u64)> {
    let mut out = Vec::with_capacity((whi - wlo).div_ceil(8));
    let mut base = wlo;
    while base < whi {
        let lanes = 8.min(whi - base);
        out.push((base, swar_mask8(soa, base, lanes, q)));
        base += 8;
    }
    out
}

/// (c) Scalar fallback — identical contract to `swar_window_mask`, plain loop.
/// Bit-for-bit equality with the SWAR path is a hard selftest invariant.
pub fn scalar_window_mask(soa: &SwaSoa, wlo: usize, whi: usize, q: &Box2) -> Vec<(usize, u64)> {
    let mut out = Vec::with_capacity((whi - wlo).div_ceil(8));
    let mut base = wlo;
    while base < whi {
        let lanes = 8.min(whi - base);
        let mut m: u64 = 0;
        for i in 0..lanes {
            let idx = base + i;
            let ok = soa.min_x[idx] <= q.max_x
                && soa.max_x[idx] >= q.min_x
                && soa.min_z[idx] <= q.max_z
                && soa.max_z[idx] >= q.min_z;
            m |= (ok as u64) << i;
        }
        out.push((base, m));
        base += 8;
    }
    out
}

/// One-shot convenience: window build + SWAR mask. `margin` inflates the
/// query on x/z; `max_span` bounds candidate box width on x so the window
/// lower bound keeps the superset guarantee.
pub fn broadphase_candidates(soa: &SwaSoa, q: &Box2, margin: f32, max_span: f32) -> Vec<(usize, u64)> {
    let x_lo = q.min_x - margin - max_span;
    let x_hi = q.max_x + margin;
    let (wlo, whi) = window_bounds(soa, x_lo, x_hi);
    let qi = q.inflate_xz(margin);
    swar_window_mask(soa, wlo, whi, &qi)
}

/// (d) Monotone SAP push-tail: one insertion-sort pass over the nearly-sorted
/// SoA (min_x column + parallel columns/ids). Insertion sort is
/// O(n + inversions): on a mostly-sorted array each tick's drift introduces
/// O(1) inversions per moved mob, so the pass is ~O(n) — the SAP incremental
/// maintenance canon (I-COLLIDE lineage, RESEARCH-458-I §1).
/// Returns the number of moves (shifts); caller may trigger a full re-sort
/// when it exceeds a degradation threshold (self-heal, iter-2).
pub fn insertion_sort_pass(soa: &mut SwaSoa) -> u64 {
    let n = soa.len();
    let mut moves: u64 = 0;
    for i in 1..n {
        let ki = f32_ordered_key(soa.min_x[i]);
        let mut j = i;
        while j > 0 && f32_ordered_key(soa.min_x[j - 1]) > ki {
            soa.min_x.swap(j - 1, j);
            soa.max_x.swap(j - 1, j);
            soa.min_z.swap(j - 1, j);
            soa.max_z.swap(j - 1, j);
            soa.ids.swap(j - 1, j);
            moves += 1;
            j -= 1;
        }
    }
    moves
}

// ---------------------------------------------------------------------------
// Selftest: 1000 synthetic boxes -> swar mask == fallback mask bit-for-bit,
// superset invariant vs exact brute force, insertion pass O(n+inv) correctness.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic xorshift64* PRNG (no external deps).
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        fn f32_in(&mut self, lo: f32, hi: f32) -> f32 {
            let u = (self.next() >> 40) as f32 / (1u64 << 24) as f32;
            lo + u * (hi - lo)
        }
    }

    const N: usize = 1000;
    /// Max x-span of synthetic boxes — must be <= MAX_SPAN so the window
    /// lower bound (min_x - margin - MAX_SPAN) keeps the superset guarantee.
    const MAX_SPAN: f32 = 4.0;
    const MARGIN: f32 = 8.0;

    fn build_soa(seed: u64, perturb: bool) -> SwaSoa {
        let mut rng = Rng(seed);
        let mut soa = SwaSoa::default();
        let mut exact: Vec<Box2> = Vec::with_capacity(N);
        for i in 0..N {
            let cx = rng.f32_in(-512.0, 512.0);
            let cz = rng.f32_in(-512.0, 512.0);
            let hw = rng.f32_in(0.3, MAX_SPAN / 2.0);
            let hd = rng.f32_in(0.3, MAX_SPAN / 2.0);
            let b = Box2 {
                min_x: cx - hw,
                max_x: cx + hw,
                min_y: 0.0,
                max_y: 2.0,
                min_z: cz - hd,
                max_z: cz + hd,
            };
            let _ = b;
            soa.min_x.push(b.min_x);
            soa.max_x.push(b.max_x);
            soa.min_z.push(b.min_z);
            soa.max_z.push(b.max_z);
            soa.ids.push(i as u32);
        }
        // The binsearch-window contract REQUIRES the min_x-sorted column
        // (SAP invariant maintained by the monotone push-tail) — establish it.
        let mut idx: Vec<usize> = (0..N).collect();
        idx.sort_by_key(|&i| f32_ordered_key(soa.min_x[i]));
        let (a, b, c, d, e) = (
            soa.min_x.clone(),
            soa.max_x.clone(),
            soa.min_z.clone(),
            soa.max_z.clone(),
            soa.ids.clone(),
        );
        for i in 0..N {
            soa.min_x[i] = a[idx[i]];
            soa.max_x[i] = b[idx[i]];
            soa.min_z[i] = c[idx[i]];
            soa.max_z[i] = d[idx[i]];
            soa.ids[i] = e[idx[i]];
        }
        if perturb {
            // shift 50 sorted elements by small offsets => a handful of
            // inversions each: the "one tick of drift" regime.
            for _ in 0..50 {
                let i = (rng.next() % (N as u64 - 2) + 1) as usize;
                let delta = rng.f32_in(-1.5, 1.5);
                soa.min_x[i] += delta;
                soa.max_x[i] += delta;
            }
        }
        soa
    }

    fn box_at(soa: &SwaSoa, i: usize) -> Box2 {
        Box2 {
            min_x: soa.min_x[i],
            max_x: soa.max_x[i],
            min_y: 0.0,
            max_y: 2.0,
            min_z: soa.min_z[i],
            max_z: soa.max_z[i],
        }
    }

    fn exact_hits(soa: &SwaSoa, q: &Box2) -> Vec<usize> {
        (0..soa.len()).filter(|&i| box_at(soa, i).intersects_xz(q)).collect()
    }

    #[test]
    fn swar_mask_equals_fallback_mask_bitwise() {
        let soa = build_soa(0xDEAD_BEEF, false);
        let mut rng = Rng(42);
        for _ in 0..100 {
            let q = Box2 {
                min_x: rng.f32_in(-520.0, 500.0),
                max_x: rng.f32_in(-500.0, 520.0),
                min_y: 0.0,
                max_y: 2.0,
                min_z: rng.f32_in(-520.0, 500.0),
                max_z: rng.f32_in(-500.0, 520.0),
            };
            let x_lo = q.min_x - MARGIN - MAX_SPAN;
            let x_hi = q.max_x + MARGIN;
            let (wlo, whi) = window_bounds(&soa, x_lo, x_hi);
            let qi = q.inflate_xz(MARGIN);
            let swar = broadphase_candidates(&soa, &q, MARGIN, MAX_SPAN);
            let fallback = scalar_window_mask(&soa, wlo, whi, &qi);
            assert_eq!(swar.len(), fallback.len(), "chunk count mismatch");
            for k in 0..swar.len() {
                assert_eq!(swar[k].0, fallback[k].0, "chunk offset mismatch");
                assert_eq!(swar[k].1, fallback[k].1, "mask bits differ (chunk {k})");
            }
        }
    }

    #[test]
    fn swar_mask_is_superset_of_exact_result() {
        let soa = build_soa(0x1234_5678, false);
        let mut rng = Rng(7);
        for _ in 0..100 {
            let q = Box2 {
                min_x: rng.f32_in(-520.0, 500.0),
                max_x: rng.f32_in(-500.0, 520.0),
                min_y: 0.0,
                max_y: 2.0,
                min_z: rng.f32_in(-520.0, 500.0),
                max_z: rng.f32_in(-500.0, 520.0),
            };
            let qi = q.inflate_xz(MARGIN);
            let swar = broadphase_candidates(&soa, &q, MARGIN, MAX_SPAN);
            let has = |i: usize| -> bool {
                swar.iter().any(|&(off, m)| {
                    let d = i as isize - off as isize;
                    (0..8).contains(&d) && (m >> d) & 1 == 1
                })
            };
            for i in exact_hits(&soa, &qi) {
                assert!(has(i), "exact hit {i} missing (box={:?} qi={:?} lo={:?}) ", box_at(&soa, i), qi, window_bounds(&soa, q.min_x - MARGIN - MAX_SPAN, q.max_x + MARGIN));
            }
        }
    }

    #[test]
    fn insertion_pass_sorts_nearly_sorted_in_o_n_plus_inv() {
        let mut soa = build_soa(0xAAAA_0001, true);
        // inversions before the pass (brute force, n=1000 is fine in test)
        let inv_before: u64 = {
            let mut c = 0u64;
            for i in 0..N {
                for j in i + 1..N {
                    if f32_ordered_key(soa.min_x[j]) < f32_ordered_key(soa.min_x[i]) {
                        c += 1;
                    }
                }
            }
            c
        };
        let moves = insertion_sort_pass(&mut soa);
        assert!(moves <= inv_before, "moves {moves} > inversions {inv_before}");
        for i in 1..N {
            assert!(
                f32_ordered_key(soa.min_x[i - 1]) <= f32_ordered_key(soa.min_x[i]),
                "array not sorted after single insertion pass at {i}"
            );
        }
        assert!(moves <= 400, "nearly-sorted (50 perturbed) must be ~O(n+inv), got {moves}");
    }

    #[test]
    fn nan_and_degenerate_fail_closed() {
        // Direct masks over a GIVEN lane range (no window): NaN lane inside.
        let mut soa = SwaSoa::default();
        soa.min_x = vec![f32::NAN, 0.0, 10.0];
        soa.max_x = vec![f32::NAN, 0.0, 11.0];
        soa.min_z = vec![0.0, 0.0, 0.0];
        soa.max_z = vec![1.0, 1.0, 1.0];
        soa.ids = vec![0, 1, 2];
        let q = Box2 { min_x: -1.0, min_y: 0.0, max_y: 2.0, max_x: 5.0, min_z: -1.0, max_z: 5.0 };
        let swar = swar_mask8(&soa, 0, 3, &q);
        let mut fb = 0u64;
        for (off, m) in scalar_window_mask(&soa, 0, 3, &q) {
            assert_eq!(off, 0);
            fb |= m;
        }
        assert_eq!(swar, fb, "SWAR vs fallback must agree with NaN lane present");
        assert_eq!(swar & 1, 0, "NaN lane must not intersect (fail-closed)");
        assert_eq!(swar & 0b010, 0b010, "lane 1 overlaps q on x/z");
        assert_eq!(swar & 0b100, 0, "lane 2 (min_x 10 > max_x 5) must not intersect");
        // Window path with NaN in the column: NaN key sorts above +inf, so the
        // NaN box sits at the tail and the binsearch window must exclude it.
        let mut sorted = SwaSoa::default();
        sorted.min_x = vec![0.0, 10.0, f32::NAN];
        sorted.max_x = vec![0.0, 11.0, f32::NAN];
        sorted.min_z = vec![0.0, 0.0, 0.0];
        sorted.max_z = vec![1.0, 1.0, 1.0];
        sorted.ids = vec![1, 2, 0];
        let w = broadphase_candidates(&sorted, &q, 0.0, MAX_SPAN);
        assert_eq!(scalar_window_mask(&sorted, 0, 1, &q), w, "window mask must match fallback");
        assert_eq!(w[0].1 & 1, 1, "box [0,0]x[0,1]z intersects q");
        assert_eq!(w[0].1 & 0b110, 0, "boxes 10..11 and NaN are outside window/predicate");
    }
}
