# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.694 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 1.9, 2.1, 2.1, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:07:50Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6733336 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [12:10:15 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [149752, 151513, 153542]
- top entity types (max seen): minecraft:item×106731, minecraft:husk×5411, minecraft:creeper×4945, minecraft:skeleton×4765, minecraft:zombie×4629, minecraft:drowned×4561, minecraft:spider×4468, minecraft:sheep×3517, minecraft:chicken×3390, minecraft:cow×3359, minecraft:pig×3198, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/IEsAGMgWYS
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20700.5 ms**, avg **181.58 ms**, max **2781.8 ms**
- heap high-water seen: **7618 MB** -> last-after: **4042 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103898)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33911 | 32.6% |
| kernel: other | 22114 | 21.3% |
| other | 11071 | 10.7% |
| chunk system (kernel) | 7675 | 7.4% |
| JDK collections | 7440 | 7.2% |
| moonrise/paper patches | 5390 | 5.2% |
| fastutil collections | 4729 | 4.6% |
| JIT stubs (vtable/itable) | 3319 | 3.2% |
| network (kernel) | 2854 | 2.7% |
| JDK invokes/VarHandle | 2778 | 2.7% |
| JDK other | 2213 | 2.1% |
| vdso (clock) | 121 | 0.1% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| bukkit api | 70 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50138 | 48.3% |
| phase: unclassified | 32705 | 31.5% |
| phase: main tick (unclassified) | 13185 | 12.7% |
| phase: chunk tick | 2629 | 2.5% |
| phase: network sync (ServerEntity) | 2138 | 2.1% |
| phase: chunk system (off-main worker) | 1264 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1063 | 1.0% |
| phase: random tick | 500 | 0.5% |
| phase: mob spawning | 272 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92276** (88.8%) · native/JVM-internal **11522** (11.1%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3924 | 3.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3385 | 3.3% |
| `vtable stub` | native/JVM-internal | 2825 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2812 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1867 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1616 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1425 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1413 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1384 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1254 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1252 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1235 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1086 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1065 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1060 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1022 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1009 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1002 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1000 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 990 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 925 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 923 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 892 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 864 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 861 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 859 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 850 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 819 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 799 | 0.8% |
| `colpush_tick` | native/JVM-internal | 799 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 753 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 737 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 714 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 705 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 673 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 646 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 617 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63647)

| bucket | self-time samples | share |
|---|---|---|
| other | 60692 | 95.4% |
| entities/mobs (kernel) | 1057 | 1.7% |
| kernel: other | 763 | 1.2% |
| JDK collections | 237 | 0.4% |
| chunk system (kernel) | 229 | 0.4% |
| moonrise/paper patches | 170 | 0.3% |
| JIT stubs (vtable/itable) | 148 | 0.2% |
| fastutil collections | 129 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 43 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61124 | 96.0% |
| phase: entity tick (AI/movement) | 1764 | 2.8% |
| phase: main tick (unclassified) | 464 | 0.7% |
| phase: chunk tick | 107 | 0.2% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: random tick | 30 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54753** (86.0%) · native/JVM-internal **8888** (14.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51880 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4726 | 7.4% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 136 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 113 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 77 | 0.1% |
| `syscall` | native/JVM-internal | 68 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 60 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 41 | 0.1% |
| `getdents64` | native/JVM-internal | 40 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 36 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 34 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 33 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 33 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3206)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3206 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1932 | 60.3% |
| phase: entity tick (AI/movement) | 954 | 29.8% |
| phase: main tick (unclassified) | 238 | 7.4% |
| phase: chunk system (off-main worker) | 37 | 1.2% |
| phase: network sync (ServerEntity) | 19 | 0.6% |
| phase: block entities (hoppers/furnaces) | 16 | 0.5% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: chunk tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3206** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 369 | 11.5% |
| `char[]_[k]` | other | 329 | 10.3% |
| `byte[]_[k]` | other | 218 | 6.8% |
| `long[]_[i]` | other | 122 | 3.8% |
| `byte[]_[i]` | other | 119 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 117 | 3.6% |
| `int[]_[i]` | other | 109 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 108 | 3.4% |
| `java.lang.Object[]_[i]` | other | 97 | 3.0% |
| `java.util.ArrayList_[i]` | other | 85 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 40 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 31 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 28 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 23 | 0.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 23 | 0.7% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 22 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103898 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19239 | 18.52% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6222 | 5.99% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5010 | 4.82% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3853 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1293 | 1.24% |
| `net/minecraft/world/entity/ai/Brain.tick` | 575 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 384 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 378 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 354 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 268 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 262 | 0.25% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 106 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 369 | 11.5% |
| `char[]_[k]` | 329 | 10.3% |
| `byte[]_[k]` | 218 | 6.8% |
| `long[]_[i]` | 122 | 3.8% |
| `byte[]_[i]` | 119 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 117 | 3.6% |
| `int[]_[i]` | 109 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 108 | 3.4% |
| `java.lang.Object[]_[i]` | 97 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20701 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148253..153542 (delta 5289, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99907->106731, minecraft:drowned 3498->4561, minecraft:zombie 3652->4629, minecraft:husk 4558->5411, minecraft:skeleton 4150->4765, minecraft:creeper 4514->4945, minecraft:spider 4089->4468, minecraft:pig 2859->3198
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5289)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52107103 B)
- `wall-collapsed.txt` (3166904 B)
- `alloc-collapsed.txt` (1831297 B)
- `cpu-flamegraph.html` (281055 B)
- `server-stdout.log` (331494 B)
- `gc.log` (108421 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
