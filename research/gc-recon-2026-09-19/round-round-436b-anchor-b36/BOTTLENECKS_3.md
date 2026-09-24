# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.083 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.7, 2.0, 2.2, 2.5, 2.5]
- spark tick-monitor MSPT: avg **415.36ms** / min 354.77ms / max **622.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:14:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6566994 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 354.77 | — | — | — | 622.98 | 415.36 |

- entity totals seen: [149101, 150212, 151495]
- top entity types (max seen): minecraft:item×103357, minecraft:creeper×5202, minecraft:husk×5177, minecraft:spider×4896, minecraft:skeleton×4869, minecraft:zombie×4671, minecraft:drowned×4532, minecraft:sheep×3515, minecraft:chicken×3438, minecraft:cow×3374, minecraft:pig×3249, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/tmcfDhYQ1Y
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **23004.5 ms**, avg **193.32 ms**, max **2650.0 ms**
- heap high-water seen: **7482 MB** -> last-after: **4216 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115273)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27560 | 23.9% |
| kernel: other | 26345 | 22.9% |
| other | 15259 | 13.2% |
| chunk system (kernel) | 10310 | 8.9% |
| moonrise/paper patches | 10302 | 8.9% |
| fastutil collections | 7404 | 6.4% |
| JDK collections | 6086 | 5.3% |
| network (kernel) | 3616 | 3.1% |
| JIT stubs (vtable/itable) | 2632 | 2.3% |
| JDK invokes/VarHandle | 2596 | 2.3% |
| JDK other | 2007 | 1.7% |
| JVM internals (GC oop barriers) | 602 | 0.5% |
| vdso (clock) | 240 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| bukkit api | 57 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90069 | 78.1% |
| phase: unclassified | 14576 | 12.6% |
| phase: main tick (unclassified) | 3717 | 3.2% |
| phase: chunk tick | 2223 | 1.9% |
| phase: network sync (ServerEntity) | 2154 | 1.9% |
| phase: chunk system (off-main worker) | 1174 | 1.0% |
| phase: block entities (hoppers/furnaces) | 751 | 0.7% |
| phase: random tick | 456 | 0.4% |
| phase: mob spawning | 149 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99664** (86.5%) · native/JVM-internal **15515** (13.5%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5104 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4022 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2419 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2272 | 2.0% |
| `vtable stub` | native/JVM-internal | 2072 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2064 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1991 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1895 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1766 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1640 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1638 | 1.4% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f32059df8e0.accept` | JVM-Java | 1521 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1491 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1329 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1217 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1176 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1146 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1103 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1102 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1087 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1078 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1070 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1064 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1012 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 971 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 929 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 893 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 868 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 814 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 767 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 748 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 694 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 668 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 648 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61365)

| bucket | self-time samples | share |
|---|---|---|
| other | 58093 | 94.7% |
| entities/mobs (kernel) | 896 | 1.5% |
| kernel: other | 892 | 1.5% |
| chunk system (kernel) | 343 | 0.6% |
| moonrise/paper patches | 319 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 194 | 0.3% |
| network (kernel) | 129 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK other | 76 | 0.1% |
| JDK invokes/VarHandle | 72 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57908 | 94.4% |
| phase: entity tick (AI/movement) | 2999 | 4.9% |
| phase: main tick (unclassified) | 182 | 0.3% |
| phase: chunk tick | 111 | 0.2% |
| phase: network sync (ServerEntity) | 54 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52411** (85.4%) · native/JVM-internal **8950** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49153 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1204 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 163 | 0.3% |
| `syscall` | native/JVM-internal | 115 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 87 | 0.1% |
| `vtable stub` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 73 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 63 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10468)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10468 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8204 | 78.4% |
| phase: entity tick (AI/movement) | 2010 | 19.2% |
| phase: chunk system (off-main worker) | 115 | 1.1% |
| phase: main tick (unclassified) | 78 | 0.7% |
| phase: network sync (ServerEntity) | 24 | 0.2% |
| phase: block entities (hoppers/furnaces) | 18 | 0.2% |
| phase: chunk tick | 11 | 0.1% |
| phase: mob spawning | 5 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10468** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1997 | 19.1% |
| `byte[]_[k]` | other | 727 | 6.9% |
| `short[]_[i]` | other | 555 | 5.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 532 | 5.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 518 | 4.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 507 | 4.8% |
| `long[]_[k]` | other | 504 | 4.8% |
| `java.lang.Object[]_[i]` | other | 485 | 4.6% |
| `char[]_[k]` | other | 443 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 436 | 4.2% |
| `byte[]_[i]` | other | 388 | 3.7% |
| `java.lang.String_[i]` | other | 208 | 2.0% |
| `java.lang.Object[]_[k]` | other | 169 | 1.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 166 | 1.6% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 148 | 1.4% |
| `long[]_[i]` | other | 144 | 1.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 125 | 1.2% |
| `java.util.Optional_[i]` | other | 124 | 1.2% |
| `java.util.ArrayList_[i]` | other | 124 | 1.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 86 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115273 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34166 | 29.64% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21649 | 18.78% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6312 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5246 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4353 | 3.78% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 939 | 0.81% |
| `net/minecraft/world/entity/ai/Brain.tick` | 925 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 418 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 212 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 201 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1997 | 19.1% |
| `byte[]_[k]` | 727 | 6.9% |
| `short[]_[i]` | 555 | 5.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 532 | 5.1% |
| `net.minecraft.world.phys.AABB_[i]` | 518 | 4.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 507 | 4.8% |
| `long[]_[k]` | 504 | 4.8% |
| `java.lang.Object[]_[i]` | 485 | 4.6% |
| `char[]_[k]` | 443 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 436 | 4.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 23005 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148186..151495 (delta 3309, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99684->103357, minecraft:drowned 3436->4532, minecraft:zombie 3662->4671, minecraft:creeper 4527->5202, minecraft:husk 4518->5177, minecraft:spider 4238->4896, minecraft:skeleton 4363->4869, minecraft:chicken 3419->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3309)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49929234 B)
- `wall-collapsed.txt` (3554468 B)
- `alloc-collapsed.txt` (3833739 B)
- `cpu-flamegraph.html` (292087 B)
- `server-stdout.log` (252936 B)
- `gc.log` (112682 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
