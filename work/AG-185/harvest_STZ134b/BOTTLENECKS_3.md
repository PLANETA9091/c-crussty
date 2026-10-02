# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.646 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.6, 1.9, 2.3, 2.5, 2.6]
- spark tick-monitor MSPT: avg **403.57ms** / min 348.69ms / max **582.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-10-01T12:37:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7016861 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384 + S99-gcw ROUND-468-S15; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity; 6 = ParallelGC + MetaspaceSize=256M + ReservedCodeCacheSize=512M — threshold-cascade kill, javap-neutral)
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
- population_seed: 523047 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)
- lever_flag=stz134_ctrl lever_arg=0 (MEGA-ROUND lever A/B marker, TASK-395; empty = vanilla bit-in-bit — canary-gate SKIP-ARMED/C85 layer-2 reads this)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 348.69 | — | — | — | 582.68 | 403.57 |

- entity totals seen: [148661, 149863, 151070]
- top entity types (max seen): minecraft:item×103221, minecraft:creeper×5124, minecraft:husk×5117, minecraft:skeleton×4877, minecraft:spider×4837, minecraft:zombie×4647, minecraft:drowned×4502, minecraft:sheep×3447, minecraft:chicken×3440, minecraft:cow×3346, minecraft:pig×3248, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/nhND1H115m
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **21370.2 ms**, avg **181.10 ms**, max **2793.3 ms**
- heap high-water seen: **7570 MB** -> last-after: **4301 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117561)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28447 | 24.2% |
| entities/mobs (kernel) | 28036 | 23.8% |
| other | 16251 | 13.8% |
| moonrise/paper patches | 9810 | 8.3% |
| chunk system (kernel) | 9335 | 7.9% |
| fastutil collections | 6872 | 5.8% |
| JDK collections | 6518 | 5.5% |
| JIT stubs (vtable/itable) | 3533 | 3.0% |
| network (kernel) | 3380 | 2.9% |
| JDK other | 2232 | 1.9% |
| JDK invokes/VarHandle | 2075 | 1.8% |
| JVM internals (GC oop barriers) | 549 | 0.5% |
| vdso (clock) | 210 | 0.2% |
| block entities/hoppers (kernel) | 95 | 0.1% |
| bukkit api | 68 | 0.1% |
| craftbukkit glue | 52 | 0.0% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 43 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93114 | 79.2% |
| phase: unclassified | 14900 | 12.7% |
| phase: main tick (unclassified) | 3537 | 3.0% |
| phase: chunk tick | 1922 | 1.6% |
| phase: network sync (ServerEntity) | 1824 | 1.6% |
| phase: chunk system (off-main worker) | 1046 | 0.9% |
| phase: block entities (hoppers/furnaces) | 684 | 0.6% |
| phase: random tick | 401 | 0.3% |
| phase: mob spawning | 132 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100569** (85.5%) · native/JVM-internal **16913** (14.4%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4450 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3139 | 2.7% |
| `vtable stub` | native/JVM-internal | 2928 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2533 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1998 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1909 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1795 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1711 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1597 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1539 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1528 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1523 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1465 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1254 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1251 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1160 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1004 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 996 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 974 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 950 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 949 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 941 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 900 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 847 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 774 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 767 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 737 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 735 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 730 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 659 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 638 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tickRunningGoals` | JVM-Java | 633 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57853 | 94.5% |
| entities/mobs (kernel) | 1039 | 1.7% |
| kernel: other | 918 | 1.5% |
| moonrise/paper patches | 332 | 0.5% |
| chunk system (kernel) | 262 | 0.4% |
| fastutil collections | 232 | 0.4% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK other | 83 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57659 | 94.1% |
| phase: entity tick (AI/movement) | 3145 | 5.1% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: chunk tick | 77 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52367** (85.5%) · native/JVM-internal **8880** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49025 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 137 | 0.2% |
| `vtable stub` | native/JVM-internal | 116 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 84 | 0.1% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 55 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3692)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3692 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2049 | 55.5% |
| phase: unclassified | 1499 | 40.6% |
| phase: main tick (unclassified) | 86 | 2.3% |
| phase: chunk system (off-main worker) | 26 | 0.7% |
| phase: network sync (ServerEntity) | 15 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3692** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 532 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 521 | 14.1% |
| `char[]_[k]` | other | 429 | 11.6% |
| `byte[]_[k]` | other | 210 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 165 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 157 | 4.3% |
| `long[]_[i]` | other | 135 | 3.7% |
| `java.lang.Object[]_[i]` | other | 117 | 3.2% |
| `java.util.ArrayList_[i]` | other | 111 | 3.0% |
| `byte[]_[i]` | other | 93 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.2% |
| `int[]_[i]` | other | 62 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 49 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f9d659d3a40_[i]` | other | 32 | 0.9% |
| `int[]_[k]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117561 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34674 | 29.49% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22599 | 19.22% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6554 | 5.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5441 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4450 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1098 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 870 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 427 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 201 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 532 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 521 | 14.1% |
| `char[]_[k]` | 429 | 11.6% |
| `byte[]_[k]` | 210 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 165 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 157 | 4.3% |
| `long[]_[i]` | 135 | 3.7% |
| `java.lang.Object[]_[i]` | 117 | 3.2% |
| `java.util.ArrayList_[i]` | 111 | 3.0% |
| `byte[]_[i]` | 93 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 21370 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147755..151070 (delta 3315, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99499->103221, minecraft:zombie 3539->4647, minecraft:drowned 3487->4502, minecraft:husk 4458->5117, minecraft:spider 4219->4837, minecraft:creeper 4508->5124, minecraft:skeleton 4428->4877, minecraft:chicken 3418->3440
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3315)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57428739 B)
- `wall-collapsed.txt` (3718754 B)
- `alloc-collapsed.txt` (2030856 B)
- `cpu-flamegraph.html` (296532 B)
- `server-stdout.log` (253470 B)
- `gc.log` (111896 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
