# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.659 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.8, 2.1, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **386.18ms** / min 309.19ms / max **520.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:26:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6951662 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:28:56 INFO]: [crussty-plugin] [cruss | 309.19 | — | — | — | 520.98 | 386.18 |

- entity totals seen: [149879, 152383, 153818]
- top entity types (max seen): minecraft:item×106942, minecraft:husk×5510, minecraft:creeper×4998, minecraft:skeleton×4789, minecraft:zombie×4607, minecraft:drowned×4552, minecraft:spider×4485, minecraft:sheep×3525, minecraft:chicken×3405, minecraft:cow×3327, minecraft:pig×3176, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/1BVnGEDftR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **111** (Full GC: **9**)
- total pause: **20532.0 ms**, avg **184.97 ms**, max **3010.2 ms**
- heap high-water seen: **7527 MB** -> last-after: **4135 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104735)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34288 | 32.7% |
| kernel: other | 21978 | 21.0% |
| other | 12029 | 11.5% |
| chunk system (kernel) | 7486 | 7.1% |
| JDK collections | 7277 | 6.9% |
| moonrise/paper patches | 5209 | 5.0% |
| fastutil collections | 4863 | 4.6% |
| JIT stubs (vtable/itable) | 3504 | 3.3% |
| JDK invokes/VarHandle | 2811 | 2.7% |
| network (kernel) | 2801 | 2.7% |
| JDK other | 2039 | 1.9% |
| vdso (clock) | 132 | 0.1% |
| bukkit api | 106 | 0.1% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| craftbukkit glue | 51 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50697 | 48.4% |
| phase: unclassified | 33410 | 31.9% |
| phase: main tick (unclassified) | 13059 | 12.5% |
| phase: chunk tick | 2310 | 2.2% |
| phase: network sync (ServerEntity) | 2197 | 2.1% |
| phase: chunk system (off-main worker) | 1177 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1065 | 1.0% |
| phase: random tick | 527 | 0.5% |
| phase: mob spawning | 293 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92512** (88.3%) · native/JVM-internal **12123** (11.6%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3952 | 3.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3128 | 3.0% |
| `vtable stub` | native/JVM-internal | 2958 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2474 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1947 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1545 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1488 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1471 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1301 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1272 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1271 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1189 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1089 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1046 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 979 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 973 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 951 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 950 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 931 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 918 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 913 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 851 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 828 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 799 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 795 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 791 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 786 | 0.8% |
| `colpush_tick` | native/JVM-internal | 774 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 725 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 723 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 710 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 708 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 699 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 685 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 684 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 684 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 658 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63662)

| bucket | self-time samples | share |
|---|---|---|
| other | 60643 | 95.3% |
| entities/mobs (kernel) | 1101 | 1.7% |
| kernel: other | 721 | 1.1% |
| chunk system (kernel) | 229 | 0.4% |
| JDK collections | 221 | 0.3% |
| JIT stubs (vtable/itable) | 175 | 0.3% |
| fastutil collections | 156 | 0.2% |
| moonrise/paper patches | 142 | 0.2% |
| network (kernel) | 94 | 0.1% |
| JDK invokes/VarHandle | 91 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60981 | 95.8% |
| phase: entity tick (AI/movement) | 1899 | 3.0% |
| phase: main tick (unclassified) | 504 | 0.8% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54750** (86.0%) · native/JVM-internal **8911** (14.0%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51811 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.5% |
| `read` | native/JVM-internal | 1230 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 157 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 49 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 35 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3276)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3276 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1930 | 58.9% |
| phase: entity tick (AI/movement) | 999 | 30.5% |
| phase: main tick (unclassified) | 248 | 7.6% |
| phase: chunk system (off-main worker) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 26 | 0.8% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: mob spawning | 13 | 0.4% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3276** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 487 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 433 | 13.2% |
| `char[]_[k]` | other | 325 | 9.9% |
| `byte[]_[k]` | other | 191 | 5.8% |
| `long[]_[i]` | other | 134 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 133 | 4.1% |
| `java.util.ArrayList_[i]` | other | 116 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 111 | 3.4% |
| `byte[]_[i]` | other | 102 | 3.1% |
| `int[]_[i]` | other | 101 | 3.1% |
| `java.lang.Object[]_[i]` | other | 101 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.1% |
| `java.math.BigInteger_[i]` | other | 30 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd2159fb2c8_[i]` | other | 28 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 24 | 0.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 24 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104735 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19223 | 18.35% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6201 | 5.92% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5167 | 4.93% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3917 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1375 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 572 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 381 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 358 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 331 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 271 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 266 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 97 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 487 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 433 | 13.2% |
| `char[]_[k]` | 325 | 9.9% |
| `byte[]_[k]` | 191 | 5.8% |
| `long[]_[i]` | 134 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 133 | 4.1% |
| `java.util.ArrayList_[i]` | 116 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 111 | 3.4% |
| `byte[]_[i]` | 102 | 3.1% |
| `int[]_[i]` | 101 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 111 pauses / total 20532 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148261..153818 (delta 5557, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99920->106942, minecraft:husk 4574->5510, minecraft:drowned 3621->4552, minecraft:zombie 3705->4607, minecraft:skeleton 4263->4789, minecraft:creeper 4525->4998, minecraft:spider 4124->4485, minecraft:pig 2818->3176
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5557)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54731454 B)
- `wall-collapsed.txt` (3329692 B)
- `alloc-collapsed.txt` (1882601 B)
- `cpu-flamegraph.html` (281482 B)
- `server-stdout.log` (326187 B)
- `gc.log` (105790 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
