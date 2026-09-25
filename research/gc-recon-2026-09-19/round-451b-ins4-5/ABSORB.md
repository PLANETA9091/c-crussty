# absorb ROUND (451b-ins4-5, run 36073698062, branch round-451-ins4-5, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6875470 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6875470 (поллов=6); TPS_exp=2.25; normalized=+17.9%
- GC: young=104, Full=9, total=18.5s, avg=163ms, max=2528ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106702 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.47% (-0.24%) флэт
  - broadphase: 15.66% -> 10.19% (-5.47%) спад
  - nav_ai: 14.16% -> 3.91% (-10.25%) спад
  - inside_volatile: 12.01% -> 16.85% (+4.84%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.08%) спад
  - java_util: 7.01% -> 8.60% (+1.59%) РОСТ
  - paletted: 6.41% -> 5.31% (-1.10%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
