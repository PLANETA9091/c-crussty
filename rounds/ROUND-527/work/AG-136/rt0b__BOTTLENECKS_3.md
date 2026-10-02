# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.231 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 0.3, 0.3, 0.3, 0.3, 0.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-10-02T21:29:55Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6748332 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384 + S99-gcw ROUND-468-S15; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity; 6 = ParallelGC + MetaspaceSize=256M + ReservedCodeCacheSize=512M — threshold-cascade kill, javap-neutral)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- fluid_dirty_ledger: 0 (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)
- fluid_bitmask: 0 (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)
- region_threads: 0 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
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
- lever_flag= lever_arg= (MEGA-ROUND lever A/B marker, TASK-395; empty = vanilla bit-in-bit — canary-gate SKIP-ARMED/C85 layer-2 reads this)
- DP-INSTALLED sha256=16fa1a32cb71966cf1e779cc0c32e1205992738fd92c865650455b9022c19ee2 files=707 dir=/home/runner/work/c-crussty/c-crussty/world3-run/server/world/datapacks/stz3v2

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148024, 148034, 147997]
- top entity types (max seen): minecraft:item×99210, minecraft:skeleton×4907, minecraft:creeper×4696, minecraft:zombie×4671, minecraft:husk×4653, minecraft:drowned×4546, minecraft:spider×4533, minecraft:sheep×3630, minecraft:chicken×3496, minecraft:cow×3454, minecraft:pig×3411, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/oykhWXJVlI
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **64** (Full GC: **9**)
- total pause: **14500.2 ms**, avg **226.57 ms**, max **2499.1 ms**
- heap high-water seen: **6448 MB** -> last-after: **3298 MB**
  - Young (Allocation Failure): 46
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 38394)

| bucket | self-time samples | share |
|---|---|---|
| moonrise/paper patches | 10987 | 28.6% |
| entities/mobs (kernel) | 10644 | 27.7% |
| other | 6335 | 16.5% |
| kernel: other | 3933 | 10.2% |
| chunk system (kernel) | 1644 | 4.3% |
| JDK collections | 1141 | 3.0% |
| fastutil collections | 961 | 2.5% |
| JDK invokes/VarHandle | 798 | 2.1% |
| JIT stubs (vtable/itable) | 664 | 1.7% |
| network (kernel) | 620 | 1.6% |
| JDK other | 434 | 1.1% |
| vdso (clock) | 182 | 0.5% |
| block entities/hoppers (kernel) | 18 | 0.0% |
| craftbukkit glue | 14 | 0.0% |
| bukkit api | 9 | 0.0% |
| redstone (kernel) | 6 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: main tick (unclassified) | 21090 | 54.9% |
| phase: entity tick (AI/movement) | 13226 | 34.4% |
| phase: unclassified | 2769 | 7.2% |
| phase: network sync (ServerEntity) | 423 | 1.1% |
| phase: chunk tick | 363 | 0.9% |
| phase: chunk system (off-main worker) | 310 | 0.8% |
| phase: block entities (hoppers/furnaces) | 113 | 0.3% |
| phase: random tick | 72 | 0.2% |
| phase: mob spawning | 27 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **34912** (90.9%) · native/JVM-internal **3475** (9.1%) · other **7** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.get` | JVM-Java | 9183 | 23.9% |
| `net/minecraft/world/entity/Entity.moonrise$getChunkStatus` | JVM-Java | 3162 | 8.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable$NodeIterator.findNext` | JVM-Java | 3130 | 8.2% |
| `net/minecraft/world/entity/Entity.getType` | JVM-Java | 2835 | 7.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 859 | 2.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 789 | 2.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 617 | 1.6% |
| `vtable stub` | native/JVM-internal | 565 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 464 | 1.2% |
| `read` | native/JVM-internal | 460 | 1.2% |
| `net/minecraft/world/entity/Entity.isAlwaysTicking` | JVM-Java | 437 | 1.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 409 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 368 | 1.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 350 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 289 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 271 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 268 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 259 | 0.7% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 254 | 0.7% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 254 | 0.7% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 232 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 204 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 198 | 0.5% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 198 | 0.5% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 197 | 0.5% |
| `net/minecraft/world/entity/monster/Zombie.getType` | JVM-Java | 195 | 0.5% |
| `[vdso]` | native/JVM-internal | 182 | 0.5% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 175 | 0.5% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 171 | 0.4% |
| `net/minecraft/world/entity/EntityType.tryCast` | JVM-Java | 164 | 0.4% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 152 | 0.4% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 152 | 0.4% |
| `java/lang/Enum.ordinal` | JVM-Java | 152 | 0.4% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 151 | 0.4% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 150 | 0.4% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 150 | 0.4% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 149 | 0.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 148 | 0.4% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 146 | 0.4% |
| `java/util/ArrayDeque.size` | JVM-Java | 140 | 0.4% |

### WALL profile — self-time by research bucket (total self-time samples: 57627)

| bucket | self-time samples | share |
|---|---|---|
| other | 56583 | 98.2% |
| moonrise/paper patches | 361 | 0.6% |
| entities/mobs (kernel) | 361 | 0.6% |
| kernel: other | 132 | 0.2% |
| chunk system (kernel) | 44 | 0.1% |
| JDK collections | 33 | 0.1% |
| fastutil collections | 30 | 0.1% |
| network (kernel) | 28 | 0.0% |
| JIT stubs (vtable/itable) | 17 | 0.0% |
| JDK invokes/VarHandle | 16 | 0.0% |
| JDK other | 15 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56427 | 97.9% |
| phase: main tick (unclassified) | 716 | 1.2% |
| phase: entity tick (AI/movement) | 444 | 0.8% |
| phase: network sync (ServerEntity) | 15 | 0.0% |
| phase: chunk tick | 11 | 0.0% |
| phase: chunk system (off-main worker) | 8 | 0.0% |
| phase: block entities (hoppers/furnaces) | 3 | 0.0% |
| phase: random tick | 2 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **49132** (85.3%) · native/JVM-internal **8494** (14.7%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47966 | 83.2% |
| `clock_nanosleep` | native/JVM-internal | 4625 | 8.0% |
| `read` | native/JVM-internal | 1218 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.1% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.get` | JVM-Java | 290 | 0.5% |
| `getdents64` | native/JVM-internal | 151 | 0.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable$NodeIterator.findNext` | JVM-Java | 121 | 0.2% |
| `net/minecraft/world/entity/Entity.moonrise$getChunkStatus` | JVM-Java | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.getType` | JVM-Java | 86 | 0.1% |
| `net/minecraft/world/entity/Entity.isAlwaysTicking` | JVM-Java | 26 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 19 | 0.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 19 | 0.0% |
| `vtable stub` | native/JVM-internal | 16 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 16 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 15 | 0.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 14 | 0.0% |
| `java/util/HashMap.getNode` | JVM-Java | 12 | 0.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 12 | 0.0% |
| `llseek` | native/JVM-internal | 12 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1600)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1600 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1362 | 85.1% |
| phase: entity tick (AI/movement) | 218 | 13.6% |
| phase: chunk system (off-main worker) | 11 | 0.7% |
| phase: main tick (unclassified) | 7 | 0.4% |
| phase: block entities (hoppers/furnaces) | 1 | 0.1% |
| phase: mob spawning | 1 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1600** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[k]` | other | 228 | 14.2% |
| `char[]_[k]` | other | 217 | 13.6% |
| `byte[]_[i]` | other | 126 | 7.9% |
| `short[]_[k]` | other | 109 | 6.8% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 79 | 4.9% |
| `java.lang.Object[]_[i]` | other | 60 | 3.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 58 | 3.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 47 | 2.9% |
| `short[]_[i]` | other | 47 | 2.9% |
| `long[]_[i]` | other | 33 | 2.1% |
| `java.lang.String_[i]` | other | 33 | 2.1% |
| `long[]_[k]` | other | 28 | 1.8% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 26 | 1.6% |
| `int[]_[i]` | other | 23 | 1.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 21 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 20 | 1.2% |
| `java.util.Optional_[i]` | other | 19 | 1.2% |
| `java.lang.Object[]_[k]` | other | 18 | 1.1% |
| `java.util.ArrayList_[i]` | other | 16 | 1.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 15 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 38394 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 5402 | 14.07% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 2872 | 7.48% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 862 | 2.25% |
| `net/minecraft/world/entity/monster/Spider.tick` | 831 | 2.16% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 664 | 1.73% |
| `net/minecraft/world/entity/ai/Brain.tick` | 153 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 113 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 92 | 0.24% |
| `net/minecraft/world/entity/npc/Villager.tick` | 82 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 44 | 0.11% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 29 | 0.08% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 27 | 0.07% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[k]` | 228 | 14.2% |
| `char[]_[k]` | 217 | 13.6% |
| `byte[]_[i]` | 126 | 7.9% |
| `short[]_[k]` | 109 | 6.8% |
| `com.mojang.serialization.DataResult$Success_[i]` | 79 | 4.9% |
| `java.lang.Object[]_[i]` | 60 | 3.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 58 | 3.6% |
| `net.minecraft.world.phys.AABB_[i]` | 47 | 2.9% |
| `short[]_[i]` | 47 | 2.9% |
| `long[]_[i]` | 33 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 64 pauses / total 14500 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147997..148058 (delta 61, churn 0.0%), summons=0
  - top movers (max-min across polls): minecraft:item 98668->99210, minecraft:pig 3269->3411, minecraft:cow 3357->3454, minecraft:spider 4445->4533, minecraft:sheep 3544->3630, minecraft:chicken 3440->3496, minecraft:skeleton 4873->4907, minecraft:zombie 4640->4671
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=61)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (11582949 B)
- `wall-collapsed.txt` (749319 B)
- `alloc-collapsed.txt` (1277375 B)
- `cpu-flamegraph.html` (141267 B)
- `server-stdout.log` (257172 B)
- `gc.log` (65403 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
