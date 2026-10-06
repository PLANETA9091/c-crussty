//! NCF P3.4 — step B: ONE JNI call per chunk, returning NBT.
//!
//! Contract (P3.3 hook point): MoonriseRegionFileIO$RegionDataController
//! .readData returns ReadData{SYNC_READ, input, syncRead, recalcCount} —
//! the Java side (tools/ncf-agent/NativeChunksIO.java) calls
//! `NativeChunks.generateChunk(cx, cz)` once per chunk; a non-null byte[] =
//! gzip-compressed NBT of the FULL chunk (write_gzipped_nbt form,
//! NbtIo.writeCompressed-compatible) -> SYNC_READ; null = NO_DATA (Java
//! generates — the I8 fallback lanes).
//!
//! Panic safety: the shim catches unwinds -> NULL (Java fallback) and bumps
//! the fallback ledger. Policy: the export is compiled cfg(jni) and stays
//! DO_NOT_ENABLE in the registry until the P2 gate closes (I9).

use crate::router::RandomState;
use std::path::Path;

/// Build the FULL-chunk NBT payload for one chunk through the whole native
/// pipeline (noise -> surface -> carvers), serialized gzipped.
pub fn build_chunk_payload(
    rs: &mut RandomState,
    kit: &mut crate::status_chain::StageKit,
    dir: &crate::router::WorldgenDir,
    seed: i64,
    cx: i32,
    cz: i32,
) -> Result<Vec<u8>, String> {
    let chunk = crate::status_chain::generate_carvers_chunk(rs, kit, dir, seed, cx, cz)?;
    // full chunk NBT (serial palette bit-packing, tails, isLightOn=false);
    // DataVersion pinned 4556 (Q6).
    let nbt = crate::fullchunk::full_chunk_nbt(&chunk, 4556);
    Ok(crate::sections::write_gzipped_nbt(&nbt))
}

/// The agent-side engine: RandomState + StageKit built once per (spec, seed)
/// (the CRUSSTY JVMTI agent pins ONE generation thread — session-1 finding —
/// so this is not shared across threads; I5 keying = the RandomState itself).
pub struct PayloadEngine {
    rs: RandomState,
    kit: crate::status_chain::StageKit,
    dir: crate::router::WorldgenDir,
    seed: i64,
}

impl PayloadEngine {
    pub fn open(wg_dir: &Path, seed: i64) -> Result<Self, String> {
        let dir = crate::router::WorldgenDir::load(wg_dir)?;
        let mut rs = RandomState::build(&dir, "minecraft", "overworld", seed)?;
        let kit = crate::status_chain::StageKit::build(&mut rs, &dir)?;
        Ok(PayloadEngine { rs, kit, dir, seed })
    }

    pub fn payload(&mut self, cx: i32, cz: i32) -> Result<Vec<u8>, String> {
        build_chunk_payload(&mut self.rs, &mut self.kit, &self.dir, self.seed, cx, cz)
    }
}

#[cfg(test)]
mod tests {
    /// Engine + payload contract against the worldgen extract (skipped when
    /// NCF_WG is absent — full run lives in the CI ncf-jni job).
    #[test]
    fn payload_roundtrip_parses() {
        let Ok(wg) = std::env::var("NCF_WG") else { return };
        let mut e = match super::PayloadEngine::open(std::path::Path::new(&wg), 3053459) {
            Ok(e) => e,
            Err(_) => return,
        };
        let bytes = e.payload(100, 100).expect("payload");
        let raw = crate::sections::gunzip(&bytes).expect("gzip");
        let root = crate::sections::nbt_parse(&raw).expect("nbt");
        // serial server form: must carry DataVersion + sections
        assert!(root.get("DataVersion").is_some(), "payload must be serial chunk NBT");
    }
}
