# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.058 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 1.9, 2.1, 2.6, 2.8, 2.9]
- spark tick-monitor MSPT: avg **358.36ms** / min 295.53ms / max **435.79ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:56:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6699453 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:58:57 INFO]: [crussty-plugin] [cruss | 295.53 | — | — | — | 435.79 | 358.36 |

- entity totals seen: [150992, 153164, 153869]
- top entity types (max seen): minecraft:item×107089, minecraft:husk×5444, minecraft:creeper×4993, minecraft:skeleton×4792, minecraft:zombie×4593, minecraft:drowned×4576, minecraft:spider×4394, minecraft:sheep×3522, minecraft:chicken×3400, minecraft:cow×3355, minecraft:pig×3184, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/YQlX4nrATH
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19369.5 ms**, avg **169.91 ms**, max **2484.9 ms**
- heap high-water seen: **7599 MB** -> last-after: **3726 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104405)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33739 | 32.3% |
| kernel: other | 22382 | 21.4% |
| other | 11469 | 11.0% |
| chunk system (kernel) | 8093 | 7.8% |
| JDK collections | 7308 | 7.0% |
| fastutil collections | 5369 | 5.1% |
| moonrise/paper patches | 4979 | 4.8% |
| JIT stubs (vtable/itable) | 3541 | 3.4% |
| network (kernel) | 2768 | 2.7% |
| JDK invokes/VarHandle | 2494 | 2.4% |
| JDK other | 1787 | 1.7% |
| vdso (clock) | 132 | 0.1% |
| bukkit api | 94 | 0.1% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51062 | 48.9% |
| phase: unclassified | 32615 | 31.2% |
| phase: main tick (unclassified) | 12950 | 12.4% |
| phase: chunk tick | 2537 | 2.4% |
| phase: network sync (ServerEntity) | 2240 | 2.1% |
| phase: chunk system (off-main worker) | 1174 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1012 | 1.0% |
| phase: random tick | 525 | 0.5% |
| phase: mob spawning | 289 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92566** (88.7%) · native/JVM-internal **11739** (11.2%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4031 | 3.9% |
| `vtable stub` | native/JVM-internal | 3059 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2919 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2704 | 2.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1565 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1533 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1384 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1337 | 1.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1335 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1307 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1303 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1276 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1168 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1154 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1093 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1079 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1028 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1003 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 953 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 940 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 932 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 892 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 889 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 889 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 863 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 833 | 0.8% |
| `colpush_tick` | native/JVM-internal | 813 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 786 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 784 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 771 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 748 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 696 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 686 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 678 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 665 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 642 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 635 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61891 | 95.4% |
| entities/mobs (kernel) | 1054 | 1.6% |
| kernel: other | 710 | 1.1% |
| chunk system (kernel) | 237 | 0.4% |
| JDK collections | 213 | 0.3% |
| JIT stubs (vtable/itable) | 180 | 0.3% |
| fastutil collections | 158 | 0.2% |
| moonrise/paper patches | 155 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK other | 77 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62211 | 95.9% |
| phase: entity tick (AI/movement) | 1934 | 3.0% |
| phase: main tick (unclassified) | 444 | 0.7% |
| phase: chunk tick | 92 | 0.1% |
| phase: network sync (ServerEntity) | 79 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 28 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55940** (86.2%) · native/JVM-internal **8911** (13.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53063 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.4% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 150 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 108 | 0.2% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 41 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 40 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 38 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3421)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3421 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2039 | 59.6% |
| phase: entity tick (AI/movement) | 1001 | 29.3% |
| phase: main tick (unclassified) | 261 | 7.6% |
| phase: chunk system (off-main worker) | 55 | 1.6% |
| phase: block entities (hoppers/furnaces) | 27 | 0.8% |
| phase: network sync (ServerEntity) | 17 | 0.5% |
| phase: mob spawning | 9 | 0.3% |
| phase: random tick | 6 | 0.2% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3421** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 468 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 454 | 13.3% |
| `char[]_[k]` | other | 388 | 11.3% |
| `byte[]_[k]` | other | 199 | 5.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 125 | 3.7% |
| `java.util.ArrayList_[i]` | other | 122 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 120 | 3.5% |
| `java.lang.Object[]_[i]` | other | 118 | 3.4% |
| `long[]_[i]` | other | 115 | 3.4% |
| `byte[]_[i]` | other | 91 | 2.7% |
| `int[]_[i]` | other | 91 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 41 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.9% |
| `java.lang.String_[i]` | other | 30 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f4bf1a0e260_[i]` | other | 30 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f4bf183b268_[i]` | other | 30 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 29 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104405 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19229 | 18.42% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6271 | 6.01% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5240 | 5.02% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3963 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1429 | 1.37% |
| `net/minecraft/world/entity/ai/Brain.tick` | 588 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 391 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 376 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 362 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 289 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 255 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 91 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 468 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | 454 | 13.3% |
| `char[]_[k]` | 388 | 11.3% |
| `byte[]_[k]` | 199 | 5.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 125 | 3.7% |
| `java.util.ArrayList_[i]` | 122 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | 120 | 3.5% |
| `java.lang.Object[]_[i]` | 118 | 3.4% |
| `long[]_[i]` | 115 | 3.4% |
| `byte[]_[i]` | 91 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19369 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148333..153869 (delta 5536, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100078->107089, minecraft:husk 4591->5444, minecraft:drowned 3734->4576, minecraft:zombie 3752->4593, minecraft:skeleton 4187->4792, minecraft:creeper 4558->4993, minecraft:pig 2862->3184, minecraft:spider 4078->4394
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5536)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52552119 B)
- `wall-collapsed.txt` (3227156 B)
- `alloc-collapsed.txt` (1908797 B)
- `cpu-flamegraph.html` (286681 B)
- `server-stdout.log` (329134 B)
- `gc.log` (108362 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
