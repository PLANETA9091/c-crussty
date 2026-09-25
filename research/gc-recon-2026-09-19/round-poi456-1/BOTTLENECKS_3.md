# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.405 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 1.8, 2.1, 2.6, 2.7, 2.8]
- spark tick-monitor MSPT: avg **348.64ms** / min 295.92ms / max **462.65ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T10:10:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6765332 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [10:12:17 INFO]: [crussty-plugin] [cruss | 295.92 | — | — | — | 462.65 | 348.64 |

- entity totals seen: [151047, 153192, 154060]
- top entity types (max seen): minecraft:item×107090, minecraft:husk×5469, minecraft:creeper×5101, minecraft:skeleton×4766, minecraft:zombie×4575, minecraft:drowned×4519, minecraft:spider×4403, minecraft:sheep×3522, minecraft:chicken×3418, minecraft:cow×3385, minecraft:pig×3189, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/eWPZsDuI58
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **19797.5 ms**, avg **170.67 ms**, max **2557.7 ms**
- heap high-water seen: **7487 MB** -> last-after: **4189 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104153)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32565 | 31.3% |
| kernel: other | 22614 | 21.7% |
| other | 12034 | 11.6% |
| chunk system (kernel) | 8317 | 8.0% |
| JDK collections | 7338 | 7.0% |
| moonrise/paper patches | 5648 | 5.4% |
| fastutil collections | 4879 | 4.7% |
| network (kernel) | 2862 | 2.7% |
| JIT stubs (vtable/itable) | 2780 | 2.7% |
| JDK invokes/VarHandle | 2582 | 2.5% |
| JDK other | 2060 | 2.0% |
| vdso (clock) | 139 | 0.1% |
| block entities/hoppers (kernel) | 105 | 0.1% |
| bukkit api | 79 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48974 | 47.0% |
| phase: unclassified | 34106 | 32.7% |
| phase: main tick (unclassified) | 13318 | 12.8% |
| phase: chunk tick | 2620 | 2.5% |
| phase: network sync (ServerEntity) | 2205 | 2.1% |
| phase: chunk system (off-main worker) | 1143 | 1.1% |
| phase: block entities (hoppers/furnaces) | 962 | 0.9% |
| phase: random tick | 513 | 0.5% |
| phase: mob spawning | 308 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92400** (88.7%) · native/JVM-internal **11636** (11.2%) · other **117** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3998 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2523 | 2.4% |
| `vtable stub` | native/JVM-internal | 2164 | 2.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1905 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1645 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1386 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1318 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1307 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1299 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1296 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1279 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1218 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1171 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1060 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1055 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 989 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 985 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 969 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 966 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 961 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 913 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 908 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 885 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 874 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 869 | 0.8% |
| `colpush_tick` | native/JVM-internal | 835 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 831 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 794 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 786 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 768 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 712 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 704 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 692 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 658 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 639 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 636 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 636 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 621 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63659)

| bucket | self-time samples | share |
|---|---|---|
| other | 60754 | 95.4% |
| entities/mobs (kernel) | 1042 | 1.6% |
| kernel: other | 686 | 1.1% |
| JDK collections | 218 | 0.3% |
| chunk system (kernel) | 205 | 0.3% |
| moonrise/paper patches | 172 | 0.3% |
| fastutil collections | 151 | 0.2% |
| JIT stubs (vtable/itable) | 110 | 0.2% |
| network (kernel) | 83 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 73 | 0.1% |
| JVM internals (GC oop barriers) | 69 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61127 | 96.0% |
| phase: entity tick (AI/movement) | 1728 | 2.7% |
| phase: main tick (unclassified) | 470 | 0.7% |
| phase: chunk tick | 149 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54435** (85.5%) · native/JVM-internal **9221** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51632 | 81.1% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 7.5% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 300 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 89 | 0.1% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 42 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 39 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3708)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3708 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2175 | 58.7% |
| phase: entity tick (AI/movement) | 1122 | 30.3% |
| phase: main tick (unclassified) | 269 | 7.3% |
| phase: chunk system (off-main worker) | 65 | 1.8% |
| phase: block entities (hoppers/furnaces) | 29 | 0.8% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3708** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 569 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 472 | 12.7% |
| `char[]_[k]` | other | 418 | 11.3% |
| `byte[]_[k]` | other | 239 | 6.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 153 | 4.1% |
| `long[]_[i]` | other | 138 | 3.7% |
| `java.lang.Object[]_[i]` | other | 129 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 123 | 3.3% |
| `java.util.ArrayList_[i]` | other | 101 | 2.7% |
| `byte[]_[i]` | other | 96 | 2.6% |
| `int[]_[i]` | other | 94 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 45 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 44 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f3d9aa0aab0_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 27 | 0.7% |
| `int[]_[k]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104153 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 17578 | 16.88% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5895 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5225 | 5.02% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3875 | 3.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1444 | 1.39% |
| `net/minecraft/world/entity/ai/Brain.tick` | 605 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 405 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 370 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 359 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 294 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 250 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 82 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 569 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 472 | 12.7% |
| `char[]_[k]` | 418 | 11.3% |
| `byte[]_[k]` | 239 | 6.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 153 | 4.1% |
| `long[]_[i]` | 138 | 3.7% |
| `java.lang.Object[]_[i]` | 129 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 123 | 3.3% |
| `java.util.ArrayList_[i]` | 101 | 2.7% |
| `byte[]_[i]` | 96 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 19797 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148300..154060 (delta 5760, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100060->107090, minecraft:drowned 3506->4519, minecraft:zombie 3612->4575, minecraft:husk 4573->5469, minecraft:skeleton 4143->4766, minecraft:creeper 4630->5101, minecraft:spider 4085->4403, minecraft:pig 2904->3189
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5760)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (60819421 B)
- `wall-collapsed.txt` (3121361 B)
- `alloc-collapsed.txt` (1862554 B)
- `cpu-flamegraph.html` (284516 B)
- `server-stdout.log` (328097 B)
- `gc.log` (110083 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
