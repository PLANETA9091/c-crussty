# absorb ROUND (diet454-4r, run 36098079369, branch round-454c-diet-4r, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6886537 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6886537 (поллов=6); TPS_exp=2.25; normalized=+17.8%
- GC: young=101, Full=9, total=19.2s, avg=175ms, max=2723ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104727 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.04% (-0.67%) флэт
  - broadphase: 15.66% -> 9.62% (-6.03%) спад
  - nav_ai: 14.16% -> 3.19% (-10.97%) спад
  - inside_volatile: 12.01% -> 16.95% (+4.94%) РОСТ
  - fastutil: 8.54% -> 6.97% (-1.57%) спад
  - java_util: 7.01% -> 8.91% (+1.90%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
