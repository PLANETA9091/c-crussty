// t38probe — T38-B (Job 441690): which structures START near the gate-P2
// blob region for seed 3053459? The blob (chunks -18..-10 x -19..-11,
// stone<->air/water substance-threshold flips, vector rigs BIT-EXACT there)
// points at the missing Beardifier structure adaptation: our noise filler
// runs BeardifierMarker (0.0) while the live NOISE-status dump includes
// Beardifier.forStructuresInChunk — terrain-adapting pieces shift the
// substance across the 0 threshold.
//
// Walks data/<ns>/structure_set/*.json from the extract, reports every
// random_spread placement chunk inside a generous window around the blob.
use chunk_factory::random_spread::RandomSpreadStructurePlacement;
use chunk_factory::router::WorldgenDir;
use std::path::Path;

fn main() {
    let seed: i64 = 3053459;
    let wg = std::env::args().nth(1).expect("usage: t38probe <worldgen-dir>");
    let dir = WorldgenDir::load(Path::new(&wg)).expect("worldgen dir");

    // generous window: the blob is chunks -18..-10 x -19..-11; structure
    // pieces span up to ~12 chunks, so report placements in chunks -40..8.
    let (cx0, cx1, cz0, cz1) = (-40i32, 8i32, -40i32, 8i32);

    let mut total = 0usize;
    let mut sets_seen = 0usize;
    let mut spreads_seen = 0usize;
    for ns in dir.namespaces() {
        for name in dir.list(&ns, "structure_set") {
            sets_seen += 1;
            let text = dir.read(&ns, "structure_set", &name).expect("structure_set");
            let json = chunk_factory::json::parse(&text).expect("structure_set json");
            // NOTE: RandomSpreadStructurePlacement::parse takes the WHOLE
            // structure_set object (it reads the nested "placement" itself)
            // and rejects non-random_spread placements.
            let rs = match RandomSpreadStructurePlacement::parse(&json) {
                Ok(rs) => rs,
                Err(_) => continue, // concentrate/other placement types
            };
            spreads_seen += 1;
            // a placement chunk (px,pz) is decided per region cell; scan the
            // window chunk-by-chunk and report when the scanned chunk IS the
            // placement chunk (dedupes the region-cell repetition).
            for cx in cx0..=cx1 {
                for cz in cz0..=cz1 {
                    if let Some((px, pz)) = rs.potential_chunk(seed, cx, cz) {
                        if px == cx && pz == cz {
                            println!(
                                "{}:{}\tchunk ({px},{pz})\tspacing {} sep {} salt {}",
                                ns, name, rs.spacing, rs.separation, rs.salt
                            );
                            total += 1;
                        }
                    }
                }
            }
        }
    }
    eprintln!("sets={sets_seen} random_spread={spreads_seen} placements in window [{cx0}..{cx1}]x[{cz0}..{cz1}]: {total}");
}
