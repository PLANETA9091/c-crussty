# absorb ROUND (chkmono457-6, run 36131753219, branch round-456c-chunkmono-6, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6423955 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6423955 (поллов=5); TPS_exp=2.15; normalized=+11.5%
- GC: young=101, Full=9, total=22.0s, avg=200ms, max=2762ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104352 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.40% (+0.68%) флэт
  - broadphase: 15.66% -> 9.36% (-6.30%) спад
  - nav_ai: 14.16% -> 3.00% (-11.16%) спад
  - inside_volatile: 12.01% -> 16.28% (+4.27%) РОСТ
  - fastutil: 8.54% -> 6.64% (-1.89%) спад
  - java_util: 7.01% -> 8.87% (+1.86%) РОСТ
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
