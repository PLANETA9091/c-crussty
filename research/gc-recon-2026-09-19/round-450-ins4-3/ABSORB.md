# absorb ROUND (450-ins4-3, run 36050690301, branch round-450-ins4-3, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7265978 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7265978 (поллов=6); TPS_exp=2.33; normalized=+15.9%
- GC: young=105, Full=9, total=20.2s, avg=177ms, max=2470ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106885 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.38% (-0.33%) флэт
  - broadphase: 15.66% -> 10.18% (-5.48%) спад
  - nav_ai: 14.16% -> 3.67% (-10.49%) спад
  - inside_volatile: 12.01% -> 16.98% (+4.97%) РОСТ
  - fastutil: 8.54% -> 6.64% (-1.90%) спад
  - java_util: 7.01% -> 8.57% (+1.56%) РОСТ
  - paletted: 6.41% -> 5.41% (-0.99%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
