# absorb ROUND (chk455-1, run 36107481987, branch round-455b-chunk-1, head c551f7f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6925383 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6925383 (поллов=6); TPS_exp=2.26; normalized=+10.8%
- GC: young=104, Full=7, total=15.3s, avg=138ms, max=1380ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104197 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 9.76% (-5.90%) спад
  - nav_ai: 14.16% -> 3.23% (-10.93%) спад
  - inside_volatile: 12.01% -> 16.94% (+4.94%) РОСТ
  - fastutil: 8.54% -> 6.47% (-2.06%) спад
  - java_util: 7.01% -> 8.61% (+1.60%) РОСТ
  - paletted: 6.41% -> 5.29% (-1.12%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
