# ROUND-396-C — ITEMS-SWEEP (lever_flag="items_sweep")

## Mechanism
Sweep-line batch-merge: ONE O(contacts) grid sweep per region bucket per tick
replaces the per-entity mergeWithNeighbours broadphase query (O(n) section
dumps). RegionTickOps.tickBucket calls ItemsSweepOps.sweepBucket(bucket,len)
before the vanilla consumer loop; the retargeted ItemEntity.tick merge site
(ItemsSweepOps.tickMerge) is suppressed inside the parallel sweep phase and
delegates to the pristine vanilla body outside it (privateLookup MethodHandle).
Buckets are disjoint 8x8-chunk regions and the merge contact reach (0.75
blocks) never crosses slot boundaries, so parallel sweeps are pair-exact.
Grid: 0.75-reach int cells, ThreadLocal buffers (zero steady-state alloc),
vanilla tryToMerge/merge applied per contact pair in id order.

Wiring: ItemsSweepOps CO-DEFINED with RegionTickOps in region_threads.rs
bridge_list (tickBucket references it; bank v4 always delivers RegionTickOps);
env gate INSIDE the ops class (SWEEP static-init = static-read no-op when
lever differs — dormant-invisible for the bank). ItemEntity byte patch
(strict single site, tick()) registered ONLY when
CRUSSTY_LEVER_FLAG=="items_sweep". sweepBucket self-quarantines to vanilla on
any internal failure. Fail-closed: strict-1 site violation or missing pristine
bytes = hook dormant.

## Parity
- Same candidate pairs (0.75-reach superset grid + exact vanilla AABB filter),
  same tryToMerge/merge application, same merge direction (smaller->bigger,
  equal -> candidate receives; id order = deterministic).
- Deviation (DOC-DEV): merge application ORDER within a tick differs from the
  vanilla per-entity sequential scan (batched by bucket, id-ordered pairs) —
  units conserved, same terminal saturation per contact component; same class
  of nondeterminism upstream broadphase replacements accept.
- despawn/pickup/baseTick untouched.

## Validation
- entityinside bridges recompiled (RegionTickOps + ItemsSweepOps + BatchCollector
  + TrackerTickOps + RngOps + BlockUpdateOps) with --release 21 vs patched-kernel.
- Rust: co-definition push + strict single-site retarget (classfile.rs
  retarget_virtual_to_static, Retargeted{sites:1} required) + BRIDGE_READY wait
  (no lazy-resolution NCDFE window on first item tick).
