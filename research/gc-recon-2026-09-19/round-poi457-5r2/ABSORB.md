# absorb ROUND (poi457-5r2, run 36132360006, branch round-456b-poi-5r2, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6733336 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6733336 (поллов=5); TPS_exp=2.22; normalized=-5.3%
- GC: young=105, Full=9, total=20.7s, avg=182ms, max=2782ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103898 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.78% (+0.06%) флэт
  - broadphase: 15.66% -> 9.90% (-5.75%) спад
  - nav_ai: 14.16% -> 3.15% (-11.01%) спад
  - inside_volatile: 12.01% -> 16.84% (+4.84%) РОСТ
  - fastutil: 8.54% -> 6.54% (-2.00%) спад
  - java_util: 7.01% -> 8.73% (+1.72%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.07%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
