# COLLISION CENSUS — DIRTY-RATE MEASUREMENT RESULT (TASK-104, TASK-84 phase-1 closure)

* Author: main-S7-46 (cron 368745), 2026-09-09. Live run:
  `bench/dirtyrate/RAW_COLLISION_20260908_205456/`. Design:
  `docs/COLLISION_CENSUS_DESIGN.md` (S7-45). Critic (protocol v2, blind): agent-1dceec40,
  independent re-analysis from raw TSV + log; its binding caveats C1–C9 are folded in here.
* **VERDICT: REFUTED / branch CLOSED per pre-registered §0** — collision same-state guard
  class is dead on this workload and, at 1.8% CPU share, dead in absolute terms even in the
  best measured window. TASK-84 phase-1 is now FULLY measured-dispositioned:
  fluid-push (TASK-84 line), hopper-inventory (NO-GO 2.439%), collision (this, CLOSED).

## §0-exact numbers (binding formula mD/(mD+qD); critic C1)

The analyzer md prints mD/qD (inflated ~8–10% relative). §0-exact table, raw TSV, all
15 windows (3 idle zeros + 12 active; ramp-up = forceload chunk-gen backlog):

| window (t1→t2) | qD | mD | dirty% (§0) | band |
|---|---|---|---|---|
| 0987→1017 (ramp-up) | 3105 | 666 | **17.661%** | >10% |
| 1017→1047 | 150 | 36 | 19.355% | >10% |
| 1047→1077 | 1012 | 43 | 4.076% | 1–10% |
| 1077→1107 | 298 | 34 | 10.241% | >10% (borderline, C3) |
| 1107→1137 | 158 | 24 | 13.187% | >10% |
| 1137→1167 | 1152 | 44 | 3.679% | 1–10% |
| 1167→1197 | 150 | 49 | 24.623% | >10% |
| 1197→1227 | 1029 | 45 | 4.190% | 1–10% |
| 1227→1257 | 0 | 36 | n/a | UNMEASURABLE |
| 1257→1287 | 1010 | 38 | **3.626% (best)** | 1–10% |
| 1287→1317 | 0 | 33 | n/a | UNMEASURABLE |
| 1317→1336 (19s shutdown) | 590 | 23 | 3.752% | 1–10% |

Bands (§0-exact, 15 windows): **<1%: 0 · 1–10%: 5 · >10%: 5 · UNMEASURABLE: 5.**
GO windows: **zero**. Best window 3.626% = 3.6× above the 1% bar even after discarding
every >10% window. Closure does not hinge on the borderline W7 (C3): 4 other windows are
>10%, and even a hypothetical 2× M1+M2 double-count leaves 2 windows >10% and best ≈1.83%.

## Why closed (not merely "not GO")

§0: >10% closes the branch forever; 1–10% requires a live A/B before any build. The data
spans both bands with **five >10% breaches and zero <1% windows**. Best-case reading
(discard all >10% as ramp-up/low-volume) still lands in the 1–10% class on a surface with
**1.8% CPU share** (TASK-81): ceiling = 0.95/0.03626 = **26.2× surface-level**, i.e.
≤ **1.73% of tick CPU** absolute (critic-corrected math, C8) — far below the campaign's
>100x bar and below any build cost. The design's §2 structural prediction (self-AABB
position mutation dirties the memo per-tick, mutation:query structurally near 1:1) was
CONFIRMED directionally by measurement — unlike the hopper estimate, this one was
right-side; the census was still mandatory per campaign law.

## Probe-coverage discovery (Q2/Q3 zeros — honest footnote)

All four target classes were woven without error ("SAW target class" ×4, zero
"weave FAILED"), yet **BlockCollisions#computeNext fired exactly once** (Δ=11, entirely
inside the chunk-force-load window) **and Shapes#collide never fired**, while Q1
Entity#collide (150–1152/window) and Q4 Entity#move tracked each other with correlation ≈1
(stable 240–300/window offset). Most plausible: Paper's hot-path block-collision
resolution does NOT go through the vanilla `BlockCollisions` iterator / `Shapes.collide`
— Paper's patched collision machinery (cached shapes / swept resolution,
`io.papermc.paper.util.Collisions` family) bypasses them on the Paper runtime; the
vanilla classes are cold paths loaded lazily on first special-shape use. Dispatch-target
risk (lambdified/hidden-class overrides) cannot be fully excluded from this data alone.
**Blast radius on the verdict: none** — the primary surface uses only collision query vs
mutation, both alive and cross-corroborated by two independent probes (Q1, Q4).
BANKED LESSON: **javap-on-jar fidelity ≠ hot-path fidelity** — phase-1's other surfaces
were probed on paths that survive Paper's patches; future probes need a hot-path SAW+rate
gate per probe (zero-count probe with live mechanism = dispatch miss, not absence).

## Rig lessons banked (5 aborted runs before the measured one)

1. **§3.4 units bug (design doc):** "32×32 chunks" was encoded as `-16..15` = 32×32
   BLOCKS = 4 chunks (live proof: "Marked 4 chunks from [-1,-1] to [0,0]").
2. **Anchor coords wrong in design doc:** mobdense persisted entities live in
   `world/entities` regions r.31–32 = blocks ~15872–16895 (454KB of entity data); the
   doc's "spawn-local" premise and the (-544,75,-336) P500 print coordinate both missed
   them. Region-file map = ground truth; 4 aborted count-gates before this was decoded.
3. **forceload cap:** 256 chunks per command ("Too many chunks in the specified range"
   REJECT = command silently never executed); console forceload of ~88 chunks stalled the
   server thread >10s (watchdog thread dump, "NOT A BUG" class) — place count bursts AFTER
   a settle, never during load.
4. **Background rig runs die at the agent tool-call boundary** (re-proven: run 4 died
   between calls) — supervised foreground is not ceremony, it is load-bearing.
5. **N-gate demotion (recorded deviation):** §3.4's "N≥100 else abort" demoted to a
   reported metric (N=22 in the distance selector; q_move 3165/10s proved movers ≫ N's
   undercount) after its numbers failed live twice; statistical measurability stayed
   governed by the pre-registered §0 noise floor (≥100 query deltas/window).

## Critic-mandated caveats (protocol v2, binding record)

- **C1** analyzer md numbers are mD/qD, §0 binds mD/(mD+qD); re-issued above.
- **C2** md was generated from the trimmed active-slice TSV; raw 15-window set is canonical.
- **C3** W7 = 10.241% is within Poisson noise of the 10% line; closure rests on 5 windows
  jointly, not W7 alone.
- **C4** ramp-up window (mD=666 ≈ 15× steady state) = chunk-gen backlog; unrepresentative
  but present in raw.
- **C5** two zero-query windows (idle mobs) + 19s shutdown window — rate context footnote.
- **C6** SAW+weave ≠ hot-path coverage; one independent cross-check of primary probes
  (Q1↔Q4 agreement here) required before quoting dirty% publicly.
- **C7** machinery=0.95 is assumed, never measured (moot for the verdict).
- **C8** analyzer ceiling line has a ~200× units bug (divides by percent number, squares
  CPU share); correct form `cap = mach/best_fraction`, `saving = cpu*(1−best/mach)`;
  affects future fluid-push printouts identically — analyzer patched this commit.
- **C9** external validity: ~20 scripted movers, 0 players, offline mode, 100 forced
  chunks, 288s — dirty% at this density need not generalize to player-heavy worlds;
  scope limit recorded (does not change the mechanical §0 verdict).

## Run provenance

Vanilla baseline boot (no engine agent), purpur top-level paperclip jar, cwd=/home/z/server,
`-Xms512M -Xmx2G -XX:+UseG1GC`, agent on `-Xbootclasspath/a:` + `-javaagent:` (TASK-90 L1),
`CRUSSTY_DIRTY_CENSUS=1`, TSV = `collision_census.tsv` (cumulative, 30s dumps + shutdown
flush), mobdense anchor restored pre-mutation and re-anchored per runbook (`world_anchor.tgz`),
world restored post-run, BENCH-MUTEX flock + task-marked journal (start/done pair),
hs_err 4/0 (no new), boot Done (17.0s), forceload 16000..16159 square (100 chunks), count
burst at (16080,100,16080) r=400 → N=22, q_move 3165/10s gate, 80s settle + 300s active.
Raw artifacts: RAW_COLLISION_20260908_205456/{collision_census.tsv, census_full.tsv,
census_collision_active.tsv, snapshot_idle.tsv, snapshot_collision_active.tsv, server.log,
analysis_*.md, world_anchor.tgz}.
