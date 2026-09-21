# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.181 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.7, 1.8, 2.1, 2.4, 2.6, 2.8]
- spark tick-monitor MSPT: avg **377.05ms** / min 319.76ms / max **517.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T13:37:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 5204352 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 319.76 | — | — | — | 517.89 | 377.05 |

- entity totals seen: [151052, 154379, 157178]
- top entity types (max seen): minecraft:item×111097, minecraft:husk×5575, minecraft:skeleton×4824, minecraft:creeper×4810, minecraft:zombie×4704, minecraft:drowned×4563, minecraft:spider×4277, minecraft:sheep×3505, minecraft:chicken×3408, minecraft:cow×3361, minecraft:pig×3190, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/SNiwYEPN4w
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **19019.9 ms**, avg **163.96 ms**, max **2326.3 ms**
- heap high-water seen: **7010 MB** -> last-after: **3200 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 110266)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29448 | 26.7% |
| kernel: other | 26245 | 23.8% |
| other | 10800 | 9.8% |
| chunk system (kernel) | 9225 | 8.4% |
| moonrise/paper patches | 8405 | 7.6% |
| JDK collections | 6740 | 6.1% |
| fastutil collections | 6505 | 5.9% |
| network (kernel) | 3805 | 3.5% |
| JIT stubs (vtable/itable) | 3412 | 3.1% |
| JDK invokes/VarHandle | 2858 | 2.6% |
| JDK other | 2222 | 2.0% |
| vdso (clock) | 265 | 0.2% |
| block entities/hoppers (kernel) | 97 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| redstone (kernel) | 62 | 0.1% |
| bukkit api | 62 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89373 | 81.1% |
| phase: unclassified | 9324 | 8.5% |
| phase: main tick (unclassified) | 3951 | 3.6% |
| phase: chunk tick | 2696 | 2.4% |
| phase: network sync (ServerEntity) | 2322 | 2.1% |
| phase: chunk system (off-main worker) | 1216 | 1.1% |
| phase: block entities (hoppers/furnaces) | 760 | 0.7% |
| phase: random tick | 461 | 0.4% |
| phase: mob spawning | 157 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99155** (89.9%) · native/JVM-internal **11027** (10.0%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4403 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3975 | 3.6% |
| `vtable stub` | native/JVM-internal | 2825 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1916 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1880 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1852 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1755 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1670 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1633 | 1.5% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f16722710d8.accept` | JVM-Java | 1531 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1491 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1343 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1251 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1219 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1205 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1195 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1118 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1114 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1099 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1058 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1055 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1035 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1003 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 997 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 976 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 974 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 938 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 833 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 790 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 784 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 769 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 725 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 718 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 704 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 682 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 681 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 632 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 58084 | 94.8% |
| entities/mobs (kernel) | 963 | 1.6% |
| kernel: other | 818 | 1.3% |
| chunk system (kernel) | 273 | 0.4% |
| moonrise/paper patches | 268 | 0.4% |
| JDK collections | 235 | 0.4% |
| fastutil collections | 179 | 0.3% |
| JIT stubs (vtable/itable) | 129 | 0.2% |
| network (kernel) | 117 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 78 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 6 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57919 | 94.6% |
| phase: entity tick (AI/movement) | 2840 | 4.6% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 126 | 0.2% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52398** (85.5%) · native/JVM-internal **8852** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49270 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4736 | 7.7% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f16722710d8.accept` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3672)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3672 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2043 | 55.6% |
| phase: unclassified | 1448 | 39.4% |
| phase: main tick (unclassified) | 89 | 2.4% |
| phase: chunk system (off-main worker) | 47 | 1.3% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3672** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 586 | 16.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 521 | 14.2% |
| `char[]_[k]` | other | 439 | 12.0% |
| `byte[]_[k]` | other | 211 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 145 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 3.5% |
| `java.util.ArrayList_[i]` | other | 130 | 3.5% |
| `long[]_[i]` | other | 114 | 3.1% |
| `java.lang.Object[]_[i]` | other | 104 | 2.8% |
| `byte[]_[i]` | other | 81 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 1.9% |
| `int[]_[i]` | other | 64 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 43 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f16729de980_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |
| `java.lang.String_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110266 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 39697 | 36.00% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 17052 | 15.46% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5988 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5086 | 4.61% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3759 | 3.41% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1305 | 1.18% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1044 | 0.95% |
| `net/minecraft/world/entity/npc/Villager.tick` | 469 | 0.43% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 279 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 252 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 243 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 219 | 0.20% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 586 | 16.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 521 | 14.2% |
| `char[]_[k]` | 439 | 12.0% |
| `byte[]_[k]` | 211 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 145 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 130 | 3.5% |
| `java.util.ArrayList_[i]` | 130 | 3.5% |
| `long[]_[i]` | 114 | 3.1% |
| `java.lang.Object[]_[i]` | 104 | 2.8% |
| `byte[]_[i]` | 81 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 19020 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148302..157178 (delta 8876, churn 5.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99852->111097, minecraft:husk 4563->5575, minecraft:drowned 3668->4563, minecraft:zombie 3872->4704, minecraft:pig 2594->3190, minecraft:skeleton 4262->4824, minecraft:sheep 3038->3505, minecraft:chicken 2979->3408
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8876)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48801627 B)
- `wall-collapsed.txt` (3157711 B)
- `alloc-collapsed.txt` (1787909 B)
- `cpu-flamegraph.html` (279421 B)
- `server-stdout.log` (252915 B)
- `gc.log` (110106 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
