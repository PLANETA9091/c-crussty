# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.196 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 2.1, 2.3, 2.6, 2.6]
- spark tick-monitor MSPT: avg **395.06ms** / min 344.15ms / max **529.26ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T13:18:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6706928 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 344.15 | — | — | — | 529.26 | 395.06 |

- entity totals seen: [149095, 150811, 151630]
- top entity types (max seen): minecraft:item×103579, minecraft:creeper×5192, minecraft:husk×5175, minecraft:skeleton×4898, minecraft:spider×4851, minecraft:zombie×4651, minecraft:drowned×4532, minecraft:sheep×3513, minecraft:chicken×3415, minecraft:cow×3371, minecraft:pig×3231, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/8RoTxMF1j1
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **19741.1 ms**, avg **165.89 ms**, max **2391.2 ms**
- heap high-water seen: **7447 MB** -> last-after: **3927 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115647)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29306 | 25.3% |
| entities/mobs (kernel) | 29074 | 25.1% |
| other | 12019 | 10.4% |
| moonrise/paper patches | 9868 | 8.5% |
| chunk system (kernel) | 9852 | 8.5% |
| fastutil collections | 6975 | 6.0% |
| JDK collections | 6294 | 5.4% |
| JIT stubs (vtable/itable) | 3553 | 3.1% |
| network (kernel) | 3448 | 3.0% |
| JDK invokes/VarHandle | 2559 | 2.2% |
| JDK other | 2158 | 1.9% |
| vdso (clock) | 242 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| bukkit api | 60 | 0.1% |
| redstone (kernel) | 55 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95852 | 82.9% |
| phase: unclassified | 9966 | 8.6% |
| phase: main tick (unclassified) | 3638 | 3.1% |
| phase: chunk tick | 1975 | 1.7% |
| phase: network sync (ServerEntity) | 1821 | 1.6% |
| phase: chunk system (off-main worker) | 1189 | 1.0% |
| phase: block entities (hoppers/furnaces) | 646 | 0.6% |
| phase: random tick | 429 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **103531** (89.5%) · native/JVM-internal **12040** (10.4%) · other **76** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4569 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3409 | 2.9% |
| `vtable stub` | native/JVM-internal | 2860 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2763 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1887 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1851 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1841 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1669 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1665 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1642 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1586 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1578 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1479 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1476 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1410 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fd8f29dfd30.accept` | JVM-Java | 1344 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1092 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1084 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1057 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1049 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 947 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 910 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 909 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 903 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 896 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 857 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 850 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 836 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 752 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 736 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 712 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 707 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 696 | 0.6% |
| `itable stub` | native/JVM-internal | 691 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 679 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 57951 | 94.6% |
| kernel: other | 945 | 1.5% |
| entities/mobs (kernel) | 927 | 1.5% |
| moonrise/paper patches | 332 | 0.5% |
| chunk system (kernel) | 289 | 0.5% |
| JDK collections | 194 | 0.3% |
| fastutil collections | 191 | 0.3% |
| JIT stubs (vtable/itable) | 112 | 0.2% |
| network (kernel) | 98 | 0.2% |
| JDK invokes/VarHandle | 65 | 0.1% |
| JDK other | 64 | 0.1% |
| JVM internals (GC oop barriers) | 58 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57708 | 94.2% |
| phase: entity tick (AI/movement) | 3084 | 5.0% |
| phase: main tick (unclassified) | 238 | 0.4% |
| phase: chunk tick | 67 | 0.1% |
| phase: network sync (ServerEntity) | 53 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: random tick | 26 | 0.0% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51966** (84.8%) · native/JVM-internal **9270** (15.1%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48738 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 309 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fd8f29dfd30.accept` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3606)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3606 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2110 | 58.5% |
| phase: unclassified | 1339 | 37.1% |
| phase: main tick (unclassified) | 89 | 2.5% |
| phase: chunk system (off-main worker) | 32 | 0.9% |
| phase: network sync (ServerEntity) | 15 | 0.4% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3606** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 560 | 15.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 533 | 14.8% |
| `char[]_[k]` | other | 437 | 12.1% |
| `byte[]_[k]` | other | 195 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 194 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 133 | 3.7% |
| `long[]_[i]` | other | 130 | 3.6% |
| `java.util.ArrayList_[i]` | other | 114 | 3.2% |
| `java.lang.Object[]_[i]` | other | 96 | 2.7% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `int[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 53 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd8f29e2f58_[i]` | other | 28 | 0.8% |
| `int[]_[k]` | other | 23 | 0.6% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 22 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115647 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35445 | 30.65% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23244 | 20.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6836 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5568 | 4.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4618 | 3.99% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1151 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 907 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 472 | 0.41% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 208 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 560 | 15.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 533 | 14.8% |
| `char[]_[k]` | 437 | 12.1% |
| `byte[]_[k]` | 195 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 194 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 133 | 3.7% |
| `long[]_[i]` | 130 | 3.6% |
| `java.util.ArrayList_[i]` | 114 | 3.2% |
| `java.lang.Object[]_[i]` | 96 | 2.7% |
| `byte[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 19741 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148248..151630 (delta 3382, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99781->103579, minecraft:drowned 3497->4532, minecraft:zombie 3727->4651, minecraft:husk 4519->5175, minecraft:creeper 4539->5192, minecraft:spider 4215->4851, minecraft:skeleton 4463->4898, minecraft:chicken 3388->3415
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3382)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (62192991 B)
- `wall-collapsed.txt` (3645847 B)
- `alloc-collapsed.txt` (2007572 B)
- `cpu-flamegraph.html` (292564 B)
- `server-stdout.log` (255514 B)
- `gc.log` (112674 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
