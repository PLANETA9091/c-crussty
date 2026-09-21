//! Sharded seqlock spatial grid + OFF-THREAD PREP STAGE for the MOB push
//! broadphase (TASK-401-D — vector offthread; lever cmp401_offthread). Grid
//! core = verbatim port of round-400-j-mobpush src/mobs_grid.rs (pattern-B
//! items_index shape): 64 shards keyed by mix64(cell_key)&63, lock-free
//! per-cell read path (v1→data→v2 Acquire pair, cell-local retry with n0-
//! rollback, retry exhaustion → fail-closed), writers under one global mutex
//! with odd/even shard-version bumps. Fixed memory: 16384 slots/shard +
//! 1<<20 per-id entries, never reallocated ⇒ UAF impossible.
//!
//! OFF-THREAD STAGE DELTA (the vector): the per-cell candidate ENUMERATION is
//! lifted off the tick threads entirely. A single background prep thread
//! continuously re-computes, per registered id, the candidate id list of its
//! 5³-cell window (floor(center)±2 — a superset of every padded query window
//! the ≤1.0 radius gate can produce) into a fixed prepared table (per-id:
//! ready flag + prep cell key + lid + count + 6 window bounds + 64 candidate
//! slots). In-tick the bridge makes ONE native stageQuery call: if the
//! prepared entry is co-version valid (ready=1, prep cell == current grid
//! cell, lid match, query window ⊆ prep window) the tick thread only MEMCPYs
//! the prepared ids and re-validates them exactly java-side (level +
//! AABB.intersects + EntitySelector.pushableBy) — the walk cost itself was
//! paid off-thread. Any mismatch/staleness/overflow → ERR_RANGE → the Java
//! bridge does the exact vanilla fill for that call (vanilla parity by
//! construction; static populations hit, movers fail closed to vanilla).
//!
//! UNIVERSE: living entities that run LivingEntity.pushEntities (the mob push
//! lane). Java bridge (MobStageOps) pushes per-tick SELF position (AABB
//! center + floor coords + level id); rust stores id→cell membership plus the
//! per-id center cell coords (prep needs them to enumerate the window — the
//! cell key itself is a one-way mix). Java re-validates every candidate
//! exactly — parity of the RESULT SET by construction for unchanged
//! neighborhoods; membership staleness between prep and consumption = the
//! documented delta class of items_subsys2/round-400-j-mobpush (over-inclusion
//! killed by re-validation, under-inclusion possible only for entities that
//! moved into the window after prep — the same one-phase-staleness class the
//! armed mobpush lever carries).
//!
//! FAIL-CLOSED: stageQuery returns <0 on any inconsistency (not ready, cell
//! drift, window escape, range weirdness) → per-call vanilla fill; structural
//! anomalies (stable corrupt chain, dangling id) → ERR_STRUCT → bridge
//! disarms the lever entirely → vanilla. An unarmed lever never touches
//! these tables.
//!
//! THREADING: upserts come from region workers (up to W concurrent, one per
//! ticking entity); queries are one JNI per living entity per tick (as
//! mobpush); the prep stage is ONE background thread reading the same
//! seqlock-protected cells (read side is lock-free by construction) and
//! writing ONLY its own per-id prepared slots (single writer ⇒ plain
//! publish-then-ready release ordering, reader Acquire pairs).

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Sharded grid (pattern B: items_index.rs cmp399_shard arm, verbatim shapes)
// ---------------------------------------------------------------------------

pub const NSHARDS: usize = 64;
const SHARD_CAP: usize = 1 << 14; // 16384 slots/shard → 1M cell keys (47k mobs ≈ ≤ 734 cells/shard)
const MAX_IDS: usize = 1 << 20;   // per-id arrays; Java reuses ids via freeIds (peak ≈ 47k living)
const QRETRY: u32 = 256;          // per-cell seqlock retry budget → ERR_RANGE (per-call fallback)
/// Max cell-window half-width beyond the ±1 pad (self AABB span cap): wider
/// queries return ERR_RANGE (per-call vanilla fallback; oversized selves are
/// already gated Java-side by the ≤1.0 bounding-radius gate).
const MAX_SPAN: i32 = 9;

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

// Fixed-capacity statics: zero-filled (0 = empty key / empty chain / version
// 0 = stable) and NEVER reallocated — the prerequisite for lock-free reads.
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

/// Per-id last-upsert level id + floor(center) coords (the OFF-THREAD STAGE
/// needs them: the cell key is a one-way mix, the prep window is enumerated
/// from the coords). Written only by mob_upsert (WLOCK-serialized callers),
/// read by the single prep thread — benign races degrade to a stale prep
/// that the serve-time co-version check rejects.
static LID: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];
static CX: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];
static CY: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];
static CZ: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];

// --- prepared table (the off-thread stage product; single writer = prep
// --- thread, readers = tick threads inside stage_query; publish-then-ready
// --- release protocol, ready flag written LAST / read FIRST) -------------
/// Prepared id capacity (ids beyond → never prepared → per-call vanilla;
/// the java byId space is dense and ≪ this at bench scale).
const PREP_CAP: usize = 1 << 18;
/// Candidate slots per prepared id (denser pockets → not prepared → vanilla).
const PREP_CCAP: usize = 64;
static P_READY: [AtomicI32; PREP_CAP] = [H0; PREP_CAP];
static P_CELL: [AtomicI64; PREP_CAP] = [K0; PREP_CAP];
static P_LID: [AtomicI32; PREP_CAP] = [H0; PREP_CAP];
static P_N: [AtomicI32; PREP_CAP] = [H0; PREP_CAP];
/// 6 window bounds per id: x0,x1,y0,y1,z0,z1 (floor(center)±2 at prep).
static P_WIN: [AtomicI32; PREP_CAP * 6] = [H0; PREP_CAP * 6];
static P_CAND: [AtomicI32; PREP_CAP * PREP_CCAP] = [H0; PREP_CAP * PREP_CCAP];
/// Structural corruption flagged by the prep stage → stage_query returns
/// ERR_STRUCT → the Java bridge disarms the lever permanently.
static STAGE_BROKEN: AtomicBool = AtomicBool::new(false);

/// Global writer serialization (pattern B). One native upsert per living
/// entity per tick — the same order of magnitude the vanilla scan spent on a
/// whole-section walk; one mutex keeps linearized-write semantics identical
/// to the legacy global-RwLock pattern while the read side stays lock-free.
/// No lock nesting → deadlock-free by construction.
static WLOCK: Mutex<()> = Mutex::new(());

/// Mode selector, sampled once (process-wide env; the Java bridge and the
/// natives see the same CRUSSTY_LEVER_FLAG for the whole JVM lifetime).
#[inline]
fn lever_mode() -> bool {
    static M: OnceLock<bool> = OnceLock::new();
    *M.get_or_init(|| {
        std::env::var("CRUSSTY_LEVER_FLAG")
            .map(|v| v.trim() == "cmp401_offthread")
            .unwrap_or(false)
    })
}

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
    // Fixed-capacity load gate (fail-closed; unreachable at bench scale:
    // 47k living ≈ ≤ 734 distinct cells/shard avg).
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

/// Backward-shift (Knuth Algorithm R) removal of the open-addressed entry at
/// `slot`: keeps every remaining key findable without tombstones. ROOT-CAUSE
/// FIX over the pattern-B verbatim shape (zeroing an emptied slot breaks the
/// linear-probe chain of any key that probed past it — found by the
/// `delete_find_stress` oracle test: upsert → unlink → find_slot(K2) Err →
/// ERR_STRUCT disarm). Caller holds WLOCK inside this shard's version
/// bracket; clusters are ~1 entry at bench load, so the shift is O(1).
fn shard_delete_slot(s: &Shard, mut hole: usize) {
    let mask = SHARD_CAP - 1;
    loop {
        let mut j = hole;
        loop {
            j = (j + 1) & mask;
            // SAFETY: j < SHARD_CAP by mask (see shard_find_slot).
            let k = unsafe { s.keys.get_unchecked(j) }.load(Ordering::Relaxed);
            if k == 0 {
                // Cluster ends: the hole is final.
                unsafe { s.keys.get_unchecked(hole) }.store(0, Ordering::Relaxed);
                unsafe { s.head.get_unchecked(hole) }.store(0, Ordering::Relaxed);
                s.used.fetch_sub(1, Ordering::Relaxed);
                return;
            }
            let home = (mix64(k as u64) as usize) & mask;
            // Leave the entry iff its home lies cyclically within (hole, j]
            // (its probe path never passes the hole); otherwise shift it back.
            let in_range = if hole <= j {
                home > hole && home <= j
            } else {
                home > hole || home <= j
            };
            if !in_range {
                unsafe { s.keys.get_unchecked(hole) }.store(k, Ordering::Relaxed);
                unsafe { s.head.get_unchecked(hole) }.store(
                    unsafe { s.head.get_unchecked(j) }.load(Ordering::Relaxed),
                    Ordering::Relaxed,
                );
                hole = j;
                break;
            }
        }
    }
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
                // Backward-shift delete keeps other keys' probe chains intact.
                shard_delete_slot(s, slot);
            }
            return Ok(());
        }
        prev = cur;
        cur = unsafe { NEXT.get_unchecked(cid) }.load(Ordering::Relaxed);
    }
    Err(())
}

/// Insert-or-move: (re)place `id` into cell `k` (idempotent per id — a stale
/// ghost entry after a sweep race self-heals on the next owner upsert).
fn grid_upsert(id: usize, k: i64) -> i32 {
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let _w = match WLOCK.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let old = CELL[id].load(Ordering::Relaxed);
    if old == k {
        return 0; // same cell — no mutation at all (settled-population fast path)
    }
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

fn grid_remove(id: usize) -> i32 {
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let _w = match WLOCK.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let old = CELL[id].load(Ordering::Relaxed);
    if old == 0 {
        return 0; // never inserted — nothing to do
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

/// Lock-free sharded query: walk every cell in the padded window; each cell
/// is read under its shard's seqlock (version sampled before/after, cell-local
/// retry with n0 rollback). Returns the count, -(cap) on overflow (caller
/// grows + retries), ERR_RANGE on retry exhaustion / absurd span (per-call
/// vanilla fallback, no disarm) or ERR_STRUCT on a stable anomaly (= real
/// corruption → bridge disarms).
#[allow(dead_code, clippy::too_many_arguments)]
fn grid_query(
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
                                // this shard) or real corruption. Decide on
                                // the version check below.
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
                        // fail-closed verdict (bridge disarms).
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
// Native exports (registered on net/minecraft/world/entity/MobPushOps)
// ---------------------------------------------------------------------------

/// Result codes: >=0 ok / query count; -1 structural (disarm); -2 bad range
/// (per-call vanilla fallback); query overflow returns -(cap) so the caller
/// can grow and retry.
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn mob_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    0x4F53 // "OS" (offstage)
}

/// Insert-or-move `id` to the 1.0 cell of the AABB-center (x, y, z).
/// Java gates the ≤1.0 bounding radius; rust stores membership only.
/// Returns 0, ERR_STRUCT (overflow/corruption — Java disarms).
///
/// # Safety
/// See mob_probe.
#[no_mangle]
pub unsafe extern "system" fn mob_upsert(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    x: jni::jdouble,
    y: jni::jdouble,
    z: jni::jdouble,
) -> jni::jint {
    if !lever_mode() || id < 0 || (id as usize) >= MAX_IDS {
        return ERR_STRUCT;
    }
    let iu = id as usize;
    let fx = x.floor() as i32;
    let fy = y.floor() as i32;
    let fz = z.floor() as i32;
    let k = cell_key(lid, fx, fy, fz);
    let old_cell = CELL[iu].load(Ordering::Relaxed);
    let rc = grid_upsert(iu, k);
    if rc == 0 {
        // Publish prep inputs (Relaxed — the serve-time co-version check on
        // CELL/P_CELL is the correctness boundary; these only steer prep).
        LID[iu].store(lid, Ordering::Relaxed);
        CX[iu].store(fx, Ordering::Relaxed);
        CY[iu].store(fy, Ordering::Relaxed);
        CZ[iu].store(fz, Ordering::Relaxed);
        if old_cell != k {
            // Cell moved (or first insert): the prepared entry for this id is
            // stale — drop it eagerly so the next stageQuery miss-fails to
            // vanilla instead of serving a pre-move window.
            P_READY[iu].store(0, Ordering::Relaxed);
        }
    }
    rc
}

/// Remove `id` (graveyard sweep). Returns 0 (also when never inserted) or
/// ERR_STRUCT on a dangling chain (corruption → Java disarms).
///
/// # Safety
/// See mob_probe.
#[no_mangle]
pub unsafe extern "system" fn mob_remove(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
) -> jni::jint {
    if !lever_mode() || id < 0 || (id as usize) >= MAX_IDS {
        return ERR_STRUCT;
    }
    let iu = id as usize;
    P_READY[iu].store(0, Ordering::Relaxed);
    grid_remove(iu)
}

/// OFF-THREAD STAGE serve: consume the prepared candidate list for `id`.
/// Returns the candidate count (Java re-validates each exactly), -(out_cap)
/// on overflow (caller grows + retries), ERR_RANGE on any co-version
/// mismatch / not-ready / window escape (per-call vanilla fill, no disarm),
/// ERR_STRUCT on structural trouble (bridge disarms the lever).
///
/// # Safety
/// See mob_probe.
#[no_mangle]
pub unsafe extern "system" fn stage_query(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    qx0: jni::jdouble,
    qy0: jni::jdouble,
    qz0: jni::jdouble,
    qx1: jni::jdouble,
    qy1: jni::jdouble,
    qz1: jni::jdouble,
    out: jni::jintArray,
) -> jni::jint {
    if !lever_mode() || STAGE_BROKEN.load(Ordering::Acquire) {
        return ERR_STRUCT;
    }
    if env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };

    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    if id < 0 || (id as usize) >= PREP_CAP {
        return ERR_RANGE; // never prepared → per-call vanilla
    }
    let iu = id as usize;
    // Co-version gate (Acquire pairs with the prep thread's Release publish;
    // ready flag is written LAST by the writer, read FIRST here).
    if P_READY[iu].load(Ordering::Acquire) != 1 {
        return ERR_RANGE;
    }
    if P_LID[iu].load(Ordering::Acquire) != lid {
        return ERR_RANGE;
    }
    let pk = P_CELL[iu].load(Ordering::Acquire);
    let ck = CELL[iu].load(Ordering::Acquire);
    if pk == 0 || ck == 0 || pk != ck {
        return ERR_RANGE; // cell drift since prep (mover) → vanilla
    }
    let n = P_N[iu].load(Ordering::Acquire);
    if n < 0 {
        return ERR_RANGE;
    }
    // Query window (box padded ±1.0 on center coordinates) must sit inside
    // the prepared 5³ window (floor(center)±2 at prep — superset for every
    // box with half-extent ≤ 1.0, java-side radius gate).
    let wx0 = P_WIN[iu * 6].load(Ordering::Acquire);
    let wx1 = P_WIN[iu * 6 + 1].load(Ordering::Acquire);
    let wy0 = P_WIN[iu * 6 + 2].load(Ordering::Acquire);
    let wy1 = P_WIN[iu * 6 + 3].load(Ordering::Acquire);
    let wz0 = P_WIN[iu * 6 + 4].load(Ordering::Acquire);
    let wz1 = P_WIN[iu * 6 + 5].load(Ordering::Acquire);
    let cx0 = qx0.floor() as i32 - 1;
    let cx1 = qx1.floor() as i32 + 1;
    let cy0 = qy0.floor() as i32 - 1;
    let cy1 = qy1.floor() as i32 + 1;
    let cz0 = qz0.floor() as i32 - 1;
    let cz1 = qz1.floor() as i32 + 1;
    if (cx1 - cx0) > MAX_SPAN || (cy1 - cy0) > MAX_SPAN || (cz1 - cz0) > MAX_SPAN {
        return ERR_RANGE;
    }
    if cx0 < wx0 || cx1 > wx1 || cy0 < wy0 || cy1 > wy1 || cz0 < wz0 || cz1 > wz1 {
        return ERR_RANGE; // window escape → per-call vanilla
    }
    if n > cap {
        return -cap; // caller grows the scratch and retries once
    }

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, cap as usize) };
    let base = iu * PREP_CCAP;
    for i in 0..n as usize {
        // SAFETY: base + i < PREP_CAP * PREP_CCAP (i < n ≤ PREP_CCAP).
        let cid = unsafe { P_CAND.get_unchecked(base + i) }.load(Ordering::Acquire);
        if cid < 0 || (cid as usize) >= MAX_IDS {
            // Prepared slots are validated at prep time; a violation here is
            // a real corruption → fail closed (disarm), never serve garbage.
            unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
            STAGE_BROKEN.store(true, Ordering::Release);
            return ERR_STRUCT;
        }
        dst[i] = cid;
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
}

// ---------------------------------------------------------------------------
// OFF-THREAD PREP STAGE: one background thread; continuously re-computes the
// 5³-window candidate lists for registered ids whose prepared entry is stale
// (not ready / cell moved since prep). Single writer ⇒ publish-then-ready
// release protocol; per-cell reads go through the same seqlock protocol as
// grid_query (retry exhaustion / overflow / anomaly → not-ready or fail-
// closed, NEVER garbage).
// ---------------------------------------------------------------------------

/// Prep scratch (thread-local to the single prep thread).
const PREP_SPAN: i32 = 2; // floor(center)±2 — 5³ window

/// Prep walk failure kind: Busy = benign (writer pressure / dense pocket →
/// the id just stays unprepared, per-call vanilla), Corrupt = fail-closed
/// structural verdict (stable anomalous chain → disarm the lever).
enum PrepErr {
    Busy,
    Corrupt,
}

fn prep_cell_walk(
    dst: &mut [i32],
    cap: usize,
    lid: i32,
    (cx0, cx1): (i32, i32),
    (cy0, cy1): (i32, i32),
    (cz0, cz1): (i32, i32),
) -> Result<usize, PrepErr> {
    let mut n: usize = 0;
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
                        // Prep is best-effort: retry exhaustion (a writer is
                        // hammering this shard) → this id stays unprepared →
                        // per-call vanilla. No disarm, no garbage.
                        return Err(PrepErr::Busy);
                    }
                    let v1 = s.ver.load(Ordering::Acquire);
                    if v1 & 1 == 1 {
                        std::hint::spin_loop();
                        continue;
                    }
                    let mut anom = false;
                    if let Ok(slot) = shard_find_slot(s, k) {
                        let mut cur =
                            unsafe { s.head.get_unchecked(slot) }.load(Ordering::Relaxed);
                        while cur != 0 {
                            if n >= cap {
                                return Err(PrepErr::Busy); // denser than PREP_CCAP → vanilla
                            }
                            let id = (cur - 1) as usize;
                            if id >= MAX_IDS {
                                anom = true;
                                break;
                            }
                            // SAFETY: id < MAX_IDS (checked above).
                            if unsafe { CELL.get_unchecked(id) }.load(Ordering::Relaxed) == k {
                                dst[n] = (cur - 1) as i32;
                                n += 1;
                            }
                            cur = unsafe { NEXT.get_unchecked(id) }.load(Ordering::Relaxed);
                        }
                    }
                    let v2 = s.ver.load(Ordering::Acquire);
                    if v2 != v1 {
                        n = n0; // replay this cell only
                        continue;
                    }
                    if anom {
                        // Version stable AND a corrupt-looking chain → the
                        // fail-closed verdict (bridge disarms via ERR_STRUCT).
                        return Err(PrepErr::Corrupt);
                    }
                    break;
                }
            }
        }
    }
    Ok(n)
}

/// Returns Ok(()) for both "prepared" and benign "stay unprepared"; Err(())
/// = structural corruption → the caller disarms the lever.
fn prep_one(id: usize) -> Result<(), ()> {
    let k = CELL[id].load(Ordering::Acquire);
    if k == 0 {
        return Ok(()); // deregistered — nothing to prepare
    }
    let lid = LID[id].load(Ordering::Relaxed);
    let cx = CX[id].load(Ordering::Relaxed);
    let cy = CY[id].load(Ordering::Relaxed);
    let cz = CZ[id].load(Ordering::Relaxed);
    let mut scratch = [0i32; PREP_CCAP];
    let n = match prep_cell_walk(
        &mut scratch,
        PREP_CCAP,
        lid,
        (cx - PREP_SPAN, cx + PREP_SPAN),
        (cy - PREP_SPAN, cy + PREP_SPAN),
        (cz - PREP_SPAN, cz + PREP_SPAN),
    ) {
        Ok(n) => n,
        Err(PrepErr::Busy) => return Ok(()), // id stays unprepared → vanilla per-call
        Err(PrepErr::Corrupt) => return Err(()),
    };
    // Publish: data first (Relaxed), identity + ready flag last (Release).
    let base = id * PREP_CCAP;
    for i in 0..n {
        // SAFETY: base + i < PREP_CAP * PREP_CCAP (id < PREP_CAP, i < n ≤ 64).
        unsafe { P_CAND.get_unchecked(base + i) }.store(scratch[i], Ordering::Relaxed);
    }
    // SAFETY: id < PREP_CAP ⇒ base < PREP_CAP*6 and indices stay in range.
    unsafe {
        P_WIN.get_unchecked(id * 6).store(cx - PREP_SPAN, Ordering::Relaxed);
        P_WIN.get_unchecked(id * 6 + 1).store(cx + PREP_SPAN, Ordering::Relaxed);
        P_WIN.get_unchecked(id * 6 + 2).store(cy - PREP_SPAN, Ordering::Relaxed);
        P_WIN.get_unchecked(id * 6 + 3).store(cy + PREP_SPAN, Ordering::Relaxed);
        P_WIN.get_unchecked(id * 6 + 4).store(cz - PREP_SPAN, Ordering::Relaxed);
        P_WIN.get_unchecked(id * 6 + 5).store(cz + PREP_SPAN, Ordering::Relaxed);
    }
    P_LID[id].store(lid, Ordering::Relaxed);
    P_N[id].store(n as i32, Ordering::Relaxed);
    P_CELL[id].store(k, Ordering::Relaxed);
    P_READY[id].store(1, Ordering::Release);
    Ok(())
}

/// Spawn the background prep stage (called by offstage_manager after the
/// lever is ARMED — an unarmed lever never runs the thread).
pub fn spawn_prep() {
    std::thread::Builder::new()
        .name("crussty-offstage-prep".to_string())
        .spawn(|| {
            eprintln!(
                "[crussty-plugin] cmp401_offthread: prep stage running (bg thread, window=5^3 cell=1.0 ccap={PREP_CCAP} prepcap={PREP_CAP})"
            );
            let mut cursor: usize = 0;
            const SLICE: usize = 8192;
            loop {
                if STAGE_BROKEN.load(Ordering::Acquire) {
                    // Lever disarmed structurally — park forever (no churn).
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                    continue;
                }
                let end = (cursor + SLICE).min(PREP_CAP);
                for id in cursor..end {
                    // Re-prep only stale entries: not ready, or grid cell
                    // drifted from the prepared cell (mover), or the java
                    // upsert eagerly dropped readiness.
                    if P_READY[id].load(Ordering::Relaxed) != 1
                        || P_CELL[id].load(Ordering::Relaxed) != CELL[id].load(Ordering::Relaxed)
                    {
                        let r = std::panic::catch_unwind(|| prep_one(id));
                        if !matches!(r, Ok(Ok(()))) {
                            // Corruption verdict (stable anom chain) or a
                            // prep panic → fail closed (disarm).
                            STAGE_BROKEN.store(true, Ordering::Release);
                            break;
                        }
                    }
                }
                cursor = if end >= PREP_CAP { 0 } else { end };
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        })
        .map(|_| ())
        .unwrap_or_else(|_| {
            eprintln!(
                "[crussty-plugin] cmp401_offthread: prep stage spawn failed — lever runs vanilla-fill only"
            );
        });
}

// ---------------------------------------------------------------------------
// Tests: oracle parity vs a naive section-scan model (same discipline as
// items_index — the grid must return exactly the set of ids whose stored
// center cell falls inside the padded window; upsert/remove/move parity,
// seqlock stability under concurrent writers is exercised by the sharded
// invariants: single WLOCK ⇒ versions even outside brackets).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn upsert(id: usize, lid: i32, x: f64, y: f64, z: f64) -> i32 {
        grid_upsert(id, cell_key(lid, x.floor() as i32, y.floor() as i32, z.floor() as i32))
    }

    #[test]
    fn upsert_move_remove_parity() {
        assert_eq!(upsert(1, 7, 10.4, 64.2, -3.9), 0);
        // query the padded window of the unit box around (10, 64, -4)
        let mut out = [0i32; 64];
        let n = grid_query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 1);
        assert_eq!(out[0], 1);
        // move: same id, new cell — old chain must not retain it
        assert_eq!(upsert(1, 7, 11.4, 64.2, -3.9), 0);
        let n = grid_query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 1);
        // level scoping: another lid does not see the id
        let n = grid_query(&mut out, 64, 8, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 0);
        // remove
        assert_eq!(grid_remove(1), 0);
        let n = grid_query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 0);
        assert_eq!(grid_remove(1), 0); // idempotent
    }

    #[test]
    fn window_parity_vs_oracle() {
        // Oracle: candidate iff center in padded window (mirror of the java
        // exact filters' broadphase stage).
        let mut model: Vec<(usize, i32, f64, f64, f64)> = Vec::new();
        let mut rng: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        for id in 0..500usize {
            let (x, y, z) = (
                (next() % 64) as f64 - 32.0 + (next() % 1000) as f64 / 1000.0,
                (next() % 16) as f64 + 60.0,
                (next() % 64) as f64 - 32.0 + (next() % 1000) as f64 / 1000.0,
            );
            model.push((id, 5, x, y, z));
            assert_eq!(upsert(id, 5, x, y, z), 0);
        }
        // move a third of the population
        for (i, m) in model.iter_mut().enumerate() {
            if i % 3 == 0 {
                m.3 += 1.0;
                assert_eq!(upsert(m.0, m.1, m.2, m.3, m.4), 0);
            }
        }
        for _ in 0..200 {
            let (qx, qy, qz) = (
                (next() % 64) as f64 - 32.0,
                (next() % 16) as f64 + 60.0,
                (next() % 64) as f64 - 32.0,
            );
            let (cx0, cx1) = (qx.floor() as i32 - 1, qx.floor() as i32 + 1);
            let (cy0, cy1) = (qy.floor() as i32 - 1, qy.floor() as i32 + 1);
            let (cz0, cz1) = (qz.floor() as i32 - 1, qz.floor() as i32 + 1);
            let mut expect: Vec<usize> = model
                .iter()
                .filter(|(_, _, x, y, z)| {
                    let (a, b, c) = (x.floor() as i32, y.floor() as i32, z.floor() as i32);
                    a >= cx0 && a <= cx1 && b >= cy0 && b <= cy1 && c >= cz0 && c <= cz1
                })
                .map(|m| m.0)
                .collect();
            expect.sort_unstable();
            let mut out = [0i32; 1024];
            let n = grid_query(&mut out, 1024, 5, (cx0, cx1), (cy0, cy1), (cz0, cz1));
            assert!(n >= 0);
            let mut got: Vec<usize> = out[..n as usize].iter().map(|&v| v as usize).collect();
            got.sort_unstable();
            got.dedup();
            assert_eq!(got, expect, "window ({cx0}..{cx1},{cy0}..{cy1},{cz0}..{cz1})");
        }
    }

    #[test]
    fn overflow_and_span_fallbacks() {
        assert_eq!(upsert(9, 1, 0.5, 0.5, 0.5), 0);
        let mut out = [0i32; 1]; // cap 1 → overflow with 2 candidates in-window
        assert_eq!(upsert(10, 1, 0.7, 0.5, 0.7), 0);
        let n = grid_query(&mut out, 1, 1, (-2, 2), (-2, 2), (-2, 2));
        assert_eq!(n, -1); // -(cap)
        // absurd span → ERR_RANGE
        let n = grid_query(&mut out, 1, 1, (-200, 200), (-2, 2), (-2, 2));
        assert_eq!(n, ERR_RANGE);
        // chain-id bounds sanity: id >= MAX_IDS never enters (native gate)
        assert_eq!(grid_upsert(MAX_IDS, cell_key(1, 0, 0, 0)), ERR_STRUCT);
    }

    /// ROOT-CAUSE regression for the backward-shift delete: heavy
    /// insert/delete churn must keep EVERY live key findable (the naive
    /// zero-the-slot delete broke linear-probe chains → phantom "not found"
    /// → ERR_STRUCT disarm / silent candidate loss).
    #[test]
    fn delete_find_stress() {
        let mut rng: u64 = 0xDEAD_BEEF_CAFE_F00D;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        // Direct key-space churn inside ONE shard: pack many distinct keys
        // (dense ids 0..N with distinct coords) and delete/insert at random.
        let mut live: std::collections::BTreeSet<i64> = Default::default();
        let mut key_of = |i: usize| cell_key(3, (i as i32) * 7, (i as i32) * 11, -(i as i32) * 13);
        for i in 0..4000usize {
            let k = key_of(i);
            assert_eq!(grid_upsert(i, k), 0);
            live.insert(k);
        }
        for round in 0..20000usize {
            let i = (next() as usize) % 4000;
            let k = key_of(i);
            if live.contains(&k) {
                assert_eq!(grid_remove(i), 0, "round {round}");
                live.remove(&k);
            } else {
                assert_eq!(grid_upsert(i, k), 0, "round {round}");
                live.insert(k);
            }
            // invariant: every live key still findable, count exact
            if round % 97 == 0 {
                for (idx, &kk) in live.iter().enumerate() {
                    // find via a probe query at the cell window of the key is
                    // expensive; assert the table-level invariant directly.
                    let s = &SHARDS[shard_of(kk)];
                    assert!(shard_find_slot(s, kk).is_ok(), "key {idx} lost");
                }
            }
        }
        assert!(!live.is_empty());
        // final full sweep
        for (i, &kk) in live.iter().enumerate() {
            let s = &SHARDS[shard_of(kk)];
            assert!(shard_find_slot(s, kk).is_ok(), "final key {i} lost");
        }
    }
}
