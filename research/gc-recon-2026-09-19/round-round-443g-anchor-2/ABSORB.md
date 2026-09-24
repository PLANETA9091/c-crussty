# absorb ROUND (round-443g-anchor-2, run 36038474938, branch round-443g-anchor-2, head b2e1993)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6640881 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6640881 (поллов=5); TPS_exp=2.20; normalized=+0.1%
- GC: young=110, Full=9, total=20.6s, avg=173ms, max=2412ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116672 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.39% (-1.78%) спад
  - fluid: 16.72% -> 15.99% (-0.73%) флэт
  - broadphase: 15.66% -> 15.13% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.79% (-0.38%) флэт
  - inside_volatile: 12.01% -> 11.23% (-0.77%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.66% (-0.36%) флэт
  - paletted: 6.41% -> 6.41% (-0.00%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
