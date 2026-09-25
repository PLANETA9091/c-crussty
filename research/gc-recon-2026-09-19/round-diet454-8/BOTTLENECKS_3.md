# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.814 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.6, 1.9, 2.0, 2.3, 2.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:39:16Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7342464 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [05:41:37 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [149595, 151402, 153330]
- top entity types (max seen): minecraft:item×106594, minecraft:husk×5388, minecraft:creeper×4977, minecraft:skeleton×4773, minecraft:zombie×4647, minecraft:drowned×4571, minecraft:spider×4482, minecraft:sheep×3520, minecraft:chicken×3385, minecraft:cow×3360, minecraft:pig×3211, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/LLrJxfkoKr
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **106** (Full GC: **9**)
- total pause: **25346.8 ms**, avg **239.12 ms**, max **3533.7 ms**
- heap high-water seen: **7588 MB** -> last-after: **4032 MB**
  - Young (Allocation Failure): 87
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103934)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34940 | 33.6% |
| kernel: other | 20848 | 20.1% |
| other | 10615 | 10.2% |
| chunk system (kernel) | 9842 | 9.5% |
| JDK collections | 7582 | 7.3% |
| moonrise/paper patches | 5937 | 5.7% |
| fastutil collections | 4320 | 4.2% |
| network (kernel) | 2761 | 2.7% |
| JIT stubs (vtable/itable) | 2677 | 2.6% |
| JDK invokes/VarHandle | 2574 | 2.5% |
| JDK other | 1421 | 1.4% |
| block entities/hoppers (kernel) | 97 | 0.1% |
| vdso (clock) | 91 | 0.1% |
| bukkit api | 76 | 0.1% |
| craftbukkit glue | 70 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49394 | 47.5% |
| phase: unclassified | 33358 | 32.1% |
| phase: main tick (unclassified) | 12745 | 12.3% |
| phase: chunk tick | 2948 | 2.8% |
| phase: network sync (ServerEntity) | 2661 | 2.6% |
| phase: chunk system (off-main worker) | 1076 | 1.0% |
| phase: block entities (hoppers/furnaces) | 895 | 0.9% |
| phase: random tick | 514 | 0.5% |
| phase: mob spawning | 338 | 0.3% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92948** (89.4%) · native/JVM-internal **10856** (10.4%) · other **130** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4893 | 4.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3548 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3001 | 2.9% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2300 | 2.2% |
| `vtable stub` | native/JVM-internal | 2276 | 2.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1593 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1569 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1542 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1510 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1457 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1370 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1269 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1245 | 1.2% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1216 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1131 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1099 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 989 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 954 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 926 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 923 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 909 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 903 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 896 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 870 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 829 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 814 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 810 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 780 | 0.8% |
| `colpush_tick` | native/JVM-internal | 778 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 747 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 725 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getHardCollidingEntities` | JVM-Java | 715 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 697 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 674 | 0.6% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 634 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 629 | 0.6% |
| `ca/spottedleaf/moonrise/common/util/TickThread.isTickThread` | JVM-Java | 609 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64857)

| bucket | self-time samples | share |
|---|---|---|
| other | 61888 | 95.4% |
| entities/mobs (kernel) | 1071 | 1.7% |
| kernel: other | 677 | 1.0% |
| chunk system (kernel) | 300 | 0.5% |
| JDK collections | 255 | 0.4% |
| moonrise/paper patches | 171 | 0.3% |
| JIT stubs (vtable/itable) | 135 | 0.2% |
| fastutil collections | 134 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| network (kernel) | 74 | 0.1% |
| JDK other | 52 | 0.1% |
| craftbukkit glue | 8 | 0.0% |
| vdso (clock) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62325 | 96.1% |
| phase: entity tick (AI/movement) | 1741 | 2.7% |
| phase: main tick (unclassified) | 470 | 0.7% |
| phase: chunk tick | 117 | 0.2% |
| phase: network sync (ServerEntity) | 90 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55970** (86.3%) · native/JVM-internal **8879** (13.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53069 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4790 | 7.4% |
| `read` | native/JVM-internal | 1211 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `vtable stub` | native/JVM-internal | 127 | 0.2% |
| `syscall` | native/JVM-internal | 94 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 86 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 41 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3085)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3085 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1927 | 62.5% |
| phase: entity tick (AI/movement) | 849 | 27.5% |
| phase: main tick (unclassified) | 225 | 7.3% |
| phase: chunk system (off-main worker) | 30 | 1.0% |
| phase: block entities (hoppers/furnaces) | 29 | 0.9% |
| phase: network sync (ServerEntity) | 15 | 0.5% |
| phase: mob spawning | 5 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3085** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 405 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 362 | 11.7% |
| `char[]_[k]` | other | 299 | 9.7% |
| `byte[]_[k]` | other | 236 | 7.6% |
| `long[]_[i]` | other | 136 | 4.4% |
| `byte[]_[i]` | other | 100 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 99 | 3.2% |
| `java.util.ArrayList_[i]` | other | 99 | 3.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 97 | 3.1% |
| `java.lang.Object[]_[i]` | other | 97 | 3.1% |
| `int[]_[i]` | other | 96 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 63 | 2.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f105f8314c0_[i]` | other | 32 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f105fa03590_[i]` | other | 30 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 26 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 25 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 25 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103934 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19307 | 18.58% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5924 | 5.70% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4943 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3806 | 3.66% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 965 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 562 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 438 | 0.42% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 414 | 0.40% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 347 | 0.33% |
| `net/minecraft/world/entity/npc/Villager.tick` | 311 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 266 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 110 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 405 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 362 | 11.7% |
| `char[]_[k]` | 299 | 9.7% |
| `byte[]_[k]` | 236 | 7.6% |
| `long[]_[i]` | 136 | 4.4% |
| `byte[]_[i]` | 100 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 99 | 3.2% |
| `java.util.ArrayList_[i]` | 99 | 3.2% |
| `net.minecraft.core.BlockPos_[i]` | 97 | 3.1% |
| `java.lang.Object[]_[i]` | 97 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 106 pauses / total 25347 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148246..153330 (delta 5084, churn 3.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99831->106594, minecraft:drowned 3516->4571, minecraft:zombie 3635->4647, minecraft:husk 4547->5388, minecraft:skeleton 4101->4773, minecraft:creeper 4508->4977, minecraft:spider 4090->4482, minecraft:pig 2880->3211
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5084)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50655985 B)
- `wall-collapsed.txt` (3136190 B)
- `alloc-collapsed.txt` (1707693 B)
- `cpu-flamegraph.html` (280332 B)
- `server-stdout.log` (324713 B)
- `gc.log` (101517 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
