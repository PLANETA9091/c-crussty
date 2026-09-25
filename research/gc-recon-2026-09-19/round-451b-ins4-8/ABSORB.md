# absorb ROUND (451b-ins4-8, run 36073728031, branch round-451-ins4-8, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6911132 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6911132 (поллов=6); TPS_exp=2.25; normalized=+13.1%
- GC: young=106, Full=9, total=20.2s, avg=176ms, max=2657ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106700 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.44% (-0.27%) флэт
  - broadphase: 15.66% -> 9.77% (-5.88%) спад
  - nav_ai: 14.16% -> 3.70% (-10.46%) спад
  - inside_volatile: 12.01% -> 17.35% (+5.34%) РОСТ
  - fastutil: 8.54% -> 6.54% (-2.00%) спад
  - java_util: 7.01% -> 8.89% (+1.88%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
