# absorb ROUND (450-ins4-1, run 36050665317, branch round-450-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7108000 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7108000 (поллов=5); TPS_exp=2.30; normalized=-4.2%
- GC: young=107, Full=9, total=19.8s, avg=171ms, max=2572ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107285 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.70% (-0.02%) флэт
  - broadphase: 15.66% -> 10.20% (-5.46%) спад
  - nav_ai: 14.16% -> 3.75% (-10.41%) спад
  - inside_volatile: 12.01% -> 16.72% (+4.71%) РОСТ
  - fastutil: 8.54% -> 6.77% (-1.77%) спад
  - java_util: 7.01% -> 8.38% (+1.37%) РОСТ
  - paletted: 6.41% -> 5.55% (-0.85%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
