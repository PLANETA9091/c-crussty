# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.824 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.5, 1.5, 1.8, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **431.93ms** / min 369.19ms / max **558.76ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:36:32Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6463213 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 369.19 | — | — | — | 558.76 | 431.93 |

- entity totals seen: [149050, 150199, 151407]
- top entity types (max seen): minecraft:item×103261, minecraft:creeper×5184, minecraft:husk×5127, minecraft:spider×4900, minecraft:skeleton×4890, minecraft:zombie×4642, minecraft:drowned×4556, minecraft:sheep×3518, minecraft:chicken×3404, minecraft:cow×3378, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Emc8iGulyf
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **23030.3 ms**, avg **193.53 ms**, max **2610.5 ms**
- heap high-water seen: **7491 MB** -> last-after: **4227 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115268)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 26918 | 23.4% |
| kernel: other | 25540 | 22.2% |
| other | 15868 | 13.8% |
| moonrise/paper patches | 11000 | 9.5% |
| chunk system (kernel) | 10524 | 9.1% |
| fastutil collections | 7264 | 6.3% |
| JDK collections | 5855 | 5.1% |
| network (kernel) | 3436 | 3.0% |
| JDK invokes/VarHandle | 2973 | 2.6% |
| JIT stubs (vtable/itable) | 2868 | 2.5% |
| JDK other | 1866 | 1.6% |
| JVM internals (GC oop barriers) | 585 | 0.5% |
| vdso (clock) | 251 | 0.2% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| bukkit api | 66 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89170 | 77.4% |
| phase: unclassified | 15394 | 13.4% |
| phase: main tick (unclassified) | 3921 | 3.4% |
| phase: chunk tick | 2375 | 2.1% |
| phase: network sync (ServerEntity) | 1916 | 1.7% |
| phase: chunk system (off-main worker) | 1217 | 1.1% |
| phase: block entities (hoppers/furnaces) | 719 | 0.6% |
| phase: random tick | 420 | 0.4% |
| phase: mob spawning | 133 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98563** (85.5%) · native/JVM-internal **16628** (14.4%) · other **77** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5107 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3590 | 3.1% |
| `vtable stub` | native/JVM-internal | 2336 | 2.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2328 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2274 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2163 | 1.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1930 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1880 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1844 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1813 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1460 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1440 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1347 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1318 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1267 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1195 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1165 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1119 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1064 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1055 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1052 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1026 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1019 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1000 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 996 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 989 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 912 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 908 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 846 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 833 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 828 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 812 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 792 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 752 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 691 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 689 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 653 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 648 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61264)

| bucket | self-time samples | share |
|---|---|---|
| other | 57955 | 94.6% |
| entities/mobs (kernel) | 926 | 1.5% |
| kernel: other | 901 | 1.5% |
| moonrise/paper patches | 351 | 0.6% |
| chunk system (kernel) | 310 | 0.5% |
| fastutil collections | 226 | 0.4% |
| JDK collections | 159 | 0.3% |
| JIT stubs (vtable/itable) | 130 | 0.2% |
| network (kernel) | 103 | 0.2% |
| JDK invokes/VarHandle | 94 | 0.2% |
| JDK other | 82 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57787 | 94.3% |
| phase: entity tick (AI/movement) | 3011 | 4.9% |
| phase: main tick (unclassified) | 191 | 0.3% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52340** (85.4%) · native/JVM-internal **8918** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49088 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 154 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 112 | 0.2% |
| `syscall` | native/JVM-internal | 109 | 0.2% |
| `vtable stub` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 66 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 42 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3683)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3683 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2031 | 55.1% |
| phase: unclassified | 1471 | 39.9% |
| phase: main tick (unclassified) | 87 | 2.4% |
| phase: chunk system (off-main worker) | 47 | 1.3% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3683** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 506 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 504 | 13.7% |
| `char[]_[k]` | other | 440 | 11.9% |
| `byte[]_[k]` | other | 208 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 184 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 160 | 4.3% |
| `long[]_[i]` | other | 137 | 3.7% |
| `java.util.ArrayList_[i]` | other | 129 | 3.5% |
| `java.lang.Object[]_[i]` | other | 97 | 2.6% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 86 | 2.3% |
| `int[]_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 48 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f10fb9eba30_[i]` | other | 31 | 0.8% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 26 | 0.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 26 | 0.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115268 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33858 | 29.37% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21690 | 18.82% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6172 | 5.35% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5134 | 4.45% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4319 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 911 | 0.79% |
| `net/minecraft/world/entity/ai/Brain.tick` | 909 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 433 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 245 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 236 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 185 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 506 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | 504 | 13.7% |
| `char[]_[k]` | 440 | 11.9% |
| `byte[]_[k]` | 208 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 184 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 160 | 4.3% |
| `long[]_[i]` | 137 | 3.7% |
| `java.util.ArrayList_[i]` | 129 | 3.5% |
| `java.lang.Object[]_[i]` | 97 | 2.6% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 23030 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148108..151407 (delta 3299, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99637->103261, minecraft:drowned 3496->4556, minecraft:zombie 3649->4642, minecraft:spider 4242->4900, minecraft:creeper 4573->5184, minecraft:husk 4516->5127, minecraft:skeleton 4465->4890, minecraft:chicken 3375->3404
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3299)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52051491 B)
- `wall-collapsed.txt` (3529177 B)
- `alloc-collapsed.txt` (2059321 B)
- `cpu-flamegraph.html` (309562 B)
- `server-stdout.log` (248889 B)
- `gc.log` (112683 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
