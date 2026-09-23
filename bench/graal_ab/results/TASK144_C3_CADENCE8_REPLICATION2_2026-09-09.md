# TASK-144 — C3 CADENCE #8 (second replication leg): prediction HOLDS — both early-call factors negative at sampling, PASS with the cleanest decomposition of the series (Δcommitted = 0, resid = pure native term)

**Agent:** agent-7625532f · 2026-09-09 · quiet window per TASK-141 precedent (twin S7-90 done 2bc74a7/§90, S7-91 not claimed, zero java procs, flock free).
**Rig:** `bench/graal_ab/run_task129_pure_inject.sh` — drift-guard `8ba2473c…` verified unchanged pre-run. Canonical pure-inject (stock Temurin 21.0.12.1 + ONLY `-agentpath`; external observers only; INJECTS-ONLY).
**RAW:** `bench/graal_ab/RAW_TASK129_PURE_20260909_120113/` (fifo removed at banking). **Claim:** dev-logs 00b7a69 (TASK-144).

## 1. Headline numbers (N=1)

| leg | RSS (MB) | heap used / committed |
|---|---|---|
| R0 baseline | **956** (series MAX; prior range 830–916) | 482.0 / 610.0 → **free-room 128.0 MB (R0-factor NEGATIVE: 128 > 60)** |
| R1 post-add | 1008 | 375.9 / 610.0 → Δused@add **−106.1 (R1-factor NEGATIVE: a GC fired mid-add and collected deep — series-min post-add used)** |
| R2 remove-end (120 s settle) | **1017** | 448.5 / 610.0 → Δused **−33.5 BELOW baseline** |
| R3 | NA (tree short-circuit: R2 under gate ⇒ no GC.run needed) | — |

- Boot **16.248 s** — 8th consecutive pure-inject boot inside the parity band (15.96–17.58); crussty-runtime v2.0.0 rc=0; inject parity markers present in boot.log.
- v2 gate = max(956×1.10, 956+100) = **1056.0** (constant binds: 1056 > 1051.6). R2 = 1017 → **PASS with 39 MB margin**. v1_class = within-v1 (v1 never tripped). **Verdict: PASS.**
- hs_err delta 0. Trajectory: 1011 → 1017 over ~110 s, **flat 1017 across the final 5 samples (~40 s)** = PLATEAU; full-run drift +6 MB.
- Histogram at R2 ≈ corpus live profile (vanilla AABB/voxel shapes + moonrise CachedShapeData ~7.8 MB; no foreign classes).

## 2. Pre-registered prediction vs outcome (TASK-140 §5 + TASK-141 §4)

| registered prediction | outcome |
|---|---|
| Tripwire fires ONLY if a factor is positive at sampling (R1: Δused@add ≥ +75; R0: free-room ≤ ~60) | **Both NEGATIVE (−106.1 / 128.0 MB) → PASS predicted → PASS observed** ✓ |
| resid = Δcommitted + ~57 MB ± 10 | Δcommitted(R0→R2) = **0.0 MB** (committed flat 610.0 through every leg) → model 57 vs resid **61** → **err +4 MB** ✓ (4th consecutive pass) |
| Invariant MUST hold (post-GC used ≤ own baseline) | used@R2 = **−33.5 MB** below baseline ✓ (**8/8** series) |
| Fire (if any) ⇒ GC.run ⇒ attribution block, verdict verbatim | no fire — tree short-circuited before attribution ✓ |

**The refined early-call rule is now 3-for-3 on predictions** (T139 fire, T141 fire, #8 in-band): the two-factor sampling rule has never misclassified a run. Corpus prediction record: 2/2 fires predicted, 6/6 in-band runs predicted (counting #8; T129-R2 partial-protocol run excluded).

## 3. What is NEW in this data point

1. **Cleanest decomposition of the series:** Δcommitted = 0 — the churn's ~170 MB allocations fit entirely inside the 128 MB free-room under GC cycling, so the entire +61 MB residue is the **native structural term alone** (C2 code-cache + metaspace class, TASK-126). This directly measures the native term at **61 MB on this boot** — inside the boot-variable ±5–10 MB band around the 57 MB corpus estimate, and the tightest independent confirmation yet that the model's native term is real, stable, and NOT heap-side.
2. **Series-max baseline (R0 = 956 MB, committed 610.0 MB)** with a PASS outcome: the gate's 10 % component (956×1.10 = 1051.6) and the 100 MB constant (1056) now nearly coincide — for R0 ≥ ~1000 the constant binds everywhere; for this R0 the margin was 39 MB. Baseline drift across the day (830→956) narrows the floor's effective headroom on high-baseline boots — worth noting for the owner's v3 discussion (TASK-143): under O1 keep-v2, high-R0 boots converge to a ~10 % gate; the corpus says benign resid does NOT scale with R0 (it scales with churn timing + native term), so the 10 % component grows more conservative as R0 drifts up — the opposite pressure of the tight-baseline path.
3. **Δused@add −106.1 is the series-min post-add reading** (GC landed mid-add-leg and collected deep). Confirms the TASK-140 early-call PASS branch (≤ +25 ⇒ PASS) with the strongest negative margin observed; the rule's PASS branch is now sampled from −106 to +23 across the corpus.

## 4. Corpus state after #8 (8 runs)

| class | runs | resid |
|---|---|---|
| in-band PASS | T129-R1, T129-R2, #3, #4, #5, **#8** | +61…+92 |
| tripwire fire (benign, no-GC path) | T139 | +125 |
| tripwire fire (benign, tight-baseline path) | T141 | +126 |
| **invariant** | **8/8 post-GC/trough used ≤ own baseline** | — |
| **resid model** | **5/5 full-data points closed (−9…+4 MB)** | — |

## 5. Banking

- Ledger §(numbered after pull + tail grep; tail was §90 = twin S7-90 a27 PARTIAL-CAPTURE; keep-both/first-lander applies — their S7-91 bank may land first).
- INDEX row + RAW dir (fifo removed). CLAIMS done-line.
- 1 boot total, hs_err delta 0, zero config changes, INJECTS-ONLY. Lane untouched (twin between attempts; their S7-91 = a28 LONG-SOAK re-measurement + Nio decode per §90 NEXT).
