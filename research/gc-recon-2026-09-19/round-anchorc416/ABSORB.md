# absorb ROUND (anchorc416, run 35745514060, branch round-416-anchorc, head 5869010)

- T1: lever=(n/a в run-env), pop=INVALID, NCDFE=0, col=PARALLEL, runner=6777842 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=1 -> **FAIL**
- GC: young=14, Full=6, total=3.4s, avg=172ms, max=816ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=65381 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 0.00% (-16.72%) спад
  - broadphase: 15.66% -> 55.07% (+39.41%) РОСТ
  - nav_ai: 14.16% -> 0.00% (-14.16%) спад
  - inside_volatile: 12.01% -> 0.00% (-12.01%) спад
  - fastutil: 8.54% -> 55.07% (+46.53%) РОСТ
  - java_util: 7.01% -> 42.78% (+35.76%) РОСТ
  - paletted: 6.41% -> 0.00% (-6.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
