//! NCF bench — the ГЕЙТ P2 line-2 measurement: pure noise-stage generation
//! (density + aquifer + ore veins + biome fill + section content — everything
//! the NOISE status oracle covers), NO NBT writing, single core.
//!
//! Java reference (P0.1 profile, warm, per chunk): noise+аквиферы+ore veins
//! cluster = 248.1 CPU-ms => the >=30x gate needs <= 8.27 ms/chunk native.
//!
//! Usage: bench [seed] [chunks] [--surface] [--carvers]
//!        bench [seed] [chunks] ledger   (SPEED LEDGER: per-stage ms/chunk,
//!         warm, fixed sqrt-N chunk corpus -> docs/NCF_SPEED.md history row)

use chunk_factory::filler::generate_noise_chunk;
use chunk_factory::fullchunk::{full_chunk_nbt, DATA_VERSION_1_21_10};
use chunk_factory::router::{RandomState, WorldgenDir};
use chunk_factory::status_chain::{generate_carvers_chunk, generate_surface_chunk, StageKit};
use chunk_factory::status_chain::{apply_carvers_pass, apply_surface_pass};
use std::path::Path;
use std::time::Instant;

/// P0.1 vanilla Java warm CPU-ms/chunk (jcmd+JFR, Purpur 1.21.10-2535, seed
/// 3053459, burst 128, 2 vCPU) — the per-stage shares for Amdahl.
const JAVA_WARM_NOISE_CLUSTER: f64 = 248.1 + 25.7; // noise+aquifers+veins + biomes (filler covers both)
const JAVA_WARM_SURFACE: f64 = 16.8;
const JAVA_WARM_CARVERS: f64 = 1.6;
const JAVA_WARM_NBT: f64 = 2.6; // NBT+compression (our ledger stage = NBT build only, no zlib)
const JAVA_WARM_TOTAL: f64 = 363.0;
const JAVA_WARM_UNPORTED: f64 = 4.8 + 4.3 + 9.1 + 48.9; // features+light+scheduler+jvm_other at 1x

#[allow(clippy::too_many_lines)]
fn run_ledger(seed: i64, chunks: usize, dir: &WorldgenDir) {
    let side = (chunks as f64).sqrt().ceil() as i32; // 169 -> 13x13 fixed corpus
    let t0 = Instant::now();
    let mut rs = RandomState::build(dir, "minecraft", "overworld", seed).expect("random state");
    let mut kit = StageKit::build(&mut rs, dir).expect("stage kit");
    let setup = t0.elapsed();

    // warmup: full chain on throwaway coords outside the corpus window
    for i in 0..3i32 {
        let mut c = chunk_factory::filler::generate_noise_chunk_with_beardifier(
            &rs,
            seed,
            500 + i,
            500,
            kit.beard.clone(),
        )
        .expect("warm noise");
        apply_surface_pass(&mut rs, &mut kit, dir, seed, &mut c).expect("warm surface");
        apply_carvers_pass(&mut rs, &mut kit, dir, seed, &mut c).expect("warm carvers");
        let nbt = full_chunk_nbt(&c, DATA_VERSION_1_21_10);
        std::hint::black_box(&nbt);
    }

    let mut acc = [0f64; 4]; // noise, surface, carvers, serialization
    let mut n = 0usize;
    let t_all = Instant::now();
    'corpus: for cx in 0..side {
        for cz in 0..side {
            if n >= chunks {
                break 'corpus;
            }
            let t_n0 = Instant::now();
            let mut chunk = chunk_factory::filler::generate_noise_chunk_with_beardifier(
                &rs,
                seed,
                cx,
                cz,
                kit.beard.clone(),
            )
            .expect("noise");
            let t_n1 = Instant::now();
            apply_surface_pass(&mut rs, &mut kit, dir, seed, &mut chunk).expect("surface");
            let t_s1 = Instant::now();
            apply_carvers_pass(&mut rs, &mut kit, dir, seed, &mut chunk).expect("carvers");
            let t_c1 = Instant::now();
            let nbt = full_chunk_nbt(&chunk, DATA_VERSION_1_21_10);
            let t_s2 = Instant::now();
            std::hint::black_box(&nbt);
            acc[0] += (t_n1 - t_n0).as_secs_f64() * 1e3;
            acc[1] += (t_s1 - t_n1).as_secs_f64() * 1e3;
            acc[2] += (t_c1 - t_s1).as_secs_f64() * 1e3;
            acc[3] += (t_s2 - t_c1).as_secs_f64() * 1e3;
            n += 1;
        }
    }
    let wall = t_all.elapsed().as_secs_f64() * 1e3;
    let per: Vec<f64> = acc.iter().map(|a| a / n as f64).collect();
    let ported_total: f64 = per.iter().sum();

    // stage speedups vs the Java warm P0.1 budget (stage-to-stage)
    let sp_noise = JAVA_WARM_NOISE_CLUSTER / per[0];
    let sp_surface = JAVA_WARM_SURFACE / per[1];
    let sp_carvers = JAVA_WARM_CARVERS / per[2];
    let sp_nbt = JAVA_WARM_NBT / per[3];
    let share = |x: f64| 100.0 * x / JAVA_WARM_TOTAL;
    let sum_terms = share(JAVA_WARM_NOISE_CLUSTER) / sp_noise
        + share(JAVA_WARM_SURFACE) / sp_surface
        + share(JAVA_WARM_CARVERS) / sp_carvers
        + share(JAVA_WARM_NBT) / sp_nbt;
    let share_unported = 100.0 * JAVA_WARM_UNPORTED / JAVA_WARM_TOTAL;
    let amdahl = 100.0 / (sum_terms + share_unported);

    println!(
        "ledger[seed={seed} chunks={n} ({side}x{side}), warm, single-core, setup {setup:.3?}]:"
    );
    println!("  stage            ms/chunk  share_of_java  vs_java_warm");
    println!(
        "  noise+biomes     {:8.2}  {:>5.1}%         {:>6.1}x  (P0.1 {} CPU-ms)",
        per[0],
        share(JAVA_WARM_NOISE_CLUSTER),
        sp_noise,
        JAVA_WARM_NOISE_CLUSTER
    );
    println!(
        "  surface          {:8.2}  {:>5.1}%         {:>6.1}x  (P0.1 {JAVA_WARM_SURFACE} CPU-ms)",
        per[1],
        share(JAVA_WARM_SURFACE),
        sp_surface
    );
    println!(
        "  carvers          {:8.2}  {:>5.1}%         {:>6.1}x  (P0.1 {JAVA_WARM_CARVERS} CPU-ms)",
        per[2],
        share(JAVA_WARM_CARVERS),
        sp_carvers
    );
    println!(
        "  serialization    {:8.2}  {:>5.1}%         {:>6.1}x  (P0.1 {JAVA_WARM_NBT} CPU-ms, NBT build only)",
        per[3],
        share(JAVA_WARM_NBT),
        sp_nbt
    );
    println!("  PORTED TOTAL     {ported_total:8.2}  (chunks/s/core {:.1})", n as f64 / (wall / 1e3));
    println!(
        "Amdahl: unported (features+light+scheduler+jvm_other) = {JAVA_WARM_UNPORTED} CPU-ms = {share_unported:.2}% at 1x"
    );
    println!("  pregen speedup = 1 / sum(share_i/speedup_i) = {amdahl:.2}x vs Java warm 363.0 CPU-ms/chunk");
    println!("  wall sanity: {n} chunks in {wall:.1} ms (sum-of-stages {:.1} ms)", ported_total * n as f64);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3053459);
    let chunks: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(64);
    let status = args.get(3).map(|s| s.as_str()).unwrap_or("noise");

    // a single 8x8 chunk block (the Java bench uses 128-chunk bursts)
    let side = (chunks as f64).sqrt().ceil() as i32;
    let wg = std::env::var("NCF_WG").unwrap_or_else(|_| "/tmp/wg-extract".to_string());
    let dir = WorldgenDir::load(Path::new(&wg)).expect("worldgen extract");
    if status == "ledger" {
        run_ledger(seed, chunks, &dir);
        return;
    }
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
