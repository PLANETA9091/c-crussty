# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 19.811 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 2.0, 2.1, 2.4, 2.6]
- spark tick-monitor MSPT: avg **431.61ms** / min 365.48ms / max **565.27ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:40:59Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6916268 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 365.48 | — | — | — | 565.27 | 431.61 |

- entity totals seen: [148979, 150122, 151388]
- top entity types (max seen): minecraft:item×103182, minecraft:creeper×5248, minecraft:husk×5209, minecraft:skeleton×4848, minecraft:spider×4825, minecraft:zombie×4720, minecraft:drowned×4586, minecraft:sheep×3505, minecraft:chicken×3417, minecraft:cow×3381, minecraft:pig×3262, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/eNjZL8gELx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **21726.7 ms**, avg **184.12 ms**, max **2612.7 ms**
- heap high-water seen: **7600 MB** -> last-after: **4341 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116329)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28730 | 24.7% |
| kernel: other | 27727 | 23.8% |
| other | 15375 | 13.2% |
| moonrise/paper patches | 9746 | 8.4% |
| chunk system (kernel) | 9275 | 8.0% |
| fastutil collections | 6790 | 5.8% |
| JDK collections | 6027 | 5.2% |
| network (kernel) | 3438 | 3.0% |
| JIT stubs (vtable/itable) | 3356 | 2.9% |
| JDK invokes/VarHandle | 2644 | 2.3% |
| JDK other | 2018 | 1.7% |
| JVM internals (GC oop barriers) | 553 | 0.5% |
| vdso (clock) | 237 | 0.2% |
| redstone (kernel) | 163 | 0.1% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| bukkit api | 63 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91936 | 79.0% |
| phase: unclassified | 14370 | 12.4% |
| phase: main tick (unclassified) | 3690 | 3.2% |
| phase: chunk tick | 2063 | 1.8% |
| phase: network sync (ServerEntity) | 1871 | 1.6% |
| phase: chunk system (off-main worker) | 1180 | 1.0% |
| phase: block entities (hoppers/furnaces) | 679 | 0.6% |
| phase: random tick | 432 | 0.4% |
| phase: mob spawning | 108 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100013** (86.0%) · native/JVM-internal **16218** (13.9%) · other **98** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4356 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3751 | 3.2% |
| `vtable stub` | native/JVM-internal | 2759 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2657 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1943 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1879 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1790 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1714 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1614 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1556 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1493 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1492 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1473 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1252 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1229 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1169 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1124 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1112 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1090 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1017 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 968 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 929 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 878 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 836 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 824 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 722 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 683 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 683 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 647 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 637 | 0.5% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 630 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57872 | 94.5% |
| entities/mobs (kernel) | 983 | 1.6% |
| kernel: other | 960 | 1.6% |
| moonrise/paper patches | 352 | 0.6% |
| chunk system (kernel) | 311 | 0.5% |
| fastutil collections | 203 | 0.3% |
| JDK collections | 182 | 0.3% |
| JIT stubs (vtable/itable) | 113 | 0.2% |
| JDK invokes/VarHandle | 100 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57700 | 94.2% |
| phase: entity tick (AI/movement) | 3084 | 5.0% |
| phase: main tick (unclassified) | 218 | 0.4% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52404** (85.6%) · native/JVM-internal **8845** (14.4%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49058 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 93 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 70 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3636)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3636 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2020 | 55.6% |
| phase: unclassified | 1454 | 40.0% |
| phase: main tick (unclassified) | 84 | 2.3% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3636** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 524 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 514 | 14.1% |
| `char[]_[k]` | other | 439 | 12.1% |
| `byte[]_[k]` | other | 215 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 164 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 4.4% |
| `long[]_[i]` | other | 135 | 3.7% |
| `java.util.ArrayList_[i]` | other | 106 | 2.9% |
| `java.lang.Object[]_[i]` | other | 100 | 2.8% |
| `byte[]_[i]` | other | 75 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 64 | 1.8% |
| `int[]_[i]` | other | 54 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `int[]_[k]` | other | 33 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `java.lang.String_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116329 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34818 | 29.93% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21959 | 18.88% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6321 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5282 | 4.54% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4369 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1107 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 881 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 435 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 256 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 222 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 194 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 524 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 514 | 14.1% |
| `char[]_[k]` | 439 | 12.1% |
| `byte[]_[k]` | 215 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 164 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 4.4% |
| `long[]_[i]` | 135 | 3.7% |
| `java.util.ArrayList_[i]` | 106 | 2.9% |
| `java.lang.Object[]_[i]` | 100 | 2.8% |
| `byte[]_[i]` | 75 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 21727 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148081..151388 (delta 3307, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99457->103182, minecraft:drowned 3492->4586, minecraft:zombie 3684->4720, minecraft:creeper 4543->5248, minecraft:husk 4535->5209, minecraft:spider 4216->4825, minecraft:skeleton 4365->4848, minecraft:chicken 3386->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3307)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55993911 B)
- `wall-collapsed.txt` (3670408 B)
- `alloc-collapsed.txt` (2041842 B)
- `cpu-flamegraph.html` (302248 B)
- `server-stdout.log` (255886 B)
- `gc.log` (111870 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
