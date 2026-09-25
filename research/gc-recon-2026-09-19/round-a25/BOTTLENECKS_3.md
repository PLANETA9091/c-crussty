# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.089 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 2.0, 2.2, 2.6, 2.7, 3.1]
- spark tick-monitor MSPT: avg **342.61ms** / min 299.95ms / max **423.51ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:46:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7551035 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 299.95 | — | — | — | 423.51 | 342.61 |

- entity totals seen: [149741, 151353, 151626]
- top entity types (max seen): minecraft:item×103567, minecraft:creeper×5240, minecraft:husk×5174, minecraft:spider×4861, minecraft:skeleton×4855, minecraft:zombie×4655, minecraft:drowned×4555, minecraft:sheep×3494, minecraft:chicken×3430, minecraft:cow×3365, minecraft:pig×3226, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/npEWh4xSIZ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **129** (Full GC: **9**)
- total pause: **24126.2 ms**, avg **187.03 ms**, max **2766.2 ms**
- heap high-water seen: **7704 MB** -> last-after: **4438 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117038)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29902 | 25.5% |
| entities/mobs (kernel) | 28227 | 24.1% |
| other | 14931 | 12.8% |
| moonrise/paper patches | 10673 | 9.1% |
| chunk system (kernel) | 9724 | 8.3% |
| fastutil collections | 6710 | 5.7% |
| JDK collections | 6168 | 5.3% |
| JIT stubs (vtable/itable) | 3135 | 2.7% |
| network (kernel) | 2473 | 2.1% |
| JDK invokes/VarHandle | 2364 | 2.0% |
| JDK other | 1638 | 1.4% |
| JVM internals (GC oop barriers) | 540 | 0.5% |
| vdso (clock) | 215 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| bukkit api | 73 | 0.1% |
| redstone (kernel) | 57 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94162 | 80.5% |
| phase: unclassified | 13201 | 11.3% |
| phase: main tick (unclassified) | 3342 | 2.9% |
| phase: chunk tick | 2185 | 1.9% |
| phase: network sync (ServerEntity) | 1892 | 1.6% |
| phase: chunk system (off-main worker) | 1025 | 0.9% |
| phase: block entities (hoppers/furnaces) | 681 | 0.6% |
| phase: random tick | 418 | 0.4% |
| phase: mob spawning | 126 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101917** (87.1%) · native/JVM-internal **14967** (12.8%) · other **154** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4741 | 4.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4112 | 3.5% |
| `vtable stub` | native/JVM-internal | 2593 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2421 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2071 | 1.8% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 2010 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2007 | 1.7% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1842 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1749 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1745 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1660 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1649 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1501 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1499 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1375 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1232 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1212 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1159 | 1.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fb1e59daae0.accept` | JVM-Java | 1112 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1111 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1067 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1047 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 989 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 959 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 934 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 857 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 848 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 847 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 804 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 777 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 766 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 743 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 731 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 727 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 714 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 645 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 623 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61238)

| bucket | self-time samples | share |
|---|---|---|
| other | 57828 | 94.4% |
| entities/mobs (kernel) | 1007 | 1.6% |
| kernel: other | 955 | 1.6% |
| moonrise/paper patches | 410 | 0.7% |
| chunk system (kernel) | 314 | 0.5% |
| fastutil collections | 233 | 0.4% |
| JDK collections | 182 | 0.3% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| network (kernel) | 77 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 47 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57617 | 94.1% |
| phase: entity tick (AI/movement) | 3196 | 5.2% |
| phase: main tick (unclassified) | 187 | 0.3% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: chunk system (off-main worker) | 27 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52371** (85.5%) · native/JVM-internal **8863** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48956 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4786 | 7.8% |
| `read` | native/JVM-internal | 1214 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 157 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 123 | 0.2% |
| `syscall` | native/JVM-internal | 98 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 95 | 0.2% |
| `vtable stub` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fb1e59daae0.accept` | JVM-Java | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 48 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4084)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4084 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2357 | 57.7% |
| phase: unclassified | 1540 | 37.7% |
| phase: main tick (unclassified) | 99 | 2.4% |
| phase: chunk system (off-main worker) | 51 | 1.2% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 6 | 0.1% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4084** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 619 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 585 | 14.3% |
| `char[]_[k]` | other | 450 | 11.0% |
| `byte[]_[k]` | other | 255 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 191 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 3.9% |
| `long[]_[i]` | other | 150 | 3.7% |
| `java.util.ArrayList_[i]` | other | 140 | 3.4% |
| `java.lang.Object[]_[i]` | other | 114 | 2.8% |
| `byte[]_[i]` | other | 87 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.7% |
| `int[]_[i]` | other | 67 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.6% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 50 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 49 | 1.2% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb1e59e2478_[i]` | other | 47 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb1e5834000_[i]` | other | 31 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117038 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35382 | 30.23% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22744 | 19.43% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6348 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5594 | 4.78% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4457 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1206 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 982 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 438 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 269 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 252 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 238 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 199 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 619 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 585 | 14.3% |
| `char[]_[k]` | 450 | 11.0% |
| `byte[]_[k]` | 255 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 191 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 3.9% |
| `long[]_[i]` | 150 | 3.7% |
| `java.util.ArrayList_[i]` | 140 | 3.4% |
| `java.lang.Object[]_[i]` | 114 | 2.8% |
| `byte[]_[i]` | 87 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 129 pauses / total 24126 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148477..151626 (delta 3149, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99880->103567, minecraft:drowned 3586->4555, minecraft:zombie 3731->4655, minecraft:creeper 4587->5240, minecraft:husk 4578->5174, minecraft:spider 4295->4861, minecraft:skeleton 4384->4855, minecraft:chicken 3405->3430
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3149)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59386403 B)
- `wall-collapsed.txt` (3795753 B)
- `alloc-collapsed.txt` (2217918 B)
- `cpu-flamegraph.html` (300391 B)
- `server-stdout.log` (262795 B)
- `gc.log` (121339 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
