# absorb ROUND (451a-anchor-1, run 36069426167, branch round-451-anchor-1, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7091497 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7091497 (поллов=5); TPS_exp=2.29; normalized=-8.4%
- GC: young=109, Full=10, total=24.1s, avg=203ms, max=2418ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116334 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.95% (-2.22%) спад
  - fluid: 16.72% -> 15.39% (-1.32%) спад
  - broadphase: 15.66% -> 14.93% (-0.73%) флэт
  - nav_ai: 14.16% -> 13.52% (-0.65%) флэт
  - inside_volatile: 12.01% -> 11.09% (-0.92%) флэт
  - fastutil: 8.54% -> 8.39% (-0.14%) флэт
  - java_util: 7.01% -> 6.54% (-0.47%) флэт
  - paletted: 6.41% -> 6.10% (-0.31%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
