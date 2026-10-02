# TASK-89 SUPPLEMENT — BE-tick `shouldTickBlocksAt`: MEASURED micro-rig + full-jar CP census

* Author: agent-7625532f (parallel session to the canonical static audit). Date: 2026-09-08.
* **Canonical closure: `docs/BETICK_STATIC_AUDIT_2026-09-08.md` (verdict NO-GO / DO-NOT-BUILD, ledger ADDENDUM-6, X1000_V3 §6.2) — this file adds MEASURED evidence, it does not change the verdict.**
* Evidence class: P500-style offline micro-rig on the REAL map class + full-jar CP call-site census. Zero boots, zero `src/` changes.

## 1. Measured micro-rig (old vs SparklyPaper-pattern cache, real `ConcurrentLong2ReferenceChainedHashTable`)

Rig: `bench/betick/BETickCacheRig.java`, compiled against the deployment `concurrentutil-0.0.7.jar`
(the exact lookup class inside Moonrise `ChunkHolderManager.chunkHolders`, javap-verified).
Model: BE list = 90% sorted-by-chunk + 10% shuffled tail (SparklyPaper's own list-shape description);
keys 90% ticking-ready, 5% absent; 11 windows × ≥2k frames, median; 3 warmup passes; anti-DCE sink.
OLD arm = the check per BE (`map.get(key)` + null-check + `tickingReady` read); NEW arm = single-entry
`(lastKey -> lastResult)` cache. Conservative bias: warm-cache makes the OLD arm look FASTER than
production → a NO-GO under this bias is robust.

| Scenario | ns/BE old | ns/BE new | ratio | saved ns/BE | ceiling % server-thread @ 20 TPS |
|---|---|---|---|---|---|
| 500 BEs × 8/chunk (fresh-gen world class) | 2.71 | 0.99 | 2.75× | 1.72 | **0.002%** |
| 5,000 BEs × 8/chunk | 2.41 | 1.04 | 2.32× | 1.37 | **0.014%** |
| 5,000 BEs × 32/chunk | 2.37 | 1.05 | 2.27× | 1.33 | 0.013% |
| 50,000 BEs × 8/chunk (hopper-dense giant) | 2.56 | 1.42 | 1.81× | 1.14 | **0.114%** |
| 50,000 BEs × 32/chunk | 2.10 | 1.14 | 1.83× | 0.95 | 0.095% |

Ceiling formula: `saved_ns × BE_count × 20 tps / 1e9` (fraction of one core). Reaching the pre-registered
>3% gate at the measured 1.14 ns saving needs **≈1.3 M block entities inside the ticking view** —
physically unrealistic. **The static audit's estimated ceiling (≤0.04% realistic) is confirmed by
measurement with an independent method.**

Note on scope split with the canonical audit: the audit correctly identified the ADDITIONAL
`LevelTicks.sortContainersToTick` `tickCheck` route (once per DUE container — the method-ref-lambda
surface, invisible to a naive CP scan); this rig measures the OTHER route the SparklyPaper patch
targets — `Level.tickBlockEntities()` per-BE `shouldTickBlocksAt(ticker.getPos())` (javap-verified
call site, `invokevirtual` at pc 135). Both routes: same verdict, guard economics refuted.

## 2. Full-jar CP call-site census (R-entries, javap-verified)

Scanner: `bench/betick/shouldtick_callsites.py` → `results/SHOULD_TICK_CALLSITES_2026-09-08.txt`.
All 9,809 classes of `versions/1.21.10/purpur-1.21.10.jar`; exactly 5 Methodref/InterfaceMethodref
call sites of `shouldTickBlocksAt` (direct `invokevirtual` dispatch sites):

| Caller | Enclosing method | Status |
|---|---|---|
| `Level.tickBlockEntities()` | per-BE check (**measured in §1**) | hot |
| `Level.shouldTickBlocksAt(BlockPos)` pc 5 | (J)-overload delegate | trivial |
| `ServerLevel.runBlockEvents()` | block events only | cold |
| `SculkSpreader$ChargeCursor` | charge-cursor apply | cold |
| `VibrationSystem$Ticker` | vibration stage | cold |

Zero CP refs from `net/minecraft/world/ticks/*` — scheduled block/fluid ticks on 1.21.10+Moonrise
never call `shouldTickBlocksAt` directly (their gate = the `tickCheck` predicate route, audited
canonically). Negative finding recorded for any future lever-hunting in the scheduled-ticks domain.

Tooling lesson (recorded for future CP scanners): method-ref lambdas (`ServerLevel::shouldTickBlocksAt`
style) bind via `BootstrapMethods` MethodHandles, NOT via a synthetic method body — an R-entry-only
scan misses them; the first draft of this scanner had exactly that gap. The canonical audit's javap
route + this scanner (R-entries + javap cross-check) together cover both dispatch forms.

## 3. Re-open criteria

Unchanged from the canonical audit (`docs/BETICK_STATIC_AUDIT_2026-09-08.md`): JFR ≥3% tick share for
the check (either route), or a structural change to the lookup. Both scanners/rigs re-run in seconds.
