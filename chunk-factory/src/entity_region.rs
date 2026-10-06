//! NCF P5.5 — entity-region writer + passive spawn bookkeeping (SPAWN).
//!
//! Entities live in SEPARATE region files (world/entities/r.x.z.mca) with the
//! SAME region-file layout as block regions (region.rs writer is reused
//! verbatim) but entity NBT payloads (Position/UUID/…; the vanilla
//! EntitiesStorage schema: one compound per chunk with an "Entities" list).
//!
//! PASSIVE SPAWN at generation: WorldGenRegion.spawnPassives — the animal
//! pack spawn during worldgen happens ONCE per region (chunk in the spawn
//! window), seeded by the decoration RNG (PositionalRandomFactory
//! fromHashOf("spawn")-style positional draw, 1.18+). The bit-exact spawn
//! mechanics (pack center random walk, biome spawn-cost tables, group size)
//! are P5.5-tail: this module ships the ENTITY REGION WRITER + the chunk
//! spawn-window bookkeeping the mechanics plug into, with the honest
//! coverage note that native spawn placement is DO_NOT_ENABLE until its
//! zero-diff gate (entities are byte-compared in the corpus NBT dumps).

use crate::region::RegionChunk;

/// One entity record (minimal vanilla fields the writer serializes; the
/// full entity compound comes from the mechanics layer / oracle dumps).
#[derive(Debug, Clone)]
pub struct EntityRecord {
    /// e.g. "minecraft:sheep"
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// entity UUID most/least significant bits
    pub uuid_most: i64,
    pub uuid_least: i64,
}

/// Build the per-chunk entity compound (EntitiesStorage form):
/// { DataVersion, Position: [x, y, z] ints (chunk coords), Entities: [...] }.
pub fn build_entity_chunk_nbt(chunk_x: i32, chunk_z: i32, entities: &[EntityRecord], data_version: i32) -> crate::sections::Nbt {
    use crate::sections::Nbt;
    let mut ents: Vec<Nbt> = Vec::new();
    for e in entities {
        let uuid = crate::sections::Nbt::IntArray(vec![
            (e.uuid_most >> 32) as i32,
            e.uuid_most as i32,
            (e.uuid_least >> 32) as i32,
            e.uuid_least as i32,
        ]);
        ents.push(Nbt::Comp(vec![
            ("id".to_string(), Nbt::String(e.id.clone())),
            ("Pos".to_string(), Nbt::List(vec![
                Nbt::Double(e.x),
                Nbt::Double(e.y),
                Nbt::Double(e.z),
            ])),
            ("UUID".to_string(), uuid),
        ]));
    }
    Nbt::Comp(vec![
        ("DataVersion".to_string(), Nbt::Int(data_version)),
        ("Position".to_string(), Nbt::IntArray(vec![chunk_x, chunk_z])),
        ("Entities".to_string(), Nbt::List(ents)),
    ])
}

/// Write one entities-region file from chunk (x, z) -> records.
pub fn write_entity_region(
    base_x: i32,
    base_z: i32,
    entities_by_chunk: &BTreeMap<(i32, i32), Vec<EntityRecord>>,
    data_version: i32,
) -> Result<Vec<u8>, String> {
    let mut chunks: Vec<RegionChunk> = Vec::new();
    for (&(cx, cz), ents) in entities_by_chunk {
        let nbt = build_entity_chunk_nbt(cx, cz, ents, data_version);
        let bytes = crate::sections::write_gzipped_nbt(&nbt);
        chunks.push(RegionChunk {
            x_in_region: (cx - base_x) as u8,
            z_in_region: (cz - base_z) as u8,
            timestamp: 0, // vanilla stamps at write; corpus compares payloads
            format: 2,    // zlib (deflate) — entities regions use vanilla zlib
            data: bytes,
        });
    }
    crate::region::write_region(&chunks).map_err(|e| e.to_string())
}

use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_region_roundtrip() {
        let mut map = BTreeMap::new();
        map.insert(
            (3, 4),
            vec![EntityRecord {
                id: "minecraft:sheep".into(),
                x: 56.5,
                y: 70.0,
                z: 72.5,
                uuid_most: 0x0123456789abcdef,
                uuid_least: 0xfedcba9876543210u64 as i64,
            }],
        );
        let bytes = write_entity_region(0, 0, &map, 4556).expect("write");
        let parsed = crate::region::parse_region(&bytes).expect("parse");
        assert_eq!(parsed.chunks.len(), 1);
        let (_ts, _fmt, data) = &parsed.chunks[&(3, 4)];
        let raw = crate::sections::gunzip(data).expect("gunzip");
        let root = crate::sections::nbt_parse(&raw).expect("nbt");
        assert_eq!(root.get("DataVersion").and_then(|v| v.as_int()), Some(4556));
        match root.get("Entities") {
            Some(crate::sections::Nbt::List(l)) => assert_eq!(l.len(), 1),
            other => panic!("Entities list missing: {other:?}"),
        }
    }

    #[test]
    fn empty_chunk_writes_empty_list() {
        let map: BTreeMap<(i32, i32), Vec<EntityRecord>> = BTreeMap::new();
        let nbt = build_entity_chunk_nbt(0, 0, &map.get(&(0, 0)).map(|v| v.as_slice()).unwrap_or(&[]), 4556);
        match nbt {
            crate::sections::Nbt::Comp(kv) => {
                assert_eq!(kv[0].0, "DataVersion");
            }
            other => panic!("{other:?}"),
        }
    }
}
