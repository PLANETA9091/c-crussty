# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.887 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.0, 2.0, 1.9, 0.2, 0.1, 0.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T11:54:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6788478 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [11:56:31 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 4.0 |

- entity totals seen: [148923, 149045, 149123]
- top entity types (max seen): minecraft:item×100950, minecraft:skeleton×4792, minecraft:husk×4672, minecraft:creeper×4663, minecraft:zombie×4591, minecraft:drowned×4568, minecraft:spider×4251, minecraft:sheep×3492, minecraft:chicken×3422, minecraft:cow×3364, minecraft:pig×3159, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QSFyPb2JxA
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **72** (Full GC: **8**)
- total pause: **11702.6 ms**, avg **162.54 ms**, max **1901.2 ms**
- heap high-water seen: **6510 MB** -> last-after: **3287 MB**
  - Young (Allocation Failure): 47
  - Young (GCLocker Initiated GC): 9
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 108291)

| bucket | self-time samples | share |
|---|---|---|
| other | 42702 | 39.4% |
| entities/mobs (kernel) | 21305 | 19.7% |
| kernel: other | 15847 | 14.6% |
| chunk system (kernel) | 6584 | 6.1% |
| JDK collections | 4947 | 4.6% |
| moonrise/paper patches | 4309 | 4.0% |
| fastutil collections | 3824 | 3.5% |
| network (kernel) | 2299 | 2.1% |
| JDK invokes/VarHandle | 1843 | 1.7% |
| JIT stubs (vtable/itable) | 1695 | 1.6% |
| JDK other | 1655 | 1.5% |
| vdso (clock) | 1035 | 1.0% |
| block entities/hoppers (kernel) | 74 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| bukkit api | 45 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 68359 | 63.1% |
| phase: unclassified | 25629 | 23.7% |
| phase: main tick (unclassified) | 8382 | 7.7% |
| phase: network sync (ServerEntity) | 1966 | 1.8% |
| phase: chunk tick | 1780 | 1.6% |
| phase: chunk system (off-main worker) | 917 | 0.8% |
| phase: block entities (hoppers/furnaces) | 658 | 0.6% |
| phase: random tick | 369 | 0.3% |
| phase: mob spawning | 231 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **65033** (60.1%) · native/JVM-internal **43059** (39.8%) · other **199** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 33791 | 31.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3169 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2286 | 2.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1452 | 1.3% |
| `vtable stub` | native/JVM-internal | 1330 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1316 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1173 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1168 | 1.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1048 | 1.0% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1039 | 1.0% |
| `[vdso]` | native/JVM-internal | 1035 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 872 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 843 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 791 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 791 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 791 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 779 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 777 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 776 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 723 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 667 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 665 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 618 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 614 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 553 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 553 | 0.5% |
| `read` | native/JVM-internal | 552 | 0.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 551 | 0.5% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 536 | 0.5% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 535 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 531 | 0.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 530 | 0.5% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 526 | 0.5% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 508 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 60873)

| bucket | self-time samples | share |
|---|---|---|
| other | 60220 | 98.9% |
| vdso (clock) | 520 | 0.9% |
| entities/mobs (kernel) | 43 | 0.1% |
| kernel: other | 34 | 0.1% |
| chunk system (kernel) | 13 | 0.0% |
| JDK collections | 10 | 0.0% |
| fastutil collections | 10 | 0.0% |
| moonrise/paper patches | 9 | 0.0% |
| JIT stubs (vtable/itable) | 5 | 0.0% |
| network (kernel) | 4 | 0.0% |
| JDK invokes/VarHandle | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56969 | 93.6% |
| phase: entity tick (AI/movement) | 3450 | 5.7% |
| phase: main tick (unclassified) | 447 | 0.7% |
| phase: network sync (ServerEntity) | 4 | 0.0% |
| phase: random tick | 1 | 0.0% |
| phase: chunk tick | 1 | 0.0% |
| phase: chunk system (off-main worker) | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48490** (79.7%) · native/JVM-internal **12383** (20.3%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48285 | 79.3% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `mob_query` | native/JVM-internal | 3374 | 5.5% |
| `read` | native/JVM-internal | 1236 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 520 | 0.9% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 46 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 38 | 0.1% |
| `org/spigotmc/WatchdogThread.monotonicMillis` | JVM-Java | 27 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 12 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 5 | 0.0% |
| `vtable stub` | native/JVM-internal | 4 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4 | 0.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/entity/LivingEntity.tick` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 3 | 0.0% |
| `clock_gettime` | native/JVM-internal | 3 | 0.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3 | 0.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 3 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1288)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1288 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1253 | 97.3% |
| phase: entity tick (AI/movement) | 22 | 1.7% |
| phase: main tick (unclassified) | 7 | 0.5% |
| phase: network sync (ServerEntity) | 2 | 0.2% |
| phase: chunk system (off-main worker) | 2 | 0.2% |
| phase: block entities (hoppers/furnaces) | 2 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1288** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 260 | 20.2% |
| `byte[]_[k]` | other | 233 | 18.1% |
| `byte[]_[i]` | other | 151 | 11.7% |
| `int[]_[i]` | other | 121 | 9.4% |
| `int[]_[k]` | other | 85 | 6.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 70 | 5.4% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 62 | 4.8% |
| `java.util.GregorianCalendar_[i]` | other | 33 | 2.6% |
| `java.util.GregorianCalendar_[k]` | other | 27 | 2.1% |
| `java.util.Calendar$Builder_[i]` | other | 26 | 2.0% |
| `boolean[]_[i]` | other | 21 | 1.6% |
| `java.util.regex.Matcher_[i]` | other | 21 | 1.6% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 18 | 1.4% |
| `java.lang.String_[i]` | other | 16 | 1.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 14 | 1.1% |
| `char[]_[i]` | other | 14 | 1.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 13 | 1.0% |
| `java.util.ArrayList$Itr_[i]` | other | 12 | 0.9% |
| `java.lang.Object[]_[i]` | other | 9 | 0.7% |
| `java.util.regex.Matcher_[k]` | other | 8 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108291 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 16824 | 15.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 12585 | 11.62% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6570 | 6.07% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4067 | 3.76% |
| `net/minecraft/world/entity/monster/Slime.tick` | 864 | 0.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 814 | 0.75% |
| `net/minecraft/world/entity/ai/Brain.tick` | 457 | 0.42% |
| `net/minecraft/world/entity/npc/Villager.tick` | 343 | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 215 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 197 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 178 | 0.16% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 118 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 260 | 20.2% |
| `byte[]_[k]` | 233 | 18.1% |
| `byte[]_[i]` | 151 | 11.7% |
| `int[]_[i]` | 121 | 9.4% |
| `int[]_[k]` | 85 | 6.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | 70 | 5.4% |
| `sun.util.calendar.Gregorian$Date_[k]` | 62 | 4.8% |
| `java.util.GregorianCalendar_[i]` | 33 | 2.6% |
| `java.util.GregorianCalendar_[k]` | 27 | 2.1% |
| `java.util.Calendar$Builder_[i]` | 26 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 72 pauses / total 11703 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148270..149123 (delta 853, churn 0.6%), summons=0
  - top movers (max-min across polls): minecraft:item 100027->100950, minecraft:spider 4034->4251, minecraft:husk 4564->4672, minecraft:creeper 4556->4663, minecraft:zombie 4485->4591, minecraft:skeleton 4696->4792, minecraft:drowned 4477->4568, minecraft:enderman 14->23
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=853)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (31950679 B)
- `wall-collapsed.txt` (295776 B)
- `alloc-collapsed.txt` (469983 B)
- `cpu-flamegraph.html` (68220 B)
- `server-stdout.log` (501997 B)
- `gc.log` (70085 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
