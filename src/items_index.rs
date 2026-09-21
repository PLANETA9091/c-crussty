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

//! TASK-399-E (cmp399_rustpre): per-id AABB snapshots (6×f64, flat SoA slab of
//! AtomicU64 bit-patterns, stride 6) + EXACT java AABB.intersects pre-filter in
//! idx_query (bit-in-bit with net/minecraft/world/phys/AABB javap'd from
//! patched-kernel.jar: strict </> via dcmpg/dcmpl — NaN ⇒ false, touching ⇒
//! false). Java pushes the snapshot at insert and on every positional change
//! right after the move-gate (bb is tick-final there), so a concurrent merge
//! query sees the same value vanilla would read fresh; the vanilla java
//! intersects re-check stays on the surviving (small) set — final candidate
//! set is bit-exact vanilla by construction, the java-side AABB/floor share
//! on the merge path collapses instead.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::RwLock;

/// cmp399_rustpre: when false (default) idx_query keeps the EXACT old path
/// (return every grid candidate; the aabb slab simply stays unused). Set by
/// items_manager::activate() when CRUSSTY_LEVER_FLAG contains cmp399_rustpre.
static RUSTPRE: AtomicBool = AtomicBool::new(false);

pub fn set_rustpre(on: bool) {
    RUSTPRE.store(on, Ordering::Relaxed);
}

/// Per-id AABB slab: 6×u64 (f64 bits) per id, [minX,minY,minZ,maxX,maxY,maxZ].
/// VALUE reads/writes are lock-free Relaxed atomics (pushes are hot); only
/// GROWTH takes the RwLock — readers/pushers clone the Arc (short read guard,
/// never blocks on a query-length critical section). Free/never-set slots are
/// NaN: every strict comparison is false ⇒ the candidate is filtered out
/// (a linked id always has real values — insert carries the snapshot).
static AABB: RwLock<Option<std::sync::Arc<Vec<AtomicU64>>>> = RwLock::new(None);

const F64_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;

#[inline]
fn aabb_read() -> std::sync::Arc<Vec<AtomicU64>> {
    let g = match AABB.read() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    match &*g {
        Some(a) => a.clone(),
        None => std::sync::Arc::new(Vec::new()),
    }
}

/// Grow the slab to cover `id` (copy old values, NaN-fill the tail) and
/// return the current snapshot.
fn aabb_ensure(id: usize) -> std::sync::Arc<Vec<AtomicU64>> {
    {
        let g = match AABB.read() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        if let Some(a) = &*g {
            if id * 6 + 5 < a.len() {
                return a.clone();
            }
        }
    }
    let mut w = match AABB.write() {
        Ok(w) => w,
        Err(p) => p.into_inner(),
    };
    let cur = match &*w {
        Some(a) => a.clone(),
        None => std::sync::Arc::new(Vec::new()),
    };
    if id * 6 + 5 >= cur.len() {
        let ncap = ((id + 1).next_power_of_two().max(1024)) * 6;
        let mut nv = Vec::with_capacity(ncap);
        for _ in 0..ncap {
            nv.push(AtomicU64::new(F64_NAN_BITS));
        }
        for i in 0..cur.len() {
            nv[i].store(cur[i].load(Ordering::Relaxed), Ordering::Relaxed);
        }
        *w = Some(std::sync::Arc::new(nv));
    }
    match &*w {
        Some(a) => a.clone(),
        None => unreachable!("aabb slab grown"),
    }
}

#[inline]
fn aabb_store(slots: &[AtomicU64], id: usize, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) {
    let base = id * 6;
    slots[base].store(x0.to_bits(), Ordering::Relaxed);
    slots[base + 1].store(y0.to_bits(), Ordering::Relaxed);
    slots[base + 2].store(z0.to_bits(), Ordering::Relaxed);
    slots[base + 3].store(x1.to_bits(), Ordering::Relaxed);
    slots[base + 4].store(y1.to_bits(), Ordering::Relaxed);
    slots[base + 5].store(z1.to_bits(), Ordering::Relaxed);
}

/// Bit-in-bit java `AABB.intersects(AABB)` with this = candidate box `b`,
/// arg = query box — javap of patched-kernel.jar (strict </> via dcmpg/dcmpl;
/// NaN ⇒ false; touching ⇒ false). Layout of `b`: [minX,minY,minZ,maxX,maxY,maxZ].
#[inline]
fn aabb_hits_candidate(
    b: &[f64; 6],
    qx0: f64,
    qy0: f64,
    qz0: f64,
    qx1: f64,
    qy1: f64,
    qz1: f64,
) -> bool {
    b[0] < qx1 && b[3] > qx0 && b[1] < qy1 && b[4] > qy0 && b[2] < qz1 && b[5] > qz0
}

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
/// TASK-399-E: carries the entity's current AABB (6×f64, [minX,minY,minZ,
/// maxX,maxY,maxZ]) so the linked id is immediately filterable. The java side
/// always passes the real `e.getBoundingBox()` bounds — the slab is correct
/// regardless of the flag; under cmp399_rustpre the query filter consumes it.
#[no_mangle]
pub unsafe extern "system" fn idx_insert(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    cx: jni::jint,
    cy: jni::jint,
    cz: jni::jint,
    bx0: jni::jdouble,
    by0: jni::jdouble,
    bz0: jni::jdouble,
    bx1: jni::jdouble,
    by1: jni::jdouble,
    bz1: jni::jdouble,
) -> jni::jint {
    if id < 0 {
        return ERR_STRUCT;
    }
    let k = cell_key(lid, cx, cy, cz);
    let mut g = idx_mut();
    ensure_id_space(&mut g, id as usize);
    if g.cell[id as usize] != 0 {
        if unlink(&mut g, id as usize).is_err() {
            return ERR_STRUCT;
        }
    }
    let slots = aabb_ensure(id as usize);
    aabb_store(&slots, id as usize, bx0, by0, bz0, bx1, by1, bz1);
    link(&mut g, id as usize, k);
    0
}

/// TASK-399-E: update the AABB snapshot of an already-linked id. Called by the
/// bridge right after the move-gate whenever the entity's position changed
/// this tick (the snapshot is tick-final there) — settled items never call
/// this, so the steady-state push rate decays to zero.
///
/// # Safety
/// See idx_probe.
#[no_mangle]
pub unsafe extern "system" fn idx_set_aabb(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    bx0: jni::jdouble,
    by0: jni::jdouble,
    bz0: jni::jdouble,
    bx1: jni::jdouble,
    by1: jni::jdouble,
    bz1: jni::jdouble,
) -> jni::jint {
    if id < 0 {
        return ERR_STRUCT;
    }
    let slots = aabb_ensure(id as usize);
    aabb_store(&slots, id as usize, bx0, by0, bz0, bx1, by1, bz1);
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
    let mut g = idx_mut();
    if (g.next.len() as i64) <= id as i64 {
        return 0; // never inserted — nothing to do
    }
    if g.cell[id as usize] == 0 {
        return 0;
    }
    match unlink(&mut g, id as usize) {
        Ok(()) => {
            // TASK-399-E hygiene: NaN-out the snapshot so a reused/never-set id
            // can never leak a stale box into the filter.
            let slots = aabb_read();
            let base = id as usize * 6;
            if base + 5 < slots.len() {
                for s in &slots[base..base + 6] {
                    s.store(F64_NAN_BITS, Ordering::Relaxed);
                }
            }
            0
        }
        Err(()) => ERR_STRUCT,
    }
}

/// Query candidates overlapping [qx0..qx1]×[qy0..qy1]×[qz0..qz1] (an
/// already-inflated vanilla merge box). Writes ids into the pinned `out`
/// array; returns the count, -(out_cap) on overflow (caller grows + retries),
/// ERR_RANGE for absurd scan widths, ERR_STRUCT on JNI trouble.
///
/// TASK-399-E (cmp399_rustpre): when armed, each grid candidate is additionally
/// narrowed by the EXACT java AABB.intersects test (strict </>, NaN⇒false,
/// touching⇒false — javap of patched-kernel.jar) against the per-id snapshot
/// slab; only intersecting ids reach the caller. Without the flag the EXACT
/// old path runs (every grid candidate; the slab is unused).
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

    let g = idx();
    let rustpre = RUSTPRE.load(Ordering::Relaxed);
    let aab = if rustpre {
        Some(aabb_read())
    } else {
        None
    };
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                if let Ok(slot) = find_slot(&g.keys, k) {
                    let mut cur = g.head[slot];
                    while cur != 0 {
                        if n >= cap {
                            unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
                            return -cap;
                        }
                        let id = (cur - 1) as usize;
                        if id >= g.next.len() {
                            // TASK-398 fail-closed: a chain id outside the id
                            // space means corruption (stale link) — never walk
                            // it. Release the pin and hand the caller a
                            // structural error so the Java bridge disarms.
                            unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
                            return ERR_STRUCT;
                        }
                        // SAFETY: id < next.len() (checked above); cell[id] == k
                        // because the chain for `k` only contains ids with cell == k.
                        if unsafe { *g.cell.get_unchecked(id) } == k {
                            let mut keep = true;
                            if let Some(slots) = &aab {
                                // TASK-399-E narrow-phase: exact java
                                // other.getBoundingBox().intersects(qb) on the
                                // snapshot (this = candidate, arg = query).
                                let base = id * 6;
                                if base + 5 < slots.len() {
                                    let bx0 =
                                        f64::from_bits(slots[base].load(Ordering::Relaxed));
                                    let by0 =
                                        f64::from_bits(slots[base + 1].load(Ordering::Relaxed));
                                    let bz0 =
                                        f64::from_bits(slots[base + 2].load(Ordering::Relaxed));
                                    let bx1 =
                                        f64::from_bits(slots[base + 3].load(Ordering::Relaxed));
                                    let by1 =
                                        f64::from_bits(slots[base + 4].load(Ordering::Relaxed));
                                    let bz1 =
                                        f64::from_bits(slots[base + 5].load(Ordering::Relaxed));
                                    keep = aabb_hits_candidate(
                                        &[bx0, by0, bz0, bx1, by1, bz1],
                                        qx0,
                                        qy0,
                                        qz0,
                                        qx1,
                                        qy1,
                                        qz1,
                                    );
                                }
                                // else: snapshot slot missing (cannot happen for
                                // a linked id — insert carries it) → defensive
                                // keep; the retained vanilla java intersects
                                // re-check filters the final set.
                            }
                            if keep {
                                dst[n as usize] = cur - 1;
                                n += 1;
                            }
                        }
                        cur = unsafe { *g.next.get_unchecked(id) };
                    }
                }
            }
        }
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
}
