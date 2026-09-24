# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.145 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.1, 2.4, 2.8, 2.8]
- spark tick-monitor MSPT: avg **367.42ms** / min 319.71ms / max **441.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:28:37Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6979160 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 319.71 | — | — | — | 441.68 | 367.42 |

- entity totals seen: [149386, 151269, 151443]
- top entity types (max seen): minecraft:item×103555, minecraft:creeper×5241, minecraft:husk×5231, minecraft:spider×4851, minecraft:skeleton×4822, minecraft:zombie×4668, minecraft:drowned×4550, minecraft:sheep×3524, minecraft:chicken×3421, minecraft:cow×3333, minecraft:pig×3220, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/s1BNsUJbf0
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **10**)
- total pause: **21700.7 ms**, avg **179.34 ms**, max **2344.1 ms**
- heap high-water seen: **7742 MB** -> last-after: **3625 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115827)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28714 | 24.8% |
| kernel: other | 28354 | 24.5% |
| other | 15914 | 13.7% |
| chunk system (kernel) | 8967 | 7.7% |
| moonrise/paper patches | 8942 | 7.7% |
| fastutil collections | 7251 | 6.3% |
| JDK collections | 6198 | 5.4% |
| JIT stubs (vtable/itable) | 3624 | 3.1% |
| network (kernel) | 2490 | 2.1% |
| JDK invokes/VarHandle | 2232 | 1.9% |
| JDK other | 2047 | 1.8% |
| JVM internals (GC oop barriers) | 544 | 0.5% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 97 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| bukkit api | 60 | 0.1% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 45 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91472 | 79.0% |
| phase: unclassified | 14154 | 12.2% |
| phase: main tick (unclassified) | 3825 | 3.3% |
| phase: chunk tick | 2097 | 1.8% |
| phase: network sync (ServerEntity) | 1895 | 1.6% |
| phase: chunk system (off-main worker) | 1115 | 1.0% |
| phase: block entities (hoppers/furnaces) | 694 | 0.6% |
| phase: random tick | 436 | 0.4% |
| phase: mob spawning | 135 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98888** (85.4%) · native/JVM-internal **16852** (14.5%) · other **87** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4235 | 3.7% |
| `vtable stub` | native/JVM-internal | 3006 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2841 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2478 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1888 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1638 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1568 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1552 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1549 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1468 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1428 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1389 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1357 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1276 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1244 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1104 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1097 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1048 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 972 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 951 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 947 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 924 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 922 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 911 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 850 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 776 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 767 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 748 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 690 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 668 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 622 | 0.5% |
| `net/minecraft/world/entity/ItemBatchOps.append` | JVM-Java | 618 | 0.5% |
| `itable stub` | native/JVM-internal | 612 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 606 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61257)

| bucket | self-time samples | share |
|---|---|---|
| other | 57974 | 94.6% |
| entities/mobs (kernel) | 965 | 1.6% |
| kernel: other | 925 | 1.5% |
| moonrise/paper patches | 302 | 0.5% |
| chunk system (kernel) | 274 | 0.4% |
| fastutil collections | 215 | 0.4% |
| JDK collections | 204 | 0.3% |
| JIT stubs (vtable/itable) | 148 | 0.2% |
| network (kernel) | 82 | 0.1% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57787 | 94.3% |
| phase: entity tick (AI/movement) | 3003 | 4.9% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: chunk tick | 86 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52365** (85.5%) · native/JVM-internal **8887** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49142 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4754 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 132 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 100 | 0.2% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3765)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3765 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2052 | 54.5% |
| phase: unclassified | 1509 | 40.1% |
| phase: main tick (unclassified) | 105 | 2.8% |
| phase: chunk system (off-main worker) | 57 | 1.5% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3765** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 554 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 497 | 13.2% |
| `char[]_[k]` | other | 443 | 11.8% |
| `byte[]_[k]` | other | 199 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 190 | 5.0% |
| `long[]_[i]` | other | 177 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.5% |
| `java.util.ArrayList_[i]` | other | 124 | 3.3% |
| `byte[]_[i]` | other | 86 | 2.3% |
| `java.lang.Object[]_[i]` | other | 81 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.0% |
| `int[]_[i]` | other | 62 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.2% |
| `net.minecraft.core.SectionPos_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc1779dc000_[i]` | other | 39 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fc17790c690_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fc1779ef1f8_[i]` | other | 34 | 0.9% |
| `int[]_[k]` | other | 34 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115827 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 29436 | 25.41% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23933 | 20.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6768 | 5.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5830 | 5.03% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4623 | 3.99% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1289 | 1.11% |
| `net/minecraft/world/entity/ai/Brain.tick` | 976 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 490 | 0.42% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 554 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | 497 | 13.2% |
| `char[]_[k]` | 443 | 11.8% |
| `byte[]_[k]` | 199 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 190 | 5.0% |
| `long[]_[i]` | 177 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.5% |
| `java.util.ArrayList_[i]` | 124 | 3.3% |
| `byte[]_[i]` | 86 | 2.3% |
| `java.lang.Object[]_[i]` | 81 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 21701 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148207..151443 (delta 3236, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99856->103555, minecraft:drowned 3530->4550, minecraft:zombie 3698->4668, minecraft:husk 4528->5231, minecraft:creeper 4548->5241, minecraft:spider 4259->4851, minecraft:skeleton 4362->4822, minecraft:chicken 3387->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3236)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59596353 B)
- `wall-collapsed.txt` (3736256 B)
- `alloc-collapsed.txt` (2142540 B)
- `cpu-flamegraph.html` (284007 B)
- `server-stdout.log` (259223 B)
- `gc.log` (115308 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
