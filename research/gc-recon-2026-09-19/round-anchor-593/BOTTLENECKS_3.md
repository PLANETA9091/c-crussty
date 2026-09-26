# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.85 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 1.5, 1.2, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **410.57ms** / min 332.95ms / max **561.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-26T01:24:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6433455 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 332.95 | — | — | — | 561.5 | 410.57 |

- entity totals seen: [149008, 150071, 151223]
- top entity types (max seen): minecraft:item×103164, minecraft:creeper×5188, minecraft:husk×5168, minecraft:skeleton×4854, minecraft:spider×4788, minecraft:zombie×4641, minecraft:drowned×4533, minecraft:sheep×3547, minecraft:chicken×3453, minecraft:cow×3371, minecraft:pig×3220, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/IUUbGytuGb
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **10**)
- total pause: **23877.5 ms**, avg **198.98 ms**, max **2589.5 ms**
- heap high-water seen: **7685 MB** -> last-after: **5662 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117444)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28946 | 24.6% |
| kernel: other | 27973 | 23.8% |
| other | 15243 | 13.0% |
| moonrise/paper patches | 10338 | 8.8% |
| chunk system (kernel) | 9428 | 8.0% |
| fastutil collections | 7020 | 6.0% |
| JDK collections | 6268 | 5.3% |
| JIT stubs (vtable/itable) | 3372 | 2.9% |
| network (kernel) | 3296 | 2.8% |
| JDK invokes/VarHandle | 2344 | 2.0% |
| JDK other | 2135 | 1.8% |
| JVM internals (GC oop barriers) | 547 | 0.5% |
| vdso (clock) | 227 | 0.2% |
| redstone (kernel) | 89 | 0.1% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| bukkit api | 75 | 0.1% |
| craftbukkit glue | 38 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93753 | 79.8% |
| phase: unclassified | 13930 | 11.9% |
| phase: main tick (unclassified) | 3624 | 3.1% |
| phase: chunk tick | 2087 | 1.8% |
| phase: network sync (ServerEntity) | 1755 | 1.5% |
| phase: chunk system (off-main worker) | 1109 | 0.9% |
| phase: block entities (hoppers/furnaces) | 675 | 0.6% |
| phase: random tick | 396 | 0.3% |
| phase: mob spawning | 110 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101662** (86.6%) · native/JVM-internal **15693** (13.4%) · other **89** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4468 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3538 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2871 | 2.4% |
| `vtable stub` | native/JVM-internal | 2755 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2014 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1861 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1721 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1630 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1525 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1492 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1490 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1432 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1359 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1344 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fa2cd9e66e0.accept` | JVM-Java | 1343 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1261 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1185 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1046 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1012 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 955 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 941 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 938 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 873 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 870 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 864 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 773 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 771 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 722 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 708 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 697 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 668 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 638 | 0.5% |
| `itable stub` | native/JVM-internal | 613 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 605 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57897 | 94.5% |
| entities/mobs (kernel) | 1027 | 1.7% |
| kernel: other | 951 | 1.6% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 267 | 0.4% |
| JDK collections | 204 | 0.3% |
| fastutil collections | 195 | 0.3% |
| network (kernel) | 115 | 0.2% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| JDK other | 75 | 0.1% |
| JDK invokes/VarHandle | 60 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57692 | 94.2% |
| phase: entity tick (AI/movement) | 3149 | 5.1% |
| phase: main tick (unclassified) | 186 | 0.3% |
| phase: chunk tick | 72 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52367** (85.5%) · native/JVM-internal **8876** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49017 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 122 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 121 | 0.2% |
| `syscall` | native/JVM-internal | 102 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `vtable stub` | native/JVM-internal | 90 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fa2cd9e66e0.accept` | JVM-Java | 55 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10423)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10423 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8001 | 76.8% |
| phase: entity tick (AI/movement) | 2137 | 20.5% |
| phase: chunk system (off-main worker) | 138 | 1.3% |
| phase: main tick (unclassified) | 103 | 1.0% |
| phase: network sync (ServerEntity) | 18 | 0.2% |
| phase: block entities (hoppers/furnaces) | 11 | 0.1% |
| phase: chunk tick | 6 | 0.1% |
| phase: mob spawning | 5 | 0.0% |
| phase: random tick | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10423** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1860 | 17.8% |
| `byte[]_[k]` | other | 711 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 549 | 5.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 533 | 5.1% |
| `short[]_[i]` | other | 520 | 5.0% |
| `long[]_[k]` | other | 485 | 4.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 474 | 4.5% |
| `java.lang.Object[]_[i]` | other | 467 | 4.5% |
| `char[]_[k]` | other | 450 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 421 | 4.0% |
| `byte[]_[i]` | other | 369 | 3.5% |
| `java.lang.String_[i]` | other | 206 | 2.0% |
| `long[]_[i]` | other | 188 | 1.8% |
| `java.lang.Object[]_[k]` | other | 158 | 1.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 150 | 1.4% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 147 | 1.4% |
| `java.util.ArrayList_[i]` | other | 146 | 1.4% |
| `java.util.Optional_[i]` | other | 127 | 1.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 126 | 1.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 89 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117444 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35208 | 29.98% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22546 | 19.20% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6598 | 5.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5443 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4494 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1168 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 845 | 0.72% |
| `net/minecraft/world/entity/npc/Villager.tick` | 427 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 240 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 213 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 180 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1860 | 17.8% |
| `byte[]_[k]` | 711 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 549 | 5.3% |
| `net.minecraft.world.phys.AABB_[i]` | 533 | 5.1% |
| `short[]_[i]` | 520 | 5.0% |
| `long[]_[k]` | 485 | 4.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | 474 | 4.5% |
| `java.lang.Object[]_[i]` | 467 | 4.5% |
| `char[]_[k]` | 450 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 421 | 4.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 23878 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148099..151223 (delta 3124, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99631->103164, minecraft:drowned 3451->4533, minecraft:zombie 3622->4641, minecraft:husk 4524->5168, minecraft:creeper 4578->5188, minecraft:spider 4226->4788, minecraft:skeleton 4436->4854, minecraft:pig 3201->3220
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3124)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57193662 B)
- `wall-collapsed.txt` (3780817 B)
- `alloc-collapsed.txt` (3779025 B)
- `cpu-flamegraph.html` (305939 B)
- `server-stdout.log` (268986 B)
- `gc.log` (114484 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
