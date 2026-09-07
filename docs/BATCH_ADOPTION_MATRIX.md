# JNI-Floor Adoption Matrix — batch-API applicability per P500 group (TASK-12)

* Author: agent-7625532f (TASK-12-w1), 2026-09-08. Analysis only — no code changes.
* Canonical baseline: `bench/p500/results/P500_REPORT_v2.md` (49 groups / 129 kernels, 0 crashes,
  REAL ~120 ms batches, commit 3baa0f7) + `P500_SCALING.md` (N-probes) + erratum in
  `crussty-dev-logs/c-crussty/review-session003-agents2-commits.md` (TASK-10).
* Companion design: `docs/BATCH_API_PROPOSAL.md` (dispatcher design is unchanged; its §8 win
  arithmetic is superseded by this document — see errata).

---

## 0. TL;DR

* With the corrected JNI transition floor **35-90 ns** (measured anchors: `StaticCacheGet` 34.6 ns,
  `RangeChoice` 81.4 ns, `SpigotLoadOrderDependency` 87.7-89.0 ns; `(I)D` kernels at N=1 = 19.9-31.2 ns),
  **16 of 49 groups** are floor-relevant (Tier A floor-resident ×4, B floor-dominant ×9, C floor-relevant ×3);
  the report's own <200 ns floor list is **13 groups / 32 kernels**.
* Batch-API applicability: **HIGH ×5** groups (g32, g35, g39, g40, g42 — all single-op, marshal-dominated),
  **MEDIUM ×10**, **LOW ×34** (body- or IO-dominated; includes all 4 do-not-wire regressions).
* Estimated recoverable cost at explicit reference loads: **S1 settled tick ≈ 0.003 ms/tick**,
  **S2 worldgen-burst tick ≈ 0.047 ms/tick** (range 0.028-0.066),
  **E boot/reload wave ≈ 0.83 ms/event** (range 0.64-1.02).
* Scaling identity: **R ns/call ≡ R ms per 10⁶ calls** — a kernel with R=60 ns/call recovers 60 ms
  per 10⁶ calls. ≥1 ms/tick requires ≥12k-40k amplified single-op calls/tick on one kernel —
  a JFR/async-profiler-profilable threshold (roadmap §3.1) before any wiring.
* Plugin-side adoption only: dispatcher bridge + call-site/byte-hook swaps in c-crussty (`src/`),
  never `native/*.so`, never engine edits, never the 4 confirmed regressions.

---

## 1. ERRATA — read before any other batch document

1. **The JNI transition floor is 35-90 ns, NOT ~115 ns.** The BATCH_NS audit (commit 3baa0f7,
   TASK-10 review) showed the pre-fix bench (120 µs batches) never reached stable C2, producing a
   bimodal baseline. After the fix (120 ms batches, `-Xbatch`, AlwaysPreTouch): global minimum
   `PaperNativeStaticCacheGet` 34.6 ns; floor-resident cluster 81.4-89.0 ns; `(I)D` N=1 = 19.9-31.2 ns
   (`P500_SCALING.md`). Everything below is **obsolete**:
   * `docs/BATCH_API_PROPOSAL.md` §1/§8 ("F ≈ 115 ns", "15-35x", "~100 ns/op saved @K=64") —
     re-derive with §4 of this document; scenario wins for floor-band groups drop to **~1.3-3x @K≥32**
     (TASK-10 erratum).
   * `crussty-dev-logs/c-crussty/batch-api-proposal.md` (e0d6e06) — same.
   * `docs/OPTIMIZATION_ROADMAP.md` §2 "The ~115 ns JNI floor insight" table (112-220 ns band).
   * `P500_REPORT.md` (v1) absolute ns/op — superseded by `P500_REPORT_v2.md` entirely.
   * `P500_SCALING.md` absolute ns/op (pair ratios remain valid — scale-invariant).
   * `P500_REPORT_v2.md` §"JNI floor groups" PROSE still says "~115 ns measured in the scaling
     study" — stale text contradicting its own data (StaticCacheGet 34.6 ns); the group list and
     numbers there are correct.
2. **"44+ groups on the floor" recount.** The figure originates from CLAIMS TASK-03 ("44+ groups
   plugin/loading sit on the floor"), derived when the floor was believed ~115 ns: the 112-220 ns
   band A (13 groups / 32 kernels) + 375-782 ns band B (+5 groups) summed to **44 KERNELS in 18
   groups** — a kernel count, not 44 groups. With the corrected floor:
   * Tier A floor-resident (every kernel ≤90 ns): **4 groups** (g35, g39, g40, g42) / 8 kernels.
   * ≤2× floor ceiling (best kernel ≤180 ns): **13 groups** / 32 kernels — identical set to the
     report's own "JNI floor groups" list (<200 ns criterion: 13 groups / 32 kernels).
   * ≤4× floor ceiling (best ≤360 ns): **16 groups** / 38 kernels (adds g13, g21, g44).
   This matrix therefore carries **49 rows (all groups)** — no group is hand-waved away — and rates
   batch applicability per group instead of asserting a stale count.
3. **`run_p500.sh` still references a non-existent `libpaper_chunk_encode_jni.so`** (only
   `libpaper_native_chunk_encode_jni.so` ships; the `[ -f ]` guard saves the run). Cosmetic; noted in
   the TASK-10 review.

---

## 2. Method: from ns/op to recoverable ns/call

Per-op cost model: `T = F(sig) + B` where `F` = Java→native transition (incl. argument
marshalling) and `B` = kernel body. A batch of K ops pays F once plus per-op decode:
per-op overhead `≈ F/K + d + c/K` with `d ≈ 2-4 ns` (scalar decode + jvalue assemble + indirect
C call, `BATCH_API_PROPOSAL.md` §8) and `c ≈ 50-100 ns` per batch (pin + stride checks + rets).
At K=64: **4-7 ns/op** (F/K term shrank 115→35-90, so the proposal's 3-8 ns/op gate still stands).

Recoverable per call = the transition the batch removes:

```
F_lo(sig) = min(20 ns + 5 ns·n_primref + 12 ns·n_objref, 87 ns)
            # 20 ns base transition: (I)D kernels at N=1 = 19.9-31.2 ns (P500_SCALING)
            # primitive-array refs ≈5 ns each (one jobject handle, zero-copy)
            # object-array/String refs ≈12 ns each (TASK-10: [LObject;/String/[J marshal ≈ 40-70 ns of the floor)
R_lo = F_lo − 3 ns                # conservative (body fully survives the batch)
R_hi = min(T_best − 3 ns, 87 ns)  # max (body-free case; 87 = observed transition ceiling)
R_mid = (R_lo + R_hi) / 2         # used for all totals below
```

Sanity anchors: g42 R=27-32 ns against T=34.6 ns (78-92% of op); g24 (8 object refs) R≈84-87 ns
against T=93.3 ns (90-93%). Both bounds behave.

### 2.1 Reference loads (model inputs — VALIDATE WITH JFR BEFORE WIRING)

P500 measures kernel cost, not production call frequency; frequency is the dominant unknown
(roadmap §3.1: profile first). Three explicit reference loads:

| Scenario | Definition | Non-zero frequency assignments |
|---|---|---|
| **S1** settled tick (50 ms) | no chunkgen; 200 active entities; per-player waypoint refresh | g13: 40/tick; g47: 40/tick (20 value N=1 + 20 manager) |
| **S2** worldgen-burst tick | 8 chunks/tick; entity churn; full waypoint scan | g9: 192/tick (24 sections/chunk); g13: 192/tick; g21: 8/tick; g44: 16/tick; g47: 1088/tick (1024 value N=1 + 64 manager) |
| **E** boot/reload wave | 30 plugins; O(P²)/2 dependency pair checks; 200 cache-gets & range-choices/plugin; 10 of each other kernel/plugin | g42: 6000; g35: 6000; g39/g40: 900 each; g18/24/28/30/31/32/33/36: 300 each |

Plugin-domain kernels are **0/tick steady-state** (event-driven); their savings are per E-event.
Sensitivity: any kernel becomes ≥1 ms/tick at `10⁶/R_mid` calls/tick (12k-40k for the R range here).

---

## 3. Matrix A — floor-relevant groups (Tiers A/B/C, 16 rows)

Tier A = every kernel ≤90 ns (floor-resident); B = best ≤180 ns (floor-dominant); C = best ≤360 ns
(floor-relevant). ns/op from `P500_REPORT_v2.md` (min-of-medians across batch strategies).

| g# | group | sig | ns/op best–worst (k) | tier | batch | R ns/call lo–hi | S1/S2 calls/tick | S1/S2 µs/tick | E µs/event | adoption path (plugin-side only) |
|---:|---|---|---|---|---|---|---|---|---:|---|
| 9 | `PaperNativeDensityAp2MinMaxFill` | `(III[J)I` | 119.8 ns–120.2 ns (2) | B | MEDIUM | 22–87 | 0/192 | 0.00/10.46 | 0 | Byte-hook on the section fill loop (area_map pattern: loader-local helper + 64-case selftest + env gate) ONLY after JFR confirms per-section call amplification. |
| 13 | `PaperNativeEntityChunkTransient` | `(IIJ[J)I` | 291.6 ns–293.7 ns (2) | C | MEDIUM | 22–87 | 40/192 | 2.18/10.46 | 0 | Byte-hook entity transient-write loop → dispatchRepeat over entities; env gate; area_map-pattern selftest; JFR-gate first. |
| 18 | `PaperNativeLegacyProvidedAliasRemoval` | `(I[Ljava/lang/Object;[Ljava/lang/Object;I[J)I` | 109.6 ns–114.9 ns (2) | B | MEDIUM | 46–87 | 0/0 | 0.00/0.00 | 20 | dispatchRepeat over plugin alias lists at enable/reload waves; refArgs v1 required. |
| 21 | `PaperNativeNoiseChunkBlendCache` | `(II[J)I` | 301.1 ns–95.30 µs (2) | C | LOW (batch) / HIGH (wire) | 22–87 | 0/8 | 0.00/0.44 | 0 | Do NOT batch — wire newEmptyBlenderSummary via blender-construction byte-hook (roadmap §2.3). |
| 24 | `PaperNativeObfHelperMaps` | `([Ljava/lang/Object;[Ljava/lang/Object;[I[I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I` | 93.3 ns–93.6 ns (3) | B | MEDIUM | 84–87 | 0/0 | 0.00/0.00 | 26 | dispatchRepeat over obf map-pair ops at boot/reload; heaviest refArgs group — measure decode cost in BenchFloor first. |
| 28 | `PaperNativePluginClassLoaderGroup` | `(I[Ljava/lang/Object;[IILjava/lang/String;[J)I` | 91.7 ns–92.4 ns (2) | B | MEDIUM | 51–87 | 0/0 | 0.00/0.00 | 21 | dispatchRepeat over plugin classloader lookups at enable wave; env gate. |
| 30 | `PaperNativePluginLoadingAllocation` | `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I` | 112.7 ns–116.2 ns (6) | B | MEDIUM | 58–87 | 0/0 | 0.00/0.00 | 22 | Adopt lazy kernels first (kernel swap), then dispatchRepeat over the per-plugin setup loop. |
| 31 | `PaperNativePluginMetaDependency` | `(I[Ljava/lang/Object;[Z[Z[I[J)I` | 117.8 ns–121.0 ns (3) | B | MEDIUM | 49–87 | 0/0 | 0.00/0.00 | 20 | dispatchRepeat over plugin meta pairs during dependency resolution. |
| 32 | `PaperNativePluginNameLog` | `(I[Ljava/lang/Object;[Ljava/lang/Object;[J)I` | 109.0 ns–115.5 ns (2) | B | HIGH | 46–87 | 0/0 | 0.00/0.00 | 20 | Canonical BenchFloor target #2: dispatchRepeat over plugin-name list at boot/reload; ThreadLocal scratch per proposal §6. |
| 33 | `PaperNativePluginStartupRollup` | `(I[Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Object;[J)I` | 95.3 ns–96.1 ns (2) | B | MEDIUM | 70–87 | 0/0 | 0.00/0.00 | 24 | dispatchRepeat rollup per plugin at enable. |
| 35 | `PaperNativeRangeChoice` | `([D[I[I[II[J)I` | 81.4 ns–81.6 ns (2) | A | HIGH | 42–78 | 0/0 | 0.00/0.00 | 361 | BenchFloor target #3: dispatchRepeat over the choice table at config load; primitive refArgs only. |
| 36 | `PaperNativeRemapperIndexCleanup` | `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;[J)I` | 91.3 ns–91.6 ns (2) | B | MEDIUM | 82–87 | 0/0 | 0.00/0.00 | 25 | dispatchRepeat per-class cleanup during remap; note runtime class-loads keep remap passes active post-boot (JFR-gate frequency). |
| 39 | `PaperNativeSpigotLoadOrderDependency` | `(I[Ljava/lang/Object;[J)I` | 87.7 ns–88.0 ns (2) | A | HIGH | 34–85 | 0/0 | 0.00/0.00 | 53 | BenchFloor target #4; dispatchRepeat over the plugin pair matrix at resolution; refArgs v1. |
| 40 | `PaperNativeSpigotLoadOrderDependency` | `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I` | 88.6 ns–89.0 ns (2) | A | HIGH | 58–86 | 0/0 | 0.00/0.00 | 65 | Same adoption as g39 (second signature of the same call site). |
| 42 | `PaperNativeStaticCacheGet` | `(IIIII[I[J)I` | 34.6 ns–34.7 ns (2) | A | HIGH | 27–32 | 0/0 | 0.00/0.00 | 176 | BenchFloor target #1 (primary): dispatchRepeat for cache-get bursts; the BatchFloor acceptance gate (≤8 ns/op @K=64) is measured against this kernel. |
| 44 | `PaperNativeTopographicGraphSortCapacity` | `(I[I[I[I[J)I` | 348.1 ns–349.0 ns (2) | C | LOW | 37–87 | 0/16 | 0.00/0.99 | 0 | None unless JFR shows per-call amplification; else fuse sort passes. |

### 3.1 Matrix A rationale (call pattern · marshalling · state affinity)

| g# | tier | production call pattern | batchability rationale |
|---:|---|---|---|
| 9 | B | worldgen · SMALL-N loop (per section) | Only worldgen group at 2-4× floor (119.8-120.2 ns, pair ratio 1.003); if production calls it per section/quart run, dispatchRepeat fuses the loop; R=22-87 ns (18-73%). |
| 13 | C | entity · SINGLE-OP (per entity-chunk transient) | T=291.6-293.7 ns ≤4× floor ceiling; R=22-87 ns/call (8-30%); entity-chunk transient rewrites are tick-periodic and loop-amplifiable. |
| 18 | B | plugin · SINGLE-OP | 3 ref slots ([LObject;×2+[J); R=46-87 ns (42-79% of T); alias sweep is a per-plugin loop at enable/reload. |
| 21 | C | worldgen · SINGLE-OP (empty-blender build) | Batch R=22-87 ns = 7-29% of alt 301.1 ns; the real 316x win is the kernel swap, not batching. |
| 24 | B | plugin · SINGLE-OP | 8 object-ref slots/op — marshalling IS most of T=93.3-93.6 ns; R=84-87 ns (90-93%); batch removes transition, keeps 8 GetObjectArrayElement/op. |
| 28 | B | plugin · SINGLE-OP | String + [LObject; marshal; T=91.7-92.4 ns; R=51-87 ns (58-95%). |
| 30 | B | plugin · SINGLE-OP | 6 kernels 112.7-116.2 ns; lazy variants already 1.55x win + sit 22-30 ns above floor (body≈25 ns); R=58-87 ns (50-77%). |
| 31 | B | plugin · SINGLE-OP | Primitive-array refs ([Z[Z[I[J) are zero-copy; T=117.8-121.0 ns; R=49-87 ns (41-74%). |
| 32 | B | plugin · SINGLE-OP | Pure per-plugin-name loop; 3 ref slots; T=109.0-115.5 ns; R=46-87 ns (40-80%); the proposal's reference kernel. |
| 33 | B | plugin · SINGLE-OP | 4 ref slots + String; T=95.3-96.1 ns; R=70-87 ns (73-91%). |
| 35 | A | plugin/config · SINGLE-OP | Floor-resident: T=81.4-81.6 ns ≤90 ns ceiling; 5 primitive-array refs (zero-copy); R=42-78 ns; body ≤36 ns. |
| 36 | B | plugin · SINGLE-OP | 6 ref slots; T=91.3-91.6 ns; R=82-87 ns (90-95%); per-class cleanup loop during remap passes. |
| 39 | A | plugin · SINGLE-OP (pair check) | Floor-resident: T=87.7-88.0 ns; dependency resolution is an O(P²) pair loop at boot/reload — archetypal batch; R=34-85 ns. |
| 40 | A | plugin · SINGLE-OP | Floor-resident: T=88.6-89.0 ns; 4 ref slots; same O(P²) loop as g39 with richer args; R=58-86 ns. |
| 42 | A | kernel-svc · SINGLE-OP (cache get) | MEASURED GLOBAL FLOOR ANCHOR: T=34.6-34.7 ns (min across all 129 kernels); pure primitives + 2 primitive-array refs; R=27-32 ns (78-92%). |
| 44 | C | worldgen · BULK (5 array refs) | R=42-87 ns = 12-25% of T=348.1-349.0 ns; body (sort) dominates. |

## 4. Matrix B — body-dominated groups (Tier D, 33 rows)

Transition share ≤ ~25% of op; batching cannot pay for itself. Listed for completeness (the
"44+"-style whole-population claim is false under the corrected floor — see erratum #2).

| g# | group | ns/op best–worst (k) | domain · pattern | batch | rationale | adoption |
|---:|---|---|---|---|---|---|
| 0 | `PaperNativeAquiferIndexStride` | 9.90 µs–10.60 µs (2) | worldgen · BULK (N=first int) | LOW | N-element batch kernel — transition already amortized in-call (≤0.4% of op); body 9.9-10.6 µs dominates. | None — body-level work only; alt is 1.07x parity. |
| 1 | `PaperNativeAquiferPositionalLocation` | 2.20 µs–2.20 µs (2) | worldgen · BULK | LOW | Position-vector sig already call-batched; body 2.2 µs ≫ floor. | None. |
| 2 | `PaperNativeAquiferSurfaceSampling` | 5.50 µs–6.00 µs (2) | worldgen · BULK | LOW | Body 5.5-6.0 µs; transition ≤1.6%. | None — 1.15x parity win only. |
| 3 | `PaperNativeBlendedNoise` | 50.90 µs–51.90 µs (2) | worldgen · BULK | LOW | Blended-noise body 50.9 µs ≫ floor. | None — adjacent path owned by improved_noise hook. |
| 4 | `PaperNativeCaveCarverSkip` | 34.20 ms–34.60 ms (3) | worldgen · BULK (SLOW-lane) | LOW | 34.2-34.6 ms body; floor share 0.0003%. | None — carver body work only. |
| 5 | `PaperNativeChunkDependencies` | 1.10 µs–1.10 µs (2) | worldgen · BULK | LOW | Transition ≤8% of 1.1 µs. | None — 1.03x parity. |
| 6 | `PaperNativeClimateParameterDistance` | 522.8 ns–526.8 ns (3) | worldgen · VECTORIZED ([J×4 per call) | LOW | Sig takes position vectors: call-level batching built in; transition ≤17% of 523 ns only if called per-position (it is not). | None — fuse more positions per call if JFR shows per-position calls. |
| 7 | `PaperNativeCubicSplineCreate` | 153.30 µs–153.50 µs (2) | worldgen · BULK | LOW | Spline-build allocation body 153 µs. | None. |
| 8 | `PaperNativeDensityAp2Fill` | 11.30 µs–22.20 µs (4) | worldgen · BULK | LOW | Transition ≤0.8% of 11.3-22.2 µs. | None — wire scratchFlat win (1.15x) instead. |
| 10 | `PaperNativeDensitySplineContext` | 1.90 µs–1.90 µs (2) | worldgen · BULK | LOW | Body 1.9 µs. | None. |
| 11 | `PaperNativeDensityVisitorHook` | 672.4 ns–678.6 ns (2) | worldgen · LOOP (per visitor) | LOW | Loop-amplified but body ≈640 ns ≫ floor; R≤87 = ≤13%. | None — body work. |
| 12 | `PaperNativeEntityBoundingBox` | 5.70 µs–5.80 µs (2) | entity · BULK | LOW | Body 5.7 µs. | None — 0.97x parity. |
| 14 | `PaperNativeEntityLookupStatus` | 1.10 µs–1.10 µs (4) | entity · BULK | LOW | 4 kernels at 1.1 µs; transition ≤8%. | None. |
| 15 | `PaperNativeImprovedNoiseDerivative` | 11.50 µs–11.70 µs (4) | noise · BULK | LOW | Body 11.5-11.7 µs. | None. |
| 16 | `PaperNativeImprovedNoiseInline` | 7.60 µs–9.30 µs (5) | noise · BULK | LOW | Body 7.6-9.3 µs; switchGradient win (1.22x) already adjacent to hook surface. | None. |
| 17 | `PaperNativeJigsawCanAttach` | 626.4 ns–628.3 ns (3) | structure · LOOP (per piece) | LOW | Per-piece attach checks loop, but body ≈600 ns ≫ floor. | None. |
| 19 | `PaperNativeLevelChunkHeightmap` | 2.80 ms–15.80 ms (2) | chunk · BULK | LOW (+REGRESSION) | alt = 5.7x REGRESSION (P500_REPORT_v2 do-not-wire). | None — never wire newCombinedUpdateSummary. |
| 20 | `PaperNativeMarkerCache` | 150.90 µs–685.00 µs (2) | chunk · BULK | LOW (+REGRESSION) | cachedSummary = 4.54x REGRESSION. | None — never wire. |
| 22 | `PaperNativeNoiseChunkFlatCacheContext` | 18.70 µs–23.50 µs (4) | worldgen · BULK | LOW | Body 18.7-23.5 µs; wins 1.18-1.24x. | None — wire wins only. |
| 23 | `PaperNativeNoiseInterpolatorSlice` | 1.90 ms–6.30 ms (2) | worldgen · BULK | LOW | flatSummary 1.9 ms vs oldJagged 6.3 ms (3.32x) — body-dominated. | None — flat-kernel wire (roadmap §2.3). |
| 25 | `PaperNativeOreFeatureLoop` | 1.90 µs–1.90 µs (2) | worldgen · BULK | LOW | 16-param sig (stride-16 assert case); body 1.9 µs. | None. |
| 26 | `PaperNativeOwnableRule` | 3.70 µs–4.20 µs (2) | worldgen · BULK | LOW | Body 3.7-4.2 µs; 1.12x parity. | None. |
| 27 | `PaperNativePalettedReencodeScratch` | 411.90 µs–1.20 ms (3) | chunk-encode · BULK | LOW (+REGRESSION) | directPacked = 2.35x REGRESSION; scratch win 1.20x. | None — wire scratchThreadLocal variant only. |
| 29 | `PaperNativePluginDirectoryScan` | 1.90 ms–2.10 ms (3) | plugin · IO-dominated | LOW | Directory walk 1.9-2.1 ms — disk IO; batching irrelevant. | None. |
| 34 | `PaperNativeProtoChunkHeightmap` | 1.20 µs–2.10 µs (2) | chunk · SINGLE-OP | LOW (+REGRESSION) | newCachedContains = 1.78x REGRESSION; R≤87 = ≤7% of 1.2 µs. | None — never wire. |
| 37 | `PaperNativeRemapperSkipHashes` | 33.00 µs–34.50 µs (2) | plugin · BULK + String | LOW | Hash loop body 33.0-34.5 µs ≫ floor. | None. |
| 38 | `PaperNativeServerEntityDeltaIdentity` | 2.20 µs–2.20 µs (1) | entity · BULK (6×[D vector) | LOW | Vector sig already call-batched; single UNPAIRED old kernel (no alt). | None — needs an alt kernel first. |
| 41 | `PaperNativeSpringFeatureMutablePos` | 2.90 µs–2.90 µs (2) | worldgen · BULK | LOW | Body 2.9 µs. | None. |
| 43 | `PaperNativeSurfaceRulesTestRuleState` | 1.10 µs–1.20 µs (2) | worldgen · SINGLE-OP (per rule state) | LOW | Per-state checks loop but body ≈1.1 µs ≫ floor. | None. |
| 45 | `PaperNativeWaypointDistanceGuard` | 9.50 µs–9.60 µs (2) | waypoint · BULK (7×[D) | LOW | Vector sig already call-batched; body 8.4-9.6 µs. | None — 1.02x parity. |
| 46 | `PaperNativeWaypointDistanceGuard` | 8.40 µs–8.50 µs (2) | waypoint · BULK (6×[D) | LOW | Same; 0.99x parity. | None. |
| 47 | `PaperNativeWaypointHotPath` | 938.4 ns–100.00 µs (10) | waypoint · MIXED: value kernels = N=1 loop; manager = BULK | MEDIUM | (I)D at N=1 = 19.9-31.2 ns ≈ pure transition (P500_SCALING) — per-waypoint loops are the HIGH case (R=17-28 ns/call); manager kernels (93.3-100.0 µs @ bench N≈256) are bulk — body-dominated. | If JFR shows per-waypoint N=1 calls: dispatchRepeat over waypoint ids; manager path = body work, never batch. |
| 48 | `PaperNativeXoroshiroPositionalDirect` | 1.50 µs–1.60 µs (4) | worldgen · BULK | LOW | Xoroshiro positional batch; body 1.5-1.6 µs. | None. |

---

## 5. Summary

### 5.1 Totals (arithmetic per row: `calls × R_mid → µs`; full sums over all 49 rows)

Per-row arithmetic (only rows with non-zero frequency):

```
  * g47 `PaperNativeWaypointHotPath`: S1 40×22 ns = 0.90 µs; S2 1088×22 ns = 24.48 µs
  * g9 `PaperNativeDensityAp2MinMaxFill`: S1 0×54 ns = 0.00 µs; S2 192×54 ns = 10.46 µs
  * g13 `PaperNativeEntityChunkTransient`: S1 40×54 ns = 2.18 µs; S2 192×54 ns = 10.46 µs
  * g44 `PaperNativeTopographicGraphSortCapacity`: S1 0×62 ns = 0.00 µs; S2 16×62 ns = 0.99 µs
  * g21 `PaperNativeNoiseChunkBlendCache`: S1 0×54 ns = 0.00 µs; S2 8×54 ns = 0.44 µs
```

* **S1 total ≈ 0.003 ms/tick; S2 total ≈ 0.047 ms/tick** (R_lo..R_hi envelope: 0.028-0.066 ms/tick).
* **E boot/reload wave ≈ 0.83 ms/event** (envelope 0.64-1.02) —
  one-off cost during a multi-second boot; real but cosmetic for ms/tick budgets.
* **Context:** a 20-TPS budget is 50 ms/tick. Even the S2 burst envelope (0.028-0.066 ms/tick) is
  ≤0.13% of one tick. The corrected floor therefore DEMOTES per-tick batch savings from
  "meaningful" (old 115 ns model) to "micro unless call amplification is proven" — while the
  structural case for the batch API (marshal-heavy single-op groups, §3 HIGH rows) is unchanged.

### 5.2 Top-10 groups by estimated savings

| # | g# | group | best ns/op | R ns/call (lo–hi) | S2 µs/tick | E ms/event | batch |
|---:|---:|---|---:|---|---:|---:|---|
| 1 | 47 | `PaperNativeWaypointHotPath` | 938.4 ns | 17–28 | 24.48 | 0.000 | MEDIUM |
| 2 | 9 | `PaperNativeDensityAp2MinMaxFill` | 119.8 ns | 22–87 | 10.46 | 0.000 | MEDIUM |
| 3 | 13 | `PaperNativeEntityChunkTransient` | 291.6 ns | 22–87 | 10.46 | 0.000 | MEDIUM |
| 4 | 44 | `PaperNativeTopographicGraphSortCapacity` | 348.1 ns | 37–87 | 0.99 | 0.000 | LOW |
| 5 | 21 | `PaperNativeNoiseChunkBlendCache` | 301.1 ns | 22–87 | 0.44 | 0.000 | LOW (batch) / HIGH (wire) |
| 6 | 35 | `PaperNativeRangeChoice` | 81.4 ns | 42–78 | 0.00 | 0.361 | HIGH |
| 7 | 42 | `PaperNativeStaticCacheGet` | 34.6 ns | 27–32 | 0.00 | 0.176 | HIGH |
| 8 | 40 | `PaperNativeSpigotLoadOrderDependency` | 88.6 ns | 58–86 | 0.00 | 0.065 | HIGH |
| 9 | 39 | `PaperNativeSpigotLoadOrderDependency` | 87.7 ns | 34–85 | 0.00 | 0.053 | HIGH |
| 10 | 24 | `PaperNativeObfHelperMaps` | 93.3 ns | 84–87 | 0.00 | 0.026 | MEDIUM |

Rows 7-10 are plugin-domain (0/tick steady-state; ranked by E-event savings). g47 outranks g13
only under the per-waypoint N=1 loop assumption for its value kernels — pattern-gated, see §3.1.

### 5.3 Adoption order (plugin-side, no .so / engine changes)

1. **Dispatcher infrastructure** (`BATCH_API_PROPOSAL.md` §9 steps 1-4, unchanged):
   `src/batch_dispatch.rs` table built during the existing `inject_surface` symbol-resolution loop;
   `PaperNativeBatchDispatch` as a 99th plugin-owned bridge class (manifest untouched,
   `render --check`/`verify` still cover only the closed 283); BatchFloor bench group with
   acceptance gates ≤8 ns/op @K=64, N=1 dispatch ≤1.5x individual, unknown-id → negative return.
   Gate kernel: **g42 StaticCacheGet** (34.6 ns — the tightest floor anchor).
2. **First consumers = c-crussty-owned call sites:** none of the HIGH groups is currently called
   from plugin-owned Java helpers, so the first real consumers arrive with the byte-hooks below.
3. **Byte-hook consumers** (area_map pattern: loader-local helpers, 64-case deterministic selftest,
   env gate `CRUSSTY_BATCH=1` default-off): g32/g35/g39/g40/g42 loops fire inside kernel
   plugin-loading classes — hook the enclosing loop method, batch through `dispatchRepeat`.
   g13/g9/g47 value-kernel paths fire in chunkgen/entity code — hook ONLY after JFR proves
   call amplification (≥12k-40k calls/tick for ≥1 ms/tick).
4. **Never wire:** g19 newCombinedUpdateSummary, g20 cachedSummary, g27 directPackedSummary,
   g34 newCachedContainsSummary (P500_REPORT_v2 regression table).
5. **Kernel-swap beats batching where a win exists:** g21 (316x), g23 (3.32x), g22 (1.18-1.24x),
   g8 (1.15x), g30 lazy (1.55x), g16 (1.22x) — wire the alt kernel first; batching is orthogonal
   and applies only to the single-op call shells around them.

---

## 6. Anomalies found while building this matrix

1. **g42 (34.6 ns) sits below the stated 35 ns floor bound** — the band is effectively
   [34.6, 90] ns. Use 34.6 ns as the floor lower anchor in future docs.
2. **`P500_REPORT_v2.md` internal inconsistency:** its "JNI floor groups" section prose still
   cites "~115 ns measured in the scaling study" while its own minimum is 34.6 ns. Text-level fix
   recommended (numbers unchanged).
3. **g47 report medians are N-amplified:** bench args ≈ N=256 waypoints (`oldReallyFarValue`
   938.4 ns ≈ 1.0 µs @N=256 in `P500_SCALING.md`), so g47 looks Tier-D in the report while its
   N=1 costs (19.9-31.2 ns) are floor values that only exist in the SCALING probe. Consumers of
   this matrix must not read g47's 93-100 µs manager numbers as floor data.
4. **g9 has two measurement variants in `p500_raw.tsv`** (119.8/120.2 and 151.4/155.4 ns) collapsed
   by min-of-medians; raw TSV holds 185 RESULT rows vs 129 collapsed kernels — parse the report,
   not raw row counts.
5. **g38 `ServerEntityDeltaIdentity` has a single unpaired old kernel** (no alt) — excluded from
   pair verdicts by design; flagged so it is not mistaken for a missing measurement.
6. **9 multi-pair warnings** (aggregator stem rule claims one old kernel for several alts) — benign,
   documented in the report's pairing diagnostics.

---

## 7. Sources

* `bench/p500/results/P500_REPORT_v2.md` — 49 groups / 129 kernels; floor anchors; regression &
  win tables; pairing diagnostics (canonical baseline, commit 3baa0f7).
* `bench/p500/results/p500_raw.tsv`, `baseline.tsv` — raw medians (min-of-medians), 70-pair baseline.
* `bench/p500/results/P500_SCALING.md` — N=1/16/256/4096 probes; (I)D N=1 = 19.9-31.2 ns;
  scale-invariant regression verdicts.
* `bench/p500/results/P500_SCALING_WAYPOINT.md` — g47 O(N) linearity (per-element 5-15 ns).
* `docs/BATCH_API_PROPOSAL.md` — dispatcher design (§2-§7, §9 adopted as-is; §8 superseded).
* `docs/OPTIMIZATION_ROADMAP.md` — §2.2 batch phase, §3.1 profile-first rule.
* `crussty-dev-logs/c-crussty/review-session003-agents2-commits.md` — TASK-10 erratum (floor
  35-90 ns; marshalling 40-70 ns of floor for [LObject;/String/[J; scenario wins 1.3-3x @K≥32).

