# absorb ROUND (436c-chk3-1, run 35930829448, branch round-436c-chk3-1, head fb82607)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9922693 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 9922693 (поллов=5); TPS_exp=2.89; normalized=+7.3%
- GC: young=120, Full=9, total=22.4s, avg=173ms, max=2943ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104830 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.30% (+0.59%) флэт
  - broadphase: 15.66% -> 9.29% (-6.37%) спад
  - nav_ai: 14.16% -> 3.56% (-10.60%) спад
  - inside_volatile: 12.01% -> 16.29% (+4.29%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.91%) спад
  - java_util: 7.01% -> 9.14% (+2.13%) РОСТ
  - paletted: 6.41% -> 6.62% (+0.21%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
