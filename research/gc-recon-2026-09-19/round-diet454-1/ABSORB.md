# absorb ROUND (diet454-1, run 36094602603, branch round-454c-diet-1, head 451c4e4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6714437 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6714437 (поллов=5); TPS_exp=2.21; normalized=-5.1%
- GC: young=107, Full=9, total=21.1s, avg=182ms, max=2538ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116871 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.23% (-1.94%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 15.39% (-0.27%) флэт
  - nav_ai: 14.16% -> 13.83% (-0.34%) флэт
  - inside_volatile: 12.01% -> 11.29% (-0.71%) флэт
  - fastutil: 8.54% -> 8.82% (+0.28%) флэт
  - java_util: 7.01% -> 6.39% (-0.63%) флэт
  - paletted: 6.41% -> 6.22% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
