use chunk_factory::jrandom::RandomSource;
fn main() {
    let seed: i64 = 3053459;
    let (cx, cz) = (-14i32, -15i32);
    let mut r = chunk_factory::xoroshiro::XoroshiroRandomSource::new(seed);
    chunk_factory::jigsaw::set_large_feature_seed(&mut r, seed, cx, cz);
    for i in 0..8 {
        println!("draw{i}: next_int_bound(4) = {}", r.next_int_bound(4));
    }
    let mut r2 = chunk_factory::xoroshiro::XoroshiroRandomSource::new(seed);
    chunk_factory::jigsaw::set_large_feature_seed(&mut r2, seed, cx, cz);
    println!("first next_long: {}", r2.next_long());
    let mut r3 = chunk_factory::xoroshiro::XoroshiroRandomSource::new(seed);
    chunk_factory::jigsaw::set_large_feature_seed(&mut r3, seed, cx, cz);
    println!("first next_int: {}", r3.next_int());
}
