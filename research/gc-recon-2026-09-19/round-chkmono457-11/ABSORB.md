# absorb ROUND (chkmono457-11, run 36143848689, branch round-456c-chunkmono-11, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7040413 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7040413 (поллов=5); TPS_exp=2.28; normalized=-3.6%
- GC: young=96, Full=9, total=22.1s, avg=211ms, max=2789ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104163 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.20% (-0.51%) флэт
  - broadphase: 15.66% -> 8.82% (-6.84%) спад
  - nav_ai: 14.16% -> 2.75% (-11.42%) спад
  - inside_volatile: 12.01% -> 16.63% (+4.62%) РОСТ
  - fastutil: 8.54% -> 6.73% (-1.80%) спад
  - java_util: 7.01% -> 8.96% (+1.95%) РОСТ
  - paletted: 6.41% -> 5.58% (-0.83%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
