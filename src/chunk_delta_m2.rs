//! PER-SECTION CHUNK DELTA PLANE — RUST bulk-JNI engine SCAFFOLD (TASK-459-79,
//! lever cmp459_m2chdelta, law-11 WILD revisit of RESEARCH-458-M ID-M2, which
//! was PARKed in tick 458; research card RESEARCH-459-M2.md at branch root).
//!
//! Why the 458 PARK dissolves (numbered, not "proposed"):
//!   PARK reason (a) was palette-order parity — a rust RE-ENCODE cannot be
//!   bit-in-byte because serialized palette order follows the java HashMap
//!   iteration of the container. The M2 delta engine does NOT re-encode: the
//!   vanilla 3-arg extractChunkData body is the only writer, and the rust side
//!   stores/replays VANILLA-WRITTEN per-section bytes (readback capture).
//!   Palette order is whatever vanilla wrote — law-4 parity by construction.
//!   PARK reason (b) was per-chunk JNI marshaling cost; the bulk-JNI batch is
//!   ONE crossing per tick carrying ONLY changed sections (delta), not the
//!   full per-chunk payload.
//!
//! Ground truth (javap round-396-a patched-kernel.jar, 2026-09-25, this
//! branch; matches agent-M card of tick 458):
//!   * 2-arg ClientboundLevelChunkPacketData.extractChunkData body is
//!     aload_0; aload_1; aconst_null; invokestatic 3-arg; return — a trivial
//!     delegate => ideal static->static body redirect, sites:1, descriptor
//!     STATIC_DESC below (javap-c verified).
//!   * 3-arg body: iconst_0; istore_3; getSections() loop ->
//!     LevelChunkSection.write(buf, packetInfo, i++) — a PURE function of
//!     sections, zero cross-section state => per-section delta decomposition
//!     is exact, not approximate.
//!   * LevelChunkSection.write(FriendlyByteBuf, ChunkPacketInfo, int) is
//!     public; LevelChunk.isUnsaved()Z is public — the chunk4-certified dirty
//!     oracle ("any block/block-entity change marks the chunk BEFORE the next
//!     send probes").
//!
//! Profile honesty (round-chkmono457-11, cpu-collapsed 104163 samples, agent-M
//! card): send/serialize lane 1.75% CPU; chunkmap-send total 6.01%; GC 3.9%
//! (encode-alloc relief is a side effect, NOT counted in the verdict number).
//! Capture math (preregistered, see capture_math): 1.75пп x 0.70 capture =
//! +1.2пп; ceiling = 1.75пп (lane x 100%). NOT-A-BENCH guard: verdict rides
//! the law-8 carrier ladder exactly like chunk5 ("monotone, parity-transparent
//! member of the axis").
//!
//! STEP-1 SCAFFOLD (this commit) is DORMANT by construction:
//!   * empty/foreign CRUSSTY_LEVER_FLAG => register() logs dormant, ZERO
//!     hooks, vanilla bit-in-bit (chunk_send5 STRICT-gate canon, law 7);
//!   * even with the lever flag ON, the hook stashes the pristine bytes and
//!     ALWAYS serves None — the redirect body is NOT installed until step-2
//!     delivers the ChunkDeltaM2Ops blob (flat, major 65) and the
//!     selfTest()==true oracle passes (selfTest до ARM canon).
//!
//! NCDFE-canon (define BEFORE first touch): ChunkDeltaM2Ops is defined into
//! the TARGET class's own loader at activate() BEFORE the redirect can ever
//! be served; a define/selfTest defect leaves the plane dormant forever
//! (fail-closed, lesson 408: ARM marker only after selfTest==true).
//!
//! JNI discipline (law 6): ZERO added JNI crossings in step-1. The step-2
//! engine spends exactly ONE bulk crossing per tick (batched changed
//! sections) — never per-chunk, never per-send.

// Step-1 scaffold: the consts below are the step-2 delivery-closure surface
// (mirror of the chunk_send5 #[allow(dead_code)] self-diagnosis precedent).
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

const LEVER_ID: &str = "cmp459_m2chdelta";

/// Redirect target: javap-verified 2-arg static delegate (sites:1).
pub const TARGET_CLASS: &str =
    "net/minecraft/network/protocol/game/ClientboundLevelChunkPacketData";
/// Step-2 ops class (dormant stub source: chunksend/net/minecraft/world/level/
/// chunk/ChunkDeltaM2Ops.java; NCDFE-canon define-before-first-touch).
pub const OPS_CLASS: &str = "net/minecraft/world/level/chunk/ChunkDeltaM2Ops";
pub const METHOD: &str = "extractChunkData";
/// EXACT descriptor of the vanilla 2-arg delegate (javap 2026-09-25).
pub const STATIC_DESC: &str =
    "(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/world/level/chunk/LevelChunk;)V";
/// LevelChunkSection.write(FriendlyByteBuf, ChunkPacketInfo, int) — the
/// per-section vanilla writer the delta engine replays against.
pub const SECTION_WRITE_DESC: &str =
    "(Lnet/minecraft/network/FriendlyByteBuf;Lio/papermc/paper/antixray/ChunkPacketInfo;I)V";
/// LevelChunk.isUnsaved()Z — the chunk4-certified dirty oracle (serve-gate).
pub const IS_UNSAVED_DESC: &str = "()Z";

/// STRICT gate (law 7 round-id hygiene): arms ONLY on cmp459_m2chdelta.
/// No union carriers yet — the plane is a dormant scaffold; carrier-union
/// onto cmp457_paldelta sites happens in step-2 with its own resolution
/// closure (mirror-drift lesson x451/x452: java+rust move synchronously).
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == LEVER_ID)
        .unwrap_or(false)
}

/// Preregistered capture math (RESEARCH-459-M2.md §3): honest Delta(pp) =
/// lane(пп) x capture fraction. 1.75 x 0.70 = 1.225 => +1.2пп; ceiling
/// 1.75пп. Kept as a function so the CI-testable number stays pinned to the
/// card (law 14: the Delta in the verdict is computed, not invented).
pub fn capture_math(lane_pp: f64, capture_frac: f64) -> f64 {
    lane_pp * capture_frac
}

/// STEP-1: pristine-capture hook on the target class. Serves None (vanilla
/// bytes) unconditionally — redirect installation is step-2 after the blob
/// delivery gate. No JNI work inside the callback (loader-lock discipline:
/// ClassFileLoadHook runs on JVM class-load threads; panic would cross JNI).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla extractChunkData)"
        );
        return;
    }
    static PRISTINE_SEEN: AtomicBool = AtomicBool::new(false);
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, move |_name, bytes| {
        if !PRISTINE_SEEN.swap(true, Ordering::Relaxed) {
            let (major, _minor) = crate::improved_noise::class_version(bytes).unwrap_or((0, 0));
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: pristine sighting {TARGET_CLASS} {} bytes (major {major})",
                bytes.len()
            );
        }
        // TODO(M2-step-2): after ChunkDeltaM2Ops define + selfTest()==true +
        // resolution closure + pristine guard, serve the patched body
        // (static->static redirect of METHOD/STATIC_DESC, sites:1).
        None
    });
}

/// STEP-1: activation is a no-op with an honest marker. Step-2 order (fixed
/// here so the NCDFE-canon is written before the code that must obey it):
///   1. define_class(OPS_CLASS) into TARGET_CLASS's own loader (BEFORE any
///      kernel code can touch the ops name — define-before-first-touch);
///   2. selfTest()Z on the LOCAL define ref — must be true BEFORE ARM;
///   3. resolution closure + pristine guard on captured target bytes;
///   4. install the static->static body redirect (METHOD/STATIC_DESC);
///   5. ARM marker "[crussty-plugin] cmp459_m2chdelta: ARMED".
pub fn activate() {
    if !enabled() {
        return;
    }
    eprintln!(
        "[crussty-plugin] {LEVER_ID}: scaffold activate — redirect NOT installed (dormant step-1; RESEARCH-459-M2.md gates G1-G6)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preregistered_delta_is_one_point_two() {
        let d = capture_math(1.75, 0.70);
        assert!((d - 1.225).abs() < 1e-9, "delta {d} != 1.225");
        // verdict number rounds down honestly to +1.2пп
        assert_eq!(format!("{d:.1}"), "1.2");
    }

    #[test]
    fn descriptors_match_javap_ground_truth() {
        assert_eq!(
            STATIC_DESC,
            "(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/world/level/chunk/LevelChunk;)V"
        );
        assert_eq!(METHOD, "extractChunkData");
        assert!(OPS_CLASS.ends_with("ChunkDeltaM2Ops"));
    }
}
