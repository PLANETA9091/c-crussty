# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.381 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.6, 2.0, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **402.27ms** / min 349.01ms / max **514.1ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:46:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7186583 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 349.01 | — | — | — | 514.1 | 402.27 |

- entity totals seen: [148944, 150080, 151279]
- top entity types (max seen): minecraft:item×103114, minecraft:creeper×5236, minecraft:husk×5128, minecraft:skeleton×4870, minecraft:spider×4817, minecraft:zombie×4674, minecraft:drowned×4528, minecraft:sheep×3525, minecraft:chicken×3469, minecraft:cow×3368, minecraft:pig×3279, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/cczgBupTcy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **10**)
- total pause: **23410.8 ms**, avg **191.89 ms**, max **2360.0 ms**
- heap high-water seen: **7516 MB** -> last-after: **5195 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116416)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28811 | 24.7% |
| entities/mobs (kernel) | 28464 | 24.5% |
| other | 12896 | 11.1% |
| chunk system (kernel) | 10057 | 8.6% |
| moonrise/paper patches | 9983 | 8.6% |
| fastutil collections | 8006 | 6.9% |
| JDK collections | 6022 | 5.2% |
| JIT stubs (vtable/itable) | 3875 | 3.3% |
| network (kernel) | 3317 | 2.8% |
| JDK invokes/VarHandle | 2441 | 2.1% |
| JDK other | 1994 | 1.7% |
| vdso (clock) | 247 | 0.2% |
| block entities/hoppers (kernel) | 88 | 0.1% |
| craftbukkit glue | 67 | 0.1% |
| bukkit api | 61 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95081 | 81.7% |
| phase: unclassified | 11220 | 9.6% |
| phase: main tick (unclassified) | 3808 | 3.3% |
| phase: chunk tick | 2135 | 1.8% |
| phase: network sync (ServerEntity) | 1876 | 1.6% |
| phase: chunk system (off-main worker) | 1114 | 1.0% |
| phase: block entities (hoppers/furnaces) | 652 | 0.6% |
| phase: random tick | 411 | 0.4% |
| phase: mob spawning | 118 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102822** (88.3%) · native/JVM-internal **13493** (11.6%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4757 | 4.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3257 | 2.8% |
| `vtable stub` | native/JVM-internal | 3246 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2584 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2145 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1859 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1805 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1713 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1658 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1531 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1506 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1463 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1416 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1393 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1359 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1322 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1277 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1116 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1054 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1012 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 881 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 877 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 868 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 867 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 865 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 832 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 811 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 791 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 786 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 782 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 766 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 694 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 660 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 635 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57945 | 94.6% |
| entities/mobs (kernel) | 907 | 1.5% |
| kernel: other | 899 | 1.5% |
| moonrise/paper patches | 312 | 0.5% |
| chunk system (kernel) | 269 | 0.4% |
| fastutil collections | 235 | 0.4% |
| JDK collections | 209 | 0.3% |
| JIT stubs (vtable/itable) | 130 | 0.2% |
| network (kernel) | 117 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 73 | 0.1% |
| JVM internals (GC oop barriers) | 60 | 0.1% |
| craftbukkit glue | 4 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57791 | 94.3% |
| phase: entity tick (AI/movement) | 2971 | 4.9% |
| phase: main tick (unclassified) | 210 | 0.3% |
| phase: chunk tick | 123 | 0.2% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: block entities (hoppers/furnaces) | 19 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52070** (85.0%) · native/JVM-internal **9181** (15.0%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48856 | 79.8% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 244 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 127 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 63 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 51 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3672)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3672 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2025 | 55.1% |
| phase: unclassified | 1468 | 40.0% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 51 | 1.4% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3672** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 531 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 528 | 14.4% |
| `char[]_[k]` | other | 434 | 11.8% |
| `byte[]_[k]` | other | 207 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 179 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.8% |
| `long[]_[i]` | other | 136 | 3.7% |
| `java.util.ArrayList_[i]` | other | 118 | 3.2% |
| `java.lang.Object[]_[i]` | other | 110 | 3.0% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.2% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007ff0c69ef520_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116416 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35548 | 30.54% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22935 | 19.70% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6632 | 5.70% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5419 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4564 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1200 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 974 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 443 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 250 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 249 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 220 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 531 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 528 | 14.4% |
| `char[]_[k]` | 434 | 11.8% |
| `byte[]_[k]` | 207 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 179 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.8% |
| `long[]_[i]` | 136 | 3.7% |
| `java.util.ArrayList_[i]` | 118 | 3.2% |
| `java.lang.Object[]_[i]` | 110 | 3.0% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 23411 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148099..151279 (delta 3180, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99465->103114, minecraft:drowned 3474->4528, minecraft:zombie 3678->4674, minecraft:creeper 4567->5236, minecraft:husk 4507->5128, minecraft:spider 4235->4817, minecraft:skeleton 4394->4870, minecraft:chicken 3442->3469
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3180)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57904221 B)
- `wall-collapsed.txt` (3625904 B)
- `alloc-collapsed.txt` (2024137 B)
- `cpu-flamegraph.html` (298764 B)
- `server-stdout.log` (246758 B)
- `gc.log` (116198 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
