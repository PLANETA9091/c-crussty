# absorb ROUND (a19, run 36108638112, branch round-455-anchor-19, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9151169 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 9151169 (поллов=5); TPS_exp=2.73; normalized=-12.0%
- GC: young=122, Full=9, total=25.7s, avg=196ms, max=3204ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117018 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.33% (-1.84%) спад
  - fluid: 16.72% -> 17.92% (+1.20%) РОСТ
  - broadphase: 15.66% -> 14.83% (-0.83%) флэт
  - nav_ai: 14.16% -> 13.12% (-1.05%) спад
  - inside_volatile: 12.01% -> 11.75% (-0.26%) флэт
  - fastutil: 8.54% -> 8.28% (-0.26%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 7.47% (+1.06%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
