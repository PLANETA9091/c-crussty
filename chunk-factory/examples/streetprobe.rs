use chunk_factory::jigsaw::{self, rotation_get_random, Rotation, ROTATION_VALUES};
use chunk_factory::jrandom::{LegacyRandomSource, RandomSource};
use chunk_factory::router::WorldgenDir;
use std::path::Path;

fn main() {
    let dir = WorldgenDir::load(Path::new("/home/z/my-project/c-crussty/ci-server/worldgen-extract")).expect("dir");
    let seed: i64 = 3053459;
    let (cx, cz) = (-14i32, -15i32);

    // GenerationContext.makeRandom (1.21.10)
    let mut rng = LegacyRandomSource::new(0);
    jigsaw::set_large_feature_seed(&mut rng, seed, cx, cz);

    // addPieces: rotation
    let rot = rotation_get_random(&mut rng);
    println!("rotation draw -> {:?}", rot);
    // town_centers pool getRandomTemplate: nextInt(size)
    let pool = jigsaw::resolve_pool(&dir, "minecraft:village/taiga/town_centers", false).unwrap();
    println!("town_centers expanded size = {}", pool.size());
    let start = pool.get_random_template(&mut rng);
    println!("start template picked");

    // start template getShuffledJigsawBlocks: shuffle of n jigsaws (n-1 draws)
    if let Some(loc) = match &start {
        chunk_factory::jigsaw::PoolElement::Single { location, .. } => Some(location.clone()),
        _ => None,
    } {
        let (ns, path) = loc.split_once(':').unwrap();
        let nbt = chunk_factory::template_cache::TemplateCache::new(&dir).get(ns, &path).unwrap();
        let t = chunk_factory::jigsaw::parse_template(&nbt).unwrap();
        println!("start template jigsaws = {}", t.jigsaws.len());
        // replicate util_shuffle consumption without mutating real list order knowledge:
        let mut order: Vec<usize> = (0..t.jigsaws.len()).collect();
        jigsaw::util_shuffle(&mut order, &mut rng);
        println!("parent jigsaw processed order: {:?}", order);
    }

    // first parent jigsaw -> streets pool getShuffledTemplates (49): Fisher-Yates
    let streets = jigsaw::resolve_pool(&dir, "minecraft:village/taiga/streets", false).unwrap();
    println!("streets expanded size = {}", streets.size());
    let mut idx: Vec<usize> = (0..streets.size() as usize).collect();
    jigsaw::util_shuffle(&mut idx, &mut rng);
    // print first 10 expanded candidates with names
    println!("streets shuffled first 12:");
    for (k, &i) in idx.iter().take(12).enumerate() {
        if let chunk_factory::jigsaw::PoolElement::Single { location, .. } = &streets.templates[i] {
            println!("  cand{k}: [{i}] {location}");
        }
    }
    let _ = (ROTATION_VALUES, Rotation::None);
}
