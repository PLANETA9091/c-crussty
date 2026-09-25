# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.175 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.1, 2.4, 2.6, 2.6]
- spark tick-monitor MSPT: avg **382.84ms** / min 325.69ms / max **484.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T16:08:59Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7020682 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:11:07 INFO]: [crussty-plugin] [cruss | 325.69 | — | — | — | 484.83 | 382.84 |

- entity totals seen: [150119, 152444, 153635]
- top entity types (max seen): minecraft:item×106796, minecraft:husk×5412, minecraft:creeper×5025, minecraft:skeleton×4751, minecraft:zombie×4569, minecraft:drowned×4516, minecraft:spider×4453, minecraft:sheep×3523, minecraft:chicken×3419, minecraft:cow×3386, minecraft:pig×3177, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/yMmB3sI1sm
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **110** (Full GC: **9**)
- total pause: **19020.9 ms**, avg **172.92 ms**, max **2494.2 ms**
- heap high-water seen: **7526 MB** -> last-after: **4230 MB**
  - Young (Allocation Failure): 92
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104726)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34379 | 32.8% |
| kernel: other | 21552 | 20.6% |
| other | 12777 | 12.2% |
| chunk system (kernel) | 7731 | 7.4% |
| JDK collections | 7259 | 6.9% |
| moonrise/paper patches | 4938 | 4.7% |
| fastutil collections | 4892 | 4.7% |
| JIT stubs (vtable/itable) | 3079 | 2.9% |
| network (kernel) | 2710 | 2.6% |
| JDK invokes/VarHandle | 2350 | 2.2% |
| JDK other | 2000 | 1.9% |
| JVM internals (GC oop barriers) | 589 | 0.6% |
| vdso (clock) | 134 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| bukkit api | 72 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49407 | 47.2% |
| phase: unclassified | 34518 | 33.0% |
| phase: main tick (unclassified) | 13319 | 12.7% |
| phase: chunk tick | 2529 | 2.4% |
| phase: network sync (ServerEntity) | 2151 | 2.1% |
| phase: chunk system (off-main worker) | 1029 | 1.0% |
| phase: block entities (hoppers/furnaces) | 978 | 0.9% |
| phase: random tick | 516 | 0.5% |
| phase: mob spawning | 277 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91386** (87.3%) · native/JVM-internal **13244** (12.6%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3817 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3404 | 3.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2499 | 2.4% |
| `vtable stub` | native/JVM-internal | 2441 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2079 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1686 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1509 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1378 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1333 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1265 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1259 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1158 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1133 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1095 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1072 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 983 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 976 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 974 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 930 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 915 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 909 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 862 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 858 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 855 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 848 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 837 | 0.8% |
| `colpush_tick` | native/JVM-internal | 799 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 765 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 763 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 708 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 695 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 677 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 675 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 667 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 667 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 649 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63653)

| bucket | self-time samples | share |
|---|---|---|
| other | 60692 | 95.3% |
| entities/mobs (kernel) | 1170 | 1.8% |
| kernel: other | 648 | 1.0% |
| chunk system (kernel) | 249 | 0.4% |
| JDK collections | 221 | 0.3% |
| moonrise/paper patches | 184 | 0.3% |
| fastutil collections | 159 | 0.2% |
| JIT stubs (vtable/itable) | 126 | 0.2% |
| network (kernel) | 77 | 0.1% |
| JDK other | 57 | 0.1% |
| JDK invokes/VarHandle | 51 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61073 | 95.9% |
| phase: entity tick (AI/movement) | 1846 | 2.9% |
| phase: main tick (unclassified) | 456 | 0.7% |
| phase: chunk tick | 93 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 43 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 15 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54824** (86.1%) · native/JVM-internal **8825** (13.9%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51890 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.5% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 132 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 119 | 0.2% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 63 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3344)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3344 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1997 | 59.7% |
| phase: entity tick (AI/movement) | 986 | 29.5% |
| phase: main tick (unclassified) | 253 | 7.6% |
| phase: chunk system (off-main worker) | 38 | 1.1% |
| phase: block entities (hoppers/furnaces) | 28 | 0.8% |
| phase: network sync (ServerEntity) | 20 | 0.6% |
| phase: mob spawning | 13 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3344** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 512 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 445 | 13.3% |
| `char[]_[k]` | other | 303 | 9.1% |
| `byte[]_[k]` | other | 182 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 138 | 4.1% |
| `long[]_[i]` | other | 122 | 3.6% |
| `int[]_[i]` | other | 111 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 104 | 3.1% |
| `byte[]_[i]` | other | 103 | 3.1% |
| `java.util.ArrayList_[i]` | other | 103 | 3.1% |
| `java.lang.Object[]_[i]` | other | 102 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 96 | 2.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.1% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.9% |
| `java.lang.String_[i]` | other | 29 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f313783eda0_[i]` | other | 27 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 27 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104726 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18552 | 17.71% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6126 | 5.85% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5115 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3873 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1376 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 537 | 0.51% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 416 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 384 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 312 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 252 | 0.24% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 94 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 512 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 445 | 13.3% |
| `char[]_[k]` | 303 | 9.1% |
| `byte[]_[k]` | 182 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 138 | 4.1% |
| `long[]_[i]` | 122 | 3.6% |
| `int[]_[i]` | 111 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 104 | 3.1% |
| `byte[]_[i]` | 103 | 3.1% |
| `java.util.ArrayList_[i]` | 103 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 110 pauses / total 19021 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148137..153635 (delta 5498, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99924->106796, minecraft:drowned 3587->4516, minecraft:husk 4528->5412, minecraft:zombie 3844->4569, minecraft:skeleton 4204->4751, minecraft:creeper 4584->5025, minecraft:spider 4042->4453, minecraft:pig 2834->3177
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5498)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55543929 B)
- `wall-collapsed.txt` (3187355 B)
- `alloc-collapsed.txt` (1915286 B)
- `cpu-flamegraph.html` (279987 B)
- `server-stdout.log` (331277 B)
- `gc.log` (104921 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
