# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.327 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 2.0, 2.2, 2.7, 2.9, 3.0]
- spark tick-monitor MSPT: avg **339.06ms** / min 300.9ms / max **400.28ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:10:56Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6840939 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:13:09 INFO]: [crussty-plugin] [cruss | 300.9 | — | — | — | 400.28 | 339.06 |

- entity totals seen: [150948, 153323, 153968]
- top entity types (max seen): minecraft:item×107034, minecraft:husk×5460, minecraft:creeper×5016, minecraft:skeleton×4806, minecraft:zombie×4581, minecraft:drowned×4540, minecraft:spider×4484, minecraft:sheep×3517, minecraft:chicken×3400, minecraft:cow×3369, minecraft:pig×3188, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QR7WvM5QEy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **18934.3 ms**, avg **167.56 ms**, max **2530.4 ms**
- heap high-water seen: **7888 MB** -> last-after: **3843 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107818)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36421 | 33.8% |
| kernel: other | 22233 | 20.6% |
| other | 11799 | 10.9% |
| chunk system (kernel) | 7953 | 7.4% |
| JDK collections | 7263 | 6.7% |
| moonrise/paper patches | 5238 | 4.9% |
| fastutil collections | 5031 | 4.7% |
| JIT stubs (vtable/itable) | 3770 | 3.5% |
| network (kernel) | 2846 | 2.6% |
| JDK invokes/VarHandle | 2399 | 2.2% |
| JDK other | 2348 | 2.2% |
| vdso (clock) | 146 | 0.1% |
| bukkit api | 100 | 0.1% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| craftbukkit glue | 95 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52871 | 49.0% |
| phase: unclassified | 34707 | 32.2% |
| phase: main tick (unclassified) | 12668 | 11.7% |
| phase: chunk tick | 2364 | 2.2% |
| phase: network sync (ServerEntity) | 2286 | 2.1% |
| phase: chunk system (off-main worker) | 1397 | 1.3% |
| phase: block entities (hoppers/furnaces) | 734 | 0.7% |
| phase: random tick | 486 | 0.5% |
| phase: mob spawning | 302 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95322** (88.4%) · native/JVM-internal **12418** (11.5%) · other **78** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3673 | 3.4% |
| `vtable stub` | native/JVM-internal | 3231 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2791 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2627 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2207 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1579 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1365 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1352 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1343 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1290 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1290 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1246 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1203 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1192 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1070 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1059 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1039 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1003 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 946 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 941 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 909 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 851 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 850 | 0.8% |
| `colpush_tick` | native/JVM-internal | 850 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 842 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 829 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 801 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 732 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 721 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 714 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 711 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 693 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 681 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 652 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 633 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64862)

| bucket | self-time samples | share |
|---|---|---|
| other | 61761 | 95.2% |
| entities/mobs (kernel) | 1177 | 1.8% |
| kernel: other | 671 | 1.0% |
| JDK collections | 237 | 0.4% |
| chunk system (kernel) | 235 | 0.4% |
| moonrise/paper patches | 202 | 0.3% |
| JIT stubs (vtable/itable) | 177 | 0.3% |
| fastutil collections | 165 | 0.3% |
| network (kernel) | 99 | 0.2% |
| JDK other | 69 | 0.1% |
| JDK invokes/VarHandle | 54 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| vdso (clock) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62120 | 95.8% |
| phase: entity tick (AI/movement) | 2037 | 3.1% |
| phase: main tick (unclassified) | 422 | 0.7% |
| phase: chunk tick | 97 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 32 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55950** (86.3%) · native/JVM-internal **8911** (13.7%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52931 | 81.6% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.4% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 159 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 94 | 0.1% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 80 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 41 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 35 | 0.1% |
| `colpush_tick` | native/JVM-internal | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3649)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3649 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2182 | 59.8% |
| phase: entity tick (AI/movement) | 1113 | 30.5% |
| phase: main tick (unclassified) | 244 | 6.7% |
| phase: chunk system (off-main worker) | 47 | 1.3% |
| phase: network sync (ServerEntity) | 31 | 0.8% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3649** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 517 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 500 | 13.7% |
| `char[]_[k]` | other | 442 | 12.1% |
| `byte[]_[k]` | other | 219 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 154 | 4.2% |
| `long[]_[i]` | other | 135 | 3.7% |
| `java.util.ArrayList_[i]` | other | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 125 | 3.4% |
| `java.lang.Object[]_[i]` | other | 104 | 2.9% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.1% |
| `int[]_[i]` | other | 63 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f53859f0ec0_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f5385830950_[i]` | other | 32 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107818 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19767 | 18.33% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6540 | 6.07% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5448 | 5.05% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4145 | 3.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1471 | 1.36% |
| `net/minecraft/world/entity/ai/Brain.tick` | 623 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 427 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 393 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 335 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 286 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 280 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 112 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 517 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 500 | 13.7% |
| `char[]_[k]` | 442 | 12.1% |
| `byte[]_[k]` | 219 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 154 | 4.2% |
| `long[]_[i]` | 135 | 3.7% |
| `java.util.ArrayList_[i]` | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 125 | 3.4% |
| `java.lang.Object[]_[i]` | 104 | 2.9% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 18934 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148218..153968 (delta 5750, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99898->107034, minecraft:drowned 3576->4540, minecraft:zombie 3669->4581, minecraft:husk 4570->5460, minecraft:skeleton 4126->4806, minecraft:creeper 4572->5016, minecraft:spider 4138->4484, minecraft:chicken 3123->3400
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5750)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58266932 B)
- `wall-collapsed.txt` (3311770 B)
- `alloc-collapsed.txt` (1840179 B)
- `cpu-flamegraph.html` (273742 B)
- `server-stdout.log` (280307 B)
- `gc.log` (107487 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
