# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.6 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.3, 1.9, 2.1, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **370.18ms** / min 308.32ms / max **472.87ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:53:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6807160 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:55:35 INFO]: [crussty-plugin] [cruss | 308.32 | — | — | — | 472.87 | 370.18 |

- entity totals seen: [150749, 152853, 153919]
- top entity types (max seen): minecraft:item×107117, minecraft:husk×5493, minecraft:creeper×5016, minecraft:skeleton×4827, minecraft:zombie×4577, minecraft:drowned×4529, minecraft:spider×4375, minecraft:sheep×3511, minecraft:chicken×3399, minecraft:cow×3365, minecraft:pig×3187, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/LJry2Lqwby
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19311.6 ms**, avg **169.40 ms**, max **2444.5 ms**
- heap high-water seen: **7409 MB** -> last-after: **4122 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104644)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34556 | 33.0% |
| kernel: other | 21913 | 20.9% |
| other | 12132 | 11.6% |
| chunk system (kernel) | 8195 | 7.8% |
| JDK collections | 7083 | 6.8% |
| moonrise/paper patches | 5521 | 5.3% |
| fastutil collections | 4671 | 4.5% |
| JIT stubs (vtable/itable) | 3273 | 3.1% |
| network (kernel) | 2787 | 2.7% |
| JDK invokes/VarHandle | 2283 | 2.2% |
| JDK other | 1763 | 1.7% |
| vdso (clock) | 139 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| craftbukkit glue | 80 | 0.1% |
| bukkit api | 78 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 22 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50004 | 47.8% |
| phase: unclassified | 33789 | 32.3% |
| phase: main tick (unclassified) | 13102 | 12.5% |
| phase: chunk tick | 2483 | 2.4% |
| phase: network sync (ServerEntity) | 2212 | 2.1% |
| phase: chunk system (off-main worker) | 1330 | 1.3% |
| phase: block entities (hoppers/furnaces) | 898 | 0.9% |
| phase: random tick | 525 | 0.5% |
| phase: mob spawning | 300 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92143** (88.1%) · native/JVM-internal **12409** (11.9%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4087 | 3.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3115 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2994 | 2.9% |
| `vtable stub` | native/JVM-internal | 2784 | 2.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1502 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1411 | 1.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1282 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1272 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1256 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1244 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1240 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1213 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1128 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1127 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1126 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1052 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1036 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 997 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 964 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 948 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 945 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 911 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 894 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 893 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 866 | 0.8% |
| `colpush_tick` | native/JVM-internal | 788 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 785 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 779 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 756 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 729 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 704 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 677 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 670 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 669 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 657 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 622 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 606 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61987 | 95.6% |
| entities/mobs (kernel) | 1003 | 1.5% |
| kernel: other | 706 | 1.1% |
| chunk system (kernel) | 226 | 0.3% |
| JDK collections | 181 | 0.3% |
| JIT stubs (vtable/itable) | 159 | 0.2% |
| moonrise/paper patches | 138 | 0.2% |
| fastutil collections | 135 | 0.2% |
| network (kernel) | 98 | 0.2% |
| JDK invokes/VarHandle | 92 | 0.1% |
| JDK other | 64 | 0.1% |
| JVM internals (GC oop barriers) | 60 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62335 | 96.1% |
| phase: entity tick (AI/movement) | 1761 | 2.7% |
| phase: main tick (unclassified) | 446 | 0.7% |
| phase: chunk tick | 141 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55469** (85.5%) · native/JVM-internal **9385** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52732 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.4% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 415 | 0.6% |
| `vtable stub` | native/JVM-internal | 139 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 104 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 41 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 37 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 37 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 35 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 33 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3676)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3676 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2194 | 59.7% |
| phase: entity tick (AI/movement) | 1112 | 30.3% |
| phase: main tick (unclassified) | 250 | 6.8% |
| phase: chunk system (off-main worker) | 45 | 1.2% |
| phase: block entities (hoppers/furnaces) | 36 | 1.0% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3676** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 504 | 13.7% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 486 | 13.2% |
| `char[]_[k]` | other | 407 | 11.1% |
| `byte[]_[k]` | other | 245 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 124 | 3.4% |
| `long[]_[i]` | other | 121 | 3.3% |
| `java.util.ArrayList_[i]` | other | 114 | 3.1% |
| `java.lang.Object[]_[i]` | other | 111 | 3.0% |
| `byte[]_[i]` | other | 107 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 82 | 2.2% |
| `int[]_[i]` | other | 80 | 2.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 52 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 45 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 42 | 1.1% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 33 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f401182bd30_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104644 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18859 | 18.02% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6081 | 5.81% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4984 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3941 | 3.77% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1423 | 1.36% |
| `net/minecraft/world/entity/ai/Brain.tick` | 595 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 417 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 384 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 374 | 0.36% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 288 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 252 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 83 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 504 | 13.7% |
| `net.minecraft.world.phys.Vec3_[i]` | 486 | 13.2% |
| `char[]_[k]` | 407 | 11.1% |
| `byte[]_[k]` | 245 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 124 | 3.4% |
| `long[]_[i]` | 121 | 3.3% |
| `java.util.ArrayList_[i]` | 114 | 3.1% |
| `java.lang.Object[]_[i]` | 111 | 3.0% |
| `byte[]_[i]` | 107 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19312 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148283..153919 (delta 5636, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100041->107117, minecraft:husk 4586->5493, minecraft:drowned 3653->4529, minecraft:zombie 3790->4577, minecraft:skeleton 4274->4827, minecraft:creeper 4563->5016, minecraft:pig 2829->3187, minecraft:cow 3011->3365
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5636)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51997547 B)
- `wall-collapsed.txt` (3047176 B)
- `alloc-collapsed.txt` (1942192 B)
- `cpu-flamegraph.html` (279957 B)
- `server-stdout.log` (331156 B)
- `gc.log` (108346 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
