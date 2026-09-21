# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.898 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.0, 1.8, 2.2, 2.5, 2.6, 2.7]
- spark tick-monitor MSPT: avg **384.92ms** / min 328.12ms / max **585.22ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T22:08:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6905070 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [crussty-plugin] cmp406_sscan: ARMED hea | 328.12 | — | — | — | 585.22 | 384.92 |

- entity totals seen: [150339, 153196, 156389]
- top entity types (max seen): minecraft:item×110522, minecraft:husk×5416, minecraft:creeper×5003, minecraft:skeleton×4860, minecraft:zombie×4676, minecraft:drowned×4568, minecraft:spider×4322, minecraft:sheep×3490, minecraft:chicken×3421, minecraft:cow×3364, minecraft:pig×3176, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xiqLIOID0b
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **19677.4 ms**, avg **165.36 ms**, max **2311.4 ms**
- heap high-water seen: **7009 MB** -> last-after: **3420 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113276)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32676 | 28.8% |
| kernel: other | 27221 | 24.0% |
| other | 10791 | 9.5% |
| chunk system (kernel) | 9691 | 8.6% |
| moonrise/paper patches | 7661 | 6.8% |
| JDK collections | 6229 | 5.5% |
| fastutil collections | 5752 | 5.1% |
| JIT stubs (vtable/itable) | 3794 | 3.3% |
| network (kernel) | 3661 | 3.2% |
| JDK other | 2672 | 2.4% |
| JDK invokes/VarHandle | 2532 | 2.2% |
| vdso (clock) | 256 | 0.2% |
| bukkit api | 99 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93473 | 82.5% |
| phase: unclassified | 9100 | 8.0% |
| phase: main tick (unclassified) | 3702 | 3.3% |
| phase: network sync (ServerEntity) | 2335 | 2.1% |
| phase: chunk tick | 2258 | 2.0% |
| phase: chunk system (off-main worker) | 1160 | 1.0% |
| phase: block entities (hoppers/furnaces) | 684 | 0.6% |
| phase: random tick | 416 | 0.4% |
| phase: mob spawning | 144 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101719** (89.8%) · native/JVM-internal **11477** (10.1%) · other **80** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4524 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3512 | 3.1% |
| `vtable stub` | native/JVM-internal | 2996 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2698 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2097 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1834 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1816 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1610 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1602 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1497 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1447 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1392 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1374 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1205 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1175 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1165 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1163 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1140 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1100 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1049 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 977 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 971 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 958 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 939 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 890 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 851 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 842 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 830 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 817 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 796 | 0.7% |
| `itable stub` | native/JVM-internal | 795 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 727 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 715 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 685 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57932 | 94.6% |
| entities/mobs (kernel) | 1070 | 1.7% |
| kernel: other | 887 | 1.4% |
| chunk system (kernel) | 300 | 0.5% |
| moonrise/paper patches | 275 | 0.4% |
| JDK collections | 177 | 0.3% |
| fastutil collections | 168 | 0.3% |
| JIT stubs (vtable/itable) | 147 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JDK other | 88 | 0.1% |
| JDK invokes/VarHandle | 71 | 0.1% |
| vdso (clock) | 15 | 0.0% |
| bukkit api | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57772 | 94.3% |
| phase: entity tick (AI/movement) | 3004 | 4.9% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: network sync (ServerEntity) | 85 | 0.1% |
| phase: chunk tick | 84 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52308** (85.4%) · native/JVM-internal **8943** (14.6%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49057 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `vtable stub` | native/JVM-internal | 120 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 87 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 39 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 38 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3649)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3649 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2008 | 55.0% |
| phase: unclassified | 1432 | 39.2% |
| phase: main tick (unclassified) | 104 | 2.9% |
| phase: chunk system (off-main worker) | 45 | 1.2% |
| phase: network sync (ServerEntity) | 35 | 1.0% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3649** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 528 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 509 | 13.9% |
| `char[]_[k]` | other | 428 | 11.7% |
| `byte[]_[k]` | other | 212 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 163 | 4.5% |
| `long[]_[i]` | other | 150 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 141 | 3.9% |
| `java.util.ArrayList_[i]` | other | 134 | 3.7% |
| `java.lang.Object[]_[i]` | other | 99 | 2.7% |
| `byte[]_[i]` | other | 89 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 2.2% |
| `int[]_[i]` | other | 68 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 42 | 1.2% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f16f19de9b8_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f16f19e1718_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113276 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 39136 | 34.55% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 18556 | 16.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6357 | 5.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5600 | 4.94% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4360 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1310 | 1.16% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1020 | 0.90% |
| `net/minecraft/world/entity/npc/Villager.tick` | 473 | 0.42% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 255 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 236 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 227 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 219 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 528 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 509 | 13.9% |
| `char[]_[k]` | 428 | 11.7% |
| `byte[]_[k]` | 212 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 163 | 4.5% |
| `long[]_[i]` | 150 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 141 | 3.9% |
| `java.util.ArrayList_[i]` | 134 | 3.7% |
| `java.lang.Object[]_[i]` | 99 | 2.7% |
| `byte[]_[i]` | 89 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 19677 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148263..156389 (delta 8126, churn 5.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99890->110522, minecraft:drowned 3645->4568, minecraft:husk 4505->5416, minecraft:zombie 3881->4676, minecraft:pig 2483->3176, minecraft:skeleton 4323->4860, minecraft:spider 3877->4322, minecraft:creeper 4559->5003
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8126)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50547176 B)
- `wall-collapsed.txt` (3492087 B)
- `alloc-collapsed.txt` (1796765 B)
- `cpu-flamegraph.html` (294576 B)
- `server-stdout.log` (258507 B)
- `gc.log` (112702 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
