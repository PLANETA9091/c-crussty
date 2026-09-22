# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.677 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 1.5, 1.7, 1.9, 2.4, 2.4]
- spark tick-monitor MSPT: avg **432.91ms** / min 365.86ms / max **622.92ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T08:27:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 4786608 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 365.86 | — | — | — | 622.92 | 432.91 |

- entity totals seen: [149219, 150373, 151672]
- top entity types (max seen): minecraft:item×103439, minecraft:creeper×5270, minecraft:husk×5242, minecraft:skeleton×4859, minecraft:spider×4797, minecraft:zombie×4701, minecraft:drowned×4564, minecraft:sheep×3528, minecraft:chicken×3417, minecraft:cow×3352, minecraft:pig×3259, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/SXv86hwa3R
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **23094.6 ms**, avg **199.09 ms**, max **2722.9 ms**
- heap high-water seen: **7819 MB** -> last-after: **3636 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 113285)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28039 | 24.8% |
| kernel: other | 25925 | 22.9% |
| other | 11354 | 10.0% |
| chunk system (kernel) | 10732 | 9.5% |
| moonrise/paper patches | 10714 | 9.5% |
| fastutil collections | 7948 | 7.0% |
| JDK collections | 6300 | 5.6% |
| network (kernel) | 4075 | 3.6% |
| JIT stubs (vtable/itable) | 3032 | 2.7% |
| JDK invokes/VarHandle | 2627 | 2.3% |
| JDK other | 1971 | 1.7% |
| vdso (clock) | 250 | 0.2% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| redstone (kernel) | 84 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 48 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92356 | 81.5% |
| phase: unclassified | 10156 | 9.0% |
| phase: main tick (unclassified) | 3870 | 3.4% |
| phase: network sync (ServerEntity) | 2239 | 2.0% |
| phase: chunk tick | 2169 | 1.9% |
| phase: chunk system (off-main worker) | 1221 | 1.1% |
| phase: block entities (hoppers/furnaces) | 693 | 0.6% |
| phase: random tick | 451 | 0.4% |
| phase: mob spawning | 129 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101702** (89.8%) · native/JVM-internal **11516** (10.2%) · other **67** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5265 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3566 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2811 | 2.5% |
| `vtable stub` | native/JVM-internal | 2420 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2418 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2223 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2094 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1839 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1726 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1704 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1562 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1545 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1433 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1376 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1376 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1258 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1250 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1170 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1151 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1145 | 1.0% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1090 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1071 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1060 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1022 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 989 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 958 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 912 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 903 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 897 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 889 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 881 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 812 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 794 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 727 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 691 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 685 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 680 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 672 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 656 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61241)

| bucket | self-time samples | share |
|---|---|---|
| other | 58001 | 94.7% |
| entities/mobs (kernel) | 921 | 1.5% |
| kernel: other | 844 | 1.4% |
| moonrise/paper patches | 320 | 0.5% |
| chunk system (kernel) | 290 | 0.5% |
| fastutil collections | 266 | 0.4% |
| JDK collections | 187 | 0.3% |
| network (kernel) | 128 | 0.2% |
| JIT stubs (vtable/itable) | 112 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 15 | 0.0% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57797 | 94.4% |
| phase: entity tick (AI/movement) | 3012 | 4.9% |
| phase: main tick (unclassified) | 203 | 0.3% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52290** (85.4%) · native/JVM-internal **8943** (14.6%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49088 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `syscall` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 72 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 60 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 59 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 22355)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 22355 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 20036 | 89.6% |
| phase: entity tick (AI/movement) | 2001 | 9.0% |
| phase: chunk system (off-main worker) | 201 | 0.9% |
| phase: main tick (unclassified) | 73 | 0.3% |
| phase: network sync (ServerEntity) | 24 | 0.1% |
| phase: block entities (hoppers/furnaces) | 11 | 0.0% |
| phase: chunk tick | 6 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **22355** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 4049 | 18.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2160 | 9.7% |
| `byte[]_[k]` | other | 1406 | 6.3% |
| `java.lang.Object[]_[i]` | other | 1243 | 5.6% |
| `short[]_[i]` | other | 1194 | 5.3% |
| `byte[]_[i]` | other | 1191 | 5.3% |
| `long[]_[k]` | other | 1045 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 925 | 4.1% |
| `java.lang.String_[i]` | other | 714 | 3.2% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 623 | 2.8% |
| `java.lang.Object[]_[k]` | other | 601 | 2.7% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 526 | 2.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 524 | 2.3% |
| `char[]_[k]` | other | 493 | 2.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 426 | 1.9% |
| `java.util.Optional_[i]` | other | 368 | 1.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 283 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 197 | 0.9% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 194 | 0.9% |
| `long[]_[i]` | other | 177 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113285 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34591 | 30.53% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22614 | 19.96% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6313 | 5.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5417 | 4.78% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4601 | 4.06% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 985 | 0.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 912 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 414 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 267 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 207 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 191 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 4049 | 18.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2160 | 9.7% |
| `byte[]_[k]` | 1406 | 6.3% |
| `java.lang.Object[]_[i]` | 1243 | 5.6% |
| `short[]_[i]` | 1194 | 5.3% |
| `byte[]_[i]` | 1191 | 5.3% |
| `long[]_[k]` | 1045 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | 925 | 4.1% |
| `java.lang.String_[i]` | 714 | 3.2% |
| `com.mojang.datafixers.util.Pair_[i]` | 623 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 23095 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148283..151672 (delta 3389, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99725->103439, minecraft:drowned 3516->4564, minecraft:zombie 3662->4701, minecraft:creeper 4559->5270, minecraft:husk 4540->5242, minecraft:spider 4235->4797, minecraft:skeleton 4329->4859, minecraft:chicken 3388->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3389)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49283767 B)
- `wall-collapsed.txt` (3403296 B)
- `alloc-collapsed.txt` (4853759 B)
- `cpu-flamegraph.html` (268617 B)
- `server-stdout.log` (250177 B)
- `gc.log` (110100 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
