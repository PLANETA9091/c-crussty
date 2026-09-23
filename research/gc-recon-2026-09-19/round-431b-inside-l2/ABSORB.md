# absorb ROUND (431b-inside-l2, run 35883948551, branch round-431b-inside-l2, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6665552 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6665552 (поллов=5); TPS_exp=2.20; normalized=+13.5%
- GC: young=104, Full=9, total=19.2s, avg=170ms, max=2560ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104597 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.14% (+0.42%) флэт
  - broadphase: 15.66% -> 9.67% (-5.99%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 15.20% (+3.20%) РОСТ
  - fastutil: 8.54% -> 6.60% (-1.93%) спад
  - java_util: 7.01% -> 8.73% (+1.72%) РОСТ
  - paletted: 6.41% -> 5.72% (-0.69%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
