# absorb ROUND (round-441-chk-1r, run 35954910561, branch round-441-chk-1r, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7217166 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7217166 (поллов=5); TPS_exp=2.32; normalized=+12.1%
- GC: young=102, Full=8, total=16.7s, avg=152ms, max=2308ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104503 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.31% (-0.40%) флэт
  - broadphase: 15.66% -> 9.93% (-5.73%) спад
  - nav_ai: 14.16% -> 3.23% (-10.94%) спад
  - inside_volatile: 12.01% -> 15.75% (+3.74%) РОСТ
  - fastutil: 8.54% -> 6.46% (-2.08%) спад
  - java_util: 7.01% -> 8.25% (+1.24%) РОСТ
  - paletted: 6.41% -> 5.33% (-1.08%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
