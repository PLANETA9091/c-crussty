# ROUND-396-I — ITEMS-WAKEUP (lever_flag="items_wakeup")

## Mechanism
Event-driven wakeup scheduling for ItemEntity merge scans — architecture swap
for the TOP-1 bottleneck (items 31.17% java, bank v4 baseline). ARCHITECTURALLY
UNLIKE the refuted fluid_dirty-memo (per-section RESULT memoization of the
fluid scan); this is a per-entity EVENT SCHEDULE of the broadphase query.
A stationary item skips mergeWithNeighbours entirely; it scans only on:
E1 spawn (first sighting), E2 displacement > 0.25 blocks since last scan,
E3 one-shot wake, E4 wake set by a scan that ACTUALLY merged (self removed or
self stack count changed — exact check vs pre-scan), plus unconditional scan
after teleport (site 2). Invariant: two stationary items inside merge radius
have already merged; a skipped scan can only DELAY a merge to the next event,
never fabricate one.

Wiring: bridge blob ItemsWakeupOps (items_wakeup/build), STRICT retargets of
both mergeWithNeighbours invoke sites (tick 471, teleport 29) from pristine
bytes; rust items_wakeup.rs arms ONLY under CRUSSTY_LEVER_FLAG="items_wakeup";
vanilla delegate via cached MethodHandle; fail-safe: delegate failure = scan
skipped (never fabricates, never crashes).

## Parity / superiority deviations (DOC-DEV)
- Stationary scenes: BIT-EXACT vs vanilla (harness S3 dense-cluster fingerprint
  equality PASS) — the bench regime (100k settled items).
- Moving scenes: merge TIMING/distribution may differ (delayed scan). Hard
  invariants proven: units conserved exactly; zero fabricated merges; zero
  extra scans (S1/S2 PASS).
- despawn/pickup/baseTick untouched (vanilla body).

## Harness
ItemsWakeupHarness: S1 moving (units conserved, no fabricated merges, no extra
scans), S2 free+drain (work reduction + invariants), S3 dense settle (BIT-EXACT
+ work reduction). Harness bugs fixed during validation: event list re-applied
every tick (resurrection churn); E4 wake on zero-add "merges"; stamp never
refreshed. HARNESS RESULT: PASS.

## Research
/home/z/rounds/ROUND-396/RESEARCH-I.md — stationary fraction dominates the
150k bench scene; precedents: Paper activation-range scheduling, Lithium
entity-iteration blocks; ceiling 15-25% TPS-equivalent.
