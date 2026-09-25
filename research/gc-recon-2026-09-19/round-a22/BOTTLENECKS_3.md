# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.978 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.6, 1.6, 1.9, 2.1, 2.5, 2.5]
- spark tick-monitor MSPT: avg **415.7ms** / min 348.15ms / max **517.61ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:45:56Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7047888 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.15 | — | — | — | 517.61 | 415.7 |

- entity totals seen: [149148, 150233, 151467]
- top entity types (max seen): minecraft:item×103343, minecraft:creeper×5192, minecraft:husk×5163, minecraft:spider×4865, minecraft:skeleton×4848, minecraft:zombie×4647, minecraft:drowned×4551, minecraft:sheep×3522, minecraft:chicken×3424, minecraft:cow×3363, minecraft:pig×3230, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/IMOcUS47Ws
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **21036.8 ms**, avg **178.28 ms**, max **2471.8 ms**
- heap high-water seen: **7575 MB** -> last-after: **4289 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116773)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27875 | 23.9% |
| kernel: other | 27827 | 23.8% |
| other | 15942 | 13.7% |
| moonrise/paper patches | 10006 | 8.6% |
| chunk system (kernel) | 9682 | 8.3% |
| fastutil collections | 6934 | 5.9% |
| JDK collections | 5852 | 5.0% |
| JIT stubs (vtable/itable) | 3521 | 3.0% |
| network (kernel) | 3213 | 2.8% |
| JDK invokes/VarHandle | 2519 | 2.2% |
| JDK other | 2345 | 2.0% |
| JVM internals (GC oop barriers) | 542 | 0.5% |
| vdso (clock) | 227 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| bukkit api | 55 | 0.0% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92300 | 79.0% |
| phase: unclassified | 14598 | 12.5% |
| phase: main tick (unclassified) | 3625 | 3.1% |
| phase: chunk tick | 2172 | 1.9% |
| phase: network sync (ServerEntity) | 1738 | 1.5% |
| phase: chunk system (off-main worker) | 1177 | 1.0% |
| phase: block entities (hoppers/furnaces) | 651 | 0.6% |
| phase: random tick | 392 | 0.3% |
| phase: mob spawning | 119 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100073** (85.7%) · native/JVM-internal **16611** (14.2%) · other **89** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4474 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3277 | 2.8% |
| `vtable stub` | native/JVM-internal | 2881 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2577 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2026 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1926 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1687 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1583 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1556 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1542 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1531 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1470 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1438 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1416 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1382 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1270 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1242 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1038 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1020 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 957 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 887 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 868 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 866 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 850 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 814 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 802 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 786 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 771 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 723 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 719 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 676 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 650 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 646 | 0.6% |
| `itable stub` | native/JVM-internal | 637 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57819 | 94.4% |
| entities/mobs (kernel) | 992 | 1.6% |
| kernel: other | 957 | 1.6% |
| moonrise/paper patches | 350 | 0.6% |
| chunk system (kernel) | 320 | 0.5% |
| fastutil collections | 230 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 123 | 0.2% |
| network (kernel) | 109 | 0.2% |
| JDK invokes/VarHandle | 77 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57632 | 94.1% |
| phase: entity tick (AI/movement) | 3176 | 5.2% |
| phase: main tick (unclassified) | 200 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52387** (85.5%) · native/JVM-internal **8867** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48969 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1259 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 155 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 23457)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 23457 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21152 | 90.2% |
| phase: entity tick (AI/movement) | 1996 | 8.5% |
| phase: chunk system (off-main worker) | 172 | 0.7% |
| phase: main tick (unclassified) | 83 | 0.4% |
| phase: network sync (ServerEntity) | 28 | 0.1% |
| phase: chunk tick | 13 | 0.1% |
| phase: block entities (hoppers/furnaces) | 6 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **23457** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3185 | 13.6% |
| `byte[]_[i]` | other | 2177 | 9.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2031 | 8.7% |
| `java.lang.String_[i]` | other | 1512 | 6.4% |
| `java.lang.Object[]_[i]` | other | 1404 | 6.0% |
| `byte[]_[k]` | other | 1228 | 5.2% |
| `short[]_[i]` | other | 949 | 4.0% |
| `long[]_[k]` | other | 847 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 846 | 3.6% |
| `java.lang.Object[]_[k]` | other | 775 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 719 | 3.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 605 | 2.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 534 | 2.3% |
| `java.util.Optional_[i]` | other | 526 | 2.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 495 | 2.1% |
| `char[]_[k]` | other | 439 | 1.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 299 | 1.3% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 223 | 1.0% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 221 | 0.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 195 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116773 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34246 | 29.33% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22733 | 19.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6301 | 5.40% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5255 | 4.50% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4577 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1116 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 942 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 495 | 0.42% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 233 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 218 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 174 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 144 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3185 | 13.6% |
| `byte[]_[i]` | 2177 | 9.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2031 | 8.7% |
| `java.lang.String_[i]` | 1512 | 6.4% |
| `java.lang.Object[]_[i]` | 1404 | 6.0% |
| `byte[]_[k]` | 1228 | 5.2% |
| `short[]_[i]` | 949 | 4.0% |
| `long[]_[k]` | 847 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | 846 | 3.6% |
| `java.lang.Object[]_[k]` | 775 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 21037 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148221..151467 (delta 3246, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99761->103343, minecraft:drowned 3475->4551, minecraft:zombie 3687->4647, minecraft:creeper 4547->5192, minecraft:husk 4538->5163, minecraft:spider 4251->4865, minecraft:skeleton 4412->4848, minecraft:chicken 3398->3424
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3246)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57326097 B)
- `wall-collapsed.txt` (3819773 B)
- `alloc-collapsed.txt` (5104826 B)
- `cpu-flamegraph.html` (305821 B)
- `server-stdout.log` (257206 B)
- `gc.log` (111865 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
