# AREAMAP BUDGETED SCRATCH — variant C results (TASK-64 phase 2, S7-17)

Agent: main (session S7-17, cron 12:43+08 Job 366450) · 2026-09-08 · base: c-crussty master 596b104
Design: `docs/AREAMAP_DENSE_APPLY_DESIGN.md` §11 (STEP-0 probe + variant C spec, S7-16)
Status: **IMPLEMENTED + GATED, oracle-green, A/B measured. Default OFF (fail-safe). Not yet live-armed** (see §6).

---

## 0. Verdict

| Gate | Result |
|---|---|
| TASK-30 oracle, BUDGET arm, FAKE (contract-emulating stub) | **268/268 PARITY**, 0 failures |
| TASK-30 oracle, BUDGET arm, **REAL (closed native)** — §11.2 falsifier 2 | **268/268 PARITY**, 0 failures |
| TASK-30 oracle, LEGACY REAL control (rig sanity) | 268/268 PARITY |
| A/B cross-arm multiset parity (sum + XOR folds, identical native+streams) | **8/8 (d,shape) pairs OK** |
| cargo test (64 incl. 3 new budget tests) / clippy Δ0 | 64/64 / Δ0 (21 pre-existing) |
| Offered capacity check (budgeted policy) | `budgetOK` on every oracle call; monotone growth, no drop, no retry-storm |

**The closed native accepts budgeted (`len ≥ n`) buffers with no hidden len-dependent semantics** on every
TASK-30 stream (move / resize / mix, d=63/255/511). Variant C is semantically clean against the deployed
`libpaper_native_jni.so` (sha256 `d8f821aa…`, the same build STEP-0 measured).

## 1. What shipped (commit-refd)

| File | Change |
|---|---|
| `area-map/budget-ca/.../SingleUserAreaMapOpsBudget.java` | NEW: budgeted policy, class name `SingleUserAreaMapOps` (file-name trick documented in-source); `-n0` single retry; monotone `min(cap, 2·n)` post-call growth; never shrinks; `req > cap`/second-fail/`n > len` → fail-safe emit-nothing (H-12 follow-up) |
| `area-map/build-budget/` | NEW artifacts (3 classes, major 52, same Scratch newarray guard as legacy build) |
| `scripts/build_area_map_budget.sh` | NEW (mirrors legacy guards; legacy `build_area_map.sh` untouched) |
| `src/area_map.rs` | `CRUSSTY_AREAMAP_BUDGET` gate (OnceLock, only exact `on` widens); byte-set selection at define time; **native-contract self-test in the live JVM before arming** (per-rect: cap control parity / `-n0` exact reject / `n0` boundary parity); markers; 3 unit tests |
| `bench/areamap/benchjava/.../OracleBench.java` | `-Dcrussty.areamap.budget=true` mode: legacy `cap ≥ maxOps` assert collapses to `roomOk` (offered ≥ actual diff); default path byte-identical |
| `bench/areamap/budgetab/` | NEW rig: contract-emulating fake (STEP-0 §11.1 behavior), `BudgetAbBench` driver |
| `bench/areamap/run_budget_ab.sh` | NEW: oracle battery + A/B under BENCH-MUTEX, file-disjoint classpaths |

Gate semantics: off (default) = legacy bytes defined verbatim, zero behavioral delta, no runtime flag.
On = contract self-test gates arming; **any violation → legacy bytes defined instead** (define_class for the
same name is one-shot, so the decision must precede definition). Kill-switch = env flip + restart, no rebuild.

## 2. A/B end-to-end per-call cost (REAL native, median of 5 window-means, BENCH-MUTEX)

`BUDGETAB arm=… median_ns_per_call` (`bench/areamap/results/budget_ab_raw.tsv`), streams = TASK-30 shapes
(8-dir bounded ring walk; mix = move every call + bounded ±1 resize every 4th):

| d | shape | legacy (cap scratch) | budgeted | win |
|---|---|---|---|---|
| 33 (prod-max) | move | 6 291.0 ns | 2 032.2 ns | **3.10×** |
| 33 | mix | 12 322.6 ns | 5 487.6 ns | **2.25×** |
| 63 | move | 20 387.4 ns | 3 381.6 ns | **6.03×** |
| 63 | mix | 27 658.5 ns | 14 356.6 ns | **1.93×** |
| 255 | move | 969 696.4 ns | 17 327.0 ns | **56.0×** |
| 255 | mix | 1 058 601.3 ns | 171 329.5 ns | **6.18×** |
| 511 | move | 3 672 911.4 ns | 39 038.4 ns | **94.1×** |
| 511 | mix | 3 752 548.8 ns | 644 135.0 ns | **5.83×** |

Cross-arm folds identical on all 8 pairs (`budget_ab_parity.txt`) — same native, same emitted multisets.

## 3. Cost-model finding (new characterization, extends STEP-0 §11.1)

Per-call `BudgetDiag` splits (scripts/budgetdiag, reflection capacity probe + per-call timing):

| shape | legacy cost | budgeted cost | decomposition |
|---|---|---|---|
| move | ∝ 9·L·k (copy-dominated) | ≈ n·C (callback-dominated, C≈12 ns in rig) | budget removes the whole copy term → 94× @d=511 |
| resize ±1 | fixed(d) **+** 9·L·k | ≈ fixed(d) | d=63: 38→17 µs (2.2×); d=255: ~1150→~650 µs (1.75×); the resize fixed part is **len-insensitive** (~650 µs @d=255) |

The mix-shape window-means are dragged by the resize tail (¼ of calls): that is a property of the closed
native's resize path, **not** of the scratch policy — the budget arm pays the same fixed part. Honest
summary: **move-shaped updates win 3.1–94×; resize-shaped win 1.75–2.2×; >100×-class stays a move-only,
d≥511 phenomenon** (native-leg alone measured 299× at d=511, S7-16).

## 4. Falsifiers (§11.2) — all resolved

1. **-n0 retry storm**: capacity trajectory monotone (`budgetOK` in every oracle line; `BudgetDiag` shows cap
   stabilizing after warmup: 4100 @d=255, 1012 @d=63); sawtooth n absorbed by the 2× adaptive headroom.
2. **Hidden len-between-n-and-cap semantics**: REAL-budget oracle 268/268 PARITY — none found on any stream.
3. **Hot-path ThreadLocal alloc**: realloc only in the retry or post-call growth; steady-state call allocates
   nothing (code invariant + capacity trajectory flat after warmup).

## 5. Markers (grep canon `[crussty-plugin] area_map: ...`)

```
budget gate CRUSSTY_AREAMAP_BUDGET=<shown> -> mode=<on|off> (<note>)        // boot, once
budget path OFF (gate off; legacy grow-to-cap scratch)                      // unarmed activation
budget self-test OK (N probes, len>=n contract + -n0 size oracle verified)  // armed activation
budget path ARMED (SingleUserAreaMapOps = budgeted scratch, -n0 retry)      // after retransform
budget self-test FAILED -> legacy scratch retained                          // fail-safe fallback
```

## 6. Rollout state and next steps

- Default OFF. Stage-1 (armed live boot) intentionally **deferred**: the server is TASK-63's claimed lane
  (paired noise-bridge A/B, neighbor agent) — no boots this session per COORDINATION no-cross rule. Live
  verification checklist for the boot session: gate marker present; contract self-test OK; `budget path ARMED`;
  `flushes=1`-style marker sanity unchanged; P500 spot check; hs_err passive monitor.
- Prod projection (d≤33, 6 maps/crossing, mostly move-shaped): native leg ≈ 2–3×; whole-call ≈ 1.3–2×
  (callbacks dominate). The lever's honest value is on large view-distance grids and teleport-like moves.
- NEXT (1): live armed boot under BENCH/server-mutex when the lane frees. NEXT (2): variant A (dense body)
  decision — with the resize fixed-cost discovery, dense-apply would ALSO bypass the ~650 µs resize fixed
  part, strengthening the A-variant case for d≥255 grids. NEXT (3): TASK-30 oracle budget arm is now part of
  the lever's re-run duty for any future kernel change.
