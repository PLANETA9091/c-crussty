# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 19.194 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.8, 1.5, 1.9, 2.1, 2.2, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:54:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6431555 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149055, 150246, 151325]
- top entity types (max seen): minecraft:item×103378, minecraft:husk×5187, minecraft:creeper×5144, minecraft:skeleton×4871, minecraft:spider×4816, minecraft:zombie×4684, minecraft:drowned×4579, minecraft:sheep×3495, minecraft:chicken×3438, minecraft:cow×3368, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/vzrub8zzO8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **10**)
- total pause: **26765.3 ms**, avg **215.85 ms**, max **2644.5 ms**
- heap high-water seen: **7357 MB** -> last-after: **4091 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115743)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27304 | 23.6% |
| entities/mobs (kernel) | 26894 | 23.2% |
| other | 15477 | 13.4% |
| moonrise/paper patches | 10741 | 9.3% |
| chunk system (kernel) | 10660 | 9.2% |
| fastutil collections | 6872 | 5.9% |
| JDK collections | 5935 | 5.1% |
| network (kernel) | 3234 | 2.8% |
| JIT stubs (vtable/itable) | 2856 | 2.5% |
| JDK invokes/VarHandle | 2813 | 2.4% |
| JDK other | 1875 | 1.6% |
| JVM internals (GC oop barriers) | 578 | 0.5% |
| vdso (clock) | 238 | 0.2% |
| block entities/hoppers (kernel) | 93 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| bukkit api | 50 | 0.0% |
| redstone (kernel) | 37 | 0.0% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90192 | 77.9% |
| phase: unclassified | 14987 | 12.9% |
| phase: main tick (unclassified) | 3731 | 3.2% |
| phase: chunk tick | 2242 | 1.9% |
| phase: network sync (ServerEntity) | 2049 | 1.8% |
| phase: chunk system (off-main worker) | 1235 | 1.1% |
| phase: block entities (hoppers/furnaces) | 758 | 0.7% |
| phase: random tick | 432 | 0.4% |
| phase: mob spawning | 114 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99628** (86.1%) · native/JVM-internal **16062** (13.9%) · other **53** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5019 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3675 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2515 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2410 | 2.1% |
| `vtable stub` | native/JVM-internal | 2345 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2159 | 1.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1980 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1893 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1823 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1805 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1393 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1390 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1351 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1272 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1219 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1184 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1149 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1148 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1077 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1065 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1062 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1055 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1046 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 978 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 951 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 900 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 840 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 776 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 723 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 723 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 684 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 641 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 640 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 58047 | 94.8% |
| entities/mobs (kernel) | 870 | 1.4% |
| kernel: other | 840 | 1.4% |
| moonrise/paper patches | 323 | 0.5% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 240 | 0.4% |
| JDK collections | 182 | 0.3% |
| network (kernel) | 111 | 0.2% |
| JIT stubs (vtable/itable) | 110 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JVM internals (GC oop barriers) | 61 | 0.1% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57858 | 94.5% |
| phase: entity tick (AI/movement) | 2924 | 4.8% |
| phase: main tick (unclassified) | 188 | 0.3% |
| phase: chunk tick | 130 | 0.2% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51883** (84.7%) · native/JVM-internal **9363** (15.3%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48769 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 301 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `vtable stub` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 58 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 49 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3660)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3660 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2157 | 58.9% |
| phase: unclassified | 1345 | 36.7% |
| phase: main tick (unclassified) | 86 | 2.3% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: chunk system (off-main worker) | 26 | 0.7% |
| phase: chunk tick | 9 | 0.2% |
| phase: block entities (hoppers/furnaces) | 4 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3660** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 611 | 16.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 489 | 13.4% |
| `char[]_[k]` | other | 449 | 12.3% |
| `byte[]_[k]` | other | 251 | 6.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 154 | 4.2% |
| `long[]_[i]` | other | 133 | 3.6% |
| `java.util.ArrayList_[i]` | other | 89 | 2.4% |
| `java.lang.Object[]_[i]` | other | 85 | 2.3% |
| `int[]_[i]` | other | 82 | 2.2% |
| `byte[]_[i]` | other | 79 | 2.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 45 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 37 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f57df9de8e0_[i]` | other | 30 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 26 | 0.7% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f57df9d6618_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115743 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33987 | 29.36% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21988 | 19.00% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6274 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5226 | 4.52% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4292 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 919 | 0.79% |
| `net/minecraft/world/entity/ai/Brain.tick` | 865 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 428 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 255 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 203 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 611 | 16.7% |
| `net.minecraft.world.phys.AABB_[i]` | 489 | 13.4% |
| `char[]_[k]` | 449 | 12.3% |
| `byte[]_[k]` | 251 | 6.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 154 | 4.2% |
| `long[]_[i]` | 133 | 3.6% |
| `java.util.ArrayList_[i]` | 89 | 2.4% |
| `java.lang.Object[]_[i]` | 85 | 2.3% |
| `int[]_[i]` | 82 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 26765 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148188..151325 (delta 3137, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99624->103378, minecraft:drowned 3490->4579, minecraft:zombie 3679->4684, minecraft:husk 4511->5187, minecraft:spider 4207->4816, minecraft:creeper 4546->5144, minecraft:skeleton 4407->4871, minecraft:chicken 3410->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3137)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58149219 B)
- `wall-collapsed.txt` (3463319 B)
- `alloc-collapsed.txt` (2052793 B)
- `cpu-flamegraph.html` (292700 B)
- `server-stdout.log` (251626 B)
- `gc.log` (117958 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
