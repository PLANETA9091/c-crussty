# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.198 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.8, 2.1, 2.5, 2.7, 2.8]
- spark tick-monitor MSPT: avg **375.09ms** / min 304.85ms / max **492.03ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:53:41Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7008384 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:55:53 INFO]: [crussty-plugin] [cruss | 304.85 | — | — | — | 492.03 | 375.09 |

- entity totals seen: [150715, 152619, 153443]
- top entity types (max seen): minecraft:item×106694, minecraft:husk×5449, minecraft:creeper×4930, minecraft:skeleton×4798, minecraft:zombie×4607, minecraft:spider×4544, minecraft:drowned×4542, minecraft:sheep×3497, minecraft:chicken×3424, minecraft:cow×3387, minecraft:pig×3201, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QjxIzXc31A
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **20074.7 ms**, avg **174.56 ms**, max **2511.5 ms**
- heap high-water seen: **7591 MB** -> last-after: **3816 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104639)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34415 | 32.9% |
| kernel: other | 22386 | 21.4% |
| other | 11712 | 11.2% |
| chunk system (kernel) | 7951 | 7.6% |
| JDK collections | 7574 | 7.2% |
| moonrise/paper patches | 4969 | 4.7% |
| fastutil collections | 4738 | 4.5% |
| JIT stubs (vtable/itable) | 3623 | 3.5% |
| network (kernel) | 2752 | 2.6% |
| JDK invokes/VarHandle | 2312 | 2.2% |
| JDK other | 1726 | 1.6% |
| vdso (clock) | 120 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| craftbukkit glue | 88 | 0.1% |
| bukkit api | 79 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50569 | 48.3% |
| phase: unclassified | 33575 | 32.1% |
| phase: main tick (unclassified) | 12960 | 12.4% |
| phase: chunk tick | 2483 | 2.4% |
| phase: network sync (ServerEntity) | 2261 | 2.2% |
| phase: chunk system (off-main worker) | 1083 | 1.0% |
| phase: block entities (hoppers/furnaces) | 899 | 0.9% |
| phase: random tick | 505 | 0.5% |
| phase: mob spawning | 303 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92036** (88.0%) · native/JVM-internal **12495** (11.9%) · other **108** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3883 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3017 | 2.9% |
| `vtable stub` | native/JVM-internal | 2992 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2457 | 2.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1527 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1476 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1392 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1390 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1300 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1280 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1210 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1151 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1115 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1093 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1077 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1042 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1003 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 948 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 925 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 920 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 917 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 909 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 894 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 849 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 826 | 0.8% |
| `colpush_tick` | native/JVM-internal | 803 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 801 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 789 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 764 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 756 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 736 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 732 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 706 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 699 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 690 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 631 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 630 | 0.6% |
| `itable stub` | native/JVM-internal | 626 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61916 | 95.5% |
| entities/mobs (kernel) | 1121 | 1.7% |
| kernel: other | 706 | 1.1% |
| chunk system (kernel) | 232 | 0.4% |
| JDK collections | 215 | 0.3% |
| moonrise/paper patches | 147 | 0.2% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| fastutil collections | 143 | 0.2% |
| network (kernel) | 91 | 0.1% |
| JDK invokes/VarHandle | 66 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62239 | 96.0% |
| phase: entity tick (AI/movement) | 1859 | 2.9% |
| phase: main tick (unclassified) | 475 | 0.7% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55994** (86.3%) · native/JVM-internal **8861** (13.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53107 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 7.3% |
| `read` | native/JVM-internal | 1223 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 128 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 70 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 36 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 35 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3472)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3472 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2061 | 59.4% |
| phase: entity tick (AI/movement) | 1064 | 30.6% |
| phase: main tick (unclassified) | 250 | 7.2% |
| phase: chunk system (off-main worker) | 34 | 1.0% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 24 | 0.7% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3472** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 504 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 436 | 12.6% |
| `char[]_[k]` | other | 397 | 11.4% |
| `byte[]_[k]` | other | 225 | 6.5% |
| `long[]_[i]` | other | 141 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 134 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 115 | 3.3% |
| `java.util.ArrayList_[i]` | other | 111 | 3.2% |
| `java.lang.Object[]_[i]` | other | 107 | 3.1% |
| `byte[]_[i]` | other | 80 | 2.3% |
| `int[]_[i]` | other | 77 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 51 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f28d2a06260_[i]` | other | 32 | 0.9% |
| `java.lang.String_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104639 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19277 | 18.42% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6098 | 5.83% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5109 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3818 | 3.65% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1390 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tick` | 625 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 382 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 352 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 343 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 274 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 504 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 436 | 12.6% |
| `char[]_[k]` | 397 | 11.4% |
| `byte[]_[k]` | 225 | 6.5% |
| `long[]_[i]` | 141 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 134 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 115 | 3.3% |
| `java.util.ArrayList_[i]` | 111 | 3.2% |
| `java.lang.Object[]_[i]` | 107 | 3.1% |
| `byte[]_[i]` | 80 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 20075 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148162..153443 (delta 5281, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99830->106694, minecraft:husk 4557->5449, minecraft:zombie 3730->4607, minecraft:drowned 3683->4542, minecraft:skeleton 4193->4798, minecraft:spider 4114->4544, minecraft:creeper 4529->4930, minecraft:pig 2864->3201
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5281)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54378736 B)
- `wall-collapsed.txt` (3128190 B)
- `alloc-collapsed.txt` (1828352 B)
- `cpu-flamegraph.html` (274144 B)
- `server-stdout.log` (325675 B)
- `gc.log` (109238 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
