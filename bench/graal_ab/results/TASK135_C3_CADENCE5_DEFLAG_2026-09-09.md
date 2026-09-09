# TASK-135 — C3 HEALTH CADENCE #5 — SECOND IN-VIVO RUN UNDER FLIPPED RIG; FIRST (v1,v2] DE-FLAG BAND CAPTURE LIVE

- Agent: agent-7625532f · 2026-09-09 17:0x+08 · claim 4e24eea
- Rig: `bench/graal_ab/run_task129_pure_inject.sh` (canonical pure-inject, gate v2 binding at source per TASK-133 flip)
- RAW: `bench/graal_ab/RAW_TASK129_PURE_20260909_090220/` (results.tsv, run.log, run_R1/{boot.log,hist_R2.txt,rss_after_remove.traj}; fifo stub removed at banking)
- Config of record verified pre-run: libcrussty_runtime.so Sep 8 15:14, CRUSSTY 4f5d5ea, purpur-1.21.10.jar Sep 8 15:06. Stock Temurin 21.0.12.1 + ONLY `-agentpath` — zero JVM options (INJECTS-ONLY).

## Why this run

First probe after a REAL CRaC image campaign (twin a20/S7-82: checkpoint + double-restore of a real 510MB image + rebind-try live on real restore) — the heaviest box-state change since the pure-inject channel opened. Also the second lean-zone data point (T134 exercised floor-binding at R0=900).

## Result: PASS under binding v2 — and the first live capture of the (v1,v2] de-flag band

| metric | value | note |
|---|---|---|
| boot wall | 16.184s | pure-inject band 5th: 16.27/15.96/16.64/17.58/16.18 — parity, no OOM/FATAL |
| R0 | 889MB | used 404M / committed 537M |
| R1 | 938MB | +49 after add-load |
| R2 | 979MB | residue **+90MB** |
| v2 gate | 989.0MB | floor binds (R0=889 < crossover 1000) → **PASS (binding)** |
| v1 class | **exceeds-v1** | 979 > 977.9 (=R0×1.10) — v1 would have FAILed and fired GC.run |
| v1_final | NA | GC.run correctly NOT fired — (v1,v2] band de-flagged by design (TASK-130 §3 / TASK-133 branch trigger) |
| hs_err delta | 0 | |

**This is the first real execution of the intended dual-report divergence:** v2 classifies +90MB benign (absolute floor), v1 classifies +10.1% as miss. The de-flagging of the (v1,v2] band — "measured benign band no longer triggers GC.run" (TASK-133) — worked mechanically as pre-registered. TSV columns captured it: v1_class=exceeds-v1 + verdict=PASS coexist honestly.

## Mandatory commentary (TASK-130 §5)

- **Trajectory: saturating plateau.** 13 samples/120s: 938→946→950→968→972→…→979 flat ×4 final samples — no creep; faster early rise than T134 (larger committed expansion under churn this boot) but clean saturation.
- **Used-heap discriminator (authoritative): CLEAN.** 404M → 427M (add) → **390M (remove_end = BELOW baseline −3.5%)** — zero live-object growth; churn returned fully.
- **Committed-side: +42MB by remove_end** (537M→557M→593M) — G1 lazy-uncommit expansion class (TASK-126 branch (a)); without GC.run not reclaimed this run. Predicted v1 path (had it bound): GC.run → R3 reclaim → PASS-LAZY ~+48MB — the same mechanism T126-R1 demonstrated.
- **Residue composition estimate:** ~42MB committed-lazy + ~48MB native structural = +90MB — INSIDE the TASK-130 benign band (50–90MB), at its TOP edge. Highest reading of the channel (T132/T134 both +61MB); not a v2 miss, no replication required by protocol; next probes watch for drift above the floor (v2-miss → leak protocol).
- **Baseline variance note:** R0 used-heap 404M vs T134's 348M (+16%) — per-boot variance law; same-boot pairing used; R0 RSS 889 vs 900 near-identical.
- hist_R2.txt: normal server population — world loaded, inject functional.

## Verdict

Channel healthy, verdict PASS under binding gate; dual-report mechanism validated live for the first time (divergence captured honestly, de-flag band silent as designed). Post-CRaC-campaign box state did not degrade the pure-inject baseline. Watch-item (non-alarming): residue at top of benign band — track the band over the next cadences before drawing trend conclusions.
