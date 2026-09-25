# absorb ROUND (poi457-7, run 36131807435, branch round-456b-poi-7, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7523645 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7523645 (поллов=5); TPS_exp=2.38; normalized=+9.1%
- GC: young=115, Full=9, total=22.9s, avg=185ms, max=3011ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105622 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.79% (+2.07%) РОСТ
  - broadphase: 15.66% -> 9.62% (-6.04%) спад
  - nav_ai: 14.16% -> 3.46% (-10.70%) спад
  - inside_volatile: 12.01% -> 17.91% (+5.90%) РОСТ
  - fastutil: 8.54% -> 5.94% (-2.60%) спад
  - java_util: 7.01% -> 8.68% (+1.67%) РОСТ
  - paletted: 6.41% -> 5.52% (-0.88%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
