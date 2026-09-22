# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.801 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 1.2, 2.1, 2.4, 2.8, 2.9]
- spark tick-monitor MSPT: avg **366.65ms** / min 315.58ms / max **496.29ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T18:59:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8541939 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 315.58 | — | — | — | 496.29 | 366.65 |

- entity totals seen: [149042, 150768, 150798]
- top entity types (max seen): minecraft:item×103039, minecraft:creeper×5237, minecraft:husk×5137, minecraft:skeleton×4864, minecraft:spider×4756, minecraft:zombie×4617, minecraft:drowned×4503, minecraft:sheep×3540, minecraft:chicken×3440, minecraft:cow×3326, minecraft:pig×3237, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tb9RsWlko8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **131** (Full GC: **10**)
- total pause: **23015.2 ms**, avg **175.69 ms**, max **2225.9 ms**
- heap high-water seen: **7693 MB** -> last-after: **3644 MB**
  - Young (Allocation Failure): 111
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112737)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27550 | 24.4% |
| kernel: other | 25416 | 22.5% |
| other | 13342 | 11.8% |
| moonrise/paper patches | 10475 | 9.3% |
| chunk system (kernel) | 10307 | 9.1% |
| fastutil collections | 7376 | 6.5% |
| JDK collections | 6305 | 5.6% |
| network (kernel) | 3630 | 3.2% |
| JIT stubs (vtable/itable) | 3144 | 2.8% |
| JDK invokes/VarHandle | 2331 | 2.1% |
| JDK other | 1734 | 1.5% |
| JVM internals (GC oop barriers) | 542 | 0.5% |
| vdso (clock) | 244 | 0.2% |
| block entities/hoppers (kernel) | 123 | 0.1% |
| redstone (kernel) | 88 | 0.1% |
| craftbukkit glue | 55 | 0.0% |
| bukkit api | 44 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88343 | 78.4% |
| phase: unclassified | 12924 | 11.5% |
| phase: main tick (unclassified) | 4048 | 3.6% |
| phase: chunk tick | 2438 | 2.2% |
| phase: network sync (ServerEntity) | 2343 | 2.1% |
| phase: chunk system (off-main worker) | 1226 | 1.1% |
| phase: block entities (hoppers/furnaces) | 797 | 0.7% |
| phase: random tick | 477 | 0.4% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98433** (87.3%) · native/JVM-internal **14208** (12.6%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5096 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3496 | 3.1% |
| `vtable stub` | native/JVM-internal | 2466 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2428 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2405 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2244 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2120 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1859 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1720 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1583 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1479 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1458 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1376 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1273 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1265 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1230 | 1.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1197 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1186 | 1.1% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1129 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1106 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1105 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1039 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1017 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 975 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 953 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 932 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 878 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 867 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 731 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 726 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 698 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 697 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 683 | 0.6% |
| `itable stub` | native/JVM-internal | 674 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57963 | 94.6% |
| entities/mobs (kernel) | 917 | 1.5% |
| kernel: other | 850 | 1.4% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 309 | 0.5% |
| fastutil collections | 252 | 0.4% |
| JDK collections | 210 | 0.3% |
| network (kernel) | 124 | 0.2% |
| JIT stubs (vtable/itable) | 113 | 0.2% |
| JDK invokes/VarHandle | 92 | 0.2% |
| JDK other | 66 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 10 | 0.0% |
| redstone (kernel) | 6 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57802 | 94.4% |
| phase: entity tick (AI/movement) | 2985 | 4.9% |
| phase: main tick (unclassified) | 186 | 0.3% |
| phase: chunk tick | 106 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: block entities (hoppers/furnaces) | 48 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52357** (85.5%) · native/JVM-internal **8902** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49104 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `syscall` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `vtable stub` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 56 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4097)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4097 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2349 | 57.3% |
| phase: unclassified | 1533 | 37.4% |
| phase: main tick (unclassified) | 95 | 2.3% |
| phase: chunk system (off-main worker) | 69 | 1.7% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 8 | 0.2% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4097** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 626 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 572 | 14.0% |
| `char[]_[k]` | other | 437 | 10.7% |
| `byte[]_[k]` | other | 238 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 189 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 3.7% |
| `java.util.ArrayList_[i]` | other | 142 | 3.5% |
| `long[]_[i]` | other | 138 | 3.4% |
| `java.lang.Object[]_[i]` | other | 123 | 3.0% |
| `byte[]_[i]` | other | 91 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.6% |
| `int[]_[i]` | other | 58 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 57 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 56 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 48 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 42 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 37 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112737 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32514 | 28.84% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21652 | 19.21% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6123 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5137 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4445 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 885 | 0.79% |
| `net/minecraft/world/entity/ai/Brain.tick` | 878 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 389 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 212 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 196 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 626 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 572 | 14.0% |
| `char[]_[k]` | 437 | 10.7% |
| `byte[]_[k]` | 238 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 189 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 151 | 3.7% |
| `java.util.ArrayList_[i]` | 142 | 3.5% |
| `long[]_[i]` | 138 | 3.4% |
| `java.lang.Object[]_[i]` | 123 | 3.0% |
| `byte[]_[i]` | 91 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 131 pauses / total 23015 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147852..150798 (delta 2946, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99436->103039, minecraft:drowned 3538->4503, minecraft:zombie 3662->4617, minecraft:creeper 4575->5237, minecraft:husk 4543->5137, minecraft:spider 4219->4756, minecraft:skeleton 4441->4864, minecraft:chicken 3404->3440
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2946)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51107822 B)
- `wall-collapsed.txt` (3441158 B)
- `alloc-collapsed.txt` (2195005 B)
- `cpu-flamegraph.html` (261650 B)
- `server-stdout.log` (239132 B)
- `gc.log` (123935 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
