//! TASK-15 — differential fuzz parity for the area-map bridge.
//!
//! Three suites, all headless (no JVM, no JNI, no .so):
//!
//!   1. `fuzz_parity_main`          — >= 10_000 randomized cases: random
//!      window dims (tiny 0x0 / 1xN / 8x8, medium 64x64, a few 256x256),
//!      random coordinates incl. edge values (MIN/MAX/zero), random mutation
//!      sequences (moves, grow/shrink, d=0, same-state pressure). Every
//!      mutation is applied through BOTH the fast-path model (exact port of
//!      `SingleUserAreaMapOps.run`, i.e. the shipped hot path) and the
//!      independent apply-loop reference (naive per-cell set difference,
//!      the TASK-11 oracle shape). Full-grid element-wise parity is asserted
//!      per mutation (small windows) and at case end (always), plus exact
//!      tracked-set equality (covers cells outside the window).
//!
//!   2. `fuzz_adversarial_idempotency` — sequences that contain ONLY no-op
//!      state writes after settling on a state S: the same-state fast path
//!      must apply ZERO enumeration calls and ZERO callbacks and leave the
//!      grid bit-identical, while the reference (which enumerates an empty
//!      difference every time) must agree. Mirrors TASK-11's S2 with the
//!      counter guard against a vacuous pass.
//!
//!   3. `fuzz_edge_coordinates` — deterministic MIN/MAX/zero coordinate
//!      sweep: key pack/decode round-trip, move/move-back/same-state/grow
//!      sequences at the coordinate seam, fast-path hit counting.
//!
//! All case seeds derive from the fixed BASE_SEED; every panic reports the
//! base seed, case index, case seed, dims and the first differing grid
//! index, so any failure is exactly reproducible.

use area_map_fuzz::*;
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

struct CasePlan {
    w: usize,
    h: usize,
    x0: i32,
    z0: i32,
    mutations: usize,
    d_max: i64,
    span: i64,
    edge_coords: bool,
    /// per-mutation element-wise grid compare for small windows only
    grid_every_mutation: bool,
}

fn plan_for(rng: &mut XorShift64Star, idx: usize) -> CasePlan {
    match idx % 100 {
        // 55%: tiny windows — 0x0, 1xN, Nx1, 1x1, 2x2, 8x8
        0..=54 => {
            let (w, h) = match rng.below(6) {
                0 => (0usize, 0usize),
                1 => (1usize, 1 + rng.below(8) as usize),
                2 => (1 + rng.below(8) as usize, 1usize),
                3 => (1, 1),
                4 => (2, 2),
                _ => (8, 8),
            };
            let w0 = -(64i64);
            CasePlan {
                w,
                h,
                x0: rng.range_i64(w0, 56) as i32,
                z0: rng.range_i64(w0, 56) as i32,
                mutations: 4 + rng.below(24) as usize,
                d_max: 8,
                span: 64,
                edge_coords: false,
                grid_every_mutation: true,
            }
        }
        // 30%: small/medium — mostly 64x64, some 16x16/32x32
        55..=84 => {
            let (w, h) = if rng.chance_pct(20) {
                let s = if rng.chance_pct(50) { 16 } else { 32 };
                (s, s)
            } else {
                (64, 64)
            };
            CasePlan {
                w,
                h,
                x0: rng.range_i64(-256, 192) as i32,
                z0: rng.range_i64(-256, 192) as i32,
                mutations: 6 + rng.below(12) as usize,
                d_max: 24,
                span: 256,
                edge_coords: rng.chance_pct(10),
                grid_every_mutation: w * h <= 4096,
            }
        }
        // 12%: medium-plus 64x64, larger d and span
        85..=96 => CasePlan {
            w: 64,
            h: 64,
            x0: rng.range_i64(-1024, 960) as i32,
            z0: rng.range_i64(-1024, 960) as i32,
            mutations: 4 + rng.below(8) as usize,
            d_max: 64,
            span: 1024,
            edge_coords: rng.chance_pct(10),
            grid_every_mutation: true,
        },
        // 4%: a few big 256x256 windows (end-of-case grid compare only)
        _ => CasePlan {
            w: 256,
            h: 256,
            x0: rng.range_i64(-1024, 768) as i32,
            z0: rng.range_i64(-1024, 768) as i32,
            mutations: 2 + rng.below(4) as usize,
            d_max: 32,
            span: 1024,
            edge_coords: true,
            grid_every_mutation: false,
        },
    }
}

/// Random mutation with same-state pressure (exact repeats) and d=0
/// pressure; `prev` is updated so repeats can be emitted.
fn sample_mutation(
    rng: &mut XorShift64Star,
    plan: &CasePlan,
    prev: &mut Option<(i32, i32, i32)>,
) -> (i32, i32, i32) {
    if let Some(p) = *prev {
        if rng.chance_pct(25) {
            return p; // exact no-op state write (fast-path pressure)
        }
    }
    let (mut x, mut z) = (rng.range_i64(-plan.span, plan.span), rng.range_i64(-plan.span, plan.span));
    if plan.edge_coords && rng.chance_pct(15) {
        match rng.below(3) {
            0 => x = i32::MIN as i64 + rng.below(64) as i64,
            1 => x = i32::MAX as i64 - rng.below(64) as i64,
            _ => x = 0,
        }
        match rng.below(3) {
            0 => z = i32::MIN as i64 + rng.below(64) as i64,
            1 => z = i32::MAX as i64 - rng.below(64) as i64,
            _ => z = 0,
        }
    }
    let d = if rng.chance_pct(20) { 0 } else { rng.range_i64(0, plan.d_max) };
    let m = (x as i32, z as i32, d as i32);
    *prev = Some(m);
    m
}

fn first_set_mismatch(a: &HashSet<(i32, i32)>, b: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    for cell in a.symmetric_difference(b) {
        return Some(*cell);
    }
    None
}

macro_rules! parity_fail {
    ($($arg:tt)*) => {
        panic!("AREA-MAP FUZZ PARITY FAIL :: {}", format_args!($($arg)*))
    };
}

fn assert_parity(
    tag: &str,
    base_seed: u64,
    case_idx: usize,
    case_seed: u64,
    plan: &CasePlan,
    mutation_idx: usize,
    f: &FastPathMap,
    r: &ReferenceMap,
    require_grid: bool,
) {
    if f.tracked != r.tracked {
        let cell = first_set_mismatch(&f.tracked, &r.tracked).unwrap();
        parity_fail!(
            "{tag}: tracked-set mismatch at base_seed={base_seed:#x} case={case_idx} \
             case_seed={case_seed:#x} dims={}x{} origin=({},{}) mutation#{mutation_idx} \
             chunk={:?} f_ops={} r_ops={} f_enumeration_calls={}",
            plan.w, plan.h, plan.x0, plan.z0, cell, f.ops_applied, r.ops_applied,
            f.enumeration_calls
        );
    }
    if require_grid {
        let win = Window { w: plan.w, h: plan.h, x0: plan.x0, z0: plan.z0 };
        if let Some((row, col, x, z)) = first_grid_mismatch(&f.tracked, &r.tracked, &win) {
            parity_fail!(
                "{tag}: grid mismatch at base_seed={base_seed:#x} case={case_idx} \
                 case_seed={case_seed:#x} dims={}x{} origin=({},{}) mutation#{mutation_idx} \
                 first_differing_index=(row {row}, col {col}) chunk=({x},{z}) \
                 f_ops={} r_ops={}",
                plan.w, plan.h, plan.x0, plan.z0, f.ops_applied, r.ops_applied
            );
        }
    }
}

// ---------------------------------------------------------------------------
// suite 1: main differential fuzz (>= 10_000 randomized cases)
// ---------------------------------------------------------------------------

#[test]
fn fuzz_parity_main() {
    const CASES: usize = 12_000;
    let base = BASE_SEED;
    let mut total_mutations = 0usize;
    let mut total_ops = 0u64;
    let mut total_fast = 0u64;
    let mut total_guards = 0u64;

    for case_idx in 0..CASES {
        let cs = case_seed(base, case_idx);
        let mut rng = XorShift64Star::new(cs);
        let plan = plan_for(&mut rng, case_idx);
        let mut f = FastPathMap::new();
        let mut r = ReferenceMap::new();
        let mut prev = None;
        for m in 0..plan.mutations {
            let (x, z, d) = sample_mutation(&mut rng, &plan, &mut prev);
            f.update(x, z, d);
            r.update(x, z, d);
            total_mutations += 1;
            assert_parity(
                "main",
                base, case_idx, cs, &plan, m, &f, &r,
                plan.grid_every_mutation,
            );
        }
        // case end: always full element-wise grid parity over the window
        assert_parity("main/end", base, case_idx, cs, &plan, plan.mutations, &f, &r, true);
        total_ops += f.ops_applied;
        total_fast += f.fast_path_hits;
        total_guards += f.min_value_guards;
    }

    assert!(
        total_fast > 0 && total_guards > 0 && total_ops > 0,
        "distribution drifted: fast_path_hits={total_fast} guards={total_guards} ops={total_ops}"
    );
    println!(
        "fuzz_parity_main: {CASES} cases / {total_mutations} mutations — ALL PASS \
         (ops_applied={total_ops}, fast_path_hits={total_fast}, min_value_guards={total_guards})"
    );
}

// ---------------------------------------------------------------------------
// suite 2: adversarial idempotency — only no-op state writes
// ---------------------------------------------------------------------------

#[test]
fn fuzz_adversarial_idempotency() {
    const CASES: usize = 1_500;
    let base = BASE_SEED;

    for case_idx in 0..CASES {
        let cs = case_seed(base, 1_000_000 + case_idx);
        let mut rng = XorShift64Star::new(cs);

        // settle on a state S through 1..=3 real mutations
        let mut px = rng.range_i64(-4096, 4096) as i32;
        let mut pz = rng.range_i64(-4096, 4096) as i32;
        let mut pd = rng.range_i64(0, 32) as i32;
        let mut f = FastPathMap::new();
        let mut r = ReferenceMap::new();
        f.update(px, pz, pd);
        r.update(px, pz, pd);
        for _ in 0..rng.below(3) {
            px = rng.range_i64(-4096, 4096) as i32;
            pz = rng.range_i64(-4096, 4096) as i32;
            pd = rng.range_i64(0, 32) as i32;
            f.update(px, pz, pd);
            r.update(px, pz, pd);
        }
        if rng.chance_pct(10) {
            // edge-valued no-op targets too
            px = match rng.below(3) {
                0 => i32::MIN + 7,
                1 => i32::MAX - 7,
                _ => 0,
            };
            pz = match rng.below(3) {
                0 => i32::MIN + 7,
                1 => i32::MAX - 7,
                _ => 0,
            };
            f.update(px, pz, pd);
            r.update(px, pz, pd);
        }
        let s = (px, pz, pd);

        // snapshot: grids, sets and the fast-path observables
        let snap_f = f.tracked.clone();
        let snap_r = r.tracked.clone();
        let (calls0, ops0, fast0) = (f.enumeration_calls, f.ops_applied, f.fast_path_hits);

        let repeats = 8 + rng.below(57) as usize;
        for k in 0..repeats {
            f.update(s.0, s.1, s.2);
            r.update(s.0, s.1, s.2);
            let d_calls = f.enumeration_calls - calls0;
            let d_ops = f.ops_applied - ops0;
            let d_fast = f.fast_path_hits - fast0;
            if d_calls != 0 || d_ops != 0 || d_fast != (k + 1) as u64 {
                parity_fail!(
                    "idempotency: fast path did NOT skip a no-op state write \
                     base_seed={base:#x} case={case_idx} case_seed={cs:#x} state={s:?} \
                     repeat#{k} d_enumeration_calls={d_calls} d_ops={d_ops} d_fast={d_fast}"
                );
            }
            if f.tracked != snap_f || r.tracked != snap_r || f.tracked != r.tracked {
                parity_fail!(
                    "idempotency: grid drifted under no-op writes \
                     base_seed={base:#x} case={case_idx} case_seed={cs:#x} state={s:?} repeat#{k}"
                );
            }
        }
    }
    println!("fuzz_adversarial_idempotency: {CASES} cases — ALL PASS (zero enumeration, zero callbacks, bit-identical grids)");
}

// ---------------------------------------------------------------------------
// suite 3: deterministic edge-coordinate sweep (MIN / MAX / zero seam)
// ---------------------------------------------------------------------------

#[test]
fn fuzz_edge_coordinates() {
    const COORDS: [(i32, i32); 11] = [
        (i32::MIN, 0),
        (0, i32::MIN),
        (i32::MAX, 0),
        (0, i32::MAX),
        (i32::MIN, i32::MIN),
        (i32::MAX, i32::MAX),
        (i32::MIN, i32::MAX),
        (i32::MAX, i32::MIN),
        (0, 0),
        (i32::MIN + 1, i32::MAX - 1),
        (i32::MAX - 1, i32::MIN + 1),
    ];
    const DS: [(i64, i64); 4] = [(0, 1), (2, 16), (1, 0), (16, 2)];

    // key pack/decode round-trip on the extreme lanes (u32 halves)
    for &(x, z) in &COORDS {
        let key = pack_key(x, z);
        let (dx, dz) = (key as i32, (key >> 32) as i32);
        assert!(
            (dx, dz) == (x, z),
            "pack/decode round-trip failed for ({x},{z}) -> {key:#x} -> ({dx},{dz})"
        );
    }

    // windows at the interesting seams
    let wins = [
        Window { w: 3, h: 3, x0: -1, z0: -1 },
        Window { w: 2, h: 2, x0: i32::MIN, z0: i32::MIN },
        Window { w: 2, h: 2, x0: i32::MAX - 1, z0: i32::MAX - 1 },
    ];

    let mut sequences = 0usize;
    for (ai, &(ax, az)) in COORDS.iter().enumerate() {
        for (bi, &(bx, bz)) in COORDS.iter().enumerate() {
            if ai == bi {
                continue; // move-back must be a REAL state change
            }
            for &(d0, d1) in &DS {
                let mut f = FastPathMap::new();
                let mut r = ReferenceMap::new();
                // init at A (from MIN -> guard), real move to B, move back,
                // then 3 exact no-op writes, then grow/shrink in place
                let seq = [
                    (ax, az, d0),
                    (bx, bz, d1),
                    (ax, az, d0),
                    (bx, bz, d1),
                    (bx, bz, d1),
                    (bx, bz, d1),
                    (bx, bz, d1),
                    (bx, bz, (d1 * 2).min(32)),
                    (bx, bz, d1),
                ];
                for (i, &(x, z, d64)) in seq.iter().enumerate() {
                    let d = d64 as i32;
                    f.update(x, z, d);
                    r.update(x, z, d);
                    if f.tracked != r.tracked {
                        let cell = first_set_mismatch(&f.tracked, &r.tracked).unwrap();
                        parity_fail!(
                            "edge: set mismatch coords=({ax},{az})->({bx},{bz}) \
                             d=({d0},{d1}) step#{i} chunk={cell:?}"
                        );
                    }
                    for (wi, win) in wins.iter().enumerate() {
                        if let Some((row, col, x2, z2)) = first_grid_mismatch(&f.tracked, &r.tracked, win) {
                            parity_fail!(
                                "edge: grid mismatch win#{wi} coords=({ax},{az})->({bx},{bz}) \
                                 d=({d0},{d1}) step#{i} first_differing_index=(row {row}, col {col}) \
                                 chunk=({x2},{z2})"
                            );
                        }
                    }
                }
                // Exactly the 3 no-op writes must hit the fast path; when
                // d1 == 0 the "grow"/"shrink" steps degenerate to the same
                // state and count too (+2). BUT the shipped MIN_VALUE guard
                // outranks the fast path: any step whose FROM state has
                // x == i32::MIN is a guard hit (no ops, no fast path) — a
                // real semantic of the patched run() that both worlds share
                // (parity holds; only the counter expectation changes).
                let grow_d = (d1 * 2).min(32);
                let expected_hits = if bx == i32::MIN {
                    0
                } else {
                    3 + if grow_d == d1 { 2 } else { 0 }
                };
                assert!(
                    f.fast_path_hits == expected_hits,
                    "edge: fast_path_hits={} != {expected_hits} for ({ax},{az})->({bx},{bz}) d=({d0},{d1})",
                    f.fast_path_hits
                );
                sequences += 1;
            }
        }
    }
    println!("fuzz_edge_coordinates: {sequences} deterministic sequences — ALL PASS (pack/decode + seam parity + fast-path counters)");
}
