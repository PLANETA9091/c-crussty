# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.186 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [25.9, 2.0, 1.5, 2.7, 3.1, 3.2]
- spark tick-monitor MSPT: avg **325.62ms** / min 289.21ms / max **395.78ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T17:26:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8841704 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 289.21 | — | — | — | 395.78 | 325.62 |

- entity totals seen: [149247, 150866, 151125]
- top entity types (max seen): minecraft:item×103216, minecraft:creeper×5249, minecraft:husk×5199, minecraft:skeleton×4861, minecraft:spider×4806, minecraft:zombie×4622, minecraft:drowned×4505, minecraft:sheep×3540, minecraft:chicken×3451, minecraft:cow×3327, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/asaciQGiMx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **143** (Full GC: **9**)
- total pause: **20194.5 ms**, avg **141.22 ms**, max **2029.8 ms**
- heap high-water seen: **7753 MB** -> last-after: **4500 MB**
  - Young (Allocation Failure): 124
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 112889)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27944 | 24.8% |
| kernel: other | 26447 | 23.4% |
| other | 12434 | 11.0% |
| chunk system (kernel) | 10567 | 9.4% |
| moonrise/paper patches | 10213 | 9.0% |
| fastutil collections | 6901 | 6.1% |
| JDK collections | 6358 | 5.6% |
| network (kernel) | 3616 | 3.2% |
| JIT stubs (vtable/itable) | 2873 | 2.5% |
| JDK invokes/VarHandle | 2402 | 2.1% |
| JDK other | 2124 | 1.9% |
| JVM internals (GC oop barriers) | 485 | 0.4% |
| vdso (clock) | 233 | 0.2% |
| block entities/hoppers (kernel) | 109 | 0.1% |
| bukkit api | 51 | 0.0% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90277 | 80.0% |
| phase: unclassified | 11743 | 10.4% |
| phase: main tick (unclassified) | 3886 | 3.4% |
| phase: network sync (ServerEntity) | 2272 | 2.0% |
| phase: chunk tick | 2210 | 2.0% |
| phase: chunk system (off-main worker) | 1200 | 1.1% |
| phase: block entities (hoppers/furnaces) | 708 | 0.6% |
| phase: random tick | 449 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100036** (88.6%) · native/JVM-internal **12733** (11.3%) · other **120** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5138 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3627 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2911 | 2.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2574 | 2.3% |
| `vtable stub` | native/JVM-internal | 2417 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2069 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1973 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1894 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1730 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1584 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1506 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1465 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1440 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1292 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1267 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1202 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1161 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1124 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1110 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1093 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1089 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1064 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1047 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1044 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1040 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 984 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 953 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 909 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 776 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 716 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 704 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 688 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 687 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 674 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 58000 | 94.7% |
| entities/mobs (kernel) | 941 | 1.5% |
| kernel: other | 867 | 1.4% |
| moonrise/paper patches | 340 | 0.6% |
| chunk system (kernel) | 297 | 0.5% |
| fastutil collections | 228 | 0.4% |
| JDK collections | 212 | 0.3% |
| network (kernel) | 130 | 0.2% |
| JIT stubs (vtable/itable) | 93 | 0.2% |
| JDK other | 70 | 0.1% |
| JDK invokes/VarHandle | 65 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57824 | 94.4% |
| phase: entity tick (AI/movement) | 2971 | 4.8% |
| phase: main tick (unclassified) | 195 | 0.3% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52412** (85.6%) · native/JVM-internal **8841** (14.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49157 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.8% |
| `read` | native/JVM-internal | 1221 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 104 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 85 | 0.1% |
| `vtable stub` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 84 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4530)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4530 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2688 | 59.3% |
| phase: unclassified | 1664 | 36.7% |
| phase: main tick (unclassified) | 98 | 2.2% |
| phase: network sync (ServerEntity) | 26 | 0.6% |
| phase: chunk system (off-main worker) | 25 | 0.6% |
| phase: chunk tick | 15 | 0.3% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4530** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 689 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 589 | 13.0% |
| `char[]_[k]` | other | 454 | 10.0% |
| `byte[]_[k]` | other | 337 | 7.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 189 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 181 | 4.0% |
| `java.util.ArrayList_[i]` | other | 162 | 3.6% |
| `long[]_[i]` | other | 129 | 2.8% |
| `java.lang.Object[]_[i]` | other | 127 | 2.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 122 | 2.7% |
| `byte[]_[i]` | other | 92 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 90 | 2.0% |
| `int[]_[i]` | other | 77 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 71 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 52 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fcbb9a4bc18_[i]` | other | 42 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 39 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112889 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33276 | 29.48% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21946 | 19.44% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6191 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5361 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4676 | 4.14% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 939 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 924 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 418 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 226 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 689 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 589 | 13.0% |
| `char[]_[k]` | 454 | 10.0% |
| `byte[]_[k]` | 337 | 7.4% |
| `net.minecraft.core.BlockPos_[i]` | 189 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 181 | 4.0% |
| `java.util.ArrayList_[i]` | 162 | 3.6% |
| `long[]_[i]` | 129 | 2.8% |
| `java.lang.Object[]_[i]` | 127 | 2.8% |
| `net.minecraft.core.BlockPos$6_[i]` | 122 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 143 pauses / total 20195 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147995..151125 (delta 3130, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99473->103216, minecraft:drowned 3507->4505, minecraft:zombie 3692->4622, minecraft:creeper 4595->5249, minecraft:husk 4571->5199, minecraft:spider 4220->4806, minecraft:skeleton 4382->4861, minecraft:chicken 3415->3451
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3130)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49728091 B)
- `wall-collapsed.txt` (3228832 B)
- `alloc-collapsed.txt` (2084917 B)
- `cpu-flamegraph.html` (268840 B)
- `server-stdout.log` (239392 B)
- `gc.log` (133378 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
