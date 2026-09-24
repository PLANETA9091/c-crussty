# absorb ROUND (450b-ins4-5, run 36055422458, branch round-450-ins4-5, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6999933 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6999933 (поллов=5); TPS_exp=2.27; normalized=-12.0%
- GC: young=103, Full=9, total=20.3s, avg=181ms, max=2667ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107456 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.14% (-0.57%) флэт
  - broadphase: 15.66% -> 9.58% (-6.08%) спад
  - nav_ai: 14.16% -> 3.63% (-10.54%) спад
  - inside_volatile: 12.01% -> 16.36% (+4.35%) РОСТ
  - fastutil: 8.54% -> 6.83% (-1.71%) спад
  - java_util: 7.01% -> 8.23% (+1.21%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
