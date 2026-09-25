# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.61 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 1.7, 1.9, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **433.47ms** / min 353.36ms / max **685.99ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:41:04Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6982027 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 353.36 | — | — | — | 685.99 | 433.47 |

- entity totals seen: [149096, 150227, 151505]
- top entity types (max seen): minecraft:item×103359, minecraft:creeper×5221, minecraft:husk×5179, minecraft:spider×4861, minecraft:skeleton×4844, minecraft:zombie×4668, minecraft:drowned×4538, minecraft:sheep×3509, minecraft:chicken×3443, minecraft:cow×3376, minecraft:pig×3246, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/VfUIvef9aF
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **10**)
- total pause: **24290.0 ms**, avg **204.12 ms**, max **2477.0 ms**
- heap high-water seen: **7436 MB** -> last-after: **5190 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116769)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28411 | 24.3% |
| kernel: other | 27920 | 23.9% |
| other | 15540 | 13.3% |
| moonrise/paper patches | 9809 | 8.4% |
| chunk system (kernel) | 9552 | 8.2% |
| fastutil collections | 7365 | 6.3% |
| JDK collections | 5822 | 5.0% |
| JIT stubs (vtable/itable) | 3561 | 3.0% |
| network (kernel) | 3294 | 2.8% |
| JDK invokes/VarHandle | 2419 | 2.1% |
| JDK other | 1996 | 1.7% |
| JVM internals (GC oop barriers) | 559 | 0.5% |
| vdso (clock) | 249 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 63 | 0.1% |
| craftbukkit glue | 46 | 0.0% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92650 | 79.3% |
| phase: unclassified | 14331 | 12.3% |
| phase: main tick (unclassified) | 3580 | 3.1% |
| phase: chunk tick | 2112 | 1.8% |
| phase: network sync (ServerEntity) | 1848 | 1.6% |
| phase: chunk system (off-main worker) | 1042 | 0.9% |
| phase: block entities (hoppers/furnaces) | 677 | 0.6% |
| phase: random tick | 411 | 0.4% |
| phase: mob spawning | 117 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100304** (85.9%) · native/JVM-internal **16378** (14.0%) · other **87** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4428 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3394 | 2.9% |
| `vtable stub` | native/JVM-internal | 2983 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2637 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1909 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1770 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1727 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1699 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1632 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1546 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1513 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1498 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1433 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1400 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1262 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1204 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1181 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1107 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1083 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 981 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 909 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 873 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 868 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 854 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 849 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 836 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 750 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 702 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 658 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 643 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 629 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 625 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 620 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57884 | 94.5% |
| entities/mobs (kernel) | 984 | 1.6% |
| kernel: other | 881 | 1.4% |
| moonrise/paper patches | 376 | 0.6% |
| chunk system (kernel) | 297 | 0.5% |
| fastutil collections | 236 | 0.4% |
| JDK collections | 167 | 0.3% |
| JIT stubs (vtable/itable) | 151 | 0.2% |
| network (kernel) | 111 | 0.2% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JDK other | 73 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| bukkit api | 6 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57687 | 94.2% |
| phase: entity tick (AI/movement) | 3138 | 5.1% |
| phase: main tick (unclassified) | 201 | 0.3% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52289** (85.4%) · native/JVM-internal **8962** (14.6%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48979 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `vtable stub` | native/JVM-internal | 123 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.2% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3552)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3552 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1984 | 55.9% |
| phase: unclassified | 1407 | 39.6% |
| phase: main tick (unclassified) | 85 | 2.4% |
| phase: chunk system (off-main worker) | 44 | 1.2% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3552** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 513 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 512 | 14.4% |
| `char[]_[k]` | other | 438 | 12.3% |
| `byte[]_[k]` | other | 238 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 162 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 133 | 3.7% |
| `java.util.ArrayList_[i]` | other | 116 | 3.3% |
| `long[]_[i]` | other | 114 | 3.2% |
| `byte[]_[i]` | other | 96 | 2.7% |
| `java.lang.Object[]_[i]` | other | 90 | 2.5% |
| `int[]_[i]` | other | 63 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 56 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f4eaf9e8498_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116769 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34409 | 29.47% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22890 | 19.60% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6339 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5335 | 4.57% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4596 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1155 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 909 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 432 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 210 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 207 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 513 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 512 | 14.4% |
| `char[]_[k]` | 438 | 12.3% |
| `byte[]_[k]` | 238 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 162 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | 133 | 3.7% |
| `java.util.ArrayList_[i]` | 116 | 3.3% |
| `long[]_[i]` | 114 | 3.2% |
| `byte[]_[i]` | 96 | 2.7% |
| `java.lang.Object[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 24290 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148183..151505 (delta 3322, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99675->103359, minecraft:drowned 3445->4538, minecraft:zombie 3691->4668, minecraft:creeper 4544->5221, minecraft:husk 4524->5179, minecraft:spider 4238->4861, minecraft:skeleton 4367->4844, minecraft:chicken 3408->3443
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3322)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59118360 B)
- `wall-collapsed.txt` (3705349 B)
- `alloc-collapsed.txt` (2058627 B)
- `cpu-flamegraph.html` (299732 B)
- `server-stdout.log` (255064 B)
- `gc.log` (113631 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
