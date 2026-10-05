//! mcaforge — P3.1: package UNCOMPRESSED chunk NBT files (produced by the
//! golden dumper's RAW mode, `chunk.<cx>.<cz>.nbt`) into vanilla .mca region
//! files the server can mount and read.
//!
//! The chunk payload is copied VERBATIM: the uncompressed NBT bytes go into
//! a zlib (format 2) envelope built from STORED deflate blocks (valid zlib,
//! no compression — the crate is dependency-free by law). The server's
//! Inflater reads stored blocks natively.
//!
//! Usage: mcaforge <raw-nbt-dir> <out-dir>
//!   reads  <raw-nbt-dir>/chunk.<cx>.<cz>.nbt
//!   writes <out-dir>/r.<rx>.<rz>.mca  (rx/rz = floorDiv(cx, 32), floorDiv(cz, 32))

use chunk_factory::region::{parse_region, write_region, RegionChunk};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: mcaforge <raw-nbt-dir> <out-dir>");
        std::process::exit(2);
    }
    let in_dir = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);
    if let Err(e) = run(&in_dir, &out_dir) {
        eprintln!("mcaforge FATAL: {e}");
        std::process::exit(1);
    }
}

fn run(in_dir: &Path, out_dir: &Path) -> Result<(), String> {
    // Collect chunk.<cx>.<cz>.nbt files (deterministic order).
    let mut entries: BTreeMap<(i32, i32), PathBuf> = BTreeMap::new();
    let rd = std::fs::read_dir(in_dir).map_err(|e| format!("{}: {e}", in_dir.display()))?;
    for e in rd {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().to_string();
        if let Some((cx, cz)) = parse_chunk_name(&name) {
            entries.insert((cx, cz), e.path());
        }
    }
    if entries.is_empty() {
        return Err(format!("no chunk.<cx>.<cz>.nbt files under {}", in_dir.display()));
    }

    // Group by region (floorDiv — negative coords go to r.-1.-1 etc.).
    let mut regions: BTreeMap<(i32, i32), Vec<RegionChunk>> = BTreeMap::new();
    for ((cx, cz), path) in &entries {
        let data = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        validate_nbt_header(&data, path)?;
        let rx = cx.div_euclid(32);
        let rz = cz.div_euclid(32);
        let x_in_region = cx.rem_euclid(32) as u8;
        let z_in_region = cz.rem_euclid(32) as u8;
        let zlib = chunk_factory::region::zlib_store(&data);
        regions.entry((rx, rz)).or_default().push(RegionChunk {
            x_in_region,
            z_in_region,
            // Region timestamps are loader-ignored metadata; a fixed constant
            // keeps the output deterministic (no wall clock in the factory).
            timestamp: 1,
            format: 2, // zlib (stored blocks)
            data: zlib,
        });
    }

    std::fs::create_dir_all(out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let mut total = 0usize;
    for ((rx, rz), chunks) in &regions {
        let bytes = write_region(chunks).map_err(|e| e.0)?;
        // Self-check: our own reader must accept what we wrote.
        let parsed = parse_region(&bytes).map_err(|e| format!("selfcheck r.{rx}.{rz}: {}", e.0))?;
        if parsed.chunks.len() != chunks.len() {
            return Err(format!("selfcheck r.{rx}.{rz}: chunk count mismatch"));
        }
        let path = out_dir.join(format!("r.{rx}.{rz}.mca"));
        std::fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        println!(
            "wrote {} ({} chunks, {} sectors)",
            path.display(),
            chunks.len(),
            bytes.len() / chunk_factory::region::SECTOR
        );
        total += chunks.len();
    }
    println!("MCAFORGE COMPLETE chunks={total} regions={}", regions.len());
    Ok(())
}

/// `chunk.<cx>.<cz>.nbt` -> (cx, cz); negative coords use the plain '-' sign.
fn parse_chunk_name(name: &str) -> Option<(i32, i32)> {
    let rest = name.strip_prefix("chunk.")?.strip_suffix(".nbt")?;
    let (cx, cz) = rest.split_once('.')?;
    Some((cx.parse().ok()?, cz.parse().ok()?))
}

/// Minimal NBT sanity check: root tag 0x0A + non-empty root name (the
/// payload itself is copied verbatim — no parsing, per the zero-dep law).
fn validate_nbt_header(data: &[u8], path: &Path) -> Result<(), String> {
    if data.len() < 3 {
        return Err(format!("{}: too small for an NBT header", path.display()));
    }
    if data[0] != 0x0A {
        return Err(format!("{}: root tag is not TAG_Compound (0x{:02x})", path.display(), data[0]));
    }
    // NOTE: the root name may legitimately be EMPTY (vanilla chunk NBT uses
    // ""), so only the framing is checked.
    let name_len = u16::from_be_bytes([data[1], data[2]]) as usize;
    if 3 + name_len > data.len() {
        return Err(format!("{}: root name length {name_len} exceeds file", path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_name_parsing() {
        assert_eq!(parse_chunk_name("chunk.0.0.nbt"), Some((0, 0)));
        assert_eq!(parse_chunk_name("chunk.100.-37.nbt"), Some((100, -37)));
        assert_eq!(parse_chunk_name("chunk.-1.-1.nbt"), Some((-1, -1)));
        assert_eq!(parse_chunk_name("manifest.tsv"), None);
        assert_eq!(parse_chunk_name("chunk.1.nbt"), None);
    }

    #[test]
    fn nbt_header_validation() {
        let mut buf = vec![0x0Au8];
        buf.extend_from_slice(&5u16.to_be_bytes());
        buf.extend_from_slice(b"level");
        buf.extend_from_slice(&[1, 2, 3]);
        assert!(validate_nbt_header(&buf, Path::new("t")).is_ok());
        // empty root name is legal (vanilla chunk NBT)
        assert!(validate_nbt_header(&[0x0A, 0, 0, 1, 2, 3], Path::new("t")).is_ok());
        assert!(validate_nbt_header(&[0x08, 0, 1, 65], Path::new("t")).is_err());
        assert!(validate_nbt_header(&[0x0A], Path::new("t")).is_err());
        assert!(validate_nbt_header(&[0x0A, 0, 9, 65], Path::new("t")).is_err());
    }
}
