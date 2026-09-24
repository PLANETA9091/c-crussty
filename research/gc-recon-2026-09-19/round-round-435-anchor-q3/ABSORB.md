# absorb ROUND (round-435-anchor-q3, run 35924395710, branch round-435-anchor-q3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6758998 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6758998 (поллов=5); TPS_exp=2.22; normalized=-1.0%
- GC: young=107, Full=8, total=21.3s, avg=185ms, max=2461ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116827 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.60% (-1.57%) спад
  - fluid: 16.72% -> 15.88% (-0.83%) флэт
  - broadphase: 15.66% -> 14.96% (-0.70%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 11.61% (-0.40%) флэт
  - fastutil: 8.54% -> 8.87% (+0.33%) флэт
  - java_util: 7.01% -> 6.98% (-0.03%) флэт
  - paletted: 6.41% -> 6.46% (+0.06%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
