# absorb ROUND (450b-ins4-6, run 36055437639, branch round-450-ins4-6, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6863514 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6863514 (поллов=5); TPS_exp=2.24; normalized=+20.3%
- GC: young=102, Full=9, total=18.5s, avg=167ms, max=2364ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105979 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.22% (-0.49%) флэт
  - broadphase: 15.66% -> 9.86% (-5.80%) спад
  - nav_ai: 14.16% -> 3.86% (-10.31%) спад
  - inside_volatile: 12.01% -> 17.05% (+5.04%) РОСТ
  - fastutil: 8.54% -> 6.81% (-1.73%) спад
  - java_util: 7.01% -> 8.52% (+1.51%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.07%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
