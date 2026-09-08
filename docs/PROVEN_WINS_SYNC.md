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

**Channel-scope note (2026-09-08+08, TASK-66 — executes TASK-63 F1's correction
requirement):** the `live` verdicts above certify correctness (self-test PASSED on
live Purpur 1.21.10, bridge round-trips) and the P500-proven value of the noise
family applies to its BATCHED surface only. The production fresh-worldgen channel is
**measured NOT addressable for the per-call bridge design**: paired real-load A/B,
armed median 48.38 s vs 43.96 s dormant (+10.0% wall, +5.24 CPU-s, Java noise
displaced 4.12→1.19 CPU-s) — `bench/e2e/results/WORLDGEN_AB_NOISE_BRIDGE_2026-09-09.md`
F1. Ops guidance: keep `CRUSSTY_NATIVE_IMPROVED_NOISE=1` OFF for worldgen-heavy
profiles. The only remaining unrefuted lever on this channel (loop-grain batching) is
specified — with pre-registered NO-GO gates — in
`docs/WORLDGEN_BATCHING_LAYER_DESIGN.md` (TASK-66); no win is claimed for it.

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

## 4. Registry-vs-report inconsistencies (documented; item 1 resolved by TASK-51, items 2-4 left as-is)

1. **RESOLVED 2026-09-08+08 (TASK-51): 5 batch-surface entries had no pair in the canonical rerun.**
   `TicketSetSearch.binarySummary` / `.uncheckedBinarySummary`,
   `NoiseInterpolatorFractions.divisionSummary`, `ClimateRTree.buildTreeHandle` /
   `nativeBuildTreeHandle` are now **calibrated** under canonical P500 methodology
   (120 ms batches, median-of-5, REAL .so, exclusive BENCH.lock) via the
   lifecycle-aware driver `bench/p500/calib/` — measured: divisionSummary 411.2 ns
   steady (CV 0.18%); TicketSetSearch steady-state 530.4/554.4 µs with cross-call
   state confirmed (cold 6.85 ms / 0.86 ms); rtree pair = transition-floor 56–79 ns
   on opaque inputs (null-handle boundary documented; id10 ≡ id11 alias confirmed).
   See `bench/p500/results/BATCH_SURFACE_CALIBRATION.md`. Verdicts remain
   `P500 PARITY (batch surface)` (no pair ⇒ no ratio; no gate change); every
   registry entry now traces to a canonical-methodology measurement or the
   canonical report.
2. **RESOLVED 2026-09-09+08 (TASK-53): two NEW rerun WINs were not in the registry.**
   `NoiseChunkFlatCacheContext.newTrueContextSummary` (1.24×, ratio 0.805/0.804) and
   `.newFalseContextSummary` (1.18×/1.17×, ratio 0.846/0.853 — both 0.0%-stable,
   twice reproduced incl. the 2026-09-09 fresh full rerun) are now **promoted** through
   the full §Lifecycle: (a) offline semantic-parity gate — old ≡ new byte-exact on 3648
   inputs per pair (`bench/p500/parity/results/FLATCACHE_PARITY.md`); (b) promotion
   binding infrastructure — `CRUSSTY_KERNEL_PROMOTE` (default OFF, fail-safe) re-binds
   the original bridge method to the WIN symbol at registration
   (`kernel_policy.rs` `PROMOTE_PAIRS`); (c) live-armed self-test through the real
   bridge — fixtures 8/8 byte-exact vs offline old-impl expectations, bridge parity
   8/8 post-rebind (`bench/p500/parity/results/PROMOTE_E2E_2026-09-09.md`); unarmed
   boot shows zero `kernel_promote` lines (dormant-invisible). Both kernels added to
   `PROVEN_WINS` ("P500 WIN + live-verified") — the deliberate gate-widening this
   section previously deferred is now the documented promotion decision.
3. **0.906 vs 0.917** for `AquiferSurfaceSampling` (rounded-median artifact, see §3.1
   note) — both parity; resolved in favor of the report's own 0.906.
4. `P500_REPORT.md ≡ v2 byte-identical` claim from TASK-12's predecessor notes applies to
   the *predecessor* v2-era file pair, not to the current pair: the current
   `P500_REPORT.md` (49 groups) differs from `P500_REPORT_v2.md` (v2-era absolutes quoted
   in old evidence strings, e.g. 217.9 ns vs 116.1 ns for PluginLoadingAllocation) — the
   rerun is a different dataset. No action.
5. **RESOLVED 2026-09-09+08 (TASK-54): the three remaining wire-eligible WIN pairs are
   now promoted (lifecycle wave 2).** `NoiseInterpolatorSlice.flatSummary` (3.32×,
   ratio 0.301/0.301), `ImprovedNoiseInline.switchGradientSummary` (1.22×, 0.819/0.819)
   and `PalettedReencodeScratch.scratchThreadLocalSummary` (1.20×, 0.834/0.838) — all
   twice-reproduced on the 2026-09-08 rerun + 2026-09-09 fresh full rerun — went through
   the full §Lifecycle: (a) offline semantic-parity gate wave 2 (`WinPairParity.java`,
   REAL .so, exclusive BENCH.lock): old ≡ new byte-exact on 1600/3036/3072 inputs per
   pair, result + full dst, two identical runs (cross-JVM fixture determinism);
   (b) `PROMOTE_PAIRS` 2 → 5 + self-test generalized to multi-shape signatures
   (`(II[J)I` / `(IIII[J)I` / `([BI[J)I` / `(I[J)I`, sig resolved from `jni_table`,
   shape drift-guarded); (c) live e2e: unarmed boot 0 `kernel_promote` lines
   (dormant-invisible), armed boot re-binds all 5 pairs and the generalized self-test
   passes fixtures 20/20, bridge parity 20/20 (`PROMOTE_E2E_2026-09-09.md` TASK-54
   section). Registry verdicts updated to "P500 WIN + live-verified (TASK-54)".

## 5. Wiring-eligibility delta

* **Hot-path swap / promotion candidates: 7 → 4 → 2 (TASK-53) → 0 (TASK-54).**
  `PluginLoadingAllocation` (`newLazyValidateSummary`, `newLazyMissingSetSummary`) and
  `AquiferSurfaceSampling.newBatchSummary` lose "P500 WIN promotion candidate" status —
  a hot path must NOT be swapped to them on win-evidence grounds (they are parity).
  They remain **batch-dispatch-eligible**: BATCH_WIRING_PLAN §B.2.3 precondition (a)
  ("single-call verdict parity or better") now holds *honestly* — this matches
  BATCH_ADOPTION_MATRIX_wave2 §4 rows #4/#5/#7 ("wire for batching, not kernel swap").
  The two `NoiseChunkFlatCacheContext` candidates were **promoted by TASK-53**, and the
  last three wire-eligible WIN pairs (`NoiseInterpolatorSlice.flatSummary`,
  `ImprovedNoiseInline.switchGradientSummary`, `PalettedReencodeScratch.scratchThreadLocalSummary`)
  were **promoted by TASK-54** (lifecycle wave 2, §4 item 5). No unpromoted WIN verdict
  remains in the registry; the only non-promoted WIN (`BlendCache.newEmptyBlenderSummary`,
  316×) is already **live-wired** through its own first-class hook.
* **Policy `Allow` set: widened by the TASK-53 promotion (2 new inputs) and the TASK-54
  promotion (3 new inputs).**
  `decide()` now returns `Allow` for `NoiseChunkFlatCacheContext.newTrueContextSummary`
  and `.newFalseContextSummary` (pinned by
  `promoted_win_kernels_are_policy_allowed_in_strict_mode`) plus — via the same generic
  test over `PROMOTE_PAIRS` — `NoiseInterpolatorSlice.flatSummary`,
  `ImprovedNoiseInline.switchGradientSummary` and
  `PalettedReencodeScratch.scratchThreadLocalSummary`. The **batch dispatcher
  table is unchanged** (14 ids — the promoted kernels are not batch-table members), and
  DO_NOT_WIRE refusals are identical.
* **DO_NOT_WIRE: unchanged (4 kernels).** All four regressions reproduce on the rerun;
  no promotion/demotion.
* **Net gate-widening: +5 (+2 the TASK-53 promotion §4 item 2, +3 the TASK-54 promotion
  §4 item 5).** Net gate-narrowing:
  zero (verdict labels only).

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

## 8. S7-14 batch-surface additions (wire v3 wave-1: ids 15/16/17)

The descriptor-parser port (BATCH_API_PROPOSAL §4/§5) + wire-v3 ref plane
landed three more batch-surface PARITY entries (g42 precedent, §4.1 pattern):

| id | kernel pair member | ratio (baseline.tsv) | evidence |
|---|---|---|---|
| 15 | `PaperNativeRangeChoice.optimizedFillArraySummary` (shape D `([D[I[I[II[J)I`) | 1.002457 / 0.0% (line 55) | direct 81.4 ns, `p500_expected_summary.tsv` §35; contract probe-verified count-written, `bench/batch/results/WAVE1_V3_SHAPES_REPORT.md` |
| 16 | `PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary` (shape E `(I[Ljava/lang/Object;[J)I`) | 1.003421 / 0.0% (line 58) | direct 87.7 ns §39 |
| 17 | `PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary` (shape F `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`) | 0.995506 / 0.0% (line 59) | direct 88.6 ns §40 |

Measured consequence (S7-14 floor bench): batch never wins on these probe
bodies (g35 5.9-10.6x worse at every K ≤ 256) — the entries mark the kernels
BATCHABLE, not batch-profitable; the Stage-1 verdict (no measured T) is
unchanged. `docs/KERNEL_POLICY.md` count note: 4+23 → 4+26 with this wave.
