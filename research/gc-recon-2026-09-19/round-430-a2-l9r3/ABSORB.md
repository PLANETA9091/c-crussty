# absorb ROUND (430-a2-l9r3, run 35873124814, branch round-430-a2-l9r3, head 18151a9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7332845 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7332845 (поллов=5); TPS_exp=2.34; normalized=+15.2%
- GC: young=107, Full=9, total=19.3s, avg=166ms, max=2348ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104571 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.63% (-0.08%) флэт
  - broadphase: 15.66% -> 10.16% (-5.50%) спад
  - nav_ai: 14.16% -> 3.25% (-10.91%) спад
  - inside_volatile: 12.01% -> 13.05% (+1.04%) РОСТ
  - fastutil: 8.54% -> 6.60% (-1.93%) спад
  - java_util: 7.01% -> 7.57% (+0.56%) флэт
  - paletted: 6.41% -> 5.69% (-0.71%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
