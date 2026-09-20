# LEVER.md — items_index (agent A, TASK-395 mega-round)

## Mechanism (2 lines)

`ItemEntity.mergeWithNeighbours()` — the self-gate and merge loop stay VANILLA
in-class bytes; only the candidate scan is retargeted (length-preserving body
swap, single retransform, `CRUSSTY_LEVER_FLAG=items_index`) from
`Level.getEntitiesOfClass(ItemEntity, inflatedBox, predicate)` (EntitySectionStorage
AABB scan) to `ItemMergeIndexOps.candidates(ItemEntity)` — a per-Level 0.5-grid
spatial hash (fastutil `Long2ObjectOpenHashMap<Cell>`, 64 ReentrantLock shards,
region-threads safe), cells keyed by quantized position, maintenance LAZY:
refresh(self) at every query, compact-on-touch (removed/superseded/aged-out
entries), exact vanilla post-filters per candidate (alive, != self, CURRENT
box intersects, distinct).

## What stays vanilla (parity by construction)

- The `isMergable()` self-gate at method entry: ORIGINAL in-class bytes.
- The loop body: per-candidate `isMergable()` re-check,
  paper `fixes.fixItemsMergingThroughWalls` gate,
  `clipDirect(...)==HitResult.Type.BLOCK -> continue` (NOT method-exit —
  byte-anchored to javap pc146 `goto 76`), `tryToMerge`, `isRemoved -> exit`:
  ORIGINAL private methods invoked with the ORIGINAL CP refs reused verbatim
  (kernel emits invokevirtual for same-class private calls — mirrored).
- Radius plumbing recomputed by the SAME formula in the ops scan:
  `spigotConfig.itemMerge`, paper `onlyMergeItemsHorizontally ? 0.0 : d-0.5`
  (both public configuration surface), applied to `getBoundingBox().inflate`.
- Empty flag = zero patch (dormant-invisible); patch is fail-closed on any
  kernel shape mismatch (rename/opcode drift -> lever stays dormant).

## Registered deviations

1. **Candidate order** = cell-key/insertion order, not section-storage order.
   For 2-candidate merges the outcome is identical (tryToMerge is
   order-symmetric for a pair); for >2 candidates the survivor selection may
   differ while the total merge math stays convergent.
2. **First-visibility lag** <= 40 ticks: an entity enters the index at its
   first executed merge query (vanilla stride-gates the same way), so a
   strictly-static mergeable newborn is visible to OTHERS up to 40 ticks
   later than vanilla would allow.
3. **Predicate timing**: vanilla filters `x.isMergable()` at query time via
   the invokedynamic predicate; the index filters alive/box and leaves
   isMergable to the loop's per-candidate re-check — the effective filter at
   tryToMerge-time is identical (vanilla evaluates it no earlier than the
   loop).
4. **Stale-cell strong refs**: entries of removed entities linger until a
   query touches their cell (bounded by distinct item count; SWEEP_DROPS
   counter tracks compactions). No MinecraftServer/world dependency.

## Profile note (honest ceiling)

Baseline run 35528326290 (bank v4, fp=4, pop 150k, 115655 samples):
`ItemEntity.tick` = 31.17% java, but `mergeWithNeighbours` inside it = 13
samples (0.04%) — the bench fixture (`BenchPopulationPlugin`) sets
`pickupDelay=32767` on all items, so `isMergable()` short-circuits and the
vanilla AABB merge scan does not run on steady state. The lever is armed and
measured as-is; expected delta ~0 +/- noise unless merge queries are
re-introduced by the scene. Pickup search is intentionally NOT touched
(separate query, out of vector scope).

## Local verification (machine-anchored)

- Real kernel `ItemEntity.class` (patched-kernel.jar, s7204): patch applies,
  `Retargeted{sites:1}`, 105-byte body; `javap -v` decodes; JVM
  `-Xverify:all` LOADS AND VERIFIES the patched class via URLClassLoader
  (kernel jar second on classpath) — StackMapTable accepted
  (append@19 [List,Iterator], append@69 [ItemEntity], chop@14).
- Ops trio: `javac --release 21` against the kernel jar + fastutil;
  `-Xverify:all` class-init OK (12 declared methods).
- `cargo check` green; `cargo test classfile::items_index_tests` green.
