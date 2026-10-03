//! noise_ab — same-process paired A/B for the p500 hoist lever (83f29e5)
//! on the recovered paper-native-core normal-noise batch fills.
//!
//! base    = CE snapshot (a4f53bf1), src/normal_noise.rs before lever
//! hoisted = base + hoist patch (bench_ab/normal_noise_hoisted.rs == src after lever)
//!
//! Protocol: per case, 20 warmup iters, then 15 alternating rounds
//! (A,B,A,B,...), each round times K inner iterations; report medians,
//! speedup %, and A/A calibration gap (must stay < 3%).

#[path = "../../bench_ab/normal_noise_base.rs"]
mod base;

#[path = "../../bench_ab/normal_noise_hoisted.rs"]
mod hoisted;

use paper_native_core::improved_noise::PERMUTATION_LENGTH;
use paper_native_core::perlin_noise::PerlinNoise;
use std::time::Instant;

fn make_noise(octaves: usize) -> PerlinNoise {
    let mut perms = vec![0u8; octaves * PERMUTATION_LENGTH];
    let mut s: u64 = 0x9E3779B97F4A7C15;
    for p in perms.iter_mut() {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *p = (s >> 33) as u8;
    }
    let active = vec![8u8; octaves];
    let y_origins: Vec<f64> = (0..octaves).map(|i| (i as f64) * 4.0).collect();
    let amplitudes: Vec<f64> = (0..octaves).map(|i| 1.0 / (1u64 << i) as f64).collect();
    PerlinNoise::new_from_flat(&perms, &active, &y_origins, &amplitudes, 1.0, 1.0)
        .expect("noise ctor")
}

struct Case {
    name: &'static str,
    n: usize,
}

fn time_round<F: FnMut()>(mut f: F, iters: usize) -> f64 {
    let t = Instant::now();
    for _ in 0..iters {
        f();
    }
    t.elapsed().as_secs_f64() / iters as f64
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let noise = std::sync::Arc::new(make_noise(8));
    let second = std::sync::Arc::new(make_noise(4));
    let cases = [
        Case { name: "fill_scaled_positions", n: 4096 },
        Case { name: "fill_scaled_positions", n: 32768 },
        Case { name: "fill_shifted_positions_in_place", n: 4096 },
        Case { name: "fill_shifted_positions_in_place", n: 32768 },
        Case { name: "fill_shift_positions", n: 4096 },
        Case { name: "fill_shift_positions", n: 32768 },
    ];
    const WARM: usize = 20;
    const ROUNDS: usize = 15;

    println!("case,n,base_med_s,hoist_med_s,speedup_pct,aa_gap_pct");
    let mut overall_ok = true;
    for c in cases {
        let xs: Vec<i32> = (0..c.n).map(|i| (i as i32) & 15).collect();
        let ys: Vec<i32> = (0..c.n).map(|i| ((i >> 4) as i32 & 255) * 4).collect();
        let zs: Vec<i32> = (0..c.n).map(|i| ((i >> 12) as i32) & 15).collect();
        let mut dst = vec![0f64; c.n];
        let mut shift_x_and_dst = vec![0.5f64; c.n];
        let shift_y: Vec<f64> = (0..c.n).map(|i| ((i & 7) as f64) * 0.25).collect();
        let shift_z: Vec<f64> = (0..c.n).map(|i| ((i & 3) as f64) * 0.5).collect();

        let run_base = |dst: &mut Vec<f64>, sx: &mut Vec<f64>| match c.name {
            "fill_scaled_positions" => base::fill_scaled_positions(
                &noise, &second, 1.0, &xs, &ys, &zs, 1.5, 2.0, dst,
            ),
            "fill_shifted_positions_in_place" => base::fill_shifted_positions_in_place(
                &noise, &second, 1.0, sx, &shift_y, &shift_z, &xs, &ys, &zs, 1.5, 2.0,
            ),
            _ => base::fill_shift_positions(&noise, &second, 1.0, &xs, &ys, &zs, dst),
        };
        let run_hoist = |dst: &mut Vec<f64>, sx: &mut Vec<f64>| match c.name {
            "fill_scaled_positions" => hoisted::fill_scaled_positions(
                &noise, &second, 1.0, &xs, &ys, &zs, 1.5, 2.0, dst,
            ),
            "fill_shifted_positions_in_place" => hoisted::fill_shifted_positions_in_place(
                &noise, &second, 1.0, sx, &shift_y, &shift_z, &xs, &ys, &zs, 1.5, 2.0,
            ),
            _ => hoisted::fill_shift_positions(&noise, &second, 1.0, &xs, &ys, &zs, dst),
        };

        // sanity: same outputs (base vs hoisted must be bit-equal on same inputs)
        {
            let mut a = vec![0f64; c.n];
            let mut b = vec![0f64; c.n];
            let mut sa = vec![0.5f64; c.n];
            let mut sb = vec![0.5f64; c.n];
            let _ = run_base(&mut a, &mut sa);
            let _ = run_hoist(&mut b, &mut sb);
            let eq = a == b && sa == sb;
            if !eq {
                println!("# WARN: {} n={}: output mismatch base vs hoisted", c.name, c.n);
                overall_ok = false;
            }
        }

        for _ in 0..WARM {
            let _ = run_base(&mut dst, &mut shift_x_and_dst);
            let _ = run_hoist(&mut dst, &mut shift_x_and_dst);
        }
        let iters = if c.n >= 32768 { 12 } else { 96 };
        let mut bt = Vec::with_capacity(ROUNDS);
        let mut ht = Vec::with_capacity(ROUNDS);
        for r in 0..ROUNDS * 2 {
            if r % 2 == 0 {
                bt.push(time_round(|| {
                    let _ = run_base(&mut dst, &mut shift_x_and_dst);
                }, iters));
            } else {
                ht.push(time_round(|| {
                    let _ = run_hoist(&mut dst, &mut shift_x_and_dst);
                }, iters));
            }
        }
        let b_med = median(&mut bt);
        let h_med = median(&mut ht);
        let speedup = (b_med - h_med) / b_med * 100.0;
        // A/A calibration: split base rounds in halves
        let half = bt.len() / 2;
        let mut b1 = bt[..half].to_vec();
        let mut b2 = bt[half..].to_vec();
        let aa = ((median(&mut b1) - median(&mut b2)).abs() / b_med) * 100.0;
        println!(
            "{},{},{:.9},{:.9},{:+.2},{:.2}",
            c.name, c.n, b_med, h_med, speedup, aa
        );
        if aa >= 3.0 {
            overall_ok = false;
        }
    }
    let verdict = if overall_ok { "OK" } else { "CHECK" };
    println!("# verdict={}", verdict);
}
