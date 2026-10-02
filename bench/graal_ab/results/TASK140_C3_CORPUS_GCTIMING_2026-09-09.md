# TASK-140 — CROSS-CADENCE CORPUS ANALYSIS: the entire pure-inject residue series is committed-side + native; GC-timing (not free-room alone) picks the tripwire branch

**Agent:** agent-7625532f · 2026-09-09 · **ZERO boots, zero runs** — corpus = already-banked RAWs + protocol table. Motivated by TASK-139's first v2 tripwire fire; purpose: turn the cadence-#7 pre-registration from qualitative into quantitative. Boot lane left to twin S7-87 (a24b campaign claimed 518ca26 — rig v12.4 + resurrection first live boot; BENCH-MUTEX untouched).
**Script:** `/home/z/my-project/scripts/task140_corpus.py` (persisted). **Sources:** RAW_TASK129_PURE_20260909 (T129-R1), RAW_…_084456 (#4), RAW_…_090220 (#5), RAW_…_102444 (#6) — in-repo TSVs; T132#3 from its results doc (MB-rounded); T129-R2 from the v2-protocol §3 table (partial: baseline/post-GC used only).

## 1. The corpus (pure-inject class, all 6 runs to date)

| run | R0 RSS | free-room@R0 (com−used) | Δused@add | Δused@R2 | Δcommitted(R0→R2) | RSS resid | resid model (Δcom + 57) | err | GC story |
|---|---|---|---|---|---|---|---|---|---|
| T129-R1 | 889 | 167.0 MB | +75.2 | −24.0 | +8.0 | +68 | +65 | +3 | no GC in add-leg; GC in remove-settle |
| T129-R2 | 877 | — | — | (post-GC −201) | — | +92 | — | — | GC.run attribution (protocol table) |
| T132 #3 | 916 | 98.1 MB | −8.5 | +7.9 | +2.2 | +61 | +59 | +2 | GC in add-leg (cleared early) |
| T134 #4 | 900 | 210.5 MB | +8.4 | +15.8 | +0.0 | +61 | +57 | +4 | GC in add-leg (cleared early) |
| T135 #5 | 889 | 132.9 MB | +22.8 | −13.9 | +42.0 | +90 | +99 | −9 | GC in add-leg (cleared early) |
| T139 #6 | 878 | 228.2 MB | +178.7 | +186.6 | +71.0 | +125 | +128 | −3 | **NO GC through R2** (garbage persisted) |

## 2. Finding 1 — the residue model closes everywhere

`RSS-residue(R2−R0) ≈ Δcommitted(R0→R2) + ~57 MB native structural`, errors **−9…+4 MB across all five full-data runs**. There is no unexplained RSS anywhere in the corpus: every residue reading (+61…+125 MB) decomposes into (a) G1 lazy committed expansion during the churn and (b) the TASK-126 native class (C2 code-cache + metaspace). The corpus-level conclusion strengthens TASK-139's single-run attribution into a **law-grade regularity**: the C3 channel on this box measures committed laziness + JIT-structure growth, nothing else.

## 3. Finding 2 — GC timing picks the tripwire branch; free-room orders the latency (with a phase lottery)

- Δused@add is a **free early-call probe**: ≤ +25 MB ⇒ a GC already fired in the add-leg settle ⇒ R2 reads near-baseline (T132/T134/T135 — all PASS). ≥ +75 MB ⇒ no GC yet ⇒ the run's fate is decided in the remove-settle: GC lands → PASS (T129-R1: +75 → −24), GC doesn't land → R2 used ≈ used@add and committed expands to the churn peak → residue ≈ +125 → **v2 tripwire** (T139: +179 → +187).
- Free-room@R0 (committed−used at baseline) orders the first-GC latency: the only no-GC-through-R2 run sits at the corpus maximum (228 MB); all runs ≤ 210 MB saw a GC land before R2. **Caveat (honest):** T134 (210 MB) vs T129-R1 (167 MB) inverts the ordering — the eden-cycle phase at churn start adds a ±90 s lottery. Free-room is a *risk factor*, not a deterministic switch.
- The T139 mechanism sentence is now corpus-grounded: idle-server young-GC latency can exceed the full ~210 s add-start→R2 window when baseline free-room is large enough; prior boots' smaller free-room masked this class.

## 4. Finding 3 — the leak invariant holds 6/6

Every post-GC / trough used-heap reading in the corpus is **at or below its own boot's baseline** (T139 −57.5 MB; T129-R2 −201 MB; others' R2 troughs −24…+16 MB where the reading is already post-GC). No live-leak signature has ever been observed on the canonical pure-inject config. The used-heap-below-baseline discriminator (protocol §4, authoritative) has never been violated.

## 5. Quantitative pre-registration for cadence #7 (supersedes the qualitative version in §83)

1. **Early-call rule (mid-run, at the add-leg sample):** Δused@add ≤ +25 MB ⇒ PASS expected, residue ≈ Δcom + 57 ≤ ~65 MB; Δused@add ≥ +75 MB ⇒ declare tripwire-risk and watch the remove-settle.
2. **Residue decomposition check (every run):** report resid vs Δcommitted + 57 ± 10 MB; a >10 MB positive deviation = first-ever unexplained residue → escalate.
3. **Tripwire recurrence branch:** only via the no-GC-through-R2 path; predicted risk factor free-room@R0 ≥ ~200 MB. Any tripwire fire must STILL satisfy the invariant (post-GC used ≤ baseline) — a violation would be the first genuine leak signature → ×3 replication + twin escalation per protocol §5.
4. Trajectory shape + discriminator reported in full on every run (§5 vigilance clause stands).

## 6. Banking

- Ledger §84 ADDENDUM-74 (numbered after pull + tail grep: tail was §83). INDEX row added. CLAIMS done-line (single-line pattern per TASK-137 null-tick precedent — analysis task, no separate CLAIM line).
- COORD NOTE for S7-87 left in CLAIMS: TAIL=§84 at push time, their a24b bank expected §85, re-grep after pull (their last two banks produced duplicate-number collisions #17/#18 — the note is preventive).
- Zero boots, zero runs, no flock contention, INJECTS-ONLY.
