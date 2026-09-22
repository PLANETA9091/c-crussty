# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.836 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.4, 1.6, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **406.57ms** / min 337.53ms / max **527.12ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T08:28:02Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6435981 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 337.53 | — | — | — | 527.12 | 406.57 |

- entity totals seen: [148992, 150432, 151394]
- top entity types (max seen): minecraft:item×103331, minecraft:creeper×5213, minecraft:husk×5186, minecraft:spider×4907, minecraft:skeleton×4867, minecraft:zombie×4635, minecraft:drowned×4554, minecraft:sheep×3521, minecraft:chicken×3417, minecraft:cow×3363, minecraft:pig×3221, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/V7N9Q1kylE
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **21261.3 ms**, avg **178.67 ms**, max **2673.7 ms**
- heap high-water seen: **7564 MB** -> last-after: **3764 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 114697)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29022 | 25.3% |
| kernel: other | 28740 | 25.1% |
| other | 11806 | 10.3% |
| moonrise/paper patches | 10025 | 8.7% |
| chunk system (kernel) | 9800 | 8.5% |
| fastutil collections | 7281 | 6.3% |
| JDK collections | 6066 | 5.3% |
| JIT stubs (vtable/itable) | 3436 | 3.0% |
| network (kernel) | 3357 | 2.9% |
| JDK invokes/VarHandle | 2601 | 2.3% |
| JDK other | 2031 | 1.8% |
| vdso (clock) | 227 | 0.2% |
| bukkit api | 78 | 0.1% |
| block entities/hoppers (kernel) | 69 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 47 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94797 | 82.6% |
| phase: unclassified | 9844 | 8.6% |
| phase: main tick (unclassified) | 3650 | 3.2% |
| phase: chunk tick | 2116 | 1.8% |
| phase: network sync (ServerEntity) | 1881 | 1.6% |
| phase: chunk system (off-main worker) | 1116 | 1.0% |
| phase: block entities (hoppers/furnaces) | 708 | 0.6% |
| phase: random tick | 457 | 0.4% |
| phase: mob spawning | 126 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102932** (89.7%) · native/JVM-internal **11665** (10.2%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4530 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3411 | 3.0% |
| `vtable stub` | native/JVM-internal | 2910 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2684 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2107 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1754 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1746 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1724 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1584 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1559 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1510 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1495 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1486 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1397 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1397 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1342 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1212 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1132 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1006 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1006 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 995 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 977 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 973 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 971 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 899 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 868 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 853 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 804 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 793 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 734 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 690 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 685 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 642 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 634 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57866 | 94.5% |
| kernel: other | 933 | 1.5% |
| entities/mobs (kernel) | 913 | 1.5% |
| chunk system (kernel) | 317 | 0.5% |
| moonrise/paper patches | 304 | 0.5% |
| fastutil collections | 268 | 0.4% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 163 | 0.3% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 90 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 14 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57692 | 94.2% |
| phase: entity tick (AI/movement) | 3120 | 5.1% |
| phase: main tick (unclassified) | 208 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52364** (85.5%) · native/JVM-internal **8885** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49062 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 157 | 0.3% |
| `vtable stub` | native/JVM-internal | 132 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 55 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3558)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3558 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2001 | 56.2% |
| phase: unclassified | 1398 | 39.3% |
| phase: main tick (unclassified) | 80 | 2.2% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 20 | 0.6% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3558** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 555 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 515 | 14.5% |
| `char[]_[k]` | other | 434 | 12.2% |
| `byte[]_[k]` | other | 180 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 4.0% |
| `java.util.ArrayList_[i]` | other | 134 | 3.8% |
| `long[]_[i]` | other | 127 | 3.6% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `java.lang.Object[]_[i]` | other | 85 | 2.4% |
| `int[]_[i]` | other | 75 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f23ea9dc238_[i]` | other | 35 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f23ea9d1b08_[i]` | other | 27 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 26 | 0.7% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f23ea82eb78_[i]` | other | 24 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 114697 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35407 | 30.87% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22977 | 20.03% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6585 | 5.74% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5445 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4637 | 4.04% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1154 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 457 | 0.40% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 228 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 211 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 182 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 555 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 515 | 14.5% |
| `char[]_[k]` | 434 | 12.2% |
| `byte[]_[k]` | 180 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 4.0% |
| `java.util.ArrayList_[i]` | 134 | 3.8% |
| `long[]_[i]` | 127 | 3.6% |
| `byte[]_[i]` | 90 | 2.5% |
| `java.lang.Object[]_[i]` | 85 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 21261 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148148..151394 (delta 3246, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99695->103331, minecraft:zombie 3614->4635, minecraft:drowned 3554->4554, minecraft:creeper 4534->5213, minecraft:spider 4235->4907, minecraft:husk 4525->5186, minecraft:skeleton 4404->4867, minecraft:chicken 3386->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3246)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53767285 B)
- `wall-collapsed.txt` (3697784 B)
- `alloc-collapsed.txt` (2048158 B)
- `cpu-flamegraph.html` (301586 B)
- `server-stdout.log` (249388 B)
- `gc.log` (112719 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
