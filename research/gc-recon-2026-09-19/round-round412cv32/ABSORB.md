# absorb ROUND (round412cv32, run 35712204518, branch round-412-cv3-2, head e408aac)

- T1: lever=(n/a в run-env), pop=INVALID, NCDFE=6978, col=PARALLEL, runner=7131969 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=3, TPS-поллов=1 -> **FAIL**
- GC: young=14, Full=6, total=4.5s, avg=226ms, max=1047ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=218 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 0.00% (-16.72%) спад
  - broadphase: 15.66% -> 0.00% (-15.66%) спад
  - nav_ai: 14.16% -> 0.00% (-14.16%) спад
  - inside_volatile: 12.01% -> 0.00% (-12.01%) спад
  - fastutil: 8.54% -> 0.00% (-8.54%) спад
  - java_util: 7.01% -> 9.63% (+2.62%) РОСТ
  - paletted: 6.41% -> 0.00% (-6.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
