# TASK-143 — V3 FLOOR DISCUSSION ITEM FOR OWNER: the C3 tripwire corpus says the 100 MB constant is empirically tight (2/7 benign fires); options enumerated, nothing changed

**Agent:** agent-7625532f · 2026-09-09 · **ZERO boots, zero runs** — decision-support package assembled entirely from banked results. Boot lane left to twin S7-90 (ATTEMPT-27 claimed a8c94ad, rig v12.8.1 LONG-SOAK; BENCH-MUTEX untouched).
**Status:** this document is the **v3 discussion item** explicitly promised in TASK-141 §3 ("banked as a v3 discussion item for the owner, with this corpus as evidence"). **The v2 gate remains binding and is NOT retuned by this task.** Nothing in the rig, protocol, or verdict handling changes without an owner mandate followed by the TASK-129→v2-style pre-registration flow.

## 1. What v2 is and why it is currently untouchable

The C3 tripwire gate (protocol v2, binding at source in `run_task129_pure_inject.sh`) fires when `RSS(R2) > max(R0×1.10, R0 + 100 MB)`. The 100 MB constant was calibrated as the upper edge of the NMT-attributed benign band on free-ish baselines, and the v2 document explicitly disclaims tuning ("not tuned to rescue the TASK-129 run"). Two laws protect the series' evidentiary value:

- **Anti-gate-shopping:** revising a gate after adverse readings destroys the meaning of every verdict the gate ever produced. Adverse readings are handled by attribution, not by moving the threshold.
- **No-reclassification:** once a verdict is rendered (e.g., `FAIL-leak-signature`), it is preserved verbatim; benign attribution is recorded in a separate block, never by editing the verdict.

Both fires below were handled under these laws. The question for the owner is only whether a **future** gate (v3) should be calibrated differently — decided with the full corpus in hand, not in the middle of a run.

## 2. The evidence: 7-run pure-inject corpus, two benign paths over the constant

| run | R0 RSS | free-room@R0 | Δused@add | RSS resid (R2−R0) | resid model err | GC story | verdict |
|---|---|---|---|---|---|---|---|
| T129-R1 | 889 | 167.0 MB | +75.2 | +68 | +3 | GC in remove-settle | PASS |
| T129-R2 | 877 | — | — | +92 | — | GC.run attribution | PASS (post-attribution) |
| T132 #3 | 916 | 98.1 MB | −8.5 | +61 | +2 | GC in add-leg | PASS |
| T134 #4 | 900 | 210.5 MB | +8.4 | +61 | +4 | GC in add-leg | PASS |
| T135 #5 | 889 | 132.9 MB | +22.8 | +90 | −9 | GC in add-leg | PASS |
| **T139 #6** | 878 | 228.2 MB | +178.7 | **+125** | −3 | **NO GC through R2** | **FAIL-leak-signature** (benign: attribution block) |
| **T141 #7** | 830 | 56.5 MB | +45.1 | **+126** | +4 | **GC fired; tight baseline forced +65 committed** | **FAIL-leak-signature** (benign: discriminator clean at R2) |

Facts that matter for the decision:

1. **2 of 7 canonical runs trip v2, and both are fully benign** — via two *structurally different* mechanisms (TASK-139: garbage-detection latency on a large-free-room baseline, used@R2 +186.6 above baseline; TASK-141: young GC succeeded — used@R2 −22.3 BELOW baseline — but the tight 56.5 MB free-room baseline forced Δcommitted +65.0 that G1 did not return).
2. **The residue model closes every point**: `resid ≈ Δcommitted(R0→R2) + ~57 MB native` with errors −9…+4 MB on all five full-data runs (TASK-140 §84). There is no unexplained RSS anywhere in the series.
3. **The leak invariant holds 7/7**: every post-GC/trough used-heap reading is at or below its own boot's baseline. The authoritative used-heap discriminator (protocol §4) has never been violated on the canonical config.
4. **The fires are now predictable mid-run**: the refined early-call rule (TASK-141 §4) covers both fires with two factors — R1: Δused@add ≥ +75 MB (no-GC path); R0: free-room@R0 ≤ ~60 MB (tight-baseline path). All five in-band runs are negative on both factors.
5. The corpus-max benign resid is **+126 MB** against a 100 MB constant — i.e., the benign band demonstrably reaches ~26 % past the floor under baseline-tightness variance, and the no-GC path can in principle push committed to the churn peak on *any* baseline (T139's free-room 228 MB was corpus-max but the mechanism is not bounded by free-room alone; the ±90 s eden-phase lottery, TASK-140 §3, is honest).

## 3. Options for v3 (discussion items only — no option is endorsed or applied)

**O1 — keep v2 as-is.** The fires are known-benign classes, predicted mid-run by the early-call rule, resolved by the attribution protocol, and honestly recorded as FAIL verdicts with attribution blocks. Cost of status quo: 2/7 runs produce scary-looking FAIL rows in the TSV that require reading the attribution block to interpret, and each fire consumes a diagnostic `GC.run` + R3 leg. Evidentiary continuity: perfect. This is the default if the owner does nothing.

**O2 — widen the floor constant (e.g., 100 → 150 MB).** Would have cleared both observed fires (T141: gate would be max(913, 980)=980 > 956; T139: max(965.8, 1028)=1028 > 1003). Tradeoffs: (a) n=2 calibration-by-anecdote — the no-GC path is not bounded by any constant, so a wider floor is a bet that future churn/free-room combinations stay under it; (b) reduces sensitivity to genuine small leaks in exactly the range (+100…+150) where a real regression would first appear; (c) still an absolute constant, still blind to whether the growth is committed-side (benign-lazy) or live-set-side (dangerous).

**O3 — structure-aware gate: fire on unexplained residue, not raw RSS.** v3 semantics: tripwire only when `resid(R2−R0) > Δcommitted(R0→R2) + 57 MB + 10 MB` — i.e., when the corpus-grade decomposition FAILS. This directly targets the dangerous class (RSS growth not accounted for by committed laziness + native structure) and is immune to both observed benign paths by construction. Tradeoffs: (a) requires heap telemetry (used/committed) inside the gate evaluation itself — the rig already collects it, but the gate's *binding* would now depend on parsed heap values, a bigger change than a constant; (b) changes verdict semantics ("unexplained RSS grew" vs "RSS grew") — every historical v2 verdict would need a mapping note if compared; (c) the +57 MB native term is box/config-specific and would need re-derivation on any environment change; (d) safest rollout = parallel-run period (v2 and v3 both evaluated on every run, both verdicts banked for N ≥ 5 runs before switching the binding).

**O4 — complement, not replace: scheduled full-GC after load drop.** The parked candidate (agent-side periodic full-GC after the load spike ends) would collapse the committed-lazy term at the source, shrinking the benign band itself instead of widening the gate around it. Tradeoffs: (a) changes runtime behavior under test — the plugin would be injecting GC activity it also ships to production, so the C3 channel would start measuring a *different* system; (b) needs its own pre-registration and its own A/B (does periodic full-GC hurt tail latency during churn?); (c) orthogonal to O1–O3 — could be adopted with any gate version, but should be a separate task, never a bundled change.

**O5 — two-tier verdict semantics (formalize what is already de-facto).** Split the current single `FAIL-leak-signature` into `FAIL-tripwire (attribution: benign class, invariant clean, decomposition closed)` vs `FAIL-leak-signature (invariant violated)`. This keeps every adverse reading adverse (sharper classification, not a lenient one) while making the TSV self-explanatory. Tradeoffs: touches the letter of the no-reclassification law (verdict strings), so it must be grandfathered: existing §83/§86 verdicts stay verbatim; the two-tier scheme applies only to runs after an owner-approved pre-registration.

## 4. Constraints on any revision (regardless of option chosen)

1. Only via the TASK-129→v2-style pre-registration flow: written protocol, drift-guarded rig commit, threshold/semantics frozen *before* the first run it judges.
2. Parallel-run validation (both gate versions evaluated and banked per run) before any binding switch — O3 in particular.
3. The no-reclassification law survives verbatim: historical verdicts are never edited, including by this document.
4. The leak invariant (post-GC used ≤ own baseline) remains the authoritative discriminator under any gate version; a v3 that fires without an invariant violation must still route through the attribution protocol.
5. Early-call rule (R1 + R0 factors) remains a mid-run *prediction* instrument only — it never gates or mutes the tripwire.

## 5. Evidence pointers

- TASK-139 §83 ADDENDUM-73 — first fire, no-GC path, attribution protocol exercised end-to-end (`results/TASK139_C3_CADENCE6_V2MISS_2026-09-09.md` + RAW_20260909_102444).
- TASK-140 §84 ADDENDUM-74 — corpus model, residue decomposition, quantitative pre-registration (`results/TASK140_C3_CORPUS_GCTIMING_2026-09-09.md`, script `/home/z/my-project/scripts/task140_corpus.py`).
- TASK-141 §86 ADDENDUM-75 — second fire, tight-baseline path, anti-gate-shopping station, refined early-call rule (`results/TASK141_C3_CADENCE7_TIGHTBASE_2026-09-09.md` + RAW_20260909_110111).
- Canonical rig: `bench/graal_ab/run_task129_pure_inject.sh` (drift-guard sha256 `8ba2473ce1f4453c…` unchanged through this task).

## 6. Banking

- Ledger ADDENDUM (numbered after pull + tail grep; tail was §88 = twin S7-89 ATTEMPT-26; their S7-90 may take the next number first — keep-both/first-lander-keeps-number applies).
- INDEX row added. Zero boots, zero runs, no flock contention, INJECTS-ONLY. The v2 gate bytes are untouched by this task (rig drift-guard verified before and after).

---

## ADDENDUM (agent-7625532f, 20:2x+08 — post-cadence-#9 evidence update; the options above are UNCHANGED, this strengthens the evidence base only)

Cadence #9 (TASK-145, ledger §92) fired the tripwire a THIRD time (R2=978 vs gate 974.0, +4 MB) with **both registered early-call factors negative** (free-room@R0 67.0 MB > ~60; Δused@add −7.7) — the binary R0-factor threshold (one-point-derived from T141's 56.5) is falsified as a sufficient predictor; the tight-band boundary is probabilistic somewhere in 67–98 MB. Corpus now: **3/9 fires, ALL benign-attributed** (no-GC ×1, tight-baseline ×2); resid model **6/6 closed including err 0.0 on this fire**; invariant 9/9. Consequences for the options: (a) the unexplained-residue reading was **0.0 MB on a fire** → an **O3** structure-aware gate would have fired **0/9** on the entire corpus while remaining the true dangerous-class detector — O3's evidence base is materially stronger after #9; (b) O1 keep-v2 costs rise (fires now 3/9, each consuming a diagnostic leg); (c) O2's calibration-by-anecdote concern deepens (the benign band now reaches +126 at free-room 67, and the binary-factor rule can't predict it). Baseline health separately proven in #9: R0 variance is committed-side (R²=0.984, slope 0.97, constant non-heap 357.8 MB), used@R0 trend rho=0.217 ns, post-full-GC live set 250.0/249.4/248.5 MB across ~3.5 h — no baseline leak signature. As before: nothing is retuned; the decision remains the owner's.
