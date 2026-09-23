# absorb ROUND (round-432-anchor-d, run 35891330601, branch round-432-anchor-d, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8529530 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8529530 (поллов=6); TPS_exp=2.60; normalized=-3.7%
- GC: young=112, Full=10, total=27.7s, avg=227ms, max=3077ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117421 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.66% (-1.51%) спад
  - fluid: 16.72% -> 17.81% (+1.10%) РОСТ
  - broadphase: 15.66% -> 14.45% (-1.21%) спад
  - nav_ai: 14.16% -> 13.76% (-0.40%) флэт
  - inside_volatile: 12.01% -> 11.42% (-0.59%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 7.60% (+0.59%) флэт
  - paletted: 6.41% -> 7.51% (+1.10%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
