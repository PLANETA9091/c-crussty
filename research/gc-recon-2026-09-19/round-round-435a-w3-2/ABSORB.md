# absorb ROUND (round-435a-w3-2, run 35924886788, branch round-435a-w3-2, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7090998 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 7090998 (поллов=6); TPS_exp=2.29; normalized=-10.6%
- GC: young=503, Full=9, total=22.2s, avg=43ms, max=2686ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110913 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 13.93% (-2.79%) спад
  - broadphase: 15.66% -> 8.78% (-6.87%) спад
  - nav_ai: 14.16% -> 5.96% (-8.20%) спад
  - inside_volatile: 12.01% -> 14.25% (+2.25%) РОСТ
  - fastutil: 8.54% -> 7.06% (-1.48%) спад
  - java_util: 7.01% -> 7.93% (+0.92%) флэт
  - paletted: 6.41% -> 5.55% (-0.86%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
