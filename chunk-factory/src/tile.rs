//! NCF P2.12 — tiles by region: cross-chunk memoization of XZ-only (y-free)
//! subtrees of the interpolated machine, keyed by WORLD column coordinates.
//!
//! Contract (I5): every entry is keyed by (spec_hash, level_seed, node-shape,
//! world x, world z). The TileCache lives INSIDE the RandomState instance —
//! which is itself built per (seed, world spec) — so the epoch is implicit in
//! the object identity; the hash mixes spec_hash + level_seed anyway for
//! belt-and-braces.
//!
//! Bit-exactness contract (I1/I2/I3): a y-free node is a pure function
//! f(spec, seed, world_x, world_z) — its value never depends on y (analysis
//! in interpolator.rs `wnode_y_free`, conservative: any doubt => not y-free).
//! Memoizing a pure function returns bit-identical f64. Values are stored as
//! raw f64 (no rounding).
//!
//! Storage (R2#4): fixed power-of-two open-addressing table with linear
//! probing (bounded probe window) + wraparound overwrite. The previous
//! RefCell<HashMap> cleared ALL entries whenever the entry cap was hit — a
//! multi-millisecond pause that repeated every `cap` inserts. The table
//! below never clears: a put whose probe window is full overwrites the home
//! slot (the victim merely becomes a future miss; for a pure-function memo a
//! miss is a recompute, never a wrong value). Key 0 is a legal tile_key
//! output (fnv1a can produce 0), so occupancy is tracked in a separate
//! 1-bit-per-slot bitmap instead of a key==0 sentinel.
//!
//! Cross-chunk win: slice/corner fills re-evaluate the same y-free chains
//! (shift noises, terrain-shaper spline coordinates) for every y corner of a
//! column AND again in the neighboring chunk's border columns. The tile cache
//! collapses both: O(1) per world column per node shape.

use std::cell::{Cell, RefCell};

/// Default slot count: 2^22 ≈ 4.19M slots ≈ 67 MB ((u64,f64) slot + 1
/// occupancy bit) — the R2#4 budget. Allocated lazily on the first put; the
/// old NCF_TILE_CAP env knob is honored as the SLOT count (rounded up to the
/// next power of two).
const DEFAULT_SLOTS: usize = 4_194_304;

/// Linear-probe window. Every insert lands at the FIRST EMPTY slot within
/// this many slots of the key's home (or overwrites the home slot when the
/// window is full), so a present key is always found within `PROBE_WINDOW`
/// probes and get() is O(WINDOW) even at 100% occupancy.
const PROBE_WINDOW: usize = 64;

struct Table {
    /// slot storage; (key, value) pairs. `slots[idx]` is meaningful iff the
    /// `occupied` bit idx is set (key 0 is a legal key, hence the bitmap).
    slots: Vec<(u64, f64)>,
    /// occupancy bitmap, 1 bit per slot
    occupied: Vec<u64>,
    /// number of occupied slots (entries are never removed)
    len: usize,
}

pub struct TileCache {
    table: RefCell<Table>,
    slot_count: usize, // power of two
    pub enabled: bool,
    pub hits: Cell<u64>,
    pub misses: Cell<u64>,
    /// API compat (old cap-clear counter): the open-addressing scheme never
    /// clears, so this stays 0.
    pub clears: Cell<u64>,
}

impl TileCache {
    pub fn new() -> Self {
        let enabled = std::env::var("NCF_TILE_CACHE").map(|v| v != "0").unwrap_or(true);
        let slot_count = std::env::var("NCF_TILE_CAP")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .map(|n| n.next_power_of_two())
            .unwrap_or(DEFAULT_SLOTS)
            .max(PROBE_WINDOW);
        TileCache {
            table: RefCell::new(Table { slots: Vec::new(), occupied: Vec::new(), len: 0 }),
            slot_count,
            enabled,
            hits: 0.into(),
            misses: 0.into(),
            clears: 0.into(),
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<f64> {
        if !self.enabled {
            return None;
        }
        let t = self.table.borrow();
        if t.len == 0 {
            self.misses.set(self.misses.get() + 1);
            return None;
        }
        let sc = self.slot_count;
        let window = if PROBE_WINDOW < sc { PROBE_WINDOW } else { sc };
        let mut idx = (key & (sc - 1) as u64) as usize;
        for _ in 0..window {
            if t.occupied[idx >> 6] >> (idx & 63) & 1 == 0 {
                // Empty slot: an insert never probes past the first empty
                // slot, and slots only ever go empty->occupied, so the key
                // cannot live beyond this point — miss.
                self.misses.set(self.misses.get() + 1);
                return None;
            }
            if t.slots[idx].0 == key {
                self.hits.set(self.hits.get() + 1);
                return Some(t.slots[idx].1);
            }
            idx += 1;
            if idx == sc {
                idx = 0;
            }
        }
        self.misses.set(self.misses.get() + 1);
        None
    }

    #[inline]
    pub fn put(&self, key: u64, value: f64) {
        if !self.enabled {
            return;
        }
        let mut t = self.table.borrow_mut();
        if t.slots.is_empty() {
            // Lazy allocation: a TileCache that never stores never pays the
            // (large, zero-filled) table. vec![(0, 0.0); n] is all-zero, so
            // this is a calloc — untouched pages stay uncommitted.
            *t = Table {
                slots: vec![(0u64, 0.0f64); self.slot_count],
                occupied: vec![0u64; (self.slot_count + 63) / 64],
                len: 0,
            };
        }
        let sc = self.slot_count;
        let window = if PROBE_WINDOW < sc { PROBE_WINDOW } else { sc };
        let mut idx = (key & (sc - 1) as u64) as usize;
        for _ in 0..window {
            let bit = 1u64 << (idx & 63);
            if t.occupied[idx >> 6] & bit == 0 {
                // first empty slot inside the window: insert here
                t.occupied[idx >> 6] |= bit;
                t.slots[idx] = (key, value);
                t.len += 1;
                return;
            }
            if t.slots[idx].0 == key {
                t.slots[idx].1 = value;
                return;
            }
            idx += 1;
            if idx == sc {
                idx = 0;
            }
        }
        // Probe window full with no match: wraparound overwrite of the HOME
        // slot (no clear-all pause). The victim becomes a future miss — for
        // a pure-function memo that is a recompute, never a wrong value.
        t.slots[(key & (sc - 1) as u64) as usize] = (key, value);
    }

    pub fn len(&self) -> usize {
        self.table.borrow().len
    }

    pub fn is_empty(&self) -> bool {
        self.table.borrow().len == 0
    }
}

impl Default for TileCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Key mix: node structural hash + world column + the (spec, seed) epoch.
/// fnv1a-style mixing so distinct (node, x, z) triples collide with
/// probability ~2^-64 per pair (same hash family the Arena interning uses
/// for structural dedup correctness).
#[inline]
pub fn tile_key(subtree_hash: u64, x: i32, z: i32, epoch: u64) -> u64 {
    let mut h = epoch ^ 0xcbf29ce484222325;
    let mut mix = |v: u64| {
        h ^= v;
        h = h.wrapping_mul(0x100000001b3);
    };
    mix(subtree_hash);
    mix((x as u64) & 0xffff_ffff);
    mix((z as u64) & 0xffff_ffff);
    h
}

/// Epoch for a RandomState: spec_hash ^ level_seed mix.
#[inline]
pub fn tile_epoch(spec_hash: u64, level_seed: i64) -> u64 {
    let mut h = spec_hash ^ 0x9e3779b97f4a7c15;
    h ^= (level_seed as u64).wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 29;
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test constructor with a small power-of-two table (no 67 MB default).
    fn cache_with_slots(slots: usize) -> TileCache {
        TileCache {
            table: RefCell::new(Table { slots: Vec::new(), occupied: Vec::new(), len: 0 }),
            slot_count: slots,
            enabled: true,
            hits: 0.into(),
            misses: 0.into(),
            clears: 0.into(),
        }
    }

    #[test]
    fn put_get_roundtrip_bit_exact() {
        let c = TileCache::new();
        let k = tile_key(12345, 100, -200, 999);
        let v = f64::from_bits(0x7fe8_0000_dead_beef); // arbitrary bit pattern
        assert!(c.get(k).is_none());
        c.put(k, v);
        assert_eq!(c.get(k).unwrap().to_bits(), v.to_bits());
        assert_eq!(c.hits.get(), 1);
        assert_eq!(c.misses.get(), 1);
    }

    #[test]
    fn distinct_keys_do_not_collide() {
        let c = TileCache::new();
        let k1 = tile_key(1, 1, 1, 1);
        let k2 = tile_key(2, 1, 1, 1);
        let k3 = tile_key(1, 1, 2, 1);
        c.put(k1, 1.0);
        c.put(k2, 2.0);
        c.put(k3, 3.0);
        assert_eq!(c.get(k1).unwrap(), 1.0);
        assert_eq!(c.get(k2).unwrap(), 2.0);
        assert_eq!(c.get(k3).unwrap(), 3.0);
    }

    #[test]
    fn key_zero_is_a_legal_key() {
        // fnv1a CAN emit 0 — the bitmap (not a key==0 sentinel) must make it
        // a first-class key.
        let c = cache_with_slots(64);
        let v = f64::from_bits(0x8000_0000_0000_0001);
        assert!(c.get(0).is_none());
        c.put(0, v);
        assert_eq!(c.get(0).unwrap().to_bits(), v.to_bits());
        assert_eq!(c.len(), 1);
        // and key 0 must not shadow or be shadowed by the slot's zero fill
        c.put(64, 1.0); // different key, home slot 0 in a 64-slot table
        assert_eq!(c.get(0).unwrap().to_bits(), v.to_bits());
        assert_eq!(c.get(64).unwrap(), 1.0);
    }

    #[test]
    fn collision_chains_wrap_and_stay_correct() {
        // All keys share ONE home slot (low 6 bits equal) in a 64-slot table:
        // inserts chain forward through the window; lookups must reproduce
        // every stored value until the window is exhausted.
        let c = cache_with_slots(64);
        let base = 0x1234_5678_9abc_def0u64 & !63u64;
        for i in 0..64u64 {
            let k = base ^ (i << 6);
            c.put(k, i as f64);
        }
        assert_eq!(c.len(), 64);
        for i in 0..64u64 {
            let k = base ^ (i << 6);
            assert_eq!(c.get(k).unwrap(), i as f64, "key {i} lost in chain");
        }
        assert_eq!(c.misses.get(), 0);
    }

    #[test]
    fn full_window_overwrites_home_no_clear_pause() {
        // 70 distinct keys with the same home in a 64-slot table: the probe
        // window fills, then puts wrap around and overwrite the home slot.
        // No clear-all (clears stays 0), never more than 64 entries, latest
        // write readable, evicted keys miss.
        let c = cache_with_slots(64);
        let base = 0x0fed_cba9_8765_4320u64 & !63u64;
        let key = |i: u64| base ^ (i << 6);
        for i in 0..70u64 {
            c.put(key(i), i as f64);
        }
        assert_eq!(c.clears.get(), 0);
        assert_eq!(c.len(), 64);
        assert_eq!(c.get(key(69)).unwrap(), 69.0); // latest write wins
        assert!(c.get(key(0)).is_none()); // first victim (home slot overwritten)
        assert!(c.get(key(64)).is_none()); // later victims (each evicted by the next put)
        for i in 1..64u64 {
            assert_eq!(c.get(key(i)).unwrap(), i as f64, "key {i} should survive");
        }
        // a fresh put into a full table still lands (overwrite) and reads back
        c.put(key(69), -1.0);
        assert_eq!(c.get(key(69)).unwrap(), -1.0);
        assert_eq!(c.len(), 64);
    }

    #[test]
    fn determinism_across_two_runs_same_key_sequence() {
        // Same key->value sequence through two fresh tables must produce
        // identical observable state (values AND hit/miss pattern) — the
        // scheme is fully deterministic (no randomness, no iteration order).
        let base = 0x00c0_ffee_0000_0000u64;
        let mut expected: Vec<(u64, Option<u64>)> = Vec::new();
        {
            let c = cache_with_slots(256);
            for i in 0..400u64 {
                let k = base ^ (i.wrapping_mul(0x9e37_79b9_7f4a_7c15));
                c.put(k, (i as f64) + 0.5);
                if i % 3 == 0 {
                    c.get(k);
                }
                if i % 5 == 0 {
                    c.get(base ^ 0xdead_beef); // mostly-miss probes
                }
            }
            for i in 0..400u64 {
                let k = base ^ (i.wrapping_mul(0x9e37_79b9_7f4a_7c15));
                expected.push((k, c.get(k).map(f64::to_bits)));
            }
        }
        let (h1, m1) = {
            let c = cache_with_slots(256);
            for i in 0..400u64 {
                let k = base ^ (i.wrapping_mul(0x9e37_79b9_7f4a_7c15));
                c.put(k, (i as f64) + 0.5);
                if i % 3 == 0 {
                    c.get(k);
                }
                if i % 5 == 0 {
                    c.get(base ^ 0xdead_beef);
                }
            }
            for (k, v) in &expected {
                assert_eq!(c.get(*k).map(f64::to_bits), *v, "replay diverged at key {k:#x}");
            }
            (c.hits.get(), c.misses.get())
        };
        // run 3: exact replay again — same snapshot AND same counters
        let c = cache_with_slots(256);
        for i in 0..400u64 {
            let k = base ^ (i.wrapping_mul(0x9e37_79b9_7f4a_7c15));
            c.put(k, (i as f64) + 0.5);
            if i % 3 == 0 {
                c.get(k);
            }
            if i % 5 == 0 {
                c.get(base ^ 0xdead_beef);
            }
        }
        for (k, v) in &expected {
            assert_eq!(c.get(*k).map(f64::to_bits), *v);
        }
        assert_eq!((c.hits.get(), c.misses.get()), (h1, m1));
    }

    #[test]
    fn update_in_place_same_key() {
        let c = cache_with_slots(64);
        c.put(7, 1.0);
        c.put(7, 2.0);
        assert_eq!(c.get(7).unwrap(), 2.0);
        assert_eq!(c.len(), 1);
        assert_eq!(c.hits.get(), 1);
    }

    #[test]
    fn disabled_cache_is_transparent() {
        let mut c = TileCache::new();
        c.enabled = false;
        c.put(42, 1.0);
        assert!(c.get(42).is_none());
        assert!(c.is_empty());
    }
}
