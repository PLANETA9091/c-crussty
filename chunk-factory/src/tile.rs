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
//! raw f64 (no rounding); the HashMap is keyed by a 64-bit structural mix.
//!
//! Cross-chunk win: slice/corner fills re-evaluate the same y-free chains
//! (shift noises, terrain-shaper spline coordinates) for every y corner of a
//! column AND again in the neighboring chunk's border columns. The tile cache
//! collapses both: O(1) per world column per node shape.

use std::cell::RefCell;
use std::collections::HashMap;

const DEFAULT_CAP: usize = 2_000_000;

pub struct TileCache {
    map: RefCell<HashMap<u64, f64>>,
    cap: usize,
    pub enabled: bool,
    pub hits: std::cell::Cell<u64>,
    pub misses: std::cell::Cell<u64>,
    pub clears: std::cell::Cell<u64>,
}

impl TileCache {
    pub fn new() -> Self {
        let enabled = std::env::var("NCF_TILE_CACHE").map(|v| v != "0").unwrap_or(true);
        let cap = std::env::var("NCF_TILE_CAP").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_CAP);
        TileCache { map: RefCell::new(HashMap::new()), cap, enabled, hits: 0.into(), misses: 0.into(), clears: 0.into() }
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<f64> {
        if !self.enabled {
            return None;
        }
        let m = self.map.borrow();
        // hit/miss counters only on the fast path (Cell, no borrow clash)
        match m.get(&key) {
            Some(v) => {
                self.hits.set(self.hits.get() + 1);
                Some(*v)
            }
            None => {
                self.misses.set(self.misses.get() + 1);
                None
            }
        }
    }

    #[inline]
    pub fn put(&self, key: u64, value: f64) {
        if !self.enabled {
            return;
        }
        let mut m = self.map.borrow_mut();
        if m.len() >= self.cap {
            m.clear();
            self.clears.set(self.clears.get() + 1);
        }
        m.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.map.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.borrow().is_empty()
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
    fn cap_triggers_clear_and_stays_correct() {
        let mut c = TileCache::new();
        c.cap = 8;
        for i in 0..64u64 {
            let k = tile_key(i, i as i32, 0, 7);
            c.put(k, i as f64);
        }
        assert_eq!(c.clears.get() >= 1, true);
        // last writes still readable
        let k = tile_key(63, 63, 0, 7);
        assert_eq!(c.get(k).unwrap(), 63.0);
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
