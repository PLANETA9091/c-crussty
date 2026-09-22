# absorb ROUND (round410bl2, run 35678632488, branch round-410-b-l2, head ea3ed47)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6982576 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 6982576 (поллов=6); TPS_exp=2.27; normalized=-9.7%
- GC: young=104, Full=9, total=21.2s, avg=187ms, max=2483ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115976 сэмплов (базлайн 115655)
  - items: 31.17% -> 31.53% (+0.36%) флэт
  - fluid: 16.72% -> 16.61% (-0.10%) флэт
  - broadphase: 15.66% -> 16.97% (+1.31%) РОСТ
  - nav_ai: 14.16% -> 13.63% (-0.53%) флэт
  - inside_volatile: 12.01% -> 10.55% (-1.45%) спад
  - fastutil: 8.54% -> 8.39% (-0.14%) флэт
  - java_util: 7.01% -> 7.39% (+0.38%) флэт
  - paletted: 6.41% -> 6.38% (-0.02%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
