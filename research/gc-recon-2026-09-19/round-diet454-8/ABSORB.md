# absorb ROUND (diet454-8, run 36098965164, branch round-454c-diet-8, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7342464 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 7342464 (поллов=6); TPS_exp=2.35; normalized=-8.3%
- GC: young=97, Full=9, total=25.3s, avg=239ms, max=3534ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103934 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.43% (+1.72%) РОСТ
  - broadphase: 15.66% -> 9.70% (-5.95%) спад
  - nav_ai: 14.16% -> 3.20% (-10.96%) спад
  - inside_volatile: 12.01% -> 16.50% (+4.49%) РОСТ
  - fastutil: 8.54% -> 5.90% (-2.64%) спад
  - java_util: 7.01% -> 8.78% (+1.77%) РОСТ
  - paletted: 6.41% -> 6.18% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
