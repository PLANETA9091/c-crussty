# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.324 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 1.7, 2.0, 2.3, 2.7, 2.7]
- spark tick-monitor MSPT: avg **387.02ms** / min 338.06ms / max **502.59ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T14:00:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7014414 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 5 (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
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
| spark tickmonitor (whole run, [⚡] lines) | 338.06 | — | — | — | 502.59 | 387.02 |

- entity totals seen: [148973, 150587, 151330]
- top entity types (max seen): minecraft:item×103338, minecraft:creeper×5199, minecraft:husk×5123, minecraft:skeleton×4873, minecraft:spider×4859, minecraft:zombie×4653, minecraft:drowned×4556, minecraft:sheep×3530, minecraft:chicken×3411, minecraft:cow×3377, minecraft:pig×3241, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/spYzUGQj2u
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **20856.2 ms**, avg **172.37 ms**, max **2406.4 ms**
- heap high-water seen: **7776 MB** -> last-after: **3646 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116376)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29419 | 25.3% |
| kernel: other | 27593 | 23.7% |
| other | 13120 | 11.3% |
| chunk system (kernel) | 10255 | 8.8% |
| moonrise/paper patches | 9959 | 8.6% |
| fastutil collections | 7511 | 6.5% |
| JDK collections | 6011 | 5.2% |
| JIT stubs (vtable/itable) | 3891 | 3.3% |
| network (kernel) | 3128 | 2.7% |
| JDK invokes/VarHandle | 2316 | 2.0% |
| JDK other | 2080 | 1.8% |
| JVM internals (GC oop barriers) | 556 | 0.5% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95164 | 81.8% |
| phase: unclassified | 11522 | 9.9% |
| phase: main tick (unclassified) | 3546 | 3.0% |
| phase: chunk tick | 1976 | 1.7% |
| phase: network sync (ServerEntity) | 1787 | 1.5% |
| phase: chunk system (off-main worker) | 1169 | 1.0% |
| phase: block entities (hoppers/furnaces) | 699 | 0.6% |
| phase: random tick | 396 | 0.3% |
| phase: mob spawning | 116 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102352** (87.9%) · native/JVM-internal **13912** (12.0%) · other **112** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4653 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3435 | 3.0% |
| `vtable stub` | native/JVM-internal | 3250 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2564 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1988 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1861 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1749 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1678 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1677 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1591 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1572 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1475 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1351 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1318 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1236 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1130 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1098 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1092 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 996 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 984 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 956 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 955 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 946 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 939 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 926 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 872 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 803 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 754 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 752 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 739 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 731 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 710 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 710 | 0.6% |
| `itable stub` | native/JVM-internal | 637 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61257)

| bucket | self-time samples | share |
|---|---|---|
| other | 57839 | 94.4% |
| entities/mobs (kernel) | 992 | 1.6% |
| kernel: other | 863 | 1.4% |
| moonrise/paper patches | 369 | 0.6% |
| chunk system (kernel) | 309 | 0.5% |
| fastutil collections | 247 | 0.4% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 168 | 0.3% |
| network (kernel) | 121 | 0.2% |
| JDK other | 89 | 0.1% |
| JDK invokes/VarHandle | 55 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57625 | 94.1% |
| phase: entity tick (AI/movement) | 3191 | 5.2% |
| phase: main tick (unclassified) | 192 | 0.3% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52378** (85.5%) · native/JVM-internal **8875** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49013 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `vtable stub` | native/JVM-internal | 141 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.2% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 74 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 60 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 43 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3671)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3671 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2194 | 59.8% |
| phase: unclassified | 1326 | 36.1% |
| phase: main tick (unclassified) | 91 | 2.5% |
| phase: chunk system (off-main worker) | 26 | 0.7% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: chunk tick | 6 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: random tick | 2 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3671** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 584 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 559 | 15.2% |
| `char[]_[k]` | other | 448 | 12.2% |
| `byte[]_[k]` | other | 223 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 191 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 169 | 4.6% |
| `long[]_[i]` | other | 140 | 3.8% |
| `java.util.ArrayList_[i]` | other | 112 | 3.1% |
| `java.lang.Object[]_[i]` | other | 99 | 2.7% |
| `byte[]_[i]` | other | 84 | 2.3% |
| `int[]_[i]` | other | 66 | 1.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.2% |
| `java.util.ArrayList$Itr_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd1959e6388_[i]` | other | 35 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 27 | 0.7% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fd1959dcdc0_[i]` | other | 25 | 0.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 22 | 0.6% |
| `java.lang.String_[i]` | other | 22 | 0.6% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 22 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116376 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35752 | 30.72% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23011 | 19.77% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6583 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5493 | 4.72% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4607 | 3.96% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1227 | 1.05% |
| `net/minecraft/world/entity/ai/Brain.tick` | 914 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 461 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 259 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 198 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 584 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | 559 | 15.2% |
| `char[]_[k]` | 448 | 12.2% |
| `byte[]_[k]` | 223 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 191 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 169 | 4.6% |
| `long[]_[i]` | 140 | 3.8% |
| `java.util.ArrayList_[i]` | 112 | 3.1% |
| `java.lang.Object[]_[i]` | 99 | 2.7% |
| `byte[]_[i]` | 84 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 20856 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148158..151330 (delta 3172, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99603->103338, minecraft:zombie 3635->4653, minecraft:drowned 3591->4556, minecraft:spider 4229->4859, minecraft:creeper 4578->5199, minecraft:husk 4508->5123, minecraft:skeleton 4439->4873, minecraft:chicken 3385->3411
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3172)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55748045 B)
- `wall-collapsed.txt` (3750324 B)
- `alloc-collapsed.txt` (2080209 B)
- `cpu-flamegraph.html` (290181 B)
- `server-stdout.log` (243141 B)
- `gc.log` (114441 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
