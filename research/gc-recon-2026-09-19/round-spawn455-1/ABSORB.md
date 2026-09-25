# absorb ROUND (spawn455-1, run 36106530105, branch round-455a-spawn-1, head c25782b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7099810 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 7099810 (поллов=6); TPS_exp=2.29; normalized=-6.3%
- GC: young=105, Full=9, total=20.7s, avg=182ms, max=2786ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104139 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.19% (-0.53%) флэт
  - broadphase: 15.66% -> 10.21% (-5.45%) спад
  - nav_ai: 14.16% -> 3.34% (-10.82%) спад
  - inside_volatile: 12.01% -> 16.82% (+4.82%) РОСТ
  - fastutil: 8.54% -> 6.63% (-1.91%) спад
  - java_util: 7.01% -> 8.71% (+1.70%) РОСТ
  - paletted: 6.41% -> 4.90% (-1.51%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
