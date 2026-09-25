//! SHARDED ENTITY-CHUNK MIRROR for the getEntities broadphase lane
//! (TASK-405-C, vector eindex — levers cmp405_eindex | cmp458_roar; bridge
//! entityquery/net/minecraft/world/entity/EntityIndexOps.java, retargets in
//! src/classfile.rs patch_eindex_*, wiring in src/entity_index_manager.rs).
//!
//! TASK-458-K (vector roar — ID-H04 + ID-H06, lever cmp458_roar): the mirror
//! gains PER-SECTION (16³) occupancy on top of the chunk chains:
//!   - 64-section window per chunk entry (SEC_OFF=32 → secY ∈ [-32..31] =
//!     [-512..511] blocks; every vanilla dim fits: overworld -4..19,
//!     nether/end 0..15 — javap-verified vanilla sectionY =
//!     clamp(blockY>>4, minSection, maxSection), the SAME formula seedAll
//!     uses → note/seed sections are bit-identical to vanilla placement,
//!     so the y-range gate cannot false-negative by construction);
//!   - per-section singly-linked chains (add/remove/section-move ops in the
//!     SAME mirror op stream; section-move = ADD with same cell, other sec);
//!   - monotone 64-bit occupancy bitmap per chunk entry (roaring
//!     bitmap-container semantics inline — docs.rs/roaring 0.11.5 heap
//!     containers are incompatible with the fixed-BSS/seqlock discipline);
//!     bits never clear (like the k2 aggregate) — a stale bit only costs a
//!     head-load of an empty chain (1 cache line);
//!   - out-of-window sections (modded dims) live on the per-entry OVERFLOW
//!     chain (the old `head`) which count walks whenever non-empty;
//!   - count_chunk walks ONLY the vanilla scan's y-section range
//!     [clamp(floor(minY-2)>>4, minSec, maxSec) ..
//!      clamp(floor(maxY+2)>>4, minSec, maxSec)] — replicating the
//!     EntityCollectionBySection.getEntities bytecode verbatim — plus the
//!     overflow chain; the k2 hull stays as the x/z gate;
//!   - ID-H06: 4KB global bloom (512×AtomicU64, k=4, deterministic
//!     splitmix64 — docs.rs/bloom 0.3.2 is GPL-2.0 + allocs, rejected) of
//!     ever-occupied CHUNK keys, insert-only under WLOCK BEFORE publish →
//!     false negatives impossible by construction; probe precedes the shard
//!     probe (one bulk eidxFlushQuery filters the tick's query batch before
//!     any java traversal; fpr<2% up to ~3k occupied chunks per the
//!     (1-e^(-kn/m))^k math, dense fixtures degrade gracefully to the exact
//!     path — a bloom hit is a fallthrough, never a skip);
//!   - selfTest every 100 eidxFlushQuery calls: per chunk entry, walked
//!     chain nodes == chain_len AND every node's s_sec matches the section
//!     chain it was found on AND chain_len>0 ⇒ bloom probes true. Any
//!     violation → BROKEN → ERR_STRUCT forever (java broken=true → exact
//!     vanilla replication).
//!
//! ARCHITECTURE (mobs_grid.rs pattern B: 64 shards, per-shard seqlock):
//! the moonrise chunk-system entity storage (EntityLookup →
//! ChunkEntitySlices per chunk → per-section entity lists) is mirrored as a
//! fixed-capacity chunk-keyed index:
//!
//!     chunk key (cx,cz)  →  open-addressed key slot  →  chain of entity
//!     slots {id, bb[6]} — the entity HOME-CHUNK membership comes straight
//!     from the vanilla call sites (ChunkEntitySlices.add/removeEntity),
//!     the bb from the 5 setBoundingBox funnel sites;
//!     a parallel id table (id → cell+slot) makes REMOVE/BB O(1).
//!
//! The query native answers ONLY "which rect chunks hold ≥1 candidate":
//! for every chunk of the vanilla ±2-padded rect it counts slots whose bb
//! passes a RELAXED (non-strict) span test — the vanilla tail's strict
//! AABB.intersects is a subset, so count==0 implies the vanilla scan would
//! have contributed nothing (superset contract; false positives are harmless
//! extra vanilla scans, false negatives are impossible by construction).
//! Candidate chunks are scanned by the VANILLA ChunkEntitySlices.getEntities
//! — result order/filters/dedup/predicate stay bit-for-bit vanilla.
//!
//! SYNC: java note sites buffer rows per-thread (zero JNI per note); every
//! query drains ALL published buffers and applies them in ONE fused JNI
//! (eidxFlushQuery = flush + per-chunk counts). Writers hold one global
//! mutex (batches, not scalars — skipped entirely for empty batches, k2);
//! readers take per-shard seqlock snapshots (QRETRY budget → ERR_RANGE →
//! per-call vanilla fallback). k2: each chunk entry carries a monotone
//! CONSERVATIVE aggregate hull (min/max over every member bb ever stored),
//! so a query AABB that misses the hull returns count 0 without the chain
//! walk — hull-miss ⇒ no member can intersect (hull ⊇ every member bb).
//! Any structural failure (table overflow / probe exhaustion) → ERR_STRUCT
//! → the bridge disarms permanently (broken=true) → exact vanilla
//! replication forever.
//!
//! FAIL-CLOSED: natives are registered only under the STRICT-eq lever flag;
//! ERR codes never change the result list, only skip-or-fallback decisions.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Capacities (fixed statics — zero-filled, NEVER reallocated; BSS ~43 MB).
// ---------------------------------------------------------------------------

pub const NSHARDS: usize = 64;
const CHUNK_CAP: usize = 1 << 12; // chunk-key slots/shard → 262k chunks total
const SLOT_CAP: usize = 1 << 13;  // entity slots/shard → 512k total (150k pop)
const ID_CAP: usize = 1 << 13;    // id-table entries/shard → 512k distinct ids
const QRETRY: u32 = 256;          // per-chunk seqlock retry budget → ERR_RANGE
const PROBE_MAX: usize = 128;     // open-addressing probe budget → ERR_STRUCT

// TASK-458-K section plane: 64-section window, biased by SEC_OFF.
const SECTIONS: usize = 64;
const SEC_OFF: i32 = 32; // window secY ∈ [-32..31] → blocks [-512..511]
const BLOOM_WORDS: usize = 512; // 4KB (ID-H06)
const BLOOM_MASK: usize = BLOOM_WORDS - 1;
const BLOOM_K: usize = 4;
const SELFTEST_EVERY: u64 = 100; // invariant check cadence (query calls ≈ ticks)

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;
pub const PROBE_MAGIC: i32 = 0x4549_4445; // "EIDX"

const K0: AtomicI64 = AtomicI64::new(0);
const U0: AtomicU64 = AtomicU64::new(0);
const I0: AtomicI32 = AtomicI32::new(0);
const V0: AtomicUsize = AtomicUsize::new(0);
/// Unset aggregate marker (quiet NaN bits) — fails every ordered f64
/// comparison, so an unset aggregate reads as "no overlap" (safe: unset
/// means the chunk entry has no members yet → count 0 is exact).
const NAN_BITS: u64 = 0x7FF8_0000_0000_0000;

struct Shard {
    /// Seqlock version: even = stable, odd = write in flight (monotonic).
    ver: AtomicUsize,
    /// Open-addressed chunk keys (forced non-zero; 0 = free slot).
    keys: [AtomicI64; CHUNK_CAP],
    /// Per-key chain head: slot+1 (0 = empty chain).
    head: [AtomicI32; CHUNK_CAP],
    /// Entity slots: s_id = id+1 (0 = free), s_next = next slot+1, bb bits.
    s_id: [AtomicI32; SLOT_CAP],
    s_next: [AtomicI32; SLOT_CAP],
    s_bb: [AtomicU64; 6 * SLOT_CAP],
    /// Per-chunk CONSERVATIVE aggregate bb (k2): min-of-mins/max-of-maxes
    /// over every bb ever stored in the chunk (monotone — REMOVE never
    /// shrinks). Unset = NaN bits. A count query whose AABB misses the
    /// aggregate can return 0 WITHOUT the chain walk (superset contract:
    /// no member bb can overlap if the aggregate does not). Bits only;
    /// compare via f64 after from_bits.
    a_bb: [AtomicU64; 6 * CHUNK_CAP],
    /// id table (open-addressed): i_key = id+1 (0 = free, never erased —
    /// dead entries have i_slot == 0 and are revived by id reuse).
    i_key: [AtomicI64; ID_CAP],
    i_cell: [AtomicI64; ID_CAP],
    i_slot: [AtomicI32; ID_CAP],
    /// TASK-458-K: per-SECTION chain heads (slot+1), window-biased
    /// (secY+SEC_OFF ∈ [0..63]); index = ki*SECTIONS + w. The per-entry
    /// `head` above is repurposed as the OVERFLOW chain (out-of-window
    /// sections / s_sec==0 slots).
    s_head: [AtomicI32; SECTIONS * CHUNK_CAP],
    /// Per-slot biased home section+1 (0 = overflow / free slot).
    s_sec: [AtomicI32; SLOT_CAP],
    /// Monotone per-entry section occupancy bitmap (bit w set ⇔ some entity
    /// was EVER added at window section w; never cleared — readers get a
    /// 1-load reject, stale bits fall through to (cheap) empty chain heads).
    sec_bits: [AtomicU64; CHUNK_CAP],
    /// Live-slot count per chunk entry (selfTest invariant vs chain walk).
    chain_len: [AtomicI32; CHUNK_CAP],
}

static SHARDS: [Shard; NSHARDS] = [const {
    Shard {
        ver: V0,
        keys: [K0; CHUNK_CAP],
        head: [I0; CHUNK_CAP],
        s_id: [I0; SLOT_CAP],
        s_next: [I0; SLOT_CAP],
        s_bb: [U0; 6 * SLOT_CAP],
        a_bb: [U0; 6 * CHUNK_CAP],
        i_key: [K0; ID_CAP],
        i_cell: [K0; ID_CAP],
        i_slot: [I0; ID_CAP],
        s_head: [I0; SECTIONS * CHUNK_CAP],
        s_sec: [I0; SLOT_CAP],
        sec_bits: [U0; CHUNK_CAP],
        chain_len: [I0; CHUNK_CAP],
    }
}; NSHARDS];

/// TASK-458-K (ID-H06): 4KB global bloom of ever-occupied CHUNK keys.
/// Insert-only (monotone): a deletion would risk a false negative; stale
/// members only cost an exact-path fallthrough. Writers insert under the
/// global WSTATE lock strictly BEFORE the seqlock publish, so any reader
/// that observes a chunk's chain also observes its bloom bit (drained ops
/// are applied by the querying thread itself — program order precedes the
/// probes; foreign threads are fenced by the shard ver Release/Acquire).
static BLOOM: [AtomicU64; BLOOM_WORDS] = [U0; BLOOM_WORDS];

/// Sticky structural failure of the section/selfTest plane → every native
/// returns ERR_STRUCT → the java bridge sets broken=true (vanilla forever).
static BROKEN: AtomicBool = AtomicBool::new(false);

/// Query-call counter (≈ tick cadence): drives the every-100 selfTest.
static QCOUNT: AtomicU64 = AtomicU64::new(0);

/// Writer-only state (under WLOCK): slot watermark + per-shard free stacks.
struct WState {
    hi: [usize; NSHARDS],
    free: [Vec<i32>; NSHARDS],
}

static WSTATE: Mutex<WState> = Mutex::new(WState {
    hi: [0; NSHARDS],
    free: [const { Vec::new() }; NSHARDS],
});

// ---------------------------------------------------------------------------
// Keys / hashing.
// ---------------------------------------------------------------------------

/// Packed chunk key, forced non-zero (bijective XOR; 0x5555..5555 is far
/// outside reachable chunk coords ±1.9M).
#[inline]
fn chunk_key(cx: i32, cz: i32) -> i64 {
    (((cx as i64) << 32) | (cz as i64 & 0xFFFF_FFFF)) ^ 0x5555_5555_5555_5555
}

#[inline]
fn shard_of(key: i64) -> usize {
    ((key as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 58) as usize
}

#[inline]
fn id_key(id: i32) -> i64 {
    (id as i64) + 1
}

#[inline]
fn id_shard(id: i32) -> usize {
    (id as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) as usize & (NSHARDS - 1)
}

// ---------------------------------------------------------------------------
// Seqlock reader.
// ---------------------------------------------------------------------------

#[inline]
fn stable(sh: &Shard) -> (bool, usize) {
    let v1 = sh.ver.load(Ordering::Acquire);
    if v1 & 1 == 1 {
        return (false, v1);
    }
    (true, v1)
}

#[inline]
fn committed(sh: &Shard, v1: usize) -> bool {
    sh.ver.load(Ordering::Acquire) == v1
}

// ---------------------------------------------------------------------------
// Op application (caller holds WSTATE lock).
// ---------------------------------------------------------------------------

/// Probe the id table for `id`. Returns Some(entry) if present. Entry.0 =
/// table index, .1 = cell key, .2 = slot idx. Dead entries (i_slot == 0)
/// count as present-with-slot-0 (revivable).
fn id_probe(sh: &Shard, id: i32) -> Option<(usize, i64, usize)> {
    let k = id_key(id);
    let mut p = (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) as usize & (ID_CAP - 1);
    for _ in 0..PROBE_MAX {
        let cur = sh.i_key[p].load(Ordering::Acquire);
        if cur == 0 {
            return None; // free slot ends the chain (no tombstone holes)
        }
        if cur == k {
            let cell = sh.i_cell[p].load(Ordering::Acquire);
            let slot = sh.i_slot[p].load(Ordering::Acquire);
            return Some((p, cell, (slot - 1).max(-1) as usize));
        }
        p = (p + 1) & (ID_CAP - 1);
    }
    None
}

/// (WLOCK) insert-or-revive an id entry; returns (entry_idx, is_new_key).
fn id_insert(sh: &Shard, id: i32) -> Result<usize, i32> {
    let k = id_key(id);
    let mut p = (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) as usize & (ID_CAP - 1);
    for _ in 0..PROBE_MAX {
        match sh.i_key[p].load(Ordering::Acquire) {
            0 => {
                sh.i_key[p].store(k, Ordering::Release);
                sh.i_cell[p].store(0, Ordering::Relaxed);
                sh.i_slot[p].store(0, Ordering::Relaxed);
                return Ok(p);
            }
            cur if cur == k => return Ok(p),
            _ => {}
        }
        p = (p + 1) & (ID_CAP - 1);
    }
    Err(ERR_STRUCT) // table full of distinct ids
}

/// Probe the chunk table for `key` under a seqlock-consistent read.
fn chunk_find(sh: &Shard, key: i64) -> Option<usize> {
    let mut p = shard_of(key) & (CHUNK_CAP - 1);
    for _ in 0..PROBE_MAX {
        let cur = sh.keys[p].load(Ordering::Acquire);
        if cur == key {
            return Some(p);
        }
        if cur == 0 {
            return None;
        }
        p = (p + 1) & (CHUNK_CAP - 1);
    }
    None
}

/// (WLOCK) chunk-table find-or-insert. Fresh entries get a NaN aggregate
/// (no members yet) BEFORE publish — readers only see stable windows.
fn chunk_entry(sh: &Shard, key: i64) -> Result<usize, i32> {
    let mut p = shard_of(key) & (CHUNK_CAP - 1);
    for _ in 0..PROBE_MAX {
        let cur = sh.keys[p].load(Ordering::Acquire);
        if cur == key {
            return Ok(p);
        }
        if cur == 0 {
            sh.keys[p].store(key, Ordering::Release);
            sh.head[p].store(0, Ordering::Relaxed);
            let mut j = 0;
            while j < 6 {
                sh.a_bb[6 * p + j].store(NAN_BITS, Ordering::Relaxed);
                j += 1;
            }
            return Ok(p);
        }
        p = (p + 1) & (CHUNK_CAP - 1);
    }
    Err(ERR_STRUCT)
}

/// (WLOCK) allocate a slot from the shard pool (free stack, else watermark).
fn slot_alloc(st: &mut WState, s: usize) -> Result<usize, i32> {
    if let Some(slot) = st.free[s].pop() {
        return Ok(slot as usize);
    }
    if st.hi[s] < SLOT_CAP {
        let slot = st.hi[s];
        st.hi[s] = slot + 1;
        return Ok(slot);
    }
    Err(ERR_STRUCT) // pool exhausted
}

/// (WLOCK) push-front a slot onto the chain of chunk entry `ki` (OVERFLOW
/// chain — out-of-window sections).
fn chain_push(sh: &Shard, ki: usize, slot: usize) {
    let old = sh.head[ki].load(Ordering::Acquire);
    sh.s_next[slot].store(old, Ordering::Relaxed);
    sh.head[ki].store(slot as i32 + 1, Ordering::Release);
}

/// (WLOCK) push-front a slot onto the SECTION chain `w` of chunk entry `ki`.
fn sec_push(sh: &Shard, ki: usize, w: usize, slot: usize) {
    let old = sh.s_head[ki * SECTIONS + w].load(Ordering::Acquire);
    sh.s_next[slot].store(old, Ordering::Relaxed);
    sh.s_head[ki * SECTIONS + w].store(slot as i32 + 1, Ordering::Release);
}

/// (WLOCK) unlink slot `slot` (id+1 = want) from a chain whose head lives in
/// `heads[idx]`. Returns false if not found (ghost — tolerated as no-op).
#[inline]
fn chain_unlink_at(heads: &[AtomicI32], idx: usize, sh: &Shard, slot: usize, want: i32) -> bool {
    let mut cur = heads[idx].load(Ordering::Acquire);
    let mut prev: usize = usize::MAX;
    while cur != 0 {
        let s = (cur - 1) as usize;
        if s == slot && sh.s_id[s].load(Ordering::Acquire) == want {
            let nxt = sh.s_next[s].load(Ordering::Acquire);
            if prev == usize::MAX {
                heads[idx].store(nxt, Ordering::Release);
            } else {
                sh.s_next[prev].store(nxt, Ordering::Release);
            }
            return true;
        }
        prev = s;
        cur = sh.s_next[s].load(Ordering::Relaxed);
    }
    false
}

/// (WLOCK) unlink slot `slot` (id+1 = want) from the OVERFLOW chain of chunk
/// entry `ki`. Returns false if not found (ghost — tolerated as no-op).
fn chain_unlink(sh: &Shard, ki: usize, slot: usize, want: i32) -> bool {
    chain_unlink_at(&sh.head, ki, sh, slot, want)
}

/// (WLOCK) push-front a slot onto the chain selected by its BIASED section
/// (0 = overflow chain, else window section w = biased-1).
fn push_slot(sh: &Shard, ki: usize, biased: i32, slot: usize) {
    if biased == 0 {
        chain_push(sh, ki, slot);
    } else {
        let w = (biased - 1) as usize;
        sec_push(sh, ki, w, slot);
        sh.sec_bits[ki].fetch_or(1u64 << w, Ordering::Relaxed);
    }
}

/// (WLOCK) unlink a slot from the chain selected by its BIASED section.
fn unlink_slot(sh: &Shard, ki: usize, biased: i32, slot: usize, want: i32) -> bool {
    if biased == 0 {
        chain_unlink(sh, ki, slot, want)
    } else {
        let w = (biased - 1) as usize;
        chain_unlink_at(&sh.s_head, ki * SECTIONS + w, sh, slot, want)
    }
}

fn write_bb(sh: &Shard, slot: usize, bb: &[f64; 6]) {
    for j in 0..6 {
        sh.s_bb[6 * slot + j].store(bb[j].to_bits(), Ordering::Relaxed);
    }
}

// ---------------------------------------------------------------------------
// TASK-458-K section helpers + ID-H06 bloom.
// ---------------------------------------------------------------------------

/// Window index of an absolute sectionY; None → overflow chain.
#[inline]
fn sec_window(sec: i32) -> Option<usize> {
    let w = sec + SEC_OFF;
    if w >= 0 && (w as usize) < SECTIONS {
        Some(w as usize)
    } else {
        None
    }
}

/// Deterministic splitmix64 (no seeds, no RNG state — bit-stable runs).
#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// k=4 word indices for a chunk key (deterministic double-hash family).
#[inline]
fn bloom_idx(key: i64) -> [usize; BLOOM_K] {
    let h = mix64(key as u64 ^ 0xD1B5_4A32_D192_ED03);
    let h2 = (h >> 27) | 1; // odd stride
    let mut out = [0usize; BLOOM_K];
    let mut i = 0;
    while i < BLOOM_K {
        out[i] = (h.wrapping_add((i as u64).wrapping_mul(h2)) as usize) & BLOOM_MASK;
        i += 1;
    }
    out
}

/// (WLOCK, before publish) mark a chunk key ever-occupied.
#[inline]
fn bloom_add(key: i64) {
    for w in bloom_idx(key) {
        let bit = 1u64 << (key as u64 % 64);
        BLOOM[w].fetch_or(bit, Ordering::Release);
    }
}

/// Lock-free probe (Relaxed — a miss on a mid-insert key falls through to
/// the exact path; the querying thread always observes its own inserts).
#[inline]
fn bloom_probe(key: i64) -> bool {
    let bit = 1u64 << (key as u64 % 64);
    for w in bloom_idx(key) {
        if BLOOM[w].load(Ordering::Relaxed) & bit == 0 {
            return false;
        }
    }
    true
}

/// Vanilla scan y-section range, replicating
/// EntityCollectionBySection.getEntities bytecode verbatim:
///   secMin = clamp(Mth.floor(box.minY - 2.0) >> 4, minSection, maxSection)
///   secMax = clamp(Mth.floor(box.maxY + 2.0) >> 4, minSection, maxSection)
/// (Mth.floor = (int)Math.floor; >> arithmetic; clamp monotone ⇒ lo ≤ hi).
fn vanilla_section_range(b: &[f64; 6], min_sec: i32, max_sec: i32) -> (i32, i32) {
    let lo = ((b[1] - 2.0).floor() as i64 >> 4) as i32;
    let hi = ((b[4] + 2.0).floor() as i64 >> 4) as i32;
    let cl = |v: i32| v.max(min_sec).min(max_sec);
    (cl(lo), cl(hi))
}

/// (WLOCK, shard odd) Monotone aggregate expansion: agg = hull(agg, bb).
/// NaN current → adopt v (first member). Never shrinks → readers may use a
/// stale-large aggregate (superset preserved; only false positives).
fn agg_expand(sh: &Shard, ki: usize, bb: &[f64; 6]) {
    for j in 0..3 {
        let cur = f64::from_bits(sh.a_bb[6 * ki + j].load(Ordering::Relaxed));
        let v = bb[j];
        if !(cur <= v) {
            sh.a_bb[6 * ki + j].store(v.to_bits(), Ordering::Relaxed);
        }
    }
    for j in 3..6 {
        let cur = f64::from_bits(sh.a_bb[6 * ki + j].load(Ordering::Relaxed));
        let v = bb[j];
        if !(cur >= v) {
            sh.a_bb[6 * ki + j].store(v.to_bits(), Ordering::Relaxed);
        }
    }
}

/// One sync op: (op, id, cx, cz, sec, bb6). op: 0=BB, 1=ADD, 2=REMOVE.
/// `sec` = vanilla sectionY (ADD/REMOVE note sites + seed; REMOVE's is
/// informational — the authoritative home section lives in s_sec).
struct Op {
    op: u8,
    id: i32,
    cx: i32,
    cz: i32,
    sec: i32,
    bb: [f64; 6],
}

fn apply_ops(ops: &[Op]) -> i32 {
    if BROKEN.load(Ordering::Acquire) {
        return ERR_STRUCT; // section/selfTest plane failed — vanilla forever
    }
    if ops.is_empty() {
        // k2: queries with no pending notes (the common case) must not pay
        // the global writer mutex — applying nothing changes nothing and
        // every shard version stays even/stable.
        return 0;
    }
    let mut st = match WSTATE.lock() {
        Ok(g) => g,
        Err(_) => return ERR_STRUCT,
    };
    let mut touched: u64 = 0;
    let mut rc = 0;
    macro_rules! bump_odd {
        ($s:expr) => {{
            let bit = 1u64 << $s;
            if touched & bit == 0 {
                SHARDS[$s].ver.fetch_add(1, Ordering::Release); // even → odd
                touched |= bit;
            }
        }};
    }
    for op in ops {
        let ishard = id_shard(op.id);
        bump_odd!(ishard);
        let ish = &SHARDS[ishard];
        match op.op {
            1 => {
                // ADD: upsert home cell + SECTION + bb (idempotent by id).
                // TASK-458-K: section-move = same cell, other window section.
                let ckey = chunk_key(op.cx, op.cz);
                let cshard = shard_of(ckey);
                bump_odd!(cshard);
                // ID-H06: mark ever-occupied BEFORE any publish (WLOCK held).
                bloom_add(ckey);
                let want = id_key(op.id) as i32;
                let new_b = sec_window(op.sec).map(|w| w as i32 + 1).unwrap_or(0);
                let entry = match id_probe(ish, op.id) {
                    Some((ei, old_cell, slot)) if slot != usize::MAX && old_cell == ckey => {
                        // same cell — bb update + possible SECTION-MOVE
                        let sh = &SHARDS[cshard];
                        if let Some(ki) = chunk_find(sh, ckey) {
                            let old_b = sh.s_sec[slot].load(Ordering::Acquire);
                            if old_b != new_b {
                                // re-home inside the chunk: old section chain
                                // → new section chain (count unchanged).
                                if unlink_slot(sh, ki, old_b, slot, want) {
                                    push_slot(sh, ki, new_b, slot);
                                    sh.s_sec[slot].store(new_b, Ordering::Relaxed);
                                }
                            }
                            write_bb(sh, slot, &op.bb);
                            agg_expand(sh, ki, &op.bb);
                        }
                        let _ = ei;
                        continue;
                    }
                    Some((ei, _old_cell, _slot)) => (ei, true), // move or revive
                    None => match id_insert(ish, op.id) {
                        Ok(ei) => (ei, true),
                        Err(e) => {
                            rc = e;
                            break;
                        }
                    },
                };
                let (ei, fresh) = entry;
                if fresh {
                    // detach from the old cell (if any), then insert into the new
                    let old_cell = ish.i_cell[ei].load(Ordering::Acquire);
                    let old_slot = ish.i_slot[ei].load(Ordering::Acquire);
                    if old_slot != 0 {
                        let os = (old_slot - 1) as usize;
                        if old_cell != 0 {
                            let ocshard = shard_of(old_cell);
                            bump_odd!(ocshard);
                            let osh = &SHARDS[ocshard];
                            if let Some(ki) = chunk_find(osh, old_cell) {
                                let old_b = osh.s_sec[os].load(Ordering::Acquire);
                                if unlink_slot(osh, ki, old_b, os, want) {
                                    st.free[ocshard].push(os as i32);
                                    osh.chain_len[ki].fetch_sub(1, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                    let csh = &SHARDS[cshard];
                    let ki = match chunk_entry(csh, ckey) {
                        Ok(k) => k,
                        Err(e) => {
                            rc = e;
                            break;
                        }
                    };
                    let slot = match slot_alloc(&mut st, cshard) {
                        Ok(s) => s,
                        Err(e) => {
                            rc = e;
                            break;
                        }
                    };
                    csh.s_id[slot].store(want, Ordering::Relaxed);
                    csh.s_sec[slot].store(new_b, Ordering::Relaxed);
                    write_bb(csh, slot, &op.bb);
                    agg_expand(csh, ki, &op.bb);
                    push_slot(csh, ki, new_b, slot);
                    csh.chain_len[ki].fetch_add(1, Ordering::Relaxed);
                    ish.i_cell[ei].store(ckey, Ordering::Release);
                    ish.i_slot[ei].store(slot as i32 + 1, Ordering::Release);
                }
            }
            2 => {
                // REMOVE: drop the id from its SECTION chain (ghost = no-op).
                // The authoritative home section is the slot's own s_sec
                // (bit-identical to vanilla placement by the javap-verified
                // note-site contract); sec_bits stay SET (monotone — a stale
                // bit costs one empty-head load on the read path).
                if let Some((ei, old_cell, slot)) = id_probe(ish, op.id) {
                    if slot != usize::MAX && old_cell != 0 {
                        let cshard = shard_of(old_cell);
                        bump_odd!(cshard);
                        let csh = &SHARDS[cshard];
                        if let Some(ki) = chunk_find(csh, old_cell) {
                            let os = (slot - 1) as usize;
                            let old_b = csh.s_sec[os].load(Ordering::Acquire);
                            if unlink_slot(csh, ki, old_b, os, id_key(op.id) as i32) {
                                csh.s_id[os].store(0, Ordering::Relaxed);
                                csh.s_sec[os].store(0, Ordering::Relaxed);
                                csh.chain_len[ki].fetch_sub(1, Ordering::Relaxed);
                                st.free[cshard].push(os as i32);
                            }
                        }
                    }
                    ish.i_slot[ei].store(0, Ordering::Release); // dead, key kept
                }
            }
            _ => {
                // BB: update the stored bb (cell unchanged).
                if let Some((_ei, old_cell, slot)) = id_probe(ish, op.id) {
                    if slot != usize::MAX && old_cell != 0 {
                        let cshard = shard_of(old_cell);
                        bump_odd!(cshard);
                        let csh = &SHARDS[cshard];
                        if let Some(ki) = chunk_find(csh, old_cell) {
                            write_bb(csh, slot, &op.bb);
                            agg_expand(csh, ki, &op.bb);
                        }
                    }
                }
            }
        }
    }
    // even → stable (release publishes every mutation above)
    let mut m = touched;
    while m != 0 {
        let s = m.trailing_zeros() as usize;
        SHARDS[s].ver.fetch_add(1, Ordering::Release); // odd → even
        m &= m - 1;
    }
    rc
}

// ---------------------------------------------------------------------------
// Query (per-chunk candidate counts over the vanilla ±2-padded rect).
// ---------------------------------------------------------------------------

/// RELAXED (non-strict) span test — superset of vanilla's strict
/// AABB.intersects: count==0 must imply "no strict intersection possible".
#[inline]
fn bb_overlaps(b: &[f64; 6], bb: &[f64; 6]) -> bool {
    b[0] <= bb[3] && b[3] >= bb[0] && b[1] <= bb[4] && b[4] >= bb[1] && b[2] <= bb[5] && b[5] >= bb[2]
}

fn count_chunk(cx: i32, cz: i32, b: &[f64; 6], min_sec: i32, max_sec: i32) -> Result<i32, i32> {
    if BROKEN.load(Ordering::Acquire) {
        return Err(ERR_STRUCT);
    }
    let key = chunk_key(cx, cz);
    // ID-H06 pre-gate: a bloom miss means the chunk never held an entity →
    // the exact path would answer 0 (FN impossible: insert-before-publish).
    if !bloom_probe(key) {
        return Ok(0);
    }
    let sh = &SHARDS[shard_of(key)];
    let mut tries: u32 = 0;
    loop {
        tries += 1;
        if tries > QRETRY {
            return Err(ERR_RANGE); // writer storm → per-call vanilla fallback
        }
        let (ok, v1) = stable(sh);
        if !ok {
            std::hint::spin_loop();
            continue;
        }
        let mut count: i32 = 0;
        match chunk_find(sh, key) {
            None => {}
            Some(ki) => {
                // k2 fast path: the conservative aggregate hull answers
                // "definitely no candidate" without the chain walk. NaN
                // (unset) reads as no-overlap — exact for member-less
                // entries, impossible for published entries WITH members
                // (agg_expand runs before the ADD publishes).
                let agg = [
                    f64::from_bits(sh.a_bb[6 * ki].load(Ordering::Relaxed)),
                    f64::from_bits(sh.a_bb[6 * ki + 1].load(Ordering::Relaxed)),
                    f64::from_bits(sh.a_bb[6 * ki + 2].load(Ordering::Relaxed)),
                    f64::from_bits(sh.a_bb[6 * ki + 3].load(Ordering::Relaxed)),
                    f64::from_bits(sh.a_bb[6 * ki + 4].load(Ordering::Relaxed)),
                    f64::from_bits(sh.a_bb[6 * ki + 5].load(Ordering::Relaxed)),
                ];
                if agg[0] <= b[3]
                    && agg[3] >= b[0]
                    && agg[1] <= b[4]
                    && agg[4] >= b[1]
                    && agg[2] <= b[5]
                    && agg[5] >= b[2]
                {
                    // TASK-458-K (ID-H04): walk ONLY the vanilla scan's
                    // y-section range (verbatim bytecode replication) plus
                    // the overflow chain. Bits are monotone: (bits & mask)==0
                    // ⇒ no entity was EVER in those sections ⇒ vanilla's
                    // per-section lists there are empty ⇒ contribution 0.
                    let (sec_lo, sec_hi) = vanilla_section_range(b, min_sec, max_sec);
                    let wlo = sec_lo + SEC_OFF;
                    let whi = sec_hi + SEC_OFF;
                    // Intersection of the vanilla range with the 64-window.
                    let lo = wlo.max(0);
                    let hi = whi.min(SECTIONS as i32 - 1);
                    let (mask, any_window) = if lo <= hi {
                        (
                            (u64::MAX << lo) & (u64::MAX >> (63 - hi as usize)),
                            true,
                        )
                    } else {
                        (0u64, false)
                    };
                    let bits = sh.sec_bits[ki].load(Ordering::Relaxed);
                    let overflow_head = sh.head[ki].load(Ordering::Acquire);
                    if (bits & mask) != 0 || overflow_head != 0 {
                        // In-window sections first.
                        if any_window && (bits & mask) != 0 {
                            for w in (lo as usize)..=(hi as usize) {
                                if bits & (1u64 << w) == 0 {
                                    continue; // never occupied — skip 1 load
                                }
                                let mut cur =
                                    sh.s_head[ki * SECTIONS + w].load(Ordering::Acquire);
                                while cur != 0 {
                                    let s = (cur - 1) as usize;
                                    let bb = [
                                        f64::from_bits(sh.s_bb[6 * s].load(Ordering::Relaxed)),
                                        f64::from_bits(sh.s_bb[6 * s + 1].load(Ordering::Relaxed)),
                                        f64::from_bits(sh.s_bb[6 * s + 2].load(Ordering::Relaxed)),
                                        f64::from_bits(sh.s_bb[6 * s + 3].load(Ordering::Relaxed)),
                                        f64::from_bits(sh.s_bb[6 * s + 4].load(Ordering::Relaxed)),
                                        f64::from_bits(sh.s_bb[6 * s + 5].load(Ordering::Relaxed)),
                                    ];
                                    if bb_overlaps(b, &bb) {
                                        count += 1;
                                    }
                                    cur = sh.s_next[s].load(Ordering::Relaxed);
                                }
                            }
                        }
                        // Overflow chain: out-of-window sections (modded dims)
                        // — must always be consulted (superset).
                        let mut cur = overflow_head;
                        while cur != 0 {
                            let s = (cur - 1) as usize;
                            let bb = [
                                f64::from_bits(sh.s_bb[6 * s].load(Ordering::Relaxed)),
                                f64::from_bits(sh.s_bb[6 * s + 1].load(Ordering::Relaxed)),
                                f64::from_bits(sh.s_bb[6 * s + 2].load(Ordering::Relaxed)),
                                f64::from_bits(sh.s_bb[6 * s + 3].load(Ordering::Relaxed)),
                                f64::from_bits(sh.s_bb[6 * s + 4].load(Ordering::Relaxed)),
                                f64::from_bits(sh.s_bb[6 * s + 5].load(Ordering::Relaxed)),
                            ];
                            if bb_overlaps(b, &bb) {
                                count += 1;
                            }
                            cur = sh.s_next[s].load(Ordering::Relaxed);
                        }
                    }
                }
            }
        }
        if committed(sh, v1) {
            return Ok(count);
        }
        std::hint::spin_loop();
    }
}

// ---------------------------------------------------------------------------
// TASK-458-K selfTest (every SELFTEST_EVERY query calls ≈ 100 ticks):
//   (a) per chunk entry: chain-walked live nodes == chain_len;
//   (b) every node found on section w carries s_sec == w+1 (overflow: 0);
//   (c) chain_len > 0 ⇒ bloom probes true (FN-invariant).
// Violation → BROKEN → ERR_STRUCT forever (java broken=true, vanilla).
// Shards mid-write (odd version) are SKIPPED (best-effort fail-loud; the
// next cadence re-checks).
// ---------------------------------------------------------------------------
fn selftest() -> bool {
    for sh in SHARDS.iter() {
        let (ok, v1) = stable(sh);
        if !ok || !committed(sh, v1) {
            continue; // writer in flight — try next cadence
        }
        for ki in 0..CHUNK_CAP {
            let key = sh.keys[ki].load(Ordering::Acquire);
            if key == 0 {
                continue;
            }
            let want = sh.chain_len[ki].load(Ordering::Acquire);
            let mut walked: i32 = 0;
            // section chains
            for w in 0..SECTIONS {
                let mut cur = sh.s_head[ki * SECTIONS + w].load(Ordering::Acquire);
                while cur != 0 {
                    let s = (cur - 1) as usize;
                    if sh.s_sec[s].load(Ordering::Acquire) != w as i32 + 1 {
                        return false; // node on the wrong section chain
                    }
                    walked += 1;
                    cur = sh.s_next[s].load(Ordering::Acquire);
                }
            }
            // overflow chain
            let mut cur = sh.head[ki].load(Ordering::Acquire);
            while cur != 0 {
                let s = (cur - 1) as usize;
                if sh.s_sec[s].load(Ordering::Acquire) != 0 {
                    return false;
                }
                walked += 1;
                cur = sh.s_next[s].load(Ordering::Acquire);
            }
            if walked != want {
                return false; // bitmap-count vs chain-count invariant
            }
            if want > 0 && !bloom_probe(key) {
                return false; // bloom false-negative — forbidden
            }
        }
    }
    true
}

// ---------------------------------------------------------------------------
// JNI natives (registered on net/minecraft/world/entity/EntityIndexOps).
// ---------------------------------------------------------------------------

struct OpArrays {
    ids: *mut i32,
    ops: *mut i8,
    bb: *mut f64,
    cells: *mut i32,
    n: i32,
}

/// Copy-in the java batch arrays (plain Elements API — batch copies, no
/// critical regions per Shipilev/IBM JNI guidance).
unsafe fn load_ops(env: *mut jni::JNIEnv, n: i32, ids: jni::jintArray, ops: jni::jbyteArray,
                   bb: jni::jdoubleArray, cells: jni::jintArray) -> Result<OpArrays, i32> {
    if env.is_null() || n < 0 {
        return Err(ERR_STRUCT);
    }
    let vt = unsafe { &**env };
    if n == 0 {
        return Ok(OpArrays { ids: std::ptr::null_mut(), ops: std::ptr::null_mut(), bb: std::ptr::null_mut(), cells: std::ptr::null_mut(), n: 0 });
    }
    let pids = unsafe { (vt.GetIntArrayElements)(env, ids, std::ptr::null_mut()) };
    if pids.is_null() { return Err(ERR_STRUCT); }
    let pops = unsafe { (vt.GetByteArrayElements)(env, ops, std::ptr::null_mut()) };
    if pops.is_null() {
        unsafe { (vt.ReleaseIntArrayElements)(env, ids, pids, jni::JNI_ABORT) };
        return Err(ERR_STRUCT);
    }
    let pbb = unsafe { (vt.GetDoubleArrayElements)(env, bb, std::ptr::null_mut()) };
    if pbb.is_null() {
        unsafe {
            (vt.ReleaseByteArrayElements)(env, ops, pops, jni::JNI_ABORT);
            (vt.ReleaseIntArrayElements)(env, ids, pids, jni::JNI_ABORT);
        }
        return Err(ERR_STRUCT);
    }
    let pcells = unsafe { (vt.GetIntArrayElements)(env, cells, std::ptr::null_mut()) };
    if pcells.is_null() {
        unsafe {
            (vt.ReleaseDoubleArrayElements)(env, bb, pbb, jni::JNI_ABORT);
            (vt.ReleaseByteArrayElements)(env, ops, pops, jni::JNI_ABORT);
            (vt.ReleaseIntArrayElements)(env, ids, pids, jni::JNI_ABORT);
        }
        return Err(ERR_STRUCT);
    }
    Ok(OpArrays { ids: pids, ops: pops, bb: pbb, cells: pcells, n })
}

unsafe fn unload_ops(env: *mut jni::JNIEnv, ids: jni::jintArray, ops: jni::jbyteArray,
                     bb: jni::jdoubleArray, cells: jni::jintArray, a: &OpArrays) {
    if a.n == 0 {
        return; // nothing was pinned (load_ops skipped the getters)
    }
    let vt = unsafe { &**env };
    unsafe {
        (vt.ReleaseIntArrayElements)(env, ids, a.ids, jni::JNI_ABORT);
        (vt.ReleaseByteArrayElements)(env, ops, a.ops, jni::JNI_ABORT);
        (vt.ReleaseDoubleArrayElements)(env, bb, a.bb, jni::JNI_ABORT);
        (vt.ReleaseIntArrayElements)(env, cells, a.cells, jni::JNI_ABORT);
    }
}

/// Materialize ops into a reusable scratch vec (thread-local to the writer —
/// WLOCK serializes anyway).
fn ops_from(a: &OpArrays) -> Vec<Op> {
    let mut v = Vec::with_capacity(a.n as usize);
    unsafe {
        for i in 0..a.n as usize {
            let op = *a.ops.add(i) as u8;
            let o6 = 6 * i;
            let o3 = 3 * i;
            v.push(Op {
                op,
                id: *a.ids.add(i),
                cx: *a.cells.add(o3),
                cz: *a.cells.add(o3 + 1),
                sec: *a.cells.add(o3 + 2),
                bb: [
                    *a.bb.add(o6),
                    *a.bb.add(o6 + 1),
                    *a.bb.add(o6 + 2),
                    *a.bb.add(o6 + 3),
                    *a.bb.add(o6 + 4),
                    *a.bb.add(o6 + 5),
                ],
            });
        }
    }
    v
}

pub unsafe extern "system" fn eidx_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    PROBE_MAGIC
}

pub unsafe extern "system" fn eidx_flush(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    ids: jni::jintArray,
    ops: jni::jbyteArray,
    bb: jni::jdoubleArray,
    cells: jni::jintArray,
) -> jni::jint {
    let a = match unsafe { load_ops(env, n, ids, ops, bb, cells) } {
        Ok(a) => a,
        Err(e) => return e,
    };
    let batch = ops_from(&a);
    unsafe { unload_ops(env, ids, ops, bb, cells, &a) };
    apply_ops(&batch)
}

pub unsafe extern "system" fn eidx_flush_query(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    ids: jni::jintArray,
    ops: jni::jbyteArray,
    bb: jni::jdoubleArray,
    cells: jni::jintArray,
    min_x: jni::jdouble,
    min_y: jni::jdouble,
    min_z: jni::jdouble,
    max_x: jni::jdouble,
    max_y: jni::jdouble,
    max_z: jni::jdouble,
    min_cx: jni::jint,
    min_cz: jni::jint,
    max_cx: jni::jint,
    max_cz: jni::jint,
    min_sec: jni::jint,
    max_sec: jni::jint,
    out_counts: jni::jintArray,
) -> jni::jint {
    if BROKEN.load(Ordering::Acquire) {
        return ERR_STRUCT;
    }
    // TASK-458-K selfTest cadence: every 100th query call verifies the
    // section-plane invariants (bitmap-count vs chain-count + bloom FN).
    let q = QCOUNT.fetch_add(1, Ordering::Relaxed);
    if q % SELFTEST_EVERY == 0 && !selftest() {
        eprintln!("[crussty-plugin] eindex: SELFTEST FAIL (sections/bloom invariant) — fail-closed disarm");
        BROKEN.store(true, Ordering::Release);
        return ERR_STRUCT;
    }
    let a = match unsafe { load_ops(env, n, ids, ops, bb, cells) } {
        Ok(a) => a,
        Err(e) => return e,
    };
    let batch = ops_from(&a);
    unsafe { unload_ops(env, ids, ops, bb, cells, &a) };

    let w = (max_cx - min_cx + 1) as i64;
    let h = (max_cz - min_cz + 1) as i64;
    if w <= 0 || h <= 0 || w > 64 || h > 64 || w * h > 4096 {
        return ERR_RANGE;
    }

    let rc = apply_ops(&batch);
    if rc < 0 {
        return rc;
    }

    let vt = unsafe { &**env };
    let out_len = unsafe { (vt.GetArrayLength)(env, out_counts) } as i64;
    if out_len < w * h {
        return ERR_RANGE;
    }
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out_counts, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, (w * h) as usize) };
    let q = [min_x, min_y, min_z, max_x, max_y, max_z];
    let mut err = 0;
    'outer: for dz in 0..h {
        for dx in 0..w {
            match count_chunk(min_cx + dx as i32, min_cz + dz as i32, &q, min_sec, max_sec) {
                Ok(c) => dst[(dz * w + dx) as usize] = c,
                Err(e) => {
                    err = e;
                    break 'outer;
                }
            }
        }
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out_counts, pinned, 0) };
    err
}
