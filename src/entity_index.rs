//! SHARDED ENTITY-CHUNK MIRROR for the getEntities broadphase lane
//! (TASK-405-C, vector eindex — lever cmp405_eindex; bridge
//! entityquery/net/minecraft/world/entity/EntityIndexOps.java, retargets in
//! src/classfile.rs patch_eindex_*, wiring in src/entity_index_manager.rs).
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
//! mutex (batches, not scalars); readers take per-shard seqlock snapshots
//! (QRETRY budget → ERR_RANGE → per-call vanilla fallback). Any structural
//! failure (table overflow / probe exhaustion) → ERR_STRUCT → the bridge
//! disarms permanently (broken=true) → exact vanilla replication forever.
//!
//! FAIL-CLOSED: natives are registered only under the STRICT-eq lever flag;
//! ERR codes never change the result list, only skip-or-fallback decisions.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
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

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;
pub const PROBE_MAGIC: i32 = 0x4549_4445; // "EIDX"

const K0: AtomicI64 = AtomicI64::new(0);
const U0: AtomicU64 = AtomicU64::new(0);
const I0: AtomicI32 = AtomicI32::new(0);
const V0: AtomicUsize = AtomicUsize::new(0);

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
    /// id table (open-addressed): i_key = id+1 (0 = free, never erased —
    /// dead entries have i_slot == 0 and are revived by id reuse).
    i_key: [AtomicI64; ID_CAP],
    i_cell: [AtomicI64; ID_CAP],
    i_slot: [AtomicI32; ID_CAP],
}

static SHARDS: [Shard; NSHARDS] = [const {
    Shard {
        ver: V0,
        keys: [K0; CHUNK_CAP],
        head: [I0; CHUNK_CAP],
        s_id: [I0; SLOT_CAP],
        s_next: [I0; SLOT_CAP],
        s_bb: [U0; 6 * SLOT_CAP],
        i_key: [K0; ID_CAP],
        i_cell: [K0; ID_CAP],
        i_slot: [I0; ID_CAP],
    }
}; NSHARDS];

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

/// (WLOCK) chunk-table find-or-insert.
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

/// (WLOCK) push-front a slot onto the chain of chunk entry `ki`.
fn chain_push(sh: &Shard, ki: usize, slot: usize) {
    let old = sh.head[ki].load(Ordering::Acquire);
    sh.s_next[slot].store(old, Ordering::Relaxed);
    sh.head[ki].store(slot as i32 + 1, Ordering::Release);
}

/// (WLOCK) unlink slot `slot` (id+1 = want) from the chain of chunk entry
/// `ki`. Returns false if not found (ghost — tolerated as no-op).
fn chain_unlink(sh: &Shard, ki: usize, slot: usize, want: i32) -> bool {
    let mut cur = sh.head[ki].load(Ordering::Acquire);
    let mut prev: usize = usize::MAX;
    while cur != 0 {
        let s = (cur - 1) as usize;
        if s == slot && sh.s_id[s].load(Ordering::Acquire) == want {
            let nxt = sh.s_next[s].load(Ordering::Acquire);
            if prev == usize::MAX {
                sh.head[ki].store(nxt, Ordering::Release);
            } else {
                sh.s_next[prev].store(nxt, Ordering::Release);
            }
            return true;
        }
        prev = s;
        cur = sh.s_next[s].load(Ordering::Acquire);
    }
    false
}

fn write_bb(sh: &Shard, slot: usize, bb: &[f64; 6]) {
    for j in 0..6 {
        sh.s_bb[6 * slot + j].store(bb[j].to_bits(), Ordering::Relaxed);
    }
}

/// One sync op: (op, id, cx, cz, bb6). op: 0=BB, 1=ADD, 2=REMOVE.
struct Op {
    op: u8,
    id: i32,
    cx: i32,
    cz: i32,
    bb: [f64; 6],
}

fn apply_ops(ops: &[Op]) -> i32 {
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
                // ADD: upsert home cell + bb (idempotent by id).
                let ckey = chunk_key(op.cx, op.cz);
                let cshard = shard_of(ckey);
                bump_odd!(cshard);
                let entry = match id_probe(ish, op.id) {
                    Some((ei, old_cell, slot)) if slot != usize::MAX && old_cell == ckey => {
                        // same cell — bb update only
                        let sh = &SHARDS[cshard];
                        if let Some(ki) = chunk_find(sh, ckey) {
                            write_bb(sh, slot, &op.bb);
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
                                if chain_unlink(osh, ki, os, id_key(op.id) as i32) {
                                    st.free[ocshard].push(os as i32);
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
                    csh.s_id[slot].store(id_key(op.id) as i32, Ordering::Relaxed);
                    write_bb(csh, slot, &op.bb);
                    chain_push(csh, ki, slot);
                    ish.i_cell[ei].store(ckey, Ordering::Release);
                    ish.i_slot[ei].store(slot as i32 + 1, Ordering::Release);
                }
            }
            2 => {
                // REMOVE: drop the id from its cell chain (ghost = no-op).
                if let Some((ei, old_cell, slot)) = id_probe(ish, op.id) {
                    if slot != usize::MAX && old_cell != 0 {
                        let cshard = shard_of(old_cell);
                        bump_odd!(cshard);
                        let csh = &SHARDS[cshard];
                        if let Some(ki) = chunk_find(csh, old_cell) {
                            let os = (slot - 1) as usize;
                            if chain_unlink(csh, ki, os, id_key(op.id) as i32) {
                                csh.s_id[os].store(0, Ordering::Relaxed);
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
                        if let Some(_ki) = chunk_find(csh, old_cell) {
                            write_bb(csh, slot, &op.bb);
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

fn count_chunk(cx: i32, cz: i32, b: &[f64; 6]) -> Result<i32, i32> {
    let key = chunk_key(cx, cz);
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
                let mut cur = sh.head[ki].load(Ordering::Acquire);
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
        if committed(sh, v1) {
            return Ok(count);
        }
        std::hint::spin_loop();
    }
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
    _min_sec: jni::jint,
    _max_sec: jni::jint,
    out_counts: jni::jintArray,
) -> jni::jint {
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
            match count_chunk(min_cx + dx as i32, min_cz + dz as i32, &q) {
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
