# TASK-157 — P501 HANDLE-CHAIN PROBE (agent-7625532f, 2026-09-10)

Status: **PROBE COMPLETE — VERDICT H-SENTINEL (both families)** — the
pre-registered H-PASS/H-SENTINEL/H-CRASH tree (CLAIMS TASK-157, written
before any data) resolved on its H-SENTINEL branch: both handle **build**
kernels return `h = 0` on synthetic inputs (graceful sentinel, no exception,
no crash), so the handle consumers (checksum/search/free) are unprobeable
without production context. Handle lane **CLOSED for synthetic probing** —
resolves the TASK-156 parked observation completely.

## Scope and method

`bench/p501_sidesurface/HandleChainProbe.java` (hand-written sidecar-own
probe; canonical p500 + generated trees untouched): one JVM per FAMILY,
in-JVM chain build → sentinel check → batched hot-loop queries over the
SAME handle → build/free cost pools (64 builds with per-index VARIED args —
anti-memoization — each handle freed exactly once; double-free impossible by
construction). Timing = canonical Bench.java discipline: 8 calls/iter,
~120 ms budget, WARM=2 / ROUNDS=5, median; build/free = median-of-64
single calls. Env: javac ToolProvider shim + system OpenJDK 21.0.12.1 +
both native .so. INJECTS-ONLY: 0 boots, 0 product changes.

## Results

| family | op | ns/op | status |
|---|---|---:|---|
| perlin | build | - | **BUILD-SENTINEL h=0** |
| perlin | consumers | - | SKIPPED-SENTINEL |
| rtree | build | - | **BUILD-SENTINEL h=0** |
| rtree | consumers | - | SKIPPED-SENTINEL |

Verdicts: perlin **H-SENTINEL**, rtree **H-SENTINEL**. SINK emitted per
family JVM (DCE-proof evidence of clean runs). No exceptions, no aborts —
the builds fail *gracefully* on synthetic arguments, distinct from the
TASK-156 consumer SIGABRTs (garbage CONSTANT handles passed straight to
consumers).

## Interpretation and consequences

1. **Handle consumers (8 exports) are CLOSED for synthetic probing:**
   FQ ClimateRTree nativeChecksumTreeHandle / nativeFreeTreeHandle /
   nativeSearchBoundedOnePacked / nativeSearchCurrentOnePacked /
   nativeSearchBoundedBatchPacked; FQ PerlinNoise nativeGetValue /
   nativeGetValueNoYScale / nativeFreeHandle. Measuring them requires a
   REAL handle, which requires production (or production-shaped) build
   inputs — **owner-ask**, same lane as the §97 in-server gate (TASK-150
   phase-2a pending /home/z/server + jdk21).
2. **TASK-156 clarification (append-only):** the sidecar's g43/g48 "OK
   ~40ns" build rows are now EXPLAINED — the rig cannot see rc semantics
   behind a J return, and those 40ns were the sentinel fast-fail path, not
   real builds. The numbers stand as sentinel-path timings; no verdict
   change.
3. The build kernels themselves are graceful-fail on bad input (h=0, no
   panic) — a positive robustness datum, distinct from the consumer
   abort-on-garbage-handle behavior.

## Honesty

- FIRST MEASUREMENTS: no baseline, no drift claims, no wiring; the probe
  measures behavior, not performance value (sentinel path only).
- Verdict tree was pre-registered in CLAIMS before the probe ever ran;
  H-SENTINEL resolved without any gate shopping.
- INJECTS-ONLY: 0 server boots, 0 product changes; canonical bench/p500/
  untouched; sidecar tree self-contained.

## Artifacts

- `bench/p501_sidesurface/HandleChainProbe.java` — the probe (pre-registered
  verdict tree in header)
- `bench/p501_sidesurface/run_p501_handles.sh` — runner (per-family JVMs)
- `bench/p501_sidesurface/aggregate_handlechain.py` — report assembler
- `bench/p501_sidesurface/results/P501_HANDLECHAIN_{RAW_2026-09-10.tsv,
  REPORT_2026-09-10.md, ENV.txt}` — raw + report + env provenance
- `bench/p501_sidesurface/logs/handlechain_{perlin,rtree}.{out,log}` —
  per-family logs (on disk, gitignored per sidecar practice)
