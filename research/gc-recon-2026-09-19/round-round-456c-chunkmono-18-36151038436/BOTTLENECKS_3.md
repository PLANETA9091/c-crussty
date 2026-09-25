# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.394 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 2.3, 2.7, 3.2, 3.2, 1.9]
- spark tick-monitor MSPT: avg **385.88ms** / min 266.48ms / max **539.92ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T15:04:41Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8988099 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:06:37 INFO]: [crussty-plugin] [cruss | 326.32 | — | — | — | 539.92 | 385.88 |

- entity totals seen: [151036, 153503, 153984]
- top entity types (max seen): minecraft:item×107298, minecraft:husk×5478, minecraft:creeper×5025, minecraft:skeleton×4818, minecraft:zombie×4568, minecraft:drowned×4538, minecraft:spider×4471, minecraft:sheep×3536, minecraft:chicken×3431, minecraft:cow×3320, minecraft:pig×3189, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KbO3krpBmy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **140** (Full GC: **10**)
- total pause: **19913.8 ms**, avg **142.24 ms**, max **2253.9 ms**
- heap high-water seen: **7682 MB** -> last-after: **3855 MB**
  - Young (Allocation Failure): 120
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103298)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34111 | 33.0% |
| kernel: other | 20137 | 19.5% |
| other | 12431 | 12.0% |
| chunk system (kernel) | 7981 | 7.7% |
| JDK collections | 7328 | 7.1% |
| moonrise/paper patches | 5455 | 5.3% |
| fastutil collections | 4564 | 4.4% |
| network (kernel) | 3252 | 3.1% |
| JDK other | 2484 | 2.4% |
| JDK invokes/VarHandle | 2344 | 2.3% |
| JIT stubs (vtable/itable) | 2287 | 2.2% |
| JVM internals (GC oop barriers) | 469 | 0.5% |
| vdso (clock) | 129 | 0.1% |
| block entities/hoppers (kernel) | 111 | 0.1% |
| bukkit api | 67 | 0.1% |
| redstone (kernel) | 65 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49156 | 47.6% |
| phase: unclassified | 33189 | 32.1% |
| phase: main tick (unclassified) | 12511 | 12.1% |
| phase: chunk tick | 2747 | 2.7% |
| phase: network sync (ServerEntity) | 2718 | 2.6% |
| phase: chunk system (off-main worker) | 1195 | 1.2% |
| phase: block entities (hoppers/furnaces) | 926 | 0.9% |
| phase: random tick | 553 | 0.5% |
| phase: mob spawning | 301 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90816** (87.9%) · native/JVM-internal **12392** (12.0%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4027 | 3.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3474 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2893 | 2.8% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2227 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1912 | 1.9% |
| `vtable stub` | native/JVM-internal | 1732 | 1.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1499 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1468 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1460 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1331 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1276 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1253 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1234 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1231 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1146 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1096 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1089 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1027 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1019 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 993 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 980 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 924 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 906 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 894 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 854 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 818 | 0.8% |
| `colpush_tick` | native/JVM-internal | 809 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 780 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 773 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 767 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 733 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 710 | 0.7% |
| `java/lang/ThreadLocal.get` | JVM-Java | 673 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 631 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 629 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 625 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 618 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 595 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63648)

| bucket | self-time samples | share |
|---|---|---|
| other | 60749 | 95.4% |
| entities/mobs (kernel) | 1098 | 1.7% |
| kernel: other | 657 | 1.0% |
| chunk system (kernel) | 243 | 0.4% |
| JDK collections | 223 | 0.4% |
| moonrise/paper patches | 179 | 0.3% |
| fastutil collections | 126 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JIT stubs (vtable/itable) | 104 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 67 | 0.1% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60967 | 95.8% |
| phase: entity tick (AI/movement) | 1931 | 3.0% |
| phase: main tick (unclassified) | 420 | 0.7% |
| phase: chunk tick | 121 | 0.2% |
| phase: network sync (ServerEntity) | 87 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54839** (86.2%) · native/JVM-internal **8808** (13.8%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51982 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.5% |
| `read` | native/JVM-internal | 1221 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 120 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 113 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.1% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 75 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3984)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3984 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2298 | 57.7% |
| phase: entity tick (AI/movement) | 1250 | 31.4% |
| phase: main tick (unclassified) | 311 | 7.8% |
| phase: chunk system (off-main worker) | 48 | 1.2% |
| phase: network sync (ServerEntity) | 29 | 0.7% |
| phase: block entities (hoppers/furnaces) | 19 | 0.5% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3984** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 593 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 505 | 12.7% |
| `char[]_[k]` | other | 440 | 11.0% |
| `byte[]_[k]` | other | 258 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 170 | 4.3% |
| `long[]_[i]` | other | 160 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 150 | 3.8% |
| `java.util.ArrayList_[i]` | other | 121 | 3.0% |
| `java.lang.Object[]_[i]` | other | 110 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 98 | 2.5% |
| `byte[]_[i]` | other | 88 | 2.2% |
| `int[]_[i]` | other | 84 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 55 | 1.4% |
| `java.util.ImmutableCollections$List12_[i]` | other | 39 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f10cdaae870_[i]` | other | 37 | 0.9% |
| `int[]_[k]` | other | 35 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103298 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18677 | 18.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5951 | 5.76% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5062 | 4.90% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3916 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1076 | 1.04% |
| `net/minecraft/world/entity/ai/Brain.tick` | 612 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 420 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 391 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 335 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 286 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 212 | 0.21% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 108 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 593 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 505 | 12.7% |
| `char[]_[k]` | 440 | 11.0% |
| `byte[]_[k]` | 258 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 170 | 4.3% |
| `long[]_[i]` | 160 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 150 | 3.8% |
| `java.util.ArrayList_[i]` | 121 | 3.0% |
| `java.lang.Object[]_[i]` | 110 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | 98 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 140 pauses / total 19914 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148111..153984 (delta 5873, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99878->107298, minecraft:drowned 3420->4538, minecraft:zombie 3538->4568, minecraft:husk 4656->5478, minecraft:skeleton 4149->4818, minecraft:creeper 4646->5025, minecraft:chicken 3079->3431, minecraft:pig 2840->3189
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5873)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48332257 B)
- `wall-collapsed.txt` (2967513 B)
- `alloc-collapsed.txt` (1999465 B)
- `cpu-flamegraph.html` (263574 B)
- `server-stdout.log` (323173 B)
- `gc.log` (131563 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
