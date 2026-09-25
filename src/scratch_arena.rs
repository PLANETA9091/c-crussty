//! SERIALIZATION SCRATCH-ARENA (ID-P27, TASK-459-63, закон-11 WILD — SCAFFOLD).
//!
//! Idea card: origin/round-458p-ideas:RESEARCH-458-P.md `[ID-P27]`; research:
//! RESEARCH-459-P27.md (foojay "Java is Very Fast" + agrona DirectBuffer docs).
//!
//! TARGET LANE (chunk4/5 MISS-path alloc burst): the chunk5 MISS path
//! (chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java
//! `encodePayload`) allocates a fresh `Unpooled.buffer(256)` (netty doubling
//! cascade) + a fresh `new byte[len]` copy-out PER MISS encode; the chunk4
//! MISS path (src/chunk_send.rs snapshot-first sender) re-runs the exact
//! vanilla construction (section buffers + heightmap NBT + block-entity tags)
//! the same way. The first 2 MISS encodes additionally re-encode a SECOND
//! time (online selftest) — the burst doubles in that window.
//!
//! P27 MECHANIC (three invariants, card-verbatim):
//!   1. per-thread arena slots (region_threads=4 -> 4 hot slots, zero
//!      contention by construction) with FULL OVERWRITE per session — every
//!      handed-off byte in [0, write_len) was appended by THIS session
//!      (enforced structurally: the slot only accepts bytes through
//!      `write()` after `reset_for_write()`; extend_from_slice from a cleared
//!      state), never a partial append on top of the previous lifetime;
//!   2. LENGTH CONTROL: `write_len` + capacity gate + expected-length seal
//!      in the re-encode selftest; growth = NEW slot (agrona
//!      ExpandableArrayBuffer canon: "when it needs to be resized, a new
//!      byte[] is created and the contents are copied over") — never an
//!      in-place resize of a live slot;
//!   3. ONE PROTECTIVE COPY ON HANDOFF: the codec/channel may hold the
//!      payload reference beyond the encode (foojay canon: "if this data has
//!      to be persisted it must first be copied, because objects are
//!      reused") — the slot never leaks.
//!
//! PARITY CONTRACT (law 4): the arena changes ONLY the allocator, never the
//! encoding — "байты те же". The re-encode selftest keeps its shape
//! (encode A -> fresh byte[], encode B -> arena slot, bit-in-bit compare +
//! length equality); a mismatch flips a one-shot DISARM latch and the
//! MISS-path serves vanilla `new byte[]` forever (fail-closed, byte-vanilla).
//!
//! THIS MODULE (scaffold): the arena ENGINE as a std-only protocol model +
//! cargo parity tests over seeded payloads (re-encode fresh-vs-arena
//! bit-equality, length control, no-stale-tails, grow protocol, burst-alloc
//! accounting). The JNI/activation wiring (early-define NCDFE canon
//! d73758a3/5ecd841a, ARM marker, lever gate) is the NEXT cycle and lives
//! dormant until then: nothing here is referenced from the hot path.
//!
//! Grep markers: "P27", "scratch_arena", "cmp459_scratcharena".

/// Card forecast: burst-alloc reduction band (fractions of fresh allocations
/// the arena must NOT perform on the steady-state MISS path).
pub const FORECAST_BAND: (f64, f64) = (0.15, 0.25);

/// Hot-slot bound per thread (region_threads=4 scene: one section payload +
/// one heightmap-NBT scratch slot is enough for the encode; the bound keeps
/// the pool bounded under pathological reentrancy). Overflow -> vanilla path.
pub const SLOTS_PER_THREAD: usize = 8;

/// Hard slot capacity (light-chunk bound): a request larger than this is
/// served vanilla (fail-closed) instead of pooling megabyte slots.
pub const SLOT_CAP: usize = 1 << 20;

/// A defect inside the arena protocol. Every defect arms the one-shot
/// DISARM latch: the arena serves nothing afterwards (fail-closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaDefect {
    /// encode() while the arena is disarmed (caller must serve vanilla).
    Disarmed,
    /// capacity request beyond SLOT_CAP (request property, pool stays alive
    /// in this model branch — but THIS encode is served vanilla).
    CapOverflow,
    /// capacity gate tripped mid-write (caller takes a NEW slot — the grow
    /// protocol — never an in-place resize).
    Overflow,
    /// write attempted outside an open session (aliasing guard).
    NoSession,
    /// sealed length != expected length (partial write / stale tail —
    /// the re-encode selftest always knows the fresh baseline length).
    LengthMismatch { got: usize, want: usize },
    /// codec error surfaced through the closure.
    CodecError,
}

/// One pooled scratch slot: fixed-capacity byte arena with write_len control.
#[derive(Debug)]
pub struct ScratchSlot {
    buf: Vec<u8>,
    write_len: usize,
    in_session: bool,
}

impl ScratchSlot {
    /// New empty slot. The capacity is the slot capacity (agrona canon:
    /// fixed-size slot; growth produces a NEW slot, never in-place resize).
    fn new(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
            write_len: 0,
            in_session: false,
        }
    }

    /// Opens a write session: full-overwrite discipline (cleared state, the
    /// ONLY path that may precede `write`). Bytes in [0, len) handed off
    /// later were ALL appended after this point.
    fn reset_for_write(&mut self) {
        self.buf.clear();
        self.write_len = 0;
        self.in_session = true;
    }

    /// Append bytes with the capacity gate; Err(Overflow) = the grow
    /// protocol belongs to the CALLER (request a NEW slot), never resize.
    fn write(&mut self, src: &[u8]) -> Result<(), ArenaDefect> {
        if !self.in_session {
            return Err(ArenaDefect::NoSession);
        }
        if self.buf.len() + src.len() > self.buf.capacity() {
            return Err(ArenaDefect::Overflow);
        }
        self.buf.extend_from_slice(src);
        Ok(())
    }

    /// Seal the session with LENGTH CONTROL: `expected` (known from the
    /// fresh baseline in the re-encode selftest) must match exactly.
    fn seal(&mut self, expected: Option<usize>) -> Result<usize, ArenaDefect> {
        if !self.in_session {
            return Err(ArenaDefect::NoSession);
        }
        if let Some(want) = expected {
            if self.buf.len() != want {
                let d = ArenaDefect::LengthMismatch {
                    got: self.buf.len(),
                    want,
                };
                self.in_session = false;
                return Err(d);
            }
        }
        self.write_len = self.buf.len();
        self.in_session = false;
        Ok(self.write_len)
    }

    /// THE PROTECTIVE COPY ON HANDOFF — the only way bytes leave the arena.
    /// The slot itself returns to the pool right after; the handed-off Vec
    /// is owned by the caller (codec/channel aliasing is safe).
    fn handoff(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.write_len);
        out.extend_from_slice(&self.buf[..self.write_len]);
        out
    }
}

/// Per-thread arena pool: reuses up to [`SLOTS_PER_THREAD`] retired slots
/// before allocating a fresh one (the burst-alloc saving is exactly the
/// retired-slot reuse count). SCAFFOLD NOTE: the real engine is per-thread
/// (region_threads=4 -> 4 independent pools); this model is single-pool and
/// the threading discipline is enforced by the activation wiring later.
#[derive(Debug, Default)]
pub struct ScratchArena {
    retired: Vec<ScratchSlot>,
    pub fresh_allocs: u64,
    pub slot_reuses: u64,
    pub cap_overflows: u64,
    pub defects: u64,
    pub disarmed: bool,
}

impl ScratchArena {
    pub fn new() -> Self {
        Self::default()
    }

    /// One-shot disarm latch (fail-closed): after ANY defect the MISS path
    /// serves vanilla `new byte[]` forever; the arena never serves again.
    pub fn disarm(&mut self) {
        self.disarmed = true;
        self.retired.clear();
    }

    fn take_slot(&mut self, cap: usize) -> ScratchSlot {
        while let Some(slot) = self.retired.pop() {
            if slot.buf.capacity() >= cap {
                self.slot_reuses += 1;
                return slot;
            }
            // undersized for this request: drop it and keep popping; the
            // pool stays bounded (no unbounded retention of big slots).
        }
        self.fresh_allocs += 1;
        ScratchSlot::new(cap)
    }

    /// Encode into the arena: `encode` writes the payload into the slot with
    /// full-overwrite semantics; the handoff is ONE protective copy and the
    /// slot auto-retires into the pool. `expected_len` (from the fresh
    /// baseline) arms the length-control seal; `None` = seal-as-written.
    /// On any defect the arena DISARMS and Err is returned (caller = vanilla).
    pub fn encode<E>(
        &mut self,
        cap: usize,
        expected_len: Option<usize>,
        mut encode: E,
    ) -> Result<Vec<u8>, ArenaDefect>
    where
        E: FnMut(&mut ScratchSlot) -> Result<(), ArenaDefect>,
    {
        if self.disarmed {
            return Err(ArenaDefect::Disarmed);
        }
        if cap > SLOT_CAP {
            self.cap_overflows += 1;
            return Err(ArenaDefect::CapOverflow);
        }
        let mut slot = self.take_slot(cap);
        slot.reset_for_write();
        if let Err(e) = encode(&mut slot) {
            self.defects += 1;
            self.disarm();
            return Err(e);
        }
        if let Err(e) = slot.seal(expected_len) {
            self.defects += 1;
            self.disarm();
            return Err(e);
        }
        let payload = slot.handoff();
        self.retire(slot);
        Ok(payload)
    }

    /// Retire a slot back into the pool (bounded by SLOTS_PER_THREAD).
    pub fn retire(&mut self, slot: ScratchSlot) {
        if self.disarmed {
            return;
        }
        if self.retired.len() < SLOTS_PER_THREAD {
            self.retired.push(slot);
        }
    }
}

/// The re-encode parity oracle (chunk5 selftest shape, arena semantics):
/// `fresh` = vanilla-allocator encode (baseline), `arena` = P27 encode.
/// Parity = bit-in-bit equality AND length equality (no stale-slot tails).
pub fn reencode_parity(fresh: &[u8], arena: &[u8]) -> bool {
    fresh.len() == arena.len() && fresh.iter().zip(arena.iter()).all(|(a, b)| a == b)
}

/// Burst-alloc model: the fraction of encodes served WITHOUT a fresh
/// allocation (reused slots / total encodes). The cargo tests use it to
/// check the card band [−15..25% fresh-alloc reduction] is reachable.
pub fn burst_saved_fresh_fraction(fresh_allocs: u64, slot_reuses: u64) -> f64 {
    let total = fresh_allocs + slot_reuses;
    if total == 0 {
        return 0.0;
    }
    slot_reuses as f64 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pseudo-encode: appends a seeded byte pattern of the
    /// requested length (stands in for chunkData.write / heightmap NBT).
    fn fill(slot: &mut ScratchSlot, len: usize, seed: u8) -> Result<(), ArenaDefect> {
        let mut chunk = [0u8; 64];
        let mut written = 0;
        while written < len {
            let n = chunk.len().min(len - written);
            for (i, b) in chunk.iter_mut().enumerate().take(n) {
                *b = seed.wrapping_add((written + i) as u8);
            }
            slot.write(&chunk[..n])?;
            written += n;
        }
        Ok(())
    }

    fn fresh_encode(len: usize, seed: u8) -> Vec<u8> {
        (0..len).map(|i| seed.wrapping_add(i as u8)).collect()
    }

    #[test]
    fn p27_reencode_parity_fresh_vs_arena() {
        for (len, seed) in [(0usize, 1u8), (1, 7), (255, 42), (256, 9), (1024, 200), (5000, 13)] {
            let fresh = fresh_encode(len, seed);
            let mut arena = ScratchArena::new();
            let out = arena
                .encode(len + 16, Some(len), |s| fill(s, len, seed))
                .expect("arena encode ok");
            assert!(reencode_parity(&fresh, &out), "len={len} parity broken");
        }
    }

    #[test]
    fn p27_length_control_catches_partial_writes() {
        // codec bug: writes 8 bytes where the fresh baseline is 64 —
        // the length-control seal must catch it and disarm (fail-closed).
        let mut arena = ScratchArena::new();
        let r = arena.encode(256, Some(64), |s| fill(s, 8, 1));
        match r {
            Err(ArenaDefect::LengthMismatch { got, want }) => {
                assert_eq!((got, want), (8, 64));
            }
            other => panic!("expected LengthMismatch, got {other:?}"),
        }
        assert!(arena.disarmed);
        assert!(arena.encode(64, None, |_| Ok(())).is_err());
    }

    #[test]
    fn p27_length_control_no_stale_tails() {
        let mut arena = ScratchArena::new();
        let big = arena.encode(4096, Some(4096), |s| fill(s, 4096, 5)).unwrap();
        assert_eq!(big.len(), 4096);
        let small = arena.encode(4096, Some(17), |s| fill(s, 17, 6)).unwrap();
        assert_eq!(small.len(), 17, "stale tail from previous lifetime leaked");
        assert!(reencode_parity(&fresh_encode(17, 6), &small));
    }

    #[test]
    fn p27_no_write_outside_session() {
        let mut slot = ScratchSlot::new(64);
        assert_eq!(slot.write(&[1u8; 4]), Err(ArenaDefect::NoSession));
        slot.reset_for_write();
        assert!(slot.write(&[1u8; 4]).is_ok());
        assert!(slot.seal(Some(4)).is_ok());
        // session closed: further writes rejected (aliasing guard)
        assert_eq!(slot.write(&[1u8; 4]), Err(ArenaDefect::NoSession));
    }

    #[test]
    fn p27_grow_protocol_new_slot_not_inplace() {
        let mut arena = ScratchArena::new();
        let a = arena.encode(128, Some(128), |s| fill(s, 128, 1)).unwrap();
        assert_eq!(a.len(), 128);
        assert_eq!(arena.fresh_allocs, 1);
        // grow: a BIGGER request than the retired (undersized) slot -> the
        // undersized slot is dropped and a NEW slot is allocated — the
        // agrona grow protocol, never an in-place resize of a live slot.
        let big = arena.encode(4096, Some(4096), |s| fill(s, 4096, 2)).unwrap();
        assert_eq!(big.len(), 4096);
        assert_eq!(arena.fresh_allocs, 2, "grow must be a new slot");
        // a smaller request after a big retired slot REUSES it (no shrink):
        let small = arena.encode(4096, Some(16), |s| fill(s, 16, 3)).unwrap();
        assert_eq!(small.len(), 16);
        assert_eq!(arena.fresh_allocs, 2, "reuse of the oversized slot");
        assert_eq!(arena.slot_reuses, 1);
    }

    #[test]
    fn p27_disarm_is_fail_closed() {
        let mut arena = ScratchArena::new();
        let r = arena.encode(64, Some(8), |s| {
            s.write(&[7u8; 8])?;
            Err(ArenaDefect::CodecError)
        });
        assert_eq!(r, Err(ArenaDefect::CodecError));
        assert!(arena.disarmed, "defect must flip the one-shot latch");
        assert!(arena.retired.is_empty());
        assert_eq!(arena.encode(64, None, |_| Ok(())), Err(ArenaDefect::Disarmed));
    }

    #[test]
    fn p27_burst_band_reachable() {
        // steady-state burst: 1 fresh slot, then N reuses (auto-retire) —
        // the saved fresh-alloc fraction must reach the card band start.
        let mut arena = ScratchArena::new();
        for i in 0..9u8 {
            let len = 2048 + i as usize;
            let out = arena.encode(4096, Some(len), |s| fill(s, len, i)).unwrap();
            assert_eq!(out.len(), len);
        }
        assert_eq!(arena.fresh_allocs, 1, "exactly one fresh slot for the burst");
        assert_eq!(arena.slot_reuses, 8);
        let frac = burst_saved_fresh_fraction(arena.fresh_allocs, arena.slot_reuses);
        assert!(
            frac >= FORECAST_BAND.0,
            "burst saving {frac:.2} below card band start {}",
            FORECAST_BAND.0
        );
    }

    #[test]
    fn p27_protective_copy_is_owned() {
        let mut arena = ScratchArena::new();
        let out = arena.encode(256, Some(128), |s| fill(s, 128, 3)).unwrap();
        // handoff must be an owned allocation — mutating it must not corrupt
        // the arena; a re-encode reproduces the same bytes bit-in-bit.
        let mut out2 = out.clone();
        out2[0] ^= 0xFF;
        let again = arena.encode(256, Some(128), |s| fill(s, 128, 3)).unwrap();
        assert_eq!(out, again);
        assert_ne!(out2, again);
    }

    #[test]
    fn p27_cap_overflow_is_fail_closed_request() {
        let mut arena = ScratchArena::new();
        let r = arena.encode(SLOT_CAP + 1, Some(16), |s| fill(s, 16, 1));
        assert_eq!(r, Err(ArenaDefect::CapOverflow));
        assert_eq!(arena.cap_overflows, 1);
        assert!(!arena.disarmed, "cap-overflow is a request property");
    }

    #[test]
    fn p27_slot_cap_bounded_pool() {
        let mut arena = ScratchArena::new();
        for i in 0..(SLOTS_PER_THREAD as u64 + 4) {
            let out = arena.encode(64, Some(8), |s| fill(s, 8, i as u8));
            assert!(out.is_ok());
        }
        assert!(arena.retired.len() <= SLOTS_PER_THREAD);
    }
}
