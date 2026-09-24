# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.996 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.9, 2.0, 2.2, 2.7, 2.9, 3.1]
- spark tick-monitor MSPT: avg **338.79ms** / min 299.8ms / max **403.72ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:06:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8606390 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 299.8 | — | — | — | 403.72 | 338.79 |

- entity totals seen: [149171, 150806, 151046]
- top entity types (max seen): minecraft:item×103082, minecraft:creeper×5220, minecraft:husk×5175, minecraft:skeleton×4858, minecraft:spider×4844, minecraft:zombie×4610, minecraft:drowned×4502, minecraft:sheep×3545, minecraft:chicken×3440, minecraft:cow×3328, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tsnwlUTos6
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **135** (Full GC: **10**)
- total pause: **22071.3 ms**, avg **163.49 ms**, max **2083.6 ms**
- heap high-water seen: **7688 MB** -> last-after: **3667 MB**
  - Young (Allocation Failure): 115
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112725)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27707 | 24.6% |
| kernel: other | 26141 | 23.2% |
| other | 13728 | 12.2% |
| chunk system (kernel) | 9997 | 8.9% |
| moonrise/paper patches | 9974 | 8.8% |
| fastutil collections | 6763 | 6.0% |
| JDK collections | 6315 | 5.6% |
| network (kernel) | 3824 | 3.4% |
| JIT stubs (vtable/itable) | 2923 | 2.6% |
| JDK invokes/VarHandle | 2420 | 2.1% |
| JDK other | 1915 | 1.7% |
| JVM internals (GC oop barriers) | 488 | 0.4% |
| vdso (clock) | 219 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| bukkit api | 65 | 0.1% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87669 | 77.8% |
| phase: unclassified | 13564 | 12.0% |
| phase: main tick (unclassified) | 4203 | 3.7% |
| phase: chunk tick | 2425 | 2.2% |
| phase: network sync (ServerEntity) | 2190 | 1.9% |
| phase: chunk system (off-main worker) | 1252 | 1.1% |
| phase: block entities (hoppers/furnaces) | 794 | 0.7% |
| phase: random tick | 483 | 0.4% |
| phase: mob spawning | 143 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98052** (87.0%) · native/JVM-internal **14566** (12.9%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4870 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3527 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2678 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2480 | 2.2% |
| `vtable stub` | native/JVM-internal | 2293 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2125 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1851 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1825 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1799 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1594 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1537 | 1.4% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1498 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1445 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1323 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1317 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1271 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1184 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1177 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1139 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1101 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1076 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1058 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1044 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1040 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1007 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 768 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 738 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 731 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 726 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 710 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 706 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 702 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 671 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 662 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 57997 | 94.7% |
| entities/mobs (kernel) | 919 | 1.5% |
| kernel: other | 835 | 1.4% |
| moonrise/paper patches | 348 | 0.6% |
| chunk system (kernel) | 297 | 0.5% |
| JDK collections | 231 | 0.4% |
| fastutil collections | 230 | 0.4% |
| network (kernel) | 136 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK other | 78 | 0.1% |
| JDK invokes/VarHandle | 57 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57851 | 94.5% |
| phase: entity tick (AI/movement) | 2897 | 4.7% |
| phase: main tick (unclassified) | 222 | 0.4% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 79 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52410** (85.6%) · native/JVM-internal **8828** (14.4%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49180 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1223 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 80 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 78 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4351)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4351 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2527 | 58.1% |
| phase: unclassified | 1625 | 37.3% |
| phase: main tick (unclassified) | 98 | 2.3% |
| phase: chunk system (off-main worker) | 52 | 1.2% |
| phase: network sync (ServerEntity) | 19 | 0.4% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: chunk tick | 12 | 0.3% |
| phase: mob spawning | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4351** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 698 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 590 | 13.6% |
| `char[]_[k]` | other | 448 | 10.3% |
| `byte[]_[k]` | other | 314 | 7.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 201 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 171 | 3.9% |
| `long[]_[i]` | other | 156 | 3.6% |
| `java.util.ArrayList_[i]` | other | 143 | 3.3% |
| `java.lang.Object[]_[i]` | other | 122 | 2.8% |
| `byte[]_[i]` | other | 89 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 1.8% |
| `int[]_[i]` | other | 65 | 1.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 53 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f957583cfb0_[i]` | other | 42 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 39 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f95759f4000_[i]` | other | 39 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 38 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f9575a46e58_[i]` | other | 36 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112725 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32276 | 28.63% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21039 | 18.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6175 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5351 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4438 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 929 | 0.82% |
| `net/minecraft/world/entity/ai/Brain.tick` | 913 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 410 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 233 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 224 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 203 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 192 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 698 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 590 | 13.6% |
| `char[]_[k]` | 448 | 10.3% |
| `byte[]_[k]` | 314 | 7.2% |
| `net.minecraft.core.BlockPos_[i]` | 201 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 171 | 3.9% |
| `long[]_[i]` | 156 | 3.6% |
| `java.util.ArrayList_[i]` | 143 | 3.3% |
| `java.lang.Object[]_[i]` | 122 | 2.8% |
| `byte[]_[i]` | 89 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 135 pauses / total 22071 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147908..151046 (delta 3138, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99432->103082, minecraft:drowned 3566->4502, minecraft:zombie 3708->4610, minecraft:creeper 4574->5220, minecraft:husk 4553->5175, minecraft:spider 4238->4844, minecraft:skeleton 4373->4858, minecraft:chicken 3408->3440
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3138)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51175760 B)
- `wall-collapsed.txt` (3324567 B)
- `alloc-collapsed.txt` (2253282 B)
- `cpu-flamegraph.html` (264862 B)
- `server-stdout.log` (239854 B)
- `gc.log` (127379 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
