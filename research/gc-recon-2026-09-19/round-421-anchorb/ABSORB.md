# absorb ROUND (421-anchorb, run 35800926548, branch round-421-anchorb, head 2d23f45)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6795554 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 6795554 (поллов=6); TPS_exp=2.23; normalized=-3.6%
- GC: young=107, Full=5, total=15.3s, avg=136ms, max=537ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115755 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.86% (-2.31%) спад
  - fluid: 16.72% -> 15.61% (-1.11%) спад
  - broadphase: 15.66% -> 16.19% (+0.53%) флэт
  - nav_ai: 14.16% -> 14.26% (+0.10%) флэт
  - inside_volatile: 12.01% -> 10.68% (-1.33%) спад
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.29% (-0.72%) флэт
  - paletted: 6.41% -> 6.05% (-0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
