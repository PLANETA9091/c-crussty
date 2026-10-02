# TASK-134 — C3 HEALTH CADENCE #4 — FIRST IN-VIVO RUN UNDER FLIPPED RIG (gate v2 binding at source)

- Agent: agent-7625532f · 2026-09-09 16:4x+08 · claim b5767b1
- Rig: `bench/graal_ab/run_task129_pure_inject.sh` (canonical pure-inject, TASK-129 skeleton; TASK-133 flip live)
- RAW: `bench/graal_ab/RAW_TASK129_PURE_20260909_084456/` (results.tsv, run.log, run_R1/{boot.log,hist_R2.txt,rss_after_remove.traj}; fifo stub removed at banking)
- Config of record verified pre-run: libcrussty_runtime.so mtime Sep 8 15:14, CRUSSTY 4f5d5ea, purpur-1.21.10.jar Sep 8 15:06. Stock Temurin 21.0.12.1 (`/home/z/jdk21`) + ONLY `-agentpath` — zero JVM options (INJECTS-ONLY law).

## Why this run

Twin landed attempt-19 boot campaign (S7-81, 4ac6aa9/97ee7c4) since the last probe (TASK-132, 16:0x) — box state changed per cadence policy. Additionally the TASK-133 rig-gate flip (f1f86bf) had never executed on a real run: smoke was synthetic-only. This run is the first in-vivo execution of the v2-binding verdict tree, and it exercises the **lean-baseline zone** (R0=900 < crossover 1000) where the v2 absolute floor binds rather than the 1.10× ratio.

## Result: PASS ×both gates (v2 binding at source)

| metric | value | note |
|---|---|---|
| boot wall | 17.575s | pure-inject band 4th: 16.27/15.96/16.64/17.58 — parity holds, no OOM/FATAL |
| R0 | 900MB | used 356864K / committed 572416K |
| R1 | 954MB | +54 after add-load |
| R2 | 961MB | residue **+61MB** |
| v2 gate | 1000.0MB | `max(990, 1000)` — **floor binds** (R0=900 lean zone) → PASS |
| v1 class | within-v1 | v1 thresh 990 → PASS, dual-report consistent |
| v1_final | NA | GC.run not needed (R2 below both gates — de-flag (v1,v2] band correctly did not fire) |
| hs_err delta | 0 | |

TSV +3 columns (v2_gate/v1_class/v1_final) populated correctly on a real run — TASK-133 flip verified in-vivo.

## Mandatory commentary (TASK-130 §5)

- **Trajectory: PLATEAU.** 13 samples/120s: 954→956→957→960→…→961, monotone saturating, deltas decay to 0 — no creep.
- **Used-heap discriminator: flat.** 356864K→365431K (R1)→373045K (R2 end) = +4.5% total; **committed heap CONSTANT 572416K at all three points** (zero committed growth — tighter than TASK-132's +2.1M).
- **Residue = native structural class.** +61MB — bit-identical to TASK-132's +61MB; agrees with TASK-126 NMT attribution (~57MB C2 code-cache + metaspace) within 4MB. No agent-arena signature, no twin escalation.
- hist_R2.txt: normal server population (AABB/VoxelShape/moonrise collision caches) — world loaded, inject functional.

## Verdict

Channel healthy: 4th pure-inject boot in parity band, residue in the attributed structural class for the second consecutive probe, plateau shape, used-heap flat. TASK-130 §5 binding expectation (PASS ≤+100MB) met. Flipped rig validated end-to-end on a real run; next cadence proceeds under the same tree.

Boot-lane note: twin S7-81 landed (a19 retry lever proven, LAW P6B-21); their a20 (latest.log close→ignore serving attempt) was unclaimed at probe start — BENCH-MUTEX serialized, no contention observed.
