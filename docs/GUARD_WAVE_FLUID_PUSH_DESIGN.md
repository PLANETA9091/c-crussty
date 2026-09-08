# GUARD WAVE — FLUID-PUSH SAME-STATE GUARD (TASK-80, Session-1)

* Author: agent-7625532f (cron 18:40+08 Job 366516), 2026-09-08. Wave-plan step 2
  (X1000_CANDIDATES_V3 §6) on the measured top-1 entity hot path: `Entity.
  updateFluidHeightAndDoFluidPushing(TagKey<Fluid>,double)` — **5.7% of
  Server-thread ExecutionSamples** under the TASK-78 census load (400 items +
  husks, BENCHFIRST_PROFILE_2026-09-08.md §2).
* Guard class physics (X1000_V3 §1): >100x-class wins come from an O(1) guard
  at method entry that skips per-call machinery. The three prior >100x
  mechanisms (area-map 1,945x–170,612x, lifecycle >571x, boot-scan) share this
  exact shape. The TASK-74 inlining-barrier finding raises live effect above
  per-call estimates (5–8x multiplier there).

---

## 1. Vanilla body semantics (javap ground truth, purpur-1.21.10.jar)

```
updateFluidHeightAndDoFluidPushing(TagKey<Fluid> tag, double speed) -> boolean
  if touchingUnloadedChunk() -> return false            (NO fluidHeight.put)
  box = getBoundingBox().deflate(0.001)
  clamp Y into [minSection<<4, (maxSection<<4)|15]; floor/ceil cell bounds
  pushFlag = isPushedByFluid()
  Moonrise bulk fetch: flat LevelChunkSection[][] rows via
      chunkSource.getChunk(cx, cz, FULL, load=true).getSections()
  scan every cell (x,y,z) in bounds:
      fs = section.states.get(x&15 | z&15<<4 | y&15<<8).getFluidState()
      if fs.isEmpty() || !fs.is(tag): continue
      if tag == FluidTags.LAVA: lastLavaContact = pos.immutable()
      d0 = y + fs.getHeight(level, pos) - box.minY;  if d0 < 0: continue
      inFluid = true;  maxDepth = max(maxDepth, d0)
      if pushFlag: flowCount++; flow = fs.getFlow(level, pos)
                   flowAcc = flowAcc.add(flow.scale(maxDepth) if maxDepth<0.4
                                                   else flow)
  fluidHeight.put(tag, maxDepth)
  if flowAcc == Vec3.ZERO (IDENTITY): return inFluid     (no push, no alloc)
  flowAcc = flowAcc.scale(1.0/flowCount); if !(this instanceof Player):
            flowAcc = flowAcc.normalize()
  flowAcc = flowAcc.scale(speed)
  if |delta.x|<0.003 && |delta.z|<0.003 && flowAcc.length()<0.0045:
      flowAcc = flowAcc.normalize().scale(0.0045)
  setDeltaMovement(delta.add(flowAcc));  return true
```

Caller (`updateInWaterStateAndDoFluidPushing`) clears `fluidHeight` FIRST, then
runs WATER(0.014) and LAVA(0.007 ultraWarm / 0.0023333) — two scans per entity
per baseTick. `fluidHeight.put(tag, maxDepth)` is therefore MANDATORY on every
non-unloaded path (the map was cleared by the caller).

## 2. Guard design — negative-only, identity-validated, zero invalidation hooks

**Cached outcome: ONLY the pure-negative result** ("no cell in the deflated box
carries a fluid state matching `tag`" → vanilla returns `false` with
`fluidHeight.put(tag, 0.0)` and NO push). This is the measured hot case: every
land entity (400 census items, husks on stone) takes it twice per tick; a
production item-farm server scales it 10–100x.

**Validation = identity re-read, no invalidation infrastructure.** Key insight:
`FluidState` instances are canonical singletons (like `BlockState`). A cell's
fluid state changes iff a different `FluidState` reference is read back. The
guard therefore re-reads the 1–8 box cells on every hit and compares
references. Any change, any block edit, any fluid tick anywhere → the cell
reads back a different singleton → miss → slow path. Soundness is structural:
no version counters, no setBlock hooks, no race windows beyond what the
vanilla body itself has (same cell reads, same tick thread).

Entry key = weak entity identity (Guava `MapMaker().weakKeys()` = identity
comparison, concurrent, GC-freed with the entity — no reaper needed, entries
hold no strong entity ref). Entry payload: 6 box bits (raw `getBoundingBox()`
double bits), `TagKey` reference identity (call sites pass `FluidTags.WATER/
LAVA` statics — fresh TagKey instances simply always miss), `Level` reference
(dimension change → miss), the per-cell `FluidState[]` in scan order + the
deflated cell bounds. Hit requires: same level ref, same 6 box bits, same tag
ref, chunk fetch (`load=false`) succeeds for every overlapped column, every
re-read `FluidState` reference-identical. Any failure → slow path (which is
itself a full vanilla-equivalent reimplementation — miss never diverges).

Hit path work: map get + box-bits compare + 1 column fetch + 1–8 palette gets
+ identity compares + `fluidHeight.put(tag, 0.0)` + `return false`. The
skipped machinery: `touchingUnloadedChunk` (inflate alloc + hasChunksAt), the
AABB deflate alloc, the sections[][] bulk fetch + init loop, MutableBlockPos,
getFluidState/is/getHeight/getFlow virtual chains, Vec3 allocs.

**Fluid-present bodies are NEVER cached** (negative-only rule, X1000_V3 §7):
`getFlow` reads the 4-horizontal neighborhood + below-fluid + FALLING solid
faces — a neighbor edit can change flow while all box cells hold identical
FluidStates; validating that neighborhood would cost most of the scan again.
Fluid-present entities keep exact vanilla semantics through the slow path.
`isPushedByFluid()` / `speed` / delta-clamp branches are always evaluated
fresh (they are entity- or call-state, not world-state).

## 3. Slow path — full reimplementation, all-public callees

We cannot call the original body (whole-body replace), so the slow path is a
faithful Java reimplementation from the javap transcript (§1). Every callee is
public: `AABB.deflate`, `WorldUtil.getMinSection/getMaxSection` (Moonrise),
`Mth.floor/ceil`, `ChunkSource.getChunk(IILChunkStatus;Z)`,
`ChunkAccess.getSections`, `LevelChunkSection.states` (public final field,
cross-package-read by Entity itself in the original body),
`PalettedContainer.get(I)`, `FluidState.isEmpty/is/getHeight/getFlow`,
`Vec3.add/scale/normalize/length`, `BlockPos.MutableBlockPos.set/immutable`.
Protected members `fluidHeight` + package access: the hook class is defined
into package `net.minecraft.world.entity` in the kernel loader (same loader,
same package — identical access rights to `Entity`; the proven
define-into-kernel-loader pattern from improved_noise/perlin_noise).
`lastLavaContact` is public. Cache refresh on the slow path only for the
pure-negative outcome (same entry shape as hit-path validation expects).

## 4. Wiring and gates

* Rust module `src/fluid_guard.rs` — structural derivative of perlin_noise.rs:
  byte hook captures Entity's pristine bytes at its own load (Entity loads
  during boot, after plugin init — proven sighting pattern); patch computed on
  the quiet activation worker via `cplug_sdk::asm::replace_body` (spec: method
  `updateFluidHeightAndDoFluidPushing`, desc `(Lnet/minecraft/tags/TagKey;D)Z`,
  bridge owner `net/minecraft/world/entity/FluidPushGuardHook`, static desc
  `(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z`, args
  slots 0(L),1(L),2(D)); single retransform; self-test reflectively drives
  `FluidPushGuardHook.selfTest()`.
* Env gate `CRUSSTY_FLUID_PUSH_GUARD` (1/true/on/yes), **default OFF,
  dormant-invisible** (no byte hook registered, no classes defined, byte-
  identity with the pre-TASK-80 plugin verified by the e2e dormant check).
* Kill-switch: the env gate is boot-time (arming) — a running server is
  reverted by restarting without the env; no runtime toggle needed for a
  default-OFF hook (same policy as perlin_noise).
* Markers (harness-greppable): `fluid_guard: dormant`, `fluid_guard: pristine
  sighting Entity`, `fluid_guard: hook armed, retransform rc=0`,
  `fluid_guard: self-test passed (guard cache operational)`.

## 5. Expected effect and falsifiers

* Per-call ESTIMATE: vanilla land-case ≈ 300–600 ns (scan machinery + allocs +
  chunk fetch) vs guard hit ≈ 100–250 ns (fetch + identity compare + put) →
  **~2.5–4x per-call** on the negative path. This is NOT a 1000x-per-call
  surface (the body is at the small end of X1000_V3 §3); it is the measured
  top-1 actionable path (5.7% server-thread share). Under the census profile
  the ceiling is ~3–4% server-thread wall; under item-farm load (10^4–10^5
  items) the share and the saving scale together.
* Falsifiers (honest kill conditions): (1) hit-rate on the live profile — if
  the A/B shows the guard path missing (e.g. entities move every tick so box
  bits never repeat), the mechanism dies like TASK-32's blend-cache; (2) JFR
  call-presence must show the hook body eating the samples; (3) any
  behavioral divergence in the trajectory oracle kills it outright.
* Parity oracle: deterministic item-physics trajectories across dormant vs
  armed boots (summon harness from TASK-78; items are AI-free ⇒ bit-exact
  physics), plus dormant/armed marker trails and byte-identity dormant boot.

## 6. Disciplines carried

Env-gated default OFF; dormant byte-identity; no gameplay-value changes (the
guard reproduces vanilla values bit-exactly — it only skips recomputation of
identical results); BENCH-MUTEX on the 2-CPU box; world tar anchors before
mutation; hs_err census before/after; P500 FULL duty on src/ changes; honest
NO-GO is a valid outcome and closes the fluid-push branch with a measurement.
