# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.05 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.6, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **414.34ms** / min 346.21ms / max **555.79ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T14:53:11Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6688321 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.21 | — | — | — | 555.79 | 414.34 |

- entity totals seen: [149022, 150112, 151328]
- top entity types (max seen): minecraft:item×103248, minecraft:husk×5171, minecraft:creeper×5152, minecraft:skeleton×4872, minecraft:spider×4840, minecraft:zombie×4623, minecraft:drowned×4542, minecraft:sheep×3525, minecraft:chicken×3425, minecraft:cow×3365, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XW0Rn8P18s
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **10**)
- total pause: **24267.3 ms**, avg **200.56 ms**, max **2419.3 ms**
- heap high-water seen: **7413 MB** -> last-after: **5217 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116819)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28136 | 24.1% |
| kernel: other | 28016 | 24.0% |
| other | 15206 | 13.0% |
| moonrise/paper patches | 10058 | 8.6% |
| chunk system (kernel) | 9717 | 8.3% |
| fastutil collections | 7023 | 6.0% |
| JDK collections | 6354 | 5.4% |
| network (kernel) | 3529 | 3.0% |
| JIT stubs (vtable/itable) | 3208 | 2.7% |
| JDK invokes/VarHandle | 2553 | 2.2% |
| JDK other | 1964 | 1.7% |
| JVM internals (GC oop barriers) | 558 | 0.5% |
| vdso (clock) | 206 | 0.2% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 58 | 0.0% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93173 | 79.8% |
| phase: unclassified | 13816 | 11.8% |
| phase: main tick (unclassified) | 3703 | 3.2% |
| phase: chunk tick | 1960 | 1.7% |
| phase: network sync (ServerEntity) | 1883 | 1.6% |
| phase: chunk system (off-main worker) | 1060 | 0.9% |
| phase: block entities (hoppers/furnaces) | 675 | 0.6% |
| phase: random tick | 414 | 0.4% |
| phase: mob spawning | 134 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101151** (86.6%) · native/JVM-internal **15573** (13.3%) · other **95** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4524 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3245 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2760 | 2.4% |
| `vtable stub` | native/JVM-internal | 2614 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2096 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1872 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1721 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1701 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1614 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1608 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1519 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1517 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1509 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1500 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1424 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1280 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f55e9a20908.accept` | JVM-Java | 1227 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1048 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1039 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 993 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 957 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 921 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 902 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 875 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 844 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 834 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 817 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 807 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 769 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 730 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 724 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 689 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 670 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 667 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 648 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 610 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57905 | 94.5% |
| entities/mobs (kernel) | 1003 | 1.6% |
| kernel: other | 885 | 1.4% |
| moonrise/paper patches | 332 | 0.5% |
| chunk system (kernel) | 304 | 0.5% |
| fastutil collections | 239 | 0.4% |
| JDK collections | 198 | 0.3% |
| JIT stubs (vtable/itable) | 117 | 0.2% |
| network (kernel) | 115 | 0.2% |
| JDK invokes/VarHandle | 68 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57713 | 94.2% |
| phase: entity tick (AI/movement) | 3112 | 5.1% |
| phase: main tick (unclassified) | 190 | 0.3% |
| phase: chunk tick | 69 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52343** (85.5%) · native/JVM-internal **8903** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49022 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.2% |
| `vtable stub` | native/JVM-internal | 95 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f55e9a20908.accept` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 54 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3737)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3737 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2092 | 56.0% |
| phase: unclassified | 1470 | 39.3% |
| phase: main tick (unclassified) | 89 | 2.4% |
| phase: chunk system (off-main worker) | 45 | 1.2% |
| phase: network sync (ServerEntity) | 17 | 0.5% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3737** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 543 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 14.4% |
| `char[]_[k]` | other | 441 | 11.8% |
| `byte[]_[k]` | other | 208 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 155 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 3.8% |
| `long[]_[i]` | other | 131 | 3.5% |
| `java.util.ArrayList_[i]` | other | 129 | 3.5% |
| `java.lang.Object[]_[i]` | other | 118 | 3.2% |
| `byte[]_[i]` | other | 101 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 2.0% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f55e99e26d8_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.8% |
| `int[]_[k]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 28 | 0.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.7% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f55e99e81f8_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116819 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34593 | 29.61% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22502 | 19.26% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6614 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5436 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4533 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1126 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 867 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 371 | 0.32% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 258 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 233 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 216 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 543 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 14.4% |
| `char[]_[k]` | 441 | 11.8% |
| `byte[]_[k]` | 208 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 155 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 3.8% |
| `long[]_[i]` | 131 | 3.5% |
| `java.util.ArrayList_[i]` | 129 | 3.5% |
| `java.lang.Object[]_[i]` | 118 | 3.2% |
| `byte[]_[i]` | 101 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 24267 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148134..151328 (delta 3194, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99662->103248, minecraft:drowned 3520->4542, minecraft:zombie 3666->4623, minecraft:husk 4537->5171, minecraft:creeper 4526->5152, minecraft:spider 4226->4840, minecraft:skeleton 4374->4872, minecraft:chicken 3394->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3194)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55625832 B)
- `wall-collapsed.txt` (3720624 B)
- `alloc-collapsed.txt` (2096385 B)
- `cpu-flamegraph.html` (291718 B)
- `server-stdout.log` (246238 B)
- `gc.log` (115356 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
