# TASK-151 — P500 floor-sanity POST-WIPE (agent-7625532f, 2026-09-09)

Status: **PASS (sanity grade)** — the rebuilt post-wipe toolchain reproduces the
committed kernels' historical old-vs-optimized ratios. This is an environment
revalidation datapoint, NOT a full re-baseline and NOT a cadence replication.

## Context

Sandbox wipe #2 (~14:29Z) erased jdk21/server/CRUSSTY. Recovery state at run time:
c-crussty re-cloned byte-intact (4b90315), CRUSSTY re-cloned at pinned 4f5d5ea,
system **OpenJDK 21.0.12.1 (Debian build)** available — same upstream version as the
wiped Temurin 21.0.12.1, but no `javac` binary (JRE-ish packaging). The canonical
`bench/p500/run_p500.sh` was **not modified**; two environment-level accommodations
were made outside the rig:

1. `javac` shim at `~/bin/javac` → `java …/JavacShim.java "$@"` — launches the SAME
   jdk.compiler 21.0.12.1 through the sanctioned `ToolProvider` API (source
   `/home/z/my-project/scripts/JavacShim.java`; shim caught its own first-draft bug in
   self-compile, fixed before use);
2. `JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64` exported to satisfy the rig's
   `set -u` (the `$JAVA_HOME/bin/javac` branch stays untaken — PATH shim serves).

Bench JVM = system `/usr/bin/java` (21.0.12.1). Committed `.so` binaries are
repo-fresh (unmodified since bank). Kernel jar NOT needed (stubs are self-contained).

## Run

Groups (4/49, representative spread): `0 AquiferIndexStride`, `12 EntityBoundingBox`,
`16 ImprovedNoiseInline` (5 kernels), `19 LevelChunkHeightmap`. One JVM per group,
time-bounded batches ~120 ms, median of 5, `-Xbatch`. Raw:
`P500_FLOORSANITY_POSTWIPE_RAW_2026-09-09.tsv` (11 RESULT rows); report:
`P500_FLOORSANITY_POSTWIPE_REPORT_2026-09-09.md`. Canonical `P500_REPORT.md` /
`p500_raw.tsv` restored to HEAD after the run (subset run must not overwrite the
full-run report).

## Result — baseline diff vs `results/baseline.tsv`

| class | pair (old / alt) | baseline | current | drift |
|---|---|---:|---:|---|
| AquiferIndexStride | oldBatch / newBatch | 0.933 | 0.936 | +0.3% |
| EntityBoundingBox | oldMakeThenSet / directDimensionsSet | 1.029 | 1.031 | +0.1% |
| ImprovedNoiseInline | oldPMethod / arithmetic | 0.864 | 0.861 | −0.3% |
| ImprovedNoiseInline | oldPMethod / flatGradient | 0.878 | 0.876 | −0.2% |
| ImprovedNoiseInline | oldPMethod / inlineByteAccess | 1.001 | 0.999 | −0.3% |
| ImprovedNoiseInline | oldPMethod / switchGradient | 0.819 | 0.822 | +0.3% |
| LevelChunkHeightmap | oldFourUpdate / newCombinedUpdate | 5.700 | 5.576 | −2.2% |

**7/7 pairs drift-ok (flag threshold |Δratio| > 20%)**, max |drift| 2.2%; every
measured kernel returned `OK`; stability 0.0% on all rows (fully deterministic
batches). The known 5.7x `LevelChunkHeightmap` combined-update win reproduces at
5.58x; the `ImprovedNoiseInline` win band (0.82–0.88 vs old) reproduces exactly.

## Honest caveats

- JDK distro build differs from the pre-wipe baseline runs (Debian patchset vs
  Temurin; same upstream 21.0.12.1): ≤2.2% drift at kernel scale suggests the distro
  effect is negligible here, but this is one datapoint, not a proof.
- 4/49 groups: 63 baseline pairs unmeasured this run — the full runs
  (`p500_raw_fullrun_*.tsv`) remain the canonical baseline source.
- n=1 run per group. Stability 0.0% on every row is unusually deterministic vs
  pre-wipe full runs (observed, not investigated — no rig changes made;
  observation only, anti-gate-shopping).
- Server-dependent channels (C3 rig, in-server phase-2a execution) remain blocked
  until provisioning restores `/home/z/server` + jdk21.

## Verdict

FLOOR-SANITY **PASS**: JNI transition floor and kernel ratios are intact on the
rebuilt environment; committed native assets trusted for further kernel-level work.
INJECTS-ONLY intact: 0 server boots, 0 product changes, bench-lane JVM flags only.
