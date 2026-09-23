# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.574 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.3, 2.2, 2.5, 3.1, 3.3, 3.1]
- spark tick-monitor MSPT: avg **401.64ms** / min 265.41ms / max **554.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T15:53:05Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7177515 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:55:16 INFO]: [crussty-plugin] [cruss | 321.3 | — | — | — | 554.3 | 401.64 |

- entity totals seen: [151446, 153904, 154206]
- top entity types (max seen): minecraft:item×107368, minecraft:husk×5449, minecraft:creeper×5002, minecraft:skeleton×4795, minecraft:zombie×4625, minecraft:drowned×4591, minecraft:spider×4497, minecraft:sheep×3528, minecraft:chicken×3394, minecraft:cow×3357, minecraft:pig×3183, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Coog1xy8vH
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **9**)
- total pause: **23356.5 ms**, avg **188.36 ms**, max **3388.8 ms**
- heap high-water seen: **7548 MB** -> last-after: **3870 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 104518)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32923 | 31.5% |
| kernel: other | 23803 | 22.8% |
| other | 11616 | 11.1% |
| chunk system (kernel) | 8699 | 8.3% |
| JDK collections | 6926 | 6.6% |
| moonrise/paper patches | 5394 | 5.2% |
| fastutil collections | 4463 | 4.3% |
| JIT stubs (vtable/itable) | 3773 | 3.6% |
| network (kernel) | 2402 | 2.3% |
| JDK invokes/VarHandle | 2349 | 2.2% |
| JDK other | 1681 | 1.6% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| craftbukkit glue | 92 | 0.1% |
| bukkit api | 83 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51247 | 49.0% |
| phase: unclassified | 32549 | 31.1% |
| phase: main tick (unclassified) | 12937 | 12.4% |
| phase: chunk tick | 2617 | 2.5% |
| phase: network sync (ServerEntity) | 2332 | 2.2% |
| phase: chunk system (off-main worker) | 998 | 1.0% |
| phase: block entities (hoppers/furnaces) | 982 | 0.9% |
| phase: random tick | 525 | 0.5% |
| phase: mob spawning | 329 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92336** (88.3%) · native/JVM-internal **12057** (11.5%) · other **125** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4001 | 3.8% |
| `vtable stub` | native/JVM-internal | 3310 | 3.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3203 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2994 | 2.9% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1598 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1573 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1468 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1389 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1355 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1294 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1285 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1138 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1113 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1074 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1063 | 1.0% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 1058 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1055 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1046 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 977 | 0.9% |
| `colpush_tick` | native/JVM-internal | 972 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 971 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 939 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 930 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 906 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 841 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 839 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 833 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 793 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 773 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 752 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 750 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 699 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 695 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 673 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 661 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 627 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 609 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 556 | 0.5% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 517 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 64851)

| bucket | self-time samples | share |
|---|---|---|
| other | 61871 | 95.4% |
| entities/mobs (kernel) | 1071 | 1.7% |
| kernel: other | 746 | 1.2% |
| chunk system (kernel) | 274 | 0.4% |
| JDK collections | 211 | 0.3% |
| moonrise/paper patches | 174 | 0.3% |
| JIT stubs (vtable/itable) | 170 | 0.3% |
| fastutil collections | 138 | 0.2% |
| network (kernel) | 62 | 0.1% |
| JDK invokes/VarHandle | 59 | 0.1% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62194 | 95.9% |
| phase: entity tick (AI/movement) | 1936 | 3.0% |
| phase: main tick (unclassified) | 435 | 0.7% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 25 | 0.0% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 20 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55969** (86.3%) · native/JVM-internal **8877** (13.7%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53091 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.4% |
| `read` | native/JVM-internal | 1211 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 155 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 108 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `syscall` | native/JVM-internal | 68 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 40 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4568)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4568 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2916 | 63.8% |
| phase: entity tick (AI/movement) | 1226 | 26.8% |
| phase: main tick (unclassified) | 297 | 6.5% |
| phase: chunk system (off-main worker) | 48 | 1.1% |
| phase: network sync (ServerEntity) | 38 | 0.8% |
| phase: block entities (hoppers/furnaces) | 25 | 0.5% |
| phase: mob spawning | 8 | 0.2% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4568** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 581 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 525 | 11.5% |
| `char[]_[k]` | other | 434 | 9.5% |
| `byte[]_[k]` | other | 352 | 7.7% |
| `byte[]_[i]` | other | 234 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 227 | 5.0% |
| `int[]_[i]` | other | 154 | 3.4% |
| `java.util.ArrayList_[i]` | other | 137 | 3.0% |
| `java.lang.Object[]_[i]` | other | 136 | 3.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 128 | 2.8% |
| `long[]_[i]` | other | 124 | 2.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 72 | 1.6% |
| `java.util.GregorianCalendar_[i]` | other | 66 | 1.4% |
| `java.util.ArrayList$Itr_[i]` | other | 64 | 1.4% |
| `java.util.Calendar$Builder_[i]` | other | 62 | 1.4% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 62 | 1.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 49 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 38 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 38 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 38 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104518 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19548 | 18.70% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6105 | 5.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5571 | 5.33% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3959 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1360 | 1.30% |
| `net/minecraft/world/entity/ai/Brain.tick` | 641 | 0.61% |
| `net/minecraft/world/entity/npc/Villager.tick` | 359 | 0.34% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 306 | 0.29% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 281 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.22% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 193 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 102 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 581 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | 525 | 11.5% |
| `char[]_[k]` | 434 | 9.5% |
| `byte[]_[k]` | 352 | 7.7% |
| `byte[]_[i]` | 234 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 227 | 5.0% |
| `int[]_[i]` | 154 | 3.4% |
| `java.util.ArrayList_[i]` | 137 | 3.0% |
| `java.lang.Object[]_[i]` | 136 | 3.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 128 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 23356 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148496..154206 (delta 5710, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100201->107368, minecraft:drowned 3446->4591, minecraft:zombie 3654->4625, minecraft:husk 4631->5449, minecraft:skeleton 4160->4795, minecraft:creeper 4610->5002, minecraft:pig 2820->3183, minecraft:chicken 3051->3394
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5710)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50881341 B)
- `wall-collapsed.txt` (3252215 B)
- `alloc-collapsed.txt` (2042108 B)
- `cpu-flamegraph.html` (289247 B)
- `server-stdout.log` (5665095 B)
- `gc.log` (117005 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
