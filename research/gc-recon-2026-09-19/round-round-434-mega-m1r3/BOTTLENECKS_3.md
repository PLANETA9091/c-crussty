# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.83 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [16.8, 1.9, 2.2, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **381.1ms** / min 312.25ms / max **479.63ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:01:53Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6564692 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:04:06 INFO]: [crussty-plugin] [cruss | 312.25 | — | — | — | 479.63 | 381.1 |

- entity totals seen: [150413, 152711, 153755]
- top entity types (max seen): minecraft:item×106917, minecraft:husk×5489, minecraft:creeper×5054, minecraft:skeleton×4745, minecraft:zombie×4617, minecraft:drowned×4558, minecraft:spider×4438, minecraft:sheep×3517, minecraft:chicken×3387, minecraft:cow×3353, minecraft:pig×3196, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Wjp5e8JOLI
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **111** (Full GC: **9**)
- total pause: **18808.8 ms**, avg **169.45 ms**, max **2422.3 ms**
- heap high-water seen: **7464 MB** -> last-after: **3613 MB**
  - Young (Allocation Failure): 90
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 102135)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32150 | 31.5% |
| kernel: other | 22359 | 21.9% |
| other | 11519 | 11.3% |
| chunk system (kernel) | 8568 | 8.4% |
| JDK collections | 6101 | 6.0% |
| moonrise/paper patches | 5346 | 5.2% |
| fastutil collections | 4880 | 4.8% |
| JIT stubs (vtable/itable) | 3666 | 3.6% |
| network (kernel) | 2786 | 2.7% |
| JDK invokes/VarHandle | 2524 | 2.5% |
| JDK other | 1766 | 1.7% |
| vdso (clock) | 125 | 0.1% |
| bukkit api | 114 | 0.1% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| redstone (kernel) | 53 | 0.1% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49681 | 48.6% |
| phase: unclassified | 31421 | 30.8% |
| phase: main tick (unclassified) | 12926 | 12.7% |
| phase: chunk tick | 2776 | 2.7% |
| phase: network sync (ServerEntity) | 2236 | 2.2% |
| phase: chunk system (off-main worker) | 1186 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1080 | 1.1% |
| phase: random tick | 538 | 0.5% |
| phase: mob spawning | 291 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90007** (88.1%) · native/JVM-internal **12024** (11.8%) · other **104** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4037 | 4.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3453 | 3.4% |
| `vtable stub` | native/JVM-internal | 3115 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2623 | 2.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1515 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1498 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1484 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1462 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1292 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1269 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1217 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1160 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1118 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1075 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1058 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1029 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 992 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 933 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 906 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 894 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 893 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 881 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 862 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 841 | 0.8% |
| `colpush_tick` | native/JVM-internal | 830 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 789 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 788 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 781 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 781 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 750 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 722 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 693 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 679 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 677 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 671 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 669 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 665 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 658 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 620 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 58373 | 95.3% |
| entities/mobs (kernel) | 993 | 1.6% |
| kernel: other | 708 | 1.2% |
| chunk system (kernel) | 240 | 0.4% |
| JIT stubs (vtable/itable) | 186 | 0.3% |
| JDK collections | 173 | 0.3% |
| moonrise/paper patches | 159 | 0.3% |
| fastutil collections | 132 | 0.2% |
| network (kernel) | 126 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58726 | 95.9% |
| phase: entity tick (AI/movement) | 1775 | 2.9% |
| phase: main tick (unclassified) | 461 | 0.8% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 82 | 0.1% |
| phase: block entities (hoppers/furnaces) | 44 | 0.1% |
| phase: chunk system (off-main worker) | 31 | 0.1% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52332** (85.4%) · native/JVM-internal **8916** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49561 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `vtable stub` | native/JVM-internal | 169 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 66 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007f138786ead8.accept` | JVM-Java | 39 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3288)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3288 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1932 | 58.8% |
| phase: entity tick (AI/movement) | 986 | 30.0% |
| phase: main tick (unclassified) | 241 | 7.3% |
| phase: chunk system (off-main worker) | 51 | 1.6% |
| phase: block entities (hoppers/furnaces) | 31 | 0.9% |
| phase: network sync (ServerEntity) | 25 | 0.8% |
| phase: mob spawning | 9 | 0.3% |
| phase: random tick | 7 | 0.2% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3288** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 487 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 445 | 13.5% |
| `char[]_[k]` | other | 342 | 10.4% |
| `byte[]_[k]` | other | 204 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 136 | 4.1% |
| `long[]_[i]` | other | 116 | 3.5% |
| `java.util.ArrayList_[i]` | other | 116 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 110 | 3.3% |
| `java.lang.Object[]_[i]` | other | 106 | 3.2% |
| `byte[]_[i]` | other | 98 | 3.0% |
| `int[]_[i]` | other | 91 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.7% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.9% |
| `int[]_[k]` | other | 29 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f138791d0c8_[i]` | other | 28 | 0.9% |
| `java.lang.String_[i]` | other | 28 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 102135 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19171 | 18.77% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6099 | 5.97% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5027 | 4.92% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3750 | 3.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1294 | 1.27% |
| `net/minecraft/world/entity/ai/Brain.tick` | 595 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 346 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 278 | 0.27% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 246 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 219 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 152 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 92 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 487 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 445 | 13.5% |
| `char[]_[k]` | 342 | 10.4% |
| `byte[]_[k]` | 204 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 136 | 4.1% |
| `long[]_[i]` | 116 | 3.5% |
| `java.util.ArrayList_[i]` | 116 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 110 | 3.3% |
| `java.lang.Object[]_[i]` | 106 | 3.2% |
| `byte[]_[i]` | 98 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 111 pauses / total 18809 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148254..153755 (delta 5501, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99963->106917, minecraft:husk 4549->5489, minecraft:drowned 3640->4558, minecraft:zombie 3841->4617, minecraft:skeleton 4195->4745, minecraft:creeper 4539->5054, minecraft:chicken 3044->3387, minecraft:spider 4098->4438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5501)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49089966 B)
- `wall-collapsed.txt` (2917254 B)
- `alloc-collapsed.txt` (1786742 B)
- `cpu-flamegraph.html` (275598 B)
- `server-stdout.log` (329083 B)
- `gc.log` (105788 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
