//! NCF bench — the ГЕЙТ P2 line-2 measurement: pure noise-stage generation
//! (density + aquifer + ore veins + biome fill + section content — everything
//! the NOISE status oracle covers), NO NBT writing, single core.
//!
//! Java reference (P0.1 profile, warm, per chunk): noise+аквиферы+ore veins
//! cluster = 248.1 CPU-ms => the >=30x gate needs <= 8.27 ms/chunk native.
//!
//! Usage: bench [seed] [chunks] [--surface] [--carvers]

use chunk_factory::filler::generate_noise_chunk;
use chunk_factory::router::{RandomState, WorldgenDir};
use chunk_factory::status_chain::{generate_carvers_chunk, generate_surface_chunk, StageKit};
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3053459);
    let chunks: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(64);
    let status = args.get(3).map(|s| s.as_str()).unwrap_or("noise");

    // a single 8x8 chunk block (the Java bench uses 128-chunk bursts)
    let side = (chunks as f64).sqrt().ceil() as i32;
    let wg = std::env::var("NCF_WG").unwrap_or_else(|_| "/tmp/wg-extract".to_string());
    let dir = WorldgenDir::load(Path::new(&wg)).expect("worldgen extract");
    let t0 = Instant::now();
    let mut rs = RandomState::build(&dir, "minecraft", "overworld", seed).expect("random state");
    let setup = t0.elapsed();
    let mut kit = if status == "noise" {
        None
    } else {
        Some(StageKit::build(&mut rs, &dir).expect("stage kit"))
    };

    // warmup (JIT-free Rust, but page the allocator)
    let _ = generate_noise_chunk(&rs, seed, 1000, 1000).expect("warmup");

    let t1 = Instant::now();
    let mut n = 0usize;
    for cx in 0..side {
        for cz in 0..side {
            if n >= chunks {
                break;
            }
            let r = match (status, kit.as_mut()) {
                ("noise", _) => generate_noise_chunk(&rs, seed, cx, cz),
                ("surface", Some(k)) => generate_surface_chunk(&mut rs, k, &dir, seed, cx, cz),
                ("carvers", Some(k)) => generate_carvers_chunk(&mut rs, k, &dir, seed, cx, cz),
                _ => Err("unsupported status".into()),
            }
            .expect("gen");
            // consume the result so codegen cannot skip the work
            let mut acc = 0u64;
            for s in &r.sections {
                acc = acc.wrapping_add(s.states.iter().fold(0u64, |a, &v| a.wrapping_add(v as u64)));
            }
            std::hint::black_box(acc);
            n += 1;
        }
    }
    let elapsed = t1.elapsed().as_secs_f64();
    let per_chunk_ms = elapsed * 1000.0 / n as f64;
    println!(
        "bench[{status}]: {n} chunks in {elapsed:.3}s = {:.2} ms/chunk single-core (setup {setup:.3?})",
        per_chunk_ms
    );
    println!("  vs Java warm noise-cluster 248.1 CPU-ms/chunk: {:.1}x", 248.1 / per_chunk_ms);
    println!("  chunks/s/core: {:.1}", n as f64 / elapsed);
}
