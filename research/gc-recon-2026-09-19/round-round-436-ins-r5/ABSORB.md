# absorb ROUND (round-436-ins-r5, run 35930411202, branch round-436-ins-r5, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7123327 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7123327 (поллов=5); TPS_exp=2.30; normalized=+4.4%
- GC: young=100, Full=6, total=12.9s, avg=122ms, max=957ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104385 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 9.90% (-5.76%) спад
  - nav_ai: 14.16% -> 3.38% (-10.78%) спад
  - inside_volatile: 12.01% -> 16.52% (+4.52%) РОСТ
  - fastutil: 8.54% -> 6.95% (-1.59%) спад
  - java_util: 7.01% -> 8.71% (+1.69%) РОСТ
  - paletted: 6.41% -> 5.00% (-1.41%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
