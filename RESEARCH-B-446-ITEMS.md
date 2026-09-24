# RESEARCH-B-446-ITEMS — javap-ценз ItemEntity.tick (TASK-446-B, vector cmp446_items)

Kernel: purpur-1.21.10 (Moismap) — disasm via javap on `/tmp/patched-kernel-396a.jar`
(= research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar, the live runtime).
Profile evidence: `profile/anchorb422/cpu-collapsed.txt` (vanilla anchor, 116234 samples).

## 1. Lane mass (why items = LAN №1)

- `ItemEntity.tick`-subtree = **34233/116234 = 29.5%** of world CPU (matches CLAIMS ×444/445: 29.5–29.7% at fresh anchors).
- Direct children of `ItemEntity.tick`:
  - `Entity.tick`→`baseTick` 31.1% (of which fluid scans dominate)
  - `Entity.applyEffectsFromBlocks` 28.4% → `checkInsideBlocks` 22.6% → `forEachBlockIntersectedBetween` 17.7% (corner block scans: fire/lava/cactus inside-checks)
  - `Entity.move` 15.0% (collision sweep, Moonrise CollisionUtil 10.0%)
  - `Level.noCollision` 10.5% (the pre-move noPhysics gate)
  - `Entity.updateInWaterStateAndDoFluidPushing` 32.2% any-depth (→ `updateFluidHeightAndDoFluidPushing` 30.2% → `getFluidState` 8.1% → `PalettedContainer.get` 10.5% any-depth, `FluidState.getFlow` 6.4%, `getHeight` 4.5%, `updateFluidOnEyes` 5.1%, `touchingUnloadedChunk` 5.5%)
  - `ItemEntity.getItem` 3.2% (synched-data read)
  - merge scans (`Level.getEntitiesOfClass`) ≈ 4257 samples world-wide (3.7%, ALL callers) — small on this fixture
  - `playerTouch` = 0 samples (fake players don't pick up)
- Conclusion: **~85% of the lane = per-tick world-voxel scans (fluid / inside-blocks / collision) for items that are 99% settled.** Killing the scans for settled items is the vector; merge/pickup/despawn tails stay vanilla.

## 2. javap-ценз ItemEntity.tick (offsets 0..588)

- 0..19: `getItem().isEmpty()` → `discard(DESPAWN)` else continue.
- 20..23: `Entity.tick()` — javap Entity.tick: `if (despawnTime >= 0 && totalEntityAge >= despawnTime) discard(DESPAWN); else baseTick();` (despawnTime = `private final int` on Entity, from WorldConfiguration map).
- 24..48: `if (pickupDelay > 0 && pickupDelay != 32767) pickupDelay--`.
- 51..72: `xo/yo/zo = x/y/z`.
- 80..139: fluid vs gravity: `isInWater() && getFluidHeight(WATER) > 0.10000000149011612` → `setUnderwaterMovement` (=setFluidMovement(0.9900000095367432)); else `isInLava() && getFluidHeight(LAVA) > 0.10000000149011612` → `setUnderLavaMovement` (0.949999988079071); else `applyGravity()`.
  - `setFluidMovement(d)`: `delta = (x*d, y + (y < 0.05999999865889549 ? 0.004999999888241291 : 0), z*d)`.
- 140..225: `noPhysics = !level.noCollision(e, bb.deflate(1.0E-7))` (server side); if noPhysics → `moveTowardsClosestSpace(x, (minY+maxY)/2, z)`.
- 226..270: move-gate: skip `move(SELF, delta)` iff `onGround() && delta.horizontalDistanceSqr() <= 9.999999747378752E-6 && (tickCount + getId()) % 4 != 0`.
- 272: `applyEffectsFromBlocks()`.
- 276..341: friction `f = 0.98f`; `frictionState == TriState.FALSE → 1.0`; else `onGround → block friction * 0.98` via `getBlockPosBelowThatAffectsMyMovement()` (= `getOnPos(0.999999f)`); `delta.multiply(f, 0.9800000190734863, f)`.
- 342..375: bounce: `onGround && delta.y < 0 → delta.multiply(1, -0.5, 1)`.
- 376..473: merge window: `moved = floor(xo)!=floor(x) || floor(yo)!=floor(y) || floor(zo)!=floor(z)`; `k = moved ? 2 : 40`; `if (tickCount % k == 0 && !clientSide && isMergable()) mergeWithNeighbours()`.
  - `mergeWithNeighbours` [0..168]: `bb.inflate(spigotConfig.itemMerge, onlyMergeItemsHorizontally ? 0 : itemMerge-0.5, itemMerge)` + predicate lambda → `getEntitiesOfClass(ItemEntity, bb, pred)`; walls-fix `paperConfig.fixes.fixItemsMergingThroughWalls` → `clipDirect` BLOCK-skip; per candidate `tryToMerge` (areMergable = same item+components && count sum ≤ maxStack; smaller count wins), break on removed.
- 474..498: `if (age != -32768) age++`.
- 499..506: `hasImpulse |= updateInWaterStateAndDoFluidPushing()`.
- 507..543: `if (!clientSide && delta.subtract(preVec).lengthSqr() > 0.01) hasImpulse = true`.
- 544..588: despawn: `if (!clientSide && age >= despawnRate) { ItemDespawnEvent cancelled ? age = 0 : discard(DESPAWN) }`.

## 3. javap-ценз ItemEntity.inactiveTick (the vanilla minimal body)

`Entity.inactiveTick()` = `return` (no-op base). ItemEntity override:
1. `if (pickupDelay > 0 && != 32767) pickupDelay--`
2. `if (age != -32768) age++`
3. `if (!clientSide && age >= despawnRate) { ItemDespawnEvent cancelled ? age = 0 : discard(DESPAWN) }` — **the full vanilla despawn flow IS inside inactiveTick.**

=> The vanilla "resting" body already exists as a vanilla method; it includes despawn counters + gate. Missing vs full tick: stack-empty discard, merge window, move/friction/bounce, fluid state refresh, inside-blocks (fire/lava), hasImpulse.

## 4. Architecture (cmp446_items) — Rust rest-plane, ONE bulk JNI / tick / thread

- **Injection**: whole-body replacement of `ItemEntity.tick()V` → static bridge `ItemBatchOps.tick(Lnet/minecraft/world/entity/item/ItemEntity;)V` (item_merge.rs ReplaceBody precedent).
- **Per tick per region-thread** (drain at first bridge entry of a tick): ONE native `planeDecide([DI[I[J)I` over the PREVIOUS tick's snapshot batch → decisions for THIS tick (1-tick-stale classification; safe: resting state is stable, unknown ids = FULL).
- **Snapshot** (java-side array writes only, ZERO per-entity JNI): stride-12 doubles: `{id, x, y, z, vx, vy, vz, age, pickupDelay, tickCount, flags, 0}`; flags bits: 1=onGround, 2=inWater, 4=inLava, 8=portalProcess!=null, 16=isRemoved, 32=clientSide. All field reads (isInWater/isInLava read cached booleans — the scans themselves stay in the FULL path only).
- **Rust plane** (src/items_batch.rs): sharded `Mutex<HashMap<id, {rest_seq, last_seen}>>` (64 shards); decision per entry:
  - eligibility = planeResting replica (items_subsys2 precedent): onGround && pd∈{0,32767} && !inWater && !inLava && !portal && !removed && !clientSide && horizontalDistanceSqr ≤ 9.999999747378752E-6.
  - ineligible → action FULL; eligible → rest_seq++; action = REST unless rest_seq==1 (landing tick) or rest_seq % 32 == 0 (faithful recheck — full vanilla body, fluid/buoyancy/inside onset latency ≤32 ticks, SAME accepted deviation as items_subsys2 REST_PLANE "fluid recheck 1/32").
- **Java paths**:
  - FULL → `vanillaTick(e)`: faithful body replica of offsets 0..588 (copied from the reviewed items_subsys2 `tickBody`, re-verified against this disasm; merge section invokes the ORIGINAL private `mergeWithNeighbours` via MethodHandle — vanilla scan, walls-fix, tryToMerge untouched; despawn gate vanilla flow).
  - REST → `e.inactiveTick()` (VANILLA method: counters + despawn flow) + vanilla merge cadence `tickCount % 40 == 0 && !clientSide → MH_MERGE.invoke` (vanilla private body self-gates isMergable at offset 0).
- **Decision lookup**: per-tick int→int open-addressing hash built at drain from the batch ids (ids monotonic, never reused; gen-64 full clear). No allocation in steady state.
- **Fail-closed**: ENABLED baked at compile (`"cmp446_items".equals(getenv)` — double gate with the rust STRICT-eq env gate); MH resolve failure / native probe / planeDecide rc<0 / retransform fail → BROKEN → every tick via replica (which is vanilla-equivalent) or, before serve, 100% vanilla class. selfTest BEFORE retransform (fail-closed).

## 5. Canonical risks / honest deviations (resting items only)

1. Fire/lava damage onset + water buoyancy onset delayed ≤32 ticks (recheck discovers; items_subsys2 REST_PLANE identical deviation).
2. Item y-position micro-drift ≤~0.02 blocks during rest windows (vanilla micro-bounce frozen; recheck re-converges).
3. Despawn/pickupDelay counters tick IDENTICALLY (inactiveTick is vanilla code) — despawn timing exact.
4. merge cadence exact (%40 on rest path, moved?2:40 on full path; candidates via vanilla getEntitiesOfClass — merge radii untouched).
5. Pickup path untouched (playerTouch is vanilla; profile shows 0 samples on fixture).

## 6. Budget of the lane vs plan

Expected cut ≈ (rest-fraction) × (1 − 1/32) × (lane minus counters) — with 90% resting ≈ 0.9 × 0.97 × ~85% of 29.5% ≈ 22% of world CPU. Historical caveat: items_subsys2 (bigger scope, index+heap+rest-plane) measured +4.8% median on the old bank v5 fixture (×398, noisy legs +0.0/+12.7) — the current population_target=150000 fixture weights the item lane much heavier (29.5% with 4 stationary fake players); verdict belongs to the bench, min-of-3.

## 7. CYCLE-3 (TASK-448-B, 2026-09-24): coverage audit + refutations + what ships

### 7.1 The BOTTLENECK-448 "collision 25% + fluid 32% residual headroom" premise is stale for resting items

The rest-plane REST path = `e.inactiveTick()` + the vanilla merge cadence — javap re-verified
on the live round-396-a kernel 2026-09-24: `Entity.inactiveTick()V` is a no-op base (`return`),
`ItemEntity.inactiveTick()` = pickupDelay-- / age++ / full despawn flow. ZERO fluid scans,
ZERO collision sweeps, ZERO inside-block checks. I.e. the collision/fluid/inside sub-lanes
are ALREADY fully cut for REST-classified items; they run only on FULL-path items
(transients, ineligible, 1/32 rechecks). The real cycle-3 question is therefore
"why is the FULL fraction high on the 150k fixture" (leg 1r2 lane only 31.17→24.57).

### 7.2 Merge-bucket spatial hash (brief option) — REFUTED on parity grounds (law 4)

Disassembly of the actual candidate enumeration (purpur-1.21.10 + Moonrise):
`mergeWithNeighbours` → `Level.getEntitiesOfClass(ItemEntity, inflate(itemMerge, …))` →
Moonrise `EntityLookup.getEntities` (z-region ⊗ x-region loops, ChunkSlicesRegion 32×32,
chunks asc) → `ChunkEntitySlices.getEntities(Class…)` → `entitiesByClass` (per-class
`EntityCollectionBySection`) → per-section entity arrays whose order includes swap-remove
on entity removal. Both a bridge-side order replica and a count-prescreen are non-airtight:
(a) ORDER — per-section list order mutates via unobservable add/remove (entities added or
discarded between bridge sightings are invisible); an order-divergent enumeration changes
which entity survives a multi-candidate merge (survivor position/age/count feed the NEXT
merge round) = exactly the S7-140 divergence class refuted for pushEntities.
(b) PRESCREEN (skip the scan when zero possible candidates) — the spawn-hole: entities
added to the section storage between the last snapshot and this scan (population-topup
drops, mob-death drops mid-tick) are vanilla candidates no bridge-side summary can count;
a stale under-count → wrong skip = behavioral break. NOT implemented.

### 7.3 Water-rest eligibility extension — REFUTED (new deviation class)

Surface-bobbing is vanilla-inherent motion: setFluidMovement adds 5.0E-4f/tick while
vy < 0.0599 (terminal rise ~0.06/tick), buoyancy push keeps re-energizing; a bobbing item
never satisfies "settled". Freezing the bob = multi-tenths-of-a-block y divergence, far
beyond the accepted §5 envelope (≤0.02 drift). Submerged items rise (same physics).

### 7.4 Pickup-index — LOW-POTENTIAL

playerTouch = 0 samples on the fixture (fake players idle); the mob-side pickup scans
(Mob.aiStep getEntitiesOfClass(ItemEntity, pickup-reach)) live in the MOB lane, outside
the items subsystem (eq-446 measured the whole looting residue at 0.6%).

### 7.5 SHIPPED — drain-throttle ÷4 with tick-true recheck cadence

- Java bridge: snapshot append + planeDecide drain happen once per DRAIN_EVERY=4 server
  ticks per region thread (meta[6]); between drains items only probe the decision hash.
  Decision staleness ≤ 8 ticks (batch age 4 at decide + up to 4 ticks of reuse) — inside
  the accepted §5.1 recheck envelope (32 ticks), magnitude strictly smaller. Unknown ids
  stay FULL fail-open; fresh drops run FULL until their first sighting.
- Rust plane: recheck cadence switched from sighting-count (rest_seq % 32) to TICK-TRUE:
  `tickCount − last_full_tick ≥ 32` (snapshot slot 9), so the cadence is invariant under
  throttling; landing = first eligible sighting ever / after any ineligible sight
  (last_full_tick = MIN sentinel); rechecks counter still marks only genuine 32-cadence
  fulls. Stale-GC unchanged (epoch-based, now drain-epochs).
- Why: saves ~3/4 of the per-tick snapshot append (12 double stores + ~10 field reads per
  item) and 3/4 of the drain/buildHash invocations — on the 150k fixture ≈ 105k items ⇒
  ~1.5-3 ms/tick.

### 7.6 SHIPPED — X-ray: ineligibility reason counters (stats[3..7])

Per drain the plane now also returns WHY items ran FULL: stats[3] not-on-ground,
stats[4] fluid (water|lava), stats[5] hdSqr above the vanilla move-gate constant,
stats[6] pickupDelay ∉ {0,32767}, stats[7] portal/removed (primary-reason priority
portal/removed → fluid → pickupDelay → ground → hdSqr). The java EFFECT log prints them.
This is the cycle-4 eligibility-extension compass: it turns "why is the FULL fraction
high" from guesswork into run data.
