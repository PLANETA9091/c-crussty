# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.12 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.2, 17.3, 16.9, 16.6, 16.7, 16.3]
- spark tick-monitor MSPT: avg **63.81ms** / min 44.53ms / max **136.79ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T17:27:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8700591 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:29:30 INFO]: [crussty-plugin] [cruss | 44.53 | — | — | — | 110.02 | 58.26 |

- entity totals seen: [12037, 11519, 10976]
- top entity types (max seen): minecraft:item_frame×2714, minecraft:armor_stand×1619, minecraft:item×1060, minecraft:husk×886, minecraft:sheep×860, minecraft:creeper×848, minecraft:chicken×772, minecraft:spider×720, minecraft:cow×677, minecraft:pig×632, minecraft:skeleton×401, minecraft:spruce_boat×285
- spark viewer report: https://spark.lucko.me/ZMmJFOWnjL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **203** (Full GC: **10**)
- total pause: **10846.8 ms**, avg **53.43 ms**, max **863.9 ms**
- heap high-water seen: **5298 MB** -> last-after: **4064 MB**
  - Young (Allocation Failure): 181
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 57176)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 14075 | 24.6% |
| entities/mobs (kernel) | 10428 | 18.2% |
| other | 9081 | 15.9% |
| chunk system (kernel) | 5383 | 9.4% |
| moonrise/paper patches | 4079 | 7.1% |
| JDK collections | 2392 | 4.2% |
| fastutil collections | 2251 | 3.9% |
| JDK invokes/VarHandle | 2000 | 3.5% |
| network (kernel) | 1798 | 3.1% |
| JDK other | 1537 | 2.7% |
| JIT stubs (vtable/itable) | 1433 | 2.5% |
| redstone (kernel) | 1007 | 1.8% |
| block entities/hoppers (kernel) | 798 | 1.4% |
| vdso (clock) | 251 | 0.4% |
| bukkit api | 247 | 0.4% |
| JVM internals (GC oop barriers) | 230 | 0.4% |
| craftbukkit glue | 102 | 0.2% |
| worldgen/noise (kernel) | 68 | 0.1% |
| tick scheduling (kernel) | 16 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 22469 | 39.3% |
| phase: unclassified | 9006 | 15.8% |
| phase: main tick (unclassified) | 8788 | 15.4% |
| phase: block entities (hoppers/furnaces) | 4426 | 7.7% |
| phase: chunk tick | 3284 | 5.7% |
| phase: random tick | 3270 | 5.7% |
| phase: mob spawning | 2866 | 5.0% |
| phase: network sync (ServerEntity) | 1574 | 2.8% |
| phase: chunk system (off-main worker) | 1485 | 2.6% |
| phase: scheduler/mid-tick tasks | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48134** (84.2%) · native/JVM-internal **8909** (15.6%) · other **133** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2199 | 3.8% |
| `net/minecraft/server/level/ServerLevel.optimiseRandomTick` | JVM-Java | 1229 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1067 | 1.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 969 | 1.7% |
| `vtable stub` | native/JVM-internal | 961 | 1.7% |
| `ca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom.advanceSeed` | JVM-Java | 944 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 798 | 1.4% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 742 | 1.3% |
| `java/lang/invoke/LambdaForm$MH.0x00007fbb95a72400.invoke` | JVM-Java | 691 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 689 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 658 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 645 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 600 | 1.0% |
| `net/minecraft/world/level/Level.tickBlockEntities` | JVM-Java | 557 | 1.0% |
| `net/minecraft/server/level/ServerLevel.tick` | JVM-Java | 547 | 1.0% |
| `net/minecraft/world/level/redstone/DefaultRedstoneWireEvaluator.updatePowerStrength` | JVM-Java | 515 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 505 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 483 | 0.8% |
| `net/minecraft/server/level/ServerLevel.tickChunk` | JVM-Java | 482 | 0.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 471 | 0.8% |
| `itable stub` | native/JVM-internal | 468 | 0.8% |
| `net/minecraft/world/level/NaturalSpawner.createState` | JVM-Java | 467 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 457 | 0.8% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 452 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 448 | 0.8% |
| `net/minecraft/world/level/block/entity/BlockEntity.isRemoved` | JVM-Java | 447 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 426 | 0.7% |
| `net/minecraft/world/level/redstone/NeighborUpdater.executeUpdate` | JVM-Java | 413 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 402 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 395 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 395 | 0.7% |
| `read` | native/JVM-internal | 395 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 387 | 0.7% |
| `colpush_tick` | native/JVM-internal | 384 | 0.7% |
| `sscan_epoch` | native/JVM-internal | 378 | 0.7% |
| `java/lang/String.equals` | JVM-Java | 372 | 0.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 367 | 0.6% |
| `net/minecraft/world/entity/Entity.push` | JVM-Java | 364 | 0.6% |
| `net/minecraft/world/level/pathfinder/Path.isDone` | JVM-Java | 357 | 0.6% |
| `net/minecraft/world/level/block/SpreadingSnowyDirtBlock.randomTick` | JVM-Java | 341 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64856)

| bucket | self-time samples | share |
|---|---|---|
| other | 63421 | 97.8% |
| kernel: other | 467 | 0.7% |
| entities/mobs (kernel) | 305 | 0.5% |
| chunk system (kernel) | 144 | 0.2% |
| moonrise/paper patches | 131 | 0.2% |
| JDK invokes/VarHandle | 72 | 0.1% |
| fastutil collections | 65 | 0.1% |
| JDK other | 51 | 0.1% |
| JDK collections | 51 | 0.1% |
| network (kernel) | 41 | 0.1% |
| JIT stubs (vtable/itable) | 30 | 0.0% |
| redstone (kernel) | 28 | 0.0% |
| block entities/hoppers (kernel) | 28 | 0.0% |
| bukkit api | 10 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 63213 | 97.5% |
| phase: entity tick (AI/movement) | 608 | 0.9% |
| phase: main tick (unclassified) | 506 | 0.8% |
| phase: block entities (hoppers/furnaces) | 148 | 0.2% |
| phase: random tick | 110 | 0.2% |
| phase: chunk tick | 99 | 0.2% |
| phase: mob spawning | 77 | 0.1% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: network sync (ServerEntity) | 44 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56235** (86.7%) · native/JVM-internal **8615** (13.3%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 54793 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1217 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/server/level/ServerLevel.tick` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.optimiseRandomTick` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 37 | 0.1% |
| `java/lang/invoke/LambdaForm$MH.0x00007fbb95a72400.invoke` | JVM-Java | 35 | 0.1% |
| `ca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom.advanceSeed` | JVM-Java | 35 | 0.1% |
| `net/minecraft/world/level/NaturalSpawner.createState` | JVM-Java | 31 | 0.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 26 | 0.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 23 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 20 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 20 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 20 | 0.0% |
| `getdents64` | native/JVM-internal | 20 | 0.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 19 | 0.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 18 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 12311)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 12311 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 6392 | 51.9% |
| phase: entity tick (AI/movement) | 4046 | 32.9% |
| phase: main tick (unclassified) | 1189 | 9.7% |
| phase: mob spawning | 238 | 1.9% |
| phase: chunk system (off-main worker) | 227 | 1.8% |
| phase: block entities (hoppers/furnaces) | 150 | 1.2% |
| phase: chunk tick | 36 | 0.3% |
| phase: random tick | 23 | 0.2% |
| phase: network sync (ServerEntity) | 10 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **12311** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[i]` | other | 1584 | 12.9% |
| `java.lang.Object[]_[i]` | other | 900 | 7.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 886 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 756 | 6.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 631 | 5.1% |
| `byte[]_[k]` | other | 479 | 3.9% |
| `char[]_[k]` | other | 473 | 3.8% |
| `java.util.ArrayList_[i]` | other | 393 | 3.2% |
| `java.util.ArrayList$Itr_[i]` | other | 317 | 2.6% |
| `org.bukkit.event.vehicle.VehicleEntityCollisionEvent_[i]` | other | 317 | 2.6% |
| `int[]_[i]` | other | 291 | 2.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 263 | 2.1% |
| `long[]_[i]` | other | 255 | 2.1% |
| `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator_[i]` | other | 230 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 227 | 1.8% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fbb95a566a8_[i]` | other | 184 | 1.5% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 169 | 1.4% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fbb95835008_[i]` | other | 156 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 148 | 1.2% |
| `org.bukkit.craftbukkit.block.CraftBlock_[i]` | other | 146 | 1.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 57176 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 6044 | 10.57% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 1874 | 3.28% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1805 | 3.16% |
| `net/minecraft/world/entity/monster/Spider.tick` | 855 | 1.50% |
| `net/minecraft/world/entity/ai/Brain.tick` | 563 | 0.98% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 538 | 0.94% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 334 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 322 | 0.56% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 284 | 0.50% |
| `net/minecraft/world/entity/npc/Villager.tick` | 207 | 0.36% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 155 | 0.27% |
| `net/minecraft/world/entity/decoration/BlockAttachedEntity.tick` | 146 | 0.26% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[i]` | 1584 | 12.9% |
| `java.lang.Object[]_[i]` | 900 | 7.3% |
| `net.minecraft.core.BlockPos_[i]` | 886 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 756 | 6.1% |
| `net.minecraft.world.phys.AABB_[i]` | 631 | 5.1% |
| `byte[]_[k]` | 479 | 3.9% |
| `char[]_[k]` | 473 | 3.8% |
| `java.util.ArrayList_[i]` | 393 | 3.2% |
| `java.util.ArrayList$Itr_[i]` | 317 | 2.6% |
| `org.bukkit.event.vehicle.VehicleEntityCollisionEvent_[i]` | 317 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 203 pauses / total 10847 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=10976..13356 (delta 2380, churn 19.7%), summons=0
  - top movers (max-min across polls): minecraft:husk 454->886, minecraft:creeper 435->848, minecraft:spider 379->720, minecraft:arrow 0->276, minecraft:skeleton 253->401, minecraft:pig 515->632, minecraft:item 947->1060, minecraft:sheep 762->860
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2380)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (30477032 B)
- `wall-collapsed.txt` (1391707 B)
- `alloc-collapsed.txt` (3567424 B)
- `cpu-flamegraph.html` (220414 B)
- `server-stdout.log` (427580208 B)
- `gc.log` (184761 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
