# absorb ROUND (g2, run 35739916185, branch round-415-b-g2, head e43a948)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7613553 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 7613553 (поллов=6); TPS_exp=2.40; normalized=-2.2%
- GC: young=109, Full=8, total=24.0s, avg=205ms, max=3024ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115033 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.42% (-0.75%) флэт
  - fluid: 16.72% -> 17.56% (+0.85%) флэт
  - broadphase: 15.66% -> 16.28% (+0.63%) флэт
  - nav_ai: 14.16% -> 5.96% (-8.20%) спад
  - inside_volatile: 12.01% -> 11.51% (-0.50%) флэт
  - fastutil: 8.54% -> 8.14% (-0.40%) флэт
  - java_util: 7.01% -> 7.05% (+0.04%) флэт
  - paletted: 6.41% -> 6.49% (+0.09%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
