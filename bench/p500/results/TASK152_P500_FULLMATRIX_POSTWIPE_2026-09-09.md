# TASK-152 — P500 FULL-MATRIX POST-WIPE (agent-7625532f, 2026-09-09)

Status: **PASS (full-matrix revalidation grade)** — all 70 committed baseline pairs
reproduced on the rebuilt post-wipe environment, 0 drift-flags at the pre-registered
20% bar, max |drift| 5.0%. The kernel lane's optimization ledger is fully revalidated
after sandbox wipe #2. This closes the environment question TASK-151 opened at
sanity grade (4/49 groups); no rig or product changes were made.

## Context and method

Direct continuation of TASK-151 (floor-sanity, 4/49 groups). The remaining 45 groups
(1–11, 13–15, 17–18, 20–48) were measured in 8 chunked foreground runs — the rig's
designed pattern (`P500_APPEND=1`) — using the identical environment as TASK-151:

- canonical `bench/p500/run_p500.sh` **UNMODIFIED** (verified by drift-guard discipline);
- `javac` = `~/bin/javac` shim → ToolProvider API (jdk.compiler 21.0.12.1, same as
  system `/usr/bin/java`); `JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64` bound for
  the rig's `set -u`;
- bench JVM = system OpenJDK 21.0.12.1 (Debian build; same upstream as the wiped
  Temurin 21.0.12.1); committed `.so` assets repo-fresh;
- one JVM per group, time-bounded batches ~120 ms, median of 5, `-Xbatch`.

## Run integrity

118 RESULT rows this run (45 groups) + 11 rows from the TASK-151 subset = the full
matrix. **0 CRASH rows; 118/118 kernels `OK s0`** (stability 0.0% — the fully
deterministic-batch trait first observed in TASK-151 persists across all 49 groups;
observed, not investigated, no rig changes; anti-gate-shopping). No retry-to-N=16/1
was needed by any group. Raw: `P500_FULLMATRIX_POSTWIPE_RAW_2026-09-09.tsv` (163
lines incl. SINKs). Canonical `p500_raw.tsv` / `P500_REPORT.md` restored to HEAD
after the runs.

## Baseline diff — 70/70 pairs

`results/baseline.tsv` holds 70 PAIR rows; drift computed as (current−baseline)/baseline,
flag bar |Δ|>20% (rig canon). Paired from this run (63) + TASK-151 subset (7, marked T151).

- **compared 70, missing 0, flagged 0**; drift mean +0.13%, median +0.12%, max 5.0%.
- Top |drift| pairs — all on the JNI micro-cost floor (~87–96 ns kernels, where
  single-digit-ns noise scales into percent):

| class | pair | baseline | current | drift |
|---|---|---:|---:|---:|
| PluginNameLog | oldTreeset / newArrayListSort | 0.944 | 0.991 | +5.0% |
| LegacyProvidedAliasRemoval | oldValuesRemoveIf / newReverseAliasRemove | 1.048 | 1.002 | −4.4% |
| PluginLoadingAllocation | oldDefaultCapacity / newPresized | 0.970 | 0.995 | +2.6% |
| MarkerCache | oldSummary / cachedSummary | 4.540 | 4.656 | +2.6% |
| PluginDirectoryScan | oldWalkDepth1 / newList | 0.939 | 0.918 | −2.3% |
| LevelChunkHeightmap | oldFourUpdate / newCombinedUpdate | 5.700 | 5.576 | −2.2% (T151) |
| PluginMetaDependency | oldStream / cached | 0.974 | 0.995 | +2.2% |
| AquiferPositionalLocation | oldBatch / directBatch | 0.993 | 1.010 | +1.7% |

- Win census (current ratios): 19 alt-wins (<0.98), 43 parity, 8 alt-slower (>1.02).

## Canonical verdicts reproduce, both directions

WIN-class promotion candidates (ratio ≤ 0.85) — all reproduce:

| class | pair | baseline | current | note |
|---|---|---:|---:|---|
| NoiseChunkBlendCache | old/newEmptyBlender | 0.003 | 0.003 | 315x-class win, exact |
| NoiseInterpolatorSlice | oldJagged / flat | 0.301 | 0.299 | 3.34x |
| NoiseChunkFlatCacheContext | old/newTrueContext | 0.805 | 0.798 | 1.25x |
| PalettedReencodeScratch | oldNewArray / scratchThreadLocal | 0.834 | 0.842 | 1.19x |
| NoiseChunkFlatCacheContext | old/newFalseContext | 0.846 | 0.844 | 1.18x |
| ImprovedNoiseInline | oldPMethod / switchGradient | 0.819 | 0.822 | band 0.82–0.88 exact (T151) |

Registered do-not-wire REGRESSIONS (ratio ≥ 1.18) — all reproduce at their baseline
magnitudes (these are optimization targets, not accidents): MarkerCache cached
4.656 (base 4.540), PalettedReencodeScratch directPacked 2.352 (base 2.346),
ProtoChunkHeightmap newCachedContains 1.773 (base 1.775), LevelChunkHeightmap
newCombinedUpdate 5.576 (base 5.700, T151 subset).

JNI floor anchors reproduce: StaticCacheGet 33.5 ns (canon 34.6), RangeChoice
83.2 ns (canon 81.4); 13 floor groups identified, matching the committed floor map.

## CORRECTION (append-only, supersedes TASK-151 prose, not its numbers)

TASK-151's doc and CLAIMS row phrased LevelChunkHeightmap 5.700→5.576 as "the known
5.7x combined-update **win** reproduces". That SEMANTIC is inverted: the canonical
pre-wipe `P500_REPORT.md` classifies this pair as **REGRESSION (do-not-wire)**
(speedup 0.18x; old kernel 2.2 ms vs new 12.4 ms; ledger §99 INDEX row same flaw).
The ratio numbers and the −2.2% drift in TASK-151 were correct; only the win/loss
label was wrong. This run's do-not-wire table (above) restores the canonical
classification. Lesson recorded: cite the aggregator's classification section, not
re-derived prose, when summarizing pair direction.

## Honest caveats

- Distro JDK build differs from the pre-wipe baseline runs (Debian vs Temurin,
  same upstream 21.0.12.1) — bounded by max 5.0% drift at kernel scale (two
  datapoints now: TASK-151's 2.2%, this run's 5.0% on a ~90 ns kernel).
- n=1 run per group per side; stability 0.0% everywhere remains atypically
  deterministic vs pre-wipe runs — consistent post-wipe environment trait.
- The +5.0%/−4.4% extremes sit on JNI-floor kernels where the absolute delta is
  ~4 ns; no action implied for any wired path.
- Server-dependent channels (TASK-150 phase-2a execution, C3 cadence) remain
  blocked until `/home/z/server` + jdk21 are restored.

## Verdict

FULL-MATRIX **PASS**: every committed old-vs-optimized pair — wins, parity, and
registered regressions alike — reproduces within the pre-registered 20% drift bar
on the rebuilt environment. The P500 ledger (baseline.tsv) remains the trusted
canonical reference for kernel-lane work. INJECTS-ONLY intact: 0 server boots,
0 product changes, bench-lane JVM flags only.
