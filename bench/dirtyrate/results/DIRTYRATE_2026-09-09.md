# TASK-90 phase-2 verdict — hopper-inventory dirty-rate census (P500-census, live)

Date: 2026-09-09 (cron ticks 01:00-02:20+08) · Agent: agent-7625532f
Deliverable of TASK-90 / TASK-84 §3 protocol. Evidence class: **MEASURED LIVE CENSUS** — vanilla
baseline boot (no engine agent; mutation:query ratio plugin-invariant), counter-agent (ASM 9.7
entry-probes: mutation=BlockEntity.setChanged, query=HopperBlockEntity.tryMoveItems+suckInItems,
push-tick=pushItemsTick spec-plus), 300s hopper-active window, unmodified TASK-84 analyzer.
Artifacts: `bench/dirtyrate/RAW_DIRTYRATE_20260908_175619/` (census TSV + idle/hopper-active slices
+ unmodified-analyzer outputs + server.log).

## 1. Rig and validity

12 entity-ticking hoppers (10-hopper east chain with stalling tail + 2-hopper ping-pong ring),
24+6 seeded items, forceload'd rig area, 80s idle baseline then 300s active window. Instrumentation
sanity gate passed pre-window (push-tick count=1920 after 10s = 12 hoppers × 16/s expected ramp).
Mechanism check: steady-state push-tick = 7,200/30s = 240/s = 12 hoppers × 20 game-ticks/s — exactly
the theoretical maximum; query 12,300/30s and mutation 300/30s are stable to the event across 10
windows. The census is internally valid.

## 2. Root-cause archaeology of the all-zero run chain (6 dead runs before a valid one)

* Wrong boot jar (versioned `1.21.10/purpur-1.21.10.jar` = raw craftbukkit Main → NCDFE joptsimple) —
  fixed to the top-level paperclip jar.
* Wrong server CWD (repo root → EULA wall + paperclip extraction into the repo) — fixed to
  `cd $SERVER` + absolute paths.
* Session-teardown kills: background runs die with the launching agent session (nohup+setsid do NOT
  survive) — census runs must live inside one supervised foreground call.
* **L1 (the all-zero cause): CNFE of the woven probe's StaticCounter under Paper's remapped loader →
  probes silently no-op'd → counters stayed 0 while hoppers ticked — fixed by
  `-Xbootclasspath/a:agent.jar`.**
* L2 (refuted hypothesis): forceload'd chunks DO entity-tick (smoke run proved nonzero counters under
  forceload alone) — the spawn-chunk START-ticket fix was never needed.
* Multi-instance reality: sibling sessions iterated the same script concurrently; BENCH-MUTEX
  serialized the lane; one of my smoke attempts was SIGKILLed by a sibling taking the lane — protocol
  held (no data loss, journal complete).

## 3. Measured result (steady state, 10×30s windows)

| metric | value | note |
|---|---|---|
| hopper-inventory query (tryMoveItems+suckInItems) | 12,300 / 30s = 410/s | spec-exact surfaces |
| hopper-inventory mutation (setChanged, ALL-BE upper bound) | 300 / 30s = 10/s | includes non-hopper BEs → true hopper share is lower |
| **dirty% (mutation / (mutation+query))** | **2.439% stable** (transit window 5.467%) | structural ratio — scales with transfer rate, not density |
| hopper-push-tick | 7,200 / 30s = 240/s | 12 × 20/s — mechanism-exact |

## 4. Verdict: NO-GO / DO-NOT-BUILD (13th closed x1000 branch)

Pre-registered §0 rule (TASK-90 claim, phase-2a): GO (design phase) requires **dirty% < 1% AND
machinery ≥ 90%**; > 10% closes the branch forever.

| gate | required | measured | result |
|---|---|---|---|
| dirty% | < 1% | **2.439%** | **FAILS the GO band by 2.4×** |
| close-forever band | > 10% | 2.439% | not hit — branch closes by gate failure, not by the heavy band |
| machinery ≥ 90% | unmeasured | — | moot: dirty% already failed |

Amdahl chain (why the GO band is unreachable in principle, not just unmet): a same-state guard on
`setChanged` eliminates at most the mutation share of the hopper-inventory surface's cost. Net
server-level ceiling = surface_CPU_share × 2.439%. To reach the campaign's >3%-of-tick GO gate the
hopper-inventory machinery alone would need to consume **>123% of the tick budget** — physically
impossible. The ratio is structural (both counters scale with transfer rate), so hopper density does
not move it. Even the pathological 12-hopper rig's full machinery is well under 1% of a tick
(~410 transfer attempts/s ≈ TASK-81-scale per-call costs put full machinery in the µs-per-second
range).

This closes the last open census item of the TASK-84 queue: **the Lithium-style hopper mod-count
guard has no prize on this stack** — same verdict shape as TASK-80 (fluid), TASK-89 (BE-tick),
TASK-92 (dfc), TASK-94 (TPS accounting).

## 5. Re-open criteria

* A real-server census (player item-sort infrastructure) shows dirty% > 10% — the pre-registered
  heavy band (would require the mutation share to be structurally 4× higher than measured).
* JFR under real hopper-dense load shows hopper-inventory machinery ≥ 3% of the tick budget
  (impossible per §4 arithmetic unless the engine's hopper code changes).
* Engine change to `tryMoveItems`/`setChanged` cost structure (version bump → re-census, cheap:
  the tooling is committed and self-tested).
