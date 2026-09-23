# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.236 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.7, 1.9, 2.1, 2.5, 2.5]
- spark tick-monitor MSPT: avg **409.39ms** / min 354.5ms / max **520.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T15:52:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6853156 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 354.5 | — | — | — | 520.68 | 409.39 |

- entity totals seen: [148947, 150120, 151452]
- top entity types (max seen): minecraft:item×103254, minecraft:creeper×5245, minecraft:husk×5218, minecraft:skeleton×4863, minecraft:spider×4859, minecraft:zombie×4712, minecraft:drowned×4586, minecraft:sheep×3513, minecraft:chicken×3422, minecraft:cow×3374, minecraft:pig×3256, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/cfAQpyVMxi
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **21272.9 ms**, avg **174.37 ms**, max **2501.4 ms**
- heap high-water seen: **7366 MB** -> last-after: **4088 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116343)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29773 | 25.6% |
| kernel: other | 27692 | 23.8% |
| other | 13927 | 12.0% |
| chunk system (kernel) | 9936 | 8.5% |
| moonrise/paper patches | 9855 | 8.5% |
| fastutil collections | 6757 | 5.8% |
| JDK collections | 6157 | 5.3% |
| JIT stubs (vtable/itable) | 3674 | 3.2% |
| network (kernel) | 3270 | 2.8% |
| JDK invokes/VarHandle | 2735 | 2.4% |
| JDK other | 2052 | 1.8% |
| vdso (clock) | 226 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 73 | 0.1% |
| redstone (kernel) | 54 | 0.0% |
| craftbukkit glue | 50 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94285 | 81.0% |
| phase: unclassified | 12068 | 10.4% |
| phase: main tick (unclassified) | 3796 | 3.3% |
| phase: chunk tick | 2017 | 1.7% |
| phase: network sync (ServerEntity) | 1875 | 1.6% |
| phase: chunk system (off-main worker) | 1122 | 1.0% |
| phase: block entities (hoppers/furnaces) | 646 | 0.6% |
| phase: random tick | 423 | 0.4% |
| phase: mob spawning | 110 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101984** (87.7%) · native/JVM-internal **14291** (12.3%) · other **68** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4416 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3286 | 2.8% |
| `vtable stub` | native/JVM-internal | 2950 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2598 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1952 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1911 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1843 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1778 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1626 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1565 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1526 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1445 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1378 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1364 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1289 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1147 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1086 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1031 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 976 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 968 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 948 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 944 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 904 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 869 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 851 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 841 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 791 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 788 | 0.7% |
| `itable stub` | native/JVM-internal | 721 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 720 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 716 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 648 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 624 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61201)

| bucket | self-time samples | share |
|---|---|---|
| other | 57938 | 94.7% |
| entities/mobs (kernel) | 970 | 1.6% |
| kernel: other | 865 | 1.4% |
| moonrise/paper patches | 303 | 0.5% |
| chunk system (kernel) | 290 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 122 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 67 | 0.1% |
| JVM internals (GC oop barriers) | 52 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| block entities/hoppers (kernel) | 8 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57768 | 94.4% |
| phase: entity tick (AI/movement) | 2962 | 4.8% |
| phase: main tick (unclassified) | 204 | 0.3% |
| phase: chunk tick | 127 | 0.2% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: chunk system (off-main worker) | 27 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51861** (84.7%) · native/JVM-internal **9332** (15.2%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48693 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 410 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 139 | 0.2% |
| `vtable stub` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3689)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3689 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2056 | 55.7% |
| phase: unclassified | 1458 | 39.5% |
| phase: main tick (unclassified) | 93 | 2.5% |
| phase: chunk system (off-main worker) | 53 | 1.4% |
| phase: network sync (ServerEntity) | 14 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3689** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 543 | 14.7% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 524 | 14.2% |
| `char[]_[k]` | other | 428 | 11.6% |
| `byte[]_[k]` | other | 201 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 150 | 4.1% |
| `long[]_[i]` | other | 109 | 3.0% |
| `java.util.ArrayList_[i]` | other | 109 | 3.0% |
| `byte[]_[i]` | other | 108 | 2.9% |
| `java.lang.Object[]_[i]` | other | 96 | 2.6% |
| `int[]_[i]` | other | 88 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 67 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 66 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f070982d9f8_[i]` | other | 34 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f07099f4000_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116343 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35173 | 30.23% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22595 | 19.42% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6641 | 5.71% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5621 | 4.83% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4565 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1209 | 1.04% |
| `net/minecraft/world/entity/ai/Brain.tick` | 904 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 433 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 247 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 215 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 188 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 543 | 14.7% |
| `net.minecraft.world.phys.Vec3_[i]` | 524 | 14.2% |
| `char[]_[k]` | 428 | 11.6% |
| `byte[]_[k]` | 201 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 150 | 4.1% |
| `long[]_[i]` | 109 | 3.0% |
| `java.util.ArrayList_[i]` | 109 | 3.0% |
| `byte[]_[i]` | 108 | 2.9% |
| `java.lang.Object[]_[i]` | 96 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 21273 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148068..151452 (delta 3384, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99506->103254, minecraft:drowned 3436->4586, minecraft:zombie 3635->4712, minecraft:creeper 4527->5245, minecraft:husk 4539->5218, minecraft:spider 4221->4859, minecraft:skeleton 4372->4863, minecraft:chicken 3391->3422
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3384)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56332610 B)
- `wall-collapsed.txt` (3559418 B)
- `alloc-collapsed.txt` (2174250 B)
- `cpu-flamegraph.html` (300286 B)
- `server-stdout.log` (252507 B)
- `gc.log` (115272 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
