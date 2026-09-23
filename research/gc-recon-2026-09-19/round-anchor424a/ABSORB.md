# absorb ROUND (anchor424a, run 35825614835, branch round-424-anchora, head a8ef2fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6911381 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6911381 (поллов=5); TPS_exp=2.25; normalized=-2.4%
- GC: young=105, Full=8, total=20.5s, avg=181ms, max=2395ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117407 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.13% (-1.04%) спад
  - fluid: 16.72% -> 15.87% (-0.84%) флэт
  - broadphase: 15.66% -> 15.08% (-0.58%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.38%) флэт
  - fastutil: 8.54% -> 8.68% (+0.14%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 6.17% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
