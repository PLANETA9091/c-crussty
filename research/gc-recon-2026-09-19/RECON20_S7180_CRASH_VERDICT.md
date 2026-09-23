# RECON-20 — s7180 verdict: STEAL v2 CRASH-REFUTED, lane CLOSED; pivot to #14 TRAVEL-ALLOC-DIET

TASK-341 (tick 02:4x +08). Run 35460026013 @ 0c65771, region_steal=1 + bu_defer=1, rest = bank v3.

## Forensics (job log 105942103924 + artifact world3-bench)

- runner_cpu_index = **6997821** — slow class, −17.6% vs anchor 8493973 → PG-P3 pairing gate would
  invalidate the leg for pairing REGARDLESS of outcome (2nd slow-class landing in a row: s7179
  6.72M, s7180 6.99M; fast class = s7178 8.49M / s7169 8.57M). **The runner pool is bimodal.**
- Boot → Done OK; injection 150000/150000 VALID (64.3s); **T+44s after INJECT DONE (18:12:51):
  server-thread crash** → graceful shutdown, JVM exit 18:13:19; soak never started (1 TPS poll
  19.2 pre-soak; churn gate polls=0 → BENCH-4 FIXTURE-INVALID exit 1).
- **Crash signature (verdict-class, NOT infra-flake):**
  `NPE ObjectOpenHashSet$SetIterator.next("this.wrapped" is null)`
  at `BlockUpdateOps.vanilla(BlockUpdateOps.java:117)`
  ← `RegionTickOps.drainDeferredBlockUpdates(RegionTickOps.java:168)`
  ← `RegionTickOps.stealTick(RegionTickOps.java:352)` ← `RegionTickOps.forEach(285)` on the
  **Server thread** — i.e. the crash is INSIDE the BU-DEFER phase-4 replay itself, the very code
  introduced to fix the s7176 race. Same fastutil race family as s7176, second strike.
- Root-cause reading: the replay path iterates `navigatingMobs` on main without the vanilla
  `isUpdatingNavigations` gate context — structural mutation of the set during the replay window
  (workers still draining deferred per-region work / recomputePath side effects) corrupts the
  fastutil iterator state. S7-169 candidate fix (NOT executed — see lane close): set the
  isUpdatingNavigations gate around BlockUpdateOps.vanilla replay body.

## Verdict (pre-registered PG-P gates)

- **STEAL v2 = CRASH-REFUTED.** PG-P2 crash-free FAIL (NPE markers > 0, mid-soak crash).
  PG-P1 delivery PASS (region_steal=1, bu_defer=1, composed marker, NCDFE=0, pop VALID).
  PG-P3 N/A (runner outside class). PG-P4 young 30 (≤174), Full 0 — GC sane.
- **Lane CLOSED (2 strikes + ceiling):** (1) s7177 REFUTED-by-TPS (normalized +9.1% < +10% bar);
  (2) s7180 CRASH-REFUTED; (3) RECON-17 ceiling for park-fill ≈ +7–9% < +10% bar. The
  park-kill→TPS lane is exhausted as a ≥+10% lever. Rollback: yml defaults already
  region_steal=0 / bu_defer=0 (bridge infra stays).

## DUAL BAR protocol (owner directive 2026-09-20)

From now on a lever banks ONLY if BOTH axes rise simultaneously on a same-class paired leg:
(1) speed-normalized delta ≥ +10% AND (2) absolute median5 TPS delta ≥ +10% vs the same-class
anchor (min-of-2 preserved). Single-axis = no GREEN.

## Pivot: lever #14 TRAVEL-ALLOC-DIET (new TOP-1: GC/JVM 36.23% CPU = alloc churn)

- v1 (dispatched as s7182): bank v3 + `zero_alloc=1` (S7-164 scalar ZeroAllocOps:
  collidedWithFluid / collidedWithShapeMovingFrom / updateFluidHeightAndDoFluidPushing — the
  fluid/collide AABB+Vec3 churn inside travel-adjacent paths) + `skip_store_bb=1` (S7-166
  value-equal store-skip for Entity.setBoundingBox — kills makeBoundingBox→setBoundingBox
  redundant stores). region_steal/bu_defer=0. Dispatch inputs carry
  `cpu_band_min=7800000 / cpu_band_max=9200000` (S7-96d pairing law, first-step fast-fail) —
  legs land only in the fast anchor class; unpairable runners die in ~1 min, not 11.
- Anchor unchanged: s7178 median5 1.40 @ 8493973 (fast class).
- If <+10% on both axes: #14 v2 = javap recon of makeBoundingBox / Vec3.add / AABB.inflate
  call-sites in the travel leaf (recon14_travel.py sites) → TravelDietOps body-redirects
  (S7-law: javap contract first).
