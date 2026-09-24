# absorb ROUND (round-440-ins4-2, run 35950591790, branch round-440-ins4-2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6888652 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6888652 (поллов=5); TPS_exp=2.25; normalized=+15.6%
- GC: young=102, Full=5, total=13.6s, avg=127ms, max=1526ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106509 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.34% (-0.38%) флэт
  - broadphase: 15.66% -> 10.14% (-5.52%) спад
  - nav_ai: 14.16% -> 3.83% (-10.33%) спад
  - inside_volatile: 12.01% -> 16.64% (+4.64%) РОСТ
  - fastutil: 8.54% -> 7.14% (-1.40%) спад
  - java_util: 7.01% -> 9.03% (+2.01%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
