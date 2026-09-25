# absorb ROUND (diet455-4, run 36104880648, branch round-455c-diet-4, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6987343 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6987343 (поллов=5); TPS_exp=2.27; normalized=+10.1%
- GC: young=104, Full=9, total=19.4s, avg=172ms, max=2536ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103781 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.31% (-0.41%) флэт
  - broadphase: 15.66% -> 9.65% (-6.00%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.47% (+4.46%) РОСТ
  - fastutil: 8.54% -> 6.31% (-2.23%) спад
  - java_util: 7.01% -> 8.38% (+1.37%) РОСТ
  - paletted: 6.41% -> 5.26% (-1.15%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
