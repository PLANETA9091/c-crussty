# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.968 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [5.7, 1.6, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **415.32ms** / min 346.25ms / max **525.08ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:18:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6887003 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.25 | — | — | — | 525.08 | 415.32 |

- entity totals seen: [149086, 150240, 151413]
- top entity types (max seen): minecraft:item×103375, minecraft:creeper×5251, minecraft:husk×5183, minecraft:skeleton×4865, minecraft:spider×4829, minecraft:zombie×4664, minecraft:drowned×4559, minecraft:sheep×3528, minecraft:chicken×3425, minecraft:cow×3336, minecraft:pig×3216, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/AP4aa21Q5v
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **10**)
- total pause: **24827.6 ms**, avg **198.62 ms**, max **2391.3 ms**
- heap high-water seen: **7573 MB** -> last-after: **5259 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117040)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28941 | 24.7% |
| kernel: other | 28333 | 24.2% |
| other | 15743 | 13.5% |
| moonrise/paper patches | 9653 | 8.2% |
| chunk system (kernel) | 9295 | 7.9% |
| fastutil collections | 6876 | 5.9% |
| JDK collections | 5996 | 5.1% |
| JIT stubs (vtable/itable) | 3480 | 3.0% |
| network (kernel) | 3105 | 2.7% |
| JDK invokes/VarHandle | 2491 | 2.1% |
| JDK other | 2047 | 1.7% |
| JVM internals (GC oop barriers) | 545 | 0.5% |
| vdso (clock) | 254 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| bukkit api | 53 | 0.0% |
| worldgen/noise (kernel) | 43 | 0.0% |
| redstone (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92457 | 79.0% |
| phase: unclassified | 14739 | 12.6% |
| phase: main tick (unclassified) | 3702 | 3.2% |
| phase: chunk tick | 2002 | 1.7% |
| phase: network sync (ServerEntity) | 1860 | 1.6% |
| phase: chunk system (off-main worker) | 1110 | 0.9% |
| phase: block entities (hoppers/furnaces) | 645 | 0.6% |
| phase: random tick | 400 | 0.3% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100422** (85.8%) · native/JVM-internal **16537** (14.1%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4372 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3299 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2974 | 2.5% |
| `vtable stub` | native/JVM-internal | 2877 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2008 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1844 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1678 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1635 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1521 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1474 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1447 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1437 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1352 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1306 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1279 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1176 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1146 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1095 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1023 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 987 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 928 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 925 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 913 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 906 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 901 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 893 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 854 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 826 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 771 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 727 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 681 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 662 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 646 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 645 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 629 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 625 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57863 | 94.5% |
| entities/mobs (kernel) | 1031 | 1.7% |
| kernel: other | 924 | 1.5% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 285 | 0.5% |
| fastutil collections | 238 | 0.4% |
| JDK collections | 186 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 109 | 0.2% |
| JDK invokes/VarHandle | 77 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57683 | 94.2% |
| phase: entity tick (AI/movement) | 3127 | 5.1% |
| phase: main tick (unclassified) | 200 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52396** (85.5%) · native/JVM-internal **8855** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49037 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 105 | 0.2% |
| `vtable stub` | native/JVM-internal | 102 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 82 | 0.1% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 54 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3793)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3793 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2149 | 56.7% |
| phase: unclassified | 1470 | 38.8% |
| phase: main tick (unclassified) | 85 | 2.2% |
| phase: chunk system (off-main worker) | 47 | 1.2% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3793** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 593 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 538 | 14.2% |
| `char[]_[k]` | other | 421 | 11.1% |
| `byte[]_[k]` | other | 217 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 158 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 156 | 4.1% |
| `java.util.ArrayList_[i]` | other | 135 | 3.6% |
| `long[]_[i]` | other | 121 | 3.2% |
| `java.lang.Object[]_[i]` | other | 103 | 2.7% |
| `byte[]_[i]` | other | 92 | 2.4% |
| `int[]_[i]` | other | 79 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 36 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f4cf79d6000_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117040 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34571 | 29.54% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22507 | 19.23% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6340 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5473 | 4.68% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4525 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1092 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 871 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 414 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 593 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | 538 | 14.2% |
| `char[]_[k]` | 421 | 11.1% |
| `byte[]_[k]` | 217 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 158 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 156 | 4.1% |
| `java.util.ArrayList_[i]` | 135 | 3.6% |
| `long[]_[i]` | 121 | 3.2% |
| `java.lang.Object[]_[i]` | 103 | 2.7% |
| `byte[]_[i]` | 92 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 24828 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148192..151413 (delta 3221, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99715->103375, minecraft:drowned 3433->4559, minecraft:zombie 3652->4664, minecraft:creeper 4555->5251, minecraft:husk 4529->5183, minecraft:spider 4257->4829, minecraft:skeleton 4356->4865, minecraft:chicken 3402->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3221)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58367375 B)
- `wall-collapsed.txt` (3735220 B)
- `alloc-collapsed.txt` (2044300 B)
- `cpu-flamegraph.html` (302017 B)
- `server-stdout.log` (258958 B)
- `gc.log` (118813 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
