# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.476 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.1, 2.2, 2.4, 2.8, 3.0]
- spark tick-monitor MSPT: avg **373.06ms** / min 306.03ms / max **567.47ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T14:26:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8463234 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 306.03 | — | — | — | 567.47 | 373.06 |

- entity totals seen: [148951, 150728, 150857]
- top entity types (max seen): minecraft:item×103022, minecraft:creeper×5206, minecraft:husk×5185, minecraft:skeleton×4875, minecraft:spider×4726, minecraft:zombie×4620, minecraft:drowned×4507, minecraft:sheep×3538, minecraft:chicken×3437, minecraft:cow×3332, minecraft:pig×3228, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/hmkwMU9XAn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **133** (Full GC: **10**)
- total pause: **22146.5 ms**, avg **166.52 ms**, max **2286.3 ms**
- heap high-water seen: **7639 MB** -> last-after: **3712 MB**
  - Young (Allocation Failure): 111
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 113013)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27488 | 24.3% |
| kernel: other | 25683 | 22.7% |
| other | 13629 | 12.1% |
| chunk system (kernel) | 10364 | 9.2% |
| moonrise/paper patches | 10281 | 9.1% |
| fastutil collections | 7426 | 6.6% |
| JDK collections | 6119 | 5.4% |
| network (kernel) | 3933 | 3.5% |
| JIT stubs (vtable/itable) | 2891 | 2.6% |
| JDK invokes/VarHandle | 2470 | 2.2% |
| JDK other | 1657 | 1.5% |
| JVM internals (GC oop barriers) | 519 | 0.5% |
| vdso (clock) | 247 | 0.2% |
| block entities/hoppers (kernel) | 93 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| bukkit api | 56 | 0.0% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88114 | 78.0% |
| phase: unclassified | 13473 | 11.9% |
| phase: main tick (unclassified) | 4140 | 3.7% |
| phase: chunk tick | 2335 | 2.1% |
| phase: network sync (ServerEntity) | 2298 | 2.0% |
| phase: chunk system (off-main worker) | 1199 | 1.1% |
| phase: block entities (hoppers/furnaces) | 824 | 0.7% |
| phase: random tick | 491 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98358** (87.0%) · native/JVM-internal **14569** (12.9%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5060 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3690 | 3.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2511 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2493 | 2.2% |
| `vtable stub` | native/JVM-internal | 2265 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2208 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2010 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1836 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1620 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1609 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1535 | 1.4% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1496 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1488 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1426 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1385 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1286 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1150 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1143 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1139 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1053 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1038 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1032 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1029 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 984 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 981 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 939 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 841 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 777 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 774 | 0.7% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 743 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 741 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 703 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 689 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 688 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 672 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 58025 | 94.7% |
| entities/mobs (kernel) | 904 | 1.5% |
| kernel: other | 813 | 1.3% |
| moonrise/paper patches | 360 | 0.6% |
| chunk system (kernel) | 321 | 0.5% |
| fastutil collections | 249 | 0.4% |
| JDK collections | 185 | 0.3% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 45 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57889 | 94.5% |
| phase: entity tick (AI/movement) | 2840 | 4.6% |
| phase: main tick (unclassified) | 223 | 0.4% |
| phase: chunk tick | 110 | 0.2% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 43 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52365** (85.5%) · native/JVM-internal **8883** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49204 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1223 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 148 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 110 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `syscall` | native/JVM-internal | 88 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 86 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 62 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 60 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4013)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4013 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2338 | 58.3% |
| phase: unclassified | 1508 | 37.6% |
| phase: main tick (unclassified) | 97 | 2.4% |
| phase: chunk system (off-main worker) | 26 | 0.6% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4013** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 643 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 570 | 14.2% |
| `char[]_[k]` | other | 439 | 10.9% |
| `byte[]_[k]` | other | 262 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 178 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 173 | 4.3% |
| `long[]_[i]` | other | 120 | 3.0% |
| `java.lang.Object[]_[i]` | other | 116 | 2.9% |
| `java.util.ArrayList_[i]` | other | 109 | 2.7% |
| `byte[]_[i]` | other | 90 | 2.2% |
| `int[]_[i]` | other | 65 | 1.6% |
| `java.util.ArrayList$Itr_[i]` | other | 59 | 1.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 58 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 48 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f9d66a4bdf0_[i]` | other | 32 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f9d66a49eb8_[i]` | other | 28 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113013 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33033 | 29.23% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21092 | 18.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6081 | 5.38% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5176 | 4.58% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4481 | 3.97% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 887 | 0.78% |
| `net/minecraft/world/entity/ai/Brain.tick` | 872 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 203 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 197 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 189 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 173 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 643 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 570 | 14.2% |
| `char[]_[k]` | 439 | 10.9% |
| `byte[]_[k]` | 262 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 178 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 173 | 4.3% |
| `long[]_[i]` | 120 | 3.0% |
| `java.lang.Object[]_[i]` | 116 | 2.9% |
| `java.util.ArrayList_[i]` | 109 | 2.7% |
| `byte[]_[i]` | 90 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 133 pauses / total 22147 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147748..150857 (delta 3109, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99401->103022, minecraft:drowned 3595->4507, minecraft:zombie 3731->4620, minecraft:husk 4534->5185, minecraft:creeper 4568->5206, minecraft:spider 4197->4726, minecraft:skeleton 4431->4875, minecraft:chicken 3396->3437
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3109)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50628757 B)
- `wall-collapsed.txt` (3308865 B)
- `alloc-collapsed.txt` (2141131 B)
- `cpu-flamegraph.html` (272782 B)
- `server-stdout.log` (243430 B)
- `gc.log` (125658 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
