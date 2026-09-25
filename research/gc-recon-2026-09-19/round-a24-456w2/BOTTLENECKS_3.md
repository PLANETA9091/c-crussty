# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.358 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.3, 1.7, 2.1, 2.4, 2.8, 2.8]
- spark tick-monitor MSPT: avg **380.18ms** / min 323.19ms / max **491.85ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T09:15:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8928192 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 323.19 | — | — | — | 491.85 | 380.18 |

- entity totals seen: [149138, 150964, 151559]
- top entity types (max seen): minecraft:item×103606, minecraft:creeper×5219, minecraft:husk×5147, minecraft:spider×4869, minecraft:skeleton×4852, minecraft:zombie×4664, minecraft:drowned×4553, minecraft:sheep×3511, minecraft:chicken×3422, minecraft:cow×3345, minecraft:pig×3199, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/RI0EeODkwQ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **23523.8 ms**, avg **196.03 ms**, max **3387.7 ms**
- heap high-water seen: **7574 MB** -> last-after: **4312 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117257)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28350 | 24.2% |
| entities/mobs (kernel) | 26578 | 22.7% |
| other | 15944 | 13.6% |
| chunk system (kernel) | 11054 | 9.4% |
| moonrise/paper patches | 10611 | 9.0% |
| JDK collections | 7521 | 6.4% |
| fastutil collections | 6815 | 5.8% |
| JIT stubs (vtable/itable) | 2912 | 2.5% |
| JDK invokes/VarHandle | 2505 | 2.1% |
| network (kernel) | 2190 | 1.9% |
| JDK other | 1775 | 1.5% |
| JVM internals (GC oop barriers) | 495 | 0.4% |
| vdso (clock) | 233 | 0.2% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94055 | 80.2% |
| phase: unclassified | 13335 | 11.4% |
| phase: main tick (unclassified) | 3355 | 2.9% |
| phase: chunk tick | 2100 | 1.8% |
| phase: network sync (ServerEntity) | 1959 | 1.7% |
| phase: chunk system (off-main worker) | 1120 | 1.0% |
| phase: block entities (hoppers/furnaces) | 703 | 0.6% |
| phase: random tick | 468 | 0.4% |
| phase: mob spawning | 161 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101002** (86.1%) · native/JVM-internal **15527** (13.2%) · other **728** (0.6%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5321 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4501 | 3.8% |
| `java/util/HashMap.getNode` | JVM-Java | 3062 | 2.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2474 | 2.1% |
| `vtable stub` | native/JVM-internal | 2438 | 2.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2414 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1810 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1772 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1737 | 1.5% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1726 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1720 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1642 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1621 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1504 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1467 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1261 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1209 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1197 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1092 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1070 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1038 | 0.9% |
| `SharedRuntime::frem` | native/JVM-internal | 1019 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 999 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 974 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 964 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 934 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 918 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 906 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 880 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 861 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 826 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 740 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 727 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 718 | 0.6% |
| `libmFmod` | other | 682 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 650 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 638 | 0.5% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 631 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 57918 | 94.5% |
| kernel: other | 932 | 1.5% |
| entities/mobs (kernel) | 885 | 1.4% |
| moonrise/paper patches | 367 | 0.6% |
| chunk system (kernel) | 325 | 0.5% |
| JDK collections | 228 | 0.4% |
| fastutil collections | 211 | 0.3% |
| JIT stubs (vtable/itable) | 126 | 0.2% |
| JDK invokes/VarHandle | 101 | 0.2% |
| network (kernel) | 72 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 16 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57717 | 94.2% |
| phase: entity tick (AI/movement) | 3113 | 5.1% |
| phase: main tick (unclassified) | 179 | 0.3% |
| phase: chunk tick | 79 | 0.1% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52328** (85.4%) · native/JVM-internal **8898** (14.5%) · other **32** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49044 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.8% |
| `read` | native/JVM-internal | 1209 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 153 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 72 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `SharedRuntime::frem` | native/JVM-internal | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3946)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3946 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2186 | 55.4% |
| phase: unclassified | 1588 | 40.2% |
| phase: main tick (unclassified) | 89 | 2.3% |
| phase: chunk system (off-main worker) | 37 | 0.9% |
| phase: network sync (ServerEntity) | 28 | 0.7% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3946** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 564 | 14.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 555 | 14.1% |
| `char[]_[k]` | other | 450 | 11.4% |
| `byte[]_[k]` | other | 255 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 184 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 170 | 4.3% |
| `long[]_[i]` | other | 157 | 4.0% |
| `java.util.ArrayList_[i]` | other | 131 | 3.3% |
| `java.lang.Object[]_[i]` | other | 122 | 3.1% |
| `byte[]_[i]` | other | 77 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.0% |
| `int[]_[i]` | other | 74 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 61 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.2% |
| `net.minecraft.core.SectionPos_[i]` | other | 43 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `int[]_[k]` | other | 32 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117257 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34965 | 29.82% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23052 | 19.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6700 | 5.71% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5621 | 4.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4567 | 3.89% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1061 | 0.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 896 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 476 | 0.41% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 298 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 277 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 221 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 201 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 564 | 14.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 555 | 14.1% |
| `char[]_[k]` | 450 | 11.4% |
| `byte[]_[k]` | 255 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 184 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 170 | 4.3% |
| `long[]_[i]` | 157 | 4.0% |
| `java.util.ArrayList_[i]` | 131 | 3.3% |
| `java.lang.Object[]_[i]` | 122 | 3.1% |
| `byte[]_[i]` | 77 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 23524 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148313..151559 (delta 3246, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99878->103606, minecraft:drowned 3583->4553, minecraft:zombie 3716->4664, minecraft:creeper 4550->5219, minecraft:husk 4507->5147, minecraft:spider 4264->4869, minecraft:skeleton 4441->4852, minecraft:chicken 3393->3422
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3246)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56881229 B)
- `wall-collapsed.txt` (3745657 B)
- `alloc-collapsed.txt` (2044347 B)
- `cpu-flamegraph.html` (305160 B)
- `server-stdout.log` (246493 B)
- `gc.log` (113579 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
