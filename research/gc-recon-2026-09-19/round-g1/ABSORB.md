# absorb ROUND (g1, run 35739896032, branch round-415-b-g1, head e43a948)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8581368 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8581368 (поллов=5); TPS_exp=2.61; normalized=-4.1%
- GC: young=119, Full=8, total=19.8s, avg=156ms, max=2052ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112429 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.74% (-2.43%) спад
  - fluid: 16.72% -> 16.01% (-0.70%) флэт
  - broadphase: 15.66% -> 15.95% (+0.29%) флэт
  - nav_ai: 14.16% -> 5.80% (-8.36%) спад
  - inside_volatile: 12.01% -> 10.05% (-1.96%) спад
  - fastutil: 8.54% -> 7.69% (-0.84%) флэт
  - java_util: 7.01% -> 7.39% (+0.38%) флэт
  - paletted: 6.41% -> 6.79% (+0.39%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
