# absorb ROUND (a13-456, run 36112204974, branch round-456-anchor-13, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8696535 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8696535 (поллов=5); TPS_exp=2.63; normalized=-8.8%
- GC: young=126, Full=10, total=21.1s, avg=155ms, max=1993ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113966 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.78% (-2.39%) спад
  - fluid: 16.72% -> 16.41% (-0.31%) флэт
  - broadphase: 15.66% -> 14.94% (-0.72%) флэт
  - nav_ai: 14.16% -> 13.59% (-0.57%) флэт
  - inside_volatile: 12.01% -> 10.97% (-1.04%) спад
  - fastutil: 8.54% -> 8.96% (+0.42%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 7.10% (+0.69%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
