# absorb ROUND (round-443g-ins4d-1, run 36038463683, branch round-443g-ins4d-1, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7314819 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.00 @ 7314819 (поллов=5); TPS_exp=2.34; normalized=+28.2%
- GC: young=109, Full=9, total=21.9s, avg=186ms, max=3007ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107209 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.23% (+1.52%) РОСТ
  - broadphase: 15.66% -> 9.58% (-6.07%) спад
  - nav_ai: 14.16% -> 4.19% (-9.97%) спад
  - inside_volatile: 12.01% -> 17.46% (+5.45%) РОСТ
  - fastutil: 8.54% -> 6.40% (-2.14%) спад
  - java_util: 7.01% -> 8.51% (+1.50%) РОСТ
  - paletted: 6.41% -> 5.58% (-0.82%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
