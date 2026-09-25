# absorb ROUND (poi457-10, run 36144102915, branch round-456b-poi-10, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7143835 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7143835 (поллов=6); TPS_exp=2.30; normalized=+12.9%
- GC: young=106, Full=9, total=20.3s, avg=177ms, max=2855ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104664 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.63% (-0.09%) флэт
  - broadphase: 15.66% -> 9.80% (-5.86%) спад
  - nav_ai: 14.16% -> 3.11% (-11.06%) спад
  - inside_volatile: 12.01% -> 16.59% (+4.58%) РОСТ
  - fastutil: 8.54% -> 6.81% (-1.73%) спад
  - java_util: 7.01% -> 8.49% (+1.47%) РОСТ
  - paletted: 6.41% -> 5.14% (-1.26%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
