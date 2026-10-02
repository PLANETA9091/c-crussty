# JNI-Floor Adoption Matrix — wave-2 companion (pair-level grades, K-projections, wiring gate)

* Author: agent-7625532f (TASK-12-sub, wave-2 proc-1), 2026-09-08. ANALYSIS ONLY — no `.rs`
  changes, no bench runs, no server starts.
* Canonical numbers: `bench/p500/results/P500_REPORT.md` @ HEAD — **byte-identical** to
  `P500_REPORT_v2.md` (verified by `diff`; one canonical 2026-09-08 rerun: 49 groups / 129
  kernels / 70 pairs / 0 CRASH, REAL ~120 ms batches, commit 3baa0f7). Floor band **35-90 ns**
  per the TASK-10 erratum (`crussty-dev-logs/c-crussty/review-session003-agents2-commits.md`);
  NOT the stale "~115 ns" phrase still present in the report's own floor-section prose.
* **Sibling doc — read together:** `docs/BATCH_ADOPTION_MATRIX.md` (TASK-12-w1, d02fbc2) is the
  group-level matrix (49 rows, tiers A-D, R-model, S1/S2/E loads). This wave-2 file is the
  union-complement, kept as a separate file per the TASK-03/TASK-11 double-delivery precedent:
  (1) per **kernel-pair** floor table (old vs optimized, ns/op, ratio), (2) H/M/L grades justified
  per argument shape from `native/JNI_EXPORTS.manifest`, (3) per-kernel projected ns/op at
  **K=64/256/4096**, (4) ranked wire-candidate table + DO-NOT-WIRE, (5) `kernel_policy::decide_id()`
  wiring-gate mechanics, (6) additional data inconsistencies. Where the two docs differ in a
  grading label, both rubrics are stated and reconciled (§2.1).

---

## 1. Floor inventory (pair level)

Selection rule (mission): any pair whose **min(median ns/op)** is ≤ 225 ns (~2.5× the 90 ns upper
floor edge), plus the strictly in-band kernels (35-90 ns). Pairing = the aggregator's P500 stem
rule re-applied offline (reproduces exactly 70 pairs, 1 unpaired, 9 multi-pair warnings).

Result: **13 groups / 17 old↔alt pairs / 32 kernels** ≤ 225 ns. Strictly inside 35-90 ns:
6 kernels (3 pairs: g35, g39, g40). Lower anchor: g42 at 34.6/34.7 ns — just *below* the stated
35 ns edge (wave-1 anomaly #1; the band is effectively [34.6, 90]).

| g# | class | sig | old kernel | old ns/op | alt kernel | alt ns/op | ratio alt/old | verdict |
|---:|---|---|---|---:|---|---:|---:|---|
| 42 | `PaperNativeStaticCacheGet` | `(IIIII[I[J)I` | `oldBatchSummary` | 34.7 | `newBatchSummary` | 34.6 | 0.997 | parity |
| 35 | `PaperNativeRangeChoice` | `([D[I[I[II[J)I` | `oldFillArraySummary` | 81.4 | `optimizedFillArraySummary` | 81.6 | 1.002 | parity |
| 39 | `PaperNativeSpigotLoadOrderDependency` | `(I[Ljava/lang/Object;[J)I` | `oldLoadAfterBuildSummary` | 87.7 | `newLoadAfterBuildSummary` | 88.0 | 1.003 | parity |
| 40 | `PaperNativeSpigotLoadOrderDependency` | `(I[Lobj;[Lobj;[Lobj;I[J)I` | `oldRemovedCountSummary` | 89.0 | `newRemovedCountSummary` | 88.6 | 0.996 | parity |
| 36 | `PaperNativeRemapperIndexCleanup` | `(I[Lobj;×5[J)I` | `oldEagerCleanupSummary` | 91.3 | `newLazyCleanupSummary` | 91.6 | 1.003 | parity |
| 28 | `PaperNativePluginClassLoaderGroup` | `(I[Lobj;[IILjava/lang/String;[J)I` | `oldLookupSummary` | 91.7 | `skipRequesterSummary` | 92.4 | 1.008 | parity |
| 24 | `PaperNativeObfHelperMaps` | `([Lobj;×2[I[I[Lobj;×6[J)I` | `oldStreamDefaultSummary` | 93.5 | `directMapsSummary` | 93.3 | 0.998 | parity |
| 24 | `PaperNativeObfHelperMaps` | (same) | `oldStreamDefaultSummary` | 93.5 | `presizedStringPoolSummary` | 93.6 | 1.001 | parity |
| 33 | `PaperNativePluginStartupRollup` | `(I[Lobj;Ljava/lang/String;[Lobj;[Lobj;[J)I` | `oldSummary` | 96.1 | `newSummary` | 95.3 | 0.992 | parity |
| 18 | `PaperNativeLegacyProvidedAliasRemoval` | `(I[Lobj;[Lobj;I[J)I` | `oldValuesRemoveIfSummary` | 109.6 | `newReverseAliasRemoveSummary` | 114.9 | 1.048 | parity |
| 30 | `PaperNativePluginLoadingAllocation` | `(I[Lobj;×3I[J)I` | `oldEagerMissingSetSummary` | 115.1 | `newLazyMissingSetSummary` | 114.6 | 0.996 | parity |
| 32 | `PaperNativePluginNameLog` | `(I[Lobj;[Lobj;[J)I` | `oldTreesetSummary` | 115.5 | `newArrayListSortSummary` | 109.0 | 0.944 | parity |
| 30 | `PaperNativePluginLoadingAllocation` | (same as above) | `oldEagerValidateSummary` | 116.1 | `newLazyValidateSummary` | 115.3 | 0.993 | parity |
| 30 | `PaperNativePluginLoadingAllocation` | (same as above) | `oldDefaultCapacitySetupSummary` | 116.2 | `newPresizedSetupSummary` | 112.7 | 0.970 | parity |
| 9 | `PaperNativeDensityAp2MinMaxFill` | `(III[J)I` | `oldSummary` | 119.8 | `newSummary` | 120.2 | 1.003 | parity |
| 31 | `PaperNativePluginMetaDependency` | `(I[Lobj;[Z[Z[I[J)I` | `oldStreamSummary` | 121.0 | `cachedSummary` | 117.8 | 0.974 | parity |
| 31 | `PaperNativePluginMetaDependency` | (same) | `oldStreamSummary` | 121.0 | `newLoopSummary` | 120.7 | 0.998 | parity |

`Lobj;` = `Ljava/lang/Object;`. All 17 ratios are inside the ±15% parity band — consistent with
"floor-sitting": both sides of every pair pay one JNI transition per call and the bodies are
≤ ~30 ns. Parity between old and alt is *expected* here; the optimization lever is batching, not
kernel selection (wave-1 §5.3.5 says the same).

## 2. Applicability grades H/M/L (argument-shape rubric, per kernel)

Rubric (mission): **H** = flat primitive arrays / fixed-shape record batches, no callbacks,
non-mutating or trivially copied. **M** = object arrays / mixed. **L** = callback-dependent,
stateful, mutating inputs, or already <40 ns.

Facts used from `native/JNI_EXPORTS.manifest` + `src/batch_api.rs`/`BATCH_API_PROPOSAL.md` §5:
all 283 exports are static bridges taking only their declared parameters (no callbacks anywhere);
the trailing `[J` is the summary/dst array (zero-copy, shape-A convention); primitive arrays can
be packed into one flat arena per batch (`GetPrimitiveArrayCritical` once), while
`[Ljava/lang/Object;`/`String` refs cannot be packed and cost one `GetObjectArrayElement` per ref
slot per op (~3-10 ns each).

* **H — 4 kernels (2 pairs):**
  * g9 `DensityAp2MinMaxFill` old/new `Summary` — `(III[J)I`: 3 scalars + 1 dst `[J`; archetype
    shape-A′ record; lowest-risk packing in the whole set.
  * g35 `RangeChoice` old/optimized `FillArraySummary` — `([D[I[I[II[J)I`: fixed-shape record of
    5 primitive arrays + 1 int (config/choice-table fill); packing cost sits in the upper half of
    the marginal band (5 slice descriptors/op) but stays arena-packed (no per-op ref fetch).
* **M — 26 kernels (14 pairs):** g18, g24 (×2 pairs), g28, g30 (×3), g31 (×2), g32, g33, g36,
  g39, g40 — every one of these signatures carries 2-10 `[Ljava/lang/Object;`/`String` slots
  (manifest rows 191-217), i.e. mixed object-array marshalling that the batch must re-fetch per op.
* **L — 2 kernels (1 pair):**
  * g42 `StaticCacheGet` old/new `BatchSummary` — 34.6/34.7 ns is **already <40 ns**: the
    amortizable headroom is ≤ ~28 ns/op while the batch marginal (3-9 ns) is 9-26% of T1; nothing
    to amortize at realistic K. (No callback-dependent, stateful or mutating-input kernel exists
    in the floor set: the known stateful/caching kernels — `MarkerCache.cachedSummary`,
    `PalettedReencodeScratch.directPackedSummary` — are µs-scale and do-not-wire anyway.)

### 2.1 Reconciliation with the wave-1 grading

Wave-1 grades g42 **HIGH** (it is the tightest floor anchor and the proposed BatchFloor
acceptance-gate kernel: measure the dispatcher's own per-op overhead against the cheapest kernel).
This doc grades g42 **L** under the mission's per-kernel adoption rubric (<40 ns ⇒ nothing to
amortize). Both are correct for their purpose: use g42 to *calibrate* the dispatcher, not as a
*savings* target. Wave-1 grades g32/g35/g39/g40 HIGH as marshal-dominated single-op groups; under
the kernel-shape rubric g35 stays H (primitive-arena packable) while g32/g39/g40 are M (2-3
object-ref slots re-fetched per op — real but smaller wins). No contradiction in the numbers;
only the lens differs.

## 3. Savings model and K-projections

Formula (both proposals, re-based on the erratum floor):

```
T(K)   = m + C/K                       per-op cost of a K-op batch
C      = w+c ≈ 35-90 ns                ONE JNI transition per batch (erratum floor band)
m      = marginal per-op element cost  3-9 ns   (P500_SCALING N=1→16 slopes: 3.1-8.7 ns/el)
S(K)   = T1 / T(K)                     projected gain; T1 = measured single-call ns/op
```

Scenario parameters: **best** C=35, m=3; **mid** C=62.5, m=6; **worst** C=90, m=9.
Fixed projection points (identical for every kernel because C, m are kernel-independent):
T(64) = 3.5 / 7.0 / 10.4 ns, T(256) = 3.1 / 6.2 / 9.4 ns, T(4096) = 3.0 / 6.0 / 9.0 ns
(best/mid/worst). If `BATCH_API_PROPOSAL.md` §8's separate `c_fill ≈ 5-12 ns/op` is added on top
of m (direct-buffer record fill), the worst case moves to ~21 ns/op — the M-adjusted column below
already covers that regime.

**M-adjusted model** (this doc's addition): for object-ref signatures the dispatcher pays
`GetObjectArrayElement` per ref slot per op: `m_M = m + 6 ns × #ref-slots` (mid; range 3-10 ns per
slot). Ref-slot counts from the manifest signatures: g39=2, g18/g32=3, g30/g40/g28=4, g33/g31=5,
g36=6, g24=10. This is the physical reason the TASK-10 erratum projected only ~1.3-3× for
ref-heavy shapes: the transition amortizes, the ref re-fetching does not.

### 3.1 Per-pair projections (K=64/256/4096)

"Naive" = mission model applied to the paired **old** kernel T1 (gain-x band = best..worst at that
K). "M-adj" = mid `T1/(m_M + C/K)` at K=256 for M pairs (H pairs use mid m). All values ns/op.

| pair (g# old→alt) | T1 | T(64) best..worst (gain) | T(256) best..worst (gain) | T(4096) best..worst (gain) | M-adj T(256) (gain) |
|---|---:|---|---|---|---|
| g42 oldBatch→newBatch | 34.7 | 3.5-10.4 (9.8-3.3×) | 3.1-9.4 (11.1-3.7×) | 3.0-9.0 (11.5-3.8×) | **L — excluded** |
| g35 oldFillArray→optimizedFillArray | 81.4 | 3.5-10.4 (22.9-7.8×) | 3.1-9.4 (26.0-8.7×) | 3.0-9.0 (27.1-9.0×) | 7.2 (11.3×) |
| g39 oldLoadAfterBuild→newLoadAfterBuild | 87.7 | 3.5-10.4 (24.7-8.4×) | 3.1-9.4 (28.0-9.4×) | 3.0-9.0 (29.2-9.7×) | 18.2 (4.8×) |
| g40 oldRemovedCount→newRemovedCount | 89.0 | 3.5-10.4 (25.1-8.6×) | 3.1-9.4 (28.4-9.5×) | 3.0-9.0 (29.6-9.9×) | 30.2 (2.9×) |
| g36 oldEagerCleanup→newLazyCleanup | 91.3 | 3.5-10.4 (25.7-8.8×) | 3.1-9.4 (29.1-9.8×) | 3.0-9.0 (30.3-10.1×) | 42.2 (2.2×) |
| g28 oldLookup→skipRequester | 91.7 | 3.5-10.4 (25.9-8.8×) | 3.1-9.4 (29.2-9.8×) | 3.0-9.0 (30.5-10.2×) | 30.2 (3.0×) |
| g24 oldStreamDefault→directMaps | 93.5 | 3.5-10.4 (26.4-9.0×) | 3.1-9.4 (29.8-10.0×) | 3.0-9.0 (31.1-10.4×) | 66.2 (1.4×) |
| g24 oldStreamDefault→presizedStringPool | 93.5 | (same) | (same) | (same) | 66.2 (1.4×) |
| g33 oldSummary→newSummary | 96.1 | 3.5-10.4 (27.1-9.2×) | 3.1-9.4 (30.6-10.3×) | 3.0-9.0 (31.9-10.7×) | 36.2 (2.7×) |
| g18 oldValuesRemoveIf→newReverseAliasRemove | 109.6 | 3.5-10.4 (30.9-10.5×) | 3.1-9.4 (34.9-11.7×) | 3.0-9.0 (36.4-12.1×) | 24.2 (4.5×) |
| g30 oldEagerMissingSet→newLazyMissingSet | 115.1 | 3.5-10.4 (32.5-11.1×) | 3.1-9.4 (36.7-12.3×) | 3.0-9.0 (38.3-12.8×) | 30.2 (3.8×) |
| g32 oldTreeset→newArrayListSort | 115.5 | 3.5-10.4 (32.6-11.1×) | 3.1-9.4 (36.8-12.4×) | 3.0-9.0 (38.4-12.8×) | 24.2 (4.8×) |
| g30 oldEagerValidate→newLazyValidate | 116.1 | 3.5-10.4 (32.7-11.2×) | 3.1-9.4 (37.0-12.4×) | 3.0-9.0 (38.6-12.9×) | 30.2 (3.8×) |
| g30 oldDefaultCapacitySetup→newPresizedSetup | 116.2 | 3.5-10.4 (32.8-11.2×) | 3.1-9.4 (37.0-12.4×) | 3.0-9.0 (38.6-12.9×) | 30.2 (3.8×) |
| g9 oldSummary→newSummary | 119.8 | 3.5-10.4 (33.8-11.5×) | 3.1-9.4 (38.2-12.8×) | 3.0-9.0 (39.8-13.3×) | 6.2 (19.2×) |
| g31 oldStream→cachedSummary | 121.0 | 3.5-10.4 (34.1-11.6×) | 3.1-9.4 (38.6-12.9×) | 3.0-9.0 (40.2-13.4×) | 36.2 (3.3×) |
| g31 oldStream→newLoopSummary | 121.0 | (same) | (same) | (same) | 36.2 (3.3×) |

Read: the naive model's 10-40× gains at K≥256 apply only to the **H** pairs and only when the
dispatcher's per-op decode really stays in the 3-9 ns band; every object-ref pair collapses to
**1.4-4.8×** under per-ref re-fetching — which reproduces the TASK-10 erratum's "~1.3-3× @K≥32"
for the marshal-heavy majority. If TASK-24 (alloc tax, below) is not fixed first, subtract a
further ~2-6 ns/op at K=256 (8 allocs/batch ≈ 0.5-1.5 µs at K=256, `HOTSPOT_CANDIDATES.md` C3).

## 4. Top-10 wire candidates (ranked) and DO-NOT-WIRE

Ranked by **risk-adjusted absolute saving** at K=256 mid (`T1 − T_M-adj(256)`); naive gain-x and
the M-adj gain shown; confidence = grade ∩ stability (all floor pairs have 0.0% spread, so
confidence tracks the shape rubric).

| # | g# | kernel pair (old→alt) | grade | adj saving ns/op | naive gain-x | adj gain-x | confidence | risk / precondition |
|---:|---:|---|---|---:|---:|---:|---|---|
| 1 | 9 | `oldSummary` → `newSummary` | H | 113.6 | 19.2× | 19.2× | HIGH | LOW — shape-A′ `(III[J)I`; wire only after JFR proves per-section call amplification (wave-1 §5.3.3) |
| 2 | 35 | `oldFillArraySummary` → `optimizedFillArraySummary` | H | 74.2 | 13.0× | 11.3× | HIGH | LOW-MED — new shape D (5-slice record); bench-N ≈ config-load, production freq unknown |
| 3 | 32 | `oldTreesetSummary` → `newArrayListSortSummary` | M(3) | 91.3 | 18.5× | 4.8× | MED | MED — sort semantics must survive batching; 2 `[Lobj;`+`[J` |
| 4 | 30 | `oldDefaultCapacitySetupSummary` → `newPresizedSetupSummary` | M(4) | 86.0 | 18.6× | 3.8× | MED | MED — v1 "1.55× WIN" is parity in the fresh rerun (§6.1): wire for *batching*, not kernel swap |
| 5 | 30 | `oldEagerValidateSummary` → `newLazyValidateSummary` | M(4) | 85.9 | 18.6× | 3.8× | MED | MED — same; PROVEN_WINS evidence stale (§6.1), re-verify before promotion |
| 6 | 18 | `oldValuesRemoveIfSummary` → `newReverseAliasRemoveSummary` | M(3) | 85.4 | 17.6× | 4.5× | MED | MED — enable/reload-wave loop |
| 7 | 30 | `oldEagerMissingSetSummary` → `newLazyMissingSetSummary` | M(4) | 84.9 | 18.4× | 3.8× | MED | MED — same signature as #4/#5 |
| 8 | 31 | `oldStreamSummary` → `cachedSummary` | M(5) | 84.8 | 19.4× | 3.3× | MED | MED — **not** the do-not-wire `MarkerCache.cachedSummary`; different class, parity here |
| 9 | 31 | `oldStreamSummary` → `newLoopSummary` | M(5) | 84.5 | 19.3× | 3.3× | MED | MED — same site, alternative alt |
| 10 | 39 | `oldLoadAfterBuildSummary` → `newLoadAfterBuildSummary` | M(2) | 69.5 | 14.1× | 4.8× | MED | LOW-MED — cheapest M (2 ref slots); O(P²) pair loop at boot/reload |

*Watch list (below the wire bar):* g28 `PluginClassLoaderGroup` 61.5 ns (3.0×), g33
`PluginStartupRollup` 59.9 ns (2.7×), g40 `SpigotLoadOrderDependency` 58.8 ns (2.9×), g36
`RemapperIndexCleanup` 49.1 ns (2.2×), g24 `ObfHelperMaps` 27.3 ns (1.4×) — wire only if JFR shows
call amplification; g24's 10 ref slots make it the least attractive M of all.

**DO-NOT-WIRE (4 confirmed scale-invariant regressions, 2026-09-08 rerun; enforced by
`kernel_policy::DO_NOT_WIRE` since e5c4fad):**

| kernel | class (g#) | ratio | batch relevance |
|---|---|---:|---|
| `newCombinedUpdateSummary` | `PaperNativeLevelChunkHeightmap` (g19) | 5.70× | µs-ms scale — batch-irrelevant AND policy-refused |
| `cachedSummary` | `PaperNativeMarkerCache` (g20) | 4.54× | stateful per-call cache bookkeeping; do not confuse with g31 `cachedSummary` (parity) |
| `directPackedSummary` | `PaperNativePalettedReencodeScratch` (g27) | 2.35× | packed-direct shape loses at every N |
| `newCachedContainsSummary` | `PaperNativeProtoChunkHeightmap` (g34) | 1.78× | cached-contains shape loses at every N |

Plus the L-grade exclusion: g42 `StaticCacheGet` (34.6/34.7 ns) — not a wiring *savings* target
(this doc), though it remains the dispatcher calibration kernel (wave-1).

## 5. ms/tick scenario estimates

All figures are ESTIMATE-pending-bench (roadmap honesty rule); production call frequency is the
dominant unknown — P500 measures cost, not frequency (wave-1 §2.1). Parameterized first, then
reconciled with the wave-1 reference loads.

Saved ms/tick = `f × (T1 − T(K)) / 1e6` with a 50 ms/20-TPS budget. Mean floor T1 ≈ 100 ns;
batched per-op mid: H ≈ 6-7 ns, M ≈ 18-66 ns (ref-slot count dependent; ≈35 ns mean).

| floor-kernel calls/tick | old cost | batched H (T≈6.5ns) | saved H | batched M (T≈35ns) | saved M |
|---:|---:|---:|---:|---:|---:|
| 1 000 | 0.100 ms | 0.0065 ms | **0.094 ms/tick** | 0.035 ms | **0.065 ms/tick** |
| 10 000 | 1.000 ms | 0.065 ms | **0.94 ms/tick** | 0.35 ms | **0.65 ms/tick** |
| 100 000 | 10.0 ms | 0.65 ms | **9.4 ms/tick** | 3.5 ms | **6.5 ms/tick** |

Assumptions: (a) f is per-tick sustained floor-kernel volume on one thread (the model is
per-call-linear; contention ignored); (b) batch K=256 (C/K ≤ 0.35 ns — below model noise);
(c) plugin-domain kernels (18/24/28/30/31/32/33/36/39/40) are **0/tick steady-state** — their
savings land per boot/reload event, not per tick; (d) worldgen H kernels (g9, g35) are the only
plausible per-tick floor traffic, at 10²-10³ calls/tick at reference chunk rates (8 chunks/tick ×
24 sections × ~6 floor calls ≈ 1.2k calls/tick).

Reconciliation with wave-1 (independent models, agrees within ~±50%): wave-1 S1 settled tick
≈ 0.003 ms/tick, S2 worldgen-burst ≈ 0.047 ms/tick, E boot/reload ≈ 0.83 ms/event. This model at
wave-1's own frequencies: S1 ≈ 50 calls × ~70 ns ≈ 0.0035 ms/tick ✓; S2 ≈ 1.3k calls × ~35 ns ≈
0.046 ms/tick ✓; E ≈ 11k calls × ~70 ns ≈ 0.77 ms/event ✓.

**Bottom line (both models):** at reference loads the JNI-floor band is worth **≤ 0.1% of one
tick** (≈0.03-0.07 ms) and ≈0.8 ms per boot/reload event. ≥1 ms/tick requires ≥12k-40k amplified
single-op floor calls/tick on one kernel (scaling identity: R ns/call ≡ R ms per 10⁶ calls) —
provable only by a JFR/async-profiler call-site profile (proposal step 0). The structural case
(marshal-dominated single-op loops, H pairs) stands; the *tick-budget* case is currently
unproven.

## 6. Data inconsistencies found (wave-2 additions; §6.1 is new vs wave-1 §6)

1. **`kernel_policy.rs::PROVEN_WINS` evidence strings are stale vs the canonical 2026-09-08
   rerun** (DO_NOT_WIRE was synced in e5c4fad, PROVEN_WINS was not):
   * `PluginLoadingAllocation.newLazyValidateSummary` "P500 WIN (1.55×) 217.9→140.6 ns" — fresh
     rerun: `oldEagerValidateSummary` 116.1 vs `newLazyValidateSummary` 115.3 → ratio 0.993
     **parity**; `newLazyMissingSetSummary` "1.53×" → 0.996 **parity**.
   * `AquiferSurfaceSampling.newBatchSummary` "WIN (1.15×) 6.3→5.5 µs" — fresh: 6.0→5.5 µs →
     0.917 **parity** (WIN_MAX=0.85).
   * `NoiseChunkBlendCache.newEmptyBlenderSummary` "244× (67.0 µs→274.5 ns)" — fresh: 316×
     (95.3 µs→301.1 ns); verdict unchanged, absolute drifted.
   * Still-valid WINs: `NoiseInterpolatorSlice` 3.32×, `ImprovedNoiseInline.switchGradientSummary`
     1.22×, `PalettedReencodeScratch.scratchThreadLocalSummary` 1.20×.
   No entry became a regression (the whitelist stays *safe*), but 3 of 7 "P500 WIN" verdict
   labels are no longer reproducible from the canonical report — needs an evidence-sync commit
   (not made here: analysis only).
2. **`baseline.tsv` is self-referential for this run:** all 70 drift rows in the report show
   baseline ratio == current ratio (±0.0%). The baseline was captured from the same rerun, so the
   TASK-14 drift gate becomes meaningful only from the *next* run. Not an error — an
   initialization fact worth documenting so a future "0% drift everywhere" report is not mistaken
   for a rigged one.
3. **`P500_REPORT.md` and `P500_REPORT_v2.md` are byte-identical** (verified by `diff`). The
   v1/v2 naming implies two datasets; there is one canonical rerun. Any doc quoting "v1
   absolutes" must use the *pre-3baa0f7* report from git history, not this file.
4. **Timestamp convention:** the report header says `Generated: 2026-09-07T16:46:41+00:00` (UTC)
   while CLAIMS/worklog date the rerun 2026-09-08 — the sandbox runs UTC+8, so both denote the
   same event (2026-09-08T00:46+08). Future docs should state the TZ explicitly.
5. **"44+ floor groups" (CLAIMS TASK-12 row) is a stale count** — fully recounted in wave-1
   erratum #2: it was 44 *kernels* in 18 groups under the obsolete ~115 ns floor; under the
   corrected floor: 13 groups / 32 kernels ≤200-225 ns (this §1), 16 groups ≤360 ns.
6. **`src/batch_table.rs` ids 0-11 cover zero floor kernels:** all are µs-scale (`g0`
   AquiferIndexStride 9.9-10.6 µs, g5 1.1 µs, g10 1.9 µs, g14 1.1 µs). Good for machinery
   validation (shapes A/B), but the floor set needs the shape extensions of §7 before any
   adoption-matrix kernel is servable.
7. **g42 vs the 35 ns band edge** (also wave-1 anomaly #1): 34.6 ns sits below the stated lower
   bound; treat the band as [34.6, 90].

## 7. Wiring plan — every batch route goes through `kernel_policy::decide_id()`

The gate exists (`src/kernel_policy.rs` @ e5c4fad: `decide()` / `decide_id(Class.kernel)` /
`audit_wire()`, modes strict|audit|off, default-deny). The batch dispatcher is a *wiring site*
like any byte-hook: a servable `kernelId` routes hot-path volume to a native kernel.

0. **Prerequisite — TASK-24 / HOTSPOT_CANDIDATES C3:** `src/batch_api.rs` still does 8 heap
   allocations + zeroing per `run()`. At the proposal's auto-threshold K=16 that tax
   (~0.5-1.5 µs/batch ≈ 30-90 ns/op) equals the floor it is meant to amortize — the dispatcher
   would burn in malloc what it saves in JNI. Move the control-plane `Vec`s into the existing
   per-thread `SCRATCH` (batch_api.rs:201-203) and bench `BatchFloorBench` K∈{1,8,16,64,256,4096}
   before/after. Do not wire before this.
1. **Gate at table build AND at serve time:** for every `batch_table.rs` entry `(class, kernel,
   shape)`, require `kernel_policy::decide_id("Class.kernel") == Decision::Allow` before the id
   becomes servable; refused ids get a new negative error (extend the `ERR_*` set, e.g.
   `ERR_POLICY_DENIED`) instead of silent service. In `audit` mode log via
   `audit_wire(class, kernel, "batch_api::init")`; pair with
   `debug_assert!(decide_id(id).is_allowed())` at init (the pattern improved_noise already uses
   since e5c4fad).
2. **Default-deny consequence:** all 32 floor kernels are currently "unproven" → `KeepJava`.
   Batch adoption therefore REQUIRES evidence-based `PROVEN_WINS` promotion — new verdict
   strings of the form `"batch WIN @K=256 (per-op ≤ X ns; SINK parity OK)"` after a
   `BatchFloorBench` A/B — never gate weakening; `CRUSSTY_KERNEL_POLICY=off` stays an A/B-rig-only
   bypass (never live).
3. **Tests to ship with the wiring:** (a) every servable id passes `decide_id` (table-build
   fails loudly otherwise); (b) hand-adding any of the 4 DO-NOT-WIRE kernels to the batch table
   is refused; (c) unknown id → `ERR_BAD_KERNEL_ID`; (d) audit-mode logs one WIRE line per id.
4. **Extension order for the floor set (shapes):** `batch_table.rs` has no floor coverage today
   (§6.6). Add: shape A′ `(III[J)I` → g9; shape D `([D[I[I[II[J)I` (5-slice record) → g35; then
   refArgs-bearing shapes for g39 (2 slots) → g32/g18 (3) → g30/g28/g40 (4) → g31/g33 (5),
   measuring the per-op `GetObjectArrayElement` decode cost in BenchFloor at each step (the M-adj
   model of §3 predicts the win halves for every +4 ref slots). Each added id passes the TASK-14
   ratio-gate plus SINK-parity vs single calls (roadmap §6.1 acceptance).
5. **Parity guard:** shared 2-CPU box ⇒ ±15% noise band; a batch kernel whose per-op cost does
   not beat single-call T1 by >15% at target K is a no-wire (the g24 class of outcomes).

## 8. Sources

* `bench/p500/results/P500_REPORT.md` (≡ `P500_REPORT_v2.md`, verified identical; 3baa0f7 rerun
  dataset) — all ns/op medians, pairing diagnostics, do-not-wire table, floor-group list.
* `bench/p500/results/P500_SCALING.md` — N=1/16/256/4096 probes; marginal element 3.1-8.7 ns
  (m-band source); scale-invariance of the 4 regressions.
* `bench/p500/aggregate_p500.py` — stem-pairing rule (re-implemented offline; 70/1/9 reproduced).
* `native/JNI_EXPORTS.manifest` rows 15-217 — argument signatures behind every H/M/L grade.
* `src/kernel_policy.rs`, `docs/KERNEL_POLICY.md` (d02fbc2) — `decide_id()`/`audit_wire()` gate,
  DO_NOT_WIRE ratios (e5c4fad sync), PROVEN_WINS (stale evidence, §6.1).
* `src/batch_api.rs`, `src/batch_table.rs`, `docs/BATCH_API_PROPOSAL.md` — dispatcher constants,
  ERR_*, shapes A/B, refArgs encoding (§5), auto-threshold; `HOTSPOT_CANDIDATES.md` C3/TASK-24.
* `docs/BATCH_ADOPTION_MATRIX.md` (wave-1, d02fbc2) — group-level matrix, S1/S2/E loads,
  reconciliation target for §5.
* `docs/OPTIMIZATION_ROADMAP.md` §5.2 — wave-3 "Batch-API adoption" placeholder (this file is the
  referenced TASK-12 input; all absolutes here are ESTIMATE-pending-bench under the 35-90 ns
  floor).

---
*TASK-12-sub (wave-2, proc-1) · agent-7625532f · 2026-09-08 · analysis only, no code touched.*
