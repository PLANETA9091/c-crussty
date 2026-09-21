# TASK-397-H / items_despawn_heap (MEGA-ROUND-2)

Event-driven despawn/pickup lifetime subsystem for `ItemEntity`
(research: `/home/z/rounds/ROUND-397/RESEARCH-H.md`).

## Mechanism

- **Hot path replaced**: the vanilla per-tick tail gate
  `if (!level().isClientSide() && age >= despawnRate) {ItemDespawnEvent / discard}`
  inside BOTH `ItemEntity.tick()` and `ItemEntity.inactiveTick()` (the only
  lifetime evaluation points, 103k items × every tick on the bank scene) is
  spliced same-length (7 bytes: `aload_0; invokevirtual level;
  invokevirtual isClientSide` → `aload_0; invokestatic ItemLifetimeOps.tailGate;
  nop×3`). Zero branch-target drift, zero StackMapTable edits; strict census
  (exactly 1 tail per method) fail-closed to pristine bytes.
- **Schedule**: at the first tail sighting `due = now + (despawnRate − age)`
  clamped to ≥ now (EXACT vanilla algebra: age grows by 1 every tick before
  the tail, so the vanilla field check fires at precisely that tail; overdue
  items are due immediately — never later than vanilla). At the due tail the
  ops re-validate against the FIELD before letting the untouched vanilla
  block run. `age == −32768` (unlimited) → `due = NEVER`.
- **JVM parity harness** (throwaway stubs + the REAL ItemLifetimeOps.java,
  run locally): fresh age0=0 / mid 2500 / edge 5999 / unlimited −32768 /
  despawn-cancel re-due / setExtendedLifetime mid-flight — ALL byte-exact
  parity vs the vanilla tail simulation (same despawn tick, same event count,
  same final age). The ONLY deviation = the documented writer-shortening
  case (makeFakeItem-style age rewrite mid-life) — dead in the bench window.
- **Heap**: flat parallel arrays `long[] heapDue / int[] heapId` (no boxing —
  kills the `java/util/PriorityQueue` lane on this subsystem) + `dueById[]`
  fast table; lazy tombstone pops (`/kill`, merge-discard, chunk unload never
  do O(N) removals). Cross-thread despawn is forbidden: each due is resolved
  ONLY by the thread that ticks the entity; the heap is scheduler index +
  observability.

## Parity (ваниль по построению)

The despawn DECISION bytes survive untouched: at due the ops write nothing
alien — they return `false` and the SURVIVING vanilla check
(`age >= despawnRate` + `ItemDespawnEvent` → cancel `age=0` / `discard(DESPAWN)`)
runs exactly as vanilla. The ops can only make the vanilla gate RUN, never
change its math. Any ops failure fails open to the surviving vanilla check.

## Documented deviations (all bounded, all dead in the bench window)

1. **pickupDelay stays vanilla** in v1 (bench scene pins 32767 = dead lane;
   writer surface = playerTake intra-tick putfields + Bukkit
   `setPickupDelay`); the heap is despawn-only.
2. **Writer-shortened lifetimes** (merge-max, makeFakeItem) despawn on their
   own heap due — bounded lateness ≤ remaining life; not present in the
   300s bench window (no merges: pickupDelay=32767 makes items un-mergeable,
   no makeFakeItem callers in scene).
3. **isClientSide probe dropped** from the hot path (dedicated kernel server:
   always false; the surviving vanilla check re-decides anyway).
4. `entityById[]` holds strong refs (observability table) — bounded by scene
   population, cleared only by heap compaction; ids are monotonic, never
   reused.

## Gate / dormancy

`CRUSSTY_LEVER_FLAG == "items_despawn_heap"` (else: exact vanilla path, byte
parity by construction; hook never registered). Rebuild the bridge with
`scripts/build_item_lifetime_ops.sh` (javac --release 21 against
`research/gc-recon-2026-09-19/run-s7204-bitmask/patched-kernel.jar`).

## Proof chain (local, pre-dispatch)

- `cargo test --lib item_despawn_heap_splice` (4 tests, REAL kernel fixture
  `tests/fixtures/ItemEntity.class`, 28904 B pristine): strict-1 both methods,
  ONLY the 7-byte windows change (6 differing bytes each behind the kept
  `aload_0`), idempotent re-patch byte-identical, ops resolution closure.
- JVM verifier: patched bytes (28978 B) `defineClass` under `-Xverify:all`
  → `VERIFIED-OK`.
- Runtime markers (server-stdout): `items_despawn_heap: computed patch for
  ... (28904 -> 28978 bytes, ...)` + `[S7-H] items_despawn_heap ARMED:` from
  the ops static-init on first item tick.
