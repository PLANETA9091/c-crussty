# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 22.382 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 1.8, 1.9, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **400.13ms** / min 343.93ms / max **531.33ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T14:52:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6748364 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.93 | — | — | — | 531.33 | 400.13 |

- entity totals seen: [148978, 150575, 151449]
- top entity types (max seen): minecraft:item×103439, minecraft:creeper×5232, minecraft:husk×5188, minecraft:skeleton×4838, minecraft:spider×4764, minecraft:zombie×4674, minecraft:drowned×4557, minecraft:sheep×3516, minecraft:chicken×3408, minecraft:cow×3361, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/vVEV29nnOm
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **10**)
- total pause: **23702.0 ms**, avg **194.28 ms**, max **2496.2 ms**
- heap high-water seen: **7638 MB** -> last-after: **5618 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115021)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28378 | 24.7% |
| entities/mobs (kernel) | 27757 | 24.1% |
| other | 15680 | 13.6% |
| moonrise/paper patches | 9530 | 8.3% |
| chunk system (kernel) | 8826 | 7.7% |
| fastutil collections | 6942 | 6.0% |
| JDK collections | 6096 | 5.3% |
| JIT stubs (vtable/itable) | 3459 | 3.0% |
| network (kernel) | 2514 | 2.2% |
| JDK invokes/VarHandle | 2497 | 2.2% |
| JDK other | 2307 | 2.0% |
| JVM internals (GC oop barriers) | 526 | 0.5% |
| vdso (clock) | 198 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 77 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| redstone (kernel) | 54 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90549 | 78.7% |
| phase: unclassified | 14026 | 12.2% |
| phase: main tick (unclassified) | 3948 | 3.4% |
| phase: chunk tick | 2181 | 1.9% |
| phase: network sync (ServerEntity) | 1856 | 1.6% |
| phase: chunk system (off-main worker) | 1214 | 1.1% |
| phase: block entities (hoppers/furnaces) | 688 | 0.6% |
| phase: random tick | 420 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98480** (85.6%) · native/JVM-internal **16442** (14.3%) · other **99** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3918 | 3.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3209 | 2.8% |
| `vtable stub` | native/JVM-internal | 2863 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2544 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2070 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1992 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1611 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1540 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1463 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1427 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1370 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1347 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1294 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1275 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1145 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1075 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1075 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 984 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 972 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 968 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 966 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 957 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 928 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 874 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 865 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 855 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 830 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 792 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 790 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 790 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 726 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 698 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 677 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 636 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 628 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 610 | 0.5% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 602 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57952 | 94.6% |
| entities/mobs (kernel) | 985 | 1.6% |
| kernel: other | 910 | 1.5% |
| moonrise/paper patches | 309 | 0.5% |
| chunk system (kernel) | 263 | 0.4% |
| fastutil collections | 251 | 0.4% |
| JDK collections | 182 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 91 | 0.1% |
| JDK other | 82 | 0.1% |
| JDK invokes/VarHandle | 80 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57775 | 94.3% |
| phase: entity tick (AI/movement) | 2985 | 4.9% |
| phase: main tick (unclassified) | 233 | 0.4% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52363** (85.5%) · native/JVM-internal **8876** (14.5%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49129 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 111 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3631)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3631 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1979 | 54.5% |
| phase: unclassified | 1483 | 40.8% |
| phase: main tick (unclassified) | 85 | 2.3% |
| phase: chunk system (off-main worker) | 33 | 0.9% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3631** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 581 | 16.0% |
| `char[]_[k]` | other | 440 | 12.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 409 | 11.3% |
| `byte[]_[k]` | other | 222 | 6.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.7% |
| `long[]_[i]` | other | 135 | 3.7% |
| `java.util.ArrayList_[i]` | other | 122 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 111 | 3.1% |
| `java.lang.Object[]_[i]` | other | 99 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 2.4% |
| `byte[]_[i]` | other | 81 | 2.2% |
| `int[]_[i]` | other | 67 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 57 | 1.6% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fbdbd9e0f10_[i]` | other | 43 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |
| `java.lang.String_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115021 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 29049 | 25.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23641 | 20.55% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6879 | 5.98% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5809 | 5.05% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4785 | 4.16% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1220 | 1.06% |
| `net/minecraft/world/entity/ai/Brain.tick` | 968 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 423 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 259 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 254 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 581 | 16.0% |
| `char[]_[k]` | 440 | 12.1% |
| `net.minecraft.world.phys.AABB_[i]` | 409 | 11.3% |
| `byte[]_[k]` | 222 | 6.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.7% |
| `long[]_[i]` | 135 | 3.7% |
| `java.util.ArrayList_[i]` | 122 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 111 | 3.1% |
| `java.lang.Object[]_[i]` | 99 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 23702 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148174..151449 (delta 3275, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99801->103439, minecraft:drowned 3515->4557, minecraft:zombie 3699->4674, minecraft:creeper 4538->5232, minecraft:husk 4502->5188, minecraft:spider 4219->4764, minecraft:skeleton 4473->4838, minecraft:chicken 3380->3408
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3275)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58445573 B)
- `wall-collapsed.txt` (3664119 B)
- `alloc-collapsed.txt` (2008925 B)
- `cpu-flamegraph.html` (298283 B)
- `server-stdout.log` (253944 B)
- `gc.log` (116240 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
