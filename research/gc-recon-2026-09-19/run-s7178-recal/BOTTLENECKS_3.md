# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.821 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 4, first-of-window values: [24.3, 1.4, 1.4, 1.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T17:30:43Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8493973 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 0 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0; requires region_threads>=2; TASK-333)
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


- entity totals seen: [148014, 147942, 148109]
- top entity types (max seen): minecraft:item×100037, minecraft:skeleton×4872, minecraft:creeper×4684, minecraft:zombie×4634, minecraft:husk×4630, minecraft:drowned×4519, minecraft:spider×4469, minecraft:sheep×3549, minecraft:chicken×3448, minecraft:cow×3357, minecraft:pig×3284, minecraft:item_frame×2714
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **210** (Full GC: **0**)
- total pause: **16013.7 ms**, avg **76.26 ms**, max **153.5 ms**
- heap high-water seen: **6076 MB** -> last-after: **3853 MB**
  - Young (Normal) (G1 Evacuation Pause): 39
  - Remark: 34
  - Cleanup: 34
  - Young (Mixed) (G1 Evacuation Pause): 33
  - Young (Prepare Mixed) (G1 Evacuation Pause): 32
  - Young (Concurrent Start) (G1 Evacuation Pause): 27

### CPU profile — self-time by research bucket (total self-time samples: 126345)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 19375 | 15.3% |
| entities/mobs (kernel) | 19274 | 15.3% |
| kernel: other | 18815 | 14.9% |
| other | 16782 | 13.3% |
| JVM internals (GC oop barriers) | 16469 | 13.0% |
| moonrise/paper patches | 7941 | 6.3% |
| chunk system (kernel) | 7621 | 6.0% |
| fastutil collections | 6326 | 5.0% |
| JDK collections | 5020 | 4.0% |
| network (kernel) | 2796 | 2.2% |
| JIT stubs (vtable/itable) | 2166 | 1.7% |
| JDK invokes/VarHandle | 1968 | 1.6% |
| JDK other | 1369 | 1.1% |
| vdso (clock) | 166 | 0.1% |
| redstone (kernel) | 65 | 0.1% |
| block entities/hoppers (kernel) | 64 | 0.1% |
| bukkit api | 61 | 0.0% |
| craftbukkit glue | 51 | 0.0% |
| worldgen/noise (kernel) | 13 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 66731 | 52.8% |
| phase: unclassified | 50597 | 40.0% |
| phase: main tick (unclassified) | 3183 | 2.5% |
| phase: chunk tick | 1995 | 1.6% |
| phase: network sync (ServerEntity) | 1796 | 1.4% |
| phase: chunk system (off-main worker) | 950 | 0.8% |
| phase: block entities (hoppers/furnaces) | 633 | 0.5% |
| phase: random tick | 343 | 0.3% |
| phase: mob spawning | 116 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **73937** (58.5%) · native/JVM-internal **52280** (41.4%) · other **128** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 7152 | 5.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 6165 | 4.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3815 | 3.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3762 | 3.0% |
| `G1CardSet::add_card` | native/JVM-internal | 3201 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2657 | 2.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2007 | 1.6% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1938 | 1.5% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1854 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1710 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1695 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1631 | 1.3% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1590 | 1.3% |
| `vtable stub` | native/JVM-internal | 1574 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1548 | 1.2% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1515 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1480 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1476 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1421 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1348 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1338 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1286 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1274 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1254 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1192 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1150 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1064 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1027 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 977 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 910 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 895 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 841 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 828 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 820 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 773 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 765 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 761 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 747 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 744 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 126345 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 26381 | 20.88% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 15627 | 12.37% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4571 | 3.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3842 | 3.04% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3306 | 2.62% |
| `net/minecraft/world/entity/ai/Brain.tick` | 694 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 608 | 0.48% |
| `net/minecraft/world/entity/npc/Villager.tick` | 326 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 188 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 187 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 168 | 0.13% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 153 | 0.12% |
- **F2 GC-churn estimate:** 210 pauses / total 16014 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=3 total=147942..148109 (delta 167, churn 0.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99169->100037, minecraft:spider 4201->4469, minecraft:skeleton 4737->4872, minecraft:zombie 4518->4634, minecraft:drowned 4416->4519, minecraft:pig 3232->3284, minecraft:husk 4583->4630, minecraft:cow 3315->3357
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=3, delta=167)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (38501938 B)
- `server-stdout.log` (232396 B)
- `gc.log` (297454 B)
- `ap.log` (197 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
