# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.438 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.9, 1.6, 1.9, 2.3, 2.5, 2.5]
- spark tick-monitor MSPT: avg **412.8ms** / min 346.1ms / max **528.07ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T19:04:09Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6841181 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.1 | — | — | — | 528.07 | 412.8 |

- entity totals seen: [148899, 150341, 151336]
- top entity types (max seen): minecraft:item×103317, minecraft:creeper×5235, minecraft:husk×5170, minecraft:spider×4862, minecraft:skeleton×4812, minecraft:zombie×4638, minecraft:drowned×4524, minecraft:sheep×3521, minecraft:chicken×3425, minecraft:cow×3394, minecraft:pig×3237, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XMUMAuWM4e
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **24156.8 ms**, avg **196.40 ms**, max **2499.7 ms**
- heap high-water seen: **7560 MB** -> last-after: **5506 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117107)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28881 | 24.7% |
| kernel: other | 28018 | 23.9% |
| other | 15169 | 13.0% |
| moonrise/paper patches | 10018 | 8.6% |
| chunk system (kernel) | 9753 | 8.3% |
| fastutil collections | 6769 | 5.8% |
| JDK collections | 6388 | 5.5% |
| JIT stubs (vtable/itable) | 3465 | 3.0% |
| network (kernel) | 3341 | 2.9% |
| JDK invokes/VarHandle | 2288 | 2.0% |
| JDK other | 1922 | 1.6% |
| JVM internals (GC oop barriers) | 551 | 0.5% |
| vdso (clock) | 224 | 0.2% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| worldgen/noise (kernel) | 42 | 0.0% |
| redstone (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93265 | 79.6% |
| phase: unclassified | 13972 | 11.9% |
| phase: main tick (unclassified) | 3744 | 3.2% |
| phase: chunk tick | 1938 | 1.7% |
| phase: network sync (ServerEntity) | 1823 | 1.6% |
| phase: chunk system (off-main worker) | 1152 | 1.0% |
| phase: block entities (hoppers/furnaces) | 674 | 0.6% |
| phase: random tick | 404 | 0.3% |
| phase: mob spawning | 135 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101136** (86.4%) · native/JVM-internal **15868** (13.6%) · other **103** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4465 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3606 | 3.1% |
| `vtable stub` | native/JVM-internal | 2882 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2564 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1986 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1835 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1832 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1806 | 1.5% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1660 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1607 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1572 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1455 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1436 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1405 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1308 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1213 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1157 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1039 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 997 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 898 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 893 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 866 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 853 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 846 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 834 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 804 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 803 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 794 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 745 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 737 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 634 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 633 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 631 | 0.5% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 629 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57894 | 94.5% |
| kernel: other | 947 | 1.5% |
| entities/mobs (kernel) | 940 | 1.5% |
| moonrise/paper patches | 321 | 0.5% |
| chunk system (kernel) | 314 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 206 | 0.3% |
| JIT stubs (vtable/itable) | 149 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 61 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57687 | 94.2% |
| phase: entity tick (AI/movement) | 3121 | 5.1% |
| phase: main tick (unclassified) | 202 | 0.3% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 59 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52354** (85.5%) · native/JVM-internal **8896** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49039 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 154 | 0.3% |
| `vtable stub` | native/JVM-internal | 116 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `syscall` | native/JVM-internal | 86 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3660)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3660 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2034 | 55.6% |
| phase: unclassified | 1464 | 40.0% |
| phase: main tick (unclassified) | 96 | 2.6% |
| phase: chunk system (off-main worker) | 36 | 1.0% |
| phase: network sync (ServerEntity) | 11 | 0.3% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3660** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 541 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 497 | 13.6% |
| `char[]_[k]` | other | 439 | 12.0% |
| `byte[]_[k]` | other | 196 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 4.3% |
| `long[]_[i]` | other | 119 | 3.3% |
| `java.util.ArrayList_[i]` | other | 114 | 3.1% |
| `java.lang.Object[]_[i]` | other | 101 | 2.8% |
| `byte[]_[i]` | other | 94 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 61 | 1.7% |
| `int[]_[i]` | other | 58 | 1.6% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 48 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd9829f2cd8_[i]` | other | 34 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117107 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34605 | 29.55% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22679 | 19.37% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6401 | 5.47% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5402 | 4.61% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4448 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1101 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 906 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 408 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 238 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 215 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 208 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 541 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 497 | 13.6% |
| `char[]_[k]` | 439 | 12.0% |
| `byte[]_[k]` | 196 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 4.3% |
| `long[]_[i]` | 119 | 3.3% |
| `java.util.ArrayList_[i]` | 114 | 3.1% |
| `java.lang.Object[]_[i]` | 101 | 2.8% |
| `byte[]_[i]` | 94 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 24157 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148080..151336 (delta 3256, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99696->103317, minecraft:drowned 3480->4524, minecraft:zombie 3637->4638, minecraft:husk 4499->5170, minecraft:creeper 4595->5235, minecraft:spider 4234->4862, minecraft:skeleton 4358->4812, minecraft:chicken 3397->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3256)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57066711 B)
- `wall-collapsed.txt` (3760398 B)
- `alloc-collapsed.txt` (2101817 B)
- `cpu-flamegraph.html` (297085 B)
- `server-stdout.log` (258045 B)
- `gc.log` (117074 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
