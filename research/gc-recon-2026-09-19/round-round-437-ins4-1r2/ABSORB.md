# absorb ROUND (round-437-ins4-1r2, run 35938720950, branch round-437-ins4-1r2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7183354 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 7183354 (поллов=6); TPS_exp=2.31; normalized=+10.3%
- GC: young=104, Full=9, total=19.9s, avg=176ms, max=2687ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106018 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.63% (-0.08%) флэт
  - broadphase: 15.66% -> 10.33% (-5.33%) спад
  - nav_ai: 14.16% -> 3.71% (-10.46%) спад
  - inside_volatile: 12.01% -> 16.65% (+4.64%) РОСТ
  - fastutil: 8.54% -> 6.47% (-2.07%) спад
  - java_util: 7.01% -> 8.66% (+1.64%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
