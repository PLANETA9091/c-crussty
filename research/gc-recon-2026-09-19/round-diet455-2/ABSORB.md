# absorb ROUND (diet455-2, run 36104860817, branch round-455c-diet-2, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7161414 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7161414 (поллов=6); TPS_exp=2.31; normalized=-0.3%
- GC: young=103, Full=9, total=18.6s, avg=166ms, max=2631ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104577 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.15% (-0.57%) флэт
  - broadphase: 15.66% -> 10.30% (-5.35%) спад
  - nav_ai: 14.16% -> 3.34% (-10.82%) спад
  - inside_volatile: 12.01% -> 16.52% (+4.51%) РОСТ
  - fastutil: 8.54% -> 6.66% (-1.88%) спад
  - java_util: 7.01% -> 8.75% (+1.74%) РОСТ
  - paletted: 6.41% -> 5.29% (-1.11%) спад
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **PARITY/LOW**
