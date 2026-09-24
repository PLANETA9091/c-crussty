# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.184 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 1.7, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **404.7ms** / min 336.5ms / max **534.37ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:55:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6725322 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 336.5 | — | — | — | 534.37 | 404.7 |

- entity totals seen: [149022, 150361, 151343]
- top entity types (max seen): minecraft:item×103294, minecraft:husk×5201, minecraft:creeper×5172, minecraft:skeleton×4858, minecraft:spider×4816, minecraft:zombie×4633, minecraft:drowned×4538, minecraft:sheep×3527, minecraft:chicken×3434, minecraft:cow×3363, minecraft:pig×3214, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/udMbpo3CjQ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **20849.6 ms**, avg **169.51 ms**, max **2377.6 ms**
- heap high-water seen: **7517 MB** -> last-after: **4245 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116938)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29104 | 24.9% |
| kernel: other | 28448 | 24.3% |
| other | 13851 | 11.8% |
| moonrise/paper patches | 10091 | 8.6% |
| chunk system (kernel) | 9688 | 8.3% |
| fastutil collections | 7375 | 6.3% |
| JDK collections | 6090 | 5.2% |
| JIT stubs (vtable/itable) | 3715 | 3.2% |
| network (kernel) | 3030 | 2.6% |
| JDK invokes/VarHandle | 2455 | 2.1% |
| JDK other | 2067 | 1.8% |
| JVM internals (GC oop barriers) | 546 | 0.5% |
| vdso (clock) | 201 | 0.2% |
| block entities/hoppers (kernel) | 74 | 0.1% |
| bukkit api | 59 | 0.1% |
| craftbukkit glue | 55 | 0.0% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93848 | 80.3% |
| phase: unclassified | 13093 | 11.2% |
| phase: main tick (unclassified) | 3697 | 3.2% |
| phase: chunk tick | 2045 | 1.7% |
| phase: network sync (ServerEntity) | 1837 | 1.6% |
| phase: chunk system (off-main worker) | 1153 | 1.0% |
| phase: block entities (hoppers/furnaces) | 705 | 0.6% |
| phase: random tick | 422 | 0.4% |
| phase: mob spawning | 129 | 0.1% |
| phase: scheduler/mid-tick tasks | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101914** (87.2%) · native/JVM-internal **14943** (12.8%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4574 | 3.9% |
| `vtable stub` | native/JVM-internal | 3050 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3032 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3014 | 2.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1994 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1776 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1651 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1649 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1584 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1567 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1538 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1526 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1525 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1289 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1286 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1238 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1191 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1100 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1084 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1083 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1068 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1042 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1019 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 959 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 949 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 945 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 939 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 915 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 869 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 847 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 820 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 757 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 752 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 721 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 661 | 0.6% |
| `itable stub` | native/JVM-internal | 661 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 660 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 629 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57826 | 94.4% |
| entities/mobs (kernel) | 1020 | 1.7% |
| kernel: other | 917 | 1.5% |
| moonrise/paper patches | 345 | 0.6% |
| chunk system (kernel) | 290 | 0.5% |
| fastutil collections | 247 | 0.4% |
| JDK collections | 189 | 0.3% |
| JIT stubs (vtable/itable) | 151 | 0.2% |
| JDK invokes/VarHandle | 96 | 0.2% |
| network (kernel) | 92 | 0.2% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57656 | 94.1% |
| phase: entity tick (AI/movement) | 3156 | 5.2% |
| phase: main tick (unclassified) | 202 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 28 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52394** (85.5%) · native/JVM-internal **8862** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49014 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `vtable stub` | native/JVM-internal | 120 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 94 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3763)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3763 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2086 | 55.4% |
| phase: unclassified | 1482 | 39.4% |
| phase: main tick (unclassified) | 92 | 2.4% |
| phase: chunk system (off-main worker) | 50 | 1.3% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: chunk tick | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3763** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 549 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 522 | 13.9% |
| `char[]_[k]` | other | 424 | 11.3% |
| `byte[]_[k]` | other | 216 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 165 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 161 | 4.3% |
| `long[]_[i]` | other | 146 | 3.9% |
| `java.util.ArrayList_[i]` | other | 125 | 3.3% |
| `java.lang.Object[]_[i]` | other | 95 | 2.5% |
| `byte[]_[i]` | other | 88 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.8% |
| `int[]_[i]` | other | 61 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 39 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 38 | 1.0% |
| `int[]_[k]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6b5d8fc800_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116938 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34479 | 29.48% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22882 | 19.57% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6708 | 5.74% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5544 | 4.74% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4580 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1168 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 909 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 465 | 0.40% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 207 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 199 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 549 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 522 | 13.9% |
| `char[]_[k]` | 424 | 11.3% |
| `byte[]_[k]` | 216 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 165 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 161 | 4.3% |
| `long[]_[i]` | 146 | 3.9% |
| `java.util.ArrayList_[i]` | 125 | 3.3% |
| `java.lang.Object[]_[i]` | 95 | 2.5% |
| `byte[]_[i]` | 88 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 20850 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148162..151343 (delta 3181, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99734->103294, minecraft:drowned 3476->4538, minecraft:zombie 3698->4633, minecraft:husk 4537->5201, minecraft:creeper 4516->5172, minecraft:spider 4222->4816, minecraft:skeleton 4435->4858, minecraft:chicken 3402->3434
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3181)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57295047 B)
- `wall-collapsed.txt` (3687827 B)
- `alloc-collapsed.txt` (2094966 B)
- `cpu-flamegraph.html` (297239 B)
- `server-stdout.log` (256865 B)
- `gc.log` (116142 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
