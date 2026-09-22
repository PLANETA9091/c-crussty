# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.934 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 1.9, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **406.5ms** / min 353.9ms / max **550.94ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T20:59:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7098787 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 353.9 | — | — | — | 550.94 | 406.5 |

- entity totals seen: [148966, 150355, 151358]
- top entity types (max seen): minecraft:item×103214, minecraft:creeper×5193, minecraft:husk×5193, minecraft:skeleton×4862, minecraft:spider×4859, minecraft:zombie×4701, minecraft:drowned×4581, minecraft:sheep×3509, minecraft:chicken×3416, minecraft:cow×3373, minecraft:pig×3246, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/yC6e0OuK5X
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **10**)
- total pause: **24687.6 ms**, avg **202.36 ms**, max **2468.3 ms**
- heap high-water seen: **7674 MB** -> last-after: **5915 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116563)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28367 | 24.3% |
| kernel: other | 28259 | 24.2% |
| other | 14206 | 12.2% |
| moonrise/paper patches | 10124 | 8.7% |
| chunk system (kernel) | 9784 | 8.4% |
| fastutil collections | 7285 | 6.2% |
| JDK collections | 6260 | 5.4% |
| JIT stubs (vtable/itable) | 3675 | 3.2% |
| network (kernel) | 3174 | 2.7% |
| JDK invokes/VarHandle | 2608 | 2.2% |
| JDK other | 1773 | 1.5% |
| JVM internals (GC oop barriers) | 530 | 0.5% |
| vdso (clock) | 252 | 0.2% |
| block entities/hoppers (kernel) | 67 | 0.1% |
| bukkit api | 60 | 0.1% |
| redstone (kernel) | 51 | 0.0% |
| craftbukkit glue | 45 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93235 | 80.0% |
| phase: unclassified | 13161 | 11.3% |
| phase: main tick (unclassified) | 3667 | 3.1% |
| phase: chunk tick | 2098 | 1.8% |
| phase: network sync (ServerEntity) | 1913 | 1.6% |
| phase: chunk system (off-main worker) | 1309 | 1.1% |
| phase: block entities (hoppers/furnaces) | 628 | 0.5% |
| phase: random tick | 421 | 0.4% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101158** (86.8%) · native/JVM-internal **15314** (13.1%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4646 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3814 | 3.3% |
| `vtable stub` | native/JVM-internal | 3038 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2604 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1995 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1972 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1736 | 1.5% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1627 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1588 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1576 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1572 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1540 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1511 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1284 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1230 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1113 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1099 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1081 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1078 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1057 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1009 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 935 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 903 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 897 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 870 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 865 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 860 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 859 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 813 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 767 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 738 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 717 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 716 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 665 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 661 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63652)

| bucket | self-time samples | share |
|---|---|---|
| other | 60240 | 94.6% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 910 | 1.4% |
| moonrise/paper patches | 347 | 0.5% |
| chunk system (kernel) | 324 | 0.5% |
| fastutil collections | 229 | 0.4% |
| JDK collections | 197 | 0.3% |
| JIT stubs (vtable/itable) | 126 | 0.2% |
| network (kernel) | 94 | 0.1% |
| JDK invokes/VarHandle | 89 | 0.1% |
| JDK other | 73 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60054 | 94.3% |
| phase: entity tick (AI/movement) | 3155 | 5.0% |
| phase: main tick (unclassified) | 194 | 0.3% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54730** (86.0%) · native/JVM-internal **8918** (14.0%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51359 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4753 | 7.5% |
| `read` | native/JVM-internal | 1232 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 140 | 0.2% |
| `vtable stub` | native/JVM-internal | 104 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 98 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3677)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3677 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2059 | 56.0% |
| phase: unclassified | 1425 | 38.8% |
| phase: main tick (unclassified) | 86 | 2.3% |
| phase: chunk system (off-main worker) | 48 | 1.3% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3677** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 569 | 15.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 531 | 14.4% |
| `char[]_[k]` | other | 439 | 11.9% |
| `byte[]_[k]` | other | 183 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 172 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 126 | 3.4% |
| `long[]_[i]` | other | 125 | 3.4% |
| `java.util.ArrayList_[i]` | other | 115 | 3.1% |
| `java.lang.Object[]_[i]` | other | 114 | 3.1% |
| `byte[]_[i]` | other | 94 | 2.6% |
| `int[]_[i]` | other | 73 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 62 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 46 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.3% |
| `net.minecraft.core.SectionPos_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8f07904c00_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116563 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34480 | 29.58% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22933 | 19.67% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6620 | 5.68% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5488 | 4.71% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4491 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1205 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 913 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 458 | 0.39% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 228 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 213 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 190 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 569 | 15.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 531 | 14.4% |
| `char[]_[k]` | 439 | 11.9% |
| `byte[]_[k]` | 183 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 172 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 126 | 3.4% |
| `long[]_[i]` | 125 | 3.4% |
| `java.util.ArrayList_[i]` | 115 | 3.1% |
| `java.lang.Object[]_[i]` | 114 | 3.1% |
| `byte[]_[i]` | 94 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 24688 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148101..151358 (delta 3257, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99532->103214, minecraft:zombie 3653->4701, minecraft:drowned 3557->4581, minecraft:creeper 4520->5193, minecraft:husk 4532->5193, minecraft:spider 4261->4859, minecraft:skeleton 4433->4862, minecraft:chicken 3380->3416
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3257)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58186814 B)
- `wall-collapsed.txt` (3781239 B)
- `alloc-collapsed.txt` (2010767 B)
- `cpu-flamegraph.html` (318465 B)
- `server-stdout.log` (260580 B)
- `gc.log` (116228 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
