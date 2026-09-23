# BATCH_SURFACE_CALIBRATION — TASK-51 (PROVEN_WINS_SYNC §4.1 closure)

**Date:** 2026-09-07T22:07Z (run) / 2026-09-08 +08 (report)
**Agent:** agent-7625532f
**BENCH.lock:** acquired 2026-09-07T22:07:48Z, exclusive, whole run
**Driver:** `bench/p500/calib/run_batch_surface_calib.sh` + `bench/p500/java/BatchSurfaceCalib.java`
**Raw:** `bench/p500/results/BATCH_SURFACE_CALIBRATION_RAW.tsv` (11 lines: 9 CALIB + 3 SINK + crash slots)

## 1. Context and task

PROVEN_WINS_SYNC §4.1 documented that 5 of the 12 (now 14) batch-surface registry
entries have **no pair in the canonical 2026-09-08 P500 rerun** — their
`P500 PARITY (batch surface)` verdicts rested on structural evidence
("registered surface, caller-initiated dispatch"), not on a measurement that
traces to canonical methodology:

| Registry id | Kernel | Prior evidence |
|---|---|---|
| 0 | `PaperNativeTicketSetSearch.binarySummary` | structural only |
| 1 | `PaperNativeTicketSetSearch.uncheckedBinarySummary` | structural only |
| 9 | `PaperNativeNoiseInterpolatorFractions.divisionSummary` | structural only |
| 10 | `PaperNativeClimateRTree.buildTreeHandle` | structural only, shape B `([J[J)J` |
| 11 | `net/minecraft/.../PaperNativeClimateRTree.nativeBuildTreeHandle` | structural only, shape B `([J[P)J` |

This task closes that gap: measure all 5 symbols under the canonical P500
methodology (120 ms time-bounded batches, median of 5, 2 discarded warmups,
DCE-proof sink, REAL closed-source `libpaper_native_jni.so`), then sync the
registry evidence strings. **No gate behavior changes** — verdicts stay
`P500 PARITY (batch surface)`; the numbers make them traceable.

## 2. Methodology — and why a dedicated driver

The generic `p500.Bench` harness was **deliberately not extended**:

1. **Handle leak:** `buildTreeHandle` returns a native handle. The generic
   driver has no per-op free hook; a 120 ms batch would leak ~1.5M native
   R-trees. A lifecycle-aware driver is mandatory.
2. **Cross-call state:** `binarySummary` per-call cost saturates under
   repetition (previously noted in session worklogs; never measured). The
   generic driver records one median, hiding the saturation shape. This driver
   adds a cold single-shot probe plus a full per-round trajectory.
3. **Gid stability:** appending rows to the generated `groups.tsv` is
   append-safe today but couples the canonical paired-group harness to
   lifecycle semantics; the registry references this calibration report
   instead. `groups.tsv`, `baseline.json`, and all historical reports are
   untouched (ratio-gate mapping unchanged, verified by `fqcn|sig` keys).

JVM flags identical to canonical: `-Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch
-XX:+UseG1GC`, libs loaded by exact path from `-Dp500.libs`, one JVM per group
(crash containment, `timeout 300`).

Per-kernel lifecycles:

* **ticketset** — `a0=1000` (the `src/lib.rs::live_proof` value), `long[256]`
  deterministic fill. Cold single-shot, then steady-state batches.
* **fractions** — `a0=64`, `long[64]` deterministic fill. Cold single-shot +
  steady-state.
* **rtree / rtree-full** — two variants:
  * `lifecycle(b+c+f)`: per op = `buildTreeHandle` + `checksumTreeHandle` +
    `freeTreeHandle`, all three inside the timed window (upper bound);
  * `build-only(K=256)`: 256 builds inside the timed window, checksum of the
    last handle as sink, all 256 frees **after** the window (bounded live set,
    free cost excluded). Inputs: `long[64]` / `long[256]` deterministic fills.

## 3. Results (medians, ns/op)

| Kernel (id) | Variant | Median ns/op | CV | Calls/120ms | Cold single-shot |
|---|---|---:|---:|---:|---:|
| TicketSetSearch.binarySummary (0) | steady-state | **530,433** | 0.41% | 228 | 6,849,571 (6.85 ms) |
| TicketSetSearch.uncheckedBinarySummary (1) | steady-state | **554,364** | 0.35% | 217 | 864,555 (0.86 ms) |
| NoiseInterpolatorFractions.divisionSummary (9) | steady-state | **411.2** | 0.18% | 291,837 | 494,365* |
| ClimateRTree.buildTreeHandle (10) | lifecycle(b+c+f) | **78.7** | 0.34% | 1,524,702 | 515,244* |
| ClimateRTree.buildTreeHandle (10) | build-only(K=256) | **56.4** | 2.31% | 256/round | — |
| ClimateRTree.nativeBuildTreeHandle (11) | lifecycle(b+c+f) | **78.7** | 0.32% | 1,530,334 | 612,145* |
| ClimateRTree.nativeBuildTreeHandle (11) | build-only(K=256) | **55.7** | 2.39% | 256/round | — |

\* single-shot numbers include first-call JNI symbol binding + JIT cold paths;
they are cold-entry references, not the calibration number.

All five per-round trajectories are in the raw TSV (`r1..r5` columns); every
method settled within the first measured round (no monotonic drift across
rounds beyond noise — see §4 for the within-run saturation that already
completed during warmup).

## 4. Stability and the TicketSetSearch saturation, now measured

* **TicketSetSearch.binarySummary** — the worklog-era note ("per-call cost
  saturates ~450–550 µs under repetition, cross-call state") is **confirmed and
  refined**: cold entry = **6.85 ms** (13× steady state — first-call index
  build), steady state = **530 µs/call** (CV 0.41%, 228 calls/120 ms window).
  The saturation completes during the discarded warmups; all 5 measured rounds
  sit at 527.6–533.6 µs. `uncheckedBinarySummary` behaves differently: cheap
  cold (0.86 ms) but slightly **more** expensive steady (554 µs) — the checked
  variant amortizes a one-time setup the unchecked variant pays per call (or
  equivalent opaque internals; closed source, both readings documented).
* **divisionSummary** — textbook floor-band+ kernel: 411 ns steady, CV 0.18%
  over 291,837 calls/window. ~3.5× above the 35–90 ns JNI transition floor →
  mildly body-dominated; batch amortization could not help it anyway (no pair,
  batch NO-GO by TASK-47/48 constants).
* **rtree pair** — CV ≤ 2.4% across both variants; lifecycle minus build-only
  ≈ 22 ns = checksum+free pair cost. **id10 ≡ id11 within noise** (78.7/78.7
  lifecycle, 56.4/55.7 build-only) — the short-form and full-name registered
  symbols are behaviorally identical, consistent with the alias registration.

## 5. Honest boundaries and findings

1. **rtree null-handle boundary (important).** The post-run `SINK 0` for both
   rtree groups proves the sampled build handles are **0**: on the synthetic
   deterministic inputs the closed-source kernel returns a null/empty tree and
   the measured 56–79 ns/op calibrate the **JNI transition floor of the
   registered symbol**, not real R-tree construction. Kernel arg semantics are
   opaque; no realistic-input source exists in-repo (`live_proof` exercises
   ticketset, not rtree). Consequence: the numbers are a *dispatch-cost lower
   bound and alias check*, NOT a gameplay construction-cost claim. The
   registry verdict (`PARITY (batch surface)`, structural) remains the honest
   performance statement for id10/id11; the measurement now also traces.
2. **TicketSetSearch is measured at fixed opaque inputs** (`a0=1000` — the
   live_proof value — over `long[256]`; live_proof itself uses `long[1]`). The
   530/554 µs steady-state numbers calibrate the registered surface at those
   inputs and confirm the cross-call-state hazard for any future bench
   re-use (HOTSPOT_CANDIDATES_V2 warning now has a measured basis).
3. **Single-shot probes** are cold-entry references (JNI bind + JIT), not
   kernel cost; they are reported for reproducibility of the cold columns.
4. **No pair, no ratio.** These kernels have no old/new counterpart, so no
   old-vs-new ratio exists anywhere in this report; the batch NO-GO verdict for
   wave-1 (TASK-47/48/50 constants: batch wins only near the 90 ns transition
   ceiling at large K) is **unchanged** — divisionSummary (411 ns) and rtree
   (56–79 ns floor) both sit where batch dispatch mathematically cannot win;
   ticketset (530 µs steady) is body-dominated but carries cross-call state
   that makes batch fan-out of interleaved calls semantically unsafe to project.
5. **Environment:** shared 2-CPU box; run held the exclusive BENCH.lock for the
   whole duration; no live-server interference windows opened (live server not
   running during this tick's bench window).

## 6. Registry sync and files changed

* `src/kernel_policy.rs` — evidence strings for ids 0, 1, 9, 10, 11 now cite
  this report's measured numbers (verdicts/Allow-set/DO_NOT_WIRE untouched;
  zero gate-widening, zero gate-narrowing).
* `docs/PROVEN_WINS_SYNC.md` — §4 item 1 marked resolved (was: "would require
  a bench run — forbidden in this task"), pointer to this report.
* `docs/KERNEL_POLICY.md` — batch-surface flag note updated (§ "no pair"
  wording → measured).
* `bench/p500/java/BatchSurfaceCalib.java`, `bench/p500/calib/run_batch_surface_calib.sh`
  — the lifecycle-aware calibration driver (permanent bench asset).
* `bench/p500/results/BATCH_SURFACE_CALIBRATION_RAW.tsv` — raw trajectories.
* Untouched by design: `groups.tsv`, `baseline.json`, `kernel_policy` verdict
  labels, `batch_table.rs`, live deployment, engine `.so` files.
