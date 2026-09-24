# absorb ROUND (round-434b-ins-l3, run 35917452819, branch round-434b-ins-l3, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6877947 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6877947 (поллов=5); TPS_exp=2.25; normalized=+6.8%
- GC: young=108, Full=8, total=19.3s, avg=167ms, max=2617ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104932 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.51% (-0.20%) флэт
  - broadphase: 15.66% -> 9.71% (-5.94%) спад
  - nav_ai: 14.16% -> 3.07% (-11.09%) спад
  - inside_volatile: 12.01% -> 16.35% (+4.34%) РОСТ
  - fastutil: 8.54% -> 6.49% (-2.05%) спад
  - java_util: 7.01% -> 8.55% (+1.54%) РОСТ
  - paletted: 6.41% -> 5.28% (-1.12%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
