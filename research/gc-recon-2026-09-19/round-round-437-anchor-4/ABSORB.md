# absorb ROUND (round-437-anchor-4, run 35935325219, branch round-437-anchor-4, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6997471 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6997471 (поллов=6); TPS_exp=2.27; normalized=+10.0%
- GC: young=114, Full=9, total=21.2s, avg=173ms, max=2396ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116867 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.58% (-1.59%) спад
  - fluid: 16.72% -> 15.68% (-1.04%) спад
  - broadphase: 15.66% -> 14.73% (-0.93%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.49%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
