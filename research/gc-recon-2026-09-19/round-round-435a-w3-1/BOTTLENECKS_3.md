# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.704 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.5, 1.6, 1.9, 1.8, 2.2, 2.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:56:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6804217 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:58:40 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [149030, 150020, 151363]
- top entity types (max seen): minecraft:item×103285, minecraft:creeper×5192, minecraft:husk×5123, minecraft:spider×4867, minecraft:skeleton×4857, minecraft:zombie×4666, minecraft:drowned×4583, minecraft:sheep×3531, minecraft:chicken×3411, minecraft:cow×3371, minecraft:pig×3245, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/3m7NZ29V69
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **516** (Full GC: **9**)
- total pause: **21862.9 ms**, avg **42.37 ms**, max **2664.3 ms**
- heap high-water seen: **8347 MB** -> last-after: **5072 MB**
  - Young (Allocation Failure): 498
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 110859)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 39179 | 35.3% |
| kernel: other | 21438 | 19.3% |
| other | 12672 | 11.4% |
| chunk system (kernel) | 9312 | 8.4% |
| JDK collections | 7028 | 6.3% |
| fastutil collections | 5161 | 4.7% |
| moonrise/paper patches | 4427 | 4.0% |
| JIT stubs (vtable/itable) | 3454 | 3.1% |
| JDK invokes/VarHandle | 2981 | 2.7% |
| network (kernel) | 2288 | 2.1% |
| JDK other | 1981 | 1.8% |
| JVM internals (GC oop barriers) | 585 | 0.5% |
| vdso (clock) | 101 | 0.1% |
| block entities/hoppers (kernel) | 66 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 48 | 0.0% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61841 | 55.8% |
| phase: unclassified | 31793 | 28.7% |
| phase: main tick (unclassified) | 11283 | 10.2% |
| phase: chunk tick | 1995 | 1.8% |
| phase: network sync (ServerEntity) | 1586 | 1.4% |
| phase: chunk system (off-main worker) | 1038 | 0.9% |
| phase: block entities (hoppers/furnaces) | 787 | 0.7% |
| phase: random tick | 427 | 0.4% |
| phase: mob spawning | 107 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96918** (87.4%) · native/JVM-internal **13813** (12.5%) · other **128** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 8316 | 7.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3272 | 3.0% |
| `vtable stub` | native/JVM-internal | 2793 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2579 | 2.3% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2403 | 2.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1387 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1352 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1287 | 1.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1275 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1195 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1179 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1017 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 998 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 938 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 842 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 841 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 820 | 0.7% |
| `colpush_tick` | native/JVM-internal | 817 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 793 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 746 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 737 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 725 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 722 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 712 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 673 | 0.6% |
| `itable stub` | native/JVM-internal | 661 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 659 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 645 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 58016 | 94.7% |
| entities/mobs (kernel) | 1349 | 2.2% |
| kernel: other | 695 | 1.1% |
| chunk system (kernel) | 287 | 0.5% |
| JDK collections | 218 | 0.4% |
| fastutil collections | 160 | 0.3% |
| moonrise/paper patches | 131 | 0.2% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| JDK invokes/VarHandle | 96 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK other | 73 | 0.1% |
| bukkit api | 5 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58384 | 95.3% |
| phase: entity tick (AI/movement) | 2210 | 3.6% |
| phase: main tick (unclassified) | 440 | 0.7% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52360** (85.5%) · native/JVM-internal **8886** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49163 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.8% |
| `read` | native/JVM-internal | 1222 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 334 | 0.5% |
| `vtable stub` | native/JVM-internal | 112 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 40 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 36 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 33 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 32 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 32 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3396)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3396 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1916 | 56.4% |
| phase: entity tick (AI/movement) | 1146 | 33.7% |
| phase: main tick (unclassified) | 239 | 7.0% |
| phase: chunk system (off-main worker) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 24 | 0.7% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: random tick | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3396** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 534 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 446 | 13.1% |
| `char[]_[k]` | other | 394 | 11.6% |
| `byte[]_[k]` | other | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 127 | 3.7% |
| `java.lang.Object[]_[i]` | other | 112 | 3.3% |
| `long[]_[i]` | other | 107 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 100 | 2.9% |
| `byte[]_[i]` | other | 95 | 2.8% |
| `int[]_[i]` | other | 86 | 2.5% |
| `java.util.ArrayList_[i]` | other | 79 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 40 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 33 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `int[]_[k]` | other | 32 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fdbb182eb78_[i]` | other | 26 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110859 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 26752 | 24.13% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7054 | 6.36% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5630 | 5.08% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4837 | 4.36% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1060 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 898 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 404 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 356 | 0.32% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 315 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 251 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 245 | 0.22% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 534 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | 446 | 13.1% |
| `char[]_[k]` | 394 | 11.6% |
| `byte[]_[k]` | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 127 | 3.7% |
| `java.lang.Object[]_[i]` | 112 | 3.3% |
| `long[]_[i]` | 107 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 100 | 2.9% |
| `byte[]_[i]` | 95 | 2.8% |
| `int[]_[i]` | 86 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 516 pauses / total 21863 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148253..151363 (delta 3110, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99600->103285, minecraft:drowned 3489->4583, minecraft:zombie 3660->4666, minecraft:creeper 4576->5192, minecraft:husk 4517->5123, minecraft:spider 4272->4867, minecraft:skeleton 4389->4857, minecraft:chicken 3378->3411
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3110)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (61949578 B)
- `wall-collapsed.txt` (3538236 B)
- `alloc-collapsed.txt` (1903989 B)
- `cpu-flamegraph.html` (297763 B)
- `server-stdout.log` (368190 B)
- `gc.log` (451497 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
