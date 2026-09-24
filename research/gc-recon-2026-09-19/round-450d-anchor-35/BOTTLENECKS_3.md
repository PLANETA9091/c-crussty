# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.352 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.7, 1.1, 2.2, 2.6, 2.7]
- spark tick-monitor MSPT: avg **406.14ms** / min 335.95ms / max **540.27ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:49:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6499754 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 335.95 | — | — | — | 540.27 | 406.14 |

- entity totals seen: [148992, 150409, 151356]
- top entity types (max seen): minecraft:item×103286, minecraft:creeper×5254, minecraft:husk×5159, minecraft:skeleton×4864, minecraft:spider×4825, minecraft:zombie×4633, minecraft:drowned×4539, minecraft:sheep×3524, minecraft:chicken×3411, minecraft:cow×3373, minecraft:pig×3231, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/SjEf4NPSfB
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **10**)
- total pause: **24037.0 ms**, avg **192.30 ms**, max **2377.1 ms**
- heap high-water seen: **7607 MB** -> last-after: **5876 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116889)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29123 | 24.9% |
| kernel: other | 28212 | 24.1% |
| other | 14957 | 12.8% |
| chunk system (kernel) | 9748 | 8.3% |
| moonrise/paper patches | 9244 | 7.9% |
| fastutil collections | 7286 | 6.2% |
| JDK collections | 6207 | 5.3% |
| JIT stubs (vtable/itable) | 3678 | 3.1% |
| network (kernel) | 3180 | 2.7% |
| JDK invokes/VarHandle | 2129 | 1.8% |
| JDK other | 2058 | 1.8% |
| JVM internals (GC oop barriers) | 535 | 0.5% |
| vdso (clock) | 246 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| bukkit api | 78 | 0.1% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93469 | 80.0% |
| phase: unclassified | 13793 | 11.8% |
| phase: main tick (unclassified) | 3641 | 3.1% |
| phase: chunk tick | 1918 | 1.6% |
| phase: network sync (ServerEntity) | 1792 | 1.5% |
| phase: chunk system (off-main worker) | 1061 | 0.9% |
| phase: block entities (hoppers/furnaces) | 679 | 0.6% |
| phase: random tick | 415 | 0.4% |
| phase: mob spawning | 117 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100918** (86.3%) · native/JVM-internal **15830** (13.5%) · other **141** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4510 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3521 | 3.0% |
| `vtable stub` | native/JVM-internal | 3016 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2796 | 2.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1792 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1746 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1745 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1669 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1607 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1589 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1484 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1463 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1429 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1348 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1281 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1256 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1076 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1067 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1006 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 984 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 933 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 901 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 899 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 850 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 838 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 815 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 767 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 739 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 713 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 685 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 680 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 661 | 0.6% |
| `itable stub` | native/JVM-internal | 658 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 649 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57878 | 94.5% |
| entities/mobs (kernel) | 969 | 1.6% |
| kernel: other | 899 | 1.5% |
| moonrise/paper patches | 350 | 0.6% |
| chunk system (kernel) | 297 | 0.5% |
| fastutil collections | 229 | 0.4% |
| JDK collections | 213 | 0.3% |
| JIT stubs (vtable/itable) | 146 | 0.2% |
| network (kernel) | 120 | 0.2% |
| JDK invokes/VarHandle | 77 | 0.1% |
| JDK other | 59 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57670 | 94.1% |
| phase: entity tick (AI/movement) | 3141 | 5.1% |
| phase: main tick (unclassified) | 196 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 56 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52309** (85.4%) · native/JVM-internal **8944** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48963 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `vtable stub` | native/JVM-internal | 122 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 56 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10158)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10158 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 7797 | 76.8% |
| phase: entity tick (AI/movement) | 2126 | 20.9% |
| phase: chunk system (off-main worker) | 105 | 1.0% |
| phase: main tick (unclassified) | 91 | 0.9% |
| phase: network sync (ServerEntity) | 14 | 0.1% |
| phase: block entities (hoppers/furnaces) | 10 | 0.1% |
| phase: chunk tick | 8 | 0.1% |
| phase: mob spawning | 6 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10158** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1746 | 17.2% |
| `byte[]_[k]` | other | 737 | 7.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 546 | 5.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 536 | 5.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 517 | 5.1% |
| `java.lang.Object[]_[i]` | other | 492 | 4.8% |
| `long[]_[k]` | other | 470 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 469 | 4.6% |
| `short[]_[i]` | other | 465 | 4.6% |
| `char[]_[k]` | other | 449 | 4.4% |
| `byte[]_[i]` | other | 399 | 3.9% |
| `java.lang.Object[]_[k]` | other | 203 | 2.0% |
| `java.lang.String_[i]` | other | 177 | 1.7% |
| `java.util.Optional_[i]` | other | 173 | 1.7% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 159 | 1.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 154 | 1.5% |
| `long[]_[i]` | other | 148 | 1.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 139 | 1.4% |
| `java.util.ArrayList_[i]` | other | 131 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 87 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116889 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34699 | 29.69% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22503 | 19.25% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6559 | 5.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5458 | 4.67% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4474 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1154 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 915 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 421 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1746 | 17.2% |
| `byte[]_[k]` | 737 | 7.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 546 | 5.4% |
| `net.minecraft.world.phys.AABB_[i]` | 536 | 5.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | 517 | 5.1% |
| `java.lang.Object[]_[i]` | 492 | 4.8% |
| `long[]_[k]` | 470 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | 469 | 4.6% |
| `short[]_[i]` | 465 | 4.6% |
| `char[]_[k]` | 449 | 4.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 24037 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148157..151356 (delta 3199, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99694->103286, minecraft:zombie 3613->4633, minecraft:drowned 3526->4539, minecraft:creeper 4583->5254, minecraft:husk 4512->5159, minecraft:spider 4247->4825, minecraft:skeleton 4456->4864, minecraft:chicken 3381->3411
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3199)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58890396 B)
- `wall-collapsed.txt` (3781424 B)
- `alloc-collapsed.txt` (4097121 B)
- `cpu-flamegraph.html` (308294 B)
- `server-stdout.log` (259087 B)
- `gc.log` (118790 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
