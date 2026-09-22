# absorb ROUND (round410multi3, run 35673637169, branch round-410-multi3, head f7d04d2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6464483 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.00 @ 6464483 (поллов=6); TPS_exp=2.16; normalized=+38.9%
- GC: young=109, Full=9, total=18.5s, avg=156ms, max=2185ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110026 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.40% (-0.31%) флэт
  - broadphase: 15.66% -> 14.31% (-1.35%) спад
  - nav_ai: 14.16% -> 7.54% (-6.62%) спад
  - inside_volatile: 12.01% -> 12.87% (+0.86%) флэт
  - fastutil: 8.54% -> 6.94% (-1.60%) спад
  - java_util: 7.01% -> 7.70% (+0.69%) флэт
  - paletted: 6.41% -> 5.86% (-0.54%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
