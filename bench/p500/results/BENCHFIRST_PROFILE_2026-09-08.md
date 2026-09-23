# BENCH-FIRST PROFILE — live entity-path census (TASK-78 part A, S7-24)

* Author: main-s7-24 (cron 17:23+08 Job 366450), 2026-09-08T10:0xZ. First-ever entity-load
  profile on this box (TASK-57 idle JFR showed only TPS accounting; this run adds the
  missing load dimension). Companion: docs/X1000_CANDIDATES_V3.md §3 (candidates) —
  this document is the ESTIMATE-column falsifier promised there.
* Artifacts: /home/z/server/logs/jfr_census/census.jfr (2.26 MB, 300 s) +
  exec_samples.txt (jfr print --events jdk.ExecutionSample). World restored from
  world_census_seed.tar.gz after the run (see protocol below).

## 1. Method

* Dormant boot (canonical scripts/e2e_orchestrate.sh, no CRUSSTY_* env): Done 16.040s.
  BENCH-MUTEX journal lock→done (entity-census-jfr). hs_err 4 family / 0 new before+after.
* World tar anchor BEFORE any mutation (world_census_seed.tar.gz, 21.5 MB); restored +
  region count verified (17 files) after the run.
* Load (via hardened rcon.py, rotated secret, port 25575): forceload 9 chunks
  ([-36,-23]..[-34,-21], covers world spawn (-544,75,-336)); **400 item entities**
  (stone, Age:-32768s, PickupDelay:-1s, 1-block grid — all "Summoned", killed 400 at
  teardown ✓); **100 husks** (20×2-spacing grid); **9 hoppers** (3×3 at y=80).
* JFR: jcmd post-boot attach (F2 lesson respected — no startup recording),
  settings=profile (jdk.ExecutionSample 10 ms), duration=300 s, auto-stop; TPS 20.0
  throughout (5m/15m dip to 19.2/19.8 during summon storm only).
* Teardown: kill @e[item] = 400, kill @e[husk] = **38** (62 despawned mid-window —
  Paper no-player hostile-despawn rules; confound for living-entity paths, ZERO effect
  on the item paths), forceload remove all, graceful stop exit 0, 0 stray JVMs.

## 2. Census (524 Server-thread ExecutionSamples of 551 total = 95%)

| Frame (any depth) | Samples | Share of server-thread samples | Verdict vs X1000_V3 estimate |
|---|---:|---:|---|
| `ItemEntity.tick` (umbrella) | 78 | 14.9% | items dominate this profile as designed |
| `Entity.move` | 28 | 5.3% | — |
| **`Entity.updateFluidHeightAndDoFluidPushing`** | **30** | **5.7%** | **CONFIRMED — top actionable guard target** |
| `Entity.baseTick` (umbrella) | 22 | 4.2% | fluid-push is its main child |
| **`Entity.checkInsideBlocks`** | **18** | **3.4%** | **CONFIRMED — above expectation** |
| `EntityGetter.getEntitiesOfClass` (entity-grid query) | 12 | 2.3% | machinery under noCollision/sensor |
| `CollisionGetter.noCollision` | 10 | 1.9% | present; per-call cost smaller than the 0.5–5 µs ESTIMATE at this density |
| `HopperBlockEntity.tryMoveItems` (9 idle hoppers) | 0 | 0% | idle-probe cost ≪ 1 µs at n=9 empty containers — the 1–10 µs ESTIMATE was too high for the empty case; not measurable at this n |
| `Entity.isInWall` | 0 | 0% | below resolution (husk decay confound + cheap non-suffocating case) |
| `Brain.forgetOutdatedMemories` | 0 | 0% | below resolution (same) |
| `NearestLivingEntitySensor.doTick` | 0 | 0% | below resolution (same) |

Honest scope notes: (1) ExecutionSample counts Java-frame presence — relative shares
among candidates are valid, absolute wall-clock shares are not derivable from this
recording alone (NativeMethodSample 14.9k = parked time dominates the box). (2) The husk
decay (100→38) shrinks LivingEntity-path exposure in the second half of the window; the
item paths (noCollision, fluid via baseTick, checkInsideBlocks via move) carried full
load the whole 300 s. (3) n=400 items / ~50-avg husks / 9 hoppers is a SMALL server
profile — a production farm/perf server runs 10–100× this, which scales all shares.

## 3. Verdicts for the guard wave (X1000_CANDIDATES_V3 §3 → §6)

1. **Guard wave top-1 = `updateFluidHeightAndDoFluidPushing`** (5.7% measured,
   was #2 by static estimate). Guard-key: quantized AABB + per-cell FluidState
   identity; motionless-entity and land fast-cases. Pure-read except the
   deltaMovement mutation, which stays outside the cached region.
2. **#2 = `checkInsideBlocks`** (3.4% measured, was #5) — cache ONLY the empty-result
   case (effects semantics preserved by construction); from/to Vec3 + cell-version key.
3. **#3 = `noCollision`** (1.9% + 2.3% grid machinery) — AABB-bits + section
   state-version cache; pure read.
4. **Hopper idle probe: DEMOTED** — measured ≪ estimate at empty containers; revisit
   only with a hopper-dense load profile (hundreds of hoppers with real inventories).
5. **isInWall / sensors / memories: UNRESOLVED at this profile** — require a
   player-attached or mob-dense profile (husk decay confound); keep ESTIMATE, do not
   implement without a better profile (TASK-32 NO-GO lesson: do not build on absent load).

## 4. Cost of the run

~9 min wall (boot 16 s + summons ~20 s + 300 s window + teardown + restore). Zero src/
changes, zero P500 duty, deployed module untouched (dormant), world byte-identical
after restore, hs_err 4/0. Fully repeatable: see §1 — the tar anchor + RCON summon
loop is the reusable load harness (first entity-load harness in the repo).

## 5. Part B — chunk-encode .so light kernel measured (first benchmark of the last dark domain)

Rig: bench/chunkencode/ (ChunkEncodeParity.java + run_chunk_encode_parity.sh; headless,
no server; BENCH-MUTEX; RAW = results/CHUNKENCODE_LIGHT_RAW.tsv). Parity WITHOUT Unsafe:
the vanilla decoder constructor (FriendlyByteBuf,int,int) decodes the native output, then
(1) decoded fields == native inputs, (2) write(decode(dst)) == dst byte-identical.

* **PARITY light: 5/5 cases PASS** (realistic/all-empty/all-set/single-section/2-long
  masks × 26 sections; n=106,622 B) — nativeEncodeLightData emits vanilla
  ClientboundLightUpdatePacketData wire format byte-identically.
* **TIMING: native 10,678 ns/call vs vanilla write() 2,665 ns/call → native 4.0x
  SLOWER** (vanilla/native = 0.25). Physics: per call the JNI path copies ~213 KB
  (2×26×2048 B nibble arrays in + ~106 KB dst out) — copy-bound, same class as the
  area-map apply finding (TASK-20-R); vanilla does the identical logical work in-JVM.
* **VERDICT: DO-NOT-WIRE class (honest negative).** The kernel is registered since
  boot but nothing routes to it — production behavior unchanged; this measurement
  closes the last dark domain with a measured negative instead of an assumption
  (TASK-32 discipline). nativeEncodeSectionData/Sized remain unmeasured (blob
  semantics need n=1 identification rows — documented follow-up); given section
  payloads are ~10x larger per chunk, copy-bound physics likely dominates them too.
* Rig lesson (caught in-session): the plain variant does NOT bound-check dst capacity
  (that is what *Sized is for) — a non-physical sec=128 probe with a 128 KiB dst
  aborted the driver JVM via Rust panic (no hs_err residue, server census stayed 4/0).
  Physical 26-section inputs only for the plain variant.

## 6. Session net effect

* Entity-path candidates: MEASURED ranking (fluid-push 5.7% / checkInsideBlocks 3.4% /
  noCollision ~4.2% incl. grid machinery; hopper idle-probe demoted; 3 paths below
  resolution pending a mob-dense profile).
* Chunk-encode domain: dark → measured (1 parity-verified regression-class kernel,
  2 follow-ups). The x1000 hunt's evidence base is now fully empirical on every
  identified surface.
