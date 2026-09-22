# absorb ROUND (cvs3, run 35738347874, branch round-415-cvs3, head 1aec4f8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6710453 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6710453 (поллов=5); TPS_exp=2.21; normalized=+22.1%
- GC: young=1141, Full=9, total=25.4s, avg=22ms, max=2255ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107919 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.05% (-0.67%) флэт
  - broadphase: 15.66% -> 14.09% (-1.56%) спад
  - nav_ai: 14.16% -> 9.31% (-4.85%) спад
  - inside_volatile: 12.01% -> 12.57% (+0.56%) флэт
  - fastutil: 8.54% -> 7.79% (-0.74%) флэт
  - java_util: 7.01% -> 7.98% (+0.97%) флэт
  - paletted: 6.41% -> 5.44% (-0.96%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
