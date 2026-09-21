//! OFF-THREAD PREPARE-STAGE entity index (TASK-401-D, vector offthread —
//! lever cmp401_offthread).
//!
//! ARCHITECTURE (the round-401-D unique vector): heavy per-tick entity
//! broadphase scans (getEntities family ≈8-9% java wall post-J) are split
//! into a BACKGROUND PREPARE stage and a tick-time EXACT APPLY:
//!
//!   1. GATHER (tick threads, ~tens of ns/entity, ZERO JNI per entity): at
//!      the end of each region bucket tick (RegionTickOps.tickBucket hook,
//!      gated) the worker packs the entities it just ticked — id, level,
//!      center, r_eff — into reused thread-local arrays and hands them to
//!      rust in ONE bulk JNI call per region per tick (4 crossings/tick vs
//!      1-per-entity-per-tick upserts: the batchjni lesson applied at the
//!      WRITE side).
//!   2. PREPARE (background rust thread): when all W region batches of a
//!      tick are in, rebuild a COMPLETE immutable snapshot of the entity
//!      spatial index — two resolution planes: 1.0 cells (push lane, small
//!      boxes) and 8.0 cells (target/avoid lane, follow-range boxes) — and
//!      publish via an Arc swap. Published snapshots are IMMUTABLE: readers
//!      never retry, never spin — no seqlock needed (the mobpush seqlock
//!      existed because writers shared the read table; here the only writer
//!      builds the NEXT snapshot privately).
//!   3. APPLY (tick threads): retargeted vanilla call sites
//!      (LivingEntity.pushEntities → PrepareOps.pushables;
//!      NearestAttackableTargetGoal.findTarget / Mob.aiStep /
//!      AvoidEntityGoal.canUse → PrepareOps.targetList/classList2) fetch the
//!      candidate id set in ONE JNI call and re-validate EVERY candidate
//!      exactly (level, isRemoved, AABB.intersects, vanilla predicate) —
//!      candidate-set parity by construction: the snapshot is a SUPERSET of
//!      the vanilla section scan (positions one tick stale, covered by
//!      QUERY_PAD; bounding radius bounded by the 1.0 r_eff gate; movement
//!      beyond MOVE_LIMIT between consecutive batched positions disarms
//!      fail-closed).
//!
//! PARITY: vanilla results, only the computation time moves (owner
//! formulation of the off-thread contract). Documented deltas, same class
//! the owner bar accepts for region ticks: candidate iteration order =
//! cell-chain order (items_subsys2/mobpush doctrine); entities spawned
//! mid-phase enter the grid next tick (they also begin ticking next tick —
//! EntityTickList semantics).
//!
//! FAIL-CLOSED: every native returns <0 on any inconsistency; ERR_STRUCT
//! (-1) disarms permanently (java bridge goes 100% vanilla), ERR_RANGE (-2)
//! falls back per call. Any fixed-table overflow = ERR_STRUCT. Unarmed
//! (flag != cmp401_offthread) nothing here runs.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, RwLock};

pub const PROBE_MAGIC: i32 = 0x5052; // "PR"

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

const NSHARDS: usize = 64;
const SHARD_CAP: usize = 1 << 14; // 16384 key slots/shard -> 1M cell keys
const MAX_IDS: usize = 1 << 20;
/// Query pad per axis (blocks): covers one tick of entity movement between
/// the snapshot moment (end of previous tick) and the query moment, plus
/// cell quantization. Movement beyond MOVE_LIMIT between two consecutive
/// batched positions of the same id disarms the lever (oversized).
pub const QUERY_PAD: f64 = 3.0;
pub const MOVE_LIMIT: f64 = 4.0;
/// r_eff gate (java computes the half-extent; > 1.0 -> oversized -> disarm),
/// mirrored rust-side from the batch rows.
pub const R_EFF_LIMIT: f64 = 1.0;
/// Max cell span per axis per plane (beyond it -> ERR_RANGE -> per-call
/// vanilla; push boxes ~8 cells on the 1.0 plane, follow-range boxes ~10 on
/// the 8.0 plane — limits only trip on pathological geometry).
const MAX_SPAN_A: i32 = 24;
const MAX_SPAN_B: i32 = 40;

/// Mode selector, sampled once (process-wide env).
#[inline]
pub fn lever_mode() -> bool {
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

/// Cell key: 21-bit symmetric cx/cz fields mixed with the Level identity.
#[inline]
fn cell_key(lid: i32, cx: i32, cz: i32) -> i64 {
    const OFF: i64 = 1 << 20;
    const MSK: i64 = (1 << 21) - 1;
    let a = ((cx as i64) + OFF) & MSK;
    let c = ((cz as i64) + OFF) & MSK;
    let packed = a | (c << 21);
    let h = mix64((packed as u64) ^ ((lid as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)));
    let h = if h == 0 { 0x9E37_79B9_7F4A_7C15 } else { h };
    h as i64
}

#[inline]
fn slot_start(k: i64) -> usize {
    (mix64(k as u64) as usize) & (SHARD_CAP - 1)
}

// ---------------------------------------------------------------------------
// Immutable snapshot (two cell planes over one flat key table)
// ---------------------------------------------------------------------------

pub struct Snap {
    pub tick: i32,
    /// Flat key table shared by both planes? No — planes have distinct key
    /// spaces; each plane owns its own table.
    pub planes: [Plane; 2],
    /// Per-id position rows of this snapshot: [x, y, z] (movement gate input
    /// for the NEXT rebuild; length = max id seen + 1).
    pub pos: Vec<[f64; 3]>,
    pub ids: usize,
}

pub struct Plane {
    pub cell_size: f64,
    pub keys: Vec<i64>, // NSHARDS*SHARD_CAP; 0 = free (real keys forced non-zero)
    pub head: Vec<i32>, // per-key chain head: id+1 (0 = empty)
    pub next: Vec<i32>, // per-id chain link: id+1 (0 = end); len = ids seen
    pub used: usize,
}

const KEYS_A0: i64 = 0;

impl Plane {
    fn new(cell_size: f64) -> Plane {
        Plane {
            cell_size,
            keys: vec![KEYS_A0; NSHARDS * SHARD_CAP],
            head: vec![0i32; NSHARDS * SHARD_CAP],
            next: Vec::new(),
            used: 0,
        }
    }

    /// Find-or-insert key slot (single background writer — plain ops).
    /// Probing: start at slot_start(k), linear wrap over the WHOLE table.
    #[inline]
    fn key_slot(&mut self, k: i64) -> Result<usize, ()> {
        let mut i = slot_start(k);
        for _ in 0..SHARD_CAP {
            let cur = self.keys[i];
            if cur == k {
                return Ok(i);
            }
            if cur == 0 {
                if self.used >= (NSHARDS * SHARD_CAP) * 5 / 8 {
                    return Err(());
                }
                self.keys[i] = k;
                self.used += 1;
                return Ok(i);
            }
            i = (i + 1) % (NSHARDS * SHARD_CAP);
        }
        Err(())
    }

    #[inline]
    fn probe(&self, k: i64) -> Result<usize, ()> {
        let mut i = slot_start(k);
        for _ in 0..SHARD_CAP {
            let cur = self.keys[i];
            if cur == k {
                return Ok(i);
            }
            if cur == 0 {
                return Err(());
            }
            i = (i + 1) % (NSHARDS * SHARD_CAP);
        }
        Err(())
    }

    #[inline]
    fn insert(&mut self, lid: i32, id: i32, x: f64, z: f64, r: f64) -> Result<(), ()> {
        if (id as usize) >= self.next.len() {
            return Err(());
        }
        let cs = self.cell_size;
        let x0 = ((x - r) / cs).floor() as i32;
        let x1 = ((x + r) / cs).floor() as i32;
        let z0 = ((z - r) / cs).floor() as i32;
        let z1 = ((z + r) / cs).floor() as i32;
        let mut cx = x0;
        while cx <= x1 {
            let mut cz = z0;
            while cz <= z1 {
                let k = cell_key(lid, cx, cz);
                let slot = self.key_slot(k)?;
                self.next[id as usize] = self.head[slot];
                self.head[slot] = id + 1;
                cz += 1;
            }
            cx += 1;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Published snapshot slot + reader caching
// ---------------------------------------------------------------------------

static SNAP_SLOT: RwLock<Option<Arc<Snap>>> = RwLock::new(None);
static PUBLISHED_TICK: AtomicI32 = AtomicI32::new(-1);
static OVERSIZED: AtomicUsize = AtomicUsize::new(0); // 1 = disarm (r/movement gate)
static BROKEN: AtomicUsize = AtomicUsize::new(0); // 1 = structural failure, disarm

pub fn published_tick() -> i32 {
    PUBLISHED_TICK.load(Ordering::Acquire)
}

pub fn is_disarmed() -> bool {
    OVERSIZED.load(Ordering::Relaxed) != 0 || BROKEN.load(Ordering::Relaxed) != 0
}

fn publish(snap: Arc<Snap>) {
    PUBLISHED_TICK.store(snap.tick, Ordering::Release);
    *SNAP_SLOT.write().unwrap() = Some(snap);
}

thread_local! {
    static SNAP_CACHE: std::cell::RefCell<Option<(i32, Arc<Snap>)>> =
        const { std::cell::RefCell::new(None) };
}

/// Reader: cheap tick fast-path (thread-local cached Arc), RwLock only on
/// tick flip. Err(ERR_RANGE) when disarmed / no snapshot yet.
#[inline]
fn current_snap() -> Result<Arc<Snap>, i32> {
    if is_disarmed() {
        return Err(ERR_RANGE);
    }
    let want = PUBLISHED_TICK.load(Ordering::Acquire);
    let cached = SNAP_CACHE
        .with(|c| c.borrow_mut().as_ref().and_then(|(t, a)| (*t == want).then(|| Arc::clone(a))));
    let snap = match cached {
        Some(a) => a,
        None => {
            let g = SNAP_SLOT.read().unwrap();
            let a = match g.as_ref() {
                Some(a) => Arc::clone(a),
                None => return Err(ERR_RANGE),
            };
            drop(g);
            SNAP_CACHE.with(|c| *c.borrow_mut() = Some((want, Arc::clone(&a))));
            a
        }
    };
    Ok(snap)
}

// ---------------------------------------------------------------------------
// Batch staging (region workers) -> background rebuild thread
// ---------------------------------------------------------------------------

struct Batch {
    slot: u32,
    meta: Vec<i32>, // 2 per entity: id, lid
    pos: Vec<f64>,  // 4 per entity: x, y, z, r
}

struct Stage {
    tick: i32,
    mask: u32,
    parts: Vec<Batch>,
}

static STAGE: Mutex<Option<Stage>> = Mutex::new(None);
static STAGE_CV: Condvar = Condvar::new();
static W_REG: AtomicUsize = AtomicUsize::new(0);
static QUEUE: Mutex<std::collections::VecDeque<(i32, Vec<Batch>)>> =
    Mutex::new(std::collections::VecDeque::new());

/// W (region workers) registration — via prepInit, idempotent.
pub fn init(w: i32) -> i32 {
    if !(1..=8).contains(&w) {
        return ERR_RANGE;
    }
    let mut st = STAGE.lock().unwrap();
    if st.is_some() {
        return PROBE_MAGIC;
    }
    W_REG.store(w as usize, Ordering::Release);
    *st = Some(Stage {
        tick: -1,
        mask: 0,
        parts: Vec::new(),
    });
    drop(st);
    std::thread::Builder::new()
        .name("crussty-prepare".into())
        .spawn(rebuild_loop)
        .map(|_| PROBE_MAGIC)
        .unwrap_or(ERR_STRUCT)
}

/// Bulk ingest: one call per region worker per tick.
pub fn batch(tick: i32, slot: i32, meta: &[i32], pos: &[f64]) -> i32 {
    if BROKEN.load(Ordering::Relaxed) != 0 {
        return ERR_STRUCT;
    }
    if OVERSIZED.load(Ordering::Relaxed) != 0 {
        return ERR_RANGE;
    }
    let w = W_REG.load(Ordering::Acquire);
    if w == 0 || slot < 0 || slot as usize >= w || meta.len() % 2 != 0 || pos.len() != meta.len() * 2 {
        BROKEN.store(1, Ordering::Relaxed);
        return ERR_STRUCT;
    }
    let n = meta.len() / 2;
    for j in 0..n {
        let r = pos[j * 4 + 3];
        if !r.is_finite() || r > R_EFF_LIMIT || r < 0.0 {
            OVERSIZED.store(1, Ordering::Relaxed);
            eprintln!(
                "[crussty-plugin] cmp401_offthread: oversized r_eff {r} (id {}) — lever reverted to vanilla",
                meta[j * 2]
            );
            return ERR_RANGE;
        }
        let id = meta[j * 2];
        if !(0..MAX_IDS as i32).contains(&id) {
            BROKEN.store(1, Ordering::Relaxed);
            return ERR_STRUCT;
        }
    }
    // Movement gate vs the PREVIOUS published snapshot's positions.
    {
        let g = SNAP_SLOT.read().unwrap();
        if let Some(s) = g.as_ref() {
            for j in 0..n {
                let id = meta[j * 2] as usize;
                if id < s.pos.len() {
                    let p = s.pos[id];
                    if p[0].is_finite() {
                        let dx = pos[j * 4] - p[0];
                        let dy = pos[j * 4 + 1] - p[1];
                        let dz = pos[j * 4 + 2] - p[2];
                        if dx.abs() > MOVE_LIMIT || dy.abs() > MOVE_LIMIT || dz.abs() > MOVE_LIMIT
                        {
                            OVERSIZED.store(1, Ordering::Relaxed);
                            eprintln!(
                                "[crussty-plugin] cmp401_offthread: oversized movement d=({dx:.2},{dy:.2},{dz:.2}) id {} — lever reverted to vanilla",
                                meta[j * 2]
                            );
                            return ERR_RANGE;
                        }
                    }
                }
            }
        }
    }
    let mut st_guard = STAGE.lock().unwrap();
    let st = match st_guard.as_mut() {
        Some(s) => s,
        None => {
            BROKEN.store(1, Ordering::Relaxed);
            return ERR_STRUCT;
        }
    };
    if st.tick != tick {
        st.tick = tick;
        st.mask = 0;
        st.parts.clear();
    }
    st.mask |= 1u32 << slot;
    st.parts.push(Batch {
        slot: slot as u32,
        meta: meta.to_vec(),
        pos: pos.to_vec(),
    });
    if st.mask.count_ones() == w {
        let parts: Vec<Batch> = st.parts.drain(..).collect();
        let t = st.tick;
        st.tick = -1;
        st.mask = 0;
        drop(st_guard);
        QUEUE.lock().unwrap().push_back((t, parts));
        STAGE_CV.notify_one();
    }
    0
}

fn rebuild_loop() {
    loop {
        let job = {
            let mut q = QUEUE.lock().unwrap();
            loop {
                if let Some(j) = q.pop_front() {
                    break j;
                }
                q = STAGE_CV.wait(q).unwrap();
            }
        };
        if is_disarmed() {
            continue;
        }
        match rebuild(job.0, &job.1) {
            Ok(snap) => publish(Arc::new(snap)),
            Err(code) => {
                BROKEN.store(1, Ordering::Relaxed);
                eprintln!(
                    "[crussty-plugin] cmp401_offthread: rebuild failed code {code} — lever disarmed (vanilla)"
                );
            }
        }
    }
}

fn rebuild(tick: i32, parts: &[Batch]) -> Result<Snap, i32> {
    // Row count + max id
    let mut total = 0usize;
    let mut max_id: i32 = -1;
    for b in parts {
        total += b.meta.len() / 2;
        for j in 0..b.meta.len() / 2 {
            let id = b.meta[j * 2];
            if id > max_id {
                max_id = id;
            }
        }
    }
    let ids = (max_id + 1).max(0) as usize;
    let mut pos: Vec<[f64; 3]> = vec![[f64::INFINITY; 3]; ids];
    let mut planes = [Plane::new(1.0), Plane::new(8.0)];
    for p in &mut planes {
        p.next = vec![0i32; ids];
    }
    for b in parts {
        for j in 0..b.meta.len() / 2 {
            let id = b.meta[j * 2] as usize;
            let lid = b.meta[j * 2 + 1];
            let x = b.pos[j * 4];
            let y = b.pos[j * 4 + 1];
            let z = b.pos[j * 4 + 2];
            let r = b.pos[j * 4 + 3];
            if !x.is_finite() || !y.is_finite() || !z.is_finite() || !r.is_finite() {
                return Err(ERR_STRUCT);
            }
            pos[id] = [x, y, z];
            planes[0].insert(lid, id as i32, x, z, r).map_err(|_| ERR_STRUCT)?;
            planes[1].insert(lid, id as i32, x, z, r).map_err(|_| ERR_STRUCT)?;
        }
    }
    Ok(Snap {
        tick,
        planes,
        pos,
        ids: total,
    })
}

// ---------------------------------------------------------------------------
// Query (tick threads)
// ---------------------------------------------------------------------------

thread_local! {
    static STAMPS: std::cell::RefCell<Vec<i32>> = const { std::cell::RefCell::new(Vec::new()) };
    static EPOCH: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

/// Query one plane. Fills `ids` with deduped candidate ids; returns the count
/// written, Err(ERR_RANGE) on span overflow (per-call vanilla fallback) or
/// Err(ERR_STRUCT) on dangling chains (permanent disarm).
fn grid_query(p: &Plane, lid: i32, n0: i32, n1: i32, z0: i32, z1: i32, ids: &mut [i32]) -> Result<usize, i32> {
    let cs = p.cell_size;
    let max_span = if cs < 2.0 { MAX_SPAN_A } else { MAX_SPAN_B };
    if (n1 - n0) > max_span || (z1 - z0) > max_span {
        return Err(ERR_RANGE);
    }
    STAMPS.with(|st| {
        {
            let mut stamps = st.borrow_mut();
            if stamps.len() < p.next.len() {
                stamps.resize(p.next.len(), 0);
            }
            let ep = EPOCH.with(|e| {
                let v = e.get().wrapping_add(1).max(0);
                e.set(v);
                v
            });
            let mut n = 0usize;
            let mut cx = n0;
            while cx <= n1 {
                let mut cz = z0;
                while cz <= z1 {
                    let k = cell_key(lid, cx, cz);
                    if let Ok(slot) = p.probe(k) {
                        let mut link = p.head[slot];
                        while link != 0 {
                            let id = link - 1;
                            if id < 0 || (id as usize) >= p.next.len() {
                                return Err(ERR_STRUCT);
                            }
                            if stamps[id as usize] != ep {
                                stamps[id as usize] = ep;
                                if n >= ids.len() {
                                    return Err(ERR_RANGE); // cap overflow -> per-call vanilla
                                }
                                ids[n] = id;
                                n += 1;
                            }
                            link = p.next[id as usize];
                        }
                    }
                    cz += 1;
                }
                cx += 1;
            }
            Ok(n)
        }
    })
}

/// Query one plane of the published snapshot. Writes out[0] = published tick,
/// ids at out[1..]. Returns ids written (excluding out[0]), or negative codes.
fn query(
    lid: i32,
    minx: f64,
    miny: f64,
    minz: f64,
    maxx: f64,
    maxy: f64,
    maxz: f64,
    plane: i32,
    out: &mut [i32],
) -> i32 {
    if out.len() < 2 || !(0..2).contains(&plane) {
        return ERR_STRUCT;
    }
    if !minx.is_finite()
        || !miny.is_finite()
        || !minz.is_finite()
        || !maxx.is_finite()
        || !maxy.is_finite()
        || !maxz.is_finite()
        || maxx < minx
        || maxy < miny
        || maxz < minz
    {
        return ERR_RANGE;
    }
    let snap = match current_snap() {
        Ok(s) => s,
        Err(code) => return code,
    };
    let p = &snap.planes[plane as usize];
    let cs = p.cell_size;
    let n0 = ((minx - QUERY_PAD) / cs).floor() as i32;
    let n1 = ((maxx + QUERY_PAD) / cs).floor() as i32;
    let z0 = ((minz - QUERY_PAD) / cs).floor() as i32;
    let z1 = ((maxz + QUERY_PAD) / cs).floor() as i32;
    let cap = out.len() - 1;
    let ids = &mut out[1..];
    match grid_query(p, lid, n0, n1, z0, z1, &mut ids[..cap]) {
        Ok(n) => {
            out[0] = published_tick();
            n as i32
        }
        Err(code) => code,
    }
}

// JNI surface (mobs_grid ABI discipline: extern "system", vt accessor,
// null-guards, fail-closed) ----------------------------------------------

/// # Safety
/// Called via RegisterNatives from the armed manager only.
pub unsafe extern "system" fn prep_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !lever_mode() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// # Safety
/// Called via RegisterNatives from the armed manager only.
pub unsafe extern "system" fn prep_init(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    w: jni::jint,
) -> jni::jint {
    if !lever_mode() {
        return ERR_STRUCT;
    }
    init(w)
}

/// # Safety
/// Called via RegisterNatives from the armed manager only.
pub unsafe extern "system" fn prep_batch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    slot: jni::jint,
    meta: jni::jintArray,
    pos: jni::jdoubleArray,
) -> jni::jint {
    if !lever_mode() {
        return ERR_STRUCT;
    }
    if env.is_null() || meta.is_null() || pos.is_null() {
        BROKEN.store(1, Ordering::Relaxed);
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let mlen = unsafe { (vt.GetArrayLength)(env, meta) };
    let plen = unsafe { (vt.GetArrayLength)(env, pos) };
    if mlen < 0 || plen < 0 || plen != mlen * 2 {
        BROKEN.store(1, Ordering::Relaxed);
        return ERR_STRUCT;
    }
    let mut mbuf: Vec<i32> = vec![0; mlen as usize];
    let mut pbuf: Vec<f64> = vec![0.0; plen as usize];
    unsafe {
        (vt.GetIntArrayRegion)(env, meta, 0, mlen, mbuf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, pos, 0, plen, pbuf.as_mut_ptr());
    }
    batch(tick, slot, &mbuf, &pbuf)
}

/// # Safety
/// Called via RegisterNatives from the armed manager only.
#[allow(clippy::too_many_arguments)]
pub unsafe extern "system" fn prep_query(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    minx: jni::jdouble,
    miny: jni::jdouble,
    minz: jni::jdouble,
    maxx: jni::jdouble,
    maxy: jni::jdouble,
    maxz: jni::jdouble,
    lid: jni::jint,
    plane: jni::jint,
    out: jni::jintArray,
) -> jni::jint {
    if !lever_mode() {
        return ERR_STRUCT;
    }
    if env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap < 2 {
        return ERR_STRUCT;
    }
    let mut obuf: Vec<i32> = vec![0; cap as usize];
    let rc = query(
        lid, minx, miny, minz, maxx, maxy, maxz, plane, &mut obuf,
    );
    if rc >= 0 {
        unsafe { (vt.SetIntArrayRegion)(env, out, 0, rc + 1, obuf.as_ptr()) };
    }
    rc
}
