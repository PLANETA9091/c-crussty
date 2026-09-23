# absorb ROUND (round-423-cwgen-l1r, run 35819873738, branch round-423-c-wgen-l1r, head b582bc3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8631590 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 8631590 (поллов=6); TPS_exp=2.62; normalized=+1.3%
- GC: young=123, Full=10, total=22.1s, avg=166ms, max=2219ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113526 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.42% (-1.76%) спад
  - fluid: 16.72% -> 16.65% (-0.07%) флэт
  - broadphase: 15.66% -> 15.09% (-0.57%) флэт
  - nav_ai: 14.16% -> 13.81% (-0.35%) флэт
  - inside_volatile: 12.01% -> 10.86% (-1.15%) спад
  - fastutil: 8.54% -> 9.01% (+0.47%) флэт
  - java_util: 7.01% -> 6.46% (-0.55%) флэт
  - paletted: 6.41% -> 6.87% (+0.47%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
