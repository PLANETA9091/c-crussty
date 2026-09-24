# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.148 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.6, 2.1, 2.4, 2.6, 2.6]
- spark tick-monitor MSPT: avg **398.55ms** / min 329.55ms / max **671.05ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:53:24Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7231382 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 329.55 | — | — | — | 671.05 | 398.55 |

- entity totals seen: [149890, 152697, 156105]
- top entity types (max seen): minecraft:item×110172, minecraft:husk×5478, minecraft:creeper×5001, minecraft:skeleton×4868, minecraft:zombie×4650, minecraft:drowned×4555, minecraft:spider×4381, minecraft:sheep×3516, minecraft:chicken×3401, minecraft:cow×3366, minecraft:pig×3182, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wHXIRu6Wmx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **20040.7 ms**, avg **168.41 ms**, max **2300.6 ms**
- heap high-water seen: **7133 MB** -> last-after: **3530 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112243)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31875 | 28.4% |
| kernel: other | 27532 | 24.5% |
| other | 11574 | 10.3% |
| chunk system (kernel) | 9141 | 8.1% |
| moonrise/paper patches | 7306 | 6.5% |
| JDK collections | 7055 | 6.3% |
| fastutil collections | 6139 | 5.5% |
| JIT stubs (vtable/itable) | 3630 | 3.2% |
| network (kernel) | 3156 | 2.8% |
| JDK invokes/VarHandle | 2283 | 2.0% |
| JDK other | 1994 | 1.8% |
| vdso (clock) | 252 | 0.2% |
| craftbukkit glue | 81 | 0.1% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 72 | 0.1% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91878 | 81.9% |
| phase: unclassified | 9369 | 8.3% |
| phase: main tick (unclassified) | 4241 | 3.8% |
| phase: network sync (ServerEntity) | 2199 | 2.0% |
| phase: chunk tick | 1975 | 1.8% |
| phase: chunk system (off-main worker) | 1174 | 1.0% |
| phase: block entities (hoppers/furnaces) | 814 | 0.7% |
| phase: random tick | 452 | 0.4% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100680** (89.7%) · native/JVM-internal **11470** (10.2%) · other **93** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4157 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3301 | 2.9% |
| `vtable stub` | native/JVM-internal | 2828 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2270 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1965 | 1.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1794 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1676 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1634 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1442 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1386 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1366 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1334 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1321 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1289 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1099 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1094 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1080 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1065 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 975 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 975 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 953 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 941 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 940 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 937 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 854 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 852 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 844 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 813 | 0.7% |
| `itable stub` | native/JVM-internal | 795 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 782 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 777 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 748 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 748 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 724 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 696 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 672 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 661 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 647 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61568 | 94.9% |
| entities/mobs (kernel) | 1062 | 1.6% |
| kernel: other | 924 | 1.4% |
| chunk system (kernel) | 264 | 0.4% |
| moonrise/paper patches | 233 | 0.4% |
| JDK collections | 227 | 0.3% |
| fastutil collections | 189 | 0.3% |
| JIT stubs (vtable/itable) | 136 | 0.2% |
| network (kernel) | 103 | 0.2% |
| JDK invokes/VarHandle | 72 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61378 | 94.6% |
| phase: entity tick (AI/movement) | 3011 | 4.6% |
| phase: main tick (unclassified) | 216 | 0.3% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 71 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56010** (86.4%) · native/JVM-internal **8849** (13.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52757 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.4% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 116 | 0.2% |
| `vtable stub` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.2% |
| `syscall` | native/JVM-internal | 94 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4045)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4045 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1987 | 49.1% |
| phase: unclassified | 1826 | 45.1% |
| phase: main tick (unclassified) | 131 | 3.2% |
| phase: chunk system (off-main worker) | 36 | 0.9% |
| phase: network sync (ServerEntity) | 32 | 0.8% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4045** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 529 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 504 | 12.5% |
| `char[]_[k]` | other | 415 | 10.3% |
| `byte[]_[k]` | other | 260 | 6.4% |
| `byte[]_[i]` | other | 225 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 3.9% |
| `int[]_[i]` | other | 147 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 3.5% |
| `java.util.ArrayList_[i]` | other | 120 | 3.0% |
| `java.lang.Object[]_[i]` | other | 116 | 2.9% |
| `long[]_[i]` | other | 116 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 62 | 1.5% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 50 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 49 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 37 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007efefda03898_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 33 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112243 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 39060 | 34.80% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 18108 | 16.13% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6350 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5633 | 5.02% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4133 | 3.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1243 | 1.11% |
| `net/minecraft/world/entity/ai/Brain.tick` | 965 | 0.86% |
| `net/minecraft/world/entity/npc/Villager.tick` | 446 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 246 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 237 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 223 | 0.20% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 529 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 504 | 12.5% |
| `char[]_[k]` | 415 | 10.3% |
| `byte[]_[k]` | 260 | 6.4% |
| `byte[]_[i]` | 225 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 158 | 3.9% |
| `int[]_[i]` | 147 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 3.5% |
| `java.util.ArrayList_[i]` | 120 | 3.0% |
| `java.lang.Object[]_[i]` | 116 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 20041 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148297..156105 (delta 7808, churn 5.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99839->110172, minecraft:drowned 3585->4555, minecraft:husk 4518->5478, minecraft:zombie 3784->4650, minecraft:pig 2559->3182, minecraft:skeleton 4353->4868, minecraft:sheep 3085->3516, minecraft:cow 2940->3366
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7808)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54427854 B)
- `wall-collapsed.txt` (3624190 B)
- `alloc-collapsed.txt` (1950059 B)
- `cpu-flamegraph.html` (297059 B)
- `server-stdout.log` (6116019 B)
- `gc.log` (112714 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
