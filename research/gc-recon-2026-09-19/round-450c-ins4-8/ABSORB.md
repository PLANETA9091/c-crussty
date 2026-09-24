# absorb ROUND (450c-ins4-8, run 36058509074, branch round-450-ins4-8, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7043209 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 7043209 (поллов=5); TPS_exp=2.28; normalized=+22.7%
- GC: young=109, Full=9, total=23.1s, avg=195ms, max=3047ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107806 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.90% (+1.19%) РОСТ
  - broadphase: 15.66% -> 9.88% (-5.78%) спад
  - nav_ai: 14.16% -> 3.82% (-10.34%) спад
  - inside_volatile: 12.01% -> 18.31% (+6.31%) РОСТ
  - fastutil: 8.54% -> 6.32% (-2.22%) спад
  - java_util: 7.01% -> 8.76% (+1.75%) РОСТ
  - paletted: 6.41% -> 5.32% (-1.09%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
