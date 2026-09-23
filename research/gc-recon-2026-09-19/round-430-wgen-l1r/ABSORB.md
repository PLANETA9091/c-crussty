# absorb ROUND (430-wgen-l1r, run 35873212174, branch round-430-wgen-l1r, head c70023c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6402349 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6402349 (поллов=6); TPS_exp=2.15; normalized=+21.1%
- GC: young=108, Full=9, total=17.7s, avg=151ms, max=2357ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105424 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.10% (+0.38%) флэт
  - broadphase: 15.66% -> 10.14% (-5.52%) спад
  - nav_ai: 14.16% -> 3.40% (-10.76%) спад
  - inside_volatile: 12.01% -> 13.24% (+1.24%) РОСТ
  - fastutil: 8.54% -> 7.00% (-1.54%) спад
  - java_util: 7.01% -> 7.55% (+0.54%) флэт
  - paletted: 6.41% -> 5.67% (-0.74%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
