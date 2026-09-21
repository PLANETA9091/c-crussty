# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.537 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.8, 2.1, 2.5, 2.0, 3.1, 3.2]
- spark tick-monitor MSPT: avg **416.6ms** / min 280.01ms / max **553.44ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T21:52:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7043848 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:54:35 INFO]: [crussty-plugin] [cruss | 352.17 | — | — | — | 553.44 | 416.6 |

- entity totals seen: [151293, 155157, 157779]
- top entity types (max seen): minecraft:item×111405, minecraft:husk×5623, minecraft:creeper×4998, minecraft:skeleton×4873, minecraft:zombie×4663, minecraft:drowned×4526, minecraft:spider×4369, minecraft:sheep×3501, minecraft:chicken×3452, minecraft:cow×3376, minecraft:pig×3229, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xIiy2dSB0Z
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **18640.1 ms**, avg **156.64 ms**, max **2279.1 ms**
- heap high-water seen: **7028 MB** -> last-after: **3764 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 111349)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33225 | 29.8% |
| kernel: other | 25453 | 22.9% |
| other | 12069 | 10.8% |
| chunk system (kernel) | 8656 | 7.8% |
| moonrise/paper patches | 7484 | 6.7% |
| JDK collections | 6378 | 5.7% |
| fastutil collections | 5959 | 5.4% |
| JIT stubs (vtable/itable) | 3397 | 3.1% |
| network (kernel) | 3018 | 2.7% |
| JDK other | 2384 | 2.1% |
| JDK invokes/VarHandle | 2291 | 2.1% |
| JVM internals (GC oop barriers) | 516 | 0.5% |
| vdso (clock) | 146 | 0.1% |
| block entities/hoppers (kernel) | 115 | 0.1% |
| bukkit api | 91 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 57741 | 51.9% |
| phase: unclassified | 34927 | 31.4% |
| phase: main tick (unclassified) | 11311 | 10.2% |
| phase: network sync (ServerEntity) | 2446 | 2.2% |
| phase: chunk tick | 2253 | 2.0% |
| phase: chunk system (off-main worker) | 1303 | 1.2% |
| phase: block entities (hoppers/furnaces) | 733 | 0.7% |
| phase: random tick | 450 | 0.4% |
| phase: mob spawning | 180 | 0.2% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98393** (88.4%) · native/JVM-internal **12877** (11.6%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3993 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3005 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2882 | 2.6% |
| `vtable stub` | native/JVM-internal | 2798 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1838 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1618 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1497 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1475 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1431 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1401 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1397 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1234 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1228 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1169 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1161 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1129 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1113 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1082 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1054 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1046 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1005 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 935 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 933 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 905 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 905 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 849 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 838 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 743 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 736 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 728 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 715 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 690 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 688 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 685 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 644 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 58012 | 94.7% |
| entities/mobs (kernel) | 1120 | 1.8% |
| kernel: other | 819 | 1.3% |
| chunk system (kernel) | 270 | 0.4% |
| moonrise/paper patches | 245 | 0.4% |
| JDK collections | 206 | 0.3% |
| fastutil collections | 184 | 0.3% |
| network (kernel) | 115 | 0.2% |
| JIT stubs (vtable/itable) | 105 | 0.2% |
| JDK other | 85 | 0.1% |
| JDK invokes/VarHandle | 65 | 0.1% |
| redstone (kernel) | 6 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58365 | 95.3% |
| phase: entity tick (AI/movement) | 2201 | 3.6% |
| phase: main tick (unclassified) | 387 | 0.6% |
| phase: chunk tick | 108 | 0.2% |
| phase: network sync (ServerEntity) | 86 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52466** (85.7%) · native/JVM-internal **8775** (14.3%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49237 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 119 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 39 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4626)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4626 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3016 | 65.2% |
| phase: entity tick (AI/movement) | 1204 | 26.0% |
| phase: main tick (unclassified) | 293 | 6.3% |
| phase: chunk system (off-main worker) | 49 | 1.1% |
| phase: network sync (ServerEntity) | 40 | 0.9% |
| phase: chunk tick | 10 | 0.2% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4626** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 602 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 538 | 11.6% |
| `char[]_[k]` | other | 444 | 9.6% |
| `byte[]_[k]` | other | 246 | 5.3% |
| `byte[]_[i]` | other | 233 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 192 | 4.2% |
| `int[]_[i]` | other | 187 | 4.0% |
| `long[]_[i]` | other | 155 | 3.4% |
| `java.util.ArrayList_[i]` | other | 150 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 3.1% |
| `java.lang.Object[]_[i]` | other | 125 | 2.7% |
| `java.util.regex.Matcher_[i]` | other | 114 | 2.5% |
| `java.util.Calendar$Builder_[i]` | other | 97 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 68 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 66 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.2% |
| `int[]_[k]` | other | 52 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111349 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20473 | 18.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7071 | 6.35% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5976 | 5.37% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4768 | 4.28% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1462 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1021 | 0.92% |
| `net/minecraft/world/entity/npc/Villager.tick` | 487 | 0.44% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 312 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 276 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 260 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 254 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 178 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 602 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | 538 | 11.6% |
| `char[]_[k]` | 444 | 9.6% |
| `byte[]_[k]` | 246 | 5.3% |
| `byte[]_[i]` | 233 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 192 | 4.2% |
| `int[]_[i]` | 187 | 4.0% |
| `long[]_[i]` | 155 | 3.4% |
| `java.util.ArrayList_[i]` | 150 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 18640 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148517..157779 (delta 9262, churn 6.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99921->111405, minecraft:drowned 3437->4526, minecraft:husk 4569->5623, minecraft:zombie 3628->4663, minecraft:skeleton 4129->4873, minecraft:pig 2636->3229, minecraft:spider 3874->4369, minecraft:creeper 4563->4998
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9262)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52964056 B)
- `wall-collapsed.txt` (3352635 B)
- `alloc-collapsed.txt` (1937053 B)
- `cpu-flamegraph.html` (287889 B)
- `server-stdout.log` (641075 B)
- `gc.log` (112636 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
