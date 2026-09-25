# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.899 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.5, 2.6, 2.9, 3.3, 3.4, 3.4]
- spark tick-monitor MSPT: avg **360.51ms** / min 254.72ms / max **468.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T10:15:06Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7294482 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [10:17:02 INFO]: [crussty-plugin] [cruss | 301.27 | — | — | — | 468.41 | 360.51 |

- entity totals seen: [149769, 151903, 152686]
- top entity types (max seen): minecraft:item×107365, minecraft:husk×5395, minecraft:drowned×5054, minecraft:creeper×4918, minecraft:spider×4385, minecraft:zombie×4336, minecraft:skeleton×4279, minecraft:sheep×3367, minecraft:cow×3332, minecraft:pig×3266, minecraft:chicken×3254, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VMF86yR8N8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **8**)
- total pause: **16004.7 ms**, avg **126.02 ms**, max **1379.8 ms**
- heap high-water seen: **8134 MB** -> last-after: **4856 MB**
  - Young (Allocation Failure): 108
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 101581)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32049 | 31.6% |
| kernel: other | 20716 | 20.4% |
| other | 10695 | 10.5% |
| chunk system (kernel) | 8985 | 8.8% |
| JDK collections | 7279 | 7.2% |
| moonrise/paper patches | 6205 | 6.1% |
| fastutil collections | 5437 | 5.4% |
| network (kernel) | 3638 | 3.6% |
| JDK invokes/VarHandle | 2424 | 2.4% |
| JDK other | 1946 | 1.9% |
| JIT stubs (vtable/itable) | 1713 | 1.7% |
| vdso (clock) | 139 | 0.1% |
| block entities/hoppers (kernel) | 132 | 0.1% |
| craftbukkit glue | 86 | 0.1% |
| bukkit api | 64 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 46531 | 45.8% |
| phase: unclassified | 32581 | 32.1% |
| phase: main tick (unclassified) | 13292 | 13.1% |
| phase: chunk tick | 2925 | 2.9% |
| phase: network sync (ServerEntity) | 2857 | 2.8% |
| phase: chunk system (off-main worker) | 1280 | 1.3% |
| phase: block entities (hoppers/furnaces) | 1100 | 1.1% |
| phase: random tick | 651 | 0.6% |
| phase: mob spawning | 364 | 0.4% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91649** (90.2%) · native/JVM-internal **9834** (9.7%) · other **98** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4536 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2915 | 2.9% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2196 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2015 | 2.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1655 | 1.6% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1588 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1552 | 1.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1504 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1478 | 1.5% |
| `vtable stub` | native/JVM-internal | 1410 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1378 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1349 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1294 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1277 | 1.3% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1248 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1125 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1081 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1049 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1005 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 982 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 962 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 907 | 0.9% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 866 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 828 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 822 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 793 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 779 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 778 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 767 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 767 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 740 | 0.7% |
| `colpush_tick` | native/JVM-internal | 728 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 720 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 710 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 688 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 687 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 670 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 653 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 632 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 620 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63650)

| bucket | self-time samples | share |
|---|---|---|
| other | 60751 | 95.4% |
| entities/mobs (kernel) | 962 | 1.5% |
| kernel: other | 736 | 1.2% |
| chunk system (kernel) | 269 | 0.4% |
| JDK collections | 209 | 0.3% |
| moonrise/paper patches | 198 | 0.3% |
| fastutil collections | 180 | 0.3% |
| network (kernel) | 128 | 0.2% |
| JIT stubs (vtable/itable) | 74 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61043 | 95.9% |
| phase: entity tick (AI/movement) | 1812 | 2.8% |
| phase: main tick (unclassified) | 458 | 0.7% |
| phase: chunk tick | 111 | 0.2% |
| phase: network sync (ServerEntity) | 100 | 0.2% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54853** (86.2%) · native/JVM-internal **8795** (13.8%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51986 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.5% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `syscall` | native/JVM-internal | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 83 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `vtable stub` | native/JVM-internal | 59 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4173)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4173 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2379 | 57.0% |
| phase: entity tick (AI/movement) | 1301 | 31.2% |
| phase: main tick (unclassified) | 343 | 8.2% |
| phase: chunk system (off-main worker) | 58 | 1.4% |
| phase: block entities (hoppers/furnaces) | 36 | 0.9% |
| phase: network sync (ServerEntity) | 32 | 0.8% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4173** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 644 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 559 | 13.4% |
| `char[]_[k]` | other | 435 | 10.4% |
| `byte[]_[k]` | other | 236 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 163 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 3.8% |
| `long[]_[i]` | other | 154 | 3.7% |
| `java.util.ArrayList_[i]` | other | 148 | 3.5% |
| `java.lang.Object[]_[i]` | other | 120 | 2.9% |
| `byte[]_[i]` | other | 111 | 2.7% |
| `int[]_[i]` | other | 98 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.4% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 49 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f19c1aa9190_[i]` | other | 45 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 41 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 101581 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 17662 | 17.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5510 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5376 | 5.29% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3907 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1169 | 1.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 488 | 0.48% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 463 | 0.46% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 366 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 219 | 0.22% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 106 | 0.10% |
| `net/minecraft/world/entity/projectile/Arrow.tick` | 76 | 0.07% |
| `net/minecraft/world/entity/projectile/AbstractArrow.tick` | 76 | 0.07% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 644 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 559 | 13.4% |
| `char[]_[k]` | 435 | 10.4% |
| `byte[]_[k]` | 236 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 163 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 3.8% |
| `long[]_[i]` | 154 | 3.7% |
| `java.util.ArrayList_[i]` | 148 | 3.5% |
| `java.lang.Object[]_[i]` | 120 | 2.9% |
| `byte[]_[i]` | 111 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 16005 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=141107..152686 (delta 11579, churn 7.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99551->107365, minecraft:husk 3987->5395, minecraft:drowned 3648->5054, minecraft:creeper 4166->4918, minecraft:spider 3778->4385, minecraft:zombie 3755->4336, minecraft:sheep 2831->3367, minecraft:cow 2802->3332
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=11579)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43938668 B)
- `wall-collapsed.txt` (2873551 B)
- `alloc-collapsed.txt` (1820595 B)
- `cpu-flamegraph.html` (230122 B)
- `server-stdout.log` (27442709 B)
- `gc.log` (118637 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
