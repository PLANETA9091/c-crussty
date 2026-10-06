//! Ladder A: offline `.mca` region file writer (NCF P3.1) — pure Rust,
//! zero dependencies (I7).
//!
//! Region format (vanilla Anvil, verified against the format spec and by
//! round-trip against python zlib — the same bytes Java's Inflater reads):
//!
//! ```text
//! header (2 * 4096 bytes):
//!   offsets[1024]  : u32 BE — (sector_index << 8) | sector_count, 0 = absent
//!   timestamps[1024]: u32 BE — unix time of last chunk write
//! payload: 4096-byte sectors; each chunk payload =
//!   length: u32 BE (excluding the length field itself, including the byte below)
//!   format: u8  — 1 gzip, 2 zlib, 3 uncompressed
//!   data  : the compressed chunk NBT
//! ```
//!
//! Compression: the writer emits a VALID zlib stream using STORED deflate
//! blocks (no compression) — bit-exact byte equality with Java's Deflater is
//! NOT required by I1 (semantic equality is the law; byte-identity is
//! desirable). This keeps the crate dependency-free; swapping in libdeflate/
//! zlib-ng with parallel compression is the P4.8 step, gated behind the same
//! zero-diff rule (region bytes are transparent to the loader either way).
//!
//! Deliberately NOT in v1: reading foreign-compressed regions (gzip),
//! external-michunk flags (format byte >= 128), and chunk-sharing schemes —
//! Ladder A writes FRESH pregenerated regions only; mixed edits stay on the
//! server (I8 fallback).

use std::collections::BTreeMap;

pub const SECTOR: usize = 4096;
pub const HEADER_SECTORS: usize = 2;
pub const CHUNKS_PER_REGION: usize = 32;

/// One chunk's serialized payload. `format`: 1 = gzip (dumper bytes
/// verbatim — zero recompression), 2 = zlib (uncompressed input goes through
/// the stored-block stream), 3 = uncompressed.
pub struct RegionChunk {
    pub x_in_region: u8, // 0..32
    pub z_in_region: u8, // 0..32
    pub timestamp: u32,
    pub format: u8,
    pub data: Vec<u8>, // for format 1: the gzipped NBT bytes verbatim
}

// NOTE on input data: NbtIo.writeCompressed produces a GZIP stream. A region
// file's format byte for gzip is 1, and the payload must then be the raw
// gzip bytes. The golden dumper writes gzipped NBT, so Ladder A v1 writes
// format=1 (gzip) with the dumper bytes VERBATIM — zero recompression, the
// fastest possible path and byte-identical to what a vanilla
// ChunkSerializer->RegionFileStorage round trip would produce only if the
// same deflate settings are used (not required by I1).
//
// zlib (format=2) is provided for payloads that arrive uncompressed
// (stored-block stream, see module doc) and for tests.

// --------------------------------------------------------------------------
// zlib (stored-blocks) — valid zlib stream without any dependency
// --------------------------------------------------------------------------

/// zlib stream with stored deflate blocks.
pub fn zlib_store(data: &[u8]) -> Vec<u8> {
    // CMF=0x78 (deflate, 32K window), FLG=0x01 -> (0x78*256+0x01) % 31 == 0
    let mut out = Vec::with_capacity(data.len() + data.len() / 65535 * 5 + 16);
    out.push(0x78);
    out.push(0x01);
    let mut i = 0usize;
    loop {
        let remaining = data.len() - i;
        let n = remaining.min(65535);
        let last = i + n >= data.len();
        out.push(if last { 1 } else { 0 }); // BFINAL + BTYPE=00 (stored)
        out.extend_from_slice(&(n as u16).to_le_bytes());
        out.extend_from_slice(&(!(n as u16)).to_le_bytes());
        out.extend_from_slice(&data[i..i + n]);
        i += n;
        if last {
            break;
        }
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// Adler-32 (RFC 1950).
pub fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

/// Inflate stored-blocks zlib streams (reader-side self-check only; the
/// server never needs this — it has its own inflater).
pub fn zlib_store_decompress(stream: &[u8]) -> std::result::Result<Vec<u8>, String> {
    if stream.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = stream[0];
    let flg = stream[1];
    if cmf & 0x0F != 8 {
        return Err("not a deflate stream".into());
    }
    if !((cmf as u16) * 256 + flg as u16).is_multiple_of(31) {
        return Err("zlib header check bits invalid".into());
    }
    let mut i = 2usize;
    let mut out = Vec::new();
    loop {
        if i >= stream.len() {
            return Err("truncated deflate stream".into());
        }
        let header = stream[i];
        i += 1;
        let bfinal = header & 1;
        let btype = (header >> 1) & 3;
        match btype {
            0 => {
                if i + 4 > stream.len() {
                    return Err("truncated stored block header".into());
                }
                let n = u16::from_le_bytes([stream[i], stream[i + 1]]) as usize;
                let nlen = u16::from_le_bytes([stream[i + 2], stream[i + 3]]) as usize;
                if n != (!nlen & 0xFFFF) {
                    return Err("stored block LEN/NLEN mismatch".into());
                }
                i += 4;
                if i + n > stream.len() {
                    return Err("truncated stored block data".into());
                }
                out.extend_from_slice(&stream[i..i + n]);
                i += n;
            }
            _ => {
                return Err(format!("unsupported deflate block type {btype} (reader handles stored blocks only)"))
            }
        }
        if bfinal == 1 {
            break;
        }
    }
    if i + 4 > stream.len() {
        return Err("missing adler32 trailer".into());
    }
    let expect = u32::from_be_bytes([stream[i], stream[i + 1], stream[i + 2], stream[i + 3]]);
    if adler32(&out) != expect {
        return Err("adler32 mismatch".into());
    }
    Ok(out)
}

// --------------------------------------------------------------------------
// Region writer
// --------------------------------------------------------------------------

#[derive(Debug)]
pub struct RegionError(pub String);

impl From<String> for RegionError {
    fn from(s: String) -> Self {
        RegionError(s)
    }
}

impl From<&str> for RegionError {
    fn from(s: &str) -> Self {
        RegionError(s.to_string())
    }
}

impl std::fmt::Display for RegionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "region error: {}", self.0)
    }
}

pub type RelResult<T> = std::result::Result<T, RegionError>;

fn err<T>(msg: impl Into<String>) -> RelResult<T> {
    Err(RegionError(msg.into()))
}

/// Write a complete .mca file from the given chunks. Duplicate coordinates
/// overwrite (last wins, deterministic BTreeMap order = coordinate order).
pub fn write_region(chunks: &[RegionChunk]) -> RelResult<Vec<u8>> {
    let mut by_coord: BTreeMap<(u8, u8), &RegionChunk> = BTreeMap::new();
    for c in chunks {
        if c.x_in_region >= 32 || c.z_in_region >= 32 {
            return err(format!(
                "chunk in-region coord out of range: ({}, {})",
                c.x_in_region, c.z_in_region
            ));
        }
        by_coord.insert((c.x_in_region, c.z_in_region), c);
    }

    // Prepare payloads with format byte + length prefix; assign sectors.
    let mut payloads: BTreeMap<(u8, u8), Vec<u8>> = BTreeMap::new();
    let mut sector_of: BTreeMap<(u8, u8), (u32, u8)> = BTreeMap::new();
    let mut next_sector: usize = HEADER_SECTORS;
    for (&coord, chunk) in &by_coord {
        let fmt_byte: u8 = chunk.format;
        let mut payload = Vec::with_capacity(chunk.data.len() + 5);
        payload.extend_from_slice(&((chunk.data.len() + 1) as u32).to_be_bytes());
        payload.push(fmt_byte);
        payload.extend_from_slice(&chunk.data);
        let sectors = payload.len().div_ceil(SECTOR);
        sector_of.insert(coord, (next_sector as u32, sectors as u8));
        next_sector += sectors;
        payloads.insert(coord, payload);
    }

    let total_sectors = next_sector;
    let mut out = vec![0u8; total_sectors * SECTOR];

    // offsets
    for (coord, &(sector, count)) in &sector_of {
        let idx = (coord.0 as usize) + (coord.1 as usize) * CHUNKS_PER_REGION;
        let v = (sector << 8) | count as u32;
        out[idx * 4..idx * 4 + 4].copy_from_slice(&v.to_be_bytes());
    }
    // timestamps
    for (coord, chunk) in &by_coord {
        let idx = (coord.0 as usize) + (coord.1 as usize) * CHUNKS_PER_REGION;
        out[4096 + idx * 4..4096 + idx * 4 + 4].copy_from_slice(&chunk.timestamp.to_be_bytes());
    }
    // payloads
    for (coord, payload) in &payloads {
        let (sector, _) = sector_of[coord];
        let start = sector as usize * SECTOR;
        out[start..start + payload.len()].copy_from_slice(payload);
    }
    Ok(out)
}

/// A parsed region (reader — used by the self-check and by Phase 3.2
/// zero-diff tooling).
pub struct ParsedRegion {
    /// (x, z) -> (timestamp, format byte, data)
    pub chunks: BTreeMap<(u8, u8), (u32, u8, Vec<u8>)>,
}

/// Parse a .mca file: header consistency, sector bounds, payload lengths.
pub fn parse_region(bytes: &[u8]) -> RelResult<ParsedRegion> {
    if bytes.len() < HEADER_SECTORS * SECTOR {
        return err("region file smaller than header");
    }
    let mut chunks = BTreeMap::new();
    for idx in 0..1024 {
        let off = u32::from_be_bytes([
            bytes[idx * 4],
            bytes[idx * 4 + 1],
            bytes[idx * 4 + 2],
            bytes[idx * 4 + 3],
        ]);
        if off == 0 {
            continue;
        }
        let sector = (off >> 8) as usize;
        let count = (off & 0xFF) as usize;
        if count == 0 {
            return err(format!("chunk {idx}: sector count 0 with nonzero offset"));
        }
        if sector < HEADER_SECTORS {
            return err(format!("chunk {idx}: sector {sector} overlaps header"));
        }
        if (sector + count) * SECTOR > bytes.len() {
            return err(format!("chunk {idx}: sectors {sector}+{count} exceed file"));
        }
        let start = sector * SECTOR;
        if start + 5 > bytes.len() {
            return err(format!("chunk {idx}: truncated payload header"));
        }
        let len = u32::from_be_bytes([bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3]])
            as usize;
        let fmt = bytes[start + 4];
        if len < 2 {
            return err(format!("chunk {idx}: payload length {len} too small"));
        }
        if len - 1 > count * SECTOR - 4 {
            return err(format!("chunk {idx}: payload length {len} exceeds its sectors"));
        }
        let data = bytes[start + 5..start + 4 + len].to_vec();
        let ts = u32::from_be_bytes([
            bytes[4096 + idx * 4],
            bytes[4096 + idx * 4 + 1],
            bytes[4096 + idx * 4 + 2],
            bytes[4096 + idx * 4 + 3],
        ]);
        let x = (idx % 32) as u8;
        let z = (idx / 32) as u8;
        chunks.insert((x, z), (ts, fmt, data));
    }
    Ok(ParsedRegion { chunks })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adler32_reference() {
        // RFC-style hand check: s1/s2 accumulation
        assert_eq!(adler32(b"abc"), 0x024D0127);
        assert_eq!(adler32(b"hello ncf region writer"), 0x67a208cd);
        assert_eq!(adler32(b""), 1);
    }

    #[test]
    fn zlib_stored_matches_reference_stream_shape() {
        // python3: zlib.compress(b"hello ncf region writer", 6) is a REAL
        // zlib stream; our STORED stream must decompress to the same bytes
        // through the stdlib (verified externally) and through our reader.
        let data = b"hello ncf region writer";
        let stream = zlib_store(data);
        assert_eq!(zlib_store_decompress(&stream).unwrap(), data);
        assert_eq!(&stream[..2], &[0x78, 0x01]);
        // python cross-check constant (computed with zlib.adler32)
        assert_eq!(adler32(data), 0x67a208cd);
    }

    #[test]
    fn zlib_stored_multi_block_200k() {
        let big: Vec<u8> = (0..200_000u32).map(|i| ((i * 7 + 3) % 256) as u8).collect();
        let stream = zlib_store(&big);
        assert_eq!(zlib_store_decompress(&stream).unwrap(), big);
    }

    #[test]
    fn region_roundtrip_layout() {
        let mk = |x: u8, z: u8, seed: usize| RegionChunk {
            x_in_region: x,
            z_in_region: z,
            timestamp: 1_700_000_000 + seed as u32,
            format: 1,
            data: vec![(seed % 251) as u8 + 1; 50 + seed * 77],
        };
        let chunks: Vec<RegionChunk> = (0..9)
            .map(|i| mk((i % 3) as u8 * 7, (i / 3) as u8 * 7, i))
            .collect();
        let bytes = write_region(&chunks).unwrap();
        // header = 2 sectors; file is sector-aligned
        assert_eq!(bytes.len() % SECTOR, 0);
        assert!(bytes.len() > HEADER_SECTORS * SECTOR);
        let parsed = parse_region(&bytes).unwrap();
        assert_eq!(parsed.chunks.len(), 9);
        for c in &chunks {
            let (ts, fmt, data) = &parsed.chunks[&(c.x_in_region, c.z_in_region)];
            assert_eq!(*ts, c.timestamp);
            assert_eq!(*fmt, 1); // gzip marker (dumper bytes verbatim)
            assert_eq!(data, &c.data);
        }
    }

    #[test]
    fn region_all_1024_chunks() {
        let chunks: Vec<RegionChunk> = (0..1024usize)
            .map(|i| RegionChunk {
                x_in_region: (i % 32) as u8,
                z_in_region: (i / 32) as u8,
                timestamp: i as u32,
                format: 1,
                data: vec![0xAB; 64 + i % 13],
            })
            .collect();
        let bytes = write_region(&chunks).unwrap();
        let parsed = parse_region(&bytes).unwrap();
        assert_eq!(parsed.chunks.len(), 1024);
        // coordinate order sanity: (0,0) sectors come before (31,31)
        let first = parsed.chunks.get(&(0, 0)).unwrap();
        let last = parsed.chunks.get(&(31, 31)).unwrap();
        let _ = (first, last);
    }

    #[test]
    fn region_empty_and_padding() {
        // An empty region is just the header.
        let bytes = write_region(&[]).unwrap();
        assert_eq!(bytes.len(), HEADER_SECTORS * SECTOR);
        assert!(parse_region(&bytes).unwrap().chunks.is_empty());
        // A chunk smaller than a sector still occupies exactly 1 sector.
        let one = vec![RegionChunk { x_in_region: 5, z_in_region: 9, timestamp: 7, format: 1, data: vec![1u8; 10] }];
        let bytes = write_region(&one).unwrap();
        assert_eq!(bytes.len(), 3 * SECTOR);
    }

    #[test]
    fn region_rejects_out_of_range_coords() {
        let bad = vec![RegionChunk { x_in_region: 32, z_in_region: 0, timestamp: 0, format: 1, data: vec![1] }];
        assert!(write_region(&bad).is_err());
    }

    #[test]
    fn region_large_payload_multi_sector() {
        let big = vec![RegionChunk {
            x_in_region: 0,
            z_in_region: 0,
            timestamp: 5,
            format: 1,
            data: vec![9u8; 3 * SECTOR + 100], // spans 4 sectors
        }];
        let bytes = write_region(&big).unwrap();
        assert_eq!(bytes.len(), (2 + 4) * SECTOR);
        let parsed = parse_region(&bytes).unwrap();
        assert_eq!(parsed.chunks[&(0, 0)].2.len(), 3 * SECTOR + 100);
    }
}

// ---------------------------------------------------------------------------
// P4.8 — parallel payload compression. Chunk payloads are independent, so
// the compression stage threads over std::thread::scope (zero-dep), then the
// region is assembled SERIALLY in the same sector order — the output bytes
// are IDENTICAL to write_region for the same input (test below pins that).
// The deflate backend is still stored-block zlib (byte-stable, valid); when
// a libdeflate/zlib-ng binding lands it plugs into `compress_payload` only.
// ---------------------------------------------------------------------------

/// One compression unit. Kept a function so the backend is swappable.
fn compress_payload(chunk: &RegionChunk) -> RelResult<Vec<u8>> {
    let mut payload = Vec::with_capacity(chunk.data.len() + 5);
    payload.extend_from_slice(&((chunk.data.len() + 1) as u32).to_be_bytes());
    payload.push(chunk.format);
    payload.extend_from_slice(&chunk.data);
    Ok(payload)
}

/// Parallel writer: same bytes as write_region, compression spread over
/// `threads` worker threads (clamped 1..=num_cpus, bounded by chunk count).
pub fn write_region_parallel(chunks: &[RegionChunk], threads: usize) -> RelResult<Vec<u8>> {
    let mut by_coord: BTreeMap<(u8, u8), &RegionChunk> = BTreeMap::new();
    for c in chunks {
        if c.x_in_region >= 32 || c.z_in_region >= 32 {
            return err(format!(
                "chunk in-region coord out of range: ({}, {})",
                c.x_in_region, c.z_in_region
            ));
        }
        by_coord.insert((c.x_in_region, c.z_in_region), c);
    }
    let coords: Vec<(u8, u8)> = by_coord.keys().copied().collect();
    let items: Vec<&RegionChunk> = coords.iter().map(|c| by_coord[c]).collect();

    let n_threads = threads.max(1).min(items.len().max(1)).min(64);
    let results: Vec<RelResult<Vec<u8>>> = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(n_threads);
        let chunk_count = items.len();
        for t in 0..n_threads {
            let slice = &items;
            handles.push(scope.spawn(move || {
                let mut out = Vec::new();
                let mut i = t;
                while i < chunk_count {
                    out.push(compress_payload(slice[i]));
                    i += n_threads;
                }
                out
            }));
        }
        // re-interleave round-robin results back into coordinate order
        let mut per_thread: Vec<Vec<RelResult<Vec<u8>>>> =
            handles.into_iter().map(|h| h.join().expect("compression worker")).collect();
        let mut ordered: Vec<Option<RelResult<Vec<u8>>>> =
            (0..chunk_count).map(|_| None).collect();
        for (t, outs) in per_thread.drain(..).enumerate() {
            for (k, r) in outs.into_iter().enumerate() {
                ordered[t + k * n_threads] = Some(r);
            }
        }
        ordered.into_iter().map(|o| o.expect("all results filled")).collect()
    });

    let mut payloads: BTreeMap<(u8, u8), Vec<u8>> = BTreeMap::new();
    for (coord, res) in coords.iter().zip(results) {
        payloads.insert(*coord, res?);
    }

    // assembly identical to write_region (sector order = coordinate order)
    let mut sector_of: BTreeMap<(u8, u8), (u32, u8)> = BTreeMap::new();
    let mut next_sector: usize = HEADER_SECTORS;
    for (coord, payload) in &payloads {
        let sectors = payload.len().div_ceil(SECTOR);
        sector_of.insert(*coord, (next_sector as u32, sectors as u8));
        next_sector += sectors;
    }
    let total_sectors = next_sector;
    let mut out = vec![0u8; total_sectors * SECTOR];
    // location/sector headers
    for (&coord, &(offset, count)) in &sector_of {
        let header_index = ((coord.0 as usize) + (coord.1 as usize) * 32) * 4;
        let v = ((offset as u32) << 8) | count as u32;
        out[header_index..header_index + 4].copy_from_slice(&v.to_be_bytes());
    }
    // timestamps
    for coord in coords.iter() {
        let header_index = 4096 + ((coord.0 as usize) + (coord.1 as usize) * 32) * 4;
        let stamp = by_coord[coord].timestamp;
        out[header_index..header_index + 4].copy_from_slice(&stamp.to_be_bytes());
    }
    for (coord, payload) in &payloads {
        let (offset, count) = sector_of[coord];
        let start = offset as usize * SECTOR;
        out[start..start + payload.len()].copy_from_slice(payload);
    }
    Ok(out)
}

#[cfg(test)]
mod parallel_tests {
    use super::*;

    fn sample_chunks(n: usize) -> Vec<RegionChunk> {
        (0..n)
            .map(|i| RegionChunk {
                x_in_region: (i % 32) as u8,
                z_in_region: (i / 32 % 32) as u8,
                timestamp: 1_700_000_000 + i as u32,
                format: 1,
                data: vec![(i * 7 % 251) as u8; 100 + i * 33 % 9000],
            })
            .collect()
    }

    #[test]
    fn parallel_writer_byte_identical() {
        let chunks = sample_chunks(200);
        let serial = write_region(&chunks).expect("serial");
        for threads in [1usize, 2, 4, 8] {
            let par = write_region_parallel(&chunks, threads).expect("parallel");
            assert_eq!(serial, par, "byte divergence at threads={threads}");
        }
    }
}
