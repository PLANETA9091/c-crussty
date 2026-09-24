# absorb ROUND (450b-ins4-4, run 36055406988, branch round-450-ins4-4, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7195647 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7195647 (поллов=5); TPS_exp=2.31; normalized=+12.3%
- GC: young=105, Full=9, total=20.2s, avg=177ms, max=2720ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106948 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.38% (-0.33%) флэт
  - broadphase: 15.66% -> 9.93% (-5.72%) спад
  - nav_ai: 14.16% -> 3.77% (-10.39%) спад
  - inside_volatile: 12.01% -> 16.69% (+4.68%) РОСТ
  - fastutil: 8.54% -> 6.63% (-1.91%) спад
  - java_util: 7.01% -> 8.38% (+1.36%) РОСТ
  - paletted: 6.41% -> 5.38% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
