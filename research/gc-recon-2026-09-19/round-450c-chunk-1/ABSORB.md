# absorb ROUND (450c-chunk-1, run 36053633635, branch round-450c-chunk-1, head 942d128)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7338218 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7338218 (поллов=5); TPS_exp=2.34; normalized=+15.2%
- GC: young=109, Full=9, total=23.0s, avg=195ms, max=3179ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105921 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.95% (+1.23%) РОСТ
  - broadphase: 15.66% -> 10.39% (-5.26%) спад
  - nav_ai: 14.16% -> 3.63% (-10.53%) спад
  - inside_volatile: 12.01% -> 16.71% (+4.71%) РОСТ
  - fastutil: 8.54% -> 6.13% (-2.40%) спад
  - java_util: 7.01% -> 8.51% (+1.49%) РОСТ
  - paletted: 6.41% -> 5.48% (-0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
