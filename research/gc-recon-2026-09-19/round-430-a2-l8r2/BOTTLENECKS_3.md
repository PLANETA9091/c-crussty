# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.461 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.9, 2.1, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **384.55ms** / min 320.77ms / max **715.11ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T14:23:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6694821 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:25:42 INFO]: [crussty-plugin] [cruss | 320.77 | — | — | — | 715.11 | 384.55 |

- entity totals seen: [150899, 152870, 153804]
- top entity types (max seen): minecraft:item×106910, minecraft:husk×5452, minecraft:creeper×5003, minecraft:skeleton×4788, minecraft:zombie×4592, minecraft:drowned×4524, minecraft:spider×4455, minecraft:sheep×3511, minecraft:chicken×3427, minecraft:cow×3370, minecraft:pig×3215, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/SeS2MqqWYr
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19460.9 ms**, avg **173.76 ms**, max **2492.7 ms**
- heap high-water seen: **7466 MB** -> last-after: **3565 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 102350)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32094 | 31.4% |
| kernel: other | 23352 | 22.8% |
| other | 11408 | 11.1% |
| chunk system (kernel) | 8077 | 7.9% |
| JDK collections | 6153 | 6.0% |
| moonrise/paper patches | 5171 | 5.1% |
| fastutil collections | 4867 | 4.8% |
| JIT stubs (vtable/itable) | 3536 | 3.5% |
| network (kernel) | 2948 | 2.9% |
| JDK invokes/VarHandle | 2479 | 2.4% |
| JDK other | 1810 | 1.8% |
| vdso (clock) | 119 | 0.1% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| bukkit api | 76 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 30 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49524 | 48.4% |
| phase: unclassified | 31676 | 30.9% |
| phase: main tick (unclassified) | 12996 | 12.7% |
| phase: chunk tick | 2630 | 2.6% |
| phase: network sync (ServerEntity) | 2366 | 2.3% |
| phase: chunk system (off-main worker) | 1199 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1099 | 1.1% |
| phase: random tick | 564 | 0.6% |
| phase: mob spawning | 294 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90196** (88.1%) · native/JVM-internal **12074** (11.8%) · other **80** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3674 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3170 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3022 | 3.0% |
| `vtable stub` | native/JVM-internal | 2940 | 2.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1708 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1519 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1492 | 1.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1444 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1309 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1225 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1205 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1205 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1198 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1189 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1064 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1025 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1022 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 968 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 937 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 909 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 899 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 895 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 883 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 875 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 834 | 0.8% |
| `colpush_tick` | native/JVM-internal | 810 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 797 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 781 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 779 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 773 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 735 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 669 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 669 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 666 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 653 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 653 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 633 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 626 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64860)

| bucket | self-time samples | share |
|---|---|---|
| other | 61983 | 95.6% |
| entities/mobs (kernel) | 973 | 1.5% |
| kernel: other | 697 | 1.1% |
| chunk system (kernel) | 265 | 0.4% |
| JDK collections | 199 | 0.3% |
| JIT stubs (vtable/itable) | 182 | 0.3% |
| moonrise/paper patches | 169 | 0.3% |
| fastutil collections | 152 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 75 | 0.1% |
| JDK other | 54 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62281 | 96.0% |
| phase: entity tick (AI/movement) | 1849 | 2.9% |
| phase: main tick (unclassified) | 438 | 0.7% |
| phase: chunk tick | 96 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: block entities (hoppers/furnaces) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55956** (86.3%) · native/JVM-internal **8896** (13.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53185 | 82.0% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 162 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 114 | 0.2% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 84 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 79 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 35 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 34 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 31 | 0.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 29 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3280)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3280 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1956 | 59.6% |
| phase: entity tick (AI/movement) | 981 | 29.9% |
| phase: main tick (unclassified) | 239 | 7.3% |
| phase: chunk system (off-main worker) | 46 | 1.4% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 19 | 0.6% |
| phase: mob spawning | 6 | 0.2% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3280** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 447 | 13.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 434 | 13.2% |
| `char[]_[k]` | other | 314 | 9.6% |
| `byte[]_[k]` | other | 193 | 5.9% |
| `long[]_[i]` | other | 130 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 127 | 3.9% |
| `byte[]_[i]` | other | 113 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 109 | 3.3% |
| `java.lang.Object[]_[i]` | other | 105 | 3.2% |
| `int[]_[i]` | other | 95 | 2.9% |
| `java.util.ArrayList_[i]` | other | 92 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8d7a9f8900_[i]` | other | 37 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.1% |
| `java.lang.String_[i]` | other | 30 | 0.9% |
| `int[]_[k]` | other | 30 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 26 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 26 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f8d7a82f160_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 102350 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18831 | 18.40% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6218 | 6.08% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4948 | 4.83% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3741 | 3.66% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1420 | 1.39% |
| `net/minecraft/world/entity/ai/Brain.tick` | 608 | 0.59% |
| `net/minecraft/world/entity/npc/Villager.tick` | 344 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 270 | 0.26% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 255 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 227 | 0.22% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 152 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 447 | 13.6% |
| `net.minecraft.world.phys.AABB_[i]` | 434 | 13.2% |
| `char[]_[k]` | 314 | 9.6% |
| `byte[]_[k]` | 193 | 5.9% |
| `long[]_[i]` | 130 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 127 | 3.9% |
| `byte[]_[i]` | 113 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 109 | 3.3% |
| `java.lang.Object[]_[i]` | 105 | 3.2% |
| `int[]_[i]` | 95 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19461 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148253..153804 (delta 5551, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99955->106910, minecraft:drowned 3637->4524, minecraft:husk 4579->5452, minecraft:zombie 3811->4592, minecraft:skeleton 4142->4788, minecraft:creeper 4545->5003, minecraft:pig 2870->3215, minecraft:spider 4126->4455
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5551)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52520631 B)
- `wall-collapsed.txt` (2958930 B)
- `alloc-collapsed.txt` (1806971 B)
- `cpu-flamegraph.html` (278428 B)
- `server-stdout.log` (320434 B)
- `gc.log` (106654 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
