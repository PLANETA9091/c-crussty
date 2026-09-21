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

use jvmti_bindings::jni;
use std::sync::RwLock;

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

    let g = idx();
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
                            dst[n as usize] = cur - 1;
                            n += 1;
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

// ---------------------------------------------------------------------------
// TASK-399-A (cmp399_batch): BATCH-JNI transport — idxBatchApply
// ---------------------------------------------------------------------------
//
// Один нативный вызов на фазу тик-бакета на поток вместо per-item
// idx_insert/idx_set_cell/idx_remove/idx_query. Java (ItemEntityManager)
// копит за фазу плоский int[]-стрим (temporal order данного потока):
//
//   [OP_INSERT=1,  id, lid, cx, cy, cz]        (6 ints)
//   [OP_SETCELL=2, id, lid, cx, cy, cz]        (6 ints)
//   [OP_REMOVE=3,  id, 0,  0,  0,  0]          (6 ints)
//   [OP_QUERY=4,   lid, cx0, cy0, cz0, cx1, cy1, cz1] (8 ints)
//
// QUERY-окно приходит уже проскейленным JAVA-side двойной математикой
// (cx0 = floor(minX)-1 …) — натив не видит double вовсе, паритет окна
// побайтовый с per-call idx_query. Rust обрабатывает записи последовательно
// под ОДНИМ захватом RwLock (write — стрим содержит и мутации, и запросы),
// с ОДНИМ GetPrimitiveArrayCritical на вход и ОДНИМ на выход. Кандидаты
// merge-запросов возвращаются одним плоским массивом:
//
//   out[0] = 0 (rc ok — код возврата несёт результат)
//   out[1] = Q (число query-записей)
//   out[2 + 2j], out[3 + 2j] = (offset, len) пары по query j
//   out[blobStart + off .. +len] = id-кандидаты (blobStart = 2 + 2*Q)
//
// ПАРИТЕТ: семантика каждой op-записи — байт-в-байт реплика
// соответствующего per-call натива; порядок кандидатов в окне — тот же
// (cz→cy→cx вложенные, chain order per cell). Точные AABB/уровень/walls
// фильтры остаются java-side (ItemEntityManager.applyMergeResults) в
// ванильном порядке.
//
// КОДЫ ВОЗВРАТА: 0 = ок; ERR_STRUCT (-1) = структурный отказ (java disarms
// подсистему целиком, как per-call); BATCH_OVERFLOW (i32::MIN) = out массив
// мал — java растит его и переигрывает ВЕСЬ батч (все ops идемпотентны:
// insert=unlink+link, setCell=unlink+link, remove=noop-безопасен).

/// Sentinel переполнения out-массива (никогда не совпадает с -1/-2/-cap).
pub const BATCH_OVERFLOW: i32 = i32::MIN;

enum BatchErr {
    Struct,
    Overflow,
}

/// Реплика per-call idx_insert (fail-closed семантика 1:1).
fn batch_insert(g: &mut Inner, id: i32, lid: i32, cx: i32, cy: i32, cz: i32) -> Result<(), BatchErr> {
    if id < 0 {
        return Err(BatchErr::Struct);
    }
    let k = cell_key(lid, cx, cy, cz);
    ensure_id_space(g, id as usize);
    if g.cell[id as usize] != 0 {
        if unlink(g, id as usize).is_err() {
            return Err(BatchErr::Struct);
        }
    }
    link(g, id as usize, k);
    Ok(())
}

/// Реплика per-call idx_set_cell.
fn batch_set_cell(g: &mut Inner, id: i32, lid: i32, cx: i32, cy: i32, cz: i32) -> Result<(), BatchErr> {
    if id < 0 {
        return Err(BatchErr::Struct);
    }
    let k = cell_key(lid, cx, cy, cz);
    if (g.next.len() as i64) <= id as i64 {
        return Err(BatchErr::Struct); // never inserted
    }
    if g.cell[id as usize] == k {
        return Ok(());
    }
    if unlink(g, id as usize).is_err() {
        return Err(BatchErr::Struct);
    }
    link(g, id as usize, k);
    Ok(())
}

/// Реплика per-call idx_remove.
fn batch_remove(g: &mut Inner, id: i32) -> Result<(), BatchErr> {
    if id < 0 {
        return Err(BatchErr::Struct);
    }
    if (g.next.len() as i64) <= id as i64 {
        return Ok(()); // never inserted — nothing to do
    }
    if g.cell[id as usize] == 0 {
        return Ok(());
    }
    unlink(g, id as usize).map_err(|_| BatchErr::Struct)
}

/// Реплика per-call idx_query по уже проскейленному java-side окну.
/// Порядок обхода клеток — cz→cy→cx (как в idx_query), chain order per cell.
#[allow(clippy::too_many_arguments)]
fn batch_query(
    g: &Inner,
    lid: i32,
    cx0: i32,
    cy0: i32,
    cz0: i32,
    cx1: i32,
    cy1: i32,
    cz1: i32,
    dst: &mut [jni::jint],
    header: usize,
    blob_cap: usize,
    blob: &mut usize,
) -> Result<(i32, i32), BatchErr> {
    // Java-side guard уже отсёк экзотические окна; защита от i32-переполнения
    // на абсурдных координатах — fail-closed (недостижимо на вооружённом пути).
    if (cx1 as i64 - cx0 as i64) > 6
        || (cy1 as i64 - cy0 as i64) > 6
        || (cz1 as i64 - cz0 as i64) > 6
    {
        return Err(BatchErr::Struct);
    }
    let off = *blob as i32;
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                if let Ok(slot) = find_slot(&g.keys, k) {
                    let mut cur = g.head[slot];
                    while cur != 0 {
                        let id = (cur - 1) as usize;
                        if id >= g.next.len() {
                            // TASK-398 fail-closed: stale link — не ходим.
                            return Err(BatchErr::Struct);
                        }
                        if unsafe { *g.cell.get_unchecked(id) } == k {
                            if *blob >= blob_cap {
                                return Err(BatchErr::Overflow);
                            }
                            dst[header + *blob] = cur - 1;
                            *blob += 1;
                            n += 1;
                        }
                        cur = unsafe { *g.next.get_unchecked(id) };
                    }
                }
            }
        }
    }
    Ok((off, n))
}

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn idx_batch_apply(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    ops: jni::jintArray,
    ops_len: jni::jint,
    out: jni::jintArray,
) -> jni::jint {
    if env.is_null() || ops.is_null() || out.is_null() || ops_len < 0 {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let ops_cap = unsafe { (vt.GetArrayLength)(env, ops) };
    if (ops_len as i64) > ops_cap as i64 {
        return ERR_STRUCT;
    }
    let out_cap = unsafe { (vt.GetArrayLength)(env, out) } as usize;

    // ОДИН критический пин входа на весь батч.
    let ops_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, ops, std::ptr::null_mut()) };
    if ops_pin.is_null() {
        return ERR_STRUCT;
    }
    let src: &[jni::jint] =
        unsafe { std::slice::from_raw_parts(ops_pin as *const jni::jint, ops_len as usize) };

    // Pre-scan: валидация записей + подсчёт query-записей (нужен для заголовка
    // out: пары offset+len идут ПЕРЕД блобом кандидатов).
    let mut q: usize = 0;
    let mut parse_ok = true;
    let mut i = 0usize;
    while i < src.len() {
        let op = unsafe { *src.get_unchecked(i) };
        let w = match op {
            1 | 2 | 3 => 6usize,
            4 => 8usize,
            _ => {
                parse_ok = false;
                break;
            }
        };
        if i + w > src.len() {
            parse_ok = false;
            break;
        }
        if op == 4 {
            q += 1;
        }
        i += w;
    }
    let header: usize = 2 + 2 * q;
    if !parse_ok || out_cap < header {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, ops, ops_pin, 0) };
        return if parse_ok { BATCH_OVERFLOW } else { ERR_STRUCT };
    }
    let blob_cap: usize = out_cap - header;

    // ОДИН критический пин выхода на весь батч.
    let out_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if out_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, ops, ops_pin, 0) };
        return ERR_STRUCT;
    }
    let dst: &mut [jni::jint] =
        unsafe { std::slice::from_raw_parts_mut(out_pin as *mut jni::jint, out_cap) };

    // ОДИН захват лока на весь батч (write: стрим содержит мутации и запросы).
    let mut g = idx_mut();
    let mut blob: usize = 0;
    let mut q_written: usize = 0;
    let mut i = 0usize;
    let mut res: Result<(), BatchErr> = Ok(());
    while i < src.len() {
        let op = unsafe { *src.get_unchecked(i) };
        let r = match op {
            1 => batch_insert(&mut g, src[i + 1], src[i + 2], src[i + 3], src[i + 4], src[i + 5]),
            2 => batch_set_cell(&mut g, src[i + 1], src[i + 2], src[i + 3], src[i + 4], src[i + 5]),
            3 => batch_remove(&mut g, src[i + 1]),
            4 => batch_query(
                &g,
                src[i + 1],
                src[i + 2],
                src[i + 3],
                src[i + 4],
                src[i + 5],
                src[i + 6],
                src[i + 7],
                dst,
                header,
                blob_cap,
                &mut blob,
            )
            .map(|(off, n)| {
                dst[2 + 2 * q_written] = off;
                dst[3 + 2 * q_written] = n;
                q_written += 1;
            }),
            _ => Err(BatchErr::Struct),
        };
        if let Err(e) = r {
            res = Err(e);
            break;
        }
        i += if op == 4 { 8 } else { 6 };
    }
    let rc = match res {
        Err(BatchErr::Overflow) => BATCH_OVERFLOW,
        Err(BatchErr::Struct) => ERR_STRUCT,
        Ok(()) => {
            dst[0] = 0;
            dst[1] = q_written as jni::jint;
            0
        }
    };
    drop(g);
    // Релиз пинов: обратный порядок захвату. Внутри критических регионов —
    // только плоские проходы по массивам, ни одного JNI-колбэка.
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, out_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, ops, ops_pin, 0) };
    rc
}
