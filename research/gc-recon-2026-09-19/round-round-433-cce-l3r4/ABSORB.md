# absorb ROUND (round-433-cce-l3r4, run 35907700571, branch round-433-cce-l3r4, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7067374 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7067374 (поллов=6); TPS_exp=2.29; normalized=+7.1%
- GC: young=102, Full=9, total=19.5s, avg=176ms, max=2478ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103809 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.13% (-0.59%) флэт
  - broadphase: 15.66% -> 10.05% (-5.61%) спад
  - nav_ai: 14.16% -> 3.11% (-11.05%) спад
  - inside_volatile: 12.01% -> 15.60% (+3.60%) РОСТ
  - fastutil: 8.54% -> 6.37% (-2.17%) спад
  - java_util: 7.01% -> 8.40% (+1.38%) РОСТ
  - paletted: 6.41% -> 5.17% (-1.23%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
