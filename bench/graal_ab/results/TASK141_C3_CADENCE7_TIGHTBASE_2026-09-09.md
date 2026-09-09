# TASK-141 — C3 CADENCE #7 (replication leg): SECOND tripwire fire via a NEW benign path — tight-baseline forced committed expansion; discriminator clean ALREADY at R2

**Agent:** agent-7625532f · 2026-09-09 · boot-lane quiet window (twin S7-87 MILESTONE banked b03b3bb/§85 per COORD NOTE — numbering prediction held; no S7-88 claim at claim time).
**Rig:** `bench/graal_ab/run_task129_pure_inject.sh` — drift-guard `8ba2473c…` verified unchanged pre-run. Canonical pure-inject (stock Temurin 21.0.12.1 + ONLY `-agentpath`; external observers; INJECTS-ONLY).
**RAW:** `bench/graal_ab/RAW_TASK129_PURE_20260909_110111/` (fifo removed at banking). **Claim:** dev-logs 302065a (TASK-141).

## 1. Headline numbers (N=1)

| leg | RSS (MB) | heap used / committed |
|---|---|---|
| R0 baseline | **830** (series min) | 430.5 / 487.0 → **free-room 56.5 MB (corpus min)** |
| R1 post-add | 922 | 475.6 / 528.9 → Δused@add **+45.1 (early-call gray zone)** |
| R2 remove-end (120 s settle) | **956** | **408.2 / 552.0** → Δused **−22.3 BELOW baseline**; Δcommitted **+65.0** |
| R3 after `GC.run` (+15 s) | 964 | **249.4 / 552.0** → Δused **−181.1 BELOW baseline** |

- Boot **16.642 s** — 7th consecutive pure-inject boot inside the parity band (15.96–17.58); crussty-runtime v2.0.0 rc=0.
- v2 gate = max(830×1.10, 830+100) = **930.0** (floor binds; R0 below crossover). R2 = 956 → **tripwire fired** → `GC.run` attribution → R3 = 964 still over → tree verdict **FAIL-leak-signature**. v1_class = exceeds-v1, v1_final = FAIL. **Verdict kept verbatim (no-reclassification law).**

## 2. NEW benign path — the tripwire fired WITHOUT the no-GC mechanism

TASK-139's fire ran through the no-GC path (churn garbage uncollected at R2: used@R2 +186.6 above baseline). **This fire is structurally different:**

1. **A young GC fired during the remove-settle** — used@R2 = 408.2 MB is **−22.3 MB BELOW the 430.5 MB baseline**. The used-heap discriminator was already clean at R2, *before* the diagnostic `GC.run`. This is the strongest benign evidence in the series: the live set shrank below baseline without any attribution step.
2. The RSS elevation is **committed-side**: Δcommitted(R0→R2) = **+65.0 MB** (487.0 → 552.0) and G1 did not return it (552.0 flat through R3). With R0-used at 430.5 of 487.0 committed, the baseline heap was **tight (free-room 56.5 MB — corpus minimum)**: the churn's ~170 MB allocation forced immediate committed expansion; even with in-settle collection the expansion persists (lazy non-uncommit).
3. **Resid decomposition (pre-registered check):** resid 126 vs model Δcommitted + 57 = 122 → **err +4 MB — second consecutive pass** (TASK-140 §5 check). The R3 leg reads 134 vs 122 (+12) — within the same structural class (native term boot-variable ±5–10 MB).
4. **Leak invariant:** post-GC used −181.1 MB below baseline — **7/7** across the corpus. Trajectory: 922 → 949 → 953 (flat) → 956, final ~90 s drift +3 MB = **plateau**. Histogram at R2 ≈ identical live profile to TASK-139 (vanilla/moonrise collision caches ~250 MB total).

## 3. Corpus synthesis — the gate floor is now empirically tight: TWO benign paths over the 100 MB constant

| path | runs | mechanism | resid |
|---|---|---|---|
| no-GC path | T139 | garbage persists through settle → committed expands to churn peak | +125 |
| **tight-baseline path (NEW, T141)** | **#7** | **GC fires, but tight R0 forces committed +65 that persists** | **+126** |
| in-band path | T129-R1/#3/#4/#5 | GC lands before R2; committed ≈ flat | +61…+90 |

**Anti-gate-shopping statement (explicit):** 2 of 7 canonical runs now trip v2 with fully benign attributions. The v2 floor constant (100 MB) was calibrated as the upper edge of the NMT-attributed benign band on free-ish baselines; the corpus now shows the benign band reaching +126 under baseline-tightness variance. **The gate is NOT being retuned.** The v2 protocol document explicitly disclaims tuning ("not tuned to rescue the TASK-129 run"); revising a gate after adverse readings is gate-shopping and would destroy the series' evidentiary value. v2 stays binding. The observation is banked as a **v3 discussion item for the owner** (with this corpus as evidence); any revision would follow the TASK-129→v2 pre-registration flow, not an in-line edit.

## 4. Early-call rule refinement (for cadence #8)

TASK-140's rule (Δused@add ≤ +25 ⇒ PASS; ≥ +75 ⇒ risk) has a **gray zone** — and #7 shows the gray zone can still trip via tightness. Refined rule (two risk factors, either suffices):

- **R1 factor:** Δused@add ≥ +75 MB (no-GC path risk).
- **R0 factor (new):** free-room@R0 ≤ ~60 MB (tight-baseline path risk — GC or no GC).
- #7's reading: Δused@add +45.1 (gray) BUT free-room 56.5 → tripwire-risk would have been declared at the baseline sample, before the boot even churned. Both corpus fires are covered by the refined rule; all five in-band runs have both factors negative.
- Prediction for #8: tripwire fires only when a factor is positive at sampling time; resid must still decompose within ±10 MB; invariant must hold. Any fire with a clean decomposition + clean invariant = benign per the corpus-grade model; a decomposition failure (> +10 MB) would be the first unexplained residue in series history → escalate.

## 5. Banking

- Ledger §86 ADDENDUM-75 (numbered after pull + tail grep: tail was §85 — twin's S7-87 MILESTONE, their COORD-NOTE numbering held exactly as predicted). INDEX row added. RAW fifo removed.
- 1 boot total, hs_err delta 0, zero config, INJECTS-ONLY. Lane untouched (twin between campaigns; S7-88 = rcon remedy + SERVING soak per their §85 NEXT).
