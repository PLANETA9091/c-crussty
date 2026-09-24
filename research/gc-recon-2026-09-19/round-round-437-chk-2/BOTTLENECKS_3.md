# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.905 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.1, 2.3, 2.6, 2.6]
- spark tick-monitor MSPT: avg **382.61ms** / min 324.41ms / max **528.57ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:52:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6866951 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:55:15 INFO]: [crussty-plugin] [cruss | 324.41 | — | — | — | 528.57 | 382.61 |

- entity totals seen: [150365, 152646, 153725]
- top entity types (max seen): minecraft:item×106740, minecraft:husk×5478, minecraft:creeper×5022, minecraft:skeleton×4773, minecraft:zombie×4633, minecraft:drowned×4575, minecraft:spider×4415, minecraft:sheep×3507, minecraft:chicken×3414, minecraft:cow×3362, minecraft:pig×3208, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xyi3aTNKLq
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **111** (Full GC: **9**)
- total pause: **19719.4 ms**, avg **177.65 ms**, max **2462.6 ms**
- heap high-water seen: **7605 MB** -> last-after: **3801 MB**
  - Young (Allocation Failure): 92
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104768)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33979 | 32.4% |
| kernel: other | 21920 | 20.9% |
| other | 11825 | 11.3% |
| chunk system (kernel) | 8456 | 8.1% |
| JDK collections | 7485 | 7.1% |
| moonrise/paper patches | 5145 | 4.9% |
| fastutil collections | 4827 | 4.6% |
| JIT stubs (vtable/itable) | 3623 | 3.5% |
| network (kernel) | 2875 | 2.7% |
| JDK invokes/VarHandle | 2283 | 2.2% |
| JDK other | 1845 | 1.8% |
| vdso (clock) | 142 | 0.1% |
| bukkit api | 105 | 0.1% |
| block entities/hoppers (kernel) | 102 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50629 | 48.3% |
| phase: unclassified | 33433 | 31.9% |
| phase: main tick (unclassified) | 13018 | 12.4% |
| phase: chunk tick | 2490 | 2.4% |
| phase: network sync (ServerEntity) | 2240 | 2.1% |
| phase: chunk system (off-main worker) | 1101 | 1.1% |
| phase: block entities (hoppers/furnaces) | 963 | 0.9% |
| phase: random tick | 589 | 0.6% |
| phase: mob spawning | 303 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92223** (88.0%) · native/JVM-internal **12444** (11.9%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3980 | 3.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3111 | 3.0% |
| `vtable stub` | native/JVM-internal | 2951 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2648 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1586 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1470 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1456 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1312 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1293 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1281 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1254 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1234 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1207 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1104 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1089 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1055 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1029 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1017 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 936 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 912 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 902 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 855 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 826 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 825 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 800 | 0.8% |
| `colpush_tick` | native/JVM-internal | 786 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 777 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 745 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 707 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 704 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 695 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 694 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 672 | 0.6% |
| `itable stub` | native/JVM-internal | 669 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 661 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 650 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 650 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64846)

| bucket | self-time samples | share |
|---|---|---|
| other | 61856 | 95.4% |
| entities/mobs (kernel) | 1126 | 1.7% |
| kernel: other | 673 | 1.0% |
| chunk system (kernel) | 241 | 0.4% |
| JDK collections | 237 | 0.4% |
| JIT stubs (vtable/itable) | 166 | 0.3% |
| moonrise/paper patches | 163 | 0.3% |
| fastutil collections | 151 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 60 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62205 | 95.9% |
| phase: entity tick (AI/movement) | 1889 | 2.9% |
| phase: main tick (unclassified) | 473 | 0.7% |
| phase: chunk tick | 107 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55978** (86.3%) · native/JVM-internal **8862** (13.7%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53081 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 137 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 112 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 41 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 37 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 36 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3247)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3247 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1875 | 57.7% |
| phase: entity tick (AI/movement) | 1006 | 31.0% |
| phase: main tick (unclassified) | 243 | 7.5% |
| phase: chunk system (off-main worker) | 47 | 1.4% |
| phase: block entities (hoppers/furnaces) | 38 | 1.2% |
| phase: network sync (ServerEntity) | 15 | 0.5% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3247** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 466 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 422 | 13.0% |
| `char[]_[k]` | other | 320 | 9.9% |
| `byte[]_[k]` | other | 204 | 6.3% |
| `long[]_[i]` | other | 138 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 126 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 109 | 3.4% |
| `java.lang.Object[]_[i]` | other | 105 | 3.2% |
| `java.util.ArrayList_[i]` | other | 99 | 3.0% |
| `byte[]_[i]` | other | 95 | 2.9% |
| `int[]_[i]` | other | 92 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 64 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.3% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f1d799fdc00_[i]` | other | 31 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.9% |
| `int[]_[k]` | other | 29 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.9% |
| `java.lang.String_[i]` | other | 27 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104768 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19274 | 18.40% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6115 | 5.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5295 | 5.05% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3847 | 3.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1395 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tick` | 584 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 427 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 400 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 333 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 328 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 262 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 85 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 466 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 422 | 13.0% |
| `char[]_[k]` | 320 | 9.9% |
| `byte[]_[k]` | 204 | 6.3% |
| `long[]_[i]` | 138 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 126 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 109 | 3.4% |
| `java.lang.Object[]_[i]` | 105 | 3.2% |
| `java.util.ArrayList_[i]` | 99 | 3.0% |
| `byte[]_[i]` | 95 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 111 pauses / total 19719 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148131..153725 (delta 5594, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99766->106740, minecraft:drowned 3681->4575, minecraft:husk 4586->5478, minecraft:zombie 3839->4633, minecraft:skeleton 4264->4773, minecraft:creeper 4531->5022, minecraft:spider 4048->4415, minecraft:pig 2862->3208
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5594)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55985688 B)
- `wall-collapsed.txt` (3182709 B)
- `alloc-collapsed.txt` (1828229 B)
- `cpu-flamegraph.html` (276672 B)
- `server-stdout.log` (331033 B)
- `gc.log` (105787 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
