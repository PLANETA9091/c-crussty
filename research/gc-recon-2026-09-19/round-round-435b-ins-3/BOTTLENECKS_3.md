# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.093 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.8, 1.9, 2.3, 2.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:16:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6941436 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148953, 149834, 151344]
- top entity types (max seen): minecraft:item×103333, minecraft:creeper×5216, minecraft:husk×5156, minecraft:skeleton×4860, minecraft:spider×4847, minecraft:zombie×4676, minecraft:drowned×4561, minecraft:sheep×3531, minecraft:chicken×3418, minecraft:cow×3363, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/aKnre6mFiq
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **10**)
- total pause: **24751.3 ms**, avg **209.76 ms**, max **2688.8 ms**
- heap high-water seen: **7821 MB** -> last-after: **5784 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 114429)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30333 | 26.5% |
| kernel: other | 26444 | 23.1% |
| other | 14288 | 12.5% |
| moonrise/paper patches | 9361 | 8.2% |
| chunk system (kernel) | 8397 | 7.3% |
| fastutil collections | 7004 | 6.1% |
| JDK collections | 6826 | 6.0% |
| JIT stubs (vtable/itable) | 3287 | 2.9% |
| network (kernel) | 3000 | 2.6% |
| JDK invokes/VarHandle | 2391 | 2.1% |
| JDK other | 2074 | 1.8% |
| JVM internals (GC oop barriers) | 547 | 0.5% |
| vdso (clock) | 185 | 0.2% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91714 | 80.1% |
| phase: unclassified | 13415 | 11.7% |
| phase: main tick (unclassified) | 3400 | 3.0% |
| phase: chunk tick | 1960 | 1.7% |
| phase: network sync (ServerEntity) | 1714 | 1.5% |
| phase: chunk system (off-main worker) | 1132 | 1.0% |
| phase: block entities (hoppers/furnaces) | 602 | 0.5% |
| phase: random tick | 376 | 0.3% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99075** (86.6%) · native/JVM-internal **15238** (13.3%) · other **116** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4087 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3415 | 3.0% |
| `vtable stub` | native/JVM-internal | 2704 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2523 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1845 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1844 | 1.6% |
| `net/minecraft/world/entity/InsideSnapOps.serveV3` | JVM-Java | 1618 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1558 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1524 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1513 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1513 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1383 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1378 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1331 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1287 | 1.1% |
| `net/minecraft/world/entity/InsideSnapOps.buildAirMask` | JVM-Java | 1219 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1199 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1138 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1104 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1013 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1007 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 961 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 912 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 907 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 851 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 848 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 841 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 819 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 746 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 638 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 620 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 618 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 603 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 62446)

| bucket | self-time samples | share |
|---|---|---|
| other | 59052 | 94.6% |
| entities/mobs (kernel) | 1031 | 1.7% |
| kernel: other | 914 | 1.5% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 284 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 218 | 0.3% |
| JIT stubs (vtable/itable) | 135 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 71 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58852 | 94.2% |
| phase: entity tick (AI/movement) | 3139 | 5.0% |
| phase: main tick (unclassified) | 224 | 0.4% |
| phase: chunk tick | 76 | 0.1% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **53529** (85.7%) · native/JVM-internal **8915** (14.3%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 50200 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.6% |
| `read` | native/JVM-internal | 1262 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 109 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `syscall` | native/JVM-internal | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 43 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 22976)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 22976 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 20613 | 89.7% |
| phase: entity tick (AI/movement) | 1997 | 8.7% |
| phase: chunk system (off-main worker) | 254 | 1.1% |
| phase: main tick (unclassified) | 84 | 0.4% |
| phase: network sync (ServerEntity) | 17 | 0.1% |
| phase: block entities (hoppers/furnaces) | 5 | 0.0% |
| phase: chunk tick | 3 | 0.0% |
| phase: random tick | 2 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **22976** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3905 | 17.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 1991 | 8.7% |
| `byte[]_[k]` | other | 1503 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 1300 | 5.7% |
| `byte[]_[i]` | other | 1297 | 5.6% |
| `java.lang.Object[]_[i]` | other | 1282 | 5.6% |
| `long[]_[k]` | other | 1141 | 5.0% |
| `short[]_[i]` | other | 1027 | 4.5% |
| `java.lang.String_[i]` | other | 825 | 3.6% |
| `java.lang.Object[]_[k]` | other | 677 | 2.9% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 642 | 2.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 549 | 2.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 496 | 2.2% |
| `char[]_[k]` | other | 452 | 2.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 443 | 1.9% |
| `java.util.Optional_[i]` | other | 365 | 1.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 266 | 1.2% |
| `long[]_[i]` | other | 203 | 0.9% |
| `java.util.ArrayList_[i]` | other | 192 | 0.8% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 189 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 114429 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35212 | 30.77% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21689 | 18.95% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6308 | 5.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5417 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4503 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1027 | 0.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 835 | 0.73% |
| `net/minecraft/world/entity/npc/Villager.tick` | 406 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 319 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 298 | 0.26% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 239 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 238 | 0.21% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3905 | 17.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | 1991 | 8.7% |
| `byte[]_[k]` | 1503 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 1300 | 5.7% |
| `byte[]_[i]` | 1297 | 5.6% |
| `java.lang.Object[]_[i]` | 1282 | 5.6% |
| `long[]_[k]` | 1141 | 5.0% |
| `short[]_[i]` | 1027 | 4.5% |
| `java.lang.String_[i]` | 825 | 3.6% |
| `java.lang.Object[]_[k]` | 677 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 24751 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148327..151344 (delta 3017, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99561->103333, minecraft:drowned 3479->4561, minecraft:zombie 3649->4676, minecraft:creeper 4574->5216, minecraft:husk 4560->5156, minecraft:spider 4270->4847, minecraft:skeleton 4361->4860, minecraft:pig 3201->3250
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3017)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55606326 B)
- `wall-collapsed.txt` (3714569 B)
- `alloc-collapsed.txt` (5513048 B)
- `cpu-flamegraph.html` (302932 B)
- `server-stdout.log` (385467 B)
- `gc.log` (112775 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
