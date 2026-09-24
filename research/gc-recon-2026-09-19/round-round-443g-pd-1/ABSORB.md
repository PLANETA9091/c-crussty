# absorb ROUND (round-443g-pd-1, run 36038596303, branch round-443g-pd-1, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6719155 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.70 @ 6719155 (поллов=5); TPS_exp=2.21; normalized=-23.2%
- GC: young=110, Full=10, total=26.2s, avg=218ms, max=2633ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116341 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.17% (-1.00%) спад
  - fluid: 16.72% -> 16.25% (-0.46%) флэт
  - broadphase: 15.66% -> 16.70% (+1.05%) РОСТ
  - nav_ai: 14.16% -> 13.63% (-0.53%) флэт
  - inside_volatile: 12.01% -> 11.07% (-0.94%) флэт
  - fastutil: 8.54% -> 8.55% (+0.02%) флэт
  - java_util: 7.01% -> 6.58% (-0.43%) флэт
  - paletted: 6.41% -> 7.12% (+0.71%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
