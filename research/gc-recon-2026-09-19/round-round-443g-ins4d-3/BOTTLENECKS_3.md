# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.301 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.3, 2.7, 3.3, 3.3, 3.3]
- spark tick-monitor MSPT: avg **388.08ms** / min 270.67ms / max **486.06ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:53:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8985189 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:55:48 INFO]: [crussty-plugin] [cruss | 321.25 | — | — | — | 486.06 | 388.08 |

- entity totals seen: [150864, 153124, 153695]
- top entity types (max seen): minecraft:item×106908, minecraft:husk×5394, minecraft:creeper×5137, minecraft:skeleton×4819, minecraft:zombie×4578, minecraft:drowned×4526, minecraft:spider×4504, minecraft:sheep×3538, minecraft:chicken×3431, minecraft:cow×3319, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KAYbFMqKH1
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **137** (Full GC: **10**)
- total pause: **21195.7 ms**, avg **154.71 ms**, max **2098.7 ms**
- heap high-water seen: **7658 MB** -> last-after: **5481 MB**
  - Young (Allocation Failure): 114
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 103132)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35033 | 34.0% |
| kernel: other | 19601 | 19.0% |
| other | 11402 | 11.1% |
| chunk system (kernel) | 8070 | 7.8% |
| JDK collections | 6991 | 6.8% |
| moonrise/paper patches | 5385 | 5.2% |
| fastutil collections | 4640 | 4.5% |
| network (kernel) | 3356 | 3.3% |
| JIT stubs (vtable/itable) | 3108 | 3.0% |
| JDK invokes/VarHandle | 2313 | 2.2% |
| JDK other | 2275 | 2.2% |
| JVM internals (GC oop barriers) | 469 | 0.5% |
| vdso (clock) | 130 | 0.1% |
| redstone (kernel) | 114 | 0.1% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| bukkit api | 65 | 0.1% |
| craftbukkit glue | 57 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49626 | 48.1% |
| phase: unclassified | 32596 | 31.6% |
| phase: main tick (unclassified) | 12242 | 11.9% |
| phase: chunk tick | 2969 | 2.9% |
| phase: network sync (ServerEntity) | 2738 | 2.7% |
| phase: chunk system (off-main worker) | 1225 | 1.2% |
| phase: block entities (hoppers/furnaces) | 850 | 0.8% |
| phase: random tick | 569 | 0.6% |
| phase: mob spawning | 316 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90406** (87.7%) · native/JVM-internal **12661** (12.3%) · other **65** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3771 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 3191 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2864 | 2.8% |
| `vtable stub` | native/JVM-internal | 2618 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2333 | 2.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1896 | 1.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1520 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1491 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1423 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1418 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1383 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1251 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1178 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1172 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1117 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1101 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1089 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1085 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1036 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1021 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1001 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 927 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 925 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 923 | 0.9% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 905 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 885 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 855 | 0.8% |
| `colpush_tick` | native/JVM-internal | 837 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 813 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 758 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 748 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 746 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 700 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 694 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 687 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 672 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 615 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 606 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 602 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 65102)

| bucket | self-time samples | share |
|---|---|---|
| other | 62166 | 95.5% |
| entities/mobs (kernel) | 1074 | 1.6% |
| kernel: other | 639 | 1.0% |
| chunk system (kernel) | 241 | 0.4% |
| JDK collections | 206 | 0.3% |
| JIT stubs (vtable/itable) | 176 | 0.3% |
| moonrise/paper patches | 174 | 0.3% |
| fastutil collections | 131 | 0.2% |
| network (kernel) | 118 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 75 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62434 | 95.9% |
| phase: entity tick (AI/movement) | 1933 | 3.0% |
| phase: main tick (unclassified) | 399 | 0.6% |
| phase: chunk tick | 115 | 0.2% |
| phase: network sync (ServerEntity) | 82 | 0.1% |
| phase: chunk system (off-main worker) | 55 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 32 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56200** (86.3%) · native/JVM-internal **8899** (13.7%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53380 | 82.0% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.3% |
| `read` | native/JVM-internal | 1222 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `vtable stub` | native/JVM-internal | 151 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 116 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 72 | 0.1% |
| `syscall` | native/JVM-internal | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 40 | 0.1% |
| `colpush_tick` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 38 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4951)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4951 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3264 | 65.9% |
| phase: entity tick (AI/movement) | 1330 | 26.9% |
| phase: main tick (unclassified) | 259 | 5.2% |
| phase: network sync (ServerEntity) | 30 | 0.6% |
| phase: chunk system (off-main worker) | 27 | 0.5% |
| phase: mob spawning | 20 | 0.4% |
| phase: block entities (hoppers/furnaces) | 11 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4951** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 690 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 513 | 10.4% |
| `char[]_[k]` | other | 451 | 9.1% |
| `byte[]_[k]` | other | 302 | 6.1% |
| `short[]_[k]` | other | 255 | 5.2% |
| `java.lang.Object[]_[i]` | other | 194 | 3.9% |
| `byte[]_[i]` | other | 153 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 2.9% |
| `long[]_[i]` | other | 140 | 2.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 137 | 2.8% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 123 | 2.5% |
| `java.util.ArrayList_[i]` | other | 110 | 2.2% |
| `short[]_[i]` | other | 97 | 2.0% |
| `long[]_[k]` | other | 84 | 1.7% |
| `java.lang.String_[i]` | other | 69 | 1.4% |
| `int[]_[i]` | other | 69 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 52 | 1.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 46 | 0.9% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 43 | 0.9% |
| `java.util.Optional_[i]` | other | 41 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103132 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19382 | 18.79% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5796 | 5.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5008 | 4.86% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3961 | 3.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1118 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 556 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 413 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 380 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 310 | 0.30% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 295 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.22% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 106 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 690 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | 513 | 10.4% |
| `char[]_[k]` | 451 | 9.1% |
| `byte[]_[k]` | 302 | 6.1% |
| `short[]_[k]` | 255 | 5.2% |
| `java.lang.Object[]_[i]` | 194 | 3.9% |
| `byte[]_[i]` | 153 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 2.9% |
| `long[]_[i]` | 140 | 2.8% |
| `net.minecraft.core.BlockPos_[i]` | 137 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 137 pauses / total 21196 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148114..153695 (delta 5581, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99925->106908, minecraft:drowned 3414->4526, minecraft:zombie 3575->4578, minecraft:husk 4640->5394, minecraft:skeleton 4131->4819, minecraft:creeper 4644->5137, minecraft:spider 4136->4504, minecraft:chicken 3117->3431
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5581)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44341365 B)
- `wall-collapsed.txt` (3026204 B)
- `alloc-collapsed.txt` (3190211 B)
- `cpu-flamegraph.html` (258528 B)
- `server-stdout.log` (275182 B)
- `gc.log` (129036 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
