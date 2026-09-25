# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.338 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.3, 1.7, 2.0, 1.4, 2.6, 2.2]
- spark tick-monitor MSPT: avg **399.41ms** / min 336.36ms / max **583.53ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:00:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6885976 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 336.36 | — | — | — | 583.53 | 399.41 |

- entity totals seen: [148950, 150387, 151351]
- top entity types (max seen): minecraft:item×103322, minecraft:creeper×5201, minecraft:husk×5135, minecraft:skeleton×4855, minecraft:spider×4816, minecraft:zombie×4672, minecraft:drowned×4554, minecraft:sheep×3501, minecraft:chicken×3429, minecraft:cow×3391, minecraft:pig×3244, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/AaNszml8J7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **20943.6 ms**, avg **174.53 ms**, max **2364.6 ms**
- heap high-water seen: **7433 MB** -> last-after: **4148 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117067)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28940 | 24.7% |
| kernel: other | 27960 | 23.9% |
| other | 15418 | 13.2% |
| chunk system (kernel) | 9765 | 8.3% |
| moonrise/paper patches | 9617 | 8.2% |
| fastutil collections | 7015 | 6.0% |
| JDK collections | 5935 | 5.1% |
| JIT stubs (vtable/itable) | 3536 | 3.0% |
| network (kernel) | 3375 | 2.9% |
| JDK invokes/VarHandle | 2468 | 2.1% |
| JDK other | 1979 | 1.7% |
| JVM internals (GC oop barriers) | 555 | 0.5% |
| vdso (clock) | 224 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93300 | 79.7% |
| phase: unclassified | 13872 | 11.8% |
| phase: main tick (unclassified) | 3657 | 3.1% |
| phase: chunk tick | 2117 | 1.8% |
| phase: network sync (ServerEntity) | 1871 | 1.6% |
| phase: chunk system (off-main worker) | 1097 | 0.9% |
| phase: block entities (hoppers/furnaces) | 635 | 0.5% |
| phase: random tick | 391 | 0.3% |
| phase: mob spawning | 126 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101091** (86.4%) · native/JVM-internal **15892** (13.6%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4502 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3313 | 2.8% |
| `vtable stub` | native/JVM-internal | 2898 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2638 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1984 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1794 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1728 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1551 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1521 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1483 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1448 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1381 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1376 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1323 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1168 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1153 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1137 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1112 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1081 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 971 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 941 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 938 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 934 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 921 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 838 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 814 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 797 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 782 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 757 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 719 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 717 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 699 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 673 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 653 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 639 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57856 | 94.5% |
| entities/mobs (kernel) | 1020 | 1.7% |
| kernel: other | 944 | 1.5% |
| moonrise/paper patches | 319 | 0.5% |
| chunk system (kernel) | 286 | 0.5% |
| fastutil collections | 213 | 0.3% |
| JDK collections | 186 | 0.3% |
| JIT stubs (vtable/itable) | 129 | 0.2% |
| network (kernel) | 119 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 73 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57645 | 94.1% |
| phase: entity tick (AI/movement) | 3159 | 5.2% |
| phase: main tick (unclassified) | 207 | 0.3% |
| phase: chunk tick | 77 | 0.1% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52405** (85.6%) · native/JVM-internal **8843** (14.4%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49028 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `vtable stub` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 93 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `syscall` | native/JVM-internal | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 22938)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 22938 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 20574 | 89.7% |
| phase: entity tick (AI/movement) | 2059 | 9.0% |
| phase: chunk system (off-main worker) | 179 | 0.8% |
| phase: main tick (unclassified) | 86 | 0.4% |
| phase: network sync (ServerEntity) | 16 | 0.1% |
| phase: block entities (hoppers/furnaces) | 12 | 0.1% |
| phase: chunk tick | 5 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **22938** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3200 | 14.0% |
| `byte[]_[i]` | other | 2237 | 9.8% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2153 | 9.4% |
| `java.lang.String_[i]` | other | 1464 | 6.4% |
| `java.lang.Object[]_[i]` | other | 1458 | 6.4% |
| `byte[]_[k]` | other | 1188 | 5.2% |
| `short[]_[i]` | other | 978 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 844 | 3.7% |
| `long[]_[k]` | other | 839 | 3.7% |
| `java.lang.Object[]_[k]` | other | 766 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 661 | 2.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 556 | 2.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 534 | 2.3% |
| `java.util.Optional_[i]` | other | 518 | 2.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 498 | 2.2% |
| `char[]_[k]` | other | 443 | 1.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 273 | 1.2% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 222 | 1.0% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 211 | 0.9% |
| `long[]_[i]` | other | 182 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117067 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34879 | 29.79% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22592 | 19.30% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6480 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5360 | 4.58% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4444 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1202 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 906 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 432 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 248 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 207 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 198 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 181 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3200 | 14.0% |
| `byte[]_[i]` | 2237 | 9.8% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2153 | 9.4% |
| `java.lang.String_[i]` | 1464 | 6.4% |
| `java.lang.Object[]_[i]` | 1458 | 6.4% |
| `byte[]_[k]` | 1188 | 5.2% |
| `short[]_[i]` | 978 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 844 | 3.7% |
| `long[]_[k]` | 839 | 3.7% |
| `java.lang.Object[]_[k]` | 766 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 20944 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148102..151351 (delta 3249, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99589->103322, minecraft:drowned 3545->4554, minecraft:zombie 3711->4672, minecraft:creeper 4556->5201, minecraft:husk 4492->5135, minecraft:spider 4251->4816, minecraft:skeleton 4374->4855, minecraft:chicken 3398->3429
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3249)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56459789 B)
- `wall-collapsed.txt` (3760701 B)
- `alloc-collapsed.txt` (5321855 B)
- `cpu-flamegraph.html` (304638 B)
- `server-stdout.log` (246253 B)
- `gc.log` (113588 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
