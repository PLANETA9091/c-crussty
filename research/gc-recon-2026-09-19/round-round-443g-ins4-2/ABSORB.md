# absorb ROUND (round-443g-ins4-2, run 36038642621, branch round-443g-ins4-2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6840939 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6840939 (поллов=5); TPS_exp=2.24; normalized=+20.6%
- GC: young=104, Full=9, total=18.9s, avg=168ms, max=2530ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107818 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 9.74% (-5.92%) спад
  - nav_ai: 14.16% -> 3.88% (-10.28%) спад
  - inside_volatile: 12.01% -> 17.45% (+5.44%) РОСТ
  - fastutil: 8.54% -> 6.57% (-1.97%) спад
  - java_util: 7.01% -> 8.72% (+1.71%) РОСТ
  - paletted: 6.41% -> 5.49% (-0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
