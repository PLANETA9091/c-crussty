# absorb ROUND (round410ak5-diag, run 35679132437, branch round-410-a-k5, head 5d8af33)

- T1: lever=(n/a в run-env), pop=INVALID, NCDFE=0, col=PARALLEL, runner=6588847 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=1 -> **FAIL**
- GC: young=49, Full=7, total=6.2s, avg=111ms, max=1447ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=65418 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 0.00% (-16.72%) спад
  - broadphase: 15.66% -> 55.04% (+39.38%) РОСТ
  - nav_ai: 14.16% -> 0.00% (-14.16%) спад
  - inside_volatile: 12.01% -> 0.00% (-12.01%) спад
  - fastutil: 8.54% -> 45.73% (+37.19%) РОСТ
  - java_util: 7.01% -> 0.06% (-6.95%) спад
  - paletted: 6.41% -> 0.00% (-6.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
