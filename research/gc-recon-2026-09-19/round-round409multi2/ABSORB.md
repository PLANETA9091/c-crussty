# absorb ROUND (round409multi2, run 35670218104, branch round-408-multi2, head f7d04d2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6507416 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 6507416 (поллов=6); TPS_exp=2.17; normalized=+33.7%
- GC: young=109, Full=9, total=19.1s, avg=162ms, max=2262ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110274 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.21% (-0.50%) флэт
  - broadphase: 15.66% -> 14.45% (-1.21%) спад
  - nav_ai: 14.16% -> 9.67% (-4.50%) спад
  - inside_volatile: 12.01% -> 12.77% (+0.76%) флэт
  - fastutil: 8.54% -> 7.48% (-1.06%) спад
  - java_util: 7.01% -> 7.71% (+0.69%) флэт
  - paletted: 6.41% -> 5.60% (-0.81%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
