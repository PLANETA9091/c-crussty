# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.347 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.9, 1.7, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **407.67ms** / min 357.99ms / max **492.97ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:53:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6436162 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 357.99 | — | — | — | 492.97 | 407.67 |

- entity totals seen: [149037, 150378, 151465]
- top entity types (max seen): minecraft:item×103377, minecraft:husk×5187, minecraft:creeper×5177, minecraft:spider×4822, minecraft:skeleton×4817, minecraft:zombie×4699, minecraft:drowned×4557, minecraft:sheep×3517, minecraft:chicken×3414, minecraft:cow×3359, minecraft:pig×3227, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wPVmlEYc11
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **24043.7 ms**, avg **195.48 ms**, max **2391.0 ms**
- heap high-water seen: **7496 MB** -> last-after: **5142 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116535)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28053 | 24.1% |
| kernel: other | 27593 | 23.7% |
| other | 15873 | 13.6% |
| chunk system (kernel) | 9993 | 8.6% |
| moonrise/paper patches | 9590 | 8.2% |
| fastutil collections | 7194 | 6.2% |
| JDK collections | 6101 | 5.2% |
| JIT stubs (vtable/itable) | 3385 | 2.9% |
| network (kernel) | 3170 | 2.7% |
| JDK invokes/VarHandle | 2266 | 1.9% |
| JDK other | 2227 | 1.9% |
| JVM internals (GC oop barriers) | 562 | 0.5% |
| vdso (clock) | 231 | 0.2% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| bukkit api | 58 | 0.0% |
| worldgen/noise (kernel) | 45 | 0.0% |
| redstone (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92047 | 79.0% |
| phase: unclassified | 14559 | 12.5% |
| phase: main tick (unclassified) | 3747 | 3.2% |
| phase: chunk tick | 2069 | 1.8% |
| phase: network sync (ServerEntity) | 1834 | 1.6% |
| phase: chunk system (off-main worker) | 1112 | 1.0% |
| phase: block entities (hoppers/furnaces) | 611 | 0.5% |
| phase: random tick | 417 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100063** (85.9%) · native/JVM-internal **16365** (14.0%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4703 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3111 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2838 | 2.4% |
| `vtable stub` | native/JVM-internal | 2792 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1985 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1773 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1715 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1541 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1513 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1510 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1489 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1448 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1373 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1322 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1263 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f1d19a2b938.accept` | JVM-Java | 1219 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1087 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1054 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1020 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 998 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 951 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 935 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 874 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 852 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 848 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 848 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 821 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 768 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 683 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 681 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 662 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 648 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 613 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57912 | 94.5% |
| entities/mobs (kernel) | 967 | 1.6% |
| kernel: other | 945 | 1.5% |
| chunk system (kernel) | 314 | 0.5% |
| moonrise/paper patches | 302 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 188 | 0.3% |
| network (kernel) | 112 | 0.2% |
| JIT stubs (vtable/itable) | 107 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57713 | 94.2% |
| phase: entity tick (AI/movement) | 3088 | 5.0% |
| phase: main tick (unclassified) | 219 | 0.4% |
| phase: chunk tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52440** (85.6%) · native/JVM-internal **8812** (14.4%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49085 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 151 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 109 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `syscall` | native/JVM-internal | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f1d19a2b938.accept` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 42 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3833)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3833 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2147 | 56.0% |
| phase: unclassified | 1498 | 39.1% |
| phase: main tick (unclassified) | 84 | 2.2% |
| phase: chunk system (off-main worker) | 66 | 1.7% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 1 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3833** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 533 | 13.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 493 | 12.9% |
| `char[]_[k]` | other | 435 | 11.3% |
| `byte[]_[k]` | other | 203 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 173 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 166 | 4.3% |
| `java.util.ArrayList_[i]` | other | 164 | 4.3% |
| `long[]_[i]` | other | 145 | 3.8% |
| `java.lang.Object[]_[i]` | other | 110 | 2.9% |
| `byte[]_[i]` | other | 88 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.0% |
| `int[]_[i]` | other | 63 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 59 | 1.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 56 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 51 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 45 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 38 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 32 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116535 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34514 | 29.62% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21948 | 18.83% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6385 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5366 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4418 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1187 | 1.02% |
| `net/minecraft/world/entity/ai/Brain.tick` | 886 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 416 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 243 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 225 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 197 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 533 | 13.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 493 | 12.9% |
| `char[]_[k]` | 435 | 11.3% |
| `byte[]_[k]` | 203 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 173 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 166 | 4.3% |
| `java.util.ArrayList_[i]` | 164 | 4.3% |
| `long[]_[i]` | 145 | 3.8% |
| `java.lang.Object[]_[i]` | 110 | 2.9% |
| `byte[]_[i]` | 88 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 24044 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148200..151465 (delta 3265, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99747->103377, minecraft:drowned 3514->4557, minecraft:zombie 3704->4699, minecraft:husk 4503->5187, minecraft:creeper 4527->5177, minecraft:spider 4247->4822, minecraft:skeleton 4395->4817, minecraft:chicken 3380->3414
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3265)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57317206 B)
- `wall-collapsed.txt` (3728897 B)
- `alloc-collapsed.txt` (2088606 B)
- `cpu-flamegraph.html` (302360 B)
- `server-stdout.log` (249439 B)
- `gc.log` (117080 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
