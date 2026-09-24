# absorb ROUND (round-433-anchor-h2, run 35910315122, branch round-433-anchor-h2, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8347026 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 8347026 (поллов=6); TPS_exp=2.56; normalized=-15.9%
- GC: young=112, Full=10, total=28.5s, avg=233ms, max=2863ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116918 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.52% (-1.65%) спад
  - fluid: 16.72% -> 17.51% (+0.79%) флэт
  - broadphase: 15.66% -> 15.32% (-0.34%) флэт
  - nav_ai: 14.16% -> 13.32% (-0.85%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.69%) флэт
  - fastutil: 8.54% -> 8.74% (+0.21%) флэт
  - java_util: 7.01% -> 6.37% (-0.64%) флэт
  - paletted: 6.41% -> 7.17% (+0.76%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
