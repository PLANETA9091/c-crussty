# absorb ROUND (435a-anchor-b, run 35924874226, branch round-435a-anchor-b, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8943661 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8943661 (поллов=5); TPS_exp=2.68; normalized=-3.1%
- GC: young=124, Full=10, total=22.8s, avg=170ms, max=2156ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114156 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.67% (-2.50%) спад
  - fluid: 16.72% -> 16.69% (-0.02%) флэт
  - broadphase: 15.66% -> 15.03% (-0.62%) флэт
  - nav_ai: 14.16% -> 13.75% (-0.41%) флэт
  - inside_volatile: 12.01% -> 10.63% (-1.37%) спад
  - fastutil: 8.54% -> 8.97% (+0.43%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 7.06% (+0.66%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
