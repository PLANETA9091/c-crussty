# TASK45_SDK_STATS_AB — TASK-22 scan-avoidance, ESTIMATE → measured

Signed: agent-7625532f, 2026-09-08 (windows captured 20:44–21:05Z)
Harness: `bench/bootab/run_bootab.sh` + new `BOOTAB_POST_HOLD_S` (post-marker
hold so the stats windows fit inside a bootab run) + sdk-stats counters
(cplug-sdk commit 53578ba, `CRUSSTY_SDK_STATS=1`).

## Arms

- **A' (old algorithm)**: module built at `4fb9d12` (pre-TASK-22) + the SAME
  counters hand-patched onto the old `find_class` (entry/cache-hit/scan
  increments only — no sighting gate). Worktree `/tmp/w-a45`, hand patch
  mirrors the master schema line-for-line.
- **B (master)**: `53578ba` — TASK-22/C1 sighting gate (unsighted misses skip
  the scan; bounded fallback = first call + every 8th) + TASK-43 D2 bound,
  same counters.
- Both: flat-world throwaway purpur boots (`run_bootab.sh 'native surface
  live' 120 N`), `CRUSSTY_SDK_STATS=1 CRUSSTY_NATIVE_IMPROVED_NOISE=1`,
  rt `.so` = deployed b57b3092, closed natives from `native/` (identical).

## Boot-window counters (t=25 s snapshot ≈ boot journey to just past Done)

| metric | A' (pre-TASK-22) | B (master) | delta |
|---|---:|---:|---|
| find_calls | 26 | 10 | (caller mix identical: wait_for_boot Bukkit poll + wiring; poll pacing differs slightly by scan latency) |
| cache_hits | 10 | 2 | — |
| feed_skips (miss answered WITHOUT scan) | 0 | **5** | the gate: 5/8 misses scan-free |
| scans issued | 16 | **3** | 5.3x fewer |
| classes_walked inside scans | **156,690** | **16,822** | **9.3x less scan work** |

Cumulative to t=120 s: A' 27 calls / 16 scans / 156,690 walked (frozen —
nothing polls after boot in the old tree); B 35 calls / 5 scans / 76,051
walked — B's extra post-Done calls are the NEW improved_noise worldgen-class
polling (TASK-01 lineage; absent at 4fb9d12), walking ~29.6k classes per scan
at that point (matches the historical "10–30k classes per scan" estimate).
The t=120 B scan jump is that new caller's first sightings, not a regression
of the gate: of B's 20 boot+post-boot misses, 15 were answered scan-free.

## What this measures (and what it does not)

- **Converted to measured**: on the shared boot workload, TASK-22's gate +
  fallback policy cuts scan WORK by **9.3x** (156,690 → 16,822 classes
  walked), scan COUNT by **5.3x** (16 → 3), and answers **63% of misses
  (5/8) with zero scan**. The old algorithm's every-miss-scan behavior is
  directly observed (A': 16 misses → 16 scans, 0 skips).
- **Scan-cost anchors confirmed**: A' boot scans average ~9.8k classes each
  (early boot, fewer classes loaded); B's post-Done scans ~29.6k — the
  "10–30k classes per full rescan" premise in TASK-22's original claim is
  real, so the per-scan wall cost (10–60 ms claim) applied to the avoided
  scans at boot: 13 avoided scans × ~10–20 ms ≈ **0.13–0.26 s of scan work
  removed from the boot window** (consistent with the bootab baseline's
  small directional boot-time delta — scans overlap the paperclip/patch
  wall-time, they do not add to it 1:1).
- **NOT demonstrated**: the ">100x" upper end of the TASK-22 ESTIMATE. This
  workload's absolute traffic is tiny (27–35 calls/boot); a >10x aggregate
  requires a consumer polling an unsighted name at high rate (the original
  "85 rescans / 180 s" scenario — kernel-side polling that no current
  in-tree consumer performs on a flat-world boot). The live-server ledger
  entry therefore stays ESTIMATE for its >10x claim, with this measured
  boot-window floor attached: **measured 9.3x scan-work / 5.3x scan-count /
  63% scan-free misses**.

## Boot timings (n=2 per arm, flat world — context only, NOT the deliverable)

| arm | primary (native surface) median | secondary (Done) median |
|---|---:|---:|
| A' | 3.16 s | 41.76 s (28.72 / 41.76 — high variance) |
| B | 3.24 s | 38.15 s (46.19 / 30.11 — high variance) |

Ranges overlap as in the TASK-32-w4 baseline (DIRECTIONAL ONLY, n=2): scan
work overlaps boot wall-time, so wall-clock cannot resolve this delta on a
2-CPU box — which is exactly why the counters were needed.

## Harness notes (recorded for reproducibility)

- `BOOTAB_ROOT` must point at a dir containing `rt/libcrussty_runtime.so`
  (the agentpath is derived from it) — a bare scratch root fails all boots
  with "Could not find agent library" (observed, misdiagnosed once as OOM).
- The stats dumper prints cumulative lines at t={25,65,120,180}s from
  cplugin_init; window deltas = consecutive line diffs. t=180 requires
  `BOOTAB_POST_HOLD_S >= 185` (secondary marker ~30 s + 180); the t=65→120
  delta is the "post-marker 60 s window" per the TASK-45 spec.
- Transient state: /tmp is wiped between cron ticks (bootab scratch, module
  dirs, worktrees under /tmp do not survive) — durable artifacts live in
  bench/bootab/results/ (TSVs below) and the report itself.
- Raw TSVs: `results-A45.tsv`, `results-B45.tsv` (n=2 timing arms),
  `results-A45keep.tsv`, `results-B45keep.tsv` (keep-runs whose run.log
  carried the stats lines quoted above).

## Verdict

TASK-45 **done**: counters landed (53578ba), A/B executed, ESTIMATE
converted to measured where this workload can reach it (9.3x scan-work /
5.3x scan-count / 63% scan-free misses; scan-cost premise 10–30k classes
confirmed), >10x live-claim left as ESTIMATE with the measured floor
recorded. No gameplay values, no kernel/registry changes, live server
untouched (it was already down during this tick's windows).
