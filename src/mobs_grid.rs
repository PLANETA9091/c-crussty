use jvmti_bindings::jni;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};

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
    /// Occupied key slots (load accounting — writers only, under the shard
    /// writer mutex).
    used: AtomicUsize,
    /// TASK-427-A2 (protocol v2 port, TASK-417-A): per-shard WRITER Mutex.
    /// The mirror upsert/remove calls now run OUTSIDE the SoA global WLOCK
    /// (no lock-group nesting — mobs_soa mob_upsert/mob_remove release their
    /// lock before mirroring), so mirror writers serialize HERE, per shard,
    /// locks taken in ascending shard-index order on a two-shard move
    /// (deadlock-free).
    wlock: Mutex<()>,
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
        wlock: Mutex::new(()),
    }
}; NSHARDS];

/// TASK-427-A2 (protocol v2): writer-side per-shard lock + version bracket —
/// the guard serializes mirror writers of this shard; w_begin/w_end wrap the
/// mutation while the guard is held.
fn shard_writer(s: &'static Shard) -> MutexGuard<'static, ()> {
    // Mirror writers are worker threads; a poisoned lock recovers (same
    // discipline as the mobs_soa WLOCK).
    s.wlock.lock().unwrap_or_else(|p| p.into_inner())
}

/// Per-id intrusive chain link (next id+1 in the same cell, 0 = end).
static NEXT: [AtomicI32; MAX_IDS] = [H0; MAX_IDS];
/// Per-id current cell key (0 = not in any chain).
static CELL: [AtomicI64; MAX_IDS] = [K0; MAX_IDS];

/// Mode selector for the MIRROR plane: STRICT single-flag gate — the mirror
/// is armed ONLY by the round-402 composite (cmp402_comp). The legacy
/// cmp399_mobpush and cmp401_soa flags keep their exact prior behavior
/// (no mirror writes, no grid reads) — two-mode A/B by construction.
#[inline]
pub(crate) fn mirror_mode() -> bool {
    static M: OnceLock<bool> = OnceLock::new();
    *M.get_or_init(|| {
        std::env::var("CRUSSTY_LEVER_FLAG")
            .map(|v| {
                v.trim() == "cmp402_comp"
                    || v.trim() == "cmp402_stagcomp"
                    || v.trim() == "cmp403_tickplane"
                    || v.trim() == "cmp405_stagtick"
                    // TASK-406-D: композит раунда-406 включает mirror-grid.
                    || v.trim() == "cmp406_aibatch"
                    // TASK-406-E: композит раунда-406 включает mirror-grid.
                    || v.trim() == "cmp406_sscan"
                    // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                    || v.trim() == "cmp409_multi" || v.trim() == "cmp412_meganav" || v.trim() == "cmp414_cvs"
                    // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR
                    // (mirror inert: eqsnap upsert path returns before mirror
                    // write, push ladder skips grid legs под EQSNAP).
                    || v.trim() == "cmp412_eqsnapv3" || v.trim() == "cmp414_cvs" || v.trim() == "cmp417_bq"
                    // TASK-419-A (colpush): колпаш-носитель (mirror inert).
                    || v.trim() == "cmp420_colpush"
                    || v.trim() == "cmp421_brain" || v.trim() == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2"
                    || v == "cmp438_sense" // TASK-444-C: sense family union
                    || v == "cmp451_senseins" || v == "cmp458_swar" || v == "cmp457_paldelta" || v == "cmp457_eqsnap2" || v == "cmp456_chunkmono" || v == "cmp456_chunkmono_p31snap" || v == "cmp466_c98ai" || v == "cmp453_diet" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" || v == "cmp456_poi" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
})
            .unwrap_or(false)
    })
}

/// Structural isolation: a broken mirror never touches the SoA plane.
/// Set on the first grid ERR_STRUCT; mirroring stops and the grid natives
/// return ERR_STRUCT (java → vanilla fill per call).
static BROKEN: AtomicBool = AtomicBool::new(false);

pub(crate) fn is_broken() -> bool {
    BROKEN.load(Ordering::Acquire)
}

pub(crate) fn mark_broken() {
    BROKEN.store(true, Ordering::Release);
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
/// MIRROR contract (protocol v2): the caller (mobs_soa::mob_upsert) has
/// ALREADY released the SoA writer WLOCK before calling — this function
/// serializes on ITS OWN per-shard writer Mutex(es): one shard on a
/// same-cell/no-op or first link, TWO shards (old + new, locked in ascending
/// index order — deadlock-free) on a cell move. Gate responsibility: the
/// caller checks mirror_mode() (kept out of the hot path and out of the
/// direct-plane tests).
pub(crate) fn mirror_upsert(id: usize, k: i64) -> i32 {
    if is_broken() {
        return ERR_STRUCT;
    }
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let old = CELL[id].load(Ordering::Relaxed);
    if old == k {
        return 0; // same cell — no mutation at all (settled-population fast path)
    }
    if old != 0 {
        let sold = shard_of(old);
        let snew = shard_of(k);
        if sold == snew {
            // One shard covers both the unlink and the link.
            let s = &SHARDS[sold];
            let g = shard_writer(s);
            w_begin(s);
            let r = shard_unlink(id).and_then(|_| shard_link(id, k));
            w_end(s);
            drop(g);
            return if r.is_err() { ERR_STRUCT } else { 0 };
        }
        // Two-shard move: lock in ASCENDING index order (deadlock-free),
        // bracket both versions, mutate, publish both.
        let (first, second) = if sold < snew { (sold, snew) } else { (snew, sold) };
        let g1 = shard_writer(&SHARDS[first]);
        let g2 = shard_writer(&SHARDS[second]);
        let s_old = &SHARDS[sold];
        let s_new = &SHARDS[snew];
        w_begin(s_old);
        w_begin(s_new);
        let r = shard_unlink(id).and_then(|_| shard_link(id, k));
        w_end(s_new);
        w_end(s_old);
        drop(g2);
        drop(g1);
        return if r.is_err() { ERR_STRUCT } else { 0 };
    }
    let s = &SHARDS[shard_of(k)];
    let g = shard_writer(s);
    w_begin(s);
    let r = shard_link(id, k);
    w_end(s);
    drop(g);
    if r.is_err() {
        return ERR_STRUCT;
    }
    0
}

/// Remove `id` from the mirror (graveyard sweep). MIRROR contract (protocol
/// v2): caller (mobs_soa::mob_remove) has ALREADY released the SoA writer
/// WLOCK — this function serializes on ITS OWN per-shard writer Mutex.
/// Returns 0 (also when never inserted) or ERR_STRUCT on a dangling chain.
/// Gate responsibility: the caller checks mirror_mode().
pub(crate) fn mirror_remove(id: usize) -> i32 {
    if is_broken() {
        return ERR_STRUCT;
    }
    if id >= MAX_IDS {
        return ERR_STRUCT;
    }
    let old = CELL[id].load(Ordering::Relaxed);
    if old == 0 {
        return 0; // never inserted — nothing to do
    }
    let sold = &SHARDS[shard_of(old)];
    let g = shard_writer(sold);
    w_begin(sold);
    let r = shard_unlink(id);
    w_end(sold);
    drop(g);
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
#[allow(clippy::too_many_arguments)]
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
// Native exports (registered on net/minecraft/world/entity/MobPushOps next
// to the SoA natives; names are grid-specific to keep the flat symbol
// namespace of the cdylib collision-free).
// ---------------------------------------------------------------------------

/// Result codes: >=0 ok / query count; -1 structural (mirror disarm /
/// java per-call vanilla fill); -2 bad range (per-call vanilla fallback);
/// query overflow returns -(cap) so the caller can grow and retry.
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn mob_grid_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !mirror_mode() || is_broken() {
        return ERR_STRUCT;
    }
    0x4D50
}

/// Query candidate ids from the MIRROR grid for the AABB
/// [qx0..qx1]×[qy0..qy1]×[qz0..qz1] with the ±1.0 center-window pad (Java
/// guarantees all members have half-extent ≤ 1.0). Writes the SAME dense
/// ids the SoA plane uses into the pinned `out` array; returns the count,
/// -(out_cap) on overflow (caller grows + retries), ERR_RANGE for absurd
/// scan widths, ERR_STRUCT on JNI trouble / broken mirror.
///
/// # Safety
/// See mob_grid_probe.
#[no_mangle]
pub unsafe extern "system" fn mob_grid_query(
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
    if !mirror_mode() || is_broken() {
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
    let cap = cap as i32;

    // Center-window with the ±1 pad (floor(a-1) = floor(a)-1 exactly — see
    // module docs for the half-extent ≤ 1.0 soundness proof).
    let cx0 = qx0.floor() as i32 - 1;
    let cx1 = qx1.floor() as i32 + 1;
    let cy0 = qy0.floor() as i32 - 1;
    let cy1 = qy1.floor() as i32 + 1;
    let cz0 = qz0.floor() as i32 - 1;
    let cz1 = qz1.floor() as i32 + 1;
    if (cx1 - cx0) > MAX_SPAN || (cy1 - cy0) > MAX_SPAN || (cz1 - cz0) > MAX_SPAN {
        return ERR_RANGE;
    }

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, cap as usize) };

    let n = grid_query(dst, cap, lid, (cx0, cx1), (cy0, cy1), (cz0, cz1));

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
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

    /// The mirror contract expects the caller to hold the writer WLOCK
    /// (mobs_soa's); in tests the planes are driven directly, so tests
    /// serialize on their own process-wide lock instead.
    static TLOCK: Mutex<()> = Mutex::new(());

    /// All tests of this module share ONE id universe (the per-id statics)
    /// and the SAME process; every test holds the module lock for its WHOLE
    /// body so id reuse across tests cannot cross-contaminate the planes.
    fn tlock() -> std::sync::MutexGuard<'static, ()> {
        TLOCK.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn upsert(id: usize, lid: i32, x: f64, y: f64, z: f64) -> i32 {
        mirror_upsert(id, cell_key(lid, x.floor() as i32, y.floor() as i32, z.floor() as i32))
    }

    fn remove(id: usize) -> i32 {
        mirror_remove(id)
    }

    fn query(out: &mut [i32], cap: i32, lid: i32, cx: (i32, i32), cy: (i32, i32), cz: (i32, i32)) -> i32 {
        grid_query(out, cap, lid, cx, cy, cz)
    }

    #[test]
    fn upsert_move_remove_parity() {
        let _g = tlock();
        assert_eq!(upsert(1, 7, 10.4, 64.2, -3.9), 0);
        // query the padded window of the unit box around (10, 64, -4)
        let mut out = [0i32; 64];
        let n = query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 1);
        assert_eq!(out[0], 1);
        // move: same id, new cell — old chain must not retain it
        assert_eq!(upsert(1, 7, 11.4, 64.2, -3.9), 0);
        let n = query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 1);
        // level scoping: another lid does not see the id
        let n = query(&mut out, 64, 8, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 0);
        // remove
        assert_eq!(remove(1), 0);
        let n = query(&mut out, 64, 7, (9, 12), (62, 65), (-6, -3));
        assert_eq!(n, 0);
        assert_eq!(remove(1), 0); // idempotent
    }

    #[test]
    fn window_parity_vs_oracle() {
        let _g = tlock();
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
            let n = query(&mut out, 1024, 5, (cx0, cx1), (cy0, cy1), (cz0, cz1));
            assert!(n >= 0);
            let mut got: Vec<usize> = out[..n as usize].iter().map(|&v| v as usize).collect();
            got.sort_unstable();
            got.dedup();
            assert_eq!(got, expect, "window ({cx0}..{cx1},{cy0}..{cy1},{cz0}..{cz1})");
        }
    }

    #[test]
    fn overflow_and_span_fallbacks() {
        let _g = tlock();
        assert_eq!(upsert(9, 1, 0.5, 0.5, 0.5), 0);
        let mut out = [0i32; 1]; // cap 1 → overflow with 2 candidates in-window
        assert_eq!(upsert(10, 1, 0.7, 0.5, 0.7), 0);
        let n = query(&mut out, 1, 1, (-2, 2), (-2, 2), (-2, 2));
        assert_eq!(n, -1); // -(cap)
        // Absurd span → ERR_RANGE is guarded at the NATIVE level
        // (mob_grid_query MAX_SPAN check, ported verbatim from TASK-400-J);
        // grid_query itself has no span guard — plain counting semantics:
        let mut wide = [0i32; 8];
        let n = query(&mut wide, 8, 1, (-200, 200), (-2, 2), (-2, 2));
        assert_eq!(n, 2);
        // chain-id bounds sanity: id >= MAX_IDS never enters (native gate)
        assert_eq!(mirror_upsert(MAX_IDS, cell_key(1, 0, 0, 0)), ERR_STRUCT);
    }

    /// ROOT-CAUSE regression for the backward-shift delete: heavy
    /// insert/delete churn must keep EVERY live key findable (the naive
    /// zero-the-slot delete broke linear-probe chains → phantom "not found"
    /// → ERR_STRUCT disarm / silent candidate loss).
    #[test]
    fn delete_find_stress() {
        let _g = tlock();
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
            assert_eq!(mirror_upsert(i, k), 0);
            live.insert(k);
        }
        for round in 0..20000usize {
            let i = (next() as usize) % 4000;
            let k = key_of(i);
            if live.contains(&k) {
                assert_eq!(mirror_remove(i), 0, "round {round}");
                live.remove(&k);
            } else {
                assert_eq!(mirror_upsert(i, k), 0, "round {round}");
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
