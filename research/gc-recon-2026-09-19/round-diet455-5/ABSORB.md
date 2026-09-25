# absorb ROUND (diet455-5, run 36108868369, branch round-455c-diet-5, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6786837 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6786837 (поллов=5); TPS_exp=2.23; normalized=-1.3%
- GC: young=102, Full=9, total=19.5s, avg=176ms, max=2530ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104363 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.35% (-0.37%) флэт
  - broadphase: 15.66% -> 10.25% (-5.41%) спад
  - nav_ai: 14.16% -> 3.06% (-11.10%) спад
  - inside_volatile: 12.01% -> 16.81% (+4.80%) РОСТ
  - fastutil: 8.54% -> 6.54% (-1.99%) спад
  - java_util: 7.01% -> 8.65% (+1.63%) РОСТ
  - paletted: 6.41% -> 5.03% (-1.38%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
