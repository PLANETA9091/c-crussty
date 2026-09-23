# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.544 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 1.6, 1.9, 2.2, 2.4, 2.5]
- spark tick-monitor MSPT: avg **430.54ms** / min 343.25ms / max **695.59ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T17:39:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6602780 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.25 | — | — | — | 695.59 | 430.54 |

- entity totals seen: [149054, 150174, 151394]
- top entity types (max seen): minecraft:item×103318, minecraft:creeper×5277, minecraft:husk×5141, minecraft:skeleton×4832, minecraft:spider×4813, minecraft:zombie×4635, minecraft:drowned×4530, minecraft:sheep×3519, minecraft:chicken×3427, minecraft:cow×3392, minecraft:pig×3232, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/HbCR1tUAcc
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **10**)
- total pause: **23477.0 ms**, avg **202.39 ms**, max **2386.6 ms**
- heap high-water seen: **7620 MB** -> last-after: **5590 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116606)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28626 | 24.5% |
| kernel: other | 28195 | 24.2% |
| other | 15117 | 13.0% |
| moonrise/paper patches | 9857 | 8.5% |
| chunk system (kernel) | 9543 | 8.2% |
| fastutil collections | 6781 | 5.8% |
| JDK collections | 6126 | 5.3% |
| network (kernel) | 3424 | 2.9% |
| JIT stubs (vtable/itable) | 3154 | 2.7% |
| JDK invokes/VarHandle | 2656 | 2.3% |
| JDK other | 2035 | 1.7% |
| JVM internals (GC oop barriers) | 570 | 0.5% |
| vdso (clock) | 220 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| redstone (kernel) | 54 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92619 | 79.4% |
| phase: unclassified | 13926 | 11.9% |
| phase: main tick (unclassified) | 3685 | 3.2% |
| phase: chunk tick | 2040 | 1.7% |
| phase: network sync (ServerEntity) | 1913 | 1.6% |
| phase: chunk system (off-main worker) | 1213 | 1.0% |
| phase: block entities (hoppers/furnaces) | 676 | 0.6% |
| phase: random tick | 385 | 0.3% |
| phase: mob spawning | 145 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100942** (86.6%) · native/JVM-internal **15585** (13.4%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4601 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3688 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2776 | 2.4% |
| `vtable stub` | native/JVM-internal | 2587 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1994 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1833 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1792 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1608 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1550 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1517 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1506 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1490 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1485 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1474 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1406 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f2ff59e5800.accept` | JVM-Java | 1291 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1224 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1096 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1095 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 996 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 923 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 906 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 863 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 845 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 816 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 792 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 777 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 753 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 721 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 654 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 650 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 637 | 0.5% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 635 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 57906 | 94.5% |
| entities/mobs (kernel) | 990 | 1.6% |
| kernel: other | 930 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 302 | 0.5% |
| fastutil collections | 239 | 0.4% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 113 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK invokes/VarHandle | 88 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57676 | 94.1% |
| phase: entity tick (AI/movement) | 3138 | 5.1% |
| phase: main tick (unclassified) | 209 | 0.3% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52334** (85.4%) · native/JVM-internal **8924** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48967 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 148 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 97 | 0.2% |
| `syscall` | native/JVM-internal | 97 | 0.2% |
| `vtable stub` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f2ff59e5800.accept` | JVM-Java | 57 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3581)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3581 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2027 | 56.6% |
| phase: unclassified | 1396 | 39.0% |
| phase: main tick (unclassified) | 85 | 2.4% |
| phase: chunk system (off-main worker) | 51 | 1.4% |
| phase: network sync (ServerEntity) | 14 | 0.4% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: random tick | 1 | 0.0% |
| phase: chunk tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3581** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 523 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 515 | 14.4% |
| `char[]_[k]` | other | 434 | 12.1% |
| `byte[]_[k]` | other | 200 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 194 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 148 | 4.1% |
| `long[]_[i]` | other | 135 | 3.8% |
| `java.util.ArrayList_[i]` | other | 135 | 3.8% |
| `java.lang.Object[]_[i]` | other | 101 | 2.8% |
| `byte[]_[i]` | other | 96 | 2.7% |
| `int[]_[i]` | other | 76 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 38 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f2ff583a950_[i]` | other | 30 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116606 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34877 | 29.91% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22259 | 19.09% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6463 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5335 | 4.58% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4564 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1116 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 915 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 444 | 0.38% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 238 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 213 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 523 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | 515 | 14.4% |
| `char[]_[k]` | 434 | 12.1% |
| `byte[]_[k]` | 200 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 194 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 148 | 4.1% |
| `long[]_[i]` | 135 | 3.8% |
| `java.util.ArrayList_[i]` | 135 | 3.8% |
| `java.lang.Object[]_[i]` | 101 | 2.8% |
| `byte[]_[i]` | 96 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 23477 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148125..151394 (delta 3269, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99683->103318, minecraft:drowned 3485->4530, minecraft:zombie 3646->4635, minecraft:creeper 4611->5277, minecraft:husk 4495->5141, minecraft:spider 4224->4813, minecraft:skeleton 4313->4832, minecraft:chicken 3400->3427
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3269)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58407723 B)
- `wall-collapsed.txt` (3730800 B)
- `alloc-collapsed.txt` (2095915 B)
- `cpu-flamegraph.html` (297750 B)
- `server-stdout.log` (248007 B)
- `gc.log` (111022 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
