# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.23 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 1.7, 2.0, 2.2, 2.5, 2.7]
- spark tick-monitor MSPT: avg **415.27ms** / min 343.42ms / max **539.27ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:57:15Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7177820 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.42 | — | — | — | 539.27 | 415.27 |

- entity totals seen: [149107, 150188, 151468]
- top entity types (max seen): minecraft:item×103401, minecraft:creeper×5233, minecraft:husk×5121, minecraft:spider×4865, minecraft:skeleton×4828, minecraft:zombie×4703, minecraft:drowned×4569, minecraft:sheep×3519, minecraft:chicken×3397, minecraft:cow×3362, minecraft:pig×3240, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/91WTSCJoTk
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **10**)
- total pause: **23724.7 ms**, avg **201.06 ms**, max **2592.3 ms**
- heap high-water seen: **7490 MB** -> last-after: **5203 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116542)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28530 | 24.5% |
| kernel: other | 28244 | 24.2% |
| other | 14095 | 12.1% |
| moonrise/paper patches | 9848 | 8.5% |
| chunk system (kernel) | 9753 | 8.4% |
| fastutil collections | 7402 | 6.4% |
| JDK collections | 6363 | 5.5% |
| JIT stubs (vtable/itable) | 3600 | 3.1% |
| network (kernel) | 3300 | 2.8% |
| JDK invokes/VarHandle | 2434 | 2.1% |
| JDK other | 1943 | 1.7% |
| JVM internals (GC oop barriers) | 516 | 0.4% |
| vdso (clock) | 212 | 0.2% |
| block entities/hoppers (kernel) | 98 | 0.1% |
| bukkit api | 56 | 0.0% |
| craftbukkit glue | 56 | 0.0% |
| worldgen/noise (kernel) | 47 | 0.0% |
| redstone (kernel) | 43 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93713 | 80.4% |
| phase: unclassified | 12780 | 11.0% |
| phase: main tick (unclassified) | 3799 | 3.3% |
| phase: chunk tick | 2055 | 1.8% |
| phase: network sync (ServerEntity) | 1912 | 1.6% |
| phase: chunk system (off-main worker) | 1093 | 0.9% |
| phase: block entities (hoppers/furnaces) | 649 | 0.6% |
| phase: random tick | 410 | 0.4% |
| phase: mob spawning | 130 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101539** (87.1%) · native/JVM-internal **14917** (12.8%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4558 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3384 | 2.9% |
| `vtable stub` | native/JVM-internal | 3019 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2593 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1889 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1779 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1762 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1741 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1608 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1589 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1558 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1521 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1474 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1434 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1380 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1217 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1142 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1102 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1091 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1068 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1056 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1007 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 911 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 909 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 906 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 873 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 852 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 834 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 734 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 707 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 691 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 667 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 661 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 644 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57858 | 94.5% |
| entities/mobs (kernel) | 1036 | 1.7% |
| kernel: other | 896 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 320 | 0.5% |
| fastutil collections | 229 | 0.4% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 139 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57685 | 94.2% |
| phase: entity tick (AI/movement) | 3114 | 5.1% |
| phase: main tick (unclassified) | 205 | 0.3% |
| phase: chunk tick | 92 | 0.2% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 30 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52358** (85.5%) · native/JVM-internal **8883** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49025 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 157 | 0.3% |
| `vtable stub` | native/JVM-internal | 121 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `syscall` | native/JVM-internal | 86 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3693)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3693 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2093 | 56.7% |
| phase: unclassified | 1423 | 38.5% |
| phase: main tick (unclassified) | 92 | 2.5% |
| phase: chunk system (off-main worker) | 51 | 1.4% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3693** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 529 | 14.3% |
| `char[]_[k]` | other | 436 | 11.8% |
| `byte[]_[k]` | other | 196 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 171 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 160 | 4.3% |
| `long[]_[i]` | other | 140 | 3.8% |
| `java.util.ArrayList_[i]` | other | 124 | 3.4% |
| `java.lang.Object[]_[i]` | other | 106 | 2.9% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.1% |
| `int[]_[i]` | other | 58 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `net.minecraft.core.SectionPos_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `java.lang.String_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f33f59dfbb8_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116542 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35002 | 30.03% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22625 | 19.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6457 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5528 | 4.74% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4632 | 3.97% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1162 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 925 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 405 | 0.35% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 212 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 210 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 529 | 14.3% |
| `char[]_[k]` | 436 | 11.8% |
| `byte[]_[k]` | 196 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 171 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 160 | 4.3% |
| `long[]_[i]` | 140 | 3.8% |
| `java.util.ArrayList_[i]` | 124 | 3.4% |
| `java.lang.Object[]_[i]` | 106 | 2.9% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23725 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148198..151468 (delta 3270, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99716->103401, minecraft:drowned 3418->4569, minecraft:zombie 3687->4703, minecraft:creeper 4548->5233, minecraft:spider 4244->4865, minecraft:husk 4503->5121, minecraft:skeleton 4386->4828, minecraft:chicken 3366->3397
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3270)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59049341 B)
- `wall-collapsed.txt` (3720510 B)
- `alloc-collapsed.txt` (2200417 B)
- `cpu-flamegraph.html` (297249 B)
- `server-stdout.log` (251818 B)
- `gc.log` (112777 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
