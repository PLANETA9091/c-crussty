# FLUID_GUARD_DESIGN — guard-wave top-1 soundness map + measured refutation of the >100x expectation (TASK-80, S7-25)

* Author: main-s7-25 (cron 18:23+08 Job 366450), 2026-09-08T11:0xZ. ANALYSIS + DESIGN ONLY —
  no src/ changes, no boots, no benches this session. Companion evidence:
  `bench/p500/results/BENCHFIRST_PROFILE_2026-09-08.md` (census) + this doc's §4 (leaf
  distribution — the decisive measurement).
* Verdict up front: **the empty-path same-state guard for
  `Entity.updateFluidHeightAndDoFluidPushing` is SOUND to build (full soundness map in §3)
  but its measured upside is 1.4–1.6x per call (~2% of server-thread CPU at the census
  profile), NOT the >100x class** — the cost is spread across irreducible per-cell
  fluid reads (§4). Implementation is specified completely (§5–§6) and is a valid
  1.4–1.6x/2%-CPU candidate, but it does not belong to the x1000 hunt; the hunt's
  honest conclusion is recorded in §7.

## 1. Target surface (Mojang 1.21.10, bytecode-verified)

`public boolean updateFluidHeightAndDoFluidPushing(TagKey<Fluid> tag, double accel)`
— desc `(Lnet/minecraft/tags/TagKey;D)Z`. Called per entity per tick for WATER
(accel 0.014) and LAVA (0.007 ultra-warm dims / 0.0023333 otherwise) via
`updateInWaterStateAndDoFluidPushing()` (which first `fluidHeight.clear()`s), plus a
second no-clear entry from `LivingEntity.checkFallDamage` →
`updateInWaterStateAndDoWaterCurrentPushing()`. Measured: 5.7% of Server-thread
ExecutionSamples at the census profile (400 items + ~50 husks) — the top entity-path
cost in the live census (BENCHFIRST_PROFILE §2).

## 2. Empty-path exact semantics (bytecode: /tmp/expl/dumps/Entity.fluidPush.txt)

* `touchingUnloadedChunk()` true → `return false` with **zero writes**.
* Else: `aabb = getBoundingBox().deflate(0.001)` (1 alloc), floor/ceil ×6 (pure),
  `isPushedByFluid()` (pure override getter), triple cell loop
  (`level().getFluidState(pos)` + `FluidState.is(tag)`; predicate
  `is(tag) && cellY + getHeight(pos) >= aabb.minY`; `MutableBlockPos` reused, no
  iterator), then on **no matching cell**: accumulator = `Vec3.ZERO` →
  `length()==0` → push block skipped → **`fluidHeight.put(tag, 0.0)`** (the ONLY
  state mutation, offsets 416–428) → `return false`. Allocations: 1 AABB + the
  `touchingUnloadedChunk()` inner `getBoundingBox().inflate(1.0)` (1 more AABB).
* **Non-empty path** (oracle assertions): returns true, puts
  `max(cellTop − aabb.minY)` (>0), and — only if `isPushedByFluid()` —
  `setDeltaMovement(getDeltaMovement().add(scaledFlow))` (NOT idempotent —
  this is why only the EMPTY verdict may ever be cached).

## 3. Soundness map (agent-verified against the jar constant pool)

* **Writers of `fluidHeight`** (Object2DoubleArrayMap, cap 2): ctor (`new`), the
  wrapper (`clear()`), the patched method (`put`). No subclass, no save/load writer.
  Jar-wide: the field is referenced only by Entity (+0 subclasses).
* **Readers**: `isInLava`, `getFluidHeight` (LivingEntity.aiStep LAVA/WATER,
  travelInFluid LAVA, ItemEntity.tick 0.1/0.1, Strider, FloatGoal/Swim/Fox) — all
  `getDouble` with fastutil default 0.0; **no reader distinguishes absent from 0.0**.
* **`wasTouchingWater`**: written only by the wrapper-callee
  (`updateInWaterStateAndDoWaterCurrentPushing`) OUTSIDE the patched body — callers
  act on the return value; skipping the inner body does not bypass those writes.
* **Overrides: NONE** anywhere in the vanilla jar for the patched method (jar-wide
  name scan); `AbstractMinecart` re-declares the wrapper as a pure super-delegate —
  the prologue still executes for minecarts.
* **Stale-entry hazard (the one real trap)**: `checkFallDamage` re-enters water
  pushing WITHOUT the wrapper's `clear()`; if a previous run found fluid
  (`put >0.0`) and the fluid then drains without the entity moving, skipping the
  body would leave a stale >0.0 entry that `aiStep`/`travelInFluid`/`ItemEntity`
  would read (vanilla zeroes it). Guard consequence: the skip must additionally
  require `fluidHeight.getDouble(tag) == 0.0` (one ArrayMap get, ~free) — and since
  `fluidHeight` is PRIVATE, the prologue needs a one-time-reflection-cached
  accessor (see §6) or its own mirrored verdict state.
* **Unloaded chunks**: guard must mirror `touchingUnloadedChunk()` (or call it —
  public) BEFORE any `getFluidState`; when it would return true, the true body
  returns false with zero writes → the guard may return false directly
  (bit-identical) without touching cells.

## 4. MEASURED REFUTATION of the >100x expectation (decisive)

Leaf distribution of the 30 census samples inside
`updateFluidHeightAndDoFluidPushing` (BENCHFIRST_PROFILE JFR, Server thread):

| Leaf frame | Samples | Share | Skippable by the guard? |
|---|---:|---:|---|
| method body itself (loop + predicate + floors + map put) | 14 | 47% | partially — the guard still loops cells; only the `put` + return path is saved |
| `PalettedContainer.get` (fluid-state palette reads) | 5 | 17% | **NO — re-verification needs the same reads** |
| `AABB.inflate` + `AABB.<init>` (touchingUnloadedChunk + deflate allocs) | 7 | 23% | **YES — pure-math mirror, zero allocs** |
| `ServerChunkCache.getChunk` | 2 | 7% | NO (same reads) |
| `Mth.floor` / `getBoundingBox` | 2 | 7% | partially (pure math) |

Skippable machinery ≈ 23–40% of the path → guard per-call ratio
**1.4–1.6x** (0.46 µs/call avg at census → ~0.29–0.33 µs), i.e. **~1.7–2.3% of
Server-thread CPU** at the census profile, scaling linearly with entity count on
production servers. This is a REAL, honest win — but the same-state-guard class
does **not** transfer to this path at >100x magnitude: unlike area-map (where the
guarded apply loop was 48 µs–4.25 ms of native enumeration + JNI copy), the
"machinery" here is already minimal and the cost is data-dependent cell reads.
**The >100x-class hunt branch "entity-path same-state guards" is FALSIFIED by
measurement** (6th refuted branch in the project ledger, after blend-cache,
batching site-level, DENSE, coalescing, and noise-batching microbench expectations
— the latter inverted into a win by the inlining-barrier mechanism, TASK-74).

## 5. Full-skip variant — why it is ENGINE-TOUCH

Skipping the cell reads entirely (the remaining 60–70%) requires knowing the cells
did not change since the previous tick: a block-version counter over overlapped
sections. Vanilla exposes none; plugin-side hooks cannot observe `Level.setBlock`.
The CRUSSTY runtime engine could maintain one (a setBlock hook bumping a global
`AtomicInteger`, read by the guard as part of the key) — that is an
ENGINE-TOUCH change (docs/BATCH_ROLLOUT_RUNBOOK rules) and a separate decision.
Not pursued this session.

## 6. Implementation specification (complete, ready for a build session)

* **Weave primitive** (`cplug-sdk/src/weave.rs`, sibling of the proven
  `insert_call_at_start`): `insert_guard_at_start(bytes, method_name, method_desc,
  guard_class, guard_desc_returning_Z, ...)` emitting
  `aload_0; aload_1; dload_2; invokestatic guard((Entity;TagKey;D)Z); ifeq →body;
  iconst_0; ireturn; <original body>` — prefix length 11; `ifeq` operand = 2
  (target = original body start at offset 11); `max_stack` bumped to
  `max(original, 4)`; switch/exception/LNT/LVT shifts identical to
  `insert_call_at_start`; StackMapTable requires ONE INSERTED `full_frame` at
  offset 11 (empty stack, locals `[Entity, TagKey, double]`) + first-frame
  offset_delta re-derivation (original first frame moves from F to F+11 → its
  delta becomes F−1; per-frame recurrence per JVMS 4.7.4). Unit-test fixture: the
  real `net/minecraft/world/entity/Entity.class` (major 65) from
  `/tmp/expl/vanilla_mojang.jar` + javap structural validation + a
  defining-classloader smoke load (verification-at-define catches frame errors).
* **Guard class** `dev/crussty/fluidguard/FluidGuardPrologue` (major 52,
  `include_bytes!` into the lane module, defined into the kernel loader):
  static `boolean probe(Entity e, TagKey<?> tag, double accel)` —
  (1) `e.touchingUnloadedChunk()` (public) → true→ return FALSE (fall through; body
  returns false with zero writes — identical, keep semantics uniform), wait — per §3
  the unloaded case may return false directly; choose: fall through when unloaded
  (simplest, still correct: the body handles it; cost = the body's early-out, rare);
  (2) compute cell range from `e.getBoundingBox()` with pure math (deflate 0.001
  inlined, no AABB alloc); (3) range != cached range or tag identity changed →
  fall through (body runs; cache refreshed by the NEXT probe via post-body hook —
  simpler: cache the verdict of the probe's OWN re-verification and only skip when
  the previous probe ALSO saw all-empty AND the range matched: two consecutive
  all-empty observations without movement); (4) re-verify cells
  (`e.level().getFluidState(pos).is(tag)` + height predicate — public API, no
  AABB allocs) — any hit → fall through; (5) `fluidHeight.getDouble(tag) == 0.0`
  via one-time-reflection accessor (setAccessible, cached Field; unnamed-module
  access OK) else fall through; (6) all-empty + range-match + zero-entry →
  return TRUE (caller returns false; body skipped; stale-window impossible —
  verified by condition 5).
* **Lane** (`src/fluid_guard.rs`, perlin_noise.rs template): byte-hook capture of
  `net/minecraft/world/entity/Entity`, env gate `CRUSSTY_FLUID_GUARD`
  (default OFF → dormant byte-identity, one `dormant (set …=1)` marker line),
  patch computed on the quiet activation worker via the new weave primitive,
  `audit_wire` + `READY` + single `retransform_class`, one-shot selftest
  (in-JVM: fabricate entity-less static assertions of the probe's cache logic +
  marker `fluid_guard: self-test OK/FAILED`), e2e rows
  (`ck_cap` armed/dormant/selftest + `ck_bad` FAILED + `FLUID_LIFESIGNS`).
* **Duty chain** (TASK-73 order): build_fluid_guard.sh → cargo build --release +
  cargo test + clippy Δ0 → **P500 FULL duty** (src/ touched) → dormant boot
  (verify ALL PASS, 0 delta) → deploy with /tmp backup → armed boot
  (CRUSSTY_FLUID_GUARD=1; live selftest drives REAL entities: land-static skip,
  water-push still applies (guard must NEVER skip with fluid present — assert
  deltaMovement accumulation identical), tag switch, drain-while-standing
  (stale-window), teleport/range-change) → hs_err census 4/0.
* **Expected result (honest)**: 1.4–1.6x per-call on the guarded path; census-profile
  server-thread CPU −1.7–2.3%; production entity-dense servers scale linearly.

## 7. Where the x1000 hunt stands after this refutation

Every plugin-reachable same-state candidate from X1000_CANDIDATES_V3 §3 has now been
measured or soundness-mapped: fluid-push (this doc — machinery 23–40%,
1.4–1.6x class), checkInsideBlocks (leaf census: 44% InsideBlockEffectApplier
machinery — same 1.5–2x class, same re-verify physics), noCollision (spread across
query machinery — needs section-version infra = engine domain), hopper idle probe
(demoted, ≪1 µs), isInWall/sensors (below resolution pending a mob-dense profile).
**Conclusion: the >100x class in this codebase remains exactly what BOOST_SWEEP §3
said — same-state guards where the guarded machinery is 95%+ of the cost (area-map
apply; the one shipped instance: 1,945x–170,612x), lifecycle hygiene (>571x), and
boot-scan caching (>10x–100x)** — plus the in-.so/engine domain (block-version
counters would unlock the full-skip variants; batch entries inside the closed .so
would change the floor arithmetic). The remaining measured-but-unbuilt wins are
honest multipliers in the 1.2–3.3x kernel-swap class (X1000 §2 table) and the
1.4–2x guard class specified here — worth building for production entity density,
but they must not be sold as x1000.
