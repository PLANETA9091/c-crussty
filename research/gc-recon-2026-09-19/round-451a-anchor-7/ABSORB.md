# absorb ROUND (451a-anchor-7, run 36069488862, branch round-451-anchor-7, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6620735 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6620735 (поллов=5); TPS_exp=2.19; normalized=+4.9%
- GC: young=111, Full=9, total=21.2s, avg=177ms, max=2665ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116955 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.72% (-1.45%) спад
  - fluid: 16.72% -> 16.09% (-0.63%) флэт
  - broadphase: 15.66% -> 15.06% (-0.59%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.42%) флэт
  - fastutil: 8.54% -> 8.51% (-0.02%) флэт
  - java_util: 7.01% -> 6.43% (-0.58%) флэт
  - paletted: 6.41% -> 6.10% (-0.30%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
