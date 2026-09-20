# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.866 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 1.5, 2.0, 2.1, 2.4, 2.6]
- spark tick-monitor MSPT: avg **424.55ms** / min 362.46ms / max **579.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T21:07:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6939260 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 362.46 | — | — | — | 579.5 | 424.55 |

- entity totals seen: [149012, 150167, 151434]
- top entity types (max seen): minecraft:item×103260, minecraft:husk×5230, minecraft:creeper×5218, minecraft:skeleton×4859, minecraft:spider×4844, minecraft:zombie×4715, minecraft:drowned×4594, minecraft:sheep×3513, minecraft:chicken×3425, minecraft:cow×3385, minecraft:pig×3282, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/11ysdeEx1F
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **21846.8 ms**, avg **183.59 ms**, max **2833.2 ms**
- heap high-water seen: **7796 MB** -> last-after: **3631 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 114847)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29622 | 25.8% |
| kernel: other | 27931 | 24.3% |
| other | 11373 | 9.9% |
| moonrise/paper patches | 10074 | 8.8% |
| chunk system (kernel) | 9294 | 8.1% |
| fastutil collections | 8101 | 7.1% |
| JDK collections | 6249 | 5.4% |
| JIT stubs (vtable/itable) | 3873 | 3.4% |
| network (kernel) | 3261 | 2.8% |
| JDK invokes/VarHandle | 2625 | 2.3% |
| JDK other | 1910 | 1.7% |
| vdso (clock) | 211 | 0.2% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| bukkit api | 75 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95334 | 83.0% |
| phase: unclassified | 9475 | 8.3% |
| phase: main tick (unclassified) | 3642 | 3.2% |
| phase: chunk tick | 2176 | 1.9% |
| phase: network sync (ServerEntity) | 1860 | 1.6% |
| phase: chunk system (off-main worker) | 1161 | 1.0% |
| phase: block entities (hoppers/furnaces) | 663 | 0.6% |
| phase: random tick | 435 | 0.4% |
| phase: mob spawning | 98 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102958** (89.6%) · native/JVM-internal **11788** (10.3%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4404 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3623 | 3.2% |
| `vtable stub` | native/JVM-internal | 3213 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2630 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1933 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1831 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1764 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1746 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1696 | 1.5% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1664 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1608 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1604 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1550 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1392 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1384 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1358 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1177 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1105 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1068 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1066 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1035 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1014 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 996 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 939 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 869 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 840 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 834 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 818 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 794 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 785 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 755 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 734 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 684 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 669 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61234)

| bucket | self-time samples | share |
|---|---|---|
| other | 57864 | 94.5% |
| entities/mobs (kernel) | 982 | 1.6% |
| kernel: other | 880 | 1.4% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 302 | 0.5% |
| fastutil collections | 259 | 0.4% |
| JDK collections | 199 | 0.3% |
| JIT stubs (vtable/itable) | 143 | 0.2% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 96 | 0.2% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57683 | 94.2% |
| phase: entity tick (AI/movement) | 3125 | 5.1% |
| phase: main tick (unclassified) | 172 | 0.3% |
| phase: chunk tick | 80 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 49 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52328** (85.5%) · native/JVM-internal **8898** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49005 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 146 | 0.2% |
| `vtable stub` | native/JVM-internal | 108 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 81 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 8897)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 8897 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 6622 | 74.4% |
| phase: entity tick (AI/movement) | 2115 | 23.8% |
| phase: main tick (unclassified) | 82 | 0.9% |
| phase: chunk system (off-main worker) | 35 | 0.4% |
| phase: network sync (ServerEntity) | 23 | 0.3% |
| phase: block entities (hoppers/furnaces) | 13 | 0.1% |
| phase: chunk tick | 3 | 0.0% |
| phase: random tick | 2 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **8897** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[i]` | other | 1393 | 15.7% |
| `java.lang.String_[i]` | other | 899 | 10.1% |
| `char[]_[k]` | other | 704 | 7.9% |
| `java.lang.Object[]_[i]` | other | 603 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 598 | 6.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 536 | 6.0% |
| `byte[]_[k]` | other | 336 | 3.8% |
| `java.lang.Object[]_[k]` | other | 235 | 2.6% |
| `short[]_[k]` | other | 233 | 2.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 227 | 2.6% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 208 | 2.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 187 | 2.1% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 167 | 1.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 1.5% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 127 | 1.4% |
| `java.util.ArrayList_[i]` | other | 122 | 1.4% |
| `long[]_[i]` | other | 122 | 1.4% |
| `short[]_[i]` | other | 104 | 1.2% |
| `int[]_[i]` | other | 104 | 1.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 83 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 114847 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35841 | 31.21% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23345 | 20.33% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6525 | 5.68% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5469 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4689 | 4.08% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1108 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 920 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 452 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 266 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 216 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 199 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[i]` | 1393 | 15.7% |
| `java.lang.String_[i]` | 899 | 10.1% |
| `char[]_[k]` | 704 | 7.9% |
| `java.lang.Object[]_[i]` | 603 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 598 | 6.7% |
| `net.minecraft.world.phys.AABB_[i]` | 536 | 6.0% |
| `byte[]_[k]` | 336 | 3.8% |
| `java.lang.Object[]_[k]` | 235 | 2.6% |
| `short[]_[k]` | 233 | 2.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | 227 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 21847 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148076..151434 (delta 3358, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99380->103260, minecraft:drowned 3449->4594, minecraft:zombie 3664->4715, minecraft:husk 4533->5230, minecraft:creeper 4538->5218, minecraft:spider 4224->4844, minecraft:skeleton 4393->4859, minecraft:chicken 3381->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3358)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54165210 B)
- `wall-collapsed.txt` (3675201 B)
- `alloc-collapsed.txt` (5358802 B)
- `cpu-flamegraph.html` (285106 B)
- `server-stdout.log` (250656 B)
- `gc.log` (112729 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
