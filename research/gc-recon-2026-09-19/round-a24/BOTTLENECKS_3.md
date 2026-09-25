# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.555 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.4, 1.7, 2.0, 2.3, 2.5, 2.7]
- spark tick-monitor MSPT: avg **413.5ms** / min 348.35ms / max **613.57ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:45:31Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6601880 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- fluid_dirty_ledger: 0 (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)
- fluid_bitmask: 0 (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
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

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 348.35 | — | — | — | 613.57 | 413.5 |

- entity totals seen: [148950, 150339, 151414]
- top entity types (max seen): minecraft:item×103327, minecraft:creeper×5315, minecraft:husk×5135, minecraft:spider×4870, minecraft:skeleton×4861, minecraft:zombie×4622, minecraft:drowned×4540, minecraft:sheep×3521, minecraft:chicken×3398, minecraft:cow×3378, minecraft:pig×3230, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/MzjsWcMzIg
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **8**)
- total pause: **23184.7 ms**, avg **199.87 ms**, max **2565.2 ms**
- heap high-water seen: **7617 MB** -> last-after: **3730 MB**
  - Young (Allocation Failure): 99
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115580)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27619 | 23.9% |
| kernel: other | 26553 | 23.0% |
| other | 15160 | 13.1% |
| moonrise/paper patches | 10485 | 9.1% |
| chunk system (kernel) | 10448 | 9.0% |
| fastutil collections | 6972 | 6.0% |
| JDK collections | 6256 | 5.4% |
| network (kernel) | 3584 | 3.1% |
| JIT stubs (vtable/itable) | 3003 | 2.6% |
| JDK invokes/VarHandle | 2446 | 2.1% |
| JDK other | 1902 | 1.6% |
| JVM internals (GC oop barriers) | 564 | 0.5% |
| vdso (clock) | 273 | 0.2% |
| block entities/hoppers (kernel) | 104 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| bukkit api | 56 | 0.0% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90013 | 77.9% |
| phase: unclassified | 14862 | 12.9% |
| phase: main tick (unclassified) | 3861 | 3.3% |
| phase: chunk tick | 2248 | 1.9% |
| phase: network sync (ServerEntity) | 2063 | 1.8% |
| phase: chunk system (off-main worker) | 1223 | 1.1% |
| phase: block entities (hoppers/furnaces) | 730 | 0.6% |
| phase: random tick | 433 | 0.4% |
| phase: mob spawning | 141 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99431** (86.0%) · native/JVM-internal **16074** (13.9%) · other **75** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5201 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4033 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2426 | 2.1% |
| `vtable stub` | native/JVM-internal | 2398 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2358 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1995 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1994 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1814 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1759 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1552 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1485 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1462 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1460 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1352 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1249 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1224 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1130 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1091 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1073 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1054 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1015 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1013 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 997 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 993 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 992 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 970 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 931 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 912 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 903 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 806 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 764 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 689 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 677 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 676 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 630 | 0.5% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 626 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57955 | 94.6% |
| entities/mobs (kernel) | 959 | 1.6% |
| kernel: other | 799 | 1.3% |
| moonrise/paper patches | 358 | 0.6% |
| chunk system (kernel) | 327 | 0.5% |
| fastutil collections | 259 | 0.4% |
| JDK collections | 208 | 0.3% |
| JIT stubs (vtable/itable) | 102 | 0.2% |
| network (kernel) | 97 | 0.2% |
| JDK invokes/VarHandle | 95 | 0.2% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57771 | 94.3% |
| phase: entity tick (AI/movement) | 2994 | 4.9% |
| phase: main tick (unclassified) | 216 | 0.4% |
| phase: chunk tick | 86 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52296** (85.4%) · native/JVM-internal **8948** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49013 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4746 | 7.7% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 160 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 127 | 0.2% |
| `syscall` | native/JVM-internal | 100 | 0.2% |
| `vtable stub` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 76 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 61 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3596)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3596 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1967 | 54.7% |
| phase: unclassified | 1448 | 40.3% |
| phase: main tick (unclassified) | 81 | 2.3% |
| phase: chunk system (off-main worker) | 41 | 1.1% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: chunk tick | 13 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3596** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 506 | 14.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 492 | 13.7% |
| `char[]_[k]` | other | 432 | 12.0% |
| `byte[]_[k]` | other | 188 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 172 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 4.5% |
| `java.util.ArrayList_[i]` | other | 139 | 3.9% |
| `long[]_[i]` | other | 124 | 3.4% |
| `java.lang.Object[]_[i]` | other | 123 | 3.4% |
| `byte[]_[i]` | other | 92 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 82 | 2.3% |
| `int[]_[i]` | other | 74 | 2.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f5fd5a15960_[i]` | other | 32 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115580 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33370 | 28.87% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22123 | 19.14% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6292 | 5.44% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5266 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4335 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 949 | 0.82% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 447 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 227 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 506 | 14.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 492 | 13.7% |
| `char[]_[k]` | 432 | 12.0% |
| `byte[]_[k]` | 188 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | 172 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 4.5% |
| `java.util.ArrayList_[i]` | 139 | 3.9% |
| `long[]_[i]` | 124 | 3.4% |
| `java.lang.Object[]_[i]` | 123 | 3.4% |
| `byte[]_[i]` | 92 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 23185 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148151..151414 (delta 3263, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99721->103327, minecraft:drowned 3502->4540, minecraft:zombie 3596->4622, minecraft:creeper 4588->5315, minecraft:husk 4504->5135, minecraft:spider 4258->4870, minecraft:skeleton 4405->4861, minecraft:chicken 3371->3398
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3263)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54313750 B)
- `wall-collapsed.txt` (3611339 B)
- `alloc-collapsed.txt` (2080624 B)
- `cpu-flamegraph.html` (285718 B)
- `server-stdout.log` (255974 B)
- `gc.log` (109250 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
