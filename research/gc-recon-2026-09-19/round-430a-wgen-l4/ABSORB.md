# absorb ROUND (430a-wgen-l4, run 35876725573, branch round-430a-wgen-l4, head a3991c2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6786277 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6786277 (поллов=6); TPS_exp=2.23; normalized=+12.2%
- GC: young=104, Full=9, total=19.8s, avg=175ms, max=2597ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102969 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.13% (+0.41%) флэт
  - broadphase: 15.66% -> 10.29% (-5.37%) спад
  - nav_ai: 14.16% -> 3.39% (-10.77%) спад
  - inside_volatile: 12.01% -> 12.72% (+0.72%) флэт
  - fastutil: 8.54% -> 6.72% (-1.82%) спад
  - java_util: 7.01% -> 7.23% (+0.22%) флэт
  - paletted: 6.41% -> 5.77% (-0.64%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
