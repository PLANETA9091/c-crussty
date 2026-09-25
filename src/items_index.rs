//! Flat 1.0-grid spatial index for the item subsystem (TASK-397-J — vector
//! subsys_index; lever items_subsys2). Lives entirely on the rust side as
//! flat arrays: an open-addressed `cellKey -> chain-head` table plus intrusive
//! per-id `next`/`cell` vectors. No fastutil, no boxing, no per-item Java
//! objects — the Java bridge (ItemEntityManager) owns only a dense
//! `ItemEntity[] byId` indexed by the same int ids and mirrors lifecycle via
//! the natives below.
//!
//! REPLACED WORK: vanilla merge candidates come from
//! `Level.getEntitiesOfClass(ItemEntity.class, bb.inflate(itemMerge), pred)`
//! (EntitySectionStorage → whole 16³-section ClassInstanceMultiMap scan +
//! per-entity AABB) — the 15.66% item-driven broadphase lane. Here a merge
//! query scans only the ≤4×4×4 1.0-block cells overlapping the inflated box
//! (floor(q)-1..floor(q)+1 per axis; proof: a candidate with bb∩query≠∅ has
//! pos ≥ q0-0.125 ⇒ cell ≥ floor(q0)-1, see ROUND-396 RESEARCH-A §1.3) and
//! the exact AABB/predicate filters run Java-side on the returned ids.
//!
//! THREADING: writes (insert/setCell/remove) come from the main thread
//! between region phases (onTickingStart/onTickingEnd retargets + phase-4
//! drain) and from worker threads for `idxSetCell` on block-crossing ticks;
//! queries run on the region workers during phase 3. A single RwLock gives
//! writer-correctness; writes are rare (moves = block-crossings only, the
//! steady settled population never crosses) so read-side cost dominates and
//! stays a few ns per query.
//!
//! cmp399_shard (TASK-399-B — vector shardgrid): optional SHARDED mode, gated
//! by CRUSSTY_LEVER_FLAG=="cmp399_shard" or =="cmp399_bfcomp" (TASK-400-A
//! composite B+F) or =="cmp402_comp" (TASK-402-B — главный композит раунда:
//! B-shardgrid ⊕ mobpush ⊕ E-soa; any other flag, incl. items_subsys2,
//! keeps the legacy single-RwLock path verbatim — two-mode A/B by
//! construction). The cell-key universe is split over 64 shards (shard =
//! mix64(cell_key) & 63); each shard owns its own open-addressed table. Read
//! path is LOCK-FREE per cell: a seqlock version per shard (even = stable,
//! odd = write in flight) is sampled before/after each cell walk; a changed
//! version replays just that cell (candidate prefix rolled back via n0).
//! Writers serialize on one global mutex (writes are rare) and bracket every
//! mutation between two version bumps of the touched shard, so readers see
//! either the pre- or the post-state — the same linearized-write semantics
//! the legacy global RwLock gave, without touching the read-side word at
//! all (no RMW ping-pong, no writer-priority reader stalls, no global
//! rehash pause). Shard tables and the per-id arrays are fixed-capacity
//! statics (never reallocated, ids are reused Java-side via freeIds):
//! overflow fails closed (ERR_STRUCT → bridge disarms → vanilla), retry
//! exhaustion degrades to ERR_RANGE (per-call vanilla fallback, no disarm).
//! Candidate sets are identical to legacy (same cells → same chains → same
//! ids, same order); cross-cell skew under a concurrent move is bounded to
//! the vanilla EntitySectionStorage per-section-snapshot semantics and every
//! candidate is re-validated Java-side (isAlive/level/AABB/mergable).
//!
//! FAIL-CLOSED: every native returns <0 on any inconsistency (range weirdness,
//! capacity overflow, dangling chain, missing id). The Java bridge falls back
//! to the vanilla MethodHandle merge for that call and disarms the subsystem
//! on structural errors. An unarmed lever never touches this table.
//!
//! PARITY NOTE: the index universe = entities that entered the vanilla
//! EntityTickList (same class as round-1/round-396 item managers: loaded-but-
//! non-ticking items are not candidates; bench populations live in active
//! chunks). Level scoping is carried by the key (identityHashCode of the
//! Level) AND re-checked Java-side (`other.level() == self.level()`).

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, RwLock};

// ---------------------------------------------------------------------------
// Flat table
// ---------------------------------------------------------------------------

const INIT_CAP: usize = 4096;

struct Inner {
    /// Open-addressed keys; 0 = free slot (real keys are forced non-zero).
    keys: Vec<i64>,
    /// Per-slot chain head: id+1 (0 = empty chain).
    head: Vec<i32>,
    /// Occupied key slots (grow accounting).
    used: usize,
    /// Per-id intrusive chain link: next id+1 in the same cell (0 = end).
    next: Vec<i32>,
    /// Per-id current cell key (0 = not in any chain).
    cell: Vec<i64>,
}

static IDX: RwLock<Inner> = RwLock::new(Inner {
    keys: Vec::new(),
    head: Vec::new(),
    used: 0,
    next: Vec::new(),
    cell: Vec::new(),
});

#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Cell key: 21-bit symmetric fields for cx/cy/cz mixed with the Level
/// identity. Field overflow (|coord| > 1M blocks) wraps harmlessly — the key
/// stays a hash, candidates are still filtered exactly Java-side.
#[inline]
fn cell_key(lid: i32, cx: i32, cy: i32, cz: i32) -> i64 {
    const OFF: i64 = 1 << 20;
    const MSK: i64 = (1 << 21) - 1;
    let a = ((cx as i64) + OFF) & MSK;
    let b = ((cy as i64) + OFF) & MSK;
    let c = ((cz as i64) + OFF) & MSK;
    let packed = a | (b << 21) | (c << 42);
    let h = mix64((packed as u64) ^ ((lid as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)));
    let h = if h == 0 { 0x9E37_79B9_7F4A_7C15 } else { h };
    h as i64
}

#[inline]
fn find_slot(keys: &[i64], k: i64) -> Result<usize, usize> {
    // TASK-398 lifetime-guard: the static IDX starts with EMPTY vecs (the old
    // code never applied INIT_CAP). find_slot on an empty table used to compute
    // `keys.len() - 1` → usize underflow → get_unchecked(wild) → SIGSEGV in
    // link (crash round-397-J). Guard: empty table = "not found", callers
    // treat Err as the insert slot only after grow_grid guarantees capacity.
    if keys.is_empty() {
        return Err(0);
    }
    let mask = keys.len() - 1;
    let mut s = (mix64(k as u64) as usize) & mask;
    loop {
        let cur = unsafe { *keys.get_unchecked(s) };
        if cur == k {
            return Ok(s);
        }
        if cur == 0 {
            return Err(s);
        }
        s = (s + 1) & mask;
    }
}

fn ensure_id_space(inner: &mut Inner, id: usize) {
    if inner.next.len() <= id {
        let cap = (id + 1).next_power_of_two().max(1024);
        inner.next.resize(cap, 0);
        inner.cell.resize(cap, 0);
    }
}

fn grow_grid(inner: &mut Inner) {
    let cap = inner.keys.len();
    let used = inner.used;
    if !inner.keys.is_empty() && used * 8 < cap * 5 {
        return; // <62.5% load
    }
    // TASK-398 root-cause fix: cap==0 on the very first link used to produce
    // ncap == 0 (0*2) — the table stayed empty and every find_slot hit the
    // underflow above (SIGSEGV). Floor the first allocation at INIT_CAP.
    let ncap = (cap * 2).max(INIT_CAP);
    let mut keys = vec![0i64; ncap];
    let mut head = vec![0i32; ncap];
    for s in 0..cap {
        let k = inner.keys[s];
        if k == 0 {
            continue;
        }
        let mask = ncap - 1;
        let mut t = (mix64(k as u64) as usize) & mask;
        while keys[t] != 0 {
            t = (t + 1) & mask;
        }
        keys[t] = k;
        head[t] = inner.head[s];
    }
    inner.keys = keys;
    inner.head = head;
}

/// Link `id` as the head of cell-key `k` (caller guarantees id is unlinked).
fn link(inner: &mut Inner, id: usize, k: i64) {
    grow_grid(inner);
    debug_assert!(!inner.keys.is_empty(), "items_index grid must be grown before link");
    let slot = match find_slot(&inner.keys, k) {
        Ok(s) => s,
        Err(s) => {
            inner.keys[s] = k;
            inner.used += 1;
            s
        }
    };
    inner.next[id] = inner.head[slot];
    inner.head[slot] = (id as i32) + 1;
    inner.cell[id] = k;
}

/// Unlink `id` from its stored cell chain. Ok(()) or Err(()) if dangling.
fn unlink(inner: &mut Inner, id: usize) -> Result<(), ()> {
    let k = inner.cell[id];
    if k == 0 {
        return Err(());
    }
    let slot = find_slot(&inner.keys, k).map_err(|_| ())?;
    let mut prev: i32 = 0;
    let mut cur = inner.head[slot];
    while cur != 0 {
        let cid = (cur - 1) as usize;
        if cid == id {
            if prev == 0 {
                inner.head[slot] = inner.next[cid];
            } else {
                inner.next[(prev - 1) as usize] = inner.next[cid];
            }
            inner.next[cid] = 0;
            inner.cell[cid] = 0;
            if inner.head[slot] == 0 {
                inner.keys[slot] = 0;
                inner.used -= 1;
            }
            return Ok(());
        }
        prev = cur;
        cur = inner.next[cid];
    }
    Err(())
}

// ---------------------------------------------------------------------------
// cmp399_shard: sharded seqlock grid (TASK-399-B, vector shardgrid)
// ---------------------------------------------------------------------------

pub const NSHARDS: usize = 64;
const SHARD_CAP: usize = 1 << 14; // 16384 slots/shard → 1M keys total (150k-cell bench ≈ 2.4k/shard)
const MAX_IDS: usize = 1 << 20;   // per-id arrays; Java reuses ids via freeIds (peak ≈ 150k)
const QRETRY: u32 = 256;          // per-cell seqlock retry budget → ERR_RANGE (per-call fallback)

struct Shard {
    /// Seqlock version: even = stable, odd = write in flight (monotonic).
    ver: AtomicUsize,
    /// Open-addressed keys; 0 = free slot (real keys are forced non-zero).
    keys: [AtomicI64; SHARD_CAP],
    /// Per-slot chain head: id+1 (0 = empty chain).
    head: [AtomicI32; SHARD_CAP],
    /// Occupied key slots (load accounting — writers only, under WLOCK).
    used: AtomicUsize,
}

const K0: AtomicI64 = AtomicI64::new(0);
const H0: AtomicI32 = AtomicI32::new(0);

// Fixed-capacity statics: zero-filled (0 = empty key / empty chain / version 0
// = stable) and NEVER reallocated — the prerequisite for lock-free reads.
static SHARDS: [Shard; NSHARDS] = [const {
    Shard {
        ver: AtomicUsize::new(0),
        keys: [K0; SHARD_CAP],
        head: [H0; SHARD_CAP],
        used: AtomicUsize::new(0),
    }
}; NSHARDS];

/// Per-id intrusive chain link (next id+1 in the same cell, 0 = end).
static NEXT: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];
/// Per-id current cell key (0 = not in any chain).
static CELL: [AtomicI64; MAX_IDS] = [K0; MAX_IDS];

/// Global writer serialization. Writes are rare (block-crossings + lifecycle)
/// and were globally serialized in the legacy path too; one mutex keeps the
/// linearized-write semantics identical to the legacy global RwLock while the
/// read side stays lock-free. No lock nesting → deadlock-free by construction.
static WLOCK: Mutex<()> = Mutex::new(());

/// Mode selector, sampled once (process-wide env; the Java bridge and the
/// natives see the same CRUSSTY_LEVER_FLAG for the whole JVM lifetime).
#[inline]
fn shard_mode() -> bool {
    static M: OnceLock<bool> = OnceLock::new();
    *M.get_or_init(|| {
        std::env::var("CRUSSTY_LEVER_FLAG")
            .map(|v| {
                let v = v.trim();
                // TASK-400-A: составной флаг cmp399_bfcomp (B+F) включает
                // sharded mode наряду с точным cmp399_shard.
                // TASK-402-B: главный композит cmp402_comp включает shardgrid
                // как суб-механизм (soa+shardgrid+mobpush одновременно).
                v == "cmp399_shard"
                    || v == "cmp399_bfcomp"
                    || v == "cmp402_comp"
                    || v == "cmp402_stagcomp"
                    || v == "cmp403_tickplane"
                    || v == "cmp405_stagtick"
                    // TASK-406-D: композит раунда-406 включает shardgrid.
                    || v == "cmp406_aibatch"
                    // TASK-406-E: композит раунда-406 включает shardgrid.
                    || v == "cmp406_sscan"
                    // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                    || v == "cmp409_multi" || v == "cmp412_meganav" || v == "cmp414_cvs"
                    // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
                    || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq"
                    // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
                    || v == "cmp420_colpush"
                    || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2" || v == "cmp436_ins4"
                    || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp456_poi" || v == "cmp456_poi_wide" || v == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
                    || v == "cmp438_sense" // TASK-444-C: sense family union
                    || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp456_poi" || v == "cmp456_poi_wide" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
            })
            .unwrap_or(false)
    })
}

#[inline]
fn shard_of(k: i64) -> usize {
    (mix64(k as u64) as usize) & (NSHARDS - 1)
}

/// Seqlock write bracket: bump to ODD before mutating the shard, to the next
/// EVEN after. Release ordering on the even bump publishes every Relaxed data
/// store made before it; the reader pairs Acquire loads on both ends, so a
/// walk validated under an unchanged even version observes a consistent state.
#[inline]
fn w_begin(s: &Shard) {
    s.ver.fetch_add(1, Ordering::Release);
}
#[inline]
fn w_end(s: &Shard) {
    s.ver.fetch_add(1, Ordering::Release);
}

/// Open-addressed lookup in one shard's table (load < 62.5% ⇒ a zero slot
/// always exists ⇒ the probe always terminates). Relaxed loads: validated by
/// the caller's seqlock version pair.
fn shard_find_slot(s: &Shard, k: i64) -> Result<usize, usize> {
    let mask = SHARD_CAP - 1;
    let mut i = (mix64(k as u64) as usize) & mask;
    loop {
        // SAFETY: i < SHARD_CAP by mask; keys has exactly SHARD_CAP slots.
        let cur = unsafe { s.keys.get_unchecked(i) }.load(Ordering::Relaxed);
        if cur == k {
            return Ok(i);
        }
        if cur == 0 {
            return Err(i);
        }
        i = (i + 1) & mask;
    }
}

/// Link `id` as the head of cell-key `k` in its shard. Caller holds WLOCK and
/// brackets this call with w_begin/w_end on the OWNING shard.
fn shard_link(id: usize, k: i64) -> Result<(), ()> {
    let s = &SHARDS[shard_of(k)];
    // Fixed-capacity load gate (legacy grew the grid; fail-closed instead —
    // unreachable at bench scale: 150k distinct cells ≈ 2.4k/shard avg).
    if s.used.load(Ordering::Relaxed) * 8 >= SHARD_CAP * 5 {
        return Err(());
    }
    let slot = match shard_find_slot(s, k) {
        Ok(x) => x,
        Err(x) => {
            // SAFETY: x < SHARD_CAP (see shard_find_slot).
            unsafe { s.keys.get_unchecked(x) }.store(k, Ordering::Relaxed);
            s.used.fetch_add(1, Ordering::Relaxed);
            x
        }
    };
    let prev_head = unsafe { s.head.get_unchecked(slot) }.load(Ordering::Relaxed);
    // SAFETY: id < MAX_IDS (checked by every native entry point).
    unsafe { NEXT.get_unchecked(id) }.store(prev_head, Ordering::Relaxed);
    unsafe { s.head.get_unchecked(slot) }.store((id as i32) + 1, Ordering::Relaxed);
    unsafe { CELL.get_unchecked(id) }.store(k, Ordering::Relaxed);
    Ok(())
}

/// Unlink `id` from its stored cell chain in its shard. Ok(()) or Err(()) if
/// dangling. Caller holds WLOCK and brackets with the owning shard's version.
fn shard_unlink(id: usize) -> Result<(), ()> {
    // SAFETY: id < MAX_IDS (checked by every native entry point).
    let k = unsafe { CELL.get_unchecked(id) }.load(Ordering::Relaxed);
    if k == 0 {
        return Err(());
    }
    let s = &SHARDS[shard_of(k)];
    let slot = shard_find_slot(s, k).map_err(|_| ())?;
    let mut prev: i32 = 0;
    let mut cur = unsafe { s.head.get_unchecked(slot) }.load(Ordering::Relaxed);
    while cur != 0 {
        let cid = (cur - 1) as usize;
        // SAFETY: chain ids are always < MAX_IDS (invariant of shard_link;
        // the read path double-checks and treats a violation as corruption).
        if cid == id {
            let nxt = unsafe { NEXT.get_unchecked(cid) }.load(Ordering::Relaxed);
            if prev == 0 {
                unsafe { s.head.get_unchecked(slot) }.store(nxt, Ordering::Relaxed);
            } else {
                unsafe { NEXT.get_unchecked((prev - 1) as usize) }.store(nxt, Ordering::Relaxed);
            }
            unsafe { NEXT.get_unchecked(cid) }.store(0, Ordering::Relaxed);
            unsafe { CELL.get_unchecked(cid) }.store(0, Ordering::Relaxed);
            if unsafe { s.head.get_unchecked(slot) }.load(Ordering::Relaxed) == 0 {
                unsafe { s.keys.get_unchecked(slot) }.store(0, Ordering::Relaxed);
                s.used.fetch_sub(1, Ordering::Relaxed);
            }
            return Ok(());
        }
        prev = cur;
        cur = unsafe { NEXT.get_unchecked(cid) }.load(Ordering::Relaxed);
    }
    Err(())
}

fn shard_insert(id: usize, k: i64) -> i32 {
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let _w = match WLOCK.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let old = CELL[id].load(Ordering::Relaxed);
    if old != 0 {
        let sold = &SHARDS[shard_of(old)];
        w_begin(sold);
        let r = shard_unlink(id);
        w_end(sold);
        if r.is_err() {
            return ERR_STRUCT;
        }
    }
    let s = &SHARDS[shard_of(k)];
    w_begin(s);
    let r = shard_link(id, k);
    w_end(s);
    if r.is_err() {
        return ERR_STRUCT;
    }
    0
}

fn shard_set_cell(id: usize, k: i64) -> i32 {
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let _w = match WLOCK.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let old = CELL[id].load(Ordering::Relaxed);
    if old == k {
        return 0;
    }
    if old == 0 {
        return ERR_STRUCT; // never inserted (legacy parity: unlink(cell=0) → Err)
    }
    let sold = &SHARDS[shard_of(old)];
    w_begin(sold);
    let r = shard_unlink(id);
    w_end(sold);
    if r.is_err() {
        return ERR_STRUCT;
    }
    let s = &SHARDS[shard_of(k)];
    w_begin(s);
    let r = shard_link(id, k);
    w_end(s);
    if r.is_err() {
        return ERR_STRUCT;
    }
    0
}

fn shard_remove(id: usize) -> i32 {
    if id >= MAX_IDS {
        return 0; // never inserted — nothing to do (legacy parity)
    }
    let _w = match WLOCK.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let old = CELL[id].load(Ordering::Relaxed);
    if old == 0 {
        return 0;
    }
    let sold = &SHARDS[shard_of(old)];
    w_begin(sold);
    let r = shard_unlink(id);
    w_end(sold);
    if r.is_err() {
        ERR_STRUCT
    } else {
        0
    }
}

/// Lock-free sharded query: walk every cell in the window; each cell is read
/// under its shard's seqlock (version sampled before/after, cell-local retry).
/// Returns the count, -(cap) on overflow (caller grows + retries), ERR_RANGE
/// on retry exhaustion (per-call vanilla fallback, no disarm) or ERR_STRUCT on
/// a stable anomaly (= real corruption → bridge disarms, legacy parity).
#[allow(clippy::too_many_arguments)]
fn shard_query(
    dst: &mut [jni::jint],
    cap: i32,
    lid: i32,
    (cx0, cx1): (i32, i32),
    (cy0, cy1): (i32, i32),
    (cz0, cz1): (i32, i32),
) -> i32 {
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                let s = &SHARDS[shard_of(k)];
                let n0 = n;
                let mut tries: u32 = 0;
                loop {
                    tries += 1;
                    if tries > QRETRY {
                        return ERR_RANGE;
                    }
                    let v1 = s.ver.load(Ordering::Acquire);
                    if v1 & 1 == 1 {
                        std::hint::spin_loop();
                        continue;
                    }
                    let mut anom = false;
                    if let Ok(slot) = shard_find_slot(s, k) {
                        let mut cur = unsafe { s.head.get_unchecked(slot) }.load(Ordering::Relaxed);
                        while cur != 0 {
                            if n >= cap {
                                return -cap;
                            }
                            let id = (cur - 1) as usize;
                            if id >= MAX_IDS {
                                // A chain id outside the id space: either a
                                // transient torn view (a write is in flight on
                                // this shard) or real corruption. Decide on the
                                // version check below.
                                anom = true;
                                break;
                            }
                            // SAFETY: id < MAX_IDS (checked above).
                            if unsafe { CELL.get_unchecked(id) }.load(Ordering::Relaxed) == k {
                                dst[n as usize] = cur - 1;
                                n += 1;
                            }
                            cur = unsafe { NEXT.get_unchecked(id) }.load(Ordering::Relaxed);
                        }
                    }
                    let v2 = s.ver.load(Ordering::Acquire);
                    if v2 != v1 {
                        n = n0; // replay this cell only; roll back its candidates
                        continue;
                    }
                    if anom {
                        // Version stable AND a corrupt-looking chain → the
                        // legacy fail-closed verdict (bridge disarms).
                        return ERR_STRUCT;
                    }
                    break;
                }
            }
        }
    }
    n
}

// ---------------------------------------------------------------------------
// Native exports (registered on net/minecraft/world/entity/ItemEntityManager)
// ---------------------------------------------------------------------------

/// Result codes: >=0 ok / query count; -1 structural (disarm); -2 bad range
/// (per-call vanilla fallback); query overflow returns -(cap) so the caller
/// can grow and retry.
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

#[inline]
fn idx() -> std::sync::RwLockReadGuard<'static, Inner> {
    // Poisoning is irrelevant: a poisoned lock means a panic inside a native
    // while holding it — recovery would leave the table inconsistent anyway;
    // the Java side disarms on the resulting error return.
    match IDX.read() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

#[inline]
fn idx_mut() -> std::sync::RwLockWriteGuard<'static, Inner> {
    match IDX.write() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn idx_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    0x1D3A
}

/// # Safety
/// See idx_probe.
#[no_mangle]
pub unsafe extern "system" fn idx_insert(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    cx: jni::jint,
    cy: jni::jint,
    cz: jni::jint,
) -> jni::jint {
    if id < 0 {
        return ERR_STRUCT;
    }
    if shard_mode() {
        return shard_insert(id as usize, cell_key(lid, cx, cy, cz));
    }
    let k = cell_key(lid, cx, cy, cz);
    let mut g = idx_mut();
    ensure_id_space(&mut g, id as usize);
    if g.cell[id as usize] != 0 {
        if unlink(&mut g, id as usize).is_err() {
            return ERR_STRUCT;
        }
    }
    link(&mut g, id as usize, k);
    0
}

/// # Safety
/// See idx_probe.
#[no_mangle]
pub unsafe extern "system" fn idx_set_cell(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    cx: jni::jint,
    cy: jni::jint,
    cz: jni::jint,
) -> jni::jint {
    if id < 0 {
        return ERR_STRUCT;
    }
    if shard_mode() {
        return shard_set_cell(id as usize, cell_key(lid, cx, cy, cz));
    }
    let k = cell_key(lid, cx, cy, cz);
    let mut g = idx_mut();
    if (g.next.len() as i64) <= id as i64 {
        return ERR_STRUCT; // never inserted
    }
    if g.cell[id as usize] == k {
        return 0;
    }
    if unlink(&mut g, id as usize).is_err() {
        return ERR_STRUCT;
    }
    link(&mut g, id as usize, k);
    0
}

/// # Safety
/// See idx_probe.
#[no_mangle]
pub unsafe extern "system" fn idx_remove(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
) -> jni::jint {
    if id < 0 {
        return ERR_STRUCT;
    }
    if shard_mode() {
        return shard_remove(id as usize);
    }
    let mut g = idx_mut();
    if (g.next.len() as i64) <= id as i64 {
        return 0; // never inserted — nothing to do
    }
    if g.cell[id as usize] == 0 {
        return 0;
    }
    match unlink(&mut g, id as usize) {
        Ok(()) => 0,
        Err(()) => ERR_STRUCT,
    }
}

/// Query candidates overlapping [qx0..qx1]×[qy0..qy1]×[qz0..qz1] (an
/// already-inflated vanilla merge box). Writes ids into the pinned `out`
/// array; returns the count, -(out_cap) on overflow (caller grows + retries),
/// ERR_RANGE for absurd scan widths, ERR_STRUCT on JNI trouble.
///
/// # Safety
/// See idx_probe.
#[no_mangle]
pub unsafe extern "system" fn idx_query(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    qx0: jni::jdouble,
    qy0: jni::jdouble,
    qz0: jni::jdouble,
    qx1: jni::jdouble,
    qy1: jni::jdouble,
    qz1: jni::jdouble,
    lid: jni::jint,
    out: jni::jintArray,
) -> jni::jint {
    if env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };

    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    let cap = cap as i32;

    // Cell window with the ±1 pad; guard absurd itemMerge configs.
    let cx0 = qx0.floor() as i32 - 1;
    let cx1 = qx1.floor() as i32 + 1;
    let cy0 = qy0.floor() as i32 - 1;
    let cy1 = qy1.floor() as i32 + 1;
    let cz0 = qz0.floor() as i32 - 1;
    let cz1 = qz1.floor() as i32 + 1;
    if (cx1 - cx0) > 6 || (cy1 - cy0) > 6 || (cz1 - cz0) > 6 {
        return ERR_RANGE;
    }

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, cap as usize) };

    let n: i32;
    if shard_mode() {
        // cmp399_shard: lock-free sharded walk (per-cell seqlock validation).
        n = shard_query(dst, cap, lid, (cx0, cx1), (cy0, cy1), (cz0, cz1));
    } else {
        // Legacy single-RwLock path (verbatim body, extracted as-is).
        let g = idx();
        n = legacy_query(&g, dst, cap, lid, (cx0, cx1), (cy0, cy1), (cz0, cz1));
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
}

/// Legacy global-RwLock query walk, extracted verbatim from the original
/// idx_query body (TASK-399-B: the non-sharded arm of the A/B two-mode).
fn legacy_query(
    g: &Inner,
    dst: &mut [jni::jint],
    cap: i32,
    lid: i32,
    (cx0, cx1): (i32, i32),
    (cy0, cy1): (i32, i32),
    (cz0, cz1): (i32, i32),
) -> i32 {
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                if let Ok(slot) = find_slot(&g.keys, k) {
                    let mut cur = g.head[slot];
                    while cur != 0 {
                        if n >= cap {
                            return -cap;
                        }
                        let id = (cur - 1) as usize;
                        if id >= g.next.len() {
                            // TASK-398 fail-closed: a chain id outside the id
                            // space means corruption (stale link) — never walk
                            // it. Hand the caller a structural error so the
                            // Java bridge disarms.
                            return ERR_STRUCT;
                        }
                        // SAFETY: id < next.len() (checked above); cell[id] == k
                        // because the chain for `k` only contains ids with cell == k.
                        if unsafe { *g.cell.get_unchecked(id) } == k {
                            dst[n as usize] = cur - 1;
                            n += 1;
                        }
                        cur = unsafe { *g.next.get_unchecked(id) };
                    }
                }
            }
        }
    }
    n
}
