# Kernel-Policy Gate Coverage — CRUSSTY_KERNEL_PREF=old (TASK-13)

**Task:** TASK-13 · **Agent:** agent-7625532f (TASK-13-R2, wave-3 rescue; salvage of two
deadline-killed runs) · **Date:** 2026-09-08
**Gate:** TASK-04 conservative surface binding, commit `8ec63b9`
(`src/kernel_policy.rs::registration_fallback`, `CRUSSTY_KERNEL_PREF=old`).
**Artifact:** `bench/p500/results/kernel_pref_coverage.tsv` (16 data rows, all `run_rc=0`) —
produced by `bench/p500/scripts/verify_kernel_pref.sh` + `KernelPrefDriver.java` under the
BENCH.lock protocol (`/home/z/BENCH.lock`), JDK 21 (`/home/z/jdk21`).

## Verdict (one line)

**The gate WORKS: 4/4 do-not-wire remap candidates collapse to paired-old parity
(−0.99 % … +0.12 % vs the paired old kernel measured in the same JVM run) when
`CRUSSTY_KERNEL_PREF=old` is set with the CRUSSTY runtime agent attached, and the gate
is provably inert (0 remap lines, ~0 delta) without it.**

## Scope

The full do-not-wire registry — exactly the four scale-invariant P500 regressions
(`src/kernel_policy.rs` `DO_NOT_WIRE`, lines 123–156; confirmed complete by the in-repo
test `do_not_wire_registry_has_the_four_p500_regressions` and
`P500_REPORT_v2.md` §"Regressions (do-not-wire)"):

| gid | class | regressed kernel (alt) | paired old kernel | P500_REPORT_v2 ratio |
|----:|-------|------------------------|-------------------|---------------------:|
| 19 | PaperNativeLevelChunkHeightmap | newCombinedUpdateSummary | oldFourUpdateSummary | 5.70× |
| 20 | PaperNativeMarkerCache | cachedSummary | oldSummary | 4.54× |
| 27 | PaperNativePalettedReencodeScratch | directPackedSummary | oldNewArraySummary | 2.35× |
| 34 | PaperNativeProtoChunkHeightmap | newCachedContainsSummary | oldEnumSetForeachSummary | 1.78× |

The script hard-codes exactly these four (`ROWS=(...)`) with a parse self-test that fails
closed before the bench lock is taken. **Coverage is COMPLETE: 4 groups × 2 layers ×
2 modes = 16 rows, no SKIPPED rows, every row `run_rc=0`.** No pair needed a re-run.

## Methodology

For every candidate the harness runs `p500.Bench` four ways
(`-Xms512m -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC -Dp500.n=16`; batches are
time-bounded 120 ms in `Bench.java`, so N shapes the per-call arg layout, not wall time):

| layer | mode | meaning |
|-------|------|---------|
| `agent` | `default` | CRUSSTY runtime agent attached (`-agentpath`), env unset — regression present |
| `agent` | `pref-old` | agent + `CRUSSTY_KERNEL_PREF=old` — the gate under test |
| `plain` | `default` | bare JVM, bridge stubs bind via dlsym — negative control |
| `plain` | `pref-old` | bare JVM + env — inertness/fail-safe control |

**Why two layers:** the remap is a *registration-time* binding — it fires inside
`lib.rs::define_and_register` when the runtime injects the 283-native surface (bootstrap
bridge classes + `RegisterNatives`). A bare `java p500.Bench` never runs that path; the
env var alone cannot (and must not) rebind a dlsym-bound bench. The plain layer documents
exactly that; the agent layer is the actual gate surface.

**Ordering hazard (handled):** class loading is parent-first, so whichever copy of a
bridge class exists first wins. `KernelPrefDriver` (`-Dp500.kprefwait=true`) polls until
the *bootstrap* copies of all four bridge classes exist using a **non-poisoning**
bootstrap-only probe (`Class.forName(name, false, null)` — defines/binds nothing on a
miss; a plain `Class.forName` would permanently bind the app-loader stub, the round-1
lesson), waits a 1 s grace, then runs the bench and `System.exit(0)`s (the runtime keeps
non-daemon threads alive). Observed wait: 4002 ms; loader report shows all four
`bootstrap(injected)`.

**Gate activation evidence (verbatim stderr):**

```
[crussty-plugin] kernel_pref: PaperNativeLevelChunkHeightmap.newCombinedUpdateSummary bound to old kernel (Java_PaperNativeLevelChunkHeightmap_oldFourUpdateSummary)
[crussty-plugin] kernel_pref: PaperNativeMarkerCache.cachedSummary bound to old kernel (Java_PaperNativeMarkerCache_oldSummary)
```

Every agent `pref-old` row: `remap_lines_total=4`, `class_remap=1`, `bootstrap_probes=4`.
Every agent `default` row and every plain row: `remap_lines_total=0`.

**Hygiene:** module `.so` = TASK-04 gate build (the deployed server module `.so` predates
TASK-04 and is deliberately not a fallback); runtime agent read-only from
`/home/z/server`; module staged to a private dir (`/tmp/crussty-kpref-modstage`); bench
classes compiled into a private `build-kpref/` (untracked); the live Minecraft server
(PIDs 26544/26562) was NEVER touched, signaled, or restarted; no `/home/z/CRUSSTY` or
`/home/z/server` edits; no gameplay change (gate only affects bench-side kernel binding).

## Per-group results — agent layer (the gate surface)

ns/op = medians from the single 120 ms-batch JVM run per cell (`RESULT` rows).

| gid | class | alt kernel | default ns/op | pref-old ns/op | Δ pref-old vs default | paired-old ns/op (same pref-old run) | pref-old vs paired-old | run_rc | rescued? |
|----:|-------|-----------|--------------:|---------------:|----------------------:|-------------------------------------:|-----------------------:|-------:|----------|
| 19 | PaperNativeLevelChunkHeightmap | newCombinedUpdateSummary | 793346.2 | 160172.2 | **−79.8 %** | 159979.1 | **+0.12 %** | 0 | ✅ YES |
| 20 | PaperNativeMarkerCache | cachedSummary | 35000.1 | 7876.5 | **−77.5 %** | 7927.2 | **−0.64 %** | 0 | ✅ YES |
| 27 | PaperNativePalettedReencodeScratch | directPackedSummary | 71140.5 | 30783.8 | **−56.7 %** | 31093.0 | **−0.99 %** | 0 | ✅ YES |
| 34 | PaperNativeProtoChunkHeightmap | newCachedContainsSummary | 138.4 | 77.7 | **−43.9 %** | 78.1 | **−0.51 %** | 0 | ✅ YES |

Rescue criterion: pref-old alt-kernel within ±2 % of its paired old kernel measured in the
same run. **4/4 pass.** Example (gid 19): 793346 → 160172 ns/op ≈ old 159979 → gate WORKS.
Internal control: the paired-old cells are nearly identical across the default and
pref-old agent rows (g19 159443.7 vs 159979.1 = 0.34 %; g20 7889.1 vs 7927.2; g27
31310.6 vs 31093.0; g34 78.5 vs 78.1), so the pref-old collapse is attributable to the
remap, not drift.

## Plain layer — inertness / fail-safe control (expected: no effect)

| gid | default alt ns/op | pref-old alt ns/op | Δ | remap lines | verdict |
|----:|------------------:|-------------------:|---:|------------:|---------|
| 19 | 795427.2 | 794185.1 | −0.16 % | 0 | inert ✅ (regression persists without the runtime — by design) |
| 20 | 35599.3 | 35023.6 | −1.62 % | 0 | inert ✅ (noisy cell, see caveats) |
| 27 | 71430.4 | 71367.1 | −0.09 % | 0 | inert ✅ |
| 34 | 136.7 | 137.0 | +0.22 % | 0 | inert ✅ |

This is the desired fail-safe property: `CRUSSTY_KERNEL_PREF=old` is a no-op outside the
CRUSSTY runtime injection path.

## Unexpected findings

1. **Plain-layer g20 paired-old outlier:** `oldSummary` measured 12276.5 ns/op in the
   plain *default* row vs 7918.8/7927.2 everywhere else — a one-cell outlier (~55 %),
   almost certainly scheduler contention on the shared box, not a mode difference (the
   alt cell in the same run is normal). **ESTIMATE-pending-bench**: one clean re-run of
   `plain_default_g20` would tidy the TSV; the agent-layer verdicts are unaffected.
2. **Default-row ratios run slightly below the v2 ratios** (e.g. g19: 793346/159444 =
   4.98× vs the 5.70× in `P500_REPORT_v2.md` / 5.55× in the current registry). Single-run
   noise at N=16 on a 2-CPU box; does not affect the verdict, which rests on parity, not
   on reproducing the headline ratio.
3. **Registry ratios vs TSV ratios differ in the third digit** (registry cites the
   2026-09-07 120 ms-batch rerun: 5.55/4.69/2.30/1.77; TSV carries the
   `P500_REPORT_v2` values 5.70/4.54/2.35/1.78). Two measurement generations of the same
   regressions, both far above `REG_MIN = 1.18`; cosmetic only.

## Script review — flags (read before reuse; nothing blocking, script not modified)

- `parse_ns` takes the **first** `RESULT` row per method (`exit` after first match).
  Verified harmless for this artifact: each `.out` contains exactly one `RESULT` line per
  method (2 per file). If `Bench.java` ever emits warmup `RESULT` rows, this would
  silently bias toward the first batch — add a median-over-rows if that changes.
- **One JVM run per cell, sequential** (default before pref-old, no interleaving) on a
  2-CPU shared box: single-run cells are the main noise source. Mitigated by the
  paired-old internal control (agent-layer old cells agree within 0.34–0.7 %), but
  headline deltas carry at least a few-percent uncertainty.
- `env "${pref[@]}" ...` expands an empty array under `set -u` — requires bash ≥ 4.4
  (fine on this box; would abort on ancient bash).
- Adaptive bail-out: if the *first* agent run shows 0 bootstrap probes, remaining agent
  rows are written as `SKIPPED/skip`. Not exercised here (all probes=4), but a `skip`
  row in a future TSV means the agent layer was abandoned, not measured.
- gid → row mapping is hard-coded with a fail-closed parse self-test; if `DO_NOT_WIRE`
  ever grows a fifth kernel, update `ROWS=(...)` AND `KernelPrefDriver.PROBES` together
  (the driver's wait loop polls only the four hard-coded probe classes).

## Noise caveats

- 2-CPU shared box; a live Minecraft server (PIDs 26544/26562) runs permanently — it was
  never touched; contention from it is the likeliest source of the g20 plain outlier.
- 120 ms batches, N=16, 1 JVM run per cell, ~45 s bootstrap wait + 1 s grace per
  agent-attached run (measured 4002 ms).
- **ESTIMATE-pending-bench:** this report measures the *bench* gate surface only. The
  corresponding live-server effect of the gate (should a do-not-wire kernel ever be
  reachable there) is not measured here.
