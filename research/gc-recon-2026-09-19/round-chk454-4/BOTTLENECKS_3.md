# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.519 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 1.9, 2.2, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **364.62ms** / min 303.21ms / max **474.52ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:16:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6904589 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [05:18:57 INFO]: [crussty-plugin] [cruss | 303.21 | — | — | — | 474.52 | 364.62 |

- entity totals seen: [150773, 152771, 153448]
- top entity types (max seen): minecraft:item×106663, minecraft:husk×5406, minecraft:creeper×5052, minecraft:skeleton×4805, minecraft:zombie×4565, minecraft:drowned×4540, minecraft:spider×4527, minecraft:sheep×3516, minecraft:chicken×3404, minecraft:cow×3367, minecraft:pig×3185, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XCQC2OcRuK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19720.7 ms**, avg **171.48 ms**, max **2720.3 ms**
- heap high-water seen: **7598 MB** -> last-after: **4134 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105397)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34847 | 33.1% |
| kernel: other | 22409 | 21.3% |
| other | 11420 | 10.8% |
| chunk system (kernel) | 8004 | 7.6% |
| JDK collections | 7059 | 6.7% |
| moonrise/paper patches | 5168 | 4.9% |
| fastutil collections | 4840 | 4.6% |
| JIT stubs (vtable/itable) | 3550 | 3.4% |
| network (kernel) | 2939 | 2.8% |
| JDK invokes/VarHandle | 2527 | 2.4% |
| JDK other | 2178 | 2.1% |
| vdso (clock) | 116 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51328 | 48.7% |
| phase: unclassified | 33286 | 31.6% |
| phase: main tick (unclassified) | 13196 | 12.5% |
| phase: chunk tick | 2535 | 2.4% |
| phase: network sync (ServerEntity) | 2173 | 2.1% |
| phase: chunk system (off-main worker) | 1081 | 1.0% |
| phase: block entities (hoppers/furnaces) | 1016 | 1.0% |
| phase: random tick | 478 | 0.5% |
| phase: mob spawning | 298 | 0.3% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93294** (88.5%) · native/JVM-internal **11973** (11.4%) · other **130** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3914 | 3.7% |
| `vtable stub` | native/JVM-internal | 2936 | 2.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2902 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2869 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1954 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1716 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1360 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1299 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1289 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1274 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1254 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1238 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1170 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1131 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1047 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1011 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 986 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 963 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 951 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 915 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 904 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 895 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 843 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 824 | 0.8% |
| `colpush_tick` | native/JVM-internal | 822 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 809 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 808 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 794 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 788 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 726 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 711 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 706 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 697 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 688 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 677 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 671 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 669 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 625 | 0.6% |
| `itable stub` | native/JVM-internal | 611 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66966)

| bucket | self-time samples | share |
|---|---|---|
| other | 63943 | 95.5% |
| entities/mobs (kernel) | 1050 | 1.6% |
| kernel: other | 691 | 1.0% |
| chunk system (kernel) | 218 | 0.3% |
| JDK collections | 212 | 0.3% |
| JIT stubs (vtable/itable) | 209 | 0.3% |
| moonrise/paper patches | 169 | 0.3% |
| fastutil collections | 138 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK invokes/VarHandle | 89 | 0.1% |
| JDK other | 59 | 0.1% |
| JVM internals (GC oop barriers) | 59 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64081 | 95.7% |
| phase: entity tick (AI/movement) | 2093 | 3.1% |
| phase: main tick (unclassified) | 502 | 0.7% |
| phase: chunk tick | 97 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 49 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 12 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **57106** (85.3%) · native/JVM-internal **9855** (14.7%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 54277 | 81.1% |
| `clock_nanosleep` | native/JVM-internal | 4924 | 7.4% |
| `epoll_wait` | native/JVM-internal | 1241 | 1.9% |
| `read` | native/JVM-internal | 1230 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `syscall` | native/JVM-internal | 622 | 0.9% |
| `vtable stub` | native/JVM-internal | 188 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 107 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 69 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 40 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3554)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3554 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2115 | 59.5% |
| phase: entity tick (AI/movement) | 1075 | 30.2% |
| phase: main tick (unclassified) | 258 | 7.3% |
| phase: chunk system (off-main worker) | 38 | 1.1% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: mob spawning | 17 | 0.5% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3554** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 547 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 459 | 12.9% |
| `char[]_[k]` | other | 368 | 10.4% |
| `byte[]_[k]` | other | 205 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 145 | 4.1% |
| `long[]_[i]` | other | 140 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.7% |
| `int[]_[i]` | other | 108 | 3.0% |
| `java.lang.Object[]_[i]` | other | 96 | 2.7% |
| `java.util.ArrayList_[i]` | other | 89 | 2.5% |
| `byte[]_[i]` | other | 86 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.5% |
| `net.minecraft.core.SectionPos_[i]` | other | 41 | 1.2% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb415a01d10_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.8% |
| `java.math.BigInteger_[i]` | other | 29 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 27 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb415830000_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105397 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19408 | 18.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6274 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5263 | 4.99% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3905 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1332 | 1.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 619 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 420 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 398 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 351 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 306 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 261 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 88 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 547 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 459 | 12.9% |
| `char[]_[k]` | 368 | 10.4% |
| `byte[]_[k]` | 205 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 145 | 4.1% |
| `long[]_[i]` | 140 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.7% |
| `int[]_[i]` | 108 | 3.0% |
| `java.lang.Object[]_[i]` | 96 | 2.7% |
| `java.util.ArrayList_[i]` | 89 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19721 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148236..153448 (delta 5212, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99926->106663, minecraft:drowned 3583->4540, minecraft:zombie 3676->4565, minecraft:husk 4585->5406, minecraft:skeleton 4182->4805, minecraft:creeper 4607->5052, minecraft:spider 4128->4527, minecraft:chicken 3078->3404
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5212)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49608348 B)
- `wall-collapsed.txt` (3215148 B)
- `alloc-collapsed.txt` (1846011 B)
- `cpu-flamegraph.html` (254563 B)
- `server-stdout.log` (328651 B)
- `gc.log` (109218 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
