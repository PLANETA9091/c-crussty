# absorb ROUND (round410anchora, run 35673607127, branch round-410-anchora, head 8ef9e5b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8548287 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8548287 (поллов=5); TPS_exp=2.60; normalized=-3.8%
- GC: young=117, Full=9, total=24.1s, avg=191ms, max=2792ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115385 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.96% (-0.21%) флэт
  - fluid: 16.72% -> 18.28% (+1.56%) РОСТ
  - broadphase: 15.66% -> 15.67% (+0.01%) флэт
  - nav_ai: 14.16% -> 14.16% (-0.01%) флэт
  - inside_volatile: 12.01% -> 11.76% (-0.24%) флэт
  - fastutil: 8.54% -> 9.21% (+0.67%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 7.74% (+1.33%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
