# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.172 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.6, 1.8, 2.0, 2.3, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:25:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7202898 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149138, 150324, 151470]
- top entity types (max seen): minecraft:item×103405, minecraft:creeper×5204, minecraft:husk×5188, minecraft:skeleton×4874, minecraft:spider×4848, minecraft:zombie×4675, minecraft:drowned×4543, minecraft:sheep×3514, minecraft:chicken×3438, minecraft:cow×3383, minecraft:pig×3256, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/qgHTrlbyMN
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19743.2 ms**, avg **171.68 ms**, max **2486.2 ms**
- heap high-water seen: **7461 MB** -> last-after: **4192 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115787)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28113 | 24.3% |
| entities/mobs (kernel) | 28108 | 24.3% |
| other | 13392 | 11.6% |
| moonrise/paper patches | 10217 | 8.8% |
| chunk system (kernel) | 9741 | 8.4% |
| fastutil collections | 7341 | 6.3% |
| JDK collections | 5948 | 5.1% |
| JIT stubs (vtable/itable) | 3908 | 3.4% |
| network (kernel) | 3216 | 2.8% |
| JDK invokes/VarHandle | 2680 | 2.3% |
| JDK other | 2011 | 1.7% |
| JVM internals (GC oop barriers) | 561 | 0.5% |
| vdso (clock) | 221 | 0.2% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| bukkit api | 83 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93288 | 80.6% |
| phase: unclassified | 12350 | 10.7% |
| phase: main tick (unclassified) | 3747 | 3.2% |
| phase: chunk tick | 2311 | 2.0% |
| phase: network sync (ServerEntity) | 1679 | 1.5% |
| phase: chunk system (off-main worker) | 1184 | 1.0% |
| phase: block entities (hoppers/furnaces) | 721 | 0.6% |
| phase: random tick | 393 | 0.3% |
| phase: mob spawning | 112 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101034** (87.3%) · native/JVM-internal **14689** (12.7%) · other **64** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4401 | 3.8% |
| `vtable stub` | native/JVM-internal | 3182 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3088 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2572 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2110 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1757 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1732 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1731 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1704 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1514 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1504 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1491 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1443 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1368 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1341 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1283 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1106 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1082 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1027 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1002 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 921 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 916 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 874 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 866 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 857 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 824 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 823 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 774 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 737 | 0.6% |
| `itable stub` | native/JVM-internal | 726 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 720 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 712 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 701 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 668 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 666 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 644 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 631 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57928 | 94.6% |
| entities/mobs (kernel) | 943 | 1.5% |
| kernel: other | 927 | 1.5% |
| moonrise/paper patches | 332 | 0.5% |
| chunk system (kernel) | 300 | 0.5% |
| fastutil collections | 240 | 0.4% |
| JDK collections | 178 | 0.3% |
| JIT stubs (vtable/itable) | 126 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 75 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57722 | 94.2% |
| phase: entity tick (AI/movement) | 3123 | 5.1% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 48 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52339** (85.4%) · native/JVM-internal **8905** (14.5%) · other **16** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49033 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 103 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 97 | 0.2% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 54 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10669)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10669 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8399 | 78.7% |
| phase: entity tick (AI/movement) | 2002 | 18.8% |
| phase: chunk system (off-main worker) | 153 | 1.4% |
| phase: main tick (unclassified) | 83 | 0.8% |
| phase: network sync (ServerEntity) | 17 | 0.2% |
| phase: block entities (hoppers/furnaces) | 7 | 0.1% |
| phase: chunk tick | 4 | 0.0% |
| phase: random tick | 3 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10669** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1983 | 18.6% |
| `byte[]_[k]` | other | 742 | 7.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 558 | 5.2% |
| `short[]_[i]` | other | 544 | 5.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 525 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 515 | 4.8% |
| `long[]_[k]` | other | 504 | 4.7% |
| `java.lang.Object[]_[i]` | other | 468 | 4.4% |
| `char[]_[k]` | other | 431 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 413 | 3.9% |
| `byte[]_[i]` | other | 398 | 3.7% |
| `java.lang.Object[]_[k]` | other | 189 | 1.8% |
| `java.lang.String_[i]` | other | 189 | 1.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 172 | 1.6% |
| `java.util.ArrayList_[i]` | other | 148 | 1.4% |
| `long[]_[i]` | other | 147 | 1.4% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 140 | 1.3% |
| `java.util.Optional_[i]` | other | 138 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 136 | 1.3% |
| `int[]_[i]` | other | 100 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115787 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34454 | 29.76% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23120 | 19.97% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6410 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5369 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4531 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1114 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 894 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 454 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 208 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1983 | 18.6% |
| `byte[]_[k]` | 742 | 7.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | 558 | 5.2% |
| `short[]_[i]` | 544 | 5.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 525 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | 515 | 4.8% |
| `long[]_[k]` | 504 | 4.7% |
| `java.lang.Object[]_[i]` | 468 | 4.4% |
| `char[]_[k]` | 431 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 413 | 3.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19743 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148170..151470 (delta 3300, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99656->103405, minecraft:drowned 3452->4543, minecraft:zombie 3705->4675, minecraft:creeper 4520->5204, minecraft:husk 4517->5188, minecraft:spider 4235->4848, minecraft:skeleton 4347->4874, minecraft:chicken 3404->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3300)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58463471 B)
- `wall-collapsed.txt` (3670782 B)
- `alloc-collapsed.txt` (3771201 B)
- `cpu-flamegraph.html` (305370 B)
- `server-stdout.log` (251580 B)
- `gc.log` (109259 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
