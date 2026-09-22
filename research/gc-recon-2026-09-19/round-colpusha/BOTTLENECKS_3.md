# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.694 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.5, 1.2, 1.3, 1.6, 1.7, 1.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T20:40:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6309799 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:42:47 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [148793, 150313, 151669]
- top entity types (max seen): minecraft:item×106959, minecraft:husk×5065, minecraft:creeper×4964, minecraft:skeleton×4863, minecraft:zombie×4720, minecraft:drowned×4595, minecraft:spider×4441, minecraft:sheep×3506, minecraft:chicken×3410, minecraft:cow×3389, minecraft:pig×3251, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/JhpxT0bfCu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **672** (Full GC: **9**)
- total pause: **48477.7 ms**, avg **72.14 ms**, max **2096.3 ms**
- heap high-water seen: **9427 MB** -> last-after: **4079 MB**
  - Young (Allocation Failure): 653
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 111676)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33986 | 30.4% |
| other | 22028 | 19.7% |
| kernel: other | 21844 | 19.6% |
| chunk system (kernel) | 6822 | 6.1% |
| moonrise/paper patches | 6043 | 5.4% |
| JDK collections | 5755 | 5.2% |
| fastutil collections | 5147 | 4.6% |
| JIT stubs (vtable/itable) | 2938 | 2.6% |
| network (kernel) | 2539 | 2.3% |
| JDK invokes/VarHandle | 2162 | 1.9% |
| JDK other | 1992 | 1.8% |
| vdso (clock) | 123 | 0.1% |
| bukkit api | 98 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| block entities/hoppers (kernel) | 61 | 0.1% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 56174 | 50.3% |
| phase: unclassified | 39515 | 35.4% |
| phase: main tick (unclassified) | 9942 | 8.9% |
| phase: chunk tick | 1962 | 1.8% |
| phase: network sync (ServerEntity) | 1655 | 1.5% |
| phase: chunk system (off-main worker) | 1053 | 0.9% |
| phase: block entities (hoppers/furnaces) | 846 | 0.8% |
| phase: random tick | 404 | 0.4% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89811** (80.4%) · native/JVM-internal **21678** (19.4%) · other **187** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.maybeEpoch` | JVM-Java | 7254 | 6.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 6628 | 5.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3201 | 2.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 3027 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2475 | 2.2% |
| `vtable stub` | native/JVM-internal | 2372 | 2.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2162 | 1.9% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 1629 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1586 | 1.4% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 1481 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1395 | 1.2% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1386 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1355 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1124 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1119 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1056 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1041 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1031 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 922 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 903 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 879 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 871 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 867 | 0.8% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 861 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 844 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 843 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 736 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 706 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 690 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 669 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 665 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 632 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 629 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 628 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 624 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 624 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61229)

| bucket | self-time samples | share |
|---|---|---|
| other | 58488 | 95.5% |
| entities/mobs (kernel) | 1141 | 1.9% |
| kernel: other | 609 | 1.0% |
| chunk system (kernel) | 198 | 0.3% |
| moonrise/paper patches | 181 | 0.3% |
| JDK collections | 181 | 0.3% |
| fastutil collections | 156 | 0.3% |
| network (kernel) | 78 | 0.1% |
| JIT stubs (vtable/itable) | 75 | 0.1% |
| JDK invokes/VarHandle | 58 | 0.1% |
| JDK other | 53 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58090 | 94.9% |
| phase: entity tick (AI/movement) | 2634 | 4.3% |
| phase: main tick (unclassified) | 300 | 0.5% |
| phase: chunk tick | 76 | 0.1% |
| phase: network sync (ServerEntity) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51876** (84.7%) · native/JVM-internal **9339** (15.3%) · other **14** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49143 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1221 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.maybeEpoch` | JVM-Java | 321 | 0.5% |
| `syscall` | native/JVM-internal | 241 | 0.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 203 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 105 | 0.2% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `vtable stub` | native/JVM-internal | 64 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 64 | 0.1% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 44 | 0.1% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 39 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 35 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 31353)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 31353 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 25895 | 82.6% |
| phase: unclassified | 4926 | 15.7% |
| phase: main tick (unclassified) | 450 | 1.4% |
| phase: network sync (ServerEntity) | 29 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.1% |
| phase: chunk system (off-main worker) | 15 | 0.0% |
| phase: mob spawning | 5 | 0.0% |
| phase: random tick | 3 | 0.0% |
| phase: chunk tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **31353** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `double[]_[i]` | other | 9447 | 30.1% |
| `int[]_[i]` | other | 7998 | 25.5% |
| `double[]_[k]` | other | 7529 | 24.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 774 | 2.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 675 | 2.2% |
| `byte[]_[k]` | other | 645 | 2.1% |
| `byte[]_[i]` | other | 534 | 1.7% |
| `char[]_[k]` | other | 450 | 1.4% |
| `java.util.Calendar$Builder_[i]` | other | 248 | 0.8% |
| `java.util.ArrayList_[i]` | other | 206 | 0.7% |
| `int[]_[k]` | other | 196 | 0.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 195 | 0.6% |
| `long[]_[i]` | other | 194 | 0.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 0.5% |
| `java.util.regex.Matcher_[i]` | other | 132 | 0.4% |
| `java.lang.Object[]_[i]` | other | 120 | 0.4% |
| `com.destroystokyo.paper.ServerSchedulerReportingWrapper_[i]` | other | 100 | 0.3% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 98 | 0.3% |
| `java.util.GregorianCalendar_[i]` | other | 84 | 0.3% |
| `boolean[]_[i]` | other | 67 | 0.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111676 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19856 | 17.78% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7676 | 6.87% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 6302 | 5.64% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5781 | 5.18% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 974 | 0.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 888 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 409 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 263 | 0.24% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 188 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 170 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 155 | 0.14% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 114 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `double[]_[i]` | 9447 | 30.1% |
| `int[]_[i]` | 7998 | 25.5% |
| `double[]_[k]` | 7529 | 24.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 774 | 2.5% |
| `net.minecraft.world.phys.AABB_[i]` | 675 | 2.2% |
| `byte[]_[k]` | 645 | 2.1% |
| `byte[]_[i]` | 534 | 1.7% |
| `char[]_[k]` | 450 | 1.4% |
| `java.util.Calendar$Builder_[i]` | 248 | 0.8% |
| `java.util.ArrayList_[i]` | 206 | 0.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 672 pauses / total 48478 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148380..151669 (delta 3289, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99508->106959, minecraft:drowned 3383->4595, minecraft:zombie 3606->4720, minecraft:skeleton 4202->4863, minecraft:pig 2690->3251, minecraft:husk 4600->5065, minecraft:sheep 3096->3506, minecraft:spider 4051->4441
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3289)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43156514 B)
- `wall-collapsed.txt` (3165008 B)
- `alloc-collapsed.txt` (2625576 B)
- `cpu-flamegraph.html` (293396 B)
- `server-stdout.log` (1063044 B)
- `gc.log` (589499 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
