# absorb ROUND (diet454-6, run 36097294915, branch round-454c-diet-6, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7056069 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 7056069 (поллов=6); TPS_exp=2.28; normalized=-10.3%
- GC: young=97, Full=9, total=19.8s, avg=187ms, max=3029ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103690 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.07% (-0.65%) флэт
  - broadphase: 15.66% -> 9.95% (-5.70%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.12% (+4.12%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.09%) спад
  - java_util: 7.01% -> 8.53% (+1.51%) РОСТ
  - paletted: 6.41% -> 5.03% (-1.38%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
