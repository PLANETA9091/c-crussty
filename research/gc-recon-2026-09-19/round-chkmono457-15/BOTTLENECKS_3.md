# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.587 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.9, 2.1, 2.2, 2.7, 2.6]
- spark tick-monitor MSPT: avg **384.0ms** / min 311.94ms / max **551.96ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:26:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6215978 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:28:40 INFO]: [crussty-plugin] [cruss | 311.94 | — | — | — | 551.96 | 384.0 |

- entity totals seen: [149783, 152370, 153676]
- top entity types (max seen): minecraft:item×106796, minecraft:husk×5427, minecraft:creeper×4979, minecraft:skeleton×4780, minecraft:zombie×4602, minecraft:drowned×4545, minecraft:spider×4476, minecraft:sheep×3526, minecraft:chicken×3402, minecraft:cow×3333, minecraft:pig×3176, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/5u4JKE3rW8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **18491.1 ms**, avg **165.10 ms**, max **2385.5 ms**
- heap high-water seen: **7543 MB** -> last-after: **4253 MB**
  - Young (Allocation Failure): 91
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 105255)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33908 | 32.2% |
| kernel: other | 21824 | 20.7% |
| other | 12731 | 12.1% |
| chunk system (kernel) | 8220 | 7.8% |
| JDK collections | 7112 | 6.8% |
| moonrise/paper patches | 5155 | 4.9% |
| fastutil collections | 4772 | 4.5% |
| JIT stubs (vtable/itable) | 3181 | 3.0% |
| network (kernel) | 2759 | 2.6% |
| JDK invokes/VarHandle | 2330 | 2.2% |
| JDK other | 2296 | 2.2% |
| JVM internals (GC oop barriers) | 550 | 0.5% |
| vdso (clock) | 109 | 0.1% |
| bukkit api | 97 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 62 | 0.1% |
| block entities/hoppers (kernel) | 53 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49790 | 47.3% |
| phase: unclassified | 34935 | 33.2% |
| phase: main tick (unclassified) | 13095 | 12.4% |
| phase: chunk tick | 2390 | 2.3% |
| phase: network sync (ServerEntity) | 2153 | 2.0% |
| phase: chunk system (off-main worker) | 1133 | 1.1% |
| phase: block entities (hoppers/furnaces) | 966 | 0.9% |
| phase: random tick | 492 | 0.5% |
| phase: mob spawning | 297 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91729** (87.1%) · native/JVM-internal **13415** (12.7%) · other **111** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3873 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3088 | 2.9% |
| `vtable stub` | native/JVM-internal | 2690 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2567 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2002 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1566 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1299 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1293 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1281 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1280 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1167 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1124 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1065 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1059 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1055 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1025 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1013 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 943 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 932 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 925 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 923 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 900 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 851 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 846 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 831 | 0.8% |
| `colpush_tick` | native/JVM-internal | 781 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 743 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 717 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 705 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 665 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 657 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 629 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 607 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 602 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63655)

| bucket | self-time samples | share |
|---|---|---|
| other | 60619 | 95.2% |
| entities/mobs (kernel) | 1130 | 1.8% |
| kernel: other | 699 | 1.1% |
| chunk system (kernel) | 247 | 0.4% |
| JDK collections | 217 | 0.3% |
| JIT stubs (vtable/itable) | 173 | 0.3% |
| moonrise/paper patches | 170 | 0.3% |
| fastutil collections | 144 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK other | 75 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61001 | 95.8% |
| phase: entity tick (AI/movement) | 1920 | 3.0% |
| phase: main tick (unclassified) | 468 | 0.7% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 29 | 0.0% |
| phase: chunk system (off-main worker) | 29 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54746** (86.0%) · native/JVM-internal **8904** (14.0%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51817 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4716 | 7.4% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 153 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 106 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `syscall` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `getdents64` | native/JVM-internal | 53 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 42 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 39 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6381)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6381 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 4998 | 78.3% |
| phase: entity tick (AI/movement) | 1005 | 15.7% |
| phase: main tick (unclassified) | 262 | 4.1% |
| phase: chunk system (off-main worker) | 36 | 0.6% |
| phase: block entities (hoppers/furnaces) | 33 | 0.5% |
| phase: network sync (ServerEntity) | 24 | 0.4% |
| phase: mob spawning | 12 | 0.2% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6381** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[i]` | other | 864 | 13.5% |
| `java.lang.Object[]_[i]` | other | 540 | 8.5% |
| `java.lang.String_[i]` | other | 490 | 7.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 457 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 448 | 7.0% |
| `char[]_[k]` | other | 431 | 6.8% |
| `byte[]_[k]` | other | 272 | 4.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 162 | 2.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 146 | 2.3% |
| `long[]_[i]` | other | 139 | 2.2% |
| `java.lang.Object[]_[k]` | other | 136 | 2.1% |
| `short[]_[k]` | other | 130 | 2.0% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 126 | 2.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 122 | 1.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 119 | 1.9% |
| `int[]_[i]` | other | 113 | 1.8% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 108 | 1.7% |
| `java.util.ArrayList_[i]` | other | 100 | 1.6% |
| `short[]_[i]` | other | 54 | 0.8% |
| `long[]_[k]` | other | 50 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105255 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18910 | 17.97% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6097 | 5.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5055 | 4.80% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3745 | 3.56% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1299 | 1.23% |
| `net/minecraft/world/entity/ai/Brain.tick` | 551 | 0.52% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 401 | 0.38% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 375 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 302 | 0.29% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 298 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 281 | 0.27% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 98 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[i]` | 864 | 13.5% |
| `java.lang.Object[]_[i]` | 540 | 8.5% |
| `java.lang.String_[i]` | 490 | 7.7% |
| `net.minecraft.world.phys.AABB_[i]` | 457 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 448 | 7.0% |
| `char[]_[k]` | 431 | 6.8% |
| `byte[]_[k]` | 272 | 4.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | 162 | 2.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 146 | 2.3% |
| `long[]_[i]` | 139 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 18491 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148251..153676 (delta 5425, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99934->106796, minecraft:drowned 3643->4545, minecraft:husk 4571->5427, minecraft:zombie 3805->4602, minecraft:skeleton 4248->4780, minecraft:creeper 4510->4979, minecraft:spider 4114->4476, minecraft:pig 2849->3176
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5425)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54019255 B)
- `wall-collapsed.txt` (3345282 B)
- `alloc-collapsed.txt` (3836165 B)
- `cpu-flamegraph.html` (284991 B)
- `server-stdout.log` (324965 B)
- `gc.log` (106629 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
