# absorb ROUND (round-432-anchor-a, run 35891248141, branch round-432-anchor-a, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7031744 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7031744 (поллов=5); TPS_exp=2.28; normalized=-3.5%
- GC: young=113, Full=9, total=20.8s, avg=171ms, max=2399ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117470 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.86% (-1.32%) спад
  - fluid: 16.72% -> 15.71% (-1.01%) спад
  - broadphase: 15.66% -> 14.97% (-0.69%) флэт
  - nav_ai: 14.16% -> 13.47% (-0.70%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.23%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.77% (-0.25%) флэт
  - paletted: 6.41% -> 6.07% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
