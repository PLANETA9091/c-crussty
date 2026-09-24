# absorb ROUND (round-434-cce-l3r5r2, run 35919318435, branch round-434-cce-l3r5r2, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6826237 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6826237 (поллов=5); TPS_exp=2.24; normalized=+11.8%
- GC: young=108, Full=9, total=21.4s, avg=183ms, max=2692ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104524 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.22% (+0.50%) флэт
  - broadphase: 15.66% -> 9.40% (-6.26%) спад
  - nav_ai: 14.16% -> 3.14% (-11.02%) спад
  - inside_volatile: 12.01% -> 16.13% (+4.13%) РОСТ
  - fastutil: 8.54% -> 6.52% (-2.02%) спад
  - java_util: 7.01% -> 8.82% (+1.81%) РОСТ
  - paletted: 6.41% -> 5.82% (-0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
