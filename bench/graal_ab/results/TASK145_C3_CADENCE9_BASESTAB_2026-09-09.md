# TASK-145 — C3 CADENCE #9 (BASELINE-STABILITY LEG): third tripwire fire via the tight-baseline path EXTENDED (free-room 67.0 MB, both early-call factors negative — binary rule honestly missed); baseline health PROVEN (R² 0.984 committed-side, live set dead-stable across the day)

**Agent:** agent-7625532f · 2026-09-09 · quiet window (S7-91 unclaimed, zero java procs, flock free).
**Rig:** `bench/graal_ab/run_task129_pure_inject.sh` — drift-guard `8ba2473c…` verified unchanged pre-run. Canonical pure-inject (stock Temurin 21.0.12.1 + ONLY `-agentpath`; INJECTS-ONLY).
**RAW:** `bench/graal_ab/RAW_TASK129_PURE_20260909_122108/` (fifo removed). **Claim:** dev-logs aa1b117. **Script:** `/home/z/my-project/scripts/task145_baseline.py` (persisted).

## 1. Headline numbers (N=1)

| leg | RSS (MB) | heap used / committed |
|---|---|---|
| R0 baseline | **874** | 462.0 / 529.0 → **free-room 67.0 MB** (R0-factor NEGATIVE under the registered ~60 threshold — second-tightest of corpus) |
| R1 post-add | 933 | 454.3 / 539.0 → Δused@add **−7.7 (R1-factor NEGATIVE; a GC fired in the add-leg)** |
| R2 remove-end | **978** | 406.2 / 576.0 → Δused **−55.8 BELOW baseline**; Δcommitted **+47.0** |
| R3 after `GC.run` (+15 s) | 984 | **248.5 / 576.0** → Δused **−213.5 below baseline** |

- Boot **18.000 s** — **FIRST pure-inject boot OUTSIDE the parity band** (15.96–17.58; +0.42 s over prior max). Disclosed honestly; boot time is not a gate input and the run verdict is unaffected. crussty-runtime v2.0.0 rc=0; inject parity markers present.
- v2 gate = max(874×1.10, 874+100) = **974.0** (constant binds). R2 = 978 → **tripwire fired by 4 MB** → `GC.run` → R3 = 984 → tree verdict **FAIL-leak-signature**. v1_class = exceeds-v1, v1_final = FAIL. **Verdict kept verbatim (no-reclassification law).**
- Trajectory: 976 flat → 978 for the final 4 samples (~40 s) = PLATEAU. Histogram ≈ corpus live profile (AABB/voxel/moonrise caches; no foreign classes).

## 2. Attribution block (separate from the verdict, per law) — benign, third path-instance, TWO new facts

**Benign by every registered discriminator:** used@R2 was −55.8 MB BELOW baseline *before* the diagnostic GC.run (discriminator clean at R2, as in T141); resid decomposition = **PERFECT pass** (resid 104 vs Δcommitted 47 + native 57 = 104, **err 0.0 MB** — first zero-error point of the series, 6/6 full-data points closed); leak invariant **9/9** (post-GC used −213.5); post-GC live set 248.5 MB ≈ T141's 249.4 (see §3). Mechanism: tight-baseline forced committed expansion (Δcommitted +47.0 persists, lazy non-uncommit) — the SAME structural class as T141, but at free-room 67.0 MB, which the registered binary R0-factor (~60 MB) classified as negative.

**NEW FACT 1 — the binary early-call rule is honestly missed (3-for-4).** #9 fired with BOTH registered factors negative (free-room 67.0 > ~60; Δused@add −7.7 ≤ +25 predicted PASS). The rule as stated in TASK-141 §4 is **falsified as a sufficient predictor**. Corpus mapping (fires at 56.5 and 67.0; passes at 98.1, 128.0, 132.9, 167.0, 210.5, 228.2): the tight-band boundary lies somewhere in **67–98 MB and is probabilistic** (eden-phase lottery — #3 at 98.1 with the SAME add-leg GC profile as #9 did not expand: Δcommitted +2.2 vs +47.0). The binary threshold was derived from a single observation (T141's 56.5) and one-point-derived thresholds break at the next point — this is the same lesson as the TASK-140 caveat, now confirmed on the factor itself. Any future rule revision must treat tight-band risk as a continuum, and per the anti-gate-shopping law this is registered as an observation, NOT a gate or rule edit (v3 discussion item evidence, TASK-143).

**NEW FACT 2 — the unexplained-residue reading is 0.0 MB on a fire.** An O3-style structure-aware gate (fire only when resid > Δcommitted + 57 ± 10) would have **fired ZERO times on the entire 9-run corpus** (max unexplained residue across all runs incl. all three fires = +4 MB) while remaining the true dangerous-class detector. This materially strengthens option O3 in the owner discussion (TASK-143) — recorded as evidence there; nothing is changed here.

## 3. PRIMARY QUESTION — baseline stability: BOTH pre-registered predictions PASS; baseline is healthy

The motivation for this leg was #8's observation of R0 drift (830 → 956, series-min → series-max, non-monotonic). Verdict on the corpus (script output verbatim):

- **P1 PASS:** `R0_RSS = 0.9746 × committed@R0 + 357.8` over 8 points with committed@R0 — **R² = 0.9838 ≥ 0.85**. The R0 variance across boots is entirely committed-side (G1 initial-heap sizing), slope ≈ 1.0 (RSS tracks committed 1:1), and the non-heap RSS remainder is a **constant 357.8 MB** — no independent drifting RSS source exists at baseline.
- **P2 PASS:** used@R0 Spearman vs boot index **rho = 0.217** (significance needs ≥ 0.683 at n = 9) — **no upward trend**. Raw used@R0 is a garbage-inclusive GC-phase reading (range 305.8–482.5, non-monotonic: 482.5 at run 3, 305.8 at run 6, 482.0 at run 8).
- **Decisive cross-check — the true live set is dead-stable:** post-full-GC used across the day = **250.0 (T129-R2) / 249.4 (T141 #7) / 248.5 (#9)** — 1.5 MB spread over ~3.5 h of operation and 9 boots. Whatever variance raw used@R0 shows is uncollected garbage at sample time, NOT live-set growth. **No baseline-level leak signature exists in this project's data.**

## 4. Corpus state after #9 (9 runs)

| class | runs | resid |
|---|---|---|
| in-band PASS | T129-R1, T129-R2, #3, #4, #5, #8 | +61…+92 |
| fire, benign (no-GC path) | T139 | +125 |
| fire, benign (tight-baseline path) | T141 (+126), **#9 (+104)** | +104…+126 |
| **fires / total** | **3/9 — ALL benign-attributed** | — |
| **invariant** | **9/9 post-GC/trough used ≤ own baseline** | — |
| **resid model** | **6/6 full-data points closed (−9…+4, incl. one 0.0)** | — |
| **early-call rule** | **3-for-4 (binary R0-factor falsified at 67 MB)** | — |

## 5. Banking

- Ledger §(numbered after pull + tail grep; tail was §91) — this block. INDEX row + RAW (fifo removed) + persisted script.
- TASK-143 discussion doc: dated ADDENDUM appended (evidence update: 3/9 fires all benign; O3 would fire 0/9; binary-factor lesson) — the doc remains a discussion item, nothing applied.
- 1 boot, hs_err delta 0, zero config, INJECTS-ONLY. Lane untouched (twin S7-91 = a28 re-measure + Nio decode per §90 NEXT).
