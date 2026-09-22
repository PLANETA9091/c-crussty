# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.485 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.3, 1.9, 2.3, 2.6, 2.9, 3.0]
- spark tick-monitor MSPT: avg **343.77ms** / min 297.42ms / max **452.61ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T00:56:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8827927 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 297.42 | — | — | — | 452.61 | 343.77 |

- entity totals seen: [149105, 150852, 151109]
- top entity types (max seen): minecraft:item×103165, minecraft:creeper×5296, minecraft:husk×5255, minecraft:skeleton×4851, minecraft:spider×4773, minecraft:zombie×4620, minecraft:drowned×4511, minecraft:sheep×3541, minecraft:chicken×3438, minecraft:cow×3326, minecraft:pig×3228, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/TYNQQHFnks
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **128** (Full GC: **7**)
- total pause: **18797.2 ms**, avg **146.85 ms**, max **2140.6 ms**
- heap high-water seen: **7900 MB** -> last-after: **4613 MB**
  - Young (Allocation Failure): 111
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2

### CPU profile — self-time by research bucket (total self-time samples: 112082)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27461 | 24.5% |
| kernel: other | 25599 | 22.8% |
| other | 11587 | 10.3% |
| moonrise/paper patches | 10699 | 9.5% |
| chunk system (kernel) | 10316 | 9.2% |
| fastutil collections | 7487 | 6.7% |
| JDK collections | 6349 | 5.7% |
| network (kernel) | 3912 | 3.5% |
| JIT stubs (vtable/itable) | 3028 | 2.7% |
| JDK invokes/VarHandle | 2798 | 2.5% |
| JDK other | 1805 | 1.6% |
| JVM internals (GC oop barriers) | 511 | 0.5% |
| vdso (clock) | 245 | 0.2% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| bukkit api | 63 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| redstone (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89458 | 79.8% |
| phase: unclassified | 11455 | 10.2% |
| phase: main tick (unclassified) | 3975 | 3.5% |
| phase: network sync (ServerEntity) | 2328 | 2.1% |
| phase: chunk tick | 2207 | 2.0% |
| phase: chunk system (off-main worker) | 1293 | 1.2% |
| phase: block entities (hoppers/furnaces) | 751 | 0.7% |
| phase: random tick | 466 | 0.4% |
| phase: mob spawning | 149 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99402** (88.7%) · native/JVM-internal **12592** (11.2%) · other **88** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5022 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3738 | 3.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2599 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2520 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2348 | 2.1% |
| `vtable stub` | native/JVM-internal | 2269 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2216 | 2.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1854 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1779 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1616 | 1.4% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1508 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1424 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1368 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1343 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1309 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1273 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1163 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1132 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1120 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1100 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1089 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1082 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1072 | 1.0% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1061 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1047 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1042 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 931 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 885 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 798 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 767 | 0.7% |
| `itable stub` | native/JVM-internal | 753 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 742 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 731 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 722 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 704 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 702 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 696 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57973 | 94.6% |
| entities/mobs (kernel) | 960 | 1.6% |
| kernel: other | 840 | 1.4% |
| moonrise/paper patches | 381 | 0.6% |
| chunk system (kernel) | 324 | 0.5% |
| fastutil collections | 220 | 0.4% |
| JDK collections | 195 | 0.3% |
| network (kernel) | 114 | 0.2% |
| JIT stubs (vtable/itable) | 84 | 0.1% |
| JDK other | 72 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57840 | 94.4% |
| phase: entity tick (AI/movement) | 2939 | 4.8% |
| phase: main tick (unclassified) | 208 | 0.3% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52364** (85.5%) · native/JVM-internal **8884** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49132 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 158 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 120 | 0.2% |
| `syscall` | native/JVM-internal | 100 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `vtable stub` | native/JVM-internal | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 63 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 45 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4295)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4295 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2403 | 55.9% |
| phase: unclassified | 1687 | 39.3% |
| phase: main tick (unclassified) | 100 | 2.3% |
| phase: chunk system (off-main worker) | 39 | 0.9% |
| phase: network sync (ServerEntity) | 33 | 0.8% |
| phase: block entities (hoppers/furnaces) | 15 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4295** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 617 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 567 | 13.2% |
| `char[]_[k]` | other | 447 | 10.4% |
| `byte[]_[k]` | other | 300 | 7.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 202 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 172 | 4.0% |
| `java.lang.Object[]_[i]` | other | 144 | 3.4% |
| `long[]_[i]` | other | 135 | 3.1% |
| `java.util.ArrayList_[i]` | other | 134 | 3.1% |
| `byte[]_[i]` | other | 89 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 2.0% |
| `int[]_[i]` | other | 61 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 57 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 51 | 1.2% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 48 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 41 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112082 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32710 | 29.18% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22209 | 19.81% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6181 | 5.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5322 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4476 | 3.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 904 | 0.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 899 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 402 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 206 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 202 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 617 | 14.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 567 | 13.2% |
| `char[]_[k]` | 447 | 10.4% |
| `byte[]_[k]` | 300 | 7.0% |
| `net.minecraft.core.BlockPos_[i]` | 202 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 172 | 4.0% |
| `java.lang.Object[]_[i]` | 144 | 3.4% |
| `long[]_[i]` | 135 | 3.1% |
| `java.util.ArrayList_[i]` | 134 | 3.1% |
| `byte[]_[i]` | 89 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 128 pauses / total 18797 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147763..151109 (delta 3346, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99407->103165, minecraft:drowned 3495->4511, minecraft:zombie 3719->4620, minecraft:creeper 4570->5296, minecraft:husk 4550->5255, minecraft:spider 4174->4773, minecraft:skeleton 4377->4851, minecraft:chicken 3407->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3346)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48308471 B)
- `wall-collapsed.txt` (3235924 B)
- `alloc-collapsed.txt` (2082443 B)
- `cpu-flamegraph.html` (277340 B)
- `server-stdout.log` (241475 B)
- `gc.log` (118685 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
