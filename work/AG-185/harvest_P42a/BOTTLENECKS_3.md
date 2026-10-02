# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 11.975 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [25.3, 2.2, 2.6, 3.2, 3.1, 3.3]
- spark tick-monitor MSPT: avg **395.75ms** / min 265.29ms / max **845.7ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-10-01T12:21:05Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 11196385 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384 + S99-gcw ROUND-468-S15; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity; 6 = ParallelGC + MetaspaceSize=256M + ReservedCodeCacheSize=512M — threshold-cascade kill, javap-neutral)
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
- population_seed: 523037 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)
- lever_flag=cmp421_brain lever_arg=p42m (MEGA-ROUND lever A/B marker, TASK-395; empty = vanilla bit-in-bit — canary-gate SKIP-ARMED/C85 layer-2 reads this)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| [12:22:46 INFO]: [crussty-plugin] [cruss | 328.58 | — | — | — | 845.7 | 395.75 |

- entity totals seen: [149689, 150938, 151254]
- top entity types (max seen): minecraft:item×103351, minecraft:creeper×5259, minecraft:husk×5109, minecraft:spider×4844, minecraft:skeleton×4787, minecraft:zombie×4633, minecraft:drowned×4475, minecraft:chicken×3553, minecraft:sheep×3410, minecraft:cow×3378, minecraft:pig×3201, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/V5FWuOeNz9
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **4430** (Full GC: **12**)
- total pause: **44244.4 ms**, avg **9.99 ms**, max **2162.1 ms**
- heap high-water seen: **10203 MB** -> last-after: **4099 MB**
  - Young (Allocation Failure): 4407
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Full (Ergonomics): 2

### CPU profile — self-time by research bucket (total self-time samples: 106289)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31554 | 29.7% |
| kernel: other | 20969 | 19.7% |
| moonrise/paper patches | 9393 | 8.8% |
| chunk system (kernel) | 9336 | 8.8% |
| other | 8835 | 8.3% |
| fastutil collections | 7381 | 6.9% |
| JDK collections | 7347 | 6.9% |
| network (kernel) | 3713 | 3.5% |
| JDK invokes/VarHandle | 2769 | 2.6% |
| JDK other | 2397 | 2.3% |
| JIT stubs (vtable/itable) | 2147 | 2.0% |
| block entities/hoppers (kernel) | 144 | 0.1% |
| vdso (clock) | 119 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| bukkit api | 51 | 0.0% |
| redstone (kernel) | 33 | 0.0% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61388 | 57.8% |
| phase: unclassified | 26633 | 25.1% |
| phase: main tick (unclassified) | 9837 | 9.3% |
| phase: network sync (ServerEntity) | 2702 | 2.5% |
| phase: chunk tick | 2472 | 2.3% |
| phase: chunk system (off-main worker) | 1350 | 1.3% |
| phase: block entities (hoppers/furnaces) | 1070 | 1.0% |
| phase: random tick | 669 | 0.6% |
| phase: mob spawning | 165 | 0.2% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97231** (91.5%) · native/JVM-internal **8949** (8.4%) · other **109** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4531 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2958 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2835 | 2.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2757 | 2.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2387 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2299 | 2.2% |
| `net/minecraft/world/entity/ai/goal/WrappedGoal.canUse` | JVM-Java | 1888 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1812 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1713 | 1.6% |
| `vtable stub` | native/JVM-internal | 1654 | 1.6% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1438 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1419 | 1.3% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1393 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1349 | 1.3% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 1319 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1272 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1244 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1173 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1129 | 1.1% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 1104 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1077 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1052 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 963 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 890 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 877 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 872 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 871 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 870 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 867 | 0.8% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 830 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 829 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 825 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 799 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 794 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 786 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 780 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 717 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 63670)

| bucket | self-time samples | share |
|---|---|---|
| other | 60453 | 94.9% |
| entities/mobs (kernel) | 1105 | 1.7% |
| kernel: other | 649 | 1.0% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 268 | 0.4% |
| fastutil collections | 258 | 0.4% |
| JDK collections | 217 | 0.3% |
| network (kernel) | 125 | 0.2% |
| JDK invokes/VarHandle | 89 | 0.1% |
| JDK other | 79 | 0.1% |
| JIT stubs (vtable/itable) | 78 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60680 | 95.3% |
| phase: entity tick (AI/movement) | 2341 | 3.7% |
| phase: main tick (unclassified) | 329 | 0.5% |
| phase: network sync (ServerEntity) | 99 | 0.2% |
| phase: chunk tick | 94 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 43 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54925** (86.3%) · native/JVM-internal **8741** (13.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51741 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.5% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 113 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 84 | 0.1% |
| `net/minecraft/world/entity/ai/goal/WrappedGoal.canUse` | JVM-Java | 83 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `syscall` | native/JVM-internal | 63 | 0.1% |
| `vtable stub` | native/JVM-internal | 61 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4177)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4177 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2183 | 52.3% |
| phase: entity tick (AI/movement) | 1600 | 38.3% |
| phase: main tick (unclassified) | 308 | 7.4% |
| phase: chunk system (off-main worker) | 34 | 0.8% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: random tick | 6 | 0.1% |
| phase: chunk tick | 6 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4177** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 625 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 539 | 12.9% |
| `char[]_[k]` | other | 439 | 10.5% |
| `byte[]_[k]` | other | 251 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 170 | 4.1% |
| `long[]_[i]` | other | 156 | 3.7% |
| `java.util.ArrayList_[i]` | other | 137 | 3.3% |
| `java.lang.Object[]_[i]` | other | 135 | 3.2% |
| `byte[]_[i]` | other | 88 | 2.1% |
| `int[]_[i]` | other | 84 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 49 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 48 | 1.1% |
| `int[]_[k]` | other | 43 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 40 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106289 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24606 | 23.15% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7039 | 6.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6132 | 5.77% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 5419 | 5.10% |
| `net/minecraft/world/entity/ai/Brain.tick` | 926 | 0.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 899 | 0.85% |
| `net/minecraft/world/entity/npc/Villager.tick` | 445 | 0.42% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 188 | 0.18% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 157 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickSensors` | 151 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 625 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | 539 | 12.9% |
| `char[]_[k]` | 439 | 10.5% |
| `byte[]_[k]` | 251 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 170 | 4.1% |
| `long[]_[i]` | 156 | 3.7% |
| `java.util.ArrayList_[i]` | 137 | 3.3% |
| `java.lang.Object[]_[i]` | 135 | 3.2% |
| `byte[]_[i]` | 88 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 4430 pauses / total 44244 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147948..151254 (delta 3306, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99692->103351, minecraft:drowned 3476->4475, minecraft:zombie 3678->4633, minecraft:husk 4461->5109, minecraft:creeper 4637->5259, minecraft:spider 4265->4844, minecraft:skeleton 4321->4787, minecraft:bee 7->29
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3306)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50033960 B)
- `wall-collapsed.txt` (3129223 B)
- `alloc-collapsed.txt` (2090940 B)
- `cpu-flamegraph.html` (269923 B)
- `server-stdout.log` (363680 B)
- `gc.log` (3812380 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
