# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.704 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.5, 1.9, 2.2, 2.5, 2.8, 2.8]
- spark tick-monitor MSPT: avg **359.95ms** / min 301.94ms / max **533.27ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:05:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6982878 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:07:47 INFO]: [crussty-plugin] [cruss | 301.94 | — | — | — | 533.27 | 359.95 |

- entity totals seen: [150800, 152829, 153521]
- top entity types (max seen): minecraft:item×106732, minecraft:husk×5373, minecraft:creeper×5018, minecraft:skeleton×4810, minecraft:zombie×4582, minecraft:drowned×4541, minecraft:spider×4466, minecraft:sheep×3515, minecraft:chicken×3399, minecraft:cow×3367, minecraft:pig×3182, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/TDBjZPF5ca
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **20013.6 ms**, avg **177.11 ms**, max **2751.4 ms**
- heap high-water seen: **7289 MB** -> last-after: **3985 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103702)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33548 | 32.4% |
| kernel: other | 22452 | 21.7% |
| other | 12067 | 11.6% |
| chunk system (kernel) | 8317 | 8.0% |
| JDK collections | 6345 | 6.1% |
| moonrise/paper patches | 5285 | 5.1% |
| fastutil collections | 4762 | 4.6% |
| JIT stubs (vtable/itable) | 3450 | 3.3% |
| network (kernel) | 2923 | 2.8% |
| JDK invokes/VarHandle | 2396 | 2.3% |
| JDK other | 1684 | 1.6% |
| vdso (clock) | 124 | 0.1% |
| block entities/hoppers (kernel) | 93 | 0.1% |
| bukkit api | 83 | 0.1% |
| redstone (kernel) | 77 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50558 | 48.8% |
| phase: unclassified | 32444 | 31.3% |
| phase: main tick (unclassified) | 12747 | 12.3% |
| phase: chunk tick | 2587 | 2.5% |
| phase: network sync (ServerEntity) | 2294 | 2.2% |
| phase: chunk system (off-main worker) | 1244 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1002 | 1.0% |
| phase: random tick | 511 | 0.5% |
| phase: mob spawning | 314 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91387** (88.1%) · native/JVM-internal **12200** (11.8%) · other **115** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4164 | 4.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3279 | 3.2% |
| `vtable stub` | native/JVM-internal | 2945 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2945 | 2.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1568 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1563 | 1.5% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1520 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1370 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1293 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1198 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1161 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1151 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1137 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 997 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 976 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 975 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 966 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 940 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 934 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 929 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 918 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 907 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 889 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 851 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 848 | 0.8% |
| `colpush_tick` | native/JVM-internal | 809 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 767 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 761 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 725 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 724 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 720 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 712 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 700 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 639 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 636 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 617 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 608 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 58385 | 95.3% |
| entities/mobs (kernel) | 1013 | 1.7% |
| kernel: other | 684 | 1.1% |
| chunk system (kernel) | 232 | 0.4% |
| JDK collections | 191 | 0.3% |
| moonrise/paper patches | 166 | 0.3% |
| JIT stubs (vtable/itable) | 166 | 0.3% |
| fastutil collections | 143 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JVM internals (GC oop barriers) | 57 | 0.1% |
| JDK other | 44 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| vdso (clock) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58704 | 95.8% |
| phase: entity tick (AI/movement) | 1761 | 2.9% |
| phase: main tick (unclassified) | 459 | 0.7% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: mob spawning | 62 | 0.1% |
| phase: block entities (hoppers/furnaces) | 46 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: random tick | 24 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52083** (85.0%) · native/JVM-internal **9165** (15.0%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49343 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 233 | 0.4% |
| `vtable stub` | native/JVM-internal | 140 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 109 | 0.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 43 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 40 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 39 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 37 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007fce7d9f7a38.accept` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 23285)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 23285 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21646 | 93.0% |
| phase: entity tick (AI/movement) | 1122 | 4.8% |
| phase: main tick (unclassified) | 273 | 1.2% |
| phase: chunk system (off-main worker) | 178 | 0.8% |
| phase: network sync (ServerEntity) | 31 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.1% |
| phase: mob spawning | 4 | 0.0% |
| phase: chunk tick | 4 | 0.0% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **23285** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3079 | 13.2% |
| `byte[]_[i]` | other | 2146 | 9.2% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 1810 | 7.8% |
| `java.lang.String_[i]` | other | 1493 | 6.4% |
| `java.lang.Object[]_[i]` | other | 1447 | 6.2% |
| `byte[]_[k]` | other | 1260 | 5.4% |
| `long[]_[k]` | other | 1101 | 4.7% |
| `short[]_[i]` | other | 968 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 820 | 3.5% |
| `java.lang.Object[]_[k]` | other | 761 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 682 | 2.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 562 | 2.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 548 | 2.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 469 | 2.0% |
| `java.util.Optional_[i]` | other | 463 | 2.0% |
| `char[]_[k]` | other | 431 | 1.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 280 | 1.2% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 232 | 1.0% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 210 | 0.9% |
| `long[]_[i]` | other | 207 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103702 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19216 | 18.53% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6237 | 6.01% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4991 | 4.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3949 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1431 | 1.38% |
| `net/minecraft/world/entity/ai/Brain.tick` | 620 | 0.60% |
| `net/minecraft/world/entity/npc/Villager.tick` | 344 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 275 | 0.27% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 260 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 167 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 109 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3079 | 13.2% |
| `byte[]_[i]` | 2146 | 9.2% |
| `com.mojang.serialization.DataResult$Success_[i]` | 1810 | 7.8% |
| `java.lang.String_[i]` | 1493 | 6.4% |
| `java.lang.Object[]_[i]` | 1447 | 6.2% |
| `byte[]_[k]` | 1260 | 5.4% |
| `long[]_[k]` | 1101 | 4.7% |
| `short[]_[i]` | 968 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 820 | 3.5% |
| `java.lang.Object[]_[k]` | 761 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 20014 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148303..153521 (delta 5218, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99992->106732, minecraft:drowned 3554->4541, minecraft:zombie 3653->4582, minecraft:husk 4554->5373, minecraft:skeleton 4229->4810, minecraft:creeper 4598->5018, minecraft:spider 4133->4466, minecraft:pig 2890->3182
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5218)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49405295 B)
- `wall-collapsed.txt` (2892071 B)
- `alloc-collapsed.txt` (6489190 B)
- `cpu-flamegraph.html` (273564 B)
- `server-stdout.log` (339959 B)
- `gc.log` (107504 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
