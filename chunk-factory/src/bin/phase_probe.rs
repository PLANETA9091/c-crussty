use chunk_factory::router::{RandomState, WorldgenDir};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bx: i32 = args[1].parse().unwrap();
    let by: i32 = args[2].parse().unwrap();
    let bz: i32 = args[3].parse().unwrap();
    let dir = WorldgenDir::load(Path::new("/tmp/wg-extract")).unwrap();
    let rs = RandomState::build(&dir, "minecraft", "overworld", 3053459).unwrap();
    let _zoom = chunk_factory::biomes::biome_zoom_seed(3053459);
    let mut src = chunk_factory::biomes::BiomeSource::new(&rs);
    let i = bx - 2; let i1 = by - 2; let i2 = bz - 2;
    let i3 = i >> 2; let i4 = i1 >> 2; let i5 = i2 >> 2;
    let d = (i & 3) as f64 / 4.0; let d1 = (i1 & 3) as f64 / 4.0; let d2 = (i2 & 3) as f64 / 4.0;
    for i7 in 0..8usize {
        let flag = (i7 & 4) == 0; let flag1 = (i7 & 2) == 0; let flag2 = (i7 & 1) == 0;
        let (qx, qy, qz) = (
            if flag { i3 } else { i3 + 1 },
            if flag1 { i4 } else { i4 + 1 },
            if flag2 { i5 } else { i5 + 1 },
        );
        let (dx, dy, dz) = (
            if flag { d } else { d - 1.0 },
            if flag1 { d1 } else { d1 - 1.0 },
            if flag2 { d2 } else { d2 - 1.0 },
        );
        let b = src.get_noise_biome(qx, qy, qz);
        println!("n{i7}: ({qx},{qy},{qz}) {b}");
        let _ = (dx, dy, dz);
    }
}
