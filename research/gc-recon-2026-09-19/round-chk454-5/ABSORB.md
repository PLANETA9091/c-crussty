# absorb ROUND (chk454-5, run 36097317654, branch round-454b-chunk-5, head 797ae4f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6539095 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.50 @ 6539095 (поллов=6); TPS_exp=2.18; normalized=+14.9%
- GC: young=102, Full=9, total=20.8s, avg=187ms, max=2940ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104053 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.61% (-0.10%) флэт
  - broadphase: 15.66% -> 9.59% (-6.07%) спад
  - nav_ai: 14.16% -> 3.29% (-10.87%) спад
  - inside_volatile: 12.01% -> 16.34% (+4.34%) РОСТ
  - fastutil: 8.54% -> 6.41% (-2.13%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.74% (-0.67%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **CRASH-REFUTED**
