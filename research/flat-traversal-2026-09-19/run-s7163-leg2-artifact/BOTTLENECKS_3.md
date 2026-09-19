# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.254 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.3, 1.5, 1.6, 1.8, 2.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T00:59:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6340327 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 1 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148356, 148935, 150111]
- top entity types (max seen): minecraft:item×103097, minecraft:husk×4992, minecraft:creeper×4984, minecraft:skeleton×4887, minecraft:zombie×4672, minecraft:spider×4601, minecraft:drowned×4550, minecraft:sheep×3525, minecraft:chicken×3445, minecraft:cow×3384, minecraft:pig×3299, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XhEncy7gbn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **189** (Full GC: **0**)
- total pause: **17363.8 ms**, avg **91.87 ms**, max **190.9 ms**
- heap high-water seen: **6975 MB** -> last-after: **3985 MB**
  - Young (Mixed) (G1 Evacuation Pause): 38
  - Remark: 30
  - Cleanup: 30
  - Young (Prepare Mixed) (G1 Evacuation Pause): 30
  - Young (Normal) (G1 Evacuation Pause): 29
  - Young (Concurrent Start) (G1 Evacuation Pause): 25

### CPU profile — self-time by research bucket (total self-time samples: 125610)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 22122 | 17.6% |
| kernel: other | 21924 | 17.5% |
| JVM internals (G1 GC) | 16635 | 13.2% |
| other | 15579 | 12.4% |
| JVM internals (GC oop barriers) | 13734 | 10.9% |
| moonrise/paper patches | 8238 | 6.6% |
| chunk system (kernel) | 7762 | 6.2% |
| fastutil collections | 5890 | 4.7% |
| JDK collections | 4728 | 3.8% |
| JIT stubs (vtable/itable) | 2627 | 2.1% |
| network (kernel) | 2624 | 2.1% |
| JDK invokes/VarHandle | 1799 | 1.4% |
| JDK other | 1545 | 1.2% |
| vdso (clock) | 162 | 0.1% |
| bukkit api | 62 | 0.0% |
| block entities/hoppers (kernel) | 61 | 0.0% |
| craftbukkit glue | 52 | 0.0% |
| redstone (kernel) | 33 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 73435 | 58.5% |
| phase: unclassified | 43968 | 35.0% |
| phase: main tick (unclassified) | 3039 | 2.4% |
| phase: chunk tick | 1723 | 1.4% |
| phase: network sync (ServerEntity) | 1449 | 1.2% |
| phase: chunk system (off-main worker) | 970 | 0.8% |
| phase: block entities (hoppers/furnaces) | 579 | 0.5% |
| phase: random tick | 341 | 0.3% |
| phase: mob spawning | 106 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **79487** (63.3%) · native/JVM-internal **46014** (36.6%) · other **109** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5822 | 4.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5551 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3602 | 2.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2867 | 2.3% |
| `G1CardSet::add_card` | native/JVM-internal | 2841 | 2.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2694 | 2.1% |
| `vtable stub` | native/JVM-internal | 2106 | 1.7% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1871 | 1.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1862 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1499 | 1.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1493 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1473 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1406 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1398 | 1.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1374 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1372 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1337 | 1.1% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1271 | 1.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1198 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1166 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1153 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1146 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1145 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1111 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1099 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1065 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1054 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1044 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 995 | 0.8% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 988 | 0.8% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fd4259e7490.accept` | JVM-Java | 865 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 831 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 826 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 799 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 793 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 767 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 763 | 0.6% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 762 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 690 | 0.5% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 680 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69648)

| bucket | self-time samples | share |
|---|---|---|
| other | 65263 | 93.7% |
| entities/mobs (kernel) | 910 | 1.3% |
| kernel: other | 852 | 1.2% |
| JVM internals (G1 GC) | 633 | 0.9% |
| JVM internals (GC oop barriers) | 513 | 0.7% |
| chunk system (kernel) | 342 | 0.5% |
| moonrise/paper patches | 316 | 0.5% |
| fastutil collections | 263 | 0.4% |
| JDK collections | 190 | 0.3% |
| JIT stubs (vtable/itable) | 133 | 0.2% |
| network (kernel) | 98 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| JDK other | 51 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66046 | 94.8% |
| phase: entity tick (AI/movement) | 3149 | 4.5% |
| phase: main tick (unclassified) | 267 | 0.4% |
| phase: network sync (ServerEntity) | 54 | 0.1% |
| phase: chunk tick | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.0% |
| phase: chunk system (off-main worker) | 29 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59225** (85.0%) · native/JVM-internal **10410** (14.9%) · other **13** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56032 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 6.9% |
| `read` | native/JVM-internal | 1234 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 220 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 215 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 148 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 127 | 0.2% |
| `syscall` | native/JVM-internal | 120 | 0.2% |
| `vtable stub` | native/JVM-internal | 109 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 91 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 76 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 61 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 16391)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 16391 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 8011 | 48.9% |
| phase: unclassified | 7918 | 48.3% |
| phase: main tick (unclassified) | 282 | 1.7% |
| phase: network sync (ServerEntity) | 88 | 0.5% |
| phase: chunk system (off-main worker) | 29 | 0.2% |
| phase: block entities (hoppers/furnaces) | 22 | 0.1% |
| phase: chunk tick | 20 | 0.1% |
| phase: mob spawning | 14 | 0.1% |
| phase: random tick | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **16391** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 2143 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2007 | 12.2% |
| `byte[]_[i]` | other | 1735 | 10.6% |
| `java.lang.String_[i]` | other | 1007 | 6.1% |
| `java.lang.Object[]_[i]` | other | 965 | 5.9% |
| `long[]_[i]` | other | 840 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 529 | 3.2% |
| `char[]_[k]` | other | 502 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 453 | 2.8% |
| `byte[]_[k]` | other | 418 | 2.6% |
| `short[]_[k]` | other | 368 | 2.2% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 343 | 2.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 296 | 1.8% |
| `java.util.ArrayList_[i]` | other | 289 | 1.8% |
| `java.lang.Object[]_[k]` | other | 244 | 1.5% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 217 | 1.3% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 195 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 173 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 170 | 1.0% |
| `short[]_[i]` | other | 165 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 125610 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28324 | 22.55% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 17568 | 13.99% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5092 | 4.05% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4146 | 3.30% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3559 | 2.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 903 | 0.72% |
| `net/minecraft/world/entity/ai/Brain.tick` | 788 | 0.63% |
| `net/minecraft/world/entity/npc/Villager.tick` | 387 | 0.31% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 231 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 169 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 152 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 2143 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 2007 | 12.2% |
| `byte[]_[i]` | 1735 | 10.6% |
| `java.lang.String_[i]` | 1007 | 6.1% |
| `java.lang.Object[]_[i]` | 965 | 5.9% |
| `long[]_[i]` | 840 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 529 | 3.2% |
| `char[]_[k]` | 502 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 453 | 2.8% |
| `byte[]_[k]` | 418 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 189 pauses / total 17364 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148253..150111 (delta 1858, churn 1.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99352->103097, minecraft:drowned 3478->4550, minecraft:zombie 3698->4672, minecraft:skeleton 4369->4887, minecraft:husk 4579->4992, minecraft:spider 4233->4601, minecraft:creeper 4623->4984, minecraft:pig 3242->3299
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1858)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49454654 B)
- `wall-collapsed.txt` (3410923 B)
- `alloc-collapsed.txt` (8854985 B)
- `cpu-flamegraph.html` (293891 B)
- `server-stdout.log` (255258 B)
- `gc.log` (269805 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
