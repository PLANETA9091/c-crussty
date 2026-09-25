# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.633 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 2.3, 2.6, 3.1, 3.1, 3.2]
- spark tick-monitor MSPT: avg **398.06ms** / min 275.01ms / max **629.63ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T17:47:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8904014 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:49:12 INFO]: [crussty-plugin] [cruss | 333.06 | — | — | — | 629.63 | 398.06 |

- entity totals seen: [151582, 153996, 154176]
- top entity types (max seen): minecraft:item×107488, minecraft:husk×5452, minecraft:creeper×5053, minecraft:skeleton×4839, minecraft:zombie×4627, minecraft:drowned×4598, minecraft:spider×4490, minecraft:sheep×3493, minecraft:chicken×3426, minecraft:cow×3364, minecraft:pig×3179, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/q36F03E8VR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **137** (Full GC: **10**)
- total pause: **21538.1 ms**, avg **157.21 ms**, max **2370.5 ms**
- heap high-water seen: **7631 MB** -> last-after: **5368 MB**
  - Young (Allocation Failure): 115
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 102592)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32656 | 31.8% |
| kernel: other | 21094 | 20.6% |
| other | 11966 | 11.7% |
| JDK collections | 7890 | 7.7% |
| chunk system (kernel) | 7162 | 7.0% |
| moonrise/paper patches | 5280 | 5.1% |
| fastutil collections | 4659 | 4.5% |
| network (kernel) | 3425 | 3.3% |
| JIT stubs (vtable/itable) | 2838 | 2.8% |
| JDK invokes/VarHandle | 2555 | 2.5% |
| JDK other | 2119 | 2.1% |
| JVM internals (GC oop barriers) | 501 | 0.5% |
| vdso (clock) | 136 | 0.1% |
| block entities/hoppers (kernel) | 95 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| bukkit api | 67 | 0.1% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49177 | 47.9% |
| phase: unclassified | 32408 | 31.6% |
| phase: main tick (unclassified) | 12468 | 12.2% |
| phase: chunk tick | 2820 | 2.7% |
| phase: network sync (ServerEntity) | 2604 | 2.5% |
| phase: chunk system (off-main worker) | 1219 | 1.2% |
| phase: block entities (hoppers/furnaces) | 984 | 1.0% |
| phase: random tick | 584 | 0.6% |
| phase: mob spawning | 327 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90105** (87.8%) · native/JVM-internal **12398** (12.1%) · other **89** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3687 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3175 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2797 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2354 | 2.3% |
| `vtable stub` | native/JVM-internal | 2308 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1886 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1701 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1567 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1451 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1446 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1333 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1299 | 1.3% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1298 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1241 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1208 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1179 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1056 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 989 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 955 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 946 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 914 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 883 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 810 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 801 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 777 | 0.8% |
| `colpush_tick` | native/JVM-internal | 776 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 762 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.get` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 728 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 701 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 700 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 684 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 665 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 654 | 0.6% |
| `net/minecraft/world/entity/Entity.lambda$checkInsideBlocks$2` | JVM-Java | 632 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 611 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 596 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64864)

| bucket | self-time samples | share |
|---|---|---|
| other | 61945 | 95.5% |
| entities/mobs (kernel) | 1009 | 1.6% |
| kernel: other | 651 | 1.0% |
| chunk system (kernel) | 253 | 0.4% |
| JDK collections | 245 | 0.4% |
| JIT stubs (vtable/itable) | 180 | 0.3% |
| moonrise/paper patches | 170 | 0.3% |
| fastutil collections | 156 | 0.2% |
| network (kernel) | 91 | 0.1% |
| JDK invokes/VarHandle | 86 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62196 | 95.9% |
| phase: entity tick (AI/movement) | 1925 | 3.0% |
| phase: main tick (unclassified) | 411 | 0.6% |
| phase: chunk tick | 137 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55991** (86.3%) · native/JVM-internal **8870** (13.7%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53188 | 82.0% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.3% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 161 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 72 | 0.1% |
| `syscall` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 69 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 49 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3892)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3892 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2139 | 55.0% |
| phase: entity tick (AI/movement) | 1334 | 34.3% |
| phase: main tick (unclassified) | 306 | 7.9% |
| phase: block entities (hoppers/furnaces) | 31 | 0.8% |
| phase: network sync (ServerEntity) | 30 | 0.8% |
| phase: chunk system (off-main worker) | 30 | 0.8% |
| phase: mob spawning | 9 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3892** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 680 | 17.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 508 | 13.1% |
| `char[]_[k]` | other | 441 | 11.3% |
| `byte[]_[k]` | other | 235 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 150 | 3.9% |
| `long[]_[i]` | other | 134 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 125 | 3.2% |
| `java.lang.Object[]_[i]` | other | 115 | 3.0% |
| `java.util.ArrayList_[i]` | other | 94 | 2.4% |
| `byte[]_[i]` | other | 93 | 2.4% |
| `int[]_[i]` | other | 81 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 67 | 1.7% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 61 | 1.6% |
| `java.util.ImmutableCollections$List12_[i]` | other | 46 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8372a5b3a0_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.8% |
| `int[]_[k]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 102592 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19135 | 18.65% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5996 | 5.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4936 | 4.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3732 | 3.64% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1055 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 570 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 307 | 0.30% |
| `net/minecraft/world/entity/npc/Villager.tick` | 295 | 0.29% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 282 | 0.27% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 212 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 193 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 84 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 680 | 17.5% |
| `net.minecraft.world.phys.AABB_[i]` | 508 | 13.1% |
| `char[]_[k]` | 441 | 11.3% |
| `byte[]_[k]` | 235 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 150 | 3.9% |
| `long[]_[i]` | 134 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 125 | 3.2% |
| `java.lang.Object[]_[i]` | 115 | 3.0% |
| `java.util.ArrayList_[i]` | 94 | 2.4% |
| `byte[]_[i]` | 93 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 137 pauses / total 21538 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148554..154176 (delta 5622, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100225->107488, minecraft:drowned 3510->4598, minecraft:zombie 3616->4627, minecraft:husk 4648->5452, minecraft:skeleton 4177->4839, minecraft:creeper 4599->5053, minecraft:pig 2846->3179, minecraft:chicken 3109->3426
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5622)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47649022 B)
- `wall-collapsed.txt` (3050931 B)
- `alloc-collapsed.txt` (1971289 B)
- `cpu-flamegraph.html` (258851 B)
- `server-stdout.log` (325423 B)
- `gc.log` (129022 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
