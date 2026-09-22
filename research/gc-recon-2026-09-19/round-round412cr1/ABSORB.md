# absorb ROUND (round412cr1, run 35706186210, branch round-412-c-r1, head f8faea2)

- T1: lever=(n/a в run-env), pop=INVALID, NCDFE=7930, col=PARALLEL, runner=7096323 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=1 -> **FAIL**
- GC: young=16, Full=6, total=3.7s, avg=169ms, max=888ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=65374 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 0.00% (-16.72%) спад
  - broadphase: 15.66% -> 53.83% (+38.18%) РОСТ
  - nav_ai: 14.16% -> 0.00% (-14.16%) спад
  - inside_volatile: 12.01% -> 0.00% (-12.01%) спад
  - fastutil: 8.54% -> 6.97% (-1.57%) спад
  - java_util: 7.01% -> 0.04% (-6.97%) спад
  - paletted: 6.41% -> 0.00% (-6.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
