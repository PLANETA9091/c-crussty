# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.478 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.5, 2.2, 2.5, 3.1, 3.0, 2.7]
- spark tick-monitor MSPT: avg **408.08ms** / min 276.02ms / max **568.02ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T18:18:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8678220 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:20:16 INFO]: [crussty-plugin] [cruss | 342.83 | — | — | — | 568.02 | 408.08 |

- entity totals seen: [152261, 157640, 159537]
- top entity types (max seen): minecraft:item×113149, minecraft:husk×5807, minecraft:creeper×5047, minecraft:skeleton×4799, minecraft:zombie×4657, minecraft:drowned×4595, minecraft:spider×4171, minecraft:sheep×3507, minecraft:chicken×3407, minecraft:cow×3362, minecraft:pig×3193, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/dGIw87C1sn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **18833.8 ms**, avg **160.97 ms**, max **2483.6 ms**
- heap high-water seen: **6972 MB** -> last-after: **3720 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 109469)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28924 | 26.4% |
| kernel: other | 21354 | 19.5% |
| other | 20892 | 19.1% |
| chunk system (kernel) | 10102 | 9.2% |
| JDK collections | 6701 | 6.1% |
| moonrise/paper patches | 5768 | 5.3% |
| fastutil collections | 5010 | 4.6% |
| network (kernel) | 2686 | 2.5% |
| JDK invokes/VarHandle | 2573 | 2.4% |
| JIT stubs (vtable/itable) | 2550 | 2.3% |
| JDK other | 1928 | 1.8% |
| JVM internals (GC oop barriers) | 429 | 0.4% |
| vdso (clock) | 128 | 0.1% |
| redstone (kernel) | 107 | 0.1% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| craftbukkit glue | 92 | 0.1% |
| bukkit api | 88 | 0.1% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 57028 | 52.1% |
| phase: unclassified | 33387 | 30.5% |
| phase: main tick (unclassified) | 10913 | 10.0% |
| phase: network sync (ServerEntity) | 2668 | 2.4% |
| phase: chunk tick | 2398 | 2.2% |
| phase: chunk system (off-main worker) | 1270 | 1.2% |
| phase: block entities (hoppers/furnaces) | 899 | 0.8% |
| phase: random tick | 515 | 0.5% |
| phase: mob spawning | 391 | 0.4% |

**JVM-vs-native split (leaf self-time):** JVM-Java **88427** (80.8%) · native/JVM-internal **20630** (18.8%) · other **412** (0.4%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 7851 | 7.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4611 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2991 | 2.7% |
| `vtable stub` | native/JVM-internal | 2114 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1624 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1544 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1537 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1512 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1426 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1411 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1288 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1281 | 1.2% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1230 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1210 | 1.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1198 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1132 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1124 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1081 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1075 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1063 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 972 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 954 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 948 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 895 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 890 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 875 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 812 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 783 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 753 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 712 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 690 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 644 | 0.6% |
| `mob_upsert` | native/JVM-internal | 630 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 624 | 0.6% |
| `net/minecraft/world/entity/Entity.setPosRaw` | JVM-Java | 601 | 0.5% |
| `net/minecraft/world/entity/MobPushOps.collect` | JVM-Java | 595 | 0.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 594 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 586 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 58395 | 95.3% |
| entities/mobs (kernel) | 953 | 1.6% |
| kernel: other | 719 | 1.2% |
| chunk system (kernel) | 273 | 0.4% |
| moonrise/paper patches | 208 | 0.3% |
| JDK collections | 200 | 0.3% |
| fastutil collections | 155 | 0.3% |
| JIT stubs (vtable/itable) | 99 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| network (kernel) | 75 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| redstone (kernel) | 8 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58523 | 95.6% |
| phase: entity tick (AI/movement) | 2033 | 3.3% |
| phase: main tick (unclassified) | 399 | 0.7% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 78 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: mob spawning | 26 | 0.0% |
| phase: random tick | 21 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52128** (85.1%) · native/JVM-internal **9105** (14.9%) · other **15** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49306 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4787 | 7.8% |
| `read` | native/JVM-internal | 1211 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 212 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 126 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `vtable stub` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4575)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4575 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3023 | 66.1% |
| phase: entity tick (AI/movement) | 1045 | 22.8% |
| phase: main tick (unclassified) | 328 | 7.2% |
| phase: chunk system (off-main worker) | 81 | 1.8% |
| phase: block entities (hoppers/furnaces) | 37 | 0.8% |
| phase: network sync (ServerEntity) | 35 | 0.8% |
| phase: chunk tick | 11 | 0.2% |
| phase: mob spawning | 11 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4575** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 551 | 12.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 529 | 11.6% |
| `char[]_[k]` | other | 434 | 9.5% |
| `int[]_[i]` | other | 232 | 5.1% |
| `byte[]_[k]` | other | 231 | 5.0% |
| `byte[]_[i]` | other | 193 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 3.5% |
| `long[]_[i]` | other | 143 | 3.1% |
| `java.util.Calendar$Builder_[i]` | other | 136 | 3.0% |
| `java.util.ArrayList_[i]` | other | 132 | 2.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 131 | 2.9% |
| `java.lang.Object[]_[i]` | other | 117 | 2.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 79 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 60 | 1.3% |
| `int[]_[k]` | other | 55 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb4f282baf0_[i]` | other | 53 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 52 | 1.1% |
| `java.util.GregorianCalendar_[i]` | other | 49 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 48 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 109469 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19337 | 17.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7113 | 6.50% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6966 | 6.36% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4786 | 4.37% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1333 | 1.22% |
| `net/minecraft/world/entity/ai/Brain.tick` | 690 | 0.63% |
| `net/minecraft/world/entity/npc/Villager.tick` | 416 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 309 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 267 | 0.24% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 191 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 96 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 551 | 12.0% |
| `net.minecraft.world.phys.AABB_[i]` | 529 | 11.6% |
| `char[]_[k]` | 434 | 9.5% |
| `int[]_[i]` | 232 | 5.1% |
| `byte[]_[k]` | 231 | 5.0% |
| `byte[]_[i]` | 193 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 3.5% |
| `long[]_[i]` | 143 | 3.1% |
| `java.util.Calendar$Builder_[i]` | 136 | 3.0% |
| `java.util.ArrayList_[i]` | 132 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 18834 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148482..159537 (delta 11055, churn 7.2%), summons=0
  - top movers (max-min across polls): minecraft:item 100111->113149, minecraft:drowned 3436->4595, minecraft:husk 4664->5807, minecraft:zombie 3542->4657, minecraft:skeleton 3983->4799, minecraft:sheep 2901->3507, minecraft:pig 2597->3193, minecraft:chicken 2848->3407
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=11055)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46932070 B)
- `wall-collapsed.txt` (3047350 B)
- `alloc-collapsed.txt` (1814603 B)
- `cpu-flamegraph.html` (272791 B)
- `server-stdout.log` (685102 B)
- `gc.log` (110899 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
