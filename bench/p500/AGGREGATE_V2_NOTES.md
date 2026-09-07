# aggregate_p500.py v2 — notes (Task B7)

`aggregate_p500.py` extends the P500 aggregator with stem-pairing diagnostics,
a noise-model classification, a stability column, JNI-floor flagging and
baseline tracking. CLI-compatible with v1: same positional TSV input, same
markdown-on-stdout output (run_p500.sh `tee` keeps working), same legacy
report sections — everything below is additive.

## CLI

    python3 aggregate_p500.py [TSV]                       # v1-compatible report (now v2-enriched)
    python3 aggregate_p500.py --self-test                 # embedded fake-TSV test, exit 0 = pass
    python3 aggregate_p500.py [TSV] --write-baseline PATH # snapshot current pair ratios as new baseline
    python3 aggregate_p500.py [TSV] --baseline PATH       # diff against a baseline (default: <tsv dir>/baseline.tsv if present)

## 1. Pairing (P500 stem rule, now explicit + diagnosed)

An alt kernel pairs with the old kernel whose **stem** — the method name after
stripping ONE leading kind prefix — shares the **longest common suffix** with
the alt's stem; ties broken by raw-name suffix, then old median (v1 behavior).

* Kind prefixes stripped: `old new direct cached scratch reused lazy index
  branch array helper lambda` plus variant words observed in real kernel names
  (`optimized guarded hooked mutable eager threadLocal subtractFirst
  targetFirst`). Exactly one prefix is stripped — never iterate (avoids
  `oldNewArraySummary` → `Summary` over-stripping and false exact matches).
* Cross-stem comparisons are refused (the old "0.01x WaypointHotPath
  regression" class of bug): `optimizedWaypointManagerValue` can only pair
  `oldWaypointManagerValue`, never `oldReallyFarValue`.
* Report section `## Pairing diagnostics` prints:
  * **UNPAIRED kernels** — no OK old in group (e.g. g38
    `oldDistanceSummary`), no alt, non-OK status or non-positive median;
  * **MULTI-pair warnings** — one old claimed by several alts (9 in the
    current run: g4, g6, g15, g16, g17, g24, g27, g29, g31) or ambiguous
    stem-suffix ties.

Current run: 70 pairs, 1 unpaired, 9 multi-pair warnings (all many-to-one:
one old, several alt variants — expected for variant-sweep groups).

## 2. Classification + stability

* ratio = **alt/old**; **WIN ≤ 0.85**, **REGRESSION ≥ 1.18**, PARITY between
  (mirrors the ±15% noise floor). Repeated measurements of the same kernel
  (retries/passes) collapse via **min-of-medians**.
* Legacy sections keep v1 semantics (speedup = old/alt, wins ≥ 1.15x,
  regressions ≤ 0.85x) so old reports stay comparable.
* **stability** = relative spread `(max−min)/median` of the underlying batch
  samples (TSV min/max columns); for a pair the worse (larger) of the two
  kernels is shown; `n/a` when unavailable. Verdicts get a `(noisy)` marker
  when pair stability > 50%.

## 3. New report sections

* `## Regressions (do-not-wire)` — kernel (alt / paired old), group, ratio,
  stability, verdict. Current run: 4, identical to the committed baseline
  findings: LevelChunkHeightmap 5.550x, MarkerCache 4.687x,
  PalettedReencodeScratch directPacked 2.297x, ProtoChunkHeightmap 1.752x.
* `## Wins (promotion candidates)` — sorted by ratio (best first). Current
  run: NoiseChunkBlendCache 0.004 (244x), NoiseInterpolatorSlice 0.304 (3.29x),
  PluginLoadingAllocation lazy 0.645/0.653, ImprovedNoiseInline switchGradient
  0.822, PalettedReencodeScratch scratchThreadLocal 0.833. (Two v1 wins —
  ImprovedNoiseInline arithmetic 1.16x and AquiferSurfaceSampling 1.15x — fall
  inside the stricter ≤ 0.85 WIN boundary and are now PARITY.)
* `## JNI floor groups` — any kernel median < 200 ns ⇒ batch-API candidate
  (per-kernel micro-optimization is pointless at the JNI transition floor;
  canon 35–90 ns per P500_REPORT_v2.md — the "~115 ns" phrasing here was
  stale pre-audit prose, TASK-33 errata). Current run: 11 groups (g18, g28,
  g30, g31, g32, g33, g35, g36, g39, g40, g42).

## 4. Baseline tracking

* `--write-baseline PATH` writes a compact snapshot (not the raw TSV):
  `PAIR\tclass\tsig\told_kernel\talt_kernel\tratio(alt/old)\tstability`,
  one line per formed pair. `results/baseline.tsv` (70 pairs) was created
  from the current run as the tracking baseline.
* `load_baseline()` also accepts a raw bench TSV — it re-runs the same
  pairing pipeline — so any old `p500_raw.tsv` can serve as a baseline.
* When a baseline exists, the report gains `## Baseline diff`: per-pair
  `drift = (current ratio − baseline ratio) / baseline ratio`, flagged
  `DRIFT >±20%`; pairs absent on either side are marked `NEW PAIR` /
  `MISSING IN CURRENT RUN`.
* Verified end-to-end: a synthetic 1.5x slowdown of `cachedSummary` shows
  `+50.0% | **DRIFT >±20%**`; the clean run shows 70/70 `ok` at ±0.0%.

## 5. Self-test

`--self-test` runs 29 assertions on an embedded 6-row fake TSV (exact-stem
pair → WIN 0.5, kind-prefix pair → REGRESSION 1.25, cross-stem trap → correct
old chosen, PARITY 1.0, CRASH-note parsing, stability 0.10, JNI-floor
non-flagging at 300 ns, section presence, classification boundaries
0.85/1.18) plus a 3-row many-to-one fixture asserting the MULTI warning.
Exit 0 on pass. Current status: **PASSED 29/29**.

## Regenerating the v2 report

    cd bench/p500
    python3 aggregate_p500.py results/p500_raw.tsv --baseline results/baseline.tsv \
        > results/P500_REPORT_v2.md

The committed `results/P500_REPORT.md` (v1 baseline) is intentionally left
untouched; `run_p500.sh` still tees its aggregate to it and can be switched
to `P500_REPORT_v2.md` whenever the owner decides to re-baseline.
