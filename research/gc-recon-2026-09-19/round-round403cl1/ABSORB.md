# absorb ROUND (round403cl1, run 35606813534, branch round-403-c-leg1, head bab6b74)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8493104 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8493104 (поллов=5); TPS_exp=2.59; normalized=-7.3%
- GC: young=106, Full=9, total=21.3s, avg=185ms, max=2646ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113305 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 19.18% (+2.46%) РОСТ
  - broadphase: 15.66% -> 15.70% (+0.04%) флэт
  - nav_ai: 14.16% -> 9.43% (-4.73%) спад
  - inside_volatile: 12.01% -> 11.99% (-0.02%) флэт
  - fastutil: 8.54% -> 7.33% (-1.21%) спад
  - java_util: 7.01% -> 7.33% (+0.31%) флэт
  - paletted: 6.41% -> 7.72% (+1.31%) РОСТ
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
