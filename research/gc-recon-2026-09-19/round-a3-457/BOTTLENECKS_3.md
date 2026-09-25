# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.415 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.1, 1.7, 1.9, 2.2, 2.6, 2.5]
- spark tick-monitor MSPT: avg **415.11ms** / min 355.24ms / max **564.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T11:56:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6977973 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 355.24 | — | — | — | 564.89 | 415.11 |

- entity totals seen: [148978, 150138, 151384]
- top entity types (max seen): minecraft:item×103167, minecraft:creeper×5267, minecraft:husk×5141, minecraft:skeleton×4872, minecraft:spider×4853, minecraft:zombie×4686, minecraft:drowned×4525, minecraft:sheep×3528, minecraft:chicken×3474, minecraft:cow×3362, minecraft:pig×3270, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/pf8ZCzTJuh
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **10**)
- total pause: **24076.6 ms**, avg **198.98 ms**, max **2485.5 ms**
- heap high-water seen: **7531 MB** -> last-after: **5579 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117010)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28809 | 24.6% |
| kernel: other | 27515 | 23.5% |
| other | 15380 | 13.1% |
| moonrise/paper patches | 10057 | 8.6% |
| chunk system (kernel) | 9323 | 8.0% |
| fastutil collections | 7266 | 6.2% |
| JDK collections | 5822 | 5.0% |
| JIT stubs (vtable/itable) | 3862 | 3.3% |
| network (kernel) | 3292 | 2.8% |
| JDK invokes/VarHandle | 2547 | 2.2% |
| JDK other | 2024 | 1.7% |
| JVM internals (GC oop barriers) | 571 | 0.5% |
| vdso (clock) | 232 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| redstone (kernel) | 51 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 8 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93320 | 79.8% |
| phase: unclassified | 13755 | 11.8% |
| phase: main tick (unclassified) | 3620 | 3.1% |
| phase: chunk tick | 2162 | 1.8% |
| phase: network sync (ServerEntity) | 1791 | 1.5% |
| phase: chunk system (off-main worker) | 1152 | 1.0% |
| phase: block entities (hoppers/furnaces) | 642 | 0.5% |
| phase: random tick | 428 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100870** (86.2%) · native/JVM-internal **16061** (13.7%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4300 | 3.7% |
| `vtable stub` | native/JVM-internal | 3149 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3123 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2502 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1970 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1904 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1707 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1639 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1604 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1541 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1537 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1533 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1459 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1402 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1240 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1206 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1185 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1097 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 994 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 933 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 883 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 878 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 867 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 863 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 852 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 849 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 826 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 762 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 750 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 748 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 736 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 709 | 0.6% |
| `itable stub` | native/JVM-internal | 709 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 682 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 672 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 669 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 661 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57845 | 94.4% |
| entities/mobs (kernel) | 1018 | 1.7% |
| kernel: other | 900 | 1.5% |
| moonrise/paper patches | 341 | 0.6% |
| chunk system (kernel) | 290 | 0.5% |
| fastutil collections | 233 | 0.4% |
| JDK collections | 186 | 0.3% |
| JIT stubs (vtable/itable) | 145 | 0.2% |
| network (kernel) | 121 | 0.2% |
| JDK invokes/VarHandle | 101 | 0.2% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57642 | 94.1% |
| phase: entity tick (AI/movement) | 3147 | 5.1% |
| phase: main tick (unclassified) | 214 | 0.3% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52410** (85.6%) · native/JVM-internal **8847** (14.4%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49049 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 129 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 60 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3603)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3603 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2021 | 56.1% |
| phase: unclassified | 1402 | 38.9% |
| phase: main tick (unclassified) | 81 | 2.2% |
| phase: chunk system (off-main worker) | 44 | 1.2% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: chunk tick | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3603** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 535 | 14.8% |
| `char[]_[k]` | other | 435 | 12.1% |
| `byte[]_[k]` | other | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.6% |
| `java.util.ArrayList_[i]` | other | 131 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 127 | 3.5% |
| `long[]_[i]` | other | 105 | 2.9% |
| `java.lang.Object[]_[i]` | other | 95 | 2.6% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `int[]_[i]` | other | 71 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 49 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 41 | 1.1% |
| `net.minecraft.core.SectionPos_[i]` | other | 36 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f20659f0000_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117010 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35194 | 30.08% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22403 | 19.15% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6366 | 5.44% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5400 | 4.61% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4589 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1135 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 889 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 405 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 272 | 0.23% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 249 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 230 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 229 | 0.20% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 535 | 14.8% |
| `char[]_[k]` | 435 | 12.1% |
| `byte[]_[k]` | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.6% |
| `java.util.ArrayList_[i]` | 131 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 127 | 3.5% |
| `long[]_[i]` | 105 | 2.9% |
| `java.lang.Object[]_[i]` | 95 | 2.6% |
| `byte[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 24077 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148060..151384 (delta 3324, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99486->103167, minecraft:drowned 3430->4525, minecraft:zombie 3666->4686, minecraft:creeper 4565->5267, minecraft:husk 4503->5141, minecraft:spider 4215->4853, minecraft:skeleton 4394->4872, minecraft:chicken 3447->3474
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3324)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56103502 B)
- `wall-collapsed.txt` (3786584 B)
- `alloc-collapsed.txt` (2008119 B)
- `cpu-flamegraph.html` (308508 B)
- `server-stdout.log` (250446 B)
- `gc.log` (115346 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
