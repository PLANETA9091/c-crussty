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
//! every member to half-extent ≤ 2.0 (`r_eff = max(hw, hh) ≤ 2.0`, oversized
//! → lever reverts to vanilla; TASK-411-C k4soa raised the gate 1.0 → 2.0 and
//! the query center-window pad to ±2 cells to cover camel/iron_golem/warden
//! population radii). A candidate whose true AABB intersects the
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
use std::cell::RefCell;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Fixed-capacity SoA plane
// ---------------------------------------------------------------------------

const IDS_CAP: usize = 1 << 20; // per-id flat arrays; java reuses ids via freeIds
const CELL_CAP: usize = 1 << 18; // 262144 open-addressed cell slots (live + tombstones)
/// TASK-427-A2 (protocol v2 port, TASK-417-A afbc1dc): striped seqlock shard count.
const N_SHARDS: usize = 64;
/// TASK-427-A2: bounded per-cell retry budget → ERR_RANGE (fail-open).
const SHARD_RETRIES: u32 = 8;
/// TASK-427-A2: linear-probe cap in find_slot (no infinite walk on a full /
/// tombstone-degenerate table; the caller treats the cap as pressure →
/// tombstone-reclaim rebuild). PROFILE ROOT-CAUSE round-mf427l2: base had NO
/// reclaim — `used` grew monotonically toward CELL_CAP under 300 ticks of mob
/// wandering (every cell-move leaves a tombstone), probe walks degenerated and
/// soa_upsert burned 12.2% self CPU (~120ms/tick) before saturating to
/// permanent ERR_RANGE. The cap + rebuild is the primary TPS-recovery fix.
const PROBE_CAP: usize = 128;
/// Sentinel Err slot of find_slot when the probe cap is hit.
const PROBE_FAIL: usize = usize::MAX;
/// TASK-411-C (k4soa): center-window pad in CELLS. The java radius gate
/// (MobPushOps.upsertSelf) admits r_eff = max(hw, hh) ≤ 2.0 (camel 1.1875 /
/// iron_golem 1.35 / warden 1.45 in the bench population), so an intersecting
/// AABB's CENTER can sit up to 2.0 blocks outside the query box and its cell
/// up to 2 cells below floor(q0). SOUNDNESS: floor(q0 − hw) ≥ floor(q0) − 2
/// for hw ≤ 2.0 (floor(a − n·cell) = floor(a) − n for integer n·cell shifts),
/// therefore pad ≥ ceil(RADIUS_GATE) = 2. MUST stay ≥ ceil of the java gate
/// (MobPushOps.RADIUS_GATE) — the pair is the superset contract.
const PAD: i32 = 2;

// TASK-411-C (eqsnap, v2 — пост-мортем run 35691270899 RED 0.5 TPS):
// cl1-профиль (84140 samples): mob_upsert = 24.9% CPU — per-entity JNI
// upsert (48k/тик), каждый под ГЛОБАЛЬНЫМ WLOCK + seqlock + 1-блочный
// cell-хэш; eq_epoch full chain build = 21 sample (0.025%) — сам rebuild НЕ
// дорог. V2 = dirty-дельты: под cmp411_eqsnap / cmp412_eqsnapv3 mob_upsert НЕ трогает
// плоскость — строка (id,alive,x,y,z,hw,hh) аппендится в ПЕР-ПОТОКОВЫЙ
// delta-шард (0 локов, 0 seqlock, 0 cell-хэша — O(десятки ns)); eq_epoch
// (ОДИН bulk JNI/тик) ПЕРЕД chain-build сливает шарды в плоские колонки
// плоскости (O(dirty), один потребитель) — стоимость per-tick = O(dirty),
// не O(population) под глобальным локом. Chain-build остаётся full-pass
// (измеримо бесплатный, сохраняет self-consistency-аргумент одиночного
// прохода). Плоские cell-цепи (keys/head/soa_scan) под eqsnap НЕ
// поддерживаются — java-лестница eqsnap НЕ вызывает mobQuery (fail-closed
// ваниль), mob_query под eqsnap отвечает ERR_RANGE (per-call vanilla, lever
// жив). Потеря/разрыв строки шарда при гонке drain vs straggler-upsert =
// тот же документированный контракт ≤1-тик ghost (доки модуля выше).
/// Max cell-window half-width beyond the ±PAD (self AABB span cap): wider
/// queries return ERR_RANGE (per-call vanilla fallback; oversized selves are
/// gated java-side by the ≤2.0 bounding-radius gate).
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
    /// TASK-427-A2 REBUILD SCRATCH (FIXED capacity, allocated once at plane
    /// init, never re-swapped): the tombstone-reclaim rebuild rehashes the
    /// live chains into THIS buffer and copies back IN PLACE. The fixed
    /// no-dangling-buffer contract for lock-free readers is preserved — a
    /// buffer-swap rebuild (`d.keys = fresh; drop(old)`) would free the old
    /// table under a lock-free reader mid-walk = UAF.
    rkeys: Vec<i64>,
    rhead: Vec<i32>,
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
            rkeys: vec![0i64; CELL_CAP],
            rhead: vec![0i32; CELL_CAP],
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
/// TASK-427-A2 (protocol v2): PER-SHARD seqlock versions (striped by
/// hash(cell_key)). Even = stable, odd = a write touching a cell of this shard
/// is in flight. A writer under WLOCK bumps only the shards of the cells it
/// mutates; a reader brackets each scanned cell against its own shard version
/// with a bounded retry — churn visible to one reader drops ~N_SHARDS-fold.
/// The GLOBAL VERSION above is retained for the ai/sscan/eq epoch-reader
/// contract (ai_window_snapshot / sscan_snapshot / eq_snapshot).
static SVER: [AtomicUsize; N_SHARDS] = [const { AtomicUsize::new(0) }; N_SHARDS];

#[inline]
fn shard_of(k: i64) -> usize {
    (mix64(k as u64) as usize) & (N_SHARDS - 1)
}

/// Writer-side shard bracket begin (odd). Callers hold WLOCK.
#[inline]
fn shard_begin(s: usize) {
    SVER[s].fetch_add(1, Ordering::Release);
}
/// Writer-side shard bracket end (even; Release publishes the data stores).
#[inline]
fn shard_end(s: usize) {
    SVER[s].fetch_add(1, Ordering::Release);
}

/// Reader backoff between per-cell seqlock retries: spin first, yield later
/// (protocol v2 bounded fail-open — no unbounded spinning).
#[inline]
fn backoff(tries: u32) {
    if tries < 4 {
        for _ in 0..(16u32 << tries) {
            std::hint::spin_loop();
        }
    } else {
        std::thread::yield_now();
    }
}

/// TASK-427-A2 (protocol v2): local per-thread scratch for the two-phase
/// query — the scan with all its retries/backoffs writes HERE; the
/// JNI-critical region is only the final memcpy. Grow-only per thread — zero
/// allocations in steady state.
thread_local! {
    static SCRATCH: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

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
/// WLOCK: readers never block writers. (Merged from meganav lineage
/// TASK-412-F: под eqsnap read-view видит состояние плоскости ПОСЛЕ
/// drain_eqsnap_shards текущего тика — тот же документированный контракт
/// ≤1-тик ghost.)
pub(crate) fn ai_window_snapshot() -> Option<(&'static [u8], &'static AtomicUsize)> {
    let d = data()?;
    // d: &'static Soa (published once, never freed) — the flags slice is
    // &'static by construction, no unsafe required.
    let flags: &'static [u8] = d.flags.as_slice();
    Some((flags, &VERSION))
}

/// TASK-406-E (sscan despawn scan): read view for the nearest-player epoch
/// pass — the f64 x/y/z position slices + the seqlock version. The caller
/// (mobs_sscan::sscan_epoch) brackets its scan with the SAME version
/// discipline as `mob_query` (even v1 → scan → even v2, bounded retries); a
/// torn snapshot retries the whole pass, so the column is always a
/// CONSISTENT SoA state (a mob whose id was assigned always has its position
/// written before the version bump — no half-assigned slots). No WLOCK:
/// readers never block writers. Positions of non-alive slots may be stale —
/// such slots are never consulted (dead mobs are not ticked).
pub(crate) fn sscan_snapshot() -> Option<(&'static [f64], &'static [f64], &'static [f64], &'static AtomicUsize)> {
    let d = data()?;
    // d: &'static Soa (published once, never freed) — the f64 slices are
    // &'static by construction, no unsafe required.
    Some((d.x.as_slice(), d.y.as_slice(), d.z.as_slice(), &VERSION))
}

/// TASK-411-C (eqsnap, v2): STRICT-eq gate of the delta-shard mode. Only the
/// exact new flag routes mob_upsert into the per-thread shards; every prior
/// flag keeps bit-in-bit prior behavior (full WLOCK plane mutation).
/// TASK-412-C (eqsnap-v3): меганав-композит cmp412_eqsnapv3 несёт
/// eqsnap-плоскость (STRICT OR: плоскости cmp412_meganav || eqsnap).
fn eqsnap_mode() -> bool {
    static FLAG: OnceLock<String> = OnceLock::new();
    let f = FLAG
        .get_or_init(|| {
            std::env::var("CRUSSTY_LEVER_FLAG")
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .as_str();
    f == "cmp411_eqsnap" || f == "cmp412_eqsnapv3" || f == "cmp414_cvs" || f == "cmp417_bq"
        // TASK-419-A (colpush): колпаш-носитель — mob natives (probe/remove)
        // и read-views живут; colpush_plane_refresh кормит колонки.
        || f == "cmp420_colpush"
        || f == "cmp421_brain" || f == "cmp422_brain2" || f == "cmp423_brain3" || f == "cmp424_mobfeed" || f == "cmp430_inside" || f == "cmp432_inside2" || f == "cmp436_ins4" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2"
        || f == "cmp451_senseins" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2" || f == "cmp453_diet" || f == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        || f == "cmp438_sense" // TASK-444-C: sense family union
        || f == "cmp451_senseins" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2" || f == "cmp453_diet" || f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4" || f == "cmp444_chunk5" || f == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
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
        // TASK-406-E: композит раунда-406 — SoA-плоскость primary push
        // broadphase + источник популяции для sscanEpoch (despawn-scan column).
        || f == "cmp406_sscan"
        || f == "cmp409_multi"
        // TASK-412-F meganav: multi ⊕ navplane+navpool (топ-композиция эры).
        || f == "cmp412_meganav" || f == "cmp414_cvs"
        // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR (плоскости
        // cmp412_meganav || eqsnap-плоскость); eqsnap-режим (DeltaShard
        // upserts, drain O(dirty)) берёт вверх в mob_upsert/лестнице push.
        || f == "cmp412_eqsnapv3" || f == "cmp414_cvs" || f == "cmp417_bq"
        // TASK-419-A (colpush): STRICT OR — колпаш-носитель несёт eqsnap-
        // плоскость (drain шардов пуст, плоские колонки кормит
        // colpush_plane_refresh одним WLOCK/тик).
        || f == "cmp420_colpush"
        || f == "cmp421_brain" || f == "cmp422_brain2" || f == "cmp423_brain3" || f == "cmp424_mobfeed" || f == "cmp430_inside" || f == "cmp432_inside2" || f == "cmp436_ins4" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2"
        || f == "cmp451_senseins" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2" || f == "cmp453_diet" || f == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        || f == "cmp438_sense" // TASK-444-C: sense family union
        || f == "cmp451_senseins" || f == "cmp457_paldelta" || f == "cmp457_eqsnap2" || f == "cmp453_diet" || f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4" || f == "cmp444_chunk5" || f == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        // TASK-410-C (eindexq): K3-пивот R2 — SoA-плоскость = источник
        // популяции для goal-query CSR-снапшота (EntityQueryOps.eqEpoch;
        // sscan-прецедент TASK-406-E).
        || f == "cmp410_eindexq"
        // TASK-411-C (k4soa): K4 — радиус-ремонт населения (gate 2.0 / pad 2)
        // + push-лейн из chain-снапшота (0 per-query JNI).
        || f == "cmp411_k4soa"
        // TASK-411-C (eqsnap, v2): dirty-дельты — upserts в пер-потоковые
        // шарды (0 локов), eq_epoch сливает их одним bulk-JNI/тик (O(dirty)).
        || f == "cmp411_eqsnap"
        // TASK-419-A (colpush): колпаш-носитель — per-entity mobUpsert не
        // вызывается (pushEntities whole-body redirect), плоскость кормится
        // colpush_plane_refresh; read-views sscan/ai/eq сохранены.
        || f == "cmp420_colpush"
}

// ---------------------------------------------------------------------------
// TASK-411-C (eqsnap, v2): per-thread delta shards
// ---------------------------------------------------------------------------

/// Shard row layout: [id, alive, x, y, z, hw, hh] (7 f64; id/alive — точные
/// малые целые, представимы f64 без потерь).
const SHARD_ROW: usize = 7;
/// Fixed per-shard row capacity. Steady-state ~48k upserts/tick across ~5
/// writer threads ⇒ ≤ ~12k rows/thread/tick; 8192 покрывает бурсты ×4+.
/// Overflow ⇒ mob_upsert ERR_RANGE (per-call vanilla, lever stays armed).
const SHARD_CAP: usize = 1 << 13;
/// 16 fixed shards; threads claim one each (sequential, ~5 live threads) —
/// no hashing, no cross-thread contention on a shard.
const SHARD_N: usize = 16;

struct DeltaShard {
    /// Reservation cursor (fetch_add) — writers claim slots.
    reserve: AtomicUsize,
    /// Published watermark: rows [0..published) are complete (Release after
    /// the row write; consumer reads Acquire). Single consumer per tick.
    published: AtomicUsize,
    /// Row storage (fixed, zero-alloc steady state).
    rows: Vec<f64>,
}

impl DeltaShard {
    /// Lock-free append. Returns false on capacity pressure (caller maps to
    /// ERR_RANGE = per-call vanilla fallback, the legacy capacity contract).
    /// Interior mutability through a raw base pointer: `rows` is sized ONCE
    /// at shard construction and never reallocated (fixed-capacity contract),
    /// so writers may mutate through `&self` — the same discipline as the
    /// published-once Soa plane above.
    fn push(&self, id: usize, alive: bool, x: f64, y: f64, z: f64, hw: f64, hh: f64) -> bool {
        let p = self.reserve.fetch_add(1, Ordering::Relaxed);
        if p >= SHARD_CAP {
            return false; // burst overflow — per-call vanilla this upsert
        }
        let b = p * SHARD_ROW;
        // SAFETY: rows sized SHARD_CAP*SHARD_ROW once at init; p < SHARD_CAP
        // checked above; single writer per shard (thread-bound), the drain
        // reads rows < published (Release/Acquire) and never writes rows.
        unsafe {
            let base = self.rows.as_ptr() as *mut f64;
            *base.add(b) = id as f64;
            *base.add(b + 1) = if alive { 1.0 } else { 0.0 };
            *base.add(b + 2) = x;
            *base.add(b + 3) = y;
            *base.add(b + 4) = z;
            *base.add(b + 5) = hw;
            *base.add(b + 6) = hh;
        }
        // Publish AFTER the row write (Release): the consumer's Acquire load
        // of `published` sees every row below the watermark complete.
        self.published.fetch_max(p + 1, Ordering::Release);
        true
    }

    /// Single-consumer drain: apply rows [0..published) to the plane's FLAT
    /// columns ONLY (id → x/y/z/hw/hh + alive bit). NO cell-chain maintenance
    /// under eqsnap (mobQuery unreachable — java ladder fail-closes to
    /// vanilla). Rows for ids ≥ the chain-build `bound` stay in the columns
    /// and link on the NEXT epoch (1-tick delay, ghost contract). Resets the
    /// cursors AFTER processing; a straggler writer racing the reset loses
    /// its row (or re-writes it — idempotent) = the documented ≤1-tick ghost,
    /// bounded by the 8-block java margin.
    /// Returns the number of applied rows.
    fn drain_into(&self, d: &mut Soa) -> usize {
        let n = self.published.load(Ordering::Acquire).min(SHARD_CAP);
        let mut applied = 0usize;
        for p in 0..n {
            let b = p * SHARD_ROW;
            // SAFETY: n ≤ SHARD_CAP and rows sized SHARD_CAP*SHARD_ROW.
            let row = unsafe { self.rows.get_unchecked(b..b + SHARD_ROW) };
            let id = row[0] as usize;
            if id >= IDS_CAP || row[1] != 1.0 {
                continue; // defensive bounds / non-upsert row — skip
            }
            let (x, y, z, hw, hh) = (row[2], row[3], row[4], row[5], row[6]);
            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || !hw.is_finite()
                || !hh.is_finite()
            {
                continue; // torn/garbage row (straggler race) — ghost ≤1 tick
            }
            d.x[id] = x;
            d.y[id] = y;
            d.z[id] = z;
            d.hw[id] = hw;
            d.hh[id] = hh;
            d.flags[id] |= 1;
            applied += 1;
        }
        // Reset AFTER processing (SeqCst): fresh generation next tick.
        self.reserve.store(0, Ordering::SeqCst);
        self.published.store(0, Ordering::SeqCst);
        applied
    }
}

/// Published once, never freed (fixed-capacity contract — Soa precedent).
static SHARDS: OnceLock<Vec<DeltaShard>> = OnceLock::new();
/// Sequential shard claim: each writer thread binds ONE shard for life
/// (region workers are a fixed pool; a dead thread's shard drains empty).
static SHARD_SEQ: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static SHARD_IDX: std::cell::Cell<usize> = const { std::cell::Cell::new(usize::MAX) };
}

fn shards() -> &'static [DeltaShard] {
    SHARDS.get_or_init(|| {
        (0..SHARD_N)
            .map(|_| DeltaShard {
                reserve: AtomicUsize::new(0),
                published: AtomicUsize::new(0),
                rows: vec![0.0; SHARD_CAP * SHARD_ROW],
            })
            .collect()
    })
}

#[inline]
fn thread_shard() -> &'static DeltaShard {
    SHARD_IDX.with(|c| {
        let mut i = c.get();
        if i >= SHARD_N {
            i = SHARD_SEQ.fetch_add(1, Ordering::Relaxed) % SHARD_N;
            c.set(i);
        }
        // SAFETY: shards() is a fixed Vec<DeltaShard> of len SHARD_N.
        unsafe { shards().get_unchecked(i) }
    })
}

/// TASK-411-C (eqsnap): ONE bulk drain per tick, called from eq_epoch
/// (entity_query) BEFORE the chain build. Applies every shard's published
/// delta rows to the flat columns. Single consumer (eq_epoch runs under the
/// java EPOCH_LOCK); writers never take locks. Returns total applied rows
/// (diagnostics/marker).
pub(crate) fn drain_eqsnap_shards() -> usize {
    if !eqsnap_mode() {
        return 0; // legacy flags: shards never written — no-op
    }
    let _g = WLOCK.lock().unwrap_or_else(|p| p.into_inner());
    ensure_plane();
    let d = data_mut();
    let mut total = 0usize;
    for s in shards() {
        total += s.drain_into(d);
    }
    total
}

/// TASK-419-A (colpush): ОДИН bulk WLOCK-рефреш плоских колонок плоскости из
/// java-строк колпаша (COL_D/COL_I — персистентные массивы ColpushOps,
/// записанные per-entity в aiStep БЕЗ JNI). Обновляются ТОЛЬКО строки с
/// fresh == want_tick (затиканные СУЩНОСТИ этого тика) — невиденные строки
/// сохраняют прежнее состояние (тот же контракт, что у eqsnap-drain: нетик
/// = не-дельта; НЕ-очистка alive у дальних мобов = паритет sscan-despawn).
/// hw = max(hx, hz) — контракт суперсета плоскости (x/z полуэкстенты).
/// Cell-цепи поддерживаются soa_upsert'ом (insert-or-move), зеркало
/// mirror_mode() под колпашем инертно (cmp402_comp-флаги не активны).
/// Один WLOCK + seqlock-брэкет — читатели sscan/ai/eq видят КОНСИСТЕНТНОЕ
/// SoA-состояние; вызывающий (colpush_tick, main-поток ДО GO-барьера) не
/// конкурирует с ai/eq-эпохами воркеров (после GO). Returns refreshed rows.
pub(crate) fn colpush_plane_refresh(n: usize, d: &[f64], i: &[i32], want_tick: i32) -> usize {
    if !lever_mode() || n == 0 {
        return 0;
    }
    if d.len() < n * crate::colpush::ROW_D || i.len() < n * crate::colpush::ROW_I {
        return 0; // структурный дрейф буферов — java дизармится отдельно
    }
    let _g = WLOCK.lock().unwrap_or_else(|p| p.into_inner());
    ensure_plane();
    let v = VERSION.fetch_add(1, Ordering::AcqRel); // → odd
    debug_assert!(v % 2 == 0);
    let plane = data_mut();
    let mut rows = 0usize;
    for id in 0..n {
        let flags = i[id * crate::colpush::ROW_I + 1];
        let fresh = i[id * crate::colpush::ROW_I + 2];
        if fresh != want_tick || flags & crate::colpush::FLAG_INCLUDE == 0 {
            continue;
        }
        let b = id * crate::colpush::ROW_D;
        let (cx, cy, cz, hx, hz, hh) = (d[b], d[b + 1], d[b + 2], d[b + 3], d[b + 4], d[b + 5]);
        if !(cx.is_finite() && cy.is_finite() && cz.is_finite())
            || !(hx.is_finite() && hz.is_finite() && hh.is_finite())
        {
            continue;
        }
        let lid = i[id * crate::colpush::ROW_I];
        let k = cell_key(
            lid,
            cx.floor() as i32,
            cy.floor() as i32,
            cz.floor() as i32,
        );
        let hw = if hx > hz { hx } else { hz };
        if soa_upsert(plane, id, k, cx, cy, cz, hw, hh) == 0 {
            rows += 1;
        }
    }
    VERSION.fetch_add(1, Ordering::AcqRel); // → even
    rows
}

/// TASK-410-C (eindexq): read view for the goal-query CSR epoch pass —
/// x/y/z/hw/hh f64 slices + alive flags + the seqlock version. The caller
/// (entity_query::eq_epoch) brackets its scan with the SAME version
/// discipline as `mob_query` (even v1 -> scan -> even v2, bounded retries);
/// a torn snapshot retries the whole pass, so the published CSR is always a
/// CONSISTENT SoA state. No WLOCK: readers never block writers. Slices are
/// &'static by construction (Soa published once, never freed).
pub(crate) fn eq_snapshot() -> Option<(
    &'static [f64],
    &'static [f64],
    &'static [f64],
    &'static [f64],
    &'static [f64],
    &'static [u8],
    &'static AtomicUsize,
)> {
    let d = data()?;
    Some((
        d.x.as_slice(),
        d.y.as_slice(),
        d.z.as_slice(),
        d.hw.as_slice(),
        d.hh.as_slice(),
        d.flags.as_slice(),
        &VERSION,
    ))
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
    let mut probes = 0usize;
    loop {
        let cur = unsafe { *keys.get_unchecked(s) };
        if cur == k {
            return Ok(s);
        }
        if cur == 0 {
            return Err(s);
        }
        probes += 1;
        if probes > PROBE_CAP {
            // TASK-427-A2 (protocol v2): a full/degenerate table can no longer
            // produce an infinite walk — the caller treats the cap as pressure
            // (tombstone-reclaim rebuild or ERR_RANGE fail-open).
            return Err(PROBE_FAIL);
        }
        s = (s + 1) & mask;
    }
}

/// TASK-427-A2 (protocol v2): ONE-SHOT tombstone-reclaim rebuild — rehash
/// every LIVE key (non-empty chain) into the FIXED rebuild scratch and copy
/// back IN PLACE, dropping tombstones. Self-contained version bracket: the
/// caller holds the WLOCK (single writer) and this function bumps EVERY shard
/// version odd → mutate → even, so any in-flight reader of ANY cell detects
/// the rebuild and retries (bounded). The in-place copy-back (no buffer swap,
/// no allocation) preserves the fixed-capacity no-dangling-buffer contract
/// for the lock-free readers. Returns false on pathological pressure (caller
/// fail-opens with ERR_RANGE — no panic path).
fn rebuild_tables(d: &mut Soa) -> bool {
    for s in 0..N_SHARDS {
        shard_begin(s);
    }
    let ok = rebuild_tables_inner(d);
    for s in 0..N_SHARDS {
        shard_end(s);
    }
    ok
}

fn rebuild_tables_inner(d: &mut Soa) -> bool {
    // Give up (fail-open) rather than build a table whose probe walks could
    // exceed the probe cap.
    if d.used + PROBE_CAP >= CELL_CAP {
        return false;
    }
    // Rehash into the FIXED scratch: zero it, place every live chain, copy
    // back. No allocation, no buffer identity change — readers can hold no
    // reference that can dangle.
    d.rkeys.fill(0);
    d.rhead.fill(0);
    let mut used = 0usize;
    for s in 0..CELL_CAP {
        let k = d.keys[s];
        if k == 0 || d.head[s] == 0 {
            continue; // never-used slot or tombstone (chain already empty)
        }
        // Fresh table: no tombstones, load < 1 ⇒ a zero slot always exists.
        if let Err(t) = find_slot(&d.rkeys, k) {
            if t == PROBE_FAIL {
                return false; // scratch untouched — the live table is intact
            }
            d.rkeys[t] = k;
            d.rhead[t] = d.head[s];
            used += 1;
        }
    }
    d.keys.copy_from_slice(&d.rkeys);
    d.head.copy_from_slice(&d.rhead);
    d.used = used;
    true
}

/// Unlink `id` from its stored cell chain. The key slot is kept (tombstone).
/// TASK-427-A2 (protocol v2): a probe-cap hit triggers one tombstone-reclaim
/// rebuild + re-find (the key MUST exist — cell[id] != 0 — so exhaustion here
/// means a pathologically tombstoned table, not a dangling chain).
fn unlink(d: &mut Soa, id: usize) -> Result<(), ()> {
    let k = d.cell[id];
    if k == 0 {
        return Err(());
    }
    let slot = match find_slot(&d.keys, k) {
        Ok(s) => s,
        Err(PROBE_FAIL) => {
            if !rebuild_tables(d) {
                return Err(());
            }
            match find_slot(&d.keys, k) {
                Ok(s) => s,
                Err(_) => return Err(()),
            }
        }
        Err(_) => return Err(()),
    };
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

/// Writer-side insert-or-move + field refresh. Caller holds WLOCK (and the
/// legacy GLOBAL VERSION odd bracket for the ai/sscan/eq epoch contract).
/// TASK-427-A2 (protocol v2): THIS function owns the per-shard brackets —
/// same-cell refresh brackets shard(cell); a move brackets the OLD and NEW
/// shards TOGETHER as one atomic transition (unlink + link + fields publish
/// inside a single combined odd-window — a reader of either cell retries into
/// the post-state, so the id is visible in EXACTLY one cell at every
/// validated snapshot). All Err paths leave the plane CONSISTENT (the id
/// either fully linked or fully unlinked). Capacity pressure / probe-cap hit
/// triggers the ONE-SHOT tombstone-reclaim rebuild (self-bracketed on ALL
/// shards); failure = ERR_RANGE fail-open.
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
        // The reader reads these fields under THIS cell's shard bracket —
        // bump it around the writes (protocol v2).
        let s = shard_of(k);
        shard_begin(s);
        d.x[id] = x;
        d.y[id] = y;
        d.z[id] = z;
        d.hw[id] = hw;
        d.hh[id] = hh;
        d.flags[id] |= 1;
        shard_end(s);
        return 0;
    }
    // Pre-resolve the target slot BEFORE unlinking (no partial state on
    // capacity pressure).
    let slot = loop {
        match find_slot(&d.keys, k) {
            Ok(s) => break s,
            Err(PROBE_FAIL) => {
                if !rebuild_tables(d) {
                    return ERR_RANGE; // pathological pressure — per-call vanilla fallback
                }
            }
            Err(s) => {
                if d.used >= CELL_CAP - 1 {
                    if !rebuild_tables(d) {
                        return ERR_RANGE; // table pressure — per-call vanilla fallback
                    }
                    continue;
                }
                d.keys[s] = k;
                d.used += 1;
                break s;
            }
        }
    };
    // Move / first insert under a COMBINED old+new shard bracket (deduped).
    let so = if d.cell[id] != 0 { shard_of(d.cell[id]) } else { usize::MAX };
    let sn = shard_of(k);
    if so != usize::MAX {
        shard_begin(so);
    }
    if so != sn {
        shard_begin(sn);
    }
    let r = if so != usize::MAX {
        unlink(d, id)
    } else {
        Ok(())
    };
    if r.is_ok() {
        d.x[id] = x;
        d.y[id] = y;
        d.z[id] = z;
        d.hw[id] = hw;
        d.hh[id] = hh;
        d.flags[id] |= 1;
        d.next[id] = d.head[slot];
        d.head[slot] = (id as i32) + 1;
        d.cell[id] = k;
    }
    if so != sn {
        shard_end(sn);
    }
    if so != usize::MAX {
        shard_end(so);
    }
    if r.is_err() {
        return ERR_STRUCT; // dangling chain — corruption
    }
    0
}

/// One CELL of the reader scan: padded-window key → chain walk → flat-field
/// coarse AABB prune. Pure (no version brackets) — the test oracle drives it
/// directly; the native reader wraps it in per-shard seqlock brackets.
/// TASK-427-A2 (protocol v2).
fn scan_cell(
    d: &Soa,
    dst: &mut [i32],
    n: &mut i32,
    cap: i32,
    k: i64,
    q: (f64, f64, f64, f64, f64, f64),
) -> CellScan {
    let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
    let Ok(slot) = find_slot(&d.keys, k) else {
        return CellScan::Ok; // absent key or probe-cap hit → no candidates here
    };
    let mut cur = unsafe { *d.head.get_unchecked(slot) };
    while cur != 0 {
        let id = (cur - 1) as usize;
        if id >= IDS_CAP {
            // The caller's version bracket decides torn vs corrupt.
            return CellScan::Struct;
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
            if *n >= cap {
                return CellScan::Overflow;
            }
            dst[*n as usize] = id as i32;
            *n += 1;
        }
        cur = unsafe { *d.next.get_unchecked(id) };
    }
    CellScan::Ok
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellScan {
    Ok,
    Overflow,
    Struct,
}

/// Test-oracle scan (no brackets): padded cell window → per-cell chain walk
/// + coarse prune. Returns the candidate count, -(cap) on overflow,
/// ERR_STRUCT on a stable anomaly. (Base soa_scan refactored onto scan_cell —
/// same discipline as items_index / mobs_grid.)
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
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                match scan_cell(d, dst, &mut n, cap, k, q) {
                    CellScan::Ok => {}
                    CellScan::Overflow => return -cap,
                    CellScan::Struct => return ERR_STRUCT,
                }
            }
        }
    }
    n
}

/// TASK-427-A2 (protocol v2 READER): per-CELL seqlock brackets against the
/// cell's shard version, bounded retries (≤8, spin→yield backoff) with n0
/// rollback, exhaustion → ERR_RANGE (java per-call fallback mirror-grid →
/// vanillaFill; NO plane disarm). Writes into the caller's LOCAL scratch —
/// no JNI-critical region is held across any retry/backoff.
#[allow(clippy::too_many_arguments)]
fn sharded_scan(
    d: &Soa,
    dst: &mut [i32],
    cap: i32,
    lid: i32,
    (cx0, cx1): (i32, i32),
    (cy0, cy1): (i32, i32),
    (cz0, cz1): (i32, i32),
    q: (f64, f64, f64, f64, f64, f64),
) -> i32 {
    let mut n: i32 = 0;
    for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                let s = shard_of(k);
                let n0 = n;
                let mut tries: u32 = 0;
                loop {
                    tries += 1;
                    if tries > SHARD_RETRIES {
                        // Bounded fail-open: per-call fallback (mirror grid →
                        // vanilla), the plane STAYS ARMED.
                        return ERR_RANGE;
                    }
                    let v1 = SVER[s].load(Ordering::Acquire);
                    if v1 & 1 == 1 {
                        backoff(tries);
                        continue;
                    }
                    match scan_cell(d, dst, &mut n, cap, k, q) {
                        CellScan::Overflow => return -cap,
                        CellScan::Struct => {
                            let v2 = SVER[s].load(Ordering::Acquire);
                            if v2 != v1 {
                                n = n0; // torn view — retry this cell
                                backoff(tries);
                                continue;
                            }
                            return ERR_STRUCT; // stable anomaly = real corruption
                        }
                        CellScan::Ok => {}
                    }
                    let v2 = SVER[s].load(Ordering::Acquire);
                    if v2 != v1 {
                        n = n0; // torn snapshot of this cell — retry
                        backoff(tries);
                        continue;
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
/// (hw = half of max(x/z)-extent, hh = half height; java gates both ≤ 2.0 —
/// TASK-411-C k4soa, was ≤ 1.0).
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
    // TASK-411-C (eqsnap, v2): dirty-delta path — the per-entity upsert lands
    // in the caller's per-thread shard (0 locks, 0 seqlock, 0 cell hash); the
    // plane is refreshed ONCE per tick by drain_eqsnap_shards() inside
    // eq_epoch. cl1 evidence: per-entity WLOCK+JNI mutation = 24.9% CPU.
    // lid is unused here (eq chains are level-agnostic — java filters
    // other.level() != level exactly); legacy paths keep the keyed plane.
    if eqsnap_mode() {
        return if thread_shard().push(id as usize, true, x, y, z, hw, hh) {
            0
        } else {
            ERR_RANGE // shard burst overflow — per-call vanilla, lever armed
        };
    }
    let k = cell_key(lid, x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let _g = WLOCK.lock().unwrap_or_else(|p| p.into_inner());
    ensure_plane();
    let v = VERSION.fetch_add(1, Ordering::AcqRel); // → odd (global; ai/sscan/eq contract)
    debug_assert!(v % 2 == 0);
    // TASK-427-A2 (protocol v2): soa_upsert self-brackets the touched shards
    // (same-cell refresh / combined old+new move bracket); probe-cap/pressure
    // triggers the one-shot tombstone-reclaim rebuild inside.
    let rc = soa_upsert(data_mut(), id as usize, k, x, y, z, hw, hh);
    VERSION.fetch_add(1, Ordering::AcqRel); // → even
    drop(_g);
    // TASK-427-A2 (protocol v2): mirror the SAME (id, cell-key) mutation into
    // the sharded grid OUTSIDE the SoA writer lock (no lock-group nesting; the
    // mirror serializes on its OWN per-shard writer Mutexes). A grid
    // structural failure is ISOLATED: mark_broken() stops mirroring, the SoA
    // rc stays authoritative.
    if rc != ERR_STRUCT && mirror_mode() {
        if crate::mobs_grid::mirror_upsert(id as usize, k) < 0 {
            crate::mobs_grid::mark_broken();
        }
    }
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
    let v = VERSION.fetch_add(1, Ordering::AcqRel); // → odd (global; ai/sscan/eq contract)
    debug_assert!(v % 2 == 0);
    let rc = if d.cell[id as usize] != 0 {
        // TASK-427-A2 (protocol v2): unlink + alive-flag clear under the id's
        // cell shard bracket (readers validate the flags column against the
        // SAME shard version — no unbracketed writer stores).
        let s = shard_of(d.cell[id as usize]);
        shard_begin(s);
        d.flags[id as usize] &= !1;
        let r = unlink(d, id as usize);
        shard_end(s);
        if r.is_err() {
            ERR_STRUCT
        } else {
            0
        }
    } else {
        // No chain to unlink (defensive path: alive flag without a link can
        // not be produced by upsert) — clear the flag unbracketed.
        d.flags[id as usize] &= !1;
        0
    };
    VERSION.fetch_add(1, Ordering::AcqRel); // → even
    drop(_g);
    // TASK-427-A2 (protocol v2): mirror OUTSIDE the SoA writer lock (no
    // lock-group nesting; failures isolated — see mob_upsert).
    if rc != ERR_STRUCT && mirror_mode() {
        if crate::mobs_grid::mirror_remove(id as usize) < 0 {
            crate::mobs_grid::mark_broken();
        }
    }
    rc
}

/// Query candidate ids for the AABB [qx0..qx1]×[qy0..qy1]×[qz0..qz1] with the
/// ±PAD(2) center-window pad (java guarantees all members have half-extent
/// ≤ 2.0 — k4soa gate). Protocol v2 TWO-PHASE shape: PHASE 1 scans + retries
/// + backoff into the thread-local scratch (per-cell shard seqlock brackets,
/// bounded — NO JNI-critical region held across any of it); PHASE 2 pins
/// `out` ONLY for the final memcpy of the ready result. Returns the count,
/// -(out_cap) on overflow (caller grows + retries), ERR_RANGE on per-cell
/// retry exhaustion / absurd widths (java per-call fallback mirror-grid →
/// vanillaFill; the plane stays armed), ERR_STRUCT on corruption/JNI trouble.
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
    // TASK-411-C (eqsnap): cell chains are NOT maintained under the delta-
    // shard mode (upserts land in shards; the drain refreshes flat columns
    // only) — a chain scan would be UNSOUND. No java path reaches this
    // native under eqsnap (pushables ladder fail-closes to vanillaFill);
    // ERR_RANGE keeps the lever ARMED if a foreign caller ever hits it.
    if eqsnap_mode() {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    let cap = cap as i32;

    // TASK-411-C (k4soa): PAD-cell center window (was ±1 for the 1.0 radius
    // gate; the 2.0 gate needs ±2 — see PAD soundness note).
    let cx0 = qx0.floor() as i32 - PAD;
    let cx1 = qx1.floor() as i32 + PAD;
    let cy0 = qy0.floor() as i32 - PAD;
    let cy1 = qy1.floor() as i32 + PAD;
    let cz0 = qz0.floor() as i32 - PAD;
    let cz1 = qz1.floor() as i32 + PAD;
    if (cx1 - cx0) > MAX_SPAN || (cy1 - cy0) > MAX_SPAN || (cz1 - cz0) > MAX_SPAN {
        return ERR_RANGE;
    }

    let Some(d) = data() else {
        return 0; // empty universe — no upserts ever, no candidates
    };

    // TASK-427-A2 (protocol v2 TWO-PHASE shape): PHASE 1 scans + retries +
    // backoff into the thread-local scratch (per-cell shard seqlock brackets,
    // bounded — NO JNI-critical region held across any of it: the GC-locker
    // convoy of the v1 whole-scan-critical shape is impossible by
    // construction). PHASE 2 pins `out` ONLY for the final memcpy of the
    // READY result.
    let q = (qx0, qy0, qz0, qx1, qy1, qz1);
    let n: i32 = SCRATCH.with(|cell| {
        let mut sc = cell.borrow_mut();
        if sc.len() < cap as usize {
            sc.resize(cap as usize, 0);
        }
        let dst: &mut [i32] = &mut sc[..cap as usize];
        sharded_scan(d, dst, cap, lid, (cx0, cx1), (cy0, cy1), (cz0, cz1), q)
    });
    if n < 0 {
        return n; // ERR_RANGE fail-open / -(cap) overflow / ERR_STRUCT corruption
    }

    // PHASE 2: the ONLY JNI-critical region = memcpy of the READY result.
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    {
        let src = SCRATCH.with(|cell| {
            let sc = cell.borrow();
            // SAFETY: n ≥ 0 and n ≤ cap (the scan never exceeds cap).
            let base = unsafe { std::slice::from_raw_parts(sc.as_ptr(), n as usize) };
            std::ptr::copy_nonoverlapping(base.as_ptr(), pinned as *mut jni::jint, n as usize);
            n
        });
        debug_assert_eq!(src, n);
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

    // TASK-411-C (k4soa): test window mirrors mob_query — MUST track PAD.
    fn query(d: &Soa, lid: i32, q: (f64, f64, f64, f64, f64, f64)) -> Vec<usize> {
        let (qx0, qy0, qz0, qx1, qy1, qz1) = q;
        let cx0 = qx0.floor() as i32 - PAD;
        let cx1 = qx1.floor() as i32 + PAD;
        let cy0 = qy0.floor() as i32 - PAD;
        let cy1 = qy1.floor() as i32 + PAD;
        let cz0 = qz0.floor() as i32 - PAD;
        let cz1 = qz1.floor() as i32 + PAD;
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

    /// TASK-411-C (k4soa): pad-2 soundness oracle — mobs with bench radii
    /// (camel 1.1875 / iron_golem 1.35 / gate-max 2.0) centered OUTSIDE the
    /// query box must still be found when their AABB intersects it (the exact
    /// case the ±1 pad missed: center cell = floor(qx0) − 2).
    #[test]
    fn k4soa_large_radius_superset() {
        let mut d = Soa::new();
        let radii = [1.1875f64, 1.35, 2.0];
        let mut id = 9000usize;
        for (i, r) in radii.iter().enumerate() {
            let r = *r;
            // Center sits (r − 0.125) blocks left of the integer box edge
            // qx0 = 10.0 with true half-extent ex = ez = r ⇒ AABB reaches
            // x = 10.125 > qx0 (genuine intersection), while the CENTER cell
            // = floor(10 − r + 0.125) = 8 = floor(qx0) − 2 for all three r
            // (pad-1 window starts at cell 9 and misses it; pad-2 covers).
            let m = Mob {
                id,
                lid: 11,
                x: 10.0 - (r - 0.125),
                y: 64.0,
                z: 10.0 + i as f64 * 0.25,
                ex: r,
                ey: 0.4,
                ez: r,
                hw: r,
                hh: r,
            };
            assert_eq!(m.upsert(&mut d), 0);
            let q = (10.0, 63.5, m.z - 0.5, 11.0, 64.5, m.z + 0.5);
            assert!(
                exact_hits(&m, q),
                "model must genuinely intersect the query (r={r})"
            );
            assert!(
                query(&d, 11, q).contains(&id),
                "pad-2 window must surface gate-max radius candidate r={r}"
            );
            id += 1;
        }
        // Mirror case above the box (y axis): center a full radius above the
        // box top edge — the ±2 y-window must reach it (live-bb intersects).
        let m = Mob { id, lid: 11, x: 50.5, y: 64.0 + 1.35, z: 50.5, ex: 0.4, ey: 1.35, ez: 0.4, hw: 1.35, hh: 1.35 };
        assert_eq!(m.upsert(&mut d), 0);
        let q = (49.5, 64.0 + 0.5 - 1e-9, 49.5, 51.5, 64.0 + 1.5 - 1e-9, 51.5);
        assert!(exact_hits(&m, q) && query(&d, 11, q).contains(&id));
        // Exact pad-2 boundary with a NON-integer box edge: center x = 7.6
        // (cell 7), hw = hh = 2.0, true ex = 1.95 — AABB reaches 9.55 >
        // qx0 = 9.5 − 1e-9; floor(qx0) − 2 = 7 → only the ±2 window reaches
        // cell 7 (the ±1 window of the old gate missed it).
        let b = Mob { id: id + 1, lid: 11, x: 7.6, y: 64.0, z: 50.5, ex: 1.95, ey: 1.0, ez: 0.5, hw: 2.0, hh: 2.0 };
        assert_eq!(b.upsert(&mut d), 0);
        let q2 = (9.5 - 1e-9, 62.0, 49.5, 11.0, 66.0, 51.5);
        assert!(exact_hits(&b, q2));
        assert!(
            query(&d, 11, q2).contains(&(id + 1)),
            "pad-2 boundary regression: center cell == floor(qx0)-2 must be scanned"
        );
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

    /// TASK-411-C (eqsnap, v2): delta-shard drain contract — published rows
    /// land in the flat columns (O(dirty)), the cursors reset after the
    /// drain, a second drain is empty, non-finite/out-of-range rows are
    /// skipped (ghost contract), and the chain structure stays untouched
    /// (cell==0 under eqsnap — mobQuery is unreachable by ladder design).
    #[test]
    fn eqsnap_shard_drain_applies_dirty_rows() {
        let shard = DeltaShard {
            reserve: AtomicUsize::new(0),
            published: AtomicUsize::new(0),
            rows: vec![0.0; SHARD_CAP * SHARD_ROW],
        };
        let mut d = Soa::new();
        // Three dirty rows for one id sequence: add + move + (finite guard).
        assert!(shard.push(11, true, 1.5, 64.0, -2.5, 0.3, 0.9));
        assert!(shard.push(12, true, 10.5, 64.0, 3.5, 1.1875, 1.3));
        assert!(shard.push(11, true, 2.5, 64.0, -2.5, 0.3, 0.9)); // move
        assert!(shard.push(IDS_CAP + 5, true, 0.0, 0.0, 0.0, 0.1, 0.1)); // OOR id
        assert!(shard.push(13, true, f64::NAN, 0.0, 0.0, 0.1, 0.1)); // torn guard
        let applied = shard.drain_into(&mut d);
        assert_eq!(applied, 3, "3 valid rows applied (11, 12, 11-move)");
        assert_eq!(d.flags[11] & 1, 1);
        assert_eq!(d.flags[12] & 1, 1);
        assert_eq!(d.flags[13] & 1, 0, "non-finite row skipped");
        // Out-of-range id: skipped BEFORE any column access (no panic).
        assert_eq!(applied, 3);
        // The MOVE landed: last write wins, flat columns carry the newest row.
        assert_eq!(d.x[11], 2.5);
        assert_eq!(d.hw[12], 1.1875);
        // Chains untouched under eqsnap (mobQuery ladder-disabled).
        assert_eq!(d.cell[11], 0);
        assert_eq!(d.cell[12], 0);
        // Generation reset: second drain is empty (fresh tick).
        assert_eq!(shard.drain_into(&mut d), 0);
        // Capacity pressure: SHARD_CAP-th push fails → ERR_RANGE contract.
        let full = DeltaShard {
            reserve: AtomicUsize::new(SHARD_CAP),
            published: AtomicUsize::new(0),
            rows: vec![0.0; SHARD_CAP * SHARD_ROW],
        };
        assert!(!full.push(1, true, 0.0, 0.0, 0.0, 0.1, 0.1));
    }

    /// TASK-427-A2 (protocol v2): the sharded per-cell-bracket reader must be
    /// parity-identical to the unbracketed oracle scan on a quiescent plane
    /// (same candidate SET — order is chain/bucket order, identical by
    /// construction here).
    #[test]
    fn sharded_scan_parity_vs_oracle() {
        let mut d = Soa::new();
        let mut rng: u64 = 0x0DDB_1A55_EEDF_BEEF;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        for id in 0..400usize {
            let (x, y, z) = (
                (next() % 48) as f64 - 24.0 + (next() % 1000) as f64 / 1000.0,
                (next() % 12) as f64 + 60.0,
                (next() % 48) as f64 - 24.0 + (next() % 1000) as f64 / 1000.0,
            );
            let m = Mob { id, lid: 2, x, y, z, ex: 0.2, ey: 0.5, ez: 0.2, hw: 0.25, hh: 0.5 };
            assert_eq!(m.upsert(&mut d), 0);
        }
        for _ in 0..60 {
            let (qx, qy, qz) = (
                (next() % 48) as f64 - 24.0,
                (next() % 12) as f64 + 60.0,
                (next() % 48) as f64 - 24.0,
            );
            let q = (qx - 0.4, qy - 0.9, qz - 0.4, qx + 0.4, qy + 0.9, qz + 0.4);
            let mut o1 = [0i32; 4096];
            let mut o2 = [0i32; 4096];
            let n1 = soa_scan(&d, &mut o1, 4096, 2, (-26, 26), (58, 72), (-26, 26), q);
            let n2 = sharded_scan(&d, &mut o2, 4096, 2, (-26, 26), (58, 72), (-26, 26), q);
            assert_eq!(n1, n2, "sharded reader diverged from the oracle");
            assert!(n1 >= 0);
            let mut a: Vec<i32> = o1[..n1 as usize].to_vec();
            let mut b: Vec<i32> = o2[..n2 as usize].to_vec();
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b);
        }
    }

    /// TASK-427-A2 (protocol v2): tombstone-reclaim rebuild — live chains
    /// survive, tombstone slots are reclaimed (used == live count), the
    /// walkability of every live chain is preserved and churn continues
    /// cleanly after a rebuild. Direct regression for the round-mf427l2
    /// TOP-1 leaf (soa_upsert 12.2% self = tombstone degeneration).
    #[test]
    fn tombstone_reclaim_rebuild() {
        let mut d = Soa::new();
        // 600 live ids, then remove every second one → ~300 tombstone keys.
        for i in 0..600i32 {
            let m = Mob { id: i as usize, lid: 11, x: (i * 3) as f64 + 0.5, y: (i * 5) as f64 + 0.5, z: (-i * 7) as f64 + 0.5, ex: 0.1, ey: 0.1, ez: 0.1, hw: 0.1, hh: 0.1 };
            assert_eq!(m.upsert(&mut d), 0);
        }
        for i in (0..600i32).step_by(2) {
            let m = Mob { id: i as usize, lid: 11, x: (i * 3) as f64 + 0.5, y: (i * 5) as f64 + 0.5, z: (-i * 7) as f64 + 0.5, ex: 0.1, ey: 0.1, ez: 0.1, hw: 0.1, hh: 0.1 };
            assert_eq!(m.remove(&mut d), 0);
        }
        let heads_before: Vec<i32> = (0..CELL_CAP)
            .filter(|&s| d.keys[s] != 0 && d.head[s] != 0)
            .map(|s| d.head[s])
            .collect();
        assert!(rebuild_tables(&mut d), "rebuild must succeed on a sane table");
        let heads_after: Vec<i32> = (0..CELL_CAP)
            .filter(|&s| d.keys[s] != 0 && d.head[s] != 0)
            .map(|s| d.head[s])
            .collect();
        assert_eq!(heads_before.len(), 300, "expected ~300 live keys pre-rebuild");
        assert_eq!(heads_before, heads_after, "live chains must survive the rebuild");
        assert_eq!(d.used, 300, "tombstones reclaimed");
        // Every live chain still walkable and every live candidate findable:
        // a query at the cell of odd id 1 (3.5, 5.5, -6.5) must return it and
        // nothing else (removed even neighbors are gone).
        let q = (3.0, 5.0, -7.0, 4.0, 6.0, -6.0);
        let got = query(&d, 11, q);
        assert_eq!(got, vec![1usize], "exactly the live odd id survives (q {q:?})");
        // Churn continues cleanly after a rebuild (probe chains intact).
        for round in 0..2000i32 {
            let i = (round % 600) as i32;
            let m = Mob { id: i as usize, lid: 11, x: (i * 3) as f64 + 0.5, y: (i * 5) as f64 + 0.5, z: (-i * 7) as f64 + 0.5, ex: 0.1, ey: 0.1, ez: 0.1, hw: 0.1, hh: 0.1 };
            if i % 2 == 0 {
                assert_eq!(m.upsert(&mut d), 0, "round {round}");
            } else {
                assert_eq!(m.remove(&mut d), 0, "round {round}");
            }
        }
    }
}
