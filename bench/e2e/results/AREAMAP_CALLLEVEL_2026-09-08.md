# AREAMAP CALL-LEVEL — TASK-68 (2026-09-08, S7-19)

## What this closes

S7-18 left TASK-64 verified to MARKER level only: the boot-time
`bridge_selftest` drives `PaperNativeAreaMap.nativeUpdateOpsBatch` DIRECTLY
(native layer), while a live CALL through the patched `update()` body was
assumed to require a moving player client. TASK-68 closes that gap without a
client: an embedded Java driver (dev.crussty.areamapprobe.AreaMapProbe +
nested RecMap, major 52, compiled against the REAL kernel fixture by
`scripts/build_area_map_probe.sh`, bytes via `include_bytes!`) is defined into
the map loader at activation and drives the PATCHED
`SingleUserAreaMap.update(int,int,int)` through the PUBLIC `add()`/`update()`
entry — the full production chain: patched body -> invokestatic
`SingleUserAreaMapOps.run` (whichever arm was defined) -> Scratch ThreadLocal
-> closed native enumeration -> callback dispatch -> recording map.

## The matrix (141 rects, deterministic, xorshift64 seed 0x9E3779B97F4A7C15)

| rows | content |
|---|---|
| guards | NOT_SET (ret=false, fields untouched, no callbacks), negative-d IAE on fresh map (guard A precedes NOT_SET), negative-d IAE after seeding (fields untouched, no delta) |
| retry | `add(0,0,8) -> update(20,20,9)`: n0 = 17²+19² = 650 > 578 = INITIAL_CAP -> budget arm takes the -n0 single retry; legacy arm doubles 578->1156. Parity both. |
| escalate | `add(0,0,13) -> update(40,40,13)`: n0 = 1458 (budget retries to exact req; legacy doubles 1156->2312) |
| grow | ring grow +72 (CHK-1 mirror) then same-state zero-delta on the same map |
| same-state | 64x identical update -> every call ret=true, zero delta |
| spots | 7 hand-verified rects (SmokeMain CHK-3 set; includes (0,0,2)->(0,0,1) 16 removes, (-3,-3,6)->(3,3,6) 120a/120r) |
| sweep | 64 seeded rects — the EXACT seed/draw-order stream of `bridge_selftest` (fx,fz,tx,tz,od,nd; coords rem21-10, d rem7): same stream proven at native layer, now through the full patched call chain |

Every rect: fresh RecMap; seeding via PUBLIC `add(fx,fz,od)` (zero
setAccessible; also verifies (2d+1)² initial adds through the original apply
path); delta-vs-naive set equality + dup check + getters + return value.
Arm discriminator: `Scratch.ops.length` after the retry row — 650 (budget,
post-growth min(cap,2n)) vs 1156 (legacy doubling), asserted by the module
against the actual BudgetMode.

## Headless preflight (before deploy)

`/tmp`-driven session harness ran the probe against the REAL closed
libpaper_native_jni.so + the REAL patched kernel bytes (3320 B, the same
bytes the live patch produces), one child JVM per arm (Boot-style loader,
parent=platform, defines patched map bytes; PaperNativeAreaMap bound by exact
JNI symbol in the same loader):

| arm | ops bytes | bits | scratch | verdict |
|---|---|---|---|---|
| legacy | area-map/build | 0 | 1156 | ALL GREEN |
| budget | area-map/build-budget | 0 | 650 | ALL GREEN |

(smoke rig re-run in the same pass: rc=0, both variants green.)

## Live boots (Purpur 1.21.10, module b715e03d-build, deployed with backup)

| check | DORMANT (gate off) | ARMED (`CRUSSTY_AREAMAP_BUDGET=on`) |
|---|---|---|
| boot Done | 15.8s | 16.1s |
| budget markers | `budget path OFF` | `budget self-test OK (18 probes, len>=n contract + -n0 size oracle verified)` + `budget path ARMED` |
| patch | `patched ... (5075 -> 3320 bytes)`, rc=0 | same, rc=0 |
| direct-native selftest | OK (64 rects) | OK (64 rects) |
| **call-level probe** | **OK (141 rects via patched update(), scratch=1156 legacy arm)** | **OK (141 rects via patched update(), scratch=650 budget arm)** |
| e2e verify | ALL PASS (new `area_map call-level` row PASS) | ALL PASS |
| Exception/ERROR in boot scope | 0 | 0 |
| graceful stop | exit 0 | exit 0 |
| new hs_err | 0 (4 old family) | 0 (4 old family) |

## P500 FULL duty re-run (gate rule: any src/ change)

49 groups (chunked 4+15+30, P500_APPEND, /home/z/BENCH.lock journaled), one
JVM per group: **70/70 pairs compared, 0 missing, 0 regressions, all ok**;
worst deltas are noise (±0.5%; one known-noisy pair +5.2% still in-gate).
Report + raw refreshed in bench/p500/results/.

## Fail-safe contract (unchanged production semantics)

- The probe is diagnostics-only: every row catches Throwable, probe() always
  returns, the boot continues on any outcome; failures surface ONLY as
  `[crussty-plugin] area_map: call-level self-test FAILED ...` marker lines.
- One-shot per JVM (CALL_LEVEL_RAN + define_class name one-shots); hot-reload
  re-init skips loudly (`skipped (already ran)`).
- No gameplay values touched, closed lib untouched, runtime engine untouched.
- e2e tripwire gained one capability-aware row (`area_map call-level`:
  PASS on OK, INFO when the module .so predates the marker, hard FAIL via
  ck_bad on the FAILED line).

## Honest scope

The probe drives patched update() on PRIVATE RecMap instances from the
activation thread — it does not prove anything about real-player movement
patterns (that remains TASK-30 oracle territory for semantics + the S7-18
honest-scope note). What it DOES prove in the live JVM: the patched body
links, dispatches and applies BOTH ops arms end-to-end — marker level is now
call level. Remaining hardening (documented, not scheduled): JNI drive of
`run()` with a RecordingMap SUBCLASS defined post-boot is now SUBSUMED by
this probe (the subclass IS defined and driven); the next genuine gap would
be a real client.
