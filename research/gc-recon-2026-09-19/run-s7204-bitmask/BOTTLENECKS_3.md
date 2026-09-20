# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.601 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.7, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **410.87ms** / min 341.83ms / max **529.51ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T15:04:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6687429 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 1 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
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
| spark tickmonitor (whole run, [⚡] lines) | 341.83 | — | — | — | 529.51 | 410.87 |

- entity totals seen: [149104, 150382, 151510]
- top entity types (max seen): minecraft:item×103378, minecraft:creeper×5207, minecraft:husk×5144, minecraft:skeleton×4877, minecraft:spider×4863, minecraft:zombie×4694, minecraft:drowned×4595, minecraft:sheep×3532, minecraft:chicken×3419, minecraft:cow×3344, minecraft:pig×3195, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gpfDk3Usyn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **8**)
- total pause: **18890.7 ms**, avg **160.09 ms**, max **2270.7 ms**
- heap high-water seen: **7914 MB** -> last-after: **4647 MB**
  - Young (Allocation Failure): 100
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115122)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28407 | 24.7% |
| kernel: other | 28399 | 24.7% |
| other | 12219 | 10.6% |
| moonrise/paper patches | 10574 | 9.2% |
| chunk system (kernel) | 9934 | 8.6% |
| fastutil collections | 7000 | 6.1% |
| JDK collections | 5851 | 5.1% |
| JIT stubs (vtable/itable) | 3665 | 3.2% |
| network (kernel) | 3149 | 2.7% |
| JDK invokes/VarHandle | 2702 | 2.3% |
| JDK other | 2180 | 1.9% |
| JVM internals (GC oop barriers) | 549 | 0.5% |
| vdso (clock) | 219 | 0.2% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| bukkit api | 62 | 0.1% |
| craftbukkit glue | 52 | 0.0% |
| redstone (kernel) | 34 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94298 | 81.9% |
| phase: unclassified | 10858 | 9.4% |
| phase: main tick (unclassified) | 3625 | 3.1% |
| phase: chunk tick | 2176 | 1.9% |
| phase: network sync (ServerEntity) | 1765 | 1.5% |
| phase: chunk system (off-main worker) | 1179 | 1.0% |
| phase: block entities (hoppers/furnaces) | 673 | 0.6% |
| phase: random tick | 428 | 0.4% |
| phase: mob spawning | 114 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101997** (88.6%) · native/JVM-internal **13032** (11.3%) · other **93** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4741 | 4.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3299 | 2.9% |
| `vtable stub` | native/JVM-internal | 3026 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2565 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2092 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1780 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1757 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1662 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1660 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1550 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1538 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1534 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1522 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1411 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1269 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1241 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1186 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1127 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1030 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 985 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 940 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 930 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 880 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 867 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 865 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 841 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 810 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 752 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 750 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 731 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 725 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 702 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 683 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 668 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57870 | 94.5% |
| entities/mobs (kernel) | 963 | 1.6% |
| kernel: other | 899 | 1.5% |
| moonrise/paper patches | 372 | 0.6% |
| chunk system (kernel) | 294 | 0.5% |
| fastutil collections | 263 | 0.4% |
| JDK collections | 180 | 0.3% |
| JIT stubs (vtable/itable) | 138 | 0.2% |
| network (kernel) | 115 | 0.2% |
| JDK other | 76 | 0.1% |
| JDK invokes/VarHandle | 65 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57702 | 94.2% |
| phase: entity tick (AI/movement) | 3134 | 5.1% |
| phase: main tick (unclassified) | 193 | 0.3% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52389** (85.5%) · native/JVM-internal **8865** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49058 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 125 | 0.2% |
| `vtable stub` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 103 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3657)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3657 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2073 | 56.7% |
| phase: unclassified | 1411 | 38.6% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 49 | 1.3% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 15 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3657** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 532 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 524 | 14.3% |
| `char[]_[k]` | other | 451 | 12.3% |
| `byte[]_[k]` | other | 193 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 192 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 4.4% |
| `long[]_[i]` | other | 137 | 3.7% |
| `java.util.ArrayList_[i]` | other | 113 | 3.1% |
| `int[]_[i]` | other | 88 | 2.4% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `java.lang.Object[]_[i]` | other | 82 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 60 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 52 | 1.4% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 40 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f24c68406c8_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115122 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34846 | 30.27% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23383 | 20.31% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6649 | 5.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5483 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4681 | 4.07% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1243 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 867 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 437 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 193 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 181 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 532 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 524 | 14.3% |
| `char[]_[k]` | 451 | 12.3% |
| `byte[]_[k]` | 193 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 192 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 4.4% |
| `long[]_[i]` | 137 | 3.7% |
| `java.util.ArrayList_[i]` | 113 | 3.1% |
| `int[]_[i]` | 88 | 2.4% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 18891 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148221..151510 (delta 3289, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99702->103378, minecraft:drowned 3444->4595, minecraft:zombie 3757->4694, minecraft:creeper 4546->5207, minecraft:husk 4506->5144, minecraft:spider 4264->4863, minecraft:skeleton 4426->4877, minecraft:chicken 3401->3419
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3289)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57444069 B)
- `wall-collapsed.txt` (3778382 B)
- `alloc-collapsed.txt` (2012859 B)
- `cpu-flamegraph.html` (303847 B)
- `server-stdout.log` (249181 B)
- `gc.log` (110959 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
