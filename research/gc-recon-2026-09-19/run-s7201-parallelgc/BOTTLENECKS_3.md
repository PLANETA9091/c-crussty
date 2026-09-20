# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.369 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.4, 1.7, 2.0, 2.2, 2.6, 2.7]
- spark tick-monitor MSPT: avg **405.32ms** / min 347.52ms / max **541.39ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T13:21:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6653417 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC; 4 = COLLECTOR ZGC generational — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
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
| spark tickmonitor (whole run, [⚡] lines) | 347.52 | — | — | — | 541.39 | 405.32 |

- entity totals seen: [148986, 150215, 151293]
- top entity types (max seen): minecraft:item×103257, minecraft:creeper×5197, minecraft:husk×5155, minecraft:skeleton×4889, minecraft:spider×4806, minecraft:zombie×4639, minecraft:drowned×4547, minecraft:sheep×3523, minecraft:chicken×3411, minecraft:cow×3375, minecraft:pig×3225, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/otckcIcqvv
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **7**)
- total pause: **18792.4 ms**, avg **162.00 ms**, max **2400.6 ms**
- heap high-water seen: **7545 MB** -> last-after: **4270 MB**
  - Young (Allocation Failure): 101
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 114891)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28814 | 25.1% |
| kernel: other | 28325 | 24.7% |
| other | 12464 | 10.8% |
| moonrise/paper patches | 10178 | 8.9% |
| chunk system (kernel) | 9244 | 8.0% |
| fastutil collections | 6995 | 6.1% |
| JDK collections | 6510 | 5.7% |
| JIT stubs (vtable/itable) | 3622 | 3.2% |
| network (kernel) | 3219 | 2.8% |
| JDK invokes/VarHandle | 2443 | 2.1% |
| JDK other | 1919 | 1.7% |
| JVM internals (GC oop barriers) | 581 | 0.5% |
| vdso (clock) | 242 | 0.2% |
| redstone (kernel) | 90 | 0.1% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| bukkit api | 82 | 0.1% |
| craftbukkit glue | 41 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93133 | 81.1% |
| phase: unclassified | 11511 | 10.0% |
| phase: main tick (unclassified) | 3672 | 3.2% |
| phase: chunk tick | 2263 | 2.0% |
| phase: network sync (ServerEntity) | 1771 | 1.5% |
| phase: chunk system (off-main worker) | 1356 | 1.2% |
| phase: block entities (hoppers/furnaces) | 665 | 0.6% |
| phase: random tick | 399 | 0.3% |
| phase: mob spawning | 120 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101183** (88.1%) · native/JVM-internal **13607** (11.8%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4389 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3570 | 3.1% |
| `vtable stub` | native/JVM-internal | 3024 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2626 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1957 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1905 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1788 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1769 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1659 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1602 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1508 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1508 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1486 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1290 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1237 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1185 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1179 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1077 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1016 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1006 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 974 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 892 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 885 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 847 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 828 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 828 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 828 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 784 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 743 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 724 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 671 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 662 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 618 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 57853 | 94.5% |
| entities/mobs (kernel) | 990 | 1.6% |
| kernel: other | 962 | 1.6% |
| moonrise/paper patches | 352 | 0.6% |
| chunk system (kernel) | 271 | 0.4% |
| fastutil collections | 235 | 0.4% |
| JDK collections | 171 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 74 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57664 | 94.1% |
| phase: entity tick (AI/movement) | 3139 | 5.1% |
| phase: main tick (unclassified) | 209 | 0.3% |
| phase: chunk tick | 80 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: chunk system (off-main worker) | 55 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52409** (85.6%) · native/JVM-internal **8836** (14.4%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49053 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4760 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 122 | 0.2% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 55 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3698)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3698 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2076 | 56.1% |
| phase: unclassified | 1446 | 39.1% |
| phase: main tick (unclassified) | 87 | 2.4% |
| phase: chunk system (off-main worker) | 42 | 1.1% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3698** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 548 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 527 | 14.3% |
| `char[]_[k]` | other | 442 | 12.0% |
| `byte[]_[k]` | other | 197 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 173 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 166 | 4.5% |
| `long[]_[i]` | other | 132 | 3.6% |
| `java.util.ArrayList_[i]` | other | 127 | 3.4% |
| `java.lang.Object[]_[i]` | other | 97 | 2.6% |
| `byte[]_[i]` | other | 94 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 70 | 1.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 57 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007ff7e59e12d8_[i]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007ff7e582a248_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 114891 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34620 | 30.13% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22487 | 19.57% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6643 | 5.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5464 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4469 | 3.89% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1140 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 965 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 446 | 0.39% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 261 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 256 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 224 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 197 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 548 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 527 | 14.3% |
| `char[]_[k]` | 442 | 12.0% |
| `byte[]_[k]` | 197 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 173 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | 166 | 4.5% |
| `long[]_[i]` | 132 | 3.6% |
| `java.util.ArrayList_[i]` | 127 | 3.4% |
| `java.lang.Object[]_[i]` | 97 | 2.6% |
| `byte[]_[i]` | 94 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 18792 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148124..151293 (delta 3169, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99662->103257, minecraft:drowned 3494->4547, minecraft:zombie 3620->4639, minecraft:husk 4506->5155, minecraft:creeper 4575->5197, minecraft:spider 4227->4806, minecraft:skeleton 4468->4889, minecraft:chicken 3384->3411
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3169)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54492435 B)
- `wall-collapsed.txt` (3620233 B)
- `alloc-collapsed.txt` (2015381 B)
- `cpu-flamegraph.html` (298267 B)
- `server-stdout.log` (259750 B)
- `gc.log` (108344 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
