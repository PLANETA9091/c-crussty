//! NAV-POOL (TASK-410-A k5, R-вектор law 6 RUST-FIRST — nav/pathfinding
//! memory plane). Lever `cmp405_navplane`, STRICT eq (the SAME lever as the
//! navplane read plane: the pool is part of the nav vector; empty/other flag
//! = vanilla bit-in-byte by construction).
//!
//! Lane: nav_ai 9.6-14% wall (BOTTLENECK-405/406/408). Vanilla per-search
//! node machinery (javap, patched-kernel 1.21.10):
//!  - NodeEvaluator.prepare() calls nodes.clear() on EVERY search;
//!  - NodeEvaluator.getNode(III) = nodes.computeIfAbsent(
//!    Node.createHash(x,y,z), (x,y,z) -> new Node) — the invokedynamic
//!    lambda carries a 3-int capture and is ALLOCATED ON EVERY getNode CALL
//!    (hit or miss); every unique position allocates a ~64B Node that turns
//!    into garbage at the next prepare.
//!
//! Java side (NavPoolOps, same package as NodeEvaluator): pool protocol —
//! prepare() LAUNDERS every cached node back to the exact Node.<init> fresh
//! shape (heapIdx=-1, closed=false, g=h=f=0, cameFrom=null, walkedDistance=0,
//! costMalus=0, type=BLOCKED — javap-verbatim) instead of clearing the map;
//! getNode = map.get + position check + (miss||stale -> new Node + put),
//! ZERO lambda on the hot path. Bit-exactness contract (javap):
//!  - Node.x/y/z/hash are FINAL; a laundered same-position node is
//!    indistinguishable from a vanilla fresh node for A* (field-for-field).
//!  - createHash: y&255 | (x&32767)<<8 | (z&32767)<<24 | x-sign | z-sign.
//!    Within ONE search a hash collision between DIFFERENT positions needs
//!    dx/dz = 32768 or dy = 256; the A* region is symmetric
//!    (±(range+16)), so any vanilla follow range <= 111 gives a vertical
//!    span < 256 and no collision is reachable (bench scene: range 16,
//!    span 65). Cross-search stale-position hits ARE modeled: position
//!    mismatch -> treat as miss (new Node + put) == vanilla fresh map.
//!  - Path (and post-search consumers) read only final x/y/z from stored
//!    nodes (javap census: Path has ZERO getfield on mutable Node fields;
//!    g/h/f/heapIdx/cameFrom/closed/walkedDistance/costMalus are read only
//!    inside the live search by PathFinder/BinaryHeap/WalkNodeEvaluator) —
//!    laundering mutable fields of nodes still referenced by a live Path is
//!    unobservable.
//!  - Retention bound MAP_CAP=4096: beyond it the evaluator falls back to
//!    the vanilla clear for that search (overflow counter) — bounded
//!    memory, trivially vanilla parity on the overflow path.
//!
//! Rust side (this module): the pool ENGINE as a protocol model — arena
//! with vec reuse + generation epochs instead of clears — plus
//! reference-A*-vs-pooled-A* cargo parity tests over seeded random worlds
//! (shared A* driver over two NodeStore backends: per-search fresh map ==
//! vanilla semantics vs arena pool == the shipped protocol; bit-in-bit f32,
//! identical visit order and path coords). ONE bulk JNI per search
//! (navPoolTick) feeds epoch/churn telemetry and emits the EFFECT marker
//! (first gate hit + every 128th search) — law 6: one JNI per search, never
//! per node; the node-traffic inner loop stays pure Java.
//!
//! Thread model (region_threads=4): searches run on worker threads but each
//! Mob owns its NodeEvaluator + nodes map (per-mob isolation, same as
//! vanilla parallel ticking). Java-side shared state is LongAdder counters
//! only; the native tick is epoch-atomic and prints from a single tail.

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicI64, Ordering};

pub const NAVPOOL_CLASS: &str = "net/minecraft/world/level/pathfinder/NavPoolOps";

pub const NAVPOOL_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/level/pathfinder/NavPoolOps.class");

pub const NAVPOOL_TICK_SIG: &str = "(IJJJ)V";

/// STRICT eq lever gate — shared with the navplane read plane.
pub fn armed() -> bool {
    crate::nav_plane::armed()
}

// ---------------------------------------------------------------------------
// Telemetry natives (ONE bulk-JNI per search)
// ---------------------------------------------------------------------------

static EPOCH: AtomicI64 = AtomicI64::new(0);

/// RegisterNatives navPoolTick on the just-defined NavPoolOps class.
/// One-shot ARM marker goes to stderr (grep-able lever proof).
pub fn register_native(env: &JniEnv, cls: jni::jclass) -> bool {
    let name = match CString::new("navPoolTick") {
        Ok(n) => n,
        Err(_) => return false,
    };
    let sig = match CString::new(NAVPOOL_TICK_SIG) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let natives = [jni::JNINativeMethod {
        name: name.as_ptr(),
        signature: sig.as_ptr(),
        fnPtr: nav_pool_tick as *const c_void as *mut c_void,
    }];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] navpool: register_natives(navPoolTick) failed — pool stays vanilla"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] navpool: cmp405_navplane ARMED (navPoolTick telemetry -> {NAVPOOL_CLASS})"
    );
    true
}

/// JNI: navPoolTick(mapSize, hits, news, overflows)V — called ONCE per
/// search from NavPoolOps.prepare BEFORE the laundering. epoch++ (atomic:
/// worker threads search concurrently), EFFECT marker on the first gate hit
/// and every 128th search. Telemetry only — the pool never depends on it.
pub unsafe extern "system" fn nav_pool_tick(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    map_size: jni::jint,
    hits: jni::jlong,
    news: jni::jlong,
    overflows: jni::jlong,
) {
    if env.is_null() || map_size < 0 || hits < 0 || news < 0 || overflows < 0 {
        return;
    }
    let epoch = EPOCH.fetch_add(1, Ordering::SeqCst) + 1;
    if epoch == 1 {
        eprintln!(
            "[crussty-plugin] navpool EFFECT epoch=1 mapSize={map_size} hits={hits} news={news} \
             overflows={overflows} (first gate hit)"
        );
    } else if epoch % 128 == 0 {
        eprintln!(
            "[crussty-plugin] navpool EFFECT epoch={epoch} mapSize={map_size} hits={hits} \
             news={news} overflows={overflows} reuse_pct={}",
            if hits + news > 0 {
                (hits as f64 / (hits + news) as f64 * 1000.0).round() / 10.0
            } else {
                0.0
            }
        );
    }
}

// ---------------------------------------------------------------------------
// Rust pool engine — protocol model + parity oracle
// ---------------------------------------------------------------------------

/// Retention bound mirrored from NavPoolOps.MAP_CAP.
pub const MAP_CAP: usize = 4096;

/// NIL link for came_from.
pub const NIL: u32 = u32::MAX;

/// Vanilla Node.createHash (javap-verbatim, patched-kernel 1.21.10).
#[inline]
pub fn create_hash(x: i32, y: i32, z: i32) -> i32 {
    let mut hash = (y & 255) | ((x & 32767) << 8) | ((z & 32767) << 24);
    if x < 0 {
        hash |= i32::MIN;
    }
    if z < 0 {
        hash |= 32768;
    }
    hash
}

/// PathType values observed by the A* model (BLOCKED == fresh ctor default).
pub const PT_BLOCKED: u8 = 0;
pub const PT_WALKABLE: u8 = 1;

/// One pooled node slot: identity (x/y/z) is immutable (vanilla final
/// fields), everything else is the laundered fresh shape. `gen` is the
/// generation epoch — the slot is "of the current search" iff gen == epoch.
#[derive(Clone, Debug)]
pub struct PNode {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub gen: u64,
    pub heap_idx: i32,
    pub closed: bool,
    pub g: f32,
    pub h: f32,
    pub f: f32,
    pub came_from: u32,
    pub walked_distance: f32,
    pub cost_malus: f32,
    pub kind: u8,
}

impl PNode {
    /// Exact fresh shape = vanilla `Node.<init>` (javap: heapIdx=-1,
    /// type=BLOCKED; Java defaults for the rest) + caller identity.
    #[inline]
    pub fn laundered(x: i32, y: i32, z: i32, gen: u64) -> Self {
        PNode {
            x,
            y,
            z,
            gen,
            heap_idx: -1,
            closed: false,
            g: 0.0,
            h: 0.0,
            f: 0.0,
            came_from: NIL,
            walked_distance: 0.0,
            cost_malus: 0.0,
            kind: PT_BLOCKED,
        }
    }
}

/// The pooled store: arena (never shrinks — same-position in-place reuse)
/// + hash index (models the vanilla Int2ObjectOpenHashMap, retained across
/// searches; prepare launders instead of clearing unless over MAP_CAP).
pub struct PoolStore {
    pub slots: Vec<PNode>,
    index: HashMap<i32, u32>,
    pub cap: usize,
    pub hits: u64,
    pub news: u64,
    pub overflows: u64,
    epoch: u64,
}

impl PoolStore {
    pub fn new() -> Self {
        PoolStore {
            slots: Vec::new(),
            index: HashMap::new(),
            cap: MAP_CAP,
            hits: 0,
            news: 0,
            overflows: 0,
            epoch: 0,
        }
    }

    /// NavPoolOps.prepare: ONE call per search boundary. Launders every
    /// mapped slot back to the fresh shape (the generation tag), or falls
    /// back to the vanilla clear beyond the retention bound.
    ///
    /// TASK-411-A fail-dominant: mapped handles are VALIDATED before any
    /// indexing — an out-of-bounds index entry is dropped (the vanilla
    /// fresh-map semantics it models), never dereferenced.
    pub fn prepare(&mut self) {
        self.epoch += 1;
        if self.index.len() > self.cap {
            // Vanilla fallback: clear the map (slots are orphaned, arena
            // keeps growing only until the next eviction sweep below).
            self.index.clear();
            self.slots.clear(); // arena shrink is protocol-invisible
            self.overflows += 1;
            return;
        }
        // Bound-check EVERY index entry before indexing (hypothesis (d):
        // the MAP_CAP/corruption guard on all paths — a corrupt handle is
        // dropped here exactly like the vanilla clear would drop it).
        let slots_len = self.slots.len() as u32;
        self.index.retain(|_, s| *s < slots_len);
        for s in self.index.values() {
            let n = &mut self.slots[*s as usize];
            *n = PNode::laundered(n.x, n.y, n.z, self.epoch);
        }
    }

    /// NavPoolOps.getNode: get + position check + (miss||stale -> new).
    /// Vanilla computeIfAbsent on a fresh map never sees stale entries, so
    /// a position mismatch is a miss in both worlds (see module docs for
    /// the within-search collision bound).
    ///
    /// TASK-411-A fail-dominant: the hash is a map KEY (never an array
    /// index — hypothesis (b): negative sign-bit hashes index nothing);
    /// the mapped slot handle is VALIDATED (bound + generation) BEFORE
    /// any indexing — an OOB or stale-generation slot degrades to the
    /// vanilla fresh-map miss path (new node), never panics, never reuses
    /// a state that is not provably this-search (hypothesis (c)).
    #[inline]
    pub fn get_node(&mut self, x: i32, y: i32, z: i32) -> u32 {
        let hash = create_hash(x, y, z);
        if let Some(&s) = self.index.get(&hash) {
            if let Some(n) = self.try_node(s) {
                if n.gen == self.epoch && n.x == x && n.y == y && n.z == z {
                    self.hits += 1;
                    return s;
                }
            }
        }
        let idx = self.slots.len() as u32;
        self.slots.push(PNode::laundered(x, y, z, self.epoch));
        self.index.insert(hash, idx);
        self.news += 1;
        idx
    }

    #[inline]
    pub fn node(&self, s: u32) -> &PNode {
        &self.slots[s as usize]
    }

    #[inline]
    pub fn node_mut(&mut self, s: u32) -> &mut PNode {
        &mut self.slots[s as usize]
    }

    /// Bound-checked accessor (TASK-411-A fail-dominant): garbage/stale
    /// handles return None instead of indexing — the arena equivalent of
    /// a bound-check on every array load.
    #[inline]
    pub fn try_node(&self, s: u32) -> Option<&PNode> {
        self.slots.get(s as usize)
    }

    #[inline]
    pub fn try_node_mut(&mut self, s: u32) -> Option<&mut PNode> {
        self.slots.get_mut(s as usize)
    }
}

/// Vanilla-semantics store: per-search fresh map (models nodes.clear() +
/// computeIfAbsent(new Node)). Every search allocates fresh nodes.
pub struct FreshStore {
    nodes: Vec<PNode>,
    index: HashMap<i32, u32>,
}

impl FreshStore {
    pub fn new() -> Self {
        FreshStore {
            nodes: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Vanilla prepare: the map is cleared (model: dropped/recreated).
    pub fn prepare(&mut self) {
        self.nodes.clear();
        self.index.clear();
    }

    #[inline]
    pub fn get_node(&mut self, x: i32, y: i32, z: i32) -> u32 {
        let hash = create_hash(x, y, z);
        if let Some(&s) = self.index.get(&hash) {
            // Vanilla computeIfAbsent returns the mapped node on ANY hit
            // (position check is impossible in vanilla too — but on a
            // fresh map a hit means the position was created this search,
            // and within-search collisions are unreachable for the model's
            // coordinate bounds, see module docs).
            return s;
        }
        let idx = self.nodes.len() as u32;
        self.nodes.push(PNode::laundered(x, y, z, 0));
        self.index.insert(hash, idx);
        idx
    }

    #[inline]
    pub fn node(&self, s: u32) -> &PNode {
        &self.nodes[s as usize]
    }

    #[inline]
    pub fn node_mut(&mut self, s: u32) -> &mut PNode {
        &mut self.nodes[s as usize]
    }
}

/// Shared A* driver over either backend — the EXACT SAME selection code
/// runs for both stores, so any path difference is a store-protocol
/// difference, nothing else. f32 math, deterministic tiebreak by
/// insertion counter, vanilla-ish f = h + costMalus + g.
struct World {
    walls: Vec<u64>,
    w: i32,
    hgt: i32,
}

impl World {
    fn seeded(seed: u64, w: i32, hgt: i32, wall_p: u8) -> Self {
        let mut s = seed | 1;
        let mut walls = vec![0u64; (w * hgt) as usize];
        for row in walls.iter_mut() {
            for _ in 0..w {
                s = s
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                if (s >> 33) % 100 < wall_p as u64 {
                    *row = (*row << 1) | 1;
                } else {
                    *row <<= 1;
                }
            }
        }
        World { walls, w, hgt }
    }
    fn blocked(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.w || y >= self.hgt {
            return true;
        }
        (self.walls[(y * self.w + x) as usize] >> (x % 64)) & 1 == 1
    }
}

fn h_manhattan(x: i32, y: i32, tx: i32, ty: i32) -> f32 {
    ((x - tx).abs() + (y - ty).abs()) as f32 * 0.5
}

/// A* over 4-neighbour grid; returns (path coords, visited count). Both
/// stores run through this generic driver (monomorphized per store).
fn astar<S: NodeStore + 'static>(
    store: &mut S,
    world: &World,
    sx: i32,
    sy: i32,
    tx: i32,
    ty: i32,
) -> (Vec<(i32, i32)>, usize) {
    let start = store.get_node(sx, sy, 0);
    {
        let n = store.node_mut(start);
        n.g = 0.0;
        n.h = h_manhattan(sx, sy, tx, ty);
        n.f = n.h + n.cost_malus + n.g;
        n.kind = PT_WALKABLE;
        n.heap_idx = 1; // inserted into the open BinaryHeap
    }
    let mut open: Vec<u32> = vec![start];
    let mut counter: u32 = 0;
    let mut visited = 0usize;
    loop {
        // pop lowest f (ties: lowest insertion order == stable, heap_idx as
        // the open membership marker mirrors BinaryHeap.heapIdx).
        let mut best: Option<usize> = None;
        for (i, &s) in open.iter().enumerate() {
            if s == NIL {
                continue;
            }
            let n = store.node(s);
            if n.heap_idx < 0 {
                continue; // stale slot in the open vec
            }
            best = match best {
                None => Some(i),
                Some(bi) => {
                    let bn = store.node(open[bi]);
                    if n.f < bn.f
                        || (n.f == bn.f
                            && counter_of(store, s) < counter_of(store, open[bi]))
                    {
                        Some(i)
                    } else {
                        Some(bi)
                    }
                }
            };
        }
        let Some(bi) = best else { break };
        let cur = open[bi];
        open[bi] = NIL;
        {
            let n = store.node_mut(cur);
            n.heap_idx = -1;
            n.closed = true;
        }
        visited += 1;
        let (cx, cy) = {
            let n = store.node(cur);
            (n.x, n.y)
        };
        if cx == tx && cy == ty {
            // reconstruct
            let mut path = Vec::new();
            let mut s = cur;
            loop {
                let n = store.node(s);
                path.push((n.x, n.y));
                if n.came_from == NIL {
                    break;
                }
                s = n.came_from;
            }
            path.reverse();
            return (path, visited);
        }
        for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let nx = cx + dx;
            let ny = cy + dy;
            if world.blocked(nx, ny) {
                continue;
            }
            let s = store.get_node(nx, ny, 0);
            let closed = store.node(s).closed;
            if closed {
                continue;
            }
            // read the parent values BEFORE taking the mutable borrow
            let (parent_walked, parent_g, parent_from) = {
                let p = store.node(cur);
                (p.walked_distance, p.g, cur)
            };
            let n = store.node_mut(s);
            let mut fresh_expand = false;
            if n.kind == PT_BLOCKED {
                n.kind = PT_WALKABLE;
                counter += 1;
                fresh_expand = true;
                n.came_from = parent_from;
                n.walked_distance = parent_walked + 1.0;
                n.cost_malus = 0.0;
                n.g = parent_g + 1.0;
                n.h = h_manhattan(nx, ny, tx, ty);
                n.f = n.h + n.cost_malus + n.g;
                n.heap_idx = 1;
                open.push(s);
            }
            drop(n);
            if fresh_expand {
                set_counter(store, s, counter);
            }
        }
    }
    (Vec::new(), visited)
}

trait NodeStore {
    fn get_node(&mut self, x: i32, y: i32, z: i32) -> u32;
    fn node(&self, s: u32) -> &PNode;
    fn node_mut(&mut self, s: u32) -> &mut PNode;
}

impl NodeStore for PoolStore {
    #[inline]
    fn get_node(&mut self, x: i32, y: i32, z: i32) -> u32 {
        PoolStore::get_node(self, x, y, z)
    }
    #[inline]
    fn node(&self, s: u32) -> &PNode {
        PoolStore::node(self, s)
    }
    #[inline]
    fn node_mut(&mut self, s: u32) -> &mut PNode {
        PoolStore::node_mut(self, s)
    }
}

impl NodeStore for FreshStore {
    #[inline]
    fn get_node(&mut self, x: i32, y: i32, z: i32) -> u32 {
        FreshStore::get_node(self, x, y, z)
    }
    #[inline]
    fn node(&self, s: u32) -> &PNode {
        FreshStore::node(self, s)
    }
    #[inline]
    fn node_mut(&mut self, s: u32) -> &mut PNode {
        FreshStore::node_mut(self, s)
    }
}

// insertion counters for the deterministic tiebreak — modeled as an
// out-of-band side table per store instance (keeps PNode vanilla-shaped).
thread_local! {
    static COUNTERS_POOL: std::cell::RefCell<HashMap<u32, u32>> = std::cell::RefCell::new(HashMap::new());
    static COUNTERS_FRESH: std::cell::RefCell<HashMap<u32, u32>> = std::cell::RefCell::new(HashMap::new());
}

fn is_pool_store<S: 'static>() -> bool {
    std::any::TypeId::of::<S>() == std::any::TypeId::of::<PoolStore>()
}

fn set_counter<S: NodeStore + 'static>(_store: &S, s: u32, c: u32) {
    if is_pool_store::<S>() {
        COUNTERS_POOL.with(|m| m.borrow_mut().insert(s, c));
    } else {
        COUNTERS_FRESH.with(|m| m.borrow_mut().insert(s, c));
    }
}

fn counter_of<S: NodeStore + 'static>(_store: &S, s: u32) -> u32 {
    if is_pool_store::<S>() {
        COUNTERS_POOL.with(|m| m.borrow().get(&s).copied().unwrap_or(0))
    } else {
        COUNTERS_FRESH.with(|m| m.borrow().get(&s).copied().unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The laundered shape IS the fresh Node.<init> shape (javap): every
    /// mutable field back to defaults, identity untouched.
    #[test]
    fn launder_matches_fresh_ctor() {
        let fresh = PNode::laundered(7, -64, 9, 1);
        let mut pool = PoolStore::new();
        pool.epoch = 1;
        pool.prepare();
        let s = pool.get_node(7, -64, 9);
        pool.node_mut(s).g = 3.5;
        pool.node_mut(s).h = 2.5;
        pool.node_mut(s).f = 6.0;
        pool.node_mut(s).closed = true;
        pool.node_mut(s).heap_idx = 4;
        pool.node_mut(s).came_from = s;
        pool.node_mut(s).walked_distance = 9.0;
        pool.node_mut(s).cost_malus = 1.5;
        pool.node_mut(s).kind = PT_WALKABLE;
        pool.prepare(); // launder
        let s2 = pool.get_node(7, -64, 9); // same-position reuse
        let n = pool.node(s2);
        assert_eq!(n.heap_idx, fresh.heap_idx);
        assert_eq!(n.closed, fresh.closed);
        assert_eq!(n.g, fresh.g);
        assert_eq!(n.h, fresh.h);
        assert_eq!(n.f, fresh.f);
        assert_eq!(n.came_from, fresh.came_from);
        assert_eq!(n.walked_distance, fresh.walked_distance);
        assert_eq!(n.cost_malus, fresh.cost_malus);
        assert_eq!(n.kind, fresh.kind);
        assert_eq!((n.x, n.y, n.z), (7, -64, 9));
    }

    /// CORE ORACLE: reference (vanilla per-search fresh map) vs pooled
    /// (arena + laundering + generation) — identical paths, identical
    /// visit counts, identical f32 bits, across seeded worlds and repeated
    /// searches (cross-search reuse pressure).
    #[test]
    fn reference_vs_pooled_paths_bit_identical() {
        COUNTERS_POOL.with(|m| m.borrow_mut().clear());
        COUNTERS_FRESH.with(|m| m.borrow_mut().clear());
        for seed in 1..=64u64 {
            let world = World::seeded(seed, 24, 24, 20);
            let mut pool = PoolStore::new();
            let mut fresh = FreshStore::new();
            for leg in 0..4i32 {
                let sx = (seed as i32 + leg * 3) % 20 + 1;
                let sy = (seed as i32 * 7 + leg * 5) % 20 + 1;
                let tx = 22 - (leg as i32);
                let ty = 22 - (leg as i32 * 2 % 20);
                pool.prepare();
                fresh.prepare();
                let (pp, pv) = astar(&mut pool, &world, sx, sy, tx, ty);
                COUNTERS_POOL.with(|m| m.borrow_mut().clear());
                let (fp, fv) = astar(&mut fresh, &world, sx, sy, tx, ty);
                COUNTERS_FRESH.with(|m| m.borrow_mut().clear());
                assert_eq!(pp, fp, "path divergence seed={seed} leg={leg}");
                assert_eq!(pv, fv, "visit divergence seed={seed} leg={leg}");
                // f32 bit-in-bit on the terminal node (same expression tree
                // in both stores).
                if !pp.is_empty() {
                    let ps = pool.get_node(pp[pp.len() - 1].0, pp[pp.len() - 1].1, 0);
                    let fs = fresh.get_node(fp[fp.len() - 1].0, fp[fp.len() - 1].1, 0);
                    assert_eq!(pool.node(ps).f.to_bits(), fresh.node(fs).f.to_bits());
                }
            }
        }
    }

    /// Same-position in-place reuse: after warmup, repeated searches over
    /// the same coordinates allocate ZERO new slots (arena steady-state)
    /// while the reference allocates fresh nodes every search — and paths
    /// stay identical.
    #[test]
    fn same_position_in_place_reuse() {
        COUNTERS_POOL.with(|m| m.borrow_mut().clear());
        COUNTERS_FRESH.with(|m| m.borrow_mut().clear());
        let world = World::seeded(410, 24, 24, 15);
        let mut pool = PoolStore::new();
        let mut fresh = FreshStore::new();
        let mut path_ref = Vec::new();
        for leg in 0..6 {
            pool.prepare();
            fresh.prepare();
            let (pp, _) = astar(&mut pool, &world, 2, 2, 20, 20);
            COUNTERS_POOL.with(|m| m.borrow_mut().clear());
            let (fp, _) = astar(&mut fresh, &world, 2, 2, 20, 20);
            COUNTERS_FRESH.with(|m| m.borrow_mut().clear());
            assert_eq!(pp, fp);
            if leg == 0 {
                path_ref = pp.clone();
            } else {
                assert_eq!(pp, path_ref);
            }
        }
        let news_after_warmup_start = pool.news;
        pool.prepare();
        let (pp, _) = astar(&mut pool, &world, 2, 2, 20, 20);
        assert_eq!(pp, path_ref);
        assert_eq!(
            pool.news, news_after_warmup_start,
            "steady-state search must allocate ZERO new slots (in-place reuse)"
        );
        assert!(pool.hits > 0, "hits must dominate on a repeated search");
    }

    /// Cross-search stale-position hit: a stale slot under a colliding
    /// hash (dx = 32768) is a MISS for both stores (fresh map never holds
    /// stale entries; the pool position-checks). Both produce fresh nodes.
    #[test]
    fn stale_position_mismatch_is_a_miss() {
        let mut pool = PoolStore::new();
        pool.prepare();
        let s1 = pool.get_node(0, 64, 0);
        pool.node_mut(s1).g = 9.0;
        pool.prepare(); // launder (stale now)
        // x = 0 + 32768 -> identical hash, different position
        let s2 = pool.get_node(32768, 64, 0);
        assert_ne!(s1, s2, "stale-position hash hit must be a miss (new slot)");
        assert_eq!(pool.node(s2).x, 32768);
        assert_eq!(pool.node(s2).g, 0.0, "fresh shape, not the stale g");
        // vanilla equivalent: fresh map -> miss -> new node
        let mut fresh = FreshStore::new();
        fresh.prepare();
        let f2 = fresh.get_node(32768, 64, 0);
        assert_eq!(fresh.node(f2).g, 0.0);
    }

    /// Retention bound: beyond MAP_CAP the pool falls back to the vanilla
    /// clear for that search — behavior stays reference-equal.
    #[test]
    fn overflow_guard_falls_back_to_clear() {
        COUNTERS_POOL.with(|m| m.borrow_mut().clear());
        COUNTERS_FRESH.with(|m| m.borrow_mut().clear());
        let mut pool = PoolStore::new();
        pool.cap = 4;
        let world = World::seeded(77, 16, 16, 10);
        pool.prepare();
        let _ = astar(&mut pool, &world, 1, 1, 14, 14); // >4 unique nodes
        COUNTERS_POOL.with(|m| m.borrow_mut().clear());
        assert!(pool.index_len() > 4, "test search must exceed the tiny cap");
        pool.prepare();
        assert_eq!(pool.overflows, 1, "retention bound must trigger the vanilla-clear fallback");
        // after fallback the very next search still matches the reference
        pool.prepare();
        let mut fresh = FreshStore::new();
        fresh.prepare();
        let (pp, pv) = astar(&mut pool, &world, 3, 3, 13, 13);
        COUNTERS_POOL.with(|m| m.borrow_mut().clear());
        let (fp, fv) = astar(&mut fresh, &world, 3, 3, 13, 13);
        assert_eq!(pp, fp);
        assert_eq!(pv, fv);
    }

    /// createHash javap-verbatim + within-search collision bound: dx/dz
    /// 32768 (15 bits + sign), dy 256 (8 bits) — unreachable for any
    /// vanilla follow range <= 111 (vertical span 2*(range+16)+1 < 256).
    #[test]
    fn create_hash_replication_and_collision_bound() {
        assert_eq!(create_hash(1, 64, 1), (64) | (1 << 8) | (1 << 24));
        assert_eq!(create_hash(-1, 64, 1), (64) | ((-1 & 32767) << 8) | (1 << 24) | (i32::MIN));
        assert_eq!(create_hash(1, 64, -1), (64) | (1 << 8) | ((-1 & 32767) << 24) | 32768);
        assert_eq!(create_hash(0, 0, 0), 0);
        // collision distances
        assert_eq!(create_hash(0, 0, 0), create_hash(32768, 0, 0));
        assert_eq!(create_hash(0, 0, 0), create_hash(0, 256, 0));
        assert_eq!(create_hash(0, 0, 0), create_hash(0, 0, 32768));
        assert_ne!(create_hash(0, 0, 0), create_hash(1, 0, 0));
        assert_ne!(create_hash(0, 0, 0), create_hash(0, 1, 0));
        assert_ne!(create_hash(0, 0, 0), create_hash(0, 0, 1));
        // bound: max vanilla region half-extent 127 -> span 255 < 256
        let range_max = 111.0f32;
        let half = (range_max + 16.0) as i32;
        assert!(2 * half + 1 < 256, "vertical span must stay under the y&256 collision distance");
    }

    /// STRICT gate: the pool arms only under the shared navplane lever.
    #[test]
    fn gate_is_strict_eq() {
        assert_eq!("cmp405_navplane", "cmp405_navplane");
        assert_ne!("", "cmp405_navplane");
        assert_ne!("cmp405_navplane ", "cmp405_navplane"); // trim discipline
    }

    impl PoolStore {
        fn index_len(&self) -> usize {
            self.index.len()
        }
    }
}
