# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.921 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [25.8, 2.4, 2.8, 3.3, 3.3, 3.4]
- spark tick-monitor MSPT: avg **374.84ms** / min 265.12ms / max **494.84ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T05:33:06Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8899646 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [05:35:00 INFO]: [crussty-plugin] [cruss | 319.4 | — | — | — | 494.84 | 374.84 |

- entity totals seen: [150989, 153548, 154127]
- top entity types (max seen): minecraft:item×107388, minecraft:husk×5496, minecraft:creeper×5051, minecraft:skeleton×4828, minecraft:zombie×4586, minecraft:drowned×4532, minecraft:spider×4413, minecraft:sheep×3537, minecraft:chicken×3425, minecraft:cow×3318, minecraft:pig×3178, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/cz2RYQGvqx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **142** (Full GC: **10**)
- total pause: **19310.2 ms**, avg **135.99 ms**, max **2189.2 ms**
- heap high-water seen: **7834 MB** -> last-after: **3849 MB**
  - Young (Allocation Failure): 120
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 104704)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34081 | 32.5% |
| kernel: other | 20375 | 19.5% |
| other | 11843 | 11.3% |
| chunk system (kernel) | 8265 | 7.9% |
| JDK collections | 7245 | 6.9% |
| moonrise/paper patches | 5505 | 5.3% |
| fastutil collections | 4969 | 4.7% |
| network (kernel) | 3610 | 3.4% |
| JIT stubs (vtable/itable) | 3205 | 3.1% |
| JDK other | 2338 | 2.2% |
| JDK invokes/VarHandle | 2293 | 2.2% |
| JVM internals (GC oop barriers) | 470 | 0.4% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| redstone (kernel) | 99 | 0.1% |
| bukkit api | 81 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50281 | 48.0% |
| phase: unclassified | 33831 | 32.3% |
| phase: main tick (unclassified) | 12217 | 11.7% |
| phase: network sync (ServerEntity) | 2836 | 2.7% |
| phase: chunk tick | 2614 | 2.5% |
| phase: chunk system (off-main worker) | 1176 | 1.1% |
| phase: block entities (hoppers/furnaces) | 836 | 0.8% |
| phase: random tick | 568 | 0.5% |
| phase: mob spawning | 344 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91325** (87.2%) · native/JVM-internal **13304** (12.7%) · other **75** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3991 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2833 | 2.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2682 | 2.6% |
| `vtable stub` | native/JVM-internal | 2595 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2282 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2079 | 2.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1571 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1503 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1496 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1462 | 1.4% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1346 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1342 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1222 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1191 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1188 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1183 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1164 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1161 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1112 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1033 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1029 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 955 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 859 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 859 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 843 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 841 | 0.8% |
| `colpush_tick` | native/JVM-internal | 837 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 819 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 804 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 761 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 726 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 719 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 642 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64853)

| bucket | self-time samples | share |
|---|---|---|
| other | 61882 | 95.4% |
| entities/mobs (kernel) | 1078 | 1.7% |
| kernel: other | 630 | 1.0% |
| chunk system (kernel) | 258 | 0.4% |
| JDK collections | 198 | 0.3% |
| JIT stubs (vtable/itable) | 188 | 0.3% |
| moonrise/paper patches | 177 | 0.3% |
| fastutil collections | 168 | 0.3% |
| network (kernel) | 109 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 64 | 0.1% |
| bukkit api | 6 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62193 | 95.9% |
| phase: entity tick (AI/movement) | 1928 | 3.0% |
| phase: main tick (unclassified) | 406 | 0.6% |
| phase: chunk tick | 108 | 0.2% |
| phase: network sync (ServerEntity) | 91 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 19 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55935** (86.2%) · native/JVM-internal **8912** (13.7%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53095 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.4% |
| `read` | native/JVM-internal | 1220 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 166 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 126 | 0.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 84 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 60 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4181)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4181 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2404 | 57.5% |
| phase: entity tick (AI/movement) | 1351 | 32.3% |
| phase: main tick (unclassified) | 300 | 7.2% |
| phase: chunk system (off-main worker) | 51 | 1.2% |
| phase: network sync (ServerEntity) | 31 | 0.7% |
| phase: mob spawning | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4181** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 680 | 16.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 538 | 12.9% |
| `char[]_[k]` | other | 452 | 10.8% |
| `byte[]_[k]` | other | 267 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 173 | 4.1% |
| `long[]_[i]` | other | 148 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.2% |
| `java.lang.Object[]_[i]` | other | 122 | 2.9% |
| `java.util.ArrayList_[i]` | other | 121 | 2.9% |
| `byte[]_[i]` | other | 95 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 73 | 1.7% |
| `int[]_[i]` | other | 64 | 1.5% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 49 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 46 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 45 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 40 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f21678314c0_[i]` | other | 40 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104704 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18920 | 18.07% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6141 | 5.87% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5214 | 4.98% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4058 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1195 | 1.14% |
| `net/minecraft/world/entity/ai/Brain.tick` | 578 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 434 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 402 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 305 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.21% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 98 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 680 | 16.3% |
| `net.minecraft.world.phys.AABB_[i]` | 538 | 12.9% |
| `char[]_[k]` | 452 | 10.8% |
| `byte[]_[k]` | 267 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 173 | 4.1% |
| `long[]_[i]` | 148 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.2% |
| `java.lang.Object[]_[i]` | 122 | 2.9% |
| `java.util.ArrayList_[i]` | 121 | 2.9% |
| `byte[]_[i]` | 95 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 142 pauses / total 19310 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148064..154127 (delta 6063, churn 4.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99813->107388, minecraft:drowned 3361->4532, minecraft:zombie 3599->4586, minecraft:husk 4646->5496, minecraft:skeleton 4164->4828, minecraft:creeper 4637->5051, minecraft:chicken 3068->3425, minecraft:sheep 3192->3537
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=6063)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47473976 B)
- `wall-collapsed.txt` (3164562 B)
- `alloc-collapsed.txt` (1945488 B)
- `cpu-flamegraph.html` (249949 B)
- `server-stdout.log` (267619 B)
- `gc.log` (133279 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
