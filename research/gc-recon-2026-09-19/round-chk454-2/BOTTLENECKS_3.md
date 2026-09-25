# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.476 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.9, 2.2, 2.4, 2.7, 2.8]
- spark tick-monitor MSPT: avg **380.83ms** / min 305.2ms / max **519.27ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T04:35:31Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6894867 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:37:46 INFO]: [crussty-plugin] [cruss | 305.2 | — | — | — | 519.27 | 380.83 |

- entity totals seen: [150433, 152583, 153797]
- top entity types (max seen): minecraft:item×106836, minecraft:husk×5544, minecraft:creeper×5020, minecraft:skeleton×4764, minecraft:zombie×4642, minecraft:drowned×4573, minecraft:spider×4524, minecraft:sheep×3507, minecraft:chicken×3407, minecraft:cow×3364, minecraft:pig×3216, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KjwNXAwmRs
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **20069.8 ms**, avg **179.19 ms**, max **2956.9 ms**
- heap high-water seen: **7449 MB** -> last-after: **4155 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105246)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35043 | 33.3% |
| kernel: other | 22562 | 21.4% |
| other | 11680 | 11.1% |
| chunk system (kernel) | 7818 | 7.4% |
| JDK collections | 7017 | 6.7% |
| moonrise/paper patches | 5113 | 4.9% |
| fastutil collections | 4896 | 4.7% |
| JIT stubs (vtable/itable) | 3376 | 3.2% |
| network (kernel) | 2768 | 2.6% |
| JDK invokes/VarHandle | 2293 | 2.2% |
| JDK other | 2237 | 2.1% |
| vdso (clock) | 117 | 0.1% |
| bukkit api | 95 | 0.1% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51052 | 48.5% |
| phase: unclassified | 33389 | 31.7% |
| phase: main tick (unclassified) | 13326 | 12.7% |
| phase: chunk tick | 2442 | 2.3% |
| phase: network sync (ServerEntity) | 2159 | 2.1% |
| phase: chunk system (off-main worker) | 1099 | 1.0% |
| phase: block entities (hoppers/furnaces) | 1007 | 1.0% |
| phase: random tick | 484 | 0.5% |
| phase: mob spawning | 288 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93170** (88.5%) · native/JVM-internal **11977** (11.4%) · other **99** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3871 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3074 | 2.9% |
| `vtable stub` | native/JVM-internal | 2898 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2536 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 2023 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1538 | 1.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1332 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1323 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1286 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1275 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1254 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1239 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1200 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1073 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1041 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1036 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1022 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 976 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 964 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 952 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 886 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 881 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 865 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 834 | 0.8% |
| `colpush_tick` | native/JVM-internal | 811 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 763 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 738 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 710 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 688 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 679 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 656 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 655 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 637 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64860)

| bucket | self-time samples | share |
|---|---|---|
| other | 61913 | 95.5% |
| entities/mobs (kernel) | 1035 | 1.6% |
| kernel: other | 729 | 1.1% |
| chunk system (kernel) | 222 | 0.3% |
| JDK collections | 212 | 0.3% |
| JIT stubs (vtable/itable) | 169 | 0.3% |
| moonrise/paper patches | 159 | 0.2% |
| fastutil collections | 123 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK other | 74 | 0.1% |
| JVM internals (GC oop barriers) | 63 | 0.1% |
| JDK invokes/VarHandle | 59 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62256 | 96.0% |
| phase: entity tick (AI/movement) | 1830 | 2.8% |
| phase: main tick (unclassified) | 499 | 0.8% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: block entities (hoppers/furnaces) | 54 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55429** (85.5%) · native/JVM-internal **9425** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52630 | 81.1% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1217 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 450 | 0.7% |
| `vtable stub` | native/JVM-internal | 149 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 102 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 79 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 36 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 35 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3536)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3536 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2060 | 58.3% |
| phase: entity tick (AI/movement) | 1109 | 31.4% |
| phase: main tick (unclassified) | 253 | 7.2% |
| phase: chunk system (off-main worker) | 43 | 1.2% |
| phase: network sync (ServerEntity) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 17 | 0.5% |
| phase: mob spawning | 14 | 0.4% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3536** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 511 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 478 | 13.5% |
| `char[]_[k]` | other | 412 | 11.7% |
| `byte[]_[k]` | other | 235 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 134 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.7% |
| `java.lang.Object[]_[i]` | other | 131 | 3.7% |
| `long[]_[i]` | other | 128 | 3.6% |
| `java.util.ArrayList_[i]` | other | 109 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 86 | 2.4% |
| `byte[]_[i]` | other | 84 | 2.4% |
| `int[]_[i]` | other | 70 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 29 | 0.8% |
| `java.lang.String_[i]` | other | 28 | 0.8% |
| `int[]_[k]` | other | 28 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105246 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19697 | 18.72% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6162 | 5.85% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5137 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3818 | 3.63% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1393 | 1.32% |
| `net/minecraft/world/entity/ai/Brain.tick` | 607 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 398 | 0.38% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 363 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 330 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 277 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 248 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 119 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 511 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 478 | 13.5% |
| `char[]_[k]` | 412 | 11.7% |
| `byte[]_[k]` | 235 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | 134 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.7% |
| `java.lang.Object[]_[i]` | 131 | 3.7% |
| `long[]_[i]` | 128 | 3.6% |
| `java.util.ArrayList_[i]` | 109 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | 86 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 20070 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148146..153797 (delta 5651, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99763->106836, minecraft:husk 4555->5544, minecraft:drowned 3622->4573, minecraft:zombie 3779->4642, minecraft:skeleton 4203->4764, minecraft:creeper 4505->5020, minecraft:spider 4088->4524, minecraft:chicken 3086->3407
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5651)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (60026612 B)
- `wall-collapsed.txt` (3156907 B)
- `alloc-collapsed.txt` (1855161 B)
- `cpu-flamegraph.html` (287633 B)
- `server-stdout.log` (332394 B)
- `gc.log` (106651 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
