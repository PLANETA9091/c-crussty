//! Probe: parse a structure template .nbt with the factory's own chain
//! (gunzip -> nbt_parse -> jigsaw::parse_template) and report where it fails.
//! usage: cargo run --release --example nbt_probe -- <file.nbt> [...]
use chunk_factory::sections::{gunzip, nbt_parse};

fn main() {
    for path in std::env::args().skip(1) {
        println!("== {path}");
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => { println!("   read FAIL: {e}"); continue; }
        };
        println!("   bytes={} magic={:02x?}{}", bytes.len(), &bytes[..3.min(bytes.len())],
            if bytes.starts_with(&[0x1f, 0x8b]) { " (gzip)" } else { " (RAW?)" });
        let raw = match gunzip(&bytes) {
            Ok(r) => r,
            Err(e) => { println!("   gunzip FAIL: {e:?}"); continue; }
        };
        let nbt = match nbt_parse(&raw) {
            Ok(n) => n,
            Err(e) => { println!("   nbt_parse FAIL: {e:?}"); continue; }
        };
        println!("   nbt_parse OK");
        match &nbt {
            chunk_factory::sections::Nbt::Comp(fields) => {
                let mut keys: Vec<_> = fields.iter().map(|(k, _)| k.clone()).collect();
                keys.sort();
                println!("   root keys: {keys:?}");
            }
            other => println!("   root is non-compound: tag_id other"),
        }
        let t = chunk_factory::jigsaw::parse_template(&nbt);
        match t {
            Some(t) => println!("   parse_template OK size={:?}", t.size),
            None => println!("   parse_template FAIL"),
        }
    }

    // --- DirPoolSource chain probe against the terralith extract ---
    println!("== DirPoolSource chain probe");
    let root = std::path::Path::new("/home/z/my-project/c-crussty/ci-datapacks/terralith-extract");
    let dir = match chunk_factory::router::WorldgenDir::load(root) {
        Ok(d) => d,
        Err(e) => { println!("   WorldgenDir::load FAIL: {e}"); return; }
    };
    println!("   dir.get(terralith,structure,underground/sunken_tower) = {:?}",
        dir.get("terralith", "structure", "underground/sunken_tower").is_some());
    let fb = dir.data_root.join("terralith/structure/underground/sunken_tower.nbt");
    println!("   fallback path {} exists={}", fb.display(), fb.exists());
    let mut pools = chunk_factory::piece_feed::DirPoolSource::new(&dir);
    let pool = chunk_factory::jigsaw::resolve_pool(&dir, "terralith:underground/sunken_tower", false);
    match pool {
        Some(p) => {
            println!("   resolve_pool OK templates={}", p.templates.len());
            let tc = chunk_factory::template_cache::TemplateCache::new(&dir);
            let t2 = tc.get("terralith", "underground/sunken_tower");
            println!("   TemplateCache::get(terralith,underground/sunken_tower) = {}", t2.is_some());
        }
        None => println!("   resolve_pool FAIL"),
    }
    println!("   missing_templates={:?} missing_pools={:?} unsupported={:?}",
        pools.missing_templates, pools.missing_pools, pools.unsupported);
}
