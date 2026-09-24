# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.426 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 1.5, 1.9, 2.1, 2.4, 2.4]
- spark tick-monitor MSPT: avg **440.64ms** / min 357.7ms / max **570.91ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:54:31Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7119774 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 357.7 | — | — | — | 570.91 | 440.64 |

- entity totals seen: [149108, 150258, 151401]
- top entity types (max seen): minecraft:item×103325, minecraft:creeper×5208, minecraft:husk×5196, minecraft:skeleton×4864, minecraft:spider×4788, minecraft:zombie×4670, minecraft:drowned×4540, minecraft:sheep×3512, minecraft:chicken×3431, minecraft:cow×3381, minecraft:pig×3260, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/rSWAd6cJzo
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20216.2 ms**, avg **177.34 ms**, max **2452.2 ms**
- heap high-water seen: **7538 MB** -> last-after: **4272 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116242)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28471 | 24.5% |
| kernel: other | 27741 | 23.9% |
| other | 13969 | 12.0% |
| chunk system (kernel) | 10016 | 8.6% |
| moonrise/paper patches | 10004 | 8.6% |
| fastutil collections | 7831 | 6.7% |
| JDK collections | 6156 | 5.3% |
| network (kernel) | 3322 | 2.9% |
| JIT stubs (vtable/itable) | 3311 | 2.8% |
| JDK invokes/VarHandle | 2251 | 1.9% |
| JDK other | 2104 | 1.8% |
| JVM internals (GC oop barriers) | 593 | 0.5% |
| vdso (clock) | 193 | 0.2% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93698 | 80.6% |
| phase: unclassified | 12644 | 10.9% |
| phase: main tick (unclassified) | 3525 | 3.0% |
| phase: chunk tick | 2013 | 1.7% |
| phase: network sync (ServerEntity) | 1890 | 1.6% |
| phase: chunk system (off-main worker) | 1185 | 1.0% |
| phase: block entities (hoppers/furnaces) | 702 | 0.6% |
| phase: random tick | 439 | 0.4% |
| phase: mob spawning | 143 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101454** (87.3%) · native/JVM-internal **14693** (12.6%) · other **95** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4608 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3317 | 2.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2995 | 2.6% |
| `vtable stub` | native/JVM-internal | 2748 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2086 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1880 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1854 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1593 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1588 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1371 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1355 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1331 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1330 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1306 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1291 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1095 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1057 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1045 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1008 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 980 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 957 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 908 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 859 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 855 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 850 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 798 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 730 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 703 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 664 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 661 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 653 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61349)

| bucket | self-time samples | share |
|---|---|---|
| other | 58025 | 94.6% |
| entities/mobs (kernel) | 1003 | 1.6% |
| kernel: other | 864 | 1.4% |
| moonrise/paper patches | 333 | 0.5% |
| chunk system (kernel) | 304 | 0.5% |
| fastutil collections | 251 | 0.4% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 121 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57822 | 94.3% |
| phase: entity tick (AI/movement) | 3069 | 5.0% |
| phase: main tick (unclassified) | 204 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52449** (85.5%) · native/JVM-internal **8890** (14.5%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49136 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4743 | 7.7% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1204 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 98 | 0.2% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 78 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3596)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3596 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2020 | 56.2% |
| phase: unclassified | 1418 | 39.4% |
| phase: main tick (unclassified) | 80 | 2.2% |
| phase: chunk system (off-main worker) | 36 | 1.0% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3596** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 561 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 504 | 14.0% |
| `char[]_[k]` | other | 441 | 12.3% |
| `byte[]_[k]` | other | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 181 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.6% |
| `java.util.ArrayList_[i]` | other | 122 | 3.4% |
| `long[]_[i]` | other | 113 | 3.1% |
| `java.lang.Object[]_[i]` | other | 94 | 2.6% |
| `byte[]_[i]` | other | 87 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 59 | 1.6% |
| `int[]_[i]` | other | 53 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 46 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 30 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f773583eb78_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116242 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34486 | 29.67% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23030 | 19.81% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6517 | 5.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5432 | 4.67% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4546 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1095 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 951 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 259 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 214 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 188 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 561 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 504 | 14.0% |
| `char[]_[k]` | 441 | 12.3% |
| `byte[]_[k]` | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 181 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.6% |
| `java.util.ArrayList_[i]` | 122 | 3.4% |
| `long[]_[i]` | 113 | 3.1% |
| `java.lang.Object[]_[i]` | 94 | 2.6% |
| `byte[]_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20216 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148202..151401 (delta 3199, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99625->103325, minecraft:drowned 3443->4540, minecraft:zombie 3706->4670, minecraft:husk 4517->5196, minecraft:creeper 4545->5208, minecraft:spider 4234->4788, minecraft:skeleton 4414->4864, minecraft:chicken 3404->3431
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3199)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (63037183 B)
- `wall-collapsed.txt` (3715444 B)
- `alloc-collapsed.txt` (2120122 B)
- `cpu-flamegraph.html` (308052 B)
- `server-stdout.log` (256842 B)
- `gc.log` (108391 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
