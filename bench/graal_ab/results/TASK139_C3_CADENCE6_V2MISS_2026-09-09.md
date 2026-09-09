# TASK-139 — C3 CADENCE #6: FIRST v2-GATE TRIPWIRE FIRE — attribution resolves to benign-lazy variant (garbage-detection latency at lean baseline)

**Agent:** agent-7625532f · 2026-09-09 · boot-lane quiet window (twin S7-85 done, S7-86 not yet claimed at claim time)
**Rig:** `bench/graal_ab/run_task129_pure_inject.sh` — drift-guard verified UNCHANGED this tick (sha256 `8ba2473c…` baseline from TASK-137 holds).
**Config:** canonical pure-inject — stock Temurin 21.0.12.1 + ONLY `-agentpath:libcrussty_runtime.so` (INJECTS-ONLY owner law; zero other JVM options). External observers only (`jcmd GC.heap_info` / `GC.class_histogram` / `/proc smaps_rollup`).
**RAW:** `bench/graal_ab/RAW_TASK129_PURE_20260909_102444/` (fifo stub removed at banking).
**Claim:** dev-logs 4ae7fe8 (TASK-139).

## 1. Headline numbers (N=1)

| leg | RSS (MB) | heap used / committed |
|---|---|---|
| R0 baseline (idle, settled) | **878** | 313,138 K / 546,816 K |
| R1 post-add (64-chunk band) | 923 | 496,112 K / 547,840 K |
| R2 post-remove + 120 s settle | **1003** | 504,174 K / 619,520 K |
| R3 after diagnostic `GC.run` (+15 s) | **1008** | **254,304 K** / 619,520 K |

- Boot **16.307 s** — 6th consecutive pure-inject boot inside the parity band (16.27 / 15.96 / 16.64 / 17.58 / 16.18 / **16.31**). crussty-runtime v2.0.0 init rc=0 marker present.
- v2 gate = max(878 × 1.10, 878 + 100) = **978.0 MB**. R2 = 1003 → tripwire fired → `GC.run` attribution step executed by the rig (per pre-registered tree, TASK-133 flip).
- Tree verdict: **FAIL-leak-signature** (R3 = 1008 still over gate). v1_class = exceeds-v1, v1_final = FAIL. **Kept verbatim — §4 law forbids reclassification at banking time.**

## 2. Why this is NOT a live leak — the authoritative discriminator (C3_GATE_V2_PROTOCOL §4)

The protocol is explicit: *"The used-heap-below-baseline discriminator remains the authoritative benign/leak signal; the RSS gate is the tripwire, not the verdict."*

1. **Post-attribution live set is BELOW baseline:** heap@final used = 254,304 K vs baseline 313,138 K → **−57 MB**. A live leak cannot subtract from the live set. `GC.class_histogram` at R2 agrees: top live objects are vanilla/moonrise collision caches (AABB 22 MB, ArrayVoxelShape 15 MB, CachedShapeData 8 MB …), total live ≈ 254 MB — nothing above baseline classes, no agent/package-resident growth.
2. **R3 RSS closes arithmetically with no unexplained residue:** R0 878 + Δcommitted **+71 MB** (546,816 → 619,520 K, G1 non-uncommit) + structural native **~57 MB** (TASK-126 NMT class: C2 code-cache + metaspace from the churn JIT) = **1006 ≈ 1008 measured (±2 MB)**. No agent-arena residual term is needed — the §5 "twin escalation on agent-arena signature" leg is **not triggered** (nothing unexplained to escalate).
3. **Trajectory shape (mandatory on non-PASS):** 923 → 967 → 989 → 997 → **1001 → 1001 → 1001 → 1003 → 1003 → 1003 → 1003** — rise saturates by ~60 s, then **flat for the final ~70 s (+2 MB noise)**. That is a **plateau**, the benign-lazy shape; a live leak under this protocol reads as creep (linear growth through the window).

## 3. New mechanism variant — why the R2-leg used-heap was above baseline for the first time

All five prior cadence runs had the remove_end used-heap **at or below** baseline (T135: 390 vs 404 MB). This run: remove_end used = **504 MB (+191 MB above baseline)** — the discriminator sample at R2 caught **uncollected garbage**, not live objects. Mechanism hypothesis (pre-registered for cadence #7):

- R0 **used** = 313 MB is the lowest of the entire series (T135 ≈ 404 MB) — per-boot G1 sizing variance (the ±13/+29 % committed variance already banked as the same-boot pairing law).
- The leaner the occupied baseline, the more free heap / eden room → during the 120 s idle settle the allocation rate (~0 on an idle server) never triggers a young GC → the ~190 MB of churn garbage simply is not collected yet when R2 samples RSS and used.
- In T135 the fatter baseline used (404 MB) left less free room → young GC(s) fired inside the settle window → used dropped below baseline by R2.
- The diagnostic `GC.run` confirms in one step: 504 → 254 MB. The "excess" over every prior benign reading was **collection latency, not retention**.

Under §5 the tripwire correctly fired (R2−R0 = +125 MB exceeds the 100 MB floor = every previously attributed benign class at R2-sampling granularity); the attribution legs (used-heap → GC.run → arena check) then resolve the signature to a **benign committed-lazy + garbage-latency variant**. The tree did its job: it flagged a reading outside all prior classes and forced the attribution step that explains it.

## 4. Verdict honesty statement

- Tree output **FAIL-leak-signature** stands in the TSV unmodified (no reclassification law).
- The leak hypothesis is **refuted** by the authoritative discriminator (post-GC live set −57 MB below baseline) plus closed RSS arithmetic (±2 MB) plus plateau trajectory. The elevated R3 RSS is committed-side non-uncommit + structural native — the same benign committed classes attributed by TASK-126 NMT, previously invisible to the gate because prior boots GC'd during settle.
- Per §5 escalation ladder this resolves WITHOUT twin escalation: no agent-arena signature, no live growth, no unexplained residue.

## 5. Pre-registration for cadence #7 (replication leg)

1. **Primary watch:** does the v2 tripwire recur? Discriminating prediction: recurrence (if any) correlates with LOW R0-used boots (≈300–350 MB), and in every recurrence post-GC.run used lands BELOW baseline. If any run shows post-GC.run used > baseline → genuine live-leak signature → full escalation (replication ×3 + twin coordination per §5).
2. Committed plateau watch: Δcommitted(R0→R3) expected ≤ ~80 MB (today +71); a monotone committed growth across cadences on the SAME boot class would be a new signal.
3. Boot-parity band watch: 6th boot in band; band edges now 15.96–17.58 s.
4. Trajectory shape reported on every run regardless of verdict (§5 vigilance clause).

## 6. Banking notes

- Ledger: §83 ADDENDUM-73 (numbered after pull + tail grep; tail was §82 after collision-#18 renumber of twin S7-86 — see §81/§82 numbering notes).
- INDEX row added at bank time. RAW fifo stub removed (banking law).
- Collisions this tick: #17 (twin S7-85 dup §80 → renumbered §81, body verbatim) and #18 (twin S7-86 dup §81 → renumbered §82, body verbatim) — both duplicate-number class, both resolved first-lander-keeps-number, zero content loss.
- 1 boot total, hs_err delta 0, zero config, INJECTS-ONLY.
