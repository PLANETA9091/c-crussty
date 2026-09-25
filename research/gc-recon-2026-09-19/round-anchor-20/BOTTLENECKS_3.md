# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.896 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.3, 1.7, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **410.34ms** / min 343.37ms / max **598.45ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:20:00Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7018724 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.37 | — | — | — | 598.45 | 410.34 |

- entity totals seen: [148971, 150283, 151362]
- top entity types (max seen): minecraft:item×103329, minecraft:creeper×5214, minecraft:husk×5151, minecraft:skeleton×4835, minecraft:spider×4824, minecraft:zombie×4625, minecraft:drowned×4514, minecraft:sheep×3523, minecraft:chicken×3432, minecraft:cow×3386, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/G6kgEZ81PR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **20715.3 ms**, avg **172.63 ms**, max **2449.3 ms**
- heap high-water seen: **7538 MB** -> last-after: **4274 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116618)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28770 | 24.7% |
| kernel: other | 27855 | 23.9% |
| other | 15355 | 13.2% |
| moonrise/paper patches | 9485 | 8.1% |
| chunk system (kernel) | 9354 | 8.0% |
| fastutil collections | 7010 | 6.0% |
| JDK collections | 6219 | 5.3% |
| JIT stubs (vtable/itable) | 3650 | 3.1% |
| network (kernel) | 3212 | 2.8% |
| JDK invokes/VarHandle | 2487 | 2.1% |
| JDK other | 2112 | 1.8% |
| JVM internals (GC oop barriers) | 570 | 0.5% |
| vdso (clock) | 227 | 0.2% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| bukkit api | 63 | 0.1% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92743 | 79.5% |
| phase: unclassified | 13805 | 11.8% |
| phase: main tick (unclassified) | 3458 | 3.0% |
| phase: chunk tick | 1955 | 1.7% |
| phase: network sync (ServerEntity) | 1805 | 1.5% |
| phase: chunk system (off-main worker) | 1518 | 1.3% |
| phase: block entities (hoppers/furnaces) | 754 | 0.6% |
| phase: random tick | 445 | 0.4% |
| phase: mob spawning | 131 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100638** (86.3%) · native/JVM-internal **15887** (13.6%) · other **93** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4526 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3274 | 2.8% |
| `vtable stub` | native/JVM-internal | 3063 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2527 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1861 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1716 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1704 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1659 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1559 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1558 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1549 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1517 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1499 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1464 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1383 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1279 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1248 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1086 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1071 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1051 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1007 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 981 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 904 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 867 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 867 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 824 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 802 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 802 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 769 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 766 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 670 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 634 | 0.5% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 624 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61247)

| bucket | self-time samples | share |
|---|---|---|
| other | 57857 | 94.5% |
| entities/mobs (kernel) | 1028 | 1.7% |
| kernel: other | 934 | 1.5% |
| moonrise/paper patches | 318 | 0.5% |
| chunk system (kernel) | 287 | 0.5% |
| JDK collections | 203 | 0.3% |
| fastutil collections | 186 | 0.3% |
| JIT stubs (vtable/itable) | 137 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK invokes/VarHandle | 95 | 0.2% |
| JDK other | 81 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57663 | 94.1% |
| phase: entity tick (AI/movement) | 3125 | 5.1% |
| phase: main tick (unclassified) | 197 | 0.3% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 60 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52384** (85.5%) · native/JVM-internal **8861** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49015 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `vtable stub` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 59 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3659)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3659 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2028 | 55.4% |
| phase: unclassified | 1450 | 39.6% |
| phase: main tick (unclassified) | 95 | 2.6% |
| phase: chunk system (off-main worker) | 48 | 1.3% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3659** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 548 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 514 | 14.0% |
| `char[]_[k]` | other | 444 | 12.1% |
| `byte[]_[k]` | other | 192 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 173 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 165 | 4.5% |
| `long[]_[i]` | other | 132 | 3.6% |
| `java.util.ArrayList_[i]` | other | 104 | 2.8% |
| `java.lang.Object[]_[i]` | other | 95 | 2.6% |
| `byte[]_[i]` | other | 76 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 70 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.3% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 43 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007ff94a9def10_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 27 | 0.7% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 26 | 0.7% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007ff94a82ab78_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116618 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34688 | 29.74% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22531 | 19.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6273 | 5.38% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5493 | 4.71% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4558 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1109 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 879 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 450 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 210 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 207 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 181 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 548 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | 514 | 14.0% |
| `char[]_[k]` | 444 | 12.1% |
| `byte[]_[k]` | 192 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | 173 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 165 | 4.5% |
| `long[]_[i]` | 132 | 3.6% |
| `java.util.ArrayList_[i]` | 104 | 2.8% |
| `java.lang.Object[]_[i]` | 95 | 2.6% |
| `byte[]_[i]` | 76 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 20715 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148128..151362 (delta 3234, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99730->103329, minecraft:drowned 3445->4514, minecraft:zombie 3634->4625, minecraft:husk 4483->5151, minecraft:creeper 4594->5214, minecraft:spider 4210->4824, minecraft:skeleton 4440->4835, minecraft:chicken 3394->3432
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3234)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56531710 B)
- `wall-collapsed.txt` (3759788 B)
- `alloc-collapsed.txt` (2086828 B)
- `cpu-flamegraph.html` (312101 B)
- `server-stdout.log` (251106 B)
- `gc.log` (113555 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
