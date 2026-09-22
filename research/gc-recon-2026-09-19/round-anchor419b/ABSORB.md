# absorb ROUND (anchor419b, run 35770236750, branch round-419-anchorb, head fbb06e3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8541939 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8541939 (поллов=5); TPS_exp=2.60; normalized=-7.6%
- GC: young=121, Full=10, total=23.0s, avg=176ms, max=2226ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112737 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.84% (-2.33%) спад
  - fluid: 16.72% -> 16.60% (-0.12%) флэт
  - broadphase: 15.66% -> 14.71% (-0.95%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 10.63% (-1.38%) спад
  - fastutil: 8.54% -> 8.79% (+0.26%) флэт
  - java_util: 7.01% -> 6.72% (-0.29%) флэт
  - paletted: 6.41% -> 6.94% (+0.54%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
