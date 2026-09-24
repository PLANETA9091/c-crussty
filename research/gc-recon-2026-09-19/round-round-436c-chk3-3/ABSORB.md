# absorb ROUND (round-436c-chk3-3, run 35931089776, branch round-436c-chk3-3, head fb82607)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7011407 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7011407 (поллов=5); TPS_exp=2.28; normalized=+14.3%
- GC: young=109, Full=9, total=19.6s, avg=166ms, max=2498ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104410 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.36% (-0.36%) флэт
  - broadphase: 15.66% -> 9.77% (-5.88%) спад
  - nav_ai: 14.16% -> 3.28% (-10.88%) спад
  - inside_volatile: 12.01% -> 15.95% (+3.95%) РОСТ
  - fastutil: 8.54% -> 6.35% (-2.19%) спад
  - java_util: 7.01% -> 8.39% (+1.37%) РОСТ
  - paletted: 6.41% -> 5.38% (-1.03%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
