# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.122 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [12.3, 1.5, 1.8, 2.0, 2.5, 2.5]
- spark tick-monitor MSPT: avg **429.02ms** / min 368.02ms / max **608.24ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T00:38:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6931670 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 368.02 | — | — | — | 608.24 | 429.02 |

- entity totals seen: [149189, 150322, 151541]
- top entity types (max seen): minecraft:item×103449, minecraft:creeper×5208, minecraft:husk×5158, minecraft:skeleton×4906, minecraft:spider×4805, minecraft:zombie×4661, minecraft:drowned×4540, minecraft:sheep×3516, minecraft:chicken×3424, minecraft:cow×3373, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/SGz8BDVVHF
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **20770.3 ms**, avg **176.02 ms**, max **2447.9 ms**
- heap high-water seen: **7484 MB** -> last-after: **4203 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116342)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28673 | 24.6% |
| kernel: other | 28107 | 24.2% |
| other | 14490 | 12.5% |
| moonrise/paper patches | 10669 | 9.2% |
| chunk system (kernel) | 8770 | 7.5% |
| fastutil collections | 7263 | 6.2% |
| JDK collections | 5932 | 5.1% |
| JIT stubs (vtable/itable) | 3598 | 3.1% |
| network (kernel) | 2966 | 2.5% |
| JDK invokes/VarHandle | 2846 | 2.4% |
| JDK other | 1915 | 1.6% |
| JVM internals (GC oop barriers) | 564 | 0.5% |
| vdso (clock) | 237 | 0.2% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| bukkit api | 77 | 0.1% |
| craftbukkit glue | 70 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93068 | 80.0% |
| phase: unclassified | 13405 | 11.5% |
| phase: main tick (unclassified) | 3576 | 3.1% |
| phase: chunk tick | 2220 | 1.9% |
| phase: network sync (ServerEntity) | 1687 | 1.5% |
| phase: chunk system (off-main worker) | 1155 | 1.0% |
| phase: block entities (hoppers/furnaces) | 722 | 0.6% |
| phase: random tick | 385 | 0.3% |
| phase: mob spawning | 119 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100859** (86.7%) · native/JVM-internal **15397** (13.2%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4075 | 3.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3687 | 3.2% |
| `vtable stub` | native/JVM-internal | 3054 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2877 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2040 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2036 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1712 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1670 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1660 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1549 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1487 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1443 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1400 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1323 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1312 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1164 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1138 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1028 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 979 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 978 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 947 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 944 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 915 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 834 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 821 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 815 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 813 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 776 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 772 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 731 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 729 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 706 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 656 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61240)

| bucket | self-time samples | share |
|---|---|---|
| other | 57850 | 94.5% |
| entities/mobs (kernel) | 1024 | 1.7% |
| kernel: other | 943 | 1.5% |
| moonrise/paper patches | 317 | 0.5% |
| chunk system (kernel) | 255 | 0.4% |
| fastutil collections | 247 | 0.4% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 101 | 0.2% |
| JDK invokes/VarHandle | 94 | 0.2% |
| JDK other | 61 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57661 | 94.2% |
| phase: entity tick (AI/movement) | 3129 | 5.1% |
| phase: main tick (unclassified) | 210 | 0.3% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 54 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52350** (85.5%) · native/JVM-internal **8887** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48998 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 112 | 0.2% |
| `vtable stub` | native/JVM-internal | 109 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 102 | 0.2% |
| `syscall` | native/JVM-internal | 74 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 68 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3576)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3576 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1996 | 55.8% |
| phase: unclassified | 1422 | 39.8% |
| phase: main tick (unclassified) | 86 | 2.4% |
| phase: chunk system (off-main worker) | 45 | 1.3% |
| phase: network sync (ServerEntity) | 15 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3576** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 521 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 516 | 14.4% |
| `char[]_[k]` | other | 420 | 11.7% |
| `byte[]_[k]` | other | 209 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 168 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 148 | 4.1% |
| `long[]_[i]` | other | 145 | 4.1% |
| `java.util.ArrayList_[i]` | other | 114 | 3.2% |
| `byte[]_[i]` | other | 104 | 2.9% |
| `java.lang.Object[]_[i]` | other | 84 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 78 | 2.2% |
| `int[]_[i]` | other | 74 | 2.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 36 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8b0f9db6e0_[i]` | other | 31 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116342 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34402 | 29.57% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22790 | 19.59% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6490 | 5.58% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5446 | 4.68% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4633 | 3.98% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1132 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 951 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 454 | 0.39% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 241 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 521 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 516 | 14.4% |
| `char[]_[k]` | 420 | 11.7% |
| `byte[]_[k]` | 209 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 168 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 148 | 4.1% |
| `long[]_[i]` | 145 | 4.1% |
| `java.util.ArrayList_[i]` | 114 | 3.2% |
| `byte[]_[i]` | 104 | 2.9% |
| `java.lang.Object[]_[i]` | 84 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 20770 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148273..151541 (delta 3268, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99722->103449, minecraft:drowned 3408->4540, minecraft:zombie 3674->4661, minecraft:husk 4505->5158, minecraft:creeper 4557->5208, minecraft:spider 4209->4805, minecraft:skeleton 4472->4906, minecraft:chicken 3398->3424
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3268)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57012238 B)
- `wall-collapsed.txt` (3766725 B)
- `alloc-collapsed.txt` (2035506 B)
- `cpu-flamegraph.html` (304898 B)
- `server-stdout.log` (255512 B)
- `gc.log` (111858 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
