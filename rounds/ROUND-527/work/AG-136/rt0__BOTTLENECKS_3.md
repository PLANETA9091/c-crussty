# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 11.139 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.6, 0.3, 0.3, 0.3, 0.3, 0.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-10-02T20:49:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 11905146 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384 + S99-gcw ROUND-468-S15; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity; 6 = ParallelGC + MetaspaceSize=256M + ReservedCodeCacheSize=512M — threshold-cascade kill, javap-neutral)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- fluid_dirty_ledger: 0 (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)
- fluid_bitmask: 0 (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)
- region_threads: 0 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 0 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 0 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
- bu_defer: 0 (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)
- lever_flag= lever_arg= (MEGA-ROUND lever A/B marker, TASK-395; empty = vanilla bit-in-bit — canary-gate SKIP-ARMED/C85 layer-2 reads this)
- DP-INSTALLED sha256=16fa1a32cb71966cf1e779cc0c32e1205992738fd92c865650455b9022c19ee2 files=707 dir=/home/runner/work/c-crussty/c-crussty/world3-run/server/world/datapacks/stz3v2

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148055, 148071, 148043]
- top entity types (max seen): minecraft:item×99230, minecraft:skeleton×4899, minecraft:creeper×4689, minecraft:zombie×4659, minecraft:husk×4652, minecraft:drowned×4543, minecraft:spider×4533, minecraft:sheep×3623, minecraft:chicken×3500, minecraft:cow×3453, minecraft:pig×3403, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BihddEeg5o
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **75** (Full GC: **10**)
- total pause: **14678.6 ms**, avg **195.71 ms**, max **2238.4 ms**
- heap high-water seen: **7173 MB** -> last-after: **5395 MB**
  - Young (Allocation Failure): 55
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 39905)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 12904 | 32.3% |
| moonrise/paper patches | 12019 | 30.1% |
| other | 6995 | 17.5% |
| kernel: other | 2730 | 6.8% |
| chunk system (kernel) | 1222 | 3.1% |
| fastutil collections | 976 | 2.4% |
| JDK collections | 855 | 2.1% |
| JDK invokes/VarHandle | 494 | 1.2% |
| network (kernel) | 420 | 1.1% |
| JIT stubs (vtable/itable) | 392 | 1.0% |
| JVM internals (GC oop barriers) | 382 | 1.0% |
| JDK other | 367 | 0.9% |
| vdso (clock) | 117 | 0.3% |
| block entities/hoppers (kernel) | 13 | 0.0% |
| craftbukkit glue | 8 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: main tick (unclassified) | 24039 | 60.2% |
| phase: entity tick (AI/movement) | 10096 | 25.3% |
| phase: unclassified | 4682 | 11.7% |
| phase: network sync (ServerEntity) | 378 | 0.9% |
| phase: chunk tick | 277 | 0.7% |
| phase: chunk system (off-main worker) | 242 | 0.6% |
| phase: block entities (hoppers/furnaces) | 93 | 0.2% |
| phase: random tick | 74 | 0.2% |
| phase: mob spawning | 22 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **34816** (87.2%) · native/JVM-internal **5070** (12.7%) · other **19** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.get` | JVM-Java | 10612 | 26.6% |
| `net/minecraft/world/entity/Entity.moonrise$getChunkStatus` | JVM-Java | 5744 | 14.4% |
| `net/minecraft/world/entity/Entity.getType` | JVM-Java | 3406 | 8.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable$NodeIterator.findNext` | JVM-Java | 2407 | 6.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 625 | 1.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 617 | 1.5% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 536 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 436 | 1.1% |
| `read` | native/JVM-internal | 434 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 401 | 1.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 363 | 0.9% |
| `net/minecraft/world/entity/Entity.isAlwaysTicking` | JVM-Java | 338 | 0.8% |
| `vtable stub` | native/JVM-internal | 329 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 280 | 0.7% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 265 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 261 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 245 | 0.6% |
| `java/util/HashMap.getNode` | JVM-Java | 245 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 237 | 0.6% |
| `net/minecraft/world/entity/Entity.setSharedFlag` | JVM-Java | 226 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 212 | 0.5% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 205 | 0.5% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 204 | 0.5% |
| `ParCompactionManager::follow_marking_stacks` | native/JVM-internal | 202 | 0.5% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 174 | 0.4% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 172 | 0.4% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 171 | 0.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 164 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 156 | 0.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 156 | 0.4% |
| `java/lang/Enum.ordinal` | JVM-Java | 155 | 0.4% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 146 | 0.4% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 146 | 0.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 141 | 0.4% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 136 | 0.3% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 130 | 0.3% |
| `ParMarkBitMap::mark_obj` | native/JVM-internal | 124 | 0.3% |
| `[vdso]` | native/JVM-internal | 117 | 0.3% |
| `java/util/ArrayDeque.size` | JVM-Java | 116 | 0.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 115 | 0.3% |

### WALL profile — self-time by research bucket (total self-time samples: 57569)

| bucket | self-time samples | share |
|---|---|---|
| other | 56491 | 98.1% |
| moonrise/paper patches | 471 | 0.8% |
| entities/mobs (kernel) | 366 | 0.6% |
| kernel: other | 93 | 0.2% |
| chunk system (kernel) | 36 | 0.1% |
| fastutil collections | 30 | 0.1% |
| JDK collections | 19 | 0.0% |
| network (kernel) | 16 | 0.0% |
| JDK invokes/VarHandle | 16 | 0.0% |
| JDK other | 15 | 0.0% |
| JIT stubs (vtable/itable) | 11 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56370 | 97.9% |
| phase: main tick (unclassified) | 849 | 1.5% |
| phase: entity tick (AI/movement) | 313 | 0.5% |
| phase: chunk tick | 14 | 0.0% |
| phase: chunk system (off-main worker) | 7 | 0.0% |
| phase: network sync (ServerEntity) | 7 | 0.0% |
| phase: block entities (hoppers/furnaces) | 4 | 0.0% |
| phase: random tick | 3 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **49080** (85.3%) · native/JVM-internal **8489** (14.7%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47902 | 83.2% |
| `clock_nanosleep` | native/JVM-internal | 4790 | 8.3% |
| `read` | native/JVM-internal | 1218 | 2.1% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.get` | JVM-Java | 430 | 0.7% |
| `net/minecraft/world/entity/Entity.moonrise$getChunkStatus` | JVM-Java | 143 | 0.2% |
| `net/minecraft/world/entity/Entity.getType` | JVM-Java | 113 | 0.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable$NodeIterator.findNext` | JVM-Java | 107 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 21 | 0.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 18 | 0.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 15 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 13 | 0.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 13 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 11 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 11 | 0.0% |
| `net/minecraft/world/entity/Entity.isAlwaysTicking` | JVM-Java | 10 | 0.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 9 | 0.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 8 | 0.0% |
| `vtable stub` | native/JVM-internal | 8 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1238)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1238 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 918 | 74.2% |
| phase: entity tick (AI/movement) | 303 | 24.5% |
| phase: network sync (ServerEntity) | 7 | 0.6% |
| phase: main tick (unclassified) | 6 | 0.5% |
| phase: block entities (hoppers/furnaces) | 2 | 0.2% |
| phase: chunk system (off-main worker) | 1 | 0.1% |
| phase: chunk tick | 1 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1238** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 308 | 24.9% |
| `byte[]_[k]` | other | 200 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 78 | 6.3% |
| `byte[]_[i]` | other | 72 | 5.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 70 | 5.7% |
| `int[]_[i]` | other | 58 | 4.7% |
| `long[]_[i]` | other | 30 | 2.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 25 | 2.0% |
| `java.lang.Object[]_[i]` | other | 25 | 2.0% |
| `int[]_[k]` | other | 21 | 1.7% |
| `char[]_[i]` | other | 16 | 1.3% |
| `java.lang.String_[i]` | other | 15 | 1.2% |
| `java.util.ArrayList_[i]` | other | 15 | 1.2% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 15 | 1.2% |
| `java.util.ArrayList$Itr_[i]` | other | 13 | 1.1% |
| `java.lang.Integer_[i]` | other | 13 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 13 | 1.1% |
| `boolean[]_[i]` | other | 13 | 1.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 12 | 1.0% |
| `java.util.GregorianCalendar_[k]` | other | 11 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 39905 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 3826 | 9.59% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 2323 | 5.82% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 715 | 1.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 634 | 1.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 525 | 1.32% |
| `net/minecraft/world/entity/ai/Brain.tick` | 101 | 0.25% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 77 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 68 | 0.17% |
| `net/minecraft/world/entity/npc/Villager.tick` | 53 | 0.13% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 20 | 0.05% |
| `net/minecraft/world/entity/animal/Cat.tick` | 15 | 0.04% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 13 | 0.03% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 308 | 24.9% |
| `byte[]_[k]` | 200 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | 78 | 6.3% |
| `byte[]_[i]` | 72 | 5.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 70 | 5.7% |
| `int[]_[i]` | 58 | 4.7% |
| `long[]_[i]` | 30 | 2.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 25 | 2.0% |
| `java.lang.Object[]_[i]` | 25 | 2.0% |
| `int[]_[k]` | 21 | 1.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 75 pauses / total 14679 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148043..148071 (delta 28, churn 0.0%), summons=0
  - top movers (max-min across polls): minecraft:item 98708->99230, minecraft:pig 3270->3403, minecraft:cow 3358->3453, minecraft:sheep 3540->3623, minecraft:spider 4456->4533, minecraft:chicken 3446->3500, minecraft:zombie 4632->4659, minecraft:skeleton 4877->4899
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=28)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (9344715 B)
- `wall-collapsed.txt` (521889 B)
- `alloc-collapsed.txt` (630095 B)
- `cpu-flamegraph.html` (137077 B)
- `server-stdout.log` (233380 B)
- `gc.log` (75716 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
