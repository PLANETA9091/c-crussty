# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.424 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 2.2, 2.5, 3.0, 3.2, 3.3]
- spark tick-monitor MSPT: avg **407.51ms** / min 276.96ms / max **492.81ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:08:02Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7314819 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:10:11 INFO]: [crussty-plugin] [cruss | 334.13 | — | — | — | 492.81 | 407.51 |

- entity totals seen: [151597, 153979, 154103]
- top entity types (max seen): minecraft:item×107434, minecraft:husk×5449, minecraft:creeper×5045, minecraft:skeleton×4773, minecraft:zombie×4638, minecraft:drowned×4587, minecraft:spider×4508, minecraft:sheep×3516, minecraft:chicken×3386, minecraft:cow×3353, minecraft:pig×3200, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/8gUssGL0xL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **21910.4 ms**, avg **185.68 ms**, max **3007.4 ms**
- heap high-water seen: **7904 MB** -> last-after: **3908 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 107209)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36186 | 33.8% |
| kernel: other | 22494 | 21.0% |
| other | 11397 | 10.6% |
| chunk system (kernel) | 8715 | 8.1% |
| JDK collections | 7021 | 6.5% |
| fastutil collections | 5033 | 4.7% |
| moonrise/paper patches | 5027 | 4.7% |
| JIT stubs (vtable/itable) | 3746 | 3.5% |
| JDK invokes/VarHandle | 2674 | 2.5% |
| network (kernel) | 2396 | 2.2% |
| JDK other | 2002 | 1.9% |
| vdso (clock) | 121 | 0.1% |
| bukkit api | 106 | 0.1% |
| craftbukkit glue | 102 | 0.1% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53301 | 49.7% |
| phase: unclassified | 33722 | 31.5% |
| phase: main tick (unclassified) | 12595 | 11.7% |
| phase: chunk tick | 2587 | 2.4% |
| phase: network sync (ServerEntity) | 2324 | 2.2% |
| phase: chunk system (off-main worker) | 1109 | 1.0% |
| phase: block entities (hoppers/furnaces) | 759 | 0.7% |
| phase: random tick | 469 | 0.4% |
| phase: mob spawning | 341 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95131** (88.7%) · native/JVM-internal **11962** (11.2%) · other **116** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3983 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 3835 | 3.6% |
| `vtable stub` | native/JVM-internal | 3285 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2945 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2134 | 2.0% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1680 | 1.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1506 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1435 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1386 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1347 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1317 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1289 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1270 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1131 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1129 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1109 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1088 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1075 | 1.0% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 1038 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1015 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 995 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 975 | 0.9% |
| `colpush_tick` | native/JVM-internal | 953 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 943 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 933 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 906 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 872 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 862 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 852 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/Entity.setPosRaw` | JVM-Java | 693 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 659 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 658 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 641 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 629 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 627 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 623 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 592 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61797 | 95.3% |
| entities/mobs (kernel) | 1153 | 1.8% |
| kernel: other | 681 | 1.0% |
| chunk system (kernel) | 239 | 0.4% |
| JDK collections | 200 | 0.3% |
| JIT stubs (vtable/itable) | 197 | 0.3% |
| moonrise/paper patches | 183 | 0.3% |
| fastutil collections | 163 | 0.3% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 77 | 0.1% |
| network (kernel) | 72 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62047 | 95.7% |
| phase: entity tick (AI/movement) | 2100 | 3.2% |
| phase: main tick (unclassified) | 411 | 0.6% |
| phase: chunk tick | 118 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55955** (86.3%) · native/JVM-internal **8903** (13.7%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52995 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4793 | 7.4% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 183 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 100 | 0.2% |
| `syscall` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 39 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 37 | 0.1% |
| `colpush_tick` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10098)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10098 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8488 | 84.1% |
| phase: entity tick (AI/movement) | 1192 | 11.8% |
| phase: main tick (unclassified) | 239 | 2.4% |
| phase: chunk system (off-main worker) | 120 | 1.2% |
| phase: network sync (ServerEntity) | 30 | 0.3% |
| phase: block entities (hoppers/furnaces) | 14 | 0.1% |
| phase: mob spawning | 13 | 0.1% |
| phase: chunk tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10098** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1698 | 16.8% |
| `byte[]_[k]` | other | 715 | 7.1% |
| `long[]_[k]` | other | 588 | 5.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 571 | 5.7% |
| `short[]_[i]` | other | 504 | 5.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 494 | 4.9% |
| `java.lang.Object[]_[i]` | other | 487 | 4.8% |
| `char[]_[k]` | other | 456 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 454 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 444 | 4.4% |
| `byte[]_[i]` | other | 400 | 4.0% |
| `long[]_[i]` | other | 185 | 1.8% |
| `java.lang.Object[]_[k]` | other | 180 | 1.8% |
| `java.lang.String_[i]` | other | 177 | 1.8% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 131 | 1.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 128 | 1.3% |
| `java.util.ArrayList_[i]` | other | 113 | 1.1% |
| `java.util.Optional_[i]` | other | 109 | 1.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 95 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107209 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20472 | 19.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6303 | 5.88% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5691 | 5.31% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3982 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1347 | 1.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 638 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 456 | 0.43% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 426 | 0.40% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 349 | 0.33% |
| `net/minecraft/world/entity/npc/Villager.tick` | 346 | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 110 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1698 | 16.8% |
| `byte[]_[k]` | 715 | 7.1% |
| `long[]_[k]` | 588 | 5.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 571 | 5.7% |
| `short[]_[i]` | 504 | 5.0% |
| `net.minecraft.world.phys.AABB_[i]` | 494 | 4.9% |
| `java.lang.Object[]_[i]` | 487 | 4.8% |
| `char[]_[k]` | 456 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 454 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 444 | 4.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 21910 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148566..154103 (delta 5537, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100262->107434, minecraft:drowned 3449->4587, minecraft:zombie 3634->4638, minecraft:husk 4659->5449, minecraft:skeleton 4156->4773, minecraft:creeper 4614->5045, minecraft:chicken 3010->3386, minecraft:cow 2992->3353
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5537)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55736297 B)
- `wall-collapsed.txt` (3287898 B)
- `alloc-collapsed.txt` (3870824 B)
- `cpu-flamegraph.html` (268452 B)
- `server-stdout.log` (281571 B)
- `gc.log` (111804 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
