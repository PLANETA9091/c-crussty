# bench/p501_sidesurface — TASK-156 (zero-evidence native surface)

Sidecar bench for the **94 zero-evidence exports** found by the TASK-155
coverage census (`bench/p500/results/TASK155_COVERAGE_CENSUS_2026-09-10.md`,
pre-registered as TASK-156 in ledger §103). Self-contained: own generated
java tree, own classes/, own RAW/REPORT artifacts. **The canonical
`bench/p500/` tree is never touched** — no `groups.tsv`/`G*.java`
regeneration, the `old*` pairing rule stays as-is for baseline continuity.

## Pattern identity

`gen_p501_bench.py` imports `bench/p500/gen_p500_bench.py` and reuses
`load_rows/parse_params/stub_source/group_class/GROUP_IFACE/BENCH` verbatim —
the emitted driver is the canonical `Bench.java` **byte-for-byte** (same
`p500` package namespace: WARM=2, ROUNDS=5, ~120 ms time-bounded batches,
min-of-two-medians, DCE-proof sink, strategy ladder s0–s3, one JVM per group,
600 s timeout, crash-retry ladder N=16→N=1). Only the group table differs:
49 groups / 37 classes selected by the census zero-evidence set (all-requirement,
asserted = 94).

## Usage

```bash
./gen_p501_bench.py          # regenerate java/ from manifest + census TSV
./run_p501.sh                # all groups (default)
./run_p501.sh 0 1 2          # subset; P501_APPEND=1 for chunked runs
```

Artifacts (this directory only): `results/P501_SIDESURFACE_RAW_<date>.tsv`,
`results/P501_SIDESURFACE_REPORT_<date>.md`, `results/P501_ENV.txt`,
`logs/g<gid>.{out,log}`.

## Honesty rules (pre-stated in aggregate_p501.py)

* Ratios ONLY for the three §103-registered in-group conventions:
  `current→optimized`, `cold→hot`, `foreach→indexed`.
* All other multi-method groups: flat ns/op tables, spread column, **no
  direction claims**.
* Cross-group same-class siblings (e.g. ChunkExpireCount cold/hot — different
  sigs, different groups): listed, never ratioed.
* State-dependent families (CraftPlayerCanSee, Waypoint*) and handle-based
  families (ClimateRTree, FQ PerlinNoise) run with synthetic args; errors,
  garbage and crashes are reported as-is, no wiring, no gate tuning.
* The `PaperNativeChunkPacketEncode` trio is the CLOSED chunk-encode surface
  of §97: standalone rc=-3 fast-fail is the documented expectation; the
  in-server gate probe is TASK-150 phase-2a (pending `/home/z/server`).
* **FIRST MEASUREMENT**: no baseline exists for this surface — every number
  is exploratory; no drift claims; results never feed adoption gates.
* INJECTS-ONLY: 0 server boots, 0 product changes.
