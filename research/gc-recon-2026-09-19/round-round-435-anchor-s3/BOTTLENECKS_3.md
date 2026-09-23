# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.92 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.8, 1.6, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **401.0ms** / min 338.0ms / max **535.54ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:52:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6761558 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 338.0 | — | — | — | 535.54 | 401.0 |

- entity totals seen: [148964, 150238, 151358]
- top entity types (max seen): minecraft:item×103275, minecraft:creeper×5217, minecraft:husk×5148, minecraft:spider×4900, minecraft:skeleton×4862, minecraft:zombie×4674, minecraft:drowned×4555, minecraft:sheep×3497, minecraft:chicken×3429, minecraft:cow×3385, minecraft:pig×3244, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/qWg8WJcvVK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **23954.7 ms**, avg **197.97 ms**, max **2768.4 ms**
- heap high-water seen: **7569 MB** -> last-after: **4283 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 116334)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27139 | 23.3% |
| kernel: other | 26375 | 22.7% |
| other | 15474 | 13.3% |
| chunk system (kernel) | 11057 | 9.5% |
| moonrise/paper patches | 10522 | 9.0% |
| fastutil collections | 7241 | 6.2% |
| JDK collections | 6080 | 5.2% |
| network (kernel) | 3604 | 3.1% |
| JIT stubs (vtable/itable) | 3053 | 2.6% |
| JDK invokes/VarHandle | 2645 | 2.3% |
| JDK other | 1996 | 1.7% |
| JVM internals (GC oop barriers) | 593 | 0.5% |
| vdso (clock) | 243 | 0.2% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| bukkit api | 67 | 0.1% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91115 | 78.3% |
| phase: unclassified | 14875 | 12.8% |
| phase: main tick (unclassified) | 3640 | 3.1% |
| phase: chunk tick | 2137 | 1.8% |
| phase: network sync (ServerEntity) | 2054 | 1.8% |
| phase: chunk system (off-main worker) | 1209 | 1.0% |
| phase: block entities (hoppers/furnaces) | 728 | 0.6% |
| phase: random tick | 422 | 0.4% |
| phase: mob spawning | 149 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100013** (86.0%) · native/JVM-internal **16218** (13.9%) · other **103** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5164 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3544 | 3.0% |
| `vtable stub` | native/JVM-internal | 2463 | 2.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2427 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2273 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2024 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1944 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1928 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1682 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1663 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1613 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1505 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1449 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1388 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1271 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1255 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1235 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1188 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1175 | 1.0% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1168 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1135 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1129 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1022 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 995 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 977 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 938 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 887 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 869 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 748 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 742 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 719 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 697 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 689 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 657 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57939 | 94.6% |
| entities/mobs (kernel) | 916 | 1.5% |
| kernel: other | 888 | 1.4% |
| moonrise/paper patches | 331 | 0.5% |
| chunk system (kernel) | 324 | 0.5% |
| fastutil collections | 234 | 0.4% |
| JDK collections | 192 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 124 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 71 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57767 | 94.3% |
| phase: entity tick (AI/movement) | 3062 | 5.0% |
| phase: main tick (unclassified) | 174 | 0.3% |
| phase: chunk tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: random tick | 30 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52326** (85.4%) · native/JVM-internal **8920** (14.6%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49051 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4753 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `syscall` | native/JVM-internal | 108 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3785)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3785 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2140 | 56.5% |
| phase: unclassified | 1449 | 38.3% |
| phase: main tick (unclassified) | 96 | 2.5% |
| phase: chunk system (off-main worker) | 48 | 1.3% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: chunk tick | 13 | 0.3% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3785** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 562 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 547 | 14.5% |
| `char[]_[k]` | other | 431 | 11.4% |
| `byte[]_[k]` | other | 220 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 176 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.8% |
| `long[]_[i]` | other | 142 | 3.8% |
| `java.util.ArrayList_[i]` | other | 119 | 3.1% |
| `byte[]_[i]` | other | 108 | 2.9% |
| `java.lang.Object[]_[i]` | other | 105 | 2.8% |
| `int[]_[i]` | other | 70 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 53 | 1.4% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 51 | 1.3% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f77b29ee2a0_[i]` | other | 32 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116334 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34350 | 29.53% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21940 | 18.86% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6329 | 5.44% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5308 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4420 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 987 | 0.85% |
| `net/minecraft/world/entity/ai/Brain.tick` | 897 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 421 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 202 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 190 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 562 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 547 | 14.5% |
| `char[]_[k]` | 431 | 11.4% |
| `byte[]_[k]` | 220 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 176 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.8% |
| `long[]_[i]` | 142 | 3.8% |
| `java.util.ArrayList_[i]` | 119 | 3.1% |
| `byte[]_[i]` | 108 | 2.9% |
| `java.lang.Object[]_[i]` | 105 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 23955 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148073..151358 (delta 3285, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99570->103275, minecraft:zombie 3588->4674, minecraft:drowned 3531->4555, minecraft:creeper 4539->5217, minecraft:spider 4252->4900, minecraft:husk 4507->5148, minecraft:skeleton 4383->4862, minecraft:chicken 3394->3429
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3285)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52759903 B)
- `wall-collapsed.txt` (3626774 B)
- `alloc-collapsed.txt` (2124393 B)
- `cpu-flamegraph.html` (290778 B)
- `server-stdout.log` (252268 B)
- `gc.log` (114457 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
