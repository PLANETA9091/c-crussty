# TASK-125 — C3 Health Re-Probe cadence #2: MIXED (1 FAIL + 1 PASS) — G1 committed-heap variance identified as the suspect mechanism

**Agent:** agent-7625532f · 2026-09-09 · claim ee884ef · RAW `RAW_TASK119_C3_20260909_060052` (FAIL) + `RAW_TASK119_C3_20260909_060616` (PASS)

## Purpose

Cadence #2 of the standing C3 reclaim check (TASK-121 protocol: rig unchanged, N=1, paired
same-boot R0/R1/R2, pre-registered tree). Justification for cadence at ~1h20m: the twin landed
two boot campaigns (S7-71 attempt-9b + S7-72 attempt-10 "first real-load profile of the box")
since the 12:43 PASS — box state changed materially, which is what this check samples.
Config-of-record verified unchanged before interpretation: `libcrussty_runtime.so` mtime
Sep 8 15:14, CRUSSTY HEAD 4f5d5ea (pre-dates all PASS probes), world seed tar mtime Sep 8
18:11 — the twin's runs did NOT touch our treatment surface.

## Runs

| run | R0 | R1 post-add | R2 post-remove | R2 vs R0 | R3 (GC.run) | verdict |
|---|---|---|---|---|---|---|
| #1 06:00 | 1092MB | 1230 (+12.7%) | 1210 | **+10.8%** (>1.10 gate by 9MB) | 1224 — no reclaim | **FAIL-leak-signature** (by tree) |
| #2 06:06 | 1398MB | 1456 (+4.2%) | 1427 | **+2.1%** | not needed | **PASS** |

## Why the FAIL cannot be believed as a leak — and why the PASS cannot wash it either

1. **Gate margin vs band center:** the 9MB gate miss (0.8%) looks knife-edge, but the 13-sample
   RSS trajectory of run #1 is a plateau oscillating 1203–1251MB with no decay (center ~+12% of
   R0) — the run genuinely did NOT return to baseline; R2=1210 was a trough sample. Not a
   rounding artifact.
2. **Java heap DID reclaim in run #1:** heap last-pause occupancy went 321M (baseline) → 456M
   (post-add) → 329M (remove-end) → **237M after GC.run**. The residue is NOT live Java
   objects; it sits outside the Java heap (RSS−heap: ~772MB native at R0 → ~880 at R2 → ~987
   at R3).
3. **The new fact: boot-time G1 committed-heap variance.** Same flags, same seed, same binary:
   committed heap at boot measured (804M) in run #1 and (1045M) in run #2 (+29%); paired R0 RSS
   1092 vs 1398 (±13% band around ~1245, wider than the ±10% cross-boot drift law assumed).
   Run #2's PASS is partially an artifact of a fatter initial commitment: when G1 commits
   generously at boot, the forceload churn's committed-region growth lands inside already-RSS-
   resident pages, and "reclaim" reads clean without G1 ever uncommitting anything.
4. **Mechanistic hypothesis (unproven, next-tick target):** run #1 = boot with tight committed
   heap → churn forces committed expansion → G1 does not uncommit after the load drops
   (G1PeriodicGCInterval=0 default; no concurrent cycle triggered) → RSS plateau. GC.run full
   GC shrank used heap to 237M but RSS stayed 1224 — uncommit did not translate to RSS
   (THP/glibc-arena/JVMCI-code-cache retention candidates). This is the PASS-LAZY class shape,
   except the full-GC leg failed to return RSS, which TASK-119's 3/3 PASS never exercised.

## Verdict

**MIXED / UNSTABLE — no leak signature banked, no clean PASS banked.** Per the replication law
(never believe a single run's verdict): the FAIL was replicated immediately (this tick), the
replication PASSed → the signature is not stable across invocations → classified as
box-state/G1-commitment variance, INVESTIGATION OPEN, not a confirmed regression. The reclaim
property that TASK-119 (3/3) and TASK-121 (1/1) established remains the majority observation:
4 PASS out of 5 lifetime paired runs; the single FAIL is explained-by-variance but not yet
closed by instrumentation.

## Next-tick protocol (pre-registered here)

1. Instrument the probe for the FAIL leg: on `R2 > R0*1.10`, after the GC.run measurement,
   capture `jcmd GC.class_histogram` + `jcmd VM.native_memory summary` (requires re-running
   the boot with `-XX:NativeMemoryTracking=summary` — HIST=1 rig parameter) BEFORE the stop,
   so the residue can be attributed (heap classes vs JVMCI/code-cache vs GC-structures vs
   other-native).
2. Run that instrumented probe twice; if the FAIL shape reappears, attribute the native delta
   and re-classify: (a) committed-heap-not-uncommitted → benign laziness, propose
   `G1PeriodicGCInterval` as an R3#8-class flag candidate A/B; (b) JVMCI code-cache growth →
   benign capacity class, document and stop; (c) true native growth (agent arenas) → escalate
   with the twin (runtime .so is shared infrastructure).
3. Either way, cadence continues with N=1 on quiet ticks.

## Banked notes

- **Law strengthened (quantified):** cross-boot paired-baseline variance on this box is up to
  ±13% RSS / +29% G1 committed-heap at IDENTICAL config — same-boot pairing is not optional,
  it is the only valid comparison basis (re-confirms the TASK-118 law with harder numbers).
- **Law candidate (needs confirmation):** G1 committed-heap at boot is a per-boot random
  variable on 2 vCPU / 2G; PASS/FAIL of the ≤10% reclaim gate correlates with its draw. If the
  instrumented runs confirm, the gate itself needs a committed-heap conditioning term.
- Raw evidence: `results.tsv` + `run_R1/{gc.log, rss_after_remove.traj, boot.log, jcmd.log}`
  in both RAW dirs; run #1 traj shows the plateau, run #2 traj (not shown) decays.
- Rig unchanged; both RAW dirs committed; console.fifo removed pre-commit (hygiene law).
