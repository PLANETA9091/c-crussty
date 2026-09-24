# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.849 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [25.7, 1.9, 2.2, 2.7, 2.8, 2.9]
- spark tick-monitor MSPT: avg **351.27ms** / min 303.47ms / max **519.34ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T22:22:05Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9036595 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 303.47 | — | — | — | 519.34 | 351.27 |

- entity totals seen: [149241, 150795, 151002]
- top entity types (max seen): minecraft:item×103086, minecraft:creeper×5244, minecraft:husk×5201, minecraft:skeleton×4832, minecraft:spider×4819, minecraft:zombie×4611, minecraft:drowned×4494, minecraft:sheep×3540, minecraft:chicken×3441, minecraft:cow×3325, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/3Co1bmIblu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **134** (Full GC: **10**)
- total pause: **22504.3 ms**, avg **167.94 ms**, max **2131.8 ms**
- heap high-water seen: **7702 MB** -> last-after: **3664 MB**
  - Young (Allocation Failure): 114
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112276)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27655 | 24.6% |
| kernel: other | 25444 | 22.7% |
| other | 13140 | 11.7% |
| moonrise/paper patches | 10494 | 9.3% |
| chunk system (kernel) | 10266 | 9.1% |
| fastutil collections | 7046 | 6.3% |
| JDK collections | 6042 | 5.4% |
| network (kernel) | 3670 | 3.3% |
| JIT stubs (vtable/itable) | 2849 | 2.5% |
| JDK invokes/VarHandle | 2637 | 2.3% |
| JDK other | 2012 | 1.8% |
| JVM internals (GC oop barriers) | 500 | 0.4% |
| vdso (clock) | 232 | 0.2% |
| block entities/hoppers (kernel) | 107 | 0.1% |
| bukkit api | 59 | 0.1% |
| craftbukkit glue | 52 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87720 | 78.1% |
| phase: unclassified | 13004 | 11.6% |
| phase: main tick (unclassified) | 4209 | 3.7% |
| phase: chunk tick | 2375 | 2.1% |
| phase: network sync (ServerEntity) | 2335 | 2.1% |
| phase: chunk system (off-main worker) | 1200 | 1.1% |
| phase: block entities (hoppers/furnaces) | 806 | 0.7% |
| phase: random tick | 480 | 0.4% |
| phase: mob spawning | 144 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98269** (87.5%) · native/JVM-internal **13910** (12.4%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4906 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3554 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2548 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2491 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2255 | 2.0% |
| `vtable stub` | native/JVM-internal | 2240 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2185 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1842 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1771 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1715 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1507 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1452 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1318 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1309 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1230 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1165 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1158 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1109 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1101 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1079 | 1.0% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1071 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1048 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1041 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 997 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 981 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 831 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 750 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 737 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 733 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 725 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 692 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 689 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 652 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61247)

| bucket | self-time samples | share |
|---|---|---|
| other | 58046 | 94.8% |
| entities/mobs (kernel) | 885 | 1.4% |
| kernel: other | 825 | 1.3% |
| moonrise/paper patches | 346 | 0.6% |
| chunk system (kernel) | 322 | 0.5% |
| fastutil collections | 226 | 0.4% |
| JDK collections | 197 | 0.3% |
| network (kernel) | 122 | 0.2% |
| JIT stubs (vtable/itable) | 101 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 80 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57870 | 94.5% |
| phase: entity tick (AI/movement) | 2884 | 4.7% |
| phase: main tick (unclassified) | 229 | 0.4% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52385** (85.5%) · native/JVM-internal **8856** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49204 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 139 | 0.2% |
| `syscall` | native/JVM-internal | 106 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 84 | 0.1% |
| `vtable stub` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3929)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3929 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2409 | 61.3% |
| phase: unclassified | 1351 | 34.4% |
| phase: main tick (unclassified) | 92 | 2.3% |
| phase: chunk system (off-main worker) | 29 | 0.7% |
| phase: network sync (ServerEntity) | 24 | 0.6% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3929** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 637 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 619 | 15.8% |
| `char[]_[k]` | other | 444 | 11.3% |
| `byte[]_[k]` | other | 273 | 6.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 171 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 148 | 3.8% |
| `java.lang.Object[]_[i]` | other | 123 | 3.1% |
| `long[]_[i]` | other | 121 | 3.1% |
| `java.util.ArrayList_[i]` | other | 106 | 2.7% |
| `byte[]_[i]` | other | 83 | 2.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 67 | 1.7% |
| `int[]_[i]` | other | 60 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 43 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 0.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007efe33a4acf0_[i]` | other | 31 | 0.8% |
| `int[]_[k]` | other | 29 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112276 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 31949 | 28.46% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21523 | 19.17% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6208 | 5.53% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5280 | 4.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4396 | 3.92% |
| `net/minecraft/world/entity/ai/Brain.tick` | 959 | 0.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 886 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 409 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 218 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 203 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 637 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | 619 | 15.8% |
| `char[]_[k]` | 444 | 11.3% |
| `byte[]_[k]` | 273 | 6.9% |
| `net.minecraft.core.BlockPos_[i]` | 171 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 148 | 3.8% |
| `java.lang.Object[]_[i]` | 123 | 3.1% |
| `long[]_[i]` | 121 | 3.1% |
| `java.util.ArrayList_[i]` | 106 | 2.7% |
| `byte[]_[i]` | 83 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 134 pauses / total 22504 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148028..151002 (delta 2974, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99522->103086, minecraft:drowned 3491->4494, minecraft:zombie 3689->4611, minecraft:husk 4585->5201, minecraft:creeper 4642->5244, minecraft:spider 4264->4819, minecraft:skeleton 4345->4832, minecraft:chicken 3404->3441
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2974)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48883128 B)
- `wall-collapsed.txt` (3299280 B)
- `alloc-collapsed.txt` (2199820 B)
- `cpu-flamegraph.html` (265159 B)
- `server-stdout.log` (242702 B)
- `gc.log` (126524 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
