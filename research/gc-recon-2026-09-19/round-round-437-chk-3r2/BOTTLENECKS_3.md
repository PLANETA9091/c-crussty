# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.457 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 1.6, 1.9, 2.0, 2.3, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T00:37:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6759060 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [00:39:53 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [149533, 151551, 153474]
- top entity types (max seen): minecraft:item×106785, minecraft:husk×5389, minecraft:creeper×4974, minecraft:skeleton×4808, minecraft:zombie×4581, minecraft:drowned×4551, minecraft:spider×4428, minecraft:sheep×3520, minecraft:chicken×3406, minecraft:cow×3369, minecraft:pig×3188, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/NM7PELsDch
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **105** (Full GC: **9**)
- total pause: **21908.2 ms**, avg **208.65 ms**, max **3219.3 ms**
- heap high-water seen: **7428 MB** -> last-after: **3945 MB**
  - Young (Allocation Failure): 87
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103687)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33997 | 32.8% |
| kernel: other | 19838 | 19.1% |
| other | 11767 | 11.3% |
| chunk system (kernel) | 8320 | 8.0% |
| JDK collections | 7952 | 7.7% |
| moonrise/paper patches | 5948 | 5.7% |
| fastutil collections | 5276 | 5.1% |
| network (kernel) | 3301 | 3.2% |
| JDK invokes/VarHandle | 2731 | 2.6% |
| JIT stubs (vtable/itable) | 2427 | 2.3% |
| JDK other | 1637 | 1.6% |
| vdso (clock) | 132 | 0.1% |
| block entities/hoppers (kernel) | 119 | 0.1% |
| bukkit api | 81 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49047 | 47.3% |
| phase: unclassified | 33629 | 32.4% |
| phase: main tick (unclassified) | 12686 | 12.2% |
| phase: chunk tick | 2928 | 2.8% |
| phase: network sync (ServerEntity) | 2368 | 2.3% |
| phase: chunk system (off-main worker) | 1266 | 1.2% |
| phase: block entities (hoppers/furnaces) | 976 | 0.9% |
| phase: random tick | 480 | 0.5% |
| phase: mob spawning | 304 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91910** (88.6%) · native/JVM-internal **11687** (11.3%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4316 | 4.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3918 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3272 | 3.2% |
| `vtable stub` | native/JVM-internal | 2038 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1837 | 1.8% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1663 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1633 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1592 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1402 | 1.4% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1361 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1303 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1247 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1199 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1106 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1086 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1074 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1013 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1012 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 983 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 972 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 936 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 923 | 0.9% |
| `colpush_tick` | native/JVM-internal | 890 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 870 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 855 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 853 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 833 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 825 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 824 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 811 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 800 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 770 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 767 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 752 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 729 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 708 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 681 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 629 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64844)

| bucket | self-time samples | share |
|---|---|---|
| other | 61800 | 95.3% |
| entities/mobs (kernel) | 1052 | 1.6% |
| kernel: other | 671 | 1.0% |
| chunk system (kernel) | 278 | 0.4% |
| JDK collections | 265 | 0.4% |
| moonrise/paper patches | 211 | 0.3% |
| fastutil collections | 153 | 0.2% |
| JIT stubs (vtable/itable) | 147 | 0.2% |
| network (kernel) | 118 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 51 | 0.1% |
| bukkit api | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| vdso (clock) | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62232 | 96.0% |
| phase: entity tick (AI/movement) | 1866 | 2.9% |
| phase: main tick (unclassified) | 442 | 0.7% |
| phase: chunk tick | 119 | 0.2% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55906** (86.2%) · native/JVM-internal **8930** (13.8%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52953 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4747 | 7.3% |
| `read` | native/JVM-internal | 1230 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 130 | 0.2% |
| `syscall` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 36 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3217)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3217 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1895 | 58.9% |
| phase: entity tick (AI/movement) | 970 | 30.2% |
| phase: main tick (unclassified) | 259 | 8.1% |
| phase: chunk system (off-main worker) | 32 | 1.0% |
| phase: block entities (hoppers/furnaces) | 20 | 0.6% |
| phase: network sync (ServerEntity) | 18 | 0.6% |
| phase: chunk tick | 11 | 0.3% |
| phase: mob spawning | 11 | 0.3% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3217** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 525 | 16.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 390 | 12.1% |
| `char[]_[k]` | other | 338 | 10.5% |
| `byte[]_[k]` | other | 214 | 6.7% |
| `long[]_[i]` | other | 127 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 104 | 3.2% |
| `java.util.ArrayList_[i]` | other | 104 | 3.2% |
| `java.lang.Object[]_[i]` | other | 97 | 3.0% |
| `byte[]_[i]` | other | 96 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 92 | 2.9% |
| `int[]_[i]` | other | 88 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 55 | 1.7% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 38 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.9% |
| `int[]_[k]` | other | 30 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fbced9feb28_[i]` | other | 27 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 27 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103687 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19185 | 18.50% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6126 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5042 | 4.86% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3823 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1009 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 579 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 407 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 382 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 332 | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 280 | 0.27% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 278 | 0.27% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 105 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 525 | 16.3% |
| `net.minecraft.world.phys.AABB_[i]` | 390 | 12.1% |
| `char[]_[k]` | 338 | 10.5% |
| `byte[]_[k]` | 214 | 6.7% |
| `long[]_[i]` | 127 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 104 | 3.2% |
| `java.util.ArrayList_[i]` | 104 | 3.2% |
| `java.lang.Object[]_[i]` | 97 | 3.0% |
| `byte[]_[i]` | 96 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | 92 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 105 pauses / total 21908 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148188..153474 (delta 5286, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99750->106785, minecraft:drowned 3514->4551, minecraft:zombie 3628->4581, minecraft:husk 4563->5389, minecraft:skeleton 4156->4808, minecraft:creeper 4522->4974, minecraft:pig 2832->3188, minecraft:sheep 3170->3520
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5286)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47899400 B)
- `wall-collapsed.txt` (3163707 B)
- `alloc-collapsed.txt` (1818869 B)
- `cpu-flamegraph.html` (274794 B)
- `server-stdout.log` (327109 B)
- `gc.log` (100615 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
