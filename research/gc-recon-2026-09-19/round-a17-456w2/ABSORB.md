# absorb ROUND (a17-456w2, run 36116678674, branch round-456-anchor-17, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7172483 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7172483 (поллов=5); TPS_exp=2.31; normalized=-9.1%
- GC: young=114, Full=9, total=21.5s, avg=175ms, max=2433ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116375 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.59% (-1.58%) спад
  - fluid: 16.72% -> 16.09% (-0.62%) флэт
  - broadphase: 15.66% -> 15.46% (-0.20%) флэт
  - nav_ai: 14.16% -> 13.78% (-0.38%) флэт
  - inside_volatile: 12.01% -> 11.50% (-0.51%) флэт
  - fastutil: 8.54% -> 8.69% (+0.15%) флэт
  - java_util: 7.01% -> 6.50% (-0.51%) флэт
  - paletted: 6.41% -> 6.25% (-0.16%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
