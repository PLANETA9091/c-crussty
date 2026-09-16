# RESEARCH — ENT-BP v2: the entity broadphase lever, measured on run#10 stacks

Agent-7625532f, 2026-09-17 (task161 per RESEARCH_BENCH3_BUCKETS_2026-09-16.md §9 ladder;
owner bar: «рисёрчи должны быть огромные» + «оптимизируй бесконечно, лимитов нет»).
Data source: run#10 CI artifact `world3-bench` re-downloaded and RE-MINED with
exact-frame aggregation (154MB cpu-collapsed.txt, 100,915 stacks / 224,660 samples,
counts verified against the banked total). This doc REFRAMES the pre-registered
RESEARCH_BENCH3_BUCKETS §2 candidate with measured numbers and banks one aggregation
lesson. Nothing here is a parity claim; CI numbers RANK hotspots (TASK-228 §pre-reg iii).

## 1. Measured anatomy of the entities/mobs bucket (12.6%, 28,256 samples)

| # | frame / family | exact presence | self-time | reading |
|---|---|---|---|---|
| 1 | `AbstractBoat.tick` (any stack) | **9.66%** (21,703) | — | THE driver: every boat ticks move→collide→entity queries |
| 2 | `EntityLookup.getEntities` (moonrise, any stack) | **4.76%** (10,690) | — | the whole entity-query broadphase lane |
| 3 | `ChunkEntitySlices$EntityCollectionBySection.getEntities` | 3.79% (8,525) | 0.68% | the innermost per-section loop |
| 4 | `AABB.intersects` leaf (total, any path) | — | **1.24%** (2,790) | 1.13% of it inside lane #3 |
| 5 | predicate family inside lane #3 (`EntitySelector` pushable lambdas) | 1.33% | — | runs ONLY on box-passing candidates — RETAINED under a correct mirror |
| 6 | `Scoreboard.getPlayersTeam` (under #5) | 0.99% (2,231) | — | string-keyed team lookup PER CANDIDATE — the pushable predicate's hidden cost |
| 7 | `CollisionUtil` leaves (block sweeps) | — | 1.84% | NOT broadphase — overlaps the PALETTE/chunk lane |
| 8 | `SynchedEntityData` leaves (all) | — | 1.73% | dominated by `isAlive→getHealth` 0.79% via `ActivationRange.checkIfActive` — separate lever |
| 9 | `EntityLookup.get` (single, exact frame) | 0.06% | — | see §7 lesson: substring aggregation said "5.04%" — WRONG |

Replaceable core (what a correct mirror displaces from lane #3): loop self 0.68%
+ `AABB.intersects` 1.13% + `Entity.getBoundingBox` 0.14% + fastutil section-map get
0.32% ≈ **2.27% of total CPU**. Retained: predicate ~1.3% (true candidates must still be
predicate-tested by the kernel per G1) + result resolution ~0.25%. Added: write-path
mirror maintenance est. 0.3–0.6% (7-scalar JNI upsert ≈15–25ns × moving entities/tick).
**Net expected 1.3–1.7% absolute ≈ 10–13% of the bucket** — pre-registered gate G2
(bucket self-time −5% min-of-2 legs) has ≈2× margin. Honest kill-criteria in §6.

## 2. Who owns the hot loop — REFRAMING of §2

The pre-registered §2 candidate assumed vanilla `EntitySectionStorage.getEntities`.
Measured truth on Paper 1.21.10 (Purpur, moonrise patches active): the hot loop is
`ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$
EntityCollectionBySection.getEntities` reached via `Level.getEntities →
EntityLookup.getEntities → ChunkEntitySlices.getEntities → EntityCollectionBySection
.getEntities → AABB.intersects`. Vanilla `EntitySectionStorage` is ABSENT from the hot
path. ENT-BP v2 therefore targets the MOONRISE structure; §2's gates (G1 superset,
G2 bucket −5%, G3 no-alloc fast path, G4 INJECTS-ONLY default-OFF) are UNCHANGED —
only the patch target moves. Consumers identified by caller chains: boat/minecart
`Entity.collide` hard-collision queries (`getHardCollidingEntities` 0.27%),
`LivingEntity.pushEntities` (chickens/sheep/fish/rabbits in farms),
`HopperBlockEntity.getEntityContainer/getItemsAtAndAbove` (item suction),
`NearestLivingEntitySensor.doTick` + `NearestAttackableTargetGoal.findTarget` (AI),
`ProjectileUtil.getEntityHitResult`, `HangingEntity.canCoexist` (via hasEntities).

## 3. Literature round (web, 2026-09-17)

- **CUDA GPU Gems 3 Ch.32 "Broad-Phase Collision Detection with CUDA"**
  (developer.nvidia.com): canonical bucket-and-prune broad-phase — compute AABBs,
  bin into a uniform grid, sort by bin, test pairs INSIDE bins only. Verdict table:
  uniform grid wins when objects are similar-size and chunk-stationary; sweep-and-prune
  wins for high-velocity/skewed-size populations. Minecraft entities are chunk-stationary
  (SectionPos-keyed) → **uniform grid + section bucketing is the literature match**; S&P
  would fight the kernel's own section partition.
- **gameprogrammingpatterns.com "Data Locality"**: hot per-object fields consumed by
  tight loops belong in contiguous SoA arrays; the object graph stays untouched. Maps to
  the banked MLA mapping (CLAIMS TASK-227): mirror the hot working set, keep the full
  graph in Java.
- **box2d.org "SIMD for Collision" + Josh Barczak (2015)**: 4-wide SIMD box-overlap
  batches; Box2D tests 4 AABBs per SIMD lane; raytracer BVH nodes tested 4-at-a-time.
  The mirror's query kernel: AVX2 `_mm256_cmpeq/_and` masked moves over 4 boxes/iter
  (6 double compares per box → 24-lane compare + movemask popcount), scalar fallback
  for tails and non-x86_64 (0 new deps law — `core::arch` only).
- **Shipilev quark 17 "Should You Use GetPrimitiaveArrayCritical()" + IBM JNI docs +
  JEP 423 (G1 region pinning, JDK 22)**: critical regions disable GC (pre-423) —
  bounded, copy-out-then-compute, NO other JNI calls inside. ENT-BP JNI contract:
  query fills a CALLER-owned id buffer (no allocation, no critical region needed —
  upsert passes 7 scalars by value; batch reconciliation uses a reusable direct
  ByteBuffer, one bounded critical copy per batch if ever needed).
- **Leaf (leafmc.one) async mob-AI target search**: moves the expensive nearby-entity
  scan off-thread; verification stays on tick. Precedent for the banked
  speculative-decoding mapping (propose off-thread, verify on tick). ENT-BP v2 keeps
  the mirror query ON-TICK (synchronous native) — off-thread query is the escalation
  lane only if G2 fails with the synchronous design; stale-reads analysis for the
  async variant is parked in §8.
- **Moonrise (Spottedleaf) entity architecture**: `EntityLookup` (id→entity CHM) +
  `ChunkEntitySlices` per-chunk 24-section slices + `EntityCollectionBySection`
  unmodifiable collections. DO-NOT-DUPLICATE list: id-indexed lookup, section-status
  transitions, entity ticking order — the mirror REPLACES ONLY the box-scan candidate
  enumeration; everything else stays kernel-owned (G1).

## 4. The staleness hazard — why query-side re-verify alone is WRONG

Pre-registered G1 says the bridge returns a SUPERSET of the kernel's box test results,
kernel predicate still decides. Superset breaks ONLY through false NEGATIVES: an entity
whose mirrored AABB is stale (moved into the query box without the mirror seeing it).
A Java-side re-verify (`box.intersects(entity.getBoundingBox()) && predicate.test(...)`)
filters false POSITIVES but can never resurrect a false negative — the entity never
reaches the verify stage. Consequences would be silent behavioral deltas: boats tunnel
through each other, hoppers miss items, AI loses targets. UNACCEPTABLE per G1.

Decision matrix for mirror freshness:

| design | freshness | verdict |
|---|---|---|
| per-tick reconciliation (batch-diff at tick entry, query mirror all tick) | half-tick stale | REJECTED — same-tick moves (boat pushed by boat earlier in the tick) are real query-visible state; misses = behavioral delta |
| inflated-box conservatism (mirror boxes + max-displacement epsilon) | stale-safe ONLY if epsilon ≥ max displacement | REJECTED — displacement is unbounded (teleports, knockback); honest epsilon degenerates to no pruning |
| **full write-path coverage** (every AABB mutation site hooks the mirror synchronously) | exact | **SELECTED** — sites enumerated by recon (§5); the module's whole-body patch machinery (area_map 5075→3320B, perlin 11030→10765B) already proves byte-level body replacement on this kernel |
| async off-thread mirror query | stale by design | PARKED (escalation lane) — needs propose-verify with tick-boundary fencing; only if synchronous G2 fails |

Write-path coverage = the recon deliverable: every site that mutates the box used by
`getBoundingBox` (setPos/setPosRaw family, setBoundingBox, dimension change, riding
transforms, `Entity.makeStuckInBlock`-style inflations) and every section-membership
transition (`EntityLookup.add/remove` + moonrise section updates). Sites are enumerated
from javap on the CI-booted jar (recon leg artifact `entity-recon`), NOT from memory —
the same discipline that caught the bare-world zip and the grep-BRE bugs.

## 5. ENT-BP v2 lever spec (pre-registered, updates §2 in place)

- **Patch target**: whole-body replacement of
  `ChunkEntitySlices$EntityCollectionBySection.getEntities` (and the wrapping
  `ChunkEntitySlices.getEntities` double-dispatch IF recon shows the per-section fan-out
  lives there) with a bridge body: `nativeQuery(mirrorHandle, cx, cz, ySectionRange,
  box, outIds) → k`, then resolve `ids → entities` through the kernel's own
  `EntityLookup` id map, re-verify `box.intersects(e.getBoundingBox()) &&
  predicate.test(e)` per candidate (G1: kernel decides), `list.add`. Bridge degrades to
  the ORIGINAL body on any mirror/handle failure (B.2.2 ladder precedent — worldgen
  never sees NaN, entity queries never see a lost entity).
- **Write path**: `EntityLookup.add/remove` + enumerated AABB mutation sites hook the
  mirror (`nativeUpsert(handle, id, x0..z1)` / `nativeRemove(handle, id)`), 7-scalar
  JNI by value, no critical region. New entities = mirror slot alloc (amortized,
  block-granular pooling per the banked PagedAttention mapping: fixed blocks, free-list
  reuse, zero alloc on query/move fast paths).
- **Mirror core (this tick, `src/entity_mirror.rs`)**: loose 16³-cell grid keyed by
  packed cell coord; entity boxes inserted into EVERY overlapped cell (loose grid —
  CUDA Ch.32 bucketing); per-query generation stamps dedup multi-cell candidates
  without clearing state; SoA slot store (x0/y0/z0/x1/y1/z1 f64 arrays) for
  data-locality; AVX2 4-wide batch box tests + scalar fallback; id-buffer query API
  with caller-owned output (no alloc). Property tests vs a linear-scan oracle:
  randomized worlds × moves × queries → candidate-set EQUALITY under full
  reconciliation (the oracle IS the kernel semantics), plus determinism and
  fast-path allocation counters.
- **Verification ladder** (per §9 protocol, one lever per round):
  1. CORE (this tick): oracle property tests green, fast-path alloc counters zero.
  2. RECON leg (next CI dispatch): javap dump of
     `EntityLookup`/`ChunkEntitySlices`/`Entity`/`Entity.collide` from the booted jar
     as workflow artifact `entity-recon` (diagnostic, INJECTS-ONLY — boots the server
     CI-side only).
  3. SHADOW-DIFF leg: mirror armed in DIAGNOSTIC mode — every query runs BOTH paths,
     diffs candidate sets, logs mismatch counts to the report (superset property
     banked with numbers over a full soak; zero mismatches required to advance).
  4. A/B legs ×2 (world-bench-3 dispatched twice, env
     `CRUSSTY_NATIVE_ENT_BP=1` vs `=0`): G2 = entities-bucket self-time −5%
     min-of-2; G3 = alloc profile shows zero new types on fast path.
  5. Promotion flow per TASK-148 precedent (kernel-policy key + env two-key contract).
- **Kill-criteria (honest)**: shadow-diff shows ANY mismatch not explained by a
  recon miss → lever parks with the diff banked; G2 two no-win legs → bucket parked
  per §9 escalation, ladder re-ranks (§7 levers next).

## 6. Sizing recap + risk register

Expected net −1.3..−1.7% absolute tick CPU (≈10–13% bucket-relative). Risks:
(i) write-path hook count larger than recon shows (riding/dimension inflation) —
mitigated by the degrade-to-original bridge + shadow-diff gate; (ii) JNI upsert cost
under boat-farm churn — 7-scalar crossing is ~10× cheaper than the 0.68% loop self it
enables removing, but the CI A/B is the judge; (iii) predicate retention means the
0.99% scoreboard cost stays — that is lever #2, NOT this lever's target (§7).

## 7. Follow-up levers in the SAME bucket (one-per-round discipline, pre-ranked)

1. **PUSH-MEMO**: pushable predicate's `Scoreboard.getPlayersTeam` string lookups
   (0.99% self) — memoize team-membership per entity per tick (1-tick memo keyed on
   entity id + scoreboard version), speculative-decoding mapping. Cheap, local,
   bridges the SAME query path if ENT-BP v2 lands.
2. **ENT-DATA / activation lane**: `ActivationRange.checkIfActive → isAlive →
   getHealth` reads the health DataItem 0.79% — a compressed mirror of the 2-3 hot
   DataItems (health/flags) behind the existing SynchedEntityData accessors is the
   MLA mapping; riskier (volatile/dirty semantics) — needs its own staleness matrix.
3. **CollisionUtil block sweeps** (1.84% self): overlaps the PALETTE/chunk-state lane
   (task162) — do NOT double-claim; coordinate after task162's design round.
4. **Boat-specific**: `AbstractBoat.tick` 9.66% presence is boats+collision+entity
   queries combined; if ENT-BP v2 + PUSH-MEMO land, re-profile before any boat
   movement-specific lever (no premature specializing).

## 8. Banked this tick (evidence pointers)

- Artifact re-download: GitHub Actions artifact 10458951869 (run 35122692415),
  `cpu-collapsed.txt` sha verified by sample count 224,660 == banked §108.
- Mining script: `scripts/bench3_research/entbp_mine.py` (exact-frame aggregation,
  caller-chain tables) — retained for future re-mining.
- Rust core: `src/entity_mirror.rs` (loose grid + SoA + AVX2 batch + oracle tests).
- Aggregation lesson (§7 of BENCH3_RUNS doc): substring frame aggregation
  overcounts (`EntityLookup.get` 5.04%→0.06% exact) — exact-frame matching is now the
  mining standard (entbp_mine.py splits on ';' and compares whole frames).

## 9. Core verification results (2026-09-17, sandbox, cargo test --release)

- Suite: **72 passed / 0 failed** single-threaded (incl. 7 new entity_mirror tests:
  property parity vs oracle ×64 trials ×3000 ops, scalar==SIMD candidate sets exact,
  multi-cell dedup, wildcard giant/NaN/inverted, churn + free-list reuse, gen-wrap,
  checksum stability).
- Zero-alloc (G3 core evidence, counting global allocator): 10,000 queries → **0
  allocs**; 3,000 same-span move upserts → **0 allocs**; cell-boundary-crossing moves
  are a DESIGNED slow path (map churn), measured **442 allocs / 3000 moves** (banked
  shape — HashMap entry + cell-Vec growth only on span change; ≈15% of moves crossed
  a boundary in this generator).
  Test-bench lesson: the first run of this test failed on its own bug (output buffer
  allocated after the counter reset) — the diag-batch harness isolated the query path
  as allocation-clean before the test was fixed; no mirror change was involved.
- CORE micro-bench (`bench_entbp_core`, NOT a CI claim): farm-world shape — 30,000
  live entities, 70% in 12 dense clusters, 10,678 grid cells; 20,000 queries of
  8×4×8 boxes averaging **315.8 true candidates**: mirror **34.2 µs/query** vs the
  full linear-scan oracle **150.5 µs/query** (4.4×). HONEST CAVEAT: the kernel is not
  a full scan — it scans only overlapping sections (its own broadphase); the sandbox
  cannot run the JVM, so mirror-vs-kernel stays the CI A/B's job (G2, min-of-2 legs).
  The 4.4× number certifies the core's internal shape, not a kernel-relative win.
