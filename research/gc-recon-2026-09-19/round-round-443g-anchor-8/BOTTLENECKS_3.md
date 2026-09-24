# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.106 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.5, 1.8, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **435.72ms** / min 369.59ms / max **543.66ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:44:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6428986 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 369.59 | — | — | — | 543.66 | 435.72 |

- entity totals seen: [149074, 150265, 151346]
- top entity types (max seen): minecraft:item×103259, minecraft:husk×5167, minecraft:creeper×5143, minecraft:skeleton×4885, minecraft:spider×4787, minecraft:zombie×4616, minecraft:drowned×4546, minecraft:sheep×3526, minecraft:chicken×3421, minecraft:cow×3368, minecraft:pig×3230, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/B5MqzWZxmX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **21492.0 ms**, avg **185.28 ms**, max **2999.0 ms**
- heap high-water seen: **7481 MB** -> last-after: **4221 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116393)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28070 | 24.1% |
| kernel: other | 27217 | 23.4% |
| other | 15659 | 13.5% |
| moonrise/paper patches | 10434 | 9.0% |
| chunk system (kernel) | 9740 | 8.4% |
| fastutil collections | 7151 | 6.1% |
| JDK collections | 5775 | 5.0% |
| JIT stubs (vtable/itable) | 3558 | 3.1% |
| network (kernel) | 3187 | 2.7% |
| JDK invokes/VarHandle | 2616 | 2.2% |
| JDK other | 1852 | 1.6% |
| JVM internals (GC oop barriers) | 567 | 0.5% |
| vdso (clock) | 236 | 0.2% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| redstone (kernel) | 84 | 0.1% |
| bukkit api | 68 | 0.1% |
| craftbukkit glue | 55 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91306 | 78.4% |
| phase: unclassified | 14642 | 12.6% |
| phase: main tick (unclassified) | 3746 | 3.2% |
| phase: chunk tick | 2286 | 2.0% |
| phase: network sync (ServerEntity) | 1770 | 1.5% |
| phase: chunk system (off-main worker) | 1342 | 1.2% |
| phase: block entities (hoppers/furnaces) | 728 | 0.6% |
| phase: random tick | 458 | 0.4% |
| phase: mob spawning | 111 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99658** (85.6%) · native/JVM-internal **16647** (14.3%) · other **88** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4391 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3606 | 3.1% |
| `vtable stub` | native/JVM-internal | 2993 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2734 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2043 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1930 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1795 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1669 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1619 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1540 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1436 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1367 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1291 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1281 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1268 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1228 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1119 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1061 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 997 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 958 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 898 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 882 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 871 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 870 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 862 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 857 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 833 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 820 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 806 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 796 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 790 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 696 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 678 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 676 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 649 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57865 | 94.5% |
| entities/mobs (kernel) | 962 | 1.6% |
| kernel: other | 944 | 1.5% |
| moonrise/paper patches | 342 | 0.6% |
| chunk system (kernel) | 325 | 0.5% |
| fastutil collections | 214 | 0.3% |
| JDK collections | 183 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 119 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57695 | 94.2% |
| phase: entity tick (AI/movement) | 3127 | 5.1% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52370** (85.5%) · native/JVM-internal **8881** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48994 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `vtable stub` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3638)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3638 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1998 | 54.9% |
| phase: unclassified | 1478 | 40.6% |
| phase: main tick (unclassified) | 74 | 2.0% |
| phase: chunk system (off-main worker) | 43 | 1.2% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3638** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 543 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 515 | 14.2% |
| `char[]_[k]` | other | 445 | 12.2% |
| `byte[]_[k]` | other | 210 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 149 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 3.9% |
| `long[]_[i]` | other | 139 | 3.8% |
| `java.util.ArrayList_[i]` | other | 130 | 3.6% |
| `java.lang.Object[]_[i]` | other | 99 | 2.7% |
| `byte[]_[i]` | other | 87 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.8% |
| `int[]_[i]` | other | 63 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 49 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f869282eb40_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 26 | 0.7% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f86929daa38_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116393 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34644 | 29.76% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21618 | 18.57% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6393 | 5.49% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5462 | 4.69% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4385 | 3.77% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1128 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 896 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 418 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 233 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 211 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 543 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 515 | 14.2% |
| `char[]_[k]` | 445 | 12.2% |
| `byte[]_[k]` | 210 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 149 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 3.9% |
| `long[]_[i]` | 139 | 3.8% |
| `java.util.ArrayList_[i]` | 130 | 3.6% |
| `java.lang.Object[]_[i]` | 99 | 2.7% |
| `byte[]_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 21492 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148162..151346 (delta 3184, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99713->103259, minecraft:drowned 3560->4546, minecraft:zombie 3682->4616, minecraft:husk 4525->5167, minecraft:creeper 4544->5143, minecraft:spider 4227->4787, minecraft:skeleton 4457->4885, minecraft:chicken 3396->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3184)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55730080 B)
- `wall-collapsed.txt` (3742332 B)
- `alloc-collapsed.txt` (2059087 B)
- `cpu-flamegraph.html` (297281 B)
- `server-stdout.log` (251482 B)
- `gc.log` (110147 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
