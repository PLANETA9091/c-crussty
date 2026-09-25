# absorb ROUND (a25-456w2, run 36116771081, branch round-456-anchor-25, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6628417 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6628417 (поллов=5); TPS_exp=2.19; normalized=-4.3%
- GC: young=108, Full=9, total=20.1s, avg=172ms, max=2357ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116490 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.24% (-1.93%) спад
  - fluid: 16.72% -> 15.60% (-1.12%) спад
  - broadphase: 15.66% -> 14.97% (-0.69%) флэт
  - nav_ai: 14.16% -> 13.78% (-0.38%) флэт
  - inside_volatile: 12.01% -> 11.25% (-0.75%) флэт
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.68% (-0.33%) флэт
  - paletted: 6.41% -> 6.30% (-0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
