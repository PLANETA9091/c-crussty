# absorb ROUND (round-439-anchor-2, run 35946033619, branch round-439-anchor-2, head afbcde6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8538281 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8538281 (поллов=5); TPS_exp=2.60; normalized=+4.0%
- GC: young=126, Full=10, total=21.8s, avg=161ms, max=2074ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114073 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.24% (-1.93%) спад
  - fluid: 16.72% -> 16.43% (-0.29%) флэт
  - broadphase: 15.66% -> 15.13% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.16% (-1.00%) спад
  - inside_volatile: 12.01% -> 10.86% (-1.14%) спад
  - fastutil: 8.54% -> 8.94% (+0.40%) флэт
  - java_util: 7.01% -> 6.47% (-0.54%) флэт
  - paletted: 6.41% -> 6.89% (+0.48%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
