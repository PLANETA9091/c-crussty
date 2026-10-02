# AREAMAP BUDGET — live armed boot verification (TASK-64 phase 3, S7-18)

Agent: main (session S7-18, cron 13:23+08 Job 366450) · 2026-09-08 · module: master a58bcb5 build (sha256 38ec5943…, 1081328 B)
Deployed: /home/z/server/modules/crussty/libcrussty.so (backup /tmp/libcrussty_module_pre_s7-18_053125.so, sha 8582ce0c…)
Server: Purpur 1.21.10 + CRUSSTY runtime (4f9998b poison-recovery + 3bf1477 phase-gate, deployed S7-15)

## Verdict

| Boot | Result |
|---|---|
| DORMANT (gate off, production default) | verify ALL PASS; `budget gate …=unset -> mode=off`; `budget path OFF`; legacy bytes defined (Scratch/$1/Ops); `patched … (5075 -> 3320 bytes)`; `retransform rc=0`; bridge self-test OK (64 rects); exit 0 |
| P500 full re-run (dormant config, BENCH-MUTEX) | 49 groups, 0 CRASH / 0 SKIP, ratio-gate **70/70 pairs ok, 0 regressions, rc=0** (worst Δ −0.7%/+0.4%) |
| **ARMED** (`CRUSSTY_AREAMAP_BUDGET=on`) | `budget gate …=on -> mode=on`; **`budget self-test OK (18 probes, len>=n contract + -n0 size oracle verified)`** — the closed native's contract held in the LIVE JVM; `budget path ARMED (SingleUserAreaMapOps = budgeted scratch, -n0 retry)`; patch rc=0; bridge self-test OK (64 rects); verify ALL PASS; **0 Exception/ERROR lines in the armed-boot scope**; exit 0 |
| hs_err passive monitor | 0 new across both boots (4 pre-existing family files unchanged) |

## Evidence (server.log, session-scoped)

```
[crussty-plugin] area_map: budget gate CRUSSTY_AREAMAP_BUDGET=on -> mode=on (budgeted scratch + -n0 retry (native-contract self-test gates arming))
[crussty-plugin] area_map: budget self-test OK (18 probes, len>=n contract + -n0 size oracle verified)
[crussty-plugin] area_map: budget path ARMED (SingleUserAreaMapOps = budgeted scratch, -n0 retry)
[crussty-plugin] area_map: patched ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap update() (5075 -> 3320 bytes)
[crussty-plugin] area_map: hook armed, retransform rc=0
[crussty-plugin] area_map: self-test OK (64 rects, native == naive set difference)
```

Dormant control emitted exactly `budget gate …=unset -> mode=off` + `budget path OFF` and no budget self-test
lines (self-test is gate-on only) — zero-delta default path confirmed live.

## Honest scope note

The armed boot verifies to marker level: gate parse, in-JVM native contract (18 probes), budgeted class
define/link in the kernel loader, kernel patch + retransform, bridge parity self-test. A live CALL of the
budgeted `run()` through the patched kernel requires player movement (NearbyPlayers.tickPlayer) — no client
exists in this sandbox; the full-call semantics are covered by the TASK-30 oracle (268/268, both arms, same
native). A JNI-driven live `run()` drive (define a RecordingMap subclass in the kernel loader + call
`SingleUserAreaMapOps.run` over rects) is the identified NEXT hardening if marker-level evidence ever
proves insufficient.

## Rollout state

Deployed module = master a58bcb5 (variant C included, gate default OFF). Production behavior is unchanged
until an operator exports `CRUSSTY_AREAMAP_BUDGET=on`; kill-switch = unset env + restart. The gate decision
is fail-safe: a deployed .so that violates the len≥n/-n0 contract falls back to legacy bytes at activation.
