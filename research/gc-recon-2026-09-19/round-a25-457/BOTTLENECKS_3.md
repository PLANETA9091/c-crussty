# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.127 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 2.1, 2.4, 2.7, 2.6]
- spark tick-monitor MSPT: avg **378.71ms** / min 328.73ms / max **470.16ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:42:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7060169 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 328.73 | — | — | — | 470.16 | 378.71 |

- entity totals seen: [149085, 150905, 151324]
- top entity types (max seen): minecraft:item×103396, minecraft:creeper×5188, minecraft:husk×5181, minecraft:skeleton×4831, minecraft:spider×4803, minecraft:zombie×4631, minecraft:drowned×4511, minecraft:sheep×3524, minecraft:chicken×3429, minecraft:cow×3390, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZQBJLjdq9I
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **21142.9 ms**, avg **171.89 ms**, max **2559.8 ms**
- heap high-water seen: **7634 MB** -> last-after: **4360 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116751)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28816 | 24.7% |
| kernel: other | 27787 | 23.8% |
| other | 14955 | 12.8% |
| moonrise/paper patches | 9520 | 8.2% |
| chunk system (kernel) | 9435 | 8.1% |
| fastutil collections | 7342 | 6.3% |
| JDK collections | 6648 | 5.7% |
| JIT stubs (vtable/itable) | 3700 | 3.2% |
| network (kernel) | 3200 | 2.7% |
| JDK invokes/VarHandle | 2321 | 2.0% |
| JDK other | 1982 | 1.7% |
| JVM internals (GC oop barriers) | 535 | 0.5% |
| vdso (clock) | 226 | 0.2% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93217 | 79.8% |
| phase: unclassified | 13799 | 11.8% |
| phase: main tick (unclassified) | 3658 | 3.1% |
| phase: chunk tick | 1972 | 1.7% |
| phase: network sync (ServerEntity) | 1826 | 1.6% |
| phase: chunk system (off-main worker) | 1097 | 0.9% |
| phase: block entities (hoppers/furnaces) | 626 | 0.5% |
| phase: random tick | 440 | 0.4% |
| phase: mob spawning | 110 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100774** (86.3%) · native/JVM-internal **15897** (13.6%) · other **80** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4526 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3292 | 2.8% |
| `vtable stub` | native/JVM-internal | 3098 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2628 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1986 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1860 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1784 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1739 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1634 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1564 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1523 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1507 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1386 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1323 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1312 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1098 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1073 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1062 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1020 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1002 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 928 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 910 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 909 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 882 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 865 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 841 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 826 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 805 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 759 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 757 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 742 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 735 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 660 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 650 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 647 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 616 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57861 | 94.4% |
| entities/mobs (kernel) | 1021 | 1.7% |
| kernel: other | 853 | 1.4% |
| moonrise/paper patches | 352 | 0.6% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 247 | 0.4% |
| JDK collections | 171 | 0.3% |
| JIT stubs (vtable/itable) | 155 | 0.3% |
| network (kernel) | 105 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 75 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57693 | 94.2% |
| phase: entity tick (AI/movement) | 3122 | 5.1% |
| phase: main tick (unclassified) | 208 | 0.3% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52369** (85.5%) · native/JVM-internal **8889** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49030 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4684 | 7.6% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `vtable stub` | native/JVM-internal | 125 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `getdents64` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9406)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9406 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 7070 | 75.2% |
| phase: entity tick (AI/movement) | 2100 | 22.3% |
| phase: chunk system (off-main worker) | 100 | 1.1% |
| phase: main tick (unclassified) | 93 | 1.0% |
| phase: network sync (ServerEntity) | 18 | 0.2% |
| phase: block entities (hoppers/furnaces) | 12 | 0.1% |
| phase: chunk tick | 9 | 0.1% |
| phase: mob spawning | 3 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9406** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1569 | 16.7% |
| `byte[]_[k]` | other | 632 | 6.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 591 | 6.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 530 | 5.6% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 512 | 5.4% |
| `short[]_[i]` | other | 454 | 4.8% |
| `char[]_[k]` | other | 448 | 4.8% |
| `java.lang.Object[]_[i]` | other | 434 | 4.6% |
| `long[]_[k]` | other | 392 | 4.2% |
| `byte[]_[i]` | other | 323 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 270 | 2.9% |
| `java.lang.String_[i]` | other | 184 | 2.0% |
| `java.util.Optional_[i]` | other | 168 | 1.8% |
| `long[]_[i]` | other | 165 | 1.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 1.6% |
| `java.lang.Object[]_[k]` | other | 150 | 1.6% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 143 | 1.5% |
| `java.util.ArrayList_[i]` | other | 137 | 1.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 132 | 1.4% |
| `int[]_[i]` | other | 80 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116751 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34493 | 29.54% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22493 | 19.27% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6462 | 5.53% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5519 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4440 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1245 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 866 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 445 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 262 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 239 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 187 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1569 | 16.7% |
| `byte[]_[k]` | 632 | 6.7% |
| `net.minecraft.world.phys.AABB_[i]` | 591 | 6.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 530 | 5.6% |
| `com.mojang.serialization.DataResult$Success_[i]` | 512 | 5.4% |
| `short[]_[i]` | 454 | 4.8% |
| `char[]_[k]` | 448 | 4.8% |
| `java.lang.Object[]_[i]` | 434 | 4.6% |
| `long[]_[k]` | 392 | 4.2% |
| `byte[]_[i]` | 323 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 21143 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148084..151324 (delta 3240, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99765->103396, minecraft:drowned 3491->4511, minecraft:zombie 3712->4631, minecraft:husk 4495->5181, minecraft:creeper 4585->5188, minecraft:spider 4212->4803, minecraft:skeleton 4436->4831, minecraft:chicken 3394->3429
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3240)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58151614 B)
- `wall-collapsed.txt` (3792476 B)
- `alloc-collapsed.txt` (3831232 B)
- `cpu-flamegraph.html` (313822 B)
- `server-stdout.log` (255269 B)
- `gc.log` (116155 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
