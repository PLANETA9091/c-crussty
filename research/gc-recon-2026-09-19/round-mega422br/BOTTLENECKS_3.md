# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.895 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.9, 2.2, 1.6, 2.8, 2.9]
- spark tick-monitor MSPT: avg **354.44ms** / min 297.31ms / max **504.07ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T02:20:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6520365 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:22:32 INFO]: [crussty-plugin] [cruss | 297.31 | — | — | — | 504.07 | 354.44 |

- entity totals seen: [150743, 153914, 156715]
- top entity types (max seen): minecraft:item×111027, minecraft:husk×5555, minecraft:creeper×4971, minecraft:skeleton×4848, minecraft:zombie×4639, minecraft:drowned×4558, minecraft:spider×4339, minecraft:sheep×3531, minecraft:chicken×3393, minecraft:cow×3365, minecraft:pig×3168, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/C3Q3L52gkc
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1130** (Full GC: **9**)
- total pause: **25036.8 ms**, avg **22.16 ms**, max **2225.9 ms**
- heap high-water seen: **7680 MB** -> last-after: **3840 MB**
  - Young (Allocation Failure): 1112
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 108266)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31269 | 28.9% |
| kernel: other | 26115 | 24.1% |
| other | 11254 | 10.4% |
| chunk system (kernel) | 7867 | 7.3% |
| moonrise/paper patches | 6924 | 6.4% |
| JDK collections | 6608 | 6.1% |
| fastutil collections | 6580 | 6.1% |
| JIT stubs (vtable/itable) | 3662 | 3.4% |
| network (kernel) | 2851 | 2.6% |
| JDK invokes/VarHandle | 2244 | 2.1% |
| JDK other | 1864 | 1.7% |
| JVM internals (GC oop barriers) | 562 | 0.5% |
| vdso (clock) | 112 | 0.1% |
| block entities/hoppers (kernel) | 104 | 0.1% |
| craftbukkit glue | 86 | 0.1% |
| bukkit api | 77 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55417 | 51.2% |
| phase: unclassified | 33758 | 31.2% |
| phase: main tick (unclassified) | 11688 | 10.8% |
| phase: chunk tick | 2374 | 2.2% |
| phase: network sync (ServerEntity) | 2238 | 2.1% |
| phase: chunk system (off-main worker) | 1094 | 1.0% |
| phase: block entities (hoppers/furnaces) | 999 | 0.9% |
| phase: random tick | 550 | 0.5% |
| phase: mob spawning | 144 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95679** (88.4%) · native/JVM-internal **12461** (11.5%) · other **126** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3573 | 3.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3076 | 2.8% |
| `vtable stub` | native/JVM-internal | 2897 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2535 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1716 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1492 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1470 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1391 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1324 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1298 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1288 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1250 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1145 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1130 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1080 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1080 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1076 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1065 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1025 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 976 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 950 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 883 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 772 | 0.7% |
| `itable stub` | native/JVM-internal | 762 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 704 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 696 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 687 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 681 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 670 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 664 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 661 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63655)

| bucket | self-time samples | share |
|---|---|---|
| other | 60493 | 95.0% |
| entities/mobs (kernel) | 1052 | 1.7% |
| kernel: other | 841 | 1.3% |
| chunk system (kernel) | 237 | 0.4% |
| moonrise/paper patches | 222 | 0.3% |
| fastutil collections | 214 | 0.3% |
| JDK collections | 202 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 67 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60930 | 95.7% |
| phase: entity tick (AI/movement) | 1996 | 3.1% |
| phase: main tick (unclassified) | 439 | 0.7% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: block entities (hoppers/furnaces) | 46 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54784** (86.1%) · native/JVM-internal **8861** (13.9%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51681 | 81.2% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.5% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 112 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 38 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 37 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 37 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4127)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4127 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2646 | 64.1% |
| phase: entity tick (AI/movement) | 1085 | 26.3% |
| phase: main tick (unclassified) | 291 | 7.1% |
| phase: chunk system (off-main worker) | 37 | 0.9% |
| phase: network sync (ServerEntity) | 37 | 0.9% |
| phase: block entities (hoppers/furnaces) | 14 | 0.3% |
| phase: chunk tick | 10 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4127** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 530 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 491 | 11.9% |
| `char[]_[k]` | other | 442 | 10.7% |
| `int[]_[i]` | other | 245 | 5.9% |
| `byte[]_[k]` | other | 208 | 5.0% |
| `byte[]_[i]` | other | 175 | 4.2% |
| `long[]_[i]` | other | 170 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 139 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 129 | 3.1% |
| `java.lang.Object[]_[i]` | other | 122 | 3.0% |
| `java.util.ArrayList_[i]` | other | 107 | 2.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 63 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 60 | 1.5% |
| `java.util.GregorianCalendar_[i]` | other | 55 | 1.3% |
| `java.util.Calendar$Builder_[i]` | other | 53 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.2% |
| `boolean[]_[i]` | other | 46 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 44 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd4ad9eb718_[i]` | other | 40 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108266 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19210 | 17.74% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6969 | 6.44% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5906 | 5.46% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4525 | 4.18% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1403 | 1.30% |
| `net/minecraft/world/entity/ai/Brain.tick` | 984 | 0.91% |
| `net/minecraft/world/entity/npc/Villager.tick` | 539 | 0.50% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 266 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 236 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 232 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 159 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 530 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | 491 | 11.9% |
| `char[]_[k]` | 442 | 10.7% |
| `int[]_[i]` | 245 | 5.9% |
| `byte[]_[k]` | 208 | 5.0% |
| `byte[]_[i]` | 175 | 4.2% |
| `long[]_[i]` | 170 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 139 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 129 | 3.1% |
| `java.lang.Object[]_[i]` | 122 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1130 pauses / total 25037 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148295..156715 (delta 8420, churn 5.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99820->111027, minecraft:drowned 3527->4558, minecraft:husk 4576->5555, minecraft:zombie 3797->4639, minecraft:skeleton 4236->4848, minecraft:pig 2596->3168, minecraft:spider 3808->4339, minecraft:sheep 3051->3531
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8420)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51919994 B)
- `wall-collapsed.txt` (3388986 B)
- `alloc-collapsed.txt` (1760807 B)
- `cpu-flamegraph.html` (301749 B)
- `server-stdout.log` (939619 B)
- `gc.log` (977346 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
