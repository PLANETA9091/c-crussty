//! NCF task 5 — sections.rs: minimal zero-dep NBT reader/writer, gzip
//! (inflate for the Java oracle files, stored-block gzip writer for the Rust
//! side), and the DECODED staged-chunk schema of task 5-a.
//!
//! Staged file schema (gzipped NBT, one file per chunk, VERIFIED contract):
//! root { ChunkX:Int, ChunkZ:Int, Status:String, DataVersion:Int, MinY:Int,
//!        Height:Int,
//!   Sections: List<{ Y:Byte, Palette: List<{Name:String, Properties:Comp}>,
//!                    Data: IntArray(4096), idx = y*256 + z*16 + x }>,
//!   Biomes:   List<{ Y:Byte, Palette: List<String>,
//!                    Data: IntArray(64), idx = y*16 + z*4 + x }>,
//!   Heightmaps: raw compound of long[] (WORLDGEN heightmaps at NOISE/SURFACE),
//!   PostProcessing: List of List<Short> (per section, packed offsets) }
//!
//! Content comparison only (I1: semantic equality; byte identity is
//! desirable, not required) — palette order is irrelevant.

#![allow(clippy::type_complexity)]

use crate::filler::{BlockStateDef, FillerChunk, HeightmapKind, StateTable};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// gzip
// ---------------------------------------------------------------------------

/// Minimal RFC1951 inflate (fixed + dynamic Huffman), enough for
/// java.util.zip.GZIPOutputStream output. Returns the raw bytes.
pub fn inflate(data: &[u8]) -> Result<Vec<u8>, String> {
    struct BitReader<'a> {
        data: &'a [u8],
        pos: usize,
        bit: u32,
        acc: u64,
    }
    impl<'a> BitReader<'a> {
        fn need(&mut self, n: u32) -> Result<(), String> {
            while self.bit < n {
                let byte = if self.pos < self.data.len() {
                    let b = self.data[self.pos];
                    self.pos += 1;
                    b
                } else {
                    return Err("inflate: out of input".into());
                };
                self.acc |= (byte as u64) << self.bit;
                self.bit += 8;
            }
            Ok(())
        }
        fn bits(&mut self, n: u32) -> Result<u32, String> {
            if n == 0 {
                return Ok(0);
            }
            self.need(n)?;
            let v = (self.acc & ((1u64 << n) - 1)) as u32;
            self.acc >>= n;
            self.bit -= n;
            Ok(v)
        }
        fn align(&mut self) {
            let drop = self.bit % 8;
            self.acc >>= drop;
            self.bit -= drop;
        }
    }

    // Canonical Huffman decode table (simple bit-by-bit walk over code lengths).
    #[derive(Clone, Default)]
    struct Huffman {
        counts: [u16; 16],
        symbols: Vec<u16>,
    }
    impl Huffman {
        fn build(lengths: &[u8]) -> Huffman {
            let mut h = Huffman::default();
            for &l in lengths {
                h.counts[l as usize] += 1;
            }
            h.counts[0] = 0;
            let mut offs = [0u16; 16];
            for i in 1..16 {
                offs[i] = offs[i - 1] + h.counts[i - 1];
            }
            h.symbols = vec![0; lengths.len()];
            for (sym, &l) in lengths.iter().enumerate() {
                if l != 0 {
                    h.symbols[offs[l as usize] as usize] = sym as u16;
                    offs[l as usize] += 1;
                }
            }
            h
        }
        fn decode(&self, br: &mut BitReader) -> Result<u16, String> {
            let mut code: i32 = 0;
            let mut first: i32 = 0;
            let mut index: i32 = 0;
            for len in 1..16 {
                code |= br.bits(1)? as i32;
                let count = self.counts[len] as i32;
                if code - first < count {
                    return Ok(self.symbols[(index + (code - first)) as usize]);
                }
                index += count;
                first += count;
                first <<= 1;
                code <<= 1;
            }
            Err("inflate: bad code".into())
        }
    }

    const LEN_BASE: [u16; 29] = [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227,
        258,
    ];
    const LEN_EXTRA: [u8; 29] = [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
    ];
    const DIST_BASE: [u16; 30] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097,
        6145, 8193, 12289, 16385, 24577,
    ];
    const DIST_EXTRA: [u8; 30] = [
        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
    ];

    let mut br = BitReader { data, pos: 0, bit: 0, acc: 0 };
    let mut out: Vec<u8> = Vec::new();
    loop {
        let last = br.bits(1)?;
        let btype = br.bits(2)?;
        match btype {
            0 => {
                br.align();
                let len = br.bits(16)? as usize;
                let _nlen = br.bits(16)?;
                for _ in 0..len {
                    out.push(br.bits(8)? as u8);
                }
            }
            1 | 2 => {
                let (lit, dist) = if btype == 1 {
                    let mut ll = [0u8; 288];
                    for (i, l) in ll.iter_mut().enumerate() {
                        *l = if i < 144 {
                            8
                        } else if i < 256 {
                            9
                        } else if i < 280 {
                            7
                        } else {
                            8
                        };
                    }
                    (Huffman::build(&ll), Huffman::build(&[5u8; 30]))
                } else {
                    let hlit = br.bits(5)? as usize + 257;
                    let hdist = br.bits(5)? as usize + 1;
                    let hclen = br.bits(4)? as usize + 4;
                    const ORDER: [usize; 19] = [
                        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
                    ];
                    let mut cl = [0u8; 19];
                    for &o in ORDER.iter().take(hclen) {
                        cl[o] = br.bits(3)? as u8;
                    }
                    let clh = Huffman::build(&cl);
                    let mut lengths = vec![0u8; hlit + hdist];
                    let mut i = 0;
                    while i < hlit + hdist {
                        let sym = clh.decode(&mut br)?;
                        match sym {
                            0..=15 => {
                                lengths[i] = sym as u8;
                                i += 1;
                            }
                            16 => {
                                if i == 0 {
                                    return Err("inflate: repeat with no previous".into());
                                }
                                let prev = lengths[i - 1];
                                let rep = 3 + br.bits(2)? as usize;
                                for _ in 0..rep {
                                    if i >= lengths.len() {
                                        return Err("inflate: repeat overflow".into());
                                    }
                                    lengths[i] = prev;
                                    i += 1;
                                }
                            }
                            17 => {
                                let rep = 3 + br.bits(3)? as usize;
                                i += rep;
                            }
                            18 => {
                                let rep = 11 + br.bits(7)? as usize;
                                i += rep;
                            }
                            _ => return Err("inflate: bad cl symbol".into()),
                        }
                    }
                    if i > hlit + hdist {
                        return Err("inflate: lengths overflow".into());
                    }
                    (Huffman::build(&lengths[..hlit]), Huffman::build(&lengths[hlit..]))
                };
                loop {
                    let sym = lit.decode(&mut br)?;
                    if sym < 256 {
                        out.push(sym as u8);
                    } else if sym == 256 {
                        break;
                    } else {
                        let li = sym as usize - 257;
                        if li >= 29 {
                            return Err("inflate: bad length symbol".into());
                        }
                        let len = LEN_BASE[li] as usize + br.bits(LEN_EXTRA[li] as u32)? as usize;
                        let dsym = dist.decode(&mut br)? as usize;
                        if dsym >= 30 {
                            return Err("inflate: bad dist symbol".into());
                        }
                        let d = DIST_BASE[dsym] as usize + br.bits(DIST_EXTRA[dsym] as u32)? as usize;
                        if d > out.len() {
                            return Err("inflate: dist too far".into());
                        }
                        let start = out.len() - d;
                        for k in 0..len {
                            let b = out[start + k];
                            out.push(b);
                        }
                    }
                }
            }
            _ => return Err("inflate: bad block type".into()),
        }
        if last == 1 {
            break;
        }
    }
    Ok(out)
}

/// Gunzip (RFC1952) — reads a member, inflates, ignores CRC (oracle files are
/// trusted; the content comparison is the real gate).
pub fn gunzip(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b {
        return Err("gunzip: bad magic".into());
    }
    let flg = data[3];
    let mut pos = 10usize;
    if flg & 4 != 0 {
        let xlen = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2 + xlen;
    }
    if flg & 8 != 0 {
        while data[pos] != 0 {
            pos += 1;
        }
        pos += 1;
    }
    if flg & 16 != 0 {
        while data[pos] != 0 {
            pos += 1;
        }
        pos += 1;
    }
    if flg & 2 != 0 {
        pos += 2;
    }
    inflate(&data[pos..])
}

fn crc32(data: &[u8]) -> u32 {
    static mut TABLE: [u32; 256] = [0; 256];
    static INIT: std::sync::Once = std::sync::Once::new();
    unsafe {
        INIT.call_once(|| {
            for i in 0..256u32 {
                let mut c = i;
                for _ in 0..8 {
                    c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
                }
                TABLE[i as usize] = c;
            }
        });
        let mut c = 0xFFFF_FFFFu32;
        for &b in data {
            c = TABLE[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
        }
        c ^ 0xFFFF_FFFF
    }
}

/// Gzip with STORED deflate blocks (legal, python/java-compatible).
pub fn gzip_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff];
    // deflate stored blocks, 65535 per block
    let mut pos = 0;
    loop {
        let chunk_len = (data.len() - pos).min(65535);
        let last = pos + chunk_len >= data.len();
        out.push(if last { 1 } else { 0 });
        out.extend_from_slice(&(chunk_len as u16).to_le_bytes());
        out.extend_from_slice(&(!(chunk_len as u16)).to_le_bytes());
        out.extend_from_slice(&data[pos..pos + chunk_len]);
        pos += chunk_len;
        if last {
            break;
        }
    }
    out.extend_from_slice(&crc32(data).to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out
}

// ---------------------------------------------------------------------------
// NBT reader
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Nbt {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Nbt>),
    Comp(Vec<(String, Nbt)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Nbt {
    pub fn get(&self, key: &str) -> Option<&Nbt> {
        if let Nbt::Comp(fields) = self {
            fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Nbt::Int(v) => Some(*v),
            Nbt::Byte(v) => Some(*v as i32),
            Nbt::Short(v) => Some(*v as i32),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Nbt::String(s) => Some(s),
            _ => None,
        }
    }
}

struct NbtReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> NbtReader<'a> {
    fn u8(&mut self) -> Result<u8, String> {
        let v = *self.data.get(self.pos).ok_or("nbt: eof")?;
        self.pos += 1;
        Ok(v)
    }
    fn u16(&mut self) -> Result<u16, String> {
        let v = u16::from_be_bytes([self.u8()?, self.u8()?]);
        Ok(v)
    }
    fn i32(&mut self) -> Result<i32, String> {
        let mut b = [0u8; 4];
        for b in b.iter_mut() {
            *b = self.u8()?;
        }
        Ok(i32::from_be_bytes(b))
    }
    fn i64(&mut self) -> Result<i64, String> {
        let mut b = [0u8; 8];
        for b in b.iter_mut() {
            *b = self.u8()?;
        }
        Ok(i64::from_be_bytes(b))
    }
    fn string(&mut self) -> Result<String, String> {
        let len = self.u16()? as usize;
        let s = self.data.get(self.pos..self.pos + len).ok_or("nbt: eof str")?;
        self.pos += len;
        Ok(String::from_utf8_lossy(s).into_owned())
    }
    fn payload(&mut self, tag: u8) -> Result<Nbt, String> {
        Ok(match tag {
            1 => Nbt::Byte(self.u8()? as i8),
            2 => Nbt::Short(self.u16()? as i16),
            3 => Nbt::Int(self.i32()?),
            4 => Nbt::Long(self.i64()?),
            5 => Nbt::Float(f32::from_bits(self.i32()? as u32)),
            6 => Nbt::Double(f64::from_bits(self.i64()? as u64)),
            7 => {
                let len = self.i32()? as usize;
                let b = self.data.get(self.pos..self.pos + len).ok_or("nbt: eof bytes")?.to_vec();
                self.pos += len;
                Nbt::ByteArray(b)
            }
            8 => Nbt::String(self.string()?),
            9 => {
                let inner = self.u8()?;
                let len = self.i32()? as usize;
                let mut items = Vec::with_capacity(len.min(4096));
                for _ in 0..len {
                    if inner == 0 {
                        items.push(Nbt::Byte(0));
                    } else {
                        items.push(self.payload(inner)?);
                    }
                }
                Nbt::List(items)
            }
            10 => {
                let mut fields = Vec::new();
                loop {
                    let t = self.u8()?;
                    if t == 0 {
                        break;
                    }
                    let name = self.string()?;
                    let v = self.payload(t)?;
                    fields.push((name, v));
                }
                Nbt::Comp(fields)
            }
            11 => {
                let len = self.i32()? as usize;
                let mut v = Vec::with_capacity(len.min(1 << 20));
                for _ in 0..len {
                    v.push(self.i32()?);
                }
                Nbt::IntArray(v)
            }
            12 => {
                let len = self.i32()? as usize;
                let mut v = Vec::with_capacity(len.min(1 << 20));
                for _ in 0..len {
                    v.push(self.i64()?);
                }
                Nbt::LongArray(v)
            }
            _ => return Err(format!("nbt: unknown tag {tag}")),
        })
    }
}

pub fn nbt_parse(data: &[u8]) -> Result<Nbt, String> {
    let mut r = NbtReader { data, pos: 0 };
    let tag = r.u8()?;
    if tag != 10 {
        return Err("nbt: root must be compound".into());
    }
    let _name = r.string()?;
    r.payload(10)
}

// ---------------------------------------------------------------------------
// NBT writer (subset needed for the staged schema)
// ---------------------------------------------------------------------------

fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_be_bytes());
}
fn put_i32(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_be_bytes());
}
fn put_i64(out: &mut Vec<u8>, v: i64) {
    out.extend_from_slice(&v.to_be_bytes());
}
fn put_str(out: &mut Vec<u8>, s: &str) {
    put_u16(out, s.len() as u16);
    out.extend_from_slice(s.as_bytes());
}

fn write_payload(out: &mut Vec<u8>, v: &Nbt) {
    match v {
        Nbt::Byte(b) => out.push(*b as u8),
        Nbt::Short(s) => out.extend_from_slice(&s.to_be_bytes()),
        Nbt::Int(i) => put_i32(out, *i),
        Nbt::Long(l) => put_i64(out, *l),
        Nbt::Float(f) => put_i32(out, f.to_bits() as i32),
        Nbt::Double(d) => put_i64(out, d.to_bits() as i64),
        Nbt::ByteArray(b) => {
            put_i32(out, b.len() as i32);
            out.extend_from_slice(b);
        }
        Nbt::String(s) => put_str(out, s),
        Nbt::List(items) => {
            let inner = items.first().map(tag_of).unwrap_or(0);
            out.push(inner);
            put_i32(out, items.len() as i32);
            for it in items {
                write_payload(out, it);
            }
        }
        Nbt::Comp(fields) => {
            for (k, val) in fields {
                out.push(tag_of(val));
                put_str(out, k);
                write_payload(out, val);
            }
            out.push(0);
        }
        Nbt::IntArray(a) => {
            put_i32(out, a.len() as i32);
            for x in a {
                put_i32(out, *x);
            }
        }
        Nbt::LongArray(a) => {
            put_i32(out, a.len() as i32);
            for x in a {
                put_i64(out, *x);
            }
        }
    }
}

fn tag_of(v: &Nbt) -> u8 {
    match v {
        Nbt::Byte(_) => 1,
        Nbt::Short(_) => 2,
        Nbt::Int(_) => 3,
        Nbt::Long(_) => 4,
        Nbt::Float(_) => 5,
        Nbt::Double(_) => 6,
        Nbt::ByteArray(_) => 7,
        Nbt::String(_) => 8,
        Nbt::List(_) => 9,
        Nbt::Comp(_) => 10,
        Nbt::IntArray(_) => 11,
        Nbt::LongArray(_) => 12,
    }
}

/// Write a root compound named "" gzipped (NbtIo.writeCompressed form).
pub fn write_gzipped_nbt(root: &Nbt) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(10);
    put_str(&mut out, "");
    write_payload(&mut out, root);
    gzip_stored(&out)
}

// ---------------------------------------------------------------------------
// Staged chunk schema
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct StagedSection {
    pub y: i8,
    /// canonical blockstate strings, palette order preserved as in the file
    pub palette: Vec<String>,
    pub data: Vec<u32>,
    /// canonical biome strings
    pub biome_palette: Vec<String>,
    pub biome_data: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct StagedChunk {
    pub x: i32,
    pub z: i32,
    pub status: String,
    pub data_version: i32,
    pub min_y: i32,
    pub height: i32,
    pub sections: Vec<StagedSection>,
    /// (name, raw longs) — WORLDGEN heightmaps at this status
    pub heightmaps: Vec<(String, Vec<i64>)>,
    /// per section: packed post-processing shorts
    pub post_processing: Vec<Vec<u16>>,
}

fn state_to_canonical(comp: &Nbt) -> Result<String, String> {
    let name = comp.get("Name").and_then(|v| v.as_str()).ok_or("palette entry missing Name")?;
    let mut props: Vec<(String, String)> = Vec::new();
    if let Some(Nbt::Comp(fields)) = comp.get("Properties") {
        for (k, v) in fields {
            props.push((k.clone(), v.as_str().unwrap_or("").to_string()));
        }
    }
    props.sort();
    let mut s = name.to_string();
    if !props.is_empty() {
        let ps: Vec<String> = props.iter().map(|(k, v)| format!("{k}={v}")).collect();
        s.push_str(&format!("[{}]", ps.join(",")));
    }
    Ok(s)
}

/// Parse one staged file (gzipped NBT bytes).
pub fn parse_staged_file(bytes: &[u8]) -> Result<StagedChunk, String> {
    let raw = gunzip(bytes)?;
    let root = nbt_parse(&raw)?;
    let x = root.get("ChunkX").and_then(|v| v.as_int()).ok_or("missing ChunkX")?;
    let z = root.get("ChunkZ").and_then(|v| v.as_int()).ok_or("missing ChunkZ")?;
    let status = root.get("Status").and_then(|v| v.as_str()).ok_or("missing Status")?.to_string();
    let data_version = root.get("DataVersion").and_then(|v| v.as_int()).unwrap_or(0);
    let min_y = root.get("MinY").and_then(|v| v.as_int()).unwrap_or(-64);
    let height = root.get("Height").and_then(|v| v.as_int()).unwrap_or(384);
    let mut sections = Vec::new();
    if let Some(Nbt::List(list)) = root.get("Sections") {
        for sec in list {
            let y = sec.get("Y").and_then(|v| v.as_int()).ok_or("section missing Y")? as i8;
            let mut palette = Vec::new();
            if let Some(Nbt::List(pl)) = sec.get("Palette") {
                for e in pl {
                    palette.push(state_to_canonical(e)?);
                }
            }
            let data = match sec.get("Data") {
                Some(Nbt::IntArray(a)) => a.iter().map(|&v| v as u32).collect(),
                _ => return Err("section missing Data".into()),
            };
            sections.push(StagedSection { y, palette, data, biome_palette: Vec::new(), biome_data: Vec::new() });
        }
    }
    // Biomes: list of { Y:Byte, Palette: List<String>, Data: IntArray(64) }
    if let Some(Nbt::List(list)) = root.get("Biomes") {
        for (i, sec) in list.iter().enumerate() {
            let mut biome_palette = Vec::new();
            if let Some(Nbt::List(pl)) = sec.get("Palette") {
                for e in pl {
                    biome_palette.push(e.as_str().unwrap_or("").to_string());
                }
            }
            let biome_data = match sec.get("Data") {
                Some(Nbt::IntArray(a)) => a.iter().map(|&v| v as u32).collect(),
                _ => Vec::new(),
            };
            if i < sections.len() {
                sections[i].biome_palette = biome_palette;
                sections[i].biome_data = biome_data;
            }
        }
    }
    let mut heightmaps = Vec::new();
    if let Some(Nbt::Comp(fields)) = root.get("Heightmaps") {
        for (k, v) in fields {
            if let Nbt::LongArray(a) = v {
                heightmaps.push((k.clone(), a.clone()));
            }
        }
    }
    let mut post_processing = Vec::new();
    if let Some(Nbt::List(list)) = root.get("PostProcessing") {
        for sec in list {
            let mut pp = Vec::new();
            if let Nbt::List(items) = sec {
                for it in items {
                    if let Nbt::Short(s) = it {
                        pp.push(*s as u16);
                    }
                }
            }
            post_processing.push(pp);
        }
    }
    Ok(StagedChunk { x, z, status, data_version, min_y, height, sections, heightmaps, post_processing })
}

/// Build the Rust-side staged chunk from a FillerChunk (blocks+biomes from
/// the fill, heightmaps packed into long[37] 9-bit SimpleBitStorage form).
pub fn filler_to_staged(fc: &FillerChunk, seed_status: &str, data_version: i32) -> StagedChunk {
    let mut sections = Vec::with_capacity(fc.sections.len());
    for (i, sec) in fc.sections.iter().enumerate() {
        let y = (fc.min_y / 16 + i as i32) as i8;
        // first-encounter palette
        let mut palette: Vec<u32> = Vec::new();
        let mut index: HashMap<u32, u32> = HashMap::new();
        let mut data = vec![0u32; 4096];
        for (j, &s) in sec.states.iter().enumerate() {
            let pi = match index.get(&s) {
                Some(&p) => p,
                None => {
                    let p = palette.len() as u32;
                    palette.push(s);
                    index.insert(s, p);
                    p
                }
            };
            data[j] = pi;
        }
        let states: Vec<String> = palette.iter().map(|&s| fc.state_table.get(s).canonical()).collect();
        let mut bpalette: Vec<u16> = Vec::new();
        let mut bindex: HashMap<u16, u32> = HashMap::new();
        let mut bdata = vec![0u32; 64];
        for (j, &b) in sec.biomes.iter().enumerate() {
            let pi = match bindex.get(&b) {
                Some(&p) => p,
                None => {
                    let p = bpalette.len() as u32;
                    bpalette.push(b);
                    bindex.insert(b, p);
                    p
                }
            };
            bdata[j] = pi;
        }
        let biomes: Vec<String> = bpalette.iter().map(|&b| fc.biome_table.names[b as usize].clone()).collect();
        sections.push(StagedSection { y, palette: states, data, biome_palette: biomes, biome_data: bdata });
    }
    let mut heightmaps = Vec::new();
    for hm in &fc.heightmaps {
        // Heightmap.data = SimpleBitStorage(9, 256): value = stored (absolute -
        // minY); 7 entries per long, least significant first.
        let mut longs = vec![0i64; 37];
        for (col, &fa) in hm.first_available.iter().enumerate() {
            let stored = (fa - fc.min_y).max(0) as i64;
            longs[col / 7] |= stored << ((col % 7 * 9) as i32);
        }
        let name = match hm.kind {
            HeightmapKind::OceanFloorWg => "OCEAN_FLOOR_WG",
            HeightmapKind::WorldSurfaceWg => "WORLD_SURFACE_WG",
        };
        heightmaps.push((name.to_string(), longs));
    }
    StagedChunk {
        x: 0,
        z: 0,
        status: seed_status.to_string(),
        data_version,
        min_y: fc.min_y,
        height: fc.height,
        sections,
        heightmaps,
        post_processing: fc.post_processing.clone(),
    }
}

/// Serialize a StagedChunk to the staged file schema (gzipped NBT).
pub fn write_staged_file(sc: &StagedChunk) -> Vec<u8> {
    let mut sections = Vec::new();
    let mut biomes = Vec::new();
    for sec in &sc.sections {
        let palette: Vec<Nbt> = sec
            .palette
            .iter()
            .map(|s| {
                let def = BlockStateDef::parse(s);
                let mut fields: Vec<(String, Nbt)> = vec![("Name".to_string(), Nbt::String(def.name.clone()))];
                if !def.props.is_empty() {
                    let props: Vec<(String, Nbt)> =
                        def.props.iter().map(|(k, v)| (k.clone(), Nbt::String(v.clone()))).collect();
                    fields.push(("Properties".to_string(), Nbt::Comp(props)));
                }
                Nbt::Comp(fields)
            })
            .collect();
        let biome_palette: Vec<Nbt> = sec.biome_palette.iter().map(|s| Nbt::String(s.clone())).collect();
        sections.push(Nbt::Comp(vec![
            ("Y".to_string(), Nbt::Byte(sec.y)),
            ("Palette".to_string(), Nbt::List(palette)),
            (
                "Data".to_string(),
                Nbt::IntArray(sec.data.iter().map(|&v| v as i32).collect()),
            ),
        ]));
        biomes.push(Nbt::Comp(vec![
            ("Y".to_string(), Nbt::Byte(sec.y)),
            ("Palette".to_string(), Nbt::List(biome_palette)),
            (
                "Data".to_string(),
                Nbt::IntArray(sec.biome_data.iter().map(|&v| v as i32).collect()),
            ),
        ]));
    }
    let hm_fields: Vec<(String, Nbt)> = sc
        .heightmaps
        .iter()
        .map(|(k, v)| (k.clone(), Nbt::LongArray(v.clone())))
        .collect();
    let pp: Vec<Nbt> = sc
        .post_processing
        .iter()
        .map(|v| Nbt::List(v.iter().map(|&s| Nbt::Short(s as i16)).collect()))
        .collect();
    let root = Nbt::Comp(vec![
        ("ChunkX".to_string(), Nbt::Int(sc.x)),
        ("ChunkZ".to_string(), Nbt::Int(sc.z)),
        ("Status".to_string(), Nbt::String(sc.status.clone())),
        ("DataVersion".to_string(), Nbt::Int(sc.data_version)),
        ("MinY".to_string(), Nbt::Int(sc.min_y)),
        ("Height".to_string(), Nbt::Int(sc.height)),
        ("Sections".to_string(), Nbt::List(sections)),
        ("Biomes".to_string(), Nbt::List(biomes)),
        ("Heightmaps".to_string(), Nbt::Comp(hm_fields)),
        ("PostProcessing".to_string(), Nbt::List(pp)),
    ]);
    write_gzipped_nbt(&root)
}

/// Build a fresh StateTable from a StagedChunk (for stagediff on Java files).
pub fn state_table_from_staged(sc: &StagedChunk) -> StateTable {
    let mut t = StateTable::new();
    for sec in &sc.sections {
        for p in &sec.palette {
            t.intern_canonical(p);
        }
    }
    t
}
