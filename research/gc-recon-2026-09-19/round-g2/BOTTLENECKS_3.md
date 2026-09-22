# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.568 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.1, 1.6, 2.0, 2.2, 2.5, 2.5]
- spark tick-monitor MSPT: avg **414.09ms** / min 345.87ms / max **541.69ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T14:30:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7613553 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [crussty-plugin] cmp415_gsel2: ARMED hea | 345.87 | — | — | — | 541.69 | 414.09 |

- entity totals seen: [149030, 150379, 151489]
- top entity types (max seen): minecraft:item×103444, minecraft:creeper×5227, minecraft:husk×5207, minecraft:skeleton×4864, minecraft:spider×4844, minecraft:zombie×4681, minecraft:drowned×4559, minecraft:sheep×3531, minecraft:chicken×3423, minecraft:cow×3335, minecraft:pig×3216, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/bCNNALfPbD
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **8**)
- total pause: **23970.0 ms**, avg **204.87 ms**, max **3023.7 ms**
- heap high-water seen: **7696 MB** -> last-after: **3982 MB**
  - Young (Allocation Failure): 99
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115033)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30794 | 26.8% |
| kernel: other | 27250 | 23.7% |
| other | 14713 | 12.8% |
| chunk system (kernel) | 10334 | 9.0% |
| moonrise/paper patches | 8159 | 7.1% |
| fastutil collections | 6566 | 5.7% |
| JDK collections | 6220 | 5.4% |
| JIT stubs (vtable/itable) | 3159 | 2.7% |
| network (kernel) | 2514 | 2.2% |
| JDK invokes/VarHandle | 2245 | 2.0% |
| JDK other | 2100 | 1.8% |
| JVM internals (GC oop barriers) | 528 | 0.5% |
| vdso (clock) | 164 | 0.1% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92597 | 80.5% |
| phase: unclassified | 12943 | 11.3% |
| phase: main tick (unclassified) | 3288 | 2.9% |
| phase: chunk tick | 2138 | 1.9% |
| phase: network sync (ServerEntity) | 1894 | 1.6% |
| phase: chunk system (off-main worker) | 997 | 0.9% |
| phase: block entities (hoppers/furnaces) | 643 | 0.6% |
| phase: random tick | 386 | 0.3% |
| phase: mob spawning | 145 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99890** (86.8%) · native/JVM-internal **14987** (13.0%) · other **156** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4801 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3491 | 3.0% |
| `vtable stub` | native/JVM-internal | 2727 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2392 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2095 | 1.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1741 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1728 | 1.5% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1652 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1640 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1618 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1611 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1456 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1443 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1379 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1317 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1242 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1154 | 1.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1147 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1072 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1014 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 977 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 946 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 848 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 847 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 843 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 843 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 822 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 813 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 812 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalBatchOps.tickGateInner` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 771 | 0.7% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 766 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 743 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 734 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 695 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 672 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 662 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 592 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61241)

| bucket | self-time samples | share |
|---|---|---|
| other | 57952 | 94.6% |
| entities/mobs (kernel) | 1040 | 1.7% |
| kernel: other | 821 | 1.3% |
| chunk system (kernel) | 338 | 0.6% |
| moonrise/paper patches | 275 | 0.4% |
| JDK collections | 208 | 0.3% |
| fastutil collections | 208 | 0.3% |
| JIT stubs (vtable/itable) | 124 | 0.2% |
| network (kernel) | 93 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 81 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57703 | 94.2% |
| phase: entity tick (AI/movement) | 3129 | 5.1% |
| phase: main tick (unclassified) | 169 | 0.3% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 34 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52348** (85.5%) · native/JVM-internal **8882** (14.5%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49070 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4754 | 7.8% |
| `read` | native/JVM-internal | 1209 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 160 | 0.3% |
| `vtable stub` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 61 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4029)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4029 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2069 | 51.4% |
| phase: unclassified | 1799 | 44.7% |
| phase: main tick (unclassified) | 91 | 2.3% |
| phase: chunk system (off-main worker) | 45 | 1.1% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 5 | 0.1% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4029** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 505 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 502 | 12.5% |
| `char[]_[k]` | other | 448 | 11.1% |
| `byte[]_[k]` | other | 225 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 189 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 174 | 4.3% |
| `byte[]_[i]` | other | 146 | 3.6% |
| `long[]_[i]` | other | 126 | 3.1% |
| `java.util.ArrayList_[i]` | other | 122 | 3.0% |
| `java.lang.Object[]_[i]` | other | 98 | 2.4% |
| `java.util.GregorianCalendar_[i]` | other | 83 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.0% |
| `int[]_[i]` | other | 61 | 1.5% |
| `int[]_[k]` | other | 56 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 45 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 41 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f5dc39e3910_[i]` | other | 35 | 0.9% |
| `java.util.Calendar$Builder_[k]` | other | 34 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115033 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34995 | 30.42% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22412 | 19.48% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6185 | 5.38% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5289 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4301 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1059 | 0.92% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 423 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 505 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | 502 | 12.5% |
| `char[]_[k]` | 448 | 11.1% |
| `byte[]_[k]` | 225 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 189 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 174 | 4.3% |
| `byte[]_[i]` | 146 | 3.6% |
| `long[]_[i]` | 126 | 3.1% |
| `java.util.ArrayList_[i]` | 122 | 3.0% |
| `java.lang.Object[]_[i]` | 98 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 23970 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148185..151489 (delta 3304, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99716->103444, minecraft:drowned 3416->4559, minecraft:zombie 3692->4681, minecraft:husk 4524->5207, minecraft:creeper 4550->5227, minecraft:spider 4247->4844, minecraft:skeleton 4384->4864, minecraft:chicken 3388->3423
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3304)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57460013 B)
- `wall-collapsed.txt` (3685705 B)
- `alloc-collapsed.txt` (2106403 B)
- `cpu-flamegraph.html` (280941 B)
- `server-stdout.log` (4663926 B)
- `gc.log` (110127 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
