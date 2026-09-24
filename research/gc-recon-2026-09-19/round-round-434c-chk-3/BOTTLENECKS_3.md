# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 11.737 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.6, 2.5, 2.8, 3.2, 3.3, 3.2]
- spark tick-monitor MSPT: avg **377.22ms** / min 264.6ms / max **484.02ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:48:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8675334 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:50:14 INFO]: [crussty-plugin] [cruss | 317.19 | — | — | — | 484.02 | 377.22 |

- entity totals seen: [151327, 153253, 153852]
- top entity types (max seen): minecraft:item×107102, minecraft:husk×5475, minecraft:creeper×5049, minecraft:skeleton×4792, minecraft:drowned×4791, minecraft:zombie×4555, minecraft:spider×4465, minecraft:sheep×3535, minecraft:chicken×3428, minecraft:cow×3322, minecraft:pig×3189, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/zm2lo59Xt1
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **137** (Full GC: **7**)
- total pause: **15706.5 ms**, avg **114.65 ms**, max **1386.1 ms**
- heap high-water seen: **7959 MB** -> last-after: **4680 MB**
  - Young (Allocation Failure): 120
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2

### CPU profile — self-time by research bucket (total self-time samples: 101657)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32654 | 32.1% |
| kernel: other | 20460 | 20.1% |
| other | 11045 | 10.9% |
| chunk system (kernel) | 8217 | 8.1% |
| JDK collections | 7402 | 7.3% |
| moonrise/paper patches | 5549 | 5.5% |
| fastutil collections | 4613 | 4.5% |
| network (kernel) | 3352 | 3.3% |
| JIT stubs (vtable/itable) | 3312 | 3.3% |
| JDK invokes/VarHandle | 2416 | 2.4% |
| JDK other | 2063 | 2.0% |
| vdso (clock) | 160 | 0.2% |
| redstone (kernel) | 158 | 0.2% |
| block entities/hoppers (kernel) | 125 | 0.1% |
| bukkit api | 56 | 0.1% |
| craftbukkit glue | 53 | 0.1% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48842 | 48.0% |
| phase: unclassified | 31148 | 30.6% |
| phase: main tick (unclassified) | 12715 | 12.5% |
| phase: chunk tick | 3057 | 3.0% |
| phase: network sync (ServerEntity) | 2650 | 2.6% |
| phase: chunk system (off-main worker) | 1331 | 1.3% |
| phase: block entities (hoppers/furnaces) | 990 | 1.0% |
| phase: random tick | 596 | 0.6% |
| phase: mob spawning | 325 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89690** (88.2%) · native/JVM-internal **11867** (11.7%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3953 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2878 | 2.8% |
| `vtable stub` | native/JVM-internal | 2759 | 2.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2607 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1860 | 1.8% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1714 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1562 | 1.5% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1515 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1491 | 1.5% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1328 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1323 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1295 | 1.3% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1211 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1207 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1179 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1171 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1107 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1081 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1065 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1015 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1004 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 909 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 905 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 879 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 868 | 0.9% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 854 | 0.8% |
| `colpush_tick` | native/JVM-internal | 839 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 821 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 796 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 796 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 782 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 779 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 768 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 704 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 684 | 0.7% |
| `java/lang/ThreadLocal.get` | JVM-Java | 648 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 626 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 618 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61933 | 95.5% |
| entities/mobs (kernel) | 969 | 1.5% |
| kernel: other | 674 | 1.0% |
| chunk system (kernel) | 246 | 0.4% |
| JDK collections | 235 | 0.4% |
| moonrise/paper patches | 190 | 0.3% |
| JIT stubs (vtable/itable) | 174 | 0.3% |
| fastutil collections | 147 | 0.2% |
| network (kernel) | 116 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 76 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62210 | 95.9% |
| phase: entity tick (AI/movement) | 1905 | 2.9% |
| phase: main tick (unclassified) | 412 | 0.6% |
| phase: chunk tick | 143 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 16 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55975** (86.3%) · native/JVM-internal **8877** (13.7%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53165 | 82.0% |
| `clock_nanosleep` | native/JVM-internal | 4781 | 7.4% |
| `read` | native/JVM-internal | 1220 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 154 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 123 | 0.2% |
| `syscall` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 39 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4293)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4293 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2468 | 57.5% |
| phase: entity tick (AI/movement) | 1337 | 31.1% |
| phase: main tick (unclassified) | 327 | 7.6% |
| phase: chunk system (off-main worker) | 64 | 1.5% |
| phase: block entities (hoppers/furnaces) | 36 | 0.8% |
| phase: network sync (ServerEntity) | 34 | 0.8% |
| phase: mob spawning | 14 | 0.3% |
| phase: chunk tick | 10 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4293** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 670 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 485 | 11.3% |
| `char[]_[k]` | other | 449 | 10.5% |
| `byte[]_[k]` | other | 262 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 175 | 4.1% |
| `long[]_[i]` | other | 144 | 3.4% |
| `java.lang.Object[]_[i]` | other | 137 | 3.2% |
| `java.util.ArrayList_[i]` | other | 136 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 115 | 2.7% |
| `byte[]_[i]` | other | 102 | 2.4% |
| `int[]_[i]` | other | 101 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 83 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.5% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f219f83b008_[i]` | other | 59 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 53 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 42 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 40 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f219fa779d0_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 101657 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18585 | 18.28% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6078 | 5.98% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5115 | 5.03% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3867 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1085 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 579 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 410 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 379 | 0.37% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 316 | 0.31% |
| `net/minecraft/world/entity/npc/Villager.tick` | 300 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 198 | 0.19% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 91 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 670 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | 485 | 11.3% |
| `char[]_[k]` | 449 | 10.5% |
| `byte[]_[k]` | 262 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 175 | 4.1% |
| `long[]_[i]` | 144 | 3.4% |
| `java.lang.Object[]_[i]` | 137 | 3.2% |
| `java.util.ArrayList_[i]` | 136 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 115 | 2.7% |
| `byte[]_[i]` | 102 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 137 pauses / total 15707 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148085..153852 (delta 5767, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99955->107102, minecraft:drowned 3457->4791, minecraft:husk 4416->5475, minecraft:zombie 3644->4555, minecraft:skeleton 4220->4792, minecraft:creeper 4630->5049, minecraft:chicken 3076->3428, minecraft:spider 4126->4465
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5767)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45214823 B)
- `wall-collapsed.txt` (2944097 B)
- `alloc-collapsed.txt` (2053785 B)
- `cpu-flamegraph.html` (259891 B)
- `server-stdout.log` (318223 B)
- `gc.log` (126291 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
