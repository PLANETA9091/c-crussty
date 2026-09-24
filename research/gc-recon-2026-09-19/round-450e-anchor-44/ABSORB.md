# absorb ROUND (450e-anchor-44, run 36066506790, branch round-450-anchor-44, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8495501 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 8495501 (поллов=6); TPS_exp=2.59; normalized=-1.5%
- GC: young=115, Full=10, total=23.7s, avg=189ms, max=2250ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113524 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.68% (-2.49%) спад
  - fluid: 16.72% -> 16.24% (-0.48%) флэт
  - broadphase: 15.66% -> 14.65% (-1.01%) спад
  - nav_ai: 14.16% -> 13.63% (-0.53%) флэт
  - inside_volatile: 12.01% -> 10.68% (-1.33%) спад
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.38% (-0.63%) флэт
  - paletted: 6.41% -> 7.04% (+0.64%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
