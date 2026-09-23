# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.969 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.8, 1.7, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **411.0ms** / min 340.21ms / max **600.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:56:42Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6775381 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 340.21 | — | — | — | 600.68 | 411.0 |

- entity totals seen: [149084, 150397, 151448]
- top entity types (max seen): minecraft:item×103388, minecraft:husk×5191, minecraft:creeper×5179, minecraft:skeleton×4830, minecraft:spider×4821, minecraft:zombie×4682, minecraft:drowned×4567, minecraft:sheep×3520, minecraft:chicken×3402, minecraft:cow×3360, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/zm0aAu5Ojo
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **20425.5 ms**, avg **166.06 ms**, max **2356.9 ms**
- heap high-water seen: **7406 MB** -> last-after: **4133 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117210)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28463 | 24.3% |
| kernel: other | 28284 | 24.1% |
| other | 15203 | 13.0% |
| chunk system (kernel) | 10023 | 8.6% |
| moonrise/paper patches | 9644 | 8.2% |
| fastutil collections | 7132 | 6.1% |
| JDK collections | 6195 | 5.3% |
| JIT stubs (vtable/itable) | 3621 | 3.1% |
| network (kernel) | 3255 | 2.8% |
| JDK invokes/VarHandle | 2411 | 2.1% |
| JDK other | 1962 | 1.7% |
| JVM internals (GC oop barriers) | 509 | 0.4% |
| vdso (clock) | 220 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 51 | 0.0% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 41 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93587 | 79.8% |
| phase: unclassified | 14015 | 12.0% |
| phase: main tick (unclassified) | 3530 | 3.0% |
| phase: chunk tick | 2096 | 1.8% |
| phase: network sync (ServerEntity) | 1813 | 1.5% |
| phase: chunk system (off-main worker) | 1007 | 0.9% |
| phase: block entities (hoppers/furnaces) | 641 | 0.5% |
| phase: random tick | 386 | 0.3% |
| phase: mob spawning | 132 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101157** (86.3%) · native/JVM-internal **15947** (13.6%) · other **106** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4574 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3188 | 2.7% |
| `vtable stub` | native/JVM-internal | 2996 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2960 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1964 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1864 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1684 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1598 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1594 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1519 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1487 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1450 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1446 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1423 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1307 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1193 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1150 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1093 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1049 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 996 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 980 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 925 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 876 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 850 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 828 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 813 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 795 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 756 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 729 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 651 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 635 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 631 | 0.5% |
| `itable stub` | native/JVM-internal | 624 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61242)

| bucket | self-time samples | share |
|---|---|---|
| other | 57848 | 94.5% |
| entities/mobs (kernel) | 998 | 1.6% |
| kernel: other | 895 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 311 | 0.5% |
| fastutil collections | 236 | 0.4% |
| JDK collections | 196 | 0.3% |
| JIT stubs (vtable/itable) | 152 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK other | 84 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57631 | 94.1% |
| phase: entity tick (AI/movement) | 3175 | 5.2% |
| phase: main tick (unclassified) | 202 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.1% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52255** (85.3%) · native/JVM-internal **8985** (14.7%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48905 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `vtable stub` | native/JVM-internal | 132 | 0.2% |
| `syscall` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 97 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6524)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6524 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 4118 | 63.1% |
| phase: entity tick (AI/movement) | 2218 | 34.0% |
| phase: main tick (unclassified) | 91 | 1.4% |
| phase: chunk system (off-main worker) | 51 | 0.8% |
| phase: network sync (ServerEntity) | 21 | 0.3% |
| phase: block entities (hoppers/furnaces) | 12 | 0.2% |
| phase: chunk tick | 10 | 0.2% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6524** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 624 | 9.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 574 | 8.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 523 | 8.0% |
| `char[]_[k]` | other | 456 | 7.0% |
| `byte[]_[k]` | other | 403 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 275 | 4.2% |
| `java.lang.Object[]_[i]` | other | 265 | 4.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 252 | 3.9% |
| `byte[]_[i]` | other | 244 | 3.7% |
| `short[]_[i]` | other | 177 | 2.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 172 | 2.6% |
| `long[]_[k]` | other | 154 | 2.4% |
| `java.lang.String_[i]` | other | 144 | 2.2% |
| `long[]_[i]` | other | 127 | 1.9% |
| `java.util.ArrayList_[i]` | other | 113 | 1.7% |
| `java.lang.Object[]_[k]` | other | 86 | 1.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 85 | 1.3% |
| `int[]_[i]` | other | 70 | 1.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 69 | 1.1% |
| `java.util.Optional_[i]` | other | 67 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117210 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34923 | 29.80% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22564 | 19.25% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6418 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5481 | 4.68% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4584 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1190 | 1.02% |
| `net/minecraft/world/entity/ai/Brain.tick` | 910 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 452 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 250 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 227 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 222 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 208 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 624 | 9.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 574 | 8.8% |
| `net.minecraft.world.phys.AABB_[i]` | 523 | 8.0% |
| `char[]_[k]` | 456 | 7.0% |
| `byte[]_[k]` | 403 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 275 | 4.2% |
| `java.lang.Object[]_[i]` | 265 | 4.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | 252 | 3.9% |
| `byte[]_[i]` | 244 | 3.7% |
| `short[]_[i]` | 177 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 20426 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148246..151448 (delta 3202, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99777->103388, minecraft:drowned 3459->4567, minecraft:zombie 3675->4682, minecraft:husk 4513->5191, minecraft:creeper 4528->5179, minecraft:spider 4249->4821, minecraft:skeleton 4460->4830, minecraft:chicken 3371->3402
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3202)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56489399 B)
- `wall-collapsed.txt` (3854326 B)
- `alloc-collapsed.txt` (3667192 B)
- `cpu-flamegraph.html` (300655 B)
- `server-stdout.log` (250370 B)
- `gc.log` (116205 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
