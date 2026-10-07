//! Amdahl measurement (directive 2) — P5.3 increment 3 Beardifier feed.
//! Run: cd chunk-factory && cargo build --release --example beard_bench
//!      ../target/release/examples/beard_bench <worldgen-dir> <extract-root> <seed>

use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: beard_bench <worldgen-dir> <extract-root> <seed>");
        std::process::exit(2);
    }
    let dir = chunk_factory::router::WorldgenDir::load(std::path::Path::new(&args[1])).expect("dir");
    let root = std::path::PathBuf::from(&args[2]);
    let seed: i64 = args[3].parse().expect("seed");

    let t0 = Instant::now();
    let rs = chunk_factory::router::RandomState::build_overworld(&dir, seed).expect("rs");
    let rs_ms = t0.elapsed().as_millis();

    let t1 = Instant::now();
    let mut feed = chunk_factory::piece_feed::BeardFeed::new(&dir, &root, &rs, seed);
    let feed_ms = t1.elapsed().as_millis();

    let mut sampler = chunk_factory::height_feed::ColumnHeightSource::new(&rs, seed);

    // Trial-chambers start chunk (4,6): full pick+assembly+filter path.
    let t2 = Instant::now();
    let beard = feed.build_for_chunk(&mut sampler, 4, 6);
    let build_ms = t2.elapsed().as_millis();
    println!(
        "build_overworld {rs_ms} ms; BeardFeed::new {feed_ms} ms; build_for_chunk(4,6) {build_ms} ms -> pieces {} junctions {}",
        beard.pieces().len(),
        beard.junctions().len()
    );

    // Repeat (cache-warm) and an empty neighborhood (cache-miss fast path).
    let t3 = Instant::now();
    let beard2 = feed.build_for_chunk(&mut sampler, 3, 6);
    let warm_ms = t3.elapsed().as_millis();
    let t4 = Instant::now();
    let empty = feed.build_for_chunk(&mut sampler, 40, 35);
    let cold_empty_ms = t4.elapsed().as_millis();
    println!(
        "build_for_chunk(3,6) cache-warm {warm_ms} ms -> pieces {}; (40,35) {cold_empty_ms} ms -> empty={}",
        beard2.pieces().len(),
        empty.is_empty()
    );

    // The fed noise fill itself vs EMPTY.
    let t5 = Instant::now();
    let _ = chunk_factory::filler::generate_noise_chunk_with_beardifier(&rs, seed, 4, 6, beard)
        .expect("fill fed");
    let fed_ms = t5.elapsed().as_millis();
    let t6 = Instant::now();
    let _ = chunk_factory::filler::generate_noise_chunk(&rs, seed, 4, 6).expect("fill empty");
    let plain_ms = t6.elapsed().as_millis();
    println!(
        "noise fill @ (4,6): fed {fed_ms} ms vs EMPTY {plain_ms} ms (beardifier compute delta {} ms)",
        fed_ms as i64 - plain_ms as i64
    );
}
