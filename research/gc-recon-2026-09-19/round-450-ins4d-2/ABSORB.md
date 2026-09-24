# absorb ROUND (450-ins4d-2, run 36050716039, branch round-450-ins4d-2, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6819073 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6819073 (поллов=5); TPS_exp=2.23; normalized=+20.8%
- GC: young=105, Full=9, total=21.5s, avg=188ms, max=2656ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105887 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.73% (+0.02%) флэт
  - broadphase: 15.66% -> 9.19% (-6.47%) спад
  - nav_ai: 14.16% -> 3.80% (-10.36%) спад
  - inside_volatile: 12.01% -> 16.10% (+4.10%) РОСТ
  - fastutil: 8.54% -> 6.51% (-2.03%) спад
  - java_util: 7.01% -> 8.11% (+1.09%) РОСТ
  - paletted: 6.41% -> 5.90% (-0.50%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
