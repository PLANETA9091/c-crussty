# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.578 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.3, 1.9, 2.1, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **360.26ms** / min 302.23ms / max **476.16ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:46:14Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7247637 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [08:48:27 INFO]: [crussty-plugin] [cruss | 302.23 | — | — | — | 476.16 | 360.26 |

- entity totals seen: [150981, 153037, 153892]
- top entity types (max seen): minecraft:item×107165, minecraft:husk×5509, minecraft:creeper×5026, minecraft:skeleton×4751, minecraft:zombie×4625, minecraft:drowned×4554, minecraft:spider×4427, minecraft:sheep×3521, minecraft:chicken×3385, minecraft:cow×3352, minecraft:pig×3206, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/jDNplyC5EC
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20373.0 ms**, avg **178.71 ms**, max **2971.4 ms**
- heap high-water seen: **7530 MB** -> last-after: **4181 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105721)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35253 | 33.3% |
| kernel: other | 21968 | 20.8% |
| other | 11909 | 11.3% |
| chunk system (kernel) | 7959 | 7.5% |
| JDK collections | 7176 | 6.8% |
| fastutil collections | 5221 | 4.9% |
| moonrise/paper patches | 4985 | 4.7% |
| JIT stubs (vtable/itable) | 3411 | 3.2% |
| network (kernel) | 2843 | 2.7% |
| JDK invokes/VarHandle | 2476 | 2.3% |
| JDK other | 2047 | 1.9% |
| vdso (clock) | 132 | 0.1% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| bukkit api | 93 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51184 | 48.4% |
| phase: unclassified | 33929 | 32.1% |
| phase: main tick (unclassified) | 13154 | 12.4% |
| phase: chunk tick | 2520 | 2.4% |
| phase: network sync (ServerEntity) | 2207 | 2.1% |
| phase: chunk system (off-main worker) | 1078 | 1.0% |
| phase: block entities (hoppers/furnaces) | 879 | 0.8% |
| phase: random tick | 479 | 0.5% |
| phase: mob spawning | 289 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93311** (88.3%) · native/JVM-internal **12310** (11.6%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3814 | 3.6% |
| `vtable stub` | native/JVM-internal | 2937 | 2.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2837 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2613 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2007 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1544 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1509 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1360 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1338 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1303 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1279 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1198 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1106 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1064 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1052 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1015 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 980 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 964 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 955 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 939 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 932 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 908 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 855 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 845 | 0.8% |
| `colpush_tick` | native/JVM-internal | 820 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 776 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 766 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 744 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 735 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 690 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 677 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 613 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 613 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64861)

| bucket | self-time samples | share |
|---|---|---|
| other | 61986 | 95.6% |
| entities/mobs (kernel) | 1014 | 1.6% |
| kernel: other | 668 | 1.0% |
| JDK collections | 226 | 0.3% |
| chunk system (kernel) | 207 | 0.3% |
| JIT stubs (vtable/itable) | 161 | 0.2% |
| moonrise/paper patches | 135 | 0.2% |
| fastutil collections | 130 | 0.2% |
| network (kernel) | 113 | 0.2% |
| JDK other | 73 | 0.1% |
| JVM internals (GC oop barriers) | 70 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62198 | 95.9% |
| phase: entity tick (AI/movement) | 1990 | 3.1% |
| phase: main tick (unclassified) | 403 | 0.6% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55294** (85.2%) · native/JVM-internal **9563** (14.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52559 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.3% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 542 | 0.8% |
| `vtable stub` | native/JVM-internal | 136 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 92 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 52 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 33 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 32 | 0.0% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 32 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6267)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6267 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 4753 | 75.8% |
| phase: entity tick (AI/movement) | 1094 | 17.5% |
| phase: main tick (unclassified) | 275 | 4.4% |
| phase: chunk system (off-main worker) | 69 | 1.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.4% |
| phase: network sync (ServerEntity) | 22 | 0.4% |
| phase: mob spawning | 14 | 0.2% |
| phase: random tick | 8 | 0.1% |
| phase: chunk tick | 7 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6267** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 590 | 9.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 526 | 8.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 475 | 7.6% |
| `char[]_[k]` | other | 422 | 6.7% |
| `byte[]_[k]` | other | 373 | 6.0% |
| `java.lang.Object[]_[i]` | other | 284 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 279 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 265 | 4.2% |
| `byte[]_[i]` | other | 215 | 3.4% |
| `long[]_[k]` | other | 213 | 3.4% |
| `short[]_[i]` | other | 180 | 2.9% |
| `long[]_[i]` | other | 137 | 2.2% |
| `java.lang.String_[i]` | other | 132 | 2.1% |
| `java.util.ArrayList_[i]` | other | 128 | 2.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 121 | 1.9% |
| `int[]_[i]` | other | 105 | 1.7% |
| `java.lang.Object[]_[k]` | other | 93 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 1.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 71 | 1.1% |
| `java.util.Optional_[i]` | other | 69 | 1.1% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105721 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19321 | 18.28% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6276 | 5.94% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5249 | 4.96% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3898 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1335 | 1.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 590 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 416 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 387 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 348 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 307 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 250 | 0.24% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 96 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 590 | 9.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 526 | 8.4% |
| `net.minecraft.world.phys.AABB_[i]` | 475 | 7.6% |
| `char[]_[k]` | 422 | 6.7% |
| `byte[]_[k]` | 373 | 6.0% |
| `java.lang.Object[]_[i]` | 284 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 279 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 265 | 4.2% |
| `byte[]_[i]` | 215 | 3.4% |
| `long[]_[k]` | 213 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20373 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148359..153892 (delta 5533, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100071->107165, minecraft:husk 4570->5509, minecraft:zombie 3719->4625, minecraft:drowned 3670->4554, minecraft:skeleton 4127->4751, minecraft:creeper 4541->5026, minecraft:cow 3014->3352, minecraft:chicken 3057->3385
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5533)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50396623 B)
- `wall-collapsed.txt` (3163076 B)
- `alloc-collapsed.txt` (3437647 B)
- `cpu-flamegraph.html` (279540 B)
- `server-stdout.log` (327301 B)
- `gc.log` (108367 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
