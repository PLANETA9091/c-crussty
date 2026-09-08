# BE-TICK shouldTickBlocksAt — STATIC AUDIT (TASK-89)

**Date:** 2026-09-08 · **Agent:** agent-7625532f · **Verdict: NO-GO / DO-NOT-BUILD — closed at the static layer, 0 boots, 0 src/ changes**

Task lineage: TASK-83 research queue item 2 ("BE-tick shouldTickBlocksAt — SparklyPaper pattern"),
claimed as TASK-88, renumbered **TASK-89** after the TASK-88 label collision with S7-32
(resolution: dev-logs CLAIMS.md 76a682a, first-pushed-wins precedent).
Claim's pre-registered GO gate: **>3% of tick share** in a census, or the guard impl proceeds.

---

## 1. Verified mechanics (javap-fidelity on the RUNNING deployment jar)

Source of truth: `versions/1.21.10/purpur-1.21.10.jar` (the assembled, Moonrise-patched runtime
this server actually boots; disassembled with the deployment JDK21 `javap -p -c`).

### 1.1 The predicate

```java
// net.minecraft.server.level.ServerLevel (verified bytecode)
public boolean shouldTickBlocksAt(long chunkPos) {
    NewChunkHolder holder = moonrise$getChunkTaskScheduler()
                              .chunkHolderManager.getChunkHolder(chunkPos);
    return holder != null && holder.isTickingReady();
}
// ca.spottedleaf.moonrise...ChunkHolderManager.getChunkHolder(long) (verified bytecode)
//   = chunkHolders.get(pos)  -- single call into
//     ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(J)
//   -- lock-free, O(1), no locks taken on the read path.
```

Total per-call cost: **one lock-free hash lookup + one flag read**. This is already the minimal
primitive available: any guard/cache replacing it would itself need a map keyed by chunk pos —
i.e. the same or higher cost (TASK-80 economics lesson: mechanism can work while economics do not).

### 1.2 Injection and the single predicate call-site

`LevelTicks<T>` is constructed with a `LongPredicate tickCheck` (= `ServerLevel::shouldTickBlocksAt(long)`
method reference). Bytecode search over `LevelTicks` shows **exactly one** `tickCheck` invocation,
inside `private void sortContainersToTick(long)`:

```
for each entry of nextTickForContainer (Long2LongMap, one entry per chunk with pending ticks):
    if (entry.nextTick > now)   -> entry.setValue(nextTick)     // pure map maintenance, NO predicate
    else /* tick is DUE */      -> if (tickCheck.test(entry.chunkPos))   // <-- THE call
                                       remove + queue container for drain this tick
```

The `BlockPos` overload `shouldTickBlocksAt(BlockPos)` has a single ServerLevel call-site in
`runBlockEvents()` (block events: note blocks, pistons) — a separate, low-frequency path outside
this audit's surface. Entity ticking uses `shouldTickEntitiesAt` (different method, different task).

### 1.3 Frequency model (the decisive fact)

The predicate fires **once per DUE chunk-container per game tick — not once per scheduled tick**.
A chunk with 50 due fluid ticks costs exactly **one** predicate call, then drains all 50.
Not-due containers (the potentially numerous "stale" set) never reach the predicate:
they are skipped by the map-maintenance branch at pure iteration cost, which a guard
cannot optimize away either (it is vanilla bookkeeping, not the check).

## 2. Ceiling arithmetic (pessimistic bounds, no live measurement needed to refute)

Let N = containers with a DUE tick in one game tick.

| Scenario | N | predicate cost @ ~40 ns/call | share of a 50 ms tick |
|---|---:|---:|---:|
| Realistic active world (fluids/crops/scaffolding settling) | ≤ 500 | ≤ 20 µs | **≤ 0.04%** |
| Pessimistic burst (mass fluid reflow across the loaded cube) | 10,000 | 400 µs | **0.8%** |
| Pre-registered GO gate | — | — | **>3%** |

Breaching the 3% gate requires ≥ 37,500 due containers per tick sustained — 4-75× beyond the
pessimistic bound and ~40× beyond any chunk count this server has ever held loaded in measured
sessions (TASK-81/82 censuses). Equivalently for the ≥1.5 s/burst payload gate: even the
pathological 0.8%/tick sustained over the 240 s TASK-82-style burst window totals ≤ 1.9 s,
realistically ≤ 0.1 s — below the gate, and that is the *upper* bound of the optimizable slice.

## 3. Stale-container accumulation hypothesis — refuted

The only mechanism that could unbound N: chunks that stop ticking but keep scheduled-tick
containers, re-tested every tick forever. Refuted on three independent grounds:

1. **Chunk unload removes containers**: `LevelTicks.removeContainer(ChunkPos)` is called from the
   chunk-unload path — unloaded chunks do not linger in `nextTickForContainer`.
2. **Loaded-but-not-ticking chunks are bounded** by the view-distance cube (thousands worst-case,
   tens typically) and, per §1.3, they skip the predicate entirely unless a tick is due.
3. Even if all of them were due every tick, §2 shows the share stays sub-1%.

## 4. Upstream context (unverified, non-load-bearing)

The "SparklyPaper mode" label from TASK-83 research refers to upstream forks reducing the cost of
this check on *vanilla* chunk handling, where `shouldTickBlocksAt` historically routed through
heavier per-call logic. On this deployment the Moonrise chunk system has **already** reduced the
check to a single lock-free hash-get (§1.1) — the upstream lever is architecturally superseded
here. (Upstream patch internals were not audited; the verdict rests solely on this jar's
verified bytecode and §2 arithmetic.)

## 5. Verdict and re-open criteria

**NO-GO / DO-NOT-BUILD.** The optimizable surface is a once-per-due-container-per-tick O(1)
lock-free lookup already at the floor of what any replacement could achieve; its measured-domain
ceiling (≤0.8% pathological, ≤0.04% realistic) cannot reach the pre-registered >3% GO gate.
A guard would add code, arming surface, and risk to buy nothing measurable — the exact
"mechanism works, economics do not pay" shape the TASK-80 fluid-guard A/B measured live at a
much larger ceiling.

Re-open the task ONLY if one of the following is measured:
1. A census (JFR, method-level) showing `LevelTicks.sortContainersToTick` + `shouldTickBlocksAt`
   frames ≥ 3% of Server-thread tick samples in a production-like profile; or
2. An engine change that makes the check structurally expensive again (e.g. Moonrise replacement
   or a vanilla-runtime build where the lookup is not the chained hash table verified in §1.1).

The static check itself is cheap to re-run: the javap commands in §1 take seconds and are the
census-first discipline's cheapest instrument (TASK-85 precedent).

## 6. Ledger

* RESULTS_LEDGER ADDENDUM-6 (§11) records this closure; §3 row 6 added.
* x1000 branch count: this is the 10th measured/statically-refuted branch
  (slot numbering follows TASK-85 = 9th; the earlier 10th-slot AppCDS refutation was overturned
  by TASK-87's GO, so the slot is reused here).
