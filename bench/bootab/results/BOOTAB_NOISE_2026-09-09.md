# BOOT A/B noise-native — hypothesis REFUTED by mechanism + measurement

* TASK-58, 2026-09-09, agent-7625532f. Follow-up of the TASK-57 JFR profile (F2 proposal):
  "boot A/B dormant vs CRUSSTY_NATIVE_IMPROVED_NOISE=1 — the 2.87 CPU-s boot noise burst
  is exactly the stack the closed-.so native noise bridge replaces."
* Verdict up front: **the hypothesis is REFUTED — twice.** (1) By mechanism: the noise
  arm chain is server-boot-gated and lands AFTER the `Done (` marker by design, so the
  pre-Done noise burst cannot route native no matter the env. (2) By measurement:
  n=5/arm paired live-dir boots — medians 16.738 s (A dormant) vs 16.542 s (B armed),
  delta −0.196 s (−1.2 %) with fully overlapping ranges — indistinguishable from the
  mechanism-derived expectation of exactly 0.

## 1. Mechanism (primary evidence — marker order in an armed boot log)

The armed activation sequence as logged (B-run, deployed module, live dir):

```
improved_noise: forcing kernel load of …/ImprovedNoise (attempt 1..7)   ← pre-boot probes (server object absent)
improved_noise: server booted, defining bridge into kernel loader       ← GATE: only after server boot
improved_noise: defined …/ImprovedNoiseNativeOps{,$Handle,$Reaper} in kernel loader
improved_noise: defined …/ImprovedNoiseBatchOps in kernel loader
improved_noise: computed patch for noise() (5691 -> 5403 bytes), orig major 65 patch major 65
improved_noise: hook serve 5403 bytes (major 65)
improved_noise: hook armed, retransform rc=0
improved_noise: self-test passed (native handle round-trip through real bridge)
```

The `Done (Xs)` marker (what boot latency measures) is crossed BEFORE `server booted,
defining bridge` — the patch therefore arms strictly post-Done. TASK-57 measured the
structure-ring noise burst spanning Done (≈0.6 s pre, ≈4 s post on the timeline) —
the pre-Done portion executes on the unpatched Java class BY CONSTRUCTION. No env
value can change boot completion time through this surface. The only pre-Done arm
delta is the worker's probe loop (7 × Class.forName ≈ sub-ms) plus one extra glob
pattern on the class-load hook (≈1.5–9 ms/boot, TASK-45 D5 band) — unmeasurable.

## 2. Measurement (confirms the mechanism)

Protocol: live-dir (`/home/z/server`) paired boots via `e2e_orchestrate.sh boot/shutdown`,
interleaved A,B,A,B,… n=5 per arm; metric = Paper self-reported `Done (X.XXXs)` from
`logs/latest.log` (fresh per boot); `flock /home/z/BENCH.lock` per run; world pre-generated
(constant worldgen work); JFR absent for timed runs (overhead isolation); B arm env =
`CRUSSTY_NATIVE_IMPROVED_NOISE=1`, A arm = unset; all other `CRUSSTY_*` unset (promote
dormant, batch off). Raw TSV: `/tmp/ab-boot/t58_live.tsv` (regenerable, not committed).
Helper: `scripts/t58_live_ab_run.sh` (lives in the agent worklog dir, not the repo —
thin wrapper over the canonical e2e script, no repo logic).

| arm | n | median | mean | min–max |
|---|---|---|---|---|
| A dormant | 5 | **16.738 s** | 16.653 | 16.038–16.938 |
| B noise-native | 5 | **16.542 s** | 16.468 | 16.113–16.777 |

Delta −0.196 s (−1.2 %), ranges overlap 100 %, per-run spread ±0.45 s — **statistically
indistinguishable; no boot-latency win, no regression** — exactly what §1 predicts.
Arm validation: `hook armed, retransform rc=0` + `self-test passed (native handle
round-trip through real bridge)` observed in 4/5 B runs (B1 verified by direct log
inspection post-run; B3/B4/B5 in-protocol; B2 killed before the post-Done chain landed —
validation-timing artifact, not a routing failure; capture-retransform counter lines are
0 in all runs — the pristine/resource-stream capture path arms without them).

## 3. Throwaway-bootab finding (infra, blocks future armed bootab runs)

The canonical throwaway harness (`run_bootab.sh`, flat/no-structures world) cannot host
armed experiments as-is: a B-arm throwaway boot (`run_bootab_noise.sh`, smoke B-1 log)
showed the worker probe loop (3 attempts, `Class.forName` succeeded, 283 natives
registered 0 unresolved, `normalNoise.nativeCheck() = 1` — kernel side healthy) but NO
`server booted, defining bridge` progression and no capture/arm markers before Done
(+kill). The boot-gate means armed throwaway runs need a post-Done grace window ≥ the
worker poll cadence (phase-2 mode) to complete arming — and even then arming is
post-Done, so throwaway boot-latency A/B of this surface is structurally unmeasurable.
The derived harness `bench/bootab/run_bootab_noise.sh` (normal worldgen + fixed seed
90919058 + vd=2) is committed for future worldgen-inclusive throwaway experiments; it
was NOT used for the verdict above (live-dir protocol instead).

## 4. Corrected record (TASK-57 I2)

The e2e courtesy guard is **flock-based**, not existence-based
(`e2e_orchestrate.sh:162`: `flock -n /tmp/crussty_bench.lock true`): a stale
zero-holder flag file does NOT block boots, and the guard's own `flock(1)` test
re-creates the file as a side effect (observed mtime during this session; 10 boots
ran fine with the file present). TASK-57's I2 ("stale lock blocks boots — removed
before boot") overstated the risk; the removal was harmless but not required.
The bootab script's own `/home/z/BENCH.lock` flock attempt (smoke "busy 30s —
proceeding") was contended by MY outer holder — correct serialization, no defect.

## 5. What remains honestly open

* The 2.87 CPU-s pre-Done noise burst is a real, measured Java-side boot cost that the
  current patch architecture cannot touch (boot-gated arming). Addressing it = an
  activation-gate redesign (pre-boot capture through the resource-stream path without
  the server object) — a new design task with real risk (the boot gate exists because
  the kernel loader is defined post-boot); NOT proposed as a default next step.
* Post-Done noise execution (the ~4 s tail of the burst + any worldgen while the server
  runs) DOES route native once armed — that is the already-proven value of the existing
  patch (PROVEN_WINS ~2x under contention; self-test passes each armed boot).

## 6. Cleanup / hygiene

Server stopped after the final run (as found). BENCH.lock released per run. No world
changes (pre-generated world, no console commands — stdin delivery still broken per
TASK-57 I1). Token not exposed. Tests/clippy not re-run (zero src/ changes this task —
report + docs + one derived bench script + harness comment only).
