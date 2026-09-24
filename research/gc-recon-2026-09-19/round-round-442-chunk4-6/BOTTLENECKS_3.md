# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.245 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.8, 2.1, 2.5, 2.7, 2.9]
- spark tick-monitor MSPT: avg **368.11ms** / min 309.53ms / max **571.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:53:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7050044 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:55:50 INFO]: [crussty-plugin] [cruss | 309.53 | — | — | — | 571.3 | 368.11 |

- entity totals seen: [150897, 153042, 153792]
- top entity types (max seen): minecraft:item×106995, minecraft:husk×5462, minecraft:creeper×4986, minecraft:skeleton×4808, minecraft:zombie×4580, minecraft:drowned×4542, minecraft:spider×4446, minecraft:sheep×3514, minecraft:chicken×3403, minecraft:cow×3369, minecraft:pig×3189, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VtfSjqNvJX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19758.7 ms**, avg **171.82 ms**, max **2524.1 ms**
- heap high-water seen: **7723 MB** -> last-after: **3791 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 103532)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33107 | 32.0% |
| kernel: other | 22251 | 21.5% |
| other | 12275 | 11.9% |
| chunk system (kernel) | 8080 | 7.8% |
| JDK collections | 7032 | 6.8% |
| fastutil collections | 5070 | 4.9% |
| moonrise/paper patches | 5008 | 4.8% |
| JIT stubs (vtable/itable) | 3683 | 3.6% |
| network (kernel) | 2662 | 2.6% |
| JDK invokes/VarHandle | 2251 | 2.2% |
| JDK other | 1625 | 1.6% |
| vdso (clock) | 136 | 0.1% |
| bukkit api | 110 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| redstone (kernel) | 68 | 0.1% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49827 | 48.1% |
| phase: unclassified | 33385 | 32.2% |
| phase: main tick (unclassified) | 12804 | 12.4% |
| phase: chunk tick | 2479 | 2.4% |
| phase: network sync (ServerEntity) | 2137 | 2.1% |
| phase: block entities (hoppers/furnaces) | 1090 | 1.1% |
| phase: chunk system (off-main worker) | 1027 | 1.0% |
| phase: random tick | 505 | 0.5% |
| phase: mob spawning | 275 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90537** (87.4%) · native/JVM-internal **12834** (12.4%) · other **161** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4044 | 3.9% |
| `vtable stub` | native/JVM-internal | 3201 | 3.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3040 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2599 | 2.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1485 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1467 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1333 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1296 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1291 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1270 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1252 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1190 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1160 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1102 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1068 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1053 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 995 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 955 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 953 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 933 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 901 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 901 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 866 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 824 | 0.8% |
| `colpush_tick` | native/JVM-internal | 806 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 767 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 766 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 744 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 743 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 723 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 723 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 711 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 695 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 682 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 672 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 651 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 61903 | 95.4% |
| entities/mobs (kernel) | 1044 | 1.6% |
| kernel: other | 750 | 1.2% |
| chunk system (kernel) | 236 | 0.4% |
| JDK collections | 214 | 0.3% |
| JIT stubs (vtable/itable) | 162 | 0.2% |
| fastutil collections | 156 | 0.2% |
| moonrise/paper patches | 156 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 66 | 0.1% |
| JDK other | 64 | 0.1% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| vdso (clock) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62220 | 95.9% |
| phase: entity tick (AI/movement) | 1892 | 2.9% |
| phase: main tick (unclassified) | 439 | 0.7% |
| phase: chunk tick | 105 | 0.2% |
| phase: network sync (ServerEntity) | 75 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: block entities (hoppers/furnaces) | 43 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56021** (86.4%) · native/JVM-internal **8825** (13.6%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53127 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.4% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 145 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 115 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 63 | 0.1% |
| `syscall` | native/JVM-internal | 56 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 39 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 38 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3568)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3568 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2103 | 58.9% |
| phase: entity tick (AI/movement) | 1066 | 29.9% |
| phase: main tick (unclassified) | 268 | 7.5% |
| phase: chunk system (off-main worker) | 52 | 1.5% |
| phase: block entities (hoppers/furnaces) | 27 | 0.8% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: mob spawning | 13 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3568** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 493 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 467 | 13.1% |
| `char[]_[k]` | other | 415 | 11.6% |
| `byte[]_[k]` | other | 212 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 155 | 4.3% |
| `long[]_[i]` | other | 150 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.9% |
| `java.util.ArrayList_[i]` | other | 103 | 2.9% |
| `byte[]_[i]` | other | 95 | 2.7% |
| `int[]_[i]` | other | 92 | 2.6% |
| `java.lang.Object[]_[i]` | other | 92 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 83 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.5% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fc9e5834440_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 34 | 1.0% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 32 | 0.9% |
| `java.lang.String_[i]` | other | 32 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.9% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103532 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19037 | 18.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6083 | 5.88% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5093 | 4.92% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3812 | 3.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1449 | 1.40% |
| `net/minecraft/world/entity/ai/Brain.tick` | 529 | 0.51% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 398 | 0.38% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 364 | 0.35% |
| `net/minecraft/world/entity/npc/Villager.tick` | 320 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 277 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.22% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 87 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 493 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | 467 | 13.1% |
| `char[]_[k]` | 415 | 11.6% |
| `byte[]_[k]` | 212 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 155 | 4.3% |
| `long[]_[i]` | 150 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.9% |
| `java.util.ArrayList_[i]` | 103 | 2.9% |
| `byte[]_[i]` | 95 | 2.7% |
| `int[]_[i]` | 92 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19759 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148262..153792 (delta 5530, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99928->106995, minecraft:drowned 3659->4542, minecraft:husk 4585->5462, minecraft:zombie 3713->4580, minecraft:skeleton 4175->4808, minecraft:creeper 4587->4986, minecraft:chicken 3055->3403, minecraft:spider 4139->4446
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5530)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52460697 B)
- `wall-collapsed.txt` (3148868 B)
- `alloc-collapsed.txt` (1870397 B)
- `cpu-flamegraph.html` (260836 B)
- `server-stdout.log` (326550 B)
- `gc.log` (109234 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
