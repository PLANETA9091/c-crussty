# absorb ROUND (round-435a-anchor-a, run 35924848594, branch round-435a-anchor-a, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6517874 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6517874 (поллов=5); TPS_exp=2.17; normalized=-7.9%
- GC: young=107, Full=9, total=23.4s, avg=202ms, max=2703ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115196 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.09% (-2.08%) спад
  - fluid: 16.72% -> 16.66% (-0.06%) флэт
  - broadphase: 15.66% -> 14.54% (-1.12%) спад
  - nav_ai: 14.16% -> 13.34% (-0.82%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.28%) спад
  - fastutil: 8.54% -> 8.50% (-0.03%) флэт
  - java_util: 7.01% -> 6.55% (-0.46%) флэт
  - paletted: 6.41% -> 7.06% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
