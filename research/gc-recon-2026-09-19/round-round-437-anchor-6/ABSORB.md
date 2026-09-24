# absorb ROUND (round-437-anchor-6, run 35935360599, branch round-437-anchor-6, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6832169 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6832169 (поллов=5); TPS_exp=2.24; normalized=-6.2%
- GC: young=115, Full=10, total=24.2s, avg=194ms, max=2449ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116483 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.87% (-1.30%) спад
  - fluid: 16.72% -> 15.97% (-0.74%) флэт
  - broadphase: 15.66% -> 15.01% (-0.65%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 11.53% (-0.48%) флэт
  - fastutil: 8.54% -> 8.51% (-0.03%) флэт
  - java_util: 7.01% -> 6.44% (-0.58%) флэт
  - paletted: 6.41% -> 6.33% (-0.07%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
