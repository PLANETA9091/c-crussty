# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.79 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.6, 1.9, 2.0, 2.5, 2.5]
- spark tick-monitor MSPT: avg **421.72ms** / min 346.61ms / max **562.75ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T09:12:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6941789 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.61 | — | — | — | 562.75 | 421.72 |

- entity totals seen: [148966, 150065, 151351]
- top entity types (max seen): minecraft:item×103236, minecraft:creeper×5248, minecraft:husk×5145, minecraft:skeleton×4882, minecraft:spider×4833, minecraft:zombie×4643, minecraft:drowned×4551, minecraft:sheep×3521, minecraft:chicken×3412, minecraft:cow×3375, minecraft:pig×3242, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZRw1tpBt4X
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **10**)
- total pause: **24146.5 ms**, avg **199.56 ms**, max **2458.3 ms**
- heap high-water seen: **7542 MB** -> last-after: **5588 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116820)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28324 | 24.2% |
| entities/mobs (kernel) | 27736 | 23.7% |
| other | 15613 | 13.4% |
| moonrise/paper patches | 10274 | 8.8% |
| chunk system (kernel) | 9575 | 8.2% |
| fastutil collections | 6865 | 5.9% |
| JDK collections | 5627 | 4.8% |
| JIT stubs (vtable/itable) | 3626 | 3.1% |
| network (kernel) | 3096 | 2.7% |
| JDK invokes/VarHandle | 2770 | 2.4% |
| JDK other | 2267 | 1.9% |
| JVM internals (GC oop barriers) | 577 | 0.5% |
| vdso (clock) | 211 | 0.2% |
| block entities/hoppers (kernel) | 73 | 0.1% |
| bukkit api | 65 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| craftbukkit glue | 44 | 0.0% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92680 | 79.3% |
| phase: unclassified | 14324 | 12.3% |
| phase: main tick (unclassified) | 3619 | 3.1% |
| phase: chunk tick | 2058 | 1.8% |
| phase: network sync (ServerEntity) | 1773 | 1.5% |
| phase: chunk system (off-main worker) | 1183 | 1.0% |
| phase: block entities (hoppers/furnaces) | 675 | 0.6% |
| phase: random tick | 379 | 0.3% |
| phase: mob spawning | 120 | 0.1% |
| phase: scheduler/mid-tick tasks | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100382** (85.9%) · native/JVM-internal **16331** (14.0%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4359 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3284 | 2.8% |
| `vtable stub` | native/JVM-internal | 3031 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2967 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2080 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1984 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1715 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1639 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1567 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1529 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1516 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1479 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1416 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1350 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1266 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1252 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1146 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1097 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1066 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1057 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 949 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 902 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 879 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 847 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 841 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 822 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 809 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 718 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 714 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 708 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 684 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 655 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 638 | 0.5% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 637 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57881 | 94.5% |
| entities/mobs (kernel) | 981 | 1.6% |
| kernel: other | 927 | 1.5% |
| moonrise/paper patches | 357 | 0.6% |
| chunk system (kernel) | 298 | 0.5% |
| fastutil collections | 204 | 0.3% |
| JDK collections | 177 | 0.3% |
| JIT stubs (vtable/itable) | 136 | 0.2% |
| network (kernel) | 111 | 0.2% |
| JDK other | 81 | 0.1% |
| JDK invokes/VarHandle | 81 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57688 | 94.2% |
| phase: entity tick (AI/movement) | 3108 | 5.1% |
| phase: main tick (unclassified) | 193 | 0.3% |
| phase: chunk tick | 95 | 0.2% |
| phase: chunk system (off-main worker) | 57 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52367** (85.5%) · native/JVM-internal **8885** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49026 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `vtable stub` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 103 | 0.2% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3583)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3583 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2008 | 56.0% |
| phase: unclassified | 1395 | 38.9% |
| phase: main tick (unclassified) | 86 | 2.4% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: network sync (ServerEntity) | 27 | 0.8% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3583** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 541 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 515 | 14.4% |
| `char[]_[k]` | other | 437 | 12.2% |
| `byte[]_[k]` | other | 214 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 178 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 126 | 3.5% |
| `long[]_[i]` | other | 114 | 3.2% |
| `java.util.ArrayList_[i]` | other | 109 | 3.0% |
| `java.lang.Object[]_[i]` | other | 105 | 2.9% |
| `byte[]_[i]` | other | 103 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.9% |
| `int[]_[i]` | other | 64 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f85329dddd8_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116820 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34113 | 29.20% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22846 | 19.56% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6476 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5417 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4480 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1202 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 924 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 459 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 216 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 208 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 541 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 515 | 14.4% |
| `char[]_[k]` | 437 | 12.2% |
| `byte[]_[k]` | 214 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 178 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 126 | 3.5% |
| `long[]_[i]` | 114 | 3.2% |
| `java.util.ArrayList_[i]` | 109 | 3.0% |
| `java.lang.Object[]_[i]` | 105 | 2.9% |
| `byte[]_[i]` | 103 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 24147 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148119..151351 (delta 3232, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99628->103236, minecraft:drowned 3469->4551, minecraft:zombie 3610->4643, minecraft:creeper 4571->5248, minecraft:husk 4508->5145, minecraft:spider 4259->4833, minecraft:skeleton 4406->4882, minecraft:chicken 3382->3412
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3232)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57287583 B)
- `wall-collapsed.txt` (3791713 B)
- `alloc-collapsed.txt` (2002560 B)
- `cpu-flamegraph.html` (306071 B)
- `server-stdout.log` (257539 B)
- `gc.log` (115350 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
