# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.288 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.2, 1.9, 2.1, 2.5, 2.7, 2.6]
- spark tick-monitor MSPT: avg **372.13ms** / min 309.61ms / max **529.74ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:54:20Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6979303 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:56:29 INFO]: [crussty-plugin] [cruss | 309.61 | — | — | — | 529.74 | 372.13 |

- entity totals seen: [150809, 152780, 153690]
- top entity types (max seen): minecraft:item×106908, minecraft:husk×5385, minecraft:creeper×5005, minecraft:skeleton×4815, minecraft:zombie×4578, minecraft:drowned×4546, minecraft:spider×4523, minecraft:sheep×3516, minecraft:chicken×3404, minecraft:cow×3370, minecraft:pig×3188, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wDd2amZYmL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **108** (Full GC: **5**)
- total pause: **13090.0 ms**, avg **121.20 ms**, max **553.9 ms**
- heap high-water seen: **9139 MB** -> last-after: **5849 MB**
  - Young (Allocation Failure): 97
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 1
  - Full (CodeCache GC Threshold): 1
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103478)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34816 | 33.6% |
| kernel: other | 22518 | 21.8% |
| other | 11538 | 11.2% |
| chunk system (kernel) | 7439 | 7.2% |
| JDK collections | 6915 | 6.7% |
| moonrise/paper patches | 4969 | 4.8% |
| fastutil collections | 4567 | 4.4% |
| JIT stubs (vtable/itable) | 3546 | 3.4% |
| network (kernel) | 2484 | 2.4% |
| JDK invokes/VarHandle | 2425 | 2.3% |
| JDK other | 1813 | 1.8% |
| vdso (clock) | 113 | 0.1% |
| bukkit api | 95 | 0.1% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| craftbukkit glue | 82 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49444 | 47.8% |
| phase: unclassified | 33150 | 32.0% |
| phase: main tick (unclassified) | 13181 | 12.7% |
| phase: chunk tick | 2697 | 2.6% |
| phase: network sync (ServerEntity) | 2043 | 2.0% |
| phase: block entities (hoppers/furnaces) | 1106 | 1.1% |
| phase: chunk system (off-main worker) | 1039 | 1.0% |
| phase: random tick | 518 | 0.5% |
| phase: mob spawning | 299 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91146** (88.1%) · native/JVM-internal **12227** (11.8%) · other **105** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3464 | 3.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3280 | 3.2% |
| `vtable stub` | native/JVM-internal | 3062 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2685 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 2304 | 2.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1644 | 1.6% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1505 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1466 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1390 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1386 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1357 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1243 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1184 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1130 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1083 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1033 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1012 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 973 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 950 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 941 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 915 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 901 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 859 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 852 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 831 | 0.8% |
| `colpush_tick` | native/JVM-internal | 802 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 794 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 793 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 778 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 739 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 730 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 686 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 664 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 631 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 602 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 601 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 570 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61863 | 95.4% |
| entities/mobs (kernel) | 1087 | 1.7% |
| kernel: other | 737 | 1.1% |
| JDK collections | 232 | 0.4% |
| chunk system (kernel) | 213 | 0.3% |
| JIT stubs (vtable/itable) | 184 | 0.3% |
| moonrise/paper patches | 174 | 0.3% |
| fastutil collections | 137 | 0.2% |
| network (kernel) | 98 | 0.2% |
| JDK other | 62 | 0.1% |
| JDK invokes/VarHandle | 57 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62204 | 95.9% |
| phase: entity tick (AI/movement) | 1908 | 2.9% |
| phase: main tick (unclassified) | 462 | 0.7% |
| phase: chunk tick | 106 | 0.2% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55952** (86.3%) · native/JVM-internal **8903** (13.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53067 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4741 | 7.3% |
| `read` | native/JVM-internal | 1233 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 157 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 99 | 0.2% |
| `syscall` | native/JVM-internal | 66 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 38 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 37 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 35 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3491)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3491 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2054 | 58.8% |
| phase: entity tick (AI/movement) | 1053 | 30.2% |
| phase: main tick (unclassified) | 266 | 7.6% |
| phase: chunk system (off-main worker) | 51 | 1.5% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: block entities (hoppers/furnaces) | 21 | 0.6% |
| phase: random tick | 11 | 0.3% |
| phase: mob spawning | 6 | 0.2% |
| phase: chunk tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3491** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 493 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 435 | 12.5% |
| `char[]_[k]` | other | 361 | 10.3% |
| `byte[]_[k]` | other | 219 | 6.3% |
| `long[]_[i]` | other | 145 | 4.2% |
| `java.lang.Object[]_[i]` | other | 136 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 128 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 121 | 3.5% |
| `java.util.ArrayList_[i]` | other | 112 | 3.2% |
| `byte[]_[i]` | other | 102 | 2.9% |
| `int[]_[i]` | other | 95 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 63 | 1.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 36 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fea07a12238_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 27 | 0.8% |
| `java.lang.String_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103478 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18985 | 18.35% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5944 | 5.74% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5097 | 4.93% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3784 | 3.66% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1391 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 586 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 384 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 359 | 0.35% |
| `net/minecraft/world/entity/npc/Villager.tick` | 301 | 0.29% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 273 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 200 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 92 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 493 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | 435 | 12.5% |
| `char[]_[k]` | 361 | 10.3% |
| `byte[]_[k]` | 219 | 6.3% |
| `long[]_[i]` | 145 | 4.2% |
| `java.lang.Object[]_[i]` | 136 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 128 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 121 | 3.5% |
| `java.util.ArrayList_[i]` | 112 | 3.2% |
| `byte[]_[i]` | 102 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 108 pauses / total 13090 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148316..153690 (delta 5374, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99986->106908, minecraft:zombie 3708->4578, minecraft:drowned 3696->4546, minecraft:husk 4583->5385, minecraft:skeleton 4223->4815, minecraft:creeper 4584->5005, minecraft:spider 4153->4523, minecraft:chicken 3080->3404
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5374)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52495258 B)
- `wall-collapsed.txt` (3194971 B)
- `alloc-collapsed.txt` (1893665 B)
- `cpu-flamegraph.html` (275286 B)
- `server-stdout.log` (322719 B)
- `gc.log` (99642 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
