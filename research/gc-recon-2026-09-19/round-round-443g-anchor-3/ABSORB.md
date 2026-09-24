# absorb ROUND (round-443g-anchor-3, run 36038509864, branch round-443g-anchor-3, head b2e1993)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7226847 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 7226847 (поллов=6); TPS_exp=2.32; normalized=-7.4%
- GC: young=109, Full=9, total=20.5s, avg=173ms, max=2424ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116286 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.34% (-1.83%) спад
  - fluid: 16.72% -> 15.86% (-0.86%) флэт
  - broadphase: 15.66% -> 15.52% (-0.14%) флэт
  - nav_ai: 14.16% -> 14.23% (+0.06%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.58%) флэт
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.41% (-0.60%) флэт
  - paletted: 6.41% -> 6.14% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
