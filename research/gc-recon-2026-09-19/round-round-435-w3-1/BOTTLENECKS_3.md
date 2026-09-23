# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.344 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [12.5, 1.9, 2.1, 2.5, 2.7, 2.8]
- spark tick-monitor MSPT: avg **369.13ms** / min 309.14ms / max **465.92ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:56:40Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8878218 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:58:57 INFO]: [crussty-plugin] [cruss | 309.14 | — | — | — | 465.92 | 369.13 |

- entity totals seen: [149086, 150875, 150915]
- top entity types (max seen): minecraft:item×103062, minecraft:creeper×5232, minecraft:husk×5229, minecraft:skeleton×4866, minecraft:spider×4799, minecraft:zombie×4617, minecraft:drowned×4509, minecraft:sheep×3535, minecraft:chicken×3438, minecraft:cow×3326, minecraft:pig×3229, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/zc5UTe9tUa
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **768** (Full GC: **10**)
- total pause: **24363.3 ms**, avg **31.72 ms**, max **2425.7 ms**
- heap high-water seen: **8408 MB** -> last-after: **4384 MB**
  - Young (Allocation Failure): 746
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 107770)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 38619 | 35.8% |
| kernel: other | 18914 | 17.6% |
| other | 11421 | 10.6% |
| chunk system (kernel) | 10181 | 9.4% |
| JDK collections | 7360 | 6.8% |
| fastutil collections | 5317 | 4.9% |
| moonrise/paper patches | 5108 | 4.7% |
| network (kernel) | 3050 | 2.8% |
| JDK invokes/VarHandle | 2566 | 2.4% |
| JIT stubs (vtable/itable) | 2554 | 2.4% |
| JDK other | 1781 | 1.7% |
| JVM internals (GC oop barriers) | 487 | 0.5% |
| vdso (clock) | 103 | 0.1% |
| block entities/hoppers (kernel) | 102 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| bukkit api | 54 | 0.1% |
| worldgen/noise (kernel) | 37 | 0.0% |
| redstone (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 59267 | 55.0% |
| phase: unclassified | 30028 | 27.9% |
| phase: main tick (unclassified) | 11378 | 10.6% |
| phase: chunk tick | 2407 | 2.2% |
| phase: network sync (ServerEntity) | 2044 | 1.9% |
| phase: chunk system (off-main worker) | 1075 | 1.0% |
| phase: block entities (hoppers/furnaces) | 927 | 0.9% |
| phase: random tick | 498 | 0.5% |
| phase: mob spawning | 143 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95489** (88.6%) · native/JVM-internal **12149** (11.3%) · other **132** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 7473 | 6.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3765 | 3.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2612 | 2.4% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2417 | 2.2% |
| `vtable stub` | native/JVM-internal | 2174 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1750 | 1.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1645 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1499 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1348 | 1.3% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 1338 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1278 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1223 | 1.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1117 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1062 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1049 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1048 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1016 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 998 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 997 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 900 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 849 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 842 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 823 | 0.8% |
| `colpush_tick` | native/JVM-internal | 812 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 790 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 783 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 779 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 736 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 731 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 730 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 714 | 0.7% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 696 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 690 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 686 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 683 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 58113 | 94.9% |
| entities/mobs (kernel) | 1293 | 2.1% |
| kernel: other | 605 | 1.0% |
| chunk system (kernel) | 310 | 0.5% |
| JDK collections | 244 | 0.4% |
| fastutil collections | 171 | 0.3% |
| moonrise/paper patches | 167 | 0.3% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 86 | 0.1% |
| JDK other | 51 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58330 | 95.2% |
| phase: entity tick (AI/movement) | 2246 | 3.7% |
| phase: main tick (unclassified) | 408 | 0.7% |
| phase: chunk tick | 116 | 0.2% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52424** (85.6%) · native/JVM-internal **8825** (14.4%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49329 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 281 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 114 | 0.2% |
| `vtable stub` | native/JVM-internal | 89 | 0.1% |
| `syscall` | native/JVM-internal | 86 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 57 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 26264)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 26264 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 24412 | 92.9% |
| phase: entity tick (AI/movement) | 1348 | 5.1% |
| phase: main tick (unclassified) | 247 | 0.9% |
| phase: chunk system (off-main worker) | 194 | 0.7% |
| phase: block entities (hoppers/furnaces) | 25 | 0.1% |
| phase: network sync (ServerEntity) | 22 | 0.1% |
| phase: chunk tick | 6 | 0.0% |
| phase: random tick | 6 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **26264** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3657 | 13.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2317 | 8.8% |
| `byte[]_[i]` | other | 2248 | 8.6% |
| `java.lang.Object[]_[i]` | other | 1622 | 6.2% |
| `java.lang.String_[i]` | other | 1615 | 6.1% |
| `byte[]_[k]` | other | 1430 | 5.4% |
| `long[]_[k]` | other | 1304 | 5.0% |
| `short[]_[i]` | other | 1144 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 953 | 3.6% |
| `java.lang.Object[]_[k]` | other | 862 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 746 | 2.8% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 627 | 2.4% |
| `java.util.Optional_[i]` | other | 578 | 2.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 2.0% |
| `char[]_[k]` | other | 449 | 1.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 445 | 1.7% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 312 | 1.2% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 259 | 1.0% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 246 | 0.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 217 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107770 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 25830 | 23.97% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6745 | 6.26% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5159 | 4.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4592 | 4.26% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 828 | 0.77% |
| `net/minecraft/world/entity/ai/Brain.tick` | 775 | 0.72% |
| `net/minecraft/world/entity/npc/Villager.tick` | 409 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 352 | 0.33% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 324 | 0.30% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 267 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 208 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 199 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3657 | 13.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2317 | 8.8% |
| `byte[]_[i]` | 2248 | 8.6% |
| `java.lang.Object[]_[i]` | 1622 | 6.2% |
| `java.lang.String_[i]` | 1615 | 6.1% |
| `byte[]_[k]` | 1430 | 5.4% |
| `long[]_[k]` | 1304 | 5.0% |
| `short[]_[i]` | 1144 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | 953 | 3.6% |
| `java.lang.Object[]_[k]` | 862 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 768 pauses / total 24363 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147934..150915 (delta 2981, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99477->103062, minecraft:drowned 3541->4509, minecraft:zombie 3707->4617, minecraft:husk 4582->5229, minecraft:creeper 4594->5232, minecraft:spider 4223->4799, minecraft:skeleton 4390->4866, minecraft:chicken 3411->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2981)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57076602 B)
- `wall-collapsed.txt` (3348128 B)
- `alloc-collapsed.txt` (5536923 B)
- `cpu-flamegraph.html` (272474 B)
- `server-stdout.log` (365813 B)
- `gc.log` (667214 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
