# absorb ROUND (round405comp2b, run 35633618630, branch round-405-comp-l2, head fef3746)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6784450 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6784450 (поллов=6); TPS_exp=2.23; normalized=+21.2%
- GC: young=104, Full=9, total=18.8s, avg=166ms, max=2293ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109723 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.65% (-0.06%) флэт
  - broadphase: 15.66% -> 14.73% (-0.93%) флэт
  - nav_ai: 14.16% -> 9.81% (-4.35%) спад
  - inside_volatile: 12.01% -> 12.31% (+0.30%) флэт
  - fastutil: 8.54% -> 7.87% (-0.67%) флэт
  - java_util: 7.01% -> 7.27% (+0.26%) флэт
  - paletted: 6.41% -> 5.60% (-0.81%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
