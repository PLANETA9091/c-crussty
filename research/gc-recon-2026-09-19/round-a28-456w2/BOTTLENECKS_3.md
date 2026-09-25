# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.206 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 2.0, 2.1, 2.4, 2.8, 2.9]
- spark tick-monitor MSPT: avg **355.14ms** / min 311.31ms / max **434.82ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T09:16:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8746488 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 311.31 | — | — | — | 434.82 | 355.14 |

- entity totals seen: [149185, 150811, 151012]
- top entity types (max seen): minecraft:item×103071, minecraft:husk×5227, minecraft:creeper×5214, minecraft:skeleton×4858, minecraft:spider×4797, minecraft:zombie×4617, minecraft:drowned×4501, minecraft:sheep×3542, minecraft:chicken×3432, minecraft:cow×3328, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/amOj9mxoXi
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **133** (Full GC: **10**)
- total pause: **22605.7 ms**, avg **169.97 ms**, max **2205.9 ms**
- heap high-water seen: **7770 MB** -> last-after: **3704 MB**
  - Young (Allocation Failure): 113
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112238)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27031 | 24.1% |
| kernel: other | 25765 | 23.0% |
| other | 13261 | 11.8% |
| moonrise/paper patches | 10652 | 9.5% |
| chunk system (kernel) | 10255 | 9.1% |
| fastutil collections | 7203 | 6.4% |
| JDK collections | 6035 | 5.4% |
| network (kernel) | 3695 | 3.3% |
| JIT stubs (vtable/itable) | 2918 | 2.6% |
| JDK invokes/VarHandle | 2719 | 2.4% |
| JDK other | 1679 | 1.5% |
| JVM internals (GC oop barriers) | 481 | 0.4% |
| vdso (clock) | 210 | 0.2% |
| block entities/hoppers (kernel) | 104 | 0.1% |
| redstone (kernel) | 87 | 0.1% |
| bukkit api | 66 | 0.1% |
| craftbukkit glue | 43 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87369 | 77.8% |
| phase: unclassified | 13156 | 11.7% |
| phase: main tick (unclassified) | 4317 | 3.8% |
| phase: chunk tick | 2511 | 2.2% |
| phase: network sync (ServerEntity) | 2238 | 2.0% |
| phase: chunk system (off-main worker) | 1223 | 1.1% |
| phase: block entities (hoppers/furnaces) | 789 | 0.7% |
| phase: random tick | 494 | 0.4% |
| phase: mob spawning | 136 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97979** (87.3%) · native/JVM-internal **14163** (12.6%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5014 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3412 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2603 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2409 | 2.1% |
| `vtable stub` | native/JVM-internal | 2317 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2284 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2175 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1844 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1833 | 1.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1691 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1604 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1491 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1344 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1335 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1272 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1238 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1226 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1118 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1104 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1055 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1019 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1004 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 995 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 993 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 992 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 959 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 931 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 846 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 768 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 726 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 721 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 718 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 708 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 695 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 683 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57974 | 94.6% |
| entities/mobs (kernel) | 937 | 1.5% |
| kernel: other | 882 | 1.4% |
| moonrise/paper patches | 363 | 0.6% |
| chunk system (kernel) | 299 | 0.5% |
| fastutil collections | 248 | 0.4% |
| JDK collections | 160 | 0.3% |
| network (kernel) | 127 | 0.2% |
| JIT stubs (vtable/itable) | 107 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 49 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57868 | 94.5% |
| phase: entity tick (AI/movement) | 2888 | 4.7% |
| phase: main tick (unclassified) | 218 | 0.4% |
| phase: chunk tick | 120 | 0.2% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52426** (85.6%) · native/JVM-internal **8825** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49197 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 102 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 93 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 89 | 0.1% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 64 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4087)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4087 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2400 | 58.7% |
| phase: unclassified | 1495 | 36.6% |
| phase: main tick (unclassified) | 82 | 2.0% |
| phase: chunk system (off-main worker) | 51 | 1.2% |
| phase: network sync (ServerEntity) | 24 | 0.6% |
| phase: block entities (hoppers/furnaces) | 17 | 0.4% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 6 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4087** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 652 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 556 | 13.6% |
| `char[]_[k]` | other | 441 | 10.8% |
| `byte[]_[k]` | other | 244 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 200 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 156 | 3.8% |
| `java.util.ArrayList_[i]` | other | 144 | 3.5% |
| `long[]_[i]` | other | 137 | 3.4% |
| `java.lang.Object[]_[i]` | other | 114 | 2.8% |
| `byte[]_[i]` | other | 99 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 84 | 2.1% |
| `int[]_[i]` | other | 73 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 54 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 51 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc08ba4c300_[i]` | other | 41 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112238 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32184 | 28.67% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21415 | 19.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6158 | 5.49% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5271 | 4.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4195 | 3.74% |
| `net/minecraft/world/entity/ai/Brain.tick` | 910 | 0.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 876 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 391 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 234 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 212 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 205 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 203 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 652 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 556 | 13.6% |
| `char[]_[k]` | 441 | 10.8% |
| `byte[]_[k]` | 244 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 200 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 156 | 3.8% |
| `java.util.ArrayList_[i]` | 144 | 3.5% |
| `long[]_[i]` | 137 | 3.4% |
| `java.lang.Object[]_[i]` | 114 | 2.8% |
| `byte[]_[i]` | 99 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 133 pauses / total 22606 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148026..151012 (delta 2986, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99535->103071, minecraft:drowned 3530->4501, minecraft:zombie 3716->4617, minecraft:husk 4580->5227, minecraft:creeper 4596->5214, minecraft:spider 4254->4797, minecraft:skeleton 4417->4858, minecraft:chicken 3404->3432
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2986)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49084195 B)
- `wall-collapsed.txt` (3306537 B)
- `alloc-collapsed.txt` (2161379 B)
- `cpu-flamegraph.html` (260480 B)
- `server-stdout.log` (241258 B)
- `gc.log` (125655 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
