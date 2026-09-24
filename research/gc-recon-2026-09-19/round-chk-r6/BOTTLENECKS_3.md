# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.991 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.1, 2.3, 2.7, 2.7]
- spark tick-monitor MSPT: avg **392.71ms** / min 328.15ms / max **514.34ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:55:11Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6720684 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:57:35 INFO]: [crussty-plugin] [cruss | 328.15 | — | — | — | 514.34 | 392.71 |

- entity totals seen: [149887, 151981, 153737]
- top entity types (max seen): minecraft:item×106809, minecraft:husk×5496, minecraft:creeper×4958, minecraft:skeleton×4767, minecraft:zombie×4636, minecraft:drowned×4573, minecraft:spider×4491, minecraft:sheep×3519, minecraft:chicken×3388, minecraft:cow×3359, minecraft:pig×3197, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/hCKGqTjhTK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **110** (Full GC: **9**)
- total pause: **21298.1 ms**, avg **193.62 ms**, max **2856.2 ms**
- heap high-water seen: **7359 MB** -> last-after: **4042 MB**
  - Young (Allocation Failure): 92
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103300)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33470 | 32.4% |
| kernel: other | 20487 | 19.8% |
| other | 11603 | 11.2% |
| chunk system (kernel) | 8308 | 8.0% |
| JDK collections | 7602 | 7.4% |
| moonrise/paper patches | 5881 | 5.7% |
| fastutil collections | 5247 | 5.1% |
| network (kernel) | 3223 | 3.1% |
| JIT stubs (vtable/itable) | 2706 | 2.6% |
| JDK invokes/VarHandle | 2676 | 2.6% |
| JDK other | 1603 | 1.6% |
| vdso (clock) | 138 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| craftbukkit glue | 80 | 0.1% |
| bukkit api | 79 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48925 | 47.4% |
| phase: unclassified | 33315 | 32.3% |
| phase: main tick (unclassified) | 12825 | 12.4% |
| phase: chunk tick | 2561 | 2.5% |
| phase: network sync (ServerEntity) | 2514 | 2.4% |
| phase: chunk system (off-main worker) | 1283 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1029 | 1.0% |
| phase: random tick | 529 | 0.5% |
| phase: mob spawning | 317 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91404** (88.5%) · native/JVM-internal **11795** (11.4%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4475 | 4.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3457 | 3.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3426 | 3.3% |
| `vtable stub` | native/JVM-internal | 2257 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1708 | 1.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1679 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1658 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1554 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1375 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1321 | 1.3% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1268 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1204 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1155 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1146 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1144 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1113 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1076 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1071 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1070 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 979 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 975 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 952 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 938 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 936 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 908 | 0.9% |
| `colpush_tick` | native/JVM-internal | 908 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 887 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 863 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 818 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 791 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 790 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 780 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 727 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 688 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 687 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 680 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 678 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 642 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64852)

| bucket | self-time samples | share |
|---|---|---|
| other | 61985 | 95.6% |
| entities/mobs (kernel) | 1023 | 1.6% |
| kernel: other | 595 | 0.9% |
| JDK collections | 225 | 0.3% |
| chunk system (kernel) | 212 | 0.3% |
| moonrise/paper patches | 178 | 0.3% |
| fastutil collections | 167 | 0.3% |
| JIT stubs (vtable/itable) | 149 | 0.2% |
| network (kernel) | 123 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JVM internals (GC oop barriers) | 64 | 0.1% |
| JDK other | 47 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62385 | 96.2% |
| phase: entity tick (AI/movement) | 1722 | 2.7% |
| phase: main tick (unclassified) | 474 | 0.7% |
| phase: chunk tick | 94 | 0.1% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 36 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55432** (85.5%) · native/JVM-internal **9411** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52720 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.3% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 363 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `vtable stub` | native/JVM-internal | 128 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 45 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 42 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3613)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3613 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2116 | 58.6% |
| phase: entity tick (AI/movement) | 1111 | 30.8% |
| phase: main tick (unclassified) | 279 | 7.7% |
| phase: chunk system (off-main worker) | 38 | 1.1% |
| phase: network sync (ServerEntity) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 16 | 0.4% |
| phase: mob spawning | 13 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3613** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 572 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 424 | 11.7% |
| `char[]_[k]` | other | 424 | 11.7% |
| `byte[]_[k]` | other | 222 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 150 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.8% |
| `long[]_[i]` | other | 137 | 3.8% |
| `java.lang.Object[]_[i]` | other | 110 | 3.0% |
| `java.util.ArrayList_[i]` | other | 96 | 2.7% |
| `byte[]_[i]` | other | 94 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 69 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `int[]_[k]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6b5fa06c88_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103300 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18994 | 18.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5983 | 5.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4973 | 4.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3854 | 3.73% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1044 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 565 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 393 | 0.38% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 367 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 321 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 300 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 267 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 92 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 572 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | 424 | 11.7% |
| `char[]_[k]` | 424 | 11.7% |
| `byte[]_[k]` | 222 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 150 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.8% |
| `long[]_[i]` | 137 | 3.8% |
| `java.lang.Object[]_[i]` | 110 | 3.0% |
| `java.util.ArrayList_[i]` | 96 | 2.7% |
| `byte[]_[i]` | 94 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 110 pauses / total 21298 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148268..153737 (delta 5469, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99898->106809, minecraft:drowned 3588->4573, minecraft:zombie 3659->4636, minecraft:husk 4560->5496, minecraft:skeleton 4269->4767, minecraft:creeper 4513->4958, minecraft:spider 4092->4491, minecraft:sheep 3174->3519
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5469)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45860831 B)
- `wall-collapsed.txt` (2982919 B)
- `alloc-collapsed.txt` (1929523 B)
- `cpu-flamegraph.html` (273119 B)
- `server-stdout.log` (329992 B)
- `gc.log` (104929 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
