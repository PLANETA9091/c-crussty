# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.87 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.5, 1.9, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **430.43ms** / min 374.26ms / max **549.4ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T09:15:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7172483 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 374.26 | — | — | — | 549.4 | 430.43 |

- entity totals seen: [149158, 150293, 151484]
- top entity types (max seen): minecraft:item×103367, minecraft:husk×5220, minecraft:creeper×5202, minecraft:skeleton×4877, minecraft:spider×4836, minecraft:zombie×4669, minecraft:drowned×4551, minecraft:sheep×3514, minecraft:chicken×3440, minecraft:cow×3382, minecraft:pig×3256, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/eWJyUUiS2d
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **21515.4 ms**, avg **174.92 ms**, max **2433.0 ms**
- heap high-water seen: **7419 MB** -> last-after: **4154 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116375)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28256 | 24.3% |
| entities/mobs (kernel) | 27398 | 23.5% |
| other | 15674 | 13.5% |
| moonrise/paper patches | 10323 | 8.9% |
| chunk system (kernel) | 9621 | 8.3% |
| fastutil collections | 7141 | 6.1% |
| JDK collections | 5863 | 5.0% |
| JIT stubs (vtable/itable) | 3571 | 3.1% |
| network (kernel) | 3011 | 2.6% |
| JDK invokes/VarHandle | 2358 | 2.0% |
| JDK other | 2078 | 1.8% |
| JVM internals (GC oop barriers) | 606 | 0.5% |
| vdso (clock) | 200 | 0.2% |
| bukkit api | 78 | 0.1% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| craftbukkit glue | 47 | 0.0% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92066 | 79.1% |
| phase: unclassified | 14397 | 12.4% |
| phase: main tick (unclassified) | 3718 | 3.2% |
| phase: chunk tick | 2226 | 1.9% |
| phase: network sync (ServerEntity) | 1703 | 1.5% |
| phase: chunk system (off-main worker) | 1121 | 1.0% |
| phase: block entities (hoppers/furnaces) | 625 | 0.5% |
| phase: random tick | 372 | 0.3% |
| phase: mob spawning | 146 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99784** (85.7%) · native/JVM-internal **16489** (14.2%) · other **102** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4467 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3306 | 2.8% |
| `vtable stub` | native/JVM-internal | 2959 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2542 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1961 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1806 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1745 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1683 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1676 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1659 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1498 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1443 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1396 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1394 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1282 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1208 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1150 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1002 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 920 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 915 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 905 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 879 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 853 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 833 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 814 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 779 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 768 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 750 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 723 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 674 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 656 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 651 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 638 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57842 | 94.4% |
| kernel: other | 984 | 1.6% |
| entities/mobs (kernel) | 970 | 1.6% |
| moonrise/paper patches | 338 | 0.6% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 235 | 0.4% |
| JDK collections | 194 | 0.3% |
| JIT stubs (vtable/itable) | 119 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK other | 72 | 0.1% |
| JDK invokes/VarHandle | 49 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57645 | 94.1% |
| phase: entity tick (AI/movement) | 3189 | 5.2% |
| phase: main tick (unclassified) | 194 | 0.3% |
| phase: chunk tick | 86 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 29 | 0.0% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52405** (85.6%) · native/JVM-internal **8842** (14.4%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49018 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 63 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3820)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3820 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2134 | 55.9% |
| phase: unclassified | 1526 | 39.9% |
| phase: main tick (unclassified) | 87 | 2.3% |
| phase: chunk system (off-main worker) | 34 | 0.9% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: block entities (hoppers/furnaces) | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3820** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 596 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 509 | 13.3% |
| `char[]_[k]` | other | 438 | 11.5% |
| `byte[]_[k]` | other | 223 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 159 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 139 | 3.6% |
| `java.lang.Object[]_[i]` | other | 126 | 3.3% |
| `java.util.ArrayList_[i]` | other | 125 | 3.3% |
| `long[]_[i]` | other | 117 | 3.1% |
| `int[]_[i]` | other | 87 | 2.3% |
| `byte[]_[i]` | other | 82 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.7% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 33 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116375 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34437 | 29.59% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22310 | 19.17% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6440 | 5.53% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5351 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4405 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1120 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 934 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 430 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 250 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 244 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 210 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 192 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 596 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | 509 | 13.3% |
| `char[]_[k]` | 438 | 11.5% |
| `byte[]_[k]` | 223 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 159 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 139 | 3.6% |
| `java.lang.Object[]_[i]` | 126 | 3.3% |
| `java.util.ArrayList_[i]` | 125 | 3.3% |
| `long[]_[i]` | 117 | 3.1% |
| `int[]_[i]` | 87 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 21515 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148214..151484 (delta 3270, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99597->103367, minecraft:drowned 3493->4551, minecraft:zombie 3662->4669, minecraft:husk 4529->5220, minecraft:creeper 4540->5202, minecraft:spider 4226->4836, minecraft:skeleton 4362->4877, minecraft:chicken 3410->3440
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3270)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56571084 B)
- `wall-collapsed.txt` (3752543 B)
- `alloc-collapsed.txt` (2096912 B)
- `cpu-flamegraph.html` (299334 B)
- `server-stdout.log` (252467 B)
- `gc.log` (116179 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
