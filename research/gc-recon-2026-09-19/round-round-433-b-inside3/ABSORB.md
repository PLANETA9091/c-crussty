# absorb ROUND (round-433-b-inside3, run 35902792520, branch round-433-b-inside3, head 5a789c4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=400104, col=PARALLEL, runner=6627811 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=14.60 @ 6627811 (поллов=5); TPS_exp=2.19; normalized=+565.3%
- GC: young=158, Full=9, total=10.5s, avg=63ms, max=1394ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=57416 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 2.94% (-13.78%) спад
  - broadphase: 15.66% -> 9.89% (-5.76%) спад
  - nav_ai: 14.16% -> 7.44% (-6.72%) спад
  - inside_volatile: 12.01% -> 3.81% (-8.20%) спад
  - fastutil: 8.54% -> 5.83% (-2.71%) спад
  - java_util: 7.01% -> 8.34% (+1.32%) РОСТ
  - paletted: 6.41% -> 6.02% (-0.38%) флэт
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
