# absorb ROUND (chkmono457-16, run 36146784734, branch round-456c-chunkmono-16, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6951662 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6951662 (поллов=6); TPS_exp=2.26; normalized=+12.7%
- GC: young=102, Full=9, total=20.5s, avg=185ms, max=3010ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104735 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.14% (-0.57%) флэт
  - broadphase: 15.66% -> 9.87% (-5.79%) спад
  - nav_ai: 14.16% -> 3.28% (-10.89%) спад
  - inside_volatile: 12.01% -> 16.45% (+4.45%) РОСТ
  - fastutil: 8.54% -> 6.85% (-1.69%) спад
  - java_util: 7.01% -> 8.59% (+1.58%) РОСТ
  - paletted: 6.41% -> 5.23% (-1.17%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
