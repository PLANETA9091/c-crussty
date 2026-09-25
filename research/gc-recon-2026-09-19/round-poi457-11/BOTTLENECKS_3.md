# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.976 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.8, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **408.82ms** / min 325.66ms / max **518.81ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:00:48Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6953086 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:02:59 INFO]: [crussty-plugin] [cruss | 325.66 | — | — | — | 518.81 | 408.82 |

- entity totals seen: [150111, 152077, 153693]
- top entity types (max seen): minecraft:item×106831, minecraft:husk×5492, minecraft:creeper×4933, minecraft:skeleton×4782, minecraft:zombie×4557, minecraft:drowned×4540, minecraft:spider×4421, minecraft:sheep×3521, minecraft:chicken×3411, minecraft:cow×3362, minecraft:pig×3163, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/NbngzxVkQy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **20069.8 ms**, avg **173.02 ms**, max **2688.6 ms**
- heap high-water seen: **7566 MB** -> last-after: **4175 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104147)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34566 | 33.2% |
| kernel: other | 21622 | 20.8% |
| other | 11647 | 11.2% |
| chunk system (kernel) | 7962 | 7.6% |
| JDK collections | 7350 | 7.1% |
| moonrise/paper patches | 5020 | 4.8% |
| fastutil collections | 4734 | 4.5% |
| JIT stubs (vtable/itable) | 3364 | 3.2% |
| network (kernel) | 2837 | 2.7% |
| JDK invokes/VarHandle | 2472 | 2.4% |
| JDK other | 2147 | 2.1% |
| vdso (clock) | 97 | 0.1% |
| bukkit api | 89 | 0.1% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| craftbukkit glue | 78 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50275 | 48.3% |
| phase: unclassified | 33205 | 31.9% |
| phase: main tick (unclassified) | 12923 | 12.4% |
| phase: chunk tick | 2632 | 2.5% |
| phase: network sync (ServerEntity) | 2129 | 2.0% |
| phase: chunk system (off-main worker) | 1116 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1041 | 1.0% |
| phase: random tick | 524 | 0.5% |
| phase: mob spawning | 299 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92244** (88.6%) · native/JVM-internal **11790** (11.3%) · other **113** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3790 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3292 | 3.2% |
| `vtable stub` | native/JVM-internal | 2837 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2655 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2015 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1522 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1410 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1381 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1372 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1311 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1233 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1214 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1112 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1105 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1068 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 989 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 970 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 960 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 924 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 919 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 903 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 869 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 842 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 834 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 821 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 738 | 0.7% |
| `colpush_tick` | native/JVM-internal | 735 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 716 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 708 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 697 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 695 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 686 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 682 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 651 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 647 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 624 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 591 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63661)

| bucket | self-time samples | share |
|---|---|---|
| other | 60786 | 95.5% |
| entities/mobs (kernel) | 1036 | 1.6% |
| kernel: other | 628 | 1.0% |
| JDK collections | 233 | 0.4% |
| chunk system (kernel) | 231 | 0.4% |
| JIT stubs (vtable/itable) | 163 | 0.3% |
| moonrise/paper patches | 137 | 0.2% |
| fastutil collections | 126 | 0.2% |
| network (kernel) | 97 | 0.2% |
| JDK invokes/VarHandle | 89 | 0.1% |
| JDK other | 61 | 0.1% |
| JVM internals (GC oop barriers) | 58 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61064 | 95.9% |
| phase: entity tick (AI/movement) | 1910 | 3.0% |
| phase: main tick (unclassified) | 403 | 0.6% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 71 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54035** (84.9%) · native/JVM-internal **9618** (15.1%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51310 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.5% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 579 | 0.9% |
| `vtable stub` | native/JVM-internal | 137 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 41 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 41 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 38 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3384)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3384 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1995 | 59.0% |
| phase: entity tick (AI/movement) | 1032 | 30.5% |
| phase: main tick (unclassified) | 256 | 7.6% |
| phase: chunk system (off-main worker) | 39 | 1.2% |
| phase: network sync (ServerEntity) | 27 | 0.8% |
| phase: block entities (hoppers/furnaces) | 20 | 0.6% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3384** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 539 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 425 | 12.6% |
| `char[]_[k]` | other | 405 | 12.0% |
| `byte[]_[k]` | other | 218 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 142 | 4.2% |
| `long[]_[i]` | other | 133 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 114 | 3.4% |
| `java.util.ArrayList_[i]` | other | 108 | 3.2% |
| `int[]_[i]` | other | 97 | 2.9% |
| `java.lang.Object[]_[i]` | other | 87 | 2.6% |
| `byte[]_[i]` | other | 65 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 56 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 45 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.2% |
| `java.math.BigInteger_[i]` | other | 33 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb199a02aa0_[i]` | other | 27 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104147 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19372 | 18.60% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6150 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5127 | 4.92% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3848 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1316 | 1.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 557 | 0.53% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 425 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 396 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 343 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 305 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 105 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 539 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | 425 | 12.6% |
| `char[]_[k]` | 405 | 12.0% |
| `byte[]_[k]` | 218 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 142 | 4.2% |
| `long[]_[i]` | 133 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 114 | 3.4% |
| `java.util.ArrayList_[i]` | 108 | 3.2% |
| `int[]_[i]` | 97 | 2.9% |
| `java.lang.Object[]_[i]` | 87 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 20070 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148176..153693 (delta 5517, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99946->106831, minecraft:drowned 3612->4540, minecraft:husk 4575->5492, minecraft:zombie 3775->4557, minecraft:skeleton 4266->4782, minecraft:creeper 4534->4933, minecraft:chicken 3061->3411, minecraft:spider 4075->4421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5517)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51480846 B)
- `wall-collapsed.txt` (3081944 B)
- `alloc-collapsed.txt` (1766632 B)
- `cpu-flamegraph.html` (280596 B)
- `server-stdout.log` (331085 B)
- `gc.log` (110103 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
