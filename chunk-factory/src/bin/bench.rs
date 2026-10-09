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
//!        bench [seed] region            (P3.2/P3.4 END-TO-END REGION: I8
//!         prescan -> decide_chunk -> native chain -> .mca writer; coverage %
//!         + ms/chunk for the native lane, projected pregen vs pure-Java)

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

    #[cfg(ncf_profile)]
    let s1_before =
        chunk_factory::biomes::S1_VOTE_CALLS.load(std::sync::atomic::Ordering::Relaxed);
    // S2 probe snapshot (after warmup, before the corpus)
    #[cfg(ncf_profile)]
    let s2_before = (
        chunk_factory::surface_rules::S2_NANOS_PASS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_TOTAL.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_VOTE.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_BADLANDS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_YLOOP.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_RULE.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_NANOS_FROZEN.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_AIR_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_FLUID_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_STONE_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_TRY_APPLIES.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2_SET_BLOCKS.load(std::sync::atomic::Ordering::Relaxed),
    );

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

    // S1 probe (standing order): biome-vote calls over the ledger corpus.
    #[cfg(ncf_profile)]
    if std::env::var("NCF_S1_PROBE").is_ok() {
        let calls = chunk_factory::biomes::S1_VOTE_CALLS
            .load(std::sync::atomic::Ordering::Relaxed)
            - s1_before;
        eprintln!(
            "[S1-probe] get_biome_voted_region: {calls} calls / {n} ledger chunks = {} per chunk (Java memoizes: <= 256 needed)",
            calls / (n as u64).max(1)
        );
    }
    // S2 probe (standing order R5): surface-stage phase split over the corpus.
    #[cfg(ncf_profile)]
    if std::env::var("NCF_S2_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        use chunk_factory::surface_rules as s;
        let (
            p_pass, p_total, p_vote, p_bad, p_yloop, p_rule, p_frozen,
            c_air, c_fluid, c_stone, c_try, c_set,
        ) = s2_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let ms = |a: u64| a as f64 / n as f64 / 1e6;
        let pass_ms = ms(g(&s::S2_NANOS_PASS, p_pass));
        let total_ms = ms(g(&s::S2_NANOS_TOTAL, p_total));
        let vote = ms(g(&s::S2_NANOS_VOTE, p_vote));
        let bad = ms(g(&s::S2_NANOS_BADLANDS, p_bad));
        let yloop = ms(g(&s::S2_NANOS_YLOOP, p_yloop));
        let rule = ms(g(&s::S2_NANOS_RULE, p_rule));
        let frozen = ms(g(&s::S2_NANOS_FROZEN, p_frozen));
        eprintln!(
            "[S2-probe] surface split ms/chunk: pass={pass_ms:.3} build={total_ms:.3} \
             vote={vote:.3} badlands={bad:.3} yloop={yloop:.3} (rule nested={rule:.3}) frozen={frozen:.3} \
             | yloop_excl={:.3} other={:.3} kit_prep={:.3} \
             | counts/chunk: air={} fluid={} stone={} try_apply={} set_block={}",
            yloop - rule,
            total_ms - vote - bad - yloop - frozen,
            pass_ms - total_ms,
            (g(&s::S2_AIR_CALLS, c_air)) / (n as u64).max(1),
            (g(&s::S2_FLUID_CALLS, c_fluid)) / (n as u64).max(1),
            (g(&s::S2_STONE_CALLS, c_stone)) / (n as u64).max(1),
            (g(&s::S2_TRY_APPLIES, c_try)) / (n as u64).max(1),
            (g(&s::S2_SET_BLOCKS, c_set)) / (n as u64).max(1),
        );
    }
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

/// P3.2/P3.4 END-TO-END REGION (owner work list item 3): one fixed region
/// file — 32x32 = 1024 chunks, seed 3053459 — driven through the I8 fallback
/// law: structure_scan prescan -> decide_chunk (THE decision point) ->
/// native chain (noise -> surface -> carvers -> FULL NBT -> gzip) ->
/// region.rs .mca writer. Fallback chunks are NOT generated here (deployment
/// shape: Moonrise NO_DATA -> Java generates them whole). Reports the
/// mandatory coverage % (I8), per-stage + end-to-end ms/chunk for the native
/// lane, and the projected pregen time vs pure-Java P0.1 warm numbers
/// (unported stages at 1x — the same Amdahl model as the ledger).
fn run_region(seed: i64, dir: &WorldgenDir, root: &Path) {
    const REGION_SIDE: i32 = 32; // one .mca: 32x32 chunk columns
    // Region origin in chunk coords (NCF_REGION_BASE, both axes; default 0
    // = r.0.0.mca). -32 -> r.-1.-1.mca (chunk coords -32..-1).
    let base = std::env::var("NCF_REGION_BASE")
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let (rx, rz) = (base >> 5, base >> 5);
    let t0 = Instant::now();
    let mut rs = RandomState::build(dir, "minecraft", "overworld", seed).expect("random state");
    let mut kit = StageKit::build(&mut rs, dir).expect("stage kit");
    let setup = t0.elapsed();

    // I8 prescan (the gate-p2 honest-exclusion mechanism): chunks the unported
    // Beardifier could touch are Java's, not ours.
    let scan_t0 = Instant::now();
    let scan = chunk_factory::structure_scan::scan_batch(
        dir,
        &rs,
        seed,
        root,
        base,
        base + REGION_SIDE - 1,
        base,
        base + REGION_SIDE - 1,
    )
    .expect("fallback prescan");
    let scan_ms = scan_t0.elapsed().as_secs_f64() * 1e3;

    // warmup: full chain incl. gzip on throwaway coords outside the corpus
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
        let gz =
            chunk_factory::sections::write_gzipped_nbt(&full_chunk_nbt(&c, DATA_VERSION_1_21_10));
        std::hint::black_box(&gz);
    }

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as u32)
        .unwrap_or(0);
    let mut acc = [0f64; 5]; // noise, surface, carvers, nbt, gzip
    let mut region_chunks: Vec<chunk_factory::region::RegionChunk> = Vec::new();
    let mut ledger = chunk_factory::fallback::CoverageLedger::default();
    let t_all = Instant::now();
    for cx in base..base + REGION_SIDE {
        for cz in base..base + REGION_SIDE {
            let has_structures = scan.chunks.contains(&(cx, cz));
            // THE decision point (fallback.rs): the only lawful switch.
            let d =
                chunk_factory::fallback::decide_chunk(false, has_structures, false, false, false, false);
            if !d.native {
                // deployment shape: Moonrise NO_DATA -> Java generates the
                // chunk whole; we neither time nor write it here.
                ledger.record_fallback(d.reason);
                continue;
            }
            ledger.record_native();
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
            let t_s1b = Instant::now();
            let gz = chunk_factory::sections::write_gzipped_nbt(&nbt);
            let t_s2 = Instant::now();
            region_chunks.push(chunk_factory::region::RegionChunk {
                x_in_region: (cx - base) as u8,
                z_in_region: (cz - base) as u8,
                timestamp: ts,
                format: 1, // gzip (NbtIo.writeCompressed stream, verbatim)
                data: gz,
            });
            acc[0] += (t_n1 - t_n0).as_secs_f64() * 1e3;
            acc[1] += (t_s1 - t_n1).as_secs_f64() * 1e3;
            acc[2] += (t_c1 - t_s1).as_secs_f64() * 1e3;
            acc[3] += (t_s1b - t_c1).as_secs_f64() * 1e3;
            acc[4] += (t_s2 - t_s1b).as_secs_f64() * 1e3;
        }
    }
    let gen_wall = t_all.elapsed().as_secs_f64() * 1e3;
    let t_w0 = Instant::now();
    let bytes = chunk_factory::region::write_region(&region_chunks).expect("write region");
    let write_ms = t_w0.elapsed().as_secs_f64() * 1e3;
    let parsed = chunk_factory::region::parse_region(&bytes).expect("parse region");
    let roundtrip = parsed.chunks.len();

    let native = ledger.report.native_chunks as f64;
    let fallback = ledger.report.java_fallback_chunks as f64;
    // reporter invariant (coverage.rs): native + fallback == total
    ledger.report.total_chunks = ledger.report.native_chunks + ledger.report.java_fallback_chunks;
    ledger.report.by_step = vec![("carvers-native", ledger.report.native_chunks, ledger.report.java_fallback_chunks)];
    assert!(native > 0.0, "no native chunks in corpus — nothing to measure");
    let per: Vec<f64> = acc.iter().map(|a| a / native).collect();
    let write_amort = write_ms / native;
    let e2e: f64 = per.iter().sum::<f64>() + write_amort;
    let java_ported = JAVA_WARM_NOISE_CLUSTER + JAVA_WARM_SURFACE + JAVA_WARM_CARVERS + JAVA_WARM_NBT;

    // persist the region so a server boot can consume it (P3.1 ladder)
    let out_dir =
        std::env::var("NCF_REGION_OUT").unwrap_or_else(|_| "region-bench-out".to_string());
    std::fs::create_dir_all(&out_dir).expect("out dir");
    let out_path = format!("{out_dir}/r.{rx}.{rz}.mca");
    std::fs::write(&out_path, &bytes).expect("write .mca");

    println!(
        "region[seed={seed} r.{rx}.{rz}.mca chunks {base}..{}x{base}..{} = 1024, warm, single-core, setup {setup:.3?}, prescan {scan_ms:.0} ms]:",
        base + REGION_SIDE - 1,
        base + REGION_SIDE - 1,
    );
    println!(
        "  I8 prescan: {} start(s) -> {} fallback chunk(s) (Beardifier law, Java lane)",
        scan.starts.len(),
        ledger.report.java_fallback_chunks
    );
    for s in scan.starts.iter().take(8) {
        println!(
            "    fallback start: {} ({}) cx={} cz={}{}",
            s.structure,
            s.set,
            s.cx,
            s.cz,
            if s.unknown_validity { " unknown-validity" } else { "" }
        );
    }
    for (k, v) in &ledger.reasons {
        println!("    fallback[{k}] = {v}");
    }
    println!("  coverage: {}", ledger.report.summary());
    println!("  native stage        ms/chunk");
    println!("    noise+biomes      {:8.2}", per[0]);
    println!("    surface           {:8.2}", per[1]);
    println!("    carvers           {:8.2}", per[2]);
    println!("    serialization     {:8.2}  (FULL NBT build)", per[3]);
    println!("    gzip payload      {:8.2}  (writeCompressed form)", per[4]);
    println!(
        "    region write      {write_amort:8.2}  (amortized, {} chunks)",
        region_chunks.len()
    );
    println!(
        "  NATIVE E2E        {e2e:8.2} ms/chunk ({:.1} chunks/s/core) vs Java warm ported stages {java_ported:.1} CPU-ms: {:.1}x",
        1000.0 / e2e,
        java_ported / e2e,
    );
    // Honest pregen projection (P0.1 CPU-ms shares, Purpur 2535, 2 vCPU):
    //   native chunk = Rust e2e (measured) + Java completion of unported
    //                  stages at 1x (features+light+scheduler+jvm_other)
    //   fallback     = Java's full pipeline (P0.1 warm total)
    let hybrid_ms = native * (e2e + JAVA_WARM_UNPORTED) + fallback * JAVA_WARM_TOTAL;
    let pure_java_ms = (native + fallback) * JAVA_WARM_TOTAL;
    println!(
        "  projected pregen (P0.1 shares; unported {:.1} CPU-ms at 1x on Java):",
        JAVA_WARM_UNPORTED
    );
    println!(
        "    hybrid {:.0} s vs pure-Java {:.0} s -> {:.2}x end-to-end pregen speedup",
        hybrid_ms / 1e3,
        pure_java_ms / 1e3,
        pure_java_ms / hybrid_ms,
    );
    println!(
        "  wall sanity: {} native chunks in {gen_wall:.1} ms (sum-of-stages {:.1} ms); region {} bytes, parse roundtrip {roundtrip}/{} -> {}",
        region_chunks.len(),
        e2e * native,
        bytes.len(),
        region_chunks.len(),
        out_path,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3053459);
    let chunks: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(64);
    let status = args.get(3).map(|s| s.as_str()).unwrap_or("noise");

    // a single 8x8 chunk block (the Java bench uses 128-chunk bursts)
    let side = (chunks as f64).sqrt().ceil() as i32;
    let wg = std::env::var("NCF_WG").unwrap_or_else(|_| "/tmp/wg-extract".to_string());
    // carver-tag expansion (StageKit) and the structure prescan read files
    // relative to the extract root via NCF_DATA_ROOT (same pattern as
    // stagediff): default it to the NCF_WG extract.
    if std::env::var_os("NCF_DATA_ROOT").is_none() {
        std::env::set_var("NCF_DATA_ROOT", &wg);
    }
    let dir = WorldgenDir::load(Path::new(&wg)).expect("worldgen extract");
    if status == "ledger" {
        run_ledger(seed, chunks, &dir);
        return;
    }
    // `bench <seed> region` (two-arg form) or `bench <seed> <chunks> region`:
    // "region" may sit in the chunks slot, so match it by position OR value.
    if status == "region" || args.get(2).map(|s| s.as_str()) == Some("region") {
        run_region(seed, &dir, Path::new(&wg));
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
