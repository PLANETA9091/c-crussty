# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.006 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 1.7, 1.9, 1.1, 2.4, 2.6]
- spark tick-monitor MSPT: avg **405.98ms** / min 348.31ms / max **511.05ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T23:43:37Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6762251 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.31 | — | — | — | 511.05 | 405.98 |

- entity totals seen: [148990, 150227, 151365]
- top entity types (max seen): minecraft:item×103292, minecraft:creeper×5238, minecraft:husk×5151, minecraft:skeleton×4874, minecraft:spider×4816, minecraft:zombie×4678, minecraft:drowned×4552, minecraft:sheep×3499, minecraft:chicken×3432, minecraft:cow×3383, minecraft:pig×3252, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/7rmsFKYODM
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **23353.9 ms**, avg **193.01 ms**, max **2563.8 ms**
- heap high-water seen: **7395 MB** -> last-after: **4105 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115857)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27147 | 23.4% |
| entities/mobs (kernel) | 26784 | 23.1% |
| other | 15929 | 13.7% |
| chunk system (kernel) | 10660 | 9.2% |
| moonrise/paper patches | 10325 | 8.9% |
| fastutil collections | 7142 | 6.2% |
| JDK collections | 5841 | 5.0% |
| network (kernel) | 3375 | 2.9% |
| JIT stubs (vtable/itable) | 2927 | 2.5% |
| JDK invokes/VarHandle | 2651 | 2.3% |
| JDK other | 1990 | 1.7% |
| JVM internals (GC oop barriers) | 578 | 0.5% |
| vdso (clock) | 221 | 0.2% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| bukkit api | 62 | 0.1% |
| craftbukkit glue | 60 | 0.1% |
| redstone (kernel) | 36 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90007 | 77.7% |
| phase: unclassified | 15315 | 13.2% |
| phase: main tick (unclassified) | 3777 | 3.3% |
| phase: chunk tick | 2179 | 1.9% |
| phase: network sync (ServerEntity) | 2046 | 1.8% |
| phase: chunk system (off-main worker) | 1209 | 1.0% |
| phase: block entities (hoppers/furnaces) | 750 | 0.6% |
| phase: random tick | 437 | 0.4% |
| phase: mob spawning | 134 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99454** (85.8%) · native/JVM-internal **16323** (14.1%) · other **80** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5121 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3532 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2806 | 2.4% |
| `vtable stub` | native/JVM-internal | 2362 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2243 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2126 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1835 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1752 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1742 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1681 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1490 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1441 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1435 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1341 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1340 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1194 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1186 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1168 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1102 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1087 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1052 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1036 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1023 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1011 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 971 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 925 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 893 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 893 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 885 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 798 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 751 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 743 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 733 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 680 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 669 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 646 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61355)

| bucket | self-time samples | share |
|---|---|---|
| other | 58094 | 94.7% |
| entities/mobs (kernel) | 928 | 1.5% |
| kernel: other | 869 | 1.4% |
| moonrise/paper patches | 347 | 0.6% |
| chunk system (kernel) | 305 | 0.5% |
| fastutil collections | 248 | 0.4% |
| JDK collections | 185 | 0.3% |
| network (kernel) | 115 | 0.2% |
| JIT stubs (vtable/itable) | 93 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57872 | 94.3% |
| phase: entity tick (AI/movement) | 3033 | 4.9% |
| phase: main tick (unclassified) | 205 | 0.3% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52378** (85.4%) · native/JVM-internal **8966** (14.6%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49123 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4755 | 7.7% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1204 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `syscall` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 114 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `vtable stub` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3698)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3698 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2094 | 56.6% |
| phase: unclassified | 1439 | 38.9% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: chunk tick | 8 | 0.2% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3698** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 569 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 541 | 14.6% |
| `char[]_[k]` | other | 447 | 12.1% |
| `byte[]_[k]` | other | 197 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 172 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 4.1% |
| `long[]_[i]` | other | 127 | 3.4% |
| `java.util.ArrayList_[i]` | other | 115 | 3.1% |
| `java.lang.Object[]_[i]` | other | 108 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.4% |
| `byte[]_[i]` | other | 81 | 2.2% |
| `int[]_[i]` | other | 62 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f17be9ef960_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 28 | 0.8% |
| `java.lang.String_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115857 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33546 | 28.95% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22021 | 19.01% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6272 | 5.41% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5265 | 4.54% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4420 | 3.82% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 950 | 0.82% |
| `net/minecraft/world/entity/ai/Brain.tick` | 875 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 399 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 203 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 569 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 541 | 14.6% |
| `char[]_[k]` | 447 | 12.1% |
| `byte[]_[k]` | 197 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 172 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 4.1% |
| `long[]_[i]` | 127 | 3.4% |
| `java.util.ArrayList_[i]` | 115 | 3.1% |
| `java.lang.Object[]_[i]` | 108 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | 89 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 23354 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148065..151365 (delta 3300, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99556->103292, minecraft:zombie 3648->4678, minecraft:drowned 3539->4552, minecraft:creeper 4538->5238, minecraft:husk 4506->5151, minecraft:spider 4231->4816, minecraft:skeleton 4341->4874, minecraft:chicken 3397->3432
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3300)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52906297 B)
- `wall-collapsed.txt` (3549937 B)
- `alloc-collapsed.txt` (2063531 B)
- `cpu-flamegraph.html` (290518 B)
- `server-stdout.log` (251954 B)
- `gc.log` (114433 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
