//! ENT-BP v2 core: the entity broadphase mirror (task161, S7-92).
//!
//! Design doc: docs/RESEARCH_ENTBP_2026-09-17.md. Measured basis (run#10 CI,
//! 224,660 samples): the moonrise `ChunkEntitySlices$EntityCollectionBySection
//! .getEntities` loop owns 3.79% stack presence; the replaceable box-scan
//! machinery is ~2.27% of total tick CPU. This module is the RUST CORE of the
//! mirror: a loose 16³-cell grid + SoA slot store + generation-stamp dedup +
//! AVX2 4-wide batch box tests (scalar fallback, 0 new deps). The JNI surface
//! and the whole-body patch wiring are the NEXT tick (recon leg first).
//!
//! Superset contract (pre-registered G1): the mirror never misses an entity the
//! kernel's box test would find. Enforced by construction here:
//!   * boxes are inserted into EVERY overlapped cell (loose grid),
//!   * oversized/NaN/inverted boxes go to a `wildcard` list scanned by every
//!     query (conservative superset),
//!   * the box-overlap predicate is BIT-IDENTICAL to the kernel's
//!     `AABB.intersects` (inclusive boundaries, same comparison order), so the
//!     SIMD path and the scalar path and the kernel agree on every comparison.
//! Correctness against live kernel state = full write-path coverage (§4 of the
//! design doc) + the CI shadow-diff leg; this core is verified against a
//! linear-scan oracle under randomized sequences (property tests below).
//!
//! Fast-path discipline (pre-registered G3): `query_*` and move-upserts of
//! existing entities allocate NOTHING (caller-owned output buffer, pre-sized
//! arrays, generation stamps instead of clearing). Tests assert this with a
//! counting global allocator.

use std::collections::HashMap;

/// Cell edge in blocks — mirrors the kernel's 16-block section grid so the JNI
/// wiring can map section coords without re-bucketing.
const CELL: f64 = 16.0;

/// Boxes spanning more cells than this go to the wildcard list (always scanned
/// by every query). 3x3x3 = 27 covers every vanilla entity box generously.
const MAX_CELLS_PER_AXIS: i32 = 3;

#[inline]
fn cell_of(v: f64) -> i32 {
    // floor division by CELL — matches SectionPos.blockToSectionCoord semantics
    // (block >> 4) for all inputs including negatives.
    (v / CELL).floor() as i32
}

#[inline]
fn pack(cx: i32, cy: i32, cz: i32) -> i64 {
    // own 21-bit-per-axis signed packing (kernel keys are mapped to this at the
    // JNI boundary — the core owns its namespace)
    let m = 0x1F_FFFF_i64;
    (((cx as i64 & m) << 42) | ((cy as i64 & m) << 21) | (cz as i64 & m)) as i64
}

/// Inclusive box-overlap predicate, BIT-IDENTICAL to the kernel's
/// `AABB.intersects` comparison set (other.max >= this.min && other.min <=
/// this.max per axis, same order). NaN compares false on both sides — same
/// semantics as the kernel's double comparisons.
#[inline]
fn intersects(m: &[f64; 6], q: &[f64; 6]) -> bool {
    m[3] >= q[0] && m[0] <= q[3] && m[4] >= q[1] && m[1] <= q[4] && m[5] >= q[2] && m[2] <= q[5]
}

/// One loose-grid cell: the slot indices whose boxes overlap this cell.
#[derive(Default)]
struct Cell {
    slots: Vec<u32>,
}

/// The mirror. See module docs. `Default` is unused; construct with `new`.
pub struct EntityMirror {
    // SoA slot store
    x0: Vec<f64>,
    y0: Vec<f64>,
    z0: Vec<f64>,
    x1: Vec<f64>,
    y1: Vec<f64>,
    z1: Vec<f64>,
    ids: Vec<u64>,
    occupied: Vec<bool>,
    // per-slot overlapped-cell span (for cheap same-span move detection)
    span: Vec<[i32; 6]>,
    free: Vec<u32>,
    live: u32,

    // loose grid
    cells: HashMap<i64, Cell>,
    // oversized/NaN boxes: always scanned by every query (conservative superset)
    wildcard: Vec<u32>,

    // id -> slot
    by_id: HashMap<u64, u32>,

    // generation stamps for dedup (query-time, no clearing)
    gen: u32,
    stamps: Vec<u32>,

    // audit counters (G3 evidence)
    pub fast_path_allocs: std::sync::atomic::AtomicU64,
    pub queries: std::sync::atomic::AtomicU64,
    pub upserts: std::sync::atomic::AtomicU64,
}

impl EntityMirror {
    pub fn new(expected_entities: usize) -> Self {
        let n = expected_entities.max(1024);
        Self {
            x0: Vec::with_capacity(n),
            y0: Vec::with_capacity(n),
            z0: Vec::with_capacity(n),
            x1: Vec::with_capacity(n),
            y1: Vec::with_capacity(n),
            z1: Vec::with_capacity(n),
            ids: Vec::with_capacity(n),
            occupied: Vec::with_capacity(n),
            span: Vec::with_capacity(n),
            free: Vec::new(),
            live: 0,
            cells: HashMap::with_capacity(n / 4 + 16),
            wildcard: Vec::new(),
            by_id: HashMap::with_capacity(n),
            gen: 1,
            stamps: Vec::with_capacity(n),
            fast_path_allocs: std::sync::atomic::AtomicU64::new(0),
            queries: std::sync::atomic::AtomicU64::new(0),
            upserts: std::sync::atomic::AtomicU64::new(0),
        }
    }

    #[inline]
    fn is_wildcard_box(b: &[f64; 6]) -> bool {
        // NaN on any axis, inverted on any axis, or span > MAX_CELLS_PER_AXIS:
        // anything the grid cannot bucket safely goes to the conservative list.
        if b[0].is_nan() || b[1].is_nan() || b[2].is_nan() || b[3].is_nan() || b[4].is_nan() || b[5].is_nan() {
            return true;
        }
        if b[3] < b[0] || b[4] < b[1] || b[5] < b[2] {
            return true;
        }
        let (cx0, cy0, cz0) = (cell_of(b[0]), cell_of(b[1]), cell_of(b[2]));
        let (cx1, cy1, cz1) = (cell_of(b[3]), cell_of(b[4]), cell_of(b[5]));
        (cx1 - cx0).abs() > MAX_CELLS_PER_AXIS
            || (cy1 - cy0).abs() > MAX_CELLS_PER_AXIS
            || (cz1 - cz0).abs() > MAX_CELLS_PER_AXIS
    }

    /// Insert or update an entity box. New entities allocate (amortized, slow
    /// path); updates of existing entities with an unchanged cell span are
    /// allocation-free (the G3 fast path — pure SoA writes).
    pub fn upsert(&mut self, id: u64, b: [f64; 6]) {
        self.upserts.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if let Some(&slot) = self.by_id.get(&id) {
            let old_span = self.span[slot as usize];
            self.write_box(slot as usize, &b);
            let new_span = compute_span(&b);
            if new_span != old_span {
                self.detach(slot as usize);
                self.attach(slot as usize, &b, new_span);
            }
            return;
        }
        // slow path: new entity
        let slot: u32 = match self.free.pop() {
            Some(s) => s,
            None => {
                let s = self.x0.len() as u32;
                self.x0.push(0.0);
                self.y0.push(0.0);
                self.z0.push(0.0);
                self.x1.push(0.0);
                self.y1.push(0.0);
                self.z1.push(0.0);
                self.ids.push(0);
                self.occupied.push(false);
                self.span.push([0; 6]);
                self.stamps.push(0);
                s
            }
        };
        let s = slot as usize;
        self.occupied[s] = true;
        self.ids[s] = id;
        self.live += 1;
        self.write_box(s, &b);
        self.attach(s, &b, compute_span(&b));
        self.by_id.insert(id, slot);
    }

    fn write_box(&mut self, s: usize, b: &[f64; 6]) {
        self.x0[s] = b[0];
        self.y0[s] = b[1];
        self.z0[s] = b[2];
        self.x1[s] = b[3];
        self.y1[s] = b[4];
        self.z1[s] = b[5];
    }

    /// Remove an entity. Returns false if the id is unknown (idempotent).
    pub fn remove(&mut self, id: u64) -> bool {
        let Some(slot) = self.by_id.remove(&id) else {
            return false;
        };
        let s = slot as usize;
        self.detach(s);
        self.occupied[s] = false;
        self.live -= 1;
        self.free.push(slot);
        true
    }

    fn detach(&mut self, s: usize) {
        if self.is_wildcard_slot(s) {
            if let Some(p) = self.wildcard.iter().position(|&x| x as usize == s) {
                self.wildcard.swap_remove(p);
            }
            return;
        }
        let sp = self.span[s];
        for cx in sp[0]..=sp[3] {
            for cy in sp[1]..=sp[4] {
                for cz in sp[2]..=sp[5] {
                    let key = pack(cx, cy, cz);
                    if let Some(cell) = self.cells.get_mut(&key) {
                        if let Some(p) = cell.slots.iter().position(|&x| x as usize == s) {
                            cell.slots.swap_remove(p);
                        }
                        if cell.slots.is_empty() {
                            self.cells.remove(&key);
                        }
                    }
                }
            }
        }
    }

    fn is_wildcard_slot(&self, s: usize) -> bool {
        let sp = self.span[s];
        sp == WILDCARD_SPAN
    }

    fn attach(&mut self, s: usize, b: &[f64; 6], span: [i32; 6]) {
        if Self::is_wildcard_box(b) {
            self.span[s] = WILDCARD_SPAN;
            self.wildcard.push(s as u32);
            return;
        }
        self.span[s] = span;
        let (cx0, cy0, cz0) = (span[0], span[1], span[2]);
        let (cx1, cy1, cz1) = (span[3], span[4], span[5]);
        for cx in cx0..=cx1 {
            for cy in cy0..=cy1 {
                for cz in cz0..=cz1 {
                    let key = pack(cx, cy, cz);
                    let cell = self.cells.entry(key).or_default();
                    cell.slots.push(s as u32);
                }
            }
        }
    }

    /// Query (scalar kernel). Writes candidate entity ids into `out_ids`
    /// (duplicates impossible: generation-stamp dedup) and returns the count.
    /// Candidates form a SUPERSET of the kernel box test; the caller re-verifies
    /// with the live kernel state (G1). No allocation.
    pub fn query_scalar(&mut self, q: &[f64; 6], out_ids: &mut [u64]) -> usize {
        self.queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let gen = self.next_gen();
        let mut n = 0usize;
        // wildcard first (conservative)
        for &slot in &self.wildcard {
            let s = slot as usize;
            if self.occupied[s] && intersects(&self.box_of(s), q) {
                if n < out_ids.len() {
                    out_ids[n] = self.ids[s];
                }
                n += 1;
            }
        }
        let (cx0, cy0, cz0) = (cell_of(q[0]), cell_of(q[1]), cell_of(q[2]));
        let (cx1, cy1, cz1) = (cell_of(q[3]), cell_of(q[4]), cell_of(q[5]));
        for cx in cx0..=cx1 {
            for cy in cy0..=cy1 {
                for cz in cz0..=cz1 {
                    let Some(cell) = self.cells.get(&pack(cx, cy, cz)) else {
                        continue;
                    };
                    for &slot in &cell.slots {
                        let s = slot as usize;
                        if self.stamps[s] == gen || !self.occupied[s] {
                            continue;
                        }
                        self.stamps[s] = gen;
                        if intersects(&self.box_of(s), q) {
                            if n < out_ids.len() {
                                out_ids[n] = self.ids[s];
                            }
                            n += 1;
                        }
                    }
                }
            }
        }
        n
    }

    /// Query (AVX2 4-wide batch kernel, x86_64 only; identical candidate sets
    /// to `query_scalar` by construction — comparisons only, no FP arithmetic).
    /// Falls back to the scalar path elsewhere.
    pub fn query(&mut self, q: &[f64; 6], out_ids: &mut [u64]) -> usize {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                return self.query_avx2(q, out_ids);
            }
        }
        self.query_scalar(q, out_ids)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn query_avx2(&mut self, q: &[f64; 6], out_ids: &mut [u64]) -> usize {
        use std::arch::x86_64::*;
        self.queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let gen = self.next_gen();
        let mut n = 0usize;
        // wildcard (scalar — rare list)
        for &slot in &self.wildcard {
            let s = slot as usize;
            if self.occupied[s] && intersects(&self.box_of(s), q) {
                if n < out_ids.len() {
                    out_ids[n] = self.ids[s];
                }
                n += 1;
            }
        }
        let (cx0, cy0, cz0) = (cell_of(q[0]), cell_of(q[1]), cell_of(q[2]));
        let (cx1, cy1, cz1) = (cell_of(q[3]), cell_of(q[4]), cell_of(q[5]));
        // staging: 4 boxes x 6 coords, layout [x0 x1 y0 y1 z0 z1] per box
        let mut bx = [0.0f64; 4]; // minX per box
        let mut ax = [0.0f64; 4]; // maxX per box
        let mut by = [0.0f64; 4];
        let mut ay = [0.0f64; 4];
        let mut bz = [0.0f64; 4];
        let mut az = [0.0f64; 4];
        let mut slotbuf = [0u32; 4];
        unsafe {
            let qx0 = _mm256_set1_pd(q[0]);
            let qx1 = _mm256_set1_pd(q[3]);
            let qy0 = _mm256_set1_pd(q[1]);
            let qy1 = _mm256_set1_pd(q[4]);
            let qz0 = _mm256_set1_pd(q[2]);
            let qz1 = _mm256_set1_pd(q[5]);
            for cx in cx0..=cx1 {
                for cy in cy0..=cy1 {
                    for cz in cz0..=cz1 {
                        let Some(cell) = self.cells.get(&pack(cx, cy, cz)) else {
                            continue;
                        };
                        let slots = &cell.slots;
                        let mut i = 0usize;
                        while i < slots.len() {
                            let chunk = (slots.len() - i).min(4);
                            for k in 0..chunk {
                                let s = slots[i + k] as usize;
                                slotbuf[k] = slots[i + k];
                                bx[k] = self.x0[s];
                                ax[k] = self.x1[s];
                                by[k] = self.y0[s];
                                ay[k] = self.y1[s];
                                bz[k] = self.z0[s];
                                az[k] = self.z1[s];
                            }
                            if chunk == 4 {
                                let mx = _mm256_loadu_pd(bx.as_ptr());
                                let Mx = _mm256_loadu_pd(ax.as_ptr());
                                let my = _mm256_loadu_pd(by.as_ptr());
                                let My = _mm256_loadu_pd(ay.as_ptr());
                                let mz = _mm256_loadu_pd(bz.as_ptr());
                                let Mz = _mm256_loadu_pd(az.as_ptr());
                                // other.max >= this.min && other.min <= this.max, per axis
                                let cx_ = _mm256_cmp_pd(Mx, qx0, _CMP_GE_OQ);
                                let cx2 = _mm256_cmp_pd(mx, qx1, _CMP_LE_OQ);
                                let cy_ = _mm256_cmp_pd(My, qy0, _CMP_GE_OQ);
                                let cy2 = _mm256_cmp_pd(my, qy1, _CMP_LE_OQ);
                                let cz_ = _mm256_cmp_pd(Mz, qz0, _CMP_GE_OQ);
                                let cz2 = _mm256_cmp_pd(mz, qz1, _CMP_LE_OQ);
                                let mut m = _mm256_movemask_pd(_mm256_and_pd(cx_, cx2));
                                m &= _mm256_movemask_pd(_mm256_and_pd(cy_, cy2));
                                m &= _mm256_movemask_pd(_mm256_and_pd(cz_, cz2));
                                for k in 0..4 {
                                    let s = slotbuf[k] as usize;
                                    if (m >> k) & 1 == 1 && self.stamps[s] != gen {
                                        self.stamps[s] = gen;
                                        if n < out_ids.len() {
                                            out_ids[n] = self.ids[s];
                                        }
                                        n += 1;
                                    }
                                }
                            } else {
                                // tail: scalar, same predicate
                                for k in 0..chunk {
                                    let s = slotbuf[k] as usize;
                                    if self.stamps[s] == gen {
                                        continue;
                                    }
                                    let boxm = [
                                        bx[k], by[k], bz[k], ax[k], ay[k], az[k],
                                    ];
                                    if intersects(&boxm, q) {
                                        self.stamps[s] = gen;
                                        if n < out_ids.len() {
                                            out_ids[n] = self.ids[s];
                                        }
                                        n += 1;
                                    } else {
                                        self.stamps[s] = gen;
                                    }
                                }
                            }
                            i += chunk;
                        }
                    }
                }
            }
        }
        n
    }

    #[inline]
    fn box_of(&self, s: usize) -> [f64; 6] {
        [
            self.x0[s], self.y0[s], self.z0[s], self.x1[s], self.y1[s], self.z1[s],
        ]
    }

    #[inline]
    fn next_gen(&mut self) -> u32 {
        let g = self.gen;
        self.gen = g.wrapping_add(1);
        if self.gen == 0 {
            // wrap: reset stamps (slow path, once per 2^32 queries)
            for st in self.stamps.iter_mut() {
                *st = 0;
            }
            self.gen = 1;
        }
        g
    }

    pub fn live(&self) -> u32 {
        self.live
    }

    pub fn cells(&self) -> usize {
        self.cells.len()
    }

    pub fn wildcard_len(&self) -> usize {
        self.wildcard.len()
    }

    /// Stability checksum (self-test/parity harness handle, ClimateRTree
    /// checksumTreeHandle precedent): folds live ids, boxes, cell count and
    /// wildcard size. Two mirrors fed the same op sequence produce the SAME
    /// checksum.
    pub fn checksum(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325; // FNV-1a basis
        let feed = |h: &mut u64, w: u64| {
            *h ^= w;
            *h = h.wrapping_mul(0x100000001b3);
        };
        feed(&mut h, self.live as u64);
        feed(&mut h, self.cells.len() as u64);
        feed(&mut h, self.wildcard.len() as u64);
        for s in 0..self.x0.len() {
            if self.occupied[s] {
                feed(&mut h, self.ids[s]);
                for v in self.box_of(s) {
                    feed(&mut h, v.to_bits());
                }
            }
        }
        h
    }
}

const WILDCARD_SPAN: [i32; 6] = [i32::MIN; 6];

fn compute_span(b: &[f64; 6]) -> [i32; 6] {
    [
        cell_of(b[0]),
        cell_of(b[1]),
        cell_of(b[2]),
        cell_of(b[3]),
        cell_of(b[4]),
        cell_of(b[5]),
    ]
}

// ---------------------------------------------------------------------------
// tests: oracle parity (property), dedup, wildcard, churn, gen-wrap, zero-alloc
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    fn xorshift(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    fn rand_f(state: &mut u64, lo: f64, hi: f64) -> f64 {
        let u = (xorshift(state) >> 11) as f64 / (1u64 << 53) as f64;
        lo + u * (hi - lo)
    }

    /// Linear-scan oracle with the WILDCARD semantics folded in: candidates =
    /// (grid-resident boxes passing the predicate) U (wildcard boxes passing).
    /// Under full reconciliation this equals the mirror result set exactly.
    fn oracle(m: &EntityMirror, q: &[f64; 6]) -> Vec<u64> {
        let mut out = Vec::new();
        for s in 0..m.x0.len() {
            if m.occupied[s] {
                // grid membership only affects which queries SEE a box; the
                // oracle scans all slots with the same predicate, so equality
                // must hold — a grid-resident box outside the query's cell
                // range can only intersect the query if the cell math is
                // wrong, which is exactly what the property test catches.
                if intersects(&m.box_of(s), q) {
                    out.push(m.ids[s]);
                }
            }
        }
        out.sort_unstable();
        out
    }

    fn mirror_sorted(m: &mut EntityMirror, q: &[f64; 6]) -> Vec<u64> {
        let mut buf = vec![0u64; 65536];
        let n = m.query(q, &mut buf);
        let mut v = buf[..n].to_vec();
        v.sort_unstable();
        v
    }

    #[test]
    fn property_parity_vs_oracle() {
        let mut st = 0x5EED_2026_0917_u64;
        for trial in 0..64 {
            let mut m = EntityMirror::new(256);
            let mut live: HashMap<u64, [f64; 6]> = HashMap::new();
            let mut next_id = 1u64;
            for step in 0..3000 {
                let r = xorshift(&mut st) % 100;
                if r < 45 || live.is_empty() {
                    // upsert (new or move)
                    let id = if !live.is_empty() && r < 30 {
                        // move an existing entity
                        let k = (xorshift(&mut st) as usize) % live.len();
                        *live.keys().nth(k).unwrap()
                    } else {
                        let id = next_id;
                        next_id += 1;
                        id
                    };
                    let cx = rand_f(&mut st, -2048.0, 2048.0);
                    let cy = rand_f(&mut st, -64.0, 320.0);
                    let cz = rand_f(&mut st, -2048.0, 2048.0);
                    let w = rand_f(&mut st, 0.4, 2.2);
                    let h = rand_f(&mut st, 0.4, 2.2);
                    let b = [cx, cy, cz, cx + w, cy + h, cz + w];
                    m.upsert(id, b);
                    live.insert(id, b);
                } else if r < 60 {
                    // remove
                    let k = (xorshift(&mut st) as usize) % live.len();
                    let id = *live.keys().nth(k).unwrap();
                    assert!(m.remove(id));
                    live.remove(&id);
                } else {
                    // query: small box at a random position (matches the boat /
                    // pushable / hopper query shapes: 1..8 blocks per axis)
                    let cx = rand_f(&mut st, -2048.0, 2048.0);
                    let cy = rand_f(&mut st, -64.0, 320.0);
                    let cz = rand_f(&mut st, -2048.0, 2048.0);
                    let w = rand_f(&mut st, 1.0, 8.0);
                    let h = rand_f(&mut st, 1.0, 4.0);
                    let q = [cx, cy, cz, cx + w, cy + h, cz + w];
                    let got = mirror_sorted(&mut m, &q);
                    let want = oracle(&m, &q);
                    assert_eq!(got, want, "trial {trial} step {step} q={q:?}");
                    // SIMD path must equal the scalar path EXACTLY (same
                    // candidate sets, not just same count)
                    let mut buf2 = vec![0u64; 65536];
                    let n2 = m.query_scalar(&q, &mut buf2);
                    let mut v2 = buf2[..n2].to_vec();
                    v2.sort_unstable();
                    assert_eq!(got, v2, "scalar-vs-simd trial {trial} step {step}");
                }
            }
            assert_eq!(m.live() as usize, live.len(), "trial {trial}");
        }
    }

    #[test]
    fn dedup_multi_cell() {
        let mut m = EntityMirror::new(64);
        // one box spanning 3x3x3 = 27 cells
        m.upsert(42, [-16.0, -16.0, -16.0, 16.0, 16.0, 16.0]);
        let mut buf = [0u64; 4096];
        let n = m.query(&[-20.0, -20.0, -20.0, 20.0, 20.0, 20.0], &mut buf);
        assert_eq!(n, 1, "multi-cell entity must appear exactly once");
        assert_eq!(buf[0], 42);
    }

    #[test]
    fn wildcard_giant_and_nan() {
        let mut m = EntityMirror::new(64);
        // giant box (>> 4 cells per axis) -> wildcard
        m.upsert(7, [-1e6, -1e6, -1e6, 1e6, 1e6, 1e6]);
        assert_eq!(m.wildcard_len(), 1);
        // NaN box -> wildcard (conservative superset)
        m.upsert(8, [f64::NAN, 0.0, 0.0, 1.0, 1.0, 1.0]);
        assert_eq!(m.wildcard_len(), 2);
        // inverted box -> wildcard
        m.upsert(9, [10.0, 10.0, 10.0, 0.0, 0.0, 0.0]);
        assert_eq!(m.wildcard_len(), 3);
        let mut buf = [0u64; 4096];
        let n = m.query(&[100.0, 100.0, 100.0, 200.0, 200.0, 200.0], &mut buf);
        // the giant wildcard box intersects everything; NaN/inverted do not
        // pass the predicate (NaN compares false) — predicate decides.
        let mut got = buf[..n].to_vec();
        got.sort_unstable();
        assert_eq!(got, vec![7]);
        assert_eq!(m.remove(7), true);
        let n = m.query(&[100.0, 100.0, 100.0, 200.0, 200.0, 200.0], &mut buf);
        assert_eq!(n, 0);
    }

    #[test]
    fn churn_and_free_list_reuse() {
        let mut m = EntityMirror::new(8);
        for round in 0..1000u64 {
            m.upsert(round, [round as f64, 0.0, 0.0, round as f64 + 1.0, 1.0, 1.0]);
        }
        assert_eq!(m.live(), 1000);
        for round in 0..1000u64 {
            assert!(m.remove(round));
        }
        assert_eq!(m.live(), 0);
        // re-add: slots must be reused, live count consistent
        for round in 0..500u64 {
            m.upsert(1_000_000 + round, [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        }
        assert_eq!(m.live(), 500);
        assert_eq!(m.cells(), 1);
        assert!(m.remove(1_000_000));
        assert!(m.remove(1_000_001));
        assert!(!m.remove(1_000_001), "double remove must be idempotent-false");
    }

    #[test]
    fn gen_wrap_resets_stamps() {
        let mut m = EntityMirror::new(8);
        m.upsert(1, [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        m.gen = u32::MAX;
        let mut buf = [0u64; 64];
        let n = m.query(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0], &mut buf);
        assert_eq!(n, 1);
        let n = m.query(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0], &mut buf);
        assert_eq!(n, 1, "post-wrap query must still see the entity");
    }

    #[test]
    fn checksum_stability() {
        let mut a = EntityMirror::new(64);
        let mut b = EntityMirror::new(64);
        let mut st = 0xC0FFEE_u64;
        for i in 0..500u64 {
            let x = rand_f(&mut st, -100.0, 100.0);
            let y = rand_f(&mut st, 0.0, 64.0);
            let z = rand_f(&mut st, -100.0, 100.0);
            let box_ = [x, y, z, x + 1.0, y + 2.0, z + 1.0];
            a.upsert(i, box_);
            b.upsert(i, box_);
        }
        assert_eq!(a.checksum(), b.checksum());
        b.remove(3);
        assert_ne!(a.checksum(), b.checksum());
    }

    /// G3 core evidence: query and same-span move-upsert allocate NOTHING.
    /// Uses a counting global allocator — run the suite single-threaded
    /// (cargo test -- --test-threads=1) or this test races other tests' allocs.
    #[test]
    fn zero_alloc_fast_path() {
        alloc_counter::reset();
        let mut m = EntityMirror::new(4096);
        let mut st = 0xD00D_u64;
        // Place every box STRICTLY INSIDE one cell interior: x,z in
        // [c*16+2, c*16+13], width 0.9 -> cell span can never change on the
        // small moves below. Cell-boundary-crossing moves are a DESIGNED slow
        // path (map churn) — measured separately at the end.
        let mut orig = Vec::with_capacity(3000);
        for i in 0..3000u64 {
            let cell_x = (xorshift(&mut st) % 64) as i32 - 32;
            let cell_z = (xorshift(&mut st) % 64) as i32 - 32;
            let x = cell_x as f64 * 16.0 + 2.0 + rand_f(&mut st, 0.0, 10.0);
            let z = cell_z as f64 * 16.0 + 2.0 + rand_f(&mut st, 0.0, 10.0);
            let y = 64.0 + rand_f(&mut st, 0.0, 8.0);
            let b = [x, y, z, x + 0.9, y + 1.8, z + 0.9];
            m.upsert(i, b);
            orig.push(b);
        }
        // (1) queries: the hot path — must be EXACTLY zero alloc
        let mut buf = vec![0u64; 1024];
        alloc_counter::reset();
        for _ in 0..10_000u64 {
            let x = rand_f(&mut st, -512.0, 512.0);
            let z = rand_f(&mut st, -512.0, 512.0);
            let _ = m.query(&[x, 64.0, z, x + 6.0, 70.0, z + 6.0], &mut buf);
        }
        assert_eq!(alloc_counter::count(), 0, "query fast path allocated");
        // (2) same-span move upserts: must be EXACTLY zero alloc
        alloc_counter::reset();
        for (i, b) in orig.iter().enumerate() {
            // +0.05/-0.05 stays inside the cell interior by construction
            let nb = [
                b[0] + 0.05, b[1], b[2] + 0.05, b[3] + 0.05, b[4], b[5] + 0.05,
            ];
            m.upsert(i as u64, nb);
        }
        assert_eq!(alloc_counter::count(), 0, "same-span move path allocated");
        assert_eq!(m.fast_path_allocs.load(Ordering::Relaxed), 0);
        // (3) boundary-crossing moves: designed slow path — bank the cost shape
        alloc_counter::reset();
        for (i, b) in orig.iter().enumerate() {
            let nb = [b[0] + 3.9, b[1], b[2] + 3.9, b[3] + 3.9, b[4], b[5] + 3.9];
            m.upsert(i as u64, nb);
        }
        let slow = alloc_counter::count();
        println!("ENT-BP core: 3000 cell-crossing move upserts -> {slow} allocs (designed slow path)");
        assert!(slow <= 3000 * 4, "slow-path alloc blowup: {slow}");
    }


    /// Manual micro-bench (run: cargo test --release bench_entbp_core -- --ignored --nocapture).
    /// CORE-level numbers only (sandbox, not a CI claim).
    #[test]
    #[ignore]
    fn bench_entbp_core() {
        let mut m = EntityMirror::new(32768);
        let mut st = 0xB0C0_u64;
        // farm-world shape: 70% of entities in ~12 dense clusters, rest spread
        let mut clusters: Vec<[f64; 3]> = Vec::new();
        for _ in 0..12 {
            clusters.push([
                rand_f(&mut st, -1024.0, 1024.0),
                rand_f(&mut st, 0.0, 128.0),
                rand_f(&mut st, -1024.0, 1024.0),
            ]);
        }
        for i in 0..30_000u64 {
            let (cx, cy, cz) = if xorshift(&mut st) % 100 < 70 {
                let c = clusters[(xorshift(&mut st) as usize) % clusters.len()];
                (c[0], c[1], c[2])
            } else {
                (rand_f(&mut st, -1024.0, 1024.0), rand_f(&mut st, 0.0, 128.0), rand_f(&mut st, -1024.0, 1024.0))
            };
            let x = cx + rand_f(&mut st, -8.0, 8.0);
            let y = cy + rand_f(&mut st, 0.0, 4.0);
            let z = cz + rand_f(&mut st, -8.0, 8.0);
            m.upsert(i, [x, y, z, x + 0.8, y + 1.8, z + 0.8]);
        }
        let mut buf = vec![0u64; 8192];
        // warmup
        for _ in 0..200 {
            m.query(&[0.0, 64.0, 0.0, 8.0, 70.0, 8.0], &mut buf);
        }
        let t0 = std::time::Instant::now();
        let mut hits = 0usize;
        let qn = 20_000;
        for _ in 0..qn {
            let c = clusters[(xorshift(&mut st) as usize) % clusters.len()];
            let x = c[0] + rand_f(&mut st, -8.0, 8.0);
            let y = c[1];
            let z = c[2] + rand_f(&mut st, -8.0, 8.0);
            hits += m.query(&[x, y, z, x + 8.0, y + 4.0, z + 8.0], &mut buf);
        }
        let dt = t0.elapsed();
        println!(
            "ENT-BP core bench: {qn} queries over {} live entities in {} cells: {:?} total, {:?}/query, avg candidates {:.1}",
            m.live(),
            m.cells(),
            dt,
            dt / qn,
            hits as f64 / qn as f64
        );
        // oracle comparison on the same shape
        let t1 = std::time::Instant::now();
        let mut ohits = 0usize;
        for _ in 0..qn {
            let c = clusters[(xorshift(&mut st) as usize) % clusters.len()];
            let x = c[0] + rand_f(&mut st, -8.0, 8.0);
            let y = c[1];
            let z = c[2] + rand_f(&mut st, -8.0, 8.0);
            ohits += oracle(&m, &[x, y, z, x + 8.0, y + 4.0, z + 8.0]).len();
        }
        println!("oracle linear scan: {:?}", t1.elapsed());
        assert_eq!(hits, ohits, "bench paths must agree on candidate totals");
    }
}

/// Minimal counting allocator used ONLY by the zero-alloc tests (G3 core
/// evidence). No dependencies; swap-in via #[global_allocator] below.
///
/// PER-THREAD counting (const-init thread_local — no lazy allocation, hence
/// no recursion into the allocator): parallel cargo-test threads each count
/// only THEIR OWN allocations, so the zero-assert is deterministic under any
/// parallel test schedule (the global-counter version raced with sibling
/// tests' allocations — S7-93 lesson).
#[cfg(test)]
pub(crate) mod alloc_counter {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    thread_local! {
        static COUNT: Cell<u64> = const { Cell::new(0) };
    }

    pub struct Counting;
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            COUNT.with(|c| c.set(c.get().wrapping_add(1)));
            unsafe { System.alloc(l) }
        }
        unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
            unsafe { System.dealloc(p, l) }
        }
    }

    /// Resets the CURRENT thread's counter only.
    pub fn reset() {
        COUNT.with(|c| c.set(0));
    }
    /// Reads the CURRENT thread's counter only.
    pub fn count() -> u64 {
        COUNT.with(|c| c.get())
    }
}

#[cfg(test)]
#[global_allocator]
static A: alloc_counter::Counting = alloc_counter::Counting;
