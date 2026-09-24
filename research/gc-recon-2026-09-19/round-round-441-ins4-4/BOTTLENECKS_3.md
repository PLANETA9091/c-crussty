# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.71 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.9, 2.3, 2.6, 1.5, 3.0]
- spark tick-monitor MSPT: avg **350.44ms** / min 312.16ms / max **406.81ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:21:30Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6855477 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:23:38 INFO]: [crussty-plugin] [cruss | 312.16 | — | — | — | 406.81 | 350.44 |

- entity totals seen: [151065, 153389, 154235]
- top entity types (max seen): minecraft:item×107252, minecraft:husk×5540, minecraft:creeper×4984, minecraft:skeleton×4790, minecraft:zombie×4602, minecraft:drowned×4531, minecraft:spider×4454, minecraft:sheep×3509, minecraft:chicken×3427, minecraft:cow×3368, minecraft:pig×3213, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QOqLAV8dNu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19473.3 ms**, avg **169.33 ms**, max **2547.8 ms**
- heap high-water seen: **7677 MB** -> last-after: **4180 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 106779)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35705 | 33.4% |
| kernel: other | 22210 | 20.8% |
| other | 11591 | 10.9% |
| chunk system (kernel) | 7931 | 7.4% |
| JDK collections | 7575 | 7.1% |
| moonrise/paper patches | 5090 | 4.8% |
| fastutil collections | 4689 | 4.4% |
| JIT stubs (vtable/itable) | 3605 | 3.4% |
| network (kernel) | 3055 | 2.9% |
| JDK other | 2575 | 2.4% |
| JDK invokes/VarHandle | 2277 | 2.1% |
| vdso (clock) | 124 | 0.1% |
| bukkit api | 101 | 0.1% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 61 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52682 | 49.3% |
| phase: unclassified | 33848 | 31.7% |
| phase: main tick (unclassified) | 12637 | 11.8% |
| phase: chunk tick | 2534 | 2.4% |
| phase: network sync (ServerEntity) | 2351 | 2.2% |
| phase: chunk system (off-main worker) | 1167 | 1.1% |
| phase: block entities (hoppers/furnaces) | 775 | 0.7% |
| phase: random tick | 470 | 0.4% |
| phase: mob spawning | 312 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94681** (88.7%) · native/JVM-internal **12013** (11.3%) · other **85** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3554 | 3.3% |
| `vtable stub` | native/JVM-internal | 3044 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2798 | 2.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2743 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2021 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1724 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1675 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1368 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1362 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1330 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1287 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1228 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1228 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1193 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1140 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1121 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1065 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 964 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 943 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 921 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 887 | 0.8% |
| `colpush_tick` | native/JVM-internal | 848 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 838 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 831 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 829 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 828 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 779 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 738 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 719 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 719 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 685 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 669 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 669 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 621 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 61881 | 95.4% |
| entities/mobs (kernel) | 1050 | 1.6% |
| kernel: other | 675 | 1.0% |
| chunk system (kernel) | 242 | 0.4% |
| JDK collections | 238 | 0.4% |
| JIT stubs (vtable/itable) | 169 | 0.3% |
| moonrise/paper patches | 167 | 0.3% |
| fastutil collections | 115 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK other | 83 | 0.1% |
| JVM internals (GC oop barriers) | 65 | 0.1% |
| JDK invokes/VarHandle | 57 | 0.1% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62237 | 96.0% |
| phase: entity tick (AI/movement) | 1847 | 2.8% |
| phase: main tick (unclassified) | 460 | 0.7% |
| phase: chunk tick | 137 | 0.2% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55545** (85.6%) · native/JVM-internal **9307** (14.4%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52732 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.4% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 323 | 0.5% |
| `vtable stub` | native/JVM-internal | 144 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 105 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 38 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3591)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3591 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2097 | 58.4% |
| phase: entity tick (AI/movement) | 1160 | 32.3% |
| phase: main tick (unclassified) | 228 | 6.3% |
| phase: chunk system (off-main worker) | 48 | 1.3% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: mob spawning | 13 | 0.4% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3591** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 454 | 12.6% |
| `char[]_[k]` | other | 369 | 10.3% |
| `byte[]_[k]` | other | 224 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 168 | 4.7% |
| `long[]_[i]` | other | 144 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 4.0% |
| `java.util.ArrayList_[i]` | other | 129 | 3.6% |
| `java.lang.Object[]_[i]` | other | 103 | 2.9% |
| `byte[]_[i]` | other | 100 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 2.3% |
| `int[]_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.3% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc6999f2058_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106779 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19939 | 18.67% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6323 | 5.92% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5312 | 4.97% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4116 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1493 | 1.40% |
| `net/minecraft/world/entity/ai/Brain.tick` | 647 | 0.61% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 436 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 408 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 391 | 0.37% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 305 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 260 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 129 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | 454 | 12.6% |
| `char[]_[k]` | 369 | 10.3% |
| `byte[]_[k]` | 224 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 168 | 4.7% |
| `long[]_[i]` | 144 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 4.0% |
| `java.util.ArrayList_[i]` | 129 | 3.6% |
| `java.lang.Object[]_[i]` | 103 | 2.9% |
| `byte[]_[i]` | 100 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19473 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148311..154235 (delta 5924, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 100001->107252, minecraft:drowned 3495->4531, minecraft:husk 4586->5540, minecraft:zombie 3725->4602, minecraft:skeleton 4113->4790, minecraft:creeper 4559->4984, minecraft:spider 4145->4454, minecraft:pig 2917->3213
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5924)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56213601 B)
- `wall-collapsed.txt` (3124174 B)
- `alloc-collapsed.txt` (1859656 B)
- `cpu-flamegraph.html` (287755 B)
- `server-stdout.log` (276700 B)
- `gc.log` (109219 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
