# absorb ROUND (a24-456w2, run 36116759710, branch round-456-anchor-24, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8928192 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8928192 (поллов=5); TPS_exp=2.68; normalized=-10.4%
- GC: young=111, Full=9, total=23.5s, avg=196ms, max=3388ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117257 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.82% (-1.35%) спад
  - fluid: 16.72% -> 18.20% (+1.49%) РОСТ
  - broadphase: 15.66% -> 14.75% (-0.91%) флэт
  - nav_ai: 14.16% -> 13.58% (-0.58%) флэт
  - inside_volatile: 12.01% -> 11.61% (-0.40%) флэт
  - fastutil: 8.54% -> 8.35% (-0.19%) флэт
  - java_util: 7.01% -> 7.87% (+0.86%) флэт
  - paletted: 6.41% -> 7.32% (+0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
