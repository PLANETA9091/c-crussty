# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.681 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.7, 2.1, 1.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **389.28ms** / min 336.29ms / max **562.85ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T12:37:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6708829 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 336.29 | — | — | — | 562.85 | 389.28 |

- entity totals seen: [148962, 150489, 151389]
- top entity types (max seen): minecraft:item×103242, minecraft:creeper×5209, minecraft:husk×5209, minecraft:spider×4878, minecraft:skeleton×4849, minecraft:zombie×4718, minecraft:drowned×4586, minecraft:sheep×3512, minecraft:chicken×3430, minecraft:cow×3381, minecraft:pig×3260, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VgC2WtEYaH
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **10**)
- total pause: **24822.0 ms**, avg **208.59 ms**, max **2495.8 ms**
- heap high-water seen: **7411 MB** -> last-after: **5230 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116132)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27340 | 23.5% |
| entities/mobs (kernel) | 27302 | 23.5% |
| other | 15575 | 13.4% |
| chunk system (kernel) | 10379 | 8.9% |
| moonrise/paper patches | 10182 | 8.8% |
| fastutil collections | 6702 | 5.8% |
| JDK collections | 6438 | 5.5% |
| network (kernel) | 3667 | 3.2% |
| JIT stubs (vtable/itable) | 2786 | 2.4% |
| JDK invokes/VarHandle | 2610 | 2.2% |
| JDK other | 1976 | 1.7% |
| JVM internals (GC oop barriers) | 604 | 0.5% |
| vdso (clock) | 256 | 0.2% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| redstone (kernel) | 67 | 0.1% |
| bukkit api | 66 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90696 | 78.1% |
| phase: unclassified | 14956 | 12.9% |
| phase: main tick (unclassified) | 3751 | 3.2% |
| phase: chunk tick | 2212 | 1.9% |
| phase: network sync (ServerEntity) | 2029 | 1.7% |
| phase: chunk system (off-main worker) | 1174 | 1.0% |
| phase: block entities (hoppers/furnaces) | 740 | 0.6% |
| phase: random tick | 450 | 0.4% |
| phase: mob spawning | 122 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100021** (86.1%) · native/JVM-internal **16035** (13.8%) · other **76** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5272 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4054 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2401 | 2.1% |
| `vtable stub` | native/JVM-internal | 2159 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2140 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2064 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1987 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1913 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1905 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1814 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1560 | 1.3% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efdcf9e44e8.accept` | JVM-Java | 1507 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1496 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1494 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1269 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1175 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1153 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1138 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1085 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1084 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1081 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1026 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 959 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 959 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 941 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 921 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 921 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 892 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 882 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 880 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 864 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 767 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 721 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 715 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 667 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 660 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 660 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61245)

| bucket | self-time samples | share |
|---|---|---|
| other | 57932 | 94.6% |
| entities/mobs (kernel) | 904 | 1.5% |
| kernel: other | 886 | 1.4% |
| moonrise/paper patches | 347 | 0.6% |
| chunk system (kernel) | 333 | 0.5% |
| fastutil collections | 232 | 0.4% |
| JDK collections | 202 | 0.3% |
| network (kernel) | 135 | 0.2% |
| JIT stubs (vtable/itable) | 108 | 0.2% |
| JDK other | 79 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57755 | 94.3% |
| phase: entity tick (AI/movement) | 3038 | 5.0% |
| phase: main tick (unclassified) | 203 | 0.3% |
| phase: chunk tick | 104 | 0.2% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52326** (85.4%) · native/JVM-internal **8913** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49042 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 169 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 108 | 0.2% |
| `vtable stub` | native/JVM-internal | 90 | 0.1% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 64 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 63 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efdcf9e44e8.accept` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 58 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 52 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3836)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3836 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2145 | 55.9% |
| phase: unclassified | 1529 | 39.9% |
| phase: main tick (unclassified) | 86 | 2.2% |
| phase: chunk system (off-main worker) | 34 | 0.9% |
| phase: network sync (ServerEntity) | 21 | 0.5% |
| phase: chunk tick | 7 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3836** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 552 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 535 | 13.9% |
| `char[]_[k]` | other | 435 | 11.3% |
| `byte[]_[k]` | other | 221 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 186 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 162 | 4.2% |
| `java.util.ArrayList_[i]` | other | 148 | 3.9% |
| `long[]_[i]` | other | 128 | 3.3% |
| `java.lang.Object[]_[i]` | other | 128 | 3.3% |
| `byte[]_[i]` | other | 80 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.0% |
| `int[]_[i]` | other | 59 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007efdcf83c6c8_[i]` | other | 36 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 32 | 0.8% |
| `int[]_[k]` | other | 32 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116132 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34710 | 29.89% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21622 | 18.62% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6305 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5287 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4361 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1000 | 0.86% |
| `net/minecraft/world/entity/ai/Brain.tick` | 926 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 415 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 245 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 237 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 552 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 535 | 13.9% |
| `char[]_[k]` | 435 | 11.3% |
| `byte[]_[k]` | 221 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 186 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 162 | 4.2% |
| `java.util.ArrayList_[i]` | 148 | 3.9% |
| `long[]_[i]` | 128 | 3.3% |
| `java.lang.Object[]_[i]` | 128 | 3.3% |
| `byte[]_[i]` | 80 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 24822 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148097..151389 (delta 3292, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99483->103242, minecraft:drowned 3494->4586, minecraft:zombie 3719->4718, minecraft:husk 4528->5209, minecraft:creeper 4533->5209, minecraft:spider 4227->4878, minecraft:skeleton 4461->4849, minecraft:chicken 3393->3430
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3292)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54160779 B)
- `wall-collapsed.txt` (3618323 B)
- `alloc-collapsed.txt` (2104581 B)
- `cpu-flamegraph.html` (298319 B)
- `server-stdout.log` (245230 B)
- `gc.log` (113611 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
