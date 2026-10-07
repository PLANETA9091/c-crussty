use chunk_factory::router::{RandomState, WorldgenDir};
use std::collections::BTreeSet;
use std::path::Path;
fn main() {
    let root = std::env::args().nth(1).expect("root");
    let dir = WorldgenDir::load(Path::new(&root)).expect("load");
    let dim = dir.get("minecraft", "dimension", "overworld").is_some();
    println!("dimension file indexed: {dim}");
    let rs = RandomState::build_overworld(&dir, 90210).expect("build_overworld");
    match &rs.biome_points {
        Some(p) => {
            let names: BTreeSet<&String> = p.iter().map(|(_, n)| n).collect();
            println!("pack table: {} entries, {} distinct biomes", p.len(), names.len());
            println!("sample: {:?}", names.iter().take(5).collect::<Vec<_>>());
        }
        None => println!("vanilla table (biome_points=None)"),
    }
}
