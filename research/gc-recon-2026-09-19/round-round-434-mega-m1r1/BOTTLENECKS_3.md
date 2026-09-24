# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.056 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.9, 2.3, 2.7, 3.0, 3.0]
- spark tick-monitor MSPT: avg **332.84ms** / min 288.18ms / max **425.84ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:23:07Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8427292 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:25:25 INFO]: [crussty-plugin] [cruss | 288.18 | — | — | — | 425.84 | 332.84 |

- entity totals seen: [151093, 153471, 154113]
- top entity types (max seen): minecraft:item×107206, minecraft:husk×5461, minecraft:creeper×4993, minecraft:skeleton×4752, minecraft:zombie×4587, minecraft:drowned×4523, minecraft:spider×4398, minecraft:sheep×3521, minecraft:chicken×3421, minecraft:cow×3386, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/r4w5A3jOEP
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **22801.2 ms**, avg **185.38 ms**, max **2901.8 ms**
- heap high-water seen: **7399 MB** -> last-after: **4117 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105785)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31143 | 29.4% |
| kernel: other | 21732 | 20.5% |
| other | 13589 | 12.8% |
| chunk system (kernel) | 10633 | 10.1% |
| JDK collections | 6749 | 6.4% |
| moonrise/paper patches | 5807 | 5.5% |
| fastutil collections | 4779 | 4.5% |
| JIT stubs (vtable/itable) | 3169 | 3.0% |
| JDK invokes/VarHandle | 2928 | 2.8% |
| network (kernel) | 2558 | 2.4% |
| JDK other | 1686 | 1.6% |
| JVM internals (GC oop barriers) | 496 | 0.5% |
| vdso (clock) | 133 | 0.1% |
| block entities/hoppers (kernel) | 121 | 0.1% |
| bukkit api | 94 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 8 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50265 | 47.5% |
| phase: unclassified | 34984 | 33.1% |
| phase: main tick (unclassified) | 12381 | 11.7% |
| phase: chunk tick | 2775 | 2.6% |
| phase: network sync (ServerEntity) | 2355 | 2.2% |
| phase: chunk system (off-main worker) | 1145 | 1.1% |
| phase: block entities (hoppers/furnaces) | 930 | 0.9% |
| phase: random tick | 580 | 0.5% |
| phase: mob spawning | 370 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91228** (86.2%) · native/JVM-internal **14403** (13.6%) · other **154** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4926 | 4.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3582 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3060 | 2.9% |
| `vtable stub` | native/JVM-internal | 2719 | 2.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1850 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1786 | 1.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1588 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1495 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1439 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1414 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1289 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1173 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1173 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1170 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1134 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1067 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1048 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1043 | 1.0% |
| `colpush_tick` | native/JVM-internal | 929 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 922 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 910 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 908 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 897 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 893 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 827 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 810 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 785 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 781 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 754 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 746 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 720 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 702 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 673 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 668 | 0.6% |
| `net/minecraft/world/entity/Entity.setPosRaw` | JVM-Java | 645 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 608 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 58384 | 95.3% |
| entities/mobs (kernel) | 956 | 1.6% |
| kernel: other | 640 | 1.0% |
| chunk system (kernel) | 327 | 0.5% |
| JDK collections | 203 | 0.3% |
| fastutil collections | 168 | 0.3% |
| moonrise/paper patches | 165 | 0.3% |
| JIT stubs (vtable/itable) | 157 | 0.3% |
| JDK invokes/VarHandle | 94 | 0.2% |
| network (kernel) | 93 | 0.2% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58684 | 95.8% |
| phase: entity tick (AI/movement) | 1870 | 3.1% |
| phase: main tick (unclassified) | 398 | 0.6% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: block entities (hoppers/furnaces) | 50 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 16 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52340** (85.4%) · native/JVM-internal **8905** (14.5%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49557 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1209 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 154 | 0.3% |
| `vtable stub` | native/JVM-internal | 139 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3875)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3875 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2230 | 57.5% |
| phase: entity tick (AI/movement) | 1222 | 31.5% |
| phase: main tick (unclassified) | 299 | 7.7% |
| phase: chunk system (off-main worker) | 57 | 1.5% |
| phase: block entities (hoppers/furnaces) | 23 | 0.6% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3875** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 622 | 16.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 458 | 11.8% |
| `char[]_[k]` | other | 434 | 11.2% |
| `byte[]_[k]` | other | 208 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 148 | 3.8% |
| `long[]_[i]` | other | 146 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 117 | 3.0% |
| `java.lang.Object[]_[i]` | other | 112 | 2.9% |
| `java.util.ArrayList_[i]` | other | 92 | 2.4% |
| `byte[]_[i]` | other | 88 | 2.3% |
| `int[]_[i]` | other | 87 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 64 | 1.7% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 56 | 1.4% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f81e9830000_[i]` | other | 48 | 1.2% |
| `net.minecraft.core.SectionPos_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 39 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 37 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105785 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19396 | 18.34% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5953 | 5.63% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5267 | 4.98% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3890 | 3.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1296 | 1.23% |
| `net/minecraft/world/entity/ai/Brain.tick` | 644 | 0.61% |
| `net/minecraft/world/entity/npc/Villager.tick` | 369 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 295 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 273 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 160 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 94 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 622 | 16.1% |
| `net.minecraft.world.phys.AABB_[i]` | 458 | 11.8% |
| `char[]_[k]` | 434 | 11.2% |
| `byte[]_[k]` | 208 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 148 | 3.8% |
| `long[]_[i]` | 146 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 117 | 3.0% |
| `java.lang.Object[]_[i]` | 112 | 2.9% |
| `java.util.ArrayList_[i]` | 92 | 2.4% |
| `byte[]_[i]` | 88 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 22801 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148217..154113 (delta 5896, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99975->107206, minecraft:drowned 3478->4523, minecraft:zombie 3628->4587, minecraft:husk 4592->5461, minecraft:skeleton 4148->4752, minecraft:creeper 4633->4993, minecraft:sheep 3183->3521, minecraft:spider 4103->4398
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5896)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50314319 B)
- `wall-collapsed.txt` (2972546 B)
- `alloc-collapsed.txt` (1898017 B)
- `cpu-flamegraph.html` (261611 B)
- `server-stdout.log` (329471 B)
- `gc.log` (116121 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
