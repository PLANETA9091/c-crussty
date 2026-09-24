# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.665 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.4, 1.8, 2.1, 2.4, 2.7, 2.8]
- spark tick-monitor MSPT: avg **368.01ms** / min 310.21ms / max **482.0ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:01:53Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7236789 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 310.21 | — | — | — | 482.0 | 368.01 |

- entity totals seen: [149327, 151204, 151439]
- top entity types (max seen): minecraft:item×103397, minecraft:creeper×5205, minecraft:husk×5187, minecraft:spider×4867, minecraft:skeleton×4838, minecraft:zombie×4702, minecraft:drowned×4564, minecraft:sheep×3512, minecraft:chicken×3420, minecraft:cow×3368, minecraft:pig×3257, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Xovdl1bI4j
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **26366.2 ms**, avg **214.36 ms**, max **2886.1 ms**
- heap high-water seen: **7639 MB** -> last-after: **5507 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115997)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29593 | 25.5% |
| entities/mobs (kernel) | 28116 | 24.2% |
| other | 14467 | 12.5% |
| chunk system (kernel) | 10263 | 8.8% |
| moonrise/paper patches | 9953 | 8.6% |
| fastutil collections | 7426 | 6.4% |
| JDK collections | 5213 | 4.5% |
| JIT stubs (vtable/itable) | 3208 | 2.8% |
| network (kernel) | 2654 | 2.3% |
| JDK invokes/VarHandle | 2250 | 1.9% |
| JDK other | 1746 | 1.5% |
| JVM internals (GC oop barriers) | 534 | 0.5% |
| vdso (clock) | 237 | 0.2% |
| bukkit api | 96 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93044 | 80.2% |
| phase: unclassified | 12850 | 11.1% |
| phase: main tick (unclassified) | 3851 | 3.3% |
| phase: chunk tick | 2181 | 1.9% |
| phase: network sync (ServerEntity) | 1822 | 1.6% |
| phase: chunk system (off-main worker) | 1064 | 0.9% |
| phase: block entities (hoppers/furnaces) | 668 | 0.6% |
| phase: random tick | 401 | 0.3% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101124** (87.2%) · native/JVM-internal **14745** (12.7%) · other **128** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4936 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4207 | 3.6% |
| `vtable stub` | native/JVM-internal | 2696 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2548 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1915 | 1.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1819 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1785 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1670 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1659 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1637 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1631 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1552 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1447 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1372 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1321 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1274 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1198 | 1.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1172 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1123 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1072 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1060 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 965 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 913 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 896 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 865 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 863 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 811 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 809 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 804 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 779 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 767 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 748 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 726 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 650 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 622 | 0.5% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 620 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 581 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 57934 | 94.6% |
| entities/mobs (kernel) | 984 | 1.6% |
| kernel: other | 928 | 1.5% |
| moonrise/paper patches | 373 | 0.6% |
| chunk system (kernel) | 301 | 0.5% |
| fastutil collections | 225 | 0.4% |
| JDK collections | 181 | 0.3% |
| JIT stubs (vtable/itable) | 116 | 0.2% |
| network (kernel) | 83 | 0.1% |
| JDK other | 66 | 0.1% |
| JDK invokes/VarHandle | 52 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57747 | 94.3% |
| phase: entity tick (AI/movement) | 3045 | 5.0% |
| phase: main tick (unclassified) | 221 | 0.4% |
| phase: chunk tick | 94 | 0.2% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: chunk system (off-main worker) | 34 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52451** (85.6%) · native/JVM-internal **8802** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49156 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4787 | 7.8% |
| `read` | native/JVM-internal | 1210 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 142 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 113 | 0.2% |
| `vtable stub` | native/JVM-internal | 90 | 0.1% |
| `syscall` | native/JVM-internal | 79 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 71 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3880)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3880 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2223 | 57.3% |
| phase: unclassified | 1465 | 37.8% |
| phase: main tick (unclassified) | 98 | 2.5% |
| phase: chunk system (off-main worker) | 48 | 1.2% |
| phase: network sync (ServerEntity) | 24 | 0.6% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: mob spawning | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3880** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 590 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 569 | 14.7% |
| `char[]_[k]` | other | 446 | 11.5% |
| `byte[]_[k]` | other | 262 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 178 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 167 | 4.3% |
| `long[]_[i]` | other | 145 | 3.7% |
| `java.lang.Object[]_[i]` | other | 111 | 2.9% |
| `byte[]_[i]` | other | 85 | 2.2% |
| `java.util.ArrayList_[i]` | other | 83 | 2.1% |
| `int[]_[i]` | other | 79 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb7ab83dc00_[i]` | other | 35 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 35 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb7ab9ef260_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 33 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115997 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35273 | 30.41% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22934 | 19.77% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6143 | 5.30% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5467 | 4.71% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4143 | 3.57% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1195 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 910 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 419 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 266 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 200 | 0.17% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 199 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 590 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 569 | 14.7% |
| `char[]_[k]` | 446 | 11.5% |
| `byte[]_[k]` | 262 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | 178 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 167 | 4.3% |
| `long[]_[i]` | 145 | 3.7% |
| `java.lang.Object[]_[i]` | 111 | 2.9% |
| `byte[]_[i]` | 85 | 2.2% |
| `java.util.ArrayList_[i]` | 83 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 26366 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148077..151439 (delta 3362, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99622->103397, minecraft:drowned 3601->4564, minecraft:zombie 3750->4702, minecraft:creeper 4526->5205, minecraft:husk 4532->5187, minecraft:spider 4229->4867, minecraft:skeleton 4438->4838, minecraft:chicken 3386->3420
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3362)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58714974 B)
- `wall-collapsed.txt` (3718733 B)
- `alloc-collapsed.txt` (2110542 B)
- `cpu-flamegraph.html` (299821 B)
- `server-stdout.log` (255864 B)
- `gc.log` (117077 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
