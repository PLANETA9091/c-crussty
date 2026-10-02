# COLLISION_CENSUS_DESIGN — S7-45 (TASK-84 phase-1, third and last open surface)

* Author: main-S7-45-collision-design, 2026-09-09. STATIC DESIGN ONLY — no server boot, no
  src/ changes (write-scope respected). Every probe claim below is backed by a javap line run
  against the **mojang-mapped runtime jar `/home/z/server/versions/1.21.10/purpur-1.21.10.jar`**
  this pass (raw excerpts: `/tmp/collision_census_findings.md`; jar identification:
  docs/TIER_R_ORDERING_AUDIT.md:4, docs/BETICK_STATIC_AUDIT_2026-09-08.md:14). Do NOT javap
  `/home/z/server/cache/mojang_1.21.10.jar` (vanilla bundler jar, no game classes) or the
  top-level paperclip jar (bootstrap only) — TASK-90 jar-confusion lesson, re-verified today.
* Family: TASK-84 dirty-rate census, phase-1 list = fluid-push (done), hopper-inventory
  (done, NO-GO 2.439%, bench/dirtyrate/results/DIRTYRATE_2026-09-09.md), **collision (this)**.
  Same dirty-rate law: arXiv 2411.10659v3 — a same-state guard pays >5.85x when
  mutation:query < 1% (dirty% < 1%). Campaign precedent: hopper 2.439% → NO-GO;
  area-map/lifecycle guards historically 1,945x–170,612x. Collision CPU share (TASK-81,
  analyzer CPU_SHARE table): **1.8%** of tick.

## §0 Pre-registered rule (wording per TASK-90 / DIRTYRATE_2026-09-09.md §4 — bind BEFORE running)

GO (design phase) requires **dirty% < 1% AND machinery ≥ 90%**; **> 10% closes the branch
forever**; 1%–10% = honest 10-100x class → measured live A/B (guard vs no-guard) required
before any build. Windows with < 100 query deltas print UNMEASURABLE (analyzer noise floor).
dirty% = mutation / (mutation + query), computed per surface by
`scripts/dirtyrate/analyze_dirtyrate.py` from cumulative TSV windows. Verdicts are data
(analyzer exit 0 always); no post-hoc gate edits.

## 1. Verified probe list (javap-verified on THIS version, 2026-09-09)

Query probes (the collision resolution work — per-tick per-moving-entity hot spots):

| # | probe | javap evidence (exact descriptor) | rationale |
|---|---|---|---|
| Q1 | `Entity#collide(Vec3)` | `private net.minecraft.world.phys.Vec3 collide(net.minecraft.world.phys.Vec3)` | THE swept-collision query; per move() resolution; private is irrelevant to ASM weaving |
| Q2 | `BlockCollisions#computeNext()` | `protected T computeNext()` on `class BlockCollisions<T> extends com.google.common.collect.AbstractIterator<T>` | one call per candidate block/shape visited along the swept AABB = the deep iteration cost (density-sensitive) |
| Q3 | `Shapes.collide(Axis, AABB, Iterable<VoxelShape>, double)` | `public static double collide(net.minecraft.core.Direction$Axis, net.minecraft.world.phys.AABB, java.lang.Iterable<net.minecraft.world.phys.shapes.VoxelShape>, double)` | per-axis resolution, 3x per collide(); Y-axis call is on every gravity entity's tick |
| Q4 | `Entity#move(MoverType, Vec3)` | `public void move(net.minecraft.world.entity.MoverType, net.minecraft.world.phys.Vec3)` | caller-context probe; verified LivingEntity does NOT override move() → full coverage from base weave |

Mutation probes (what would invalidate a collide() same-state memo):

| # | probe | javap evidence | rationale |
|---|---|---|---|
| M1 | `Level#setBlock(BlockPos, BlockState, int)` | `public boolean setBlock(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, int)` (Level re-declares LevelWriter's default) | block shape-change near path — ALL-block-update upper bound (analog of TASK-90 M=BlockEntity.setChanged) |
| M2 | `Entity#setPos(double,double,double)` | `public void setPos(double, double, double)` (+ `setPosRaw(double,double,double[,boolean])` Paper overload) | entity's own position/AABB change — the *other* invalidator; include M2 because position is part of collide()'s state input |

STALE-DOC TRAP: `net.minecraft.world.phys.shapes.CollisionSpliterator` and
`CollisionShapeSplitter` **do not exist in 1.21.10** (unzip -l: zero matches). In 1.21 the
swept-path iterator is `net.minecraft.world.level.BlockCollisions` (AbstractIterator-based).
Any impl plan citing the old names is pre-1.20.5 literature — use Q2/M1/M2 above.
Paper-added variants exist on `CollisionGetter` (`getPreMoveCollisions(Entity,AABB,Vec3)`,
`getBlockAndLiquidCollisions(Entity,AABB)`) — javap-verified; note them, don't probe them in
phase 1 (they funnel into the same BlockCollisions walk).

## 2. Same-state guard theory for Q1 — ESTIMATE, must be measured live

`collide(Vec3)` result ≈ pure function of (entity AABB @start, motion delta, surrounding
block/entity shapes along the swept path). Invalidate components:

- **Entity position/AABB: mutates EVERY TICK for every moving entity** (M2). This is the
  structural difference vs hopper-inventory (whose state sat still and only the query
  recurred). A memo keyed on state *including position* is dirtied by the very motion that
  triggers the query → mutation:query ratio structurally near 1:1 per moving entity.
- **Block shapes near path: rarely change** (M1) in a static-farm workload — the component
  a same-state guard could actually exploit; but vanilla already skips the zero-delta case
  (X1000_CANDIDATES_V3.md:88), removing the only free ride.
- Block-candidate *sets* (Q2) are position-derivative, so they inherit the same dirtying.

ESTIMATE (direction only, labeled per campaign law — the hopper ESTIMATE was wrong-direction
historically, and TASK-90's 2.439% was only knowable live): for a mob-dense world where
collide()-reachable entities tick constantly, expected dirty% is **HIGH — plausibly >10%**
(closes-forever band), because M2 (position writes) fire per-tick per-mover while Q1 fires
once per mover. A guarded version could only win if the memo excluded position-dependent
inputs, i.e. it would no longer be a same-state guard on collide(). The census exists to
measure this honestly and close the 14th branch if it lands >10%; only an unexpectedly
motion-sparse anchor (many entities idle/zero-delta, few movers) could pull it under 1%.
The hopper precedent (estimate optimistic, measurement NO-GO) is the reason §0 binds first.

## 3. Workload spec (run protocol for next session — reuse TASK-90 rig verbatim)

1. **Lane**: BENCH-MUTEX — `flock /home/z/BENCH.lock` + journal start/done + `pgrep` lane-busy
   abort (run_dirty_census.sh lines already do this; keep the SIGPIPE-safe logging, FIFO
   console, supervised foreground — background runs die with the agent session).
2. **World**: restore mob-dense anchor BEFORE mutation: `tar -C /home/z/server -xzf
   /home/z/server/world_mobdense_anchor.tar.gz` (single `world/`, 17 region files).
   Anchor again post-restore into the run dir (`world_anchor.tgz`) per runbook.
3. **Boot**: top-level paperclip jar with cwd=`/home/z/server`, prod default v2 flags +
   counter-agent: `-Xbootclasspath/a:$AGENT -javaagent:$AGENT` (L1: without bootclasspath/a
   StaticCounter CNFEs under Paper's remapped loader → silent zero counters). Env:
   `CRUSSTY_DIRTY_CENSUS=1`, `CRUSSTY_DIRTY_OUT=$OUT/collision_census.tsv`.
4. **Forceload + entity gate**: FIFO console: `forceload add -16 -16 15 15` (32×32 chunks
   centered on spawn; anchor is spawn-local — if the dense farm sits outside, re-center on
   the entity-rich coords visible in the rig: verify with `/forceload query` + `execute if
   entity @e[distance=..128] run say` count). Sanity gate (TASK-90 style): counter-agent
   Q4 (move) must show ≥ 20×entities/s within 10s of gate-open; mob count in area ≥ 100
   (mob-dense premise) else abort.
5. **Windows**: 80s idle baseline → 300s active window (mobs unfrozen, natural motion),
   TSV dumps every 30s (agent built-in) → 10 analysis windows, matching TASK-90 exactly.
6. **TSV schema** (unchanged format, new surface+counters — analyzer-compatible 4-col):
   `epoch_s \t collision \t <counter> \t value` with counters:
   `q_collide` (Q1), `q_block_iter` (Q2), `q_shapes_axis` (Q3), `q_move` (Q4),
   `m_setblock` (M1), `m_setpos` (M2).
   dirty% (primary) = (m_setblock + m_setpos) / ((m_setblock + m_setpos) + q_collide),
   computed per 30s window by analyze_dirtyrate.py; q_block_iter/q_shapes_axis reported
   as amplification ratios (per q_collide) to size the deep-iteration cost; q_move is the
   mechanism sanity line (≈ entities × 20/s like hopper's push-tick line).
7. **Verdict**: §0 above; record in bench/dirtyrate/results/ + RESULTS_LEDGER.

## 4. Risks (TASK-90 lessons carried forward)

- **R-hot-probe overhead**: collision is the hottest surface probed yet (every mover, every
  tick, plus 3 Shapes.axis calls per collide, plus N computeNext() calls). Counter increment
  must stay `AtomicLongArray.incrementAndGet` — no boxing/alloc, no map lookup; expect
  measurable agent tax on q_block_iter if woven — if BlockCollisions#computeNext tax skews
  TPS > 2%, drop Q2 to a 60s sampled window and mark it in the results doc.
- **R-visibility (L1)**: StaticCounter must ride `-Xbootclasspath/a:` — CNFE under Paper's
  remapped loader produced the all-zero run chain in TASK-90. Instrumentation sanity gate
  (step 4) is mandatory before any counted window; zero counters with live movers = rig bug,
  not a census result.
- **R-entity-ticking (L2)**: forceload'd chunks DO entity-tick (TASK-90 L2 smoke-proof) —
  no spawn-chunk ticket hack needed; still gate on nonzero q_move before trusting windows.
- **R-jar/wrong-CWD**: versioned jar is the RUNTIME mojang-mapped jar (javap target) but the
  wrong BOOT jar (raw craftbukkit Main → NCDFE); boot the top-level paperclip jar with
  cwd=$SERVER and absolute output paths only.
- **R-motion-floor**: if the anchor's movers are fewer than ~5 entities×20/s, q_collide may
  fall under the 100-query noise floor → UNMEASURABLE windows; mitigation: lengthen active
  window to 600s or widen forceload to 48×48 chunks (2-core box: stay ≤ 2G heap as in rig).

## 5. Impl checklist (next session, ≤15 min)

1. (3 min) Copy scripts/dirtyrate/agent/ → edit DirtyCensusAgent.java probe table: 6 entries
   above (class + method name only; descriptors in §1 / findings file). Keep env gate,
   AtomicLongArray, 30s TSV dump, shutdown flush, shaded build.
2. (2 min) Extend SelfTest fake set: fake Entity#collide + Level#setBlock bytes → verifier
   self-test (counter delta == N) — pattern from 5a8577b (catches nested-class binary-name
   class of bugs; TASK-90 proven).
3. (2 min) `bash scripts/dirtyrate/agent/build.sh` (rebuilds + self-tests).
4. (8 min) Run §3 protocol via a copy of run_dirty_census.sh (env/forceload/counters per §3),
   supervised foreground, inside BENCH-MUTEX. Then `python3 scripts/dirtyrate/
   analyze_dirtyrate.py $OUT/collision_census.tsv` → verdict vs §0. Stop: if §0 says
   >10%, record NO-GO/closed and stop — no guard build.
