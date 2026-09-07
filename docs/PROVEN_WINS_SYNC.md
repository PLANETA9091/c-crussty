# PROVEN_WINS / DO_NOT_WIRE evidence sync — canonical 2026-09-08 rerun (TASK-31)

**Status:** complete · **Agent:** agent-7625532f (TASK-31-sub) · **Date:** 2026-09-08
**Scope:** docs/registry evidence sync ONLY — no `.so`, no engine, no gate logic
(`decide`/`decide_id`/`batch_api` untouched; the `Allow`/`KeepJava` sets are byte-identical
before and after).

## 1. Why

TASK-12 (wave-2, `docs/BATCH_ADOPTION_MATRIX_wave2.md` §6.1) found that `kernel_policy.rs`
carried stale P500 evidence: 3 of 7 "P500 WIN" verdicts did not reproduce on the canonical
full rerun, and the batch-wiring gate (`kernel_policy::decide_id`, consumed by
`batch_api::run()` → `ERR_KERNEL_REFUSED`) reads this registry. DO_NOT_WIRE was already
synced in e5c4fad; PROVEN_WINS was not. This document is the audit trail for that sync.

## 2. Method

1. Canonical source: `bench/p500/results/P500_REPORT.md` — the completed full rerun
   (report generated 2026-09-07T16:46Z = 2026-09-08 UTC+8; **49 groups / 70 pairs /
   0 CRASH / 129 kernels**). This is the ONLY authoritative ns/op source now;
   `P500_REPORT_v2.md` is the historical (pre-rerun) report the old evidence pointed to.
2. Every registry entry carrying a P500-based verdict/number was re-checked
   group-by-group against the rerun tables (per-group medians, "Wins (promotion
   candidates)", "Regressions (do-not-wire)", "Parity", baseline-diff ratios).
3. Classification uses the canonical aggregator noise model (mirrored in
   `kernel_policy::WIN_MAX`/`REG_MIN`): **WIN** ratio ≤ 0.85, **REGRESSION** ≥ 1.18,
   **PARITY** in between — all on min-of-medians with stability shown. (Cross-check under
   the stricter wave-2 rubric 0.8/1.25: `switchGradientSummary` 0.819 and
   `scratchThreadLocalSummary` 0.834 sit in 0.8–0.85 — WIN under the canonical model,
   near-boundary parity under the stricter one; verdicts unchanged because the aggregator
   is the source of truth, and both are 0.0%-stability.)
4. Historical numbers were kept only as clearly-labelled "v2 said …" history inside the
   evidence strings / tables below.
5. Gate code was NOT changed: the 3 stale WIN entries stay inside `PROVEN_WINS` (verdict
   string reclassified to `P500 PARITY (2026-09-08 rerun)`), so `decide()` returns the
   same `Decision` for every input as before the sync.

## 3. Before/after — every registry entry

### 3.1 `PROVEN_WINS` — P500-WIN section (7 entries at sync time)

| Kernel (class.kernel) | 2026-09-07 v2 (old registry) | 2026-09-08 rerun (P500_REPORT.md) | Verdict after sync | Evidence pointer |
|---|---|---|---|---|
| `NoiseChunkBlendCache.newEmptyBlenderSummary` | "P500 WIN (244x)" — 67.0 µs → 274.5 ns | **316.45×** (95.3 µs → 301.1 ns, ratio 0.003, stability 0.0%) | **WIN** — direction unchanged, **number refreshed 244x → 316x** | P500_REPORT.md §Wins (g21) |
| `NoiseInterpolatorSlice.flatSummary` | "P500 WIN (3.29x)" — 6.2 ms → 1.9 ms | 3.32× (6.3 ms → 1.9 ms, ratio 0.301, 0.0%) | **WIN** (number 3.29x → 3.32x) | P500_REPORT.md §Wins (g23) |
| `ImprovedNoiseInline.switchGradientSummary` | "P500 WIN (1.22x)" — 9.3 → 7.6 µs | 1.22× (9.3 → 7.6 µs, ratio 0.819, 0.0%) | **WIN** — reproduced | P500_REPORT.md §Wins (g16) |
| `PalettedReencodeScratch.scratchThreadLocalSummary` | "P500 WIN (1.20x)" — 483.9 → 403.2 µs | 1.20× (493.6 → 411.9 µs, ratio 0.834, 0.0%) | **WIN** — reproduced (absolutes refreshed) | P500_REPORT.md §Wins (g27) |
| `PluginLoadingAllocation.newLazyValidateSummary` | "P500 WIN (1.55x)" — 217.9 → 140.6 ns | 0.993 (116.1 → 115.3 ns) | **PARITY — reclassified** (stale win #1) | P500_REPORT.md g30 / §JNI floor |
| `PluginLoadingAllocation.newLazyMissingSetSummary` | "P500 WIN (1.53x)" — 218.3 → 142.5 ns | 0.996 (115.1 → 114.6 ns) | **PARITY — reclassified** (stale win #2) | P500_REPORT.md g30 / §JNI floor |
| `AquiferSurfaceSampling.newBatchSummary` | "P500 WIN (1.15x)" — 6.3 → 5.5 µs | 0.906 (6.0 → 5.5 µs; speedup 1.10×) | **PARITY — reclassified** (stale win #3) | P500_REPORT.md g2 / §Parity |

Note on #3: TASK-12 quoted 0.917 (computed from the rounded per-group medians 5.5/6.0 µs);
the report's own baseline-diff pair table gives **0.906** (computed from unrounded medians).
Either number is inside the parity band — verdict identical; 0.906 (the report's own value)
is used everywhere in this sync.

### 3.2 `PROVEN_WINS` — live wirings (4 entries)

| Kernel | Rerun check | Verdict |
|---|---|---|
| `AreaMap.nativeUpdateOpsBatch` | live-verified wiring; no P500 number claimed in the entry | **live — unchanged** |
| `ImprovedNoise.nativeNoise` | live-verified (self-test PASSED, audit-wire armed in e5c4fad) | **live — unchanged** |
| `ImprovedNoise.nativeBuildHandle` | live-verified bridge | **live — unchanged** |
| `ImprovedNoise.nativeFreeHandle` | live-verified bridge | **live — unchanged** |

### 3.3 `PROVEN_WINS` — batch-dispatch surface (12 entries, `batch_table.rs` ids 0-11)

| id | Kernel | Rerun evidence | After sync |
|---|---|---|---|
| 0 | `TicketSetSearch.binarySummary` | **no pair in the canonical report** (§4) | PARITY (batch surface) — unchanged, flagged |
| 1 | `TicketSetSearch.uncheckedBinarySummary` | **no pair in the canonical report** (§4) | PARITY (batch surface) — unchanged, flagged |
| 2 | `AquiferIndexStride.oldBatchSummary` | g0 old-side (10.6 µs) — pair 0.934 | PARITY — unchanged |
| 3 | `AquiferIndexStride.newBatchSummary` | 1.07× parity (9.9 vs 10.6 µs) | PARITY — **evidence string fixed** (old note "1.15x-win stem family" was a stale v2 reference) |
| 4 | `ChunkDependencies.oldImmutableListSummary` | g5 old-side (1.1 µs) — pair 0.972 | PARITY — unchanged |
| 5 | `ChunkDependencies.arraySummary` | 1.03× parity | PARITY — unchanged |
| 6 | `DensitySplineContext.oldWrapperSummary` | g10 old-side (1.9 µs) — pair 1.004 | PARITY — unchanged |
| 7 | `DensitySplineContext.newDirectSummary` | 1.00× parity | PARITY — **evidence string fixed** ("newDirect 3.29x stem" cited the *interpolator-slice* win, now 3.32x, different class) |
| 8 | `EntityLookupStatus.oldStatusSummary` | g14 pair 0.999 (1.1 µs) | PARITY — unchanged |
| 9 | `NoiseInterpolatorFractions.divisionSummary` | **no pair in the canonical report** (§4) | PARITY (batch surface) — unchanged, flagged |
| 10 | `ClimateRTree.buildTreeHandle` | **no pair in the canonical report** (§4) | PARITY (batch surface) — unchanged, flagged |
| 11 | `ClimateRTree.nativeBuildTreeHandle` (full-name form) | **no pair in the canonical report** (§4) | PARITY (batch surface) — unchanged, flagged |

### 3.4 `DO_NOT_WIRE` (4 entries — e5c4fad already synced; re-verified, nothing drifted)

| Kernel | Pre-e5c4fad | 2026-09-08 rerun | Registry now | Verdict |
|---|---:|---:|---:|---|
| `LevelChunkHeightmap.newCombinedUpdateSummary` | 5.55× | 5.700 (2.8 ms → 15.8 ms) | 5.70 ✓ | REGRESSION — reproduced |
| `MarkerCache.cachedSummary` | 4.69× | 4.540 (150.9 → 685.0 µs) | 4.54 ✓ | REGRESSION — reproduced |
| `PalettedReencodeScratch.directPackedSummary` | 2.30× | 2.346 (493.6 µs → 1.2 ms) | 2.35 ✓ | REGRESSION — reproduced |
| `ProtoChunkHeightmap.newCachedContainsSummary` | 1.77× | 1.775 (1.2 → 2.1 µs) | 1.78 ✓ | REGRESSION — reproduced |

Remaining stale artifact fixed: the `DO_NOT_WIRE` doc comment still said "P500 2026-09-07
120ms-batch rerun" (UTC-named old label) — harmonized to the canonical report name
(2026-09-07T16:46Z = 2026-09-08 UTC+8, 49 groups / 70 pairs).

## 4. Registry-vs-report inconsistencies NOT resolved (documented, left as-is)

1. **5 batch-surface entries have no pair in the canonical rerun:**
   `TicketSetSearch.binarySummary` / `.uncheckedBinarySummary`,
   `NoiseInterpolatorFractions.divisionSummary`, `ClimateRTree.buildTreeHandle` /
   `nativeBuildTreeHandle` — the 2026-09-08 rerun contains 0 mentions of these classes.
   Their `P500 PARITY (batch surface)` verdicts rest on earlier evidence (pre-rerun
   reports / batch calibration); they are infrastructure (caller-initiated dispatch of
   already-registered pointers), so no gate behavior depends on a fresh number, but a
   future calibration rerun should add them as groups so every registry entry traces to
   the canonical report. NOT fixed here (would require a bench run — forbidden in this
   task; CPU busy with sibling benches).
2. **Two NEW rerun WINs are not in the registry (documented, deliberately NOT added):**
   `NoiseChunkFlatCacheContext.newTrueContextSummary` (1.24×, ratio 0.805) and
   `.newFalseContextSummary` (1.18×, ratio 0.846), both 0.0%-stable. Adding them would
   WIDEN the gate (new `Allow` inputs) — that is a promotion decision, not an evidence
   sync. They are recorded here as the top future promotion candidates, pending the
   BATCH_WIRING_PLAN lifecycle (P500 WIN + live verification).
3. **0.906 vs 0.917** for `AquiferSurfaceSampling` (rounded-median artifact, see §3.1
   note) — both parity; resolved in favor of the report's own 0.906.
4. `P500_REPORT.md ≡ v2 byte-identical` claim from TASK-12's predecessor notes applies to
   the *predecessor* v2-era file pair, not to the current pair: the current
   `P500_REPORT.md` (49 groups) differs from `P500_REPORT_v2.md` (v2-era absolutes quoted
   in old evidence strings, e.g. 217.9 ns vs 116.1 ns for PluginLoadingAllocation) — the
   rerun is a different dataset. No action.

## 5. Wiring-eligibility delta

* **Hot-path swap / promotion candidates: 7 → 4.** `PluginLoadingAllocation`
  (`newLazyValidateSummary`, `newLazyMissingSetSummary`) and
  `AquiferSurfaceSampling.newBatchSummary` lose "P500 WIN promotion candidate" status —
  a hot path must NOT be swapped to them on win-evidence grounds (they are parity).
  They remain **batch-dispatch-eligible**: BATCH_WIRING_PLAN §B.2.3 precondition (a)
  ("single-call verdict parity or better") now holds *honestly* — this matches
  BATCH_ADOPTION_MATRIX_wave2 §4 rows #4/#5/#7 ("wire for batching, not kernel swap").
* **Batch dispatcher `Allow` set: unchanged (12 ids).** `decide()`/`decide_id()` return
  the same `Decision` for every input; `ERR_KERNEL_REFUSED` behavior identical.
* **DO_NOT_WIRE: unchanged (4 kernels).** All four regressions reproduce on the rerun;
  no promotion/demotion.
* **Net gate-widening: zero.** Net gate-narrowing: zero (verdict labels only).

## 6. Risk note

The only behavioral surface touched by this sync is *documentation inside registry
metadata* (verdict/evidence strings and comments). The gate's `Allow`/`KeepJava` sets are
identical before and after, so the batch-wiring gate, `ERR_KERNEL_REFUSED` semantics, and
both live wirings behave exactly as before; the drift-guard tests (registry ↔ jni_table
symbol checks, whitelist/regression disjointness) are unaffected because no (class,
kernel) pair moved between registries and no ratio crossed a threshold in the
DO_NOT_WIRE direction. The residual risk is *decision-quality*, not behavior: until now,
batch-wiring proposals for g30/g2 could have cited non-reproducible 1.5×/1.15× wins;
after this sync such citations are impossible from the registry. If a future rerun
re-demonstrates a win for the three reclassified kernels, re-promotion follows the
normal lifecycle (§Lifecycle in docs/KERNEL_POLICY.md), not a registry edit.

## 7. Files changed by this sync

* `src/kernel_policy.rs` — PROVEN_WINS verdict/evidence strings (7 WIN-section entries),
  2 batch-surface evidence strings, DO_NOT_WIRE doc comment; no code/logic changes.
* `docs/KERNEL_POLICY.md` — PROVEN_WINS registry section replaced with the synced
  tables; entry count note 4+11 → 4+23.
* `docs/PROVEN_WINS_SYNC.md` — this document.
