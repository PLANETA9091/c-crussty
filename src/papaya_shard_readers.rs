//! papaya_shard_readers — ID-H05 scaffold (round-459-h05, STRICT DORMANT).
//!
//! papaya-style lock-free shard-readers for EntitySection arrays with epoch
//! publication. Read path = pin epoch (1 acquire load + 1 reader-counter
//! increment) + acquire-load of the shard snapshot pointer + epoch check.
//! Readers NEVER block and are NEVER blocked; writers publish an immutable
//! replacement snapshot with a release store and retire the old one; retired
//! snapshots are reclaimed only when the reader count is quiesced (EBR,
//! conservative fail-closed: a reader whose pinned epoch lags the published
//! snapshot epoch gets `None` and MUST fall back to the vanilla locked path).
//!
//! Mechanics borrowed from the papaya crate (docs.rs/papaya: "lock-free API,
//! no more deadlocks", Guard/pin read path, read-heavy optimization), the
//! anchored-elements model (github.com/mcrepeau/anchormap: entries never move
//! under concurrent access) and relativistic-programming hash tables
//! (Triplett 2008: readers synchronization-free, updates publish-after-fill).
//! Reclamation is deliberately conservative per VBR (Sheffi 2021): provable
//! lock-freedom is NOT attempted; drift -> silent fallback (fail-closed).
//!
//! Shard = 32x32-chunk region (ChunkSlicesRegion precedent, RESEARCH-459-L11).
//! `Snapshot.sections` carries opaque EntitySection slice ids (u64 keys), NOT
//! raw kernel pointers — this scaffold is DORMANT: nothing wires it into any
//! hot path yet. Parity contract (law 16): emission order must stay identical
//! to vanilla (z,x,y-asc, storage-idx); gates G1-G6 preregistered in
//! RESEARCH-459-H05.md (repo copy: research/RESEARCH-459-H05.md).
//!
//! STRICT DORMANT: no `pub` entry point is referenced outside this module;
//! counters stay at 0 on the live bank until a future wiring task flips the
//! ARM marker (boot marker `papaya_shard_reads ARMED`, fallbacks < 1%).

#![allow(dead_code)]

use std::sync::atomic::{AtomicI32, AtomicPtr, AtomicU64, Ordering};
use std::sync::Mutex;

/// Broadphase shard count: 32x32-chunk regions (L11 precedent).
pub const SHARD_COUNT: usize = 32;

/// Immutable, epoch-stamped snapshot of one shard's EntitySection slice ids.
/// Published once (release store) and never mutated in place afterwards —
/// "anchored" semantics: a snapshot a reader has already loaded stays valid
/// until reclaimed, and reclamation only happens when no reader is active.
pub struct Snapshot {
    pub epoch: u64,
    pub sections: Vec<u64>,
}

impl Snapshot {
    fn new(epoch: u64, sections: Vec<u64>) -> Box<Snapshot> {
        Box::new(Snapshot { epoch, sections })
    }
}

struct Retired {
    /// Publish epoch of the retired snapshot (for diagnostics).
    epoch: u64,
    ptr: *mut Snapshot,
}

struct ShardSlot {
    /// Current published snapshot (null until the first publish).
    snap: AtomicPtr<Snapshot>,
    /// Readers currently inside read_shard() on this slot (pinned before the
    /// pointer load — the invariant that makes quiesced reclamation safe).
    readers: AtomicI32,
    retired: Mutex<Vec<Retired>>,
}

impl ShardSlot {
    pub const fn new() -> ShardSlot {
        ShardSlot {
            snap: AtomicPtr::new(std::ptr::null_mut()),
            readers: AtomicI32::new(0),
            retired: Mutex::new(Vec::new()),
        }
    }
}

// SAFETY: ShardSlot is only accessed through atomics and a Mutex; Snapshot
// objects are immutable after publish and freed exclusively by collect()
// while `readers == 0`, so no concurrent mutable access exists.
unsafe impl Send for ShardSlot {}
unsafe impl Sync for ShardSlot {}

/// Global epoch, bumped by every publish. Readers pin it at entry.
static EPOCH: AtomicU64 = AtomicU64::new(0);
/// Readers currently pinned anywhere in the table (pin precedes pointer load).
static READERS: AtomicI32 = AtomicI32::new(0);

// Effect counters (gate G1 effect markers; all zero while dormant).
static STAT_PINS: AtomicU64 = AtomicU64::new(0);
static STAT_STABLE_READS: AtomicU64 = AtomicU64::new(0);
static STAT_FALLBACKS: AtomicU64 = AtomicU64::new(0);
static STAT_PUBLISHES: AtomicU64 = AtomicU64::new(0);
static STAT_RECLAIMED: AtomicU64 = AtomicU64::new(0);

/// RAII epoch pin: increments the global reader count for its lifetime.
pub struct EpochGuard {
    epoch: u64,
}

impl EpochGuard {
    #[inline]
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

impl Drop for EpochGuard {
    fn drop(&mut self) {
        READERS.fetch_sub(1, Ordering::Release);
    }
}

/// Pin the current epoch (papaya `guard()`/`pin()` analogue). Lock-free:
/// one Acquire load + one fetch_add.
#[must_use = "drop the guard to release the epoch pin"]
#[inline]
pub fn pin() -> EpochGuard {
    // Increment BEFORE latching the epoch so a concurrent publish either
    // completes before our epoch is latched (we then see the new snapshot,
    // drift check handles the mismatch) or after (we keep the old epoch).
    READERS.fetch_add(1, Ordering::AcqRel);
    STAT_PINS.fetch_add(1, Ordering::Relaxed);
    EpochGuard { epoch: EPOCH.load(Ordering::Acquire) }
}

/// Lock-free reader: acquire-load the shard snapshot and validate the epoch.
/// Returns `None` (caller MUST fall back to the vanilla locked path) when
/// the shard has no snapshot yet or the snapshot is newer than the pinned
/// epoch (drift — fail-closed, no partial results).
#[inline]
pub fn read_shard<'a>(
    table: &'a PapayaShardTable,
    shard: usize,
    guard: &EpochGuard,
) -> Option<&'a Snapshot> {
    debug_assert!(shard < SHARD_COUNT);
    if shard >= SHARD_COUNT {
        STAT_FALLBACKS.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    let slot: &ShardSlot = &table.slots[shard];
    slot.readers.fetch_add(1, Ordering::AcqRel);
    let ptr = slot.snap.load(Ordering::Acquire);
    let out = (|| {
        if ptr.is_null() {
            STAT_FALLBACKS.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        // SAFETY: ptr was published with a release store and cannot be freed
        // while slot.readers > 0 (this reader) — collect() only frees at
        // readers == 0 and retired pointers are unreachable via slot.snap.
        let snap = unsafe { &*ptr };
        if snap.epoch > guard.epoch {
            // Drift: writer published a newer snapshot inside our pin window.
            STAT_FALLBACKS.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        STAT_STABLE_READS.fetch_add(1, Ordering::Relaxed);
        Some(snap)
    })();
    slot.readers.fetch_sub(1, Ordering::Release);
    out
}

/// Writer: build an immutable replacement snapshot for the shard, publish it
/// with a release store, bump the global epoch, retire the old snapshot.
/// The old snapshot stays valid for readers that loaded it before the swap.
/// Reclaim pressure is bounded by the tick-phase publish cadence (rebuild
/// per tick phase, fallback share expected < 1% — gate G1 effect marker).
pub fn publish(table: &PapayaShardTable, shard: usize, sections: Vec<u64>) {
    debug_assert!(shard < SHARD_COUNT);
    assert!(shard < SHARD_COUNT, "shard index out of range");
    let slot: &ShardSlot = &table.slots[shard];
    let new_epoch = EPOCH.fetch_add(1, Ordering::AcqRel) + 1;
    let fresh = Box::into_raw(Snapshot::new(new_epoch, sections));
    let old = slot.snap.swap(fresh, Ordering::Release);
    STAT_PUBLISHES.fetch_add(1, Ordering::Relaxed);
    if !old.is_null() {
        let mut q = slot.retired.lock().unwrap();
        q.push(Retired { epoch: EPOCH.load(Ordering::Relaxed), ptr: old });
    }
    collect(table);
}

/// Conservative EBR reclamation: free retired snapshots only when NO reader
/// is pinned anywhere. Retired snapshots are unreachable through slot.snap,
/// and readers pin before loading the pointer, so quiescence (READERS == 0)
/// guarantees nobody holds a pre-swap reference. New readers always observe
/// the current pointer, never a retired one.
pub fn collect(table: &PapayaShardTable) {
    if READERS.load(Ordering::Acquire) != 0 {
        return;
    }
    for slot in table.slots.iter() {
        let mut q = slot.retired.lock().unwrap();
        while !q.is_empty() {
            // Re-check under the lock: a reader may have pinned meanwhile.
            if READERS.load(Ordering::Acquire) != 0 {
                return;
            }
            let last = q.len() - 1;
            let r = q.remove(last);
            unsafe { drop(Box::from_raw(r.ptr)) };
            STAT_RECLAIMED.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// The shard table (32 slots). DORMANT: no caller outside this module.
pub struct PapayaShardTable {
    slots: [ShardSlot; SHARD_COUNT],
}

impl PapayaShardTable {
    pub fn new() -> PapayaShardTable {
        PapayaShardTable { slots: std::array::from_fn(|_| ShardSlot::new()) }
    }

    #[inline]
    pub fn stat_pins() -> u64 {
        STAT_PINS.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn stat_stable_reads() -> u64 {
        STAT_STABLE_READS.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn stat_fallbacks() -> u64 {
        STAT_FALLBACKS.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn stat_publishes() -> u64 {
        STAT_PUBLISHES.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn stat_reclaimed() -> u64 {
        STAT_RECLAIMED.load(Ordering::Relaxed)
    }

    /// Queue depth across shards (diagnostics for gate G3 memory pressure).
    pub fn retired_len(&self) -> usize {
        self.slots.iter().map(|s| s.retired.lock().unwrap().len()).sum()
    }
}

impl Default for PapayaShardTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Tests share process-global statics (EPOCH/READERS/counters); serialize
    /// them with a lock and assert on counter DELTAS so ordering can never
    /// flip an assertion (deterministic under cargo's parallel harness).
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn publish_then_stable_read() {
        let _lg = TEST_LOCK.lock().unwrap();
        let t = PapayaShardTable::new();
        let (fb0, sr0, pb0) = (
            STAT_FALLBACKS.load(Ordering::Relaxed),
            STAT_STABLE_READS.load(Ordering::Relaxed),
            STAT_PUBLISHES.load(Ordering::Relaxed),
        );
        // No snapshot yet -> fallback (vanilla path).
        let g0 = pin();
        assert!(read_shard(&t, 3, &g0).is_none());
        drop(g0);

        publish(&t, 3, vec![10, 11, 12]);
        let g = pin();
        let snap = read_shard(&t, 3, &g).expect("stable read after publish");
        assert_eq!(snap.sections, vec![10, 11, 12]);
        drop(g);
        assert_eq!(STAT_STABLE_READS.load(Ordering::Relaxed) - sr0, 1);
        assert_eq!(STAT_PUBLISHES.load(Ordering::Relaxed) - pb0, 1);
        assert_eq!(STAT_FALLBACKS.load(Ordering::Relaxed) - fb0, 1);
    }

    #[test]
    fn drift_is_fail_closed() {
        let _lg = TEST_LOCK.lock().unwrap();
        let t = PapayaShardTable::new();
        let (fb0, sr0) = (
            STAT_FALLBACKS.load(Ordering::Relaxed),
            STAT_STABLE_READS.load(Ordering::Relaxed),
        );
        publish(&t, 5, vec![1]);
        let stale = pin(); // pinned before the second publish
        publish(&t, 5, vec![2, 3]);
        // Stale pin: snapshot epoch newer than pinned epoch -> fallback.
        assert!(read_shard(&t, 5, &stale).is_none());
        drop(stale);
        // Fresh pin sees the new snapshot.
        let fresh = pin();
        assert_eq!(read_shard(&t, 5, &fresh).unwrap().sections, vec![2, 3]);
        drop(fresh);
        assert_eq!(STAT_FALLBACKS.load(Ordering::Relaxed) - fb0, 1);
        assert_eq!(STAT_STABLE_READS.load(Ordering::Relaxed) - sr0, 1);
    }

    #[test]
    fn retire_and_reclaim_quiesced() {
        let _lg = TEST_LOCK.lock().unwrap();
        let t = PapayaShardTable::new();
        let rc0 = STAT_RECLAIMED.load(Ordering::Relaxed);

        // No readers pinned: publish's internal collect reclaims eagerly and
        // exactly once (first publish has nothing to retire).
        for i in 0..3u64 {
            publish(&t, 7, vec![i; 8]);
        }
        assert_eq!(t.retired_len(), 0);
        assert_eq!(STAT_RECLAIMED.load(Ordering::Relaxed) - rc0, 2);

        // Pinned reader defers reclamation: retired queue grows, explicit
        // collect() must not free while a pin is held.
        let held = pin();
        publish(&t, 7, vec![9; 8]);
        publish(&t, 7, vec![10; 8]);
        assert_eq!(t.retired_len(), 2);
        collect(&t);
        assert_eq!(t.retired_len(), 2);
        drop(held);
        // Quiesced: all retired snapshots reclaimed exactly once.
        collect(&t);
        assert_eq!(t.retired_len(), 0);
        assert_eq!(STAT_RECLAIMED.load(Ordering::Relaxed) - rc0, 4);
    }

    #[test]
    fn out_of_range_shard_falls_back() {
        let _lg = TEST_LOCK.lock().unwrap();
        let t = PapayaShardTable::new();
        let fb0 = STAT_FALLBACKS.load(Ordering::Relaxed);
        let g = pin();
        assert!(read_shard(&t, SHARD_COUNT, &g).is_none());
        drop(g);
        assert_eq!(STAT_FALLBACKS.load(Ordering::Relaxed) - fb0, 1);
    }
}
