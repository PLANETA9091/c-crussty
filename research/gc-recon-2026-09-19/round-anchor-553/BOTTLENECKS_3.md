# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.441 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.3, 1.6, 1.9, 1.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **441.5ms** / min 378.51ms / max **677.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-26T01:06:36Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6097651 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 378.51 | — | — | — | 677.83 | 441.5 |

- entity totals seen: [148991, 150129, 151193]
- top entity types (max seen): minecraft:item×103177, minecraft:creeper×5219, minecraft:husk×5146, minecraft:skeleton×4867, minecraft:spider×4803, minecraft:zombie×4683, minecraft:drowned×4558, minecraft:sheep×3502, minecraft:chicken×3428, minecraft:cow×3399, minecraft:pig×3260, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/siw0WIFNkG
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **21119.5 ms**, avg **178.98 ms**, max **2470.3 ms**
- heap high-water seen: **7440 MB** -> last-after: **4155 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116496)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28547 | 24.5% |
| kernel: other | 27773 | 23.8% |
| other | 15417 | 13.2% |
| moonrise/paper patches | 10050 | 8.6% |
| chunk system (kernel) | 9760 | 8.4% |
| fastutil collections | 6611 | 5.7% |
| JDK collections | 5925 | 5.1% |
| JIT stubs (vtable/itable) | 3653 | 3.1% |
| network (kernel) | 3223 | 2.8% |
| JDK invokes/VarHandle | 2507 | 2.2% |
| JDK other | 1925 | 1.7% |
| JVM internals (GC oop barriers) | 554 | 0.5% |
| vdso (clock) | 267 | 0.2% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 41 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92359 | 79.3% |
| phase: unclassified | 14040 | 12.1% |
| phase: main tick (unclassified) | 3777 | 3.2% |
| phase: chunk tick | 2255 | 1.9% |
| phase: network sync (ServerEntity) | 1750 | 1.5% |
| phase: chunk system (off-main worker) | 1130 | 1.0% |
| phase: block entities (hoppers/furnaces) | 656 | 0.6% |
| phase: random tick | 397 | 0.3% |
| phase: mob spawning | 130 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100116** (85.9%) · native/JVM-internal **16282** (14.0%) · other **98** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4554 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3212 | 2.8% |
| `vtable stub` | native/JVM-internal | 2928 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2580 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2040 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1893 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1659 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1580 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1537 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1512 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1437 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1436 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1406 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1361 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1254 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1184 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1129 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 980 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 953 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 933 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 921 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 905 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 883 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 881 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 860 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 836 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 738 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 731 | 0.6% |
| `itable stub` | native/JVM-internal | 722 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 685 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 681 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 678 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 678 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 655 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61237)

| bucket | self-time samples | share |
|---|---|---|
| other | 57898 | 94.5% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 884 | 1.4% |
| moonrise/paper patches | 360 | 0.6% |
| chunk system (kernel) | 281 | 0.5% |
| fastutil collections | 195 | 0.3% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 151 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 61 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57690 | 94.2% |
| phase: entity tick (AI/movement) | 3072 | 5.0% |
| phase: main tick (unclassified) | 220 | 0.4% |
| phase: chunk tick | 94 | 0.2% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52343** (85.5%) · native/JVM-internal **8890** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49055 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4756 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 126 | 0.2% |
| `syscall` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 55 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3408)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3408 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2008 | 58.9% |
| phase: unclassified | 1253 | 36.8% |
| phase: main tick (unclassified) | 86 | 2.5% |
| phase: chunk system (off-main worker) | 26 | 0.8% |
| phase: network sync (ServerEntity) | 14 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3408** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 568 | 16.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 535 | 15.7% |
| `char[]_[k]` | other | 441 | 12.9% |
| `byte[]_[k]` | other | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 168 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 4.0% |
| `long[]_[i]` | other | 123 | 3.6% |
| `byte[]_[i]` | other | 97 | 2.8% |
| `java.util.ArrayList_[i]` | other | 89 | 2.6% |
| `java.lang.Object[]_[i]` | other | 82 | 2.4% |
| `int[]_[i]` | other | 58 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 43 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f5ffa9e7ab0_[i]` | other | 29 | 0.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 25 | 0.7% |
| `java.lang.String_[i]` | other | 23 | 0.7% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f5ffa9eb8e8_[i]` | other | 22 | 0.6% |
| `int[]_[k]` | other | 20 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116496 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34544 | 29.65% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22284 | 19.13% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6432 | 5.52% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5388 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4525 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1106 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 898 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 404 | 0.35% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 253 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 246 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 210 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 568 | 16.7% |
| `net.minecraft.world.phys.AABB_[i]` | 535 | 15.7% |
| `char[]_[k]` | 441 | 12.9% |
| `byte[]_[k]` | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 168 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 137 | 4.0% |
| `long[]_[i]` | 123 | 3.6% |
| `byte[]_[i]` | 97 | 2.8% |
| `java.util.ArrayList_[i]` | 89 | 2.6% |
| `java.lang.Object[]_[i]` | 82 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 21120 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148061..151193 (delta 3132, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99495->103177, minecraft:drowned 3507->4558, minecraft:zombie 3654->4683, minecraft:creeper 4556->5219, minecraft:husk 4510->5146, minecraft:spider 4226->4803, minecraft:skeleton 4379->4867, minecraft:chicken 3398->3428
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3132)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56269404 B)
- `wall-collapsed.txt` (3740797 B)
- `alloc-collapsed.txt` (1991118 B)
- `cpu-flamegraph.html` (313590 B)
- `server-stdout.log` (252092 B)
- `gc.log` (111842 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
