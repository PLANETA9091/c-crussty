# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.116 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 2.0, 2.4, 2.7, 2.8]
- spark tick-monitor MSPT: avg **378.77ms** / min 321.6ms / max **491.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:52:09Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8960455 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 321.6 | — | — | — | 491.5 | 378.77 |

- entity totals seen: [148516, 150267, 150864]
- top entity types (max seen): minecraft:item×102982, minecraft:creeper×5215, minecraft:husk×5189, minecraft:skeleton×4863, minecraft:spider×4772, minecraft:zombie×4615, minecraft:drowned×4508, minecraft:sheep×3541, minecraft:chicken×3440, minecraft:cow×3325, minecraft:pig×3239, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/GiRQbIV1Qw
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **128** (Full GC: **10**)
- total pause: **23548.9 ms**, avg **183.98 ms**, max **2276.7 ms**
- heap high-water seen: **7712 MB** -> last-after: **3686 MB**
  - Young (Allocation Failure): 108
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112915)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27737 | 24.6% |
| kernel: other | 24560 | 21.8% |
| other | 14104 | 12.5% |
| moonrise/paper patches | 10627 | 9.4% |
| chunk system (kernel) | 10273 | 9.1% |
| fastutil collections | 7201 | 6.4% |
| JDK collections | 6121 | 5.4% |
| network (kernel) | 3978 | 3.5% |
| JIT stubs (vtable/itable) | 2675 | 2.4% |
| JDK invokes/VarHandle | 2555 | 2.3% |
| JDK other | 1974 | 1.7% |
| JVM internals (GC oop barriers) | 526 | 0.5% |
| vdso (clock) | 236 | 0.2% |
| block entities/hoppers (kernel) | 121 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| bukkit api | 60 | 0.1% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87191 | 77.2% |
| phase: unclassified | 14169 | 12.5% |
| phase: main tick (unclassified) | 4161 | 3.7% |
| phase: chunk tick | 2531 | 2.2% |
| phase: network sync (ServerEntity) | 2190 | 1.9% |
| phase: chunk system (off-main worker) | 1230 | 1.1% |
| phase: block entities (hoppers/furnaces) | 812 | 0.7% |
| phase: random tick | 487 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97784** (86.6%) · native/JVM-internal **15049** (13.3%) · other **82** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5031 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3749 | 3.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2609 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2566 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2383 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2242 | 2.0% |
| `vtable stub` | native/JVM-internal | 2211 | 2.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 2036 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1911 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1723 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1517 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1514 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1482 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1293 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1246 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1231 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1158 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1154 | 1.0% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1113 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1110 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1069 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1053 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 938 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 936 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 922 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 916 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 809 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 800 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 751 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 745 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 740 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 725 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 694 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 691 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61212)

| bucket | self-time samples | share |
|---|---|---|
| other | 57977 | 94.7% |
| entities/mobs (kernel) | 917 | 1.5% |
| kernel: other | 836 | 1.4% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 324 | 0.5% |
| fastutil collections | 240 | 0.4% |
| JDK collections | 215 | 0.4% |
| network (kernel) | 105 | 0.2% |
| JIT stubs (vtable/itable) | 94 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 64 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57829 | 94.5% |
| phase: entity tick (AI/movement) | 2892 | 4.7% |
| phase: main tick (unclassified) | 219 | 0.4% |
| phase: chunk tick | 108 | 0.2% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52353** (85.5%) · native/JVM-internal **8856** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49164 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.8% |
| `read` | native/JVM-internal | 1223 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 106 | 0.2% |
| `syscall` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.2% |
| `vtable stub` | native/JVM-internal | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 76 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 61 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3975)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3975 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2292 | 57.7% |
| phase: unclassified | 1504 | 37.8% |
| phase: main tick (unclassified) | 88 | 2.2% |
| phase: chunk system (off-main worker) | 47 | 1.2% |
| phase: network sync (ServerEntity) | 17 | 0.4% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 10 | 0.3% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3975** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 594 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 538 | 13.5% |
| `char[]_[k]` | other | 441 | 11.1% |
| `byte[]_[k]` | other | 230 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 197 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 4.1% |
| `long[]_[i]` | other | 150 | 3.8% |
| `java.util.ArrayList_[i]` | other | 135 | 3.4% |
| `java.lang.Object[]_[i]` | other | 113 | 2.8% |
| `byte[]_[i]` | other | 97 | 2.4% |
| `int[]_[i]` | other | 83 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 1.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 43 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 42 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 40 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 39 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007faaa982ece8_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112915 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32029 | 28.37% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21366 | 18.92% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6092 | 5.40% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5070 | 4.49% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4388 | 3.89% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 825 | 0.73% |
| `net/minecraft/world/entity/ai/Brain.tick` | 824 | 0.73% |
| `net/minecraft/world/entity/npc/Villager.tick` | 452 | 0.40% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 196 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 188 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 173 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 594 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 538 | 13.5% |
| `char[]_[k]` | 441 | 11.1% |
| `byte[]_[k]` | 230 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 197 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 4.1% |
| `long[]_[i]` | 150 | 3.8% |
| `java.util.ArrayList_[i]` | 135 | 3.4% |
| `java.lang.Object[]_[i]` | 113 | 2.8% |
| `byte[]_[i]` | 97 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 128 pauses / total 23549 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147771..150864 (delta 3093, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99444->102982, minecraft:drowned 3501->4508, minecraft:zombie 3627->4615, minecraft:creeper 4559->5215, minecraft:husk 4538->5189, minecraft:spider 4204->4772, minecraft:skeleton 4469->4863, minecraft:chicken 3407->3440
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3093)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48625918 B)
- `wall-collapsed.txt` (3373833 B)
- `alloc-collapsed.txt` (2175757 B)
- `cpu-flamegraph.html` (267371 B)
- `server-stdout.log` (244002 B)
- `gc.log` (121353 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
