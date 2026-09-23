# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.426 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 1.5, 1.9, 2.1, 2.5, 2.5]
- spark tick-monitor MSPT: avg **418.85ms** / min 342.26ms / max **538.77ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T17:40:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6969636 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 342.26 | — | — | — | 538.77 | 418.85 |

- entity totals seen: [148996, 150127, 151307]
- top entity types (max seen): minecraft:item×103229, minecraft:creeper×5235, minecraft:husk×5166, minecraft:skeleton×4852, minecraft:spider×4762, minecraft:zombie×4659, minecraft:drowned×4534, minecraft:sheep×3522, minecraft:chicken×3432, minecraft:cow×3399, minecraft:pig×3254, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/3JRByVybBs
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **10**)
- total pause: **23136.8 ms**, avg **199.46 ms**, max **2435.7 ms**
- heap high-water seen: **7475 MB** -> last-after: **5130 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115933)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28790 | 24.8% |
| kernel: other | 27176 | 23.4% |
| other | 15328 | 13.2% |
| moonrise/paper patches | 9883 | 8.5% |
| chunk system (kernel) | 9447 | 8.1% |
| fastutil collections | 7121 | 6.1% |
| JDK collections | 5908 | 5.1% |
| JIT stubs (vtable/itable) | 3423 | 3.0% |
| network (kernel) | 3169 | 2.7% |
| JDK invokes/VarHandle | 2647 | 2.3% |
| JDK other | 1964 | 1.7% |
| JVM internals (GC oop barriers) | 569 | 0.5% |
| vdso (clock) | 205 | 0.2% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| bukkit api | 63 | 0.1% |
| redstone (kernel) | 51 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91140 | 78.6% |
| phase: unclassified | 14364 | 12.4% |
| phase: main tick (unclassified) | 4055 | 3.5% |
| phase: chunk tick | 2074 | 1.8% |
| phase: network sync (ServerEntity) | 1746 | 1.5% |
| phase: chunk system (off-main worker) | 1377 | 1.2% |
| phase: block entities (hoppers/furnaces) | 645 | 0.6% |
| phase: random tick | 417 | 0.4% |
| phase: mob spawning | 112 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99534** (85.9%) · native/JVM-internal **16305** (14.1%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4381 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3550 | 3.1% |
| `vtable stub` | native/JVM-internal | 2848 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2469 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2018 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1727 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1717 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1672 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1624 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1606 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1565 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1549 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1449 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1418 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1332 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1224 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1131 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 967 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 935 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 928 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 913 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 911 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 880 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 824 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 822 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007f57729e9008.accept` | JVM-Java | 744 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 723 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 721 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 717 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 712 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 675 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 665 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 654 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57915 | 94.6% |
| entities/mobs (kernel) | 1013 | 1.7% |
| kernel: other | 897 | 1.5% |
| moonrise/paper patches | 318 | 0.5% |
| chunk system (kernel) | 275 | 0.4% |
| fastutil collections | 227 | 0.4% |
| JDK collections | 201 | 0.3% |
| JIT stubs (vtable/itable) | 122 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK invokes/VarHandle | 86 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57729 | 94.2% |
| phase: entity tick (AI/movement) | 3085 | 5.0% |
| phase: main tick (unclassified) | 196 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52366** (85.5%) · native/JVM-internal **8880** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49051 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4754 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 99 | 0.2% |
| `syscall` | native/JVM-internal | 94 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 60 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3747)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3747 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2056 | 54.9% |
| phase: unclassified | 1513 | 40.4% |
| phase: main tick (unclassified) | 83 | 2.2% |
| phase: chunk system (off-main worker) | 60 | 1.6% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3747** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 548 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 533 | 14.2% |
| `char[]_[k]` | other | 436 | 11.6% |
| `byte[]_[k]` | other | 205 | 5.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 170 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 150 | 4.0% |
| `long[]_[i]` | other | 133 | 3.5% |
| `java.util.ArrayList_[i]` | other | 133 | 3.5% |
| `java.lang.Object[]_[i]` | other | 96 | 2.6% |
| `byte[]_[i]` | other | 90 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 67 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 62 | 1.7% |
| `int[]_[i]` | other | 58 | 1.5% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 41 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 34 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f577283b490_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115933 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34641 | 29.88% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22066 | 19.03% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6393 | 5.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5291 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4278 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1077 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 934 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 437 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 238 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 218 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 548 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | 533 | 14.2% |
| `char[]_[k]` | 436 | 11.6% |
| `byte[]_[k]` | 205 | 5.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 170 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 150 | 4.0% |
| `long[]_[i]` | 133 | 3.5% |
| `java.util.ArrayList_[i]` | 133 | 3.5% |
| `java.lang.Object[]_[i]` | 96 | 2.6% |
| `byte[]_[i]` | 90 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 23137 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148141..151307 (delta 3166, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99574->103229, minecraft:drowned 3426->4534, minecraft:zombie 3629->4659, minecraft:husk 4505->5166, minecraft:creeper 4592->5235, minecraft:spider 4199->4762, minecraft:skeleton 4482->4852, minecraft:pig 3220->3254
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3166)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57357814 B)
- `wall-collapsed.txt` (3655206 B)
- `alloc-collapsed.txt` (2052173 B)
- `cpu-flamegraph.html` (305151 B)
- `server-stdout.log` (251060 B)
- `gc.log` (111037 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
