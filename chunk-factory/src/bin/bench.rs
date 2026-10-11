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
    // S2B probe snapshot (after warmup, before the corpus)
    #[cfg(ncf_profile)]
    let s2b_before = (
        chunk_factory::surface_rules::S2B_NANOS_TRY.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2B_NANOS_INTERN.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2B_NANOS_SET.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2B_INTERN_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2B_BIOMEIS_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::surface_rules::S2B_BIOMEIS_MISS.load(std::sync::atomic::Ordering::Relaxed),
    );

    // N1 probe snapshot (after warmup, before the corpus)
    #[cfg(ncf_profile)]
    let n1_before = (
        chunk_factory::noise::N1_PERLIN_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::noise::N1_PERLIN_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_TILE_HITS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_TILE_MISSES.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_FILL_NODE_VISITS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_FILL_ELEMS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_FILL_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_SLICE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_SLICE_LEAF_YDEP.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_SLICE_LEAF_YFREE.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N1_DRIVE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
    );

    // N2 probe snapshot (after warmup, before the corpus). The per-unit
    // arrays are snapshotted element-wise too — the warmup chunks must not
    // leak into the per-chunk unit table.
    #[cfg(ncf_profile)]
    let n2_before = (
        chunk_factory::interpolator::N2_FILL_CALLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N2_CLONE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N2_SCOPE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N2_MAIN_ROWS_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N2_WORKER_ROWS_NANOS
            .load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N2_MERGE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
    );
    #[cfg(ncf_profile)]
    let n5_fcm_before = (
        chunk_factory::interpolator::N5_FCM_HITS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N5_FCM_MISSES.load(std::sync::atomic::Ordering::Relaxed),
    );
    #[cfg(ncf_profile)]
    let n2_units_before: (
        [u64; chunk_factory::interpolator::N2_UNIT_LEN],
        [u64; chunk_factory::interpolator::N2_UNIT_LEN],
    ) = {
        use std::sync::atomic::Ordering::Relaxed as R;
        let mut nanos = [0u64; chunk_factory::interpolator::N2_UNIT_LEN];
        let mut icd = [0u64; chunk_factory::interpolator::N2_UNIT_LEN];
        for i in 0..chunk_factory::interpolator::N2_UNIT_LEN {
            nanos[i] = chunk_factory::interpolator::N2_UNIT_NANOS[i].load(R);
            icd[i] = chunk_factory::interpolator::N2_UNIT_IC_DELTA[i].load(R);
        }
        (nanos, icd)
    };

    // SUB1 probe snapshot (after warmup, before the corpus) — substance-fill
    // interior split (see the SUB1 block in interpolator.rs).
    #[cfg(ncf_profile)]
    let sub1_before = (
        chunk_factory::interpolator::SUB_SQUEEZE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_ADD_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_MIN_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_MUL_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_MAX_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_RC_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_MULORA_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_BEARD_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_PFD_INTERP_NANOS
            .load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_PFD_OTHER_NANOS
            .load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_NOISE_LEAF_CALLS
            .load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::SUB_CACHEWRAP_CALLS
            .load(std::sync::atomic::Ordering::Relaxed),
        // 12: SUB1 SoA kernel cells (honesty counter; expect 768/chunk ON)
        chunk_factory::interpolator::SUB_SOA_CELLS.load(std::sync::atomic::Ordering::Relaxed),
    );

    // N3 probe snapshot (after warmup, before the corpus) — NP3 pipelined
    // drive overlap split (all zero unless NCF_PAR_FILL=2).
    #[cfg(ncf_profile)]
    let n3_before = (
        chunk_factory::interpolator::N3_HIDDEN_FILLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_SKIP_FILLS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_FORK_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_WALK_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_WORKER_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_JOIN_BLOCK_NANOS
            .load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_MERGE_NANOS.load(std::sync::atomic::Ordering::Relaxed),
        chunk_factory::interpolator::N3_RESIDUAL_NANOS.load(std::sync::atomic::Ordering::Relaxed),
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
    // S2B probe (standing order R5, S4 inner split): rule-hit path nanos.
    #[cfg(ncf_profile)]
    if std::env::var("NCF_S2B_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        use chunk_factory::surface_rules as s;
        let (p_try, p_intern, p_set, c_intern, p_bis, c_miss) = s2b_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let ms = |a: u64| a as f64 / n as f64 / 1e6;
        let t = ms(g(&s::S2B_NANOS_TRY, p_try));
        let i = ms(g(&s::S2B_NANOS_INTERN, p_intern));
        let st = ms(g(&s::S2B_NANOS_SET, p_set));
        let bis = ms(g(&s::S2B_BIOMEIS_NANOS, p_bis));
        eprintln!(
            "[S2B-probe] hit-path split ms/chunk: try={t:.3} intern={i:.3} set={st:.3} \
             (sum={:.3}) | biomeis nested={bis:.3} | counts/chunk: intern_calls={} biomeis_miss={}",
            t + i + st,
            g(&s::S2B_INTERN_CALLS, c_intern) / (n as u64).max(1),
            g(&s::S2B_BIOMEIS_MISS, c_miss) / (n as u64).max(1),
        );
    }
    // N1 probe (standing order R5): perlin-core vs tree-walk split (SIMD gate).
    #[cfg(ncf_profile)]
    if std::env::var("NCF_N1_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        let (c0, n0, th0, tm0, v0, e0, fn0, sl0, yd0, yf0, dr0) = n1_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let calls = g(&chunk_factory::noise::N1_PERLIN_CALLS, c0);
        let nanos = g(&chunk_factory::noise::N1_PERLIN_NANOS, n0);
        let perlin_ms = nanos as f64 / n as f64 / 1e6;
        let noise_ms = per[0];
        let th = g(&chunk_factory::interpolator::N1_TILE_HITS, th0);
        let tm = g(&chunk_factory::interpolator::N1_TILE_MISSES, tm0);
        let visits = g(&chunk_factory::interpolator::N1_FILL_NODE_VISITS, v0);
        let elems = g(&chunk_factory::interpolator::N1_FILL_ELEMS, e0);
        let fill_ms = g(&chunk_factory::interpolator::N1_FILL_NANOS, fn0) as f64 / n as f64 / 1e6;
        let slice_ms = g(&chunk_factory::interpolator::N1_SLICE_NANOS, sl0) as f64 / n as f64 / 1e6;
        let ydep = g(&chunk_factory::interpolator::N1_SLICE_LEAF_YDEP, yd0);
        let yfree = g(&chunk_factory::interpolator::N1_SLICE_LEAF_YFREE, yf0);
        let drive_ms = g(&chunk_factory::interpolator::N1_DRIVE_NANOS, dr0) as f64 / n as f64 / 1e6;
        eprintln!(
            "[N1-probe] perlin-core {perlin_ms:.3} ms/chunk of noise-stage {noise_ms:.3} = {:.1}% | calls/chunk = {} | tree-walk+biomes+rest = {:.3} ms/chunk (probe clock-pairs inflate slices)",
            100.0 * perlin_ms / noise_ms.max(1e-9),
            calls / (n as u64).max(1),
            (noise_ms - perlin_ms).max(0.0),
        );
        eprintln!(
            "[N1-probe] substance(final_density) fill = {fill_ms:.3} ms/chunk (COARSE clock, no inflation) | visits/chunk = {} elems/chunk = {} | y-free tiles: hit = {} miss = {} (rate = {:.1}%)",
            visits / (n as u64).max(1),
            elems / (n as u64).max(1),
            th / (n as u64).max(1),
            tm / (n as u64).max(1),
            100.0 * th as f64 / (th + tm).max(1) as f64,
        );
        eprintln!(
            "[N1-probe] slice fills = {slice_ms:.3} ms/chunk COARSE | drive_blocks total = {drive_ms:.3} ms/chunk COARSE (drive includes substance+slice fills, subtract) | slice leaf calls/chunk: ydep = {} yfree = {} (ydep share = {:.1}%)",
            ydep / (n as u64).max(1),
            yfree / (n as u64).max(1),
            100.0 * ydep as f64 / (ydep + yfree).max(1) as f64,
        );
    }
    // SUB1 probe (R5): exclusive-time interior split of the substance fill.
    // Scopes tile the N1_FILL_NANOS envelope pairwise-disjointly; the
    // Beardifier pfd is the zero-lerp dispatch-floor control for the MulOrAdd
    // (interp0 trilerp) scope. Verdict arithmetic vs the 1.2 ms worklist gate
    // happens offline (scale s = clean/probe-build noise stage).
    #[cfg(ncf_profile)]
    if std::env::var("NCF_SUB1_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        use chunk_factory::interpolator as ip;
        let (sq0, ad0, mn0, mu0, mx0, rc0, mo0, bd0, pi0, po0, nl0, cw0, sc0) = sub1_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let ms = |nanos: u64| nanos as f64 / n as f64 / 1e6;
        let sq = g(&ip::SUB_SQUEEZE_NANOS, sq0);
        let ad = g(&ip::SUB_ADD_NANOS, ad0);
        let mn = g(&ip::SUB_MIN_NANOS, mn0);
        let mu = g(&ip::SUB_MUL_NANOS, mu0);
        let mx = g(&ip::SUB_MAX_NANOS, mx0);
        let rc = g(&ip::SUB_RC_NANOS, rc0);
        let mo = g(&ip::SUB_MULORA_NANOS, mo0);
        let bd = g(&ip::SUB_BEARD_NANOS, bd0);
        let pi = g(&ip::SUB_PFD_INTERP_NANOS, pi0);
        let po = g(&ip::SUB_PFD_OTHER_NANOS, po0);
        let nl = g(&ip::SUB_NOISE_LEAF_CALLS, nl0);
        let cw = g(&ip::SUB_CACHEWRAP_CALLS, cw0);
        let env = g(&ip::N1_FILL_NANOS, n1_before.6);
        let elems = g(&ip::N1_FILL_ELEMS, n1_before.5);
        let env_ms = ms(env);
        let sum_ms = ms(sq + ad + mn + mu + mx + rc + mo + bd + pi + po);
        let arith_high = ms(sq + ad + mn + mu + mx + rc + mo + pi + po);
        let arith_low =
            ms(sq + ad + mu + mx + rc + mn.saturating_sub(bd) + mo.saturating_sub(bd));
        eprintln!(
            "[SUB1-probe] interior split ms/chunk: envelope E = {env_ms:.3} | squeeze = {:.3} add = {:.3} min_loop = {:.3} | mulora_pfd = {:.3} beard_pfd = {:.3} (floor ctrl) | interp_pfd = {:.3} pfd_other = {:.3} | mul/max/rc = {:.3}/{:.3}/{:.3}",
            ms(sq),
            ms(ad),
            ms(mn),
            ms(mo),
            ms(bd),
            ms(pi),
            ms(po),
            ms(mu),
            ms(mx),
            ms(rc),
        );
        eprintln!(
            "[SUB1-probe] counts/chunk: noise_leaves = {} (expect 0) cachewraps = {} (expect 0) elems = {} | beard floor = {:.1} ns/elem | cross-check: sum(scopes) = {sum_ms:.3} vs E = {env_ms:.3} (residual = {:.3} = {:.1}%)",
            nl / (n as u64).max(1),
            cw / (n as u64).max(1),
            elems / (n as u64).max(1),
            bd as f64 / (elems as f64).max(1.0),
            (env_ms - sum_ms).max(0.0),
            100.0 * (env_ms - sum_ms).max(0.0) / env_ms.max(1e-9),
        );
        eprintln!(
            "[SUB1-probe] ARITH_HIGH = {arith_high:.3} ARITH_LOW = {arith_low:.3} ms/chunk PROBE-BUILD scale (mulora+min_loop hold the trilerp mass; scale s = clean/probe before gate 1.200 worklist / 0.834 = 5% of mode0 noise 16.68)",
        );
        // SUB1 SoA honesty: cells through the kernel (768/chunk when
        // NCF_SUB1_SOA=1 and the detector matched; 0 = generic path ran —
        // SUB_* scopes then tile the envelope as before).
        let soa_cells = g(&ip::SUB_SOA_CELLS, sc0);
        eprintln!(
            "[SUB1-probe] soa_cells = {} (expect 768 x chunks when NCF_SUB1_SOA=1 + detector matched; 0 = generic path)",
            soa_cells / (n as u64).max(1),
        );
    }
    // N2 probe (NP2 go/no-go, standing order R5): parallel fill_slice split.
    // NOTE the approximation: main/worker times are GLOBAL atomics — the
    // per-call split is not captured, so imbalance is the per-chunk estimate
    // |main_total - worker_total| / 2 (labeled as such below).
    #[cfg(ncf_profile)]
    if std::env::var("NCF_N2_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        use chunk_factory::interpolator as ip;
        let (fc0, cl0, sc0, mr0, wr0, mg0) = n2_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let ms = |nanos: u64| nanos as f64 / n as f64 / 1e6;
        let fill_calls = g(&ip::N2_FILL_CALLS, fc0);
        let clone_ms = ms(g(&ip::N2_CLONE_NANOS, cl0));
        let scope_ms = ms(g(&ip::N2_SCOPE_NANOS, sc0));
        let main_ms = ms(g(&ip::N2_MAIN_ROWS_NANOS, mr0));
        let worker_ms = ms(g(&ip::N2_WORKER_ROWS_NANOS, wr0));
        let merge_ms = ms(g(&ip::N2_MERGE_NANOS, mg0));
        // fixed = clone + (scope - max(main, worker)): spawn/join + protocol
        // overhead (the merge block is inside scope and NOT subtracted).
        let fixed_ms = clone_ms + (scope_ms - main_ms.max(worker_ms));
        // imbalance loss, per-chunk APPROXIMATION (see header note).
        let imbalance_ms = (main_ms - worker_ms).abs() / 2.0;
        eprintln!(
            "[N2-probe] parallel fill_slice ms/chunk (APPROX: imbalance from global atomics, per-call split not captured): fill_calls = {} (expect 5) | clone = {clone_ms:.3} scope = {scope_ms:.3} main_rows = {main_ms:.3} worker_rows = {worker_ms:.3} merge = {merge_ms:.3}",
            fill_calls / (n as u64).max(1),
        );
        eprintln!(
            "[N2-probe] fixed (clone + scope - max(main,worker), incl. merge) = {fixed_ms:.3} ms/chunk | imbalance_loss (approx |m-w|/2) = {imbalance_ms:.3} ms/chunk | m/w totals = {main_ms:.3}/{worker_ms:.3} | |m-w|/(m+w) = {:.3}",
            (main_ms - worker_ms).abs() / (main_ms + worker_ms).max(1e-9),
        );
        // NP5 R5 probe: FlatCacheW hit/miss split (miss = full inner recompute,
        // no store). Misses ~ 0 on slice fills => window-geometry lever dead,
        // interp[0] mass is all in-window work (dispatch+splines+noodle).
        {
            let fcm_h = g(&ip::N5_FCM_HITS, n5_fcm_before.0);
            let fcm_m = g(&ip::N5_FCM_MISSES, n5_fcm_before.1);
            eprintln!(
                "[N5-probe] FlatCacheW visits/chunk = {} (hit {} / miss {}; miss share = {:.1}%)",
                (fcm_h + fcm_m) / (n as u64).max(1),
                fcm_h / (n as u64).max(1),
                fcm_m / (n as u64).max(1),
                100.0 * fcm_m as f64 / (fcm_h + fcm_m).max(1) as f64,
            );
        }
        if ip::n2_unit_probe_enabled() {
            let interps = ip::N2_INTERPS_LEN.load(R) as usize;
            if interps == 0 {
                eprintln!("[N2-probe] unit table: no unit-probed fill ran (interps unknown)");
            } else {
                let mut per_interp = [0u64; ip::N2_UNIT_LEN];
                let mut sum_nanos = 0u64;
                let mut ic_min = u64::MAX;
                let mut ic_max = 0u64;
                let mut i0_min = u64::MAX;
                let mut i0_max = 0u64;
                let mut rows_seen = 0usize;
                for idx in 0..ip::N2_UNIT_LEN {
                    let nanos = ip::N2_UNIT_NANOS[idx].load(R) - n2_units_before.0[idx];
                    if nanos == 0 {
                        continue;
                    }
                    let (row_i, id) = (idx / interps, idx % interps);
                    rows_seen = rows_seen.max(row_i + 1);
                    sum_nanos += nanos;
                    if id < ip::N2_UNIT_LEN {
                        per_interp[id] += nanos;
                    }
                    if id == 0 {
                        i0_min = i0_min.min(nanos);
                        i0_max = i0_max.max(nanos);
                    }
                    let icd = ip::N2_UNIT_IC_DELTA[idx].load(R) - n2_units_before.1[idx];
                    ic_min = ic_min.min(icd);
                    ic_max = ic_max.max(icd);
                }
                let sum_ms = sum_nanos as f64 / n as f64 / 1e6;
                let share = |v: u64| {
                    100.0 * v as f64 / sum_nanos.max(1) as f64
                };
                let interp_list: String = per_interp
                    .iter()
                    .take(interps.min(ip::N2_UNIT_LEN))
                    .enumerate()
                    .map(|(id, &v)| format!("[{id}]={:.3}({:.1}%)", v as f64 / n as f64 / 1e6, share(v)))
                    .collect::<Vec<_>>()
                    .join(" ");
                eprintln!(
                    "[N2-probe] unit table (interps = {interps}, rows = {rows_seen}, per-chunk ms/share of unit-clock sum): {interp_list}",
                );
                eprintln!(
                    "[N2-probe] unit table: interp[0] share = {:.1}% | interp[0] per-row spread = {}..{} ns | unit ic-delta (units with time>0) = {}..{} | sum of unit clocks = {sum_ms:.3} ms/chunk",
                    share(per_interp[0]),
                    i0_min, i0_max, ic_min, ic_max,
                );
                // NP4 go/no-go (R5): per-row fill cost (all interps of one row),
                // per fill call. Row-0 total f0 gates the parent-assist: with
                // rows=5 and F-W = (worker-walk)/3 per column, p=1 gain =
                // min(f0, (F-W)-f0) per hidden fill — GO iff 3*(0.795-f0) >= 0.70.
                if fill_calls > 0 {
                    let mut row_line = String::new();
                    for r in 0..rows_seen {
                        let mut row_total = 0u64;
                        for id in 0..interps.min(ip::N2_UNIT_LEN) {
                            let idx = r * interps + id;
                            if idx < ip::N2_UNIT_LEN {
                                row_total += ip::N2_UNIT_NANOS[idx].load(R)
                                    - n2_units_before.0[idx];
                            }
                        }
                        row_line.push_str(&format!(
                            "row[{r}]={:.3}",
                            row_total as f64 / fill_calls as f64 / 1e6
                        ));
                        row_line.push(' ');
                    }
                    eprintln!("[N2-probe] unit per-row ms/fill (all interps, unit-clock incl. overhead): {row_line}");
                }
            }
        }
    }
    // N3 probe (NP3 pipelined drive, standing order R5): fill/walk overlap.
    // hidden fills run on the worker while the parent walks the column two
    // behind; overlap efficiency and the exposed residual (walk time NOT
    // covered by a concurrent worker fill — exact per-scope sum) are the
    // NP3 decision payload.
    #[cfg(ncf_profile)]
    if std::env::var("NCF_N3_PROBE").is_ok() {
        use std::sync::atomic::Ordering::Relaxed as R;
        use chunk_factory::interpolator as ip;
        let (hf0, sk0, fk0, wk0, wrk0, jb0, mg0, rs0) = n3_before;
        let g = |a: &std::sync::atomic::AtomicU64, b: u64| a.load(R) - b;
        let ms = |nanos: u64| nanos as f64 / n as f64 / 1e6;
        let hidden = g(&ip::N3_HIDDEN_FILLS, hf0);
        let skips = g(&ip::N3_SKIP_FILLS, sk0);
        let walk_ms = ms(g(&ip::N3_WALK_NANOS, wk0));
        let worker_ms = ms(g(&ip::N3_WORKER_NANOS, wrk0));
        eprintln!(
            "[N3-probe] pipelined drive (NCF_PAR_FILL=2): hidden_fills = {} (expect 3/chunk) skip_fills = {} (expect 1/chunk) | fork(clone) = {:.3} walk = {walk_ms:.3} worker = {worker_ms:.3} join_block = {:.3} merge = {:.3} ms/chunk",
            hidden / (n as u64).max(1),
            skips / (n as u64).max(1),
            ms(g(&ip::N3_FORK_NANOS, fk0)),
            ms(g(&ip::N3_JOIN_BLOCK_NANOS, jb0)),
            ms(g(&ip::N3_MERGE_NANOS, mg0)),
        );
        eprintln!(
            "[N3-probe] overlap efficiency worker/walk = {:.3} | exposed residual Σmax(0,worker−walk) = {:.3} ms/chunk (exact per-scope sum)",
            worker_ms / walk_ms.max(1e-9),
            ms(g(&ip::N3_RESIDUAL_NANOS, rs0)),
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
