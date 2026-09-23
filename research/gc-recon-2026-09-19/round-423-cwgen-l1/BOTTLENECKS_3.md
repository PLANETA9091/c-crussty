# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.39 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.7, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **413.15ms** / min 341.2ms / max **539.02ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T04:27:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 5804616 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 341.2 | — | — | — | 539.02 | 413.15 |

- entity totals seen: [149026, 150133, 151388]
- top entity types (max seen): minecraft:item×103289, minecraft:husk×5234, minecraft:creeper×5134, minecraft:skeleton×4865, minecraft:spider×4838, minecraft:zombie×4617, minecraft:drowned×4548, minecraft:sheep×3524, minecraft:chicken×3417, minecraft:cow×3366, minecraft:pig×3223, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/GzwNRd2rXY
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **10**)
- total pause: **25125.6 ms**, avg **197.84 ms**, max **2459.0 ms**
- heap high-water seen: **7640 MB** -> last-after: **5681 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117591)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28673 | 24.4% |
| kernel: other | 27270 | 23.2% |
| other | 15812 | 13.4% |
| moonrise/paper patches | 10243 | 8.7% |
| chunk system (kernel) | 10213 | 8.7% |
| fastutil collections | 7087 | 6.0% |
| JDK collections | 5904 | 5.0% |
| JIT stubs (vtable/itable) | 3559 | 3.0% |
| network (kernel) | 3178 | 2.7% |
| JDK invokes/VarHandle | 2540 | 2.2% |
| JDK other | 1989 | 1.7% |
| JVM internals (GC oop barriers) | 591 | 0.5% |
| vdso (clock) | 217 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94022 | 80.0% |
| phase: unclassified | 13935 | 11.9% |
| phase: main tick (unclassified) | 3190 | 2.7% |
| phase: chunk tick | 2096 | 1.8% |
| phase: network sync (ServerEntity) | 1680 | 1.4% |
| phase: chunk system (off-main worker) | 1508 | 1.3% |
| phase: block entities (hoppers/furnaces) | 659 | 0.6% |
| phase: random tick | 374 | 0.3% |
| phase: mob spawning | 127 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101387** (86.2%) · native/JVM-internal **16093** (13.7%) · other **111** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5079 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3166 | 2.7% |
| `vtable stub` | native/JVM-internal | 2962 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2570 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1932 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1760 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1723 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1701 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1654 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1605 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1515 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1509 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1438 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1372 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1292 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1163 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1093 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1064 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1015 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1001 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 997 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 961 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 912 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 842 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 838 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 824 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 772 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 759 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 750 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 716 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 687 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 683 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 651 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 631 | 0.5% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 629 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 63656)

| bucket | self-time samples | share |
|---|---|---|
| other | 60255 | 94.7% |
| entities/mobs (kernel) | 1037 | 1.6% |
| kernel: other | 857 | 1.3% |
| chunk system (kernel) | 332 | 0.5% |
| moonrise/paper patches | 321 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 150 | 0.2% |
| network (kernel) | 109 | 0.2% |
| JDK other | 83 | 0.1% |
| JDK invokes/VarHandle | 71 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60038 | 94.3% |
| phase: entity tick (AI/movement) | 3186 | 5.0% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 10 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54760** (86.0%) · native/JVM-internal **8892** (14.0%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51397 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.5% |
| `read` | native/JVM-internal | 1233 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 154 | 0.2% |
| `vtable stub` | native/JVM-internal | 127 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 43 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3915)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3915 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2179 | 55.7% |
| phase: unclassified | 1566 | 40.0% |
| phase: main tick (unclassified) | 92 | 2.3% |
| phase: chunk system (off-main worker) | 31 | 0.8% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 10 | 0.3% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3915** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 581 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 548 | 14.0% |
| `char[]_[k]` | other | 449 | 11.5% |
| `byte[]_[k]` | other | 230 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.5% |
| `long[]_[i]` | other | 145 | 3.7% |
| `java.util.ArrayList_[i]` | other | 143 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 3.5% |
| `java.lang.Object[]_[i]` | other | 99 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 90 | 2.3% |
| `byte[]_[i]` | other | 79 | 2.0% |
| `int[]_[i]` | other | 72 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 44 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 39 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 38 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f3e1190a000_[i]` | other | 34 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 34 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117591 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34993 | 29.76% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23351 | 19.86% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6387 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5447 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4342 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1111 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 927 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 423 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 227 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 224 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 196 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 581 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 548 | 14.0% |
| `char[]_[k]` | 449 | 11.5% |
| `byte[]_[k]` | 230 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.5% |
| `long[]_[i]` | 145 | 3.7% |
| `java.util.ArrayList_[i]` | 143 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 137 | 3.5% |
| `java.lang.Object[]_[i]` | 99 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | 90 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 25126 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148156..151388 (delta 3232, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99689->103289, minecraft:drowned 3484->4548, minecraft:zombie 3618->4617, minecraft:husk 4543->5234, minecraft:creeper 4546->5134, minecraft:spider 4254->4838, minecraft:skeleton 4383->4865, minecraft:chicken 3394->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3232)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (60962465 B)
- `wall-collapsed.txt` (3759493 B)
- `alloc-collapsed.txt` (2125430 B)
- `cpu-flamegraph.html` (303296 B)
- `server-stdout.log` (260084 B)
- `gc.log` (120555 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
