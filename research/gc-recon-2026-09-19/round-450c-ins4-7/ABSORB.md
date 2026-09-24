# absorb ROUND (450c-ins4-7, run 36058494698, branch round-450-ins4-7, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9082256 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 9082256 (поллов=5); TPS_exp=2.71; normalized=-7.8%
- GC: young=117, Full=9, total=23.8s, avg=189ms, max=3441ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107825 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.39% (+0.68%) флэт
  - broadphase: 15.66% -> 9.45% (-6.21%) спад
  - nav_ai: 14.16% -> 4.02% (-10.14%) спад
  - inside_volatile: 12.01% -> 18.37% (+6.37%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 9.29% (+2.28%) РОСТ
  - paletted: 6.41% -> 6.04% (-0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
