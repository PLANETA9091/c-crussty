# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.77 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.6, 2.0, 0.4, 0.0, 0.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T14:32:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7159725 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [crussty-plugin] cmp415_mcomp: ARMED hea | — | — | — | — | — | 4.0 |

- entity totals seen: [149244, 149273, 149285]
- top entity types (max seen): minecraft:item×101070, minecraft:skeleton×4871, minecraft:creeper×4723, minecraft:husk×4671, minecraft:zombie×4633, minecraft:drowned×4535, minecraft:spider×4280, minecraft:sheep×3516, minecraft:chicken×3400, minecraft:cow×3365, minecraft:pig×3172, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/G5wPwCzi5n
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **80** (Full GC: **9**)
- total pause: **14678.4 ms**, avg **183.48 ms**, max **2098.8 ms**
- heap high-water seen: **6451 MB** -> last-after: **3486 MB**
  - Young (Allocation Failure): 57
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (GCLocker Initiated GC): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113305)

| bucket | self-time samples | share |
|---|---|---|
| other | 38293 | 33.8% |
| entities/mobs (kernel) | 24411 | 21.5% |
| kernel: other | 19580 | 17.3% |
| chunk system (kernel) | 6821 | 6.0% |
| JDK collections | 5433 | 4.8% |
| moonrise/paper patches | 4498 | 4.0% |
| fastutil collections | 3798 | 3.4% |
| network (kernel) | 2767 | 2.4% |
| JIT stubs (vtable/itable) | 2284 | 2.0% |
| JDK other | 2026 | 1.8% |
| JDK invokes/VarHandle | 1709 | 1.5% |
| vdso (clock) | 1398 | 1.2% |
| craftbukkit glue | 88 | 0.1% |
| bukkit api | 77 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| block entities/hoppers (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 96007 | 84.7% |
| phase: unclassified | 8462 | 7.5% |
| phase: main tick (unclassified) | 3291 | 2.9% |
| phase: network sync (ServerEntity) | 1888 | 1.7% |
| phase: chunk tick | 1532 | 1.4% |
| phase: chunk system (off-main worker) | 1017 | 0.9% |
| phase: block entities (hoppers/furnaces) | 641 | 0.6% |
| phase: random tick | 378 | 0.3% |
| phase: mob spawning | 87 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **74174** (65.5%) · native/JVM-internal **38862** (34.3%) · other **269** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 29036 | 25.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3346 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2976 | 2.6% |
| `vtable stub` | native/JVM-internal | 1864 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1519 | 1.3% |
| `[vdso]` | native/JVM-internal | 1398 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1351 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1272 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1177 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1120 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1104 | 1.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1088 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1029 | 0.9% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fc78328dfc8.accept` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 947 | 0.8% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 847 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 845 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 832 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 830 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 796 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 776 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 766 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 729 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 725 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 717 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 706 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 690 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 618 | 0.5% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 607 | 0.5% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 591 | 0.5% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 587 | 0.5% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 582 | 0.5% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 538 | 0.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 533 | 0.5% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 531 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 525 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 60410)

| bucket | self-time samples | share |
|---|---|---|
| other | 59734 | 98.9% |
| vdso (clock) | 593 | 1.0% |
| entities/mobs (kernel) | 23 | 0.0% |
| kernel: other | 16 | 0.0% |
| bukkit api | 11 | 0.0% |
| craftbukkit glue | 9 | 0.0% |
| JDK collections | 6 | 0.0% |
| moonrise/paper patches | 5 | 0.0% |
| chunk system (kernel) | 4 | 0.0% |
| JIT stubs (vtable/itable) | 2 | 0.0% |
| network (kernel) | 2 | 0.0% |
| JDK invokes/VarHandle | 2 | 0.0% |
| JDK other | 2 | 0.0% |
| fastutil collections | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56812 | 94.0% |
| phase: entity tick (AI/movement) | 3240 | 5.4% |
| phase: main tick (unclassified) | 353 | 0.6% |
| phase: chunk tick | 3 | 0.0% |
| phase: chunk system (off-main worker) | 1 | 0.0% |
| phase: network sync (ServerEntity) | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48145** (79.7%) · native/JVM-internal **12263** (20.3%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47973 | 79.4% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.9% |
| `mob_query` | native/JVM-internal | 3184 | 5.3% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 593 | 1.0% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 59 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 43 | 0.1% |
| `org/spigotmc/WatchdogThread.monotonicMillis` | JVM-Java | 27 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 14 | 0.0% |
| `org/bukkit/Bukkit.getServer` | JVM-Java | 11 | 0.0% |
| `org/bukkit/craftbukkit/CraftServer.getLogger` | JVM-Java | 9 | 0.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4 | 0.0% |
| `getdents64` | native/JVM-internal | 4 | 0.0% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3 | 0.0% |
| `__poll` | native/JVM-internal | 3 | 0.0% |
| `clock_gettime@plt` | other | 2 | 0.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 2 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1539)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1539 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1505 | 97.8% |
| phase: entity tick (AI/movement) | 32 | 2.1% |
| phase: random tick | 1 | 0.1% |
| phase: chunk tick | 1 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1539** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 337 | 21.9% |
| `byte[]_[k]` | other | 254 | 16.5% |
| `int[]_[i]` | other | 172 | 11.2% |
| `byte[]_[i]` | other | 122 | 7.9% |
| `int[]_[k]` | other | 78 | 5.1% |
| `boolean[]_[i]` | other | 55 | 3.6% |
| `java.util.Calendar$Builder_[i]` | other | 55 | 3.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 55 | 3.6% |
| `java.util.regex.Matcher_[i]` | other | 54 | 3.5% |
| `java.util.GregorianCalendar_[k]` | other | 52 | 3.4% |
| `java.util.GregorianCalendar_[i]` | other | 43 | 2.8% |
| `java.lang.String_[i]` | other | 40 | 2.6% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 31 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 23 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 21 | 1.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 15 | 1.0% |
| `java.util.Calendar$Builder_[k]` | other | 11 | 0.7% |
| `char[]_[i]` | other | 9 | 0.6% |
| `java.math.BigInteger_[i]` | other | 9 | 0.6% |
| `java.util.regex.Matcher_[k]` | other | 9 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113305 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 30864 | 27.24% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16409 | 14.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 13309 | 11.75% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6461 | 5.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4125 | 3.64% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 970 | 0.86% |
| `net/minecraft/world/entity/ai/Brain.tick` | 460 | 0.41% |
| `net/minecraft/world/entity/npc/Villager.tick` | 340 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 205 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 190 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 169 | 0.15% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 115 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 337 | 21.9% |
| `byte[]_[k]` | 254 | 16.5% |
| `int[]_[i]` | 172 | 11.2% |
| `byte[]_[i]` | 122 | 7.9% |
| `int[]_[k]` | 78 | 5.1% |
| `boolean[]_[i]` | 55 | 3.6% |
| `java.util.Calendar$Builder_[i]` | 55 | 3.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | 55 | 3.6% |
| `java.util.regex.Matcher_[i]` | 54 | 3.5% |
| `java.util.GregorianCalendar_[k]` | 52 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 80 pauses / total 14678 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148291..149285 (delta 994, churn 0.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99965->101070, minecraft:spider 4040->4280, minecraft:drowned 4382->4535, minecraft:zombie 4482->4633, minecraft:husk 4531->4671, minecraft:creeper 4596->4723, minecraft:skeleton 4761->4871, minecraft:bee 21->29
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=994)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (35743583 B)
- `wall-collapsed.txt` (199072 B)
- `alloc-collapsed.txt` (499983 B)
- `cpu-flamegraph.html` (92332 B)
- `server-stdout.log` (425131 B)
- `gc.log` (77899 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
