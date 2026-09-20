# ROUND-397-A — ITEMS-COMPOSE (lever_flag="items_compose_ai")

## Mechanism — COMPOSITION of round-1 levers A + I on ONE ItemEntity pipeline
- **Vector A (items_index, ROUND-396-A, +2.9% ARMED)** decides WHERE the
  neighbours are: 1.0-grid hashed merge-candidate index
  (ConcurrentHashMap<Long,ArrayList> buckets + per-entity current-bucket side
  table) replaces the per-scan O(section-population) broadphase dump
  `Level.getEntitiesOfClass(ItemEntity, AABB.inflate(merge), pred)`.
- **Vector I (items_wakeup, ROUND-396-I, +4.0% ARMED)** decides WHEN to scan:
  event-driven wakeup — the scan body runs ONLY on E1 first sighting / E2
  displacement > 0.25 since last scan / E3 one-shot wake / E4 post-merge
  neighbour wake (+ unconditional post-teleport scan, vanilla parity).

FOUR strict receiver-prepended static retargets, each byte-identical to a
round-1-proven leg, all computed from pristine bytes with sites==1:
1. `mergeWithNeighbours()V`: getEntitiesOfClass site (mw:67) ->
   `ItemsComposeOps.getMergeCandidates` (grid index; exact vanilla filter
   chain incl. the ORIGINAL predicate instance — candidate set identical).
2. `tick()V`: `ItemEntity.move(MoverType;Vec3)` site (tick:269, invokevirtual
   #410, CP-owner ItemEntity — javap-verified on the bank kernel) ->
   `ItemsComposeOps.moveIndexed` = exact vanilla move + O(1) reconcile.
3. `tick()V`: mergeWithNeighbours site (tick:471) ->
   `ItemsComposeOps.mergeWithNeighbours(ItemEntity)` — E1/E2/E3 gate, then the
   REAL vanilla body via reflection delegate (zero logic drift).
4. `teleport(TeleportTransition)`: mergeWithNeighbours site (tp:29) ->
   `ItemsComposeOps.mergeAfterTeleport(ItemEntity)` — unconditional scan.

## Composition contracts (beyond each lever standalone)
- C1 wakeup gates the index: no event -> vanilla scan body never runs ->
  getMergeCandidates never invoked -> neither scan NOR index enumeration.
  The index itself stays position-exact for OTHER kverying items via the
  per-move reconcile (O(1) map ops; every live item moves every tick, and
  tick runs move @269 BEFORE merge @471).
- C2 merge candidates come ONLY from grid buckets — the item merge path
  contains ZERO broadphase dumps.
- C3 E4 neighbour wake reads the same grid buckets (superset box + exact
  AABB filter) — the standalone wakeup lever still used
  level.getEntitiesOfClass here; composed lever does not.

## Gate / fail-closed matrix
`CRUSSTY_LEVER_FLAG == "items_compose_ai"` (rust arm + ops class-init
re-check + JNI selfTest() BEFORE retransform: delegate must resolve AND
index structures must be usable, else ItemEntity stays vanilla). Patch
rejected / non-strict site count / define failure / selfTest failure ->
hook dormant. Post-retransform delegate failure -> skipped scan (log-once,
never a fabricated merge, never a crash).

## Parity / superiority deviations (DOC-DEV — union of donor levers)
- Candidate SET identical (harness T1: 1000 randomized queries PASS);
  candidate ITERATION ORDER may differ from EntitySectionStorage (donor A
  DOC-DEV, same class of nondeterminism accepted upstream when Paper/
  Moonrise replaced broadphase ordering).
- Merge TIMING may differ by one event latency (donor I DOC-DEV): skipped
  scans only DELAY a merge to the next event. Hard invariants hold:
  units conserved exactly (T2/T3), zero fabricated merges, pair invariant
  after settle (T2), E2 threshold and ACTIVE_ONCE semantics exact (T3).
- despawn/pickup/baseTick untouched (vanilla bodies).

## Verification
- Rust: `cargo test itemscompose` 6/6 PASS — incl.
  `itemscompose_patches_real_kernel_bytes_strict_four_sites` against the
  committed bank-v4 kernel fixture (ItemEntity.kernel65.class, major 65,
  extracted from run-s7204-bitmask/patched-kernel.jar): all four retargets
  strict-1 + idempotency (re-patch rejected, no double rewrite).
- Java harness `items_compose/harness/ItemsComposeHarness.java`:
  T1 SET-EQUALITY PASS / T2 GATING+CONSERVATION PASS / T3 EVENT-COVERAGE
  PASS -> HARNESS RESULT: PASS.
- javap census of the bank kernel: all four sites are invokevirtual with
  CP refs #410 (ItemEntity.move), #462 (ItemEntity.mergeWithNeighbours),
  #562 (Level.getEntitiesOfClass) — retarget_virtual_to_static applies.

## Research
/home/z/rounds/ROUND-397/RESEARCH-A.md — spatial-hash broadphase precedents
(metafunctor sparse spatial hash, gameprogrammingpatterns spatial partition),
Lithium item_entity_merging candidate-query redirect, Pufferfish DAB/dirty-
flag event scheduling; vanilla byte census.

## Build
`scripts/build_items_compose_ops.sh <javac> <kernel.jar>` — exactly ONE
classfile (no nested classes; the single lambda = invokedynamic).
