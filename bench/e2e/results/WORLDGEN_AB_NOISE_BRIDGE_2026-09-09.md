# WORLDGEN A/B — NOISE-NATIVE BRIDGE UNDER REAL LOAD: P500 WIN DOES NOT TRANSFER

* TASK-63, 2026-09-09, agent-7625532f. First REAL-LOAD paired A/B of the noise-native bridge
  (`CRUSSTY_NATIVE_IMPROVED_NOISE=1`), interleaved ABBA, same deployed module `.so` in both arms
  (HEAD 966e0c4 build, ABI 262165), differential = the env var only.
* Harness: `bench/e2e/run_worldgen_ab.sh` (derived; `scripts/e2e_orchestrate.sh` boot/shutdown UNTOUCHED).
* RAW: `bench/e2e/results/WORLDGEN_AB_RAW_2026-09-09/` (per-run boot/marker/trace/query/shutdown files +
  `summary.csv`); aborted first series archived as `WORLDGEN_AB_RAW_ATTEMPT1_2026-09-09/` (honest record).
* JFR mechanism-proof: `/home/z/server/logs/jfr_task63/task63_b_midburst.jfr` (5.5 MB mid-burst dump),
  analysis `scripts/jfr_task63_mechanism.py` (my-project).
* Hygiene: world restored byte-identical after EVERY run from a session seed tar; final `diff -r` clean;
  server stopped as found (graceful fifo stop, exit 0 — TASK-59 path); `/home/z/BENCH.lock` held per run;
  token not exposed; launcher.jar not touched.

## 1. Verdict

**The P500-proven ~2× noise premium does NOT transfer to the fresh-worldgen channel through the existing
per-call JNI bridge — the point estimate is NEGATIVE (armed ≈ 10% SLOWER than dormant).**

| metric (128-chunk fresh burst) | A dormant (n=5) | B armed (n=5) | Δ (B−A) |
|---|---|---|---|
| t_burst wall median | **43.96 s** | **48.38 s** | **+4.42 s (+10.0%)** |
| cpu_burst median (child JVM) | 53.62 CPU-s | 58.86 CPU-s | +5.24 CPU-s (+9.8%) |
| boot `Done (` median | 20.11 s | 20.15 s | flat (re-confirms TASK-58) |

Every valid B-run except one (45.60 s) is above the A median; the single A-run at 51.26 s (watchdog-heaviest
A sample, 235 watchdog lines) overlaps the B range, so Mann–Whitney two-sided at n=5/5 gives p ≈ 0.15 —
**not significant at 0.05; what IS decided is the refutation of the hypothesised WIN** (TASK-62 F2 estimated
~2 CPU-s recoverable; measured point estimate is the opposite sign). Boot `Done (` is flat between arms —
arming is post-Done and the boot channel remains non-addressable (TASK-58), now under burst conditions too.

**Operational bottom line:** `CRUSSTY_NATIVE_IMPROVED_NOISE=1` must stay OFF for worldgen-heavy profiles —
it costs ~0.5 s wall per 128-chunk burst-equivalent at this box scale. The bridge's measured value surface
remains the P500 batched kernels (amortised crossings); a worldgen win would need a BATCHING layer at the
worldgen call sites (per-column/per-cell buffering) — a new design, deliberately NOT proposed here.

## 2. Method

* **Arms.** A = dormant (no `CRUSSTY_*` overrides — production default). B = `CRUSSTY_NATIVE_IMPROVED_NOISE=1`.
  Same deployed `modules/crussty/libcrussty.so` (TASK-61 build: 21 kernels, helper self-test abi 262165);
  the ONLY differential between arms is the env var. JFR was excluded from all timed runs (TASK-58 discipline);
  the mechanism proof is a separate non-timed run.
* **Burst.** Per run: boot via canonical orchestrate → `Done (` → arming gate → uniform 4 s settle →
  `forceload add 1600 1600 1727 1727` + `forceload add -1728 -1728 -1601 -1601` (the exact TASK-62 coords;
  128 fresh chunks outside the pre-generated spawn, full density-function pipeline) → completion detector →
  `forceload query` (relay-alive proof; "Marked 128" observed in every run) → `forceload remove all` →
  graceful fifo stop → world restore from seed tar.
* **Completion detector.** `/proc/<child>/stat` total-CPU (utime+stime, all threads) sampled at 2 Hz; burst end
  = first point where the rate falls below 0.15 cores for 3 consecutive samples after a >0.5-core phase was
  observed (idle baseline ≈ 0.08 cores, TASK-62 minute profile). Zero-touch (no console probing inside the
  timed window), identical for both arms. Full per-run rate traces in RAW (`*_cputrace.txt`).
* **Arming gate (the load-bearing detail).** Arming is server-boot-gated and POST-Done. Attempt-1 of this
  series checked markers at Done+0 s and RACED the arming chain (run2_B showed `hook armed` in the log only
  afterwards; its force-load loop was at attempt 6 of retries). Attempt-1's run7_B hit the opposite failure:
  arming still unfinished at the 25 s cap (force-load at attempt 4) — that run would have silently measured
  the DORMANT arm under a B label. Final gate: poll for `hook armed|self-test passed` up to 90 s; abort the
  run (shutdown + restore, `NO_ARM@<t>` row) rather than benchmark a mislabelled arm. Arming latency observed:
  11–13 s in armed runs.
* **Driver.** The full-series background mode was killed twice by a non-deterministic sandbox SIGKILL reaper
  (§5 F4), so the series was driven as **one run per agent tool call** (`run_worldgen_ab.sh one <idx> <arm>`,
  inline `flock` on `/home/z/BENCH.lock`, lifetime = call lifetime) — 10 runs ABBA (`A B B A | A B | B A | A B`)
  plus one replacement B (run11) after run7's NO_ARM quarantine. Resumable: `summary.csv` rows are the
  progress ledger.
* **World discipline.** Seed tar (world/, 220 KB) created ONCE per series from the verified as-found state;
  `world_restore()` hard-guards against missing/small tars; every run boots the byte-identical pre-generated
  world; final `diff -r` against a fresh extraction: IDENTICAL.

## 3. Evidence

`summary.csv` (idx, arm, t_burst_s, cpu_burst_s, done_s, arm_validation):

```
1,A,43.66,55.41,20.622,ok@0     | 6,B,47.48,53.04,20.368,ok@13
2,B,48.58,58.86,21.206,ok@13    | 7,B,46.75,54.36,19.670,NO_ARM@25   (quarantined)
3,B,49.82,64.89,20.469,ok@13    | 8,A,45.10,53.04,20.387,ok@0
4,A,43.26,51.39,20.110,ok@0     | 9,A,51.26,55.21,19.769,ok@0
5,A,43.96,53.62,20.038,ok@0     | 10,B,48.38,60.93,20.152,ok@13
                                | 11,B,45.60,58.13,19.860,ok@11 (replacement for 7)
```

* **Watchdog covariate** (the stall dump itself burns CPU; a confound if asymmetric): watchdog lines per run
  A = {192, 187, 225, 162, 235}, B = {192, 240, 174, 218, 192} — fires in EVERY run (128-chunk forceload
  stalls the main thread >10 s, TASK-62 observation), roughly uniform across arms, no systematic bias.
  Excluding the two extreme runs (A: 51.26 s/235; B: 49.82 s/240) keeps the verdict unchanged (Δmedian +3.5 s).
* **CPU-rate trace shape** (example run1_A): idle ≈ 0.08 cores → burst plateaus 1.4–1.6 cores for ~40 s →
  settle. Same shape in B (plateau slightly longer, not higher) — the armed arm spends LONGER, not HARDER.
* **Determinism of work:** identical seed tar per run, identical forceload coords, `Marked 128` in every run;
  boot `Done (` flat across arms and runs (19.7–21.2 s, no fresh-world symptom — the canary guard was armed
  at >22 s in every run).
* **Quarantined run7_B:** `NO_ARM@25` under the old 25 s cap; its 46.75 s was measured with the bridge NOT
  armed (markers show force-load attempt 4 still retrying at cap). Recorded, excluded from B stats, and it
  triggered the 90 s cap + hard-abort fix.

## 4. Mechanism proof (JFR, non-timed arm-B run)

Mid-burst explicit `JFR.dump` (window: burst start +45 s, 1640 ExecutionSample events):

* **The bridge ENGAGES under real load.** `ImprovedNoiseNativeOps` appears as leaf class in 44 samples
  (~0.44 CPU-s inside the native-stub boundary) — TASK-62's dormant profile had ZERO such frames by
  construction (no NativeOps exists in the dormant env). Class-level proof that burst-time noise traffic
  routes through the bridge, not just boot-time arming markers.
* **Java-noise CPU is displaced, not eliminated:** residual `ImprovedNoise` (33) + `PerlinNoise` (32) leaves ≈
  1.19 CPU-s in the window vs **4.12 CPU-s** of `ImprovedNoise`/`PerlinNoise` anywhere-in-stack in TASK-62's
  dormant burst minute (same coords, same 128 chunks). Pipeline structure intact: `fillArray` appears in 98
  window stacks (TASK-62: 120 in a 60 s minute), `lerp3` 21.
* **Economics of the loss.** The bridge replaces hot, small, JIT-inlined noise methods with per-call JNI
  crossings: every single-value `noise(...)` call pays the transition barrier, loses inlining/escape analysis
  across the boundary, and adds a `NativeOps` dispatch hop. Under dense terrain sampling that is millions of
  crossings per burst. The P500 2× premium was measured on BATCHED kernels — ONE crossing amortised over a
  whole array op — a fundamentally different call economy. The A/B numbers (+4.4 s wall, +5.2 CPU-s) are the
  per-call overhead minus the native compute gain, measured end-to-end.
* Honest gaps: `jdk.NativeMethodSample` recorded nothing (native-side CPU share not directly measured —
  the 0.44 CPU-s stub figure is a lower bound of crossing cost); the fillArray fine-structure under the patch
  (which internal calls reroute) is dist/engine bytecode territory and was not opened.

## 5. Findings

**F1 — Transfer refutation (headline).** The proven noise bridge's value does NOT carry to the production
worldgen channel via the current per-call JNI design: point estimate negative (−10% wall), hypothesis from
TASK-62 F2 (~2 CPU-s recoverable per burst) refuted with direction opposite. The bridge stays correct and
P500-proven for its batched surface; PROVEN_WINS §4.2's "production channel" framing must cite this A/B:
fresh worldgen is NOT an addressable channel FOR THIS BRIDGE DESIGN.

**F2 — Boot channel re-confirmed flat under burst conditions.** Boot `Done (` medians 20.11 vs 20.15 s —
the TASK-58 boot-invariance (arming post-Done by construction) holds with the burst workload present in the
same sessions.

**F3 — Arming latency is variable and boot-gated:** 11–13 s typical, >25 s observed (force-load retry loop);
any future armed experiment MUST gate the workload on the `hook armed` marker with a generous cap and
hard-abort on timeout (this harness now does). Ops corollary: on a live server the first ~10–15 s post-boot
runs Java noise in the armed arm by design — irrelevant for correctness, relevant for micro-timing.

**F4 — Sandbox reaper (infrastructure, affects all future long background work here).** Three background
process trees were SIGKILL'd non-deterministically (20–90 s after the launching tool call ended), with NO
trap firing (traps were installed on TERM/HUP/INT/EXIT — KILL only), no kernel OOM in dmesg, and inconsistent
victim sets (script only; script+launcher+child). Consequence: long benchmark series in this sandbox must be
driven one-run-per-tool-call (each call holds the box lock inline), or be resumable by an on-disk ledger.
The harness now supports both (`one`/`jfr`/full-series modes).

## 6. Incident and recovery (recorded for honesty — world data was at risk)

During the first (background) series attempt, a harness ordering bug ran `selfheal → world_restore` BEFORE
the session seed tar existed: the live `world/`, `world_nether/`, `world_the_end/` were `rm -rf`'d and the
extraction was a no-op; the next boot generated a FRESH RANDOM world (symptom: `Done (` = 27.3 s vs 16–20 s
for the pre-generated world — this now-abort-threshold canary exists because of it).

* **Overworld: fully recovered.** TASK-62's pre-burst tar (`world_bak_pre_task62.tar.gz`) is the as-found
  anchor (TASK-62 verified byte-identical restore from it; nothing modified world/ since). Restored and
  verified `diff -r` IDENTICAL — twice (post-incident, post-series).
* **Nether/End: not recoverable byte-wise; assessed as data-empty.** No backup existed. Evidence of emptiness:
  every session on this box was console-driven (no player joins; forceload/workload commands overworld-only;
  no nether portals, no teleport history in logs; Paper creates the dimension directories at boot with
  `level.dat` + empty `DIM-1/region` until first access). The current tree contains fresh Paper-created
  dimension dirs (regenerated at each boot) — functionally equivalent to as-found, byte-differences limited to
  boot-created metadata (`level.dat`, `uid.dat`). Residual uncertainty (some earlier session accessing the
  nether without a trace) is acknowledged and considered negligible; no playerdata ever existed on this box.
* **Harness hardening (all landed in this commit):** backup-FIRST ordering with an abort on suspiciously small
  tars; `world_restore` refuses to delete live dirs without a verified seed tar; seed tar = session anchor
  created once (never re-tarred from a possibly-dirty world); fresh-world boot canary (Done > 22 s aborts the
  run and restores); per-run restore + final `diff -r` verification.

## 7. Honest limits

* n=5 valid runs per arm; Mann–Whitney p ≈ 0.15 — the regression direction is consistent but the magnitude
  (−10%) carries wide error bars; one A-run outlier overlaps B. The DECIDED claim is the absence of a win
  (all plausible effects ≤ 0), not the exact regression size.
* Cold-arming caveat: B-runs measure the first burst after retransform+class-load; production would amortise
  one-time JIT costs over the server lifetime. The sustained +5.2 CPU-s (whole-burst) points at real
  per-crossing overhead rather than one-time compilation, and the JFR stub-time evidence supports this, but a
  long-lived-server experiment (arming at boot T0, bursts hours later) would close this gap.
* Detector resolution: 0.5 s samples × 3-streak rule (~1.5 s quantisation on ~45 s bursts, ~3%); identical
  across arms.
* The burst is forceload-shaped (main-thread stall + single dominant worker, TASK-62 F3); player-driven
  chunk-load distributions may differ. The verdict concerns the bridge's call economics, which scale with
  noise-call density, not with the load driver.
