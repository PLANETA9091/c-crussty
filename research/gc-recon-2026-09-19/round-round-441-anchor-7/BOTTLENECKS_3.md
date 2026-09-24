# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.024 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 2.0, 2.3, 2.8, 3.0, 3.0]
- spark tick-monitor MSPT: avg **338.84ms** / min 292.32ms / max **456.7ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:21:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8541634 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 292.32 | — | — | — | 456.7 | 338.84 |

- entity totals seen: [149274, 150712, 151003]
- top entity types (max seen): minecraft:item×103126, minecraft:creeper×5245, minecraft:husk×5175, minecraft:skeleton×4827, minecraft:spider×4817, minecraft:zombie×4602, minecraft:drowned×4495, minecraft:sheep×3540, minecraft:chicken×3433, minecraft:cow×3320, minecraft:pig×3233, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gT4NL3TLQf
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **136** (Full GC: **10**)
- total pause: **22076.3 ms**, avg **162.33 ms**, max **2094.7 ms**
- heap high-water seen: **7614 MB** -> last-after: **3746 MB**
  - Young (Allocation Failure): 115
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 113255)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27441 | 24.2% |
| kernel: other | 26285 | 23.2% |
| other | 13681 | 12.1% |
| moonrise/paper patches | 10429 | 9.2% |
| chunk system (kernel) | 10246 | 9.0% |
| fastutil collections | 7342 | 6.5% |
| JDK collections | 6067 | 5.4% |
| network (kernel) | 3809 | 3.4% |
| JIT stubs (vtable/itable) | 2627 | 2.3% |
| JDK invokes/VarHandle | 2483 | 2.2% |
| JDK other | 1863 | 1.6% |
| JVM internals (GC oop barriers) | 462 | 0.4% |
| vdso (clock) | 226 | 0.2% |
| block entities/hoppers (kernel) | 101 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| bukkit api | 48 | 0.0% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88417 | 78.1% |
| phase: unclassified | 13385 | 11.8% |
| phase: main tick (unclassified) | 4185 | 3.7% |
| phase: chunk tick | 2356 | 2.1% |
| phase: network sync (ServerEntity) | 2223 | 2.0% |
| phase: chunk system (off-main worker) | 1264 | 1.1% |
| phase: block entities (hoppers/furnaces) | 779 | 0.7% |
| phase: random tick | 506 | 0.4% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99101** (87.5%) · native/JVM-internal **14049** (12.4%) · other **105** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4931 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3482 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2531 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2430 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2164 | 1.9% |
| `vtable stub` | native/JVM-internal | 2151 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2149 | 1.9% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f0751a84350.accept` | JVM-Java | 1789 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1782 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1752 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1611 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1422 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1395 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1377 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1370 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1322 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1302 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1207 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1138 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1083 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1038 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1016 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1001 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 994 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 983 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 960 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 958 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 925 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 914 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 742 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 733 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 724 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 693 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 671 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 659 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 657 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 57991 | 94.7% |
| entities/mobs (kernel) | 897 | 1.5% |
| kernel: other | 880 | 1.4% |
| moonrise/paper patches | 355 | 0.6% |
| chunk system (kernel) | 320 | 0.5% |
| fastutil collections | 272 | 0.4% |
| JDK collections | 162 | 0.3% |
| network (kernel) | 122 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK invokes/VarHandle | 63 | 0.1% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57834 | 94.4% |
| phase: entity tick (AI/movement) | 2925 | 4.8% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 84 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52344** (85.5%) · native/JVM-internal **8901** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49120 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 158 | 0.3% |
| `syscall` | native/JVM-internal | 103 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 97 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 95 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 86 | 0.1% |
| `vtable stub` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f0751a84350.accept` | JVM-Java | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4236)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4236 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2441 | 57.6% |
| phase: unclassified | 1612 | 38.1% |
| phase: main tick (unclassified) | 104 | 2.5% |
| phase: chunk system (off-main worker) | 52 | 1.2% |
| phase: network sync (ServerEntity) | 12 | 0.3% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 1 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4236** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 676 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 559 | 13.2% |
| `char[]_[k]` | other | 448 | 10.6% |
| `byte[]_[k]` | other | 287 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 204 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 170 | 4.0% |
| `long[]_[i]` | other | 143 | 3.4% |
| `java.util.ArrayList_[i]` | other | 138 | 3.3% |
| `java.lang.Object[]_[i]` | other | 110 | 2.6% |
| `byte[]_[i]` | other | 89 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 1.9% |
| `int[]_[i]` | other | 55 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 54 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 48 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f0751860950_[i]` | other | 45 | 1.1% |
| `net.minecraft.core.SectionPos_[i]` | other | 42 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 42 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 41 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113255 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32577 | 28.76% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21583 | 19.06% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6052 | 5.34% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5262 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4444 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 921 | 0.81% |
| `net/minecraft/world/entity/ai/Brain.tick` | 887 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 378 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 211 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 202 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 188 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 676 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 559 | 13.2% |
| `char[]_[k]` | 448 | 10.6% |
| `byte[]_[k]` | 287 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | 204 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 170 | 4.0% |
| `long[]_[i]` | 143 | 3.4% |
| `java.util.ArrayList_[i]` | 138 | 3.3% |
| `java.lang.Object[]_[i]` | 110 | 2.6% |
| `byte[]_[i]` | 89 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 136 pauses / total 22076 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148005..151003 (delta 2998, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99564->103126, minecraft:drowned 3384->4495, minecraft:zombie 3693->4602, minecraft:creeper 4650->5245, minecraft:spider 4251->4817, minecraft:husk 4610->5175, minecraft:skeleton 4398->4827, minecraft:chicken 3397->3433
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2998)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51752174 B)
- `wall-collapsed.txt` (3352544 B)
- `alloc-collapsed.txt` (2295263 B)
- `cpu-flamegraph.html` (272017 B)
- `server-stdout.log` (241343 B)
- `gc.log` (128250 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
