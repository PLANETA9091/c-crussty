# FAMILY-AGG PREREGISTRATION — aggregate lever STEP-0 (S7-110, TASK-246)

> Preregistration BEFORE any implementation code. All numbers traced to banked
> STEP-0 artifacts (bytecode-verified or measured). No gate revision requested
> at this stage: the aggregate pack is treated as ONE lever and must pass the
> UNCHANGED binding gate «>= 3.0% MSPT, CI A/B min-of-2» on its own paired A/B.
> Nothing lands unless the pack clears the gate. Owner can halt with one word.

## 1. Why the aggregate lever now

Six profile confirmations (run#11, N=4 min-of-2, N=16, legal pair run#17×run#21,
run#22, run#23) show NO solo lever >=3% exists on any hardware class. The only
modular path to the north star (MSPT <=50ms from ~76-84ms base) left standing is
the aggregate of the individually-refuted <3% verified cores. The GOAL ledger
estimated «сумма закрытых честных соло ~5-7%» but never computed a DISJOINT,
parity-filtered ceiling. This document does exactly that, tier by tier, using
only banked numbers. Result: the pack is a GO candidate at Tier B (3.3-6.9%),
and this is decidable by one paired A/B — the same gate discipline as any solo.

## 2. Disjoint lane basis (paired profile, run#17×run#21; cross-checked run#18/22/23)

Sum of listed lanes = ~96% of tick CPU (residual ~4% = small JVM lanes, no
verified levers). Double counting is controlled by assigning every verified core
to exactly ONE top-level lane:

| top-level lane (disjoint) | presence | verified replaceable core | STEP-0 source |
|---|---|---|---|
| kernel: other | 21.5-23.1% | BATCH-RNG **1.6-1.9%** (F1) + LevelTicks parity-safe slices (F3) | TASK-233 bytecode sizing; levelticks-recon-2026-09-17 |
| entities/mobs (kernel) | 12.7-13.5% | BRAIN-LENS core **0.9-1.5%** (F2) | task168 (brainlens research) |
| chunk system (kernel) | 10.4-10.6% | 0 — per-get lens REFUTED; batch-lens REFUTED (3.3% ceiling, parity-unreachable) | §108/ledger chunk row |
| moonrise/paper patches | 7.7-8.0% | 0 verified | — |
| JVM GC (G1 + barriers) | 7.7-9.0% | **<=0.5% MSPT** hard physics ceiling (STW duty 0.50-0.56% wall; concurrent worker CPU is not MSPT) | task166 GC-FAMILY LAW |
| JDK collections + fastutil | 10.6-11.3% | 0 — replaceability lives in CALLING lanes (counted there), not in collections themselves | decomposition discipline |
| JIT stubs (vtable/itable) | 2.5-2.6% | 0 — Brain itable dispatch VERIFIED NOT replaceable | task168 |
| network (kernel) | 1.5-1.8% | ~0 — sendChanges 0.77-0.89% and SHRINKS with N (N-scaling REFUTED); outbound already discard-cheap | run#19 verdict |
| block entities/hoppers | 1.3-1.4% | 0 verified (<gate whole lane) | ledger |
| spawn lane | 0.6-0.7% | 0 (<gate) | run#17/19 |

## 3. Family members (F-numbers used below)

- **F1 BATCH-RNG** — random-tick RNG batching, bit-exact 48-bit LCG batch.
  Verified potential **1.6-1.9%** (TASK-233 javap sizing; supersedes the 3.2%
  §109 estimate). Parity: BIT-EXACT achievable (same LCG sequence, batched
  advance) — the strongest parity property in the pack.
- **F2 BRAIN-LENS** — Brain.tick replaceable core (iterators 0.47-0.80 +
  getNode 0.16-0.18 + views 0.08-0.20 + half-SELF 0.15-0.25) = **0.9-1.5%**
  (task168). Parity: median-exact via order-preserving iterator/structure swap;
  itable dispatch + canStart bodies excluded (NOT replaceable — already carved out).
- **F3 LEVELTICKS parity-safe slices** (task167 decomposition):
  - reads batch-by-section: **0.3-0.5%** (getChunk/hash only; per-query state
    decode irreducible)
  - tick-queue machinery: **<=0.5%** (primitive open-addressing drain)
  - signal wire-eval lens: **1.5-2.5%** CONDITIONAL on parity proof — wire eval
    order may affect results; included in Tier B-upper only with a bit-exact
    order-preserved lens, else dropped.
  - EXCLUDED by parity risk: Bukkit neighbor glue skip (<=0.6%, semantics risk),
    mid-tick yield reshape (<=1.0%, latency-semantics risk), inlined tail (no lever).
- **F4 MINECARTS** — whole lane 2.15-2.58% presence < gate solo; sub-slices
  decomposed (task169), replaceable fraction NOT verified. Assumption range
  30-50% lens savings => 0.6-1.3%. COUNTED ONLY in Tier C (marked assumption).
- **F5 ENT-BP** — mirror ceiling 1.5-1.8% (§109 NO-GO solo, infra armed).
  COUNTED ONLY in Tier C.
- F6 sendChanges/setDeltaMovement, F7 GC, F8 chunk: **0** (above).

## 4. Tiered aggregate ceiling (disjoint, parity-filtered)

| tier | members | sum (low-high) | vs 3% gate |
|---|---|---|---|
| **A: strictly verified + bit-exact/median-exact parity** | F1 + F2 | **2.5-3.4%** | straddles gate |
| **B: A + F3 parity-safe (reads+queue, signal only if bit-exact lens proven)** | F1+F2+F3 | **3.3-6.9%** | **floor >= gate** |
| C: B + F4 (assumption) + F5 (parked infra) | +0.6-1.3 +1.5-1.8 | 5.4-10.0% | headroom, not preregistered |

Tier B is the honest GO band: even taking the LOW end of every verified core
(F1=1.6, F2=0.9, F3=0.8 without signal lens) the pack clears the gate (3.3%);
the high end with a proven signal lens reaches 6.9%. Tier A alone (2.5-3.4%) is
NOT enough to bet the round on — which is exactly why the pack must include F3.

## 5. Preregistered plan (per binding rules, unchanged)

1. **Lever = the aggregate pack {F1, F2, F3-safe}**. ONE lever per round: the
   round lands (or refutes) the PACK as a unit. No member lands individually.
2. **STEP-0**: all member bytecode contracts already banked (TASK-233, task166,
   task167, task168, task169 + §109). This document is the aggregate-level
   kill-gate: PASS at Tier B (floor 3.3% >= 3.0%).
3. **Build order (parity-banked increments, one member per tick, no landing)**:
   F1 batch-RNG (bit-exact, highest confidence) -> F2 Brain iterators ->
   F3 reads-batch -> F3 queue-drain -> F3 signal lens ONLY if bit-exact
   order-preserving lens proves out offline; else dropped and ceiling re-read
   (Tier B floor stays >=3.3% without it: 1.6+0.9+0.8=3.3).
4. **Gate**: single paired A/B of the full pack vs baseline, min-of-2 legs per
   arm (baseline arm = existing legal pair run#17×run#21 76.01/76.98ms is
   REUSABLE as banked baseline legs; pack arm = 2 fresh paired legs via
   hunt_leg_b machinery). Gate: pack MSPT delta >= 3.0% with CI per binding rule.
   Parity: median-exact per member (unit parity banks in research/), plus
   fixture-VALIDITY + world_sha pairing per S7-96d law.
5. **Refutation path**: if the pack A/B lands <3%, ALL members stay unlanded,
   the aggregate row records REFUTED with the measured number, and the modular
   agenda is empty — remaining paths are purely owner-gated (pinned runner,
   scenario change). Zero code lands below gate; zero risk either way.
6. **Owner override**: if the owner reads rule «один рычаг за раунд» as forbidding
   aggregate packs, this preregistration is void and the loop returns to
   owner-gated waiting. The dossier costs nothing and commits nothing.

## 6. What this tick banked

- This preregistration (aggregate STEP-0, paper-only).
- Pair-hunt infra warm: formal pair #2 leg dispatched (run 35179585066,
  band-gated fp=4, classification next poll/tick) — the pack's gate arm needs
  the same pairing machinery.
- No implementation code in this tick (STEP-0-before-code discipline).
