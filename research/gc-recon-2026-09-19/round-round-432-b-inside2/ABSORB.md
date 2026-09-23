# absorb ROUND (round-432-b-inside2, run 35894909390, branch round-432-b-inside2, head fd0cf0b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=473412, col=PARALLEL, runner=8700591 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=16.70 @ 8700591 (поллов=5); TPS_exp=2.63; normalized=+534.7%
- GC: young=193, Full=10, total=10.8s, avg=53ms, max=864ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=57176 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 3.47% (-13.24%) спад
  - broadphase: 15.66% -> 8.58% (-7.08%) спад
  - nav_ai: 14.16% -> 7.53% (-6.63%) спад
  - inside_volatile: 12.01% -> 3.77% (-8.24%) спад
  - fastutil: 8.54% -> 5.67% (-2.87%) спад
  - java_util: 7.01% -> 7.45% (+0.44%) флэт
  - paletted: 6.41% -> 6.52% (+0.11%) флэт
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
