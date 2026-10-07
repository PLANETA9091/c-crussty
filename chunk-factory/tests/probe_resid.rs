//! Regression anchor (addendum 23, Job 441690): the aquifer candidate RNG
//! (positional at(gx,gy,gz) -> nextInt(10/9/10) -> BlockPos.asLong) must stay
//! bit-exact against the JAVA meta capture at the resid1 column (253,-67),
//! chunk (15,-5), seed 3053459 — INCLUDING negative grid-Y cells (gy -4..-2),
//! the class never covered by the blob corpus (band 40..100 = gridY 2..9).
//! Java values decoded from ci-server/golden/resid1/vectors/aquifer_meta.txt
//! (run of 2026-10-07, local ci_density.sh capture).

use chunk_factory::jrandom::RandomSource;
use chunk_factory::xoroshiro::XoroshiroRandomSource;

const X_RANGE: i32 = 10;
const Y_RANGE: i32 = 9;
const Z_RANGE: i32 = 10;

/// (gx, gy, gz) -> (offsetX, offsetY, offsetZ) as captured from java.
const JAVA_CANDIDATES: [((i32, i32, i32), (i32, i32, i32)); 12] = [
    ((15, -4, -5), (7, 5, 4)),
    ((15, -4, -4), (6, 4, 8)),
    ((15, -3, -5), (0, 3, 6)),
    ((15, -3, -4), (0, 3, 1)),
    ((15, -2, -5), (8, 2, 7)),
    ((15, -2, -4), (7, 6, 2)),
    ((16, -4, -5), (6, 1, 2)),
    ((16, -4, -4), (7, 8, 5)),
    ((16, -3, -5), (4, 2, 3)),
    ((16, -3, -4), (5, 5, 2)),
    ((16, -2, -5), (9, 8, 6)),
    ((16, -2, -4), (1, 4, 3)),
];

#[test]
fn aquifer_candidates_match_java_meta_at_negative_grid_y() {
    let mut base = XoroshiroRandomSource::new(3053459);
    let mut worldgen = base.fork_positional();
    let mut aquifer_src = worldgen.from_hash_of("minecraft:aquifer");
    let factory = aquifer_src.fork_positional();
    for ((gx, gy, gz), expected) in JAVA_CANDIDATES {
        let mut rng = factory.at(gx, gy, gz);
        let ox = rng.next_int_bound(X_RANGE);
        let oy = rng.next_int_bound(Y_RANGE);
        let oz = rng.next_int_bound(Z_RANGE);
        assert_eq!(
            (ox, oy, oz),
            expected,
            "aquifer candidate offsets diverge from java at grid cell ({gx},{gy},{gz})"
        );
    }
}
