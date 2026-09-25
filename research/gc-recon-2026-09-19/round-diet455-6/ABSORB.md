# absorb ROUND (diet455-6, run 36108879067, branch round-455c-diet-6, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7046953 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7046953 (поллов=6); TPS_exp=2.28; normalized=-3.6%
- GC: young=103, Full=9, total=19.3s, avg=173ms, max=2498ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105277 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.56% (-0.16%) флэт
  - broadphase: 15.66% -> 10.11% (-5.55%) спад
  - nav_ai: 14.16% -> 3.17% (-10.99%) спад
  - inside_volatile: 12.01% -> 16.86% (+4.85%) РОСТ
  - fastutil: 8.54% -> 6.79% (-1.75%) спад
  - java_util: 7.01% -> 8.86% (+1.85%) РОСТ
  - paletted: 6.41% -> 5.12% (-1.29%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
