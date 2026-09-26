# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.649 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 2.1, 2.6, 2.9, 3.0, 3.0]
- spark tick-monitor MSPT: avg **410.07ms** / min 287.39ms / max **557.21ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T23:36:02Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8183686 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 287.39 | — | — | — | 557.21 | 366.28 |

- entity totals seen: [150874, 154725, 157182]
- top entity types (max seen): minecraft:item×110989, minecraft:husk×5667, minecraft:creeper×5041, minecraft:skeleton×4873, minecraft:zombie×4631, minecraft:drowned×4517, minecraft:spider×4266, minecraft:sheep×3542, minecraft:chicken×3430, minecraft:cow×3318, minecraft:pig×3184, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/zyXXDH8O7b
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **133** (Full GC: **10**)
- total pause: **20042.8 ms**, avg **150.70 ms**, max **2123.4 ms**
- heap high-water seen: **7227 MB** -> last-after: **3528 MB**
  - Young (Allocation Failure): 113
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 110320)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31631 | 28.7% |
| kernel: other | 24130 | 21.9% |
| other | 11217 | 10.2% |
| chunk system (kernel) | 9476 | 8.6% |
| JDK collections | 7682 | 7.0% |
| moonrise/paper patches | 7561 | 6.9% |
| fastutil collections | 6314 | 5.7% |
| network (kernel) | 3694 | 3.3% |
| JIT stubs (vtable/itable) | 2710 | 2.5% |
| JDK other | 2492 | 2.3% |
| JDK invokes/VarHandle | 2425 | 2.2% |
| JVM internals (GC oop barriers) | 463 | 0.4% |
| vdso (clock) | 247 | 0.2% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| bukkit api | 60 | 0.1% |
| redstone (kernel) | 36 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87283 | 79.1% |
| phase: unclassified | 11130 | 10.1% |
| phase: main tick (unclassified) | 4177 | 3.8% |
| phase: network sync (ServerEntity) | 2604 | 2.4% |
| phase: chunk tick | 2268 | 2.1% |
| phase: chunk system (off-main worker) | 1374 | 1.2% |
| phase: block entities (hoppers/furnaces) | 854 | 0.8% |
| phase: random tick | 492 | 0.4% |
| phase: mob spawning | 137 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98186** (89.0%) · native/JVM-internal **11985** (10.9%) · other **149** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4822 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3583 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2427 | 2.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2401 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2232 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2205 | 2.0% |
| `vtable stub` | native/JVM-internal | 2062 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1666 | 1.5% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f7f8ba68238.accept` | JVM-Java | 1592 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1565 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1466 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1443 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1217 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1100 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1093 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1089 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1088 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1066 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1063 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1046 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1036 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 956 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 947 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 940 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 865 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 862 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 860 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 855 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 855 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 793 | 0.7% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 770 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 720 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 674 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 667 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64865)

| bucket | self-time samples | share |
|---|---|---|
| other | 61623 | 95.0% |
| entities/mobs (kernel) | 1042 | 1.6% |
| kernel: other | 825 | 1.3% |
| chunk system (kernel) | 308 | 0.5% |
| JDK collections | 236 | 0.4% |
| fastutil collections | 221 | 0.3% |
| moonrise/paper patches | 218 | 0.3% |
| network (kernel) | 117 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK other | 78 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61516 | 94.8% |
| phase: entity tick (AI/movement) | 2828 | 4.4% |
| phase: main tick (unclassified) | 235 | 0.4% |
| phase: chunk tick | 112 | 0.2% |
| phase: network sync (ServerEntity) | 78 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56097** (86.5%) · native/JVM-internal **8766** (13.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52898 | 81.6% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.4% |
| `read` | native/JVM-internal | 1223 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 142 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 97 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `vtable stub` | native/JVM-internal | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 76 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 62 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f7f8ba68238.accept` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 61 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4517)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4517 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2513 | 55.6% |
| phase: unclassified | 1692 | 37.5% |
| phase: main tick (unclassified) | 170 | 3.8% |
| phase: chunk system (off-main worker) | 60 | 1.3% |
| phase: block entities (hoppers/furnaces) | 41 | 0.9% |
| phase: network sync (ServerEntity) | 31 | 0.7% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4517** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 668 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 667 | 14.8% |
| `char[]_[k]` | other | 441 | 9.8% |
| `byte[]_[k]` | other | 312 | 6.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 193 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 179 | 4.0% |
| `long[]_[i]` | other | 176 | 3.9% |
| `java.util.ArrayList_[i]` | other | 163 | 3.6% |
| `java.lang.Object[]_[i]` | other | 129 | 2.9% |
| `byte[]_[i]` | other | 93 | 2.1% |
| `int[]_[i]` | other | 77 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 64 | 1.4% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f7f8baa26b0_[i]` | other | 48 | 1.1% |
| `int[]_[k]` | other | 47 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 44 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f7f8ba5efb0_[i]` | other | 41 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110320 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35849 | 32.50% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 18080 | 16.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6073 | 5.50% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5375 | 4.87% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4144 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 984 | 0.89% |
| `net/minecraft/world/entity/ai/Brain.tick` | 936 | 0.85% |
| `net/minecraft/world/entity/npc/Villager.tick` | 419 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 294 | 0.27% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 263 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 213 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 668 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 667 | 14.8% |
| `char[]_[k]` | 441 | 9.8% |
| `byte[]_[k]` | 312 | 6.9% |
| `net.minecraft.core.BlockPos_[i]` | 193 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 179 | 4.0% |
| `long[]_[i]` | 176 | 3.9% |
| `java.util.ArrayList_[i]` | 163 | 3.6% |
| `java.lang.Object[]_[i]` | 129 | 2.9% |
| `byte[]_[i]` | 93 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 133 pauses / total 20043 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148175..157182 (delta 9007, churn 5.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99732->110989, minecraft:drowned 3334->4517, minecraft:zombie 3592->4631, minecraft:husk 4638->5667, minecraft:skeleton 4169->4873, minecraft:pig 2655->3184, minecraft:sheep 3034->3542, minecraft:creeper 4559->5041
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9007)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48345092 B)
- `wall-collapsed.txt` (3347935 B)
- `alloc-collapsed.txt` (2135307 B)
- `cpu-flamegraph.html` (276667 B)
- `server-stdout.log` (328062 B)
- `gc.log` (125608 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
