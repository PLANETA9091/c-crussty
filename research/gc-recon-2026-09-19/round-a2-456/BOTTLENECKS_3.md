# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.015 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.6, 2.0, 2.2, 2.5, 2.5]
- spark tick-monitor MSPT: avg **423.9ms** / min 357.25ms / max **584.13ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:23:04Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7003062 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 357.25 | — | — | — | 584.13 | 423.9 |

- entity totals seen: [149004, 150133, 151347]
- top entity types (max seen): minecraft:item×103252, minecraft:creeper×5206, minecraft:husk×5151, minecraft:skeleton×4866, minecraft:spider×4795, minecraft:zombie×4685, minecraft:drowned×4559, minecraft:sheep×3496, minecraft:chicken×3425, minecraft:cow×3392, minecraft:pig×3260, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/jikmhNaaaG
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **21349.4 ms**, avg **175.00 ms**, max **2611.4 ms**
- heap high-water seen: **7641 MB** -> last-after: **4384 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 117490)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28687 | 24.4% |
| entities/mobs (kernel) | 27901 | 23.7% |
| other | 15175 | 12.9% |
| moonrise/paper patches | 10195 | 8.7% |
| chunk system (kernel) | 9462 | 8.1% |
| fastutil collections | 7354 | 6.3% |
| JDK collections | 6343 | 5.4% |
| JIT stubs (vtable/itable) | 3479 | 3.0% |
| network (kernel) | 3115 | 2.7% |
| JDK invokes/VarHandle | 2712 | 2.3% |
| JDK other | 1991 | 1.7% |
| JVM internals (GC oop barriers) | 543 | 0.5% |
| vdso (clock) | 217 | 0.2% |
| bukkit api | 91 | 0.1% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93215 | 79.3% |
| phase: unclassified | 14460 | 12.3% |
| phase: main tick (unclassified) | 3704 | 3.2% |
| phase: chunk tick | 2036 | 1.7% |
| phase: network sync (ServerEntity) | 1776 | 1.5% |
| phase: chunk system (off-main worker) | 1117 | 1.0% |
| phase: block entities (hoppers/furnaces) | 644 | 0.5% |
| phase: random tick | 394 | 0.3% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101250** (86.2%) · native/JVM-internal **16148** (13.7%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4621 | 3.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3138 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3134 | 2.7% |
| `vtable stub` | native/JVM-internal | 2827 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2153 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2011 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1851 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1757 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1685 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1633 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1493 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1478 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1380 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1379 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1314 | 1.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1282 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1162 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1122 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1043 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1013 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 997 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 931 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 891 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 881 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 860 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 817 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 796 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 783 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 731 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 725 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 715 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 712 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 670 | 0.6% |
| `itable stub` | native/JVM-internal | 650 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 646 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 633 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57849 | 94.4% |
| entities/mobs (kernel) | 971 | 1.6% |
| kernel: other | 956 | 1.6% |
| moonrise/paper patches | 374 | 0.6% |
| chunk system (kernel) | 302 | 0.5% |
| fastutil collections | 238 | 0.4% |
| JDK collections | 201 | 0.3% |
| JIT stubs (vtable/itable) | 107 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK other | 80 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57672 | 94.2% |
| phase: entity tick (AI/movement) | 3138 | 5.1% |
| phase: main tick (unclassified) | 200 | 0.3% |
| phase: chunk tick | 77 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 10 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52365** (85.5%) · native/JVM-internal **8889** (14.5%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48981 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `syscall` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.2% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 45 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3785)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3785 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2152 | 56.9% |
| phase: unclassified | 1472 | 38.9% |
| phase: main tick (unclassified) | 93 | 2.5% |
| phase: chunk system (off-main worker) | 26 | 0.7% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3785** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 575 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 536 | 14.2% |
| `char[]_[k]` | other | 430 | 11.4% |
| `byte[]_[k]` | other | 216 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 212 | 5.6% |
| `long[]_[i]` | other | 128 | 3.4% |
| `java.util.ArrayList_[i]` | other | 126 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 124 | 3.3% |
| `java.lang.Object[]_[i]` | other | 93 | 2.5% |
| `byte[]_[i]` | other | 90 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.4% |
| `int[]_[i]` | other | 67 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 48 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fdce59de560_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fdce58296c8_[i]` | other | 30 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117490 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34304 | 29.20% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22767 | 19.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6420 | 5.46% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5488 | 4.67% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4605 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1100 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 902 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 446 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 241 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 224 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 218 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 194 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 575 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 536 | 14.2% |
| `char[]_[k]` | 430 | 11.4% |
| `byte[]_[k]` | 216 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 212 | 5.6% |
| `long[]_[i]` | 128 | 3.4% |
| `java.util.ArrayList_[i]` | 126 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 124 | 3.3% |
| `java.lang.Object[]_[i]` | 93 | 2.5% |
| `byte[]_[i]` | 90 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 21349 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148077..151347 (delta 3270, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99548->103252, minecraft:drowned 3477->4559, minecraft:zombie 3636->4685, minecraft:creeper 4535->5206, minecraft:husk 4503->5151, minecraft:spider 4236->4795, minecraft:skeleton 4425->4866, minecraft:chicken 3403->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3270)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57383607 B)
- `wall-collapsed.txt` (3791139 B)
- `alloc-collapsed.txt` (2098411 B)
- `cpu-flamegraph.html` (298461 B)
- `server-stdout.log` (263165 B)
- `gc.log` (115300 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
