# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.251 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.0, 2.3, 2.5, 3.2, 3.2, 3.3]
- spark tick-monitor MSPT: avg **386.81ms** / min 264.88ms / max **486.1ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T18:07:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8960321 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:09:40 INFO]: [crussty-plugin] [cruss | 324.64 | — | — | — | 486.1 | 386.81 |

- entity totals seen: [151090, 155428, 157207]
- top entity types (max seen): minecraft:item×111151, minecraft:husk×5474, minecraft:creeper×5126, minecraft:skeleton×4854, minecraft:zombie×4628, minecraft:drowned×4522, minecraft:spider×4246, minecraft:sheep×3538, minecraft:chicken×3428, minecraft:cow×3317, minecraft:pig×3187, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/qdyvkJZ0zL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1210** (Full GC: **9**)
- total pause: **24382.4 ms**, avg **20.15 ms**, max **1788.3 ms**
- heap high-water seen: **8540 MB** -> last-after: **4257 MB**
  - Young (Allocation Failure): 1187
  - Young (GCLocker Initiated GC): 5
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105902)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30284 | 28.6% |
| kernel: other | 22422 | 21.2% |
| other | 11003 | 10.4% |
| chunk system (kernel) | 8608 | 8.1% |
| moonrise/paper patches | 7944 | 7.5% |
| JDK collections | 7115 | 6.7% |
| fastutil collections | 6657 | 6.3% |
| network (kernel) | 3512 | 3.3% |
| JIT stubs (vtable/itable) | 2693 | 2.5% |
| JDK invokes/VarHandle | 2548 | 2.4% |
| JDK other | 2129 | 2.0% |
| JVM internals (GC oop barriers) | 484 | 0.5% |
| vdso (clock) | 135 | 0.1% |
| block entities/hoppers (kernel) | 114 | 0.1% |
| redstone (kernel) | 87 | 0.1% |
| bukkit api | 70 | 0.1% |
| craftbukkit glue | 67 | 0.1% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53600 | 50.6% |
| phase: unclassified | 32366 | 30.6% |
| phase: main tick (unclassified) | 11510 | 10.9% |
| phase: network sync (ServerEntity) | 2767 | 2.6% |
| phase: chunk tick | 2630 | 2.5% |
| phase: chunk system (off-main worker) | 1290 | 1.2% |
| phase: block entities (hoppers/furnaces) | 984 | 0.9% |
| phase: random tick | 567 | 0.5% |
| phase: mob spawning | 185 | 0.2% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94183** (88.9%) · native/JVM-internal **11568** (10.9%) · other **151** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4084 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3083 | 2.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2399 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2360 | 2.2% |
| `vtable stub` | native/JVM-internal | 1980 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1969 | 1.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1639 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1499 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1462 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1440 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1380 | 1.3% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1377 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1358 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1299 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1174 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1153 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1138 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1120 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1046 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1041 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1032 | 1.0% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 957 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 937 | 0.9% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 908 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 864 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 849 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 848 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 835 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 827 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 816 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 814 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 755 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 752 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 729 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 723 | 0.7% |
| `java/lang/ThreadLocal.get` | JVM-Java | 720 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 708 | 0.7% |
| `itable stub` | native/JVM-internal | 706 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 58233 | 95.1% |
| entities/mobs (kernel) | 975 | 1.6% |
| kernel: other | 701 | 1.1% |
| moonrise/paper patches | 269 | 0.4% |
| chunk system (kernel) | 247 | 0.4% |
| fastutil collections | 229 | 0.4% |
| JDK collections | 224 | 0.4% |
| network (kernel) | 124 | 0.2% |
| JIT stubs (vtable/itable) | 89 | 0.1% |
| JDK invokes/VarHandle | 75 | 0.1% |
| JDK other | 61 | 0.1% |
| redstone (kernel) | 10 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58627 | 95.7% |
| phase: entity tick (AI/movement) | 1900 | 3.1% |
| phase: main tick (unclassified) | 402 | 0.7% |
| phase: chunk tick | 113 | 0.2% |
| phase: network sync (ServerEntity) | 93 | 0.2% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52456** (85.6%) · native/JVM-internal **8783** (14.3%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49461 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1220 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 129 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `vtable stub` | native/JVM-internal | 67 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 59 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4967)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4967 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3187 | 64.2% |
| phase: entity tick (AI/movement) | 1292 | 26.0% |
| phase: main tick (unclassified) | 352 | 7.1% |
| phase: network sync (ServerEntity) | 43 | 0.9% |
| phase: chunk system (off-main worker) | 42 | 0.8% |
| phase: block entities (hoppers/furnaces) | 37 | 0.7% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4967** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 705 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 586 | 11.8% |
| `char[]_[k]` | other | 434 | 8.7% |
| `byte[]_[k]` | other | 310 | 6.2% |
| `int[]_[i]` | other | 249 | 5.0% |
| `byte[]_[i]` | other | 202 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 165 | 3.3% |
| `long[]_[i]` | other | 158 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 145 | 2.9% |
| `java.util.ArrayList_[i]` | other | 128 | 2.6% |
| `java.lang.Object[]_[i]` | other | 125 | 2.5% |
| `java.util.Calendar$Builder_[i]` | other | 92 | 1.9% |
| `java.util.GregorianCalendar_[i]` | other | 90 | 1.8% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 77 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 64 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 60 | 1.2% |
| `java.lang.String_[i]` | other | 52 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 50 | 1.0% |
| `java.util.ArrayList$Itr_[i]` | other | 49 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f0c4b832608_[i]` | other | 48 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105902 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19248 | 18.18% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6522 | 6.16% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5693 | 5.38% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4611 | 4.35% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1049 | 0.99% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1040 | 0.98% |
| `net/minecraft/world/entity/npc/Villager.tick` | 446 | 0.42% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 244 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 239 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 237 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 220 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 154 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 705 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 586 | 11.8% |
| `char[]_[k]` | 434 | 8.7% |
| `byte[]_[k]` | 310 | 6.2% |
| `int[]_[i]` | 249 | 5.0% |
| `byte[]_[i]` | 202 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 165 | 3.3% |
| `long[]_[i]` | 158 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 145 | 2.9% |
| `java.util.ArrayList_[i]` | 128 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1210 pauses / total 24382 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148189..157207 (delta 9018, churn 5.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99768->111151, minecraft:drowned 3338->4522, minecraft:zombie 3628->4628, minecraft:husk 4635->5474, minecraft:skeleton 4163->4854, minecraft:creeper 4584->5126, minecraft:pig 2648->3187, minecraft:sheep 3122->3538
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9018)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47086541 B)
- `wall-collapsed.txt` (2902872 B)
- `alloc-collapsed.txt` (1931067 B)
- `cpu-flamegraph.html` (271662 B)
- `server-stdout.log` (1000732 B)
- `gc.log` (1045770 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
