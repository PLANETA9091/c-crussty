# LEVELTICKS / REDSTONE-LENS STEP-0 + GC-SHAPE-1 refutation — 2026-09-17 (S7-97, task166/167)

## 1. GC-SHAPE-1 (task166) — REFUTED pre-code by GC physics

STEP-0 law applied: javap new-scan first, then honest sizing vs measured GC reality.

### Verified alloc sites (javap new-scan, run#14 booted kernel + /tmp/pp materialized kernel)

- `ServerLevel.optimiseRandomTick` (research/rng-recon-2026-09-16/optimiseRandomTick.javap):
  EXACTLY ONE `new` in the whole method — `new BlockPos` @180, guarded by
  `rand(0..4095) < tickingList.size()` (@145-149). Allocates ONLY on hit picks.
  Retention consumers: `BlockState.randomTick` + `FluidState.randomTick` — any
  `scheduleTick(pos, ...)` inside a body RETAINS the BlockPos (ScheduledTick stores it),
  so reuse/flyweight is unsound without copy-on-enqueue proof (moot — see sizing).
- Brain LinkedHashMap iterator churn (secondary target): alloc-collapsed share
  run#12 = 0.69%, run#15 = 0.40% of allocation pressure (Brain total 6.73/6.60%).

### Measured GC reality (scripts/gc_steady_scan.py on run#12/run#15 gc.log — BANKED)

| metric | run#12 | run#15 |
|---|---|---|
| steady young-GC interval | ~5.9s | ~4.1s |
| eden fill per GC | ~2.43 GB | ~2.44 GB |
| allocation rate | ~412 MB/s | ~598 MB/s |
| steady young-GC STW duty | 0.50% of wall | 0.56% of wall |
| avg / max young pause | 17.1 / 67.0 ms | 13.2 / 73.9 ms |

### Refutation arithmetic

- Hit-path CPU bounds BlockPos rate: random-tick lane 5.46% total, RNG 1.57% +
  optimiseRandomTick self 2.2% => hit work (PalettedContainer.get + randomTick
  bodies) <= ~1.7% of tick (~1.3ms @ 77ms) => <= ~30-50K hits/tick =>
  BlockPos alloc <= 0.7-1.2 MB/tick = **2-3% of the ~40 MB/tick total**.
- Relief ceiling = alloc_share x GC_STW_duty = 2-3% x 0.5% = **<=0.015% MSPT**.
  Brain-LHM: 0.4-0.69% x 0.5% = **<=0.005% MSPT**. Both are 200-600x below the 3% gate.
- PORTFOLIO LAW (bigger than the lever): the GOAL-ledger "GC 9.5%" presence is
  CONCURRENT GC WORKER CPU (`OopOopIterateDispatch<G1CM/G1RebuildRemSet>` leaves),
  not MSPT. On spare cores concurrent GC CPU does not extend the tick; the only
  MSPT-additive part is STW duty = 0.5%. Garbage-shape optimization (young-dying
  objects) scales GC FREQUENCY (relief = share x 0.5%), pause SIZE scales with LIVE
  set (not garbage). => THE WHOLE ALLOCATION-SHAPE FAMILY IS DEAD AS MSPT LEVER
  on this bench. GC row in GOAL doc re-annotated; no further alloc-shape STEP-0s.
- GC-SHAPE-1: REFUTED. Fourth consecutive STEP-0 kill (presence-mirage pattern).

## 2. REDSTONE / LEVELTICKS-LENS (task167) — STEP-0 executed, REFUTED as solo lever

### Measured lane (cpu-collapsed, redstone/ticks bucket, entry-census)

- run#12: **8.28%** of 51,971 samples; run#15: **11.55%** of 128,922 samples.
- Entry census: `net/minecraft/world/ticks/LevelTicks.tick` = 8.08% / 11.35%
  (everything else — updateNeighbourForOutputSignal, container-menu signal,
  CollectingNeighborUpdater — ~0.1-0.2%). The ledger's old "~3.8% redstone" was
  a presence-mirage from a narrower bucket def; the real lane is the
  SCHEDULED-TICK DRAIN.

### Verified bytecode contract (research/levelticks-recon-2026-09-17/)

- `LevelTicks.runCollectedTicks(BiConsumer)` (LevelTicks.javap @420):
  `while (!toRunThisTick.isEmpty()) { t = poll(); toRunThisTickSet.remove(t);
  alreadyRunThisTick.add(t); biConsumer.accept(t.pos(), t.type()); }` —
  queue + set + list bookkeeping per tick.
- `ServerLevel.tickBlock(BlockPos, Block)` (tickBlock.javap — the lambda target of
  invokedynamic #5 in `ServerLevel.tick`):
  `getBlockState(pos)` -> `state.is(block)` -> `state.tick(level, pos, random)` ->
  `if ((++tickedBlocksOrFluids & 7) != 0) moonrise$executeMidTickTasks()`.
  MAX-BLOCK-TICKS cap comes from paperConfig `environment.maxBlockTicks`.

### Decomposition (leaves inside the LevelTicks.tick subtree, run#12/run#15)

| slice | run#12 | run#15 | replaceability verdict |
|---|---|---|---|
| block-state reads (getChunk + getBlockStateFinal + PalettedContainer.get + SimpleBitStorage + readPalette) | 2.65% | 3.77% | per-query irreducible; batch-by-section resolve saves only getChunk/hash ~0.3-0.5% |
| signal eval (getDirectSignal + getSignal + DefaultRedstoneWireEvaluator.updatePowerStrength + ZeroCollidingReferenceStateTable.get) | 1.18% | 1.43% | wire-eval batch lens ceiling ~1.5-2.5% |
| tick-queue machinery (ConcurrentLong2Reference getValueVolatile/getNode) | 0.76% | 0.50% | primitive open-addressing drain <=0.5% |
| Bukkit neighbor glue (CraftBlock.<init> + CraftBlockData.clone) | 0.29% | 0.62% | ctor cost, listener-conditional skip = semantics risk, <=0.6% |
| mid-tick yield (moonrise$executeMidTickTasks, whole-server lane) | 1.57% | 1.83% | frequency reshape <=1.0% relief, latency-semantics risk |
| tail (inlined heterogeneous scheduledTick bodies) | ~2.5-5% | — | no dominant single body (no block class >0.02% — JIT-inlined) |

**Verdict: lane presence 8.28-11.55% decomposes into slices ALL <3% solo =>
no single replaceable site clears the gate => REDSTONE/LEVELTICKS-LENS REFUTED
as solo lever (pre-code). Family-bank option (batch-resolve + queue + glue +
mid-tick, aggregate A/B) parked per GOAL family-path note — NOT scheduled;
current family economics <3% each with parity engineering cost.**

## 3. Cross-run stability of the new numbers

- LevelTicks lane: 8.28% vs 11.55% across runs on DIFFERENT world snapshots
  (runner-variance law context) — lane ranking is robust (2nd after entities).
- mid-tick-yield: 1.57/1.83%; ConcurrentLong2Ref-anywhere: 2.21/2.18% — stable.

## 4. Re-ranked lever queue (after this round)

1. **BRAIN-LENS** (task168): research-verified ~3.0% ceiling (Object[]+bitmap,
   insertion-order parity) — the only remaining >=3% VERIFIED-GO candidate.
2. minecarts STEP-0 (task169): ~5.3% presence, anatomy pending.
3. bench-4 fake-players: owner-scenario (spawn as-if-players) — infrastructure leg.
4. PARKED: LevelTicks family-bank, MIDTICK-LENS, ENT-BP infra A/B.

## Method notes

- GC sizing tool: scripts/gc_steady_scan.py (eden-fill x interval => alloc rate;
  steady-state window = last 2/3 of log span; STW duty = sum(pauses)/span).
- javap source: /tmp/pp materialized kernel purpur-1.21.10.jar
  (sha256 e2992d63..., materialized S7-96, killed pre-main = NOT a boot).
- Profile sources: run#12 (51,971 samples), run#15 (128,922 samples) cpu-collapsed
  + alloc-collapsed; gc.log steady-state parse.
