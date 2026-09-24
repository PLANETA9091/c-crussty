# absorb ROUND (round-449c-mega4-2, run 36045109689, branch round-449c-mega4, head 5188f18)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6796602 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6796602 (поллов=5); TPS_exp=2.23; normalized=+7.6%
- GC: young=102, Full=9, total=20.2s, avg=182ms, max=3111ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105459 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.32% (-0.40%) флэт
  - broadphase: 15.66% -> 9.90% (-5.76%) спад
  - nav_ai: 14.16% -> 3.00% (-11.16%) спад
  - inside_volatile: 12.01% -> 16.46% (+4.46%) РОСТ
  - fastutil: 8.54% -> 6.18% (-2.35%) спад
  - java_util: 7.01% -> 8.73% (+1.72%) РОСТ
  - paletted: 6.41% -> 5.29% (-1.12%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
