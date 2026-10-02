# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.844 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 1.8, 2.0, 2.3, 2.6, 1.5]
- spark tick-monitor MSPT: avg **376.45ms** / min 328.76ms / max **514.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-10-02T21:36:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6695946 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- region_threads: 8 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 0 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 1 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
- bu_defer: 0 (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)
- lever_flag= lever_arg= (MEGA-ROUND lever A/B marker, TASK-395; empty = vanilla bit-in-bit — canary-gate SKIP-ARMED/C85 layer-2 reads this)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 328.76 | — | — | — | 514.83 | 376.45 |

- entity totals seen: [148789, 150560, 150879]
- top entity types (max seen): minecraft:item×103019, minecraft:creeper×5219, minecraft:husk×5158, minecraft:skeleton×4847, minecraft:spider×4734, minecraft:zombie×4609, minecraft:drowned×4496, minecraft:sheep×3539, minecraft:chicken×3433, minecraft:cow×3331, minecraft:pig×3227, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/zJTXWZ7aec
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **23466.6 ms**, avg **197.20 ms**, max **2685.9 ms**
- heap high-water seen: **7864 MB** -> last-after: **3708 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 118294)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28435 | 24.0% |
| entities/mobs (kernel) | 27641 | 23.4% |
| other | 13339 | 11.3% |
| chunk system (kernel) | 11066 | 9.4% |
| moonrise/paper patches | 11034 | 9.3% |
| fastutil collections | 7616 | 6.4% |
| JDK collections | 6955 | 5.9% |
| network (kernel) | 3847 | 3.3% |
| JIT stubs (vtable/itable) | 3209 | 2.7% |
| JDK invokes/VarHandle | 2824 | 2.4% |
| JDK other | 1839 | 1.6% |
| vdso (clock) | 163 | 0.1% |
| block entities/hoppers (kernel) | 117 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| bukkit api | 56 | 0.0% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95563 | 80.8% |
| phase: unclassified | 12736 | 10.8% |
| phase: main tick (unclassified) | 2513 | 2.1% |
| phase: network sync (ServerEntity) | 2443 | 2.1% |
| phase: chunk tick | 2379 | 2.0% |
| phase: chunk system (off-main worker) | 1226 | 1.0% |
| phase: block entities (hoppers/furnaces) | 762 | 0.6% |
| phase: random tick | 516 | 0.4% |
| phase: mob spawning | 154 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **104438** (88.3%) · native/JVM-internal **13775** (11.6%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5605 | 4.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4270 | 3.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2732 | 2.3% |
| `vtable stub` | native/JVM-internal | 2546 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2391 | 2.0% |
| `WallClock::signalHandler` | native/JVM-internal | 2337 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2222 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2205 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 2095 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1939 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1886 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1828 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1571 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1469 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1405 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1334 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1272 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1221 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1214 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1190 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1174 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1110 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1103 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1102 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1099 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1096 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 995 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 982 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 944 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 874 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 805 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 780 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 766 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 765 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 754 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 733 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 718 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 694 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66054)

| bucket | self-time samples | share |
|---|---|---|
| other | 59757 | 90.5% |
| entities/mobs (kernel) | 1822 | 2.8% |
| kernel: other | 1553 | 2.4% |
| moonrise/paper patches | 694 | 1.1% |
| chunk system (kernel) | 637 | 1.0% |
| fastutil collections | 478 | 0.7% |
| JDK collections | 373 | 0.6% |
| network (kernel) | 239 | 0.4% |
| JIT stubs (vtable/itable) | 215 | 0.3% |
| JDK invokes/VarHandle | 149 | 0.2% |
| JDK other | 116 | 0.2% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| bukkit api | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 59455 | 90.0% |
| phase: entity tick (AI/movement) | 6131 | 9.3% |
| phase: main tick (unclassified) | 148 | 0.2% |
| phase: chunk tick | 109 | 0.2% |
| phase: chunk system (off-main worker) | 75 | 0.1% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 30 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56880** (86.1%) · native/JVM-internal **9166** (13.9%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 50648 | 76.7% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.2% |
| `read` | native/JVM-internal | 1235 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `accept` | native/JVM-internal | 1201 | 1.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 302 | 0.5% |
| `syscall` | native/JVM-internal | 249 | 0.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 221 | 0.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 183 | 0.3% |
| `vtable stub` | native/JVM-internal | 163 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 149 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 148 | 0.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 148 | 0.2% |
| `java/util/HashMap.getNode` | JVM-Java | 126 | 0.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 103 | 0.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 101 | 0.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 95 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 95 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 93 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 91 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 5709)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 5709 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 4162 | 72.9% |
| phase: unclassified | 1358 | 23.8% |
| phase: main tick (unclassified) | 107 | 1.9% |
| phase: chunk system (off-main worker) | 33 | 0.6% |
| phase: network sync (ServerEntity) | 20 | 0.4% |
| phase: block entities (hoppers/furnaces) | 13 | 0.2% |
| phase: chunk tick | 11 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **5709** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1116 | 19.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1029 | 18.0% |
| `char[]_[k]` | other | 436 | 7.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 316 | 5.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 279 | 4.9% |
| `long[]_[i]` | other | 252 | 4.4% |
| `byte[]_[k]` | other | 193 | 3.4% |
| `java.lang.Object[]_[i]` | other | 185 | 3.2% |
| `java.util.ArrayList_[i]` | other | 181 | 3.2% |
| `byte[]_[i]` | other | 93 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 79 | 1.4% |
| `int[]_[i]` | other | 70 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 70 | 1.2% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f61fe9e1000_[i]` | other | 69 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 64 | 1.1% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 61 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 58 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.0% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 45 | 0.8% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 44 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 118294 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35553 | 30.05% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23282 | 19.68% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6561 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5596 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4734 | 4.00% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1065 | 0.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 979 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 473 | 0.40% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 246 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 232 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 218 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1116 | 19.5% |
| `net.minecraft.world.phys.AABB_[i]` | 1029 | 18.0% |
| `char[]_[k]` | 436 | 7.6% |
| `net.minecraft.core.BlockPos_[i]` | 316 | 5.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 279 | 4.9% |
| `long[]_[i]` | 252 | 4.4% |
| `byte[]_[k]` | 193 | 3.4% |
| `java.lang.Object[]_[i]` | 185 | 3.2% |
| `java.util.ArrayList_[i]` | 181 | 3.2% |
| `byte[]_[i]` | 93 | 1.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 23467 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147789..150879 (delta 3090, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99482->103019, minecraft:drowned 3574->4496, minecraft:zombie 3751->4609, minecraft:creeper 4584->5219, minecraft:husk 4537->5158, minecraft:spider 4184->4734, minecraft:skeleton 4433->4847, minecraft:chicken 3405->3433
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3090)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46692288 B)
- `wall-collapsed.txt` (6436204 B)
- `alloc-collapsed.txt` (2468379 B)
- `cpu-flamegraph.html` (295410 B)
- `server-stdout.log` (252128 B)
- `gc.log` (112759 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
