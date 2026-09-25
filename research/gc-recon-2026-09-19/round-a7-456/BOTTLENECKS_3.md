# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.711 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.0, 1.9, 2.1, 2.5, 2.8, 2.8]
- spark tick-monitor MSPT: avg **364.85ms** / min 318.32ms / max **463.76ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:25:07Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7264766 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 318.32 | — | — | — | 463.76 | 364.85 |

- entity totals seen: [149457, 151301, 151447]
- top entity types (max seen): minecraft:item×103524, minecraft:creeper×5206, minecraft:husk×5203, minecraft:skeleton×4860, minecraft:spider×4810, minecraft:zombie×4652, minecraft:drowned×4563, minecraft:sheep×3494, minecraft:chicken×3437, minecraft:cow×3363, minecraft:pig×3224, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/UNsTSGp9vV
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **135** (Full GC: **10**)
- total pause: **28455.9 ms**, avg **210.78 ms**, max **2861.9 ms**
- heap high-water seen: **7555 MB** -> last-after: **4697 MB**
  - Young (Allocation Failure): 115
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117211)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29613 | 25.3% |
| entities/mobs (kernel) | 27654 | 23.6% |
| other | 15121 | 12.9% |
| moonrise/paper patches | 10793 | 9.2% |
| chunk system (kernel) | 10683 | 9.1% |
| fastutil collections | 6504 | 5.5% |
| JDK collections | 5738 | 4.9% |
| JIT stubs (vtable/itable) | 3467 | 3.0% |
| network (kernel) | 2603 | 2.2% |
| JDK invokes/VarHandle | 2273 | 1.9% |
| JDK other | 1711 | 1.5% |
| JVM internals (GC oop barriers) | 549 | 0.5% |
| vdso (clock) | 212 | 0.2% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| bukkit api | 67 | 0.1% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93289 | 79.6% |
| phase: unclassified | 13805 | 11.8% |
| phase: main tick (unclassified) | 3412 | 2.9% |
| phase: chunk tick | 2240 | 1.9% |
| phase: network sync (ServerEntity) | 1899 | 1.6% |
| phase: chunk system (off-main worker) | 1015 | 0.9% |
| phase: block entities (hoppers/furnaces) | 699 | 0.6% |
| phase: random tick | 413 | 0.4% |
| phase: scheduler/mid-tick tasks | 298 | 0.3% |
| phase: mob spawning | 141 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101376** (86.5%) · native/JVM-internal **15682** (13.4%) · other **153** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4773 | 4.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3185 | 2.7% |
| `vtable stub` | native/JVM-internal | 2845 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2599 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2135 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2066 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1825 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1822 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1737 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1672 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1456 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1445 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1372 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1346 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1342 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1277 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1249 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1236 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1152 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1121 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1075 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1055 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 909 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 905 | 0.8% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 896 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 868 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 843 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 802 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 742 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 740 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 703 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 702 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 662 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 631 | 0.5% |
| `itable stub` | native/JVM-internal | 617 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57862 | 94.5% |
| entities/mobs (kernel) | 1007 | 1.6% |
| kernel: other | 922 | 1.5% |
| moonrise/paper patches | 360 | 0.6% |
| chunk system (kernel) | 303 | 0.5% |
| fastutil collections | 227 | 0.4% |
| JDK collections | 184 | 0.3% |
| JIT stubs (vtable/itable) | 134 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 62 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57669 | 94.1% |
| phase: entity tick (AI/movement) | 3164 | 5.2% |
| phase: main tick (unclassified) | 170 | 0.3% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: chunk system (off-main worker) | 29 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: scheduler/mid-tick tasks | 10 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52336** (85.4%) · native/JVM-internal **8909** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48989 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4783 | 7.8% |
| `read` | native/JVM-internal | 1212 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 142 | 0.2% |
| `vtable stub` | native/JVM-internal | 112 | 0.2% |
| `syscall` | native/JVM-internal | 103 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4245)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4245 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2419 | 57.0% |
| phase: unclassified | 1614 | 38.0% |
| phase: main tick (unclassified) | 106 | 2.5% |
| phase: chunk system (off-main worker) | 58 | 1.4% |
| phase: network sync (ServerEntity) | 21 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4245** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 604 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 589 | 13.9% |
| `char[]_[k]` | other | 437 | 10.3% |
| `byte[]_[k]` | other | 285 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 204 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 166 | 3.9% |
| `long[]_[i]` | other | 147 | 3.5% |
| `java.util.ArrayList_[i]` | other | 137 | 3.2% |
| `java.lang.Object[]_[i]` | other | 116 | 2.7% |
| `byte[]_[i]` | other | 98 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.1% |
| `int[]_[i]` | other | 56 | 1.3% |
| `java.util.ImmutableCollections$List12_[i]` | other | 48 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 44 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 42 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 41 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 40 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f69839ed010_[i]` | other | 38 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117211 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35295 | 30.11% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22752 | 19.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6311 | 5.38% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5406 | 4.61% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4358 | 3.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1154 | 0.98% |
| `net/minecraft/world/entity/ai/Brain.tick` | 955 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 433 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 244 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 189 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 604 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 589 | 13.9% |
| `char[]_[k]` | 437 | 10.3% |
| `byte[]_[k]` | 285 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | 204 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 166 | 3.9% |
| `long[]_[i]` | 147 | 3.5% |
| `java.util.ArrayList_[i]` | 137 | 3.2% |
| `java.lang.Object[]_[i]` | 116 | 2.7% |
| `byte[]_[i]` | 98 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 135 pauses / total 28456 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148264..151447 (delta 3183, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99821->103524, minecraft:drowned 3587->4563, minecraft:zombie 3699->4652, minecraft:husk 4505->5203, minecraft:creeper 4562->5206, minecraft:spider 4233->4810, minecraft:skeleton 4431->4860, minecraft:chicken 3410->3437
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3183)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58044970 B)
- `wall-collapsed.txt` (3792079 B)
- `alloc-collapsed.txt` (2301769 B)
- `cpu-flamegraph.html` (302680 B)
- `server-stdout.log` (254187 B)
- `gc.log` (127432 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
