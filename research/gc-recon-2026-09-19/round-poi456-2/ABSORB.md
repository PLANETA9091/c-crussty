# absorb ROUND (poi456-2, run 36122777112, branch round-456b-poi-2, head 084368d)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=6014, col=PARALLEL, runner=8791818 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8791818 (поллов=6); TPS_exp=2.65; normalized=+17.0%
- GC: young=110, Full=9, total=22.2s, avg=186ms, max=3050ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104800 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.91% (+2.20%) РОСТ
  - broadphase: 15.66% -> 10.73% (-4.92%) спад
  - nav_ai: 14.16% -> 2.98% (-11.19%) спад
  - inside_volatile: 12.01% -> 18.16% (+6.15%) РОСТ
  - fastutil: 8.54% -> 7.41% (-1.12%) спад
  - java_util: 7.01% -> 9.80% (+2.79%) РОСТ
  - paletted: 6.41% -> 6.73% (+0.33%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **DELIVERY-FAIL**
