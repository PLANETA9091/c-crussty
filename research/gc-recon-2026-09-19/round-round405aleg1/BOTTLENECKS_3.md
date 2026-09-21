# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.83 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.2, 1.6, 1.9, 2.2, 2.4, 2.5]
- spark tick-monitor MSPT: avg **411.93ms** / min 335.48ms / max **548.18ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T20:15:06Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6507187 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 335.48 | — | — | — | 548.18 | 411.93 |

- entity totals seen: [148991, 150149, 151400]
- top entity types (max seen): minecraft:item×103347, minecraft:husk×5184, minecraft:creeper×5169, minecraft:skeleton×4890, minecraft:spider×4871, minecraft:zombie×4628, minecraft:drowned×4550, minecraft:sheep×3524, minecraft:chicken×3425, minecraft:cow×3364, minecraft:pig×3219, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/GZfffu4RnH
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **21849.5 ms**, avg **186.75 ms**, max **2493.0 ms**
- heap high-water seen: **7473 MB** -> last-after: **3858 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 112436)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27514 | 24.5% |
| kernel: other | 26041 | 23.2% |
| other | 11516 | 10.2% |
| chunk system (kernel) | 10551 | 9.4% |
| moonrise/paper patches | 10548 | 9.4% |
| JDK collections | 7558 | 6.7% |
| fastutil collections | 6874 | 6.1% |
| network (kernel) | 3621 | 3.2% |
| JIT stubs (vtable/itable) | 2922 | 2.6% |
| JDK invokes/VarHandle | 2829 | 2.5% |
| JDK other | 1914 | 1.7% |
| vdso (clock) | 254 | 0.2% |
| block entities/hoppers (kernel) | 88 | 0.1% |
| bukkit api | 70 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 10 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91014 | 80.9% |
| phase: unclassified | 9179 | 8.2% |
| phase: main tick (unclassified) | 3692 | 3.3% |
| phase: chunk system (off-main worker) | 2831 | 2.5% |
| phase: chunk tick | 2291 | 2.0% |
| phase: network sync (ServerEntity) | 2010 | 1.8% |
| phase: block entities (hoppers/furnaces) | 834 | 0.7% |
| phase: random tick | 468 | 0.4% |
| phase: mob spawning | 117 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100732** (89.6%) · native/JVM-internal **11612** (10.3%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5173 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3948 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2629 | 2.3% |
| `vtable stub` | native/JVM-internal | 2318 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2297 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2241 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1993 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1858 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1661 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1654 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1551 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1488 | 1.3% |
| `java/util/concurrent/atomic/Striped64$Cell.cas` | JVM-Java | 1478 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1406 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1228 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1156 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1152 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1126 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1029 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1027 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1011 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1000 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 964 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 947 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 914 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 902 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 873 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 846 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 724 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 721 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 712 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 697 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 689 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 680 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 58042 | 94.8% |
| entities/mobs (kernel) | 872 | 1.4% |
| kernel: other | 772 | 1.3% |
| chunk system (kernel) | 343 | 0.6% |
| moonrise/paper patches | 333 | 0.5% |
| JDK collections | 239 | 0.4% |
| fastutil collections | 187 | 0.3% |
| network (kernel) | 129 | 0.2% |
| JIT stubs (vtable/itable) | 117 | 0.2% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 62 | 0.1% |
| JVM internals (GC oop barriers) | 58 | 0.1% |
| block entities/hoppers (kernel) | 9 | 0.0% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57657 | 94.1% |
| phase: entity tick (AI/movement) | 3123 | 5.1% |
| phase: main tick (unclassified) | 154 | 0.3% |
| phase: chunk system (off-main worker) | 111 | 0.2% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 45 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 5 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51788** (84.5%) · native/JVM-internal **9462** (15.4%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48663 | 79.4% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 480 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 165 | 0.3% |
| `vtable stub` | native/JVM-internal | 101 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 98 | 0.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 66 | 0.1% |
| `java/util/concurrent/atomic/Striped64$Cell.cas` | JVM-Java | 63 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 43 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 23672)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 23672 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21248 | 89.8% |
| phase: entity tick (AI/movement) | 2058 | 8.7% |
| phase: chunk system (off-main worker) | 178 | 0.8% |
| phase: main tick (unclassified) | 132 | 0.6% |
| phase: network sync (ServerEntity) | 23 | 0.1% |
| phase: block entities (hoppers/furnaces) | 17 | 0.1% |
| phase: chunk tick | 6 | 0.0% |
| phase: random tick | 6 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **23672** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3129 | 13.2% |
| `byte[]_[i]` | other | 2172 | 9.2% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2070 | 8.7% |
| `java.lang.String_[i]` | other | 1534 | 6.5% |
| `java.lang.Object[]_[i]` | other | 1462 | 6.2% |
| `byte[]_[k]` | other | 1241 | 5.2% |
| `short[]_[i]` | other | 982 | 4.1% |
| `long[]_[k]` | other | 835 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 826 | 3.5% |
| `java.lang.Object[]_[k]` | other | 776 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 703 | 3.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 561 | 2.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 532 | 2.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 532 | 2.2% |
| `java.util.Optional_[i]` | other | 498 | 2.1% |
| `char[]_[k]` | other | 454 | 1.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 295 | 1.2% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 227 | 1.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 209 | 0.9% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 208 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112436 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34030 | 30.27% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22164 | 19.71% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6552 | 5.83% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5286 | 4.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4473 | 3.98% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 932 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 930 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 227 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 208 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3129 | 13.2% |
| `byte[]_[i]` | 2172 | 9.2% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2070 | 8.7% |
| `java.lang.String_[i]` | 1534 | 6.5% |
| `java.lang.Object[]_[i]` | 1462 | 6.2% |
| `byte[]_[k]` | 1241 | 5.2% |
| `short[]_[i]` | 982 | 4.1% |
| `long[]_[k]` | 835 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 826 | 3.5% |
| `java.lang.Object[]_[k]` | 776 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 21849 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148113..151400 (delta 3287, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99679->103347, minecraft:drowned 3492->4550, minecraft:zombie 3574->4628, minecraft:husk 4536->5184, minecraft:spider 4230->4871, minecraft:creeper 4531->5169, minecraft:skeleton 4395->4890, minecraft:chicken 3400->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3287)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52595513 B)
- `wall-collapsed.txt` (3465264 B)
- `alloc-collapsed.txt` (5197024 B)
- `cpu-flamegraph.html` (284287 B)
- `server-stdout.log` (251752 B)
- `gc.log` (110964 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
