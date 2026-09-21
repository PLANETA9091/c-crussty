# absorb ROUND (round403stc5, run 35608239985, branch round-403-stcomp1, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8537609 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.15 @ 8537609 (поллов=6); TPS_exp=2.60; normalized=+21.3%
- GC: young=120, Full=9, total=23.6s, avg=183ms, max=2989ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112988 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.56% (+0.85%) флэт
  - broadphase: 15.66% -> 13.29% (-2.37%) спад
  - nav_ai: 14.16% -> 8.96% (-5.20%) спад
  - inside_volatile: 12.01% -> 12.73% (+0.73%) флэт
  - fastutil: 8.54% -> 7.38% (-1.16%) спад
  - java_util: 7.01% -> 7.15% (+0.14%) флэт
  - paletted: 6.41% -> 7.46% (+1.05%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
