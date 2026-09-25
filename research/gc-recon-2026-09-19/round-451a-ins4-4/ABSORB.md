# absorb ROUND (451a-ins4-4, run 36069583114, branch round-451-ins4-4, head 0707800)

- T1: lever=(n/a в run-env), pop=INVALID, NCDFE=0, col=PARALLEL, runner=6873790 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=1 -> **FAIL**
- GC: young=15, Full=6, total=3.2s, avg=153ms, max=969ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=65325 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 0.00% (-16.72%) спад
  - broadphase: 15.66% -> 53.65% (+37.99%) РОСТ
  - nav_ai: 14.16% -> 0.00% (-14.16%) спад
  - inside_volatile: 12.01% -> 0.00% (-12.01%) спад
  - fastutil: 8.54% -> 52.74% (+44.20%) РОСТ
  - java_util: 7.01% -> 42.79% (+35.77%) РОСТ
  - paletted: 6.41% -> 0.00% (-6.41%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
