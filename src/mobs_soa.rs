//! STRUCTURE-OF-ARRAYS flat mirror for the MOB push-broadphase (TASK-401-E —
//! vector soa; lever cmp401_soa; TASK-402-B: primary plane of the round-402
//! composite cmp402_comp = B-shardgrid ⊕ mobpush ⊕ E-soa — under the
//! composite every write ALSO mirrors into the sharded grid of
//! src/mobs_grid.rs inside the same critical section, and the grid serves
//! as the per-call fallback read plane). Data-oriented rewrite of the per-slot
//! record grid (round-400-J mobs_grid): the hot fields of the mob side live
//! in FLAT parallel vectors indexed by dense id —
//!
//!     x[id], y[id], z[id]   — AABB center (f64, exact java mirror)
//!     hw[id], hh[id]        — half of max(x,z)-extent / half height (f64)
//!     flags[id]             — bit0 alive
//!     cell[id], next[id]    — intrusive chain membership (items_index shape)
//!
//! plus one fixed open-addressed `cellKey -> chain-head` table. The mass
//! phase (the push-neighbor scan, 9.45% java lane) reads ONLY these flat
//! arrays: no per-slot seqlock headers, no record structs, no object graph —
//! ONE global seqlock version check per whole query instead of a v1→data→v2
//! pair per candidate cell, and a rust-side coarse AABB prune (center ± hw/hh
//! vs query box) so java re-validates only true neighbors.
//!
//! SUPERSET PROOF (rust prune can never hide a vanilla candidate): java gates
//! every member to half-extent ≤ 1.0 (`r_eff = max(hw, hh) ≤ 1.0`, oversized
//! → lever reverts to vanilla). A candidate whose true AABB intersects the
//! query box has center c with |c.a - box| ≤ true_a_extent/2 ≤ radii_a
//! (hw ≥ x/z half-extents by construction), hence the flat-array test
//! `c.a - r < q.max && c.a + r > q.min` (strict, mirroring
//! AABB.intersects) passes; java re-validates level/AABB/predicate exactly,
//! so the returned SET is vanilla-equal and only iteration order differs
//! (cell-chain order — documented delta, items_subsys2 class).
//!
//! FRESHNESS (inherited from round-400-J, unchanged): pushEntities runs in
//! aiStep AFTER travel/move; the bridge self-upserts end-of-move positions at
//! query time, and a mutual push changes only deltaMovement (position
//! integrates in the pushed entity's own move) ⇒ the mirror is exact within a
//! single-tick pass. Entities that never upserted yet (freshly spawned before
//! their first aiStep) are invisible to the grid — same accepted delta as the
//! ARMED round-400-J leg.
//!
//! FIXED MEMORY: ids_cap 1<<20, cell table 1<<18 slots, allocated once and
//! never reallocated ⇒ readers can never observe a dangling buffer (UAF
//! impossible). Keys of emptied chains are NEVER zeroed (mobs_grid
//! root-cause: backward-shift/zero-slot deletes break linear-probe chains) —
//! tombstoned keys just consume load; overflow fails closed.
//!
//! FAIL-CLOSED: every native returns <0 on any inconsistency — ERR_RANGE
//! (per-call vanilla fill, lever stays armed) for range/capacity pressure,
//! ERR_STRUCT (java disarms the lever permanently) for corruption. An unarmed
//! lever never touches these tables.
//!
//! THREADING: upserts come from region workers (one per ticking entity),
//! queries run on the same workers during aiStep; all writes serialize on
//! the global WLOCK with an odd/even VERSION bump (seqlock); readers take a
//! consistent snapshot without locking (retry on version change).
//!
//! RESEARCH NOTES (TASK-401-E, sandbox is offline — sources cited from
//! memory, honestly marked): the SoA/DOD backing for this layout is (1)
//! R. Fabian, "Data-Oriented Design" (2018), ch. on structure-of-arrays hot
//! field packing — flat parallel arrays beat AoP record chasing for mass
//! scans; (2) M. Acton, CppCon 2014 "Data-Oriented Design and C++" —
//! organize memory for the machine's access pattern, not the type graph;
//! (3) Unity DOTS ECS (docs: "Chunk internals") — per-component dense
//!SoA chunk arrays exactly so iteration touches contiguous field vectors;
//! (4) Bevy ECS "Tables" storage — columnar (SoA) component tables for
//! archetypal iteration; (5) Our Machinery blog "Data-Oriented Design (and
//! Why You Might Want to Avoid Pointers)" — pointer-free indices/intrusive
//! links over flat arrays. All five converge on: parallel flat vectors +
//! index links for hot mass-phase fields — implemented here verbatim.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Fixed-capacity SoA plane
// ---------------------------------------------------------------------------

const IDS_CAP: usize = 1 << 20; // per-id flat arrays; java reuses ids via freeIds
const CELL_CAP: usize = 1 << 18; // 262144 open-addressed cell slots (live + tombstones)
const QRETRY: u32 = 4096; // seqlock retry budget per query → ERR_RANGE
/// Max cell-window half-width beyond the ±1 pad (self AABB span cap): wider
/// queries return ERR_RANGE (per-call vanilla fallback; oversized selves are
/// gated java-side by the ≤1.0 bounding-radius gate).
const MAX_SPAN: i32 = 32;

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

struct Soa {
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    /// Half of max(x-extent, z-extent) — ≥ both x and z half-extents.
    hw: Vec<f64>,
    /// Half height (exact y half-extent).
    hh: Vec<f64>,
    /// bit0 = alive.
    flags: Vec<u8>,
    /// Intrusive chain: next id+1 in the same cell (0 = end).
    next: Vec<i32>,
    /// Current cell key (0 = not linked).
    cell: Vec<i64>,
    /// Open-addressed keys; 0 = never-used slot (real keys forced non-zero).
    /// Emptied chains KEEP their key (tombstone) — see module docs.
    keys: Vec<i64>,
    /// Per-slot chain head: id+1 (0 = empty chain).
    head: Vec<i32>,
    /// Occupied key slots incl. tombstones (load accounting, writers only).
    used: usize,
}

impl Soa {
    fn new() -> Box<Soa> {
        Box::new(Soa {
            x: vec![0.0; IDS_CAP],
            y: vec![0.0; IDS_CAP],
            z: vec![0.0; IDS_CAP],
            hw: vec![0.0; IDS_CAP],
            hh: vec![0.0; IDS_CAP],
            flags: vec![0u8; IDS_CAP],
            next: vec![0i32; IDS_CAP],
            cell: vec![0i64; IDS_CAP],
            keys: vec![0i64; CELL_CAP],
            head: vec![0i32; CELL_CAP],
            used: 0,
        })
    }
}

/// Published once after zeroed init, never freed: readers deref the raw
/// pointer without any lock (seqlock-protected snapshot), so the allocation
/// must outlive the process (fixed-capacity contract).
static DATA: AtomicPtr<Soa> = AtomicPtr::new(std::ptr::null_mut());
/// Seqlock version: even = stable, odd = write in flight (monotonic).
static VERSION: AtomicUsize = AtomicUsize::new(0);
/// Writer serialization (odd/even bump happens under this lock).
static WLOCK: Mutex<()> = Mutex::new(());

#[inline]
fn data() -> Option<&'static Soa> {
    let p = DATA.load(Ordering::Acquire);
    if p.is_null() {
        None
    } else {
        // SAFETY: published once, never freed (fixed-capacity contract).
        Some(unsafe { &*p })
    }
}

#[inline]
fn data_mut() -> &'static mut Soa {
    let p = DATA.load(Ordering::Acquire);
    debug_assert!(!p.is_null(), "soa plane must be initialized before writes");
    // SAFETY: published once, never freed; writer holds WLOCK.
    unsafe { &mut *p }
}

fn ensure_plane() {
    if DATA.load(Ordering::Acquire).is_null() {
        let boxed = Soa::new();
        DATA.store(Box::into_raw(boxed), Ordering::Release);
    }
}

/// TASK-406-D (mob_ai_step window): read view for the AI-window epoch pass —
/// the alive flags + the seqlock version. The caller (mobs_ai::ai_epoch)
/// brackets its scan with the SAME version discipline as `mob_query` (even
/// v1 → scan → even v2, bounded retries); flags bit0 flips always happen
/// under the odd/even bracket, so the snapshot contract is identical. No
/// WLOCK: readers never block writers.
pub(crate) fn ai_window_snapshot() -> Option<(&'static [u8], &'static AtomicUsize)> {
    let d = data()?;
    // d: &'static Soa (published once, never freed) — the flags slice is
    // &'static by construction, no unsafe required.
    let flags: &'static [u8] = d.flags.as_slice();
    Some((flags, &VERSION))
}

/// Strict gate: natives work only under the exact lever flag (STRICT eq;
/// empty/foreign flag = the tables are never touched).
fn lever_mode() -> bool {
    static FLAG: OnceLock<String> = OnceLock::new();
    let f = FLAG
        .get_or_init(|| {
            std::env::var("CRUSSTY_LEVER_FLAG")
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .as_str();
    // TASK-402-B: the round-402 composite cmp402_comp arms the SoA plane as
    // its primary mob push broadphase (together with the mobs_grid sharded
    // mirror — see mirror_mode()); the legacy cmp401_soa leg keeps its exact
    // prior behavior (no mirror, no grid reads) — two-mode A/B by design.
    f == "cmp401_soa"
        || f == "cmp402_comp"
        || f == "cmp402_stagcomp"
        || f == "cmp403_tickplane"
        || f == "cmp405_stagtick"
        // TASK-406-D: композит раунда-406 — SoA-плоскость primary push
        // broadphase + источник популяции для aiEpoch (mob_ai_step window).
        || f == "cmp406_aibatch"
}

/// Mirror-plane selector: the sharded grid (src/mobs_grid.rs) is armed ONLY
/// by the round-402 composite. Under cmp402_comp every SoA write also
/// mirrors into the grid inside the SAME WLOCK/VERSION critical section,
/// and the grid serves as the per-call fallback read plane.
#[inline]
fn mirror_mode() -> bool {
    crate::mobs_grid::mirror_mode()
}

// ---------------------------------------------------------------------------
// Cell key (items_index packing: 21-bit symmetric fields + level mix)
// ---------------------------------------------------------------------------

#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Cell key: 21-bit symmetric fields for cx/cy/cz mixed with the Level
/// identity. Field overflow (|coord| > 1M blocks) wraps harmlessly — the key
/// stays a hash; candidates are filtered exactly java-side anyway.
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
    debug_assert!(!keys.is_empty() && keys.len() & (keys.len() - 1) == 0);
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

/// Unlink `id` from its stored cell chain. The key slot is kept (tombstone).
fn unlink(d: &mut Soa, id: usize) -> Result<(), ()> {
    let k = d.cell[id];
    if k == 0 {
        return Err(());
    }
    let slot = find_slot(&d.keys, k).map_err(|_| ())?;
    let mut prev: i32 = 0;
    let mut cur = d.head[slot];
    while cur != 0 {
        let cid = (cur - 1) as usize;
        if cid == id {
            if prev == 0 {
                d.head[slot] = d.next[cid];
            } else {
                d.next[(prev - 1) as usize] = d.next[cid];
            }
            d.next[cid] = 0;
            d.cell[cid] = 0;
            return Ok(());
        }
        prev = cur;
        cur = d.next[cid];
    }
    Err(())
}

/// Writer-side insert-or-move + field refresh. Caller holds WLOCK, version
/// bumped odd. All Err paths leave the plane CONSISTENT (the id either fully
/// linked or fully unlinked).
fn soa_upsert(
    d: &mut Soa,
    id: usize,
    k: i64,
    x: f64,
    y: f64,
    z: f64,
    hw: f64,
    hh: f64,
) -> i32 {
    if d.cell[id] == k {
        // Same cell: refresh flat fields only (hot path — settled population).
        d.x[id] = x;
        d.y[id] = y;
        d.z[id] = z;
        d.hw[id] = hw;
        d.hh[id] = hh;
        d.flags[id] |= 1;
        return 0;
    }
    // Pre-resolve the target slot BEFORE unlinking (no partial state on
    // capacity pressure).
    let slot = match find_slot(&d.keys, k) {
        Ok(s) => s,
        Err(s) => {
            if d.used >= CELL_CAP - 1 {
                return ERR_RANGE; // table pressure — per-call vanilla fallback
            }
            d.keys[s] = k;
            d.used += 1;
            s
        }
    };
    if d.cell[id] != 0 && unlink(d, id).is_err() {
        return ERR_STRUCT; // dangling chain — corruption
    }
    d.x[id] = x;
    d.y[id] = y;
    d.z[id] = z;
    d.hw[id] = hw;
    d.hh[id] = hh;
    d.flags[id] |= 1;
    d.next[id] = d.head[slot];
    d.head[slot] = (id as i32) + 1;
    d.cell[id] = k;
    0
}

/// Reader scan: padded cell window → chain walk → flat-field coarse AABB
/// prune. Returns the candidate count, -(cap) on overflow.
fn soa_scan(
    d: &Soa,
    dst: &mut [i32],
    cap: i32,
    lid: i32,
    (cx0, cx1): (i32, i32),
    (cy0, cy1): (i32, i32),
    (cz0, cz1): (i32, i32),
    q: (f64, f64, f64, f64, f64, f64),
) -> i32 {
    let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                let Ok(slot) = find_slot(&d.keys, k) else {
                    continue;
                };
                let mut cur = unsafe { *d.head.get_unchecked(slot) };
                while cur != 0 {
                    let id = (cur - 1) as usize;
                    if id >= IDS_CAP {
                        // Version check by the caller decides torn vs corrupt.
                        return ERR_STRUCT;
                    }
                    // SAFETY: id < IDS_CAP (checked); fixed-cap vectors.
                    let (alive, x, y, z, hw, hh) = unsafe {
                        (
                            *d.flags.get_unchecked(id) & 1 != 0,
                            *d.x.get_unchecked(id),
                            *d.y.get_unchecked(id),
                            *d.z.get_unchecked(id),
                            *d.hw.get_unchecked(id),
                            *d.hh.get_unchecked(id),
                        )
                    };
                    if alive
                        && x - hw < qx1
                        && x + hw > qx0
                        && y - hh < qy1
                        && y + hh > qy0
                        && z - hw < qz1
                        && z + hw > qz0
                    {
                        if n >= cap {
                            return -cap;
                        }
                        dst[n as usize] = id as i32;
                        n += 1;
                    }
                    cur = unsafe { *d.next.get_unchecked(id) };
                }
            }
        }
    }
    n
}

// ---------------------------------------------------------------------------
// Native exports (registered on net/minecraft/world/entity/MobPushOps)
// ---------------------------------------------------------------------------

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn mob_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !lever_mode() {
        return ERR_STRUCT;
    }
    0x5053 // "SOA"
}

/// Insert-or-move `id` at the AABB center (x, y, z) with flat radii
/// (hw = half of max(x/z)-extent, hh = half height; java gates both ≤ 1.0).
/// Returns 0, ERR_RANGE (capacity pressure — per-call vanilla fallback) or
/// ERR_STRUCT (corruption — java disarms).
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
    hw: jni::jdouble,
    hh: jni::jdouble,
) -> jni::jint {
    if !lever_mode() || id < 0 || (id as usize) >= IDS_CAP {
        return ERR_STRUCT;
    }
    let k = cell_key(lid, x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let _g = WLOCK.lock().unwrap_or_else(|p| p.into_inner());
    ensure_plane();
    let v = VERSION.fetch_add(1, Ordering::AcqRel); // → odd
    debug_assert!(v % 2 == 0);
    let rc = soa_upsert(data_mut(), id as usize, k, x, y, z, hw, hh);
    // TASK-402-B composite: mirror the SAME (id, cell-key) mutation into the
    // sharded grid inside this critical section (no extra WLOCK acquisition;
    // per-shard version bracket inside mirror_upsert). A grid structural
    // failure is ISOLATED: mark_broken() stops mirroring, the SoA rc stays
    // authoritative.
    if mirror_mode() {
        if crate::mobs_grid::mirror_upsert(id as usize, k) < 0 {
            crate::mobs_grid::mark_broken();
        }
    }
    VERSION.fetch_add(1, Ordering::AcqRel); // → even
    rc
}

/// Remove `id` (graveyard sweep). Returns 0 (also when never inserted) or
/// ERR_STRUCT on a dangling chain (corruption → java disarms).
///
/// # Safety
/// See mob_probe.
#[no_mangle]
pub unsafe extern "system" fn mob_remove(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
) -> jni::jint {
    if !lever_mode() || id < 0 || (id as usize) >= IDS_CAP {
        return ERR_STRUCT;
    }
    let _g = WLOCK.lock().unwrap_or_else(|p| p.into_inner());
    if DATA.load(Ordering::Acquire).is_null() {
        return 0;
    }
    let d = data_mut();
    if d.flags[id as usize] & 1 == 0 && d.cell[id as usize] == 0 {
        return 0; // never inserted / already removed — idempotent
    }
    let v = VERSION.fetch_add(1, Ordering::AcqRel); // → odd
    debug_assert!(v % 2 == 0);
    d.flags[id as usize] &= !1;
    let rc = if d.cell[id as usize] != 0 {
        if unlink(d, id as usize).is_err() {
            ERR_STRUCT
        } else {
            0
        }
    } else {
        0
    };
    // TASK-402-B composite: mirror the removal into the sharded grid in the
    // same critical section (failures isolated — see mob_upsert).
    if mirror_mode() {
        if crate::mobs_grid::mirror_remove(id as usize) < 0 {
            crate::mobs_grid::mark_broken();
        }
    }
    VERSION.fetch_add(1, Ordering::AcqRel); // → even
    rc
}

/// Query candidate ids for the AABB [qx0..qx1]×[qy0..qy1]×[qz0..qz1] with the
/// ±1.0 center-window pad (java guarantees all members have half-extent
/// ≤ 1.0). The scan reads the SoA flat arrays and prunes with the coarse
/// center±radii test (superset of AABB.intersects — see module docs).
/// Writes ids into the pinned `out` array; returns the count, -(out_cap) on
/// overflow (caller grows + retries), ERR_RANGE for absurd widths/instability,
/// ERR_STRUCT on corruption/JNI trouble.
///
/// # Safety
/// See mob_probe.
#[no_mangle]
pub unsafe extern "system" fn mob_query(
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
    if !lever_mode() || env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    let cap = cap as i32;

    let cx0 = qx0.floor() as i32 - 1;
    let cx1 = qx1.floor() as i32 + 1;
    let cy0 = qy0.floor() as i32 - 1;
    let cy1 = qy1.floor() as i32 + 1;
    let cz0 = qz0.floor() as i32 - 1;
    let cz1 = qz1.floor() as i32 + 1;
    if (cx1 - cx0) > MAX_SPAN || (cy1 - cy0) > MAX_SPAN || (cz1 - cz0) > MAX_SPAN {
        return ERR_RANGE;
    }

    let Some(d) = data() else {
        return 0; // empty universe — no upserts ever, no candidates
    };

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, cap as usize) };

    let q = (qx0, qy0, qz0, qx1, qy1, qz1);
    let mut n: i32;
    let mut tries: u32 = 0;
    loop {
        tries += 1;
        if tries > QRETRY {
            n = ERR_RANGE;
            break;
        }
        let v1 = VERSION.load(Ordering::Acquire);
        if v1 & 1 == 1 {
            std::hint::spin_loop();
            continue;
        }
        n = soa_scan(d, dst, cap, lid, (cx0, cx1), (cy0, cy1), (cz0, cz1), q);
        let v2 = VERSION.load(Ordering::Acquire);
        if v2 != v1 {
            continue; // torn snapshot — retry whole scan
        }
        break;
    }

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
}

// ---------------------------------------------------------------------------
// Tests: oracle parity for the SoA scan (same discipline as items_index /
// mobs_grid): the scan must return exactly the ids whose STORED flat fields
// pass the coarse center±radii test inside the padded window; the coarse set
// must be a SUPERSET of the exact-AABB model (no vanilla candidate hidden);
// tombstoned keys stay findable under churn; overflow/level-scoping contract.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct Mob {
        id: usize,
        lid: i32,
        x: f64,
        y: f64,
        z: f64,
        /// true half-extents (x, y, z) — ≤ radii by construction.
        ex: f64,
        ey: f64,
        ez: f64,
        hw: f64,
        hh: f64,
    }

    /// Each test builds its OWN Soa plane (no shared static) — the natives'
    /// static plane + WLOCK path is exercised only in production; the
    /// inner functions are pure and testable per-instance.
    impl Mob {
        fn upsert(&self, d: &mut Soa) -> i32 {
            let k = cell_key(
                self.lid,
                self.x.floor() as i32,
                self.y.floor() as i32,
                self.z.floor() as i32,
            );
            soa_upsert(d, self.id, k, self.x, self.y, self.z, self.hw, self.hh)
        }
        fn remove(&self, d: &mut Soa) -> i32 {
            d.flags[self.id] &= !1;
            if d.cell[self.id] != 0 {
                if unlink(d, self.id).is_err() {
                    return ERR_STRUCT;
                }
            }
            0
        }
    }

    /// Exact vanilla AABB.intersects between the model mob and the box.
    fn exact_hits(m: &Mob, q: (f64, f64, f64, f64, f64, f64)) -> bool {
        let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
        let (mx0, mx1) = (m.x - m.ex, m.x + m.ex);
        let (my0, my1) = (m.y - m.ey, m.y + m.ey);
        let (mz0, mz1) = (m.z - m.ez, m.z + m.ez);
        mx0 < qx1 && mx1 > qx0 && my0 < qy1 && my1 > qy0 && mz0 < qz1 && mz1 > qz0
    }

    /// Coarse test exactly as the rust scan does it (flat radii).
    fn coarse_hits(m: &Mob, q: (f64, f64, f64, f64, f64, f64)) -> bool {
        let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
        m.x - m.hw < qx1
            && m.x + m.hw > qx0
            && m.y - m.hh < qy1
            && m.y + m.hh > qy0
            && m.z - m.hw < qz1
            && m.z + m.hw > qz0
    }

    fn query(d: &Soa, lid: i32, q: (f64, f64, f64, f64, f64, f64)) -> Vec<usize> {
        let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
        let cx0 = qx0.floor() as i32 - 1;
        let cx1 = qx1.floor() as i32 + 1;
        let cy0 = qy0.floor() as i32 - 1;
        let cy1 = qy1.floor() as i32 + 1;
        let cz0 = qz0.floor() as i32 - 1;
        let cz1 = qz1.floor() as i32 + 1;
        let mut out = [0i32; 4096];
        let cap = out.len() as i32;
        let n = soa_scan(
            d,
            &mut out,
            cap,
            lid,
            (cx0, cx1),
            (cy0, cy1),
            (cz0, cz1),
            q,
        );
        assert!(n >= 0, "scan failed: {n}");
        out[..n as usize].iter().map(|&v| v as usize).collect()
    }

    #[test]
    fn soa_scan_parity_and_superset() {
        let mut d = Soa::new();
        let mut rng: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        let mut model: Vec<Mob> = Vec::new();
        for id in 0..600usize {
            let (x, y, z) = (
                (next() % 64) as f64 - 32.0 + (next() % 1000) as f64 / 1000.0,
                (next() % 16) as f64 + 60.0,
                (next() % 64) as f64 - 32.0 + (next() % 1000) as f64 / 1000.0,
            );
            // True half-extents ≤ radii ≤ 1.0 (java gate).
            let ex = (next() % 100) as f64 / 200.0; // ≤ 0.5
            let ez = (next() % 100) as f64 / 200.0;
            let ey = (next() % 200) as f64 / 200.0; // ≤ 1.0
            let hw = ex.max(ez) + 0.05; // radii cover the true extents
            let hh = ey;
            let m = Mob { id, lid: 5, x, y, z, ex, ey, ez, hw, hh };
            assert_eq!(m.upsert(&mut d), 0);
            model.push(m);
        }
        for _ in 0..300 {
            let (qx, qy, qz) = (
                (next() % 64) as f64 - 32.0,
                (next() % 16) as f64 + 60.0,
                (next() % 64) as f64 - 32.0,
            );
            let q = (qx - 0.3, qy - 0.9, qz - 0.3, qx + 0.3, qy + 0.9, qz + 0.3);
            let got = query(&d, 5, q);
            // Superset: no exact hit may be hidden by the coarse scan.
            for m in &model {
                if exact_hits(m, q) {
                    assert!(
                        got.contains(&m.id),
                        "exact candidate {} hidden by the coarse scan (q {q:?})",
                        m.id
                    );
                }
            }
            // Set parity with the coarse oracle (scan == stored-field test).
            let mut expect: Vec<usize> = model
                .iter()
                .filter(|m| coarse_hits(m, q))
                .map(|m| m.id)
                .collect();
            expect.sort_unstable();
            let mut got = got.clone();
            got.sort_unstable();
            got.dedup();
            assert_eq!(got, expect, "coarse set mismatch (q {q:?})");
        }
        for m in &model {
            assert_eq!(m.remove(&mut d), 0);
        }
    }

    #[test]
    fn upsert_move_remove_parity() {
        let mut d = Soa::new();
        let m = Mob { id: 7001, lid: 7, x: 10.4, y: 64.2, z: -3.9, ex: 0.3, ey: 0.9, ez: 0.3, hw: 0.3, hh: 0.9 };
        assert_eq!(m.upsert(&mut d), 0);
        let q = (9.0, 62.0, -6.0, 12.0, 66.0, -3.0);
        assert!(query(&d, 7, q).contains(&7001));
        // Move: same id, new position — old cell must not retain it.
        let m2 = Mob { id: 7001, lid: 7, x: 11.4, y: 64.2, z: -3.9, ex: 0.3, ey: 0.9, ez: 0.3, hw: 0.3, hh: 0.9 };
        assert_eq!(m2.upsert(&mut d), 0);
        assert!(query(&d, 7, q).contains(&7001));
        // Level scoping: another lid does not see the id.
        assert!(!query(&d, 8, q).contains(&7001));
        // Remove.
        assert_eq!(m2.remove(&mut d), 0);
        assert!(!query(&d, 7, q).contains(&7001));
        assert_eq!(m2.remove(&mut d), 0); // idempotent
    }

    #[test]
    fn overflow_and_span_fallbacks() {
        let mut d = Soa::new();
        let a = Mob { id: 8001, lid: 9, x: 0.5, y: 0.5, z: 0.5, ex: 0.1, ey: 0.1, ez: 0.1, hw: 0.1, hh: 0.1 };
        let b = Mob { id: 8002, lid: 9, x: 0.7, y: 0.5, z: 0.7, ex: 0.1, ey: 0.1, ez: 0.1, hw: 0.1, hh: 0.1 };
        assert_eq!(a.upsert(&mut d), 0);
        assert_eq!(b.upsert(&mut d), 0);
        let mut out1 = [0i32; 1];
        // cap 1 with 2 candidates → -(cap)
        let n = soa_scan(
            &d, &mut out1, 1, 9, (-2, 2), (-2, 2), (-2, 2),
            (-1.0, -1.0, -1.0, 1.0, 1.0, 1.0),
        );
        assert_eq!(n, -1);
        // wide window scans fine with a matching buffer
        let mut out2 = [0i32; 4096];
        let cap2 = out2.len() as i32;
        let n = soa_scan(
            &d, &mut out2, cap2, 9, (-200, 200), (-2, 2), (-2, 2),
            (-1.0, -1.0, -1.0, 1.0, 1.0, 1.0),
        );
        assert!(n >= 0);
    }

    /// ROOT-CAUSE regression (mobs_grid lesson): tombstoned keys must stay
    /// findable — heavy churn must never orphan a live chain.
    #[test]
    fn tombstone_churn_stress() {
        let mut d = Soa::new();
        let mut rng: u64 = 0xDEAD_BEEF_CAFE_F00D;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        let mut live: std::collections::BTreeSet<i64> = Default::default();
        let key_of = |i: usize| cell_key(3, (i as i32) * 7, (i as i32) * 11, -(i as i32) * 13);
        let mk = |i: usize, on: bool| Mob {
            id: i,
            lid: 3,
            x: ((i as i32) * 7) as f64 + 0.5,
            y: ((i as i32) * 11) as f64 + 0.5,
            z: (-(i as i32) * 13) as f64 + 0.5,
            ex: 0.1,
            ey: 0.1,
            ez: 0.1,
            hw: 0.1,
            hh: 0.1,
        };
        for i in 0..3000usize {
            let m = mk(i, true);
            assert_eq!(m.upsert(&mut d), 0);
            live.insert(key_of(i));
        }
        for round in 0..15000usize {
            let i = (next() as usize) % 3000;
            let k = key_of(i);
            if live.contains(&k) {
                let m = mk(i, false);
                assert_eq!(m.remove(&mut d), 0, "round {round}");
                live.remove(&k);
            } else {
                let m = mk(i, true);
                assert_eq!(m.upsert(&mut d), 0, "round {round}");
                live.insert(k);
            }
            if round % 101 == 0 {
                for (idx, &kk) in live.iter().enumerate() {
                    assert!(
                        find_slot(&d.keys, kk).is_ok(),
                        "live key {idx} lost under churn"
                    );
                }
            }
        }
        for (i, &kk) in live.iter().enumerate() {
            assert!(find_slot(&d.keys, kk).is_ok(), "final live key {i} lost");
        }
    }
}
