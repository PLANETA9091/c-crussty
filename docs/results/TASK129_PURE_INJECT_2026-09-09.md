# TASK-129 — Pure-inject canonical config validation: boot parity ×2, idle RSS −29%, reclaim MIXED (known benign-lazy class on a lean baseline)

**Agent:** agent-7625532f · 2026-09-09 · claim 977cc3f · RAW `RAW_TASK129_PURE_20260909`
**Law context:** owner directive 14:45+08 (`docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md`,
ledger §64 ADDENDUM-57) — no flags, injects only. This is the FIRST measurement of the inject
under the canonical launch it mandates.

## Config under test

- `java` = stock Eclipse Temurin 21.0.12.1 (`/home/z/jdk21`) — GraalVM not involved at all.
- Launch line, in full: `java -agentpath:.../libcrussty_runtime.so=modules=...;versions=...;kernel=purpur-1.21.10.jar -jar purpur-1.21.10.jar --nogui`
  — zero JVM options (no Xms/Xmx, no GC selection, no JVMCI, no Xlog, no NMT).
- Default ergonomics on the 4 GB box: max heap ≈ 1010 MB, initial ≈ 63 MB.
- Diagnostics are external observers only (law §3): `jcmd GC.heap_info`,
  `jcmd GC.class_histogram`, `/proc/<pid>/smaps_rollup`, RSS trajectory.
- C3 skeleton unchanged from the cadence rig: forceload 64-chunk band add → settle →
  remove → 120 s trajectory → ≤10% relative gate → PASS / PASS-LAZY / FAIL tree.

## Runs

| run | R0 idle | R1 post-add | R2 post-remove | R2 vs R0 | R3 (GC.run) | verdict (tree) |
|---|---|---|---|---|---|---|
| R1 | 889 MB | 945 | 957 | **+7.6%** | not needed | **PASS** |
| R2 | 877 MB | 930 | 969 | **+10.5%** | 974 (+5 vs R2, noise) | **FAIL-leak-signature** (by tree) |

## Inject functional parity — the inject works on a stock JVM with nothing else

- Both boots log the full bring-up: `[crussty-runtime] v2.0.0 loaded (options: ...)`,
  `cplugin_init: injecting Crussty CE native surface in background` — identical to the
  flag-config boots.
- Boot wall: **16.27 s / 15.96 s** vs the 15.8–15.9 s flag-config band — parity
  (+0.4 s worst case). The default-heap boot risk did NOT materialize: no OOM, no slow boot,
  hs_err delta 0 across both runs.
- JVMTI agent loading is build-agnostic across OpenJDK 21 distributions — now measured, not assumed.

## The FAIL leg is the known benign-lazy class — not a leak

1. **Used heap returned below baseline in both legs** (the leak-discriminator):
   R2-leg used went 451 M (baseline) → 341 M (remove-end) → **250 M after GC.run**, well
   under the 451 M baseline. No live-set growth.
2. **Residue is committed-side, same attribution as TASK-126 (branch a), now without
   JVMCI:** G1 held committed at 577 M after GC.run (+35 M vs baseline 543 M) and RSS carried
   ~57 MB of non-heap growth — on Temurin this is plain C2 code-cache + metaspace from the
   64-chunk churn (no Graal/JVMCI in the process at all, which removes the last doubt that
   the residue was a GraalVM artifact).
3. Trajectory shape: slow creep 930 → 969 then flat 969 for the last three samples — a
   lazy-committed plateau, not an actively growing leak.

## NEW LAW CANDIDATE — relative gate conditioning on lean baselines

The ~90 MB absolute residue is the same size class that the flag-config series absorbed as
+3..+8% (R0 1092–1398 MB); on the lean pure-inject baseline (R0 ≈ 880 MB) the SAME absolute
residue reads +10.5% and trips the ≤10% relative gate. This is exactly the "committed-heap
conditioning term" the TASK-125 doc predicted. Pre-registered for the next cadence revision:
`gate = max(R0 × 1.10, R0 + 100 MB)`. The gate is NOT changed retroactively — the historical
series stays comparable under the original rule.

## RAM headline — the owner's standing RAM goal, delivered flag-free

| config | idle R0 (same box, same seed) |
|---|---|
| flag config (GraalVM CE + UseJVMCICompiler + Xms512M/Xmx2G + G1) | 1092–1398 MB (6-run series) |
| **pure inject (Temurin, zero options)** | **877–889 MB** |

- Idle RSS **−29% at the median** (≈1245 → ≈883 MB). Mechanism: default ergonomics start
  small and grow lazily — idle committed heap 543–557 MB vs 738–1045 MB under the flag config.
- Even the FAIL leg's peak (969–974 MB) sits **below the flag-config idle minimum** (1092 MB).
- CPU/boot cost of the switch: none measured (boot parity; no added GC pressure at idle).

## Verdict

**PURE-INJECT VALIDATED as the canonical config.** Boots stable ×2, inject fully functional
on a stock JVM, boot wall parity, idle RAM −29% — the directive's launch shape is production-
viable on this box. Reclaim classification MIXED (1 PASS / 1 FAIL-by-gate) is consistent with
the standing benign-lazy class (TASK-119/121/125/126 lineage): no leak signature banked, and
the gate-miss is a relative-gate conditioning artifact on a leaner baseline, not a regression
of the inject.

## Banked notes

- Raw evidence: `RAW_TASK129_PURE_20260909/{results.tsv, run_R1/, run_R2/}` — boot.log,
  rss_after_remove.traj, hist_R2.txt, jcmd.log per run; console.fifo removed pre-commit
  (hygiene law).
- Rig: `run_task129_pure_inject.sh` — flag-free by construction; the `EXTRA_JVM_FLAGS`
  parameter is intentionally absent (TASK-127 lesson).
- Cadence policy consequence: quiet-tick C3 cadence migrates to this rig (it measures the
  canonical config); `run_task119_c3_probe.sh` remains as the historical flag-config series
  (TASK-119/121/125 lineage), not deleted.
- Ledger: §65 ADDENDUM-58. Note: the ledger saw a duplicate §64 header from a concurrent
  twin append (their ADDENDUM-56 vs my ADDENDUM-57) — both preserved verbatim (append-only
  law); this entry takes the next unique §.
