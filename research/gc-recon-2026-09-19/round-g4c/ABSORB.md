# absorb ROUND (g4c, run 35754138828, branch round-416-b-g4c, head 89b3bff)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6607482 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6607482 (поллов=6); TPS_exp=2.19; normalized=+11.9%
- GC: young=104, Full=9, total=20.9s, avg=185ms, max=2746ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113071 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.54% (-2.17%) спад
  - broadphase: 15.66% -> 15.60% (-0.05%) флэт
  - nav_ai: 14.16% -> 6.75% (-7.41%) спад
  - inside_volatile: 12.01% -> 11.64% (-0.36%) флэт
  - fastutil: 8.54% -> 8.71% (+0.17%) флэт
  - java_util: 7.01% -> 7.34% (+0.33%) флэт
  - paletted: 6.41% -> 5.63% (-0.78%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
