# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.518 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 1.6, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **410.3ms** / min 340.86ms / max **549.77ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:43:00Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6579335 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 340.86 | — | — | — | 549.77 | 410.3 |

- entity totals seen: [149003, 150207, 151436]
- top entity types (max seen): minecraft:item×103371, minecraft:creeper×5188, minecraft:husk×5164, minecraft:spider×4849, minecraft:skeleton×4832, minecraft:zombie×4707, minecraft:drowned×4566, minecraft:sheep×3520, minecraft:chicken×3405, minecraft:cow×3360, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/cTW34X7a1l
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **9**)
- total pause: **22847.9 ms**, avg **184.26 ms**, max **2615.0 ms**
- heap high-water seen: **7617 MB** -> last-after: **4354 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115868)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27221 | 23.5% |
| kernel: other | 26449 | 22.8% |
| other | 15372 | 13.3% |
| chunk system (kernel) | 10620 | 9.2% |
| moonrise/paper patches | 10347 | 8.9% |
| fastutil collections | 7062 | 6.1% |
| JDK collections | 6841 | 5.9% |
| network (kernel) | 3516 | 3.0% |
| JIT stubs (vtable/itable) | 2894 | 2.5% |
| JDK invokes/VarHandle | 2642 | 2.3% |
| JDK other | 1780 | 1.5% |
| JVM internals (GC oop barriers) | 579 | 0.5% |
| vdso (clock) | 252 | 0.2% |
| block entities/hoppers (kernel) | 88 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| bukkit api | 56 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90333 | 78.0% |
| phase: unclassified | 15201 | 13.1% |
| phase: main tick (unclassified) | 3862 | 3.3% |
| phase: chunk tick | 2163 | 1.9% |
| phase: network sync (ServerEntity) | 1977 | 1.7% |
| phase: chunk system (off-main worker) | 1043 | 0.9% |
| phase: block entities (hoppers/furnaces) | 719 | 0.6% |
| phase: random tick | 422 | 0.4% |
| phase: mob spawning | 144 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99493** (85.9%) · native/JVM-internal **16291** (14.1%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5192 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3532 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2495 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2365 | 2.0% |
| `vtable stub` | native/JVM-internal | 2338 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1906 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1905 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1831 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1824 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1740 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1647 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1503 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1409 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1391 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1355 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1266 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1200 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1114 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1099 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1068 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1064 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 993 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 987 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 961 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 954 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 865 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 860 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 847 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 821 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 767 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 759 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 695 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 681 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 673 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 662 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 650 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57906 | 94.5% |
| kernel: other | 937 | 1.5% |
| entities/mobs (kernel) | 928 | 1.5% |
| moonrise/paper patches | 350 | 0.6% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 230 | 0.4% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 110 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57748 | 94.3% |
| phase: entity tick (AI/movement) | 3057 | 5.0% |
| phase: main tick (unclassified) | 183 | 0.3% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52365** (85.5%) · native/JVM-internal **8882** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49071 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `syscall` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `vtable stub` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 66 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 58 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 55 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3775)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3775 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2144 | 56.8% |
| phase: unclassified | 1479 | 39.2% |
| phase: main tick (unclassified) | 83 | 2.2% |
| phase: chunk system (off-main worker) | 30 | 0.8% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: chunk tick | 9 | 0.2% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3775** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 613 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 510 | 13.5% |
| `char[]_[k]` | other | 441 | 11.7% |
| `byte[]_[k]` | other | 216 | 5.7% |
| `long[]_[i]` | other | 148 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 147 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 140 | 3.7% |
| `java.util.ArrayList_[i]` | other | 118 | 3.1% |
| `java.lang.Object[]_[i]` | other | 105 | 2.8% |
| `byte[]_[i]` | other | 85 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 1.9% |
| `int[]_[i]` | other | 65 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.8% |
| `java.lang.String_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115868 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34045 | 29.38% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22149 | 19.12% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6124 | 5.29% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5362 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4360 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 965 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 860 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 401 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 218 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 214 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 210 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 181 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 613 | 16.2% |
| `net.minecraft.world.phys.AABB_[i]` | 510 | 13.5% |
| `char[]_[k]` | 441 | 11.7% |
| `byte[]_[k]` | 216 | 5.7% |
| `long[]_[i]` | 148 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 147 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 140 | 3.7% |
| `java.util.ArrayList_[i]` | 118 | 3.1% |
| `java.lang.Object[]_[i]` | 105 | 2.8% |
| `byte[]_[i]` | 85 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 22848 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148198..151436 (delta 3238, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99738->103371, minecraft:drowned 3468->4566, minecraft:zombie 3684->4707, minecraft:creeper 4525->5188, minecraft:husk 4502->5164, minecraft:spider 4251->4849, minecraft:skeleton 4380->4832, minecraft:chicken 3374->3405
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3238)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52756562 B)
- `wall-collapsed.txt` (3584033 B)
- `alloc-collapsed.txt` (2112972 B)
- `cpu-flamegraph.html` (291931 B)
- `server-stdout.log` (260136 B)
- `gc.log` (117010 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
