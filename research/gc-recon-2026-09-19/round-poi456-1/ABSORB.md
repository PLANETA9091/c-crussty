# absorb ROUND (poi456-1, run 36121914548, branch round-456b-poi-1, head 084368d)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6765332 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6765332 (поллов=5); TPS_exp=2.22; normalized=+16.9%
- GC: young=107, Full=9, total=19.8s, avg=171ms, max=2558ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104153 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.48% (-0.24%) флэт
  - broadphase: 15.66% -> 10.92% (-4.74%) спад
  - nav_ai: 14.16% -> 2.94% (-11.22%) спад
  - inside_volatile: 12.01% -> 17.17% (+5.17%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 8.55% (+1.54%) РОСТ
  - paletted: 6.41% -> 5.45% (-0.95%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
