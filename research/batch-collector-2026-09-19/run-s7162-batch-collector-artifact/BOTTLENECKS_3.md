# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.097 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 1, first-of-window values: [19.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T21:23:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7025009 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.

- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **46** (Full GC: **0**)
- total pause: **1824.7 ms**, avg **39.67 ms**, max **139.3 ms**
- heap high-water seen: **3567 MB** -> last-after: **2286 MB**
  - Young (Mixed) (G1 Evacuation Pause): 10
  - Remark: 8
  - Cleanup: 8
  - Young (Prepare Mixed) (G1 Evacuation Pause): 8
  - Young (Concurrent Start) (Metadata GC Threshold): 4
  - Young (Concurrent Start) (G1 Evacuation Pause): 4

### CPU profile — self-time by research bucket (total self-time samples: 96604)

| bucket | self-time samples | share |
|---|---|---|
| fastutil collections | 96013 | 99.4% |
| other | 590 | 0.6% |
| JDK collections | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 48599 | 50.3% |
| phase: main tick (unclassified) | 48005 | 49.7% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96017** (99.4%) · native/JVM-internal **587** (0.6%) · other **0** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.find` | JVM-Java | 48005 | 49.7% |
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.shiftKeys` | JVM-Java | 47355 | 49.0% |
| `it/unimi/dsi/fastutil/HashCommon.mix` | JVM-Java | 653 | 0.7% |
| `read` | native/JVM-internal | 541 | 0.6% |
| `IndexSetIterator::advance_and_next` | native/JVM-internal | 3 | 0.0% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 2 | 0.0% |
| `open` | native/JVM-internal | 2 | 0.0% |
| `PhaseLive::compute` | native/JVM-internal | 2 | 0.0% |
| `PhaseIdealLoop::Dominators` | native/JVM-internal | 2 | 0.0% |
| `PhaseAggressiveCoalesce::insert_copies` | native/JVM-internal | 1 | 0.0% |
| `GrowableArrayResourceAllocator::allocate` | native/JVM-internal | 1 | 0.0% |
| `MultiNode::proj_out_or_null` | native/JVM-internal | 1 | 0.0% |
| `NodeHash::hash_find_insert` | native/JVM-internal | 1 | 0.0% |
| `PhaseCFG::schedule_local` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::build_ifg_physical` | native/JVM-internal | 1 | 0.0% |
| `RegionNode::is_CFG` | native/JVM-internal | 1 | 0.0% |
| `java/util/concurrent/ThreadPoolExecutor.runWorker` | JVM-Java | 1 | 0.0% |
| `NodeHash::hash_delete` | native/JVM-internal | 1 | 0.0% |
| `CallNode::Value` | native/JVM-internal | 1 | 0.0% |
| `Compile::final_graph_reshaping_walk` | native/JVM-internal | 1 | 0.0% |
| `Node::remove_dead_region` | native/JVM-internal | 1 | 0.0% |
| `__close` | native/JVM-internal | 1 | 0.0% |
| `MachCallJavaNode::in_RegMask` | native/JVM-internal | 1 | 0.0% |
| `PhaseOutput::BuildOopMaps` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::Split` | native/JVM-internal | 1 | 0.0% |
| `PhaseIterGVN::remove_globally_dead_node` | native/JVM-internal | 1 | 0.0% |
| `Node::set_req_X` | native/JVM-internal | 1 | 0.0% |
| `IfNode::Ideal_common` | native/JVM-internal | 1 | 0.0% |
| `NTarjan::DFS` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::Simplify` | native/JVM-internal | 1 | 0.0% |
| `State::_sub_Op_AddI` | native/JVM-internal | 1 | 0.0% |
| `PhaseIdealLoop::compute_lca_of_uses` | native/JVM-internal | 1 | 0.0% |
| `Type::hashcons` | native/JVM-internal | 1 | 0.0% |
| `Unique_Node_List::remove` | native/JVM-internal | 1 | 0.0% |
| `PhaseIdealLoop::build_loop_tree_impl` | native/JVM-internal | 1 | 0.0% |
| `PhaseLive::add_liveout` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::elide_copy` | native/JVM-internal | 1 | 0.0% |
| `Parse::do_all_blocks` | native/JVM-internal | 1 | 0.0% |
| `Parse::return_current` | native/JVM-internal | 1 | 0.0% |
| `PhaseCFG::sched_call` | native/JVM-internal | 1 | 0.0% |

### WALL profile — self-time by research bucket (total self-time samples: 8)

| bucket | self-time samples | share |
|---|---|---|
| other | 8 | 100.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 8 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **8** (100.0%) · native/JVM-internal **0** (0.0%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 8 | 100.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 16)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 16 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 16 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **16** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[i]` | other | 8 | 50.0% |
| `byte[]_[i]` | other | 7 | 43.8% |
| `int[]_[i]` | other | 1 | 6.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 96604 = **0.00%** — §8 PASS (< 2%)
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[i]` | 8 | 50.0% |
| `byte[]_[i]` | 7 | 43.8% |
| `int[]_[i]` | 1 | 6.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 46 pauses / total 1825 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** insufficient polls (need >=2 `paper entity list` outputs)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls: NONE parsed from `paper mobcaps world` output
- alive-check heartbeat: NONE parsed (plugin heartbeat missing — check plugin log)
- gate 1a spawnable chunks > 0: FAIL
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): FAIL (summons=0, polls=0, delta=n/a)
- gate 1c alive-check steady at N=4: FAIL

- **FIXTURE-VALIDITY: INVALID**
  - bench-4 run INVALID as owner-scenario evidence: do NOT use its MSPT as the canonical-scenario baseline; fix the fixture first.

## Artifacts in this run

- `cpu-collapsed.txt` (53406 B)
- `wall-collapsed.txt` (1918 B)
- `alloc-collapsed.txt` (39701 B)
- `cpu-flamegraph.html` (17133 B)
- `server-stdout.log` (942234 B)
- `gc.log` (65082 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
