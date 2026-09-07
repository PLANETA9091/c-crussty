//! Differential-fuzz model for the area-map bridge (TASK-15).
//!
//! The product hot path lives in the JAVA half
//! (`area-map/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.java`):
//! `SingleUserAreaMap.update()` is byte-patched to
//! `SingleUserAreaMapOps.run(...)`, which does exactly:
//!
//!   1. `fromX == Integer.MIN_VALUE` guard (map never initialized),
//!   2. same-state fast path: `from==to && oldD==newD` -> skip everything,
//!   3. native enumeration (`nativeUpdateOpsBatch`) into grow-only
//!      ThreadLocal scratch (`ops` 0=Add/1=Remove, `keys` = z<<32 | x,
//!      u32 lanes; contract: adds = new square \ old square, removes =
//!      old square \ new square — the contract TASK-11 verified against the
//!      REAL `libpaper_native_jni.so`, smoke S5 + live bridge selftest),
//!   4. the apply loop: decode key, one abstract callback per op.
//!
//! This crate is a faithful, executable RUST MODEL of that contract
//! ([`FastPathMap`]) plus a fully independent reference implementation
//! ([`ReferenceMap`]) — the naive per-cell set difference, same shape the
//! TASK-11 driver used as its oracle. [`Window`] materializes both worlds
//! into a w x h grid for element-wise parity checks.
//!
//! No JVM, no JNI, no .so: everything runs as a plain headless
//! `cargo test -p area-map-fuzz`. The closed native itself is NOT exercised
//! here (that is TASK-11's S5 job); the fuzz verifies the in-repo Java-half
//! semantics: the fast path must be semantically transparent — skipping the
//! enumeration when the state did not change must produce bit-identical
//! tracked sets to always-enumerate-and-apply, over randomized grids,
//! coordinates (incl. MIN/MAX/zero edges) and mutation sequences.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// RNG: xorshift64* + splitmix64 seeding (inline, no external deps)
// ---------------------------------------------------------------------------

/// Fixed base seed: all case seeds derive from this, so any failure is
/// reproducible from the reported (base, case_index) pair alone.
pub const BASE_SEED: u64 = 0x5EED_5325_97B0_0015; // nods at TASK-11 commit 532597b

/// xorshift64* (Vigna) — small, deterministic, dependency-free.
pub struct XorShift64Star {
    s: u64,
}

impl XorShift64Star {
    /// Zero seeds are forbidden by the algorithm; fold them away.
    pub fn new(seed: u64) -> Self {
        Self {
            s: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed },
        }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.s;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.s = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform-ish value in [0, n) — n must be > 0. Modulo bias is
    /// irrelevant for fuzz semantics.
    #[inline]
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }

    /// Value in [lo, hi] inclusive.
    #[inline]
    pub fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
        debug_assert!(hi >= lo);
        lo + self.below((hi - lo + 1) as u64) as i64
    }

    #[inline]
    pub fn chance_pct(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
}

/// splitmix64 finalizer — derives independent per-case seeds from the base.
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic per-case seed from the fixed [`BASE_SEED`].
pub fn case_seed(base: u64, case_index: usize) -> u64 {
    let mut st = base ^ (case_index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    splitmix64(&mut st)
}

// ---------------------------------------------------------------------------
// Shared square-membership predicate (the CONTRACT, not an implementation
// choice): cell (x,z) is covered by the square centered (cx,cz) radius d.
// Wrapping arithmetic mirrors the Java reference (`Math.abs(x - toX) > d`
// in the TASK-11 oracle) and keeps i32 cell coords closed under the ops.
// Both worlds use this exact predicate, so the differential isolates the
// fast-path / apply-loop / scratch / key-decode wiring.
// ---------------------------------------------------------------------------

#[inline]
fn in_square(x: i32, cx: i32, d: i32, z: i32, cz: i32) -> bool {
    (x.wrapping_sub(cx)).wrapping_abs() <= d && (z.wrapping_sub(cz)).wrapping_abs() <= d
}

// ---------------------------------------------------------------------------
// World F: faithful port of SingleUserAreaMapOps.run() (+ the patched
// update() field-write shape). This is the "fast path" side.
// ---------------------------------------------------------------------------

/// Mirrors `SingleUserAreaMapOps.INITIAL_CAP = 2*(2*8+1)^2 = 578`
/// (covers d=8 for BOTH squares; grow-only beyond).
pub const INITIAL_CAP: usize = 2 * (2 * 8 + 1) * (2 * 8 + 1);

/// Upper bound on difference ops for rect sizes oldD/newD — exact port of
/// `SingleUserAreaMapOps.maxOps` (i64 math, clamped to i32::MAX).
pub fn max_ops(old_d: i32, new_d: i32) -> usize {
    let old_side = 2i64 * old_d as i64 + 1;
    let new_side = 2i64 * new_d as i64 + 1;
    let cap = old_side * old_side + new_side * new_side;
    cap.min(i32::MAX as i64) as usize
}

/// Model of the patched `SingleUserAreaMap` as seen by `run()`: the tracked
/// set (add/remove callbacks) plus the last-state fields the byte hook
/// writes before invoking the static helper.
pub struct FastPathMap {
    /// Cells currently reported active (addCallback minus removeCallback).
    pub tracked: HashSet<(i32, i32)>,
    last: Option<(i32, i32, i32)>,
    /// ThreadLocal Scratch equivalent: grow-only ops/keys pair, same length.
    ops: Vec<u8>,
    keys: Vec<i64>,
    // Observability (mirrors the counting-stub observables of TASK-11):
    /// Times the enumeration stand-in (== native call) was reached.
    pub enumeration_calls: u64,
    /// Callbacks applied by the apply loop.
    pub ops_applied: u64,
    /// Same-state fast-path hits (no enumeration, no callbacks).
    pub fast_path_hits: u64,
    /// MIN_VALUE guard hits (never-initialized update).
    pub min_value_guards: u64,
}

impl FastPathMap {
    pub fn new() -> Self {
        Self {
            tracked: HashSet::new(),
            last: None,
            ops: vec![0u8; INITIAL_CAP],
            keys: vec![0i64; INITIAL_CAP],
            enumeration_calls: 0,
            ops_applied: 0,
            fast_path_hits: 0,
            min_value_guards: 0,
        }
    }

    /// One patched `update(parameter, newX, newZ, newDistance)`: fields are
    /// written FIRST, then the hook tail calls `run(from..., to...)`.
    pub fn update(&mut self, to_x: i32, to_z: i32, to_d: i32) {
        let from = self
            .last
            .replace((to_x, to_z, to_d))
            .unwrap_or((i32::MIN, i32::MIN, 0)); // kernel init: lastX = Integer.MIN_VALUE
        self.run(from.0, from.1, from.2, to_x, to_z, to_d);
    }

    /// Exact port of `SingleUserAreaMapOps.run`.
    pub fn run(&mut self, from_x: i32, from_z: i32, old_d: i32, to_x: i32, to_z: i32, new_d: i32) {
        if from_x == i32::MIN {
            self.min_value_guards += 1;
            return; // never initialized: no enumeration, no callbacks
        }
        // Same-state fast path: S \ S = 0 by definition -> skip the native
        // call entirely; field writes are NOT skipped (already done above).
        if from_x == to_x && from_z == to_z && old_d == new_d {
            self.fast_path_hits += 1;
            return;
        }
        let cap = max_ops(old_d, new_d);
        // Grow-only doubling with the same overflow guard shape as Java.
        if self.ops.len() < cap {
            let mut grown = self.ops.len() as u64;
            let cap64 = cap as u64;
            while grown < cap64 {
                grown = if grown <= (i32::MAX as u64 - grown) / 2 {
                    grown * 2
                } else {
                    cap64
                };
            }
            self.ops = vec![0u8; grown as usize];
            self.keys = vec![0i64; grown as usize];
        }
        let n = enumerate_ops_batch(
            from_x, from_z, old_d, to_x, to_z, new_d, &mut self.ops, &mut self.keys,
        );
        self.enumeration_calls += 1;
        if n < 0 {
            return; // native error path (cannot happen for cap, kept for fidelity)
        }
        let n = n as usize;
        debug_assert!(n <= cap, "enumeration overflowed the scratch cap");
        // The apply loop: one key decode + one callback per op.
        for i in 0..n {
            let key = self.keys[i];
            let x = key as i32; // chunk_as_long: x in the low 32 bits
            let z = (key >> 32) as i32; // z in the high 32 bits
            if self.ops[i] == 0 {
                self.tracked.insert((x, z)); // AreaOp::Add
            } else {
                self.tracked.remove(&(x, z)); // AreaOp::Remove
            }
            self.ops_applied += 1;
        }
    }
}

impl Default for FastPathMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Stand-in for the closed `nativeUpdateOpsBatch` enumeration, honoring the
/// TASK-11-verified contract (adds = new\old as op 0, removes = old\new as
/// anything else, packed keys, n <= cap, no duplicates). Structurally
/// different from the [`ReferenceMap`] scan: z-outer/x-inner traversal,
/// fused <= predicates, buffer emission instead of direct set mutation.
/// Returns the op count as i32 (-1 reserved for the never-taken error path).
fn enumerate_ops_batch(
    from_x: i32,
    from_z: i32,
    old_d: i32,
    to_x: i32,
    to_z: i32,
    new_d: i32,
    ops: &mut [u8],
    keys: &mut [i64],
) -> i32 {
    let cap = ops.len().min(keys.len());
    let mut n = 0usize;
    // removes: sweep the OLD square, z-outer / x-inner.
    let mut dz = -old_d;
    while dz <= old_d {
        let z = from_z.wrapping_add(dz);
        let mut dx = -old_d;
        while dx <= old_d {
            let x = from_x.wrapping_add(dx);
            if !in_square(x, to_x, new_d, z, to_z) {
                if n >= cap {
                    return -1; // buffer too small: contract violation
                }
                ops[n] = 1;
                keys[n] = pack_key(x, z);
                n += 1;
            }
            dx += 1;
        }
        dz += 1;
    }
    // adds: sweep the NEW square, z-outer / x-inner.
    let mut dz = -new_d;
    while dz <= new_d {
        let z = to_z.wrapping_add(dz);
        let mut dx = -new_d;
        while dx <= new_d {
            let x = to_x.wrapping_add(dx);
            if !in_square(x, from_x, old_d, z, from_z) {
                if n >= cap {
                    return -1;
                }
                ops[n] = 0;
                keys[n] = pack_key(x, z);
                n += 1;
            }
            dx += 1;
        }
        dz += 1;
    }
    n as i32
}

/// `chunk_as_long`: z in the high u32 lane, x in the low u32 lane.
#[inline]
pub fn pack_key(x: i32, z: i32) -> i64 {
    (((z as u32 as u64) << 32) | (x as u32 as u64)) as i64
}

// ---------------------------------------------------------------------------
// World R: the apply-loop reference — independent naive per-cell set
// difference, the shape TASK-11's driver oracle used. No fast path, no
// scratch, no keys: enumerates and mutates the set directly.
// ---------------------------------------------------------------------------

pub struct ReferenceMap {
    pub tracked: HashSet<(i32, i32)>,
    last: Option<(i32, i32, i32)>,
    pub ops_applied: u64,
}

impl ReferenceMap {
    pub fn new() -> Self {
        Self {
            tracked: HashSet::new(),
            last: None,
            ops_applied: 0,
        }
    }

    /// Same field-write shape as the patched update() — the reference must
    /// see the identical (from, to) stream.
    pub fn update(&mut self, to_x: i32, to_z: i32, to_d: i32) {
        let from = self
            .last
            .replace((to_x, to_z, to_d))
            .unwrap_or((i32::MIN, i32::MIN, 0));
        self.enumerate_naive(from.0, from.1, from.2, to_x, to_z, to_d);
    }

    /// TASK-11 oracle shape: removes = old square cells not covered by the
    /// new square, then adds = new square cells not covered by the old one.
    fn enumerate_naive(&mut self, from_x: i32, from_z: i32, old_d: i32, to_x: i32, to_z: i32, new_d: i32) {
        if from_x == i32::MIN {
            return; // uninitialized map: no difference (same as the smoke oracle)
        }
        for dx in -old_d..=old_d {
            for dz in -old_d..=old_d {
                let x = from_x.wrapping_add(dx);
                let z = from_z.wrapping_add(dz);
                if !in_square(x, to_x, new_d, z, to_z) {
                    self.tracked.remove(&(x, z));
                    self.ops_applied += 1;
                }
            }
        }
        for dx in -new_d..=new_d {
            for dz in -new_d..=new_d {
                let x = to_x.wrapping_add(dx);
                let z = to_z.wrapping_add(dz);
                if !in_square(x, from_x, old_d, z, from_z) {
                    self.tracked.insert((x, z));
                    self.ops_applied += 1;
                }
            }
        }
    }
}

impl Default for ReferenceMap {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Full-grid parity: materialize a w x h window of chunk coords into both
// worlds and compare element-wise (row-major). Returns the first differing
// (row, col) plus the chunk coord it maps to, for failure reporting.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub w: usize,
    pub h: usize,
    pub x0: i32,
    pub z0: i32,
}

impl Window {
    /// Cell (row, col) -> chunk coord (guarded against i32 overflow by
    /// construction; windows never touch the coordinate wrap seam unless a
    /// test explicitly places them there via u32-safe i64 math here).
    #[inline]
    pub fn cell(&self, row: usize, col: usize) -> (i32, i32) {
        let x = (self.x0 as i64 + row as i64) as i32;
        let z = (self.z0 as i64 + col as i64) as i32;
        (x, z)
    }

    pub fn cells(&self) -> usize {
        self.w.saturating_mul(self.h)
    }
}

/// First row-major cell where the two tracked sets disagree.
pub fn first_grid_mismatch(
    a: &HashSet<(i32, i32)>,
    b: &HashSet<(i32, i32)>,
    win: &Window,
) -> Option<(usize, usize, i32, i32)> {
    for row in 0..win.h {
        for col in 0..win.w {
            let cell = win.cell(row, col);
            if a.contains(&cell) != b.contains(&cell) {
                return Some((row, col, cell.0, cell.1));
            }
        }
    }
    None
}
