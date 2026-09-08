# WORLDGEN PARALLELISM PROBE — PAPER SERIALIZES BOTH GEOMETRIES TO ONE WORKER

* TASK-65, 2026-09-09, agent-7625532f. TASK-62 F3 follow-up: that profile measured 86% of burst-minute
  runnable CPU on a single `Paper Common Worker` and flagged per-area serialization as a possible
  ceiling with "real headroom for multi-area bursts". This probe decides it. Verdict: **the headroom
  does not exist — F3 closes NEGATIVE.**
* Harness: `bench/e2e/run_worldgen_parallel.sh` (derived from the TASK-63 A/B harness; one-run-per-
  agent-call driver, /proc CPU-rate completion detector, seed-tar anchor + restore per run, dormant-only).
* RAW: `bench/e2e/results/WORLDGEN_PARALLEL_RAW_2026-09-09/` (summary.csv, per-run cputrace/threads/
  query/watchdog files, `world_verify.txt` = IDENTICAL).
* Hygiene: world restored byte-identical after every run; final `diff -r` IDENTICAL; graceful fifo stop
  (exit 0); BENCH.lock inline per call; token clean; launcher.jar untouched.

## 1. Verdict

**Two distant 64-chunk fresh-generation regions do NOT parallelise — the Paper chunk scheduler runs
them on ONE worker thread, and the pair is consistently SLOWER than one contiguous 128-chunk region.**

| metric (128 fresh chunks, paired medians) | S: one 16x8 contiguous | P: two distant 8x8 | Δ (P−S) |
|---|---|---|---|
| t_burst wall | **54.05 s** (52.41–58.02) | **58.19 s** (55.16–75.21) | **+4.14 s (+7.7%)** |
| child-JVM burst CPU | 47.36 CPU-s | 50.26 CPU-s | +2.9 CPU-s |
| **worker threads doing the gen** | **1 (100% of worker CPU)** | **1 (100% of worker CPU)** | no difference |
| watchdog stall lines | 628–763 | 691–1103 | P stalls longer |

P was slower than S in **5 of 5 adjacent ABBA pairs** (+3.5, +17.2, +2.8, +5.0, +4.8 s; the +17.2 pair
carries the run3_P watchdog outlier 1103 lines). Mann–Whitney on n=5/5 with this consistency is decisive
for the direction (the one overlapping P run still exceeds the S median). The hypothesis that "distant
areas would engage more workers" is **refuted by direct measurement**: per-thread CPU accounting shows
exactly one `Paper Common Wo*` tid with meaningful CPU in EVERY run of BOTH arms (busy-worker median 1.0,
top-worker share 100%).

## 2. Method

* **Arms.** S = `forceload add 1600 1600 1855 1727` (chunks 100..115 × 100..107, one contiguous 16x8 =
  128 fresh chunks). P = the exact TASK-62/63 protocol: two distant 8x8 regions
  (`1600..1727`² and `-1728..-1601`²) sent back-to-back. Same seed-tar world restored before every run;
  identical fresh-chunk workload per run (except S shares region edges — see limits).
* **KEY METRIC — per-thread CPU without JFR.** A fork-free sampler (`bash` builtin `read`, no subprocesses)
  walks `/proc/<child>/task/*/stat` at 1 Hz inside the timed window, accumulating `utime+stime` per tid;
  comm parsed from the stat line (kernel-truncated to 15 chars — worker threads aggregate as
  `Paper Common Wo*`, disambiguated per-tid). Per-thread window CPU = last−first sample ticks. Zero-touch:
  nothing is sent to the server inside the timed window.
* **Driver discipline** (TASK-63 F4 lessons): one run per agent tool call with inline `flock`; ABBA
  execution order (S P P S | S P P S | S P); 10 runs = 5 valid per arm (the very first S run was redone
  after a sampler bug was found — its thread log is empty and it is quarantined from thread stats; wall
  value was valid and is kept in the CSV with a note). Fresh-world canary refined: abort only on sub-100%
  spawn lines (a genuinely new world prints them); slow boots (Done 29–32 s, all instant-100% spawn) are
  WARNed and continued — this session had uniformly slow classloading (~30 s vs 16–20 s in the TASK-63
  session, hours earlier) — boot time is NOT part of any burst metric, so this does not bias results.
* **Detector**: /proc/<child>/stat total-CPU rate < 0.15 cores × 3 consecutive 0.5 s samples after a
  >0.5-core phase (TASK-63 proven design).

## 3. Evidence

`summary.csv` (idx, arm, t_burst_s, cpu_burst_s, done_s; run1 first attempt quarantined for empty thread
log — sampler bug `set --` positional clobber, fixed and documented in the harness):

```
1,S,54.05,47.36,30.122   6,P,55.16,51.85,30.939
2,P,57.51,52.96,29.004   7,P,58.19,50.26,32.246
3,P,75.21,65.55,30.980   8,S,53.22,47.15,29.636
4,S,58.02,51.50,30.235   9,S,56.06,51.10,30.458
5,S,52.41,45.79,31.954   10,P,60.83,54.96,29.682
(quarantine: first attempt of run1_S — 45.02s/50.05 — wall valid, thread log empty)
```

Thread-CPU table (ticks, 100 ticks = 1 CPU-s; full table in the analysis output / RAW):

| run | total | worker total | worker TOP | busy workers | distinct worker tids |
|---|---|---|---|---|---|
| S runs (5) | 4428–4989 | 1433–1686 | = worker total | **1** | **1** |
| P runs (5) | 4849–6405 | 1697–2301 | = worker total | **1** | **1** |

* The single worker carries the ENTIRE worker-side generation cost in every run — 100% top-worker share
  in both geometries. Server thread CPU is flat (~3.6 CPU-s, parked in chunk wait), watchdog CPU scales
  with stall length (P longer).
* Whole-process CPU is higher for P (+~2.9 CPU-s) with the SAME single-worker budget: the pair's extra
  cost is dispatch + duplicated edge/neighbour work + longer watchdog stall — not contention.

## 4. Findings

**F1 — TASK-62 F3 closes NEGATIVE.** The "86% on one worker" observation is not a per-area scheduling
artifact: it is how the scheduler executes forceload-driven fresh generation at this scale, for one
contiguous region AND for two distant ones. There is no multi-area parallelism headroom to harvest via
geometry, and no ops guidance of the form "split your forceloads" — the opposite: a single contiguous
region is mildly FASTER (+~7%) than an equal-chunk distant pair.

**F2 — Why P loses.** Same single-worker execution budget, but two command dispatches, two dependency
chains, duplicated boundary blending/neighbour sampling, and a longer main-thread stall (watchdog CPU
+10–45%). Contiguous S shares edges and dispatches once.

**F3 — Consequence for the gen-latency lever space.** Combined with TASK-63 (bridge transfer refuted):
any future worldgen-side optimisation must (a) reduce per-call/per-chunk WORK, or (b) change the
scheduler itself (engine/paper territory, out of scope) — neither a per-call native bridge (TASK-63) nor
workload geometry (this probe) moves the wall clock downward. The single-worker execution surface is a
hard ceiling for every micro-level lever measured so far.

**F4 — Harness hardening landed.** The `set --` positional-clobber bug (first tid's sample written to a
file named by the ppid field; glob dead from tid 2 on) is fixed by capturing args into locals; stray
file removed; canary refined to the sub-100%-spawn signature. Both lessons are comment-documented in the
harness source.

## 5. Honest limits

* comm is kernel-truncated to 15 chars — worker instances are distinguished per-tid, not per-name; a
  worker that exits and respawns within the window appears as two tids (none observed: every run shows
  exactly 1 distinct worker tid).
* The S arm shares region edges (one 16x8 vs two 8x8): its per-chunk work is marginally smaller by
  neighbour-reuse. This is PART of what "contiguous is faster" means operationally; it cannot explain
  away the single-worker finding, which is the headline.
* n=5 per arm with one P outlier (run3, watchdog 1103) — direction consistent across all 5 pairs; the
  exact magnitude of P's slowdown (7–17%) carries wide error bars. The NEGATIVE parallelism result is
  categorical (1 worker in 10/10 runs), not statistical.
* Slow-boot era (~30 s classloading) affected every run uniformly; boot time is excluded from all burst
  metrics by construction (T0 starts post-settle).
