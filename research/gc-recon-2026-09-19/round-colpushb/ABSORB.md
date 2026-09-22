# absorb ROUND (colpushb, run 35781338277, branch round-419-a-cpb, head dc4afd9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=3270, col=PARALLEL, runner=6593086 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.40 @ 6593086 (поллов=5); TPS_exp=2.19; normalized=-36.0%
- GC: young=927, Full=10, total=57.7s, avg=62ms, max=2238ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110795 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 11.33% (-5.39%) спад
  - broadphase: 15.66% -> 12.45% (-3.21%) спад
  - nav_ai: 14.16% -> 17.52% (+3.36%) РОСТ
  - inside_volatile: 12.01% -> 8.71% (-3.30%) спад
  - fastutil: 8.54% -> 5.70% (-2.84%) спад
  - java_util: 7.01% -> 5.89% (-1.12%) спад
  - paletted: 6.41% -> 3.99% (-2.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
