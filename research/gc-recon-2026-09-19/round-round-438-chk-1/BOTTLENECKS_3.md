# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.901 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.0, 1.7, 2.8, 2.6]
- spark tick-monitor MSPT: avg **362.15ms** / min 302.67ms / max **504.24ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T01:16:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6856648 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:18:37 INFO]: [crussty-plugin] [cruss | 302.67 | — | — | — | 504.24 | 362.15 |

- entity totals seen: [150481, 152762, 153642]
- top entity types (max seen): minecraft:item×106900, minecraft:husk×5435, minecraft:creeper×4963, minecraft:skeleton×4788, minecraft:zombie×4564, minecraft:drowned×4538, minecraft:spider×4455, minecraft:sheep×3524, minecraft:chicken×3410, minecraft:cow×3362, minecraft:pig×3174, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/WXOVcfLHaO
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19492.5 ms**, avg **170.99 ms**, max **2659.6 ms**
- heap high-water seen: **7461 MB** -> last-after: **4163 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 103995)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33563 | 32.3% |
| kernel: other | 21654 | 20.8% |
| other | 13670 | 13.1% |
| chunk system (kernel) | 7691 | 7.4% |
| JDK collections | 6938 | 6.7% |
| moonrise/paper patches | 5009 | 4.8% |
| fastutil collections | 4527 | 4.4% |
| JIT stubs (vtable/itable) | 3326 | 3.2% |
| network (kernel) | 2774 | 2.7% |
| JDK invokes/VarHandle | 2142 | 2.1% |
| JDK other | 1672 | 1.6% |
| JVM internals (GC oop barriers) | 570 | 0.5% |
| vdso (clock) | 102 | 0.1% |
| bukkit api | 94 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 72 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49087 | 47.2% |
| phase: unclassified | 35044 | 33.7% |
| phase: main tick (unclassified) | 12522 | 12.0% |
| phase: chunk tick | 2388 | 2.3% |
| phase: network sync (ServerEntity) | 1994 | 1.9% |
| phase: chunk system (off-main worker) | 1142 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1032 | 1.0% |
| phase: random tick | 500 | 0.5% |
| phase: mob spawning | 285 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89363** (85.9%) · native/JVM-internal **14476** (13.9%) · other **156** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3763 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3125 | 3.0% |
| `vtable stub` | native/JVM-internal | 2845 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2595 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1501 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1384 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1322 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1315 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1302 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1262 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1213 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1206 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1192 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1134 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1071 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1032 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1018 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 963 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 935 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 927 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 889 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 888 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 818 | 0.8% |
| `WallClock::signalHandler` | native/JVM-internal | 814 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 794 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 782 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 780 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 773 | 0.7% |
| `colpush_tick` | native/JVM-internal | 761 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 714 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 694 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 687 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 681 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 665 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 662 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 659 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64851)

| bucket | self-time samples | share |
|---|---|---|
| other | 61828 | 95.3% |
| entities/mobs (kernel) | 1062 | 1.6% |
| kernel: other | 768 | 1.2% |
| chunk system (kernel) | 252 | 0.4% |
| JDK collections | 221 | 0.3% |
| JIT stubs (vtable/itable) | 164 | 0.3% |
| moonrise/paper patches | 158 | 0.2% |
| fastutil collections | 150 | 0.2% |
| network (kernel) | 91 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JDK other | 62 | 0.1% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62176 | 95.9% |
| phase: entity tick (AI/movement) | 1919 | 3.0% |
| phase: main tick (unclassified) | 462 | 0.7% |
| phase: chunk tick | 109 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56026** (86.4%) · native/JVM-internal **8821** (13.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53089 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.4% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 146 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 125 | 0.2% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 69 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 38 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 38 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3619)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3619 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2162 | 59.7% |
| phase: entity tick (AI/movement) | 1065 | 29.4% |
| phase: main tick (unclassified) | 273 | 7.5% |
| phase: chunk system (off-main worker) | 47 | 1.3% |
| phase: block entities (hoppers/furnaces) | 32 | 0.9% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3619** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 481 | 13.3% |
| `char[]_[k]` | other | 415 | 11.5% |
| `byte[]_[k]` | other | 243 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 135 | 3.7% |
| `long[]_[i]` | other | 133 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 131 | 3.6% |
| `java.util.ArrayList_[i]` | other | 129 | 3.6% |
| `java.lang.Object[]_[i]` | other | 99 | 2.7% |
| `byte[]_[i]` | other | 94 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 2.0% |
| `int[]_[i]` | other | 70 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f709582f3c0_[i]` | other | 39 | 1.1% |
| `int[]_[k]` | other | 35 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f70959fd0e8_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 32 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103995 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18781 | 18.06% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5883 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4921 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3844 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1389 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 583 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 378 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 371 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 355 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 284 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 257 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 114 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 481 | 13.3% |
| `char[]_[k]` | 415 | 11.5% |
| `byte[]_[k]` | 243 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | 135 | 3.7% |
| `long[]_[i]` | 133 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 131 | 3.6% |
| `java.util.ArrayList_[i]` | 129 | 3.6% |
| `java.lang.Object[]_[i]` | 99 | 2.7% |
| `byte[]_[i]` | 94 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19492 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148205..153642 (delta 5437, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99960->106900, minecraft:drowned 3676->4538, minecraft:husk 4594->5435, minecraft:zombie 3817->4564, minecraft:skeleton 4253->4788, minecraft:creeper 4535->4963, minecraft:spider 4100->4455, minecraft:pig 2843->3174
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5437)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51048881 B)
- `wall-collapsed.txt` (3300670 B)
- `alloc-collapsed.txt` (1878856 B)
- `cpu-flamegraph.html` (280326 B)
- `server-stdout.log` (331895 B)
- `gc.log` (108371 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
