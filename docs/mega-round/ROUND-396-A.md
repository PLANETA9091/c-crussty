# ROUND-396-A — ITEMS-INDEX (lever_flag="items_index")

## Mechanism
1.0-grid hashed index of ItemEntity merge-candidates (Long2Object bucket map +
per-entity current-bucket side table), maintained by the items' own ticked
move. Replaces the per-item-per-tick `Level.getEntitiesOfClass(ItemEntity,
AABB.inflate(0.5), pred)` broadphase dump (TOP-1 bottleneck: items 31.17% java
+ item-driven broadphase 15.66%, bank v4 baseline 115655 samples).

Two STRICT single-site receiver-prepended static retargets (region_threads
discipline, sites==1 from pristine bytes):
1. `ItemEntity.mergeWithNeighbours` — the getEntitiesOfClass site ->
   `ItemMergeIndexOps.getMergeCandidates(Level;Class;AABB;Predicate)List`
   (bucket superset enumeration + EXACT vanilla box.intersects + the very
   vanilla Predicate instance = same candidate set by construction).
2. `ItemEntity.tick` — the single `ItemEntity.move(MoverType;Vec3)` site ->
   `ItemMergeIndexOps.moveIndexed(...)` = exact vanilla move + index
   reconcile. tick runs move BEFORE merge, so the querying item is
   index-exact at its own query.

Gate: `CRUSSTY_LEVER_FLAG == "items_index"` (rust arm + belt-and-braces
class-init re-check). Absent flag = exact vanilla path (parity by construction).
Cohabitation: sole owner of the ItemEntity byte pipeline (census: no
bank-active lever hooks ItemEntity). Fails closed.

## Parity / superiority deviations (DOC-DEV)
- Candidate SET is bit-equal (harness T1: 1000 randomized queries, set
  equality PASS incl. boundary AABBs).
- Candidate ITERATION ORDER may differ from vanilla EntitySectionStorage
  order -> which stack survives a multi-candidate cascade can differ by <=1
  entity; hard invariants hold: units conserved exactly, entitiesLeft within
  cap-64 bound [ceil(units/64), units] (harness T2 PASS, 20 trials x 3
  orderings, max variance 1). Same class of nondeterminism upstream accepted
  when Paper/Moonrise replaced broadphase iteration order. Merge/despawn/
  pickup semantics untouched.
- Bounded index staleness <=1 tick for items spawned/teleported between
  ticks (documented; those are re-registered by their first move/tick).

## Harness
`items/harness/ItemMergeIndexHarness.java` (pure-java, runnable):
T1 SET-EQUALITY PASS / T2 MERGE-CASCADE AGGREGATE INVARIANCE PASS / T3
pack-key parity PASS (handles proven by javac compile vs patched-kernel.jar;
full kernel-lib graph not shipped locally -> reflective handles probe SKIP).
Rebuild: `scripts/build_item_merge_ops.sh <javac> <kernel.jar>`.

## Research
/home/z/rounds/ROUND-396/RESEARCH-A.md — vanilla O(n) merge scan vs grid
index; upstream precedents (Moonrise EntityLookup per-class section walks,
Lithium item_entity_merging QUERY_LIMIT) — ceiling 10-20% TPS-equivalent.
